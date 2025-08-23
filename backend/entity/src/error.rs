use actix_web::http::StatusCode;
use shared::impl_nested_status_code;

#[derive(Debug, thiserror::Error)]
pub enum EntityError
{
    #[error("Error creating new instance of type: {0}")]
    InstantiationError(#[from] InstantiationError),
}
impl_nested_status_code!(EntityError, InstantiationError);

#[derive(Debug, thiserror::Error)]
pub enum InstantiationError
{
    #[error("Image URL cannot be empty")]
    ImageURLEmpty,
    #[error("Software name cannot be empty")]
    SoftwareNameEmpty,
    #[error("Software short description cannot be empty")]
    SoftwareShortDescriptionEmpty,
    #[error("Software long description cannot be empty")]
    SoftwareLongDescriptionEmpty,
    #[error("Software web link cannot be empty")]
    SoftwareWebLinkEmpty,
}

impl actix_web::ResponseError for InstantiationError
{
    fn status_code(&self) -> actix_web::http::StatusCode
    {
        match self
        {
            Self::ImageURLEmpty
            | Self::SoftwareNameEmpty
            | Self::SoftwareShortDescriptionEmpty
            | Self::SoftwareLongDescriptionEmpty
            | Self::SoftwareWebLinkEmpty => StatusCode::BAD_REQUEST,
        }
    }
}
