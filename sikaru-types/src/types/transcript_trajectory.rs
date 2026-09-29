pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TranscriptTrajectory {
    #[serde(default)]
    pub agent: HashMap<String, String>,
    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_metrics: Option<HashMap<String, i64>>,
    #[serde(default)]
    pub schema_version: String,
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub steps: Vec<TranscriptStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagent_trajectories: Option<Vec<Box<TranscriptTrajectory>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trajectory_id: Option<String>,
}

impl TranscriptTrajectory {
    pub fn builder() -> TranscriptTrajectoryBuilder {
        <TranscriptTrajectoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptTrajectoryBuilder {
    agent: Option<HashMap<String, String>>,
    extra: Option<HashMap<String, serde_json::Value>>,
    final_metrics: Option<HashMap<String, i64>>,
    schema_version: Option<String>,
    session_id: Option<String>,
    steps: Option<Vec<TranscriptStep>>,
    subagent_trajectories: Option<Vec<Box<TranscriptTrajectory>>>,
    trajectory_id: Option<String>,
}

impl TranscriptTrajectoryBuilder {
    pub fn agent(mut self, value: HashMap<String, String>) -> Self {
        self.agent = Some(value);
        self
    }

    pub fn extra(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.extra = Some(value);
        self
    }

    pub fn final_metrics(mut self, value: HashMap<String, i64>) -> Self {
        self.final_metrics = Some(value);
        self
    }

    pub fn schema_version(mut self, value: impl Into<String>) -> Self {
        self.schema_version = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn steps(mut self, value: Vec<TranscriptStep>) -> Self {
        self.steps = Some(value);
        self
    }

    pub fn subagent_trajectories(mut self, value: Vec<Box<TranscriptTrajectory>>) -> Self {
        self.subagent_trajectories = Some(value);
        self
    }

    pub fn trajectory_id(mut self, value: impl Into<String>) -> Self {
        self.trajectory_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranscriptTrajectory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent`](TranscriptTrajectoryBuilder::agent)
    /// - [`extra`](TranscriptTrajectoryBuilder::extra)
    /// - [`schema_version`](TranscriptTrajectoryBuilder::schema_version)
    /// - [`session_id`](TranscriptTrajectoryBuilder::session_id)
    /// - [`steps`](TranscriptTrajectoryBuilder::steps)
    pub fn build(self) -> Result<TranscriptTrajectory, BuildError> {
        Ok(TranscriptTrajectory {
            agent: self.agent.ok_or_else(|| BuildError::missing_field("agent"))?,
            extra: self.extra.ok_or_else(|| BuildError::missing_field("extra"))?,
            final_metrics: self.final_metrics,
            schema_version: self.schema_version.ok_or_else(|| BuildError::missing_field("schema_version"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            steps: self.steps.ok_or_else(|| BuildError::missing_field("steps"))?,
            subagent_trajectories: self.subagent_trajectories,
            trajectory_id: self.trajectory_id,
        })
    }
}
