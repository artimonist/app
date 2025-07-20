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
      div { id: "simple-diagram", class: "diagram", {cells} }
      button {
        onclick: move |_| {
            let state = use_context::<ValueState>();
            tracing::debug!("values: {:?}", state.values.read());
        },
        "Reset"
      }
    }
}

#[component]
fn SimpleCell(index: usize) -> Element {
    let mut state = use_context::<ValueState>();

    rsx! {
      textarea {
        value: "{state.values.read()[index]}",
        required: true,
        maxlength: "2",
        placeholder: " ",
        class: "cell",
        // onchange: move |e| {
        //     content.set(e.value().chars().take(1).collect());
        // },
        oninput: move |e| {
            state.values.write()[index] = e.value().chars().take(1).collect();
        },
      }
    }
}
