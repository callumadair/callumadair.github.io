use std::fmt::Display;

use dioxus::prelude::*;
use gloo::utils::document;
use web_sys::wasm_bindgen::JsCast;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps<T>
where
    T: IntoDynNode + Clone + PartialEq + Display,
{
    /// Is this a wide view modal.
    #[props(default)]
    pub large:   bool,
    /// Content for the modal.
    pub content: T,
    /// ID for the modal element.
    #[props(default)]
    pub id:      String,
    /// The text for the title.
    #[props(default)]
    pub title:   String,
}

#[component]
pub fn Modal<T>(props: ModalProps<T>) -> Element
where
    T: IntoDynNode + Clone + PartialEq + Display,
{
    let ModalProps {
        large,
        content,
        id,
        title,
    } = props;

    let inner_classes = if large
    {
        "w-3/4 h-3/4 border-b-2 overflow-auto"
    }
    else
    {
        "modal-box border-b-2 overflow-auto"
    };

    rsx! {
        dialog {
            class: "modal",
            id,

            div { class: inner_classes,

                h3 {
                    class: "text-lg font-bold",
                    title,
                }
                {content}
            }

            form {
                method: "dialog",
                class: "modal-backdrop",

                button {
                    class: "hover:cursor-default"
                }
            }

        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalButtonProps
{
    /// The id of the modal to target.
    pub modal_id:          String,
    /// The text to be displayed inside the button.
    pub modal_button_text: String,
}

#[component]
pub fn Modal_button(props: ModalButtonProps) -> Element
{
    let ModalButtonProps {
        modal_id,
        modal_button_text,
    } = props;

    let onclick = {
        crate::clone!(modal_id);
        Callback::from(move |_evt: MouseEvent| {
            let modal_element = document()
                .get_element_by_id(modal_id.as_str())
                .expect("Failed to get modal element by id")
                .dyn_into::<web_sys::HtmlDialogElement>()
                .expect("Failed to convert to HtmlDialogElement");

            modal_element
                .show_modal()
                .expect("Failed to show modal element");
        })
    };

    rsx! {
        button {
            class: "btn btn-ghost",
            onclick,
            {modal_button_text}
        }
    }
}
