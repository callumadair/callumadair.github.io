use yew::Html;

pub(crate) trait ModalDisplay
{
    fn display(&self) -> Html;
}
