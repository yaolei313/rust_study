use axum::{
    extract::{Json, Path, Query},
    http::{
        header::{self, COOKIE},
        HeaderMap, HeaderName, HeaderValue, StatusCode,
    },
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use axum_extra::{response::Html, TypedHeader};
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/path/:id", get(path_request));
    let address = ("0.0.0.0", 8080);
    let listener = TcpListener::bind(address).await.expect("bind fail");
    axum::serve(listener, router).await.expect("serve fail");
}

async fn path_request(Path(id): Path<String>) {}

// response: String will get a `text/plain; charset=utf-8` content-type
async fn header_request(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}

// response: Bytes will get a `application/octet-stream` content-type
async fn query_request(Query(param): Query<SomeQuery>) -> Vec<u8> {
    todo!()
}

// response: `Html` will get a `text/html` content-type
async fn form_request(Form(form): Form<SomeForm>) -> Html<&'static str> {
    todo!()
}

// response: `Json` will get a `application/json` content-type and work with anything that implements `serde::Serialize`
async fn json_request(Json(json): Json<SomeJson>) -> Json<String> {
    todo!()
}

// empty response with special status code
async fn status() -> StatusCode {
    StatusCode::OK
}

async fn header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::SERVER, "axum".parse().unwrap());
    headers
}

async fn array_headers() -> [(HeaderName, HeaderValue); 2] {
    [
        (header::SERVER, "axum".parse().unwrap()),
        (header::CONTENT_TYPE, "text/plain".parse().unwrap()),
    ]
}

async fn impl_trait() -> impl IntoResponse {
    Html("hello world")
}

#[derive(Deserialize)]
struct SomeQuery {
    user_id: u64,
}

#[derive(Deserialize)]
struct SomeForm {
    a: String,
    b: Option<String>,
    c: u64,
    d: Option<u64>,
}

#[derive(Deserialize)]
struct SomeJson {}
