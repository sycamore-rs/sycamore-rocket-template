use sycamore::prelude::*;

#[component(Nav<G>)]
pub fn nav() -> Template<G> {
    template! {
        nav {
            a(href="/") { "Home" }
            a(href="/counter") { "Counter" }
            a(href="/blog") { "Blog" }
        }
    }
}