use sycamore::prelude::*;

#[component(Nav<G>)]
pub fn nav() -> View<G> {
    view! {
        nav {
            a(href="/") { "Home" }
            a(href="/counter") { "Counter" }
            a(href="/blog") { "Blog" }
        }
    }
}
