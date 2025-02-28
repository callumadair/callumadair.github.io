use std::rc::Rc;

use dioxus::prelude::*;
use web_sys::{
    wasm_bindgen::JsCast,
    HtmlInputElement,
};

use crate::{
    components::input::SearchInput,
    traits::contains::Contains,
};

#[derive(Props, PartialEq, Clone)]
pub struct TableProps<T>
where
    T: PartialEq + IntoDynNode + Clone + Contains,
{
    pub id:         String,
    #[props(default)]
    pub title:      String,
    #[props(default)]
    pub cols:       Rc<[String]>,
    #[props(default)]
    pub rows:       Vec<T>,
    #[props(default)]
    pub searchable: bool,
}

#[component]
pub fn base<T: PartialEq + IntoDynNode + Clone + Contains + 'static>(
    props: TableProps<T>
) -> Element
{
    let TableProps {
        id,
        title,
        cols,
        searchable,
        ..
    } = props;

    let search_id = format!("{id}-search");
    let rows = use_signal(|| props.rows.clone());
    let mut display_rows = use_signal(|| props.rows.clone());

    let onkeyup = {
        move |event: KeyboardEvent| {
            search_table(&event, &rows, &mut display_rows);
        }
    };

    rsx! {
        div {
            class: "grid grid-cols-1 gap-5 p-3 overflow-auto rounded-lg rounded-box",

            h2 {
                class: "flex w-full p-3 items-center justify-between text-left text-2xl",

                { title.to_string() }

                {if *searchable {
                    rsx! {
                        SearchInput {
                            id: search_id,
                            onkeyup
                        }
                    }
                } else {
                    rsx! {}
                }}
            },

            table {
                class: "table table-pin-cols table-sm w-full",
                id,

                thead {
                    class: "font-bold",

                    tr {

                    {
                        cols.iter().map(|col_header| {
                            rsx!{
                                td {
                                    {col_header.clone()}
                                }
                            }
                        }).collect::<Vec<Element>>()
                    }

                    }
                }
                tbody {
                    class: "",
                    {(*display_rows).clone()}
                    }
            }
        }
    }
}

fn search_table<T: PartialEq + Clone + Contains + 'static>(
    evt: &KeyboardEvent,
    rows: &Signal<Vec<T>>,
    display_rows: &mut Signal<Vec<T>>,
)
{
    let input_value = evt
        .target()
        .expect("Event should have an originating target when dispatched.")
        .unchecked_into::<HtmlInputElement>()
        .value();

    let new_rows = (**rows)
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
