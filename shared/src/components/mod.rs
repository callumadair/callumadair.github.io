mod button;
mod carousel;
mod footer;
mod input;
mod modal;
mod navigation;
mod table;

pub use carousel::*;
pub use footer::*;
pub use input::*;
pub use modal::*;
pub use navigation::*;
pub use table::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Colour
{
    #[default]
    Primary,
    PrimaryContent,
    Secondary,
    SecondaryContent,
    Accent,
    AccentContent,
    Neutral,
    NeutralContent,
    Base100,
    Base200,
    Base300,
    BaseContent,
    Info,
    InfoContent,
    Success,
    SuccessContent,
    Warning,
    WarningContent,
    Error,
    ErrorContent,
}
