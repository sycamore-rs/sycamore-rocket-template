use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use sycamore::{prelude::*, futures::ScopeSpawnFuture};
use wasm_bindgen::JsCast;
use web_sys::Element;
use log::debug;

#[component]
pub fn PostList<G: Html>(ctx: ScopeRef) -> View<G> {
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
                    "Loading... PostData"
                }
            })
        }
}

fn print_type_of<T>(_: &T) {
    debug!("{}", std::any::type_name::<T>())
}

#[component]
pub fn Post<G: Html>(ctx: ScopeRef, path: String) -> View<G> {
    if G::IS_BROWSER {
        let html = ctx.create_signal(String::new());

        let container_ref = ctx.create_node_ref();

        ctx.spawn_future(async move {
            let resp = Request::get(&format!("/posts/{}", path)).send().await.unwrap();
            debug!("Request::get resp : {:?}", resp);

            html.set(resp.text().await.unwrap());

            ctx.create_effect(on([html], || {
                debug!("container_ref: {:?}", container_ref.try_get_raw());
                print_type_of(&container_ref.get_raw());
                
                if let Some(dom_node) = container_ref.try_get::<HydrateNode>() {

                    dom_node
                        .inner_element()
                        .unchecked_into::<Element>()
                        .set_inner_html(html.get().as_str());
                }else{
                    debug!("no HydrateNode");
                }
            }));
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
