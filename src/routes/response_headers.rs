use axum::{TypedHeader, http::HeaderName, response::IntoResponse};

use crate::headers::hello::Hello;

pub async fn response_headers() -> impl IntoResponse {
    let hello = Hello("hi from Axum".to_owned());

    ([("hello", "hello"), ("another", "header")], "hi there")
}