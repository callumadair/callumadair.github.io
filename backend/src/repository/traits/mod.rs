use entity::software::Model as SoftwareModel;

use crate::{
    error::Result,
    repository::types::software::CreateSoftwareRequest,
};
pub trait SoftwareRepository
{
    fn create_software(
        &self,
        req: &CreateSoftwareRequest,
    ) -> Result<SoftwareModel>;
}
