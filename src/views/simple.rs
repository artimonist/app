use crate::components::SimpleDiagram;
use dioxus::prelude::*;

const SIMPLE_CSS: Asset = asset!("/assets/styling/simple.css");

#[component]
pub fn Simple() -> Element {
    rsx! {
      document::Link { rel: "stylesheet", href: SIMPLE_CSS }
      SimpleDiagram {}
    }
}
