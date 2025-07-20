use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::array::from_fn;

#[component]
pub fn Complex(id: i32) -> Element {
    rsx! {
      ComplexDiagram {}
      br {}
      button {
        class: "button",
        "data-style": "primary",
        onclick: move |_| {
            tracing::debug!("values: {:?}", COMPLEX_VALUES.read());
        },
        "Generate"
      }
    }
}

const COMPLEX_CSS: Asset = asset!("/assets/styling/complex.css");
static COMPLEX_VALUES: GlobalSignal<[String; 49]> = Signal::global(|| from_fn(|_| String::new()));

#[component]
pub fn ComplexDiagram() -> Element {
    let cells = (0..49).map(|i| rsx!(
      ComplexCell { index: i }
    ));
    rsx! {
      document::Link { rel: "stylesheet", href: COMPLEX_CSS }
      div { class: "complex-diagram", {cells} }
    }
}

#[component]
fn ComplexCell(index: usize) -> Element {
    let content = COMPLEX_VALUES.read()[index].clone();

    rsx! {
      textarea {
        value: "{content}",
        required: true,
        maxlength: "50",
        placeholder: " ",
        class: "complex-cell",
        spellcheck: false,
        font_size: "1.0rem",
        oninput: move |e| {
            let content = e
                .value()
                .chars()
                .filter(|c| !matches!(c, '\r' | '\n'))
                .take(20)
                .collect::<String>();
            COMPLEX_VALUES.write()[index] = content;
        },
      }
    }
}
