use actix_web::http::StatusCode;
use entity::{
    image::ImageURL,
    software::SoftwareName,
};
use shared::impl_nested_error;

use crate::impl_nested_status_code;

#[derive(thiserror::Error, Debug)]
pub enum DomainModelError
{
    #[error("Model creation error: {0}")]
    CreateModelError(#[from] CreateModelError),
}
impl_nested_error!(
    DomainModelError,
    CreateModelError,
    CreateImageError,
    CreateSoftwareError
);
impl_nested_status_code!(DomainModelError, CreateModelError);

#[derive(thiserror::Error, Debug)]
pub enum CreateModelError
{
    #[error("{0}")]
    CreateImageError(#[from] CreateImageError),
    #[error("{0}")]
    CreateSoftwareError(#[from] CreateSoftwareError),
}
impl_nested_status_code!(CreateModelError, CreateImageError, CreateSoftwareError);

#[derive(thiserror::Error, Debug)]
pub enum CreateImageError
{
    #[error("Image entry with url: {url} already exists.")]
    Duplicate
    {
        url: ImageURL
    },
}
impl actix_web::ResponseError for CreateImageError
{
    fn status_code(&self) -> actix_web::http::StatusCode
    {
        match self
        {
            Self::Duplicate { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateSoftwareError
{
    #[error("Software entry with url: {name} already exists.")]
    Duplicate
    {
        name: SoftwareName
    },
}

impl actix_web::ResponseError for CreateSoftwareError
{
    fn status_code(&self) -> actix_web::http::StatusCode
    {
        match self
        {
            Self::Duplicate { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
