//! Dataset-driven Create Result Document workflow.
//!
//! The modal is a transaction editor, not a navigation shortcut. Every
//! dataset, family, viewer, layout, and title is re-resolved when Create is
//! pressed; only a fully validated [`VisualizationDocument`] enters the
//! project-owned collection. Solver samples and retained run history are
//! never mutated.

use rspice_results::document_creation::{ResultDocumentFamily, ResultDocumentLayout};
use rspice_results_ui::create_document::{
    self, CreateDocumentHost, CreateResultDocumentDialogState, ViewerChoice,
};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::product::{AnalysisInstanceId, DatasetBinding, DatasetId, ResultDocumentId};
use crate::results::viewer_catalog::{
    VIEWER_DOCUMENTS, ViewerArt, ViewerCapabilities, ViewerCompatibility, ViewerDocumentDefinition,
    ViewerReleaseClass, viewer_compatibility, viewer_document,
};
use crate::results::visualization_document::{
    AxisOrientation, ColumnRole, DocumentEdit, EntityRef, LinkKind, MAX_SOURCE_CELLS_PER_DATASET,
    MAX_SOURCE_ROWS, MAX_SOURCE_TEXT_BYTES, NewPagePane, NewPane, PageUpdatePolicy,
    PaneDataBinding, PaneKind, PanePlacement, ResultDocumentTracking, ResultDocumentTrackingMode,
    SourceColumn, SourceDataset, SourceRow, TypedValue, ValueType, VisualizationDocument,
    VisualizationError,
};
use crate::state::workspace::VisualizationDocumentPersistenceError;
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationRunLifecycle};
use crate::workbench::state::Workspace;

use super::{ResultsParts, ResultsReads};

const MAX_DOCUMENT_NAME_BYTES: usize = 120;

#[derive(Debug, thiserror::Error)]
pub enum CreateResultDocumentError {
    #[error("{0}")]
    InvalidDraft(String),
    #[error(transparent)]
    Visualization(#[from] VisualizationError),
    #[error(transparent)]
    Persistence(#[from] VisualizationDocumentPersistenceError),
}

#[derive(Debug, Clone, Copy)]
pub struct ResolvedDraft<'a> {
    name: &'a str,
    run: &'a SimulationRun,
    analysis: &'a AnalysisResult,
    pub viewer: &'static ViewerDocumentDefinition,
    family: ResultDocumentFamily,
    layout: ResultDocumentLayout,
}

/// Open a fresh, dataset-aware transaction draft.
pub fn open(state: &mut ResultsParts<'_>) {
    let dataset_id = state
        .model
        .simulation
        .active_run()
        .or_else(|| state.model.simulation.retained.runs.last())
        .map(|run| run.dataset_id);
    let family = ResultDocumentFamily::WaveformWorksheet;
    let viewer_id = first_compatible_viewer(state.results_reads(), dataset_id, family)
        .unwrap_or("viewer-waveform");
    let name = next_document_name(state.results_reads(), family);
    *state.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name,
        name_touched: false,
        dataset_id,
        family_id: family.id().to_owned(),
        viewer_id: viewer_id.to_owned(),
        layout_id: ResultDocumentLayout::TwoLinkedPanes.id().to_owned(),
        validation_error: None,
    };
}

/// Render the modal transaction editor.
pub(crate) fn show(ctx: &egui::Context, state: &mut ResultsParts<'_>) {
    if !state.create_result_document.open {
        return;
    }
    let mut draft = state.create_result_document.clone();
    let submit = create_document::show(
        ctx,
        &mut draft,
        &CreateDocumentSources(state.results_reads()),
    );
    *state.create_result_document = draft;
    if submit {
        let name = state.create_result_document.name.trim().to_owned();
        match commit(state) {
            Ok(_) => {
                state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Created result document {name}."),
                );
            }
            Err(error) => {
                state.create_result_document.validation_error = Some(error.to_string());
            }
        }
    }
}

pub struct CreateDocumentSources<'a>(pub ResultsReads<'a>);

impl CreateDocumentHost for CreateDocumentSources<'_> {
    fn validation_message(&self, draft: &CreateResultDocumentDialogState) -> Option<String> {
        resolve_draft(self.0, draft)
            .as_ref()
            .err()
            .map(ToString::to_string)
    }

    fn can_create(&self, draft: &CreateResultDocumentDialogState) -> bool {
        resolve_draft(self.0, draft).is_ok()
    }

    fn select_family(
        &self,
        draft: &mut CreateResultDocumentDialogState,
        family: ResultDocumentFamily,
    ) {
        draft.family_id = family.id().to_owned();
        if !draft.name_touched {
            draft.name = next_document_name(self.0, family);
        }
        if !viewer_is_creatable(self.0, draft.dataset_id, family, &draft.viewer_id) {
            draft.viewer_id = first_compatible_viewer(self.0, draft.dataset_id, family)
                .unwrap_or_default()
                .to_owned();
        }
        draft.validation_error = None;
    }

    fn select_dataset(&self, draft: &mut CreateResultDocumentDialogState, dataset_id: DatasetId) {
        draft.dataset_id = Some(dataset_id);
        let family = ResultDocumentFamily::from_id(&draft.family_id)
            .unwrap_or(ResultDocumentFamily::WaveformWorksheet);
        if !viewer_is_creatable(self.0, draft.dataset_id, family, &draft.viewer_id) {
            draft.viewer_id = first_compatible_viewer(self.0, draft.dataset_id, family)
                .unwrap_or_default()
                .to_owned();
        }
        draft.validation_error = None;
    }

    fn dataset_label(&self, dataset_id: Option<DatasetId>) -> String {
        dataset_id
            .and_then(|id| retained_run(self.0, id))
            .map_or_else(|| "Select retained dataset".to_owned(), dataset_label)
    }

    fn datasets(&self) -> impl Iterator<Item = (DatasetId, String)> {
        self.0
            .model
            .simulation
            .retained
            .runs
            .iter()
            .map(|run| (run.dataset_id, dataset_label(run)))
    }

    fn viewer_catalog(&self, dataset_id: Option<DatasetId>) -> impl Iterator<Item = ViewerChoice> {
        let capabilities = dataset_id
            .and_then(|id| retained_run(self.0, id))
            .map(run_analysis_ids)
            .unwrap_or_default();
        VIEWER_DOCUMENTS.iter().map(move |viewer| {
            let compatibility = viewer_compatibility(
                viewer.id,
                ViewerCapabilities {
                    analysis_ids: &capabilities,
                    external_capabilities: &[],
                },
            );
            let renderer_available = dataset_id
                .and_then(|id| retained_run(self.0, id))
                .is_some_and(|run| {
                    run.analyses.iter().any(|analysis| {
                        super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
                    })
                });
            ViewerChoice {
                viewer,
                compatibility,
                renderer_available,
            }
        })
    }
}

fn retained_run(state: ResultsReads<'_>, dataset_id: DatasetId) -> Option<&SimulationRun> {
    state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.dataset_id == dataset_id)
}

fn dataset_label(run: &SimulationRun) -> String {
    let lifecycle = match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy retained",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "running",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed => "completed",
        SimulationRunLifecycle::Failed => "failed · partial",
        SimulationRunLifecycle::Aborted => "aborted · partial",
        SimulationRunLifecycle::Interrupted => "interrupted · partial",
    };
    format!(
        "Run {} · {} · {} analyses · {lifecycle}",
        run.id,
        run.label,
        run.analyses.len()
    )
}

fn run_analysis_ids(run: &SimulationRun) -> Vec<&'static str> {
    let mut ids = run
        .analyses
        .iter()
        .flat_map(|analysis| {
            let analysis_type = analysis.analysis_type;
            std::iter::once(analysis_manifest_id(analysis_type))
                .chain((analysis_type == AnalysisType::Corner).then_some("temp"))
        })
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    ids
}

pub(super) fn analysis_matches_viewer(
    viewer: &ViewerDocumentDefinition,
    analysis: AnalysisType,
) -> bool {
    viewer.analysis_ids.is_empty()
        || viewer.analysis_ids.contains(&analysis_manifest_id(analysis))
        // TEMP is the canonical tabular temperature-sweep authority. RSpice's
        // current typed producer retains it as a Corner family analysis, so it
        // carries both identities instead of silently making the release-target
        // table impossible to create.
        || (analysis == AnalysisType::Corner && viewer.analysis_ids.contains(&"temp"))
}

pub(super) const fn analysis_manifest_id(analysis: AnalysisType) -> &'static str {
    match analysis {
        AnalysisType::DcOp => "op",
        AnalysisType::DcSweep | AnalysisType::Parametric => "dc",
        AnalysisType::Ac => "ac",
        AnalysisType::Disto => "disto",
        AnalysisType::Transient => "tran",
        AnalysisType::Noise => "noise",
        AnalysisType::PoleZero => "pz",
        AnalysisType::Tf => "xf",
        AnalysisType::Sensitivity => "sens",
        AnalysisType::Pac => "pac",
        AnalysisType::Pnoise => "pnoise",
        AnalysisType::Pxf => "pxf",
        AnalysisType::Pstb => "pstb",
        AnalysisType::Stb => "stb",
        AnalysisType::MonteCarlo => "mc",
        AnalysisType::Corner => "corner",
        AnalysisType::Optimization => "opt",
        AnalysisType::Soa => "soa",
        AnalysisType::SParameter => "sp",
        AnalysisType::Envelope => "envelope",
        AnalysisType::Fourier => "fourier",
        AnalysisType::HarmonicBalance => "hb",
        AnalysisType::Pss => "pss",
        AnalysisType::Qpss => "qpss",
        AnalysisType::Hbsp => "hbsp",
        AnalysisType::Hbnoise => "hbnoise",
        AnalysisType::Psp => "psp",
        AnalysisType::Qpac => "qpac",
        AnalysisType::Qpnoise => "qpnoise",
        AnalysisType::Qpxf => "qpxf",
        AnalysisType::TransientNoise => "tnoise",
        AnalysisType::DcMismatch => "dcmatch",
    }
}

pub fn first_compatible_viewer(
    state: ResultsReads<'_>,
    dataset_id: Option<DatasetId>,
    family: ResultDocumentFamily,
) -> Option<&'static str> {
    let run = dataset_id.and_then(|id| retained_run(state, id))?;
    let analysis_ids = run_analysis_ids(run);
    VIEWER_DOCUMENTS
        .iter()
        .find(|viewer| viewer_is_creatable_for_run(run, family, viewer, &analysis_ids))
        .map(|viewer| viewer.id)
}

pub fn viewer_is_creatable(
    state: ResultsReads<'_>,
    dataset_id: Option<DatasetId>,
    family: ResultDocumentFamily,
    viewer_id: &str,
) -> bool {
    let Some(run) = dataset_id.and_then(|id| retained_run(state, id)) else {
        return false;
    };
    let Some(viewer) = viewer_document(viewer_id) else {
        return false;
    };
    let analysis_ids = run_analysis_ids(run);
    viewer_is_creatable_for_run(run, family, viewer, &analysis_ids)
}

fn viewer_is_creatable_for_run(
    run: &SimulationRun,
    family: ResultDocumentFamily,
    viewer: &ViewerDocumentDefinition,
    analysis_ids: &[&str],
) -> bool {
    family.includes(viewer)
        && viewer.release == ViewerReleaseClass::ReleaseTarget
        && viewer_compatibility(
            viewer.id,
            ViewerCapabilities {
                analysis_ids,
                external_capabilities: &[],
            },
        )
        .is_compatible()
        && run.analyses.iter().any(|analysis| {
            super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
        })
}

fn next_document_name(state: ResultsReads<'_>, family: ResultDocumentFamily) -> String {
    for sequence in 1..=u32::MAX {
        let candidate = format!("{} · {sequence:02}", family.label());
        if !state
            .model
            .workspace
            .content
            .visualization_documents
            .iter()
            .any(|document| document.title().eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    family.label().to_owned()
}

pub fn resolve_draft<'a>(
    state: ResultsReads<'a>,
    draft: &'a CreateResultDocumentDialogState,
) -> Result<ResolvedDraft<'a>, CreateResultDocumentError> {
    let name = draft.name.as_str();
    if name.trim().is_empty() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Enter a document name.".to_owned(),
        ));
    }
    if name != name.trim() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Document names cannot begin or end with whitespace.".to_owned(),
        ));
    }
    if name.len() > MAX_DOCUMENT_NAME_BYTES {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "Document names cannot exceed {MAX_DOCUMENT_NAME_BYTES} UTF-8 bytes."
        )));
    }
    if name.chars().any(char::is_control) {
        return Err(CreateResultDocumentError::InvalidDraft(
            "Document names cannot contain control characters.".to_owned(),
        ));
    }
    if state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .any(|document| document.title().eq_ignore_ascii_case(name))
    {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "A result document named '{name}' already exists."
        )));
    }

    let family = ResultDocumentFamily::from_id(&draft.family_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "Select a registered result-document family.".to_owned(),
        )
    })?;
    let layout = ResultDocumentLayout::from_id(&draft.layout_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "Select a registered result-document layout.".to_owned(),
        )
    })?;
    let dataset_id = draft.dataset_id.ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft("Select a retained result dataset.".to_owned())
    })?;
    let run = retained_run(state, dataset_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(
            "The selected result dataset is no longer retained.".to_owned(),
        )
    })?;
    if matches!(
        run.lifecycle,
        SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
    ) {
        return Err(CreateResultDocumentError::InvalidDraft(
            "A result document requires a retained, non-running dataset.".to_owned(),
        ));
    }
    if run.analyses.is_empty() {
        return Err(CreateResultDocumentError::InvalidDraft(
            "The selected dataset contains no retained analyses.".to_owned(),
        ));
    }

    let viewer = viewer_document(&draft.viewer_id).ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft("Select a viewer type.".to_owned())
    })?;
    if !family.includes(viewer) {
        return Err(CreateResultDocumentError::InvalidDraft(format!(
            "{} is not part of the {} document family.",
            viewer.title,
            family.label()
        )));
    }
    let analysis_ids = run_analysis_ids(run);
    match viewer_compatibility(
        viewer.id,
        ViewerCapabilities {
            analysis_ids: &analysis_ids,
            external_capabilities: &[],
        },
    ) {
        ViewerCompatibility::Compatible => {}
        ViewerCompatibility::MissingAnalysis {
            accepted_analysis_ids,
        } => {
            return Err(CreateResultDocumentError::InvalidDraft(format!(
                "{} requires {} analysis data.",
                viewer.title,
                accepted_analysis_ids.join(" / ").to_ascii_uppercase()
            )));
        }
        ViewerCompatibility::MissingExternalCapability { capability_id } => {
            return Err(CreateResultDocumentError::InvalidDraft(format!(
                "{} requires a qualified {capability_id} specialist dataset.",
                viewer.title
            )));
        }
        ViewerCompatibility::UnknownDocument => {
            return Err(CreateResultDocumentError::InvalidDraft(
                "The selected viewer is not registered.".to_owned(),
            ));
        }
    }
    let analysis = run.analyses.iter().find(|analysis| {
        analysis_matches_viewer(viewer, analysis.analysis_type)
            && super::persistent_document::renderer_supports_analysis(viewer.id, analysis)
    })
    .ok_or_else(|| {
        CreateResultDocumentError::InvalidDraft(format!(
            "{} cannot render any retained analysis in the selected dataset without substituting a different result contract.",
            viewer.title
        ))
    })?;

    Ok(ResolvedDraft {
        name,
        run,
        analysis,
        viewer,
        family,
        layout,
    })
}

pub fn source_analysis_identity(
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> AnalysisInstanceId {
    analysis.provenance().map_or_else(
        || {
            let name = format!("legacy-analysis-v1/{}", analysis.id);
            AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
        },
        |provenance| provenance.source_instance_id(),
    )
}

pub(super) fn source_dataset(
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Result<SourceDataset, VisualizationError> {
    let binding = DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
    let analysis_id = source_analysis_identity(run, analysis).to_string();

    // Each trace keeps its own exact coordinate grid. Row predicates on the
    // canonical Trace entity select one waveform/component from this lossless
    // long-form source without assuming that every signal shares a sample
    // count or X grid. Analysis-native payloads use tagged JSON chunk rows in
    // the same table. Their component is never `display`, so generic trace
    // provisioning cannot mistake document evidence for plotted samples.
    let columns = vec![
        SourceColumn::new(
            "trace-index",
            "Trace index",
            ValueType::Integer,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "trace-name",
            "Trace",
            ValueType::Text,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "component",
            "Component",
            ValueType::Text,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "sample",
            "Sample",
            ValueType::Integer,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new("x", "X", ValueType::Real, ColumnRole::Coordinate, None)?,
        SourceColumn::new("y", "Y", ValueType::Real, ColumnRole::Signal, None)?,
        SourceColumn::new(
            "analysis-id",
            "Analysis identity",
            ValueType::Text,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "record-kind",
            "Record kind",
            ValueType::Text,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "evidence-chunk",
            "Data chunk",
            ValueType::Integer,
            ColumnRole::Coordinate,
            None,
        )?,
        SourceColumn::new(
            "evidence-json",
            "Analysis data (JSON)",
            ValueType::Text,
            ColumnRole::Signal,
            None,
        )?,
    ];
    let artifacts = analysis_evidence_artifacts(analysis)?;
    let evidence_rows = artifacts.iter().try_fold(0_usize, |total, (_, json)| {
        total.checked_add(json_chunk_count(json))
    });
    let projected_rows = analysis
        .waveforms
        .iter()
        .try_fold(0_usize, |total, waveform| {
            if waveform.x.len() != waveform.y.len()
                || waveform.complex.as_ref().is_some_and(|components| {
                    components.real.len() != waveform.x.len()
                        || components.imag.len() != waveform.x.len()
                })
            {
                return None;
            }
            total.checked_add(waveform.x.len())?.checked_add(
                waveform
                    .complex
                    .as_ref()
                    .map_or(0, |_| waveform.x.len().saturating_mul(2)),
            )
        })
        .and_then(|rows| rows.checked_add(evidence_rows?));
    let Some(projected_rows) = projected_rows else {
        return Err(VisualizationError::InvalidValue {
            field: "source-dataset.samples",
            message:
                "a retained trace has mismatched coordinate, display, or complex sample counts"
                    .to_owned(),
        });
    };
    let maximum_rows = MAX_SOURCE_ROWS.min(MAX_SOURCE_CELLS_PER_DATASET / columns.len());
    if projected_rows > maximum_rows {
        return Err(VisualizationError::InvalidValue {
            field: "source-dataset.rows",
            message: format!(
                "the selected analysis has {projected_rows} rows; this document format supports {maximum_rows}"
            ),
        });
    }

    let mut rows = Vec::with_capacity(projected_rows);
    for (trace_index, waveform) in analysis.waveforms.iter().enumerate() {
        append_component_rows(
            &mut rows,
            trace_index,
            &waveform.name,
            "display",
            &waveform.x,
            &waveform.y,
            &analysis_id,
        )?;
        if let Some(complex) = &waveform.complex {
            append_component_rows(
                &mut rows,
                trace_index,
                &waveform.name,
                "real",
                &waveform.x,
                &complex.real,
                &analysis_id,
            )?;
            append_component_rows(
                &mut rows,
                trace_index,
                &waveform.name,
                "imag",
                &waveform.x,
                &complex.imag,
                &analysis_id,
            )?;
        }
    }
    for (kind, json) in artifacts {
        append_json_artifact_rows(&mut rows, kind, &json, &analysis_id)?;
    }
    SourceDataset::new(binding, columns, rows)
}

fn analysis_evidence_artifacts(
    analysis: &AnalysisResult,
) -> Result<Vec<(&'static str, String)>, VisualizationError> {
    let mut artifacts = Vec::new();
    let waveform_descriptors = analysis
        .waveforms
        .iter()
        .map(|waveform| {
            serde_json::json!({
                "name": waveform.name,
                "unit": waveform.unit,
                "color": waveform.color,
                "visible_by_default": waveform.visible,
                "sample_count": waveform.x.len(),
                "has_complex_source": waveform.complex.is_some(),
            })
        })
        .collect::<Vec<_>>();
    artifacts.push((
        "analysis-descriptor",
        serialize_source_evidence(
            "analysis descriptor",
            &serde_json::json!({
                "id": analysis.id,
                "analysis_type": analysis_manifest_id(analysis.analysis_type),
                "label": analysis.label,
                "timestamp": analysis.timestamp,
                "success": analysis.success,
                "error_message": analysis.error_message,
                "waveforms": waveform_descriptors,
                "measurement_count": analysis.measurements.len(),
                "saved_output_receipt_count": analysis.saved_output_receipts.len(),
            }),
        )?,
    ));

    if let Some(dc_op) = &analysis.dc_op {
        let values = |source: &'static str, values: &[crate::state::OperatingPointValue]| {
            values
                .iter()
                .map(|value| {
                    serde_json::json!({
                        "source": source,
                        "name": value.name,
                        "value": value.value,
                        "unit": value.unit,
                    })
                })
                .collect::<Vec<_>>()
        };
        artifacts.push((
            "dc-operating-point",
            serialize_source_evidence(
                "DC operating-point results",
                &serde_json::json!({
                    "node_voltages": values("node", &dc_op.node_voltages),
                    "branch_currents": values("branch", &dc_op.branch_currents),
                    "power_dissipation": values("device", &dc_op.power_dissipation),
                }),
            )?,
        ));
    }
    if let Some(report) = &analysis.device_op {
        let entries = report
            .entries
            .iter()
            .map(|entry| {
                let params = entry
                    .params
                    .iter()
                    .map(|(name, value)| serde_json::json!({ "name": name, "value": value }))
                    .collect::<Vec<_>>();
                serde_json::json!({
                    "name": entry.name,
                    "device_kind": entry.device_kind,
                    "region": entry.region,
                    "params": params,
                })
            })
            .collect::<Vec<_>>();
        artifacts.push((
            "device-operating-point",
            serialize_source_evidence(
                "device operating-point results",
                &serde_json::json!({ "entries": entries }),
            )?,
        ));
    }
    if let Some(metadata) = &analysis.family_metadata {
        artifacts.push((
            "analysis-family-metadata",
            serialize_source_evidence("analysis family metadata", metadata)?,
        ));
    }
    if let Some(payload) = &analysis.result_payload {
        artifacts.push((
            "analysis-result-payload",
            serialize_source_evidence("analysis result payload", payload)?,
        ));
    }
    Ok(artifacts)
}

fn serialize_source_evidence<T: serde::Serialize + ?Sized>(
    label: &'static str,
    value: &T,
) -> Result<String, VisualizationError> {
    serde_json::to_string(value).map_err(|error| VisualizationError::InvalidValue {
        field: "source-dataset.evidence-json",
        message: format!("could not serialize {label}: {error}"),
    })
}

const JSON_CHUNK_BYTES: usize = MAX_SOURCE_TEXT_BYTES - 96;

fn json_chunk_count(value: &str) -> usize {
    let mut start = 0;
    let mut count = 0;
    while start < value.len() {
        let mut end = (start + JSON_CHUNK_BYTES).min(value.len());
        while !value.is_char_boundary(end) {
            end -= 1;
        }
        start = end;
        count += 1;
    }
    count.max(1)
}

fn append_json_artifact_rows(
    rows: &mut Vec<SourceRow>,
    kind: &'static str,
    json: &str,
    analysis_id: &str,
) -> Result<(), VisualizationError> {
    let mut start = 0;
    let mut chunk = 0_usize;
    while start < json.len() {
        let mut end = (start + JSON_CHUNK_BYTES).min(json.len());
        while !json.is_char_boundary(end) {
            end -= 1;
        }
        let chunk_index = i64::try_from(chunk).map_err(|_| VisualizationError::InvalidValue {
            field: "source-dataset.evidence-chunk",
            message: "the analysis data needs too many chunks".to_owned(),
        })?;
        rows.push(SourceRow::new(vec![
            TypedValue::Integer(-1),
            TypedValue::Text("analysis-evidence".to_owned()),
            TypedValue::Text(kind.to_owned()),
            TypedValue::Integer(chunk_index),
            TypedValue::Real(0.0),
            TypedValue::Real(0.0),
            TypedValue::Text(analysis_id.to_owned()),
            TypedValue::Text("typed-evidence".to_owned()),
            TypedValue::Integer(chunk_index),
            TypedValue::Text(json[start..end].to_owned()),
        ]));
        start = end;
        chunk += 1;
    }
    Ok(())
}

fn append_component_rows(
    rows: &mut Vec<SourceRow>,
    trace_index: usize,
    trace_name: &str,
    component: &str,
    x: &[f64],
    y: &[f64],
    analysis_id: &str,
) -> Result<(), VisualizationError> {
    let trace_index = i64::try_from(trace_index).map_err(|_| VisualizationError::InvalidValue {
        field: "source-dataset.trace-index",
        message: "trace index exceeds the supported signed integer range".to_owned(),
    })?;
    for (sample, (&x, &y)) in x.iter().zip(y).enumerate() {
        rows.push(SourceRow::new(vec![
            TypedValue::Integer(trace_index),
            TypedValue::Text(trace_name.to_owned()),
            TypedValue::Text(component.to_owned()),
            TypedValue::Integer(i64::try_from(sample).map_err(|_| {
                VisualizationError::InvalidValue {
                    field: "source-dataset.sample",
                    message: "sample index exceeds the supported signed integer range".to_owned(),
                }
            })?),
            TypedValue::Real(x),
            TypedValue::real_sample(y),
            TypedValue::Text(analysis_id.to_owned()),
            TypedValue::Text("waveform-sample".to_owned()),
            TypedValue::Integer(0),
            TypedValue::Text("{}".to_owned()),
        ]));
    }
    Ok(())
}

pub(super) fn pane_kind(art: ViewerArt) -> PaneKind {
    match art {
        ViewerArt::Smith => PaneKind::Smith,
        ViewerArt::Polar => PaneKind::Polar,
        ViewerArt::Histogram => PaneKind::Histogram,
        ViewerArt::Table => PaneKind::Table,
        ViewerArt::Wave
        | ViewerArt::Bode
        | ViewerArt::Spectrum
        | ViewerArt::Phase
        | ViewerArt::Field
        | ViewerArt::Contour
        | ViewerArt::Wireless
        | ViewerArt::Scatter
        | ViewerArt::Eye
        | ViewerArt::Bathtub
        | ViewerArt::Margin
        | ViewerArt::DigitalEvents
        | ViewerArt::Soa
        | ViewerArt::Optimization
        | ViewerArt::PoleZero
        | ViewerArt::Thermal
        | ViewerArt::Mesh => PaneKind::Cartesian,
    }
}

fn build_document(
    resolved: ResolvedDraft<'_>,
    tracking: ResultDocumentTracking,
) -> Result<VisualizationDocument, VisualizationError> {
    let source = source_dataset(resolved.run, resolved.analysis)?;
    let binding = PaneDataBinding {
        analysis_id: source_analysis_identity(resolved.run, resolved.analysis),
        dataset: source.binding(),
    };
    let mut document = VisualizationDocument::new_with_initial_pane(
        resolved.name,
        vec![source],
        NewPagePane {
            title: resolved.viewer.title.to_owned(),
            kind: pane_kind(resolved.viewer.art),
            viewer_id: resolved.viewer.id.to_owned(),
            binding: Some(binding),
        },
    )?;
    let page_id = document.pages()[0].id;
    let first_pane_id = document.panes()[0].id;
    document.transact(
        document.revision(),
        vec![
            DocumentEdit::SetTracking(tracking),
            DocumentEdit::Rename {
                entity: EntityRef::Page(page_id),
                value: resolved.family.label().to_owned(),
            },
            DocumentEdit::SetPageComposition {
                page_id,
                layout: resolved.layout.page_layout(),
                template_id: resolved.layout.template_id().to_owned(),
                update_policy: PageUpdatePolicy::RefreshLinkedFigures,
            },
            DocumentEdit::Rename {
                entity: EntityRef::Pane(first_pane_id),
                value: resolved.viewer.title.to_owned(),
            },
        ],
    )?;

    let mut pane_ids = vec![first_pane_id];
    for pane_index in 1..resolved.layout.pane_count() {
        let anchor = if resolved.layout == ResultDocumentLayout::EngineeringGrid && pane_index == 2
        {
            first_pane_id
        } else {
            *pane_ids.last().expect("the initial pane always exists")
        };
        let placement =
            if resolved.layout == ResultDocumentLayout::EngineeringGrid && pane_index % 2 == 1 {
                PanePlacement::RightOf {
                    anchor_pane_id: anchor,
                }
            } else {
                PanePlacement::Below {
                    anchor_pane_id: anchor,
                }
            };
        let receipt = document.transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(NewPane {
                page_id,
                title: format!("{} {}", resolved.viewer.title, pane_index + 1),
                kind: pane_kind(resolved.viewer.art),
                viewer_id: resolved.viewer.id.to_owned(),
                binding: Some(binding),
                placement,
            })],
        )?;
        let pane_id = receipt
            .created
            .into_iter()
            .find_map(|entity| match entity {
                EntityRef::Pane(id) => Some(id),
                _ => None,
            })
            .expect("AddBoundPane transaction returns its stable pane identity");
        pane_ids.push(pane_id);
    }

    let horizontal_axes = pane_ids
        .iter()
        .filter_map(|pane_id| {
            document
                .axes()
                .iter()
                .find(|axis| {
                    axis.pane_id == *pane_id && axis.orientation == AxisOrientation::Horizontal
                })
                .map(|axis| EntityRef::Axis(axis.id))
        })
        .collect::<Vec<_>>();
    if resolved.layout == ResultDocumentLayout::TwoLinkedPanes {
        document.transact(
            document.revision(),
            vec![DocumentEdit::AddLinkGroup {
                label: "Linked horizontal viewport".to_owned(),
                kind: LinkKind::HorizontalViewport,
                members: horizontal_axes,
            }],
        )?;
    }
    Ok(document)
}

fn document_tracking(
    runs: &[SimulationRun],
    selected_run: &SimulationRun,
    selected_analysis: &AnalysisResult,
) -> ResultDocumentTracking {
    let identities = selected_run
        .prepared_receipt()
        .and_then(|receipt| receipt.simulation_plan_id())
        .zip(
            selected_analysis
                .provenance()
                .map(|provenance| provenance.authored_source_instance_id()),
        );
    let Some((simulation_plan_id, authored_analysis_id)) = identities else {
        return ResultDocumentTracking::pinned();
    };
    let newest_for_plan = runs
        .iter()
        .find(|run| {
            run.prepared_receipt()
                .and_then(|receipt| receipt.simulation_plan_id())
                == Some(simulation_plan_id)
        })
        .is_some_and(|run| run.dataset_id == selected_run.dataset_id);
    ResultDocumentTracking::for_plan(
        if newest_for_plan {
            ResultDocumentTrackingMode::Latest
        } else {
            ResultDocumentTrackingMode::Pinned
        },
        simulation_plan_id,
        authored_analysis_id,
    )
}

/// Revalidate and commit the current draft into the project authority.
pub fn commit(state: &mut ResultsParts<'_>) -> Result<ResultDocumentId, CreateResultDocumentError> {
    let resolved = resolve_draft(state.results_reads(), state.create_result_document)?;
    let tracking = document_tracking(
        &state.model.simulation.retained.runs,
        resolved.run,
        resolved.analysis,
    );
    let document = build_document(resolved, tracking)?;
    let previous_documents_dirty = state.model.workspace.content.visualization_documents_dirty;
    let previous_workspace = state.workbench.workspace;
    let document_id = state
        .model
        .workspace
        .content
        .insert_visualization_document(document)?;

    if state.workbench.current_route().surface_id() != crate::workbench::SurfaceId::Results {
        state.workbench.activate(Workspace::Results);
    }
    if !crate::workbench::chrome::document_bar::results::show_visualization_document(
        state,
        &document_id,
    ) {
        if let Some(index) = state
            .model
            .workspace
            .content
            .visualization_documents
            .iter()
            .position(|document| document.id() == document_id)
        {
            state
                .model
                .workspace
                .content
                .visualization_documents
                .remove(index);
        }
        state.model.workspace.content.visualization_documents_dirty = previous_documents_dirty;
        state
            .model
            .workspace
            .content
            .visualization_documents_modified
            .remove(&document_id);
        state.workbench.activate(previous_workspace);
        return Err(CreateResultDocumentError::InvalidDraft(
            "The created result document could not resolve its retained dataset binding."
                .to_owned(),
        ));
    }
    state.create_result_document.open = false;
    state.create_result_document.validation_error = None;
    Ok(document_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, DcOpResult, OperatingPointValue, RetainedTrace, SimulationRun,
    };

    #[test]
    fn source_projection_preserves_independent_waveform_grids_without_truncation() {
        let analysis =
            AnalysisResult::new(11, AnalysisType::Transient, "analysis").with_waveforms(vec![
                RetainedTrace::new(
                    "V(out)",
                    vec![0.0, 0.5, 1.0],
                    vec![1.0, 2.0, 3.0],
                    "#55aaff",
                ),
                RetainedTrace::new("I(R1)", vec![10.0, 20.0], vec![0.25, -0.5], "#ffaa55"),
            ]);
        let mut run = SimulationRun::new(7);
        run.add_analysis(analysis);
        let source = source_dataset(&run, &run.analyses[0]).expect("lossless source projection");

        assert_eq!(source.rows().len(), 6);
        assert_eq!(source.columns().len(), 10);
        assert_eq!(source.columns()[4].key(), "x");
        assert_eq!(source.columns()[4].role(), ColumnRole::Coordinate);
        assert_eq!(source.columns()[5].key(), "y");
        assert_eq!(source.columns()[5].role(), ColumnRole::Signal);
        assert_eq!(
            source.rows()[4].values(),
            &[
                TypedValue::Integer(1),
                TypedValue::Text("I(R1)".to_owned()),
                TypedValue::Text("display".to_owned()),
                TypedValue::Integer(1),
                TypedValue::Real(20.0),
                TypedValue::Real(-0.5),
                TypedValue::Text(source_analysis_identity(&run, &run.analyses[0]).to_string()),
                TypedValue::Text("waveform-sample".to_owned()),
                TypedValue::Integer(0),
                TypedValue::Text("{}".to_owned()),
            ]
        );
    }

    #[test]
    fn source_projection_retains_unavailable_waveform_samples_as_typed_rows() {
        let mut run = SimulationRun::new(7);
        let mut analysis =
            AnalysisResult::new(11, AnalysisType::Transient, "gaps").with_waveforms(vec![
                RetainedTrace::new(
                    "out",
                    vec![0.0, 1.0, 2.0],
                    vec![1.0, f64::NAN, -0.0],
                    "#55aaff",
                ),
            ]);
        analysis.import_source = Some(rspice_results::result_import::ResultImportSource {
            source_name: "gaps.raw".into(),
            format: rspice_results::result_import::ResultImportFormat::SpiceRaw,
            coordinate: None,
        });
        analysis.validate_retained_evidence().unwrap();
        run.add_analysis(analysis);
        let source = source_dataset(&run, &run.analyses[0]).unwrap();
        assert_eq!(
            source.rows()[1].values()[5],
            TypedValue::Missing(ValueType::Real)
        );
        assert_eq!(source.rows()[2].values()[5], TypedValue::Real(-0.0));
        let document = VisualizationDocument::new("gaps", vec![source]).unwrap();
        let encoded = serde_json::to_string(&document).unwrap();
        let restored: VisualizationDocument = serde_json::from_str(&encoded).unwrap();
        assert_eq!(
            restored.content_digest().unwrap(),
            document.content_digest().unwrap()
        );
    }

    #[test]
    fn source_projection_embeds_typed_operating_point_evidence_without_waveforms() {
        let mut analysis = AnalysisResult::new(12, AnalysisType::DcOp, "OP");
        analysis.dc_op = Some(DcOpResult {
            node_voltages: vec![OperatingPointValue {
                name: "V(out)".to_owned(),
                value: 1.234_567_890_123_456,
                unit: "V".to_owned(),
            }],
            branch_currents: Vec::new(),
            power_dissipation: Vec::new(),
        });
        let mut run = SimulationRun::new(8);
        run.add_analysis(analysis);

        let source = source_dataset(&run, &run.analyses[0]).expect("typed source projection");
        assert!(source.rows().iter().all(|row| {
            !matches!(row.values().get(7), Some(TypedValue::Text(kind)) if kind == "waveform-sample")
        }));
        let json = source
            .rows()
            .iter()
            .find_map(|row| match (row.values().get(2), row.values().get(9)) {
                (Some(TypedValue::Text(kind)), Some(TypedValue::Text(json)))
                    if kind == "dc-operating-point" =>
                {
                    Some(json)
                }
                _ => None,
            })
            .expect("embedded DC operating-point artifact");
        let parsed: serde_json::Value = serde_json::from_str(json).expect("canonical JSON");
        let retained = parsed["node_voltages"][0]["value"]
            .as_f64()
            .expect("retained f64");
        assert_eq!(retained.to_bits(), 1.234_567_890_123_456_f64.to_bits());
    }
}
