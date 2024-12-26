use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
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
        Route::Home => {
            html! {
                <Home/>
            }
        }
        Route::NotFound => {
            html! {
                <NotFound/>
            }
        }
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

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="h-screen flex justify-center items-center">
            <h1 class="text-2xl text-error">
                {"404 | Page not found"}
            </h1>
        </div>
    }
}
