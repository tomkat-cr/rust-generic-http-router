//! Defines the `HttpHandler` trait that all route handlers must implement.

use crate::request::HttpRequest;
use crate::response::HttpResponse;

/// A trait for handling HTTP requests.
///
/// Any struct that implements this trait can be registered as a handler
/// in the `Router`. The `Send` and `Sync` bounds are required to allow
//  the handler to be shared safely across threads in a concurrent server.
pub trait HttpHandler {
    /// Handles an incoming request and returns a response.
    ///
    /// # Arguments
    ///
    /// * `req` - An `HttpRequest` containing all the details of the request,
    ///           including headers, body, and parsed path parameters.
    ///
    /// # Returns
    ///
    /// An `HttpResponse` to be sent back to the client.
    fn handle(&self, req: HttpRequest) -> HttpResponse;
}
