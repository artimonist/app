use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::array::from_fn;

const COMPLEX_CSS: Asset = asset!("/assets/styling/complex.css");

#[component]
pub fn Complex(id: i32) -> Element {
    rsx! {
      document::Link { rel: "stylesheet", href: COMPLEX_CSS }

      ComplexDiagram {}
      br {}
      div {
        input {
          class: "input",
          placeholder: "At least 5 unicode characters",
        }
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
}

static COMPLEX_VALUES: GlobalSignal<[String; 49]> = Signal::global(|| from_fn(|_| String::new()));

#[component]
pub fn ComplexDiagram() -> Element {
    let cells = (0..49).map(|i| {
        let content = COMPLEX_VALUES.read()[i].clone();
        rsx! {
          textarea {
            value: "{content}",
            title: "{content}",
            required: true,
            maxlength: "50",
            placeholder: " ",
            class: "complex-cell",
            spellcheck: false,
            font_size: "1.0rem",
            oninput: move |e| {
                COMPLEX_VALUES.write()[i] = e
                    .value()
                    .chars()
                    .filter(|c| !matches!(c, '\r' | '\n'))
                    .take(20)
                    .collect::<String>();
            },
          }
        }
    });

    rsx! {
      div { class: "complex-diagram", {cells} }
    }
}
