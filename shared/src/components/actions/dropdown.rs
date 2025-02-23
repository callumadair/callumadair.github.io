use strum::{
    AsRefStr,
    Display,
};
use yew::prelude::*;

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

#[derive(Properties, Clone, PartialEq)]
pub struct DropdownProps
{
    #[prop_or_default]
    pub background_colours: BackgroundColours,
    #[prop_or_default]
    pub border_radius:      BorderRadius,
    #[prop_or_default]
    pub children:           Children,
    #[prop_or_default]
    pub modifier:           DropdownModifier,
    #[prop_or_default]
    pub placement:          DropdownPlacement,
    #[prop_or_default]
    pub text_colours:       TextColours,
    #[prop_or_default]
    pub button_props:       ButtonProps,
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html
{
    let class = DropdownClasses::from(props.clone()).to_string();
    let DropdownProps { button_props, .. } = props.clone();
    let ButtonProps {
        behaviour,
        children,
        colour,
        modifier,
        style,
        size,
    } = button_props;

    html! {
        <div class="dropdown mb-72">
            <Button
                {behaviour}
                {colour}
                {modifier}
                {style}
                {size}
            >
                {children}
            </Button>

            <ul {class}>
                {props.children.clone()}
            </ul>
        </div>
    }
}
