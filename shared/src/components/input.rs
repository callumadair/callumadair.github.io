use dioxus::prelude::*;
use lucide_dioxus::ScanSearch;

#[component]
pub fn area() -> Element
{
    rsx! {}
}

#[component]
pub fn file() -> Element
{
    rsx! {}
}

#[derive(Props, PartialEq, Clone)]
pub struct SearchInputProps
{
    pub id:      String,
    pub onkeyup: Callback<KeyboardEvent>,
}

#[component]
pub fn SearchInput(props: SearchInputProps) -> Element
{
    let SearchInputProps { id, onkeyup } = props;

    rsx! {
        label {
            class: "input input-bordered input-sm flex items-center gap-2 max-w-48",
            input {
                type: "text",
                class: "grow",
                placeholder: "Search",
                id,
                onkeyup,
            },

            ScanSearch {
                size: 20
            }

        }
    }
}

#[component]
pub fn text() -> Element
{
    rsx! {}
}
