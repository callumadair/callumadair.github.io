use dioxus::prelude::*;

#[component]
pub fn ProjectBase() -> Element
{
    rsx! {
        div {
            class: "flex items-center justify-center text-center",
            {CurrentProjects}
        }
    }
}

#[component]
fn CurrentProjects() -> Element
{
    rsx! {
        h2 {
            class: "text-2xl text-nowrap",
            { "Current Projects" }
        }
    }
}
