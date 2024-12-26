use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct BaseErrorPageProps {
    error_text: AttrValue,
}

#[function_component(BaseErrorPage)]
fn base_error_page(props: &BaseErrorPageProps) -> Html {
    html! {
        <div class="h-screen flex justify-center items-center">
            <h1 class="text-2xl text-error">
                {props.error_text.clone()}
            </h1>
        </div>
    }
}

#[function_component(Unauthorized)]
pub(crate) fn unauthorized() -> Html {
    html! {
        <BaseErrorPage error_text="401 | Unauthorized"/>
    }
}

#[function_component(Forbidden)]
pub(crate) fn forbidden() -> Html {
    html! {
        <BaseErrorPage error_text="403 | Forbidden"/>
    }
}

#[function_component(NotFound)]
pub(crate) fn not_found() -> Html {
    html! {
        <BaseErrorPage error_text="404 | Page Not Found"/>
    }
}

#[function_component(UnsupportedMediaType)]
pub(crate) fn unsupported_media_type() -> Html {
    html! {
        <BaseErrorPage error_text="415 | Unsupported Media Type"/>
    }
}

#[function_component(IAmTeapot)]
pub(crate) fn im_a_teapot() -> Html {
    html! {
        <BaseErrorPage error_text="418 | I'm a teapot"/>
    }
}

#[function_component(UnavailableForLegalReasons)]
pub(crate) fn unavailable_for_legal_reasons() -> Html {
    html! {
        <BaseErrorPage error_text="451 | Unavailable For Legal Reasons"/>
    }
}
