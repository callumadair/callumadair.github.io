use dioxus::prelude::*;
use strum::{
    AsRefStr,
    Display,
};

use crate::components::{
    BorderRadius,
    RingColour,
    Width,
};

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum AvatarModifier
{
    #[strum(serialize = "")]
    #[default]
    None,
    #[strum(serialize = "avatar-placeholder")]
    Placeholder,
}

#[derive(Clone, Copy, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum AvatarPresence
{
    #[strum(serialize = "")]
    #[default]
    None,
    #[strum(serialize = "avatar-online")]
    Online,
    #[strum(serialize = "avatar-offline")]
    Offline,
}

#[derive(Clone, Default, PartialEq, Eq, derive_more::Display)]
#[display("{border_radius} {online_status} {placeholder} {ring_colour} {width}")]
pub struct AvatarClasses
{
    pub border_radius: BorderRadius,
    pub online_status: AvatarPresence,
    pub placeholder:   AvatarModifier,
    pub ring_colour:   RingColour,
    pub width:         Width,
}

impl From<AvatarProps> for AvatarClasses
{
    fn from(value: AvatarProps) -> Self
    {
        Self {
            border_radius: value.border_radius.unwrap_or_default(),
            online_status: value.avatar_presence.unwrap_or_default(),
            placeholder:   value.placeholder.unwrap_or_default(),
            ring_colour:   value.ring_colour.unwrap_or_default(),
            width:         value.width.unwrap_or_default(),
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AvatarProps
{
    pub avatar_presence: Option<AvatarPresence>,
    pub border_radius:   Option<BorderRadius>,
    pub img_src:         Option<String>,
    pub name:            String,
    pub placeholder:     Option<AvatarModifier>,
    pub ring_colour:     Option<RingColour>,
    pub width:           Option<Width>,
}
#[component]
pub fn Avatar(props: AvatarProps) -> Element
{
    let class = AvatarClasses::from(props.clone()).to_string();
    let AvatarProps { img_src, name, .. } = props;

    rsx! {
        div {
            class: "avatar",
            div {
                class,
                img {
                    src: img_src,
                    alt: "Avatar image for {name}"
                }
            }
        }
    }
}