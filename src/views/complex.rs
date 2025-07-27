use artimonist::{BIP85, GenericDiagram, Matrix, ToMatrix};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::array::from_fn;

const COMPLEX_CSS: Asset = asset!("/assets/styling/complex.css");

#[component]
pub fn Complex(id: i32) -> Element {
    let mut result = use_signal(|| String::new());

    rsx! {
      document::Link { rel: "stylesheet", href: COMPLEX_CSS }

      ComplexDiagram {}
      input { class: "input", placeholder: "At least 5 unicode characters" }
      button {
        class: "button",
        "data-style": "primary",
        onclick: move |_| {
            let mx: Matrix<_> = COMPLEX_VALUES
                .read()
                .iter()
                .map(|s| s.chars().take(20).collect::<String>())
                .collect::<Vec<_>>()
                .to_matrix();
            let salt = PASS_PHRASE.read().clone();
            if let Ok(master) = artimonist::ComplexDiagram(mx).bip32_master(salt.as_bytes())
                && let Ok(mnemonic) = master.bip85_mnemonic(Default::default(), 12, 0)
            {
                tracing::debug!("master: {:?}", mnemonic);
                result.set(mnemonic);
            } else {
                tracing::error!("Failed to generate master key from diagram");
            }
        },
        "Generate"
      }
      div { "Mnemonic: {result}" }
    }
}

static COMPLEX_VALUES: GlobalSignal<[String; 49]> = Signal::global(|| from_fn(|_| String::new()));

#[component]
fn ComplexDiagram() -> Element {
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

static PASS_PHRASE: GlobalSignal<String> = Signal::global(|| String::new());

#[component]
fn Passphrase() -> Element {
    rsx! {
      input {
        class: "input",
        placeholder: "At least 5 unicode characters",
        value: PASS_PHRASE.read().clone(),
        oninput: move |e| {
            PASS_PHRASE.write().replace_range(.., &e.value());
        },
      }
    }
}
