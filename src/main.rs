use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div { "Hello, World!" }
        div { "This is second project of Dioxus." }
    ))
}
