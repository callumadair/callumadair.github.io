use std::rc::Rc;

use yew::prelude::*;

use crate::components::table::Table;

#[derive(PartialEq)]
pub(crate) struct CLIToolsRow
{
    pub(crate) name:        String,
    pub(crate) description: String,
    pub(crate) link:        String,
}

impl ToHtml for CLIToolsRow
{
    fn to_html(&self) -> Html
    {
        html! {
            <tr>

                <td>
                    {self.name.clone()}
                </td>

                <td>
                    {self.description.clone()}
                </td>

                <td>

                    <a href={self.link.clone()}
                        target="_blank"
                        rel="noreferrer noopener"
                    >

                        {"More info"}

                    </a>

                </td>


            </tr>
        }
    }
}

#[function_component(SoftwareBase)]
pub fn base() -> Html
{
    html! {
        <div class="flex items-center justify-center text-center">
            <CLIBase/>
        </div>
    }
}

#[function_component(CLIBase)]
fn cli_tools() -> Html
{
    let cols = Rc::from(["Name", "Description", "Link"].map(AttrValue::from));

    let rows = get_rows();

    html! {
        <div class="flex flex-col text-primary">

            <Table<CLIToolsRow> caption="CLI Tools I like"
                {cols}
                {rows}
            />

        </div>
    }
}

// TODO replace this with an HTTP GET to the backend whenever I get round to making it if ever.
fn get_rows() -> Vec<CLIToolsRow>
{
    let starship = CLIToolsRow {
        name:        "Starship".to_string(),
        description: "A nice modern terminal prompt".to_string(),
        link:        "https://starship.rs".to_string(),
    };
    let hyperfine = CLIToolsRow {
        name:        "Hyperfine".to_string(),
        description: "A benchmarking tool written in rust".to_string(),
        link:        "https://github.com/sharkdp/hyperfine".to_string(),
    };
    let nushell = CLIToolsRow {
        name:        "Nushell".to_string(),
        description: "A new way of doing shells".to_string(),
        link:        "https://www.nushell.sh".to_string(),
    };

    vec![starship, hyperfine, nushell]
}
