use hyper_util::client::legacy::connect::HttpConnector;

#[cfg(feature = "rustls")]
use hyper_rustls::HttpsConnector;

/// Creates an HTTP-only connector.
pub fn http_connector() -> HttpConnector {
    HttpConnector::new()
}

/// Creates an HTTPS connector with rustls.
///
/// This connector supports both HTTP and HTTPS URLs.
#[cfg(feature = "rustls")]
pub fn https_connector() -> HttpsConnector<HttpConnector> {
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("failed to load native root certificates")
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build()
}
