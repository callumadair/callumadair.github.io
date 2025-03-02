use dioxus::prelude::*;
use shared::components::Carousel;

#[component]
pub fn ReadingBase() -> Element
{
    rsx! {
        div {
            class: "flex items-center justify-center text-center",
            CurrentReading {}
        }
    }
}

#[component]
fn CurrentReading() -> Element
{
    let img_paths = vec![
        asset!("./assets/networking.jpg"),
        asset!("./assets/linux.jpg"),
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
