pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentDefinitionSourceKind {
    AgentsMd,
    AgentMd,
    AgentSkill,
    SkillMd,
    SkillsMd,
    SkillAsset,
    EvalMd,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentDefinitionSourceKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AgentsMd => serializer.serialize_str("agents_md"),
            Self::AgentMd => serializer.serialize_str("agent_md"),
            Self::AgentSkill => serializer.serialize_str("agent_skill"),
            Self::SkillMd => serializer.serialize_str("skill_md"),
            Self::SkillsMd => serializer.serialize_str("skills_md"),
            Self::SkillAsset => serializer.serialize_str("skill_asset"),
            Self::EvalMd => serializer.serialize_str("eval_md"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentDefinitionSourceKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "agents_md" => Ok(Self::AgentsMd),
            "agent_md" => Ok(Self::AgentMd),
            "agent_skill" => Ok(Self::AgentSkill),
            "skill_md" => Ok(Self::SkillMd),
            "skills_md" => Ok(Self::SkillsMd),
            "skill_asset" => Ok(Self::SkillAsset),
            "eval_md" => Ok(Self::EvalMd),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentDefinitionSourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentsMd => write!(f, "agents_md"),
            Self::AgentMd => write!(f, "agent_md"),
            Self::AgentSkill => write!(f, "agent_skill"),
            Self::SkillMd => write!(f, "skill_md"),
            Self::SkillsMd => write!(f, "skills_md"),
            Self::SkillAsset => write!(f, "skill_asset"),
            Self::EvalMd => write!(f, "eval_md"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
