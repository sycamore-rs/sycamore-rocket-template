use sycamore::prelude::*;

#[component(Index<G>)]
pub fn index() -> Template<G> {
    template! {
        div(class="index") {
            h1 { "Hello Sycamore!" }
            img(class="cover-img", src="/static/images/glastonburygrove.jpg")
            p {
                "Welcome to your new Sycamore app. Edit "
                strong { "app/src/index.rs" }
                " and reload this page to see your changes."
            }
        }
    }
}
