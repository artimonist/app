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

fn main() {
    dioxus::launch(App);
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const THEME_CSS: Asset = asset!("/assets/styling/theme.css");
const STYLE_CSS: Asset = asset!("/assets/styling/style.css");

#[component]
fn App() -> Element {
    rsx! {
      document::Title { "Artimonist" }
      document::Link { rel: "icon", href: FAVICON }

      document::Link { rel: "stylesheet", href: MAIN_CSS }
      document::Link { rel: "stylesheet", href: THEME_CSS }
      document::Link { rel: "stylesheet", href: STYLE_CSS }

      Router::<Route> {}
    }
}
