use std::any::TypeId;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::{prelude::*, futures::spawn_local_scoped};
use wasm_bindgen::JsCast;
use web_sys::Element;

#[component]
pub fn PostList<G: Html>(cx: Scope) -> View<G> {

    if TypeId::of::<G>() == TypeId::of::<HydrateNode>() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct PostData {
            title: String,
            path: String,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct PostList {
            posts: Vec<PostData>,
        }
        let post_list = create_signal(cx, None::<PostList>);

        spawn_local_scoped(cx, async move {
            let resp = Request::get(&format!("/posts")).send().await.unwrap();
            post_list.set(Some(resp.json().await.expect("cannot parse post list")))
        });

        create_effect_scoped(cx, move |_| {
            log::info!("{:?}", post_list);
        });

        view! { cx,
            (if let Some(post_list) = post_list.get().as_ref() {
                let templates = post_list.posts.iter().cloned().map(|post| {
                    let PostData { title, path } = post;
                    view! { cx,
                        li {
                            a(href=format!("/blog/{}", path)) { (title) }
                        }
                    }
                }).collect();
                let templates = View::new_fragment(templates);
                view! { cx,
                    ul {
                        (templates)
                    }
                }
            }
            else {
                view! { cx,
                    "Loading..."
                }
            })
        }
    } else {
        view! { cx,
            "Loading..."
        }
    }
}

#[component]
pub fn Post<G: Html>(cx: Scope, path: String) -> View<G> {
    if G::IS_BROWSER {
        let html = create_signal(cx, String::new());

        let container_ref = create_node_ref(cx);

        spawn_local_scoped(cx, async move {
            let resp = Request::get(&format!("/posts/{}", path)).send().await.unwrap();
            html.set(resp.text().await.unwrap());

            create_effect_scoped(cx, |_| {
                if let Some(dom_node) = container_ref.try_get::<HydrateNode>() {
                    dom_node
                        .inner_element()
                        .unchecked_into::<Element>()
                        .set_inner_html(html.get().as_str());
                }
            });
        });

        view! { cx,
            div(class="container", ref=container_ref)
        }
    } else {
        view! { cx,
            "Loading..."
        }
    }
}
