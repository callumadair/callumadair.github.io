use dioxus::prelude::*;
use gloo::storage::Storage;
use lucide_dioxus::{
    House,
    Palette,
    SlidersHorizontal,
};
use shared::components::{
    MenuContent,
    MenuToggle,
    NavbarLink,
};
use strum::IntoEnumIterator;

use crate::{
    Route,
    Theme,
    THEME_STORAGE_KEY,
};

#[component]
pub fn Navbar() -> Element
{
    rsx! {
        div {
            class: "navbar px-4 w-full bg-base-100",
            div {
                class: "navbar-start space-x-2",

                MenuToggle {
                    menu_id: "my-menu"
                }

                div {
                    class: "divider divider-accent divider-horizontal"
                }

                HomeLink {}
                ReadingLink {}
                ProjectLink {}
                SoftwareLink {}
            }

            div {
                class: "navbar-end space-x-2",
                SettingsLink {}
                ThemeControl {}
            }
        }
    }
}

#[component]
fn HomeLink() -> Element
{
    rsx! {
            NavbarLink<Route> {
                route: Route::Home {},
                House {}
                {"Home"}
            }
    }
}

#[component]
fn ReadingLink() -> Element
{
    rsx! {
        NavbarLink<Route> {
            route: Route::ReadingBase {},
            {"Reading List"}
        }
    }
}

#[component]
fn ProjectLink() -> Element
{
    rsx! {
            NavbarLink<Route>{
                route: Route::ProjectBase {},
                {"Projects"}
            }
    }
}

#[component]
fn SoftwareLink() -> Element
{
    rsx! {
        NavbarLink<Route> {
            route: Route::SoftwareBase {},
            {"Software"}
        }
    }
}

#[component]
fn SettingsLink() -> Element
{
    rsx! {
        Link {
            to: Route::SettingsBase {},
            button {
                class: "btn btn-square btn-ghost",
                SlidersHorizontal {
                    size: 20
                }
            }
        }
    }
}

// TODO make this use a list of themes I choose and also
// retain the theme value on reload (probably a use_state
// val?)
#[component]
fn ThemeControl() -> Element
{
    rsx! {
            div {
                class:"dropdown dropdown-end",

                div {
                    tabindex: "0",
                    role: "button",
                    class: "btn btn-square btn-ghost",

                    Palette {
                        size: 20
                    }

                  }

              ul {
                tabindex: "0",
                class: "dropdown-content bg-base-200 z-1 p-2 gap-y-5 w-40 max-h-80 rounded-box overflow-auto shadow-2xl",

                ThemeControlDropdownContent {}

              }

            }
    }
}

#[component]
fn ThemeControlDropdownContent() -> Element
{
    let mut theme = use_context::<Signal<Theme>>();

    rsx! {
        for theme_variant in Theme::iter() {


            li {
                input {
                    type: "radio",
                    name: "theme-dropdown",
                    class: "theme-controller btn btn-sm btn-block btn-ghost justify-start",
                    aria_label: theme_variant.to_string() ,
                    value: theme_variant.to_string().to_lowercase() ,
                    onclick:
                        move |_evt: MouseEvent| {
                            gloo::storage::LocalStorage::set(THEME_STORAGE_KEY, theme_variant)
                            .expect("Failed updating stored theme.");
                            theme.set(theme_variant);
                    },
                }
            }
       }

    }
}
