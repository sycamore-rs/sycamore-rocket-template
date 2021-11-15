use std::any::TypeId;

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::futures::spawn_local_in_scope;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;

#[component(PostsList<G>)]
pub fn posts_list() -> View<G> {
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
        let post_list = Signal::new(None::<PostList>);

        spawn_local_in_scope(cloned!((post_list) => async move {
            let resp = Request::get(&format!("/posts")).send().await.unwrap();
            post_list.set(Some(resp.json().await.expect("cannot parse post list")))
        }));

        create_effect(cloned!((post_list) => move || {
            log::info!("{:?}", post_list);
        }));

        view! {
            (if let Some(post_list) = post_list.get().as_ref() {
                let templates = post_list.posts.iter().cloned().map(|post| {
                    let PostData { title, path } = post;
                    view! {
                        li {
                            a(href=format!("/blog/{}", path)) { (title) }
                        }
                    }
                }).collect();
                let templates = View::new_fragment(templates);
                view! {
                    ul {
                        (templates)
                    }
                }
            }
            else {
                view! {
                    "Loading..."
                }
            })
        }
    } else {
        view! {
            "Loading..."
        }
    }
}

#[component(Post<G>)]
pub fn post(path: String) -> View<G> {
    if TypeId::of::<G>() == TypeId::of::<DomNode>() {
        let html = Signal::new(String::new());

        let container_ref = NodeRef::new();

        spawn_local(cloned!((container_ref, html) => async move {
            let resp = Request::get(&format!("/posts/{}", path)).send().await.unwrap();
            html.set(resp.text().await.unwrap());

            create_effect(cloned!((container_ref, html) => move || {
                if let Some(dom_node) = container_ref.try_get::<DomNode>() {
                    dom_node
                        .inner_element()
                        .unchecked_into::<Element>()
                        .set_inner_html(html.get().as_str());
                }
            }));
        }));

        view! {
            div(class="container", ref=container_ref)
        }
    } else {
        view! {
            "Loading..."
        }
    }
}
