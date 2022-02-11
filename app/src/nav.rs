use sycamore::prelude::*;

#[component]
pub fn Nav<G: Html>(ctx: ScopeRef) -> View<G> {
    view! { ctx,
        nav {
            a(href="/") { "Home" }
            a(href="/counter") { "Counter" }
            a(href="/blog") { "Blog" }
        }
    }
}
