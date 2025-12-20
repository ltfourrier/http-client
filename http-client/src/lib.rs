//! A client-agnostic HTTP client crate for LTFNet and Tiwind Software applications.
//!
//! This crate provides traits and data structures for making HTTP requests without being tied to a
//! specific HTTP client implementation.
//!
//! This means that crate, on its own, does nothing and is just an abstraction layer over an HTTP
//! crate (like `hyper`, etc...). At least one implementation crate should be used to use this
//! crate (like `http-client-hyper`, etc...).
//!
//! # Features
//!
//! - `json` - Enables automatic JSON serialization/deserialization support via serde.

mod client;
mod error;
mod method;
mod request;
mod response;

pub use client::HttpClient;
pub use error::HttpError;
pub use method::HttpMethod;
pub use request::{HttpRequest, HttpRequestBuilder};
pub use response::{HttpResponse, ResponseBody};
