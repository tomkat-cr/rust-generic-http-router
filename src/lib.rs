//! # Generic HTTP Router
//!
//! A flexible, generic HTTP router for Rust that loads routing definitions
//! from a JSON file. This library allows you to decouple your routing logic
//! from your application code.

use http::{Method, Request, Response, StatusCode};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// Publicly export modules and key types for easy access by library users.
pub mod config;
pub mod error;
pub mod handler;
pub mod request;
pub mod response;

use crate::config::{Config, Endpoint};
pub use crate::error::RouterError;
pub use crate::handler::HttpHandler;
pub use crate::request::HttpRequest;
pub use crate::response::HttpResponse;

/// The main router struct.
///
/// It holds the routing tables and the registered handlers. It is the primary
/// entry point for the library.
pub struct Router {
    /// A map from HTTP methods to a radix tree (`matchit::Router`) for routing.
    /// Each tree stores paths for a specific method and maps them to a controller name.
    trees: HashMap<Method, matchit::Router<String>>,
    /// A map from controller names (from the JSON config) to actual handler implementations.
    /// This allows for dynamic dispatch to the correct handler at runtime.
    handlers: HashMap<String, Box<dyn HttpHandler + Send + Sync>>,
}

impl Router {
    /// Creates a new `Router` by loading and parsing a JSON configuration file.
    ///
    /// # Arguments
    ///
    /// * `config_path` - A path to the JSON file containing the route definitions.
    ///
    /// # Errors
    ///
    /// Returns a `RouterError` if the file cannot be opened, read, or parsed, or if
    /// there's an issue inserting a route into the routing tree.
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self, RouterError> {
        // Open and parse the JSON configuration file.
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;

        let mut trees = HashMap::<Method, matchit::Router<String>>::new();

        // Populate the routing trees from the parsed configuration.
        for endpoint in config.endpoints {
            let Endpoint {
                method,
                path,
                controller,
                ..
            } = endpoint;

            // Get the tree for the current HTTP method, or create it if it doesn't exist.
            let method_str = method.as_str().to_string();
            // let tree = trees.entry(method).or_insert_with(matchit::Router::new);
            let tree = trees.entry(method).or_default();

            // Insert the route into the tree. The path is the key, and the controller
            // name is the value.
            tree.insert(path.clone(), controller.clone())?;
            eprintln!("Registered route: {method_str} {path} -> {controller}");
        }

        Ok(Self {
            trees,
            handlers: HashMap::new(),
        })
    }

    /// Registers a handler for a given controller name.
    ///
    /// The `controller_name` must exactly match the `controller` string specified
    /// in the `routes.json` file.
    ///
    /// # Arguments
    ///
    /// * `controller_name` - The name of the controller to register.
    /// * `handler` - A boxed, dynamically-dispatchable `HttpHandler` implementation.
    pub fn register<S: Into<String>>(
        &mut self,
        controller_name: S,
        handler: Box<dyn HttpHandler + Send + Sync>,
    ) {
        self.handlers.insert(controller_name.into(), handler);
    }

    /// Routes an incoming HTTP request to the appropriate handler.
    ///
    /// This is the main method that performs the routing logic.
    ///
    /// # Arguments
    ///
    /// * `req` - The incoming `http::Request`. The body is expected to be `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// An `http::Response` with a `Vec<u8>` body, produced by the matched handler
    /// or an appropriate HTTP error response.
    pub fn route(&self, req: Request<Vec<u8>>) -> Response<Vec<u8>> {
        let path = req.uri().path();
        let method = req.method();
        eprintln!("Processing request: {method} {path}");
        eprintln!(
            "Available methods: {:?}",
            self.trees.keys().collect::<Vec<_>>()
        );

        // First check if the path exists in any method's tree
        let mut allowed_methods = Vec::new();
        for (tree_method, tree) in &self.trees {
            if tree.at(path).is_ok() {
                allowed_methods.push(tree_method);
            }
        }

        // If we found the path in some trees but not for this method, return 405
        if !allowed_methods.is_empty() && !allowed_methods.contains(&method) {
            let mut response = HttpResponse::new(StatusCode::METHOD_NOT_ALLOWED, Vec::new());
            let allow_header = allowed_methods
                .iter()
                .map(|m| m.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            response.add_header(http::header::ALLOW, &allow_header);
            return response.into();
        }

        // Get the tree for the current method
        let tree = match self.trees.get(method) {
            Some(tree) => tree,
            // If no routes are defined for this method and we didn't find the path
            // in any other method's tree, it's a 404 Not Found
            None => return HttpResponse::new(StatusCode::NOT_FOUND, Vec::new()).into(),
        };

        // Attempt to match the request's path against the tree.
        match tree.at(path) {
            // A route was successfully matched.
            Ok(match_result) => {
                let controller_name = match_result.value;

                // Check if a handler has been registered for this controller name.
                match self.handlers.get(controller_name) {
                    Some(handler) => {
                        // A handler exists. Extract path parameters and create our custom HttpRequest.
                        let params: HashMap<String, String> = match_result
                            .params
                            .iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect();

                        let (parts, body) = req.into_parts();
                        let custom_req = HttpRequest {
                            uri: parts.uri,
                            method: parts.method,
                            headers: parts.headers,
                            params,
                            body,
                        };

                        // Invoke the handler and return its response.
                        handler.handle(custom_req).into()
                    }
                    // The route is in the JSON, but no handler was registered.
                    // This is a server misconfiguration.
                    None => {
                        let body =
                            format!("Error: Handler for '{controller_name}' is not implemented.");
                        HttpResponse::new(StatusCode::NOT_IMPLEMENTED, body.into_bytes()).into()
                    }
                }
            }
            // No route matched the path.
            Err(_) => HttpResponse::new(StatusCode::NOT_FOUND, Vec::new()).into(),
        }
    }
}
