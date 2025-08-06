//! Defines the data structures for parsing the JSON configuration.

use http::Method;
use serde::{Deserialize, Deserializer};

/// Represents the top-level structure of the `routes.json` file.
#[derive(Deserialize, Debug)]
pub struct Config {
    pub endpoints: Vec<Endpoint>,
}

/// Represents a single endpoint definition in the configuration.
#[derive(Deserialize, Debug)]
pub struct Endpoint {
    #[serde(deserialize_with = "deserialize_method")]
    pub method: Method,
    pub path: String,
    pub controller: String,
    pub description: String,
}

/// Custom deserializer for `http::Method`.
///
/// `serde` doesn't know how to deserialize a string into a `http::Method` by default,
/// so we provide this helper function to do the conversion.
fn deserialize_method<'de, D>(deserializer: D) -> Result<Method, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // Convert known methods to their canonical form
    let method = match s.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        "CONNECT" => Method::CONNECT,
        "PATCH" => Method::PATCH,
        "TRACE" => Method::TRACE,
        _ => Method::from_bytes(s.as_bytes()).map_err(serde::de::Error::custom)?
    };
    Ok(method)
}
