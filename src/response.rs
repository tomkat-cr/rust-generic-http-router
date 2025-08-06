//! Defines a custom `HttpResponse` struct for convenience.

use http::{Response, StatusCode, HeaderMap, HeaderValue};

/// A representation of an outgoing HTTP response.
///
/// Handlers create and return this struct. It can be easily converted into
/// a standard `http::Response<Vec<u8>>`.
#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Creates a new `HttpResponse`.
    pub fn new(status: StatusCode, body: Vec<u8>) -> Self {
        Self {
            status,
            headers: HeaderMap::new(),
            body,
        }
    }

    /// Creates a new `HttpResponse` with headers.
    pub fn with_headers(status: StatusCode, headers: HeaderMap, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    /// Adds a header to the response.
    pub fn add_header<K>(&mut self, name: K, value: &str) -> &mut Self 
    where
        K: http::header::IntoHeaderName,
    {
        if let Ok(header_value) = HeaderValue::from_str(value) {
            self.headers.insert(name, header_value);
        }
        self
    }
}

/// Allows converting our custom `HttpResponse` into the standard `http::Response`.
impl From<HttpResponse> for Response<Vec<u8>> {
    fn from(res: HttpResponse) -> Self {
        let mut response = Response::builder().status(res.status);
        
        // Add all headers to the response
        if let Some(headers) = response.headers_mut() {
            headers.extend(res.headers);
        }
        
        response.body(res.body)
            .unwrap() // This unwrap is safe as we control the inputs.
    }
}
