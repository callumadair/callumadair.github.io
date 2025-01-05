use lucide_yew::Search;
use yew::prelude::*;

#[function_component(TextArea)]
pub fn area() -> Html
{
    html! {}
}

#[function_component(FileUpload)]
pub fn file() -> Html
{
    html! {}
}

#[derive(Properties, PartialEq)]
pub struct SearchInputProps
{
    pub id: AttrValue,
}

#[function_component(SearchInput)]
pub fn search(props: &SearchInputProps) -> Html
{
    let SearchInputProps { id } = props;
    html! {
        <label class="input input-bordered flex items-center">

            <input type="text"
                class="grow"
                placeholder="Search"
                value="text"
                {id}
            />

            <Search/>
        </label>
    }
}

#[function_component(TextInput)]
pub fn text() -> Html
{
    html! {}
}
