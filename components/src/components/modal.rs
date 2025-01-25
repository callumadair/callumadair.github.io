use std::fmt::Display;

use gloo::utils::document;
use shared::software::SoftwareTool;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::traits::modal::ModalDisplay;

#[derive(Properties, Clone, PartialEq)]
pub struct ModalProps<T>
where
    T: ToHtml + Clone + PartialEq + Display,
{
    /// Is this a wide view modal.
    #[prop_or_default]
    pub large:   bool,
    /// Content for the modal.
    pub content: T,
    /// ID for the modal element.
    #[prop_or_default]
    pub id:      AttrValue,
    /// The text for the title.
    #[prop_or_default]
    pub title:   AttrValue,
}

#[function_component(Modal)]
pub fn modal<T>(props: &ModalProps<T>) -> Html
where
    T: ToHtml + Clone + PartialEq + Display,
{
    let ModalProps {
        large,
        content,
        id,
        title,
    } = props.clone();

    let inner_classes = if large
    {
        "w-3/4 h-3/4 border-b-2 border-accent overflow-auto"
    }
    else
    {
        "modal-box border-b-2 border-accent overflow-auto"
    };

    html! {
        <dialog class="modal"
            {id}
        >

            <div class={inner_classes}>

                <h3 class="text-lg font-bold">
                    {title}
                </h3>

                {content}


            </div>

            <form method="dialog"
                class="modal-backdrop"
            >

                <button class="hover:cursor-default"/>

            </form>

        </dialog>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ModalButtonProps
{
    /// The id of the modal to target.
    pub modal_id:          AttrValue,
    /// The text to be displayed inside the button.
    pub modal_button_text: AttrValue,
}

#[function_component(ModalButton)]
pub fn modal_button(props: &ModalButtonProps) -> Html
{
    let ModalButtonProps {
        modal_id,
        modal_button_text,
    } = props.clone();

    let onclick = {
        shared::clone!(modal_id);
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

    html! {
        <button class="btn btn-ghost"
            {onclick}
        >

            {modal_button_text}

        </button>
    }
}

impl ModalDisplay for SoftwareTool
{
    fn display(&self) -> Html
    {
        html! {
            <>

                <ModalButton modal_id={format!("{}-modal", self.name.clone())}
                    modal_button_text="More Info"
                />

                <Modal<AttrValue>
                    id={format!("{}-modal", self.name.clone())}
                    content={format!("{} is neat.", self.name.clone())}
                />
            </>
        }
    }
}
