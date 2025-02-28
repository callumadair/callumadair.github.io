use chrono::{
    Datelike,
    Utc,
};
use dioxus::prelude::*;

#[component]
pub fn footer() -> Element
{
    let cur_year = Utc::now().year();
    rsx! {
        footer {
            class: "footer footer-center bg-base-300 text-base-content p-4",
            aside {
                    p {
                        {format!("Copyright © 2024 - {cur_year}. All rights reserved by Callum Adair." )}
                    }
            }

        }
    }
}
