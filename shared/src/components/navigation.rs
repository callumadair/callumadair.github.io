use dioxus::prelude::*;
use lucide_dioxus::PanelLeft;

#[derive(Props, Clone, PartialEq)]
pub struct MenuToggleProps
{
    pub menu_id: String,
}

#[component]
pub fn MenuToggle(props: MenuToggleProps) -> Element
{
    let MenuToggleProps { menu_id } = props;
    rsx! {
            label {
                for: menu_id,
                class: "btn btn-sm btn-square btn-ghost drawer-button",

                PanelLeft {
                    size: 20
                }

            }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MenuContentProps
{
    pub children: Element,
    pub menu_id:  String,
}

#[component]
pub fn MenuContent(props: MenuContentProps) -> Element
{
    let MenuContentProps { children, menu_id } = props;

    rsx! {
        div {
            class: "drawer-side z-30",

            label {
                for: menu_id,
                aria_label: "close sidebar",
                class:"drawer-overlay"
            },

            ul {
                class: "menu bg-base-300 text-base-content min-h-full w-80 p-4",
                {children}
            }

        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct NavbarLinkProps<T>
where
    T: Routable + PartialEq + Clone,
{
    pub route:    T,
    pub children: Element,
}

#[component]
pub fn NavbarLink<T>(props: NavbarLinkProps<T>) -> Element
where
    T: Routable + PartialEq + Clone + 'static,
{
    let cur_route: T = use_route();
    let NavbarLinkProps {
        route,
        children: content,
    } = props;

    let mut class = String::new();
    if cur_route == route
    {
        class.push_str("w-9/10 border border-secondary");
    }

    rsx! {
            div {
                class: "flex flex-col items-center justify-center",
                Link {
                    class: "btn btn-ghost",
                    to: route,
                    {content}
                }

                div {
                    class,
                }
            }
    }
}
