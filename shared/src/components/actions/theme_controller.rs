use dioxus::{
    dioxus_core::DynamicNode,
    prelude::*,
};

use crate::components::Dropdown;

// TODO add in logic for actually changing and storing theme
// state.
#[derive(Clone, PartialEq)]
pub enum Toggle
{
    ExternalIcons(Element, String, Element, String),
    InternalIcons(Element, String, Element, String),
    Text(String, String),
    NoDisplay(String, String),
}

impl IntoDynNode for Toggle
{
    fn into_dyn_node(self) -> DynamicNode
    {
        let node = match self
        {
            Toggle::ExternalIcons(left_icon_html, _left_theme, right_icon_html, right_theme) =>
            {
                rsx! {
                    label {
                        class: "flex cursor-pointer gap-2",
                        {left_icon_html.clone()},

                        input {
                            type: "checkbox",
                            value: right_theme.clone(),
                            class: "toggle theme-controller"
                        },

                        {right_icon_html.clone()},
                    }
                }
            }

            Toggle::InternalIcons(left_icon_html, _left_theme, right_icon_html, right_theme) =>
            {
                rsx! {
                    label {
                        class: "toggle text-base-content",

                        input { type: "checkbox",
                            value: right_theme.clone(),
                            class:"theme-controller",
                        },

                        {left_icon_html.clone()},

                        {right_icon_html.clone()}
                    }
                }
            }
            Toggle::Text(left_theme, right_theme) =>
            {
                rsx! {
                    label {class : "flex cursor-pointer gap-2" ,
                        span { class :"label-text" ,
                                {left_theme.clone()}
                        },

                        input { type : "checkbox",
                            value : right_theme.clone(),
                            class : "toggle theme-controller",
                        }

                        span {
                            class : "label-text" ,
                            { right_theme.clone() },
                        }
                    }
                }
            }
            Toggle::NoDisplay(_left_theme, right_theme) =>
            {
                rsx! {
                    input {
                        type : "checkbox",
                        value :right_theme.clone(),
                        class : "toggle theme-controller",
                    }
                }
            }
        };
        node.into_dyn_node()
    }
}

#[derive(Clone, PartialEq)]
pub enum Radio
{
    Dropdown(Vec<String>),
    Input(Vec<String>),
    Standard(Vec<String>),
}

impl IntoDynNode for Radio
{
    fn into_dyn_node(self) -> DynamicNode
    {
        let res = match self
        {
            Radio::Dropdown(options) =>
            {
                rsx! {
                 Dropdown {}
                }
            }

            Radio::Input(options) =>
            {
                rsx! {
                    fieldset {
                        class: "fieldset",
                        for option in options.iter() {
                            label {
                                class: "flex gap-2 cursor-pointer items-center",
                                input {
                                    type: "radio",
                                    name: "theme-radios",
                                    class: "radio radio-sm theme-controller",
                                    value: option.clone(),
                                }
                            }
                        }
                    }
                }
            }

            Radio::Standard(options) =>
            {
                rsx! {
                    div {
                        class: "join join-vertical",
                        for option in options.iter() {
                            input {
                                type: "radio",
                                name: "theme-buttons",
                                class: "btn theme-controller join-item",
                                value: option.clone(),
                                aria_label: option.clone(),
                            }
                        }
                    }
                }
            }
        };
        res.into_dyn_node()
    }
}
// TODO impl IntoDynNode on this enum.
#[derive(Clone, PartialEq)]
pub enum Type
{
    CheckBox(String, String),
    Toggle(Toggle),
    Radio(Radio),
    Swap(String, String),
}

#[derive(Props, Clone, PartialEq)]
pub struct ThemeControllerProps
{
    pub controller_type: Type,
}

#[component]
pub fn ThemeController(props: ThemeControllerProps) -> Element
{
    // TODO actually use this.
    // props.controller_type.clone().to_html()
    rsx! {}
}
