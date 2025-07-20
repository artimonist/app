use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::array::from_fn;

const SIMPLE_CSS: Asset = asset!("/assets/styling/simple.css");

#[component]
pub fn Simple() -> Element {
    rsx! {
      document::Link { rel: "stylesheet", href: SIMPLE_CSS }

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

static SIMPLE_VALUES: GlobalSignal<[String; 49]> = Signal::global(|| from_fn(|_| String::new()));

#[component]
pub fn SimpleDiagram() -> Element {
    let cells = (0..49).map(|i| {
        let content = SIMPLE_VALUES.read()[i].clone();
        rsx! {
          textarea {
            value: "{content}",
            title: "{content}",
            required: true,
            maxlength: "2",
            placeholder: " ",
            class: "simple-cell",
            oninput: move |e| {
                SIMPLE_VALUES.write()[i] = e.value().chars().take(1).collect::<String>();
            },
          }
        }
    });
    rsx! {
      div { class: "simple-diagram", {cells} }
    }
}
