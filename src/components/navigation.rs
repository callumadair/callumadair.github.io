use capitalize::Capitalize;
use gloo::storage::Storage;
use lucide_yew::{
    House,
    Palette,
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

                <div class="navbar px-4 w-full bg-primary-content text-primary">

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
            class="btn btn-sm btn-square btn-ghost drawer-button"
        >
            <PanelLeft size=20 />
        </label>
    }
}

#[function_component(MenuContent)]
fn menu_content() -> Html
{
    html! {
        <div class="drawer-side z-30">

            <label for="my-drawer"
                aria-label="close sidebar"
                class="drawer-overlay"
            />

            <ul class="menu bg-base-200 text-base-content min-h-full w-80 p-4">

                <li><a>{"Totally an item"}</a></li>

                <li><a>{"Totally another item"}</a></li>

            </ul>

        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct NavbarLinkProps
{
    route:    Route,
    children: Children,
}

#[function_component(NavbarLink)]
fn navbar_link(props: &NavbarLinkProps) -> Html
{
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let NavbarLinkProps {
        route,
        children: content,
    } = props.clone();

    let onclick = Callback::from(move |_| navigator.push(&route));
    html! {
        <a
            class="btn btn-ghost"
            {onclick}
        >
            {content}
        </a>
    }
}

#[function_component(HomeLink)]
fn home() -> Html
{
    html! {
        <NavbarLink route={Route::Home} >
            <House/>
            {"Home"}
        </NavbarLink>
    }
}

#[function_component(ReadingLink)]
fn reading() -> Html
{
    html! {
        <NavbarLink route={Route::ReadingList}>
            {"Reading List"}
        </NavbarLink>
    }
}

#[function_component(ProjectLink)]
fn project() -> Html
{
    html! {
        <NavbarLink route={Route::Projects}>
            {"Projects"}
        </NavbarLink>
    }
}

#[function_component(SoftwareLink)]
fn software() -> Html
{
    html! {
        <NavbarLink route={Route::Software}>
            {"Software"}
        </NavbarLink>
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
