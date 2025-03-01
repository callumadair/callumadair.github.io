use actix_web::{
    body::BoxBody,
    http::{
        header::ContentType,
        StatusCode,
    },
    HttpResponse,
    ResponseError,
};
use strum::Display;

#[derive(thiserror::Error, Display, Debug)]
pub enum Base
{
    Demo,
    OtherVariant,
}

impl ResponseError for Base
{
    fn status_code(&self) -> StatusCode
    {
        match self
        {
            Base::Demo => StatusCode::IM_A_TEAPOT,
            Base::OtherVariant => StatusCode::NOT_ACCEPTABLE,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody>
    {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

// impl IntoDynNode for Base
// {
//     fn into_dyn_node(self) -> DynamicNode
//     {
//         let res = rsx! {
//         div {
//             class: "alert alert-error",
//             role: "alert",
//             CircleX {
//                 size: 20
//             },
//             {self.to_string()}}
//         };
//         res.into_dyn_node()
//     }
// }
