use crate::common::app::AppState;
use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

const NO_RESULTS_MESSAGE: &str = "No simulation results to export. Run a simulation first.";
const NO_SAMPLES_MESSAGE: &str = "No waveform samples available to export.";

pub(crate) fn action_export_csv_with_io(
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
        Ok(Some(mut path)) => {
            crate::common::file_actions::ensure_file_extension(&mut path, "csv");

            match io.write_waveform_csv(&prepared.dataset, &path) {
                Ok(()) => {
                    let detail = format!(
                        "{} signals, {} points",
                        prepared.dataset.signal_count(),
                        prepared.dataset.point_count()
                    );
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                        "{}",
                        crate::common::export_workflow::export_completion_message(
                            "CSV",
                            &path,
                            Some(detail),
                            io
                        )
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
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "CSV export failed: {}",
                e
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

fn prepare_waveform_dataset(state: &AppState) -> Result<PreparedWaveformDataset, String> {
    if let Some(run) = state.simulation.active_run() {
        let analyses: Vec<_> = run
            .analyses
            .iter()
            .filter(|analysis| !analysis.waveforms.is_empty())
            .collect();

        return match analyses.as_slice() {
            [] => Err(NO_SAMPLES_MESSAGE.to_string()),
            [analysis] => prepare_single_analysis_dataset(analysis),
            _ => prepare_multi_analysis_dataset(&analyses),
        };
    }

    prepare_selected_waveform_dataset(state)
}

fn prepare_selected_waveform_dataset(state: &AppState) -> Result<PreparedWaveformDataset, String> {
    if state.simulation.waveforms.is_empty() {
        return Err(NO_RESULTS_MESSAGE.to_string());
    }

    let (x_name, x_signal_type) = detect_x_axis_signal(state);
    prepare_flat_waveform_dataset(&state.simulation.waveforms, x_name, x_signal_type)
}

fn prepare_single_analysis_dataset(
    analysis: &crate::state::AnalysisResult,
) -> Result<PreparedWaveformDataset, String> {
    let (x_name, x_signal_type) = axis_signal_for_analysis_type(Some(analysis.analysis_type));
    prepare_flat_waveform_dataset(&analysis.waveforms, x_name, x_signal_type)
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
            None,
            &waveform.name,
            waveform,
            reference_len,
        );
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

fn prepare_multi_analysis_dataset(
    analyses: &[&crate::state::AnalysisResult],
) -> Result<PreparedWaveformDataset, String> {
    let mut dataset = crate::io::WaveformDataset::new("Simulation Results");
    let max_points = analyses
        .iter()
        .flat_map(|analysis| analysis.waveforms.iter())
        .map(|waveform| waveform.x.len().max(waveform.y.len()))
        .max()
        .ok_or_else(|| NO_SAMPLES_MESSAGE.to_string())?;

    if max_points == 0 {
        return Err(NO_SAMPLES_MESSAGE.to_string());
    }

    let mut sample = crate::io::WaveformSignal::new("sample", crate::io::SignalType::Unknown);
    sample.data.extend((0..max_points).map(|idx| idx as f64));
    dataset.set_x(sample);

    let mut warnings = Vec::new();
    let prefixes = unique_analysis_prefixes(analyses);
    for (analysis, prefix) in analyses.iter().zip(prefixes.iter()) {
        let reference_waveform = analysis
            .waveforms
            .iter()
            .filter(|waveform| !waveform.x.is_empty())
            .max_by_key(|waveform| waveform.x.len());
        let Some(reference_waveform) = reference_waveform else {
            warnings.push(format!(
                "{} has no x-axis samples; skipped from CSV export.",
                prefix
            ));
            continue;
        };

        let (axis_name, axis_type) = axis_signal_for_analysis_type(Some(analysis.analysis_type));
        validate_shared_x_axis(&analysis.waveforms, reference_waveform.x.as_ref())
            .map_err(|message| format!("{}: {}", prefix, message))?;
        let mut axis_signal =
            crate::io::WaveformSignal::new(qualified_column_name(prefix, &axis_name), axis_type);
        axis_signal
            .data
            .extend(reference_waveform.x.iter().copied());
        dataset.add_signal(axis_signal);

        let reference_len = reference_waveform.x.len();
        for waveform in &analysis.waveforms {
            append_waveform_signal(
                &mut dataset,
                &mut warnings,
                Some(prefix),
                &waveform.name,
                waveform,
                reference_len,
            );
        }
    }

    if dataset.signals.is_empty() {
        return Err(NO_SAMPLES_MESSAGE.to_string());
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

    axis_signal_for_analysis_type(analysis_type)
}

fn axis_signal_for_analysis_type(
    analysis_type: Option<crate::state::AnalysisType>,
) -> (String, crate::io::SignalType) {
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

fn append_waveform_signal(
    dataset: &mut crate::io::WaveformDataset,
    warnings: &mut Vec<String>,
    prefix: Option<&str>,
    signal_name: &str,
    waveform: &crate::state::WaveformData,
    reference_len: usize,
) {
    append_signal_values(
        dataset,
        warnings,
        prefix,
        ExportSignalSlice {
            name: signal_name,
            signal_type: signal_type_from_waveform_name(signal_name),
            x_values: waveform.x.as_ref(),
            y_values: waveform.y.as_ref(),
        },
        reference_len,
    );

    if let Some(complex) = &waveform.complex {
        let real_name = format!("re({})", complex.source_name);
        append_signal_values(
            dataset,
            warnings,
            prefix,
            ExportSignalSlice {
                name: &real_name,
                signal_type: complex_signal_type(&complex.source_name, true),
                x_values: waveform.x.as_ref(),
                y_values: complex.real.as_ref(),
            },
            reference_len,
        );
        let imag_name = format!("im({})", complex.source_name);
        append_signal_values(
            dataset,
            warnings,
            prefix,
            ExportSignalSlice {
                name: &imag_name,
                signal_type: complex_signal_type(&complex.source_name, false),
                x_values: waveform.x.as_ref(),
                y_values: complex.imag.as_ref(),
            },
            reference_len,
        );
    }
}

fn append_signal_values(
    dataset: &mut crate::io::WaveformDataset,
    warnings: &mut Vec<String>,
    prefix: Option<&str>,
    signal: ExportSignalSlice<'_>,
    reference_len: usize,
) {
    let export_name = prefix
        .map(|prefix| qualified_column_name(prefix, signal.name))
        .unwrap_or_else(|| sanitize_column_label(signal.name));
    let mut export_signal = crate::io::WaveformSignal::new(&export_name, signal.signal_type);

    let available_points = signal.x_values.len().min(signal.y_values.len());
    if signal.x_values.len() != signal.y_values.len() {
        warnings.push(format!(
            "Signal '{}' has {} x samples and {} y samples; exported {} aligned samples.",
            export_name,
            signal.x_values.len(),
            signal.y_values.len(),
            available_points
        ));
    }

    let export_points = available_points.min(reference_len);
    if available_points > reference_len {
        warnings.push(format!(
            "Signal '{}' has {} samples, exceeding shared x-axis length {}; truncated.",
            export_name, available_points, reference_len
        ));
    }

    export_signal
        .data
        .extend(signal.y_values.iter().take(export_points).copied());
    dataset.add_signal(export_signal);
}

fn unique_analysis_prefixes(analyses: &[&crate::state::AnalysisResult]) -> Vec<String> {
    use std::collections::HashMap;

    let mut seen = HashMap::<String, usize>::new();
    let mut prefixes = Vec::with_capacity(analyses.len());
    for analysis in analyses {
        let mut base = sanitize_column_label(if analysis.label.trim().is_empty() {
            analysis.analysis_type.display_name()
        } else {
            analysis.label.trim()
        });
        if base.is_empty() {
            base = analysis.analysis_type.display_name().to_string();
        }

        let count = seen.entry(base.clone()).or_insert(0);
        *count += 1;
        if *count == 1 {
            prefixes.push(base);
        } else {
            prefixes.push(format!("{} {}", base, count));
        }
    }
    prefixes
}

fn qualified_column_name(prefix: &str, signal_name: &str) -> String {
    format!(
        "{}/{}",
        sanitize_column_label(prefix),
        sanitize_column_label(signal_name)
    )
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

    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, WaveformData};

    #[derive(Debug)]
    struct MockExportWorkflowIo {
        datasets: RefCell<Vec<crate::io::WaveformDataset>>,
        paths: RefCell<Vec<PathBuf>>,
        saved_paths_are_reopenable: bool,
    }

    impl Default for MockExportWorkflowIo {
        fn default() -> Self {
            Self {
                datasets: RefCell::default(),
                paths: RefCell::default(),
                saved_paths_are_reopenable: true,
            }
        }
    }

    impl ExportWorkflowIo for MockExportWorkflowIo {
        fn show_save_dialog(
            &self,
            _config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            Ok(Some(PathBuf::from("waveforms.csv")))
        }

        fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
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

    #[test]
    fn csv_export_includes_every_plottable_analysis_in_active_run() {
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
        state.simulation.active_analysis_idx = Some(0);
        state
            .simulation
            .replace_waveforms(transient.waveforms.clone());

        let io = MockExportWorkflowIo::default();
        action_export_csv_with_io(&mut state, &io);

        let datasets = io.datasets.borrow();
        assert_eq!(datasets.len(), 1);
        let dataset = &datasets[0];
        assert_eq!(
            dataset.x_signal.as_ref().map(|signal| signal.name.as_str()),
            Some("sample")
        );
        assert_eq!(
            dataset.signal_names(),
            vec![
                "Transient/time",
                "Transient/V(out)",
                "AC Analysis/frequency",
                "AC Analysis/|V(out)|",
            ]
        );
        assert_eq!(dataset.point_count(), 3);
        assert_eq!(
            dataset
                .get_signal("Transient/time")
                .map(|signal| signal.data.as_slice()),
            Some(&[0.0, 1.0e-6][..])
        );
        assert_eq!(
            dataset
                .get_signal("AC Analysis/frequency")
                .map(|signal| signal.data.as_slice()),
            Some(&[1.0e3, 1.0e4, 1.0e5][..])
        );
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
    fn csv_export_rejects_multi_analysis_divergent_x_axes() {
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
        state.simulation.active_analysis_idx = Some(0);

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
