use yew::prelude::*;

use crate::components::carousel::Carousel;

#[function_component(ReadingBase)]
pub fn base() -> Html
{
    html! {
        <div class="flex items-center justify-center text-center">
            <CurrentReading/>
        </div>
    }
}

#[function_component(CurrentReading)]
fn current() -> Html
{
    let img_paths = vec![
        AttrValue::from("assets/networking.jpg"),
        AttrValue::from("assets/linux.jpg"),
    ];
    html! {
        <>
            <caption class="text-2xl text-primary text-nowrap">
                { "Current reading" }
            </caption>
            <Carousel {img_paths} class="w-80"/>
        </>
    }
}
