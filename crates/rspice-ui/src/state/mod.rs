//! State Management Module
//!
//! Application state for simulation, project, and UI state.
//! Core data structures that are shared across multiple modules.

mod configuration_set;
mod design_management;
pub mod library_browser;
mod model_bound_symbol;
pub mod model_library;
pub mod pdk_config;
mod project_sources;
pub mod property_types;
mod schematic;
mod simulation;
mod symbol;
pub mod symbol_resolver;
pub mod workspace;

pub use configuration_set::{
    ALLOWED_EXECUTABLE_VIEW_TYPES, CONFIGURATION_SET_CATALOG_SCHEMA_VERSION,
    ConfigurationBlackBoxPolicy, ConfigurationCloneScope, ConfigurationModelProfile,
    ConfigurationPlatform, ConfigurationSet, ConfigurationSetCatalog, ConfigurationSetDefinition,
    ConfigurationSetError, ConfigurationSetId, ConfigurationSetLineage, ConfigurationSetOverride,
    UnresolvedBindingPolicy,
};
pub use design_management::*;
pub use library_browser::{
    Cell, Library, LibraryCellPlacementCandidate, LibraryCellPlacementParameter, LibraryManager,
    NavColumn, View, ViewType, library_cell_placement_candidates,
};
pub use model_bound_symbol::{
    GeneratedSymbolViews, ImportedGraphicFormat, ImportedGraphicSource, ImportedPinAnchor,
    MODEL_BOUND_SYMBOL_METADATA_KEY, MODEL_BOUND_SYMBOL_SCHEMA_VERSION, ModelBoundSymbolDefinition,
    ParameterInheritance, SYMBOL_IMPORT_SOURCE_METADATA_KEY, SYMBOL_PARAMETER_FORM_METADATA_KEY,
    SymbolConstructionPlan, SymbolConstructionReceipt, SymbolDefinitionError,
    SymbolDefinitionImport, SymbolElectricalType, SymbolFormDiagnostic, SymbolGraphicTemplate,
    SymbolIdentity, SymbolImplementationView, SymbolImportFormat, SymbolImportReport,
    SymbolModelReference, SymbolNetlistBinding, SymbolParameterConstraints, SymbolParameterDefault,
    SymbolParameterField, SymbolParameterForm, SymbolParameterSection, SymbolParameterVisibility,
    SymbolPinDefinition, SymbolPinSide, SymbolSourceContract, SymbolTestFixtureAccess,
    SymbolTestFixtureContract,
};
pub use model_library::ModelLibraryManager;
pub use pdk_config::{ConfigError, DiscoveredFile, LibraryPathEntry, PdkConfig};
pub(crate) use project_sources::{CanonicalCellViewOwnerKey, canonical_cell_view_owner_key};
pub use project_sources::{
    MAX_PROJECT_CODE_SOURCE_BYTES, MAX_PROJECT_SOURCE_BUNDLE_BYTES,
    MAX_PROJECT_SOURCE_DEPENDENCIES, MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH, MAX_PROJECT_SOURCE_FILES,
    MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES, PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION,
    ProjectSourceBundle, ProjectSourceDependency, ProjectSourceDocument, ProjectSourceError,
    ProjectSourceFile, ProjectSourceId, ProjectSourceIdError, ProjectSourceIdParseError,
    ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry,
    ProjectSourceValidationIdentity, project_veriloga_bundle_alias,
    project_veriloga_bundle_source_key,
};
pub use property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertySheet, PropertyType, PropertyValue,
    VisibilityCondition, format_engineering,
};
pub use schematic::*;
pub use simulation::{
    AcBodeMetrics, AcBodeSummary, AnalysisResult, AnalysisResultFamilyMetadata,
    AnalysisResultPayload, AnalysisResultProvenance, AnalysisResultSourceDomain, AnalysisType,
    ComplexResultValue, CrossProbeMapping, DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, DcOpResult,
    DisplayWaveformCache, ExecutionTarget, MonteCarloVariableMetadata, NoiseContributorRow,
    NoiseSummary, OperatingPointValue, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence,
    ReliabilityShiftEvidence, ReliabilityStressEvidence, SavedOutputMaterializationStatus,
    SavedOutputReceipt, SensitivityResultMode, SensitivityResultRow, SharedWaveformValues,
    SimulationExecutionIdentity, SimulationRun, SimulationRunIntent, SimulationRunLifecycle,
    SimulationRunProvenance, SimulationState, SoaEvaluationEvidence, SoaParameterEvidence,
    SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
    TransferFunctionAccuracyEvidence, TransferFunctionNormalizationEvidence,
    TransferFunctionQuantityEvidence, TransferFunctionScalarEvidence, WaveformData,
    ac_bode_summary_for_run, ac_bode_summary_for_selection, ac_bode_summary_for_source_instance,
};
pub use symbol::{
    PinFinding, PinFindingKind, PinSummary, SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_TERMINAL_GRID,
    SymbolDocument, SymbolLabelAnchors, SymbolPin, SymbolShape,
};
pub use symbol_resolver::{
    ResolvedCellSymbol, ResolvedSymbolIssue, ResolvedSymbolIssueKind, ResolvedSymbolPin,
    ResolvedSymbolSource, SymbolResolver,
};
pub use workspace::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableScopeKind,
    DesignVariableSweepEligibility, OpenCellView, OwnedNetlistDescriptor, OwnedNetlistEditStrategy,
    OwnedNetlistSaveRecord, PROJECT_DESCRIPTOR_SCHEMA_VERSION,
    PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION, ProjectConfigurationMutationError,
    ProjectDescriptor, ProjectDescriptorError, ProjectTechnologyBinding, ProjectWorkspace,
    RegressionComparisonMethod, RegressionComparisonWindow, RegressionTargetKind,
    RegressionTargetSelector, RegressionToleranceRule, ResolvedHierarchyBinding, SavedOutput,
    SavedOutputCompatibility, SavedOutputCompatibilityKind, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, SimulationConfigurationError,
    SimulationPlanPayload, SimulationPlanPayloadRecord, SpecEntry, TechnologyBindingError,
};

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
    fn reconcile_ports_adds_new_unplaced_pins_without_overwriting_existing_art() {
        let mut doc = SymbolDocument::generated_from_ports(&ota_ports());
        let original_inp = doc.pin("INP").expect("INP exists").position;
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
        assert_eq!(doc.pin("IBIAS").expect("new pin exists").position, None);
        assert_eq!(doc.pin_summary(&ports), PinSummary::Unplaced(1));
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
