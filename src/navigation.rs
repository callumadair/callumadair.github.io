use capitalize::Capitalize;
use gloo::storage::Storage;
use lucide_yew::Menu;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    Route,
    Theme,
    THEME_STORAGE_KEY,
};

#[function_component(Navbar)]
pub(crate) fn navbar() -> Html
{
    html! {
        <div class="navbar bg-primary-content text-primary">

            <div class="navbar-start space-x-2">
                <MenuToggle/>
                <HomeLink/>
                <ReadingLink/>
                <ProjectLink/>
            </div>

            <div class="navbar-center">
            </div>

            <div class="navbar-end space-x-2">
                <ThemeControl/>
            </div>

        </div>

    }
}

#[function_component(MenuToggle)]
fn menu() -> Html
{
    html! {
        <Menu/>
    }
}

#[function_component(HomeLink)]
fn home() -> Html
{
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <a
            class="btn btn-ghost"
            {onclick}
        >
            {"Home"}
        </a>
    }
}

#[function_component(ReadingLink)]
fn reading() -> Html
{
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let onclick = Callback::from(move |_| navigator.push(&Route::ReadingList));

    html! {
        <a
            class="btn btn-ghost"
            {onclick}
        >
            {"Reading List"}
        </a>
    }
}

#[function_component(ProjectLink)]
fn project() -> Html
{
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let onclick = Callback::from(move |_| navigator.push(&Route::Projects));

    html! {
        <a
            class="btn btn-ghost"
            {onclick}
        >
            {"Projects"}
        </a>
    }
}

// TODO make this use a list of themes I choose and also
// retain the theme value on reload (probably a use_state
// val?)
#[function_component(ThemeControl)]
fn theme() -> Html
{
    html! {
        <div class="dropdown dropdown-end">

          <div tabindex="0" role="button" class="btn btn-ghost">

            {"Theme"}

            <svg
              width="12px"
              height="12px"
              class="inline-block h-2 w-2 fill-current opacity-60"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 2048 2048"
            >
              <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"/>
            </svg>

          </div>

          <ul tabindex="0"
            class="dropdown-content max-h-80 overflow-auto bg-base-300 rounded-box z-[1] w-52 p-2 shadow-2xl"
            >

            <ThemeControlDropdownContent/>

          </ul>

        </div>
    }
}

#[function_component(ThemeControlDropdownContent)]
fn theme_content() -> Html
{
    let theme = use_context::<UseStateHandle<Theme>>().expect("Failed getting theme hook.");

    Theme::iter()
        .map(|theme_variant| {
            let onclick = {
                let theme = theme.clone();
                let theme_variant = theme_variant.clone();
                Callback::from(move |_| {
                    let theme_variant = theme_variant.clone();
                    gloo::storage::LocalStorage::set(THEME_STORAGE_KEY, theme_variant)
                        .expect("Failed updating stored theme.");
                    theme.set(theme_variant)
                })
            };

            html! {

                <li>

                  <input
                    type="radio"
                    name="theme-dropdown"
                    class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                    aria-label={ theme_variant.to_string().capitalize() }
                    value={ theme_variant.to_string() }
                    {onclick}
                    />

                </li>

            }
        })
        .collect::<Html>()
}
