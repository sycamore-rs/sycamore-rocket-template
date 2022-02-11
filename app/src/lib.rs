mod counter;
mod index;
mod nav;
mod post;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, StaticRouter,
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

fn switch<'a, G: Html>(ctx: ScopeRef<'a>, route: &'a ReadSignal<AppRoutes>) -> View<G> {
    view! { ctx,
        div {
            nav::Nav()
            (match route.get().as_ref() {
                AppRoutes::Index => view! { ctx,
                    index::Index()
                },
                AppRoutes::Counter => view! { ctx,
                    counter::Counter()
                },
                AppRoutes::PostsList => view! { ctx,
                    post::PostList()
                },
                AppRoutes::Post { path } => view! { ctx,
                    post::Post(path.clone())
                },
                AppRoutes::NotFound => view! { ctx,
                    "404 Not Found"
                },
            })
        }
    }
}

/// # Props
/// * `pathname` - Set to `Some(_)` if running on the server.
#[component]
pub fn App<G: Html>(ctx: ScopeRef, pathname: Option<String>) -> View<G> {
    match pathname {
        Some(pathname) => {
            let route = AppRoutes::match_path(&pathname);
            view! { ctx,
                StaticRouter {
                    view: switch,
                    route: route,
                }
            }
        }
        None => view! { ctx,
            Router {
                view: switch,
                integration: HistoryIntegration::new(),
            }
        },
    }
}
