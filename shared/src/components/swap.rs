use strum::{
    AsRefStr,
    Display,
};
use yew::prelude::*;

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

#[derive(Properties, Clone, PartialEq)]
pub struct SwapProps
{
    pub on:         Html,
    pub off:        Html,
    pub swap_style: SwapStyle,
}

#[function_component(Swap)]
pub fn swap(props: &SwapProps) -> Html
{
    let SwapProps { on, off, .. } = props.clone();
    let class = SwapClasses::from(props.clone()).to_string();

    html! {
       <label {class}>
            <input type="checkbox" />

            <div class="swap-on">
                {on}
            </div>

            <div class="swap-off">
                {off}
            </div>
       </label>
    }
}
