use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

use crate::components::{
    BackgroundColour,
    TextColour,
};

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
#[display("collapse bg-base-100 border border-base-300 {open_state} {symbol}")]
struct CollapseClasses
{
    open_state: CollapseModifier,
    symbol:     CollapseModifier,
}

impl From<CollapseProps> for CollapseClasses
{
    fn from(value: CollapseProps) -> Self
    {
        Self {
            open_state: value.open_state.unwrap_or_default(),
            symbol:     value.symbol.unwrap_or_default(),
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CollapseProps
{
    checked_background_colour: Option<BackgroundColour>,
    checked_text_colour:       Option<TextColour>,
    initial_background_colour: Option<BackgroundColour>,
    initial_text_colour:       Option<TextColour>,
    children:                  Element,
    open_state:                Option<CollapseModifier>,
    symbol:                    Option<CollapseModifier>,
    title:                     String,
}

#[component]
pub fn Collapse(props: CollapseProps) -> Element
{
    let class = CollapseClasses::from(props.clone()).to_string();

    let CollapseProps {
        checked_background_colour,
        checked_text_colour,
        initial_background_colour,
        initial_text_colour,
        children,
        title,
        ..
    } = props;

    let title_class = format!(
        "collapse-title {} {} peer-checked:{} peer-checked:{}",
        initial_background_colour.unwrap_or_default(),
        initial_text_colour.unwrap_or_default(),
        checked_background_colour.unwrap_or_default(),
        checked_text_colour.unwrap_or_default(),
    );

    let content_class = format!(
        "collapse-content {} {} peer-checked:{} peer-checked:{}",
        initial_background_colour.unwrap_or_default(),
        initial_text_colour.unwrap_or_default(),
        checked_background_colour.unwrap_or_default(),
        checked_text_colour.unwrap_or_default(),
    );

    rsx! {
        div {
            class,
            input {
                type: "checkbox",
                class: "peer"
            }

            div {
                class: title_class,
                {title}
            }

            div {
                class: content_class,
                {children}
            }
        }
    }
}
