//! Hyper-based implementation of the `http-client` crate.
//!
//! This crate provides [`HyperClient`], an HTTP client built on hyper 1.x.
//!
//! # Features
//!
//! - `rustls` - Enables HTTPS support via `hyper-rustls`.
//! - `json` - Enables JSON serialization/deserialization.
//!
//! # Example
//!
//! ```rust
//! use http_client::HttpClient;
//! use http_client_hyper::HttpHyperClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = HttpHyperClient::new();
//!
//!     let request = client.get("http://httpbin.org/get")
//!         .query("key", "value")
//!         .build();
//!
//!     let response = client.send(request).await.unwrap();
//!     println!("Status: {}", response.status);
//! }
//! ```

mod client;
mod connector;
mod error;
mod response;

pub use client::{HttpHyperClient, HyperClient};
pub use error::HyperError;
pub use response::HyperResponseBody;

#[cfg(feature = "rustls")]
pub use client::HttpsHyperClient;

// Re-export http-client traits for convenience
pub use http_client::{HttpClient, HttpError, HttpMethod, HttpRequest, HttpResponse, ResponseBody};
