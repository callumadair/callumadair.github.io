#[derive(Debug, thiserror::Error)]
pub enum Error
{
    #[error("Error creating new instance of type: {0}")]
    InstantiationError(#[from] InstantiationError),
}

#[derive(Debug, thiserror::Error)]
pub enum InstantiationError
{
    #[error("Image path cannot be empty")]
    ImagePathEmpty,
    #[error("Software name cannot be empty")]
    SoftwareNameEmpty,
    #[error("Software short description cannot be empty")]
    SoftwareShortDescriptionEmpty,
    #[error("Software long description cannot be empty")]
    SoftwareLongDescriptionEmpty,
    #[error("Software web link cannot be empty")]
    SoftwareWebLinkEmpty,
}
