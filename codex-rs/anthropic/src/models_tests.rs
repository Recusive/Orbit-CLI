//! Tests for the Anthropic Models API client.

use crate::client::AnthropicAuth;
use crate::models::AnthropicModelsClient;
use crate::models::AnthropicModelsResponse;
use async_trait::async_trait;
use orbit_code_client::HttpTransport;
use orbit_code_client::Request;
use orbit_code_client::Response;
use orbit_code_client::StreamResponse;
use orbit_code_client::TransportError;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

/// Test transport that returns pre-queued responses and records requests.
struct TestTransport {
    responses: Mutex<Vec<Response>>,
    requests: Mutex<Vec<Request>>,
}

impl TestTransport {
    fn new(responses: Vec<Response>) -> Self {
        // Reverse so we can pop from the back efficiently.
        let mut responses = responses;
        responses.reverse();
        Self {
            responses: Mutex::new(responses),
            requests: Mutex::new(Vec::new()),
        }
    }

    fn recorded_requests(&self) -> Vec<Request> {
        self.requests.lock().expect("lock").clone()
    }
}

#[async_trait]
impl HttpTransport for TestTransport {
    async fn execute(&self, req: Request) -> Result<Response, TransportError> {
        self.requests.lock().expect("lock").push(req);
        let resp = self
            .responses
            .lock()
            .expect("lock")
            .pop()
            .expect("no more responses queued");
        Ok(resp)
    }

    async fn stream(&self, _req: Request) -> Result<StreamResponse, TransportError> {
        unreachable!("stream is not used by the models client")
    }
}

fn ok_response(body: &serde_json::Value) -> Response {
    Response {
        status: http::StatusCode::OK,
        headers: Default::default(),
        body: serde_json::to_vec(body).expect("serialize").into(),
    }
}

fn test_client(transport: Arc<TestTransport>) -> AnthropicModelsClient {
    AnthropicModelsClient::with_transport(
        transport,
        "https://api.anthropic.com".to_string(),
        Duration::from_secs(5),
    )
}

fn auth() -> AnthropicAuth {
    AnthropicAuth::ApiKey("test-key".to_string())
}

#[tokio::test]
async fn deserializes_single_page_response() {
    let body = serde_json::json!({
        "data": [{
            "id": "claude-opus-4-6",
            "display_name": "Claude Opus 4.6",
            "max_input_tokens": 1000000,
            "max_tokens": 128000,
            "capabilities": {
                "thinking": {
                    "supported": true,
                    "types": {
                        "enabled": { "supported": true },
                        "adaptive": { "supported": true }
                    }
                },
                "effort": {
                    "supported": true,
                    "low": { "supported": true },
                    "medium": { "supported": true },
                    "high": { "supported": true },
                    "max": { "supported": true }
                }
            }
        }],
        "has_more": false
    });
    let transport = Arc::new(TestTransport::new(vec![ok_response(&body)]));
    let client = test_client(transport);
    let models = client.list_models(&auth()).await.expect("list_models");

    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, "claude-opus-4-6");
    assert_eq!(models[0].max_input_tokens, 1_000_000);
    assert_eq!(models[0].max_output_tokens, 128_000);
    assert!(models[0].capabilities.thinking.types.adaptive.supported);
    assert!(models[0].capabilities.effort.max.supported);
}

#[tokio::test]
async fn handles_pagination() {
    let page1 = serde_json::json!({
        "data": [{"id": "claude-sonnet-4-6", "display_name": "Sonnet", "max_input_tokens": 200000, "max_tokens": 64000, "capabilities": {}}],
        "has_more": true,
        "last_id": "claude-sonnet-4-6"
    });
    let page2 = serde_json::json!({
        "data": [{"id": "claude-haiku-4-5-20251001", "display_name": "Haiku", "max_input_tokens": 200000, "max_tokens": 64000, "capabilities": {}}],
        "has_more": false
    });
    let transport = Arc::new(TestTransport::new(vec![
        ok_response(&page1),
        ok_response(&page2),
    ]));
    let client = test_client(transport);
    let models = client.list_models(&auth()).await.expect("list_models");

    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id, "claude-sonnet-4-6");
    assert_eq!(models[1].id, "claude-haiku-4-5-20251001");
}

#[tokio::test]
async fn api_key_auth_sets_header() {
    let body = serde_json::json!({"data": [], "has_more": false});
    let transport = Arc::new(TestTransport::new(vec![ok_response(&body)]));
    let client = test_client(transport.clone());
    let _ = client.list_models(&auth()).await.expect("list_models");

    let requests = transport.recorded_requests();
    assert_eq!(requests.len(), 1);
    let req = &requests[0];
    assert_eq!(
        req.headers
            .get("x-api-key")
            .expect("x-api-key")
            .to_str()
            .expect("str"),
        "test-key"
    );
    assert!(req.headers.get("authorization").is_none());
}

#[tokio::test]
async fn bearer_auth_sets_authorization_header() {
    let body = serde_json::json!({"data": [], "has_more": false});
    let transport = Arc::new(TestTransport::new(vec![ok_response(&body)]));
    let client = test_client(transport.clone());
    let _ = client
        .list_models(&AnthropicAuth::BearerToken("tok-123".to_string()))
        .await
        .expect("list_models");

    let requests = transport.recorded_requests();
    assert_eq!(requests.len(), 1);
    let req = &requests[0];
    assert_eq!(
        req.headers
            .get("authorization")
            .expect("authorization")
            .to_str()
            .expect("str"),
        "Bearer tok-123"
    );
    assert!(req.headers.get("x-api-key").is_none());
}

#[tokio::test]
async fn returns_error_on_401() {
    let transport = Arc::new(TestTransport::new(vec![Response {
        status: http::StatusCode::UNAUTHORIZED,
        headers: Default::default(),
        body: b"{\"error\": \"unauthorized\"}".to_vec().into(),
    }]));
    let client = test_client(transport);
    let err = client.list_models(&auth()).await.expect_err("should fail");

    assert!(err.is_unauthorized());
    assert!(!err.is_retryable());
}

#[tokio::test]
async fn returns_retryable_on_500() {
    let transport = Arc::new(TestTransport::new(vec![Response {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        headers: Default::default(),
        body: b"internal error".to_vec().into(),
    }]));
    let client = test_client(transport);
    let err = client.list_models(&auth()).await.expect_err("should fail");

    assert!(err.is_retryable());
    assert!(!err.is_unauthorized());
}

#[test]
fn deserializes_response_with_missing_capabilities() {
    let json = r#"{
        "data": [{"id": "claude-new-model", "display_name": "New Model"}],
        "has_more": false
    }"#;
    let resp: AnthropicModelsResponse = serde_json::from_str(json).expect("parse");
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].max_input_tokens, 0);
    assert_eq!(resp.data[0].max_output_tokens, 0);
    assert!(!resp.data[0].capabilities.thinking.supported);
    assert!(!resp.data[0].capabilities.effort.supported);
}
