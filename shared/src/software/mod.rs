use dioxus::{
    dioxus_core::DynamicNode,
    prelude::*,
};
use regex::Regex;
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
        let re = Regex::new(key).unwrap();
        re.is_match(&self.short_desc)
            || re.is_match(&self.long_desc)
            || re.is_match(&self.web_link)
            || re.is_match(&self.name)
    }
}

impl ModalDisplay for SoftwareTool
{
    fn display(&self) -> Element
    {
        rsx! {
            ModalButton {
                modal_id: {format!("{}-modal", self.name.clone())},
                modal_button_text: "More Info",
            }

            Modal<String> {
                id: {format!("{}-modal", self.name.clone())},
                content: {format!("{} is neat.", self.name.clone())}
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
                        href: {self.web_link.clone()},
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
