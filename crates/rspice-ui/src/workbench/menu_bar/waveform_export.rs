//! Waveform export actions.

use crate::workbench::EngineeringExportFormat;
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

const NO_ACTIVE_ANALYSIS_MESSAGE: &str = "No active result analysis is selected for export.";
const NO_SAMPLES_MESSAGE: &str = "No waveform samples available to export.";

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
        Ok(()) => state.push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
            "Exported {} exact retained result item(s).",
            keys.len()
        ))),
        Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "Could not export exact retained evidence: {error}"
        ))),
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

    if let Some(sheet) = prepare_active_sheet_csv(state, &displayed) {
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

fn prepare_active_sheet_csv(
    state: &AppState,
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
) -> Option<PreparedTypedResultCsv> {
    use crate::workbench::ResultViewer;
    use crate::workbench::documents::result_document;

    let sheet = match displayed.viewer {
        ResultViewer::Manifest => Some(result_document::export_manifest_csv(displayed.run(state)?)),
        ResultViewer::Op => {
            result_document::export_operating_point_csv(displayed.primary_analysis(state)?)
        }
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
    Some(PreparedTypedResultCsv {
        default_name: sheet.default_name,
        contents: sheet.contents,
        detail: sheet.detail,
    })
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

fn eye_measurements_csv(state: &AppState) -> Option<PreparedTypedResultCsv> {
    let eye = &state.analysis.eye_diagram_state;
    if eye.data.traces.is_empty() {
        return None;
    }
    let m = &eye.measurements;
    let mut contents = String::from("field,value,unit\n");
    for (field, value, unit) in [
        ("acquisitions", eye.data.traces.len().to_string(), ""),
        ("unit_intervals", eye.data.ui_count.to_string(), ""),
        ("data_rate", format!("{:.17e}", m.data_rate), "b/s"),
        ("unit_interval", format!("{:.17e}", m.unit_interval), "s"),
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
        ("rise_time", format!("{:.17e}", m.rise_time), "s"),
        ("fall_time", format!("{:.17e}", m.fall_time), "s"),
        ("jitter_pp", format!("{:.17e}", m.jitter_pp), "s"),
        ("jitter_rms", format!("{:.17e}", m.jitter_rms), "s"),
        ("jitter_dj", format!("{:.17e}", m.jitter_dj), "s"),
        ("crossing_level", format!("{:.17e}", m.crossing_level), "V"),
        (
            "crossing_percentage",
            format!("{:.17e}", m.crossing_percentage),
            "",
        ),
        ("snr", format!("{:.17e}", m.snr_db), "dB"),
        ("q_factor", format!("{:.17e}", m.q_factor), ""),
        ("estimated_ber", format!("{:.17e}", m.estimated_ber), ""),
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
                Ok(()) => state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                    crate::workbench::workflows::export_workflow::export_completion_message(
                        "CSV",
                        &path,
                        Some(prepared.detail.clone()),
                        io,
                    ),
                )),
                Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(
                    format!("CSV export failed: {error}"),
                )),
            }
        }
        Ok(None) => {}
        Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "CSV export failed: {error}"
        ))),
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
                Ok(()) => state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                    crate::workbench::workflows::export_workflow::export_completion_message(
                        "TSV",
                        &path,
                        Some(prepared.detail.clone()),
                        io,
                    ),
                )),
                Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(
                    format!("TSV export failed: {error}"),
                )),
            }
        }
        Ok(None) => {}
        Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "TSV export failed: {error}"
        ))),
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
                Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(
                    format!("TSV export failed: {error}"),
                )),
            }
        }
        Ok(None) => {}
        Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "TSV export failed: {error}"
        ))),
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
                Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(
                    format!("Touchstone export failed: {error}"),
                )),
            }
        }
        Ok(None) => {}
        Err(error) => state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "Touchstone export failed: {error}"
        ))),
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

fn prepare_waveform_dataset(
    state: &AppState,
    displayed: &crate::workbench::documents::result_document::view_context::ResolvedResultView,
) -> Result<PreparedWaveformDataset, String> {
    if displayed.analysis_indices.is_empty() {
        return Err(NO_SAMPLES_MESSAGE.to_owned());
    }
    let analysis = displayed
        .primary_analysis(state)
        .ok_or_else(|| NO_ACTIVE_ANALYSIS_MESSAGE.to_string())?;
    prepare_single_analysis_dataset(analysis)
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
        for waveform in analysis
            .waveforms
            .iter()
            .filter(|waveform| waveform.visible)
        {
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

#[allow(clippy::too_many_arguments)]
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
) -> Result<PreparedWaveformDataset, String> {
    let (x_name, x_signal_type) = axis_signal_for_analysis_type(analysis.analysis_type);
    let mut prepared = prepare_flat_waveform_dataset(&analysis.waveforms, x_name, x_signal_type)?;
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
    waveforms: &[crate::state::WaveformData],
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
    waveforms: &[crate::state::WaveformData],
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
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::path::{Path, PathBuf};

    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType,
        ComplexResultValue, ReliabilityCheckpointEvidence, ReliabilityDeviceEvidence,
        ReliabilityShiftEvidence, ReliabilityStressEvidence, SensitivityResultMode,
        SensitivityResultRow, SimulationRun, SoaEvaluationEvidence, SoaParameterEvidence,
        SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence, WaveformData,
    };

    #[derive(Debug)]
    struct MockExportWorkflowIo {
        datasets: RefCell<Vec<crate::io::WaveformDataset>>,
        paths: RefCell<Vec<PathBuf>>,
        text_files: RefCell<Vec<(PathBuf, String)>>,
        dialog_titles: RefCell<Vec<String>>,
        saved_paths_are_reopenable: bool,
    }

    impl Default for MockExportWorkflowIo {
        fn default() -> Self {
            Self {
                datasets: RefCell::default(),
                paths: RefCell::default(),
                text_files: RefCell::default(),
                dialog_titles: RefCell::default(),
                saved_paths_are_reopenable: true,
            }
        }
    }

    impl ExportWorkflowIo for MockExportWorkflowIo {
        fn show_save_dialog(
            &self,
            config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            self.dialog_titles
                .borrow_mut()
                .push(config.title.to_owned());
            Ok(Some(PathBuf::from(config.default_name)))
        }

        fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.text_files
                .borrow_mut()
                .push((path.to_path_buf(), contents.to_owned()));
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            dataset: &crate::io::WaveformDataset,
            path: &Path,
        ) -> Result<(), String> {
            self.datasets.borrow_mut().push(dataset.clone());
            self.paths.borrow_mut().push(path.to_path_buf());
            Ok(())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            self.saved_paths_are_reopenable
        }
    }

    fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> WaveformData {
        WaveformData::new(name.to_owned(), x, y, "#4f81bd")
    }

    #[test]
    fn export_registry_matches_the_fifteen_contract_ids_and_governs_availability() {
        assert_eq!(
            ResultExportFormat::ALL.map(ResultExportFormat::canonical_id),
            [
                "rspice-result-bundle",
                "rspice-dataset-bundle",
                "csv-rfc4180",
                "tsv",
                "touchstone-v2",
                "hdf5",
                "arrow-ipc",
                "parquet",
                "numpy-npy",
                "numpy-npz",
                "matlab-v5",
                "matlab-v7.3",
                "json-lines",
                "vcd",
                "fst",
            ]
        );
        for format in ResultExportFormat::ALL {
            let availability = result_export_format_availability(format);
            if matches!(
                format,
                ResultExportFormat::CsvRfc4180
                    | ResultExportFormat::Tsv
                    | ResultExportFormat::TouchstoneV2
            ) {
                assert!(availability.is_ok(), "{}", format.canonical_id());
            } else {
                let error = availability.expect_err("encoder is governed unavailable");
                assert!(error.contains(format.canonical_id()), "{error}");
                assert!(error.contains("verified lossless encoder"), "{error}");
            }
        }
    }

    fn complex_waveform(
        name: &str,
        source_name: &str,
        x: Vec<f64>,
        display_y: Vec<f64>,
        real: Vec<f64>,
        imag: Vec<f64>,
    ) -> WaveformData {
        WaveformData::new(name.to_owned(), x, display_y, "#4f81bd").with_complex_components(
            source_name.to_owned(),
            real,
            imag,
        )
    }

    fn last_log_message(state: &AppState) -> String {
        state
            .log_buffer
            .entries()
            .last()
            .expect("a user-facing log line is emitted")
            .message
            .clone()
    }

    fn state_with_typed_result(analysis: AnalysisResult) -> AppState {
        let viewer = match analysis.analysis_type {
            AnalysisType::DcOp => crate::workbench::ResultViewer::Op,
            AnalysisType::PoleZero => crate::workbench::ResultViewer::PoleZero,
            AnalysisType::Sensitivity => crate::workbench::ResultViewer::Contribution,
            AnalysisType::Tf => crate::workbench::ResultViewer::TransferFunction,
            AnalysisType::Reliability => crate::workbench::ResultViewer::Reliability,
            AnalysisType::Soa => crate::workbench::ResultViewer::Soa,
            AnalysisType::Optimization => crate::workbench::ResultViewer::Optimization,
            kind if kind.is_time_domain() => crate::workbench::ResultViewer::Waves,
            kind if kind.is_bode_response() || kind.is_raw_frequency_curve() => {
                crate::workbench::ResultViewer::Bode
            }
            _ => crate::workbench::ResultViewer::Table,
        };
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, viewer);
        state
    }

    fn activate_result_document(state: &mut AppState, viewer: crate::workbench::ResultViewer) {
        let dataset_id = state
            .simulation
            .active_run()
            .expect("test retains an active result run")
            .dataset_id;
        state.workbench.documents.activate(
            crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
        );
        state.ui.results.viewer = viewer;
    }

    #[test]
    fn report_and_table_sheets_export_the_evidence_they_render() {
        let mut manifest_state = state_with_typed_result(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient")
                .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0], vec![0.0, 1.0])]),
        );
        activate_result_document(
            &mut manifest_state,
            crate::workbench::ResultViewer::Manifest,
        );
        let manifest_io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut manifest_state, &manifest_io);
        let manifest = &manifest_io.text_files.borrow()[0].1;
        assert!(manifest.contains("dataset_id"));
        assert!(manifest.contains("inventory,,,Transient"));

        let mut op = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
        op.dc_op = Some(crate::state::DcOpResult {
            node_voltages: vec![crate::state::OperatingPointValue {
                name: "V(out)".to_owned(),
                value: 1.25,
                unit: "V".to_owned(),
            }],
            ..crate::state::DcOpResult::default()
        });
        let mut op_state = state_with_typed_result(op);
        let op_io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut op_state, &op_io);
        let op_csv = &op_io.text_files.borrow()[0].1;
        assert!(op_csv.contains("node_voltage,V(out),,,value,1.25000000000000000e0,V"));

        let mut specs_state = state_with_typed_result(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.5)]),
        );
        specs_state.workspace.specs.push(crate::state::SpecEntry {
            measurement: "gain".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(1.0),
            max: Some(2.0),
            unit: "V/V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        });
        activate_result_document(&mut specs_state, crate::workbench::ResultViewer::Specs);
        let specs_io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut specs_state, &specs_io);
        let specs = &specs_io.text_files.borrow()[0].1;
        assert!(specs.contains("gain,max V(out),1.50000000000000000e0"));
        assert!(specs.contains(",pass,"));

        let optimization = AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
            .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
                iterations: vec![0.0, 1.0],
                best_cost: 0.25,
                best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
                converged: true,
            })
            .with_waveforms(vec![
                waveform("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25]),
                waveform("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6]),
            ]);
        let mut optimization_state = state_with_typed_result(optimization);
        let optimization_io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut optimization_state, &optimization_io);
        let optimization = &optimization_io.text_files.borrow()[0].1;
        assert!(optimization.contains("converged,true"));
        assert!(optimization.contains("iteration,cost,w"));
        assert!(optimization.contains("1.00000000000000000e0,2.50000000000000000e-1"));
    }

    /// A derived sheet must export the curve it drew, not the samples it was
    /// derived from.
    ///
    /// The export routed on the retained payload alone, so exporting from the
    /// FFT sheet wrote the transient — the input to the spectrum, never the
    /// spectrum. The distribution and the eye had the same gap, and none of
    /// the three is retained anywhere the payload path can see.
    #[test]
    fn csv_export_from_a_derived_sheet_publishes_what_that_sheet_draws() {
        use crate::analysis::fft::data::{FftData, SpectrumNormalization};
        use crate::analysis::histogram::data::{Histogram, HistogramBin};

        let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.0])]);
        let mut state = state_with_typed_result(transient);
        state.analysis.fft_state.data = Some(FftData::from_spectrum_with_normalization(
            "V(out)",
            &[0.0, 1.0e3, 2.0e3],
            &[1.0, 0.5, 0.25],
            &[0.0, 0.1, 0.2],
            8_192.0,
            SpectrumNormalization::Peak,
        ));
        state.ui.results.viewer = crate::workbench::ResultViewer::Fft;
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        let files = io.text_files.borrow();
        assert_eq!(files[0].0, PathBuf::from("rspice-spectrum.csv"));
        let spectrum = files[0].1.clone();
        drop(files);
        assert!(
            spectrum.contains("frequency_hz,magnitude,magnitude_db,phase_rad"),
            "{spectrum}"
        );
        assert!(spectrum.contains("2.00000000000000000e3"), "{spectrum}");
        assert!(
            !spectrum.contains("V(out)\ntime"),
            "the transient leaked into a spectrum export: {spectrum}"
        );

        // The distribution exports its bins, not the Monte-Carlo waveform.
        let mut state =
            state_with_typed_result(
                AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC")
                    .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0], vec![0.9, 1.1])]),
            );
        state.analysis.histogram_state.load_histogram(Histogram {
            name: "V(out)".to_owned(),
            bins: vec![HistogramBin {
                lower: 0.9,
                upper: 1.0,
                count: 3,
                weight: 3.0,
            }],
            total_count: 3,
            total_weight: 3.0,
            underflow: 0,
            overflow: 0,
            data_min: 0.9,
            data_max: 1.0,
        });
        state.ui.results.viewer = crate::workbench::ResultViewer::Hist;
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        let files = io.text_files.borrow();
        assert_eq!(files[0].0, PathBuf::from("rspice-distribution.csv"));
        assert!(
            files[0].1.contains("bin_lower,bin_upper,count,weight"),
            "{}",
            files[0].1
        );
    }

    /// A sheet that plots a retained vector keeps the payload path: its
    /// export already is what it shows, and a second writer would be a
    /// second answer to the same question.
    #[test]
    fn csv_export_from_a_retained_sheet_is_unchanged_by_the_derived_route() {
        let mut state =
            state_with_typed_result(
                AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                    .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0], vec![0.0, 2.0])]),
            );
        state.ui.results.viewer = crate::workbench::ResultViewer::Waves;
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        assert!(
            io.text_files.borrow().is_empty(),
            "a waveform sheet must still go through the waveform dataset writer"
        );
        assert_eq!(io.datasets.borrow().len(), 1);
    }

    #[test]
    fn csv_export_publishes_exact_pole_zero_evidence_without_pseudo_waveforms() {
        let analysis = AnalysisResult::new(1, AnalysisType::PoleZero, "PZ").with_result_payload(
            AnalysisResultPayload::PoleZero {
                poles: vec![ComplexResultValue {
                    real: -1.0,
                    imaginary: 2.0,
                }],
                zeros: vec![ComplexResultValue {
                    real: -3.0,
                    imaginary: 0.0,
                }],
                gain: 4.25,
            },
        );
        let mut state = state_with_typed_result(analysis);
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        assert!(io.datasets.borrow().is_empty());
        assert_eq!(io.dialog_titles.borrow().as_slice(), &["Export Result CSV"]);
        let files = io.text_files.borrow();
        assert_eq!(files[0].0, PathBuf::from("pole-zero.csv"));
        assert_eq!(
            files[0].1,
            concat!(
                "record,index,real_rad_per_s,imaginary_rad_per_s,value\n",
                "gain,,,,4.25000000000000000e0\n",
                "pole,0,-1.00000000000000000e0,2.00000000000000000e0,\n",
                "zero,0,-3.00000000000000000e0,0.00000000000000000e0,\n",
            )
        );
    }

    #[test]
    fn csv_export_publishes_canonical_sensitivity_rows_and_basis() {
        let analysis = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out), differential".to_owned(),
                result_mode: SensitivityResultMode::Ac {
                    frequency_hz: 10_000.0,
                },
                rows: vec![SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 2.0,
                    normalized: 0.5,
                }],
            });
        let mut state = state_with_typed_result(analysis);
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        let files = io.text_files.borrow();
        assert_eq!(files[0].0, PathBuf::from("sensitivity.csv"));
        assert_eq!(
            files[0].1,
            concat!(
                "parameter,raw_sensitivity,normalized_sensitivity,output,mode,frequency_hz\n",
                "width,2.00000000000000000e0,5.00000000000000000e-1,\"V(out), differential\",ac,1.00000000000000000e4\n",
            )
        );
    }

    #[test]
    fn csv_export_publishes_exact_reliability_device_and_shift_evidence() {
        let analysis = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability { years: vec![10.0] })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![ReliabilityDeviceEvidence {
                    device_id: "M1".to_owned(),
                    stress: ReliabilityStressEvidence {
                        average_gate_stress_v: 1.2,
                        average_drain_stress_v: 1.8,
                        average_temperature_k: 358.15,
                        duration_s: 3_600.0,
                    },
                    checkpoints: vec![ReliabilityCheckpointEvidence {
                        years: 10.0,
                        shift: ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 0.0125,
                            mobility_shift: -0.003,
                            drain_source_resistance_shift: 0.001,
                        },
                    }],
                }],
            });
        let mut state = state_with_typed_result(analysis);
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        let files = io.text_files.borrow();
        assert_eq!(files[0].0, PathBuf::from("reliability-evidence.csv"));
        assert!(
            files[0]
                .1
                .starts_with("device,lifetime_years,average_gate_stress_v")
        );
        assert!(
            files[0]
                .1
                .contains("M1,1.00000000000000000e1,1.19999999999999996e0")
        );
        assert!(files[0].1.contains("1.25000000000000007e-2"));
    }

    #[test]
    fn typed_csv_rejects_payloads_that_contradict_their_retained_axis() {
        let analysis = AnalysisResult::new(1, AnalysisType::Reliability, "Reliability")
            .with_family_metadata(AnalysisResultFamilyMetadata::Reliability { years: vec![1.0] })
            .with_result_payload(AnalysisResultPayload::Reliability {
                devices: vec![ReliabilityDeviceEvidence {
                    device_id: "M1".to_owned(),
                    stress: ReliabilityStressEvidence {
                        average_gate_stress_v: 1.2,
                        average_drain_stress_v: 1.8,
                        average_temperature_k: 358.15,
                        duration_s: 3_600.0,
                    },
                    checkpoints: vec![ReliabilityCheckpointEvidence {
                        years: 10.0,
                        shift: ReliabilityShiftEvidence {
                            threshold_voltage_shift_v: 0.0125,
                            mobility_shift: -0.003,
                            drain_source_resistance_shift: 0.001,
                        },
                    }],
                }],
            });
        let state = state_with_typed_result(analysis);

        assert!(prepare_typed_result_csv(state.simulation.active_analysis().unwrap()).is_none());
    }

    #[test]
    fn csv_export_publishes_complete_soa_rules_and_exact_events() {
        let analysis = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time: vec![1.0e-6] })
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations: vec![SoaEvaluationEvidence {
                    device_id: "M1".to_owned(),
                    parameter: SoaParameterEvidence::DrainSourceVoltage,
                    limit_value: 3.3,
                    worst_actual_value: 3.2,
                    worst_time_s: 1.0e-6,
                    sample_count: 1,
                    unit: "V".to_owned(),
                    description: "Maximum drain-source voltage".to_owned(),
                    verdict: SoaRuleVerdictEvidence::Warning,
                }],
                violations: vec![SoaViolationEvidence {
                    device_id: "M1".to_owned(),
                    parameter: SoaParameterEvidence::DrainSourceVoltage,
                    limit_value: 3.3,
                    actual_value: 3.2,
                    time_s: 1.0e-6,
                    severity: SoaViolationSeverityEvidence::Warning,
                }],
            });
        let mut state = state_with_typed_result(analysis);
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        let files = io.text_files.borrow();
        assert_eq!(files[0].0, PathBuf::from("soa-evidence.csv"));
        assert!(files[0].1.contains("evaluation,M1,vds"));
        assert!(files[0].1.contains("Maximum drain-source voltage,warning"));
        assert!(files[0].1.contains("event,M1,vds"));
    }

    #[test]
    fn bode_export_uses_the_displayed_frequency_analysis_not_the_global_selector() {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
        let ac =
            AnalysisResult::new(2, AnalysisType::Ac, "AC Analysis").with_waveforms(vec![waveform(
                "|V(out)|",
                vec![1.0e3, 1.0e4, 1.0e5],
                vec![0.1, 0.2, 0.3],
            )]);

        let mut run = SimulationRun::new(7);
        run.add_analysis(transient.clone());
        run.add_analysis(ac);

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        // Deliberately leave the simulation selector on TRAN while the Results
        // document displays Bode. Export authority is the document, not this
        // unrelated ordinal.
        state.simulation.active_analysis_idx = Some(0);
        state
            .simulation
            .replace_waveforms(transient.waveforms.clone());
        activate_result_document(&mut state, crate::workbench::ResultViewer::Bode);

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        let datasets = io.datasets.borrow();
        assert_eq!(datasets.len(), 1);
        let dataset = &datasets[0];
        assert_eq!(
            dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
            Some("frequency")
        );
        assert_eq!(dataset.signal_names(), vec!["|V(out)|"]);
        assert_eq!(dataset.point_count(), 3);
        assert_eq!(
            dataset
                .x_signal
                .as_ref()
                .map(|signal| signal.data.as_slice()),
            Some(&[1.0e3, 1.0e4, 1.0e5][..])
        );
        assert!(dataset.get_signal("V(out)").is_none());
    }

    #[test]
    fn waves_export_preserves_every_displayed_analysis_and_independent_axes() {
        let tran_a = AnalysisResult::new(1, AnalysisType::Transient, "TRAN A")
            .with_waveforms(vec![waveform("V(a)", vec![0.0, 1.0], vec![1.0, 2.0])]);
        let tran_b =
            AnalysisResult::new(2, AnalysisType::Transient, "TRAN B").with_waveforms(vec![
                waveform("V(b)", vec![10.0, 20.0, 30.0], vec![3.0, 4.0, 5.0]),
            ]);
        let mut run = SimulationRun::new(7);
        run.add_analysis(tran_a);
        run.add_analysis(tran_b);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);
        let io = MockExportWorkflowIo::default();

        action_export_csv_with_io(&mut state, &io);

        assert!(io.datasets.borrow().is_empty());
        let files = io.text_files.borrow();
        assert_eq!(files[0].0, PathBuf::from("rspice-displayed-results.csv"));
        let csv = &files[0].1;
        assert!(csv.starts_with("dataset_id,analysis_sequence,analysis_label"));
        assert!(csv.contains(",1,TRAN A,TR,V(a),display,1,1.00000000000000000e0"));
        assert!(csv.contains(",2,TRAN B,TR,V(b),display,2,3.00000000000000000e1"));
    }

    #[test]
    fn csv_export_preserves_single_analysis_axis_shape() {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);

        let mut run = SimulationRun::new(7);
        run.add_analysis(transient.clone());

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state
            .simulation
            .replace_waveforms(transient.waveforms.clone());
        activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        let datasets = io.datasets.borrow();
        assert_eq!(datasets.len(), 1);
        let dataset = &datasets[0];
        assert_eq!(
            dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
            Some("time")
        );
        assert_eq!(dataset.signal_names(), vec!["V(out)"]);
        assert_eq!(dataset.point_count(), 2);
    }

    #[test]
    fn csv_export_uses_the_displayed_analysis_without_a_global_analysis_selector() {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
        let mut run = SimulationRun::new(7);
        run.add_analysis(transient.clone());

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state
            .simulation
            .replace_waveforms(transient.waveforms.clone());
        activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        assert_eq!(io.datasets.borrow().len(), 1);
        assert_eq!(io.datasets.borrow()[0].signal_names(), vec!["V(out)"]);
    }

    #[test]
    fn csv_export_fails_closed_when_the_active_analysis_has_no_samples() {
        let mut run = SimulationRun::new(7);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "Transient"));

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        assert!(io.dialog_titles.borrow().is_empty());
        assert!(io.datasets.borrow().is_empty());
        assert_eq!(last_log_message(&state), NO_SAMPLES_MESSAGE);
    }

    #[test]
    fn csv_export_rejects_single_analysis_divergent_x_axes() {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 2.0]),
                waveform("V(in)", vec![10.0, 20.0], vec![5.0, 6.0]),
            ]);

        let mut run = SimulationRun::new(7);
        run.add_analysis(transient);

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        assert!(io.datasets.borrow().is_empty());
        assert!(
            state
                .log_buffer
                .entries()
                .any(|entry| entry.message.contains("different x-axis samples"))
        );
    }

    #[test]
    fn csv_export_ignores_divergent_axes_owned_by_an_inactive_analysis() {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![
                waveform("V(out)", vec![0.0, 1.0], vec![0.0, 1.0]),
                waveform("V(in)", vec![10.0, 20.0], vec![5.0, 6.0]),
            ]);
        let ac =
            AnalysisResult::new(2, AnalysisType::Ac, "AC Analysis").with_waveforms(vec![waveform(
                "|V(out)|",
                vec![1.0e3, 1.0e4],
                vec![0.1, 0.2],
            )]);

        let mut run = SimulationRun::new(7);
        run.add_analysis(transient);
        run.add_analysis(ac);

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(1);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Bode);

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        let datasets = io.datasets.borrow();
        assert_eq!(datasets.len(), 1);
        let dataset = &datasets[0];
        assert_eq!(
            dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
            Some("frequency")
        );
        assert_eq!(dataset.signal_names(), vec!["|V(out)|"]);
    }

    #[test]
    fn csv_export_includes_complex_real_and_imaginary_columns() {
        let ac = AnalysisResult::new(1, AnalysisType::Ac, "AC Analysis").with_waveforms(vec![
            complex_waveform(
                "|V(out)|",
                "V(out)",
                vec![1.0e3, 1.0e4],
                vec![1.0, 2.0],
                vec![0.8, 1.6],
                vec![0.6, 1.2],
            ),
        ]);

        let mut run = SimulationRun::new(7);
        run.add_analysis(ac);

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Bode);

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        let datasets = io.datasets.borrow();
        assert_eq!(datasets.len(), 1);
        let dataset = &datasets[0];
        assert_eq!(
            dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
            Some("frequency")
        );
        assert_eq!(
            dataset.signal_names(),
            vec!["|V(out)|", "re(V(out))", "im(V(out))"]
        );
        assert_eq!(
            dataset
                .get_signal("re(V(out))")
                .map(|signal| signal.data.as_slice()),
            Some(&[0.8, 1.6][..])
        );
        assert_eq!(
            dataset
                .get_signal("im(V(out))")
                .map(|signal| signal.data.as_slice()),
            Some(&[0.6, 1.2][..])
        );
    }

    #[test]
    fn engineering_export_preference_dispatches_compatible_touchstone() {
        let frequency = vec![1.0e6, 2.0e6];
        let waveforms = ["S11", "S12", "S21", "S22"]
            .into_iter()
            .enumerate()
            .map(|(index, name)| {
                complex_waveform(
                    &format!("|{name}|"),
                    name,
                    frequency.clone(),
                    vec![0.1 + index as f64 * 0.1, 0.2 + index as f64 * 0.1],
                    vec![0.01 + index as f64 * 0.1, 0.02 + index as f64 * 0.1],
                    vec![0.001 + index as f64 * 0.01, 0.002 + index as f64 * 0.01],
                )
            })
            .collect::<Vec<_>>();
        let ac = AnalysisResult::new(1, AnalysisType::SParameter, "S-parameters")
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![50.0, 75.0],
            })
            .with_waveforms(waveforms);
        let mut run = SimulationRun::new(7);
        run.add_analysis(ac);

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Smith);
        state
            .ui
            .preferences
            .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 1)
            .unwrap();

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        assert!(io.datasets.borrow().is_empty());
        assert_eq!(io.dialog_titles.borrow().as_slice(), &["Export Touchstone"]);
        let text_files = io.text_files.borrow();
        assert_eq!(text_files.len(), 1);
        assert_eq!(text_files[0].0, PathBuf::from("waveforms.snp"));
        assert!(text_files[0].1.contains("[Version] 2.0"));
        assert!(text_files[0].1.contains("[Reference] 5e1 7.5e1"));
        assert!(text_files[0].1.contains("# Hz S RI R 50"));
        assert!(text_files[0].1.contains("1e6"));
    }

    #[test]
    fn engineering_export_preference_dispatches_exact_tsv() {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![waveform(
                "V(out)",
                vec![0.0, 1.0e-6],
                vec![0.0, 1.234_567_890_123_456],
            )]);
        let mut state = state_with_typed_result(transient);
        state
            .ui
            .preferences
            .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 2)
            .expect("TSV preference");
        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        assert_eq!(
            io.dialog_titles.borrow().as_slice(),
            &["Export Waveform TSV"]
        );
        let files = io.text_files.borrow();
        assert_eq!(files[0].0, PathBuf::from("waveforms.tsv"));
        assert!(files[0].1.starts_with("time\tV(out)\n"));
        assert!(files[0].1.contains("1.234567890123456"));
    }

    #[test]
    fn flat_export_refuses_mismatched_sample_lengths_without_opening_a_picker() {
        let transient =
            AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
                waveform("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0]),
            ]);
        let mut state = state_with_typed_result(transient);
        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);
        assert!(io.dialog_titles.borrow().is_empty());
        assert!(last_log_message(&state).contains("refused instead of truncating evidence"));
    }

    #[test]
    fn incompatible_touchstone_result_is_rejected_before_save_picker() {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
        let mut run = SimulationRun::new(7);
        run.add_analysis(transient);

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);
        state
            .ui
            .preferences
            .set_choice(crate::workbench::ChoicePreference::EngineeringExport, 1)
            .unwrap();

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        assert!(io.dialog_titles.borrow().is_empty());
        assert!(io.text_files.borrow().is_empty());
        assert!(last_log_message(&state).contains("not compatible"));
    }

    #[test]
    fn displayed_digits_never_reduce_csv_source_precision() {
        let exact = 1.234_567_890_123_456_f64;
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![waveform("V(out)", vec![0.0], vec![exact])]);
        let mut run = SimulationRun::new(7);
        run.add_analysis(transient);

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);
        state
            .ui
            .preferences
            .set_scalar(
                crate::workbench::ScalarPreference::DisplayedSignificantDigits,
                3,
            )
            .unwrap();

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        let datasets = io.datasets.borrow();
        assert_eq!(datasets[0].get_signal("V(out)").unwrap().data, vec![exact]);
    }

    #[test]
    fn csv_export_reports_browser_download_start_without_claiming_file_written() {
        let transient = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, 1.2])]);
        let mut run = SimulationRun::new(7);
        run.add_analysis(transient.clone());

        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state
            .simulation
            .replace_waveforms(transient.waveforms.clone());
        activate_result_document(&mut state, crate::workbench::ResultViewer::Waves);

        let io = MockExportWorkflowIo {
            saved_paths_are_reopenable: false,
            ..Default::default()
        };
        action_export_csv_with_io(&mut state, &io);

        assert_eq!(
            last_log_message(&state),
            "CSV download started: waveforms.csv (1 signals, 2 points; confirm the browser accepted the download)"
        );
    }
}
