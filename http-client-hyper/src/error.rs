use core::fmt;

/// Errors that can occur when using the Hyper HTTP client.
#[derive(Debug)]
pub enum HyperError {
    /// Error from hyper's HTTP layer.
    Hyper(hyper::Error),
    /// Error from hyper-util's client layer.
    Client(hyper_util::client::legacy::Error),
    /// Error building the HTTP request.
    Http(hyper::http::Error),
    /// Error reading the response body.
    Body(String),
    /// JSON deserialization error.
    #[cfg(feature = "json")]
    Json(serde_json::Error),
}

impl fmt::Display for HyperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HyperError::Hyper(e) => write!(f, "hyper error: {e}"),
            HyperError::Client(e) => write!(f, "client error: {e}"),
            HyperError::Http(e) => write!(f, "http error: {e}"),
            HyperError::Body(msg) => write!(f, "body error: {msg}"),
            #[cfg(feature = "json")]
            HyperError::Json(e) => write!(f, "json error: {e}"),
        }
    }
}

impl std::error::Error for HyperError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HyperError::Hyper(e) => Some(e),
            HyperError::Client(e) => Some(e),
            HyperError::Http(e) => Some(e),
            HyperError::Body(_) => None,
            #[cfg(feature = "json")]
            HyperError::Json(e) => Some(e),
        }
    }
}

impl From<hyper::Error> for HyperError {
    fn from(err: hyper::Error) -> Self {
        HyperError::Hyper(err)
    }
}

impl From<hyper_util::client::legacy::Error> for HyperError {
    fn from(err: hyper_util::client::legacy::Error) -> Self {
        HyperError::Client(err)
    }
}

impl From<hyper::http::Error> for HyperError {
    fn from(err: hyper::http::Error) -> Self {
        HyperError::Http(err)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for HyperError {
    fn from(err: serde_json::Error) -> Self {
        HyperError::Json(err)
    }
}
