use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

use crate::components::{
    actions::button::{
        Button,
        ButtonProps,
    },
    BackgroundColour,
    BorderRadius,
    TextColour,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum DropdownPlacement
{
    #[default]
    #[strum(serialize = "dropdown-start")]
    Start,
    #[strum(serialize = "dropdown-center")]
    Center,
    #[strum(serialize = "dropdown-end")]
    End,
    #[strum(serialize = "dropdown-top")]
    Top,
    #[strum(serialize = "dropdown-bottom")]
    Bottom,
    #[strum(serialize = "dropdown-left")]
    Left,
    #[strum(serialize = "dropdown-right")]
    Right,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum DropdownModifier
{
    #[strum(serialize = "dropdown-hover")]
    Hover,
    #[strum(serialize = "dropdown-open")]
    Open,
    #[default]
    #[strum(serialize = "")]
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, derive_more::Display)]
#[display(
    "dropdown-content {background_colours} {border_radius} {modifier} {placement} {text_colours}"
)]
pub struct DropdownClasses
{
    pub background_colours: BackgroundColour,
    pub border_radius:      BorderRadius,
    pub modifier:           DropdownModifier,
    pub placement:          DropdownPlacement,
    pub text_colours:       TextColour,
}

impl From<DropdownProps> for DropdownClasses
{
    fn from(value: DropdownProps) -> Self
    {
        Self {
            background_colours: value.background_colours,
            border_radius:      value.border_radius,
            modifier:           value.modifier,
            placement:          value.placement,
            text_colours:       value.text_colours,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps
{
    #[props(default)]
    pub background_colours: BackgroundColour,
    #[props(default)]
    pub border_radius:      BorderRadius,
    pub children:           Element,
    #[props(default)]
    pub modifier:           DropdownModifier,
    #[props(default)]
    pub placement:          DropdownPlacement,
    #[props(default)]
    pub text_colours:       TextColour,
    #[props(default)]
    pub button_props:       ButtonProps,
}

impl Default for DropdownProps
{
    fn default() -> Self
    {
        Self {
            background_colours: BackgroundColour::default(),
            border_radius:      BorderRadius::default(),
            children:           Ok(VNode::default()),
            modifier:           DropdownModifier::default(),
            placement:          DropdownPlacement::default(),
            text_colours:       TextColour::default(),
            button_props:       ButtonProps::default(),
        }
    }
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element
{
    let class = DropdownClasses::from(props.clone()).to_string();
    let DropdownProps { button_props, .. } = props;
    let ButtonProps {
        behaviour,
        children,
        colour,
        modifier,
        style,
        size,
    } = button_props;

    rsx! {
            div {
                class: "dropdown mb-72",
                Button {
                    behaviour,
                    colour,
                    modifier,
                    style,
                    size,
                    children
                },

                ul { class,
                    tabindex: "0",
                    {props.children}
                }
            }
    }
}
