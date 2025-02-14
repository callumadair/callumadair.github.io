use actix_settings::{
    ApplySettings as _,
    Mode,
    Settings,
};
use actix_web::{
    middleware::{
        Compress,
        Condition,
    },
    web::Data,
    App,
    HttpServer,
};
use portfolio_backend_lib::http_api::handlers::index::index;
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::{
    scope,
    AppExt,
};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> std::io::Result<()>
{
    let mut settings =
        Settings::parse_toml("./Server.toml").expect("Failed to parse `Settings` from Server.toml");

    // If the environment variable `$APPLICATION__HOSTS` is set,
    // have its value override the `settings.actix.hosts`
    // setting:
    Settings::override_field_with_env_var(&mut settings.actix.hosts, "APPLICATION__HOSTS")?;

    init_logger(&settings);

    HttpServer::new( {
        // clone settings into each worker thread
        let settings = settings.clone();

        move || {
            App::new()
                .into_utoipa_app()
                .map(|app|
                    app.wrap(Condition::new(
                       settings.actix.enable_compression,
                       Compress::default(),
                   ))
                   .app_data(Data::new(settings.clone()))
                   .wrap(TracingLogger::default())
                )
                .service(index)
                .openapi_service(|api| {
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
                })
                .into_app()

                // add request handlers as normal
        }
    })
        // apply the `Settings` to Actix Web's `HttpServer` 
        .try_apply_settings(&settings)?
        .run()
        .await
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
