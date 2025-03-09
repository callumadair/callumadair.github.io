use dioxus::prelude::*;
use shared::components::{
    Alert,
    AlertColour,
    AlertDirection,
    AlertStyle,
    Button,
    ButtonColour,
};

#[component]
pub fn ProjectBase() -> Element
{
    rsx! {
        div {
            class: "flex flex-col gap-5 p-5",
            CurrentProjects {}
        }
    }
}

#[component]
fn CurrentProjects() -> Element
{
    rsx! {
        div {
            class: "flex flex-col gap-3",

            h2 {
                class: "text-2xl text-nowrap",
                { "Current Projects" }
            }

            div {
                Alert {
                    colour: AlertColour::Success,
                    message: "This is the message",
                }

                Button {
                    colour: ButtonColour::Secondary,
                    "Button"
                }
            }
        }
    }
}
