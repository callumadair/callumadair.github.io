use strum::{
    AsRefStr,
    Display,
};
use yew::prelude::*;

use crate::components::{
    BackgroundColours,
    BorderRadius,
    TextColours,
};

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
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

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "dropdown-", serialize_all = "kebab-case")]
pub enum DropdownModifier
{
    Hover,
    Open,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, derive_more::Display)]
#[display("dropdown {background_colours} {border_radius} {modifier} {placement} {text_colours}")]
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

#[derive(Properties, Clone, Debug, PartialEq)]
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
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html
{
    let dropdown_classes = DropdownClasses::from(props.clone());
    let class = dropdown_classes.to_string();

    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}
