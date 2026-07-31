//! Retained and prepared hardcopy sources.
//!
//! A *retained* source is a cheap descriptor — enough for command enablement
//! and the dialog's document and scope selectors, and building one never
//! clones an engineering document, resolves a plot scene, or hashes samples.
//! A *prepared* source is the sealed form a publication executes against.
//!
//! Preparation is the authority boundary. Everything a run will read is
//! identified here by digest and revision, and every validator refuses on a
//! mismatch rather than substituting what is current. A published sheet has
//! to be reproducible from what it says it came from, so a source set that
//! drifted after preparation is an error, not a silent re-resolve.

use super::*;

/// Cheap, semantic-free descriptor used by command enablement and the
/// hardcopy dialog's document/scope selectors. Building this value never
/// clones an engineering document, resolves a plot scene, or hashes samples.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetainedHardcopySourceDescriptor {
    pub source_key: String,
    pub display_name: String,
    pub document_kind: HardcopyDocumentKind,
    pub allowed_scopes: Vec<HardcopyScope>,
    pub availability: RetainedHardcopySourceAvailability,
}

impl RetainedHardcopySourceDescriptor {
    #[must_use]
    pub fn supports_scope(&self, scope: &HardcopyScope) -> bool {
        self.allowed_scopes.contains(scope)
    }
}

/// Owned, `Send`-safe retained-source snapshot prepared on the UI thread
/// without hashing samples, resolving symbols, or constructing semantic
/// geometry. The worker consumes it with [`Self::resolve_owned`].
pub(crate) struct PreparedRetainedHardcopyResolution {
    pub(super) payload: PreparedRetainedHardcopyPayload,
}

pub(super) enum PreparedRetainedHardcopyPayload {
    Schematic {
        project_id: ProjectId,
        identity: HardcopySourceIdentity,
        schematic: SchematicState,
        library_manager: crate::state::LibraryManager,
        schematic_buffers: std::collections::HashMap<String, SchematicState>,
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
        run: SimulationRun,
        presentation: ResultsQuickViewPresentation,
        scope: HardcopyScope,
    },
    Studio {
        source_key: String,
        project_id: ProjectId,
        studio: VisualizationStudioState,
        simulation: SimulationState,
        pane_id: u64,
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
pub(super) struct CanonicalHardcopyOwner(pub(super) serde_json::Value);

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
    fn capture(schematic: SchematicState) -> Self {
        Self {
            components: schematic.components,
            wires: schematic.wires,
            buses: schematic.buses,
            bus_taps: schematic.bus_taps,
            junctions: schematic.junctions,
            net_labels: schematic.net_labels,
            design_notes: schematic.design_notes,
            documentation_shapes: schematic.documentation_shapes,
            selection: schematic.selection,
        }
    }

    fn restore(self) -> SchematicState {
        let mut schematic = SchematicState::default();
        schematic.components = self.components;
        schematic.wires = self.wires;
        schematic.buses = self.buses;
        schematic.bus_taps = self.bus_taps;
        schematic.junctions = self.junctions;
        schematic.net_labels = self.net_labels;
        schematic.design_notes = self.design_notes;
        schematic.documentation_shapes = self.documentation_shapes;
        schematic.selection = self.selection;
        schematic
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
    fn capture(schematic: SchematicState) -> Self {
        Self {
            components: schematic.components,
        }
    }

    fn restore(self) -> SchematicState {
        let mut schematic = SchematicState::default();
        schematic.components = self.components;
        schematic
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedFftNormalization {
    Peak,
    Rms,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedFftWindow {
    Rectangular,
    Hanning,
    Hamming,
    Blackman,
    BlackmanHarris,
    FlatTop,
    Kaiser,
    Gaussian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedFftInputFidelity {
    Reference,
    Interactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PreparedHistogramMode {
    Count,
    Pdf,
    Cdf,
    Percent,
}

/// Only the persisted controls that affect quick-result semantic geometry.
/// FFT caches and every other viewer/runtime field are deliberately absent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PreparedResultsPresentation {
    viewer: ResultViewer,
    fft_selected_source: Option<String>,
    fft_normalization: PreparedFftNormalization,
    fft_window: PreparedFftWindow,
    fft_input_fidelity: PreparedFftInputFidelity,
    fft_time_window_auto: bool,
    fft_time_window_start: f64,
    fft_time_window_end: f64,
    fft_sample_count_auto: bool,
    fft_sample_count: usize,
    histogram_selected: usize,
    histogram_bin_count: usize,
    histogram_custom_range: bool,
    histogram_custom_min: f64,
    histogram_custom_max: f64,
    histogram_mode: PreparedHistogramMode,
}

impl PreparedResultsPresentation {
    fn capture(value: ResultsQuickViewPresentation) -> Result<Self, HardcopySourceError> {
        validate_optional_label(
            "prepared FFT source",
            value.fft.selected_source.as_deref(),
            DISPLAY_NAME_LIMIT,
        )?;
        let captured = Self {
            viewer: value.viewer,
            fft_selected_source: value.fft.selected_source,
            fft_normalization: match value.fft.normalization {
                crate::analysis::fft::data::SpectrumNormalization::Peak => {
                    PreparedFftNormalization::Peak
                }
                crate::analysis::fft::data::SpectrumNormalization::Rms => {
                    PreparedFftNormalization::Rms
                }
            },
            fft_window: match value.fft.window {
                crate::analysis::WindowFunction::Rectangular => PreparedFftWindow::Rectangular,
                crate::analysis::WindowFunction::Hanning => PreparedFftWindow::Hanning,
                crate::analysis::WindowFunction::Hamming => PreparedFftWindow::Hamming,
                crate::analysis::WindowFunction::Blackman => PreparedFftWindow::Blackman,
                crate::analysis::WindowFunction::BlackmanHarris => {
                    PreparedFftWindow::BlackmanHarris
                }
                crate::analysis::WindowFunction::FlatTop => PreparedFftWindow::FlatTop,
                crate::analysis::WindowFunction::Kaiser => PreparedFftWindow::Kaiser,
                crate::analysis::WindowFunction::Gaussian => PreparedFftWindow::Gaussian,
            },
            fft_input_fidelity: match value.fft.input_fidelity {
                crate::analysis::InputFidelity::Reference => PreparedFftInputFidelity::Reference,
                crate::analysis::InputFidelity::Interactive => {
                    PreparedFftInputFidelity::Interactive
                }
            },
            fft_time_window_auto: value.fft.time_window_auto,
            fft_time_window_start: value.fft.time_window_start,
            fft_time_window_end: value.fft.time_window_end,
            fft_sample_count_auto: value.fft.sample_count_auto,
            fft_sample_count: value.fft.sample_count,
            histogram_selected: value.histogram_selected,
            histogram_bin_count: value.histogram_bin_count,
            histogram_custom_range: value.histogram_custom_range,
            histogram_custom_min: value.histogram_custom_min,
            histogram_custom_max: value.histogram_custom_max,
            histogram_mode: match value.histogram_mode {
                crate::analysis::HistogramDisplayMode::Count => PreparedHistogramMode::Count,
                crate::analysis::HistogramDisplayMode::Pdf => PreparedHistogramMode::Pdf,
                crate::analysis::HistogramDisplayMode::Cdf => PreparedHistogramMode::Cdf,
                crate::analysis::HistogramDisplayMode::Percent => PreparedHistogramMode::Percent,
            },
        };
        captured.validate()?;
        Ok(captured)
    }

    fn validate(&self) -> Result<(), HardcopySourceError> {
        validate_optional_label(
            "prepared FFT source",
            self.fft_selected_source.as_deref(),
            DISPLAY_NAME_LIMIT,
        )?;
        for (field, value) in [
            ("FFT time-window start", self.fft_time_window_start),
            ("FFT time-window end", self.fft_time_window_end),
            ("histogram custom minimum", self.histogram_custom_min),
            ("histogram custom maximum", self.histogram_custom_max),
        ] {
            if !value.is_finite() {
                return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                    "{field} is not finite"
                )));
            }
        }
        if self.fft_sample_count == 0 {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "FFT sample count is zero".to_owned(),
            ));
        }
        if self.histogram_bin_count == 0 {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "histogram bin count is zero".to_owned(),
            ));
        }
        if !self.fft_time_window_auto && self.fft_time_window_start >= self.fft_time_window_end {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "manual FFT time window is empty or reversed".to_owned(),
            ));
        }
        if self.histogram_custom_range && self.histogram_custom_min >= self.histogram_custom_max {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "custom histogram range is empty or reversed".to_owned(),
            ));
        }
        Ok(())
    }

    fn restore(self) -> Result<ResultsQuickViewPresentation, HardcopySourceError> {
        self.validate()?;
        let mut fft = crate::analysis::FftState::default();
        fft.selected_source = self.fft_selected_source;
        fft.normalization = match self.fft_normalization {
            PreparedFftNormalization::Peak => {
                crate::analysis::fft::data::SpectrumNormalization::Peak
            }
            PreparedFftNormalization::Rms => crate::analysis::fft::data::SpectrumNormalization::Rms,
        };
        fft.window = match self.fft_window {
            PreparedFftWindow::Rectangular => crate::analysis::WindowFunction::Rectangular,
            PreparedFftWindow::Hanning => crate::analysis::WindowFunction::Hanning,
            PreparedFftWindow::Hamming => crate::analysis::WindowFunction::Hamming,
            PreparedFftWindow::Blackman => crate::analysis::WindowFunction::Blackman,
            PreparedFftWindow::BlackmanHarris => crate::analysis::WindowFunction::BlackmanHarris,
            PreparedFftWindow::FlatTop => crate::analysis::WindowFunction::FlatTop,
            PreparedFftWindow::Kaiser => crate::analysis::WindowFunction::Kaiser,
            PreparedFftWindow::Gaussian => crate::analysis::WindowFunction::Gaussian,
        };
        fft.input_fidelity = match self.fft_input_fidelity {
            PreparedFftInputFidelity::Reference => crate::analysis::InputFidelity::Reference,
            PreparedFftInputFidelity::Interactive => crate::analysis::InputFidelity::Interactive,
        };
        fft.time_window_auto = self.fft_time_window_auto;
        fft.time_window_start = self.fft_time_window_start;
        fft.time_window_end = self.fft_time_window_end;
        fft.sample_count_auto = self.fft_sample_count_auto;
        fft.sample_count = self.fft_sample_count;
        Ok(ResultsQuickViewPresentation {
            viewer: self.viewer,
            fft,
            histogram_selected: self.histogram_selected,
            histogram_bin_count: self.histogram_bin_count,
            histogram_custom_range: self.histogram_custom_range,
            histogram_custom_min: self.histogram_custom_min,
            histogram_custom_max: self.histogram_custom_max,
            histogram_mode: match self.histogram_mode {
                PreparedHistogramMode::Count => crate::analysis::HistogramDisplayMode::Count,
                PreparedHistogramMode::Pdf => crate::analysis::HistogramDisplayMode::Pdf,
                PreparedHistogramMode::Cdf => crate::analysis::HistogramDisplayMode::Cdf,
                PreparedHistogramMode::Percent => crate::analysis::HistogramDisplayMode::Percent,
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source-family", rename_all = "kebab-case", deny_unknown_fields)]
pub(super) enum PreparedRetainedHardcopyWorkerPayload {
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
        presentation: PreparedResultsPresentation,
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
pub(super) struct PreparedRetainedHardcopyWorkerSnapshot {
    pub(super) schema_version: u32,
    pub(super) payload: PreparedRetainedHardcopyWorkerPayload,
    pub(super) transport_digest: ContentDigest,
}

#[derive(Serialize)]
struct PreparedRetainedHardcopyWorkerDigestMaterial<'a> {
    schema_version: u32,
    payload: &'a PreparedRetainedHardcopyWorkerPayload,
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

    pub(super) fn compute_transport_digest(&self) -> Result<ContentDigest, HardcopySourceError> {
        canonical_digest(
            b"rspice-prepared-hardcopy-worker-snapshot-v1",
            &PreparedRetainedHardcopyWorkerDigestMaterial {
                schema_version: self.schema_version,
                payload: &self.payload,
            },
        )
    }

    fn validate(&self) -> Result<(), HardcopySourceError> {
        self.validate_shape()?;
        let actual = self.compute_transport_digest()?;
        if actual != self.transport_digest {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                "transport digest does not authenticate the prepared owner snapshot".to_owned(),
            ));
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
    fn capture(payload: PreparedRetainedHardcopyPayload) -> Result<Self, HardcopySourceError> {
        Ok(match payload {
            PreparedRetainedHardcopyPayload::Schematic {
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
                let schematic = PreparedSchematicOwner::capture(schematic);
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
            PreparedRetainedHardcopyPayload::Symbol {
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
            PreparedRetainedHardcopyPayload::Results {
                source_key,
                project_id,
                run,
                presentation,
                scope,
            } => {
                let mut simulation = SimulationState::default();
                simulation.next_run_id = run.id;
                simulation.active_run_idx = Some(0);
                simulation.active_analysis_idx = Some(0);
                simulation.runs = vec![run];
                Self::Results {
                    source_key,
                    project_id,
                    simulation_results: CanonicalHardcopyOwner::capture(
                        "prepared result history",
                        &ProjectSimulationResults::from_state(&simulation),
                    )?,
                    presentation: PreparedResultsPresentation::capture(presentation)?,
                    scope,
                }
            }
            PreparedRetainedHardcopyPayload::Studio {
                source_key,
                project_id,
                studio,
                simulation,
                pane_id,
                all_panes,
                scope,
            } => Self::Studio {
                source_key,
                project_id,
                studio: CanonicalHardcopyOwner::capture("prepared visualization studio", &studio)?,
                simulation_results: CanonicalHardcopyOwner::capture(
                    "prepared studio result history",
                    &ProjectSimulationResults::from_state(&simulation),
                )?,
                pane_id,
                all_panes,
                scope,
            },
            PreparedRetainedHardcopyPayload::Report {
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
            PreparedRetainedHardcopyPayload::SourceSet {
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
        match self {
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
                crate::state::validate_project_drawing_sheet_title_field_values(
                    project_title_block_field_values,
                )
                .map_err(|error| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
                })?;
                project_default_drawing_sheet.validate().map_err(|error| {
                    HardcopySourceError::InvalidPreparedWorkerSnapshot(error.to_string())
                })?;
                if project_default_drawing_sheet.inheritance
                    != crate::state::DrawingSheetInheritance::ProjectDefault
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
                if nested {
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
    }

    fn restore(self) -> Result<PreparedRetainedHardcopyPayload, HardcopySourceError> {
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
                let schematic = schematic
                    .restore::<PreparedSchematicOwner>("prepared schematic")?
                    .restore();
                let library_manager = library_manager
                    .restore::<crate::state::LibraryManager>("prepared symbol library")?;
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
                PreparedRetainedHardcopyPayload::Schematic {
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
                }
            }
            Self::Symbol {
                project_id,
                identity,
                document,
                scope,
            } => {
                validate_prepared_base_design_identity(project_id, &identity)?;
                PreparedRetainedHardcopyPayload::Symbol {
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
                let simulation = simulation_results
                    .into_simulation_state()
                    .map_err(HardcopySourceError::InvalidPreparedWorkerSnapshot)?;
                if simulation.runs.len() != 1 || simulation.runs[0].analyses.len() != 1 {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "prepared result history must contain exactly one run and analysis"
                            .to_owned(),
                    ));
                }
                let run = simulation.runs.into_iter().next().ok_or_else(|| {
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
                PreparedRetainedHardcopyPayload::Results {
                    source_key,
                    project_id,
                    run,
                    presentation: presentation.restore()?,
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
                let studio =
                    studio.restore::<VisualizationStudioState>("prepared visualization studio")?;
                let simulation = simulation_results
                    .restore::<ProjectSimulationResults>("prepared studio result history")?
                    .into_simulation_state()
                    .map_err(HardcopySourceError::InvalidPreparedWorkerSnapshot)?;
                validate_prepared_studio_snapshot(
                    project_id,
                    &source_key,
                    &studio,
                    &simulation,
                    pane_id,
                    all_panes,
                )?;
                PreparedRetainedHardcopyPayload::Studio {
                    source_key,
                    project_id,
                    studio,
                    simulation,
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
                let expected_key =
                    format!("project:{}:report:{}", project_id.as_uuid(), document.id());
                if source_key != expected_key {
                    return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(
                        "report document identity does not match its source key".to_owned(),
                    ));
                }
                PreparedRetainedHardcopyPayload::Report {
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
                PreparedRetainedHardcopyPayload::SourceSet {
                    source_set,
                    members,
                }
            }
        };
        Ok(restored)
    }
}

fn validate_optional_label(
    field: &'static str,
    value: Option<&str>,
    maximum_bytes: usize,
) -> Result<(), HardcopySourceError> {
    if let Some(value) = value {
        validate_label(field, value, maximum_bytes)?;
    }
    Ok(())
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
    studio: &VisualizationStudioState,
    simulation: &SimulationState,
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
    let actual_analyses = simulation
        .runs
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
        PreparedRetainedHardcopyPayload::Schematic {
            identity, scope, ..
        }
        | PreparedRetainedHardcopyPayload::Symbol {
            identity, scope, ..
        } => Ok((identity.clone(), scope.clone())),
        PreparedRetainedHardcopyPayload::Results {
            source_key,
            project_id,
            run,
            presentation,
            scope,
        } => {
            let analysis = run.analyses.first().ok_or_else(|| {
                HardcopySourceError::InvalidPreparedWorkerSnapshot(
                    "prepared result lost its analysis".to_owned(),
                )
            })?;
            Ok((
                results_quick_view_identity(
                    source_key,
                    *project_id,
                    presentation.viewer,
                    run,
                    analysis,
                )?,
                scope.clone(),
            ))
        }
        PreparedRetainedHardcopyPayload::Studio {
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
                studio_source_identity(source_key, *project_id, studio, pane)?,
                scope.clone(),
            ))
        }
        PreparedRetainedHardcopyPayload::Report {
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
        PreparedRetainedHardcopyPayload::SourceSet { .. } => {
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
    /// Serialize the exact prepared owner snapshot for a browser dedicated
    /// worker. Consuming `self` avoids cloning large retained result arrays.
    /// The returned bytes are bounded and authenticated as one atomic unit.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn to_worker_snapshot_json(self) -> Result<Vec<u8>, HardcopySourceError> {
        let snapshot = PreparedRetainedHardcopyWorkerSnapshot::capture(self)?;
        let bytes = serde_json::to_vec(&snapshot)
            .map_err(|error| HardcopySourceError::Serialization(error.to_string()))?;
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
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn from_worker_snapshot_json(bytes: &[u8]) -> Result<Self, HardcopySourceError> {
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

    pub(crate) fn resolve_owned(self) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
        match self.payload {
            PreparedRetainedHardcopyPayload::Schematic {
                project_id: _,
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
                let resolver = SymbolResolver::new(&library_manager, &schematic_buffers);
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
                    expected_topology_version: schematic.topology_version(),
                    symbol_resolver: Some(&resolver),
                    sheet_catalog: sheet_catalog.as_ref(),
                    sheet_id,
                    project_default_drawing_sheet: Some(&project_default_drawing_sheet),
                    project_title_block_field_values: Some(&project_title_block_field_values),
                    scope,
                })
            }
            PreparedRetainedHardcopyPayload::Symbol {
                project_id: _,
                identity,
                document,
                scope,
            } => resolve_symbol_source(SymbolHardcopySource {
                identity,
                document: &document,
                selection: None,
                scope,
            }),
            PreparedRetainedHardcopyPayload::Results {
                source_key,
                project_id,
                run,
                presentation,
                scope,
            } => {
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
                analysis
                    .validate_retained_evidence()
                    .map_err(HardcopySourceError::InvalidVisualizationSource)?;
                resolve_results_quick_view_parts(
                    source_key,
                    project_id,
                    scope,
                    ActiveQuickResult {
                        run: &run,
                        analysis,
                    },
                    &presentation,
                )
            }
            PreparedRetainedHardcopyPayload::Studio {
                source_key,
                project_id,
                studio,
                simulation,
                pane_id,
                all_panes,
                scope,
            } => {
                if all_panes {
                    let mut resolved = resolve_all_studio_panes(project_id, &studio, &simulation)?;
                    resolved.source_key = source_key;
                    Ok(resolved)
                } else {
                    resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
                        source_key,
                        project_id,
                        studio: &studio,
                        simulation: &simulation,
                        pane_id,
                        scope,
                    })
                }
            }
            PreparedRetainedHardcopyPayload::Report {
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
            PreparedRetainedHardcopyPayload::SourceSet {
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
