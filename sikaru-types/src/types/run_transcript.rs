pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunTranscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<PersonalConnectionPrompt>>,
    #[serde(default)]
    pub events: Vec<TranscriptEvent>,
    pub evidence: RunTranscriptEvidence,
    pub run: TranscriptRun,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<TranscriptTrajectory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TranscriptUsage>,
}

impl RunTranscript {
    pub fn builder() -> RunTranscriptBuilder {
        <RunTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunTranscriptBuilder {
    connections: Option<Vec<PersonalConnectionPrompt>>,
    events: Option<Vec<TranscriptEvent>>,
    evidence: Option<RunTranscriptEvidence>,
    run: Option<TranscriptRun>,
    trajectory: Option<TranscriptTrajectory>,
    usage: Option<TranscriptUsage>,
}

impl RunTranscriptBuilder {
    pub fn connections(mut self, value: Vec<PersonalConnectionPrompt>) -> Self {
        self.connections = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<TranscriptEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn evidence(mut self, value: RunTranscriptEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    pub fn run(mut self, value: TranscriptRun) -> Self {
        self.run = Some(value);
        self
    }

    pub fn trajectory(mut self, value: TranscriptTrajectory) -> Self {
        self.trajectory = Some(value);
        self
    }

    pub fn usage(mut self, value: TranscriptUsage) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](RunTranscriptBuilder::events)
    /// - [`evidence`](RunTranscriptBuilder::evidence)
    /// - [`run`](RunTranscriptBuilder::run)
    pub fn build(self) -> Result<RunTranscript, BuildError> {
        Ok(RunTranscript {
            connections: self.connections,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
            run: self.run.ok_or_else(|| BuildError::missing_field("run"))?,
            trajectory: self.trajectory,
            usage: self.usage,
        })
    }
}
