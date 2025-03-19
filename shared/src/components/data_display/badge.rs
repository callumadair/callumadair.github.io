use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum BadgeStyle
{
    #[strum(serialize = "badge-dash")]
    Dash,
    #[strum(serialize = "badge-ghost")]
    Ghost,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "badge-outline")]
    Outline,
    #[strum(serialize = "soft")]
    Soft,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum BadgeColour
{
    #[strum(serialize = "badge-accent")]
    Accent,
    #[strum(serialize = "badge-error")]
    Error,
    #[strum(serialize = "badge-info")]
    Info,
    #[strum(serialize = "badge-neutral")]
    Neutral,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "badge-primary")]
    Primary,
    #[strum(serialize = "badge-secondary")]
    Secondary,
    #[strum(serialize = "badge-success")]
    Success,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum BadgeSize
{
    #[strum(serialize = "badge-xs")]
    ExtraSmall,
    #[strum(serialize = "badge-sm")]
    Small,
    #[default]
    #[strum(serialize = "badge-md")]
    Medium,
    #[strum(serialize = "badge-lg")]
    Large,
    #[strum(serialize = "badge-xl")]
    ExtraLarge,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, derive_more::Display)]
#[display("badge {style} {colour} {size}")]
struct BadgeClasses
{
    style:  BadgeStyle,
    colour: BadgeColour,
    size:   BadgeSize,
}

impl From<BadgeProps> for BadgeClasses
{
    fn from(value: BadgeProps) -> Self
    {
        Self {
            style:  value.badge_style.unwrap_or_default(),
            colour: value.badge_colour.unwrap_or_default(),
            size:   value.badge_size.unwrap_or_default(),
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps
{
    pub badge_style:  Option<BadgeStyle>,
    pub badge_colour: Option<BadgeColour>,
    pub badge_size:   Option<BadgeSize>,
    pub content:      Option<String>,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element
{
    let class = BadgeClasses::from(props.clone()).to_string();
    rsx! {
        div {
            class,
            {props.content.unwrap_or_default()}
        }
    }
}
