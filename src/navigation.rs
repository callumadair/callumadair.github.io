use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub(crate) fn navbar() -> Html {
    html! {
        <div class="navbar space-x-2 bg-primary-content">
            <HomeButton/>
            <ReadingButton/>
        </div>

    }
}

#[function_component(HomeButton)]
fn home_button() -> Html {
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <button
            class="btn btn-primary"
            {onclick}
        >
            {"Home"}
        </button>
    }
}

#[function_component(ReadingButton)]
fn reading_button() -> Html {
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let onclick = Callback::from(move |_| navigator.push(&Route::ReadingList));

    html! {
        <button
            class="btn btn-primary"
            {onclick}
        >
            {"Reading List"}
        </button>
    }
}

#[function_component(ProjectButton)]
fn project_button() -> Html {
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let onclick = Callback::from(move |_| navigator.push(&Route::Projects));

    html! {
        <button
            class="btn btn-primary"
            {onclick}
        >
            {"Projects"}
        </buttons>
    }
}
