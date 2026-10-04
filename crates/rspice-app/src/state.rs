//! State Management Module
//!
//! Application state for simulation, project, and UI state.
//! Core data structures that are shared across multiple modules.

use rspice_design::configuration_set;
use rspice_design::connectivity_contract;
pub(crate) mod engineering_table;
pub(crate) mod library_browser;
mod model_bound_symbol;
pub(crate) mod model_hub;
pub(crate) mod model_library;
pub(crate) use rspice_design::netlist_document;
pub(crate) use rspice_design::parameters as params_string;
pub(crate) mod pdk_config;
use rspice_design::physical_layout;
pub(crate) use rspice_design::project_sources;
pub(crate) use rspice_design::properties as property_types;
pub(crate) use rspice_results::result_presentation;
mod schematic;
mod simulation;
pub(crate) mod stimulus_library;
use rspice_design::symbol;
pub(crate) mod symbol_resolver;
pub(crate) mod workspace;

#[cfg(test)]
pub(crate) use rspice_design::symbol::SYMBOL_VIEW_PARSES;

#[cfg(test)]
pub(crate) use rspice_model_library::CATALOG_LIBRARY_SERIALIZATIONS;

pub use configuration_set::{
    ConfigurationBlackBoxPolicy, ConfigurationCloneScope, ConfigurationModelProfile,
    ConfigurationPlatform, ConfigurationSet, ConfigurationSetCatalog, ConfigurationSetDefinition,
    ConfigurationSetError, ConfigurationSetId, ConfigurationSetOverride, UnresolvedBindingPolicy,
};
pub use connectivity_contract::{
    BundleWidthMismatchPolicy, ConnectivityContract, ConnectivityPolicy,
    GlobalAliasComparisonPolicy, GlobalNetPromotionPolicy,
};
// Test-only aliases: the submodule is private, so this path is the only
// way the tests can name these.
#[cfg(test)]
pub use connectivity_contract::{
    ConnectivityAliasGroup, DialectAliasCatalog, TechnologyGlobalNetCatalog,
};
pub use engineering_table::{
    EngineeringDataset, EngineeringFilterGrammar, EngineeringSortRule, EngineeringTableView,
    EngineeringViewScope, EngineeringVirtualizationPolicy, FrozenIdentifierPolicy,
    SavedEngineeringTableView, SortDirection,
};
pub use library_browser::{
    Cell, Library, LibraryCellPlacementCandidate, LibraryManager, ProjectLibraryLockAuthority,
    View, ViewType, library_cell_placement_candidates,
};
#[cfg(test)]
pub(crate) use model_bound_symbol::store_model_bound_symbol;
pub use model_bound_symbol::{
    GeneratedSymbolViews, MODEL_BOUND_SYMBOL_METADATA_KEY, MODEL_BOUND_SYMBOL_SCHEMA_VERSION,
    ModelBoundSymbolDefinition, ParameterInheritance, SymbolDefinitionImport, SymbolElectricalType,
    SymbolFormDiagnostic, SymbolGraphicTemplate, SymbolIdentity, SymbolImplementationView,
    SymbolModelReference, SymbolNetlistBinding, SymbolParameterConstraints, SymbolParameterDefault,
    SymbolParameterField, SymbolParameterForm, SymbolParameterSection, SymbolParameterVisibility,
    SymbolPinDefinition, SymbolPinSide, SymbolSourceContract, build_symbol_test_fixture,
    load_model_bound_symbol, materialize_symbol_document, prepare_symbol_construction,
};
pub use model_library::ModelLibraryManager;
#[cfg(test)]
pub use netlist_document::DocumentOwnership;
pub use netlist_document::{
    BoundedFindMatches, DependencyMetadata, DependencyResolution, DependencySourceAuthority,
    DiagnosticSeverity, FindDirection, FindError, FindMatch, FindOptions, GeneratedArtifact,
    GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput, NetlistDocument,
    NetlistDocumentId, NetlistSourceIndex, OutlineEntry, OutlineEntryKind, OutlineSection,
    OutlineSectionKind, SourceLocator, ValidationDiagnostic, content_digest,
    find_all_in_source_bounded, replace_source_ranges,
};
pub(crate) use netlist_document::{
    card_tokens, find_all_in_source_range_bounded_filter, parse_include_directives,
    same_include_graph,
};
pub use params_string::{format_params_string, parse_params_string};
pub use physical_layout::{
    LayoutEdit, LayoutGeometry, LayoutLayerPurpose, LayoutObjectId, LayoutPoint, LayoutShape,
    LayoutTechnologyBinding, PhysicalLayoutDocument,
};
#[cfg(test)]
pub use physical_layout::{LayoutInstance, LayoutOrientation, LayoutTransform};
#[cfg(not(target_arch = "wasm32"))]
pub use project_sources::MAX_PROJECT_SOURCE_DEPENDENCIES;
#[cfg(not(target_arch = "wasm32"))]
pub use project_sources::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH;
pub use project_sources::{
    AutomationStarterFile, DEFAULT_AUTOMATION_PERMISSIONS, DEFAULT_AUTOMATION_PYTHON,
    DEFAULT_AUTOMATION_RUN_PLAN, DEFAULT_ENVIRONMENT_LOCK, MAX_PROJECT_CODE_SOURCE_BYTES,
    MAX_PROJECT_SOURCE_BUNDLE_BYTES, MAX_PROJECT_SOURCE_FILES,
    MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS, PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION,
    ProjectSourceBundle, ProjectSourceDependency, ProjectSourceDocument, ProjectSourceFile,
    ProjectSourceId, ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceQualificationAttempt,
    ProjectSourceQualificationCheck, ProjectSourceQualificationDisposition,
    ProjectSourceQualificationTarget, ProjectSourceRegistry, ProjectSourceRole,
    ProjectSourceRoleBinding,
};
pub(crate) use project_sources::{
    CanonicalCellViewOwnerKey, canonical_cell_view_owner_key, project_source_path_key,
    project_source_paths_equal,
};
pub use property_types::{
    ContractStrength, DisplayMode, PropertyDefinition, PropertySheet, PropertyType, PropertyValue,
    SourceContractFinding, format_engineering, format_engineering_display,
};
pub use rspice_app_types::hierarchy_path::{
    HierarchyPathError, InstancePath, InstancePathPattern, MAX_INSTANCE_PATH_BYTES,
    MAX_INSTANCE_PATH_DEPTH, PatternSegment, ProbeTarget,
};
// The design-management model is a crate of its own so the offline
// drawing-sheet publisher can link it without linking the GUI. It is
// re-exported here because it is still the application's state authority and
// every caller names it through `crate::state`.
pub use rspice_design_model::design_management::*;
pub(crate) use schematic::bulk_edit;
pub(crate) use schematic::named_net;
pub use schematic::*;
// Test-only aliases: the submodule is private, so this path is the only way
// the tests can name an attribution's vocabulary directly.
#[cfg(test)]
pub use simulation::CanonicalAnalysisKind;

pub use rspice_model_library::sealed_model_sources;
#[cfg(test)]
pub use rspice_results::noise::{NoiseFigureEvidence, NoiseSummary};
#[cfg(test)]
pub use simulation::DcMismatchEvidence;
pub(crate) use simulation::RunHistoryRevision;
#[cfg(test)]
pub use simulation::SensitivityBasisEvidence;
#[cfg(test)]
pub use simulation::TransientConvergenceEvidence;
#[cfg(test)]
pub(crate) use simulation::current_impulse_history_fixture;
pub use simulation::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultSourceDomain, AnalysisType, ComplexResultValue, ConvergenceAttribution,
    CrossProbeIndex, DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, DcOpResult, DigitalBusEvidence,
    DigitalBusSourceEvidence, DigitalEventPointEvidence, DigitalEventTraceEvidence, EvidenceDomain,
    ExecutedDeck, ExecutedDeckArchive, ExecutedDeckPoint, ExecutionTarget, FamilyMemberId,
    FloquetOrbitKindEvidence, FloquetSpectrumEvidence, FloquetStabilityVerdictEvidence,
    MonteCarloVariableMetadata, OccurrenceProbeSpelling, OperatingPointAnnotationEvidence,
    OperatingPointValue, PoleZeroRootSetEvidence, PreparedRunReceipt, PreparedSpecification,
    PstbStabilityClassificationEvidence, RealEventPointEvidence, RealEventTraceEvidence,
    ResultImportFormat, ResultImportSource, RunHistory, RunRetention,
    SavedOutputMaterializationStatus, SensitivityResultMode, SensitivityResultRow,
    SharedWaveformValues, SignOffStanding, SimulationCampaignMembership, SimulationRun,
    SimulationRunIntent, SimulationRunLifecycle, SimulationRunProvenance, SimulationState,
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence,
    SpecificationVerdictStatus, WaveformData, absent_deck_reason, ac_bode_shape_for_analysis,
    ac_bode_shape_for_selection, ac_bode_summary_for_analysis, ac_bode_summary_for_selection,
};
#[cfg(test)]
pub use simulation::{
    AnalysisResultPvtPoint, FamilyMeasurementEvidence, FamilyMemberMeasurements, HierarchyMapRow,
    NoiseContributorRow, OperatingPointDeviceDetailEvidence, PeriodicNoiseConversionEvidence,
    PoleZeroSpectrumCertificate, PreparedModelSourceIdentity, PreparedRunReceiptInput,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, PreparedSpecificationPolicy,
    SavedOutputReceipt, SoaSourceHistory, SoaSourceWaveform, TransferFunctionAccuracyEvidence,
    TransferFunctionNormalizationEvidence, TransferFunctionQuantityEvidence,
    TransferFunctionScalarEvidence,
};
pub use simulation::{ConvergenceReport, PeriodicInitializationMethod};
pub use simulation::{DcCurveSelection, DcSweepFamily};
#[cfg(test)]
pub use simulation::{DcMismatchContributorEvidence, DcMismatchScopeEvidence};
#[cfg(test)]
pub use simulation::{DcSweepDirection, DcSweepQuantity};
#[cfg(test)]
pub use simulation::{DcSweepEvidence, SavedOutputDcMember};
#[cfg(test)]
pub use simulation::{
    FloquetSpectrumCertificateEvidence, OperatingPointAccuracyEvidence,
    OperatingPointHomotopyEvidence, OperatingPointInitialGuessEvidence,
    OperatingPointNodeInitializationEvidence, OperatingPointPreviousStateEvidence,
    OperatingPointProcessEvidence, OperatingPointSaveDeviceEvidence,
    OperatingPointTemperatureEvidence, PeriodicNoiseOutputQuantity, PssFloquetMultiplierEvidence,
    PstbFloquetModeEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
};
#[cfg(test)]
pub use simulation::{SavedOutputAxis, SavedOutputBoundSource, SavedOutputSourceBindings};
pub use simulation::{SensitivityStudyEvidence, SensitivityStudyRow};
// Only the two types the persisted model itself names are hoisted here. The
// rest of the stimulus vocabulary — the definition record, the draft state
// machine, the adoption verbs — is read through `state::stimulus_library::*`,
// where the module the name belongs to is part of the path: `adopt` and
// `extract` say nothing on their own, and a `use` list is not where a reader
// should have to learn what they act on.
pub use stimulus_library::library::StimulusLibrary;
pub use symbol::{
    PinFindingKind, PinSummary, SYMBOL_DOCUMENT_METADATA_KEY, SymbolAttributeKind, SymbolDocument,
    SymbolEditorMetadata, SymbolPin, SymbolShape,
};
#[cfg(test)]
pub use symbol_resolver::ResolvedCellSymbol;
pub use symbol_resolver::SymbolResolver;
pub(crate) use workspace::PreparedProjectLibraryMutation;
pub use workspace::{
    CaptureGroup, CaptureGroupMembership, CaptureGroupRule, CellViewRef, ComplexExpressionPolicy,
    DesignVariable, DesignVariableDefect, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, MembershipMove,
    MissingMeasurementPolicy, MonteCarloSpecificationGate, NetlistExecutionProfile,
    NetlistLineEnding, NetlistSourceDialect, NetlistTextEncoding, NominalFailurePolicy,
    OpenCellView, OutputSelectionMode, OwnedNetlistDescriptor, OwnedNetlistEditStrategy,
    OwnedNetlistIncludeDescriptor, OwnedNetlistSaveRecord, PROJECT_DESCRIPTOR_SCHEMA_VERSION,
    PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION, ProjectCloudPublicationBinding, ProjectDescriptor,
    ProjectLibraryMutation, ProjectTechnologyBinding, ProjectTechnologyChangeAuthority,
    ProjectTechnologyChangeContext, ProjectWorkspace, RegressionComparisonMethod,
    RegressionComparisonWindow, RegressionSpecificationPolicy, RegressionTargetKind,
    RegressionTargetSelector, RegressionToleranceRule, ResolvedHierarchyBinding,
    RetainedOwnedNetlistDeck, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
    SavedOutputOrigin, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    SimulationPlanPayload, SimulationPlanPayloadRecord, SpecEntry, SpecPointScope,
    SpecificationDefinition, SpecificationPolicy, SpecificationRole, UNGROUPED_NAME, group_namer,
    validate_owned_netlist_artifact_path,
};

pub use simulation::{MonteCarloCheckpointEvidence, MonteCarloCheckpointLibrary};
#[cfg(test)]
pub use workspace::{SavedOutputDisplayIntent, SpecificationComparison};

#[cfg(test)]
mod symbol_document_tests {
    use super::{
        PinFindingKind, PinSummary, Point, PortDirection, PortSpec, SymbolDocument, View, ViewType,
    };

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn ota_ports() -> Vec<PortSpec> {
        vec![
            port("INP", PortDirection::In),
            port("INN", PortDirection::In),
            port("OUT", PortDirection::Out),
            port("VDD", PortDirection::Supply),
            port("VSS", PortDirection::Supply),
        ]
    }

    #[test]
    fn generated_symbol_document_places_schematic_ports_in_order() {
        let doc = SymbolDocument::generated_from_ports(&ota_ports());

        let names: Vec<&str> = doc.pins.iter().map(|pin| pin.name.as_str()).collect();
        assert_eq!(names, ["INP", "INN", "OUT", "VDD", "VSS"]);
        assert_eq!(doc.pin_summary(&ota_ports()), PinSummary::Match);
        assert!(doc.pins.iter().all(|pin| pin.position.is_some()));
        assert!(doc.pins.iter().all(|pin| pin.terminal_on_grid()));
        assert!(!doc.body.is_empty(), "generated symbols include a body");
    }

    #[test]
    fn symbol_document_round_trips_through_view_metadata() {
        let mut view = View::new("symbol", ViewType::Symbol);
        let mut doc = SymbolDocument::generated_from_ports(&ota_ports());
        doc.name_anchor = Point::new(-20, -40);

        doc.store_in_view(&mut view)
            .expect("serialize symbol document");
        let restored = SymbolDocument::load_from_view(&view).expect("read symbol document");

        assert_eq!(restored, doc);
    }

    #[test]
    fn reconcile_ports_places_new_pins_without_overwriting_existing_art() {
        let mut doc = SymbolDocument::generated_from_ports(&ota_ports());
        let original_inp = doc.pin("INP").expect("INP exists").position;
        let body = doc.body.clone();
        doc.pin_mut("INP").expect("INP exists").position = Some(Point::new(-50, -10));

        let mut ports = ota_ports();
        ports.push(port("IBIAS", PortDirection::In));
        doc.reconcile_ports(&ports);

        assert_eq!(
            doc.pin("INP").expect("INP exists").position,
            Some(Point::new(-50, -10)),
            "hand-edited pin placement must survive additive reconciliation"
        );
        assert_ne!(doc.pin("INP").expect("INP exists").position, original_inp);
        assert_eq!(doc.body, body, "reconciliation never redraws the body");
        let added = doc.pin("IBIAS").expect("new pin exists");
        assert!(
            added.position.is_some(),
            "a pin the contract declares is placed against the body, not left \
             for the author to find in the unplaced list"
        );
        assert_ne!(
            added.offset,
            doc.pin("INP").expect("INP exists").offset,
            "a new pin takes a free offset rather than stacking on one in use"
        );
        assert_eq!(doc.pin_summary(&ports), PinSummary::Match);
    }

    #[test]
    fn dropped_schematic_ports_report_orphaned_symbol_pins() {
        let doc = SymbolDocument::generated_from_ports(&ota_ports());
        let ports = vec![port("INP", PortDirection::In)];

        assert_eq!(doc.pin_summary(&ports), PinSummary::Orphaned(4));
    }

    #[test]
    fn imported_off_grid_pin_is_reported_even_though_editor_snaps_new_pins() {
        let mut doc = SymbolDocument::generated_from_ports(&ota_ports());
        doc.pin_mut("OUT").expect("OUT exists").position = Some(Point::new(13, 0));

        let findings = doc.pin_findings(&ota_ports());

        assert!(
            findings
                .iter()
                .any(|finding| finding.kind == PinFindingKind::PinOffGrid
                    && finding.pin_name == "OUT")
        );
    }

    #[test]
    fn no_schematic_ports_is_an_idle_symbol_contract() {
        let doc = SymbolDocument::default();

        assert_eq!(doc.pin_summary(&[]), PinSummary::NoSchematic);
    }
}

pub(crate) mod project_snapshot;
