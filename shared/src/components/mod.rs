mod actions;
mod carousel;
mod feedback;
mod footer;
mod input;
mod modal;
mod navigation;
mod table;

pub use actions::{
    button::*,
    dropdown::*,
    swap::*,
    theme_controller::*,
};
pub use carousel::*;
use derive_more::Display;
pub use feedback::alert::*;
pub use footer::*;
pub use input::*;
pub use modal::*;
pub use navigation::*;
use strum::AsRefStr;
pub use table::*;

#[macro_export]
macro_rules! component_colours {
    ($name:ident, $strum_prefix:literal) => {
        #[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
        #[strum(prefix = $strum_prefix, serialize_all = "kebab_case")]
        pub enum $name
        {
            Accent,
            AccentContent,
            Base100,
            Base200,
            Base300,
            BaseContent,
            Error,
            ErrorContent,
            Info,
            InfoContent,
            Neutral,
            NeutralContent,
            #[default]
            None,
            Primary,
            PrimaryContent,
            Secondary,
            SecondaryContent,
            Success,
            SuccessContent,
            Warning,
            WarningContent,
        }
    };
}

component_colours!(BackgroundColours, "bg-");
component_colours!(BorderColours, "border-");
component_colours!(TextColours, "text-");

#[macro_export]
macro_rules! component_sizes {
    ($name:ident, $strum_prefix:literal) => {
        #[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
        #[strum(prefix = $strum_prefix, serialize_all = "kebab_case")]
        pub enum $name
        {
            #[strum(serialize = "xs")]
            ExtraSmall,
            #[strum(serialize = "sm")]
            Small,
            #[default]
            #[strum(serialize = "md")]
            Medium,
            #[strum(serialize = "lg")]
            Large,
            #[strum(serialize = "xl")]
            ExtraLarge,
            #[strum(serialize = "2xl")]
            ExtraExtraLarge,
            #[strum(serialize = "3xl")]
            ExtraExtraExtraLarge,
        }
    };
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
#[strum(prefix = "border", serialize_all = "kebab_case")]
pub enum BorderRadius
{
    #[strum(serialize = "xs")]
    ExtraSmall,
    #[strum(serialize = "sm")]
    Small,
    #[default]
    #[strum(serialize = "md")]
    Medium,
    #[strum(serialize = "lg")]
    Large,
    #[strum(serialize = "xl")]
    ExtraLarge,
    #[strum(serialize = "2xl")]
    ExtraExtraLarge,
    #[strum(serialize = "3xl")]
    ExtraExtraExtraLarge,
    #[strum(serialize = "box")]
    Box,
    #[strum(serialize = "field")]
    Field,
    #[strum(serialize = "selector")]
    Selector,
}
