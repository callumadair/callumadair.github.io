use dioxus::prelude::*;

#[component]
pub fn SettingsBase() -> Element
{
    rsx! {
        div {
            class: "flex items-start justify-start",
        }
    }
}
