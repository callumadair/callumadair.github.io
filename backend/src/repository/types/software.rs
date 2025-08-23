use entity::software::types::{
    SoftwareLongDescription,
    SoftwareName,
    SoftwareShortDescription,
    SoftwareWebLink,
};
use shared::software::SoftwareTool;

use crate::error::Result;

#[derive(bon::Builder)]
pub struct CreateSoftwareRequest
{
    #[builder(with = |name: &str| -> Result<_> {Ok(SoftwareName::new(name)?)})]
    name:              SoftwareName,
    #[builder(with = |short_desc: &str| -> Result<_> {Ok(SoftwareShortDescription::new(short_desc)?)})]
    short_description: SoftwareShortDescription,
    #[builder(with = |long_desc: &str| -> Result<_> {Ok(SoftwareLongDescription::new(long_desc)?)})]
    long_description:  SoftwareLongDescription,
    #[builder(with = |web_link: &str| -> Result<_> {Ok(SoftwareWebLink::new(web_link)?)})]
    web_link:          SoftwareWebLink,
}

impl TryFrom<SoftwareTool> for CreateSoftwareRequest
{
    type Error = crate::error::BackendError;

    fn try_from(value: SoftwareTool) -> Result<Self>
    {
        Ok(CreateSoftwareRequest::builder()
            .name(&value.name)?
            .short_description(&value.short_desc)?
            .long_description(&value.long_desc)?
            .web_link(&value.web_link)?
            .build())
    }
}
