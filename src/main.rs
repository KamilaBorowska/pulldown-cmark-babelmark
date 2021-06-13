use pulldown_cmark::{html, Parser};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{get, launch, routes};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Output {
    name: &'static str,
    version: &'static str,
    html: String,
}

#[get("/?<text>")]
fn output_html(text: &str) -> Json<Output> {
    let parser = Parser::new(text);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    Json(Output {
        name: "pulldown-cmark",
        version: "0.8.0",
        html,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/babelmark/pulldown-cmark", routes![output_html])
}
