//! Project-owned documents, configuration and revisioned catalogs.

use crate::OpenCellView;
use rspice_app_types::product::{
    AnalysisInstanceId, CaptureGroupId, ContentDigest, DesignVariableId, ObjectRevision,
    ResultDocumentId, RevisionError, SavedOutputId, SimulationPlanId, SpecificationId,
};
use rspice_design::library::ViewType;
use rspice_design::owned_netlist::*;
use rspice_design::project_sources::*;
use rspice_design_model::cell_view::{CellViewRef, DEFAULT_SCHEMATIC_VIEW};
use rspice_project_contract::*;
use rspice_results::specification::*;
use rspice_simulation_contract::capture_group::{self, CaptureGroup, CaptureGroupError};
use rspice_simulation_contract::design_variable::*;
use rspice_simulation_contract::plan_payload::{
    SimulationPlanPayload, SimulationPlanPayloadRecord,
};
use rspice_simulation_contract::regression_policy::*;
use rspice_simulation_contract::saved_output::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use uuid::Uuid;
mod annotation_restore;
pub use annotation_restore::PreparedAnnotation;
mod catalog;
mod errors;
mod hierarchy;
mod open_documents;
mod operations;
pub use operations::bind_generated_netlist_provenance;
mod plan_data;
mod reference_changes;
mod reference_preparation;
pub use catalog::PreparedPhysicalLayoutCatalog;
pub use errors::*;
pub use hierarchy::ProjectHierarchy;
pub use reference_changes::{PreparedReferences, ReferenceChanges};
pub use reference_preparation::{ProjectReferenceTransaction, reference_from_key};
pub const MAX_PROJECT_VISUALIZATION_DOCUMENTS: usize = 1_024;
pub const MAX_PROJECT_HARDCOPY_SOURCE_SETS: usize = 64;

/// Project-level workspace state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectWorkspace {
    pub project: ProjectDescriptor,
    /// Project-owned hierarchy/view-resolution authority. Empty catalogs
    /// preserve legacy deterministic resolution; once populated, the active
    /// configuration is the exact authority used by preflight and netlisting.
    #[serde(default)]
    pub configuration_sets: rspice_design::configuration_set::ConfigurationSetCatalog,
    /// Project-owned schematic sheet, assembly variant, annotation, and
    /// hierarchy-audit authority. The catalog is deliberately separate from
    /// simulation configuration sets: it describes design identity, while a
    /// configuration set describes how that identity is executed.
    #[serde(default)]
    pub design_management: rspice_design_model::design_management::DesignManagementCatalog,
    /// Project-owned bus-width and global-net policy. Older projects migrate
    /// to strict fail-closed defaults instead of inheriting UI state.
    #[serde(default)]
    pub connectivity: rspice_design::connectivity_contract::ConnectivityContract,
    pub active_view: CellViewRef,
    pub open_views: Vec<OpenCellView>,
    /// Masters on the active document's occurrence, outermost first — a
    /// runtime projection of `open_views[active].occurrence`, never the place
    /// it is stored. Deserialized so a save written before documents owned
    /// their occurrence can be folded onto the document it described, and
    /// never serialized back.
    #[serde(default, skip_serializing)]
    pub hierarchy_stack: Vec<CellViewRef>,
    /// Instance names on the same occurrence, aligned with
    /// `hierarchy_stack[1..]`, and a projection for the same reason.
    #[serde(default, skip_serializing)]
    pub hierarchy_instances: Vec<String>,
    pub schematic_buffers: HashMap<String, rspice_design::schematic::owned::Schematic>,
    /// Measurement specifications for the results specs matrix. Project
    /// design intent, so it persists with the workspace.
    #[serde(default)]
    pub specs: Vec<SpecEntry>,
    /// Plan-owned variables, output contracts, and specifications. Projects
    /// predating this feature migrate the active legacy `specs` projection
    /// into one record after execution-context migration.
    #[serde(default)]
    pub simulation_plan_payloads: Vec<SimulationPlanPayloadRecord>,
    /// Authoritative physical geometry for layout cell views, keyed by the
    /// exact `library/cell/view` owner. Documents use integral PDK database
    /// units and expected-revision transactions; no schematic or display
    /// geometry is projected into this store.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    physical_layout_documents:
        BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>,
    /// Project-owned, append-only evidence for exact signed-PDK callback
    /// executions. Each entry retains canonical inputs, derived metadata,
    /// package/runtime provenance, active-plan identity, and project revision
    /// authority; callback output is never accepted as ambient mutable state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pdk_callback_receipts: Vec<rspice_model_library::pdk::callback::ProjectPdkCallbackReceipt>,
    /// Versioned, per-document Page Setup contracts used by schematic,
    /// symbol, result, and report hardcopy workflows. Publication artifacts
    /// and transient preview state are intentionally not persisted here.
    #[serde(default)]
    pub hardcopy_setups: rspice_hardcopy_contract::HardcopySetupStore,
    /// Bounded, digest-sealed outcome history for print and export
    /// publications. Failures and cancellations are retained alongside
    /// successful artifacts so project evidence never implies more than the
    /// platform actually accepted.
    #[serde(default)]
    pub hardcopy_receipts: rspice_hardcopy_contract::HardcopyReceiptLedger,
    /// Reusable print-mapping sets owned by this project. Personal portable
    /// presets are persisted by `UserPreferences`; document mappings remain
    /// embedded in `hardcopy_setups` for reproducible publication.
    #[serde(default)]
    pub project_print_mappings: rspice_hardcopy_contract::PrintMappingPresetCatalog,
    /// Project-owned named engineering-table views. Working and personal
    /// views are device preferences; only explicitly project-scoped views
    /// participate in project revisioning and collaboration.
    #[serde(default)]
    pub engineering_table_views: rspice_results::engineering_table::EngineeringTableViewStore,
    /// Ordered, exact source aggregates used by all-sheets/all-panes and
    /// named print-set publication. Every member pins its document revision
    /// and content digest; stale members fail closed when resolved.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    hardcopy_source_sets: Vec<rspice_hardcopy_contract::sources::HardcopySourceSet>,
    /// Project-owned, versioned engineering report sources. Rendered review
    /// artifacts are derived from these documents and are never represented
    /// here unless a publication writer has produced and verified them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report_documents: Vec<rspice_results::report_document::ReportDocument>,
    /// Project-owned, dataset-bound result documents. Immutable solver
    /// datasets remain owned by result history; each visualization document
    /// pins exact dataset digests and owns only its versioned presentation
    /// graph.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visualization_documents: Vec<rspice_results::visualization_document::VisualizationDocument>,
    /// Reusable stimulus definitions this project owns. Placed sources adopt
    /// them by copy and keep a provenance receipt, so nothing here is read
    /// during netlist generation or execution.
    #[serde(
        default,
        skip_serializing_if = "rspice_design::stimulus_library::library::StimulusLibrary::is_empty"
    )]
    pub stimulus_library: rspice_design::stimulus_library::library::StimulusLibrary,
    /// Manually edited netlist source. When set, simulations run this
    /// deck instead of regenerating from the schematic (text-first mode);
    /// `None` means the netlist view shows the generated artifact.
    #[serde(default)]
    pub netlist_source: Option<String>,
    /// Canonical owned-source identity, provenance, generated base, sealed
    /// dependency metadata, revision, and validation evidence. The legacy
    /// `netlist_source` projection remains for backwards compatibility and
    /// must exactly match this document when both are present.
    #[serde(default)]
    pub netlist_document: Option<rspice_design::netlist_document::NetlistDocument>,
    /// Ownership-dialog selection for the project-owned source artifact.
    #[serde(default)]
    pub netlist_descriptor: Option<OwnedNetlistDescriptor>,
    /// Complete inactive top-level source decks. Only the active deck is
    /// projected into `netlist_source`/`netlist_document` and may execute.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub retained_netlist_decks: Vec<RetainedOwnedNetlistDeck>,
    /// Project-owned source documents shown by the Verilog-A and Automation
    /// pages of the Code workspace. Older projects intentionally restore an
    /// empty registry rather than receiving demonstration content.
    #[serde(default, skip_serializing_if = "ProjectSourceRegistry::is_empty")]
    pub project_sources: ProjectSourceRegistry,
    /// Native filesystem origin for `netlist_source`, used to resolve relative
    /// `.include`/`.lib` paths for imported decks. Edits retain this origin:
    /// changing document bytes does not change the directory against which its
    /// authored relative dependencies resolve. Browser imports have no native
    /// path authority and therefore leave it absent.
    #[serde(default)]
    pub netlist_source_path: Option<PathBuf>,
    /// Runtime dirty bit for `netlist_source`; skipped because dirty state is
    /// session-local while the source itself is persisted with the project.
    #[serde(default, skip)]
    pub netlist_source_dirty: bool,
    /// Runtime dirty state for `project_sources`. Source bytes and validation
    /// identities persist; dirty state is derived against the accepted project
    /// and remains session-local.
    #[serde(default, skip)]
    pub project_sources_dirty: bool,
    /// Runtime dirty state for project-owned metadata such as an exact
    /// technology attachment. The binding itself persists in `project`.
    #[serde(default, skip)]
    #[doc(hidden)]
    pub project_metadata_dirty: bool,
    /// Runtime dirty projection for project-owned report sources. The report
    /// documents themselves persist; accepted-baseline comparison remains the
    /// canonical save/revert authority.
    #[serde(default, skip)]
    pub report_documents_dirty: bool,
    /// Runtime dirty projection for project-owned result documents.
    #[serde(default, skip)]
    pub visualization_documents_dirty: bool,
    /// Runtime dirty projection for committed per-document page setups.
    #[serde(default, skip)]
    pub hardcopy_setups_dirty: bool,
    /// Runtime dirty projection for the durable hardcopy outcome ledger.
    #[serde(default, skip)]
    pub hardcopy_receipts_dirty: bool,
    /// Runtime dirty projection for reusable project-owned print mappings.
    #[serde(default, skip)]
    pub project_print_mappings_dirty: bool,
    /// Runtime dirty projection for project-owned hardcopy source sets.
    #[serde(default, skip)]
    hardcopy_source_sets_dirty: bool,
}

impl Default for ProjectWorkspace {
    fn default() -> Self {
        let active_view = CellViewRef::default_top();
        let schematic_buffers = HashMap::from([(
            active_view.key(),
            rspice_design::schematic::owned::Schematic::default(),
        )]);

        Self {
            project: ProjectDescriptor::default(),
            configuration_sets: rspice_design::configuration_set::ConfigurationSetCatalog::default(
            ),
            design_management:
                rspice_design_model::design_management::DesignManagementCatalog::default(),
            connectivity: rspice_design::connectivity_contract::ConnectivityContract::default(),
            active_view: active_view.clone(),
            open_views: vec![OpenCellView::new(active_view.clone(), ViewType::Schematic)],
            hierarchy_stack: vec![active_view],
            hierarchy_instances: Vec::new(),
            schematic_buffers,
            specs: Vec::new(),
            simulation_plan_payloads: Vec::new(),
            physical_layout_documents: BTreeMap::new(),
            pdk_callback_receipts: Vec::new(),
            hardcopy_setups: rspice_hardcopy_contract::HardcopySetupStore::default(),
            hardcopy_receipts: rspice_hardcopy_contract::HardcopyReceiptLedger::default(),
            project_print_mappings: rspice_hardcopy_contract::PrintMappingPresetCatalog::new(
                rspice_hardcopy_contract::PrintMappingCatalogOwner::Project,
            ),
            engineering_table_views:
                rspice_results::engineering_table::EngineeringTableViewStore::default(),
            hardcopy_source_sets: Vec::new(),
            report_documents: Vec::new(),
            visualization_documents: Vec::new(),
            stimulus_library: rspice_design::stimulus_library::library::StimulusLibrary::default(),
            netlist_source: None,
            netlist_document: None,
            netlist_descriptor: None,
            retained_netlist_decks: Vec::new(),
            project_sources: ProjectSourceRegistry::default(),
            netlist_source_path: None,
            netlist_source_dirty: false,
            project_sources_dirty: false,
            project_metadata_dirty: false,
            report_documents_dirty: false,
            visualization_documents_dirty: false,
            hardcopy_setups_dirty: false,
            hardcopy_receipts_dirty: false,
            project_print_mappings_dirty: false,
            hardcopy_source_sets_dirty: false,
        }
    }
}
