use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        h1 { "Dev2 Branch: Hello Rust!" }
    }
}
