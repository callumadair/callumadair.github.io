use strum::{
    AsRefStr,
    Display,
};
use yew::prelude::*;

crate::component_colours!(ButtonColour, "btn-");
crate::component_sizes!(ButtonSize, "btn-");

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "btn-", serialize_all = "kebab-case")]
pub enum ButtonBehaviour
{
    #[default]
    Active,
    Disabled,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
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

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
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

#[derive(Clone, Copy, Default, PartialEq, Eq, derive_more::Display)]
#[display("btn {behaviour} {colour} {modifier} {size} {style}")]
pub struct ButtonClasses
{
    behaviour: ButtonBehaviour,
    colour:    ButtonColour,
    modifier:  ButtonModifier,
    size:      ButtonSize,
    style:     ButtonStyle,
}

impl From<ButtonProps> for ButtonClasses
{
    fn from(value: ButtonProps) -> Self
    {
        Self {
            behaviour: value.behaviour,
            colour:    value.colour,
            modifier:  value.modifier,
            size:      value.size,
            style:     value.style,
        }
    }
}

#[derive(Properties, PartialEq, Clone, Default)]
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
    let btn_classes: ButtonClasses = props.clone().into();
    let class = btn_classes.to_string();

    gloo::console::log!(&class);
    html! {
        <button {class}>
            {props.children.clone()}
        </button>
    }
}
