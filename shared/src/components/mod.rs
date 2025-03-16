mod actions;
mod carousel;
mod data_display;
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
pub use data_display::{
    accordion::*,
    collapse::*,
};
pub use feedback::alert::*;
pub use footer::*;
pub use input::*;
pub use modal::*;
pub use navigation::*;
use strum::{
    AsRefStr,
    Display,
};
pub use table::*;

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum BackgroundColour
{
    #[strum(serialize = "bg-accent")]
    Accent,
    #[strum(serialize = "bg-accent-content")]
    AccentContent,
    #[strum(serialize = "bg-base-100")]
    Base100,
    #[strum(serialize = "bg-base-200")]
    Base200,
    #[strum(serialize = "bg-base-300")]
    Base300,
    #[strum(serialize = "bg-base-content")]
    BaseContent,
    #[strum(serialize = "bg-error")]
    Error,
    #[strum(serialize = "bg-error-content")]
    ErrorContent,
    #[strum(serialize = "bg-info")]
    Info,
    #[strum(serialize = "bg-info-content")]
    InfoContent,
    #[strum(serialize = "bg-neutral")]
    Neutral,
    #[strum(serialize = "bg-neutral-content")]
    NeutralContent,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "bg-primary")]
    Primary,
    #[strum(serialize = "bg-primary-content")]
    PrimaryContent,
    #[strum(serialize = "bg-secondary")]
    Secondary,
    #[strum(serialize = "bg-secondary-content")]
    SecondaryContent,
    #[strum(serialize = "bg-success")]
    Success,
    #[strum(serialize = "bg-success-content")]
    SuccessContent,
    #[strum(serialize = "bg-warning")]
    Warning,
    #[strum(serialize = "bg-warning-content")]
    WarningContent,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum BorderColour
{
    #[strum(serialize = "border-accent")]
    Accent,
    #[strum(serialize = "border-accent-content")]
    AccentContent,
    #[strum(serialize = "border-base-100")]
    Base100,
    #[strum(serialize = "border-base-200")]
    Base200,
    #[strum(serialize = "border-base-300")]
    Base300,
    #[strum(serialize = "border-base-content")]
    BaseContent,
    #[strum(serialize = "border-error")]
    Error,
    #[strum(serialize = "border-error-content")]
    ErrorContent,
    #[strum(serialize = "border-info")]
    Info,
    #[strum(serialize = "border-info-content")]
    InfoContent,
    #[strum(serialize = "border-neutral")]
    Neutral,
    #[strum(serialize = "border-neutral-content")]
    NeutralContent,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "border-primary")]
    Primary,
    #[strum(serialize = "border-primary-content")]
    PrimaryContent,
    #[strum(serialize = "border-secondary")]
    Secondary,
    #[strum(serialize = "border-secondary-content")]
    SecondaryContent,
    #[strum(serialize = "border-success")]
    Success,
    #[strum(serialize = "border-success-content")]
    SuccessContent,
    #[strum(serialize = "border-warning")]
    Warning,
    #[strum(serialize = "border-warning-content")]
    WarningContent,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum BorderRadius
{
    #[strum(serialize = "border-xs")]
    ExtraSmall,
    #[strum(serialize = "border-sm")]
    Small,
    #[default]
    #[strum(serialize = "border-md")]
    Medium,
    #[strum(serialize = "border-lg")]
    Large,
    #[strum(serialize = "border-xl")]
    ExtraLarge,
    #[strum(serialize = "border-2xl")]
    ExtraExtraLarge,
    #[strum(serialize = "border-3xl")]
    ExtraExtraExtraLarge,
    #[strum(serialize = "border-box")]
    Box,
    #[strum(serialize = "border-field")]
    Field,
    #[strum(serialize = "border-selector")]
    Selector,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum PeerBackgroundColour
{
    #[strum(serialize = "peer-checked:bg-accent")]
    Accent,
    #[strum(serialize = "peer-checked:bg-accent-content")]
    AccentContent,
    #[strum(serialize = "peer-checked:bg-base-100")]
    Base100,
    #[strum(serialize = "peer-checked:bg-base-200")]
    Base200,
    #[strum(serialize = "peer-checked:bg-base-300")]
    Base300,
    #[strum(serialize = "peer-checked:bg-base-content")]
    BaseContent,
    #[strum(serialize = "peer-checked:bg-error")]
    Error,
    #[strum(serialize = "peer-checked:bg-error-content")]
    ErrorContent,
    #[strum(serialize = "peer-checked:bg-info")]
    Info,
    #[strum(serialize = "peer-checked:bg-info-content")]
    InfoContent,
    #[strum(serialize = "peer-checked:bg-neutral")]
    Neutral,
    #[strum(serialize = "peer-checked:bg-neutral-content")]
    NeutralContent,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "peer-checked:bg-primary")]
    Primary,
    #[strum(serialize = "peer-checked:bg-primary-content")]
    PrimaryContent,
    #[strum(serialize = "peer-checked:bg-secondary")]
    Secondary,
    #[strum(serialize = "peer-checked:bg-secondary-content")]
    SecondaryContent,
    #[strum(serialize = "peer-checked:bg-success")]
    Success,
    #[strum(serialize = "peer-checked:bg-success-content")]
    SuccessContent,
    #[strum(serialize = "peer-checked:bg-warning")]
    Warning,
    #[strum(serialize = "peer-checked:bg-warning-content")]
    WarningContent,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum PeerTextColour
{
    #[strum(serialize = "peer-checked:text-accent")]
    Accent,
    #[strum(serialize = "peer-checked:text-accent-content")]
    AccentContent,
    #[strum(serialize = "peer-checked:text-base-100")]
    Base100,
    #[strum(serialize = "peer-checked:text-base-200")]
    Base200,
    #[strum(serialize = "peer-checked:text-base-300")]
    Base300,
    #[strum(serialize = "peer-checked:text-base-content")]
    BaseContent,
    #[strum(serialize = "peer-checked:text-error")]
    Error,
    #[strum(serialize = "peer-checked:text-error-content")]
    ErrorContent,
    #[strum(serialize = "peer-checked:text-info")]
    Info,
    #[strum(serialize = "peer-checked:text-info-content")]
    InfoContent,
    #[strum(serialize = "peer-checked:text-neutral")]
    Neutral,
    #[strum(serialize = "peer-checked:text-neutral-content")]
    NeutralContent,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "peer-checked:text-primary")]
    Primary,
    #[strum(serialize = "peer-checked:text-primary-content")]
    PrimaryContent,
    #[strum(serialize = "peer-checked:text-secondary")]
    Secondary,
    #[strum(serialize = "peer-checked:text-secondary-content")]
    SecondaryContent,
    #[strum(serialize = "peer-checked:text-success")]
    Success,
    #[strum(serialize = "peer-checked:text-success-content")]
    SuccessContent,
    #[strum(serialize = "peer-checked:text-warning")]
    Warning,
    #[strum(serialize = "peer-checked:text-warning-content")]
    WarningContent,
}

#[derive(Clone, Copy, Debug, Eq, Default, Display, PartialEq, AsRefStr)]
pub enum TextColour
{
    #[strum(serialize = "text-accent")]
    Accent,
    #[strum(serialize = "text-accent-content")]
    AccentContent,
    #[strum(serialize = "text-base-100")]
    Base100,
    #[strum(serialize = "text-base-200")]
    Base200,
    #[strum(serialize = "text-base-300")]
    Base300,
    #[strum(serialize = "text-base-content")]
    BaseContent,
    #[strum(serialize = "text-error")]
    Error,
    #[strum(serialize = "text-error-content")]
    ErrorContent,
    #[strum(serialize = "text-info")]
    Info,
    #[strum(serialize = "text-info-content")]
    InfoContent,
    #[strum(serialize = "text-neutral")]
    Neutral,
    #[strum(serialize = "text-neutral-content")]
    NeutralContent,
    #[default]
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "text-primary")]
    Primary,
    #[strum(serialize = "text-primary-content")]
    PrimaryContent,
    #[strum(serialize = "text-secondary")]
    Secondary,
    #[strum(serialize = "text-secondary-content")]
    SecondaryContent,
    #[strum(serialize = "text-success")]
    Success,
    #[strum(serialize = "text-success-content")]
    SuccessContent,
    #[strum(serialize = "text-warning")]
    Warning,
    #[strum(serialize = "text-warning-content")]
    WarningContent,
}
