//! Tests for Anthropic API → ModelInfo mapping.

use orbit_code_anthropic::AnthropicCapabilities;
use orbit_code_anthropic::AnthropicModelInfo;
use orbit_code_protocol::openai_models::ApplyPatchToolType;
use orbit_code_protocol::openai_models::ConfigShellToolType;
use orbit_code_protocol::openai_models::ModelVisibility;
use orbit_code_protocol::openai_models::ThinkingStyle;
use pretty_assertions::assert_eq;

use super::merge_anthropic_capabilities;
use super::model_info_from_anthropic_api;
use crate::models_manager::model_info::model_info_from_slug;

fn make_api_model(id: &str, max_input: i64, max_output: i64) -> AnthropicModelInfo {
    AnthropicModelInfo {
        id: id.to_string(),
        display_name: format!("Test {id}"),
        max_input_tokens: max_input,
        max_output_tokens: max_output,
        capabilities: AnthropicCapabilities::default(),
    }
}

fn make_adaptive_api_model(id: &str) -> AnthropicModelInfo {
    let mut model = make_api_model(id, 1_000_000, 128_000);
    model.capabilities.thinking.supported = true;
    model.capabilities.thinking.types.adaptive.supported = true;
    model.capabilities.effort.supported = true;
    model.capabilities.effort.max.supported = true;
    model
}

#[test]
fn merge_updates_context_window_from_api() {
    let bundled = model_info_from_slug("claude-test");
    let api = make_api_model("claude-test", 500_000, 0);
    let merged = merge_anthropic_capabilities(&bundled, &api);
    assert_eq!(merged.context_window, Some(500_000));
}

#[test]
fn merge_keeps_bundled_context_window_when_api_is_zero() {
    let bundled = model_info_from_slug("claude-test");
    let api = make_api_model("claude-test", 0, 0);
    let merged = merge_anthropic_capabilities(&bundled, &api);
    // Bundled fallback context window (from model_info_from_slug) is preserved.
    assert_eq!(merged.context_window, Some(272_000));
}

#[test]
fn merge_sets_adaptive_thinking_from_api() {
    let bundled = model_info_from_slug("claude-test");
    let api = make_adaptive_api_model("claude-test");
    let merged = merge_anthropic_capabilities(&bundled, &api);
    assert_eq!(merged.thinking_style, ThinkingStyle::Adaptive);
    assert!(merged.supports_effort);
    assert!(merged.supports_effort_max);
}

#[test]
fn merge_preserves_requires_extended_context_beta_from_bundled() {
    let mut bundled = model_info_from_slug("claude-test");
    bundled.requires_extended_context_beta = true;
    let api = make_api_model("claude-test", 1_000_000, 128_000);
    let merged = merge_anthropic_capabilities(&bundled, &api);
    assert!(merged.requires_extended_context_beta);
}

#[test]
fn merge_preserves_base_instructions_from_bundled() {
    let bundled = model_info_from_slug("claude-test");
    let original_instructions = bundled.base_instructions.clone();
    let api = make_adaptive_api_model("claude-test");
    let merged = merge_anthropic_capabilities(&bundled, &api);
    assert_eq!(merged.base_instructions, original_instructions);
}

#[test]
fn unknown_model_gets_claude_template() {
    let api = make_adaptive_api_model("claude-new-model");
    let info = model_info_from_anthropic_api(&api);
    assert_eq!(info.slug, "claude-new-model");
    assert_eq!(info.display_name, "Test claude-new-model");
    assert_eq!(info.visibility, ModelVisibility::List);
    assert_eq!(info.shell_type, ConfigShellToolType::ShellCommand);
    assert_eq!(
        info.apply_patch_tool_type,
        Some(ApplyPatchToolType::Freeform)
    );
    assert!(info.supported_in_api);
    assert_eq!(info.context_window, Some(1_000_000));
    assert_eq!(info.max_output_tokens, Some(128_000));
    assert_eq!(info.thinking_style, ThinkingStyle::Adaptive);
    assert!(info.supports_effort);
    assert!(info.supports_effort_max);
    assert!(!info.requires_extended_context_beta); // conservative default
    assert!(!info.used_fallback_model_metadata);
}

#[test]
fn unknown_model_with_zero_tokens_gets_defaults() {
    let api = make_api_model("claude-zero", 0, 0);
    let info = model_info_from_anthropic_api(&api);
    assert_eq!(info.context_window, Some(200_000)); // safe Claude default
    assert_eq!(info.max_output_tokens, None);
    assert_eq!(info.thinking_style, ThinkingStyle::Budgeted);
}

#[test]
fn unknown_model_uses_api_display_name() {
    let api = make_api_model("claude-fancy", 200_000, 64_000);
    let info = model_info_from_anthropic_api(&api);
    assert_eq!(info.display_name, "Test claude-fancy");
}

#[test]
fn unknown_model_with_empty_display_name_falls_back_to_id() {
    let mut api = make_api_model("claude-x", 200_000, 64_000);
    api.display_name = String::new();
    let info = model_info_from_anthropic_api(&api);
    assert_eq!(info.display_name, "claude-x");
}
