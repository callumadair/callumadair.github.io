use std::rc::Rc;

use web_sys::{
    wasm_bindgen::JsValue,
    HtmlInputElement,
};
use yew::prelude::*;

use crate::{
    clone,
    components::input::SearchInput,
    traits::contains::Contains,
};

#[derive(Properties, PartialEq, Clone)]
pub struct TableProps<T>
where
    T: PartialEq + ToHtml + Clone + Contains,
{
    pub id:         AttrValue,
    #[prop_or_default]
    pub caption:    AttrValue,
    #[prop_or_default]
    pub cols:       Rc<[AttrValue]>,
    #[prop_or_default]
    pub rows:       Vec<T>,
    #[prop_or_default]
    pub searchable: bool,
}

#[function_component(Table)]
pub fn base<T: PartialEq + ToHtml + Clone + Contains + 'static>(props: &TableProps<T>) -> Html
{
    let TableProps {
        id,
        caption,
        cols,
        rows,
        searchable,
    } = props;

    let search_id: AttrValue = format!("{}-search", id).into();
    let rows = use_state_eq(|| rows.clone());

    let onchange = {
        clone!(props, search_id, rows);
        Callback::from(move |_| {
            clone!(props, rows, search_id);
            if !search_table(rows.clone(), search_id)
            {
                rows.set(props.rows.clone());
            }
        })
    };

    html! {
        <div class="overflow-auto rounded-lg bg-primary-content p-3 w-full">

            {if *searchable {
                html! {
                    <div class="mb-3">
                        <SearchInput id={search_id}
                            {onchange}
                        />
                    </div>
                }
            } else {
                html! {}
            }}

            <table class="table table-pin-cols table-md w-full"
                {id}
            >

                <caption class="text-left text-lg caption-top">

                    { caption.to_string() }


                </caption>


                <thead class="font-bold text-primary">

                    <tr>

                    {
                        cols.iter().map(|col_header| {
                            html!{
                                <td>

                                    {col_header}

                                </td>
                            }
                        }).collect::<Html>()
                    }

                    </tr>

                </thead>

                <tbody class="">

                    {(*rows).clone()}

                </tbody>

            </table>

        </div>
    }
}

fn search_table<T: PartialEq + ToHtml + Clone + Contains + 'static>(
    rows: UseStateHandle<Vec<T>>,
    input_id: AttrValue,
) -> bool
{
    let input_element: HtmlInputElement = JsValue::from(
        gloo::utils::document()
            .get_element_by_id(input_id.as_ref())
            .expect("Failed getting search input element."),
    )
    .into();

    gloo::console::log!(input_element.value());
    let input_value = input_element.value();
    let new_rows = (*rows)
        .clone()
        .into_iter()
        .filter(|row| row.contains(&input_value))
        .collect::<Vec<T>>();

    rows.set(new_rows);

    if input_value == ""
    {
        false
    }
    else
    {
        true
    }
}
