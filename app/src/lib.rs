#![allow(non_snake_case)]
mod counter;
mod index;
mod nav;
mod post;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, StaticRouter};

#[derive(Route, Clone)]
pub enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/counter")]
    Counter,
    #[to("/blog")]
    PostsList,
    #[to("/blog/<path>")]
    Post {
        path: String
    },
    #[not_found]
    NotFound,
}

fn switch<'a, G: Html>(cx: Scope<'a>, route: &'a ReadSignal<AppRoutes>) -> View<G> {
    view! { cx,
        div {
            nav::Nav()
            (match route.get().as_ref() {
                AppRoutes::Index => view! { cx,
                    index::Index()
                },
                AppRoutes::Counter => view! { cx,
                    counter::Counter()
                },
                AppRoutes::PostsList => view! { cx,
                    post::PostList()
                },
                AppRoutes::Post { path } => view! { cx,
                    post::Post(path.clone())
                },
                AppRoutes::NotFound => view! { cx,
                    "404 Not Found"
                },
            })
        }
    }
}

/// # Props
/// * `pathname` - Set to `Some(_)` if running on the server.
#[component]
pub fn App<G: Html>(cx: Scope, pathname: Option<String>) -> View<G> {
    match pathname {
        Some(pathname) => {
            let route = AppRoutes::default().match_path(&pathname);
            view! { cx,
                StaticRouter(
                    view=switch,
                    route=route,
                )
            }
        }
        None => view! { cx,
            Router(
                view=switch,
                integration=HistoryIntegration::new(),
            )
        },
    }
}
