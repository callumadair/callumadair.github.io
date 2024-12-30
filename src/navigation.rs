use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub(crate) fn navbar() -> Html {
    html! {
        <div class="navbar p-3 bg-primary-content">

            <div class="navbar-start space-x-2">
                <HomeButton/>
                <ReadingButton/>
                <ProjectButton/>
            </div>

            <div class="navbar-center">
            </div>

            <div class="navbar-end space-x-2">
                <ThemeControl/>
            </div>

        </div>

    }
}

#[function_component(HomeButton)]
fn home() -> Html {
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
fn reading() -> Html {
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
fn project() -> Html {
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let onclick = Callback::from(move |_| navigator.push(&Route::Projects));

    html! {
        <button
            class="btn btn-primary"
            {onclick}
        >
            {"Projects"}
        </button>
    }
}

//TODO make this use a list of themes I choose
#[function_component(ThemeControl)]
fn theme() -> Html {
    html! {
        <div class="dropdown dropdown-end">

          <div tabindex="0" role="button" class="btn btn-primary">

            {"Theme"}

            <svg
              width="12px"
              height="12px"
              class="inline-block h-2 w-2 fill-current opacity-60"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 2048 2048">
              <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
            </svg>

          </div>

          <ul tabindex="0" class="dropdown-content bg-base-300 rounded-box z-[1] w-52 p-2 shadow-2xl">
            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Default"
                value="default" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Corporate"
                value="corporate" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Business"
                value="business" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Autumn"
                value="autumn" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Dracula"
                value="dracula" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Lofi"
                value="lofi" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Retro"
                value="retro" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Cyberpunk"
                value="cyberpunk" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Valentine"
                value="valentine" />
            </li>

            <li>
              <input
                type="radio"
                name="theme-dropdown"
                class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                aria-label="Aqua"
                value="aqua" />
            </li>

          </ul>

        </div>
    }
}
