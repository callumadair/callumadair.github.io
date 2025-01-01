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
use strum::{
    AsRefStr,
    Display,
    EnumIter,
};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    error::pages::{
        Forbidden,
        IAmTeapot,
        NotFound,
        Unauthorized,
        UnavailableForLegalReasons,
        UnsupportedMediaType,
    },
    navigation::Navbar,
    pages::{
        projects::ProjectBase,
        reading::ReadingBase,
        software::SoftwareBase,
    },
};

mod components;
mod error;
mod navigation;
mod pages;
mod macros;

const THEME_ATTRIBUTE_NAME: &str = "data-theme";
const THEME_STORAGE_KEY: &str = "current-theme-name";

#[derive(
    Default, PartialEq, Eq, Clone, Copy, Display, AsRefStr, EnumIter, Serialize, Deserialize,
)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum Theme
{
    #[default]
    Dark,
    Light,
    Cupcake,
    Bumblebee,
    Emerald,
    Corporate,
    Synthwave,
    Retro,
    Cyberpunk,
    Valentine,
    Halloween,
    Garden,
    Forest,
    Aqua,
    Lofi,
    Pastel,
    Fantasy,
    Wireframe,
    Black,
    Luxury,
    Dracula,
    Cmyk,
    Autumn,
    Business,
    Acid,
    Lemonade,
    Night,
    Coffee,
    Winter,
    Dim,
    Nord,
    Sunset,
}

#[derive(Clone, Routable, PartialEq)]
enum Route
{
    #[at("/")]
    Home,
    #[at("/401")]
    Unauthorized,
    #[at("/403")]
    Forbidden,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/418")]
    ImATeapot,
    #[at("/415")]
    UnsupportedMediaType,
    #[at("/451")]
    UnavailableForLegalReasons,
    #[at("/reading")]
    ReadingList,
    #[at("/projects")]
    Projects,
    #[at("/software")]
    Software,
}

#[function_component(App)]
pub fn app() -> Html
{
    let theme = LocalStorage::get::<Theme>(THEME_STORAGE_KEY).unwrap_or(Theme::default());
    let theme = use_state_eq(|| theme);

    document()
        .document_element()
        .expect("Failed getting root document as element.")
        .set_attribute(THEME_ATTRIBUTE_NAME, (*theme).as_ref())
        .expect("Failed setting the theme value.");

    html! {
        <ContextProvider<UseStateHandle<Theme>> context={theme}>
            <BrowserRouter>
                <Navbar/>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<UseStateHandle<Theme>>>
    }
}

fn switch(routes: Route) -> Html
{
    match routes
    {
        Route::Home => html! { <Home/> },
        Route::Unauthorized => html! { <Unauthorized/> },
        Route::Forbidden => html! { <Forbidden/> },
        Route::NotFound => html! { <NotFound/> },
        Route::ImATeapot => html! { <IAmTeapot/> },
        Route::UnsupportedMediaType => html! { <UnsupportedMediaType/> },
        Route::UnavailableForLegalReasons => html! { <UnavailableForLegalReasons/> },
        Route::ReadingList => html! { <ReadingBase/> },
        Route::Projects => html! { <ProjectBase/> },
        Route::Software =>
        {
            html! { <SoftwareBase/> }
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html
{
    html! {
        <div class="h-screen flex justify-center items-center">
            <h1 class="text-primary text-4xl">
                {"Hello, world!"}
            </h1>
        </div>
    }
}
