use std::rc::Rc;

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TableProps<T>
where
    T: PartialEq + ToHtml,
{
    #[prop_or_default]
    pub caption: AttrValue,
    #[prop_or_default]
    pub cols:    Rc<[AttrValue]>,
    #[prop_or_default]
    pub rows:    Vec<T>,
}

#[function_component(Table)]
pub fn base<T: PartialEq + ToHtml>(props: &TableProps<T>) -> Html
{
    let TableProps {
        caption,
        cols,
        rows,
    } = props.clone();

    html! {
        <div class="overflow-x-auto bg-neutral rounded-md p-3 w-full">

            <table class="table table-md w-full">

                <caption class="text-left caption-top">

                    { caption.to_string() }

                </caption>

                <thead class="text-primary">

                    <tr>

                    {
                        cols.iter().map(|col_header| {
                            html!{
                                <th>

                                    {col_header}

                                </th>
                            }
                        }).collect::<Html>()
                    }

                    </tr>

                </thead>

                <tbody class="text-secondary">

                    {rows}

                </tbody>

            </table>

        </div>
    }
}
