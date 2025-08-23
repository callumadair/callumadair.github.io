use dioxus::prelude::*;
use shared::components::{
    AccordionItem,
    BackgroundColour,
    Collapse,
    CollapseModifier,
    PeerBackgroundColour,
    PeerTextColour,
    TextColour,
};

#[component]
pub fn ProjectBase() -> Element
{
    rsx! {
        div {
            class: "flex flex-col gap-5 p-5",
            CurrentProjects {}
        }
    }
}

#[component]
fn CurrentProjects() -> Element
{
    rsx! {
        div {
            class: "flex flex-col gap-3",

            h2 {
                class: "text-2xl text-nowrap",
                { "Current Projects" }
            }

            div {
                class: "flex flex-col w-full items-center justify-center gap-3",

                Collapse {
                    title: "A collapse",
                    checked_background_colour: PeerBackgroundColour::Accent,
                    checked_text_colour: PeerTextColour::AccentContent,
                    initial_background_colour: BackgroundColour::Secondary,
                    initial_text_colour: TextColour::SecondaryContent,
                    symbol: CollapseModifier::Plus,
                    "Item"
                }

                AccordionItem {
                    title: "Accordion Item One",
                    checked_background_colour: PeerBackgroundColour::Accent,
                    checked_text_colour: PeerTextColour::AccentContent,
                    initial_background_colour: BackgroundColour::Primary,
                    initial_text_colour: TextColour::PrimaryContent,
                    name: "accordion-1",
                    "Item"
                }

                AccordionItem {
                    title: "Accordion Item Two",
                    checked_background_colour: PeerBackgroundColour::Accent,
                    checked_text_colour: PeerTextColour::AccentContent,
                    initial_background_colour: BackgroundColour::Primary,
                    initial_text_colour: TextColour::PrimaryContent,
                    name: "accordion-1",
                    "Item"
                }
            }
        }
    }
}
