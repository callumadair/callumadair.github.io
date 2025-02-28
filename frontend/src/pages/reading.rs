use shared::components::Carousel;
use dioxus::prelude::*;

#[component]
pub fn Reading() -> Element
{
    rsx! {
        div {
            class: "flex items-center justify-center text-center",
            {CurrentReading}
        }
    }
}

#[component]
fn CurrentReading() -> Element
{
    let img_paths = vec![
        "assets/networking.jpg".into(),
        "assets/linux.jpg".into(),
    ];
    rsx! {
        div {
            class: "flex flex-col my-3",
            h2 {
                class: "text-2xl text-nowrap",
                { "Current reading" }
            }

            Carousel {
                img_paths,
                class:"w-80"
            }
        }
    }
}
