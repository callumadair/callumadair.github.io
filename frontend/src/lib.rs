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
    #[at("/")]
    Home,
    #[at("/403")]
    Forbidden,
    #[at("/418")]
    ImATeapot,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/projects")]
    Projects,
    #[at("/reading")]
    ReadingList,
    #[at("/software")]
    Software,
    #[at("/401")]
    Unauthorized,
    #[at("/451")]
    UnavailableForLegalReasons,
    #[at("/415")]
    UnsupportedMediaType,
}

#[function_component(App)]
pub fn app() -> Html
{
    let theme = LocalStorage::get::<Theme>(THEME_STORAGE_KEY).unwrap_or(Theme::default());
    let theme = use_state_eq(|| theme);

    document()
        .document_element()
        .expect("Failed getting root document as element.")
        .set_attribute(THEME_ATTRIBUTE_NAME, &(*theme).to_string().to_lowercase())
        .expect("Failed setting the theme value.");

    html! {
        <ContextProvider<UseStateHandle<Theme>> context={theme}>
            <BrowserRouter>
                <Page>
                    <main class="grow">
                        <Navbar/>

                        <Switch<Route> render={switch} />
                    </main>

                    <Footer/>
                </Page>
            </BrowserRouter>
        </ContextProvider<UseStateHandle<Theme>>>
    }
}

#[derive(Properties, PartialEq)]
struct PageProps
{
    children: Children,
}

#[function_component(Page)]
fn page(props: &PageProps) -> Html
{
    html! {
        <div class="flex flex-col bg-base-100 min-h-screen justify-between">
            {props.children.clone()}
        </div>
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
        <div class="flex justify-center items-center">
            <h1 class="text-primary text-4xl">
                {"Hello, world!"}
            </h1>
        </div>
    }
}
