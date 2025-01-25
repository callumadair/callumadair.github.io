use serde::{
    Deserialize,
    Serialize,
};

use crate::traits::contains::Contains;
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct SoftwareTool
{
    pub name:        String,
    pub short_desc:  String,
    pub long_desc:   String,
    pub web_link:    String,
    pub image_links: Vec<String>,
}

impl Contains for SoftwareTool
{
    fn contains(
        &self,
        key: &str,
    ) -> bool
    {
        self.short_desc.contains(key)
            || self.long_desc.contains(key)
            || self.web_link.contains(key)
            || self.name.contains(key)
    }
}
