use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar))
    .route("/plain_text", get(plain_text))
    .route("/json", get(json));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  println!("listening on port 3000");
  axum::serve(listener, app).await.unwrap();
}

// which calls one of these handlers
async fn root() -> String {
  "hello".to_string()
}
async fn get_foo() -> String {
  "foo".to_string()
}
async fn post_foo() -> String {
  "post_foo".to_string()
}
async fn foo_bar() -> String {
  "foo_bar".to_string()
}

// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn plain_text() -> &'static str {
  "foo"
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
  Json(json!({ "data": 42 }))
}
