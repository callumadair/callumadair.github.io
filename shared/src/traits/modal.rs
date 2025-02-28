use dioxus::prelude::*;

pub trait ModalDisplay
{
    fn display(&self) -> Element;
}
