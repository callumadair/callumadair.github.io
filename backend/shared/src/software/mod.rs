use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    body::EitherBody,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, utoipa::ToSchema)]
pub struct SoftwareTool
{
    pub name:        String,
    pub short_desc:  String,
    pub long_desc:   String,
    pub web_link:    String,
    pub image_links: Vec<String>,
}

impl Responder for SoftwareTool
{
    type Body = EitherBody<String>;

    fn respond_to(
        self,
        _req: &HttpRequest,
    ) -> HttpResponse<Self::Body>
    {
        let json = actix_web::web::Json(&self);
        json.respond_to(_req)
    }
}
