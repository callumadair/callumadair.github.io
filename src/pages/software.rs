use yew::prelude::*;

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
    html! {
        {"CLI Tools I like"}
    }
}
