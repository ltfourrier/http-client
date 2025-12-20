use core::fmt;

/// Errors that can occur when building and sending HTTP requests.
#[derive(Debug)]
pub enum HttpError<E> {
    /// The URL provided was invalid.
    InvalidUrl(String),

    /// An error occurred during JSON serialization.
    #[cfg(feature = "json")]
    Serialization(serde_json::Error),

    /// An error from the underlying HTTP client implementation.
    Client(E),
}

impl<E: fmt::Display> fmt::Display for HttpError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::InvalidUrl(url) => write!(f, "invalid URL: {url}"),
            #[cfg(feature = "json")]
            HttpError::Serialization(err) => write!(f, "JSON serialization error: {err}"),
            HttpError::Client(err) => write!(f, "client error: {err}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for HttpError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HttpError::InvalidUrl(_) => None,
            #[cfg(feature = "json")]
            HttpError::Serialization(err) => Some(err),
            HttpError::Client(err) => Some(err),
        }
    }
}

impl<E> From<E> for HttpError<E> {
    fn from(value: E) -> Self {
        HttpError::Client(value)
    }
}
