use crate::components::ComplexDiagram;
use dioxus::prelude::*;

#[component]
pub fn Complex(id: i32) -> Element {
    rsx! {
      ComplexDiagram {}
    }
}
