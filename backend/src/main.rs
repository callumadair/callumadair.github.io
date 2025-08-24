use actix_settings::{
    ApplySettings,
    Mode,
    Settings,
};
use actix_web::{
    App,
    HttpServer,
    middleware::{
        Compress,
        Condition,
    },
    web::Data,
};
use portfolio_backend_lib::{
    AppState,
    http_api::handlers::{
        self,
        index::index,
    },
    metrics::types::{
        CounterName,
        Prometheus,
    },
    repositories::connections::SeaOrmDataBaseConnection,
    services::types::Service,
};
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

const DATABASE_PATH: &str = "postgres://postgres:password@localhost:5432";

#[tokio::main]
async fn main() -> portfolio_backend_lib::error::Result<()>
{
    color_eyre::install()?;
    let mut settings = Settings::parse_toml("./Server.toml")?;

    // If the environment variable `$APPLICATION__HOSTS` is set,
    // have its value override the `settings.actix.hosts`
    // setting:
    Settings::override_field_with_env_var(&mut settings.actix.hosts, "APPLICATION__HOSTS")?;

    init_logger(&settings);
    let repository = SeaOrmDataBaseConnection::new(DATABASE_PATH).await?;
    let software_opts = prometheus::Opts::new(
        "software_creation_failure",
        "Number of attempts to create a software entry that have failed.",
    );
    let prometheus_client = Prometheus::builder()
        .counter_opt(CounterName::SoftwareCreationFailure, software_opts)
        .build()?;
    let service = Service::new(repository, prometheus_client);
    let app_state = AppState::new(service);
    // let rustls_config = load_rustls_config()?;

    HttpServer::new({
        // clone settings into each worker thread
        let settings = settings.clone();
        let app_state = app_state.clone();

        move || {
            App::new()
                .into_utoipa_app()
                .map(|app|
                    app.wrap(Condition::new(
                        settings.actix.enable_compression,
                        Compress::default(),
                    ))
                        .app_data(Data::new(settings.clone()))
                        .app_data(Data::new(app_state.clone()))
                        .wrap(TracingLogger::default())
                )
                .service(index)
                .service(
                    utoipa_actix_web::scope("/software")
                        .configure(|cfg| {
                            cfg.service(handlers::software::index).service(handlers::software::create);
                        })
                )
                .openapi_service(|api| {
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
                })
                .into_app()
        }
    })
        // apply the `Settings` to Actix Web's `HttpServer` 
        .try_apply_settings(&settings)?
        // .bind_rustls_0_23(&format!("{}:{}", settings.actix.hosts[0].host, settings.actix.hosts[0].port),rustls_config)?
        .run()
        .await?;

    Ok(())
}

/// Initialize the logging infrastructure.
fn init_logger(settings: &Settings)
{
    if !settings.actix.enable_log
    {
        return;
    }

    unsafe {
        std::env::set_var(
            "RUST_LOG",
            match settings.actix.mode
            {
                Mode::Development => "actix_web=debug",
                Mode::Production => "actix_web=info",
            },
        );

        std::env::set_var("RUST_BACKTRACE", "1");
    }

    tracing_subscriber::fmt::init();
}

// TODO (CA): Actually start using this.
// #[allow(dead_code)]
// fn load_rustls_config() ->
// portfolio_backend_lib::error::Result<rustls::ServerConfig>
// {
//     rustls::crypto::aws_lc_rs::default_provider()
//         .install_default()?;

//     // load TLS key/cert files
//     let cert_chain =
// CertificateDer::pem_file_iter("cert.pem")?
//         .flatten()
//         .collect();

//     let key_der =
// PrivateKeyDer::from_pem_file("key.pem")?;

//     let server_config = ServerConfig::builder()
//         .with_no_client_auth()
//         .with_single_cert(cert_chain, key_der)?;
//     Ok(server_config)
// }
