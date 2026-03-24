//! Maps Anthropic `/v1/models` API responses to `ModelInfo` entries.
//!
//! Two entry points:
//! - [`merge_anthropic_capabilities`]: enriches an existing bundled `ModelInfo` with
//!   API-reported capabilities (context window, output tokens, thinking, effort).
//! - [`model_info_from_anthropic_api`]: creates a Claude-specific `ModelInfo` for
//!   models discovered via the API that have no bundled catalog entry.

use orbit_code_anthropic::AnthropicModelInfo;
use orbit_code_protocol::config_types::ReasoningSummary;
use orbit_code_protocol::openai_models::ApplyPatchToolType;
use orbit_code_protocol::openai_models::ConfigShellToolType;
use orbit_code_protocol::openai_models::ModelInfo;
use orbit_code_protocol::openai_models::ModelVisibility;
use orbit_code_protocol::openai_models::ThinkingStyle;
use orbit_code_protocol::openai_models::TruncationPolicyConfig;
use orbit_code_protocol::openai_models::WebSearchToolType;
use orbit_code_protocol::openai_models::default_input_modalities;

use super::model_info::BASE_INSTRUCTIONS;

/// Merge API-reported capabilities into an existing bundled `ModelInfo`.
///
/// API values override bundled values for numeric fields (context window,
/// output tokens) when the API reports a positive value. Behavior fields
/// (instructions, personality, truncation policy) are always kept from the
/// bundled entry. `requires_extended_context_beta` is also kept from bundled
/// because it is a runtime policy flag, not API-advertised.
pub(crate) fn merge_anthropic_capabilities(
    bundled: &ModelInfo,
    api: &AnthropicModelInfo,
) -> ModelInfo {
    let mut merged = bundled.clone();

    if api.max_input_tokens > 0 {
        merged.context_window = Some(api.max_input_tokens);
    }
    if api.max_output_tokens > 0 {
        merged.max_output_tokens = Some(api.max_output_tokens);
    }

    merged.thinking_style = if api.capabilities.thinking.types.adaptive.supported {
        ThinkingStyle::Adaptive
    } else {
        ThinkingStyle::Budgeted
    };
    merged.supports_effort = api.capabilities.effort.supported;
    merged.supports_effort_max = api.capabilities.effort.max.supported;

    // requires_extended_context_beta is kept from bundled (runtime policy, not API data)

    merged
}

/// Create a `ModelInfo` for a Claude model discovered via the API that has
/// no entry in the bundled catalog.
///
/// This does NOT use `model_info_from_slug()` because that generic fallback
/// produces a hidden entry (`visibility: None`) with non-Claude defaults.
/// Instead, we start from a Claude-specific template that makes the model
/// immediately usable in the picker with correct Claude behavior.
pub(crate) fn model_info_from_anthropic_api(api: &AnthropicModelInfo) -> ModelInfo {
    let context_window = if api.max_input_tokens > 0 {
        api.max_input_tokens
    } else {
        200_000 // safe Claude default
    };
    let max_output_tokens = if api.max_output_tokens > 0 {
        Some(api.max_output_tokens)
    } else {
        None
    };
    let thinking_style = if api.capabilities.thinking.types.adaptive.supported {
        ThinkingStyle::Adaptive
    } else {
        ThinkingStyle::Budgeted
    };

    ModelInfo {
        slug: api.id.clone(),
        display_name: if api.display_name.is_empty() {
            api.id.clone()
        } else {
            api.display_name.clone()
        },
        description: None,
        default_reasoning_level: None,
        supported_reasoning_levels: Vec::new(),
        shell_type: ConfigShellToolType::ShellCommand,
        visibility: ModelVisibility::List,
        supported_in_api: true,
        priority: 95,
        availability_nux: None,
        upgrade: None,
        base_instructions: BASE_INSTRUCTIONS.to_string(),
        model_messages: None,
        supports_reasoning_summaries: false,
        default_reasoning_summary: ReasoningSummary::Auto,
        support_verbosity: false,
        default_verbosity: None,
        apply_patch_tool_type: Some(ApplyPatchToolType::Freeform),
        web_search_tool_type: WebSearchToolType::Text,
        truncation_policy: TruncationPolicyConfig::bytes(/*limit*/ 10_000),
        supports_parallel_tool_calls: false,
        supports_image_detail_original: false,
        context_window: Some(context_window),
        auto_compact_token_limit: None,
        effective_context_window_percent: 95,
        experimental_supported_tools: Vec::new(),
        input_modalities: default_input_modalities(),
        used_fallback_model_metadata: false,
        supports_search_tool: false,
        thinking_style,
        supports_effort: api.capabilities.effort.supported,
        supports_effort_max: api.capabilities.effort.max.supported,
        requires_extended_context_beta: false, // conservative default for unknown models
        max_output_tokens,
    }
}

#[cfg(test)]
#[path = "anthropic_mapping_tests.rs"]
mod tests;
