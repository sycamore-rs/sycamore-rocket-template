mod counter;
mod index;
mod nav;

use sycamore::prelude::*;
use sycamore_router::{BrowserRouter, Route, StaticRouter};

#[derive(Route)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/counter")]
    Counter,
    #[to("/blog")]
    Blog,
    #[not_found]
    NotFound,
}

fn switch<G: GenericNode>(route: AppRoutes) -> Template<G> {
    template! {
        div {
            nav::Nav()
            (match route {
                AppRoutes::Index => template! {
                    index::Index()
                },
                AppRoutes::Counter => template! {
                    counter::Counter()
                },
                AppRoutes::Blog => template! {
                    "This is the blog page. Not much to see here."
                },
                AppRoutes::NotFound => template! {
                    "404 Not Found"
                },
            })
        }
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
