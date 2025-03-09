use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum ButtonBehaviour
{
    #[default]
    #[strum(serialize = "btn-active")]
    Active,
    #[strum(serialize = "btn-disabled")]
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum ButtonColour
{
    #[strum(serialize = "btn-accent")]
    Accent,
    #[strum(serialize = "btn-accent-content")]
    AccentContent,
    #[strum(serialize = "btn-base-100")]
    Base100,
    #[strum(serialize = "btn-base-200")]
    Base200,
    #[strum(serialize = "btn-base-300")]
    Base300,
    #[strum(serialize = "btn-base-content")]
    BaseContent,
    #[strum(serialize = "btn-error")]
    Error,
    #[strum(serialize = "btn-error-content")]
    ErrorContent,
    #[strum(serialize = "btn-info")]
    Info,
    #[strum(serialize = "btn-info-content")]
    InfoContent,
    #[strum(serialize = "btn-neutral")]
    Neutral,
    #[strum(serialize = "btn-neutral-content")]
    NeutralContent,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "btn-primary")]
    Primary,
    #[strum(serialize = "btn-primary-content")]
    PrimaryContent,
    #[strum(serialize = "btn-secondary")]
    Secondary,
    #[strum(serialize = "btn-secondary-content")]
    SecondaryContent,
    #[strum(serialize = "btn-success")]
    Success,
    #[strum(serialize = "btn-success-content")]
    SuccessContent,
    #[strum(serialize = "btn-warning")]
    Warning,
    #[strum(serialize = "btn-warning-content")]
    WarningContent,
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

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum ButtonSize
{
    #[strum(serialize = "btn-xs")]
    ExtraSmall,
    #[strum(serialize = "btn-sm")]
    Small,
    #[default]
    #[strum(serialize = "btn-md")]
    Medium,
    #[strum(serialize = "btn-lg")]
    Large,
    #[strum(serialize = "btn-xl")]
    ExtraLarge,
    #[strum(serialize = "btn-2xl")]
    ExtraExtraLarge,
    #[strum(serialize = "btn-3xl")]
    ExtraExtraExtraLarge,
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
