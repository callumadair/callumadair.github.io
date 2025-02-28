use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SwapStyle
{
    Flip,
    Rotate,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, derive_more::Display)]
#[display("swap {swap_style}")]
pub struct SwapClasses
{
    pub swap_style: SwapStyle,
}

impl From<SwapProps> for SwapClasses
{
    fn from(value: SwapProps) -> Self
    {
        Self {
            swap_style: value.swap_style,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SwapProps
{
    pub on:         Element,
    pub off:        Element,
    pub swap_style: SwapStyle,
}

#[component]
pub fn Swap(props: SwapProps) -> Element
{
    let class = SwapClasses::from(props.clone()).to_string();
    let SwapProps { on, off, .. } = props;

    rsx! {
       label {
            {class},

            input {
                type: "checkbox"
            }

            div {
                class:"swap-on",
                {on}
            }

            div {
                class:"swap-off",
                {off}
            }
       }
    }
}
