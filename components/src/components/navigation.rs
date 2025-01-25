use lucide_yew::{
    House,
    Palette,
    PanelLeft,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(MenuToggle)]
pub fn menu_toggle() -> Html
{
    html! {
        <label
            for="my-drawer"
            class="btn btn-sm btn-square btn-ghost drawer-button"
        >
            <PanelLeft size=20 />
        </label>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct MenuContentProps
{
    pub children: Children,
}

#[function_component(MenuContent)]
pub fn menu_content(props: &MenuContentProps) -> Html
{
    let MenuContentProps { children } = props.clone();

    html! {
        <div class="drawer-side z-30">

            <label for="my-drawer"
                aria-label="close sidebar"
                class="drawer-overlay"
            />

            <ul class="menu bg-base-200 text-base-content min-h-full w-80 p-4">
                {children}
            </ul>

        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NavbarLinkProps<T>
where
    T: Routable,
{
    pub route:    T,
    pub children: Children,
}

#[function_component(NavbarLink)]
pub fn navbar_link<T>(props: &NavbarLinkProps<T>) -> Html
where
    T: Routable + 'static,
{
    let navigator = use_navigator().expect("Failed getting navigator hook.");

    let NavbarLinkProps {
        route,
        children: content,
    } = props.clone();

    let onclick = Callback::from(move |_| navigator.push(&route));
    html! {
        <a
            class="btn btn-ghost"
            {onclick}
        >
            {content}
        </a>
    }
}
