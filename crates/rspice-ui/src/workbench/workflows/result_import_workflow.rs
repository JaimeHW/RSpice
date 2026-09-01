//! Validated external result-dataset ingest.
//!
//! Supported interchange adapters are deliberately classified as
//! legacy-unattributed external evidence. They become immutable retained runs,
//! but never claim native-solver provenance or prepared-plan authority.

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, SimulationRunLifecycle,
    SimulationRunProvenance, WaveformData,
};
use crate::ui::tokens::Tokens;
use crate::workbench::app_state::AppState;
use crate::workbench::state::{ResultImportDialogState, ResultImportStage};
use std::collections::{BTreeMap, HashSet};
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read as _;
use std::path::Path;
use std::sync::Arc;

#[path = "result_import_adapters.rs"]
mod adapters;

type ComplexComponentColumns = (Option<Vec<f64>>, Option<Vec<f64>>);

pub(crate) const RESULT_DATASET_FILTER: (&str, &[&str]) = (
    "Result dataset",
    &[
        "rspiceresult",
        "rspicedata",
        "csv",
        "tsv",
        "tab",
        "s1p",
        "s2p",
        "s3p",
        "s4p",
        "snp",
        "ts",
        "h5",
        "hdf5",
        "arrow",
        "feather",
        "parquet",
        "npy",
        "npz",
        "mat",
        "raw",
        "psfascii",
        "vcd",
        "fst",
    ],
);
pub(crate) const MAX_RESULT_DATASET_BYTES: u64 = 64 * 1024 * 1024;
const MAX_RESULT_COLUMNS: usize = 1_024;
const MAX_RESULT_ROWS: usize = 1_000_000;
const MAX_HEADER_BYTES: usize = 256;
const MIN_RESULT_ROWS: usize = 2;
const RESULT_IMPORT_WINDOW_MARGIN: f32 = 24.0;
const RESULT_IMPORT_FOOTER_RESERVE: f32 = 82.0;

#[derive(Debug, Clone, Copy, PartialEq)]
enum UnitDimension {
    Dimensionless,
    Time,
    Frequency,
    Voltage,
    Current,
    Resistance,
    Conductance,
    Power,
    Capacitance,
    Inductance,
    Temperature,
    Angle,
    LogRatio,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct UnitContract {
    dimension: UnitDimension,
    scale: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct ColumnContract {
    name: String,
    unit: Option<UnitContract>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParsedResultDataset {
    pub(crate) source_format: ResultImportFormat,
    pub(crate) analysis_type: AnalysisType,
    pub(crate) coordinate_name: String,
    pub(crate) sample_count: usize,
    pub(crate) waveforms: Vec<WaveformData>,
    pub(crate) family_metadata: Option<AnalysisResultFamilyMetadata>,
    pub(crate) delimiter: u8,
}

/// Every import identifier declared by the neutral result-data contract.
///
/// Being present here means the format can be identified and governed. It
/// does not mean an adapter is available; `parse_result_dataset` fails closed
/// for identified formats without a lossless implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResultImportFormat {
    RSpiceResultBundle,
    RSpiceDatasetBundle,
    CsvRfc4180,
    Tsv,
    TouchstoneV1,
    TouchstoneV2,
    Hdf5,
    ArrowIpc,
    Parquet,
    NumpyNpy,
    NumpyNpz,
    MatlabV5,
    MatlabV73,
    SpiceRaw,
    PsfAscii,
    Vcd,
    Fst,
}

impl ResultImportFormat {
    const ALL: [Self; 17] = [
        Self::RSpiceResultBundle,
        Self::RSpiceDatasetBundle,
        Self::CsvRfc4180,
        Self::Tsv,
        Self::TouchstoneV1,
        Self::TouchstoneV2,
        Self::Hdf5,
        Self::ArrowIpc,
        Self::Parquet,
        Self::NumpyNpy,
        Self::NumpyNpz,
        Self::MatlabV5,
        Self::MatlabV73,
        Self::SpiceRaw,
        Self::PsfAscii,
        Self::Vcd,
        Self::Fst,
    ];

    pub(crate) const fn canonical_id(self) -> &'static str {
        match self {
            Self::RSpiceResultBundle => "rspice-result-bundle",
            Self::RSpiceDatasetBundle => "rspice-dataset-bundle",
            Self::CsvRfc4180 => "csv-rfc4180",
            Self::Tsv => "tsv",
            Self::TouchstoneV1 => "touchstone-v1",
            Self::TouchstoneV2 => "touchstone-v2",
            Self::Hdf5 => "hdf5",
            Self::ArrowIpc => "arrow-ipc",
            Self::Parquet => "parquet",
            Self::NumpyNpy => "numpy-npy",
            Self::NumpyNpz => "numpy-npz",
            Self::MatlabV5 => "matlab-v5",
            Self::MatlabV73 => "matlab-v7.3",
            Self::SpiceRaw => "spice-raw",
            Self::PsfAscii => "psf-ascii",
            Self::Vcd => "vcd",
            Self::Fst => "fst",
        }
    }

    fn from_canonical_id(id: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|format| format.canonical_id() == id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoordinateDirection {
    Increasing,
    Decreasing,
}

pub(crate) fn import_result_dataset(state: &mut AppState) -> bool {
    if let Some(reason) = result_import_block_reason(state) {
        state.push_user_message(ConsoleMessage::warning(reason));
        return false;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(path) = rfd::FileDialog::new()
            .add_filter(RESULT_DATASET_FILTER.0, RESULT_DATASET_FILTER.1)
            .set_title("Import result dataset")
            .pick_file()
        else {
            return false;
        };
        match read_native_file_bounded(&path).and_then(|bytes| {
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            stage_imported_result_dataset(state, &display_name, &bytes)
        }) {
            Ok(()) => true,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Result dataset import failed: {error}"
                )));
                false
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        match start_browser_result_dataset_import() {
            Ok(()) => true,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Result dataset import failed: {error}"
                )));
                false
            }
        }
    }
}

fn result_import_block_reason(state: &AppState) -> Option<String> {
    if !state.project_lifecycle.project_open {
        return Some("Open a project before importing a result dataset.".to_owned());
    }
    if state.workbench.safe_mode.project_read_only() {
        return Some("The project is read-only; result history cannot be changed.".to_owned());
    }
    if state.simulation.has_active_execution() {
        return Some(
            "Wait for the active simulation execution to finish before importing result data."
                .to_owned(),
        );
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn read_native_file_bounded(path: &Path) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(path)
        .map_err(|error| format!("could not open {}: {error}", path.display()))?;
    let size = file
        .metadata()
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
        .len();
    if size > MAX_RESULT_DATASET_BYTES {
        return Err(format!(
            "{} is {} bytes; the import limit is {} bytes",
            path.display(),
            size,
            MAX_RESULT_DATASET_BYTES
        ));
    }

    let mut bytes = Vec::with_capacity(
        usize::try_from(size.min(MAX_RESULT_DATASET_BYTES))
            .unwrap_or(MAX_RESULT_DATASET_BYTES as usize),
    );
    file.take(MAX_RESULT_DATASET_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    if bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(format!(
            "{} exceeds the {}-byte import limit",
            path.display(),
            MAX_RESULT_DATASET_BYTES
        ));
    }
    Ok(bytes)
}

/// Parse a selected file into a reviewable draft without mutating retained
/// simulation history.
pub(crate) fn stage_imported_result_dataset(
    state: &mut AppState,
    source_name: &str,
    bytes: &[u8],
) -> Result<(), String> {
    if let Some(reason) = result_import_block_reason(state) {
        return Err(reason);
    }
    let parsed = parse_result_dataset(source_name, bytes)?;
    let signal_count = parsed.waveforms.len();
    state.workbench.result_import = ResultImportDialogState {
        open: true,
        stage: ResultImportStage::Detect,
        source_name: source_name.to_owned(),
        source_format_id: parsed.source_format.canonical_id().to_owned(),
        delimiter: parsed.delimiter,
        analysis_type: parsed.analysis_type,
        coordinate_name: parsed.coordinate_name,
        sample_count: parsed.sample_count,
        waveforms: Arc::new(parsed.waveforms),
        family_metadata: parsed.family_metadata,
        selected_signals: vec![true; signal_count],
        validation_error: None,
    };
    Ok(())
}

fn parsed_result_from_draft(
    draft: &ResultImportDialogState,
) -> Result<ParsedResultDataset, String> {
    if draft.source_name.trim().is_empty() {
        return Err("The import source name is empty.".to_owned());
    }
    let coordinate_name = draft.coordinate_name.trim();
    if coordinate_name.is_empty() {
        return Err("Enter a coordinate name before committing the import.".to_owned());
    }
    if !matches!(
        draft.analysis_type,
        AnalysisType::Transient
            | AnalysisType::Ac
            | AnalysisType::DcSweep
            | AnalysisType::SParameter
    ) {
        return Err(
            "Select a supported transient, AC, DC-sweep, or S-parameter domain.".to_owned(),
        );
    }
    let source_format = ResultImportFormat::from_canonical_id(&draft.source_format_id)
        .ok_or_else(|| "The staged import format identity is missing or invalid.".to_owned())?;
    let touchstone = matches!(
        source_format,
        ResultImportFormat::TouchstoneV1 | ResultImportFormat::TouchstoneV2
    );
    if touchstone && draft.analysis_type != AnalysisType::SParameter {
        return Err("Touchstone network data must remain in the S-parameter domain.".to_owned());
    }
    if !touchstone && draft.analysis_type == AnalysisType::SParameter {
        return Err(
            "S-parameter mapping requires a Touchstone source with reference-impedance authority."
                .to_owned(),
        );
    }
    if draft.selected_signals.len() != draft.waveforms.len() {
        return Err("The signal mapping no longer matches the parsed source.".to_owned());
    }
    let waveforms = draft
        .waveforms
        .iter()
        .zip(&draft.selected_signals)
        .filter_map(|(waveform, selected)| selected.then_some(waveform.clone()))
        .collect::<Vec<_>>();
    if waveforms.is_empty() {
        return Err("Select at least one signal to import.".to_owned());
    }
    if waveforms.iter().any(|waveform| {
        waveform.x.len() != draft.sample_count || waveform.y.len() != draft.sample_count
    }) {
        return Err("A selected signal no longer matches the detected coordinate grid.".to_owned());
    }
    if matches!(
        draft.analysis_type,
        AnalysisType::Ac | AnalysisType::SParameter
    ) && waveforms
        .first()
        .is_some_and(|waveform| waveform.x.iter().any(|frequency| *frequency <= 0.0))
    {
        return Err("Frequency coordinates must all be greater than zero.".to_owned());
    }
    if draft.analysis_type == AnalysisType::SParameter {
        let Some(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
        }) = draft.family_metadata.as_ref()
        else {
            return Err(
                "Touchstone import is missing per-port reference-impedance authority.".to_owned(),
            );
        };
        if reference_impedances_ohm.is_empty()
            || waveforms.iter().any(|waveform| waveform.complex.is_none())
        {
            return Err("Touchstone import contains incomplete complex network data.".to_owned());
        }
    }
    Ok(ParsedResultDataset {
        source_format,
        analysis_type: draft.analysis_type,
        coordinate_name: coordinate_name.to_owned(),
        sample_count: draft.sample_count,
        waveforms,
        family_metadata: draft.family_metadata.clone(),
        delimiter: draft.delimiter,
    })
}

/// Atomically commit the currently reviewed draft into retained result
/// history. Any failure restores the simulation and workbench selections that
/// existed before the transaction.
pub(crate) fn commit_result_import_draft(state: &mut AppState) -> Result<(), String> {
    if let Some(reason) = result_import_block_reason(state) {
        return Err(reason);
    }
    let draft = state.workbench.result_import.clone();
    if !draft.open {
        return Err("No result import draft is open.".to_owned());
    }
    let parsed = parsed_result_from_draft(&draft)?;
    commit_parsed_result_dataset(state, &draft.source_name, parsed)?;
    state.workbench.result_import = ResultImportDialogState::default();
    Ok(())
}

fn commit_parsed_result_dataset(
    state: &mut AppState,
    source_name: &str,
    parsed: ParsedResultDataset,
) -> Result<(), String> {
    let analysis_type = parsed.analysis_type;
    let coordinate_name = parsed.coordinate_name.clone();
    let sample_count = parsed.sample_count;
    let signal_count = parsed.waveforms.len();
    let analysis_label = match parsed.analysis_type {
        AnalysisType::Transient => format!("Imported transient · {coordinate_name}"),
        AnalysisType::Ac => format!("Imported AC · {coordinate_name}"),
        AnalysisType::DcSweep => format!("Imported DC sweep · {coordinate_name}"),
        AnalysisType::SParameter => format!("Imported S-parameters · {coordinate_name}"),
        _ => return Err("import inferred an unsupported analysis domain".to_owned()),
    };
    let mut analysis =
        AnalysisResult::new(1, analysis_type, analysis_label).with_waveforms(parsed.waveforms);
    if let Some(metadata) = parsed.family_metadata {
        metadata.validate_for(analysis_type)?;
        analysis = analysis.with_family_metadata(metadata);
    }
    analysis.validate_retained_evidence()?;

    let previous_simulation = state.simulation.clone();
    let previous_workbench = state.workbench.clone();

    let (run_sequence, run_id, dataset_id) = {
        let run = state.simulation.start_run();
        run.job_id = None;
        run.execution_target = None;
        run.label = format!("Imported · {source_name}");
        run.add_analysis(analysis);
        let run_sequence = run.id;
        let run_id = run.run_id;
        let dataset_id = run.dataset_id;
        let sealed = run
            .restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .and_then(|()| run.mark_running())
            .and_then(|()| run.finish_lifecycle(SimulationRunLifecycle::Completed));
        if let Err(error) = sealed {
            state.simulation = previous_simulation;
            return Err(format!("could not seal imported result history: {error}"));
        }
        (run_sequence, run_id, dataset_id)
    };

    state.simulation.complete_run();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    let document = crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id);
    if !crate::workbench::chrome::document_bar::activate_document_by_id(state, &document) {
        state.simulation = previous_simulation;
        state.workbench = previous_workbench;
        return Err(
            "The imported dataset could not be activated; no result history was changed."
                .to_owned(),
        );
    }
    state.ui.results.viewer = if analysis_type == AnalysisType::SParameter {
        crate::workbench::ResultViewer::Smith
    } else {
        crate::workbench::ResultViewer::Waves
    };
    state.synchronize_specialized_viewer_cache_authority();
    state.push_user_message(ConsoleMessage::info(format!(
        "Imported {source_name} as immutable dataset {dataset_id}: {signal_count} signals × {sample_count} samples ({})",
        analysis_domain_label(analysis_type)
    )));
    state.push_user_message(ConsoleMessage::warning(format!(
        "Dataset {dataset_id} / run {run_id} (Run {run_sequence}) is external legacy-unattributed evidence, not native RSpice solver output"
    )));
    Ok(())
}

/// Render the staged external-result import transaction.
///
/// Closing or cancelling this window only discards the runtime draft. The
/// retained simulation model is touched exclusively by the final commit.
pub(crate) fn show_result_import_dialog(ctx: &egui::Context, state: &mut AppState) {
    if !state.workbench.result_import.open {
        return;
    }

    let mut draft = state.workbench.result_import.clone();
    let mut window_open = true;
    let mut cancel = false;
    let mut go_back = false;
    let mut advance = false;
    let mut commit = false;
    let tokens = Tokens::get(ctx);
    let (max_window_size, body_height) = result_import_dialog_geometry(ctx.content_rect().size());
    let max_window_width = max_window_size.x;
    let max_window_height = max_window_size.y;

    egui::Window::new("Import result dataset")
        .id(egui::Id::new("rspice.result-import"))
        .open(&mut window_open)
        .collapsible(false)
        .resizable(true)
        .default_width(860.0_f32.min(max_window_width))
        .min_width(680.0_f32.min(max_window_width))
        .max_width(max_window_width)
        .min_height(520.0_f32.min(max_window_height))
        .max_height(max_window_height)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("rspice.result-import.body")
                .max_height(body_height)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("RESULTS · EXTERNAL DATASET")
                            .monospace()
                            .color(tokens.color.text_dim),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        "Review the detected structure and mapping before RSpice creates immutable external result history.",
                    );
                    ui.add_space(10.0);
                    result_import_stage_header(ui, draft.stage);
                    ui.separator();

                    match draft.stage {
                        ResultImportStage::Detect => result_import_detect_page(ui, &draft),
                        ResultImportStage::Map => result_import_map_page(ui, &mut draft),
                        ResultImportStage::Validate => result_import_validate_page(ui, &draft),
                    }

                    ui.add_space(8.0);
                    if let Some(error) = draft.validation_error.as_deref() {
                        ui.colored_label(tokens.color.err, error);
                    }
                });

            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                match draft.stage {
                    ResultImportStage::Detect => {
                        advance = ui.button("Map signals").clicked();
                    }
                    ResultImportStage::Map => {
                        let valid = parsed_result_from_draft(&draft).is_ok();
                        advance = ui
                            .add_enabled(valid, egui::Button::new("Validate import"))
                            .clicked();
                        go_back = ui.button("Back").clicked();
                    }
                    ResultImportStage::Validate => {
                        let valid = parsed_result_from_draft(&draft).is_ok()
                            && result_import_block_reason(state).is_none();
                        commit = ui
                            .add_enabled(valid, egui::Button::new("Import dataset"))
                            .clicked();
                        go_back = ui.button("Back").clicked();
                    }
                }
                cancel = ui.button("Cancel").clicked();
            });
        });

    if !window_open || cancel {
        discard_result_import_draft(state);
        return;
    }
    if go_back {
        draft.stage = match draft.stage {
            ResultImportStage::Detect | ResultImportStage::Map => ResultImportStage::Detect,
            ResultImportStage::Validate => ResultImportStage::Map,
        };
        draft.validation_error = None;
    } else if advance {
        draft.stage = match draft.stage {
            ResultImportStage::Detect => ResultImportStage::Map,
            ResultImportStage::Map => ResultImportStage::Validate,
            ResultImportStage::Validate => ResultImportStage::Validate,
        };
        draft.validation_error = parsed_result_from_draft(&draft).err();
    }

    state.workbench.result_import = draft;
    if let Some(Err(error)) = commit.then(|| commit_result_import_draft(state)) {
        state.workbench.result_import.open = true;
        state.workbench.result_import.stage = ResultImportStage::Validate;
        state.workbench.result_import.validation_error = Some(error);
    }
}

fn result_import_dialog_geometry(available: egui::Vec2) -> (egui::Vec2, f32) {
    let maximum = egui::vec2(
        (available.x - RESULT_IMPORT_WINDOW_MARGIN).max(1.0),
        (available.y - RESULT_IMPORT_WINDOW_MARGIN).max(1.0),
    );
    let body_height = (maximum.y - RESULT_IMPORT_FOOTER_RESERVE).max(1.0);
    (maximum, body_height)
}

fn discard_result_import_draft(state: &mut AppState) {
    state.workbench.result_import = ResultImportDialogState::default();
}

fn result_import_stage_header(ui: &mut egui::Ui, active: ResultImportStage) {
    ui.horizontal(|ui| {
        for (index, (stage, label)) in [
            (ResultImportStage::Detect, "1  Detect"),
            (ResultImportStage::Map, "2  Map"),
            (ResultImportStage::Validate, "3  Validate"),
        ]
        .into_iter()
        .enumerate()
        {
            let mut label = egui::RichText::new(label);
            if stage == active {
                label = label.strong();
            }
            if stage == active {
                ui.label(label.color(Tokens::get(ui.ctx()).color.accent));
            } else {
                ui.label(label.color(Tokens::get(ui.ctx()).color.text_dim));
            }
            if index < 2 {
                ui.label(egui::RichText::new("›").color(Tokens::get(ui.ctx()).color.text_dim));
            }
        }
    });
}

fn result_import_detect_page(ui: &mut egui::Ui, draft: &ResultImportDialogState) {
    ui.heading("Detected source");
    egui::Grid::new("rspice.result-import.detected")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            result_import_summary_row(ui, "File", &draft.source_name);
            result_import_summary_row(ui, "Format", &draft.source_format_id);
            if matches!(draft.source_format_id.as_str(), "csv-rfc4180" | "tsv") {
                result_import_summary_row(
                    ui,
                    "Delimiter",
                    if draft.delimiter == b'\t' {
                        "Tab"
                    } else {
                        "Comma"
                    },
                );
            }
            result_import_summary_row(
                ui,
                "Inferred domain",
                analysis_domain_label(draft.analysis_type),
            );
            result_import_summary_row(ui, "Coordinate", &draft.coordinate_name);
            result_import_summary_row(ui, "Signals", &draft.waveforms.len().to_string());
            result_import_summary_row(ui, "Samples per signal", &draft.sample_count.to_string());
        });
    ui.add_space(12.0);
    ui.label(
        "No run or analysis has been created. Continue to confirm the engineering domain, coordinate, and included signals.",
    );
}

fn result_import_map_page(ui: &mut egui::Ui, draft: &mut ResultImportDialogState) {
    ui.heading("Map engineering data");
    egui::Grid::new("rspice.result-import.mapping")
        .num_columns(2)
        .spacing([18.0, 8.0])
        .show(ui, |ui| {
            ui.label("Analysis domain");
            let domain_locked = matches!(
                draft.source_format_id.as_str(),
                "touchstone-v1" | "touchstone-v2"
            );
            ui.add_enabled_ui(!domain_locked, |ui| {
                egui::ComboBox::from_id_salt("rspice.result-import.domain")
                    .selected_text(analysis_domain_label(draft.analysis_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut draft.analysis_type,
                            AnalysisType::Transient,
                            "Transient",
                        );
                        ui.selectable_value(&mut draft.analysis_type, AnalysisType::Ac, "AC");
                        ui.selectable_value(
                            &mut draft.analysis_type,
                            AnalysisType::DcSweep,
                            "DC sweep",
                        );
                    });
            });
            ui.end_row();
            ui.label("Coordinate name");
            ui.add(
                egui::TextEdit::singleline(&mut draft.coordinate_name)
                    .id_salt("rspice.result-import.coordinate")
                    .desired_width(320.0),
            );
            ui.end_row();
        });

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.strong("Signals");
        if ui.small_button("Select all").clicked() {
            draft.selected_signals.fill(true);
        }
        if ui.small_button("Select none").clicked() {
            draft.selected_signals.fill(false);
        }
        ui.label(
            egui::RichText::new(format!(
                "{} of {} selected",
                draft
                    .selected_signals
                    .iter()
                    .filter(|selected| **selected)
                    .count(),
                draft.selected_signals.len()
            ))
            .small()
            .color(Tokens::get(ui.ctx()).color.text_dim),
        );
    });
    egui::ScrollArea::vertical()
        .id_salt("rspice.result-import.signals")
        .max_height(132.0)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (waveform, selected) in draft
                .waveforms
                .iter()
                .zip(draft.selected_signals.iter_mut())
            {
                ui.checkbox(selected, result_import_signal_label(waveform));
            }
        });

    ui.add_space(10.0);
    ui.strong("Source preview");
    result_import_preview(ui, draft);
    draft.validation_error = parsed_result_from_draft(draft).err();
}

fn result_import_validate_page(ui: &mut egui::Ui, draft: &ResultImportDialogState) {
    ui.heading("Validate import");
    let selected = draft
        .selected_signals
        .iter()
        .filter(|selected| **selected)
        .count();
    egui::Grid::new("rspice.result-import.validation")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            result_import_summary_row(ui, "Source", &draft.source_name);
            result_import_summary_row(ui, "Format", &draft.source_format_id);
            result_import_summary_row(ui, "Domain", analysis_domain_label(draft.analysis_type));
            result_import_summary_row(ui, "Coordinate", &draft.coordinate_name);
            result_import_summary_row(ui, "Included signals", &selected.to_string());
            result_import_summary_row(ui, "Samples per signal", &draft.sample_count.to_string());
            result_import_summary_row(
                ui,
                "Retained evidence",
                "Immutable · external legacy-unattributed",
            );
        });
    ui.add_space(12.0);
    match parsed_result_from_draft(draft) {
        Ok(_) => {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.ok,
                "Validation passed. Import will add one completed retained run and open it in Results.",
            );
        }
        Err(error) => {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
    }
    ui.add_space(10.0);
    result_import_preview(ui, draft);
}

fn result_import_summary_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.label(egui::RichText::new(label).strong());
    ui.label(value);
    ui.end_row();
}

fn result_import_signal_label(waveform: &WaveformData) -> String {
    waveform.unit.as_deref().map_or_else(
        || waveform.name.clone(),
        |unit| format!("{}  [{}]", waveform.name, unit),
    )
}

fn result_import_preview(ui: &mut egui::Ui, draft: &ResultImportDialogState) {
    const MAX_PREVIEW_ROWS: usize = 8;
    const MAX_PREVIEW_SIGNALS: usize = 8;

    let included = draft
        .waveforms
        .iter()
        .zip(&draft.selected_signals)
        .filter_map(|(waveform, selected)| selected.then_some(waveform))
        .take(MAX_PREVIEW_SIGNALS)
        .collect::<Vec<_>>();
    if included.is_empty() {
        ui.label(
            egui::RichText::new("Select at least one signal to preview.")
                .color(Tokens::get(ui.ctx()).color.warn),
        );
        return;
    }

    egui::ScrollArea::horizontal()
        .id_salt("rspice.result-import.preview-scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("rspice.result-import.preview")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.strong(&draft.coordinate_name);
                    for waveform in &included {
                        ui.strong(result_import_signal_label(waveform));
                    }
                    ui.end_row();
                    for row in 0..draft.sample_count.min(MAX_PREVIEW_ROWS) {
                        ui.monospace(format!("{:.6e}", included[0].x[row]));
                        for waveform in &included {
                            ui.monospace(format!("{:.6e}", waveform.y[row]));
                        }
                        ui.end_row();
                    }
                });
        });
    if draft.sample_count > MAX_PREVIEW_ROWS || included.len() < draft.waveforms.len() {
        ui.label(
            egui::RichText::new(format!(
                "Preview is limited to {MAX_PREVIEW_ROWS} rows and {MAX_PREVIEW_SIGNALS} included signals."
            ))
            .small()
            .color(Tokens::get(ui.ctx()).color.text_dim),
        );
    }
}

#[cfg(test)]
fn apply_imported_result_dataset(
    state: &mut AppState,
    source_name: &str,
    bytes: &[u8],
) -> Result<(), String> {
    stage_imported_result_dataset(state, source_name, bytes)?;
    commit_result_import_draft(state)
}

fn analysis_domain_label(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::Transient => "transient",
        AnalysisType::Ac => "AC",
        AnalysisType::DcSweep => "DC sweep",
        AnalysisType::SParameter => "S-parameter",
        _ => "unsupported",
    }
}

pub(crate) fn parse_result_dataset(
    source_name: &str,
    bytes: &[u8],
) -> Result<ParsedResultDataset, String> {
    if bytes.is_empty() {
        return Err("the selected file is empty".to_owned());
    }
    if bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(format!(
            "the selected file exceeds the {}-byte import limit",
            MAX_RESULT_DATASET_BYTES
        ));
    }
    let format = identify_result_import_format(source_name, bytes)?;
    match format {
        ResultImportFormat::CsvRfc4180 | ResultImportFormat::Tsv => {
            let text = std::str::from_utf8(bytes)
                .map_err(|error| format!("the selected file is not valid UTF-8: {error}"))?;
            let delimiter = if format == ResultImportFormat::Tsv {
                b'\t'
            } else {
                b','
            };
            let mut parsed = parse_delimited_result_dataset(text, delimiter)?;
            parsed.source_format = format;
            Ok(parsed)
        }
        ResultImportFormat::TouchstoneV1 | ResultImportFormat::TouchstoneV2 => {
            parse_touchstone_result_dataset(source_name, bytes, format)
        }
        ResultImportFormat::RSpiceResultBundle | ResultImportFormat::RSpiceDatasetBundle => {
            adapters::parse_native_bundle(bytes, format)
        }
        ResultImportFormat::Hdf5 => adapters::parse_hdf5(bytes, format),
        ResultImportFormat::ArrowIpc => adapters::parse_arrow_ipc(bytes, format),
        ResultImportFormat::Parquet => adapters::parse_parquet(bytes, format),
        ResultImportFormat::NumpyNpy => adapters::parse_npy(bytes, format),
        ResultImportFormat::NumpyNpz => adapters::parse_npz(bytes, format),
        ResultImportFormat::MatlabV5 => adapters::parse_matlab_v5(bytes, format),
        ResultImportFormat::MatlabV73 => adapters::parse_matlab_v73(bytes, format),
        ResultImportFormat::SpiceRaw => adapters::parse_spice_raw(bytes, format),
        ResultImportFormat::PsfAscii => adapters::parse_psf_ascii(bytes, format),
        ResultImportFormat::Vcd => adapters::parse_vcd(bytes, format),
        ResultImportFormat::Fst => adapters::parse_fst(bytes, format),
    }
}

fn identify_result_import_format(
    source_name: &str,
    bytes: &[u8],
) -> Result<ResultImportFormat, String> {
    let extension = Path::new(source_name)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    let strong_signature = strong_result_format_signature(bytes);
    let by_extension = match extension.as_deref() {
        Some("rspiceresult") => Some(ResultImportFormat::RSpiceResultBundle),
        Some("rspicedata") => Some(ResultImportFormat::RSpiceDatasetBundle),
        Some("csv") => Some(ResultImportFormat::CsvRfc4180),
        Some("tsv" | "tab") => Some(ResultImportFormat::Tsv),
        Some("ts") => Some(ResultImportFormat::TouchstoneV2),
        Some("snp") => Some(if text_looks_touchstone_v2(bytes) {
            ResultImportFormat::TouchstoneV2
        } else {
            ResultImportFormat::TouchstoneV1
        }),
        Some(extension)
            if extension.starts_with('s')
                && extension.ends_with('p')
                && extension.len() > 2
                && extension[1..extension.len() - 1]
                    .bytes()
                    .all(|byte| byte.is_ascii_digit()) =>
        {
            Some(ResultImportFormat::TouchstoneV1)
        }
        Some("h5" | "hdf5") => Some(ResultImportFormat::Hdf5),
        Some("arrow" | "feather") => Some(ResultImportFormat::ArrowIpc),
        Some("parquet") => Some(ResultImportFormat::Parquet),
        Some("npy") => Some(ResultImportFormat::NumpyNpy),
        Some("npz") => Some(ResultImportFormat::NumpyNpz),
        Some("mat") => Some(if bytes.starts_with(b"\x89HDF\r\n\x1a\n") {
            ResultImportFormat::MatlabV73
        } else {
            ResultImportFormat::MatlabV5
        }),
        Some("raw") => Some(ResultImportFormat::SpiceRaw),
        Some("psfascii") => Some(ResultImportFormat::PsfAscii),
        Some("vcd") => Some(ResultImportFormat::Vcd),
        Some("fst") => Some(ResultImportFormat::Fst),
        _ => None,
    };

    if let (Some(extension_format), Some(signature_format)) = (by_extension, strong_signature)
        && extension_format != signature_format
        && !matches!(
            (extension_format, signature_format),
            (ResultImportFormat::MatlabV73, ResultImportFormat::Hdf5)
        )
    {
        return Err(format!(
            "the .{} extension identifies '{}' but the file signature identifies '{}'; refusing an ambiguous import",
            extension.as_deref().unwrap_or(""),
            extension_format.canonical_id(),
            signature_format.canonical_id()
        ));
    }
    if let Some(format) = by_extension.or(strong_signature) {
        return Ok(format);
    }

    let text = std::str::from_utf8(bytes).map_err(|_| {
        "the result format is unknown and has no recognized binary signature".to_owned()
    })?;
    if text_looks_touchstone_v2(bytes) {
        return Ok(ResultImportFormat::TouchstoneV2);
    }
    if text.lines().any(|line| line.trim_start().starts_with('#'))
        && text.to_ascii_lowercase().contains(" s ")
    {
        return Ok(ResultImportFormat::TouchstoneV1);
    }
    let delimiter = infer_delimiter(source_name, text)?;
    Ok(if delimiter == b'\t' {
        ResultImportFormat::Tsv
    } else {
        ResultImportFormat::CsvRfc4180
    })
}

fn strong_result_format_signature(bytes: &[u8]) -> Option<ResultImportFormat> {
    if bytes.starts_with(b"\x89HDF\r\n\x1a\n") {
        Some(ResultImportFormat::Hdf5)
    } else if bytes.starts_with(b"MATLAB 5.0 MAT-file") {
        Some(ResultImportFormat::MatlabV5)
    } else if bytes.starts_with(b"\x93NUMPY") {
        Some(ResultImportFormat::NumpyNpy)
    } else if bytes.starts_with(b"PK\x03\x04") {
        None
    } else if bytes.starts_with(b"PAR1") && bytes.ends_with(b"PAR1") {
        Some(ResultImportFormat::Parquet)
    } else if bytes.starts_with(b"ARROW1") || bytes.ends_with(b"ARROW1") {
        Some(ResultImportFormat::ArrowIpc)
    } else if adapters::looks_like_fst(bytes) {
        Some(ResultImportFormat::Fst)
    } else {
        let prefix = std::str::from_utf8(bytes.get(..bytes.len().min(8_192))?).ok()?;
        if prefix.contains("$timescale") && prefix.contains("$scope") {
            Some(ResultImportFormat::Vcd)
        } else if prefix.lines().any(|line| line.starts_with("Plotname:"))
            && prefix
                .lines()
                .any(|line| line.starts_with("No. Variables:"))
        {
            Some(ResultImportFormat::SpiceRaw)
        } else {
            None
        }
    }
}

fn text_looks_touchstone_v2(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes.get(..bytes.len().min(8_192)).unwrap_or(bytes)).is_ok_and(|prefix| {
        prefix.lines().any(|line| {
            line.trim_start()
                .to_ascii_lowercase()
                .starts_with("[version]")
        })
    })
}

fn parse_touchstone_result_dataset(
    source_name: &str,
    bytes: &[u8],
    identified_format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let dataset = crate::io::waveform_io::read_touchstone_bytes(source_name, bytes)?;
    let version = dataset
        .metadata
        .get("touchstone_version")
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| "Touchstone adapter did not return a version identity".to_owned())?;
    let parsed_format = if version >= 2 {
        ResultImportFormat::TouchstoneV2
    } else {
        ResultImportFormat::TouchstoneV1
    };
    if identified_format != parsed_format {
        return Err(format!(
            "the source was identified as '{}' but declares '{}'; refusing an ambiguous import",
            identified_format.canonical_id(),
            parsed_format.canonical_id()
        ));
    }
    let x = dataset
        .x_signal
        .as_ref()
        .ok_or_else(|| "Touchstone source has no frequency axis".to_owned())?;
    let coordinate = Arc::new(x.data.clone());
    let mut components: BTreeMap<String, ComplexComponentColumns> = BTreeMap::new();
    for signal in dataset.signals {
        let (base, imaginary) = if let Some(base) = signal.name.strip_suffix("_RE") {
            (base, false)
        } else if let Some(base) = signal.name.strip_suffix("_IM") {
            (base, true)
        } else {
            return Err(format!(
                "Touchstone adapter returned an untyped component '{}'",
                signal.name
            ));
        };
        let entry = components.entry(base.to_owned()).or_default();
        let slot = if imaginary {
            &mut entry.1
        } else {
            &mut entry.0
        };
        if slot.replace(signal.data).is_some() {
            return Err(format!("Touchstone source repeats component '{base}'"));
        }
    }
    let mut waveforms = Vec::with_capacity(components.len());
    for (index, (name, (real, imaginary))) in components.into_iter().enumerate() {
        let real = real.ok_or_else(|| format!("Touchstone source is missing {name}_RE"))?;
        let imaginary =
            imaginary.ok_or_else(|| format!("Touchstone source is missing {name}_IM"))?;
        if real.len() != coordinate.len() || imaginary.len() != coordinate.len() {
            return Err(format!(
                "Touchstone parameter {name} does not match the frequency grid"
            ));
        }
        let magnitude = real
            .iter()
            .zip(&imaginary)
            .map(|(real, imaginary)| real.hypot(*imaginary))
            .collect::<Vec<_>>();
        waveforms.push(
            WaveformData::new(
                format!("|{name}|"),
                Arc::clone(&coordinate),
                magnitude,
                trace_color(index),
            )
            .with_complex_components(name, real, imaginary),
        );
    }
    let reference_impedances_ohm = dataset
        .metadata
        .get("z0_ports")
        .ok_or_else(|| "Touchstone source has no reference-impedance metadata".to_owned())?
        .split(',')
        .map(|value| {
            value
                .parse::<f64>()
                .map_err(|_| "Touchstone adapter returned invalid impedance metadata".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let family_metadata = AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
    };
    family_metadata.validate_for(AnalysisType::SParameter)?;
    Ok(ParsedResultDataset {
        source_format: parsed_format,
        analysis_type: AnalysisType::SParameter,
        coordinate_name: "frequency".to_owned(),
        sample_count: coordinate.len(),
        waveforms,
        family_metadata: Some(family_metadata),
        delimiter: 0,
    })
}

fn infer_delimiter(source_name: &str, text: &str) -> Result<u8, String> {
    let extension = Path::new(source_name)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    match extension.as_deref() {
        Some("csv") => return Ok(b','),
        Some("tsv") => return Ok(b'\t'),
        Some(extension) => {
            return Err(format!(
                "unsupported .{extension} result format; select a UTF-8 CSV or TSV file"
            ));
        }
        None => {}
    }

    let header = text
        .lines()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| "the selected file contains no header row".to_owned())?;
    let commas = header.bytes().filter(|byte| *byte == b',').count();
    let tabs = header.bytes().filter(|byte| *byte == b'\t').count();
    match (commas, tabs) {
        (0, 0) => Err("could not infer CSV or TSV delimiter from the header".to_owned()),
        (commas, tabs) if commas > 0 && tabs > 0 => Err(
            "the header mixes comma and tab delimiters; use a consistent CSV or TSV file"
                .to_owned(),
        ),
        (_, 0) => Ok(b','),
        (0, _) => Ok(b'\t'),
        _ => unreachable!(),
    }
}

fn parse_delimited_result_dataset(
    text: &str,
    delimiter: u8,
) -> Result<ParsedResultDataset, String> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .flexible(false)
        .trim(csv::Trim::All)
        .from_reader(text.as_bytes());
    let raw_headers = reader
        .headers()
        .map_err(|error| csv_error("header", error))?
        .clone();
    if raw_headers.len() < 2 {
        return Err("the header must contain one coordinate and at least one signal".to_owned());
    }
    if raw_headers.len() > MAX_RESULT_COLUMNS {
        return Err(format!(
            "the dataset has {} columns; the import limit is {}",
            raw_headers.len(),
            MAX_RESULT_COLUMNS
        ));
    }

    let mut headers = Vec::with_capacity(raw_headers.len());
    let mut unique_names = HashSet::with_capacity(raw_headers.len());
    for (index, raw) in raw_headers.iter().enumerate() {
        let raw = if index == 0 {
            raw.strip_prefix('\u{feff}').unwrap_or(raw)
        } else {
            raw
        };
        let header = parse_column_header(raw, index + 1)?;
        let key = header.name.to_lowercase();
        if !unique_names.insert(key) {
            return Err(format!(
                "column {} repeats the signal/header name {:?}",
                index + 1,
                header.name
            ));
        }
        headers.push(header);
    }

    let (analysis_type, coordinate_scale) = infer_analysis_type(&headers[0])?;
    let signal_scales: Vec<f64> = headers[1..]
        .iter()
        .map(|header| header.unit.map_or(1.0, |unit| unit.scale))
        .collect();
    let mut coordinate = Vec::new();
    let mut signal_values = vec![Vec::new(); headers.len() - 1];
    let mut direction = None;

    for (row_index, record) in reader.records().enumerate() {
        let line = row_index + 2;
        if row_index >= MAX_RESULT_ROWS {
            return Err(format!(
                "the dataset exceeds the {}-row import limit",
                MAX_RESULT_ROWS
            ));
        }
        let record = record.map_err(|error| csv_error(&format!("row {line}"), error))?;
        let x = parse_finite_cell(record.get(0), line, 1, &headers[0].name, coordinate_scale)?;
        if analysis_type == AnalysisType::Ac && x <= 0.0 {
            return Err(format!(
                "row {line} frequency must be greater than zero after unit conversion"
            ));
        }
        if let Some(previous) = coordinate.last().copied() {
            let step = if x > previous {
                CoordinateDirection::Increasing
            } else if x < previous {
                CoordinateDirection::Decreasing
            } else {
                return Err(format!(
                    "row {line} repeats coordinate {x}; coordinates must be strictly monotonic"
                ));
            };
            if let Some(expected) = direction {
                if step != expected {
                    return Err(format!(
                        "row {line} reverses coordinate ordering; coordinates must remain strictly monotonic"
                    ));
                }
            } else {
                direction = Some(step);
            }
        }
        coordinate.push(x);

        for (signal_index, values) in signal_values.iter_mut().enumerate() {
            let column = signal_index + 2;
            values.push(parse_finite_cell(
                record.get(signal_index + 1),
                line,
                column,
                &headers[signal_index + 1].name,
                signal_scales[signal_index],
            )?);
        }
    }
    if coordinate.len() < MIN_RESULT_ROWS {
        return Err(format!(
            "the dataset contains {} sample rows; at least {} are required",
            coordinate.len(),
            MIN_RESULT_ROWS
        ));
    }

    let coordinate = Arc::new(coordinate);
    let waveforms = signal_values
        .into_iter()
        .enumerate()
        .map(|(index, values)| {
            WaveformData::new(
                headers[index + 1].name.clone(),
                Arc::clone(&coordinate),
                values,
                trace_color(index),
            )
        })
        .collect();
    Ok(ParsedResultDataset {
        source_format: if delimiter == b'\t' {
            ResultImportFormat::Tsv
        } else {
            ResultImportFormat::CsvRfc4180
        },
        analysis_type,
        coordinate_name: headers[0].name.clone(),
        sample_count: coordinate.len(),
        waveforms,
        family_metadata: None,
        delimiter,
    })
}

fn parse_column_header(raw: &str, column: usize) -> Result<ColumnContract, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(format!("column {column} has an empty header"));
    }
    if raw.len() > MAX_HEADER_BYTES {
        return Err(format!(
            "column {column} header exceeds the {}-byte limit",
            MAX_HEADER_BYTES
        ));
    }
    if raw.chars().any(char::is_control) {
        return Err(format!(
            "column {column} header contains a control character"
        ));
    }

    let (name, unit_text) = if raw.ends_with(']') {
        let open = raw.rfind('[').ok_or_else(|| {
            format!("column {column} header has a closing unit bracket without an opening bracket")
        })?;
        let name = raw[..open].trim();
        let unit = raw[open + 1..raw.len() - 1].trim();
        if name.is_empty() || unit.is_empty() {
            return Err(format!(
                "column {column} must provide both a name and a non-empty bracketed unit"
            ));
        }
        (name, Some(unit))
    } else {
        if raw.contains('[') || raw.contains(']') {
            return Err(format!("column {column} has an unmatched unit bracket"));
        }
        (raw, None)
    };
    let unit = unit_text
        .map(parse_unit)
        .transpose()
        .map_err(|error| format!("column {column} ({name:?}) has an invalid unit: {error}"))?;
    Ok(ColumnContract {
        name: name.to_owned(),
        unit,
    })
}

fn parse_unit(raw: &str) -> Result<UnitContract, String> {
    let symbol = raw
        .trim()
        .replace(['µ', 'μ'], "u")
        .replace('Ω', "ohm")
        .replace('°', "deg");
    let contract = match symbol.as_str() {
        "s" => (UnitDimension::Time, 1.0),
        "ms" => (UnitDimension::Time, 1e-3),
        "us" => (UnitDimension::Time, 1e-6),
        "ns" => (UnitDimension::Time, 1e-9),
        "ps" => (UnitDimension::Time, 1e-12),
        "fs" => (UnitDimension::Time, 1e-15),
        "Hz" | "hz" => (UnitDimension::Frequency, 1.0),
        "mHz" => (UnitDimension::Frequency, 1e-3),
        "kHz" | "KHz" => (UnitDimension::Frequency, 1e3),
        "MHz" => (UnitDimension::Frequency, 1e6),
        "GHz" => (UnitDimension::Frequency, 1e9),
        "THz" => (UnitDimension::Frequency, 1e12),
        "V" => (UnitDimension::Voltage, 1.0),
        "mV" => (UnitDimension::Voltage, 1e-3),
        "uV" => (UnitDimension::Voltage, 1e-6),
        "nV" => (UnitDimension::Voltage, 1e-9),
        "kV" => (UnitDimension::Voltage, 1e3),
        "A" => (UnitDimension::Current, 1.0),
        "mA" => (UnitDimension::Current, 1e-3),
        "uA" => (UnitDimension::Current, 1e-6),
        "nA" => (UnitDimension::Current, 1e-9),
        "pA" => (UnitDimension::Current, 1e-12),
        "ohm" => (UnitDimension::Resistance, 1.0),
        "mohm" => (UnitDimension::Resistance, 1e-3),
        "kohm" => (UnitDimension::Resistance, 1e3),
        "Mohm" => (UnitDimension::Resistance, 1e6),
        "Gohm" => (UnitDimension::Resistance, 1e9),
        "S" => (UnitDimension::Conductance, 1.0),
        "mS" => (UnitDimension::Conductance, 1e-3),
        "uS" => (UnitDimension::Conductance, 1e-6),
        "nS" => (UnitDimension::Conductance, 1e-9),
        "W" => (UnitDimension::Power, 1.0),
        "mW" => (UnitDimension::Power, 1e-3),
        "uW" => (UnitDimension::Power, 1e-6),
        "nW" => (UnitDimension::Power, 1e-9),
        "kW" => (UnitDimension::Power, 1e3),
        "F" => (UnitDimension::Capacitance, 1.0),
        "mF" => (UnitDimension::Capacitance, 1e-3),
        "uF" => (UnitDimension::Capacitance, 1e-6),
        "nF" => (UnitDimension::Capacitance, 1e-9),
        "pF" => (UnitDimension::Capacitance, 1e-12),
        "fF" => (UnitDimension::Capacitance, 1e-15),
        "H" => (UnitDimension::Inductance, 1.0),
        "mH" => (UnitDimension::Inductance, 1e-3),
        "uH" => (UnitDimension::Inductance, 1e-6),
        "nH" => (UnitDimension::Inductance, 1e-9),
        "K" => (UnitDimension::Temperature, 1.0),
        "C" | "degC" => (UnitDimension::Temperature, 1.0),
        _ => match symbol.to_ascii_lowercase().as_str() {
            "1" | "unitless" | "dimensionless" => (UnitDimension::Dimensionless, 1.0),
            "second" | "seconds" => (UnitDimension::Time, 1.0),
            "hertz" => (UnitDimension::Frequency, 1.0),
            "volt" | "volts" => (UnitDimension::Voltage, 1.0),
            "amp" | "amps" | "ampere" | "amperes" => (UnitDimension::Current, 1.0),
            "ohms" => (UnitDimension::Resistance, 1.0),
            "megohm" | "megohms" => (UnitDimension::Resistance, 1e6),
            "siemens" | "siemen" => (UnitDimension::Conductance, 1.0),
            "watt" | "watts" => (UnitDimension::Power, 1.0),
            "farad" | "farads" => (UnitDimension::Capacitance, 1.0),
            "henry" | "henries" => (UnitDimension::Inductance, 1.0),
            "kelvin" | "celsius" => (UnitDimension::Temperature, 1.0),
            "rad" | "radian" | "radians" => (UnitDimension::Angle, 1.0),
            "deg" | "degree" | "degrees" => (UnitDimension::Angle, 1.0),
            "db" => (UnitDimension::LogRatio, 1.0),
            "%" | "percent" => (UnitDimension::Dimensionless, 0.01),
            _ => return Err(format!("{raw:?} is not a recognized engineering unit")),
        },
    };
    Ok(UnitContract {
        dimension: contract.0,
        scale: contract.1,
    })
}

fn infer_analysis_type(header: &ColumnContract) -> Result<(AnalysisType, f64), String> {
    let normalized: String = header
        .name
        .chars()
        .filter(|character| !matches!(character, '_' | '-' | ' '))
        .flat_map(char::to_lowercase)
        .collect();
    let (analysis_type, required_dimension, default_scale) = match normalized.as_str() {
        "time" | "t" | "timestamp" => (AnalysisType::Transient, Some(UnitDimension::Time), 1.0),
        "frequency" | "freq" => (AnalysisType::Ac, Some(UnitDimension::Frequency), 1.0),
        "f" if header
            .unit
            .is_some_and(|unit| unit.dimension == UnitDimension::Frequency) =>
        {
            (AnalysisType::Ac, Some(UnitDimension::Frequency), 1.0)
        }
        "f" => {
            return Err(
                "coordinate header \"f\" is ambiguous without an explicit frequency unit such as [Hz]"
                    .to_owned(),
            );
        }
        _ if looks_like_signal_expression(&header.name) => {
            return Err(format!(
                "first column {:?} looks like a signal, not a coordinate; use time, frequency, or a DC sweep variable header",
                header.name
            ));
        }
        _ if header
            .unit
            .is_some_and(|unit| unit.dimension == UnitDimension::Time) =>
        {
            (AnalysisType::Transient, Some(UnitDimension::Time), 1.0)
        }
        _ if header
            .unit
            .is_some_and(|unit| unit.dimension == UnitDimension::Frequency) =>
        {
            (AnalysisType::Ac, Some(UnitDimension::Frequency), 1.0)
        }
        _ => (AnalysisType::DcSweep, None, 1.0),
    };

    if let (Some(required), Some(unit)) = (required_dimension, header.unit)
        && unit.dimension != required
    {
        return Err(format!(
            "{} coordinate {:?} requires a {} unit",
            analysis_domain_label(analysis_type),
            header.name,
            match required {
                UnitDimension::Time => "time",
                UnitDimension::Frequency => "frequency",
                _ => "compatible",
            }
        ));
    }
    if analysis_type == AnalysisType::DcSweep
        && header.unit.is_some_and(|unit| {
            matches!(
                unit.dimension,
                UnitDimension::Angle | UnitDimension::LogRatio
            )
        })
    {
        return Err(format!(
            "DC sweep coordinate {:?} cannot use an angular or logarithmic display unit",
            header.name
        ));
    }
    Ok((
        analysis_type,
        header.unit.map_or(default_scale, |unit| unit.scale),
    ))
}

fn looks_like_signal_expression(name: &str) -> bool {
    let trimmed = name.trim();
    let lower = trimmed.to_ascii_lowercase();
    (lower.starts_with("v(") || lower.starts_with("i(")) && trimmed.ends_with(')')
}

fn parse_finite_cell(
    value: Option<&str>,
    row: usize,
    column: usize,
    header: &str,
    scale: f64,
) -> Result<f64, String> {
    let value =
        value.ok_or_else(|| format!("row {row}, column {column} ({header:?}) is missing"))?;
    if value.is_empty() {
        return Err(format!("row {row}, column {column} ({header:?}) is empty"));
    }
    let parsed = value.parse::<f64>().map_err(|_| {
        format!("row {row}, column {column} ({header:?}) contains non-numeric value {value:?}")
    })?;
    if !parsed.is_finite() {
        return Err(format!(
            "row {row}, column {column} ({header:?}) must be finite"
        ));
    }
    // Every accepted engineering-unit scale is an exact decimal power of
    // ten. Parse the combined decimal exponent in one operation so values
    // such as `10 [uA]` round directly to the nearest representation of
    // `1e-5`, instead of accumulating a second rounding from `10.0 * 1e-6`.
    let scaled = decimal_power_scaled(value, scale).unwrap_or(parsed * scale);
    if !scaled.is_finite() {
        return Err(format!(
            "row {row}, column {column} ({header:?}) overflows after unit conversion"
        ));
    }
    Ok(scaled)
}

fn decimal_power_scaled(value: &str, scale: f64) -> Option<f64> {
    let scale_exponent = [
        (1e-15, -15),
        (1e-12, -12),
        (1e-9, -9),
        (1e-6, -6),
        (1e-3, -3),
        (0.01, -2),
        (1.0, 0),
        (1e3, 3),
        (1e6, 6),
        (1e9, 9),
        (1e12, 12),
    ]
    .into_iter()
    .find_map(|(candidate, exponent)| (scale == candidate).then_some(exponent))?;
    let (mantissa, source_exponent) = if let Some(separator) = value.find(['e', 'E']) {
        (
            &value[..separator],
            value[separator + 1..].parse::<i32>().ok()?,
        )
    } else {
        (value, 0_i32)
    };
    let combined_exponent = source_exponent.checked_add(scale_exponent)?;
    format!("{mantissa}e{combined_exponent}")
        .parse::<f64>()
        .ok()
}

fn csv_error(context: &str, error: csv::Error) -> String {
    if let Some(position) = error.position() {
        format!(
            "{context} is malformed near row {}, byte {}: {}",
            position.line(),
            position.byte(),
            error
        )
    } else {
        format!("{context} is malformed: {error}")
    }
}

fn trace_color(index: usize) -> &'static str {
    const COLORS: &[&str] = &[
        "#4FC3F7", "#FFB74D", "#81C784", "#BA68C8", "#E57373", "#4DB6AC", "#FFF176", "#7986CB",
    ];
    COLORS[index % COLORS.len()]
}

#[cfg(target_arch = "wasm32")]
enum BrowserResultDatasetImportResult {
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
    Failed(String),
    Cancelled,
}

#[cfg(target_arch = "wasm32")]
struct BrowserResultDatasetImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: BrowserResultDatasetImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_RESULT_DATASET_IMPORT_RESULT:
        std::cell::RefCell<Option<BrowserResultDatasetImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_result_dataset_import() -> Result<(), String> {
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::ResultDataset,
    )?;
    crate::workbench::browser::file_import::pick_text_file(
        RESULT_DATASET_FILTER.0,
        RESULT_DATASET_FILTER.1,
        move |result| {
            if !crate::workbench::browser::file_import::text_import_is_current(token) {
                return;
            }
            let result = match result {
                Ok(Some(file)) => BrowserResultDatasetImportResult::Loaded(file),
                Ok(None) => BrowserResultDatasetImportResult::Cancelled,
                Err(error) => BrowserResultDatasetImportResult::Failed(error),
            };
            BROWSER_RESULT_DATASET_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserResultDatasetImportCompletion { token, result });
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_result_dataset_import(state: &mut AppState) -> bool {
    let Some(completion) =
        BROWSER_RESULT_DATASET_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return false;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return false;
    }
    match completion.result {
        BrowserResultDatasetImportResult::Loaded(file) => {
            let bytes = file
                .original_bytes
                .as_deref()
                .unwrap_or(file.contents.as_bytes());
            match stage_imported_result_dataset(state, &file.name, bytes) {
                Ok(()) => true,
                Err(error) => {
                    state.push_user_message(ConsoleMessage::error(format!(
                        "Result dataset import failed: {error}"
                    )));
                    false
                }
            }
        }
        BrowserResultDatasetImportResult::Failed(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Result dataset import failed: {error}"
            )));
            false
        }
        BrowserResultDatasetImportResult::Cancelled => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loaded_project_state() -> AppState {
        let mut state = AppState::default();
        let baseline = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
            .expect("project snapshot");
        crate::workbench::lifecycle::project_lifecycle::accept_loaded_project(
            &mut state, baseline, None,
        );
        state
    }

    #[test]
    fn import_dialog_reserves_a_visible_footer_inside_desktop_viewports() {
        for available in [egui::vec2(1_440.0, 900.0), egui::vec2(1_024.0, 768.0)] {
            let (maximum, body) = result_import_dialog_geometry(available);
            assert_eq!(maximum, available - egui::vec2(24.0, 24.0));
            assert!(body > 0.0);
            assert!(body + RESULT_IMPORT_FOOTER_RESERVE <= maximum.y);
            assert!(maximum.x <= available.x);
            assert!(maximum.y <= available.y);
        }
    }

    #[test]
    fn parses_utf8_csv_transient_with_units_and_shared_coordinate() {
        let parsed = parse_result_dataset(
            "scope.csv",
            b"time [ms],V(out) [mV],I(VDD) [uA]\n0,0,10\n0.5,1250,11\n1,2500,12\n",
        )
        .expect("valid transient CSV");

        assert_eq!(parsed.analysis_type, AnalysisType::Transient);
        assert_eq!(parsed.sample_count, 3);
        assert_eq!(parsed.waveforms.len(), 2);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[0.0, 5e-4, 1e-3]);
        assert_eq!(parsed.waveforms[0].y.as_slice(), &[0.0, 1.25, 2.5]);
        assert_eq!(parsed.waveforms[1].y.as_slice(), &[10e-6, 11e-6, 12e-6]);
        assert!(Arc::ptr_eq(&parsed.waveforms[0].x, &parsed.waveforms[1].x));
        assert_eq!(parsed.source_format, ResultImportFormat::CsvRfc4180);
        assert!(parsed.family_metadata.is_none());
    }

    #[test]
    fn format_registry_matches_the_seventeen_contract_import_ids() {
        let ids = ResultImportFormat::ALL.map(ResultImportFormat::canonical_id);
        assert_eq!(
            ids,
            [
                "rspice-result-bundle",
                "rspice-dataset-bundle",
                "csv-rfc4180",
                "tsv",
                "touchstone-v1",
                "touchstone-v2",
                "hdf5",
                "arrow-ipc",
                "parquet",
                "numpy-npy",
                "numpy-npz",
                "matlab-v5",
                "matlab-v7.3",
                "spice-raw",
                "psf-ascii",
                "vcd",
                "fst",
            ]
        );
        for extension in [
            "rspiceresult",
            "rspicedata",
            "csv",
            "tsv",
            "tab",
            "s1p",
            "s2p",
            "s3p",
            "s4p",
            "snp",
            "ts",
            "h5",
            "hdf5",
            "arrow",
            "feather",
            "parquet",
            "npy",
            "npz",
            "mat",
            "raw",
            "psfascii",
            "vcd",
            "fst",
        ] {
            assert!(RESULT_DATASET_FILTER.1.contains(&extension), "{extension}");
        }
    }

    #[test]
    fn structured_formats_reject_malformed_sources_and_signatures_defeat_spoofed_extensions() {
        for (name, bytes, id) in [
            ("samples.npy", b"\x93NUMPY\x01\x00".as_slice(), "numpy-npy"),
            ("table.parquet", b"PAR1payloadPAR1".as_slice(), "parquet"),
            (
                "capture.vcd",
                b"$timescale 1ns $end\n$scope module top $end\n".as_slice(),
                "vcd",
            ),
        ] {
            let error = parse_result_dataset(name, bytes).expect_err("malformed source rejects");
            assert!(error.contains(id), "{error}");
        }

        let error = parse_result_dataset("spoofed.csv", b"PAR1payloadPAR1")
            .expect_err("extension/signature mismatch must reject");
        assert!(error.contains("csv-rfc4180"), "{error}");
        assert!(error.contains("parquet"), "{error}");
        assert!(error.contains("ambiguous"), "{error}");
    }

    #[test]
    fn touchstone_v2_import_retains_complex_matrix_and_reference_authority() {
        let mut state = loaded_project_state();
        apply_imported_result_dataset(
            &mut state,
            "network.ts",
            b"[Version] 2.0\n[Number of Ports] 2\n[Number of Frequencies] 2\n# MHz S RI R 50\n[Reference] 50 75\n[Network Data]\n1 0.1 0 0.2 0 0.3 0 0.4 0\n2 0.5 0 0.6 0 0.7 0 0.8 0\n[End]\n",
        )
        .expect("Touchstone import succeeds");

        let analysis = state.simulation.active_analysis().expect("active import");
        assert_eq!(analysis.analysis_type, AnalysisType::SParameter);
        assert_eq!(analysis.waveforms.len(), 4);
        assert_eq!(analysis.waveforms[0].x.as_slice(), &[1.0e6, 2.0e6]);
        assert!(analysis.waveforms.iter().all(|waveform| {
            waveform.name.starts_with("|S")
                && waveform
                    .complex
                    .as_ref()
                    .is_some_and(|complex| complex.real.len() == 2 && complex.imag.len() == 2)
        }));
        assert_eq!(
            analysis.family_metadata,
            Some(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![50.0, 75.0],
            })
        );
        assert!(analysis.validate_retained_evidence().is_ok());
        assert_eq!(
            state.ui.results.viewer,
            crate::workbench::ResultViewer::Smith
        );
        assert_eq!(
            state.workbench.result_import.source_format_id, "",
            "successful commit discards the runtime import draft"
        );
    }

    #[test]
    fn parses_tsv_frequency_as_ac_and_requires_positive_ordered_frequency() {
        let parsed = parse_result_dataset(
            "response.tsv",
            b"frequency [kHz]\tV(out) [V]\n1\t0.5\n10\t0.25\n100\t0.1\n",
        )
        .expect("valid AC TSV");
        assert_eq!(parsed.analysis_type, AnalysisType::Ac);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[1e3, 1e4, 1e5]);

        let error = parse_result_dataset("response.csv", b"frequency [Hz],V(out) [V]\n1,1\n0,2\n")
            .expect_err("zero frequency rejected");
        assert!(error.contains("greater than zero"));
    }

    #[test]
    fn infers_domain_from_unambiguous_coordinate_units() {
        let transient = parse_result_dataset("capture.csv", b"sample [us],V(out) [V]\n0,0\n1,1\n")
            .expect("time unit identifies transient data");
        assert_eq!(transient.analysis_type, AnalysisType::Transient);

        let ac = parse_result_dataset("capture.csv", b"axis [MHz],V(out) [V]\n1,1\n2,0.5\n")
            .expect("frequency unit identifies AC data");
        assert_eq!(ac.analysis_type, AnalysisType::Ac);
    }

    #[test]
    fn infers_dc_sweep_and_accepts_strictly_decreasing_coordinate() {
        let parsed = parse_result_dataset("sweep.csv", b"V1 [V],V(out) [V]\n5,4.5\n2.5,2.2\n0,0\n")
            .expect("decreasing DC sweep remains ordered");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.waveforms[0].x.as_slice(), &[5.0, 2.5, 0.0]);
    }

    #[test]
    fn preserves_case_sensitive_siemens_units() {
        let parsed = parse_result_dataset("conductance.csv", b"bias [V],g(out) [mS]\n0,1\n1,2\n")
            .expect("mS is conductance, not milliseconds");
        assert_eq!(parsed.analysis_type, AnalysisType::DcSweep);
        assert_eq!(parsed.waveforms[0].y.as_slice(), &[1e-3, 2e-3]);
    }

    #[test]
    fn rejects_duplicate_headers_unknown_units_nonfinite_values_and_reversals() {
        for (source, expected) in [
            (
                b"time [s],V(out) [V],v(OUT) [mV]\n0,0,0\n1,1,1\n".as_slice(),
                "repeats",
            ),
            (
                b"time [fortnight],V(out) [V]\n0,0\n1,1\n".as_slice(),
                "not a recognized engineering unit",
            ),
            (
                b"time [s],V(out) [V]\n0,NaN\n1,1\n".as_slice(),
                "must be finite",
            ),
            (
                b"time [s],V(out) [V]\n0,0\n2,1\n1,2\n".as_slice(),
                "reverses coordinate ordering",
            ),
        ] {
            let error = parse_result_dataset("bad.csv", source).expect_err("invalid import");
            assert!(error.contains(expected), "{error:?}");
        }
    }

    #[test]
    fn imported_dataset_is_completed_stable_immutable_and_selected() {
        let mut state = loaded_project_state();
        assert!(
            !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state),
            "accepted baseline starts clean"
        );
        apply_imported_result_dataset(
            &mut state,
            "external.csv",
            b"time [s],V(out) [V]\n0,0\n1,1\n",
        )
        .expect("import succeeds");

        let run = state
            .simulation
            .active_run()
            .expect("imported run selected");
        assert_eq!(run.lifecycle, SimulationRunLifecycle::Completed);
        assert!(run.job_id.is_none());
        assert!(run.execution_target.is_none());
        assert!(matches!(
            run.provenance(),
            Some(SimulationRunProvenance::LegacyUnattributed)
        ));
        assert_eq!(run.analyses.len(), 1);
        assert_eq!(
            state
                .simulation
                .active_analysis()
                .map(|analysis| analysis.analysis_type),
            Some(AnalysisType::Transient)
        );
        assert_ne!(run.run_id.to_string(), run.dataset_id.to_string());
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Results
        );
        assert_eq!(
            state
                .workbench
                .documents
                .active(crate::workbench::state::Workspace::Results),
            Some(&crate::workbench::state::WorkspaceDocumentId::ResultDataset(run.dataset_id))
        );
        assert!(
            crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state),
            "retaining external evidence must dirty canonical result history"
        );
    }

    #[test]
    fn staging_and_discarding_import_never_mutates_retained_history() {
        let mut state = loaded_project_state();
        let initial_run_count = state.simulation.runs.len();
        let initial_workspace = state.workbench.workspace;

        stage_imported_result_dataset(
            &mut state,
            "external.csv",
            b"time [s],V(out) [V]\n0,0\n1,1\n",
        )
        .expect("stage succeeds");

        assert!(state.workbench.result_import.open);
        assert_eq!(
            state.workbench.result_import.stage,
            ResultImportStage::Detect
        );
        assert_eq!(state.simulation.runs.len(), initial_run_count);
        assert_eq!(state.workbench.workspace, initial_workspace);
        assert!(
            !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state),
            "a review draft is presentation state, not project history"
        );

        discard_result_import_draft(&mut state);
        assert!(!state.workbench.result_import.open);
        assert_eq!(state.simulation.runs.len(), initial_run_count);
        assert_eq!(state.workbench.workspace, initial_workspace);
        assert!(!crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state));
    }

    #[test]
    fn validation_failure_retains_draft_and_leaves_history_unchanged() {
        let mut state = loaded_project_state();
        stage_imported_result_dataset(
            &mut state,
            "external.csv",
            b"time [s],V(out) [V]\n0,0\n1,1\n",
        )
        .expect("stage succeeds");
        state.workbench.result_import.selected_signals.fill(false);

        let error = commit_result_import_draft(&mut state).expect_err("empty mapping rejected");
        assert!(error.contains("at least one signal"), "{error}");
        assert!(state.workbench.result_import.open);
        assert!(state.simulation.runs.is_empty());
        assert!(!crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&state));
    }

    #[test]
    fn commit_imports_only_explicitly_selected_signals() {
        let mut state = loaded_project_state();
        stage_imported_result_dataset(
            &mut state,
            "external.csv",
            b"time [s],V(out) [V],I(VDD) [A]\n0,0,1\n1,1,2\n",
        )
        .expect("stage succeeds");
        state.workbench.result_import.selected_signals[1] = false;

        commit_result_import_draft(&mut state).expect("commit succeeds");

        let analysis = state.simulation.active_analysis().expect("active import");
        assert_eq!(analysis.waveforms.len(), 1);
        assert_eq!(analysis.waveforms[0].name, "V(out)");
        assert!(!state.workbench.result_import.open);
    }

    #[test]
    fn remapping_zero_coordinate_data_to_ac_is_rejected_before_commit() {
        let mut state = loaded_project_state();
        stage_imported_result_dataset(
            &mut state,
            "external.csv",
            b"time [s],V(out) [V]\n0,0\n1,1\n",
        )
        .expect("stage succeeds");
        state.workbench.result_import.analysis_type = AnalysisType::Ac;

        let error = commit_result_import_draft(&mut state).expect_err("invalid frequency rejected");
        assert!(error.contains("greater than zero"), "{error}");
        assert!(state.simulation.runs.is_empty());
    }
}
