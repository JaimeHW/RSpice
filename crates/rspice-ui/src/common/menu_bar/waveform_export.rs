use crate::common::app::AppState;
use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

const NO_RESULTS_MESSAGE: &str = "No simulation results to export. Run a simulation first.";
const NO_SAMPLES_MESSAGE: &str = "No waveform samples available to export.";

pub(super) fn action_export_csv(state: &mut AppState) {
    let io = crate::common::export_workflow::NativeExportWorkflowIo;
    action_export_csv_with_io(state, &io);
}

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
            super::file_actions::ensure_file_extension(&mut path, "csv");

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct SaveDialogConfigSnapshot {
        title: String,
        default_name: String,
        filter_name: String,
        filter_extensions: Vec<String>,
    }

    #[derive(Default)]
    struct MockWaveformExportIo {
        save_dialog_results: RefCell<VecDeque<Option<PathBuf>>>,
        write_csv_results: RefCell<VecDeque<Result<(), String>>>,
        save_dialog_calls: Cell<usize>,
        write_csv_calls: Cell<usize>,
        last_save_dialog_config: RefCell<Option<SaveDialogConfigSnapshot>>,
        last_write_csv_path: RefCell<Option<PathBuf>>,
        last_written_dataset: RefCell<Option<crate::io::WaveformDataset>>,
    }

    impl MockWaveformExportIo {
        fn push_save_dialog_result(&self, result: Option<PathBuf>) {
            self.save_dialog_results.borrow_mut().push_back(result);
        }

        fn push_write_csv_result(&self, result: Result<(), String>) {
            self.write_csv_results.borrow_mut().push_back(result);
        }

        fn save_dialog_calls(&self) -> usize {
            self.save_dialog_calls.get()
        }

        fn write_csv_calls(&self) -> usize {
            self.write_csv_calls.get()
        }

        fn last_save_dialog_config(&self) -> Option<SaveDialogConfigSnapshot> {
            self.last_save_dialog_config.borrow().clone()
        }

        fn last_write_csv_path(&self) -> Option<PathBuf> {
            self.last_write_csv_path.borrow().clone()
        }

        fn last_written_dataset(&self) -> Option<crate::io::WaveformDataset> {
            self.last_written_dataset.borrow().clone()
        }
    }

    impl crate::common::export_workflow::ExportWorkflowIo for Rc<MockWaveformExportIo> {
        fn show_save_dialog(
            &self,
            config: crate::common::export_workflow::SaveDialogConfig<'_>,
        ) -> Option<PathBuf> {
            self.save_dialog_calls
                .set(self.save_dialog_calls.get().saturating_add(1));
            *self.last_save_dialog_config.borrow_mut() = Some(SaveDialogConfigSnapshot {
                title: config.title.to_string(),
                default_name: config.default_name.to_string(),
                filter_name: config.filter_name.to_string(),
                filter_extensions: config
                    .filter_extensions
                    .iter()
                    .map(|ext| (*ext).to_string())
                    .collect(),
            });
            self.save_dialog_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide show_save_dialog result")
        }

        fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
            Err("unexpected write_text_file call in waveform export test".to_string())
        }

        fn write_waveform_csv(
            &self,
            dataset: &crate::io::WaveformDataset,
            path: &Path,
        ) -> Result<(), String> {
            self.write_csv_calls
                .set(self.write_csv_calls.get().saturating_add(1));
            *self.last_write_csv_path.borrow_mut() = Some(path.to_path_buf());
            *self.last_written_dataset.borrow_mut() = Some(dataset.clone());
            self.write_csv_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide write_waveform_csv result")
        }
    }

    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, WaveformData};

    fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> WaveformData {
        WaveformData::new(name, x, y, "#FFFFFF")
    }

    #[test]
    fn test_signal_type_from_waveform_name_detects_voltage_current_and_unknown() {
        assert_eq!(
            signal_type_from_waveform_name("V(out)"),
            crate::io::SignalType::Voltage
        );
        assert_eq!(
            signal_type_from_waveform_name("i(vdd)"),
            crate::io::SignalType::Current
        );
        assert_eq!(
            signal_type_from_waveform_name("gain"),
            crate::io::SignalType::Unknown
        );
    }

    #[test]
    fn test_detect_x_axis_signal_defaults_to_time() {
        let state = AppState::default();
        assert_eq!(
            detect_x_axis_signal(&state),
            ("time".to_string(), crate::io::SignalType::Time)
        );
    }

    #[test]
    fn test_detect_x_axis_signal_uses_active_analysis_metadata() {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC Analysis"));
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        assert_eq!(
            detect_x_axis_signal(&state),
            ("frequency".to_string(), crate::io::SignalType::Frequency)
        );
    }

    #[test]
    fn test_prepare_waveform_dataset_rejects_empty_waveforms() {
        let state = AppState::default();
        let result = prepare_waveform_dataset(&state);
        assert!(matches!(result, Err(ref message) if message == NO_RESULTS_MESSAGE));
    }

    #[test]
    fn test_prepare_waveform_dataset_uses_longest_x_axis_as_reference() {
        let mut state = AppState::default();
        state
            .simulation
            .waveforms
            .push(waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0]));
        state.simulation.waveforms.push(waveform(
            "I(V1)",
            vec![0.0, 1.0, 2.0],
            vec![0.1, 0.2, 0.3],
        ));

        let prepared = prepare_waveform_dataset(&state).expect("dataset should prepare");
        let x_signal = prepared
            .dataset
            .x_signal
            .as_ref()
            .expect("x signal should exist");
        assert_eq!(x_signal.name, "time");
        assert_eq!(prepared.dataset.point_count(), 3);
        assert_eq!(x_signal.data, vec![0.0, 1.0, 2.0]);
        assert_eq!(prepared.dataset.signal_count(), 2);
        assert!(prepared.warnings.is_empty());
    }

    #[test]
    fn test_prepare_waveform_dataset_warns_on_xy_length_mismatch() {
        let mut state = AppState::default();
        state
            .simulation
            .waveforms
            .push(waveform("V(out)", vec![0.0, 1.0, 2.0], vec![1.0, 2.0]));

        let prepared = prepare_waveform_dataset(&state).expect("dataset should prepare");
        assert_eq!(prepared.dataset.signal_count(), 1);
        assert_eq!(prepared.dataset.signals[0].data, vec![1.0, 2.0]);
        assert_eq!(prepared.warnings.len(), 1);
        assert!(prepared.warnings[0].contains("V(out)"));
        assert!(prepared.warnings[0].contains("x samples"));
    }

    #[test]
    fn test_action_export_csv_with_io_warns_when_no_results_and_skips_dialog() {
        let io = Rc::new(MockWaveformExportIo::default());
        let mut state = AppState::default();

        action_export_csv_with_io(&mut state, &io);

        assert_eq!(io.save_dialog_calls(), 0);
        assert_eq!(io.write_csv_calls(), 0);
        assert!(
            state
                .console_messages
                .iter()
                .any(|message| message.message == NO_RESULTS_MESSAGE),
            "missing-waveform export should produce a direct warning"
        );
    }

    #[test]
    fn test_action_export_csv_with_io_cancelled_dialog_skips_write() {
        let io = Rc::new(MockWaveformExportIo::default());
        io.push_save_dialog_result(None);

        let mut state = AppState::default();
        state
            .simulation
            .waveforms
            .push(waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0]));
        let baseline_messages = state.console_messages.len();

        action_export_csv_with_io(&mut state, &io);

        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.write_csv_calls(), 0);
        assert_eq!(state.console_messages.len(), baseline_messages);
    }

    #[test]
    fn test_action_export_csv_with_io_success_writes_csv_and_logs_info() {
        let io = Rc::new(MockWaveformExportIo::default());
        io.push_save_dialog_result(Some(PathBuf::from("results/sweep")));
        io.push_write_csv_result(Ok(()));

        let mut state = AppState::default();
        state.simulation.waveforms.push(waveform(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![1.0, 2.0, 3.0],
        ));
        state.simulation.waveforms.push(waveform(
            "I(V1)",
            vec![0.0, 1.0, 2.0],
            vec![0.1, 0.2, 0.3],
        ));

        action_export_csv_with_io(&mut state, &io);

        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.write_csv_calls(), 1);
        assert_eq!(
            io.last_save_dialog_config(),
            Some(SaveDialogConfigSnapshot {
                title: "Export Waveform CSV".to_string(),
                default_name: "waveforms.csv".to_string(),
                filter_name: "CSV Files".to_string(),
                filter_extensions: vec!["csv".to_string()],
            })
        );
        assert_eq!(
            io.last_write_csv_path(),
            Some(PathBuf::from("results/sweep.csv"))
        );
        let dataset = io
            .last_written_dataset()
            .expect("written dataset should be captured");
        assert_eq!(dataset.signal_count(), 2);
        assert_eq!(dataset.point_count(), 3);
        assert!(
            state
                .console_messages
                .iter()
                .any(|message| message.message.contains("Exported CSV:")),
            "successful csv export should log an informational summary"
        );
    }

    #[test]
    fn test_action_export_csv_with_io_write_error_logs_failure() {
        let io = Rc::new(MockWaveformExportIo::default());
        io.push_save_dialog_result(Some(PathBuf::from("results/fail.csv")));
        io.push_write_csv_result(Err("permission denied".to_string()));

        let mut state = AppState::default();
        state
            .simulation
            .waveforms
            .push(waveform("V(out)", vec![0.0, 1.0], vec![1.0, 2.0]));

        action_export_csv_with_io(&mut state, &io);

        assert_eq!(io.write_csv_calls(), 1);
        assert!(
            state.console_messages.iter().any(|message| message
                .message
                .contains("CSV export failed: permission denied")),
            "csv export write errors should be surfaced"
        );
    }

    #[test]
    fn test_action_export_csv_with_io_emits_dataset_warnings_before_export() {
        let io = Rc::new(MockWaveformExportIo::default());
        io.push_save_dialog_result(Some(PathBuf::from("results/warn.csv")));
        io.push_write_csv_result(Ok(()));

        let mut state = AppState::default();
        state
            .simulation
            .waveforms
            .push(waveform("V(out)", vec![0.0, 1.0, 2.0], vec![1.0, 2.0]));
        state
            .simulation
            .waveforms
            .push(waveform("I(V1)", vec![0.0, 1.0], vec![0.1, 0.2]));

        action_export_csv_with_io(&mut state, &io);

        let warning_messages: Vec<&str> = state
            .console_messages
            .iter()
            .filter(|message| message.level == crate::common::app::ConsoleLevel::Warning)
            .map(|message| message.message.as_str())
            .collect();
        assert!(
            warning_messages
                .iter()
                .any(|message| message.contains("V(out)") && message.contains("x samples")),
            "xy mismatch warnings should be surfaced before export"
        );
    }
}
