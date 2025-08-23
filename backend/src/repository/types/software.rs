use entity::{
    image::types::ImageURL,
    software::types::{
        SoftwareLongDescription,
        SoftwareName,
        SoftwareShortDescription,
        SoftwareWebLink,
    },
};
use shared::software::SoftwareTool;

use crate::error::Result;

#[derive(bon::Builder)]
pub struct CreateSoftwareRequest
{
    #[builder(with = |name: &str| -> Result<_> {Ok(SoftwareName::new(name)?)})]
    name:       SoftwareName,
    #[builder(with = |short_desc: &str| -> Result<_> {Ok(SoftwareShortDescription::new(short_desc)?)})]
    short_desc: SoftwareShortDescription,
    #[builder(with = |long_desc: &str| -> Result<_> {Ok(SoftwareLongDescription::new(long_desc)?)})]
    long_desc:  SoftwareLongDescription,
    #[builder(with = |web_link: &str| -> Result<_> {Ok(SoftwareWebLink::new(web_link)?)})]
    web_link:   SoftwareWebLink,
    image_urls: Vec<ImageURL>,
}

impl CreateSoftwareRequest
{
    pub fn name(&self) -> &SoftwareName { &self.name }

    pub fn short_desc(&self) -> &SoftwareShortDescription { &self.short_desc }

    pub fn long_desc(&self) -> &SoftwareLongDescription { &self.long_desc }

    pub fn web_link(&self) -> &SoftwareWebLink { &self.web_link }

    pub fn image_urls(&self) -> &Vec<ImageURL> { &self.image_urls }
}

impl TryFrom<SoftwareTool> for CreateSoftwareRequest
{
    type Error = crate::error::BackendError;

    fn try_from(value: SoftwareTool) -> Result<Self>
    {
        Ok(CreateSoftwareRequest::builder()
            .name(&value.name)?
            .short_desc(&value.short_desc)?
            .long_desc(&value.long_desc)?
            .web_link(&value.web_link)?
            .image_urls(
                value
                    .image_links
                    .iter()
                    // TODO (CA): Kill this unwrap
                    .filter_map(|link| {
                        let image_url_res = ImageURL::new(link);
                        image_url_res.ok()
                    })
                    .collect::<Vec<ImageURL>>(),
            )
            .build())
    }
}
