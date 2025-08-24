use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum CardStyle
{
    #[strum(serialize = "card-border")]
    Border,
    #[strum(serialize = "card-dash")]
    Dash,
    #[default]
    #[strum(serialize = "")]
    None,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum CardModifier
{
    #[strum(serialize = "image-full")]
    ImageFull,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "card-border")]
    Side,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum CardSize
{
    #[strum(serialize = "card-xs")]
    ExtraSmall,
    #[strum(serialize = "card-sm")]
    Small,
    #[default]
    #[strum(serialize = "card-md")]
    Medium,
    #[strum(serialize = "card-lg")]
    Large,
    #[strum(serialize = "card-xl")]
    ExtraLarge,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, derive_more::Display)]
#[display("card {style} {modifier} {size}")]
struct CardClasses
{
    style:    CardStyle,
    modifier: CardModifier,
    size:     CardSize,
}

impl From<CardProps> for CardClasses
{
    fn from(value: CardProps) -> Self
    {
        Self {
            style:    value.style.unwrap_or_default(),
            modifier: value.modifier.unwrap_or_default(),
            size:     value.size.unwrap_or_default(),
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CardProps
{
    pub actions:  Option<Element>,
    pub children: Option<Element>,
    pub img_src:  Option<String>,
    pub modifier: Option<CardModifier>,
    pub size:     Option<CardSize>,
    pub style:    Option<CardStyle>,
    pub title:    Option<String>,
}

#[component]
pub fn Card(props: CardProps) -> Element
{
    let class = CardClasses::from(props.clone()).to_string();

    let CardProps {
        actions,
        children,
        img_src,
        title,
        ..
    } = props;

    let card_image = match img_src
    {
        Some(src) =>
        {
            rsx! {
            figure { img {
                src,
            } }
                    }
        }
        None => rsx!(),
    };
    rsx! {
       div {
        class,
            {card_image},
            div {
                class: "card-body",

                if title.is_some() {
                    h2 {
                        class: "card-title",
                        {title}
                    }
                }

                {children}

                if actions.is_some() {
                    div {
                        class: "card-actions",
                        {actions}
                    }
                }
            }
       }
    }
}
