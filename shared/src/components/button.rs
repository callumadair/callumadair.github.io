use strum::{
    AsRefStr,
    Display,
};
use yew::prelude::*;

use crate::components::Size;

crate::component_colours!(ButtonColour, "btn-");
crate::component_sizes!(ButtonSize, "btn-");

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "btn-", serialize_all = "kebab-case")]
pub enum ButtonBehaviour
{
    #[default]
    Active,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "btn-", serialize_all = "kebab-case")]
pub enum ButtonStyle
{
    Dash,
    Ghost,
    Link,
    #[default]
    None,
    Outline,
    Soft,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "btn-", serialize_all = "kebab-case")]
pub enum ButtonModifier
{
    Block,
    Circle,
    #[default]
    None,
    Square,
    Wide,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, derive_more::Display)]
#[display("btn {behaviour} {colour} {modifier} {size} {style}")]
struct ButtonClasses
{
    behaviour: ButtonBehaviour,
    colour:    ButtonColour,
    modifier:  ButtonModifier,
    size:      ButtonSize,
    style:     ButtonStyle,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps
{
    #[prop_or_default]
    pub behaviour: ButtonBehaviour,
    #[prop_or_default]
    pub children:  Children,
    #[prop_or_default]
    pub colour:    ButtonColour,
    #[prop_or_default]
    pub modifier:  ButtonModifier,
    #[prop_or_default]
    pub style:     ButtonStyle,
    #[prop_or_default]
    pub size:      ButtonSize,
}
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html
{
    let ButtonProps {
        behaviour,
        colour,
        style,
        size,
        modifier,
        children,
    } = props.clone();

    let class = ButtonClasses {
        behaviour,
        colour,
        modifier,
        size,
        style,
    }
    .to_string();

    gloo::console::log!(&class);
    html! {
        <button {class}>
            {children}
        </button>
    }
}
