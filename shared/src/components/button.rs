use strum::AsRefStr;
use yew::prelude::*;

use crate::components::Colour;

crate::component_colours!(ButtonColour, "btn-");

#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, AsRefStr)]
#[strum(prefix = "btn", serialize_all = "kebab-case")]
pub enum Behaviour
{
    #[default]
    Active,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, Default, PartialEq, AsRefStr)]
#[strum(prefix = "btn", serialize_all = "kebab-case")]
pub enum Style
{
    Outline,
    Soft,
    Ghost,
    Link,
    #[strum(serialize = "")]
    #[default]
    None,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps
{
    pub colour:   ButtonColour,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html
{
    let ButtonProps { colour, children } = props.clone();
    let class = colour.as_ref().to_string();
    let colour_string = ButtonColour::Accent.as_ref();
    gloo::console::log!(colour_string);

    html! {
        <button {class}>
            {children}
        </button>
    }
}
