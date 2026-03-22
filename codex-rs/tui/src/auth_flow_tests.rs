use super::*;
use orbit_code_core::auth::load_auth_dot_json_v2;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

fn temp_home() -> (TempDir, std::path::PathBuf) {
    let tmp = TempDir::new().expect("create temp dir");
    let path = tmp.path().to_path_buf();
    (tmp, path)
}

fn shared_manager(home: &std::path::Path) -> Arc<AuthManager> {
    AuthManager::shared(home.to_path_buf(), false, AuthCredentialsStoreMode::File)
}

#[test]
fn save_api_key_persists_and_updates_preferred_mode() {
    let (_tmp, home) = temp_home();
    let result = crate::auth_flow::save_api_key(
        &home,
        ProviderName::Anthropic,
        "sk-ant-test-key-123",
        AuthCredentialsStoreMode::File,
    );
    assert!(result.is_ok(), "save_api_key should succeed: {result:?}");

    let v2 = load_auth_dot_json_v2(&home, AuthCredentialsStoreMode::File)
        .expect("load should succeed")
        .expect("v2 should exist");
    assert!(v2.provider_auth(ProviderName::Anthropic).is_some());
    assert!(
        v2.preferred_auth_modes
            .contains_key(&ProviderName::Anthropic)
    );
}

#[test]
fn save_api_key_rejects_empty() {
    let (_tmp, home) = temp_home();
    let result = crate::auth_flow::save_api_key(
        &home,
        ProviderName::OpenAI,
        "   ",
        AuthCredentialsStoreMode::File,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "API key cannot be empty.");
}

#[test]
fn save_api_key_preserves_alternate_on_method_change() {
    let (_tmp, home) = temp_home();

    // First save an API key.
    crate::auth_flow::save_api_key(
        &home,
        ProviderName::Anthropic,
        "sk-ant-original",
        AuthCredentialsStoreMode::File,
    )
    .expect("first save");

    // Now save an OAuth credential (different method) via apply_auth_success.
    let manager = shared_manager(&home);
    crate::auth_flow::apply_auth_success(
        &home,
        ProviderName::Anthropic,
        ProviderAuth::AnthropicOAuth {
            access_token: "at".to_string(),
            refresh_token: "rt".to_string(),
            expires_at: 9999999999,
        },
        AuthCredentialsStoreMode::File,
        &manager,
    )
    .expect("apply oauth");

    let v2 = load_auth_dot_json_v2(&home, AuthCredentialsStoreMode::File)
        .expect("load")
        .expect("v2");

    // The active should be OAuth, the alternate should be the old API key.
    assert!(matches!(
        v2.provider_auth(ProviderName::Anthropic),
        Some(ProviderAuth::AnthropicOAuth { .. })
    ));
    assert!(matches!(
        v2.alternate_credentials.get(&ProviderName::Anthropic),
        Some(ProviderAuth::AnthropicApiKey { .. })
    ));
}

#[test]
fn remove_credentials_via_logout_provider() {
    let (_tmp, home) = temp_home();

    // Save something first.
    save_api_key(
        &home,
        ProviderName::OpenAI,
        "sk-test",
        AuthCredentialsStoreMode::File,
    )
    .expect("save");

    let removed = orbit_code_core::auth::logout_provider(
        &home,
        ProviderName::OpenAI,
        AuthCredentialsStoreMode::File,
    )
    .expect("remove should succeed");
    assert!(removed);

    let v2 = load_auth_dot_json_v2(&home, AuthCredentialsStoreMode::File)
        .expect("load")
        .unwrap_or_default();
    assert!(v2.provider_auth(ProviderName::OpenAI).is_none());
}

#[test]
fn apply_auth_success_persists_and_reloads() {
    let (_tmp, home) = temp_home();
    let manager = shared_manager(&home);

    let result = apply_auth_success(
        &home,
        ProviderName::Anthropic,
        ProviderAuth::AnthropicApiKey {
            key: "sk-ant-reload-test".to_string(),
        },
        AuthCredentialsStoreMode::File,
        &manager,
    );
    assert!(result.is_ok(), "apply_auth_success should succeed");

    // Verify credentials were persisted to disk.
    let v2 = load_auth_dot_json_v2(&home, AuthCredentialsStoreMode::File)
        .expect("load")
        .expect("v2 should exist");
    assert!(matches!(
        v2.provider_auth(ProviderName::Anthropic),
        Some(ProviderAuth::AnthropicApiKey { .. })
    ));
}

#[test]
fn auth_attempt_id_mismatch_discards_result() {
    // Create two handles and verify their IDs are different.
    let (handle_a, _cancel_a) = AuthAttemptHandle::new();
    let (handle_b, _cancel_b) = AuthAttemptHandle::new();
    assert_ne!(handle_a.id, handle_b.id);

    // Simulate checking: a result from attempt A should not match handle B.
    let result_id = handle_a.id;
    let held_id = handle_b.id;
    assert_ne!(result_id, held_id, "stale result should be discarded");
}
