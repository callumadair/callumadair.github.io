use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

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

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps
{
    #[props(default)]
    pub behaviour: ButtonBehaviour,
    pub children:  Element,
    #[props(default)]
    pub colour:    ButtonColour,
    #[props(default)]
    pub modifier:  ButtonModifier,
    #[props(default)]
    pub style:     ButtonStyle,
    #[props(default)]
    pub size:      ButtonSize,
}
#[component]
pub fn Button(props: ButtonProps) -> Element
{
    let btn_classes: ButtonClasses = props.clone().into();
    let class = btn_classes.to_string();

    gloo::console::log!(&class);
    rsx! {
        button {
            class,
            "{props.children}"
        }
    }
}
