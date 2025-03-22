use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum CountdownType
{
    #[strum(serialize = "rounded-box text-neutral-content flex flex-col p-2")]
    Boxes(usize),
    #[strum(serialize = "font-mono text-2xl")]
    Clock(usize),
    #[strum(serialize = "font-mono text-6xl")]
    Large(usize),
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "")]
    Regular(usize),
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum LabelStyle
{
    Left,
    #[default]
    None,
    Right,
    Underneath,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, derive_more::Display)]
#[display("countdown {countdown_type}")]
struct CountdownClasses
{
    countdown_type: CountdownType,
}

impl From<CountdownProps> for CountdownClasses
{
    fn from(value: CountdownProps) -> Self
    {
        Self {
            countdown_type: value.countdown_type,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CountdownProps
{
    pub countdown_type: CountdownType,
    pub delimiter:      Option<String>,
    pub label_style:    Option<LabelStyle>,
}

#[component]
pub fn Countdown(props: CountdownProps) -> Element
{
    let class = CountdownClasses::from(props.clone()).to_string();
    rsx! {
       span {
        class,
            span {

            }
        }
    }
}
