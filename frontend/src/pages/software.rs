use std::rc::Rc;

use dioxus::prelude::*;
use shared::{
    components::Table,
    software::SoftwareTool,
};

use crate::{
    AppState,
    api::{
        clients::reqwest::ReqwestClient,
        services::{
            traits::ApiService,
            types::Service,
        },
    },
};

// TODO (CA): consider the need for this.
#[derive(PartialEq, Clone)]
#[allow(dead_code)]
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

#[component]
pub fn SoftwareBase() -> Element
{
    rsx! {
        div {
            class: "flex space-y-4 p-3",
            CLIBase {}
        }
    }
}

#[component]
fn CLIBase() -> Element
{
    let mut app_state: Signal<AppState<Service<ReqwestClient>>> = use_context();
    let cols = Rc::from(["Name", "Description", "Link"].map(String::from));
    // let rows =
    //     use_resource(move || async move {
    // app_state.read().service().get_software_index().await })
    //         .read()
    //         .clone()
    //         .unwrap()?;
    let rows = get_rows();
    let searchable = true;

    rsx! {
        div {
            class: "flex flex-col w-full",

            Table<SoftwareTool> {
                id: "cli-table",
                title: "CLI Tools I like",
                cols,
                rows,
                searchable
            }

        }
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
