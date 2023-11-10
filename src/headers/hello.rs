use axum::{
    headers::{Error, Header},
    http::{HeaderName, HeaderValue},
};

// We need the header name to be a static so that we can return a reference in the name method below. As a static it is guaranteed to live for the entire life of the program.
static HEADER_NAME: HeaderName = HeaderName::from_static("hello");

#[derive(Debug)]
pub struct Hello(pub String);

impl Header for Hello {
    fn name() -> &'static axum::http::HeaderName {
        &HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let value = values.next().ok_or_else(|| Error::invalid())?;
        let message = value.to_str().map_err(|_| Error::invalid())?;
        Ok(Self(message.to_owned()))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        let value = HeaderValue::from_str(&self.0).unwrap();
        values.extend(std::iter::once(value));
    }
}
