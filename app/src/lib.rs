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

fn switch<G: Html>(route: ReadSignal<AppRoutes>) -> View<G> {
    view! {
        div {
            nav::Nav()
            (match route.get().as_ref() {
                AppRoutes::Index => view! {
                    index::Index()
                },
                AppRoutes::Counter => view! {
                    counter::Counter()
                },
                AppRoutes::PostsList => view! {
                    post::PostsList()
                },
                AppRoutes::Post { path } => view! {
                    post::Post(path.clone())
                },
                AppRoutes::NotFound => view! {
                    "404 Not Found"
                },
            })
        }
    }
}

/// # Props
/// * `pathname` - Set to `Some(_)` if running on the server.
#[component(App<G>)]
pub fn app(pathname: Option<String>) -> View<G> {
    match pathname {
        Some(pathname) => {
            let route = AppRoutes::match_path(&pathname);
            view! {
                StaticRouter(StaticRouterProps::new(route, |route: AppRoutes| switch(Signal::new(route).handle())))
            }
        }
        None => view! {
            Router(RouterProps::new(HistoryIntegration::new(), switch))
        },
    }
}
