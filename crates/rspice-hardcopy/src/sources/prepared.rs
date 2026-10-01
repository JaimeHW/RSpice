//! Owned hardcopy inputs and authenticated native/worker source resolution.
//!
//! Capture checks identity and shape. Worker decoding also checks byte budgets,
//! the closed schema, canonical owners and transport digest before resolution.

// A prepared source is the sealed form of a whole engineering document, so
// the schematic variant is inherently far larger than the symbol or results
// ones. Exactly one exists per publication, built immediately before the run
// that consumes it, so the size the lint measures is never multiplied.
#![allow(clippy::large_enum_variant)]

use super::*;
use rspice_app_types::product::{ObjectRevision, ProjectId};
use rspice_design::schematic::{
    bus::{Bus, BusTap},
    design_note::DesignNote,
    documentation_shape::DocumentationShape,
    net_label::{Junction, NetLabel},
    selection::Selection,
    wire::Wire,
};
use rspice_design_model::design_management::{
    DrawingSheetInheritance, DrawingSheetTitleFieldId, SheetCatalog, SheetId,
    validate_project_drawing_sheet_title_field_values,
};
use rspice_formats::project_results::ProjectSimulationResults;
use rspice_hardcopy_contract::sources::MAX_HARDCOPY_SOURCE_SET_MEMBERS;
use rspice_results::{
    report_document::{ReportDocument, ReportReferenceInventory},
    studio_presentation::VisualizationStudioPresentation,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

const PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION: u32 = 8;

#[cfg(test)]
mod tests;

/// Owned, `Send`-safe retained-source snapshot prepared on the UI thread
/// without hashing samples, resolving symbols, or constructing semantic
/// geometry. The worker consumes it with [`Self::resolve_owned`].
pub struct PreparedRetainedHardcopyResolution {
    payload: RetainedHardcopySourceInput,
}

/// Unprepared source owners captured by a host; these inputs grant no publication permission.
pub enum RetainedHardcopySourceInput {
    Schematic {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        schematic: rspice_design::schematic::owned::Schematic,
        selection: Selection,
        library_manager: rspice_project_contract::ProjectLibraries,
        schematic_buffers:
            std::collections::HashMap<String, rspice_design::schematic::owned::Schematic>,
        sheet_catalog: Option<SheetCatalog>,
        sheet_id: Option<SheetId>,
        project_default_drawing_sheet: SchematicSheetFormat,
        project_title_block_field_values:
            std::collections::BTreeMap<DrawingSheetTitleFieldId, String>,
        all_sheets: bool,
        scope: HardcopyScope,
    },
    Symbol {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        document: SymbolDocument,
        scope: HardcopyScope,
    },
    Results {
        source_key: String,
        project_id: ProjectId,
        run: HardcopyRun,
        presentation: ResultsQuickViewPresentation,
        scope: HardcopyScope,
    },
    Studio {
        source_key: String,
        project_id: ProjectId,
        studio: VisualizationStudioPresentation,
        runs: Vec<HardcopyRun>,
        pane_id: u64,
        all_panes: bool,
        scope: HardcopyScope,
    },
    VisualizationDocument {
        source_key: String,
        project_id: ProjectId,
        document: VisualizationDocument,
        page_id: PageId,
        pane_id: PaneId,
        all_panes: bool,
        scope: HardcopyScope,
    },
    Report {
        project_id: ProjectId,
        source_key: String,
        document: ReportDocument,
        reference_inventory: ReportReferenceInventory,
        scope: HardcopyScope,
    },
    SourceSet {
        source_set: HardcopySourceSet,
        members: Vec<PreparedRetainedHardcopyResolution>,
    },
}

/// A canonical owner value keeps the worker schema closed even when an
/// application-owned type accepts omitted/defaulted fields for project-file
/// migration. Decoding must reproduce the exact JSON value; an ignored
/// unknown field, non-canonical alias, or lossy default therefore fails before
/// any source resolution begins.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
struct CanonicalHardcopyOwner(serde_json::Value);

impl CanonicalHardcopyOwner {
    fn capture<T: Serialize>(field: &'static str, owner: &T) -> Result<Self, HardcopySourceError> {
        serde_json::to_value(owner).map(Self).map_err(|error| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{field} cannot be serialized: {error}"
            ))
        })
    }

    fn restore<T>(self, field: &'static str) -> Result<T, HardcopySourceError>
    where
        T: DeserializeOwned + Serialize,
    {
        let owner: T = serde_json::from_value(self.0.clone()).map_err(|error| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{field} is invalid: {error}"
            ))
        })?;
        let canonical = serde_json::to_value(&owner).map_err(|error| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{field} cannot be canonicalized: {error}"
            ))
        })?;
        if canonical != self.0 {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{field} contains unknown, aliased, or non-canonical fields"
            )));
        }
        Ok(owner)
    }
}

/// Exact schematic owner fields consumed by semantic hardcopy resolution.
/// Editor gestures, clipboard, viewport, history caches, and save paths never
/// cross the worker boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedSchematicOwner {
    components: Vec<Component>,
    wires: Vec<Wire>,
    buses: Vec<Bus>,
    bus_taps: Vec<BusTap>,
    junctions: Vec<Junction>,
    net_labels: Vec<NetLabel>,
    design_notes: Vec<DesignNote>,
    documentation_shapes: Vec<DocumentationShape>,
    selection: Selection,
}

impl PreparedSchematicOwner {
    fn capture(
        schematic: rspice_design::schematic::owned::Schematic,
        selection: Selection,
    ) -> Self {
        let document = schematic.into_document();
        Self {
            components: document.components,
            wires: document.wires,
            buses: document.buses,
            bus_taps: document.bus_taps,
            junctions: document.junctions,
            net_labels: document.net_labels,
            design_notes: document.design_notes,
            documentation_shapes: document.documentation_shapes,
            selection,
        }
    }

    fn restore(self) -> (rspice_design::schematic::owned::Schematic, Selection) {
        let schematic = rspice_design::schematic::owned::Schematic::from_document(
            rspice_design::schematic::document::SchematicDocument {
                components: self.components,
                wires: self.wires,
                buses: self.buses,
                bus_taps: self.bus_taps,
                junctions: self.junctions,
                net_labels: self.net_labels,
                design_notes: self.design_notes,
                documentation_shapes: self.documentation_shapes,
                ..Default::default()
            },
        );
        (schematic, self.selection)
    }
}

/// Hierarchical symbol fallback needs only ordered interface-port components
/// from each retained schematic cell, never the rest of its editor document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedSchematicInterfaceOwner {
    components: Vec<Component>,
}

impl PreparedSchematicInterfaceOwner {
    fn capture(schematic: rspice_design::schematic::owned::Schematic) -> Self {
        Self {
            components: schematic.into_document().components,
        }
    }

    fn restore(self) -> rspice_design::schematic::owned::Schematic {
        rspice_design::schematic::owned::Schematic::from_document(
            rspice_design::schematic::document::SchematicDocument {
                components: self.components,
                ..Default::default()
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source-family", rename_all = "kebab-case", deny_unknown_fields)]
enum PreparedRetainedHardcopyWorkerPayload {
    Schematic {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        schematic: CanonicalHardcopyOwner,
        library_manager: CanonicalHardcopyOwner,
        schematic_buffers: CanonicalHardcopyOwner,
        sheet_catalog: Option<CanonicalHardcopyOwner>,
        sheet_id: Option<SheetId>,
        project_default_drawing_sheet: SchematicSheetFormat,
        project_title_block_field_values:
            std::collections::BTreeMap<DrawingSheetTitleFieldId, String>,
        all_sheets: bool,
        scope: HardcopyScope,
    },
    Symbol {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        document: CanonicalHardcopyOwner,
        scope: HardcopyScope,
    },
    Results {
        source_key: String,
        project_id: ProjectId,
        simulation_results: CanonicalHardcopyOwner,
        presentation: ResultsQuickViewPresentation,
        scope: HardcopyScope,
    },
    Studio {
        source_key: String,
        project_id: ProjectId,
        studio: CanonicalHardcopyOwner,
        simulation_results: CanonicalHardcopyOwner,
        pane_id: u64,
        all_panes: bool,
        scope: HardcopyScope,
    },
    VisualizationDocument {
        source_key: String,
        project_id: ProjectId,
        document: CanonicalHardcopyOwner,
        page_id: PageId,
        pane_id: PaneId,
        all_panes: bool,
        scope: HardcopyScope,
    },
    Report {
        project_id: ProjectId,
        source_key: String,
        document: CanonicalHardcopyOwner,
        reference_inventory: ReportReferenceInventory,
        scope: HardcopyScope,
    },
    SourceSet {
        source_set: HardcopySourceSet,
        members: Vec<PreparedRetainedHardcopyWorkerPayload>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreparedRetainedHardcopyWorkerSnapshot {
    schema_version: u32,
    payload: PreparedRetainedHardcopyWorkerPayload,
    transport_digest: ContentDigest,
}

#[derive(Serialize)]
struct PreparedRetainedHardcopyWorkerDigestMaterial<'a> {
    schema_version: u32,
    payload: &'a serde_json::Value,
}

// Native capture and transport decoding enforce the same metadata shape.
macro_rules! validate_prepared_shape {
    ($payload:expr, $nested:expr) => {{
        match $payload {
            Self::Schematic {
                project_id,
                identity,
                sheet_catalog,
                sheet_id,
                project_default_drawing_sheet,
                project_title_block_field_values,
                all_sheets,
                scope,
                ..
            } => {
                validate_project_source_identity(*project_id, identity, "cell-view")?;
                validate_project_drawing_sheet_title_field_values(project_title_block_field_values)
                    .map_err(|error| {
                        HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
                    })?;
                project_default_drawing_sheet.validate().map_err(|error| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
                })?;
                if project_default_drawing_sheet.inheritance
                    != DrawingSheetInheritance::ProjectDefault
                {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared schematic project default has non-default inheritance".to_owned(),
                    ));
                }
                match (*all_sheets, sheet_catalog.is_some(), *sheet_id, scope) {
                    (true, true, None, HardcopyScope::AllSheetsOrPanes)
                    | (false, true, Some(_), HardcopyScope::CurrentSheet)
                    | (
                        false,
                        false,
                        None,
                        HardcopyScope::Selection
                        | HardcopyScope::CurrentSheet
                        | HardcopyScope::ActiveDocument,
                    )
                    | (
                        false,
                        true,
                        None,
                        HardcopyScope::Selection | HardcopyScope::ActiveDocument,
                    ) => {}
                    _ => {
                        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                            "schematic sheet selection and scope are inconsistent".to_owned(),
                        ));
                    }
                }
            }
            Self::Symbol {
                project_id,
                identity,
                scope,
                ..
            } => {
                validate_project_source_identity(*project_id, identity, "cell-view")?;
                if !matches!(scope, HardcopyScope::ActiveDocument) {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "symbol worker source has an unsupported scope".to_owned(),
                    ));
                }
            }
            Self::Results {
                source_key,
                project_id,
                presentation,
                scope,
                ..
            } => {
                validate_label("prepared result source key", source_key, SOURCE_KEY_LIMIT)?;
                presentation.validate()?;
                if !matches!(
                    scope,
                    HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
                ) {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "result worker source has an unsupported scope".to_owned(),
                    ));
                }
                require_project_source_prefix(*project_id, source_key, "result-dataset")?;
            }
            Self::Studio {
                source_key,
                project_id,
                pane_id,
                all_panes,
                scope,
                ..
            } => {
                validate_label("prepared studio source key", source_key, SOURCE_KEY_LIMIT)?;
                let expected_key = format!(
                    "project:{}:visualization-pane:{pane_id}",
                    project_id.as_uuid()
                );
                if source_key != &expected_key {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "studio pane identity does not match its source key".to_owned(),
                    ));
                }
                if (*all_panes && !matches!(scope, HardcopyScope::AllSheetsOrPanes))
                    || (!*all_panes
                        && !matches!(
                            scope,
                            HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
                        ))
                {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "studio aggregate flag and scope are inconsistent".to_owned(),
                    ));
                }
            }
            Self::VisualizationDocument {
                source_key,
                project_id,
                pane_id,
                all_panes,
                scope,
                ..
            } => {
                validate_label(
                    "prepared visualization-document source key",
                    source_key,
                    SOURCE_KEY_LIMIT,
                )?;
                require_project_source_prefix(*project_id, source_key, "result-document")?;
                let pane_suffix = format!(":pane:{}", pane_id.get());
                if !source_key.ends_with(&pane_suffix) {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "result-document pane identity does not match its source key".to_owned(),
                    ));
                }
                if (*all_panes && !matches!(scope, HardcopyScope::AllSheetsOrPanes))
                    || (!*all_panes
                        && !matches!(
                            scope,
                            HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
                        ))
                {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "result-document aggregate flag and scope are inconsistent".to_owned(),
                    ));
                }
            }
            Self::Report {
                project_id,
                source_key,
                reference_inventory,
                scope,
                ..
            } => {
                validate_label("prepared report source key", source_key, SOURCE_KEY_LIMIT)?;
                require_project_source_prefix(*project_id, source_key, "report")?;
                if !matches!(
                    scope,
                    HardcopyScope::CompleteReport | HardcopyScope::ActiveDocument
                ) {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "report worker source has an unsupported scope".to_owned(),
                    ));
                }
                reference_inventory.validate().map_err(|error| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
                })?;
            }
            Self::SourceSet {
                source_set,
                members,
            } => {
                if $nested {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared source sets cannot nest".to_owned(),
                    ));
                }
                source_set.validate()?;
                if members.len() != source_set.members().len()
                    || members.is_empty()
                    || members.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS
                {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared source-set members do not match its governed definition"
                            .to_owned(),
                    ));
                }
                for member in members {
                    member.validate_shape(true)?;
                }
            }
        }
        Ok(())
    }};
}

impl RetainedHardcopySourceInput {
    fn validate_shape(&self, nested: bool) -> Result<(), HardcopySourceError> {
        validate_prepared_shape!(self, nested)
    }

    fn validate_owner_bindings(&self) -> Result<(), HardcopySourceError> {
        match self {
            Self::Schematic {
                project_id,
                identity,
                sheet_catalog,
                sheet_id,
                ..
            } => validate_prepared_schematic_identity(
                *project_id,
                identity,
                sheet_catalog.as_ref(),
                *sheet_id,
            ),
            Self::Symbol {
                project_id,
                identity,
                ..
            } => validate_prepared_base_design_identity(*project_id, identity),
            Self::Results {
                source_key,
                project_id,
                run,
                presentation,
                ..
            } => validate_prepared_result_history(
                source_key,
                *project_id,
                std::slice::from_ref(run),
                presentation,
            ),
            Self::Studio {
                source_key,
                project_id,
                studio,
                runs,
                pane_id,
                all_panes,
                ..
            } => validate_prepared_studio_snapshot(
                *project_id,
                source_key,
                studio,
                runs,
                *pane_id,
                *all_panes,
            ),
            Self::VisualizationDocument {
                source_key,
                project_id,
                document,
                page_id,
                pane_id,
                ..
            } => validate_prepared_visualization_identity(
                source_key,
                *project_id,
                document,
                *page_id,
                *pane_id,
            ),
            Self::Report {
                source_key,
                project_id,
                document,
                ..
            } => validate_prepared_report_identity(source_key, *project_id, document),
            // Exact source-set identities include sample digests. The existing
            // resolver checks them after each member resolves, off the UI thread.
            Self::SourceSet { .. } => Ok(()),
        }
    }
}

impl PreparedRetainedHardcopyWorkerSnapshot {
    fn capture(prepared: PreparedRetainedHardcopyResolution) -> Result<Self, HardcopySourceError> {
        let payload = PreparedRetainedHardcopyWorkerPayload::capture(prepared.payload)?;
        let mut snapshot = Self {
            schema_version: PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION,
            payload,
            transport_digest: ContentDigest::from_bytes([0; 32]),
        };
        snapshot.validate_shape()?;
        snapshot.transport_digest = snapshot.compute_transport_digest()?;
        Ok(snapshot)
    }

    fn compute_transport_digest(&self) -> Result<ContentDigest, HardcopySourceError> {
        // Authenticate the canonical JSON value that crosses the worker
        // boundary. Hashing the typed payload directly let randomized map
        // iteration leak into serialization order; a valid snapshot could
        // then reject itself after JSON round-trip. `serde_json::Map` gives
        // the value a stable key order while preserving exact scalar values.
        let payload = serde_json::to_value(&self.payload)
            .map_err(|error| HardcopySourceError::Serialization(error.to_string()))?;
        canonical_digest(
            b"rspice-prepared-hardcopy-worker-snapshot-v2",
            &PreparedRetainedHardcopyWorkerDigestMaterial {
                schema_version: self.schema_version,
                payload: &payload,
            },
        )
    }

    fn validate(&self) -> Result<(), HardcopySourceError> {
        self.validate_shape()?;
        let actual = self.compute_transport_digest()?;
        if actual != self.transport_digest {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "transport digest does not authenticate the prepared owner snapshot (expected {}, computed {})",
                self.transport_digest, actual
            )));
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<(), HardcopySourceError> {
        if self.schema_version != PREPARED_WORKER_SNAPSHOT_SCHEMA_VERSION {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "unsupported schema version {}",
                self.schema_version
            )));
        }
        self.payload.validate_shape(false)
    }

    fn into_prepared(self) -> Result<PreparedRetainedHardcopyResolution, HardcopySourceError> {
        self.validate()?;
        Ok(PreparedRetainedHardcopyResolution {
            payload: self.payload.restore()?,
        })
    }
}

impl PreparedRetainedHardcopyWorkerPayload {
    fn capture(payload: RetainedHardcopySourceInput) -> Result<Self, HardcopySourceError> {
        Ok(match payload {
            RetainedHardcopySourceInput::Schematic {
                project_id,
                identity,
                schematic,
                selection,
                library_manager,
                schematic_buffers,
                sheet_catalog,
                sheet_id,
                project_default_drawing_sheet,
                project_title_block_field_values,
                all_sheets,
                scope,
            } => {
                let schematic = PreparedSchematicOwner::capture(schematic, selection);
                let mut library_manager = library_manager;
                library_manager.selected_library = None;
                library_manager.selected_cell = None;
                library_manager.selected_view = None;
                library_manager.filter_text.clear();
                library_manager.show_read_only = false;
                let schematic_buffers = schematic_buffers
                    .into_iter()
                    .map(|(key, schematic)| {
                        (key, PreparedSchematicInterfaceOwner::capture(schematic))
                    })
                    .collect::<std::collections::BTreeMap<_, _>>();
                Self::Schematic {
                    project_id,
                    identity,
                    schematic: CanonicalHardcopyOwner::capture("prepared schematic", &schematic)?,
                    library_manager: CanonicalHardcopyOwner::capture(
                        "prepared symbol library",
                        &library_manager,
                    )?,
                    schematic_buffers: CanonicalHardcopyOwner::capture(
                        "prepared schematic symbol buffers",
                        &schematic_buffers,
                    )?,
                    sheet_catalog: sheet_catalog
                        .as_ref()
                        .map(|catalog| {
                            CanonicalHardcopyOwner::capture("prepared sheet catalog", catalog)
                        })
                        .transpose()?,
                    sheet_id,
                    project_default_drawing_sheet,
                    project_title_block_field_values,
                    all_sheets,
                    scope,
                }
            }
            RetainedHardcopySourceInput::Symbol {
                project_id,
                identity,
                document,
                scope,
            } => Self::Symbol {
                project_id,
                identity,
                document: CanonicalHardcopyOwner::capture("prepared symbol document", &document)?,
                scope,
            },
            RetainedHardcopySourceInput::Results {
                source_key,
                project_id,
                run,
                presentation,
                scope,
            } => {
                let simulation_results = capture_prepared_result_history(&[run], true);
                Self::Results {
                    source_key,
                    project_id,
                    simulation_results: CanonicalHardcopyOwner::capture(
                        "prepared result history",
                        &simulation_results,
                    )?,
                    presentation,
                    scope,
                }
            }
            RetainedHardcopySourceInput::Studio {
                source_key,
                project_id,
                studio,
                runs,
                pane_id,
                all_panes,
                scope,
            } => Self::Studio {
                source_key,
                project_id,
                studio: CanonicalHardcopyOwner::capture("prepared visualization studio", &studio)?,
                simulation_results: CanonicalHardcopyOwner::capture(
                    "prepared studio result history",
                    &capture_prepared_result_history(&runs, false),
                )?,
                pane_id,
                all_panes,
                scope,
            },
            RetainedHardcopySourceInput::VisualizationDocument {
                source_key,
                project_id,
                document,
                page_id,
                pane_id,
                all_panes,
                scope,
            } => Self::VisualizationDocument {
                source_key,
                project_id,
                document: CanonicalHardcopyOwner::capture(
                    "prepared visualization document",
                    &document,
                )?,
                page_id,
                pane_id,
                all_panes,
                scope,
            },
            RetainedHardcopySourceInput::Report {
                project_id,
                source_key,
                document,
                reference_inventory,
                scope,
            } => Self::Report {
                project_id,
                source_key,
                document: CanonicalHardcopyOwner::capture("prepared report document", &document)?,
                reference_inventory,
                scope,
            },
            RetainedHardcopySourceInput::SourceSet {
                source_set,
                members,
            } => Self::SourceSet {
                source_set,
                members: members
                    .into_iter()
                    .map(|member| Self::capture(member.payload))
                    .collect::<Result<Vec<_>, _>>()?,
            },
        })
    }

    fn validate_shape(&self, nested: bool) -> Result<(), HardcopySourceError> {
        validate_prepared_shape!(self, nested)
    }

    fn restore(self) -> Result<RetainedHardcopySourceInput, HardcopySourceError> {
        self.validate_shape(false)?;
        let restored = match self {
            Self::Schematic {
                project_id,
                identity,
                schematic,
                library_manager,
                schematic_buffers,
                sheet_catalog,
                sheet_id,
                project_default_drawing_sheet,
                project_title_block_field_values,
                all_sheets,
                scope,
            } => {
                let (schematic, selection) = schematic
                    .restore::<PreparedSchematicOwner>("prepared schematic")?
                    .restore();
                let library_manager = library_manager
                    .restore::<rspice_project_contract::ProjectLibraries>(
                        "prepared symbol library",
                    )?;
                let schematic_buffers = schematic_buffers
                    .restore::<std::collections::BTreeMap<String, PreparedSchematicInterfaceOwner>>(
                        "prepared schematic symbol buffers",
                    )?
                    .into_iter()
                    .map(|(key, schematic)| (key, schematic.restore()))
                    .collect::<std::collections::HashMap<_, _>>();
                let sheet_catalog = sheet_catalog
                    .map(|catalog| catalog.restore::<SheetCatalog>("prepared sheet catalog"))
                    .transpose()?;
                validate_prepared_schematic_identity(
                    project_id,
                    &identity,
                    sheet_catalog.as_ref(),
                    sheet_id,
                )?;
                RetainedHardcopySourceInput::Schematic {
                    project_id,
                    identity,
                    schematic,
                    selection,
                    library_manager,
                    schematic_buffers,
                    sheet_catalog,
                    sheet_id,
                    project_default_drawing_sheet,
                    project_title_block_field_values,
                    all_sheets,
                    scope,
                }
            }
            Self::Symbol {
                project_id,
                identity,
                document,
                scope,
            } => {
                validate_prepared_base_design_identity(project_id, &identity)?;
                RetainedHardcopySourceInput::Symbol {
                    project_id,
                    identity,
                    document: document.restore::<SymbolDocument>("prepared symbol document")?,
                    scope,
                }
            }
            Self::Results {
                source_key,
                project_id,
                simulation_results,
                presentation,
                scope,
            } => {
                let simulation_results = simulation_results
                    .restore::<ProjectSimulationResults>("prepared result history")?;
                let runs = restore_hardcopy_runs(simulation_results)
                    .map_err(HardcopySourceError::InvalidPreparedWorkerSnapshot)?;
                validate_prepared_result_history(&source_key, project_id, &runs, &presentation)?;
                let run = runs.into_iter().next().ok_or_else(|| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared result history lost its run".to_owned(),
                    )
                })?;
                RetainedHardcopySourceInput::Results {
                    source_key,
                    project_id,
                    run,
                    presentation,
                    scope,
                }
            }
            Self::Studio {
                source_key,
                project_id,
                studio,
                simulation_results,
                pane_id,
                all_panes,
                scope,
            } => {
                let studio = studio
                    .restore::<VisualizationStudioPresentation>("prepared visualization studio")?;
                let runs = restore_hardcopy_runs(
                    simulation_results
                        .restore::<ProjectSimulationResults>("prepared studio result history")?,
                )
                .map_err(HardcopySourceError::InvalidPreparedWorkerSnapshot)?;
                validate_prepared_studio_snapshot(
                    project_id,
                    &source_key,
                    &studio,
                    &runs,
                    pane_id,
                    all_panes,
                )?;
                RetainedHardcopySourceInput::Studio {
                    source_key,
                    project_id,
                    studio,
                    runs,
                    pane_id,
                    all_panes,
                    scope,
                }
            }
            Self::VisualizationDocument {
                source_key,
                project_id,
                document,
                page_id,
                pane_id,
                all_panes,
                scope,
            } => {
                let document =
                    document.restore::<VisualizationDocument>("prepared visualization document")?;
                validate_prepared_visualization_identity(
                    &source_key,
                    project_id,
                    &document,
                    page_id,
                    pane_id,
                )?;
                RetainedHardcopySourceInput::VisualizationDocument {
                    source_key,
                    project_id,
                    document,
                    page_id,
                    pane_id,
                    all_panes,
                    scope,
                }
            }
            Self::Report {
                project_id,
                source_key,
                document,
                reference_inventory,
                scope,
            } => {
                let document = document.restore::<ReportDocument>("prepared report document")?;
                validate_prepared_report_identity(&source_key, project_id, &document)?;
                RetainedHardcopySourceInput::Report {
                    project_id,
                    source_key,
                    document,
                    reference_inventory,
                    scope,
                }
            }
            Self::SourceSet {
                source_set,
                members,
            } => {
                let members = members
                    .into_iter()
                    .map(|member| {
                        member
                            .restore()
                            .map(|payload| PreparedRetainedHardcopyResolution { payload })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                validate_prepared_source_set_members(&source_set, &members)?;
                RetainedHardcopySourceInput::SourceSet {
                    source_set,
                    members,
                }
            }
        };
        Ok(restored)
    }
}

fn validate_prepared_result_history(
    source_key: &str,
    project_id: ProjectId,
    runs: &[HardcopyRun],
    presentation: &ResultsQuickViewPresentation,
) -> Result<(), HardcopySourceError> {
    presentation.validate()?;
    let analysis_count = runs.first().map_or(0, |run| run.analyses.len());
    let has_exact_shape = runs.len() == 1
        && match presentation.viewer() {
            ResultViewer::Manifest | ResultViewer::Specs => true,
            viewer if rspice_results::result_presentation::viewer_uses_wave_stack(viewer) => {
                (1..=MAX_HARDCOPY_SOURCE_SET_MEMBERS).contains(&analysis_count)
            }
            _ => analysis_count == 1,
        };
    if !has_exact_shape {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "prepared result history has an analysis count incompatible with its Results viewer"
                .to_owned(),
        ));
    }
    let run = runs.first().ok_or_else(|| {
        HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "prepared result history lost its run".to_owned(),
        )
    })?;
    let expected_key = format!(
        "project:{}:result-dataset:{}",
        project_id.as_uuid(),
        run.dataset_id
    );
    if source_key != expected_key {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "result dataset identity does not match its source key".to_owned(),
        ));
    }
    Ok(())
}

fn validate_prepared_visualization_identity(
    source_key: &str,
    project_id: ProjectId,
    document: &VisualizationDocument,
    page_id: PageId,
    pane_id: PaneId,
) -> Result<(), HardcopySourceError> {
    let expected_key = visualization_document_pane_source_key(project_id, document.id(), pane_id);
    if source_key != expected_key
        || !document.pages().iter().any(|page| page.id == page_id)
        || !document
            .panes()
            .iter()
            .any(|pane| pane.id == pane_id && pane.page_id == page_id)
    {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "result-document source identity is not retained by its exact document".to_owned(),
        ));
    }
    Ok(())
}

fn validate_prepared_report_identity(
    source_key: &str,
    project_id: ProjectId,
    document: &ReportDocument,
) -> Result<(), HardcopySourceError> {
    let expected_key = format!("project:{}:report:{}", project_id.as_uuid(), document.id());
    if source_key != expected_key {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "report document identity does not match its source key".to_owned(),
        ));
    }
    Ok(())
}

fn capture_prepared_result_history(
    runs: &[HardcopyRun],
    select_first: bool,
) -> ProjectSimulationResults {
    use rspice_formats::project_results::{
        ProjectSimulationResultsData, ProjectSimulationRun, ProjectWaveformData,
    };

    let active_run = select_first.then(|| runs.first()).flatten();
    ProjectSimulationResultsData {
        runs: runs
            .iter()
            .map(|run| {
                ProjectSimulationRun::from_run(run, |waveform: &HardcopyWaveform| {
                    ProjectWaveformData::from_waveform(
                        &waveform.data,
                        waveform.color.clone(),
                        waveform.visible,
                    )
                })
            })
            .collect(),
        next_run_id: runs.iter().map(|run| run.id).max().unwrap_or(0),
        active_run_stable_id: active_run.map(|run| run.run_id),
        active_dataset_id: active_run.map(|run| run.dataset_id),
        active_analysis_sequence: active_run
            .and_then(|run| run.analyses.first())
            .map(|analysis| analysis.id),
        ..ProjectSimulationResultsData::default()
    }
    .into()
}

fn require_project_source_prefix<'a>(
    project_id: ProjectId,
    source_key: &'a str,
    family: &'static str,
) -> Result<&'a str, HardcopySourceError> {
    let prefix = format!("project:{}:{family}:", project_id.as_uuid());
    source_key
        .strip_prefix(&prefix)
        .filter(|tail| !tail.is_empty())
        .ok_or_else(|| {
            HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "{family} source key does not belong to its captured project"
            ))
        })
}

fn validate_project_source_identity(
    project_id: ProjectId,
    identity: &HardcopySourceIdentity,
    family: &'static str,
) -> Result<(), HardcopySourceError> {
    identity.validate()?;
    validate_label(
        "prepared source key",
        &identity.source_key,
        SOURCE_KEY_LIMIT,
    )?;
    validate_label(
        "prepared source display name",
        &identity.display_name,
        DISPLAY_NAME_LIMIT,
    )?;
    require_project_source_prefix(project_id, &identity.source_key, family)?;
    Ok(())
}

fn prepared_base_design_document_id(
    project_id: ProjectId,
    view_key: &str,
) -> Result<HardcopyDocumentId, HardcopySourceError> {
    let mut identity_material = b"rspice-cell-view-hardcopy-v1:".to_vec();
    identity_material.extend_from_slice(view_key.as_bytes());
    HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_material))
        .map_err(|error| HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string()))
}

fn validate_prepared_base_design_identity(
    project_id: ProjectId,
    identity: &HardcopySourceIdentity,
) -> Result<(), HardcopySourceError> {
    let view_key = require_project_source_prefix(project_id, &identity.source_key, "cell-view")?;
    if view_key.contains(":sheet:") {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "base design source unexpectedly names a sheet".to_owned(),
        ));
    }
    let expected = prepared_base_design_document_id(project_id, view_key)?;
    if identity.document_id != expected {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "design document identity is not derived from its project and cell view".to_owned(),
        ));
    }
    Ok(())
}

fn validate_prepared_schematic_identity(
    project_id: ProjectId,
    identity: &HardcopySourceIdentity,
    sheet_catalog: Option<&SheetCatalog>,
    sheet_id: Option<SheetId>,
) -> Result<(), HardcopySourceError> {
    let qualified = require_project_source_prefix(project_id, &identity.source_key, "cell-view")?;
    match sheet_id {
        None => validate_prepared_base_design_identity(project_id, identity),
        Some(sheet_id) => {
            let suffix = format!(":sheet:{sheet_id}");
            let view_key = qualified.strip_suffix(&suffix).ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "schematic sheet identity does not match its source key".to_owned(),
                )
            })?;
            if view_key.is_empty() {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "schematic sheet source has an empty cell-view key".to_owned(),
                ));
            }
            let catalog = sheet_catalog.ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "schematic sheet source has no governed catalog".to_owned(),
                )
            })?;
            catalog.validate().map_err(|error| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
            })?;
            let sheet = catalog.find(sheet_id).ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                    "schematic sheet {sheet_id} is absent from its governed catalog"
                ))
            })?;
            let base_document_id = prepared_base_design_document_id(project_id, view_key)?;
            let mut identity_material = b"rspice-hardcopy-schematic-sheet-v1:".to_vec();
            identity_material.extend_from_slice(sheet_id.as_uuid().as_bytes());
            let expected_document_id = HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
                &base_document_id.as_uuid(),
                &identity_material,
            ))
            .map_err(|error| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
            })?;
            let expected_revision = ObjectRevision::new(sheet.revision()).map_err(|error| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
            })?;
            if identity.document_id != expected_document_id
                || identity.revision != expected_revision
            {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "schematic sheet document identity or revision is stale".to_owned(),
                ));
            }
            Ok(())
        }
    }
}

fn validate_prepared_studio_snapshot(
    project_id: ProjectId,
    source_key: &str,
    studio: &VisualizationStudioPresentation,
    runs: &[HardcopyRun],
    pane_id: u64,
    all_panes: bool,
) -> Result<(), HardcopySourceError> {
    let expected_key = format!(
        "project:{}:visualization-pane:{pane_id}",
        project_id.as_uuid()
    );
    if source_key != expected_key {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "studio pane identity does not match its captured project".to_owned(),
        ));
    }
    if studio.panes.is_empty() || studio.panes.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "prepared studio pane count is outside the governed boundary".to_owned(),
        ));
    }
    if !all_panes
        && (studio.panes.len() != 1
            || studio.panes[0].id != pane_id
            || studio.active_pane != Some(pane_id))
    {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "single-pane studio snapshot contains unrelated pane state".to_owned(),
        ));
    }
    if all_panes && !studio.panes.iter().any(|pane| pane.id == pane_id) {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "aggregate studio snapshot lost its selected pane".to_owned(),
        ));
    }
    let mut pane_ids = std::collections::HashSet::new();
    let expected_analyses = studio
        .panes
        .iter()
        .map(|pane| {
            if !pane_ids.insert(pane.id) {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                    "duplicate prepared studio pane {}",
                    pane.id
                )));
            }
            Ok((pane.dataset_id, pane.analysis_sequence))
        })
        .collect::<Result<std::collections::HashSet<_>, _>>()?;
    let actual_analyses = runs
        .iter()
        .flat_map(|run| {
            run.analyses
                .iter()
                .map(move |analysis| (run.dataset_id, analysis.id))
        })
        .collect::<std::collections::HashSet<_>>();
    if expected_analyses != actual_analyses {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "studio result history is not the exact pane-owned analysis set".to_owned(),
        ));
    }
    Ok(())
}

fn prepared_payload_identity(
    prepared: &PreparedRetainedHardcopyResolution,
) -> Result<(HardcopySourceIdentity, HardcopyScope), HardcopySourceError> {
    match &prepared.payload {
        RetainedHardcopySourceInput::Schematic {
            identity, scope, ..
        }
        | RetainedHardcopySourceInput::Symbol {
            identity, scope, ..
        } => Ok((identity.clone(), scope.clone())),
        RetainedHardcopySourceInput::Results {
            source_key,
            project_id,
            run,
            presentation,
            scope,
        } => {
            if presentation.viewer() == ResultViewer::Manifest {
                return Ok((
                    results_manifest_identity(source_key, *project_id, run)?,
                    scope.clone(),
                ));
            }
            if presentation.viewer() == ResultViewer::Specs {
                return Ok((
                    results_specs_identity(source_key, *project_id, run, presentation.specs())?,
                    scope.clone(),
                ));
            }
            if run.analyses.len() > 1 {
                return Ok((
                    results_stack_identity(source_key, *project_id, run, presentation.viewer())?,
                    scope.clone(),
                ));
            }
            let analysis = run.analyses.first().ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "prepared result lost its analysis".to_owned(),
                )
            })?;
            Ok((
                results_quick_view_identity(
                    source_key,
                    *project_id,
                    presentation.viewer(),
                    run,
                    analysis,
                )?,
                scope.clone(),
            ))
        }
        RetainedHardcopySourceInput::Studio {
            source_key,
            project_id,
            studio,
            pane_id,
            scope,
            ..
        } => {
            let pane = studio
                .panes
                .iter()
                .find(|pane| pane.id == *pane_id)
                .ok_or_else(|| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared studio source lost its selected pane".to_owned(),
                    )
                })?;
            Ok((
                studio_source_identity(source_key, *project_id, studio.revision, pane)?,
                scope.clone(),
            ))
        }
        RetainedHardcopySourceInput::VisualizationDocument {
            source_key,
            document,
            scope,
            ..
        } => Ok((
            HardcopySourceIdentity::try_new(
                source_key,
                HardcopyDocumentId::try_from_uuid(document.id().as_uuid()).map_err(|error| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
                })?,
                document.revision(),
                document.title(),
            )?,
            scope.clone(),
        )),
        RetainedHardcopySourceInput::Report {
            source_key,
            document,
            scope,
            ..
        } => Ok((
            HardcopySourceIdentity::try_new(
                source_key,
                HardcopyDocumentId::try_from_uuid(document.id().as_uuid()).map_err(|error| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
                })?,
                document.revision(),
                document.title(),
            )?,
            scope.clone(),
        )),
        RetainedHardcopySourceInput::SourceSet { .. } => {
            Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "prepared source sets cannot nest".to_owned(),
            ))
        }
    }
}

fn validate_prepared_source_set_members(
    source_set: &HardcopySourceSet,
    members: &[PreparedRetainedHardcopyResolution],
) -> Result<(), HardcopySourceError> {
    source_set.validate()?;
    if source_set.members().len() != members.len() {
        return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
            "prepared source-set member count changed during transfer".to_owned(),
        ));
    }
    for (expected, prepared) in source_set.members().iter().zip(members) {
        let (identity, scope) = prepared_payload_identity(prepared)?;
        if identity.source_key != expected.source_key()
            || identity.display_name != expected.display_name()
            || identity.document_id != expected.document_id()
            || identity.revision != expected.revision()
            || &scope != expected.scope()
        {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "prepared source-set member `{}` is stale or belongs to another owner",
                expected.source_key()
            )));
        }
    }
    Ok(())
}

impl PreparedRetainedHardcopyResolution {
    /// Check source ownership and metadata without hashing samples or resolving
    /// geometry. Full content authentication remains at decode/resolution; this
    /// capture grants no permission to publish.
    pub fn try_capture(payload: RetainedHardcopySourceInput) -> Result<Self, HardcopySourceError> {
        payload.validate_shape(false)?;
        payload.validate_owner_bindings()?;
        Ok(Self { payload })
    }

    fn validate_shape(&self, nested: bool) -> Result<(), HardcopySourceError> {
        self.payload.validate_shape(nested)
    }

    /// Serialize the exact prepared owner snapshot for a browser dedicated
    /// worker. Consuming `self` avoids cloning large retained result arrays.
    /// The returned bytes are bounded and authenticated as one atomic unit.
    pub fn into_worker_snapshot_json(self) -> Result<Vec<u8>, HardcopySourceError> {
        let snapshot = PreparedRetainedHardcopyWorkerSnapshot::capture(self)?;
        let bytes = serde_json::to_vec(&snapshot)
            .map_err(|error| HardcopySourceError::Serialization(error.to_string()))?;
        #[cfg(test)]
        {
            let decoded: PreparedRetainedHardcopyWorkerSnapshot = serde_json::from_slice(&bytes)
                .map_err(|error| {
                    HardcopySourceError::Serialization(format!(
                        "fresh worker snapshot cannot decode itself: {error}"
                    ))
                })?;
            let actual = decoded.compute_transport_digest()?;
            if actual != snapshot.transport_digest {
                let before = serde_json::to_value(&snapshot.payload)
                    .map_err(|error| HardcopySourceError::Serialization(error.to_string()))?;
                let after = serde_json::to_value(&decoded.payload)
                    .map_err(|error| HardcopySourceError::Serialization(error.to_string()))?;
                return Err(HardcopySourceError::Serialization(format!(
                    "fresh worker snapshot changed during JSON round-trip at {}",
                    first_json_difference(&before, &after, "$"),
                )));
            }
        }
        if bytes.len() > MAX_WORKER_SNAPSHOT_BYTES {
            return Err(HardcopySourceError::PreparedWorkerSnapshotTooLarge(
                bytes.len(),
            ));
        }
        Ok(bytes)
    }

    /// Deserialize a dedicated-worker request only after its byte boundary,
    /// closed schema, transport digest, owner schemas, and source identities
    /// all validate. No partially restored source can escape on failure.
    pub fn from_worker_snapshot_json(bytes: &[u8]) -> Result<Self, HardcopySourceError> {
        if bytes.len() > MAX_WORKER_SNAPSHOT_BYTES {
            return Err(HardcopySourceError::PreparedWorkerSnapshotTooLarge(
                bytes.len(),
            ));
        }
        if bytes.is_empty() {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "worker request is empty".to_owned(),
            ));
        }
        let snapshot: PreparedRetainedHardcopyWorkerSnapshot = serde_json::from_slice(bytes)
            .map_err(|error| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
            })?;
        snapshot.into_prepared()
    }

    pub fn resolve_owned(self) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
        match self.payload {
            RetainedHardcopySourceInput::Schematic {
                project_id: _,
                identity,
                schematic,
                selection,
                library_manager,
                schematic_buffers,
                sheet_catalog,
                sheet_id,
                project_default_drawing_sheet,
                project_title_block_field_values,
                all_sheets,
                scope,
            } => {
                let resolver = rspice_design::symbol_resolver::SymbolResolver::new(
                    library_manager.catalog(),
                    &schematic_buffers,
                );
                if all_sheets {
                    let catalog = sheet_catalog.as_ref().ok_or_else(|| {
                        HardcopySourceError::InvalidSheetPartition(
                            "prepared all-sheets source lost its catalog".to_owned(),
                        )
                    })?;
                    return resolve_all_schematic_sheets(SchematicSheetSetHardcopySource {
                        identity,
                        schematic: &schematic,
                        expected_topology_version: schematic.topology_version(),
                        symbol_resolver: Some(&resolver),
                        sheet_catalog: catalog,
                        project_default_drawing_sheet: &project_default_drawing_sheet,
                        project_title_block_field_values: &project_title_block_field_values,
                    });
                }
                resolve_schematic_source(SchematicHardcopySource {
                    identity,
                    schematic: &schematic,
                    selection: SchematicHardcopySelection::capture(&selection, &scope),
                    expected_topology_version: schematic.topology_version(),
                    symbol_resolver: Some(&resolver),
                    sheet_catalog: sheet_catalog.as_ref(),
                    sheet_id,
                    project_default_drawing_sheet: Some(&project_default_drawing_sheet),
                    project_title_block_field_values: Some(&project_title_block_field_values),
                    scope,
                })
            }
            RetainedHardcopySourceInput::Symbol {
                project_id: _,
                identity,
                document,
                scope,
            } => resolve_symbol_document(identity, document, scope),
            RetainedHardcopySourceInput::Results {
                source_key,
                project_id,
                run,
                presentation,
                scope,
            } => {
                if presentation.viewer() == ResultViewer::Manifest {
                    return resolve_results_manifest_source(
                        source_key,
                        project_id,
                        scope,
                        run.as_ref(),
                    );
                }
                if presentation.viewer() == ResultViewer::Specs {
                    return resolve_results_specs_source(
                        source_key,
                        project_id,
                        scope,
                        run.as_ref(),
                        presentation.specs(),
                    );
                }
                if run.analyses.len() > 1 {
                    return resolve_results_quick_view_stack(
                        source_key,
                        project_id,
                        scope,
                        run.as_ref(),
                        &presentation,
                        |waveform: &HardcopyWaveform| waveform.visible,
                    );
                }
                let analysis = run.analyses.first().ok_or_else(|| {
                    HardcopySourceError::UnretainedResult(
                        "prepared result lost its exact analysis".to_owned(),
                    )
                })?;
                if !run.lifecycle.is_terminal() || !analysis.success {
                    return Err(HardcopySourceError::UnretainedResult(
                        "prepared result is not terminal and successful".to_owned(),
                    ));
                }
                resolve_results_quick_view_parts(
                    source_key,
                    project_id,
                    scope,
                    &RetainedQuickViewSource::try_new(
                        run.as_ref(),
                        analysis.id,
                        |waveform: &HardcopyWaveform| waveform.visible,
                    )?,
                    &presentation,
                )
            }
            RetainedHardcopySourceInput::Studio {
                source_key,
                project_id,
                studio,
                runs,
                pane_id,
                all_panes,
                scope,
            } => {
                let source = StudioHardcopySource {
                    project_id,
                    studio: (&studio).into(),
                    runs: &runs,
                    waveform_style: |waveform: &HardcopyWaveform| StudioWaveformStyle {
                        color: &waveform.color,
                        visible: waveform.visible,
                    },
                };
                if all_panes {
                    resolve_studio_document(&source)?.with_source_key(source_key)
                } else {
                    resolve_studio_pane(&source, source_key, pane_id, scope)
                }
            }
            RetainedHardcopySourceInput::VisualizationDocument {
                source_key,
                project_id,
                document,
                page_id,
                pane_id,
                all_panes,
                scope,
            } => resolve_visualization_document_source(
                source_key, project_id, &document, page_id, pane_id, all_panes, scope,
            ),
            RetainedHardcopySourceInput::Report {
                project_id: _,
                source_key,
                document,
                reference_inventory,
                scope,
            } => resolve_report_source(ReportHardcopySource {
                source_key,
                document: &document,
                reference_inventory: Some(&reference_inventory),
                scope,
            }),
            RetainedHardcopySourceInput::SourceSet {
                source_set,
                members,
            } => {
                let mut members = members.into_iter();
                resolve_hardcopy_source_set_with(&source_set, |_| {
                    members
                        .next()
                        .ok_or_else(|| {
                            HardcopySourceError::InvalidSourceSet(
                                "prepared source set lost an ordered member".to_owned(),
                            )
                        })?
                        .resolve_owned()
                })
            }
        }
    }
}

#[cfg(test)]
fn first_json_difference(
    before: &serde_json::Value,
    after: &serde_json::Value,
    path: &str,
) -> String {
    if before == after {
        return "no structural difference".to_owned();
    }
    match (before, after) {
        (serde_json::Value::Object(before), serde_json::Value::Object(after)) => {
            for key in before.keys().chain(after.keys()) {
                match (before.get(key), after.get(key)) {
                    (Some(before), Some(after)) if before != after => {
                        return first_json_difference(before, after, &format!("{path}.{key}"));
                    }
                    (Some(_), None) => return format!("{path}.{key} (removed)"),
                    (None, Some(_)) => return format!("{path}.{key} (added)"),
                    _ => {}
                }
            }
        }
        (serde_json::Value::Array(before), serde_json::Value::Array(after)) => {
            if before.len() != after.len() {
                return format!("{path}.length ({} -> {})", before.len(), after.len());
            }
            for (index, (before, after)) in before.iter().zip(after).enumerate() {
                if before != after {
                    return first_json_difference(before, after, &format!("{path}[{index}]"));
                }
            }
        }
        _ => {
            return format!("{path} ({before:?} -> {after:?})");
        }
    }
    format!("{path} (different serialized ordering)")
}
