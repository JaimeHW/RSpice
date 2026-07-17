//! State Management Module
//!
//! Application state for simulation, project, and UI state.
//! Core data structures that are shared across multiple modules.

pub mod library_browser;
pub mod model_library;
pub mod pdk_config;
pub mod property_types;
mod schematic;
mod simulation;
mod symbol;
pub mod symbol_resolver;
pub mod workspace;

pub use library_browser::{Cell, Library, LibraryManager, NavColumn, View, ViewType};
pub use model_library::ModelLibraryManager;
pub use pdk_config::{ConfigError, DiscoveredFile, LibraryPathEntry, PdkConfig};
pub use property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertySheet, PropertyType, PropertyValue,
    VisibilityCondition, format_engineering,
};
pub use schematic::*;
pub use simulation::{
    AcBodeMetrics, AcBodeSummary, AnalysisResult, AnalysisResultProvenance,
    AnalysisResultSourceDomain, AnalysisType, CrossProbeMapping,
    DEFAULT_DISPLAY_WAVEFORM_CACHE_SAMPLES, DcOpResult, DisplayWaveformCache, NoiseContributorRow,
    NoiseSummary, OperatingPointValue, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, SavedOutputMaterializationStatus, SavedOutputReceipt,
    SharedWaveformValues, SimulationRun, SimulationRunIntent, SimulationRunProvenance,
    SimulationState, WaveformData, ac_bode_summary_for_run, ac_bode_summary_for_selection,
    ac_bode_summary_for_source_instance,
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
    DesignVariableSweepEligibility, MAX_PROJECT_CODE_SOURCE_BYTES, OpenCellView,
    OwnedNetlistDescriptor, OwnedNetlistEditStrategy, OwnedNetlistSaveRecord,
    PROJECT_DESCRIPTOR_SCHEMA_VERSION, PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
    ProjectDescriptor, ProjectDescriptorError, ProjectSourceDocument, ProjectSourceError,
    ProjectSourceLanguage, ProjectSourceRegistry, ProjectSourceValidationIdentity,
    ProjectTechnologyBinding, ProjectWorkspace, RegressionComparisonMethod,
    RegressionComparisonWindow, RegressionTargetKind, RegressionTargetSelector,
    RegressionToleranceRule, SavedOutput, SavedOutputCompatibility, SavedOutputCompatibilityKind,
    SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    SimulationConfigurationError, SimulationPlanPayload, SimulationPlanPayloadRecord, SpecEntry,
    TechnologyBindingError,
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
