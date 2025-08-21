pub struct SoftwareName(String);
pub struct SoftwareShortDescription(String);
pub struct SoftwareLongDescription(String);
pub struct SoftwareUrl(String);

pub struct CreateSoftwareRequest
{
    name:              SoftwareName,
    short_description: SoftwareShortDescription,
    long_description:  SoftwareLongDescription,
    url:               SoftwareUrl,
}
