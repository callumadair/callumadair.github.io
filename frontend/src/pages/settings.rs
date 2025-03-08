use dioxus::prelude::*;

#[component]
pub fn SettingsBase() -> Element
{
    rsx! {
        div {
            class: "flex items-start justify-start p-5",

            h2 {
                class: "text-xl",
                "Settings"
            }

            h3 {
                class: "text-lg",
                "Themes"
            }
        }
    }
}
