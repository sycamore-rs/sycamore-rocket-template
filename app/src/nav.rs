use sycamore::prelude::*;

#[component]
pub fn Nav<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        nav {
            a(href="/") { "Home" }
            a(href="/counter") { "Counter" }
            a(href="/blog") { "Blog" }
        }
    }
}
