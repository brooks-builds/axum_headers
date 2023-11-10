use axum::{headers::HeaderMapExt, http::HeaderMap};

use crate::headers::hello::Hello;

pub async fn custom_header(headers: HeaderMap) {
    let message = headers.typed_get::<Hello>();

    tracing::info!("message is: {message:?}");
}
