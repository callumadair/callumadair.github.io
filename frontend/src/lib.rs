use dioxus::prelude::*;
use gloo::{
    storage::{
        LocalStorage,
        Storage,
    },
    utils::document,
};
use serde::{
    Deserialize,
    Serialize,
};
use shared::components::Footer;
use strum::{
    AsRefStr,
    Display,
    EnumIter,
};

use crate::{
    error::pages::{
        Forbidden,
        IAmTeapot,
        NotFound,
        Unauthorized,
        UnavailableForLegalReasons,
        UnsupportedMediaType,
    },
    navbar::Navbar,
    pages::{
        projects::ProjectBase,
        reading::ReadingBase,
        software::SoftwareBase,
    },
};

mod error;
mod macros;
mod navbar;
mod pages;

const THEME_ATTRIBUTE_NAME: &str = "data-theme";
const THEME_STORAGE_KEY: &str = "current-theme-name";

#[derive(
    Default, PartialEq, Eq, Clone, Copy, Display, AsRefStr, EnumIter, Serialize, Deserialize,
)]
pub(crate) enum Theme
{
    // Abyss,
    // Acid,
    // Aqua,
    // Autumn,
    // Black,
    // Bumblebee,
    // Business,
    // Caramellatte,
    // Cmyk,
    // Coffee,
    // Corporate,
    // Cupcake,
    // Cyberpunk,
    // Dark,
    // Dim,
    // Dracula,
    // Emerald,
    // Fantasy,
    // Forest,
    // Garden,
    // Halloween,
    // Lemonade,
    // Light,
    // Lofi,
    // Luxury,
    #[default]
    NewLight,
    NewDark,
    // Night,
    // Nord,
    // Pastel,
    // Retro,
    // Silk,
    // Sunset,
    // Synthwave,
    // Valentine,
    // Winter,
    // Wireframe,
}

#[derive(Clone, Copy, Routable, PartialEq)]
enum Route
{
    #[layout(Navbar)]
    #[route("/", Home)]
    Home,
    #[route("/403", Forbidden)]
    Forbidden,
    #[route("/418", IAmTeapot)]
    ImATeapot,
    #[route("/projects", ProjectBase)]
    Projects,
    #[route("/reading", ReadingBase)]
    ReadingList,
    #[route("/software", SoftwareBase)]
    Software,
    #[route("/401", Unauthorized)]
    Unauthorized,
    #[route("/451", UnavailableForLegalReasons)]
    UnavailableForLegalReasons,
    #[route("/415", UnsupportedMediaType)]
    UnsupportedMediaType,
    #[end_layout]
    #[route("/404")]
    NotFound,
}

#[component]
pub fn App() -> Element
{
    let theme_value = LocalStorage::get::<Theme>(THEME_STORAGE_KEY).unwrap_or(Theme::default());
    let theme_signal = use_signal(|| theme_value);
    let theme = use_context_provider(|| theme_signal);

    document()
        .document_element()
        .expect("Failed getting root document as element.")
        .set_attribute(THEME_ATTRIBUTE_NAME, &theme().to_string().to_lowercase())
        .expect("Failed setting the theme value.");

    rsx! {
        Router::<Route> {}

        Page {
            main {
                class: "grow",
            }
            Footer {
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct PageProps
{
    children: Element,
}

#[component]
fn Page(props: PageProps) -> Element
{
    rsx! {
        div {
            class: "flex flex-col bg-base-100 min-h-screen justify-between",
            {props.children.clone()}
        }
    }
}

#[component]
pub fn Home() -> Element
{
    rsx! {
            div {
                class: "flex justify-center items-center",

                h1 {
                    class: "text-4xl",
                    {"Hello, world!"}
                }
            }
    }
}
