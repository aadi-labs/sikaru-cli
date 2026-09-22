pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateManagedAgentSourceRequestSourceKind {
    WorkflowJson,
    Langgraph,
    N8NJson,
    YamlDag,
    SourceBundle,
    RepoRef,
    ContainerRef,
    AdapterEndpoint,
    PromptBundle,
    SkillTree,
    MemoryProfile,
    EvalSuite,
    TraceBundle,
    NaturalLanguage,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateManagedAgentSourceRequestSourceKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WorkflowJson => serializer.serialize_str("workflow_json"),
            Self::Langgraph => serializer.serialize_str("langgraph"),
            Self::N8NJson => serializer.serialize_str("n8n_json"),
            Self::YamlDag => serializer.serialize_str("yaml_dag"),
            Self::SourceBundle => serializer.serialize_str("source_bundle"),
            Self::RepoRef => serializer.serialize_str("repo_ref"),
            Self::ContainerRef => serializer.serialize_str("container_ref"),
            Self::AdapterEndpoint => serializer.serialize_str("adapter_endpoint"),
            Self::PromptBundle => serializer.serialize_str("prompt_bundle"),
            Self::SkillTree => serializer.serialize_str("skill_tree"),
            Self::MemoryProfile => serializer.serialize_str("memory_profile"),
            Self::EvalSuite => serializer.serialize_str("eval_suite"),
            Self::TraceBundle => serializer.serialize_str("trace_bundle"),
            Self::NaturalLanguage => serializer.serialize_str("natural_language"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateManagedAgentSourceRequestSourceKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "workflow_json" => Ok(Self::WorkflowJson),
            "langgraph" => Ok(Self::Langgraph),
            "n8n_json" => Ok(Self::N8NJson),
            "yaml_dag" => Ok(Self::YamlDag),
            "source_bundle" => Ok(Self::SourceBundle),
            "repo_ref" => Ok(Self::RepoRef),
            "container_ref" => Ok(Self::ContainerRef),
            "adapter_endpoint" => Ok(Self::AdapterEndpoint),
            "prompt_bundle" => Ok(Self::PromptBundle),
            "skill_tree" => Ok(Self::SkillTree),
            "memory_profile" => Ok(Self::MemoryProfile),
            "eval_suite" => Ok(Self::EvalSuite),
            "trace_bundle" => Ok(Self::TraceBundle),
            "natural_language" => Ok(Self::NaturalLanguage),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateManagedAgentSourceRequestSourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkflowJson => write!(f, "workflow_json"),
            Self::Langgraph => write!(f, "langgraph"),
            Self::N8NJson => write!(f, "n8n_json"),
            Self::YamlDag => write!(f, "yaml_dag"),
            Self::SourceBundle => write!(f, "source_bundle"),
            Self::RepoRef => write!(f, "repo_ref"),
            Self::ContainerRef => write!(f, "container_ref"),
            Self::AdapterEndpoint => write!(f, "adapter_endpoint"),
            Self::PromptBundle => write!(f, "prompt_bundle"),
            Self::SkillTree => write!(f, "skill_tree"),
            Self::MemoryProfile => write!(f, "memory_profile"),
            Self::EvalSuite => write!(f, "eval_suite"),
            Self::TraceBundle => write!(f, "trace_bundle"),
            Self::NaturalLanguage => write!(f, "natural_language"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
