//! API client and types for the Sikaru API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    ActivationClient, AgentDocumentsClient, AgentImportsClient, AgentsClient, ApiClient,
    AuthClient, CapabilityCeilingsClient, ChangesetsClient, ChannelsClient, ChecksClient,
    ComputeAttachmentsClient, ComputeCredentialsClient, ComputeEnvironmentsClient,
    ComputeOperationsClient, ComputeWorkersClient, ComputeWorkspacesClient, ConnectionsClient,
    ContextRegistryClient, ConversationsClient, DatasetsClient, DeploymentsClient,
    EnvironmentsClient, EvaluationComparisonsClient, EvaluationCriteriaClient,
    EvaluationJobsClient, EvaluationResultsClient, EvaluatorRunsClient, ExecutionObjectivesClient,
    ExecutionSessionsClient, ExecutionsClient, FeedbackClient, GitCredentialsClient,
    HarnessesClient, HttpChannelsClient, ImportSessionsClient, IssueClustersClient,
    JudgeAlignmentClient, ManagedAgentsClient, MemoryRegistryClient, ModelGatewayClient,
    ModelSettingsClient, OnlineEvaluationsClient, OrganizationsClient, PersonalChannelsClient,
    ReleaseWatchesClient, RetentionPoliciesClient, ReviewQueueClient, RunReferencesClient,
    RunSchedulesClient, RunWebhooksClient, RunsClient, SessionsClient, SpecialistsClient,
    ToolProvidersClient, TraceImportConnectionsClient, TraceImportsClient, TraceStreamsClient,
    WorkflowIntentsClient, WorkflowRunsClient, WorkflowsClient,
};

pub use sikaru_types::*;
