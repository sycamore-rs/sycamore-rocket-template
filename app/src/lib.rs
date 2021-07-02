use sycamore::prelude::*;
use sycamore_router::{BrowserRouter, Route, StaticRouter};

#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/about")]
    About,
    #[not_found]
    NotFound,
}

fn switch<G: GenericNode>(route: AppRoutes) -> Template<G> {
    match route {
        AppRoutes::Index => template! {
            div {
                "Hello Sycamore!"
                br
                a(href="/about") { "About" }
            }
        },
        AppRoutes::About => template! {
            "This is the about page. Not much to see here."
        },
        AppRoutes::NotFound => template! {
            "404 Not Found"
        },
    }
}

/// # Props
/// * `pathname` - Set to `Some(_)` if running on the server.
#[component(App<G>)]
pub fn app(pathname: Option<String>) -> Template<G> {
    match pathname {
        Some(pathname) => template! {
            StaticRouter((pathname, switch))
        },
        None => template! {
            BrowserRouter(switch)
        },
    }
}
