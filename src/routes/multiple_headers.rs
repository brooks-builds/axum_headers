use axum::{http::HeaderMap, headers::{HeaderMapExt, UserAgent}};

use crate::headers::hello::Hello;

pub async fn multiple_headers(headers: HeaderMap) {
    let user_agent = headers.typed_get::<UserAgent>();
    let accept = headers.get("accept");
    let hello = headers.typed_get::<Hello>();

    tracing::info!("user agent is {user_agent:?}");
    tracing::info!("accept is: {accept:?}");
    tracing::info!("hello is: {hello:?}");
}