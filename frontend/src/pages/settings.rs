use dioxus::prelude::*;

#[component]
pub fn SettingsBase() -> Element
{
    rsx! {
        div {
            class: "flex flex-col gap-5 items-start justify-start p-5",

            h2 {
                class: "text-xl",
                "Settings"
            }

            div {
                class: "flex flex-col gap-3",
                h3 {
                    class: "text-lg",
                    "Themes"
                }

                code {
                    class: "p-3 rounded-box bg-base-300",
                    "Hello, world!"
                }
            }
        }
    }
}
