// SPDX-FileCopyrightText: 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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

#[get("/api/babelmark/pulldown-cmark?<text>")]
fn output_html(text: &str) -> Json<Output> {
    let parser = Parser::new(text);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    Json(Output {
        name: "pulldown-cmark",
        version: env!("PULLDOWN_CMARK_VERSION"),
        html,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![output_html])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use rocket::uri;
    use serde_json::json;

    #[test]
    fn test_output() {
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let response = client
            .get(uri!(super::output_html(text = "**bold**")))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(
            response.into_json(),
            Some(json!({
                "name": "pulldown-cmark",
                "version": env!("PULLDOWN_CMARK_VERSION"),
                "html": "<p><strong>bold</strong></p>\n",
            })),
        );
    }
}
