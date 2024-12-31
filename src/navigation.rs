use capitalize::Capitalize;
use gloo::storage::Storage;
use lucide_yew::{
    ChevronDown,
    House,
    PanelLeft,
};
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
        // This is just to enable the menu sidebar.
        <div class="drawer">

            <input id="my-drawer"
                type="checkbox"
                class="drawer-toggle"
            />

            // Actual navbar stuff goes here.
            <div class="drawer-content flex flex-col">

                <div class="navbar w-full bg-primary-content text-primary">

                    <div class="navbar-start space-x-2">
                        <MenuToggle/>
                        <ReadingLink/>
                        <ProjectLink/>
                        <SoftwareLink/>
                    </div>

                    <div class="navbar-center">
                        <HomeLink/>
                    </div>

                    <div class="navbar-end space-x-2">
                        <ThemeControl/>
                    </div>

                </div>

            </div>

            // Sidebar stuff goes here
            <MenuContent/>

        </div>

    }
}

#[function_component(MenuToggle)]
fn menu_toggle() -> Html
{
    html! {
        <label
            for="my-drawer"
            class="btn btn-ghost drawer-button"
        >
            <PanelLeft size=20 />
        </label>
    }
}

#[function_component(MenuContent)]
fn menu_content() -> Html
{
    html! {
        <div class="drawer-side">

            <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"/>

            <ul class="menu bg-base-200 text-base-content min-h-full w-80 p-4">

                <li><a>{"Totally an item"}</a></li>

                <li><a>{"Totally another item"}</a></li>

            </ul>

        </div>
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
            <House/>
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

#[function_component(SoftwareLink)]
fn software() -> Html
{
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let onclick = Callback::from(move |_| navigator.push(&Route::Software));

    html! {
        <a
            class="btn btn-ghost"
            {onclick}
        >
        {"Software"}
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

            <ChevronDown/>

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
