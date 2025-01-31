use yew::Html;

pub trait ModalDisplay
{
    fn display(&self) -> Html;
}
