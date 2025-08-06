//! Defines the custom error type for the router library.

use thiserror::Error;

/// Represents all possible errors that can occur in this library.
#[derive(Error, Debug)]
pub enum RouterError {
    /// Error originating from I/O operations (e.g., reading the config file).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Error originating from JSON deserialization.
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// Error from the `matchit` router, e.g., inserting a conflicting route.
    #[error("Routing error: {0}")]
    MatchIt(#[from] matchit::InsertError),
}
