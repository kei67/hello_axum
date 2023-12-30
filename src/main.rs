use axum::{
  error_handling::HandleError,
  http::{Response, StatusCode},
  response::Json,
  routing::get,
  Router,
};
use serde::{Deserialize, Serialize};

use anyhow::anyhow;
use rand::Rng;

#[tokio::main]
async fn main() {
  // this service might fail with `anyhow::Error`
  let some_fallible_service = tower::service_fn(|_req| async {
    let result = thing_that_might_fail().await?;
    Ok::<_, anyhow::Error>(Response::new(result))
  });

  let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar))
    .route("/plain_text", get(plain_text))
    .route("/json", get(json))
    .route_service(
      "/error",
      // we cannot route to `some_fallible_service` directly since it might fail.
      // we have to use `handle_error` which converts its errors into responses
      // and changes its error type from `anyhow::Error` to `Infallible`.
      HandleError::new(some_fallible_service, handle_anyhow_error),
    );

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

#[derive(Serialize, Deserialize)]
struct Address {
  street: String,
  city: String,
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Address> {
  let address = Address {
    street: "10 Downing Street".to_owned(),
    city: "London".to_owned(),
  };

  Json(address)
}

async fn thing_that_might_fail() -> Result<String, anyhow::Error> {
  let mut rng = rand::thread_rng();
  let random_bool: bool = rng.gen(); // 乱数で true または false を生成

  if random_bool {
    // 50% の確率でエラーを返す
    Err(anyhow!("Function failed"))
  } else {
    // 成功した場合
    Ok("Success".to_string())
  }
}

// handle errors by converting them into something that implements
// `IntoResponse`
async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
  println!("Error occured {:?}", err);
  (
    StatusCode::INTERNAL_SERVER_ERROR,
    format!("Something went wrong: {err}"),
  )
}
