//! Waveform export actions.

use crate::analysis::eye_diagram::EyeTimebaseProvenance;
use crate::workbench::EngineeringExportFormat;
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

const NO_ACTIVE_ANALYSIS_MESSAGE: &str = "No active result analysis is selected for export.";
const NO_SAMPLES_MESSAGE: &str = "No waveform samples available to export.";
/// An export that silently wrote an empty file would be indistinguishable
/// from one that wrote the dataset, so the hidden traces are named as the
/// reason rather than reported as an absence of samples.
const ALL_TRACES_HIDDEN_MESSAGE: &str = "Every trace in the displayed analysis is hidden, so there is nothing to export. \
     Show at least one trace first.";

fn note_result_export_failure(state: &mut AppState, detail: impl Into<String>) {
    let data_version = state.simulation.data_version;
    state.ui.results.record_runtime_condition(
        crate::workbench::documents::result_document::operational_state::ResultRuntimeConditionKind::Failed,
        detail,
        data_version,
    );
}

fn note_result_export_success(state: &mut AppState, format: &str) {
    let data_version = state.simulation.data_version;
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
    ArrowIpc,
    Parquet,
    NumpyNpy,
    NumpyNpz,
    MatlabV5,
    MatlabV73,
    JsonLines,
    Vcd,
    Fst,
}

impl ResultExportFormat {
    const ALL: [Self; 15] = [
        Self::RSpiceResultBundle,
        Self::RSpiceDatasetBundle,
        Self::CsvRfc4180,
        Self::Tsv,
        Self::TouchstoneV2,
        Self::Hdf5,
        Self::ArrowIpc,
        Self::Parquet,
        Self::NumpyNpy,
        Self::NumpyNpz,
        Self::MatlabV5,
        Self::MatlabV73,
        Self::JsonLines,
        Self::Vcd,
        Self::Fst,
    ];

    const fn canonical_id(self) -> &'static str {
        match self {
            Self::RSpiceResultBundle => "rspice-result-bundle",
            Self::RSpiceDatasetBundle => "rspice-dataset-bundle",
            Self::CsvRfc4180 => "csv-rfc4180",
            Self::Tsv => "tsv",
            Self::TouchstoneV2 => "touchstone-v2",
            Self::Hdf5 => "hdf5",
            Self::ArrowIpc => "arrow-ipc",
            Self::Parquet => "parquet",
            Self::NumpyNpy => "numpy-npy",
            Self::NumpyNpz => "numpy-npz",
            Self::MatlabV5 => "matlab-v5",
            Self::MatlabV73 => "matlab-v7.3",
            Self::JsonLines => "json-lines",
            Self::Vcd => "vcd",
            Self::Fst => "fst",
        }
    }

    const fn encoder_available(self) -> bool {
        matches!(self, Self::CsvRfc4180 | Self::Tsv | Self::TouchstoneV2)
    }
}

fn result_export_format_availability(format: ResultExportFormat) -> Result<(), String> {
    if format.encoder_available() {
        Ok(())
    } else {
        Err(format!(
            "Format '{}' is declared by the result-data contract, but this build does not include a verified lossless encoder.",
            format.canonical_id()
        ))
    }
}

fn result_export_format_availability_by_id(canonical_id: &str) -> Result<(), String> {
    let format = ResultExportFormat::ALL
        .into_iter()
        .find(|format| format.canonical_id() == canonical_id)
        .ok_or_else(|| format!("Unknown result export format '{canonical_id}'."))?;
    result_export_format_availability(format)
}

/// Export the exact dataset-bound Data Browser selection, never whatever
/// payload the currently visible sheet happens to choose as its default.
pub(crate) fn action_export_result_selection_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    keys: &[crate::workbench::documents::result_document::ResultBrowserSelectionKey],
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
            &state.simulation.runs,
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
            &state.simulation.runs,
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
        EngineeringExportFormat::Hdf5EngineeringDataset => ResultExportFormat::Hdf5,
    };
    if let Err(error) = result_export_format_availability_by_id(contract_format.canonical_id()) {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
        return;
    }
    // What the reader is looking at comes first. Three sheets derive their
    // curve in the viewer rather than reading a retained vector, so routing
    // on the payload alone handed back the transient samples the spectrum was
    // computed from and called it the result.
    if let Some(derived) = prepare_active_derived_view_csv(state, &displayed) {
        match export_format {
            EngineeringExportFormat::Csv => export_typed_result_csv(state, io, &derived),
            EngineeringExportFormat::Tsv => export_typed_result_tsv(state, io, &derived),
            EngineeringExportFormat::TouchstoneWhereCompatible => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Touchstone export is not compatible with a derived viewer; select CSV export."
                        .to_owned(),
                ))
            }
            EngineeringExportFormat::Hdf5EngineeringDataset => {
                unreachable!("unsupported export preferences resolve to CSV")
            }
        }
        return;
    }

    let sheet = match prepare_active_sheet_csv(state, &displayed) {
        // The sheet owns this viewer's export and cannot produce it. Falling
        // through to the payload router below is how an operating-point
        // export came back holding a transient's event history.
        Some(Err(reason)) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(reason));
            return;
        }
        Some(Ok(sheet)) => Some(sheet),
        None => None,
    };
    if let Some(sheet) = sheet {
        match export_format {
            EngineeringExportFormat::Csv => export_typed_result_csv(state, io, &sheet),
            EngineeringExportFormat::Tsv => export_typed_result_tsv(state, io, &sheet),
            EngineeringExportFormat::TouchstoneWhereCompatible => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Touchstone export is not compatible with this Results sheet; select CSV export."
                        .to_owned(),
                ));
            }
            EngineeringExportFormat::Hdf5EngineeringDataset => {
                unreachable!("unsupported export preferences resolve to CSV")
            }
        }
        return;
    }

    if let Some(typed) = displayed
        .primary_analysis(state)
        .and_then(prepare_typed_result_csv)
    {
        match export_format {
            EngineeringExportFormat::Csv => export_typed_result_csv(state, io, &typed),
            EngineeringExportFormat::Tsv => export_typed_result_tsv(state, io, &typed),
            EngineeringExportFormat::TouchstoneWhereCompatible => state.push_user_message(
                crate::diagnostics::ConsoleMessage::warning(
                    "Touchstone export is not compatible with the active typed result; select CSV export."
                        .to_owned(),
                ),
            ),
            EngineeringExportFormat::Hdf5EngineeringDataset => {
                unreachable!("unsupported export preferences resolve to CSV")
            }
        }
        return;
    }

    if displayed.analysis_indices.len() > 1 {
        let prepared = match prepare_displayed_analysis_stack_csv(state, &displayed) {
            Ok(prepared) => prepared,
            Err(message) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
                return;
            }
        };
        match export_format {
            EngineeringExportFormat::Csv => export_typed_result_csv(state, io, &prepared),
            EngineeringExportFormat::Tsv => export_typed_result_tsv(state, io, &prepared),
            EngineeringExportFormat::TouchstoneWhereCompatible => state.push_user_message(
                crate::diagnostics::ConsoleMessage::warning(
                    "Touchstone export requires one selected analysis; maximize one displayed strip or select CSV export."
                        .to_owned(),
                ),
            ),
            EngineeringExportFormat::Hdf5EngineeringDataset => {
                unreachable!("unsupported export preferences resolve to CSV")
            }
        }
        return;
    }

    let prepared = match prepare_waveform_dataset(state, &displayed) {
        Ok(prepared) => prepared,
        Err(message) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
            return;
        }
    };

    for warning in &prepared.warnings {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(warning.clone()));
    }

    match export_format {
        EngineeringExportFormat::Csv => export_csv(state, io, &prepared.dataset),
        EngineeringExportFormat::Tsv => export_tsv(state, io, &prepared.dataset),
        EngineeringExportFormat::TouchstoneWhereCompatible => {
            export_touchstone(state, io, &prepared.dataset)
        }
        EngineeringExportFormat::Hdf5EngineeringDataset => {
            unreachable!("unsupported export preferences resolve to CSV")
        }
    }
}

struct PreparedTypedResultCsv {
    default_name: &'static str,
    contents: String,
    detail: String,
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
            &state.workspace.specs,
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
) -> Option<PreparedTypedResultCsv> {
    match displayed.viewer {
        crate::workbench::ResultViewer::Fft => fft_spectrum_csv(state),
        crate::workbench::ResultViewer::Hist => histogram_bins_csv(state),
        crate::workbench::ResultViewer::Eye => eye_measurements_csv(state),
        _ => None,
    }
}

fn fft_spectrum_csv(state: &AppState) -> Option<PreparedTypedResultCsv> {
    let fft = &state.analysis.fft_state;
    let data = fft.data.as_ref()?;
    if data.points.is_empty() {
        return None;
    }
    let source = fft
        .source_cache
        .as_ref()
        .map_or(data.name.as_str(), |cache| cache.name.as_str());
    let mut contents = String::from("field,value,unit\n");
    for (field, value, unit) in [
        ("source", csv_text(source), ""),
        ("window", csv_text(data.window.display_name()), ""),
        ("fft_size", data.fft_size.to_string(), ""),
        ("sample_rate", format!("{:.17e}", data.sample_rate), "Hz"),
        (
            "resolution_bandwidth",
            format!("{:.17e}", data.resolution_bandwidth()),
            "Hz",
        ),
        (
            "normalization",
            csv_text(data.normalization.display_name()),
            "",
        ),
    ] {
        contents.push_str(&format!("{field},{value},{unit}\n"));
    }
    contents.push_str("\nfrequency_hz,magnitude,magnitude_db,phase_rad\n");
    for point in &data.points {
        contents.push_str(&format!(
            "{:.17e},{:.17e},{:.17e},{:.17e}\n",
            point.frequency,
            point.magnitude,
            point.magnitude_db(),
            point.phase
        ));
    }
    Some(PreparedTypedResultCsv {
        default_name: "rspice-spectrum.csv",
        detail: format!("{} spectrum points", data.points.len()),
        contents,
    })
}

fn histogram_bins_csv(state: &AppState) -> Option<PreparedTypedResultCsv> {
    let histogram = state
        .analysis
        .histogram_state
        .histograms
        .get(state.analysis.histogram_state.selected)?;
    if histogram.bins.is_empty() {
        return None;
    }
    let mut contents = String::from("field,value,unit\n");
    for (field, value) in [
        ("measurement", csv_text(&histogram.name)),
        ("total_count", histogram.total_count.to_string()),
        ("total_weight", format!("{:.17e}", histogram.total_weight)),
        ("underflow", histogram.underflow.to_string()),
        ("overflow", histogram.overflow.to_string()),
        ("data_min", format!("{:.17e}", histogram.data_min)),
        ("data_max", format!("{:.17e}", histogram.data_max)),
    ] {
        contents.push_str(&format!("{field},{value},\n"));
    }
    contents.push_str("\nbin_lower,bin_upper,count,weight\n");
    for bin in &histogram.bins {
        contents.push_str(&format!(
            "{:.17e},{:.17e},{},{:.17e}\n",
            bin.lower, bin.upper, bin.count, bin.weight
        ));
    }
    Some(PreparedTypedResultCsv {
        default_name: "rspice-distribution.csv",
        detail: format!("{} distribution bins", histogram.bins.len()),
        contents,
    })
}

/// A measurement that could not be made exports as an empty cell, not as a
/// zero: a spreadsheet that averages a column of rise times must not be handed
/// a `0 s` that no acquisition contains.
///
/// An *unbounded* measurement keeps its value and exports as `inf`. The Q and
/// SNR of a noiseless eye are unbounded, which is an answer — the sheet prints
/// `∞` for them — and blanking it here would tell a reader the eye had no Q at
/// all, which is the one thing an empty cell in this column means.
fn csv_measurement(value: Option<f64>) -> String {
    value.map_or_else(String::new, |value| format!("{value:.17e}"))
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
    let mut contents = String::from("field,value,unit\n");
    for (field, value, unit) in [
        ("acquisitions", eye.data.traces.len().to_string(), ""),
        ("unit_intervals", eye.data.ui_count.to_string(), ""),
        ("data_rate", format!("{:.17e}", m.data_rate), "b/s"),
        ("unit_interval", format!("{:.17e}", m.unit_interval), "s"),
        ("unit_interval_source", csv_text(&unit_interval_source), ""),
        ("eye_height", format!("{:.17e}", m.eye_height), "V"),
        ("eye_width", format!("{:.17e}", m.eye_width), "UI"),
        ("eye_area", format!("{:.17e}", m.eye_area), ""),
        (
            "vertical_margin",
            format!("{:.17e}", m.vertical_margin),
            "V",
        ),
        (
            "horizontal_margin",
            format!("{:.17e}", m.horizontal_margin),
            "UI",
        ),
        ("rise_time", csv_measurement(m.rise_time), "s"),
        ("fall_time", csv_measurement(m.fall_time), "s"),
        ("jitter_pp", format!("{:.17e}", m.jitter_pp), "s"),
        ("jitter_rms", format!("{:.17e}", m.jitter_rms), "s"),
        ("jitter_dj", format!("{:.17e}", m.jitter_dj), "s"),
        ("crossing_level", csv_measurement(m.crossing_level), "V"),
        (
            "crossing_percentage",
            csv_measurement(m.crossing_percentage),
            "",
        ),
        ("snr", csv_measurement(m.snr_db), "dB"),
        ("q_factor", csv_measurement(m.q_factor), ""),
        ("estimated_ber", csv_measurement(m.estimated_ber), ""),
    ] {
        contents.push_str(&format!("{field},{value},{unit}\n"));
    }
    Some(PreparedTypedResultCsv {
        default_name: "rspice-eye.csv",
        detail: format!("{} eye acquisitions", eye.data.traces.len()),
        contents,
    })
}

fn prepare_typed_result_csv(
    analysis: &crate::state::AnalysisResult,
) -> Option<PreparedTypedResultCsv> {
    let payload = analysis.result_payload.as_ref()?;
    if !analysis.success || analysis.validate_retained_evidence().is_err() {
        return None;
    }

    use crate::state::{AnalysisResultPayload, SensitivityResultMode};
    match payload {
        AnalysisResultPayload::OperatingPoint {
            temperature_mode,
            temperature_celsius,
            initial_guess,
            node_initialization,
            homotopy,
            annotation,
            device_detail,
            save_device_op,
            accuracy,
            selected_devices,
            violation_devices,
            violation_source_content_digest,
            validated_startup_directives,
            mna_node_names,
            mna_branch_names,
            mna_solution,
            effective_source_content_digest,
            run_point_index,
            run_point_count,
            run_point_process,
            run_point_supply_voltage,
            run_point_nominal_supply_voltage,
        } => {
            let mut contents = String::from("field,value,unit\n");
            for (field, value, unit) in [
                (
                    "temperature_mode",
                    serialized_enum_name(temperature_mode),
                    "",
                ),
                (
                    "temperature_celsius",
                    format!("{temperature_celsius:.17e}"),
                    "degC",
                ),
                ("initial_guess", serialized_enum_name(initial_guess), ""),
                (
                    "node_initialization",
                    serialized_enum_name(node_initialization),
                    "",
                ),
                ("homotopy", serialized_enum_name(homotopy), ""),
                ("annotation", serialized_enum_name(annotation), ""),
                ("device_detail", serialized_enum_name(device_detail), ""),
                ("save_device_op", serialized_enum_name(save_device_op), ""),
                ("accuracy", serialized_enum_name(accuracy), ""),
                (
                    "validated_startup_directives",
                    validated_startup_directives.to_string(),
                    "count",
                ),
                ("selected_devices", selected_devices.join(";"), ""),
                ("violation_devices", violation_devices.join(";"), ""),
                (
                    "violation_source_content_digest",
                    violation_source_content_digest
                        .map_or_else(String::new, |digest| digest.to_string()),
                    "sha256",
                ),
                ("mna_nodes", mna_node_names.len().to_string(), "count"),
                ("mna_branches", mna_branch_names.len().to_string(), "count"),
                ("mna_values", mna_solution.len().to_string(), "count"),
                (
                    "effective_source_content_digest",
                    effective_source_content_digest
                        .map_or_else(String::new, |digest| digest.to_string()),
                    "sha256",
                ),
                ("run_point_index", run_point_index.to_string(), "zero_based"),
                ("run_point_count", run_point_count.to_string(), "count"),
                (
                    "run_point_process",
                    serialized_enum_name(run_point_process),
                    "",
                ),
                (
                    "run_point_supply_voltage",
                    run_point_supply_voltage
                        .map(|voltage| format!("{voltage:.17e}"))
                        .unwrap_or_default(),
                    "V",
                ),
                (
                    "run_point_nominal_supply_voltage",
                    run_point_nominal_supply_voltage
                        .map(|voltage| format!("{voltage:.17e}"))
                        .unwrap_or_default(),
                    "V",
                ),
            ] {
                contents.push_str(&format!(
                    "{},{},{}\n",
                    csv_text(field),
                    csv_text(&value),
                    csv_text(unit)
                ));
            }
            Some(PreparedTypedResultCsv {
                default_name: "operating-point-contract.csv",
                contents,
                detail: "exact operating-point execution and retention contract".to_owned(),
            })
        }
        AnalysisResultPayload::PoleZero { poles, zeros, gain } => {
            let mut contents =
                String::from("record,index,real_rad_per_s,imaginary_rad_per_s,value\n");
            contents.push_str(&format!("gain,,,,{gain:.17e}\n"));
            for (kind, roots) in [("pole", poles), ("zero", zeros)] {
                for (index, root) in roots.iter().enumerate() {
                    contents.push_str(&format!(
                        "{kind},{index},{:.17e},{:.17e},\n",
                        root.real, root.imaginary
                    ));
                }
            }
            Some(PreparedTypedResultCsv {
                default_name: "pole-zero.csv",
                contents,
                detail: format!("{} poles, {} zeros, exact gain", poles.len(), zeros.len()),
            })
        }
        AnalysisResultPayload::Sensitivity {
            output,
            result_mode,
            rows,
        } => {
            let (mode, frequency) = match result_mode {
                SensitivityResultMode::Dc => ("dc", String::new()),
                SensitivityResultMode::Ac { frequency_hz } => {
                    ("ac", format!("{frequency_hz:.17e}"))
                }
            };
            let escaped_output = csv_text(output);
            let mut contents = String::from(
                "parameter,raw_sensitivity,normalized_sensitivity,output,mode,frequency_hz\n",
            );
            for row in rows {
                contents.push_str(&format!(
                    "{},{:.17e},{:.17e},{escaped_output},{mode},{frequency}\n",
                    csv_text(&row.parameter),
                    row.raw,
                    row.normalized,
                ));
            }
            Some(PreparedTypedResultCsv {
                default_name: "sensitivity.csv",
                contents,
                detail: format!("{} exact sensitivity rows", rows.len()),
            })
        }
        AnalysisResultPayload::ScalarMeasurements { values } => {
            let mut contents = String::from("name,value\n");
            for (name, value) in values {
                contents.push_str(&format!("{},{value:.17e}\n", csv_text(name)));
            }
            Some(PreparedTypedResultCsv {
                default_name: "scalar-results.csv",
                contents,
                detail: format!("{} exact scalar values", values.len()),
            })
        }
        AnalysisResultPayload::TransferFunction {
            input_source,
            output_expression,
            input_quantity,
            output_quantity,
            input_unit,
            output_unit,
            normalization,
            accuracy,
            gain,
            input_resistance,
            output_resistance,
            nominal_input,
            nominal_output,
        } => {
            let normalization_label = match normalization {
                crate::state::TransferFunctionNormalizationEvidence::None => "disabled",
                crate::state::TransferFunctionNormalizationEvidence::RelativeToNominal => {
                    "relative_to_nominal"
                }
                crate::state::TransferFunctionNormalizationEvidence::PerSourceUnit => {
                    "per_source_unit"
                }
            };
            let accuracy_label = match accuracy {
                crate::state::TransferFunctionAccuracyEvidence::Fast => "fast",
                crate::state::TransferFunctionAccuracyEvidence::Balanced => "balanced",
                crate::state::TransferFunctionAccuracyEvidence::Accurate => "accurate",
                crate::state::TransferFunctionAccuracyEvidence::Robust => "robust",
            };
            let gain_unit = if matches!(
                normalization,
                crate::state::TransferFunctionNormalizationEvidence::RelativeToNominal
            ) {
                "1"
            } else {
                match (input_quantity, output_quantity) {
                    (
                        crate::state::TransferFunctionQuantityEvidence::Voltage,
                        crate::state::TransferFunctionQuantityEvidence::Voltage,
                    ) => "V/V",
                    (
                        crate::state::TransferFunctionQuantityEvidence::Voltage,
                        crate::state::TransferFunctionQuantityEvidence::Current,
                    ) => "A/V",
                    (
                        crate::state::TransferFunctionQuantityEvidence::Current,
                        crate::state::TransferFunctionQuantityEvidence::Voltage,
                    ) => "V/A",
                    (
                        crate::state::TransferFunctionQuantityEvidence::Current,
                        crate::state::TransferFunctionQuantityEvidence::Current,
                    ) => "A/A",
                }
            };
            let mut contents = String::from(
                "quantity,value,unit,input_source,output_expression,normalization,accuracy,solve_point\n",
            );
            let mut rows = 0usize;
            let mut push_scalar = |quantity: &str,
                                   value: &crate::state::TransferFunctionScalarEvidence,
                                   unit: &str| {
                let value = match value {
                    crate::state::TransferFunctionScalarEvidence::Finite(value) => {
                        format!("{value:.17e}")
                    }
                    crate::state::TransferFunctionScalarEvidence::PositiveInfinity => {
                        "+infinity".to_owned()
                    }
                    crate::state::TransferFunctionScalarEvidence::NegativeInfinity => {
                        "-infinity".to_owned()
                    }
                };
                contents.push_str(&format!(
                    "{quantity},{value},{},{},{},{normalization_label},{accuracy_label},dc_operating_point\n",
                    csv_text(unit),
                    csv_text(input_source),
                    csv_text(output_expression),
                ));
                rows += 1;
            };
            if let Some(gain) = gain {
                push_scalar("transfer_gain", gain, gain_unit);
            }
            if let Some(input_resistance) = input_resistance {
                push_scalar("input_resistance", input_resistance, "ohm");
            }
            if let Some(output_resistance) = output_resistance {
                push_scalar("output_resistance", output_resistance, "ohm");
            }
            if let Some(value) = nominal_input {
                contents.push_str(&format!(
                    "nominal_input,{value:.17e},{},{},{},{normalization_label},{accuracy_label},dc_operating_point\n",
                    csv_text(input_unit), csv_text(input_source), csv_text(output_expression)
                ));
                rows += 1;
            }
            if let Some(value) = nominal_output {
                contents.push_str(&format!(
                    "nominal_output,{value:.17e},{},{},{},{normalization_label},{accuracy_label},dc_operating_point\n",
                    csv_text(output_unit), csv_text(input_source), csv_text(output_expression)
                ));
                rows += 1;
            }
            Some(PreparedTypedResultCsv {
                default_name: "transfer-function.csv",
                contents,
                detail: format!("{rows} exact transfer-function values"),
            })
        }
        AnalysisResultPayload::Reliability { devices } => {
            let mut contents = String::from(
                "device,lifetime_years,average_gate_stress_v,average_drain_stress_v,average_temperature_k,duration_s,threshold_voltage_shift_v,mobility_shift,drain_source_resistance_shift\n",
            );
            let row_count = devices
                .iter()
                .map(|device| device.checkpoints.len())
                .sum::<usize>();
            for device in devices {
                for checkpoint in &device.checkpoints {
                    let shift = &checkpoint.shift;
                    contents.push_str(&format!(
                        "{},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e},{:.17e}\n",
                        csv_text(&device.device_id),
                        checkpoint.years,
                        device.stress.average_gate_stress_v,
                        device.stress.average_drain_stress_v,
                        device.stress.average_temperature_k,
                        device.stress.duration_s,
                        shift.threshold_voltage_shift_v,
                        shift.mobility_shift,
                        shift.drain_source_resistance_shift,
                    ));
                }
            }
            Some(PreparedTypedResultCsv {
                default_name: "reliability-evidence.csv",
                contents,
                detail: format!(
                    "{} devices, {} exact lifetime-shift rows",
                    devices.len(),
                    row_count
                ),
            })
        }
        AnalysisResultPayload::Soa {
            evaluations,
            violations,
        } => {
            let mut contents = String::from(
                "record,device,parameter,limit_value,actual_value,time_s,sample_count,unit,description,verdict\n",
            );
            for evaluation in evaluations {
                contents.push_str(&format!(
                    "evaluation,{},{},{:.17e},{:.17e},{:.17e},{},{},{},{}\n",
                    csv_text(&evaluation.device_id),
                    soa_parameter_csv(evaluation.parameter),
                    evaluation.limit_value,
                    evaluation.worst_actual_value,
                    evaluation.worst_time_s,
                    evaluation.sample_count,
                    csv_text(&evaluation.unit),
                    csv_text(&evaluation.description),
                    soa_verdict_csv(evaluation.verdict),
                ));
            }
            for violation in violations {
                contents.push_str(&format!(
                    "event,{},{},{:.17e},{:.17e},{:.17e},,,,{}\n",
                    csv_text(&violation.device_id),
                    soa_parameter_csv(violation.parameter),
                    violation.limit_value,
                    violation.actual_value,
                    violation.time_s,
                    soa_violation_severity_csv(violation.severity),
                ));
            }
            Some(PreparedTypedResultCsv {
                default_name: "soa-evidence.csv",
                contents,
                detail: format!(
                    "{} evaluated rules, {} warning/violation events",
                    evaluations.len(),
                    violations.len()
                ),
            })
        }
        AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
        } => {
            let mut contents = String::from("node,domain,time_s,value_code,value\n");
            for trace in digital_traces {
                for point in &trace.points {
                    contents.push_str(&format!(
                        "{},digital,{:.17e},{},\n",
                        csv_text(&trace.node_name),
                        point.time_s,
                        point.value_code,
                    ));
                }
            }
            for trace in real_traces {
                for point in &trace.points {
                    contents.push_str(&format!(
                        "{},real,{:.17e},,{:.17e}\n",
                        csv_text(&trace.node_name),
                        point.time_s,
                        point.value,
                    ));
                }
            }
            let events: usize = digital_traces
                .iter()
                .map(|trace| trace.points.len())
                .chain(real_traces.iter().map(|trace| trace.points.len()))
                .sum();
            Some(PreparedTypedResultCsv {
                default_name: "event-history.csv",
                contents,
                detail: format!(
                    "{} event nodes, {events} committed events",
                    digital_traces.len() + real_traces.len()
                ),
            })
        }
    }
}

fn soa_parameter_csv(parameter: crate::state::SoaParameterEvidence) -> &'static str {
    use crate::state::SoaParameterEvidence;
    match parameter {
        SoaParameterEvidence::GateSourceVoltage => "vgs",
        SoaParameterEvidence::DrainSourceVoltage => "vds",
        SoaParameterEvidence::GateDrainVoltage => "vgd",
        SoaParameterEvidence::BaseEmitterVoltage => "vbe",
        SoaParameterEvidence::CollectorEmitterVoltage => "vce",
        SoaParameterEvidence::BaseCollectorVoltage => "vbc",
        SoaParameterEvidence::DrainCurrent => "id",
        SoaParameterEvidence::CollectorCurrent => "ic",
        SoaParameterEvidence::PowerDissipation => "pdiss",
        SoaParameterEvidence::Temperature => "temperature",
    }
}

fn soa_verdict_csv(verdict: crate::state::SoaRuleVerdictEvidence) -> &'static str {
    use crate::state::SoaRuleVerdictEvidence;
    match verdict {
        SoaRuleVerdictEvidence::Pass => "pass",
        SoaRuleVerdictEvidence::Warning => "warning",
        SoaRuleVerdictEvidence::Violation => "violation",
        SoaRuleVerdictEvidence::Critical => "critical",
    }
}

fn soa_violation_severity_csv(
    severity: crate::state::SoaViolationSeverityEvidence,
) -> &'static str {
    use crate::state::SoaViolationSeverityEvidence;
    match severity {
        SoaViolationSeverityEvidence::Warning => "warning",
        SoaViolationSeverityEvidence::Violation => "violation",
        SoaViolationSeverityEvidence::Critical => "critical",
    }
}

fn csv_text(value: &str) -> String {
    if value
        .chars()
        .any(|character| matches!(character, ',' | '"' | '\r' | '\n'))
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn serialized_enum_name<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .expect("retained evidence enums are JSON-serializable")
        .trim_matches('"')
        .to_owned()
}

fn export_typed_result_csv(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    prepared: &PreparedTypedResultCsv,
) {
    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Result CSV",
        default_name: prepared.default_name,
        filter_name: "CSV Files",
        filter_extensions: &["csv"],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "csv");
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_text_file_observed(&destination, &prepared.contents)
            });
            match export {
                Ok(()) => {
                    note_result_export_success(state, "CSV");
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "CSV",
                            &path,
                            Some(prepared.detail.clone()),
                            io,
                        ),
                    ));
                }
                Err(error) => {
                    note_result_export_failure(state, format!("CSV export failed: {error}"));
                    state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                        "CSV export failed: {error}"
                    )));
                }
            }
        }
        Ok(None) => {}
        Err(error) => {
            note_result_export_failure(state, format!("CSV destination failed: {error}"));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "CSV export failed: {error}"
            )));
        }
    }
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
    let default_name = format!("{stem}.tsv");
    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Result TSV",
        default_name: &default_name,
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
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "TSV",
                            &path,
                            Some(prepared.detail.clone()),
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

fn csv_to_tsv(contents: &str) -> Result<String, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(contents.as_bytes());
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(Vec::new());
    for record in reader.records() {
        let record = record.map_err(|error| format!("could not parse staged CSV rows: {error}"))?;
        writer
            .write_record(&record)
            .map_err(|error| format!("could not encode TSV row: {error}"))?;
    }
    let bytes = writer
        .into_inner()
        .map_err(|error| format!("could not finish TSV encoding: {}", error.error()))?;
    String::from_utf8(bytes).map_err(|error| format!("TSV encoder returned invalid UTF-8: {error}"))
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

#[derive(Debug)]
struct PreparedWaveformDataset {
    dataset: crate::io::WaveformDataset,
    warnings: Vec<String>,
}

struct ExportSignalSlice<'a> {
    name: &'a str,
    signal_type: crate::io::SignalType,
    x_values: &'a [f64],
    y_values: &'a [f64],
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
    use crate::workbench::documents::result_document::{
        AnalysisPresentationKey, SourceWaveformPresentationKey,
    };

    let analysis_key = AnalysisPresentationKey::new(dataset_id, analysis);
    analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            state.ui.results.waveform_visibility(
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
) -> Result<PreparedWaveformDataset, String> {
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
    prepare_single_analysis_dataset(analysis, &waveforms)
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
    let mut contents = String::from(
        "dataset_id,analysis_sequence,analysis_label,analysis_type,trace,component,sample_index,x,y\n",
    );
    let mut rows = 0_usize;
    let mut traces = 0_usize;
    for analysis in analyses {
        for waveform in exported_waveforms(state, displayed.dataset_id, analysis) {
            append_long_form_component(
                &mut contents,
                displayed.dataset_id,
                analysis,
                &waveform.name,
                "display",
                waveform.x.as_ref(),
                waveform.y.as_ref(),
                &mut rows,
            )?;
            traces += 1;
            if let Some(complex) = &waveform.complex {
                append_long_form_component(
                    &mut contents,
                    displayed.dataset_id,
                    analysis,
                    &complex.source_name,
                    "real",
                    waveform.x.as_ref(),
                    complex.real.as_ref(),
                    &mut rows,
                )?;
                append_long_form_component(
                    &mut contents,
                    displayed.dataset_id,
                    analysis,
                    &complex.source_name,
                    "imaginary",
                    waveform.x.as_ref(),
                    complex.imag.as_ref(),
                    &mut rows,
                )?;
            }
        }
    }
    if rows == 0 {
        return Err(NO_SAMPLES_MESSAGE.to_owned());
    }
    Ok(PreparedTypedResultCsv {
        default_name: "rspice-displayed-results.csv",
        contents,
        detail: format!("{traces} visible traces, {rows} exported samples"),
    })
}

fn append_long_form_component(
    contents: &mut String,
    dataset_id: crate::product::DatasetId,
    analysis: &crate::state::AnalysisResult,
    trace: &str,
    component: &str,
    x: &[f64],
    y: &[f64],
    rows: &mut usize,
) -> Result<(), String> {
    if x.len() != y.len() {
        return Err(format!(
            "Displayed trace '{}' has {} coordinates and {} {component} samples; export refused instead of truncating evidence.",
            sanitize_column_label(trace),
            x.len(),
            y.len(),
        ));
    }
    let dataset = dataset_id.to_string();
    let label = csv_text(&analysis.label);
    let analysis_type = csv_text(analysis.analysis_type.short_label());
    let trace = csv_text(trace);
    for (sample_index, (&x, &y)) in x.iter().zip(y).enumerate() {
        if !x.is_finite() || !y.is_finite() {
            return Err(format!(
                "Displayed trace {} contains a non-finite {component} sample at index {sample_index}.",
                sanitize_column_label(&trace),
            ));
        }
        contents.push_str(&format!(
            "{dataset},{},{label},{analysis_type},{trace},{component},{sample_index},{x:.17e},{y:.17e}\n",
            analysis.id,
        ));
        *rows += 1;
    }
    Ok(())
}

fn prepare_single_analysis_dataset(
    analysis: &crate::state::AnalysisResult,
    waveforms: &[&crate::state::WaveformData],
) -> Result<PreparedWaveformDataset, String> {
    let (x_name, x_signal_type) = axis_signal_for_analysis_type(analysis.analysis_type);
    let mut prepared = prepare_flat_waveform_dataset(waveforms, x_name, x_signal_type)?;
    if let Some(crate::state::AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
    }) = analysis.family_metadata.as_ref()
    {
        if reference_impedances_ohm.is_empty()
            || reference_impedances_ohm
                .iter()
                .any(|impedance| !impedance.is_finite() || *impedance <= 0.0)
        {
            return Err(
                "S-parameter export requires finite positive per-port reference impedances."
                    .to_owned(),
            );
        }
        prepared
            .dataset
            .metadata
            .insert("touchstone_version".to_owned(), "2".to_owned());
        prepared.dataset.metadata.insert(
            "z0_ports".to_owned(),
            reference_impedances_ohm
                .iter()
                .map(f64::to_string)
                .collect::<Vec<_>>()
                .join(","),
        );
        prepared
            .dataset
            .metadata
            .insert("z0".to_owned(), reference_impedances_ohm[0].to_string());
    }
    Ok(prepared)
}

fn prepare_flat_waveform_dataset(
    waveforms: &[&crate::state::WaveformData],
    x_name: String,
    x_signal_type: crate::io::SignalType,
) -> Result<PreparedWaveformDataset, String> {
    let reference_waveform = waveforms
        .iter()
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or_else(|| NO_SAMPLES_MESSAGE.to_string())?;

    let reference_len = reference_waveform.x.len();
    validate_shared_x_axis(waveforms, reference_waveform.x.as_ref())?;

    let mut dataset = crate::io::WaveformDataset::new("Simulation Results");
    let mut x_signal = crate::io::WaveformSignal::new(x_name, x_signal_type);
    x_signal.data.extend(reference_waveform.x.iter().copied());
    dataset.set_x(x_signal);

    let mut warnings = Vec::new();
    for waveform in waveforms {
        append_waveform_signal(
            &mut dataset,
            &mut warnings,
            &waveform.name,
            waveform,
            reference_len,
        )?;
    }

    Ok(PreparedWaveformDataset { dataset, warnings })
}

fn validate_shared_x_axis(
    waveforms: &[&crate::state::WaveformData],
    reference_x: &[f64],
) -> Result<(), String> {
    for waveform in waveforms {
        if waveform.x.as_ref() != reference_x {
            return Err(format!(
                "CSV export requires all signals in a shared-axis result to use identical x-axis samples; '{}' has different x-axis samples.",
                sanitize_column_label(&waveform.name)
            ));
        }
    }

    Ok(())
}

fn axis_signal_for_analysis_type(
    analysis: crate::state::AnalysisType,
) -> (String, crate::io::SignalType) {
    let axis_label = analysis.axis_info().0;
    if axis_label.eq_ignore_ascii_case("time") {
        ("time".to_string(), crate::io::SignalType::Time)
    } else if axis_label.eq_ignore_ascii_case("frequency") {
        ("frequency".to_string(), crate::io::SignalType::Frequency)
    } else if axis_label.trim().is_empty() {
        ("x".to_string(), crate::io::SignalType::Unknown)
    } else {
        (
            axis_label
                .trim()
                .to_ascii_lowercase()
                .replace([' ', '-'], "_"),
            crate::io::SignalType::Unknown,
        )
    }
}

fn append_waveform_signal(
    dataset: &mut crate::io::WaveformDataset,
    warnings: &mut Vec<String>,
    signal_name: &str,
    waveform: &crate::state::WaveformData,
    reference_len: usize,
) -> Result<(), String> {
    append_signal_values(
        dataset,
        warnings,
        ExportSignalSlice {
            name: signal_name,
            signal_type: signal_type_from_waveform_name(signal_name),
            x_values: waveform.x.as_ref(),
            y_values: waveform.y.as_ref(),
        },
        reference_len,
    )?;

    if let Some(complex) = &waveform.complex {
        let real_name = format!("re({})", complex.source_name);
        append_signal_values(
            dataset,
            warnings,
            ExportSignalSlice {
                name: &real_name,
                signal_type: complex_signal_type(&complex.source_name, true),
                x_values: waveform.x.as_ref(),
                y_values: complex.real.as_ref(),
            },
            reference_len,
        )?;
        let imag_name = format!("im({})", complex.source_name);
        append_signal_values(
            dataset,
            warnings,
            ExportSignalSlice {
                name: &imag_name,
                signal_type: complex_signal_type(&complex.source_name, false),
                x_values: waveform.x.as_ref(),
                y_values: complex.imag.as_ref(),
            },
            reference_len,
        )?;
    }
    Ok(())
}

fn append_signal_values(
    dataset: &mut crate::io::WaveformDataset,
    warnings: &mut Vec<String>,
    signal: ExportSignalSlice<'_>,
    reference_len: usize,
) -> Result<(), String> {
    let export_name = sanitize_column_label(signal.name);
    let mut export_signal = crate::io::WaveformSignal::new(&export_name, signal.signal_type);

    let available_points = signal.x_values.len().min(signal.y_values.len());
    if signal.x_values.len() != signal.y_values.len() {
        return Err(format!(
            "Signal '{}' has {} x samples and {} y samples; export refused instead of truncating evidence.",
            export_name,
            signal.x_values.len(),
            signal.y_values.len(),
        ));
    }

    if available_points > reference_len {
        return Err(format!(
            "Signal '{}' has {} samples, exceeding shared x-axis length {}; export refused instead of truncating evidence.",
            export_name, available_points, reference_len
        ));
    }

    export_signal.data.extend(signal.y_values.iter().copied());
    dataset.add_signal(export_signal);
    let _ = warnings;
    Ok(())
}

fn sanitize_column_label(label: &str) -> String {
    let sanitized = label
        .trim()
        .chars()
        .map(|ch| match ch {
            ',' | '\t' | '\r' | '\n' => ' ',
            _ => ch,
        })
        .collect::<String>();
    sanitized.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn signal_type_from_waveform_name(name: &str) -> crate::io::SignalType {
    if name.starts_with("V(") || name.starts_with("v(") {
        crate::io::SignalType::Voltage
    } else if name.starts_with("I(") || name.starts_with("i(") {
        crate::io::SignalType::Current
    } else {
        crate::io::SignalType::Unknown
    }
}

fn complex_signal_type(source_name: &str, real: bool) -> crate::io::SignalType {
    if source_name.starts_with("V(") || source_name.starts_with("v(") {
        if real {
            crate::io::SignalType::VoltageReal
        } else {
            crate::io::SignalType::VoltageImag
        }
    } else if source_name.starts_with("I(") || source_name.starts_with("i(") {
        if real {
            crate::io::SignalType::CurrentReal
        } else {
            crate::io::SignalType::CurrentImag
        }
    } else if source_name.starts_with('S') || source_name.starts_with('s') {
        crate::io::SignalType::SParameter
    } else {
        crate::io::SignalType::Unknown
    }
}

#[cfg(test)]
mod tests;
