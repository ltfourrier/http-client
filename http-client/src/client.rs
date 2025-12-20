use core::future::Future;

use crate::error::HttpError;
use crate::method::HttpMethod;
use crate::request::{HttpRequest, HttpRequestBuilder};
use crate::response::{HttpResponse, ResponseBody};

/// Trait for HTTP clients that can send requests.
pub trait HttpClient {
    /// The response body type returned by the underlying HTTP crate.
    type Body: ResponseBody;

    /// The error type returned by the underlying HTTP crate.
    type Error;

    /// Creates a GET request builder for the given URL path.
    fn get(&self, url_path: impl Into<String>) -> HttpRequestBuilder {
        HttpRequestBuilder::new(HttpMethod::Get, url_path)
    }

    /// Creates a POST request builder for the given URL path.
    fn post(&self, url_path: impl Into<String>) -> HttpRequestBuilder {
        HttpRequestBuilder::new(HttpMethod::Post, url_path)
    }

    /// Creates a PUT request builder for the given URL path.
    fn put(&self, url_path: impl Into<String>) -> HttpRequestBuilder {
        HttpRequestBuilder::new(HttpMethod::Put, url_path)
    }

    /// Creates a PATCH request builder for the given URL path.
    fn patch(&self, url_path: impl Into<String>) -> HttpRequestBuilder {
        HttpRequestBuilder::new(HttpMethod::Patch, url_path)
    }

    /// Creates a DELETE request builder for the given URL path.
    fn delete(&self, url_path: impl Into<String>) -> HttpRequestBuilder {
        HttpRequestBuilder::new(HttpMethod::Delete, url_path)
    }

    /// Sends an HTTP request and returns the response.
    fn send(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse<Self::Body>, HttpError<Self::Error>>> + Send;
}
