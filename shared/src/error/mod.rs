use actix_web::{
    body::BoxBody,
    http::StatusCode,
    HttpResponse,
    ResponseError,
};
use strum::Display;
use yew::prelude::*;
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

impl ToHtml for Base
{
    fn to_html(&self) -> Html
    {
        html! {
            <div class="alert alert-error"
                role="alert"
             >
                <lucide_yew::CircleX/>
                {self.to_string()}
            </div>
        }
    }
}
