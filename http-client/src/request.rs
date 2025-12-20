use crate::method::HttpMethod;

/// An HTTP request ready to be sent.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// The HTTP method (GET, POST, etc.).
    pub method: HttpMethod,
    /// The base URL for the request.
    pub url: String,
    /// Query parameters as key-value pairs so that multiple query parameters with the same name
    /// can be added.
    pub query: Vec<(String, String)>,
    /// HTTP headers as key-value pairs so that multiple headers with the same key can be added.
    pub headers: Vec<(String, String)>,
    /// Optional request body.
    pub body: Option<Vec<u8>>,
}

/// Builder for constructing HTTP requests.
#[derive(Debug, Clone)]
pub struct HttpRequestBuilder {
    method: HttpMethod,
    url: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

impl HttpRequestBuilder {
    /// Creates a new request builder with the given method and base URL.
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            query: Vec::new(),
            headers: Vec::new(),
            body: None,
        }
    }

    /// Appends a path segment to the URL.
    ///
    /// The segment will be appended with a `/` separator if needed.
    pub fn path(mut self, segment: impl AsRef<str>) -> Self {
        let segment = segment.as_ref();
        if !self.url.ends_with('/') && !segment.starts_with('/') {
            self.url.push('/');
        }
        self.url.push_str(segment.trim_start_matches('/'));
        self
    }

    /// Adds a query parameter.
    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    /// Adds an HTTP header.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// Sets the request body as raw bytes.
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Sets the request body as JSON, serializing the provided value.
    ///
    /// This also sets the `Content-Type` header to `application/json`.
    #[cfg(feature = "json")]
    pub fn json<T: serde::Serialize>(mut self, value: &T) -> Result<Self, serde_json::Error> {
        let json_bytes = serde_json::to_vec(value)?;
        self.body = Some(json_bytes);
        self.headers
            .push(("Content-Type".to_string(), "application/json".to_string()));
        Ok(self)
    }

    /// Builds the final HTTP request.
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            query: self.query,
            headers: self.headers,
            body: self.body,
        }
    }
}
