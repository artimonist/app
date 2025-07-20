use crate::components::SimpleDiagram;
use dioxus::prelude::*;

#[component]
pub fn Simple() -> Element {
    rsx! {
      SimpleDiagram {}
    }
}
