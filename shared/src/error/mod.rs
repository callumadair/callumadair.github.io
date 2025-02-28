use actix_web::{
    body::BoxBody,
    http::StatusCode,
    HttpResponse,
    ResponseError,
};
use dioxus::{
    dioxus_core::DynamicNode,
    prelude::*,
};
use lucide_dioxus::CircleX;
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

    // fn error_response(&self) -> HttpResponse<BoxBody> {
    //    match self {
    //        Base::Demo => {}
    //        Base::OtherVariant => {}
    //    }
    // }
}

impl IntoDynNode for Base
{
    fn into_dyn_node(self) -> DynamicNode
    {
        let res = rsx! {
            div {
                class: "alert alert-error",
                role: "alert",
                {CircleX},
                {self.to_string()}
            }
        };
        res.into_dyn_node()
    }
}
