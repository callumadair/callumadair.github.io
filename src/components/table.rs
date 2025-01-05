use std::rc::Rc;

use web_sys::{
    wasm_bindgen::JsCast,
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
        searchable,
        ..
    } = props;

    let search_id: AttrValue = format!("{}-search", id).into();
    let rows = use_state(|| props.rows.clone());
    let display_rows = use_state(|| props.rows.clone());

    let onkeyup = {
        clone!(rows, display_rows);
        Callback::from(move |event: KeyboardEvent| {
            clone!(rows, display_rows);
            search_table(event, rows, display_rows);
        })
    };

    html! {
        <div class="overflow-auto rounded-lg bg-primary-content p-3 w-full">

            {if *searchable {
                html! {
                    <div class="mb-3">
                        <SearchInput id={search_id}
                            {onkeyup}
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

                    {(*display_rows).clone()}

                </tbody>

            </table>

        </div>
    }
}

fn search_table<T: PartialEq + Clone + Contains + 'static>(
    evt: KeyboardEvent,
    rows: UseStateHandle<Vec<T>>,
    display_rows: UseStateHandle<Vec<T>>,
)
{
    let input_value = evt
        .target()
        .expect("Event should have an originating target when dispatched.")
        .unchecked_into::<HtmlInputElement>()
        .value();

    let new_rows = (*rows)
        .clone()
        .into_iter()
        .filter(|row| row.contains(&input_value))
        .collect::<Vec<T>>();

    display_rows.set(new_rows);
}

// fn sort_table<T>(
//     evt: MouseEvent,
//     rows: UseStateHandle<Vec<T>>,
//     display_rows: UseStateHandle<Vec<T>>,
// )
// {
//     let field_to_sort_by = evt.target().
// }
