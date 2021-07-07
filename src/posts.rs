use pulldown_cmark::{html::push_html, Parser};
use rocket::get;
use rocket::tokio::{fs::File, io::AsyncReadExt};

#[get("/posts")]
pub async fn posts() -> File {
    File::open("posts/index.json")
        .await
        .expect("cannot read posts/index.json")
}

#[get("/posts/<path>")]
pub async fn get_post(path: &str) -> Option<String> {
    let filename = format!("posts/{}.md", path);

    let mut file = File::open(&filename).await.ok()?;
    let mut md_buf = String::new();
    file.read_to_string(&mut md_buf)
        .await
        .expect("markdown file should be valid utf-8");

    let parser = Parser::new(&md_buf);

    let mut html_buf = String::new();
    push_html(&mut html_buf, parser);

    Some(html_buf)
}
