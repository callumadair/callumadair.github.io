use std::borrow::Cow;

use dioxus::{
    dioxus_core::DynamicNode,
    prelude::*,
};
use serde::{
    Deserialize,
    Serialize,
};
use utoipa::openapi::{
    RefOr,
    Schema,
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

#[derive(Serialize, Deserialize, PartialEq, Clone)]
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

impl utoipa::ToSchema for SoftwareTool
{
    fn name() -> Cow<'static, str> { Cow::Borrowed("Software Tool") }
}

impl utoipa::PartialSchema for SoftwareTool
{
    // TODO fix this schema definition.
    fn schema() -> RefOr<Schema>
    {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                "Name",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String),
            )
            .property(
                "Short Description",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String),
            )
            .property(
                "Long Description",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String),
            )
            .property(
                "Web Link",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::String),
            )
            .property(
                "Image Links",
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Array),
            )
            .required("Name")
            .required("Short Description")
            .required("Long Description")
            .required("Web Link")
            .examples(Some(serde_json::json! {"message: server bad"}))
            .into()
    }
}
