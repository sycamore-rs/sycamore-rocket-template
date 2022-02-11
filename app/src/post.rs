use std::any::TypeId;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::{prelude::*, futures::ScopeSpawnFuture};
use wasm_bindgen::JsCast;
use web_sys::Element;

#[component]
pub fn PostList<G: Html>(ctx: ScopeRef) -> View<G> {
    if TypeId::of::<G>() == TypeId::of::<DomNode>() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct PostData {
            title: String,
            path: String,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct PostList {
            posts: Vec<PostData>,
        }
        let post_list = ctx.create_signal(None::<PostList>);

        ctx.spawn_future(async move {
            let resp = Request::get(&format!("/posts")).send().await.unwrap();
            post_list.set(Some(resp.json().await.expect("cannot parse post list")))
        });

        ctx.create_effect(move || {
            log::info!("{:?}", post_list);
        });

        view! { ctx,
            (if let Some(post_list) = post_list.get().as_ref() {
                let templates = post_list.posts.iter().cloned().map(|post| {
                    let PostData { title, path } = post;
                    view! { ctx,
                        li {
                            a(href=format!("/blog/{}", path)) { (title) }
                        }
                    }
                }).collect();
                let templates = View::new_fragment(templates);
                view! { ctx,
                    ul {
                        (templates)
                    }
                }
            }
            else {
                view! { ctx,
                    "Loading..."
                }
            })
        }
    } else {
        view! { ctx,
            "Loading..."
        }
    }
}

#[component]
pub fn Post<G: Html>(ctx: ScopeRef, path: String) -> View<G> {
    if G::IS_BROWSER {
        let html = ctx.create_signal(String::new());

        let container_ref = ctx.create_node_ref();

        ctx.spawn_future(async move {
            let resp = Request::get(&format!("/posts/{}", path)).send().await.unwrap();
            html.set(resp.text().await.unwrap());

            ctx.create_effect(|| {
                if let Some(dom_node) = container_ref.try_get::<DomNode>() {
                    dom_node
                        .inner_element()
                        .unchecked_into::<Element>()
                        .set_inner_html(html.get().as_str());
                }
            });
        });

        view! { ctx,
            div(class="container", ref=container_ref)
        }
    } else {
        view! { ctx,
            "Loading..."
        }
    }
}
