use shared::software::SoftwareTool;
use yew::prelude::*;

use crate::traits::modal::ModalDisplay;

pub trait ToHtmlWrapper
{
    fn html(&self) -> Html;
}

impl<T> ToHtmlWrapper for Vec<T>
where
    T: ToHtmlWrapper,
{
    fn html(&self) -> Html { self.iter().map(ToHtmlWrapper::html).collect::<Html>() }
}
impl ToHtmlWrapper for SoftwareTool
{
    fn html(&self) -> Html
    {
        html! {
            <tr>

                <td>
                    {self.name.clone()}
                </td>

                <td>
                    {self.short_desc.clone()}
                </td>

                <td>
                    <a target="_blank" href={self.web_link.clone()}>
                        {"Website"}
                    </a>
                </td>

                <td>
                    {self.display()}
                </td>


            </tr>
        }
    }
}
