use std::rc::Rc;

use shared::software::SoftwareTool;
use yew::prelude::*;

use crate::{
    components::{
        modal::{
            Modal,
            ModalButton,
        },
        table::Table,
    },
    traits::{
        contains::Contains,
        modal::ModalDisplay,
    },
};

#[derive(PartialEq, Clone)]
pub(crate) struct SoftwareToolRow
{
    pub(crate) name:        String,
    pub(crate) short_desc:  String,
    pub(crate) long_desc:   String,
    pub(crate) web_link:    String,
    pub(crate) image_links: Vec<String>,
}

impl From<SoftwareTool> for SoftwareToolRow
{
    // TODO add a derive macro for this?
    fn from(value: SoftwareTool) -> Self
    {
        Self {
            name:        value.name,
            short_desc:  value.short_desc,
            long_desc:   value.long_desc,
            web_link:    value.web_link,
            image_links: value.image_links,
        }
    }
}

impl ModalDisplay for SoftwareToolRow
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

impl ToHtml for SoftwareToolRow
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

impl Contains for SoftwareToolRow
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

#[function_component(SoftwareBase)]
pub fn base() -> Html
{
    html! {
        <div class="flex space-y-4 p-3">
            <CLIBase/>
        </div>
    }
}

#[function_component(CLIBase)]
fn cli_tools() -> Html
{
    let cols = Rc::from(["Name", "Description", "Link"].map(AttrValue::from));
    let rows = get_rows();
    let searchable = true;

    html! {
        <div class="flex flex-col w-full text-primary">

            <Table<SoftwareToolRow>
                id="cli-table"
                title="CLI Tools I like"
                {cols}
                {rows}
                {searchable}
            />

        </div>
    }
}

// TODO replace this with an HTTP GET to the backend
// whenever I get round to making it if ever.
fn get_rows() -> Vec<SoftwareToolRow>
{
    let starship = SoftwareToolRow {
        name:        "Starship".to_string(),
        short_desc:  "A nice modern terminal prompt".to_string(),
        web_link:    "https://starship.rs".to_string(),
        long_desc:   String::new(),
        image_links: Vec::new(),
    };
    let hyperfine = SoftwareToolRow {
        name:        "Hyperfine".to_string(),
        short_desc:  "A benchmarking tool written in rust".to_string(),
        web_link:    "https://github.com/sharkdp/hyperfine".to_string(),
        long_desc:   String::new(),
        image_links: Vec::new(),
    };
    let nushell = SoftwareToolRow {
        name:        "Nushell".to_string(),
        short_desc:  "A new way of doing shells".to_string(),
        web_link:    "https://www.nushell.sh".to_string(),
        long_desc:   String::new(),
        image_links: Vec::new(),
    };

    vec![starship, hyperfine, nushell]
}
