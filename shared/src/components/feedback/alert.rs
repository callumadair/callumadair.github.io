use dioxus::prelude::*;
use lucide_dioxus::CircleAlert;
use strum::{
    AsRefStr,
    Display,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "alert-", serialize_all = "kebab-case")]
pub enum AlertStyle
{
    Dash,
    #[default]
    None,
    Soft,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "alert-", serialize_all = "kebab-case")]
pub enum AlertColour
{
    Error,
    Info,
    #[default]
    None,
    Success,
    Warning,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "alert-", serialize_all = "kebab-case")]
pub enum AlertDirection
{
    Horizontal,
    #[default]
    None,
    Vertical,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, derive_more::Display)]
#[display("alert {style} {colour} {direction}")]
pub struct AlertClasses
{
    style:     AlertStyle,
    colour:    AlertColour,
    direction: AlertDirection,
}

impl From<AlertProps> for AlertClasses
{
    fn from(value: AlertProps) -> Self
    {
        Self {
            style:     value.style,
            colour:    value.colour,
            direction: value.direction,
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AlertProps
{
    #[props(default)]
    pub colour:    AlertColour,
    #[props(default)]
    pub direction: AlertDirection,
    pub message:   String,
    #[props(default)]
    pub style:     AlertStyle,
}
#[component]
pub fn Alert(props: AlertProps) -> Element
{
    let msg = props.message.clone();
    let class = AlertClasses::from(props).to_string();

    rsx! {
        div {
            role: "alert",
            class,

            CircleAlert {}

            span {
               {msg}
            }
        }
    }
}
