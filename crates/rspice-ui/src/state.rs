//! State Management Module
//!
//! Application state for simulation, project, and UI state.
//! Core data structures that are shared across multiple modules.

mod configuration_set;
mod connectivity_contract;
pub(crate) mod engineering_table;
mod hierarchy_path;
pub(crate) mod include_search;
pub(crate) mod library_browser;
mod model_bound_symbol;
pub(crate) mod model_hub;
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

#[cfg(test)]
thread_local! {
    /// How many symbol cellviews have been deserialized on this thread.
    ///
    /// A symbol cellview keeps its typed model binding and its artwork as two
    /// JSON documents in the view's metadata, and reading either one costs a
    /// deserialization plus a validation pass. That is the expensive half of
    /// deriving anything about a symbol, it depends only on the view, and a
    /// surface that derives its rows from the whole registry on every frame
    /// pays it per symbol in the corpus rather than per row on screen — which
    /// is what the Symbols page did. Counting is the only way to state that as
    /// a test: the work is invisible in the result, and a timing assertion on a
    /// fixture small enough to build is noise.
    pub(crate) static SYMBOL_VIEW_PARSES: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };

    /// How many model libraries have been serialized whole on this thread.
    ///
    /// Canonicalizing a library routes it through `serde_json::Value`, which
    /// allocates a node per model, a node per parameter of every model, and —
    /// retained source bytes being a `Vec<u8>` — a node per byte of every
    /// pinned source file, so one pass over a production catalogue is the whole
    /// corpus several times over. The result is only ever *compared*, so the
    /// cost leaves no trace in what a frame paints; counting is the only way to
    /// state it as a test.
    pub(crate) static CATALOG_LIBRARY_SERIALIZATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

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
    EngineeringTableViewStore, EngineeringViewScope, EngineeringVirtualizationPolicy,
    FrozenIdentifierPolicy, SavedEngineeringTableView, SortDirection,
};
// The one hierarchical path grammar. Every consumer of an instance path,
// pattern, or probe target names these types; nothing else may split a path.
pub use hierarchy_path::{
    HierarchyPathError, InstancePath, InstancePathPattern, MAX_INSTANCE_PATH_BYTES,
    MAX_INSTANCE_PATH_DEPTH, PatternSegment, ProbeTarget,
};
// The one include search chain. Every host-file parse and every dependency
// acquisition walks this, so a relative name resolves once.
pub use include_search::IncludeSearchChain;
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
pub use netlist_document::{
    BoundedFindMatches, DependencyMetadata, DependencyResolution, DependencySourceAuthority,
    DiagnosticSeverity, DocumentOwnership, FindDirection, FindError, FindMatch, FindOptions,
    GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
    NetlistDocument, NetlistDocumentId, NetlistSourceIndex, OutlineEntry, OutlineEntryKind,
    OutlineSection, OutlineSectionKind, SourceLocator, ValidationDiagnostic, content_digest,
    find_all_in_source_bounded, replace_source_ranges,
};
pub(crate) use netlist_document::{
    card_tokens, expand_retained_netlist_dependencies, find_all_in_source_range_bounded_filter,
    parse_include_directives, same_include_graph,
};
pub use params_string::{format_params_string, parse_params_string};
pub use physical_layout::{
    LayoutDocumentError, LayoutEdit, LayoutGeometry, LayoutLayerPurpose, LayoutObjectId,
    LayoutPoint, LayoutShape, LayoutTechnologyBinding, PhysicalLayoutDocument,
};
#[cfg(test)]
pub use physical_layout::{LayoutInstance, LayoutOrientation, LayoutTransform};
#[cfg(not(target_arch = "wasm32"))]
pub use project_sources::MAX_PROJECT_SOURCE_DEPENDENCIES;
pub use project_sources::{
    AutomationStarterFile, DEFAULT_AUTOMATION_PERMISSIONS, DEFAULT_AUTOMATION_PYTHON,
    DEFAULT_AUTOMATION_RUN_PLAN, DEFAULT_ENVIRONMENT_LOCK, MAX_PROJECT_CODE_SOURCE_BYTES,
    MAX_PROJECT_SOURCE_BUNDLE_BYTES, MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH, MAX_PROJECT_SOURCE_FILES,
    MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES, MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS,
    PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION, ProjectSourceBundle, ProjectSourceDependency,
    ProjectSourceDocument, ProjectSourceFile, ProjectSourceId, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceQualificationCheck, ProjectSourceQualificationDisposition,
    ProjectSourceQualificationRecord, ProjectSourceQualificationTarget, ProjectSourceRegistry,
    ProjectSourceRole, ProjectSourceRoleBinding, project_veriloga_bundle_alias,
    project_veriloga_bundle_source_key,
};
pub(crate) use project_sources::{
    CanonicalCellViewOwnerKey, canonical_cell_view_owner_key, project_source_path_key,
    project_source_paths_equal,
};
pub use property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertySheet, PropertyType, PropertyValue,
    format_engineering,
};
// The design-management model is a crate of its own so the offline
// drawing-sheet publisher can link it without linking the GUI. It is
// re-exported here because it is still the application's state authority and
// every caller names it through `crate::state`.
pub use rspice_design_model::design_management::*;
pub use schematic::*;
// Test-only aliases: the submodule is private, so this path is the only way
// the tests can name an attribution's vocabulary directly.
#[cfg(test)]
pub use simulation::ConvergenceFailureClass;
pub use simulation::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisResultPvtPoint, AnalysisResultSourceDomain, AnalysisType, CanonicalAnalysisKind,
    ComplexResultValue, ConvergenceAttribution, CrossProbeIndex,
    DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, DcOpResult, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, EvidenceDomain, ExecutedDeck, ExecutedDeckArchive,
    ExecutedDeckPoint, ExecutionTarget, FamilyMeasurementEvidence, FamilyMemberId,
    FamilyMemberMeasurements, HierarchyMapRow, MonteCarloVariableMetadata, NoiseContributorRow,
    NoiseSummary, OccurrenceProbeSpelling, OperatingPointAccuracyEvidence,
    OperatingPointAnnotationEvidence, OperatingPointDeviceDetailEvidence,
    OperatingPointHomotopyEvidence, OperatingPointInitialGuessEvidence,
    OperatingPointNodeInitializationEvidence, OperatingPointProcessEvidence,
    OperatingPointSaveDeviceEvidence, OperatingPointTemperatureEvidence, OperatingPointValue,
    PeriodicNoiseOutputQuantity, PreparedModelQualification, PreparedModelSourceIdentity,
    PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt, PreparedSpecification,
    PreparedSpecificationPolicy, RealEventPointEvidence, RealEventTraceEvidence,
    ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence, ReliabilityShiftEvidence,
    ReliabilityStressEvidence, RunRetention, SavedOutputMaterializationStatus, SavedOutputReceipt,
    SensitivityResultMode, SensitivityResultRow, SharedWaveformValues,
    SimulationCampaignMembership, SimulationRun, SimulationRunIntent, SimulationRunLifecycle,
    SimulationRunProvenance, SimulationState, SoaEvaluationEvidence, SoaParameterEvidence,
    SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence,
    SpecificationVerdict, SpecificationVerdictStatus, TransferFunctionAccuracyEvidence,
    TransferFunctionNormalizationEvidence, TransferFunctionQuantityEvidence,
    TransferFunctionScalarEvidence, WaveformData, ac_bode_summary_for_analysis,
    ac_bode_summary_for_selection, sealed_model_sources,
};
pub use symbol::{
    MAX_SYMBOL_DOCUMENT_BYTES, MAX_SYMBOL_PIN_NAME_BYTES, MAX_SYMBOL_TEXT_BYTES, PinFindingKind,
    PinSummary, SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_EDITOR_METADATA_KEY, SYMBOL_TERMINAL_GRID,
    SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata, SymbolPin, SymbolPinElectricalKind,
    SymbolShape, SymbolTextAlign, SymbolTextPlacement, SymbolTextSize, pin_side_against_body,
    symbol_text_bounds,
};
pub use symbol_resolver::{
    ResolvedCellSymbol, ResolvedSymbolIssueKind, ResolvedSymbolSource, SymbolResolver,
};
pub(crate) use workspace::PreparedProjectLibraryMutation;
pub use workspace::{
    CellViewRef, DesignVariable, DesignVariableDefect, DesignVariableOverridePolicy,
    DesignVariableQuantity, DesignVariableRange, DesignVariableScope,
    DesignVariableSweepEligibility, MissingMeasurementPolicy, MonteCarloSpecificationGate,
    NetlistExecutionProfile, NetlistLineEnding, NetlistSourceDialect, NetlistTextEncoding,
    NominalFailurePolicy, OpenCellView, OutputSelectionMode, OwnedNetlistDescriptor,
    OwnedNetlistEditStrategy, OwnedNetlistIncludeDescriptor, OwnedNetlistSaveRecord,
    PROJECT_DESCRIPTOR_SCHEMA_VERSION, PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
    ProjectCloudPublicationBinding, ProjectDescriptor, ProjectLibraryMutation,
    ProjectTechnologyBinding, ProjectTechnologyChangeAuthority, ProjectTechnologyChangeContext,
    ProjectWorkspace, RegressionComparisonMethod, RegressionComparisonWindow,
    RegressionSpecificationPolicy, RegressionTargetKind, RegressionTargetSelector,
    RegressionToleranceRule, ResolvedHierarchyBinding, RetainedOwnedNetlistDeck, SavedOutput,
    SavedOutputCompatibility, SavedOutputDisplayIntent, SavedOutputKind, SavedOutputOrigin,
    SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SimulationPlanPayload,
    SimulationPlanPayloadRecord, SpecEntry, SpecPointScope, SpecificationComparison,
    SpecificationDefinition, SpecificationPolicy, SpecificationRole,
    validate_owned_netlist_artifact_path,
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
