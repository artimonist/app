use dioxus::prelude::*;
use views::{Complex, Navbar, Simple};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]

      #[route("/")]
      Simple {},

      #[route("/blog/:id")]
      Complex { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
      document::Link { rel: "icon", href: FAVICON }
      document::Link { rel: "stylesheet", href: MAIN_CSS }

      Router::<Route> {}
    }
}
