use core::future::Future;

/// An HTTP response from the server.
#[derive(Debug)]
pub struct HttpResponse<B> {
    /// The HTTP status code.
    pub status: u16,
    /// Response headers as key-value pairs.
    pub headers: Vec<(String, String)>,
    /// The response body.
    pub body: B,
}

impl<B> HttpResponse<B> {
    /// Returns true if the status code indicates success (2xx).
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Returns true if the status code indicates a client error (4xx).
    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    /// Returns true if the status code indicates a server error (5xx).
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }
}

/// Trait for response bodies that can be consumed.
///
/// Implementations should provide their own streaming body type and error handling for consumption
/// operations.
pub trait ResponseBody: Sized {
    /// The error type for body consumption operations.
    type Error;

    /// Consumes the body and returns it as raw bytes.
    fn into_bytes(self) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Consumes the body and deserializes it as JSON.
    #[cfg(feature = "json")]
    fn json<T: serde::de::DeserializeOwned + Send>(
        self,
    ) -> impl Future<Output = Result<T, Self::Error>> + Send
    where
        Self: Send,
        Self::Error: Send + From<serde_json::Error>,
    {
        async move {
            let bytes = self.into_bytes().await?;
            Ok(serde_json::from_slice(&bytes)?)
        }
    }
}
