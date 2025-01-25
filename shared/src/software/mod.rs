use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct SoftwareTool
{
    pub name:        String,
    pub short_desc:  String,
    pub long_desc:   String,
    pub web_link:    String,
    pub image_links: Vec<String>,
}
