use axum::{
    headers::{ContentType, HeaderMapExt, UserAgent},
    http::HeaderMap,
};

pub async fn many_typed_headers(headers: HeaderMap) {
    let user_agent = headers.typed_get::<UserAgent>();
    let content_type = headers.typed_get::<ContentType>();

    tracing::info!("User agent: {user_agent:?}");
    tracing::info!("Content type: {content_type:?}");
}
