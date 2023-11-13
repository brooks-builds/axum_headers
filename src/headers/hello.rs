use axum::{headers::{Header, Error}, http::{HeaderName, HeaderValue}};

static HEADER_NAME: axum::http::HeaderName = HeaderName::from_static("hello");

#[derive(Debug)]
pub struct Hello(pub String);

impl Header for Hello {
    fn name() -> &'static axum::http::HeaderName {
        &HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue> {
        let value = values.next().ok_or_else(|| Error::invalid())?;
        let message = value.to_str().map_err(|_| Error::invalid())?;

        Ok(Self(message.to_owned()))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(HeaderValue::from_str(self.0.as_str()).expect("Error encoding header")))
    }
}