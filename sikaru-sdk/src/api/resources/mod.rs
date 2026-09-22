//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Activation**
//! - **AgentImports**
//! - **Agents**
//! - **Changesets**
//! - **ComputeAttachments**
//! - **ComputeOperations**
//! - **ComputeCredentials**
//! - **ComputeEnvironments**
//! - **ComputeWorkers**
//! - **Connections**
//! - **ContextRegistry**
//! - **Conversations**
//! - **Deployments**
//! - **Environments**
//! - **EvalSeeds**
//! - **EvaluationComparisons**
//! - **EvaluationCriteria**
//! - **EvaluationJobs**
//! - **EvaluationResults**
//! - **EvaluatorRuns**
//! - **ExecutionObjectives**
//! - **ExecutionSessions**
//! - **Specialists**
//! - **Executions**
//! - **Feedback**
//! - **HarnessVersions**
//! - **AgentBudgets**
//! - **Harnesses**
//! - **Runs**
//! - **ImportSessions**
//! - **IssueClusters**
//! - **JudgeAlignment**
//! - **ManagedAgents**
//! - **MemoryRegistry**
//! - **ModelGateway**
//! - **ModelSettings**
//! - **OnlineEvaluations**
//! - **ReleaseWatches**
//! - **RetentionPolicies**
//! - **ReviewQueue**
//! - **RunSchedules**
//! - **RunWebhooks**
//! - **Sessions**
//! - **ToolProviders**
//! - **TraceImportConnections**
//! - **TraceImports**
//! - **WorkflowIntents**
//! - **WorkflowRuns**
//! - **Workflows**
//! - **TraceStreams**

use crate::{ApiError, ClientConfig};

pub mod activation;
pub mod agent_budgets;
pub mod agent_imports;
pub mod agents;
pub mod changesets;
pub mod compute_attachments;
pub mod compute_credentials;
pub mod compute_environments;
pub mod compute_operations;
pub mod compute_workers;
pub mod connections;
pub mod context_registry;
pub mod conversations;
pub mod deployments;
pub mod environments;
pub mod eval_seeds;
pub mod evaluation_comparisons;
pub mod evaluation_criteria;
pub mod evaluation_jobs;
pub mod evaluation_results;
pub mod evaluator_runs;
pub mod execution_objectives;
pub mod execution_sessions;
pub mod executions;
pub mod feedback;
pub mod harness_versions;
pub mod harnesses;
pub mod import_sessions;
pub mod issue_clusters;
pub mod judge_alignment;
pub mod managed_agents;
pub mod memory_registry;
pub mod model_gateway;
pub mod model_settings;
pub mod online_evaluations;
pub mod release_watches;
pub mod retention_policies;
pub mod review_queue;
pub mod run_schedules;
pub mod run_webhooks;
pub mod runs;
pub mod sessions;
pub mod specialists;
pub mod tool_providers;
pub mod trace_import_connections;
pub mod trace_imports;
pub mod trace_streams;
pub mod workflow_intents;
pub mod workflow_runs;
pub mod workflows;
pub struct ApiClient {
    pub config: ClientConfig,
    pub activation: ActivationClient,
    pub agent_imports: AgentImportsClient,
    pub agents: AgentsClient,
    pub changesets: ChangesetsClient,
    pub compute_attachments: ComputeAttachmentsClient,
    pub compute_operations: ComputeOperationsClient,
    pub compute_credentials: ComputeCredentialsClient,
    pub compute_environments: ComputeEnvironmentsClient,
    pub compute_workers: ComputeWorkersClient,
    pub connections: ConnectionsClient,
    pub context_registry: ContextRegistryClient,
    pub conversations: ConversationsClient,
    pub deployments: DeploymentsClient,
    pub environments: EnvironmentsClient,
    pub eval_seeds: EvalSeedsClient,
    pub evaluation_comparisons: EvaluationComparisonsClient,
    pub evaluation_criteria: EvaluationCriteriaClient,
    pub evaluation_jobs: EvaluationJobsClient,
    pub evaluation_results: EvaluationResultsClient,
    pub evaluator_runs: EvaluatorRunsClient,
    pub execution_objectives: ExecutionObjectivesClient,
    pub execution_sessions: ExecutionSessionsClient,
    pub specialists: SpecialistsClient,
    pub executions: ExecutionsClient,
    pub feedback: FeedbackClient,
    pub harness_versions: HarnessVersionsClient,
    pub agent_budgets: AgentBudgetsClient,
    pub harnesses: HarnessesClient,
    pub runs: RunsClient,
    pub import_sessions: ImportSessionsClient,
    pub issue_clusters: IssueClustersClient,
    pub judge_alignment: JudgeAlignmentClient,
    pub managed_agents: ManagedAgentsClient,
    pub memory_registry: MemoryRegistryClient,
    pub model_gateway: ModelGatewayClient,
    pub model_settings: ModelSettingsClient,
    pub online_evaluations: OnlineEvaluationsClient,
    pub release_watches: ReleaseWatchesClient,
    pub retention_policies: RetentionPoliciesClient,
    pub review_queue: ReviewQueueClient,
    pub run_schedules: RunSchedulesClient,
    pub run_webhooks: RunWebhooksClient,
    pub sessions: SessionsClient,
    pub tool_providers: ToolProvidersClient,
    pub trace_import_connections: TraceImportConnectionsClient,
    pub trace_imports: TraceImportsClient,
    pub workflow_intents: WorkflowIntentsClient,
    pub workflow_runs: WorkflowRunsClient,
    pub workflows: WorkflowsClient,
    pub trace_streams: TraceStreamsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            activation: ActivationClient::new(config.clone())?,
            agent_imports: AgentImportsClient::new(config.clone())?,
            agents: AgentsClient::new(config.clone())?,
            changesets: ChangesetsClient::new(config.clone())?,
            compute_attachments: ComputeAttachmentsClient::new(config.clone())?,
            compute_operations: ComputeOperationsClient::new(config.clone())?,
            compute_credentials: ComputeCredentialsClient::new(config.clone())?,
            compute_environments: ComputeEnvironmentsClient::new(config.clone())?,
            compute_workers: ComputeWorkersClient::new(config.clone())?,
            connections: ConnectionsClient::new(config.clone())?,
            context_registry: ContextRegistryClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            deployments: DeploymentsClient::new(config.clone())?,
            environments: EnvironmentsClient::new(config.clone())?,
            eval_seeds: EvalSeedsClient::new(config.clone())?,
            evaluation_comparisons: EvaluationComparisonsClient::new(config.clone())?,
            evaluation_criteria: EvaluationCriteriaClient::new(config.clone())?,
            evaluation_jobs: EvaluationJobsClient::new(config.clone())?,
            evaluation_results: EvaluationResultsClient::new(config.clone())?,
            evaluator_runs: EvaluatorRunsClient::new(config.clone())?,
            execution_objectives: ExecutionObjectivesClient::new(config.clone())?,
            execution_sessions: ExecutionSessionsClient::new(config.clone())?,
            specialists: SpecialistsClient::new(config.clone())?,
            executions: ExecutionsClient::new(config.clone())?,
            feedback: FeedbackClient::new(config.clone())?,
            harness_versions: HarnessVersionsClient::new(config.clone())?,
            agent_budgets: AgentBudgetsClient::new(config.clone())?,
            harnesses: HarnessesClient::new(config.clone())?,
            runs: RunsClient::new(config.clone())?,
            import_sessions: ImportSessionsClient::new(config.clone())?,
            issue_clusters: IssueClustersClient::new(config.clone())?,
            judge_alignment: JudgeAlignmentClient::new(config.clone())?,
            managed_agents: ManagedAgentsClient::new(config.clone())?,
            memory_registry: MemoryRegistryClient::new(config.clone())?,
            model_gateway: ModelGatewayClient::new(config.clone())?,
            model_settings: ModelSettingsClient::new(config.clone())?,
            online_evaluations: OnlineEvaluationsClient::new(config.clone())?,
            release_watches: ReleaseWatchesClient::new(config.clone())?,
            retention_policies: RetentionPoliciesClient::new(config.clone())?,
            review_queue: ReviewQueueClient::new(config.clone())?,
            run_schedules: RunSchedulesClient::new(config.clone())?,
            run_webhooks: RunWebhooksClient::new(config.clone())?,
            sessions: SessionsClient::new(config.clone())?,
            tool_providers: ToolProvidersClient::new(config.clone())?,
            trace_import_connections: TraceImportConnectionsClient::new(config.clone())?,
            trace_imports: TraceImportsClient::new(config.clone())?,
            workflow_intents: WorkflowIntentsClient::new(config.clone())?,
            workflow_runs: WorkflowRunsClient::new(config.clone())?,
            workflows: WorkflowsClient::new(config.clone())?,
            trace_streams: TraceStreamsClient::new(config.clone())?,
        })
    }
}

pub use activation::ActivationClient;
pub use agent_budgets::AgentBudgetsClient;
pub use agent_imports::AgentImportsClient;
pub use agents::AgentsClient;
pub use changesets::ChangesetsClient;
pub use compute_attachments::ComputeAttachmentsClient;
pub use compute_credentials::ComputeCredentialsClient;
pub use compute_environments::ComputeEnvironmentsClient;
pub use compute_operations::ComputeOperationsClient;
pub use compute_workers::ComputeWorkersClient;
pub use connections::ConnectionsClient;
pub use context_registry::ContextRegistryClient;
pub use conversations::ConversationsClient;
pub use deployments::DeploymentsClient;
pub use environments::EnvironmentsClient;
pub use eval_seeds::EvalSeedsClient;
pub use evaluation_comparisons::EvaluationComparisonsClient;
pub use evaluation_criteria::EvaluationCriteriaClient;
pub use evaluation_jobs::EvaluationJobsClient;
pub use evaluation_results::EvaluationResultsClient;
pub use evaluator_runs::EvaluatorRunsClient;
pub use execution_objectives::ExecutionObjectivesClient;
pub use execution_sessions::ExecutionSessionsClient;
pub use executions::ExecutionsClient;
pub use feedback::FeedbackClient;
pub use harness_versions::HarnessVersionsClient;
pub use harnesses::HarnessesClient;
pub use import_sessions::ImportSessionsClient;
pub use issue_clusters::IssueClustersClient;
pub use judge_alignment::JudgeAlignmentClient;
pub use managed_agents::ManagedAgentsClient;
pub use memory_registry::MemoryRegistryClient;
pub use model_gateway::ModelGatewayClient;
pub use model_settings::ModelSettingsClient;
pub use online_evaluations::OnlineEvaluationsClient;
pub use release_watches::ReleaseWatchesClient;
pub use retention_policies::RetentionPoliciesClient;
pub use review_queue::ReviewQueueClient;
pub use run_schedules::RunSchedulesClient;
pub use run_webhooks::RunWebhooksClient;
pub use runs::RunsClient;
pub use sessions::SessionsClient;
pub use specialists::SpecialistsClient;
pub use tool_providers::ToolProvidersClient;
pub use trace_import_connections::TraceImportConnectionsClient;
pub use trace_imports::TraceImportsClient;
pub use trace_streams::TraceStreamsClient;
pub use workflow_intents::WorkflowIntentsClient;
pub use workflow_runs::WorkflowRunsClient;
pub use workflows::WorkflowsClient;
