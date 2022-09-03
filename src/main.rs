mod posts;

use std::io;

use rocket::fs::Options;
use rocket::http::uri::fmt::Path;
use rocket::http::uri::Segments;
use rocket::{fs::FileServer, get, launch, response, routes, tokio::fs};
use sycamore::prelude::*;

#[get("/<path..>", rank = 2)]
async fn index(path: Segments<'_, Path>) -> io::Result<response::content::RawHtml<String>> {
    let index_html = String::from_utf8(fs::read("app/dist/index.html").await?)
        .expect("app/dist/index.html should be valid utf-8");

    let mut pathname = String::new();
    for segment in path {
        pathname += &segment.to_string();
        pathname += "/";
    }

    let rendered = sycamore::render_to_string(|cx| {
        view! { cx,
            app::App(Some(pathname))
        }
    });

    let index_html = index_html.replace("%sycamore.body", &rendered);

    Ok(response::content::RawHtml(index_html))
}

#[get("/favicon.ico")]
fn favicon() -> Option<()> {
    None
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon, posts::posts, posts::get_post])
        .mount("/", FileServer::new("app/dist", Options::None).rank(1))
}
