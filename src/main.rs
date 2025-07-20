use dioxus::prelude::*;
use views::{Complex, Navbar, Simple};

mod components;
mod views;

const THEME_CSS: Asset = asset!("/assets/styling/theme.css");
const STYLE_CSS: Asset = asset!("/assets/styling/style.css");

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
      document::Link { rel: "stylesheet", href: THEME_CSS }
      document::Link { rel: "stylesheet", href: STYLE_CSS }

      Router::<Route> {}
    }
}
