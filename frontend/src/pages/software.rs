use std::rc::Rc;

use shared::{
    components::Table,
    software::SoftwareTool,
};
use yew::prelude::*;

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
fn get_rows() -> Vec<SoftwareTool>
{
    let starship = SoftwareTool {
        name:        "Starship".to_string(),
        short_desc:  "A nice modern terminal prompt".to_string(),
        web_link:    "https://starship.rs".to_string(),
        long_desc:   String::new(),
        image_links: Vec::new(),
    };
    let hyperfine = SoftwareTool {
        name:        "Hyperfine".to_string(),
        short_desc:  "A benchmarking tool written in rust".to_string(),
        web_link:    "https://github.com/sharkdp/hyperfine".to_string(),
        long_desc:   String::new(),
        image_links: Vec::new(),
    };
    let nushell = SoftwareTool {
        name:        "Nushell".to_string(),
        short_desc:  "A new way of doing shells".to_string(),
        web_link:    "https://www.nushell.sh".to_string(),
        long_desc:   String::new(),
        image_links: Vec::new(),
    };

    vec![starship, hyperfine, nushell]
}
