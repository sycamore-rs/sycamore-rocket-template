mod counter;
mod index;
mod nav;
mod post;

use sycamore::prelude::*;
use sycamore_router::{
    HistoryIntegration, Route, Router, RouterProps, StaticRouter, StaticRouterProps,
};

#[derive(Route, Clone)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/counter")]
    Counter,
    #[to("/blog")]
    PostsList,
    #[to("/blog/<path>")]
    Post { path: String },
    #[not_found]
    NotFound,
}

fn switch<G: GenericNode>(route: StateHandle<AppRoutes>) -> Template<G> {
    template! {
        div {
            nav::Nav()
            (match route.get().as_ref() {
                AppRoutes::Index => template! {
                    index::Index()
                },
                AppRoutes::Counter => template! {
                    counter::Counter()
                },
                AppRoutes::PostsList => template! {
                    post::PostsList()
                },
                AppRoutes::Post { path } => template! {
                    post::Post(path.clone())
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
        Some(pathname) => {
            let route = AppRoutes::match_path(&pathname);
            template! {
                StaticRouter(StaticRouterProps::new(route, |route: AppRoutes| switch(Signal::new(route).handle())))
            }
        }
        None => template! {
            Router(RouterProps::new(HistoryIntegration::new(), switch))
        },
    }
}
