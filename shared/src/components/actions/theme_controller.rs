use yew::prelude::*;

use crate::components::Dropdown;

// TODO add in logic for actually changing and storing theme
// state.
#[derive(Clone, PartialEq)]
pub enum Toggle
{
    ExternalIcons(Html, String, Html, String),
    InternalIcons(Html, String, Html, String),
    Text(String, String),
    NoDisplay(String, String),
}

impl ToHtml for Toggle
{
    fn to_html(&self) -> Html
    {
        match self
        {
            Toggle::ExternalIcons(left_icon_html, _left_theme, right_icon_html, right_theme) =>
            {
                html! {
                    <label class="flex cursor-pointer gap-2">

                        {left_icon_html.clone()}

                        <input type="checkbox"
                            value={right_theme.clone()}
                            class="toggle theme-controller"
                        />

                        {right_icon_html.clone()}

                    </label>
                }
            }
            Toggle::InternalIcons(left_icon_html, _left_theme, right_icon_html, right_theme) =>
            {
                html! {
                    <label class="toggle text-base-content">

                        <input type="checkbox"
                            value={right_theme.clone()}
                            class="theme-controller"
                        />

                        {left_icon_html.clone()}

                        {right_icon_html.clone()}

                    </label>
                }
            }
            Toggle::Text(left_theme, right_theme) =>
            {
                html! {
                    <label class="flex cursor-pointer gap-2">

                        <span class="label-text">
                            {left_theme.clone()}
                        </span>

                        <input type="checkbox"
                            value={right_theme.clone()}
                            class="toggle theme-controller"
                        />

                        <span class="label-text">
                            {right_theme.clone()}
                        </span>

                    </label>
                }
            }
            Toggle::NoDisplay(_left_theme, right_theme) =>
            {
                html! {
                    <input type="checkbox" value={right_theme.clone()} class="toggle theme-controller"/>
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Radio
{
    Dropdown(Vec<String>),
    Input(Vec<String>),
    Standard(Vec<String>),
}

impl ToHtml for Radio
{
    fn to_html(&self) -> Html
    {
        match self
        {
            Radio::Dropdown(options) =>
            {
                html! {
                   <Dropdown

                   >

                   </Dropdown>
                }
            }
            Radio::Input(options) =>
            {
                html! {
                    <fieldset class="fieldset">
                        {
                            options.iter().map(|option| {
                                html! {
                                    <label class="flex gap-2 cursor-pointer items-center">
                                        <input type="radio"
                                            name="theme-radios"
                                            class="radio radio-sm theme-controller"
                                            value={option.clone()}
                                        />
                                    </label>
                                }
                            }).collect::<Html>()
                        }

                    </fieldset>
                }
            }
            Radio::Standard(options) =>
            {
                html! {
                    <div class="join join-vertical">
                        {
                            options.iter().map(|option| {
                                html! {
                                    <input type="radio"
                                        name="theme-buttons"
                                        class="btn theme-controller join-item"
                                        aria-label={option.clone()}
                                        value={option.clone()}
                                    />
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
            }
        }
    }
}

// TODO impl ToHtml on this enum.
#[derive(Clone, PartialEq)]
pub enum Type
{
    CheckBox(String, String),
    Toggle(Toggle),
    Radio(Radio),
    Swap(String, String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct ThemeControllerProps
{
    pub controller_type: Type,
}

#[function_component(ThemeController)]
pub fn theme_controller(props: &ThemeControllerProps) -> Html
{
    // props.controller_type.clone().to_html()
    html! {}
}
