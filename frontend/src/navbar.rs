use dioxus::prelude::*;
use gloo::storage::Storage;
use lucide_dioxus::{
    House,
    Palette,
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
        // This is just to enable the menu sidebar.
        div {
            class: "drawer",

            input {
                id: "my-menu",
                type: "checkbox",
                class:"drawer-toggle",
            },

            // Sidebar stuff goes here
            MenuContent {
                menu_id: "my-menu",
                li {
                    a {
                        "Totally a link"
                    }
                },
                li {
                    a {
                        "Totally another link"
                    }
                }
            },

            // Actual navbar stuff goes here.
            div {
                class: "drawer-content flex flex-col",

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
                        ThemeControl {}
                    }

                }

            }

        }

    }
}

#[component]
fn HomeLink() -> Element
{
    rsx! {
            NavbarLink<Route> {
                route: Route::Home,
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
            route: Route::ReadingList,
            {"Reading List"}
        }
    }
}

#[component]
fn ProjectLink() -> Element
{
    rsx! {
            NavbarLink<Route>{
                route: Route::Projects,
                {"Projects"}
            }
    }
}

#[component]
fn SoftwareLink() -> Element
{
    rsx! {
        NavbarLink<Route> {
            route: Route::Software,
            {"Software"}
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
                    class: "btn btn-sm btn-circle btn-ghost",

                    Palette {
                        size: 20
                    }

                  }

              ul {
                tabindex: "0",
                class: "dropdown-content z-1 p-2 gap-y-5 w-40 max-h-80 rounded-box overflow-auto shadow-2xl",

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
