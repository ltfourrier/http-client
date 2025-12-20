use http_client::{HttpClient, HttpError, ResponseBody};
use http_client_hyper::{HttpHyperClient, HyperError};
use serde::Deserialize;
use testcontainers::{
    core::wait::HttpWaitStrategy,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};

async fn start_httpbin() -> (testcontainers::ContainerAsync<GenericImage>, String) {
    let container = GenericImage::new("kennethreitz/httpbin", "latest")
        .with_exposed_port(80.tcp())
        .with_wait_for(WaitFor::http(
            HttpWaitStrategy::new("/get").with_expected_status_code(200u16),
        ))
        .start()
        .await
        .expect("Failed to start httpbin container");

    let port = container.get_host_port_ipv4(80).await.unwrap();
    let base_url = format!("http://127.0.0.1:{}", port);

    (container, base_url)
}

#[derive(Debug, Deserialize)]
struct HttpbinResponse {
    url: String,
    #[serde(default)]
    args: std::collections::HashMap<String, String>,
    #[serde(default)]
    headers: std::collections::HashMap<String, String>,
    #[serde(default)]
    data: String,
    #[serde(default)]
    json: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct HttpbinHeadersResponse {
    headers: std::collections::HashMap<String, String>,
}

#[tokio::test]
async fn test_get_request() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client.get(format!("{}/get", base_url)).build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());
    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn test_post_request() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client
        .post(format!("{}/post", base_url))
        .body("test body")
        .build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());
    assert_eq!(response.status, 200);

    let body_bytes = response.body.into_bytes().await.unwrap();
    let body: HttpbinResponse = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body.data, "test body");
}

#[tokio::test]
async fn test_put_request() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client
        .put(format!("{}/put", base_url))
        .body("put body")
        .build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());
    assert_eq!(response.status, 200);

    let body_bytes = response.body.into_bytes().await.unwrap();
    let body: HttpbinResponse = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body.data, "put body");
}

#[tokio::test]
async fn test_patch_request() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client
        .patch(format!("{}/patch", base_url))
        .body("patch body")
        .build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());
    assert_eq!(response.status, 200);

    let body_bytes = response.body.into_bytes().await.unwrap();
    let body: HttpbinResponse = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body.data, "patch body");
}

#[tokio::test]
async fn test_delete_request() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client.delete(format!("{}/delete", base_url)).build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());
    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn test_query_parameters() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client
        .get(format!("{}/get", base_url))
        .query("foo", "bar")
        .query("baz", "qux")
        .build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());

    let body_bytes = response.body.into_bytes().await.unwrap();
    let body: HttpbinResponse = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body.args.get("foo"), Some(&"bar".to_string()));
    assert_eq!(body.args.get("baz"), Some(&"qux".to_string()));
}

#[tokio::test]
async fn test_query_parameters_with_special_characters() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client
        .get(format!("{}/get", base_url))
        .query("key", "value with spaces")
        .query("special", "a=b&c=d")
        .build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());

    let body_bytes = response.body.into_bytes().await.unwrap();
    let body: HttpbinResponse = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body.args.get("key"), Some(&"value with spaces".to_string()));
    assert_eq!(body.args.get("special"), Some(&"a=b&c=d".to_string()));
}

#[tokio::test]
async fn test_custom_headers() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client
        .get(format!("{}/headers", base_url))
        .header("X-Custom-Header", "custom-value")
        .header("X-Another-Header", "another-value")
        .build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());

    let body_bytes = response.body.into_bytes().await.unwrap();
    let body: HttpbinHeadersResponse = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(
        body.headers.get("X-Custom-Header"),
        Some(&"custom-value".to_string())
    );
    assert_eq!(
        body.headers.get("X-Another-Header"),
        Some(&"another-value".to_string())
    );
}

#[tokio::test]
async fn test_response_headers() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client
        .get(format!("{}/response-headers", base_url))
        .query("X-Test-Header", "test-value")
        .build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());

    let header = response
        .headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("x-test-header"));
    assert!(header.is_some());
    assert_eq!(header.unwrap().1, "test-value");
}

#[tokio::test]
async fn test_post_raw_body() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let body_content = b"raw binary content \x00\x01\x02";
    let request = client
        .post(format!("{}/post", base_url))
        .body(body_content.to_vec())
        .build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());
}

#[tokio::test]
async fn test_response_status_codes() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    // Test 200
    let request = client.get(format!("{}/status/200", base_url)).build();
    let response = client.send(request).await.unwrap();
    assert_eq!(response.status, 200);
    assert!(response.is_success());

    // Test 201
    let request = client.get(format!("{}/status/201", base_url)).build();
    let response = client.send(request).await.unwrap();
    assert_eq!(response.status, 201);
    assert!(response.is_success());

    // Test 400
    let request = client.get(format!("{}/status/400", base_url)).build();
    let response = client.send(request).await.unwrap();
    assert_eq!(response.status, 400);
    assert!(response.is_client_error());

    // Test 404
    let request = client.get(format!("{}/status/404", base_url)).build();
    let response = client.send(request).await.unwrap();
    assert_eq!(response.status, 404);
    assert!(response.is_client_error());

    // Test 500
    let request = client.get(format!("{}/status/500", base_url)).build();
    let response = client.send(request).await.unwrap();
    assert_eq!(response.status, 500);
    assert!(response.is_server_error());
}

#[tokio::test]
async fn test_response_body_bytes() {
    let (_, base_url) = start_httpbin().await;
    let client = HttpHyperClient::new();

    let request = client.get(format!("{}/bytes/100", base_url)).build();
    let response = client.send(request).await.unwrap();

    assert!(response.is_success());

    let bytes = response.body.into_bytes().await.unwrap();
    assert_eq!(bytes.len(), 100);
}

#[tokio::test]
async fn test_invalid_url() {
    let client = HttpHyperClient::new();

    let request = client.get("not-a-valid-url").build();
    let result = client.send(request).await;

    match result {
        Err(HttpError::InvalidUrl(_)) => {}
        Err(other) => panic!("Expected InvalidUrl error, got: {:?}", other),
        Ok(_) => panic!("Expected error, got Ok"),
    }
}

#[tokio::test]
async fn test_connection_refused() {
    let client = HttpHyperClient::new();

    // Try to connect to a port that's not listening
    let request = client.get("http://127.0.0.1:59999/get").build();
    let result = client.send(request).await;

    match result {
        Err(HttpError::Client(HyperError::Client(_))) => {}
        Err(other) => panic!("Expected Client error, got: {:?}", other),
        Ok(_) => panic!("Expected error, got Ok"),
    }
}

#[cfg(feature = "json")]
mod json_tests {
    use super::*;

    #[derive(Debug, serde::Serialize, Deserialize, PartialEq)]
    struct TestPayload {
        name: String,
        value: i32,
    }

    #[tokio::test]
    async fn test_json_response() {
        let (_, base_url) = start_httpbin().await;
        let client = HttpHyperClient::new();

        let request = client.get(format!("{}/get", base_url)).build();
        let response = client.send(request).await.unwrap();

        assert!(response.is_success());

        let body: HttpbinResponse = response.body.json().await.unwrap();
        assert!(body.url.contains("/get"));
    }

    #[tokio::test]
    async fn test_post_json_body() {
        let (_, base_url) = start_httpbin().await;
        let client = HttpHyperClient::new();

        let payload = TestPayload {
            name: "test".to_string(),
            value: 42,
        };

        let request = client
            .post(format!("{}/post", base_url))
            .json(&payload)
            .unwrap()
            .build();
        let response = client.send(request).await.unwrap();

        assert!(response.is_success());

        let body: HttpbinResponse = response.body.json().await.unwrap();
        let received: TestPayload = serde_json::from_value(body.json.unwrap()).unwrap();
        assert_eq!(received, payload);
    }

    #[tokio::test]
    async fn test_json_content_type_header() {
        let (_, base_url) = start_httpbin().await;
        let client = HttpHyperClient::new();

        let payload = TestPayload {
            name: "test".to_string(),
            value: 42,
        };

        let request = client
            .post(format!("{}/post", base_url))
            .json(&payload)
            .unwrap()
            .build();
        let response = client.send(request).await.unwrap();

        let body_bytes = response.body.into_bytes().await.unwrap();
        let body: HttpbinResponse = serde_json::from_slice(&body_bytes).unwrap();

        // httpbin echoes headers back - check that Content-Type was set
        let content_type = body.headers.get("Content-Type");
        assert_eq!(content_type, Some(&"application/json".to_string()));
    }
}
