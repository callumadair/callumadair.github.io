use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

crate::component_colours!(ButtonColour, "btn-");
crate::component_sizes!(ButtonSize, "btn-");

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum ButtonBehaviour
{
    #[default]
    #[strum(serialize = "btn-active")]
    Active,
    #[strum(serialize = "btn-disabled")]
    Disabled,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum ButtonStyle
{
    #[strum(serialize = "btn-dash")]
    Dash,
    #[strum(serialize = "btn-ghost")]
    Ghost,
    #[strum(serialize = "btn-link")]
    Link,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "btn-outline")]
    Outline,
    #[strum(serialize = "btn-soft")]
    Soft,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum ButtonModifier
{
    #[strum(serialize = "btn-block")]
    Block,
    #[strum(serialize = "btn-circle")]
    Circle,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "btn-square")]
    Square,
    #[strum(serialize = "btn-wide")]
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

impl Default for ButtonProps
{
    fn default() -> Self
    {
        Self {
            behaviour: ButtonBehaviour::default(),
            children:  Ok(VNode::default()),
            colour:    ButtonColour::default(),
            modifier:  ButtonModifier::default(),
            style:     ButtonStyle::default(),
            size:      ButtonSize::default(),
        }
    }
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
            {props.children}
        }
    }
}
