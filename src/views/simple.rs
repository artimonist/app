use artimonist::{BIP85, GenericDiagram, Matrix, ToMatrix};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::array::from_fn;
const SIMPLE_CSS: Asset = asset!("/assets/styling/simple.css");

#[component]
pub fn Simple() -> Element {
    let mut result = use_signal(|| String::new());
    rsx! {
      document::Link { rel: "stylesheet", href: SIMPLE_CSS }

      SimpleDiagram {}
      Passphrase {}
      button {
        class: "button",
        "data-style": "primary",
        onclick: move |_| {
            result.set(String::new());
            let mx: Matrix<_> = SIMPLE_VALUES
                .read()
                .iter()
                .map(|s| s.chars().next())
                .collect::<Vec<_>>()
                .to_matrix();
            let salt = PASS_PHRASE.read().clone();
            if let Ok(master) = artimonist::SimpleDiagram(mx).bip32_master(salt.as_bytes())
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

static SIMPLE_VALUES: GlobalSignal<[String; 49]> = Signal::global(|| from_fn(|_| String::new()));

#[component]
fn SimpleDiagram() -> Element {
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
