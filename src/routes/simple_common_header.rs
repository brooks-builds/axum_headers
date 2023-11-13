use axum::{headers::UserAgent, TypedHeader};

pub async fn simple_common_header(TypedHeader(user_agent): TypedHeader<UserAgent>) {
    tracing::info!("user agent is: {user_agent}");
}