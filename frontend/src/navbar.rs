use capitalize::Capitalize;
use gloo::storage::Storage;
use lucide_yew::{
    House,
    Palette,
};
use shared::components::{
    MenuContent,
    MenuToggle,
    NavbarLink,
};
use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::{
    Route,
    Theme,
    THEME_STORAGE_KEY,
};

#[function_component(Navbar)]
pub fn navbar() -> Html
{
    html! {
        // This is just to enable the menu sidebar.
        <div class="drawer">

            <input id="my-menu"
                type="checkbox"
                class="drawer-toggle"
            />

            // Sidebar stuff goes here
            <MenuContent menu_id="my-menu">
                <li><a>{"Totally a link"}</a></li>
                <li><a>{"Totally another link"}</a></li>
            </MenuContent>

            // Actual navbar stuff goes here.
            <div class="drawer-content flex flex-col">

                <div class="navbar px-4 w-full bg-primary-content text-primary">

                    <div class="navbar-start space-x-2">
                        <MenuToggle menu_id="my-menu"/>
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

        </div>

    }
}

#[function_component(HomeLink)]
fn home() -> Html
{
    html! {
        <NavbarLink<Route> route={Route::Home} >
            <House/>
            {"Home"}
        </NavbarLink<Route>>
    }
}

#[function_component(ReadingLink)]
fn reading() -> Html
{
    html! {
        <NavbarLink<Route> route={Route::ReadingList}>
            {"Reading List"}
        </NavbarLink<Route>>
    }
}

#[function_component(ProjectLink)]
fn project() -> Html
{
    html! {
        <NavbarLink<Route> route={Route::Projects}>
            {"Projects"}
        </NavbarLink<Route>>
    }
}

#[function_component(SoftwareLink)]
fn software() -> Html
{
    html! {
        <NavbarLink<Route> route={Route::Software}>
            {"Software"}
        </NavbarLink<Route>>
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


              <div tabindex="0"
                role="button"
                class="btn btn-sm btn-circle btn-ghost"
                >

                <Palette size=20/>

              </div>

          <ul tabindex="0"
            class="dropdown-content z-[1] p-2 gap-y-5 w-40 max-h-80 rounded-box overflow-auto bg-primary-content shadow-2xl"
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
                crate::clone!(theme, theme_variant);
                Callback::from(move |_| {
                    gloo::storage::LocalStorage::set(THEME_STORAGE_KEY, theme_variant)
                        .expect("Failed updating stored theme.");
                    theme.set(theme_variant);
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
