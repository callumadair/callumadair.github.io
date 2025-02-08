use yew::prelude::*;

pub trait ModalDisplay
{
    fn display(&self) -> Html;
}
