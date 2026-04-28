use crate::common::app::AppState;
use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

const NO_RESULTS_MESSAGE: &str = "No simulation results to export. Run a simulation first.";
const NO_SAMPLES_MESSAGE: &str = "No waveform samples available to export.";

pub(super) fn action_export_csv_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let prepared = match prepare_waveform_dataset(state) {
        Ok(prepared) => prepared,
        Err(message) => {
            state.push_user_message(crate::common::app::ConsoleMessage::warning(message));
            return;
        }
    };

    for warning in &prepared.warnings {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(warning.clone()));
    }

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Waveform CSV",
        default_name: "waveforms.csv",
        filter_name: "CSV Files",
        filter_extensions: &["csv"],
    }) {
        Some(mut path) => {
            crate::common::file_actions::ensure_file_extension(&mut path, "csv");

            match io.write_waveform_csv(&prepared.dataset, &path) {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                        "Exported CSV: {} ({} signals, {} points)",
                        path.display(),
                        prepared.dataset.signal_count(),
                        prepared.dataset.point_count()
                    )));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                        "CSV export failed: {}",
                        e
                    )));
                }
            }
        }
        None => {
            // User cancelled - no message needed
        }
    }
}

#[derive(Debug)]
struct PreparedWaveformDataset {
    dataset: crate::io::WaveformDataset,
    warnings: Vec<String>,
}

fn prepare_waveform_dataset(state: &AppState) -> Result<PreparedWaveformDataset, String> {
    if state.simulation.waveforms.is_empty() {
        return Err(NO_RESULTS_MESSAGE.to_string());
    }

    let reference_waveform = state
        .simulation
        .waveforms
        .iter()
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or_else(|| NO_SAMPLES_MESSAGE.to_string())?;

    let reference_len = reference_waveform.x.len();
    let (x_name, x_signal_type) = detect_x_axis_signal(state);

    let mut dataset = crate::io::WaveformDataset::new("Simulation Results");
    let mut x_signal = crate::io::WaveformSignal::new(x_name, x_signal_type);
    x_signal.data.extend(reference_waveform.x.iter().copied());
    dataset.set_x(x_signal);

    let mut warnings = Vec::new();
    for waveform in &state.simulation.waveforms {
        let mut signal = crate::io::WaveformSignal::new(
            &waveform.name,
            signal_type_from_waveform_name(&waveform.name),
        );

        let available_points = waveform.x.len().min(waveform.y.len());
        if waveform.x.len() != waveform.y.len() {
            warnings.push(format!(
                "Signal '{}' has {} x samples and {} y samples; exported {} aligned samples.",
                waveform.name,
                waveform.x.len(),
                waveform.y.len(),
                available_points
            ));
        }

        let export_points = available_points.min(reference_len);
        if available_points > reference_len {
            warnings.push(format!(
                "Signal '{}' has {} samples, exceeding shared x-axis length {}; truncated.",
                waveform.name, available_points, reference_len
            ));
        }

        signal
            .data
            .extend(waveform.y.iter().take(export_points).copied());
        dataset.add_signal(signal);
    }

    Ok(PreparedWaveformDataset { dataset, warnings })
}

fn detect_x_axis_signal(state: &AppState) -> (String, crate::io::SignalType) {
    let analysis_type = state
        .simulation
        .active_run_idx
        .and_then(|run_idx| state.simulation.runs.get(run_idx))
        .and_then(|run| {
            state
                .simulation
                .active_analysis_idx
                .and_then(|analysis_idx| run.analyses.get(analysis_idx))
        })
        .map(|analysis| analysis.analysis_type)
        .or_else(|| {
            state
                .simulation
                .runs
                .first()
                .and_then(|run| run.analyses.first())
                .map(|analysis| analysis.analysis_type)
        });

    match analysis_type {
        Some(analysis) => {
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
        None => ("time".to_string(), crate::io::SignalType::Time),
    }
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
