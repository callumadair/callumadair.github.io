use actix_web::http::StatusCode;
use shared::{
    impl_nested_error,
    impl_nested_status_code,
};

#[derive(Debug, thiserror::Error)]
pub enum EntityError
{
    #[error("Error creating new instance of type: {0}")]
    InstantiationError(#[from] InstantiationError),
}
impl_nested_status_code!(EntityError, InstantiationError);
impl_nested_error!(
    EntityError,
    InstantiationError,
    ImageInstantiationError,
    SoftwareInstantiationError
);

#[derive(Debug, thiserror::Error)]
pub enum InstantiationError
{
    #[error("{0}")]
    ImageInstantiationError(#[from] ImageInstantiationError),
    #[error("{0}")]
    SoftwareInstantiationError(#[from] SoftwareInstantiationError),
}
impl_nested_status_code!(
    InstantiationError,
    ImageInstantiationError,
    SoftwareInstantiationError
);

#[derive(Debug, thiserror::Error)]
pub enum ImageInstantiationError
{
    #[error("Image URL cannot be empty")]
    ImageURLEmpty,
}

impl actix_web::ResponseError for ImageInstantiationError
{
    fn status_code(&self) -> StatusCode
    {
        match self
        {
            Self::ImageURLEmpty => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SoftwareInstantiationError
{
    #[error("Software name cannot be empty")]
    SoftwareNameEmpty,
    #[error("Software short description cannot be empty")]
    SoftwareShortDescriptionEmpty,
    #[error("Software long description cannot be empty")]
    SoftwareLongDescriptionEmpty,
    #[error("Software web link cannot be empty")]
    SoftwareWebLinkEmpty,
}

impl actix_web::ResponseError for SoftwareInstantiationError
{
    fn status_code(&self) -> StatusCode
    {
        match self
        {
            Self::SoftwareNameEmpty
            | Self::SoftwareShortDescriptionEmpty
            | Self::SoftwareLongDescriptionEmpty
            | Self::SoftwareWebLinkEmpty => StatusCode::BAD_REQUEST,
        }
    }
}
