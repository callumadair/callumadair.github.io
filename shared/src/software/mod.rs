use serde::{
    Deserialize,
    Serialize,
};
use yew::prelude::*;

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
    fn display(&self) -> Html
    {
        html! {
            <>

                <ModalButton modal_id={format!("{}-modal", self.name.clone())}
                    modal_button_text="More Info"
                />

                <Modal<AttrValue>
                    id={format!("{}-modal", self.name.clone())}
                    content={format!("{} is neat.", self.name.clone())}
                />
            </>
        }
    }
}

impl ToHtml for SoftwareTool
{
    fn to_html(&self) -> Html
    {
        html! {
            <tr>

                <td>
                    {self.name.clone()}
                </td>

                <td>
                    {self.short_desc.clone()}
                </td>

                <td>
                    <a target="_blank" href={self.web_link.clone()}>
                        {"Website"}
                    </a>
                </td>

                <td>
                    {self.display()}
                </td>


            </tr>
        }
    }
}
