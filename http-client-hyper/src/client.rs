use bytes::Bytes;
use http_body_util::Full;
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::TokioExecutor,
};

#[cfg(feature = "rustls")]
use hyper_rustls::HttpsConnector;

use http_client::{HttpClient, HttpError, HttpMethod, HttpRequest, HttpResponse};

use crate::connector::http_connector;
use crate::error::HyperError;
use crate::response::HyperResponseBody;

#[cfg(feature = "rustls")]
use crate::connector::https_connector;

/// A hyper-based HTTP client.
///
/// The connector type `C` determines what protocols are supported:
/// - `HttpConnector`: Only HTTP (no TLS).
/// - `HttpsConnector<HttpConnector>`: Both HTTP and HTTPS (requires `rustls` feature).
pub struct HyperClient<C> {
    inner: Client<C, Full<Bytes>>,
}

/// Type alias for an HTTP-only client.
pub type HttpHyperClient = HyperClient<HttpConnector>;

/// Type alias for an HTTPS-capable client.
#[cfg(feature = "rustls")]
pub type HttpsHyperClient = HyperClient<HttpsConnector<HttpConnector>>;

impl HttpHyperClient {
    /// Creates a new HTTP-only client.
    ///
    /// This client can only connect to `http://` URLs.
    /// For HTTPS support, enable the `rustls` feature and use [`HttpsHyperClient::new`].
    pub fn new() -> Self {
        let connector = http_connector();
        let client = Client::builder(TokioExecutor::new()).build(connector);
        Self { inner: client }
    }
}

impl Default for HttpHyperClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "rustls")]
impl HttpsHyperClient {
    /// Creates a new HTTPS-capable client.
    ///
    /// This client can connect to both `http://` and `https://` URLs.
    pub fn new() -> Self {
        let connector = https_connector();
        let client = Client::builder(TokioExecutor::new()).build(connector);
        Self { inner: client }
    }
}

#[cfg(feature = "rustls")]
impl Default for HttpsHyperClient {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> HttpClient for HyperClient<C>
where
    C: hyper_util::client::legacy::connect::Connect + Clone + Send + Sync + 'static,
{
    type Body = HyperResponseBody;
    type Error = HyperError;

    async fn send(
        &self,
        request: HttpRequest,
    ) -> Result<HttpResponse<Self::Body>, HttpError<Self::Error>> {
        // Parse the URL
        let mut url = url::Url::parse(&request.url)
            .map_err(|e| HttpError::InvalidUrl(format!("{}: {}", request.url, e)))?;

        // Append query parameters
        {
            let mut query_pairs = url.query_pairs_mut();
            for (key, value) in &request.query {
                query_pairs.append_pair(key, value);
            }
        }

        // Convert method
        let method = match request.method {
            HttpMethod::Get => hyper::Method::GET,
            HttpMethod::Post => hyper::Method::POST,
            HttpMethod::Put => hyper::Method::PUT,
            HttpMethod::Patch => hyper::Method::PATCH,
            HttpMethod::Delete => hyper::Method::DELETE,
        };

        // Build the request
        let mut builder = hyper::Request::builder().method(method).uri(url.as_str());

        // Add headers
        for (key, value) in &request.headers {
            builder = builder.header(key.as_str(), value.as_str());
        }

        // Set body
        let body = match request.body {
            Some(bytes) => Full::new(Bytes::from(bytes)),
            None => Full::new(Bytes::new()),
        };

        let hyper_request = builder
            .body(body)
            .map_err(|e| HttpError::Client(HyperError::Http(e)))?;

        // Send request
        let response = self
            .inner
            .request(hyper_request)
            .await
            .map_err(|e| HttpError::Client(HyperError::Client(e)))?;

        // Convert response
        let status = response.status().as_u16();

        let headers: Vec<(String, String)> = response
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str().to_string(),
                    value.to_str().unwrap_or("").to_string(),
                )
            })
            .collect();

        let body = HyperResponseBody::new(response.into_body());

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}
