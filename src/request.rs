//! Defines a custom `HttpRequest` struct that wraps the standard `http::Request`
//! and includes parsed path parameters.

use http::{HeaderMap, Method, Uri};
use std::collections::HashMap;

/// A representation of an incoming HTTP request.
///
/// This struct is passed to `HttpHandler` implementations. It provides easy
/// access to all parts of the request, including path parameters extracted
//  by the router.
#[derive(Debug)]
pub struct HttpRequest {
    pub uri: Uri,
    pub method: Method,
    pub headers: HeaderMap,
    /// Path parameters extracted from the URL (e.g., `:id` from `/users/:id`).
    pub params: HashMap<String, String>,
    pub body: Vec<u8>,
}
