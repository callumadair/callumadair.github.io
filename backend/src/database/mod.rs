// use std::time::Duration;
// use sea_orm::sqlx::PgConnection;
// use sea_orm::sqlx::postgres::PgConnectOptions;
// 
// const DATABASE_URL: &str = "";
// fn init_connection() -> PgConnection {
//     let mut opt = PgConnectOptions::new(DATABASE_URL);
//     
//     opt.max_connections(100)
//     .min_connections(5)
//     .connect_timeout(Duration::from_secs(8))
//     .acquire_timeout(Duration::from_secs(8))
//     .idle_timeout(Duration::from_secs(8))
//     .max_lifetime(Duration::from_secs(8))
//     .sqlx_logging(true)
//     .sqlx_logging_level(log::LevelFilter::Info)
//     .set_schema_search_path("my_schema"); 
// }