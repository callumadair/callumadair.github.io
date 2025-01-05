pub(crate) trait Contains
{
    fn contains(
        &self,
        key: &str,
    ) -> bool;
}
