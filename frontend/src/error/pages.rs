use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct BaseErrorPageProps
{
    error_text: String,
}

#[component]
fn BaseErrorPage(props: BaseErrorPageProps) -> Element
{
    rsx! {
        div {
            class:"h-screen flex justify-center items-center",

            h1 {
                class: "text-2xl text-error",
                {props.error_text.clone()}
            }
        }
    }
}

#[component]
pub(crate) fn Unauthorized() -> Element
{
    rsx! {
        BaseErrorPage {
            error_text: "401 | Unauthorized"
        }
    }
}

#[component]
pub(crate) fn Forbidden() -> Element
{
    rsx! {
        BaseErrorPage {
            error_text: "403 | Forbidden"
        }
    }
}

#[component]
pub(crate) fn NotFound() -> Element
{
    rsx! {
        BaseErrorPage {
            error_text:"404 | Page Not Found"
        }
    }
}

#[component]
pub(crate) fn UnsupportedMediaType() -> Element
{
    rsx! {
        BaseErrorPage {
            error_text: "415 | Unsupported Media Type"
        }
    }
}

#[component]
pub(crate) fn IAmTeapot() -> Element
{
    rsx! {
        BaseErrorPage {
            error_text: "418 | I'm a teapot"
        }
    }
}

#[component]
pub(crate) fn UnavailableForLegalReasons() -> Element
{
    rsx! {
        BaseErrorPage {
            error_text: "451 | Unavailable For Legal Reasons"
        }
    }
}
