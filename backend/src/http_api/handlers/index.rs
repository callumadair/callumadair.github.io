use actix_settings::{
    Mode,
    Settings,
};
use actix_web::{
    Responder,
    get,
    web,
};

#[utoipa::path(responses((status = 200, body = str)))]
#[get("/")]
pub async fn index(settings: web::Data<Settings>) -> impl Responder
{
    format!(
        r#"{{
  "mode": "{}",
  "hosts": ["{}"]
}}"#,
        match settings.actix.mode
        {
            Mode::Development => "development_tony",
            Mode::Production => "production",
        },
        settings
            .actix
            .hosts
            .iter()
            .map(|addr| { format!("{}:{}", addr.host, addr.port) })
            .collect::<Vec<_>>()
            .join(", "),
    )
    .customize()
    .insert_header(("content-type", "application/json"))
}
