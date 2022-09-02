use sycamore::prelude::*;

#[component]
pub fn Index<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="index") {
            h1 { "Hello Sycamore!" }
            img(class="cover-img", src="/static/images/glastonburygrove.jpg")
            p {
                "Welcome to your new Sycamore app. Edit "
                strong { "app/src/index.rs" }
                " and reload this page to see your changes.s"
            }
        }
    }
}
