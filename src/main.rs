#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "text-size: 200px",
            "Hello!"
        }
    }
}
// blah