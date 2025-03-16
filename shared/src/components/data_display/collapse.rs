use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

use crate::components::BackgroundColour;

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum CollapseModifier
{
    #[strum(serialize = "collapse-arrow")]
    Arrow,
    #[strum(serialize = "collapse-close")]
    Close,
    #[strum(serialize = "collapse-open")]
    Open,
    #[strum(serialize = "collapse-plus")]
    Plus,
    #[default]
    #[strum(serialize = "")]
    None,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, derive_more::Display)]
#[display("collapse {background_colour} {open_state} {symbol}")]
struct CollapseClasses
{
    background_colour: BackgroundColour,
    open_state:        CollapseModifier,
    symbol:            CollapseModifier,
}

impl From<CollapseProps> for CollapseClasses
{
    fn from(value: CollapseProps) -> Self
    {
        Self {
            background_colour: value.background_colour.unwrap_or_default(),
            open_state:        value.open_state.unwrap_or_default(),
            symbol:            value.symbol.unwrap_or_default(),
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CollapseProps
{
    background_colour: Option<BackgroundColour>,
    children:          Element,
    open_state:        Option<CollapseModifier>,
    symbol:            Option<CollapseModifier>,
    title:             String,
}

#[component]
pub fn Collapse(props: CollapseProps) -> Element
{
    let outer_classes = CollapseClasses::from(props.clone()).to_string();

    rsx! {
        div {
            class: outer_classes,
            input {
                type: "checkbox",
                class: "peer"
            }

            div {
                class: "collapse-title",
                {props.title}
            }

            div {
                class: "collapse-content",
                {props.children}
            }
        }
    }
}
