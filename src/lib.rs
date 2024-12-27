use crate::{
    error::pages::{
        Forbidden, IAmTeapot, NotFound, Unauthorized, UnavailableForLegalReasons,
        UnsupportedMediaType,
    },
    pages::reading::ReadingBase,
};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod error;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
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
    Reading,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Unauthorized => html! { <Unauthorized/> },
        Route::Forbidden => html! { <Forbidden/> },
        Route::NotFound => html! { <NotFound/> },
        Route::ImATeapot => html! { <IAmTeapot/> },
        Route::UnsupportedMediaType => html! { <UnsupportedMediaType/> },
        Route::UnavailableForLegalReasons => html! { <UnavailableForLegalReasons/> },
        Route::Reading => html! { <ReadingBase/> },
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="h-screen flex justify-center items-center">
            <h1 class="text-info text-4xl">
                {"Hello, world!"}
            </h1>
        </div>
    }
}
