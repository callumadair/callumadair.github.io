use crate::error::pages::{
    Forbidden, IAmTeapot, NotFound, Unauthorized, UnavailableForLegalReasons, UnsupportedMediaType,
};
use yew::prelude::*;
use yew_router::prelude::*;

mod error;

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
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h1 class="text-primary">
                {"Hello, world!"}
            </h1>
        </>
    }
}
