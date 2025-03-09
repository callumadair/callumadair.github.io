use dioxus::prelude::*;
use lucide_dioxus::CircleAlert;
use strum::{
    AsRefStr,
    Display,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum AlertStyle
{
    #[strum(serialize = "alert-dash")]
    Dash,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "alert-soft")]
    Soft,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum AlertColour
{
    #[strum(serialize = "alert-error")]
    Error,
    #[strum(serialize = "alert-info")]
    Info,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "alert-success")]
    Success,
    #[strum(serialize = "alert-warning")]
    Warning,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum AlertDirection
{
    #[strum(serialize = "alert-horizontal")]
    Horizontal,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "alert-vertical")]
    Vertical,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, derive_more::Display)]
#[display("alert {style} {colour} {direction} p-5")]
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
