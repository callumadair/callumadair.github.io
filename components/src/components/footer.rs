use chrono::{
    Datelike,
    Utc,
};
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html
{
    let cur_year = Utc::now().year();
    html! {
        <footer class="absolute bottom-0 p-4 footer footer-center bg-base-300 text-base-content">

            <aside>
                <p>
                    {format!("Copyright © 2024 - {cur_year}. All rights reserved by Callum Adair." )}
                </p>
            </aside>

        </footer>
    }
}
