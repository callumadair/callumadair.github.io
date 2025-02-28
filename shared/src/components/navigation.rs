use dioxus::prelude::*;
use lucide_dioxus::PanelLeft;

#[derive(Props, Clone, PartialEq)]
pub struct MenuToggleProps
{
    pub menu_id: String,
}

#[component]
pub fn menu_toggle(props: MenuToggleProps) -> Element
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
pub fn menu_content(props: MenuContentProps) -> Element
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
    T: Routable,
{
    pub route:    T,
    pub children: Element,
}

#[component]
pub fn navbar_link<T>(props: &NavbarLinkProps<T>) -> Element
where
    T: Routable + 'static,
{
    let NavbarLinkProps {
        route,
        children: content,
    } = props;

    rsx! {
            Link {
                class:"btn btn-ghost",
                to: route,
                {content}
            }
    }
}
