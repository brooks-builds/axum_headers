use axum::{headers::ContentType, TypedHeader};

pub async fn simple_header_response() -> (TypedHeader<ContentType>, &'static str) {
    (TypedHeader(ContentType::text()), "hello from Axum")
}
