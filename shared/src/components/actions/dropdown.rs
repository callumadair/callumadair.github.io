use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

use crate::components::{
    BackgroundColours,
    BorderRadius,
    Button,
    ButtonProps,
    TextColours,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "dropdown-", serialize_all = "kebab-case")]
pub enum DropdownPlacement
{
    #[default]
    Start,
    Center,
    End,
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "dropdown-", serialize_all = "kebab-case")]
pub enum DropdownModifier
{
    Hover,
    Open,
    #[default]
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, derive_more::Display)]
#[display(
    "dropdown-content {background_colours} {border_radius} {modifier} {placement} {text_colours}"
)]
pub struct DropdownClasses
{
    pub background_colours: BackgroundColours,
    pub border_radius:      BorderRadius,
    pub modifier:           DropdownModifier,
    pub placement:          DropdownPlacement,
    pub text_colours:       TextColours,
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
    pub background_colours: BackgroundColours,
    #[props(default)]
    pub border_radius:      BorderRadius,
    pub children:           Element,
    #[props(default)]
    pub modifier:           DropdownModifier,
    #[props(default)]
    pub placement:          DropdownPlacement,
    #[props(default)]
    pub text_colours:       TextColours,
    #[props(default)]
    pub button_props:       ButtonProps,
}

impl Default for DropdownProps
{
    fn default() -> Self
    {
        Self {
            background_colours: BackgroundColours::default(),
            border_radius:      BorderRadius::default(),
            children:           Ok(VNode::default()),
            modifier:           DropdownModifier::default(),
            placement:          DropdownPlacement::default(),
            text_colours:       TextColours::default(),
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
