use crate::components::ComplexDiagram;
use dioxus::prelude::*;

const COMPLEX_CSS: Asset = asset!("/assets/styling/complex.css");

#[component]
pub fn Complex(id: i32) -> Element {
    rsx! {
      document::Link { rel: "stylesheet", href: COMPLEX_CSS }
      ComplexDiagram {}
    }
}
