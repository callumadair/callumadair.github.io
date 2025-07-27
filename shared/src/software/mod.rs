#[cfg(feature = "backend")]
use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    HttpRequest,
    HttpResponse,
    Responder,
};
use dioxus::{
    dioxus_core::DynamicNode,
    prelude::*,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    components::{
        Modal,
        ModalButton,
    },
    traits::{
        contains::Contains,
        modal::ModalDisplay,
    },
};

#[derive(Serialize, Deserialize, PartialEq, Clone, utoipa::ToSchema)]
pub struct SoftwareTool
{
    pub name:        String,
    pub short_desc:  String,
    pub long_desc:   String,
    pub web_link:    String,
    pub image_links: Vec<String>,
}

impl Contains for SoftwareTool
{
    fn contains(
        &self,
        key: &str,
    ) -> bool
    {
        self.short_desc.contains(key)
            || self.long_desc.contains(key)
            || self.web_link.contains(key)
            || self.name.contains(key)
    }
}

impl ModalDisplay for SoftwareTool
{
    fn display(&self) -> Element
    {
        rsx! {
            ModalButton {
                modal_id: format!("{}-modal", self.name.clone()),
                modal_button_text: "More Info",
            }

            Modal<String> {
                id: format!("{}-modal", self.name.clone()),
                content: format!("{} is neat.", self.name.clone())
            }
        }
    }
}

impl IntoDynNode for SoftwareTool
{
    fn into_dyn_node(self) -> DynamicNode
    {
        rsx! {
            tr {

                td {
                    {self.name.clone()}
                },

                td {
                   { self.short_desc.clone() }
                   },

                td{
                    a {
                        target: "_blank",
                        href: self.web_link.clone(),
                        {"Website"}
                    }
                },

                td {
                    { self.display() }
                }
            }
        }
        .into_dyn_node()
    }
}

#[cfg(feature = "backend")]
impl Responder for SoftwareTool
{
    type Body = BoxBody;

    fn respond_to(
        self,
        _req: &HttpRequest,
    ) -> HttpResponse<Self::Body>
    {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
