pub(crate) mod pages;

pub struct Error
{
    inner: reqwest::Error,
}
