mod button;
mod carousel;
mod footer;
mod input;
mod modal;
mod navigation;
mod table;

pub use button::*;
pub use carousel::*;
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

#[derive(Clone, Copy, Debug, Eq, Default, derive_more::Display, PartialEq, AsRefStr)]
#[strum(serialize_all = "kebab_case")]
pub enum Colour
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

#[derive(Clone, Copy, Debug, Eq, Default, derive_more::Display, PartialEq, AsRefStr)]
#[strum(serialize_all = "kebab_case")]
pub enum Size
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
}
