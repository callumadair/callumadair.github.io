use dioxus::prelude::*;
use lucide_dioxus::{
    ChevronLeft,
    ChevronRight,
};

#[derive(Props, PartialEq)]
pub struct CarouselProps
{
    #[props(default)]
    pub img_paths: Vec<String>,
    #[props(default)]
    pub class:     String,
}

#[component]
pub fn CarouselView(props: &CarouselProps) -> Element
{
    let carousel_contents = props
        .img_paths
        .clone()
        .iter()
        .enumerate()
        .map(|(idx, path)|
            rsx! {
                div {
                    id: {"slide{idx}"},
                    class:"carousel-item relative w-full",

                    img {
                        src: path,
                        class: "w-full"
                    },

                    div {
                        class: "absolute left-5 right-5 top-1/2 flex translate-y-1/2 transform justify-between",

                        a {
                            href: {format!("#slide{}", idx.saturating_sub(1))},
                            class: "btn btn-circle",
                            {ChevronLeft},
                        },

                        a {
                            href: {format!("#slide{}", idx + 1)},
                            class: "btn btn-circle",
                            {ChevronRight},
                        }
                    }
                }
            }
        )
        .collect::<Vec<Element>>();

    let mut class = props.class.clone();
    class.push_str(" carousel");

    rsx! {
        div {
            class,
            {"Carousel"}
            {carousel_contents}
        }
    }
}
