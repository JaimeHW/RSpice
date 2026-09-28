//! Refusals from project validation and catalog transactions.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SimulationConfigurationError {
    #[error("project configuration-set catalog is invalid: {message}")]
    InvalidConfigurationSetCatalog { message: String },
    #[error("project design-management catalog is invalid: {message}")]
    InvalidDesignManagementCatalog { message: String },
    #[error("project connectivity contract is invalid: {message}")]
    InvalidConnectivityContract { message: String },
    #[error("project-owned netlist document is invalid: {message}")]
    InvalidNetlistDocumentProjection { message: String },
    #[error("project-owned Code source registry is invalid: {message}")]
    InvalidProjectSourceRegistry { message: String },
    #[error("project hardcopy source-set catalog is invalid: {message}")]
    InvalidHardcopySourceSetCatalog { message: String },
    #[error("project hardcopy receipt ledger is invalid: {message}")]
    InvalidHardcopyReceiptLedger { message: String },
    #[error("project signed-PDK callback receipt ledger is invalid: {message}")]
    InvalidPdkCallbackReceiptLedger { message: String },
    #[error("project physical-layout document catalog is invalid: {message}")]
    InvalidPhysicalLayoutCatalog { message: String },
    #[error("report_documents[{index}] is invalid: {message}")]
    InvalidReportDocument { index: usize, message: String },
    #[error("report document identity {document_id} is duplicated")]
    DuplicateReportDocumentIdentity { document_id: ResultDocumentId },
    #[error("visualization_documents[{index}] is invalid: {message}")]
    InvalidVisualizationDocument { index: usize, message: String },
    #[error("visualization document identity {document_id} is duplicated")]
    DuplicateVisualizationDocumentIdentity { document_id: ResultDocumentId },
    #[error(
        "visualization document title {title:?} is duplicated by entries {first_index} and {index}"
    )]
    DuplicateVisualizationDocumentTitle {
        title: String,
        first_index: usize,
        index: usize,
    },
    #[error("simulation_plan_payloads contains duplicate owner {plan_id}")]
    DuplicatePlanPayload { plan_id: SimulationPlanId },
    #[error("simulation_plan_payloads[{plan_id}].design_variables[{index}] is invalid: {message}")]
    InvalidDesignVariable {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].design_variables[{index}] duplicates the case-insensitive name of design_variables[{first_index}]"
    )]
    DuplicateDesignVariableName {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("design variable identity {id} is reused by plans {first_plan_id} and {plan_id}")]
    DuplicateDesignVariableIdentity {
        id: DesignVariableId,
        first_plan_id: SimulationPlanId,
        plan_id: SimulationPlanId,
    },
    #[error("simulation_plan_payloads[{plan_id}].saved_outputs[{index}] is invalid: {message}")]
    InvalidSavedOutput {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].saved_outputs[{index}] duplicates the case-insensitive name of saved_outputs[{first_index}]"
    )]
    DuplicateSavedOutputName {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("saved output identity {id} is reused by plans {first_plan_id} and {plan_id}")]
    DuplicateSavedOutputIdentity {
        id: SavedOutputId,
        first_plan_id: SimulationPlanId,
        plan_id: SimulationPlanId,
    },
    /// Every way a capture group can be refused, as that module states them.
    #[error("simulation plan {plan_id}: {source}")]
    CaptureGroup {
        plan_id: SimulationPlanId,
        #[source]
        source: CaptureGroupError,
    },
    #[error("simulation_plan_payloads[{plan_id}].specs[{index}] is invalid: {message}")]
    InvalidSpecification {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].specs[{index}] duplicates the case-insensitive measurement of specs[{first_index}]"
    )]
    DuplicateSpecification {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].specification_definitions[{index}] is invalid: {message}"
    )]
    InvalidSpecificationDefinition {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error("specification identity {id} is reused by plans {first_plan_id} and {plan_id}")]
    DuplicateSpecificationIdentity {
        id: SpecificationId,
        first_plan_id: SimulationPlanId,
        plan_id: SimulationPlanId,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].specification_definitions[{index}] duplicates the case-insensitive requirement key of entry {first_index}"
    )]
    DuplicateSpecificationRequirementKey {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("simulation_plan_payloads[{plan_id}].specification_policy is invalid: {message}")]
    InvalidSpecificationPolicy {
        plan_id: SimulationPlanId,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].regression_tolerances[{index}] is invalid: {message}"
    )]
    InvalidRegressionTolerance {
        plan_id: SimulationPlanId,
        index: usize,
        message: String,
    },
    #[error(
        "simulation_plan_payloads[{plan_id}].regression_tolerances[{index}] duplicates target owned by entry {first_index}"
    )]
    DuplicateRegressionTolerance {
        plan_id: SimulationPlanId,
        index: usize,
        first_index: usize,
    },
    #[error("simulation plan {plan_id} already owns a design variable named '{name}'")]
    DesignVariableNameConflict {
        plan_id: SimulationPlanId,
        name: String,
    },
    #[error("simulation plan {plan_id} has no design variable with identity {variable_id}")]
    DesignVariableNotFound {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
    },
    #[error(
        "design variable {variable_id} in simulation plan {plan_id} could not advance its revision: {source}"
    )]
    DesignVariableRevision {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
        #[source]
        source: RevisionError,
    },
    #[error(
        "design variable {variable_id} is repeated in one update transaction for simulation plan {plan_id}"
    )]
    DuplicateDesignVariableUpdate {
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
    },
    #[error("simulation plan {plan_id} already owns a saved output named '{name}'")]
    SavedOutputNameConflict {
        plan_id: SimulationPlanId,
        name: String,
    },
    #[error("simulation plan {plan_id} has no saved output with identity {output_id}")]
    SavedOutputNotFound {
        plan_id: SimulationPlanId,
        output_id: SavedOutputId,
    },
    #[error(
        "saved output {output_id} in simulation plan {plan_id} could not advance its revision: {source}"
    )]
    SavedOutputRevision {
        plan_id: SimulationPlanId,
        output_id: SavedOutputId,
        #[source]
        source: RevisionError,
    },
    #[error("simulation plan {plan_id} has no configuration payload")]
    PlanPayloadMissing { plan_id: SimulationPlanId },
    #[error("simulation plan {plan_id} already has a configuration payload")]
    PlanPayloadAlreadyExists { plan_id: SimulationPlanId },
    #[error("cloned plan payload has no destination mapping for source analysis {analysis_id}")]
    MissingClonedAnalysisMapping { analysis_id: AnalysisInstanceId },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VisualizationDocumentPersistenceError {
    #[error("the project already contains the visualization document identity {document_id}")]
    DuplicateIdentity { document_id: ResultDocumentId },
    #[error("the project already contains a result document named {title:?}")]
    DuplicateTitle { title: String },
    #[error(
        "the project already contains the supported limit of {MAX_PROJECT_VISUALIZATION_DOCUMENTS} result documents"
    )]
    CatalogFull,
    #[error("the visualization document is invalid: {message}")]
    Invalid { message: String },
    #[error("the project does not contain visualization document {document_id}")]
    NotFound { document_id: ResultDocumentId },
    #[error("visualization document {document_id} transaction failed: {message}")]
    Transaction {
        document_id: ResultDocumentId,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProjectConfigurationMutationError {
    #[error("configuration-set catalog is invalid: {0}")]
    InvalidCatalog(#[from] rspice_design::configuration_set::ConfigurationSetError),
    #[error("design-management catalog is invalid: {message}")]
    InvalidDesignManagementCatalog { message: String },
    #[error("configuration '{configuration}' root {root} is not a schematic or testbench view")]
    UnsupportedRootView { configuration: String, root: String },
    #[error("configuration '{configuration}' root {root} has no authoritative schematic buffer")]
    MissingRootBuffer { configuration: String, root: String },
    #[error("project revision could not advance: {0}")]
    ProjectRevision(#[from] RevisionError),
    #[error("configuration-set transaction has no semantic changes")]
    NoChanges,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HardcopySourceSetPersistenceError {
    #[error("hardcopy source set is invalid: {message}")]
    Invalid { message: String },
    #[error(
        "project hardcopy source-set catalog is full ({MAX_PROJECT_HARDCOPY_SOURCE_SETS} sets)"
    )]
    CatalogFull,
    #[error("hardcopy source-set name '{name}' is already owned by another retained set")]
    DuplicateName { name: String },
}
