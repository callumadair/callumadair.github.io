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
    http_api,
    http_api::handlers::index::index,
};
use sea_orm::{
    Database,
    DatabaseConnection,
};
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> color_eyre::Result<()>
{
    let mut settings =
        Settings::parse_toml("./Server.toml").expect("Failed to parse `Settings` from Server.toml");

    // If the environment variable `$APPLICATION__HOSTS` is set,
    // have its value override the `settings.actix.hosts`
    // setting:
    Settings::override_field_with_env_var(&mut settings.actix.hosts, "APPLICATION__HOSTS")?;
    let db_conn: DatabaseConnection =
        Database::connect("postgres://postgres:password@localhost:5432").await?;

    init_logger(&settings);

    HttpServer::new({
        // clone settings into each worker thread
        let settings = settings.clone();
        let db_conn = db_conn.clone();

        move || {
            App::new()
                .into_utoipa_app()
                .map(|app|
                    app.wrap(Condition::new(
                        settings.actix.enable_compression,
                        Compress::default(),
                    ))
                        .app_data(Data::new(settings.clone()))
                        .app_data(Data::new(
                            AppState {
                                db_conn: db_conn.clone(),
                            }
                        ))
                        .wrap(TracingLogger::default())
                )
                .service(index)
                .service(http_api::handlers::software::software_scope())
                .openapi_service(|api| {
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
                })
                .into_app()
        }
    })
        // apply the `Settings` to Actix Web's `HttpServer` 
        .try_apply_settings(&settings)?
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
