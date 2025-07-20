use dioxus::{logger::tracing, prelude::*};
use std::array::from_fn;

const COMPLEX_CSS: Asset = asset!("/assets/styling/complex.css");

#[derive(Clone, Copy)]
struct ValueState {
    values: Signal<[String; 49]>,
}

#[component]
pub fn ComplexDiagram() -> Element {
    let _ = use_context_provider(|| ValueState {
        values: Signal::new(from_fn(|_| String::new())),
    });

    let cells = (0..49).map(|i| rsx!(ComplexCell { index: i }));
    rsx! {
      document::Link { rel: "stylesheet", href: COMPLEX_CSS }
      div { class: "diagram", {cells} }
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
fn ComplexCell(index: usize) -> Element {
    let mut state = use_context::<ValueState>();
    let content = state.values.read()[index].clone();

    rsx! {
      textarea {
        value: "{content}",
        required: true,
        maxlength: "50",
        placeholder: " ",
        class: "cell",
        spellcheck: false,
        font_size: "1.0rem",
        // onchange: move |e| {
        //     content.set(e.value().chars().take(1).collect());
        // },
        oninput: move |e| {
            let content = e
                .value()
                .chars()
                .filter(|c| !matches!(c, '\r' | '\n'))
                .take(20)
                .collect::<String>();
            state.values.write()[index] = content.clone();
        },
      }
    }
}
