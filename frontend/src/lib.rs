mod api;
use std::collections::HashMap;

use dioxus::prelude::*;
use gloo::{
    storage::{
        LocalStorage,
        Storage,
    },
    utils::document,
};
use reqwest::Client;
use serde::{
    Deserialize,
    Serialize,
};
use shared::components::{
    Footer,
    MenuContent,
};
use strum::{
    AsRefStr,
    Display,
    EnumIter,
};

use crate::{
    api::{
        clients::{
            reqwest::ReqwestClient,
            traits::{
                ApiClient,
                ApiUrl,
                ApiUrlMap,
            },
        },
        services::{
            traits::ApiService,
            types::Service,
        },
    },
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
        settings::SettingsBase,
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
    #[default]
    Light,
    Dark,
}

#[derive(Clone, Copy, Routable, PartialEq)]
#[rustfmt::skip]
enum Route
{
    #[layout(Page)]
        #[route("/")]
        Home {},
        #[route("/projects")]
        ProjectBase {},
        #[route("/reading")]
        ReadingBase {},
        #[route("/software")]
        SoftwareBase {},
        #[route("/settings")]
        SettingsBase {},
    #[end_layout]
    #[route("/403")]
    Forbidden {},
    #[route("/418")]
    IAmTeapot {},
    #[route("/401")]
    Unauthorized {},
    #[route("/451")]
    UnavailableForLegalReasons {},
    #[route("/415")]
    UnsupportedMediaType {},
    #[route("/404")]
    NotFound {},
}

#[derive(Clone)]
struct AppState<S>
where
    S: ApiService,
{
    service: S,
}

impl<S> AppState<S>
where
    S: ApiService,
{
    pub fn new(service: S) -> Self { Self { service } }

    pub fn service(&self) -> &S { &self.service }
}

#[component]
pub fn App() -> Element
{
    use_context_provider(|| {
        let api_url_map = HashMap::from([(
            ApiUrl::SoftwareIndex,
            "http://localhost.:8080/software/index".into(),
        )]);

        AppState::new(Service::new(ReqwestClient::new(
            Client::new(),
            ApiUrlMap::new(api_url_map),
        )))
    });
    let theme_value = LocalStorage::get::<Theme>(THEME_STORAGE_KEY).unwrap_or(Theme::default());
    let theme_signal = use_signal(|| theme_value);
    let theme = use_context_provider(|| theme_signal);

    match document().document_element()
    {
        Some(document_element) =>
        {
            match document_element
                .set_attribute(THEME_ATTRIBUTE_NAME, &theme().to_string().to_lowercase())
            {
                Ok(_) => gloo::console::log!("Theme successfully changed."),
                Err(value) => gloo::console::log!(value),
            }

            rsx! {
                Router::<Route> {}

                document::Stylesheet {
                    href: asset!("assets/out.css")
                }
            }
        }
        None => rsx!(),
    }
}

#[component]
pub fn Page() -> Element
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
                    class: "flex flex-col bg-base-100 h-screen justify-between",

                    Navbar {}

                    main {
                        class: "grow",

                        // This is where our page content will be displayed.
                        Outlet::<Route> {}
                    }

                    Footer {
                    }
                }


            }

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
