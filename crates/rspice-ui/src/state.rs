//! State Management Module
//!
//! Application state for simulation, project, and UI state.
//! Core data structures that are shared across multiple modules.

mod configuration_set;
mod connectivity_contract;
mod design_management;
pub(crate) mod engineering_table;
pub(crate) mod library_browser;
mod model_bound_symbol;
pub(crate) mod model_library;
pub(crate) mod netlist_document;
pub(crate) mod params_string;
pub(crate) mod pdk_config;
mod physical_layout;
mod project_sources;
pub(crate) mod property_types;
mod schematic;
mod simulation;
mod symbol;
pub(crate) mod symbol_resolver;
pub(crate) mod workspace;

pub use configuration_set::{
    ConfigurationBlackBoxPolicy, ConfigurationCloneScope, ConfigurationModelProfile,
    ConfigurationPlatform, ConfigurationSet, ConfigurationSetCatalog, ConfigurationSetDefinition,
    ConfigurationSetError, ConfigurationSetId, ConfigurationSetOverride, UnresolvedBindingPolicy,
};
pub use connectivity_contract::{
    BundleDirection, BundleDiscipline, BundleExpansionPolicy, BundleIndexOrderPolicy,
    BundleWidthMismatchPolicy, ConnectivityContract, ConnectivityPolicy,
    GlobalAliasComparisonPolicy, GlobalNetPromotionPolicy, LocalGlobalShadowingPolicy,
    MAX_NAMED_BUNDLE_MEMBERS, NamedSignalBundleMember, QualifiedNetReference,
};
// Test-only aliases: the submodule is private, so this path is the only
// way the tests can name these.
#[cfg(test)]
pub use configuration_set::ALLOWED_EXECUTABLE_VIEW_TYPES;
#[cfg(test)]
pub use connectivity_contract::{
    ConnectivityAliasGroup, DialectAliasCatalog, TechnologyGlobalNetCatalog,
};
pub use design_management::*;
pub use engineering_table::{
    EngineeringDataset, EngineeringFilterGrammar, EngineeringSortRule, EngineeringTableView,
    EngineeringTableViewStore, EngineeringViewScope, EngineeringVirtualizationPolicy,
    FrozenIdentifierPolicy, SavedEngineeringTableView, SortDirection,
};
pub use library_browser::{
    Cell, Library, LibraryCellPlacementCandidate, LibraryManager, ProjectLibraryLockAuthority,
    View, ViewType, library_cell_placement_candidates,
};
pub use model_bound_symbol::{
    GeneratedSymbolViews, MODEL_BOUND_SYMBOL_METADATA_KEY, MODEL_BOUND_SYMBOL_SCHEMA_VERSION,
    ModelBoundSymbolDefinition, ParameterInheritance, SymbolDefinitionImport, SymbolElectricalType,
    SymbolFormDiagnostic, SymbolGraphicTemplate, SymbolIdentity, SymbolImplementationView,
    SymbolModelReference, SymbolNetlistBinding, SymbolParameterConstraints, SymbolParameterDefault,
    SymbolParameterField, SymbolParameterForm, SymbolParameterSection, SymbolParameterVisibility,
    SymbolPinDefinition, SymbolPinSide, SymbolSourceContract,
};
pub use model_library::ModelLibraryManager;
pub(crate) use netlist_document::parse_include_directives;
pub use netlist_document::{
    DependencyMetadata, DependencyResolution, DiagnosticSeverity, DocumentOwnership, FindDirection,
    FindError, FindMatch, FindOptions, GeneratedArtifact, GeneratedProvenance,
    GeneratedSourceMapEntry, GenerationInput, NetlistDocument, NetlistDocumentId, NetlistOutline,
    OutlineEntry, OutlineEntryKind, ReplaceScope, SourceLocator, ValidationDiagnostic,
    content_digest, find_all_in_source, replace_in_source,
};
pub use params_string::{format_params_string, parse_params_string};
pub use physical_layout::{
    LayoutDocumentError, LayoutEdit, LayoutGeometry, LayoutLayerPurpose, LayoutObjectId,
    LayoutPoint, LayoutShape, LayoutTechnologyBinding, PhysicalLayoutDocument,
};
#[cfg(test)]
pub use physical_layout::{LayoutInstance, LayoutOrientation, LayoutTransform};
#[cfg(test)]
pub use project_sources::ProjectSourceDependency;
pub(crate) use project_sources::{CanonicalCellViewOwnerKey, canonical_cell_view_owner_key};
pub use project_sources::{
    MAX_PROJECT_CODE_SOURCE_BYTES, MAX_PROJECT_SOURCE_BUNDLE_BYTES,
    MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH, MAX_PROJECT_SOURCE_FILES,
    MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES, PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION,
    ProjectSourceBundle, ProjectSourceDocument, ProjectSourceFile, ProjectSourceId,
    ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry,
    project_veriloga_bundle_alias, project_veriloga_bundle_source_key,
};
pub use property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertySheet, PropertyType, PropertyValue,
    format_engineering,
};
pub use schematic::*;
pub use simulation::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultSourceDomain, AnalysisType, ComplexResultValue,
    DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, DcOpResult, ExecutionTarget,
    MonteCarloVariableMetadata, NoiseContributorRow, NoiseSummary, OperatingPointAccuracyEvidence,
    OperatingPointAnnotationEvidence, OperatingPointDeviceDetailEvidence,
    OperatingPointHomotopyEvidence, OperatingPointInitialGuessEvidence,
    OperatingPointNodeInitializationEvidence, OperatingPointProcessEvidence,
    OperatingPointSaveDeviceEvidence, OperatingPointTemperatureEvidence, OperatingPointValue,
    PeriodicNoiseOutputQuantity, PreparedModelSourceIdentity, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, ReliabilityCheckpointEvidence,
    ReliabilityDeviceEvidence, ReliabilityShiftEvidence, ReliabilityStressEvidence,
    SavedOutputMaterializationStatus, SavedOutputReceipt, SensitivityResultMode,
    SensitivityResultRow, SharedWaveformValues, SimulationRun, SimulationRunIntent,
    SimulationRunLifecycle, SimulationRunProvenance, SimulationState, SoaEvaluationEvidence,
    SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence, TransferFunctionAccuracyEvidence,
    TransferFunctionNormalizationEvidence, TransferFunctionQuantityEvidence,
    TransferFunctionScalarEvidence, WaveformData, ac_bode_summary_for_selection,
};
pub use symbol::{
    MAX_SYMBOL_DOCUMENT_BYTES, MAX_SYMBOL_PIN_NAME_BYTES, MAX_SYMBOL_TEXT_BYTES, PinFindingKind,
    PinSummary, SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_EDITOR_METADATA_KEY, SYMBOL_TERMINAL_GRID,
    SymbolAttribute, SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata, SymbolPin,
    SymbolPinElectricalKind, SymbolShape, SymbolTextObject,
};
pub use symbol_resolver::{
    ResolvedCellSymbol, ResolvedSymbolIssueKind, ResolvedSymbolSource, SymbolResolver,
};
pub(crate) use workspace::PreparedProjectLibraryMutation;
pub use workspace::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, OpenCellView,
    OwnedNetlistDescriptor, OwnedNetlistEditStrategy, OwnedNetlistSaveRecord,
    PROJECT_DESCRIPTOR_SCHEMA_VERSION, PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
    ProjectCloudPublicationBinding, ProjectDescriptor, ProjectLibraryMutation,
    ProjectTechnologyBinding, ProjectTechnologyChangeAuthority, ProjectTechnologyChangeContext,
    ProjectWorkspace, RegressionComparisonMethod, RegressionComparisonWindow, RegressionTargetKind,
    RegressionTargetSelector, RegressionToleranceRule, ResolvedHierarchyBinding, SavedOutput,
    SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming, SimulationPlanPayload, SimulationPlanPayloadRecord, SpecEntry,
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
