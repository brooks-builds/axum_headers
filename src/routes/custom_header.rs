use axum::{
    headers::{Header, HeaderMapExt},
    http::HeaderMap,
    response::IntoResponse,
};

use crate::headers::hello::Hello;

pub async fn custom_header(headers: HeaderMap) -> impl IntoResponse {
    let message = headers.typed_get::<Hello>();

    tracing::info!("message is: {message:?}");

    let mut header_map = HeaderMap::new();
    header_map.typed_insert::<Hello>(Hello("Axum says hi!!!".to_owned()));

    header_map
}
