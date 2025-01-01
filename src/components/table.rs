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
    } = props;

    html! {
        <div class="overflow-auto rounded-lg bg-primary-content p-3 w-full">

            <table class="table table-pin-cols table-md w-full">

                <caption class="text-left text-lg caption-top">

                    { caption.to_string() }

                </caption>

                <thead class="text-bold text-primary">

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

                    {rows}

                </tbody>

            </table>

        </div>
    }
}
