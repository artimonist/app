use dioxus::{logger::tracing, prelude::*};
use std::array::from_fn;

const SIMPLE_CSS: Asset = asset!("/assets/styling/simple.css");

#[derive(Clone, Copy)]
struct ValueState {
    values: Signal<[String; 49]>,
}

#[component]
pub fn SimpleDiagram() -> Element {
    let _ = use_context_provider(|| ValueState {
        values: Signal::new(from_fn(|_| String::new())),
    });

    let cells = (0..49).map(|i| rsx!(SimpleCell { index: i }));
    rsx! {
      document::Link { rel: "stylesheet", href: SIMPLE_CSS }
      div { class: "simple-diagram", {cells} }
      br {}
      button {
        onclick: move |_| {
            let state = use_context::<ValueState>();
            tracing::debug!("values: {:?}", state.values.read());
        },
        "Generate"
      }
    }
}

#[component]
fn SimpleCell(index: usize) -> Element {
    let mut state = use_context::<ValueState>();
    let content = state.values.read()[index].clone();

    rsx! {
      textarea {
        value: "{content}",
        required: true,
        maxlength: "2",
        placeholder: " ",
        class: "simple-cell",
        oninput: move |e| {
            let content = e.value().chars().take(1).collect::<String>();
            state.values.write()[index] = content.clone();
        },
      }
    }
}
