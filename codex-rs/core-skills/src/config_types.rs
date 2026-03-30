use orbit_code_utils_absolute_path::AbsolutePathBuf;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SkillConfig {
    pub path: AbsolutePathBuf,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BundledSkillsConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

impl Default for BundledSkillsConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct SkillsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundled: Option<BundledSkillsConfig>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<SkillConfig>,
}

fn default_enabled() -> bool {
    true
}
