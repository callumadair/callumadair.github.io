use lucide_yew::{
    ChevronLeft,
    ChevronRight,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CarouselProps
{
    #[prop_or_default]
    pub img_paths: Vec<AttrValue>,
    #[prop_or_default]
    pub class:     Classes,
}

#[function_component(Carousel)]
pub fn carousel_view(props: &CarouselProps) -> Html
{
    let carousel_contents = props
        .img_paths
        .clone()
        .iter()
        .enumerate()
        .map(|(idx, path)|
            html! {
                <div id={format!("slide{}",idx)} class="carousel-item relative w-full">
                    <img src={path} class="w-full"/>

                    <div class="absolute left-5 right-5 top-1/2 flex translate-y-1/2 transform justify-between">

                        <a href={format!("#slide{}", idx.saturating_sub(1))} class="btn btn-circle">
                            <ChevronLeft/>
                        </a>

                        <a href={format!("#slide{}", idx + 1)} class="btn btn-circle">
                            <ChevronRight/>
                        </a>

                    </div>
                </div>
            }
        )
        .collect::<Html>();

    let mut class = props.class.clone();
    class.push(classes!("carousel"));

    html! {
        <div {class}>
            {"Carousel"}
            {carousel_contents}
        </div>
    }
}
