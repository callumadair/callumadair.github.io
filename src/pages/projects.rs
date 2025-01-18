use yew::prelude::*;

#[function_component(ProjectBase)]
pub fn base() -> Html
{
    html! {
        <div class="flex items-center justify-center text-center">
            <CurrentProjects/>
        </div>
    }
}

#[function_component(CurrentProjects)]
fn current() -> Html
{
    html! {
        <caption class="text-2xl text-primary text-nowrap">
            { "Current Projects" }
        </caption>
    }
}
