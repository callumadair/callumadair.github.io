use std::rc::Rc;

use yew::prelude::*;

use crate::{
    components::{
        modal::{
            Modal,
            ModalButton,
        },
        table::Table,
    },
    traits::contains::Contains,
};

#[derive(PartialEq, Clone)]
pub(crate) struct SoftwareTool
{
    pub(crate) name:       String,
    pub(crate) short_desc: String,
    pub(crate) link:       String,
    pub(crate) long_desc:  String,
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
                    <a target="_blank" href={self.link.clone()}>
                        {"Website"}
                    </a>
                </td>

                <td>
                    <ModalButton modal_id={format!("{}-modal", self.name.clone())}
                        modal_button_text="More Info"
                    />

                    <Modal<AttrValue>
                        id={format!("{}-modal", self.name.clone())}
                        content={format!("{} is neat.", self.name.clone())}
                    >


                    </Modal<AttrValue>>
                </td>


            </tr>
        }
    }
}

impl Contains for SoftwareTool
{
    fn contains(
        &self,
        key: &str,
    ) -> bool
    {
        self.short_desc.contains(key) || self.link.contains(key) || self.name.contains(key)
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

            <Table<SoftwareTool>
                id="cli-table"
                caption="CLI Tools I like"
                {cols}
                {rows}
                {searchable}
            />

        </div>
    }
}

// TODO replace this with an HTTP GET to the backend
// whenever I get round to making it if ever.
fn get_rows() -> Vec<SoftwareTool>
{
    let starship = SoftwareTool {
        name:       "Starship".to_string(),
        short_desc: "A nice modern terminal prompt".to_string(),
        link:       "https://starship.rs".to_string(),
        long_desc:  String::new(),
    };
    let hyperfine = SoftwareTool {
        name:       "Hyperfine".to_string(),
        short_desc: "A benchmarking tool written in rust".to_string(),
        link:       "https://github.com/sharkdp/hyperfine".to_string(),
        long_desc:  String::new(),
    };
    let nushell = SoftwareTool {
        name:       "Nushell".to_string(),
        short_desc: "A new way of doing shells".to_string(),
        link:       "https://www.nushell.sh".to_string(),
        long_desc:  String::new(),
    };

    vec![starship, hyperfine, nushell]
}
