use strum::{
    AsRefStr,
    Display,
};
use yew::prelude::*;

crate::component_colours!(ButtonColour, "btn-");

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "btn-", serialize_all = "kebab-case")]
pub enum Behaviour
{
    Active,
    Disabled,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "btn-", serialize_all = "kebab-case")]
pub enum ButtonStyle
{
    Outline,
    Soft,
    Ghost,
    Link,
    #[default]
    None,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps
{
    #[prop_or_default]
    pub colour:   ButtonColour,
    #[prop_or_default]
    pub style:    ButtonStyle,
    pub children: Children,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, derive_more::Display)]
#[display("btn {colour} {style}")]
struct ButtonClasses
{
    colour: ButtonColour,
    style:  ButtonStyle,
}
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html
{
    let ButtonProps {
        colour,
        style,
        children,
    } = props.clone();
    let class = ButtonClasses {
        colour: ButtonColour::Accent,
        style,
    }
    .to_string();
    gloo::console::log!(&class);
    html! {
        <button {class}>
            {children}
        </button>
    }
}
