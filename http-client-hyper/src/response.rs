use http_body_util::BodyExt;
use hyper::body::Incoming;

use crate::error::HyperError;

/// A wrapper around hyper's `Incoming` body that implements `ResponseBody`.
pub struct HyperResponseBody {
    inner: Incoming,
}

impl HyperResponseBody {
    /// Creates a new `HyperBody` from hyper's `Incoming` body.
    pub(crate) fn new(incoming: Incoming) -> Self {
        Self { inner: incoming }
    }
}

impl http_client::ResponseBody for HyperResponseBody {
    type Error = HyperError;

    async fn into_bytes(self) -> Result<Vec<u8>, Self::Error> {
        let collected = self
            .inner
            .collect()
            .await
            .map_err(|e| HyperError::Body(e.to_string()))?;
        Ok(collected.to_bytes().to_vec())
    }
}
