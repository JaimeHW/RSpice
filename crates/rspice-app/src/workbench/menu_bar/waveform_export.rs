//! Waveform export actions.

mod hdf5;
mod matlab;
mod noise_stack;
mod numpy;
mod typed_csv;
mod vcd;

use crate::workbench::EngineeringExportFormat;
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use rspice_formats::table::csv_to_tsv;
use rspice_formats::waveform_io::result::{
    WaveformProjectionError, project_waveforms as prepare_single_analysis_dataset,
};
use rspice_results_ui::eye_diagram::EyeTimebaseProvenance;
use typed_csv::prepare_typed_result_csv;

const NO_ACTIVE_ANALYSIS_MESSAGE: &str = "No active result analysis is selected for export.";
const NO_SAMPLES_MESSAGE: &str = "No waveform samples available to export.";

/// Headers and container metadata also count against the receiving workflow's
/// byte limit. Refuse an oversized artifact before selecting a destination.
fn admit_result_export_bytes(bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    let limit = crate::workbench::workflows::result_import_workflow::MAX_RESULT_DATASET_BYTES;
    if bytes.len() as u64 > limit {
        return Err(format!(
            "The encoded result is {} bytes; the import limit is {limit} bytes. Hide traces or export a smaller selection.",
            bytes.len()
        ));
    }
    Ok(bytes)
}
/// An export that silently wrote an empty file would be indistinguishable
/// from one that wrote the dataset, so the hidden traces are named as the
/// reason rather than reported as an absence of samples.
const ALL_TRACES_HIDDEN_MESSAGE: &str = "Every trace in the displayed analysis is hidden, so there is nothing to export. \
     Show at least one trace first.";

// Touchstone carries an S-parameter network on a frequency axis and nothing
// else, so each table route names what it holds instead of the reader being
// told only that the format did not fit.
const DERIVED_VIEWER_TOUCHSTONE_REFUSAL: &str =
    "Touchstone export is not compatible with a derived viewer; select CSV export.";
const RESULTS_SHEET_TOUCHSTONE_REFUSAL: &str =
    "Touchstone export is not compatible with this Results sheet; select CSV export.";
const TYPED_RESULT_TOUCHSTONE_REFUSAL: &str =
    "Touchstone export is not compatible with the active typed result; select CSV export.";
const ANALYSIS_STACK_TOUCHSTONE_REFUSAL: &str = "Touchstone export requires one selected analysis; maximize one displayed strip or select CSV export.";

fn note_result_export_failure(state: &mut AppState, detail: impl Into<String>) {
    let data_version = state.simulation.view.data_version;
    state.ui.results.record_runtime_condition(
        crate::workbench::documents::result_document::operational_state::ResultRuntimeConditionKind::Failed,
        detail,
        data_version,
    );
}

fn note_result_export_success(state: &mut AppState, format: &str) {
    let data_version = state.simulation.view.data_version;
    state.ui.results.record_runtime_recovery_if(
        crate::workbench::documents::result_document::operational_state::ResultRuntimeConditionKind::Failed,
        format!("{format} publication succeeded after the recorded export failure."),
        data_version,
    );
}

/// Canonical export vocabulary from the result-data contract. Availability is
/// explicit: an entry is never implied to have an encoder merely because it
/// is part of the design contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResultExportFormat {
    RSpiceResultBundle,
    RSpiceDatasetBundle,
    CsvRfc4180,
    Tsv,
    TouchstoneV2,
    Hdf5,
    NumpyNpy,
    NumpyNpz,
    MatlabV5,
    Vcd,
}

impl ResultExportFormat {
    const ALL: [Self; 10] = [
        Self::RSpiceResultBundle,
        Self::RSpiceDatasetBundle,
        Self::CsvRfc4180,
        Self::Tsv,
        Self::TouchstoneV2,
        Self::Hdf5,
        Self::NumpyNpy,
        Self::NumpyNpz,
        Self::MatlabV5,
        Self::Vcd,
    ];

    const fn canonical_id(self) -> &'static str {
        match self {
            Self::RSpiceResultBundle => "rspice-result-bundle",
            Self::RSpiceDatasetBundle => "rspice-dataset-bundle",
            Self::CsvRfc4180 => "csv-rfc4180",
            Self::Tsv => "tsv",
            Self::TouchstoneV2 => "touchstone-v2",
            Self::Hdf5 => "hdf5",
            Self::NumpyNpy => "numpy-npy",
            Self::NumpyNpz => "numpy-npz",
            Self::MatlabV5 => "matlab-v5",
            Self::Vcd => "vcd",
        }
    }
}

/// Refuse an id this build cannot write.
///
/// Every id in [`ResultExportFormat`] now has an encoder, so the only refusal
/// left is an id that is not in the vocabulary at all. The check stays because
/// the vocabulary is the contract's, not the picker's: a caller may hand over
/// a string that came from a durable preference or an automation script.
fn result_export_format_availability_by_id(canonical_id: &str) -> Result<(), String> {
    ResultExportFormat::ALL
        .into_iter()
        .find(|format| format.canonical_id() == canonical_id)
        .map(|_| ())
        .ok_or_else(|| format!("Unknown result export format '{canonical_id}'."))
}

/// The formats a table of the displayed view can answer.
///
/// Every other offered format publishes something a table is not — a native
/// bundle, a transient's event schedule, a NumPy array — and is routed to its
/// own module before any table is prepared. Narrowing to those three once, at
/// the point the routing is decided, is what keeps the two table routers below
/// from each restating which formats never reach them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TabularExportFormat {
    Csv,
    Tsv,
    TouchstoneWhereCompatible,
}

/// Export the exact dataset-bound Data Browser selection, never whatever
/// payload the currently visible sheet happens to choose as its default.
pub(crate) fn action_export_result_selection_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    keys: &[rspice_results_ui::selection::ResultBrowserSelectionKey],
) {
    if keys.is_empty() {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            "No Data Browser quantities are selected for exact export.".to_owned(),
        ));
        return;
    }
    let contents =
        match crate::workbench::documents::result_document::exact_result_browser_selection_bundle(
            keys,
            &state.simulation.retained.runs,
        ) {
            Ok(contents) => contents,
            Err(message) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
                return;
            }
        };
    let name_source = if keys.len() == 1 {
        crate::workbench::documents::result_document::result_browser_selection_stable_path(
            &keys[0],
            &state.simulation.retained.runs,
        )
        .ok()
        .and_then(|path| path.rsplit('/').next().map(str::to_owned))
        .unwrap_or_else(|| "result-evidence".to_owned())
    } else {
        "selected-result-evidence".to_owned()
    };
    let slug = name_source
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    let default_name = format!("rspice-{slug}.txt");
    let export = match io.show_save_dialog(SaveDialogConfig {
        title: "Export Exact Result Evidence",
        default_name: &default_name,
        filter_name: "Text Evidence",
        filter_extensions: &["txt"],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "txt");
            io.observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &contents))
        }
        Ok(None) => return,
        Err(error) => Err(error),
    };
    match export {
        Ok(()) => {
            note_result_export_success(state, "Exact-result text");
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                "Exported {} exact retained result item(s).",
                keys.len()
            )));
        }
        Err(error) => {
            note_result_export_failure(
                state,
                format!("Exact retained-evidence export failed: {error}"),
            );
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Could not export exact retained evidence: {error}"
            )));
        }
    }
}

pub(crate) fn action_export_csv_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let displayed = match crate::workbench::documents::result_document::view_context::resolve_displayed_result_view(state) {
        Ok(displayed) => displayed,
        Err(message) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
            return;
        }
    };
    // Viewer compatibility intentionally excludes quarantined analyses. Check
    // the retained dataset itself here so that filtering cannot turn corrupt
    // evidence into an apparently empty, exportable presentation.
    if let Some(error) = displayed.run(state).and_then(|run| {
        run.analyses
            .iter()
            .find_map(|analysis| analysis.validate_retained_evidence().err())
    }) {
        state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "Result export was quarantined because retained-evidence verification failed: {error}"
        )));
        return;
    }
    let export_format = state
        .ui
        .preferences
        .result_presentation_policy()
        .engineering_export();
    let contract_format = match export_format {
        EngineeringExportFormat::Csv => ResultExportFormat::CsvRfc4180,
        EngineeringExportFormat::Tsv => ResultExportFormat::Tsv,
        EngineeringExportFormat::TouchstoneWhereCompatible => ResultExportFormat::TouchstoneV2,
        EngineeringExportFormat::RSpiceResultBundle => ResultExportFormat::RSpiceResultBundle,
        EngineeringExportFormat::RSpiceDatasetBundle => ResultExportFormat::RSpiceDatasetBundle,
        EngineeringExportFormat::ValueChangeDump => ResultExportFormat::Vcd,
        EngineeringExportFormat::NumpyArray => ResultExportFormat::NumpyNpy,
        EngineeringExportFormat::NumpyArchive => ResultExportFormat::NumpyNpz,
        EngineeringExportFormat::Hdf5EngineeringDataset => ResultExportFormat::Hdf5,
        EngineeringExportFormat::MatlabV5File => ResultExportFormat::MatlabV5,
    };
    if let Err(error) = result_export_format_availability_by_id(contract_format.canonical_id()) {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
        return;
    }
    // These encoders currently publish one retained analysis. Enforce that
    // before any route selects the primary analysis and discards other strips.
    let single_analysis_export = match export_format {
        EngineeringExportFormat::Csv
        | EngineeringExportFormat::Tsv
        | EngineeringExportFormat::TouchstoneWhereCompatible => false,
        EngineeringExportFormat::RSpiceResultBundle
        | EngineeringExportFormat::RSpiceDatasetBundle
        | EngineeringExportFormat::ValueChangeDump
        | EngineeringExportFormat::NumpyArray
        | EngineeringExportFormat::NumpyArchive
        | EngineeringExportFormat::Hdf5EngineeringDataset
        | EngineeringExportFormat::MatlabV5File => true,
    };
    if single_analysis_export && displayed.analysis_indices.len() > 1 {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            "This export supports one analysis, and this view shows several. Maximize one displayed strip to export it, or choose CSV/TSV to export the displayed stack."
                .to_owned(),
        ));
        return;
    }
    if let Some(kind) = match export_format {
        EngineeringExportFormat::RSpiceResultBundle => {
            Some(rspice_formats::native_bundle::NativeBundleKind::Result)
        }
        EngineeringExportFormat::RSpiceDatasetBundle => {
            Some(rspice_formats::native_bundle::NativeBundleKind::Dataset)
        }
        _ => None,
    } {
        // A native bundle carries the exact retained waveform analysis. It
        // must not quietly substitute that source for a derived or tabular
        // sheet that the reader is currently looking at.
        let owns_non_waveform_view = prepare_displayed_table(state, &displayed).is_some();
        if owns_non_waveform_view {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                "Native RSpice bundles export one retained shared-axis waveform analysis. Select a waveform sheet, or choose CSV/TSV for this derived or tabular view."
                    .to_owned(),
            ));
            return;
        }
        export_native_result_bundle(state, io, &displayed, kind);
        return;
    }
    // Binary payloads of the retained analysis, each publishing its own bytes
    // and returning here. A dump is the transient's event schedule, which no
    // Results sheet renders and no viewer derives; a NumPy export keeps the
    // complex signals complex, which the flat dataset below has already split
    // into magnitude and re/im columns. Routing either through the table
    // routers below would hand it whatever table those happened to find. A
    // format that falls through is narrowed to a table immediately after, and
    // that narrowing is exhaustive, so a new variant cannot be lost here.
    match export_format {
        EngineeringExportFormat::ValueChangeDump => {
            vcd::export_vcd(state, io, &displayed);
            return;
        }
        EngineeringExportFormat::NumpyArray => {
            numpy::export_numpy(state, io, &displayed, numpy::NumpyKind::Array);
            return;
        }
        EngineeringExportFormat::NumpyArchive => {
            numpy::export_numpy(state, io, &displayed, numpy::NumpyKind::Archive);
            return;
        }
        EngineeringExportFormat::Hdf5EngineeringDataset => {
            hdf5::export_hdf5(state, io, &displayed);
            return;
        }
        EngineeringExportFormat::MatlabV5File => {
            matlab::export_matlab(state, io, &displayed);
            return;
        }
        _ => {}
    }
    // Everything past this point publishes a table of the displayed view, so
    // the format is narrowed once here rather than re-examined by each router.
    let export_format = match export_format {
        EngineeringExportFormat::Csv => TabularExportFormat::Csv,
        EngineeringExportFormat::Tsv => TabularExportFormat::Tsv,
        EngineeringExportFormat::TouchstoneWhereCompatible => {
            TabularExportFormat::TouchstoneWhereCompatible
        }
        EngineeringExportFormat::RSpiceResultBundle
        | EngineeringExportFormat::RSpiceDatasetBundle
        | EngineeringExportFormat::ValueChangeDump
        | EngineeringExportFormat::NumpyArray
        | EngineeringExportFormat::NumpyArchive
        | EngineeringExportFormat::Hdf5EngineeringDataset
        | EngineeringExportFormat::MatlabV5File => {
            unreachable!("native bundle, dump, NumPy, HDF5 and MATLAB formats dispatch above")
        }
    };
    // What the reader is looking at comes first, and exactly one view owns
    // that table. Three sheets derive their curve in the viewer rather than
    // reading a retained vector, so routing on the payload alone handed back
    // the transient samples the spectrum was computed from and called it the
    // result.
    if let Some(table) = prepare_displayed_table(state, &displayed) {
        match table {
            Ok((prepared, touchstone_refusal)) => {
                export_prepared_table(state, io, export_format, &prepared, touchstone_refusal);
            }
            // The view owns this export and cannot produce it. Falling through
            // to the waveform router below is how an operating-point export
            // came back holding a transient's event history.
            Err(reason) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(reason));
            }
        }
        return;
    }

    let prepared = match prepare_waveform_dataset(
        state,
        &displayed,
        matches!(
            export_format,
            TabularExportFormat::TouchstoneWhereCompatible
        ),
    ) {
        Ok(prepared) => prepared,
        Err(message) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
            return;
        }
    };

    match export_format {
        TabularExportFormat::Csv => export_csv(state, io, &prepared),
        TabularExportFormat::Tsv => export_tsv(state, io, &prepared),
        TabularExportFormat::TouchstoneWhereCompatible => export_touchstone(state, io, &prepared),
    }
}

struct PreparedTypedResultCsv {
    default_name: &'static str,
    contents: String,
    detail: String,
}

/// The one table the displayed view publishes, and the sentence Touchstone
/// refuses that table with.
///
/// `None` means no view owns the export and the waveform payload below
/// answers for it. `Some(Err)` means a view owns it and cannot produce it,
/// which is a refusal rather than a reason to keep looking down the list.
fn prepare_displayed_table(
    state: &AppState,
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
) -> Option<Result<(PreparedTypedResultCsv, &'static str), String>> {
    if let Some(derived) = prepare_active_derived_view_csv(state, displayed) {
        return Some(derived.map(|derived| (derived, DERIVED_VIEWER_TOUCHSTONE_REFUSAL)));
    }
    if let Some(sheet) = prepare_active_sheet_csv(state, displayed) {
        return Some(sheet.map(|sheet| (sheet, RESULTS_SHEET_TOUCHSTONE_REFUSAL)));
    }
    if displayed.analysis_indices.len() > 1 {
        return Some(
            prepare_displayed_analysis_stack_csv(state, displayed)
                .map(|prepared| (prepared, ANALYSIS_STACK_TOUCHSTONE_REFUSAL)),
        );
    }
    if let Some(typed) = displayed
        .primary_analysis(state)
        .and_then(prepare_typed_result_csv)
    {
        return Some(Ok((typed, TYPED_RESULT_TOUCHSTONE_REFUSAL)));
    }
    None
}

/// Publish one prepared table, or say why Touchstone cannot carry this one.
fn export_prepared_table(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    format: TabularExportFormat,
    prepared: &PreparedTypedResultCsv,
    touchstone_refusal: &'static str,
) {
    match format {
        TabularExportFormat::Csv => export_typed_result_csv(state, io, prepared),
        TabularExportFormat::Tsv => export_typed_result_tsv(state, io, prepared),
        TabularExportFormat::TouchstoneWhereCompatible => state.push_user_message(
            crate::diagnostics::ConsoleMessage::warning(touchstone_refusal.to_owned()),
        ),
    }
}

/// The export a Results sheet owns, if this viewer has one.
///
/// `None` means the viewer has no sheet export and the payload router below
/// it should answer. `Some(Err)` means the sheet owns the export and cannot
/// produce it: the reader is told why rather than handed whatever the router
/// finds attached to the same analysis.
fn prepare_active_sheet_csv(
    state: &AppState,
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
) -> Option<Result<PreparedTypedResultCsv, String>> {
    use crate::workbench::ResultViewer;
    use crate::workbench::documents::result_document;

    if displayed.viewer == ResultViewer::NoiseContrib
        && displayed.analyses(state).any(|analysis| {
            matches!(
                analysis.result_payload,
                Some(crate::state::AnalysisResultPayload::Qpnoise { .. })
            )
        })
    {
        return Some(noise_stack::prepare(
            displayed.run(state)?,
            &displayed.analysis_indices,
        ));
    }
    let sheet = match displayed.viewer {
        ResultViewer::Manifest => Some(result_document::export_manifest_csv(displayed.run(state)?)),
        // The operating point is the one sheet whose refusal has to be
        // stated. Its export needs a successful DC solve, and the payload
        // router underneath it answers for an `OperatingPoint` payload with
        // the execution *contract* rather than the solved point, and for a
        // transient carrying `dc_op` with that transient's event history —
        // both under a menu item the reader pressed on the OP sheet.
        ResultViewer::Op => {
            let Some(analysis) = displayed.primary_analysis(state) else {
                // Nothing is bound to the sheet. A failed solve keeps the
                // payload router's answer, which names the run and carries
                // the engine's own reason. A *successful* result that simply
                // is not an operating point is the case the sheet has to
                // speak for: the router underneath would report on waveform
                // samples, which is not what the reader pressed.
                let selected = state.simulation.active_analysis()?;
                if !selected.success || selected.analysis_type == crate::state::AnalysisType::DcOp {
                    return None;
                }
                return Some(Err(format!(
                    "The operating point cannot be exported: analysis '{}' is a {} result, not \
                     a DC operating point.",
                    selected.label,
                    selected.analysis_type.short_label()
                )));
            };
            if !analysis.success {
                return Some(Err(format!(
                    "The operating point cannot be exported: analysis '{}' did not complete \
                     successfully.{}",
                    analysis.label,
                    analysis
                        .error_message
                        .as_deref()
                        .map_or_else(String::new, |error| format!(" {error}"))
                )));
            }
            let Some(sheet) = result_document::export_operating_point_csv(analysis) else {
                // Two refusals, because the export refuses for two reasons and
                // only one of them is about missing evidence. A transient that
                // retained its bias solution has a node DC solution; it is
                // simply not an operating-point result, and saying otherwise
                // sent the reader looking for evidence that was already there.
                return Some(Err(
                    if analysis.analysis_type != crate::state::AnalysisType::DcOp {
                        format!(
                            "The operating point cannot be exported: analysis '{}' is a {} \
                             result, not a DC operating point.",
                            analysis.label,
                            analysis.analysis_type.short_label()
                        )
                    } else {
                        format!(
                            "The operating point cannot be exported: analysis '{}' retains no \
                             node DC solution and no device operating-point report.",
                            analysis.label
                        )
                    },
                ));
            };
            Some(sheet)
        }
        // The workspace contract is handed over as the legacy fallback only:
        // the sheet judges the run against the requirements the run froze, and
        // `export_specs_csv` resolves that itself so this arm cannot write a
        // bound the sheet never showed.
        ResultViewer::Specs => Some(result_document::export_specs_csv(
            displayed.run(state)?,
            &state.workspace.content.specs,
        )),
        ResultViewer::Optimization => {
            result_document::export_optimization_csv(displayed.primary_analysis(state)?)
        }
        ResultViewer::NoiseContrib => result_document::export_noise_contribution_csv(
            displayed.run(state)?,
            &displayed.analysis_indices,
        ),
        _ => None,
    }?;
    Some(Ok(PreparedTypedResultCsv {
        default_name: sheet.default_name,
        contents: sheet.contents,
        detail: sheet.detail,
    }))
}

/// The exact numbers behind a sheet that derives its own curve.
///
/// The spectrum, the folded eye and the binned distribution exist only in the
/// viewer: nothing retains them, so the payload-driven export below cannot
/// see them and wrote out the source samples instead. A reader who exports
/// from the FFT sheet wants the spectrum, and every value here is the one
/// that was drawn — full `f64`, never the displayed rounding.
///
/// Sheets that plot a retained vector are deliberately absent. Their export
/// already is what they show.
fn prepare_active_derived_view_csv(
    state: &AppState,
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
) -> Option<Result<PreparedTypedResultCsv, String>> {
    match displayed.viewer {
        crate::workbench::ResultViewer::Fft => Some(fft_spectrum_csv(state, displayed)),
        crate::workbench::ResultViewer::Hist => Some(histogram_bins_csv(state).ok_or_else(|| {
            "Distribution export is unavailable because the active derived histogram is incomplete"
                .to_owned()
        })),
        crate::workbench::ResultViewer::Eye => Some(eye_measurements_csv(state).ok_or_else(|| {
            "Eye export is unavailable because the active derived eye diagram is incomplete"
                .to_owned()
        })),
        _ => None,
    }
}

fn fft_spectrum_csv(
    state: &AppState,
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
) -> Result<PreparedTypedResultCsv, String> {
    let displayed_analysis = displayed.primary_analysis(state).ok_or_else(|| {
        "FFT spectrum export is unavailable because the displayed analysis identity is no longer retained"
            .to_owned()
    })?;
    let expected_authority =
        crate::workbench::app_state::SpecializedViewerCacheProvenance::for_analysis(
            displayed.dataset_id,
            displayed_analysis,
        );
    if state.analysis.cache_authority.fft != Some(expected_authority) {
        return Err(
            "FFT spectrum export is unavailable because the derived spectrum does not belong to the displayed result"
                .to_owned(),
        );
    }
    let fft = &state.analysis.fft_state;
    if !fft.has_data() {
        return Err(fft.last_error.as_ref().map_or_else(
            || {
                "FFT spectrum export is unavailable because the derived analysis is incomplete"
                    .to_owned()
            },
            |error| format!("FFT spectrum export is unavailable — {error}"),
        ));
    }
    let data = fft.data.as_ref().ok_or_else(|| {
        "FFT spectrum export is unavailable because the transformed curve is missing".to_owned()
    })?;
    if data.points.is_empty() {
        return Err("FFT spectrum export is unavailable because it has no points".to_owned());
    }
    let source = fft
        .source_cache
        .as_ref()
        .map_or(data.name.as_str(), |cache| cache.name.as_str());
    let contents = rspice_formats::result_csv::encode_spectrum_csv(source, data);
    Ok(PreparedTypedResultCsv {
        default_name: "rspice-spectrum.csv",
        detail: format!("{} spectrum points", data.points.len()),
        contents,
    })
}

fn histogram_bins_csv(state: &AppState) -> Option<PreparedTypedResultCsv> {
    let histogram = crate::workbench::documents::result_document::active_histogram(state)?;
    let display = crate::workbench::documents::result_document::active_histogram_display(state)?;
    if histogram.bins.is_empty() {
        return None;
    }
    let contents = rspice_formats::result_csv::encode_histogram_csv(&histogram, &display);
    Some(PreparedTypedResultCsv {
        default_name: "rspice-distribution.csv",
        detail: format!("{} distribution ({})", histogram.name, display.mode.label()),
        contents,
    })
}

fn eye_measurements_csv(state: &AppState) -> Option<PreparedTypedResultCsv> {
    let eye = &state.analysis.eye_diagram_state;
    if eye.data.traces.is_empty() {
        return None;
    }
    let m = &eye.measurements;
    // An exported eye measurement is quoted against a bit period, and where
    // that period came from is part of the measurement: a rate the reader
    // stated and one recovered from six edges are different claims.
    let unit_interval_source = match eye.timebase_provenance() {
        Some(EyeTimebaseProvenance::Auto {
            edge_count,
            low_confidence,
            ..
        }) => {
            let confidence = if *low_confidence {
                " (low confidence)"
            } else {
                ""
            };
            format!("auto from {edge_count} edges{confidence}")
        }
        Some(EyeTimebaseProvenance::Explicit { .. }) => "explicit".to_owned(),
        Some(EyeTimebaseProvenance::AutoRejected(_)) | None => "unknown".to_owned(),
    };
    let contents = rspice_formats::result_csv::encode_eye_measurements_csv(
        eye.data.traces.len(),
        eye.data.ui_count,
        &unit_interval_source,
        m,
    );
    Some(PreparedTypedResultCsv {
        default_name: "rspice-eye.csv",
        detail: format!("{} eye acquisitions", eye.data.traces.len()),
        contents,
    })
}

fn export_typed_result_csv(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    prepared: &PreparedTypedResultCsv,
) {
    publish_result_text(
        state,
        io,
        "CSV",
        "csv",
        prepared.default_name,
        &prepared.contents,
        &prepared.detail,
    );
}

fn export_typed_result_tsv(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    prepared: &PreparedTypedResultCsv,
) {
    let contents = match csv_to_tsv(&prepared.contents) {
        Ok(contents) => contents,
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "TSV export failed before destination selection: {error}"
            )));
            return;
        }
    };
    let stem = prepared
        .default_name
        .strip_suffix(".csv")
        .unwrap_or(prepared.default_name);
    publish_result_text(
        state,
        io,
        "TSV",
        "tsv",
        &format!("{stem}.tsv"),
        &contents,
        &prepared.detail,
    );
}

/// Ask for a destination, write the text there, and tell the reader what they
/// now have.
///
/// CSV and TSV differ in the delimiter, the extension and the label they are
/// reported under. Nothing else about publishing a table of text differs, so
/// the dialog, the observed compare-and-exchange write and both completion
/// messages are stated once here.
fn publish_result_text(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    label: &str,
    extension: &str,
    default_name: &str,
    contents: &str,
    detail: &str,
) {
    let title = format!("Export Result {label}");
    let filter_name = format!("{label} Files");
    let (path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: &title,
        default_name,
        filter_name: &filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, extension);
            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, contents))
                .map_err(|error| (format!("{label} export failed: {error}"), error));
            (path, export)
        }
        Ok(None) => return,
        Err(error) => (
            std::path::PathBuf::new(),
            Err((format!("{label} destination failed: {error}"), error)),
        ),
    };
    match export {
        Ok(()) => {
            note_result_export_success(state, label);
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                crate::workbench::workflows::export_workflow::export_completion_message(
                    label,
                    &path,
                    Some(detail.to_owned()),
                    io,
                ),
            ));
        }
        Err((note, error)) => {
            note_result_export_failure(state, note);
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "{label} export failed: {error}"
            )));
        }
    }
}

fn export_native_result_bundle(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
    kind: rspice_formats::native_bundle::NativeBundleKind,
) {
    use rspice_formats::native_bundle::encode_native_bundle;
    use rspice_formats::native_bundle::result::project_native_bundle;

    let analysis = match displayed.primary_analysis(state) {
        Some(analysis) => analysis,
        None => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                NO_ACTIVE_ANALYSIS_MESSAGE.to_owned(),
            ));
            return;
        }
    };
    let native_analysis = match analysis.analysis_type {
        crate::state::AnalysisType::Transient => rspice_formats::WaveformDomain::Transient,
        crate::state::AnalysisType::Ac => rspice_formats::WaveformDomain::Ac,
        crate::state::AnalysisType::DcSweep => rspice_formats::WaveformDomain::DcSweep,
        other => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "{} export currently preserves transient, AC, and DC-sweep waveform domains; the active {} analysis cannot be represented by the version-1 native waveform schema.",
                kind.display_name(),
                other.short_label(),
            )));
            return;
        }
    };
    let waveforms = exported_waveforms(state, displayed.dataset_id, analysis);
    if waveforms.is_empty() {
        let message = if analysis.waveforms.is_empty() {
            NO_SAMPLES_MESSAGE
        } else {
            ALL_TRACES_HIDDEN_MESSAGE
        };
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            message.to_owned(),
        ));
        return;
    }
    let native_dataset = match project_native_bundle(analysis, &waveforms, native_analysis) {
        Ok(dataset) => dataset,
        Err(WaveformProjectionError::NoSamples) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                NO_SAMPLES_MESSAGE.to_owned(),
            ));
            return;
        }
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(error.to_string()));
            return;
        }
    };
    let bytes = match encode_native_bundle(
        kind,
        &native_dataset,
        crate::workbench::workflows::result_import_workflow::MAX_RESULT_DATASET_BYTES,
    ) {
        Ok(bytes) => bytes,
        Err(error) => {
            note_result_export_failure(
                state,
                format!("{} encoding failed: {error}", kind.display_name()),
            );
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "{} export failed before destination selection: {error}",
                kind.display_name()
            )));
            return;
        }
    };
    let exported_signal_count = native_dataset.signals.len();
    let exported_point_count = native_dataset.coordinate.len();
    drop(native_dataset);
    drop(waveforms);
    let extension = kind.extension();
    let default_name = format!("waveforms.{extension}");
    let filter_extensions = [extension];
    let (published_path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: match kind {
            rspice_formats::native_bundle::NativeBundleKind::Result => {
                "Export RSpice Result Bundle"
            }
            rspice_formats::native_bundle::NativeBundleKind::Dataset => {
                "Export RSpice Dataset Bundle"
            }
        },
        default_name: &default_name,
        filter_name: kind.display_name(),
        filter_extensions: &filter_extensions,
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, extension);
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_bytes_file_observed(&destination, &bytes, kind.media_type())
            });
            (path, export)
        }
        Ok(None) => return,
        Err(error) => (std::path::PathBuf::from(default_name), Err(error)),
    };
    match export {
        Ok(()) => {
            note_result_export_success(state, kind.display_name());
            let detail = format!(
                "{} signals, {} points, SHA-256-bound dataset",
                exported_signal_count, exported_point_count
            );
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                crate::workbench::workflows::export_workflow::export_completion_message(
                    kind.display_name(),
                    &published_path,
                    Some(detail),
                    io,
                ),
            ));
        }
        Err(error) => {
            note_result_export_failure(
                state,
                format!("{} export failed: {error}", kind.display_name()),
            );
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "{} export failed: {error}",
                kind.display_name()
            )));
        }
    }
}

fn export_csv(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    dataset: &crate::io::WaveformDataset,
) {
    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Waveform CSV",
        default_name: "waveforms.csv",
        filter_name: "CSV Files",
        filter_extensions: &["csv"],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "csv");

            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_waveform_csv_observed(dataset, &destination));
            match export {
                Ok(()) => {
                    note_result_export_success(state, "CSV");
                    let detail = format!(
                        "{} signals, {} points",
                        dataset.signal_count(),
                        dataset.point_count()
                    );
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "CSV",
                            &path,
                            Some(detail),
                            io,
                        ),
                    ));
                }
                Err(e) => {
                    note_result_export_failure(state, format!("CSV export failed: {e}"));
                    state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                        "CSV export failed: {}",
                        e
                    )));
                }
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            note_result_export_failure(state, format!("CSV destination failed: {e}"));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "CSV export failed: {}",
                e
            )));
        }
    }
}

fn export_tsv(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    dataset: &crate::io::WaveformDataset,
) {
    let contents =
        match crate::io::WaveformWriter::new(crate::io::WaveformFormat::Tsv).write_text(dataset) {
            Ok(contents) => contents,
            Err(error) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                    "TSV export failed before destination selection: {error}"
                )));
                return;
            }
        };
    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Waveform TSV",
        default_name: "waveforms.tsv",
        filter_name: "TSV Files",
        filter_extensions: &["tsv"],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "tsv");
            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &contents));
            match export {
                Ok(()) => {
                    note_result_export_success(state, "TSV");
                    let detail = format!(
                        "{} signals, {} points",
                        dataset.signal_count(),
                        dataset.point_count()
                    );
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "TSV",
                            &path,
                            Some(detail),
                            io,
                        ),
                    ));
                }
                Err(error) => {
                    note_result_export_failure(state, format!("TSV export failed: {error}"));
                    state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                        "TSV export failed: {error}"
                    )));
                }
            }
        }
        Ok(None) => {}
        Err(error) => {
            note_result_export_failure(state, format!("TSV destination failed: {error}"));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "TSV export failed: {error}"
            )));
        }
    }
}

fn export_touchstone(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    dataset: &crate::io::WaveformDataset,
) {
    // Validate and serialize before opening a save picker. An incompatible
    // result never asks the user for a destination it cannot publish.
    let mut dataset = dataset.clone();
    dataset
        .metadata
        .insert("touchstone_version".to_owned(), "2".to_owned());
    let contents = match crate::io::WaveformWriter::new(crate::io::WaveformFormat::Touchstone)
        .write_text(&dataset)
    {
        Ok(contents) => contents,
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Touchstone export is not compatible with the active result: {error}"
            )));
            return;
        }
    };
    let port_count = crate::io::WaveformWriter::touchstone_port_count(&dataset)
        .expect("successful Touchstone validation always identifies a port matrix");
    let default_name = "waveforms.snp";

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Touchstone",
        default_name,
        filter_name: "Touchstone v2 Files",
        filter_extensions: &["snp", "ts"],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "snp");
            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &contents));
            match export {
                Ok(()) => {
                    note_result_export_success(state, "Touchstone");
                    let detail = format!(
                        "{}-port matrix, {} signals, {} points",
                        port_count,
                        dataset.signal_count(),
                        dataset.point_count()
                    );
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "Touchstone",
                            &path,
                            Some(detail),
                            io,
                        ),
                    ));
                }
                Err(error) => {
                    note_result_export_failure(state, format!("Touchstone export failed: {error}"));
                    state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                        "Touchstone export failed: {error}"
                    )));
                }
            }
        }
        Ok(None) => {}
        Err(error) => {
            note_result_export_failure(state, format!("Touchstone destination failed: {error}"));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Touchstone export failed: {error}"
            )));
        }
    }
}

/// The traces an export of this analysis carries.
///
/// One population for every export route. The dataset's own `visible` flag is
/// the default, and the reader's per-trace override for this analysis wins
/// over it — that override is what the legend chips and the navigator's
/// check-marks write, so an export that ignores it does not export what is on
/// the sheet. It was ignored twice over: the long-form route read the raw
/// dataset flag, and the single-analysis route read neither, so hiding a
/// trace changed the file only when the viewer happened to be showing two
/// analyses or more.
fn exported_waveforms<'a>(
    state: &AppState,
    dataset_id: crate::product::DatasetId,
    analysis: &'a crate::state::AnalysisResult,
) -> Vec<&'a crate::state::WaveformData> {
    use crate::workbench::documents::result_document::AnalysisPresentationKey;
    use rspice_results_ui::selection::SourceWaveformPresentationKey;

    let analysis_key = AnalysisPresentationKey::new(dataset_id, analysis);
    analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            state.ui.results.session.waveform_visibility(
                &SourceWaveformPresentationKey::new(analysis_key, &waveform.name),
                waveform.visible,
            )
        })
        .collect()
}

/// Why a dataset offers the displayed view no analysis at all.
///
/// Viewer compatibility excludes an unsuccessful solve, so a run that failed
/// resolves to an empty view — and every route above this one then declines
/// in turn until the last of them reports "no waveform samples", which is
/// true and says nothing. A reader who pressed Export on the operating-point
/// sheet of a run that did not converge is told that, and told the engine's
/// own reason with it.
fn failed_run_refusal(run: &crate::state::SimulationRun) -> Option<String> {
    if run.analyses.is_empty() || run.analyses.iter().any(|analysis| analysis.success) {
        return None;
    }
    let detail = run
        .analyses
        .iter()
        .find_map(|analysis| analysis.error_message.as_deref());
    Some(format!(
        "The displayed result cannot be exported: no analysis in '{}' completed successfully.{}",
        run.label,
        detail.map_or_else(String::new, |error| format!(" {error}"))
    ))
}

fn prepare_waveform_dataset(
    state: &AppState,
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
    touchstone: bool,
) -> Result<crate::io::WaveformDataset, String> {
    if displayed.analysis_indices.is_empty() {
        return Err(displayed
            .run(state)
            .and_then(failed_run_refusal)
            .unwrap_or_else(|| NO_SAMPLES_MESSAGE.to_owned()));
    }
    let analysis = displayed
        .primary_analysis(state)
        .ok_or_else(|| NO_ACTIVE_ANALYSIS_MESSAGE.to_string())?;
    let waveforms = exported_waveforms(state, displayed.dataset_id, analysis);
    if waveforms.is_empty() && !analysis.waveforms.is_empty() {
        return Err(ALL_TRACES_HIDDEN_MESSAGE.to_owned());
    }
    prepare_single_analysis_dataset(analysis, &waveforms, touchstone)
        .map_err(|error| error.to_string())
}

/// Long-form export for a viewer that is displaying more than one analysis.
/// Each row carries its immutable dataset and run-local analysis identity, so
/// different coordinate grids and trace counts remain exact without padding,
/// truncation, or an invented shared axis.
fn prepare_displayed_analysis_stack_csv(
    state: &AppState,
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
) -> Result<PreparedTypedResultCsv, String> {
    let analyses = displayed.analyses(state).collect::<Vec<_>>();
    if analyses.is_empty() {
        return Err(NO_SAMPLES_MESSAGE.to_owned());
    }
    let mut csv = rspice_formats::result_csv::AnalysisStackCsv::new();
    let mut traces = 0_usize;
    for analysis in analyses {
        for waveform in exported_waveforms(state, displayed.dataset_id, analysis) {
            csv.append_component(
                displayed.dataset_id,
                analysis,
                &waveform.name,
                "display",
                waveform.x.as_ref(),
                waveform.y.as_ref(),
            )
            .map_err(|error| error.to_string())?;
            traces += 1;
            if let Some(complex) = &waveform.complex {
                csv.append_component(
                    displayed.dataset_id,
                    analysis,
                    &complex.source_name,
                    "real",
                    waveform.x.as_ref(),
                    complex.real.as_ref(),
                )
                .map_err(|error| error.to_string())?;
                csv.append_component(
                    displayed.dataset_id,
                    analysis,
                    &complex.source_name,
                    "imaginary",
                    waveform.x.as_ref(),
                    complex.imag.as_ref(),
                )
                .map_err(|error| error.to_string())?;
            }
        }
    }
    let rows = csv.row_count();
    if rows == 0 {
        return Err(NO_SAMPLES_MESSAGE.to_owned());
    }
    Ok(PreparedTypedResultCsv {
        default_name: "rspice-displayed-results.csv",
        contents: csv.into_string(),
        detail: format!("{traces} visible traces, {rows} exported samples"),
    })
}

#[cfg(test)]
mod tests;
