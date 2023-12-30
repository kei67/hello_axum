use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar));

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
