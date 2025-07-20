use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::array::from_fn;

#[component]
pub fn Simple() -> Element {
    rsx! {
      SimpleDiagram {}
      br {}
      button {
        class: "button",
        "data-style": "primary",
        onclick: move |_| {
            tracing::debug!("values: {:?}", SIMPLE_VALUES.read());
        },
        "Generate"
      }
    }
}

const SIMPLE_CSS: Asset = asset!("/assets/styling/simple.css");
static SIMPLE_VALUES: GlobalSignal<[String; 49]> = Signal::global(|| from_fn(|_| String::new()));

#[component]
pub fn SimpleDiagram() -> Element {
    let cells = (0..49).map(|i| rsx!(
      SimpleCell { index: i }
    ));
    rsx! {
      document::Link { rel: "stylesheet", href: SIMPLE_CSS }
      div { class: "simple-diagram", {cells} }
    }
}

#[component]
fn SimpleCell(index: usize) -> Element {
    let content = SIMPLE_VALUES.read()[index].clone();

    rsx! {
      textarea {
        value: "{content}",
        required: true,
        maxlength: "2",
        placeholder: " ",
        class: "simple-cell",
        oninput: move |e| {
            let content = e.value().chars().take(1).collect::<String>();
            SIMPLE_VALUES.write()[index] = content.clone();
        },
      }
    }
}
