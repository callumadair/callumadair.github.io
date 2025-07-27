use std::time::Duration;

use actix_settings::{
    ApplySettings,
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
use migration::{
    Migrator,
    MigratorTrait,
};
use portfolio_backend_lib::{
    http_api::{
        handlers,
        handlers::index::index,
    },
    AppState,
};
use sea_orm::{
    ConnectOptions,
    Database,
    DatabaseConnection,
};
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

const DATABASE_PATH: &str = "postgres://postgres:password@localhost:5432";

#[tokio::main]
async fn main() -> color_eyre::Result<()>
{
    color_eyre::install()?;
    let mut settings =
        Settings::parse_toml("./Server.toml").expect("Failed to parse `Settings` from Server.toml");

    // If the environment variable `$APPLICATION__HOSTS` is set,
    // have its value override the `settings.actix.hosts`
    // setting:
    Settings::override_field_with_env_var(&mut settings.actix.hosts, "APPLICATION__HOSTS")?;

    init_logger(&settings);
    let db_conn = init_database().await?;

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
async fn init_database() -> color_eyre::Result<DatabaseConnection>
{
    let mut opt = ConnectOptions::new(DATABASE_PATH);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(30))
        .sqlx_logging(true);
    let db_conn: DatabaseConnection = Database::connect(opt).await?;
    Migrator::up(&db_conn, None).await?;

    Ok(db_conn)
}
