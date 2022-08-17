use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let user = Some("Toshiki");

    cx.render(rsx!(
        div {
            h1 { "Hello, ", [if let Some(name) = user { name } else { "World" }], "!" }
            h2 { onclick: move |_|  println!("h2 clicked"), "This is a Rust app" }
            ol {
                li { "Rust" }
                li { "is" }
                li { "awesome" }
            }
            div { "This is second project of Dioxus." }
        }
    ))
}
