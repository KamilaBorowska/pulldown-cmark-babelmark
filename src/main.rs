use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use warp::{path, Filter};

#[derive(Deserialize)]
struct Query {
    text: String,
}

#[derive(Serialize)]
struct Output {
    name: &'static str,
    version: &'static str,
    html: String,
}

fn main() {
    env_logger::init();
    let route = path!("api" / "babelmark" / "pulldown-cmark")
        .and(warp::path::end())
        .and(warp::query())
        .map(|Query { text }| {
            let parser = Parser::new(&text);
            let mut html = String::new();
            html::push_html(&mut html, parser);
            warp::reply::json(&Output {
                name: "pulldown-cmark",
                version: "0.5.2",
                html,
            })
        })
        .with(warp::log("babelmark"));
    warp::serve(route).run(([127, 0, 0, 1], 8081));
}
