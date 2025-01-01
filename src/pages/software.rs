use std::rc::Rc;

use yew::prelude::*;

use crate::components::table::Table;

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

    html! {
        <div class="flex flex-col text-primary">

            <Table caption="CLI Tools I like" {cols}/>

        </div>
    }
}
