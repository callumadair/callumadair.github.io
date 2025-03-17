use dioxus::prelude::*;

use crate::components::{
    BackgroundColour,
    CollapseModifier,
    PeerBackgroundColour,
    PeerTextColour,
    TextColour,
};

#[derive(Clone, Copy, Default, PartialEq, Eq, derive_more::Display)]
#[display("collapse bg-base-100 border border-base-300 {open_state} {symbol}")]
struct AccordionClasses
{
    open_state: CollapseModifier,
    symbol:     CollapseModifier,
}

impl From<AccordionItemProps> for AccordionClasses
{
    fn from(value: AccordionItemProps) -> Self
    {
        Self {
            open_state: value.open_state.unwrap_or_default(),
            symbol:     value.symbol.unwrap_or_default(),
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionItemProps
{
    name:                      String,
    checked_background_colour: Option<PeerBackgroundColour>,
    checked_text_colour:       Option<PeerTextColour>,
    children:                  Element,
    initial_background_colour: Option<BackgroundColour>,
    initial_text_colour:       Option<TextColour>,
    open_state:                Option<CollapseModifier>,
    symbol:                    Option<CollapseModifier>,
    title:                     String,
}

#[component]
pub fn AccordionItem(props: AccordionItemProps) -> Element
{
    let class = AccordionClasses::from(props.clone()).to_string();

    let AccordionItemProps {
        name,
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
                type: "radio",
                name,
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
