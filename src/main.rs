use std::{io, path::PathBuf};

use rocket::{fs::FileServer, get, launch, response, routes, tokio::fs};
use sycamore::prelude::*;

#[get("/<path..>", rank = 2)]
async fn index(path: PathBuf) -> io::Result<response::content::Html<String>> {
    let index_html = String::from_utf8(fs::read("app/dist/index.html").await?)
        .expect("index.html should be valid utf-8");

    let pathname = path.to_str().unwrap().to_string();
    dbg!(&pathname);

    let rendered = render_to_string(|| {
        template! {
            app::App(Some(pathname))
        }
    });

    let index_html = index_html.replace("%sycamore.body", &rendered);

    Ok(response::content::Html(index_html))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static/", FileServer::from("app/dist").rank(1))
}
