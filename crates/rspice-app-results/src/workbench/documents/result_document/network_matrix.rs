//! Exact, frequency-indexed network matrices from retained power-wave data.
//!
//! A periodic sideband block is kept separate from every other block. Missing
//! cells, ambiguous names, and differing grids cannot become zero coefficients.

use rspice_results_ui::presentation::well_hint;
use std::collections::HashSet;
use std::sync::Arc;

use egui::Ui;
#[cfg(test)]
use rspice_results::network_matrix::channel_reference;
use rspice_results::network_matrix::forms::NetworkForm;
use rspice_results::network_matrix::{NetworkLayout, resolve};
use rspice_results_ui::network_matrix as view;
use rspice_vocabulary::product::DatasetId;
#[cfg(test)]
use view::{formatted, magnitude_db};

use super::{AnalysisPresentationKey, ResultsReads, SheetContext};
use crate::results::visualization_document::{
    NetworkMatrixForm, NetworkMatrixFormat, SheetSettingValue,
};
use crate::state::AnalysisResult;
#[cfg(test)]
use crate::state::AnalysisType;

#[derive(Debug, Clone, Default)]
pub(super) struct NetworkMatrixState {
    source: Option<(
        AnalysisPresentationKey,
        crate::state::RunHistoryRevision,
        u64,
    )>,
    controls: view::NetworkMatrixControls,
    open_trace: bool,
    layout: Option<Arc<NetworkLayout>>,
}

impl NetworkMatrixState {
    /// Whether the polar window over the selected coefficient is open.
    pub(super) fn polar_open(&self) -> bool {
        self.open_trace
    }

    /// Release a discarded source even while its sheet is inactive.
    pub(super) fn retain_datasets(&mut self, retained: &HashSet<DatasetId>) {
        if self
            .source
            .as_ref()
            .is_some_and(|(analysis, _, _)| !retained.contains(&analysis.dataset_id()))
        {
            self.source = None;
            self.controls.reset_source();
            self.open_trace = false;
            self.layout = None;
        }
    }

    fn bind(&mut self, key: AnalysisPresentationKey, simulation: &crate::state::SimulationState) {
        let source = (
            key,
            simulation.retained.runs.revision(),
            simulation.view.data_version,
        );
        if self.source.as_ref() != Some(&source) {
            self.source = Some(source);
            self.controls.reset_source();
            self.open_trace = false;
            self.layout = None;
        }
    }
}

pub(super) fn structure_is_renderable(analysis: &AnalysisResult) -> bool {
    rspice_results::renderability::network_matrix_structure_is_renderable(
        analysis,
        super::frame_work::note_scan,
    )
}

/// What the tab strip says about this sheet, in the sheet's own words.
///
/// The structural verdict comes from the workspace memo rather than a fresh
/// walk of every retained coefficient.
pub(super) fn availability(state: ResultsReads<'_>) -> super::ViewerAvailability {
    if state.model.simulation.active_run().is_some_and(|run| {
        state
            .model
            .simulation
            .active_analysis()
            .is_some_and(|analysis| {
                super::analysis_answers_structural_gate(
                    state,
                    run.dataset_id,
                    analysis,
                    super::StructuralGate::NetworkMatrix,
                ) && super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
            })
    }) {
        super::ViewerAvailability::available(
            "Complete retained network matrices and port references are available",
        )
    } else {
        super::ViewerAvailability::unavailable(
            "Requires complete SP, PSP, or HBSP complex matrices on a shared frequency grid with port references",
        )
    }
}

/// The active analysis and its resolved matrix, once the matrix qualified for
/// this source generation; otherwise the sentence the well states, if any.
///
/// The bar and the body both enter here, so either one binds the source and
/// resolves the layout once per retained generation.
fn bound<'a>(
    context: &mut SheetContext<'a>,
) -> Result<(&'a AnalysisResult, Arc<NetworkLayout>), Option<&'static str>> {
    let simulation = context.simulation;
    let Some(run) = simulation.active_run() else {
        return Err(Some("Run a network analysis to inspect its matrix"));
    };
    let Some(analysis) = simulation.active_analysis() else {
        return Err(None);
    };
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let valid = context.results.structural_gates.get_or_insert_with(
        context.simulation,
        (key, super::StructuralGate::NetworkMatrix),
        || structure_is_renderable(analysis),
    ) && context
        .results
        .retained_evidence_validity
        .get_or_insert_with(context.simulation, key, || {
            super::frame_work::note(super::frame_work::DatasetWalk::EvidenceValidation);
            analysis.validate_retained_evidence().map_err(Into::into)
        })
        .is_ok();
    if !valid {
        return Err(Some(
            "The saved network matrix is incomplete or has invalid coefficient or frequency data",
        ));
    }
    let controls = &mut context.results.network_matrix;
    controls.bind(key, simulation);
    // Matrix topology can be large even when only a few rows are visible.
    // Resolve names and block membership once per retained source generation;
    // subsequent frames borrow its index inventory and only read visible cells.
    if controls.layout.is_none() {
        let Some(matrix) = resolve(analysis) else {
            return Err(Some(
                "Requires a complete complex network matrix and its physical port references",
            ));
        };
        controls.layout = Some(Arc::new(matrix));
    }
    Ok((
        analysis,
        controls.layout.as_ref().expect("resolved layout").clone(),
    ))
}

/// The frequency grid of the block the sheet shows, once it has arrived.
fn shown_grid(
    analysis: &AnalysisResult,
    matrix: &NetworkLayout,
    block: usize,
) -> Option<rspice_results::waveform::HeldColumn> {
    let block = matrix.blocks().get(block)?;
    analysis.waveforms[block.cells()[0]].x.fetch_whole().ready()
}

/// The readout pin that keeps the matrix's frequency in the run's view: the
/// authored analysis shown and this sheet.
fn pin_key(
    context: &SheetContext<'_>,
    analysis: &AnalysisResult,
) -> Option<(
    rspice_results::result_presentation::AnalysisPresentationSource,
    super::ResultViewer,
)> {
    let run = context.simulation.active_run()?;
    Some((
        AnalysisPresentationKey::new(run.dataset_id, analysis).authored(),
        super::ResultViewer::NetworkMatrix,
    ))
}

/// Show the frequency the view keeps, at the nearest retained sample.
fn follow_pin(context: &mut SheetContext<'_>, analysis: &AnalysisResult, grid: &[f64]) {
    let Some(key) = pin_key(context, analysis) else {
        return;
    };
    if let Some(&frequency) = context.results.session.readout_pins.get(&key) {
        context.results.network_matrix.controls.sample = view::nearest_sample(grid, frequency);
    }
}

/// Keep the frequency the reader chose in the view.
fn keep_pin(context: &mut SheetContext<'_>, analysis: &AnalysisResult, frequency: f64) {
    if let Some(key) = pin_key(context, analysis) {
        context.results.session.readout_pins.insert(key, frequency);
    }
}

/// The form and format the view keeps, as the controls hold them.
fn reading_of(setting: &SheetSettingValue) -> Option<(NetworkForm, usize)> {
    let SheetSettingValue::NetworkMatrix { form, format } = setting else {
        return None;
    };
    let form = match form {
        NetworkMatrixForm::S => NetworkForm::S,
        NetworkMatrixForm::Y => NetworkForm::Y,
        NetworkMatrixForm::Z => NetworkForm::Z,
        NetworkMatrixForm::H => NetworkForm::H,
        NetworkMatrixForm::Abcd => NetworkForm::Abcd,
    };
    let format = match format {
        NetworkMatrixFormat::RealImaginary => 0,
        NetworkMatrixFormat::MagnitudeAngle => 1,
        NetworkMatrixFormat::DecibelAngle => 2,
    };
    Some((form, format))
}

/// The controls' form and format as the view keeps them.
fn setting_of(form: NetworkForm, format: usize) -> SheetSettingValue {
    SheetSettingValue::NetworkMatrix {
        form: match form {
            NetworkForm::S => NetworkMatrixForm::S,
            NetworkForm::Y => NetworkMatrixForm::Y,
            NetworkForm::Z => NetworkMatrixForm::Z,
            NetworkForm::H => NetworkMatrixForm::H,
            NetworkForm::Abcd => NetworkMatrixForm::Abcd,
        },
        format: match format {
            1 => NetworkMatrixFormat::MagnitudeAngle,
            2 => NetworkMatrixFormat::DecibelAngle,
            _ => NetworkMatrixFormat::RealImaginary,
        },
    }
}

/// Show the form and format the view keeps.
fn follow_reading(context: &mut SheetContext<'_>, analysis: &AnalysisResult) {
    let Some((source, sheet)) = pin_key(context, analysis) else {
        return;
    };
    let kept = context
        .results
        .session
        .sheet_settings
        .get(&(source, sheet))
        .and_then(reading_of);
    let controls = &mut context.results.network_matrix.controls;
    (controls.form, controls.representation) = kept.unwrap_or((NetworkForm::S, 0));
}

/// Keep the form and format the reader chose in the view.
fn keep_reading(context: &mut SheetContext<'_>, analysis: &AnalysisResult) {
    let Some(key) = pin_key(context, analysis) else {
        return;
    };
    let controls = &context.results.network_matrix.controls;
    let setting = setting_of(controls.form, controls.representation);
    context.results.session.sheet_settings.insert(key, setting);
}

/// The matrix's own controls in its sheet bar. The frequency, the form and
/// the format the reader chooses are kept in the run's view, so they survive
/// a re-run and a restart, and undo steps them back.
pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let Ok((analysis, matrix)) = bound(context) else {
        return false;
    };
    let block = context.results.network_matrix.controls.block;
    let grid = shown_grid(analysis, &matrix, block);
    if let Some(grid) = &grid {
        follow_pin(context, analysis, grid);
    }
    follow_reading(context, analysis);
    let reading = {
        let controls = &context.results.network_matrix.controls;
        (controls.form, controls.representation)
    };
    let before = context.results.network_matrix.controls.sample;
    view::domain_bar(
        ui,
        analysis,
        &matrix,
        &mut context.results.network_matrix.controls,
        context.significant_digits,
    );
    let after = context.results.network_matrix.controls.sample;
    if after != before
        && let Some(&frequency) = grid.as_ref().and_then(|grid| grid.get(after))
    {
        keep_pin(context, analysis, frequency);
    }
    let controls = &context.results.network_matrix.controls;
    if (controls.form, controls.representation) != reading {
        keep_reading(context, analysis);
    }
    true
}

pub(super) fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let (analysis, matrix) = match bound(context) {
        Ok(bound) => bound,
        Err(hint) => {
            if let Some(hint) = hint {
                well_hint(ui, hint);
            }
            return;
        }
    };
    let controls = &mut context.results.network_matrix;
    if let Some(coefficient) = view::show(
        ui,
        analysis,
        &matrix,
        &mut controls.controls,
        context.policy.angle_display,
        context.significant_digits,
    ) {
        context.results.session.polar.quantity = Some(coefficient.waveform_name);
        context.results.session.cursors.a = Some(coefficient.frequency);
        context.results.session.cursors.b = None;
        controls.open_trace = true;
    }
    let mut open_trace = controls.open_trace;
    if open_trace {
        let available =
            (ui.ctx().content_rect().size() - egui::vec2(24.0, 24.0)).max(egui::vec2(280.0, 280.0));
        egui::Window::new("Network coefficient · Polar")
            .id(ui.id().with("network-matrix-polar"))
            .open(&mut open_trace)
            .default_size(egui::vec2(600.0, 520.0).min(available))
            .max_size(available)
            .show(ui.ctx(), |ui| {
                ui.horizontal_wrapped(|ui| {
                    super::polar::domain_bar(ui, context);
                });
                ui.collapsing("Cursor measurements", |ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, true])
                        .max_height(200.0)
                        .show(ui, |ui| super::polar::right_panel(ui, context));
                });
                super::polar::show(ui, context);
            });
    }
    context.results.network_matrix.open_trace = open_trace;
}

/// The matrix the sheet shows, in its words, for a printed page; nothing
/// while the sheet holds no matrix of the active analysis.
pub(super) fn printed(state: ResultsReads<'_>) -> rspice_results_ui::table::SheetPrint {
    use rspice_results_ui::table::SheetPrint;
    let simulation = &state.model.simulation;
    let (Some(analysis), Some(layout)) = (
        shown_matrix(state.results, simulation),
        state.results.network_matrix.layout.as_ref(),
    ) else {
        return SheetPrint::default();
    };
    view::printed_matrix(
        analysis,
        layout,
        &state.results.network_matrix.controls,
        state
            .model
            .preferences
            .quantity_presentation_policy()
            .angle_display,
        usize::from(
            state
                .presentation_policy()
                .displayed_significant_digits()
                .get(),
        ),
    )
    .map_or_else(SheetPrint::default, |(title, table)| {
        SheetPrint::table(Some(analysis.id), title, table)
    })
}

/// The active analysis, while the sheet holds its matrix as it was last
/// shown.
fn shown_matrix<'a>(
    results: &super::ResultsState,
    simulation: &'a crate::state::SimulationState,
) -> Option<&'a AnalysisResult> {
    let analysis = simulation.active_analysis()?;
    let key = AnalysisPresentationKey::new(simulation.active_run()?.dataset_id, analysis);
    let source = (
        key,
        simulation.retained.runs.revision(),
        simulation.view.data_version,
    );
    (results.network_matrix.source.as_ref() == Some(&source)
        && results.network_matrix.layout.is_some())
    .then_some(analysis)
}

/// Whether the sheet holds a matrix to print, without reading its words.
pub(super) fn is_printable(
    results: &super::ResultsState,
    simulation: &crate::state::SimulationState,
) -> bool {
    shown_matrix(results, simulation).is_some()
}

/// The exact matrix the sheet shows, as the clipboard takes it: the block and
/// the frequency sample on screen, every coefficient's parts at full
/// precision. `None` while the sheet holds no matrix of the active analysis
/// or its samples have not arrived.
pub(crate) fn exact_values(state: ResultsReads<'_>) -> Option<String> {
    let sheet = &state.results.network_matrix;
    let simulation = &state.model.simulation;
    let analysis = simulation.active_analysis()?;
    let key = AnalysisPresentationKey::new(simulation.active_run()?.dataset_id, analysis);
    let source = (
        key,
        simulation.retained.runs.revision(),
        simulation.view.data_version,
    );
    if sheet.source.as_ref() != Some(&source) {
        return None;
    }
    let table = sheet.layout.as_ref()?.exact_form_table(
        analysis,
        sheet.controls.block,
        sheet.controls.sample,
        sheet.controls.form,
    )?;
    rspice_formats::table::csv_to_tsv(&rspice_formats::result_csv::encode_network_matrix_csv(
        &table,
    ))
    .ok()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::state::{AnalysisResultFamilyMetadata, SimulationRun};
    use crate::workbench::documents::result_document::test_fixtures::network_matrix_analysis;
    use rspice_results::network_matrix::forms as view_form;

    #[test]
    fn complete_physical_and_mixed_matrices_have_exact_references_and_values() {
        for mixed in [false, true] {
            let analysis = network_matrix_analysis(mixed);
            assert!(structure_is_renderable(&analysis));
            let matrix = resolve(&analysis).unwrap();
            let csv = rspice_formats::result_csv::encode_network_matrix_csv(
                &matrix.exact_table(&analysis, 0, 1).unwrap(),
            );
            assert!(csv.contains("2.00000000000000000e6"));
            assert_eq!(
                channel_reference(0, matrix.references(), mixed),
                if mixed { 100.0 } else { 50.0 }
            );
            if mixed {
                assert_eq!(channel_reference(2, matrix.references(), mixed), 25.0);
            }
        }
    }

    /// A passivity verdict for the first block's first sample.
    fn first_sample_verdict(line: &str) -> view::NetworkDiagnostic {
        view::NetworkDiagnostic {
            block: 0,
            sample: 0,
            line: line.to_owned(),
            detail: String::new(),
        }
    }

    #[test]
    fn replacing_evidence_cannot_reuse_a_sampled_diagnostic() {
        let analysis = network_matrix_analysis(false);
        let mut simulation = crate::state::SimulationState::default();
        let mut run = SimulationRun::new(1);
        let key = AnalysisPresentationKey::new(run.dataset_id, &analysis);
        run.add_analysis(analysis);
        simulation.retained.runs.push(run);
        let mut controls = NetworkMatrixState::default();
        controls.bind(key, &simulation);
        controls.controls.diagnostic = Some(first_sample_verdict("old evidence"));
        controls.bind(key, &simulation.clone());
        assert!(controls.controls.diagnostic.is_some());
        simulation.view.data_version = simulation.view.data_version.wrapping_add(1);
        controls.bind(key, &simulation);
        assert!(controls.controls.diagnostic.is_none());
        controls.controls.diagnostic = Some(first_sample_verdict("old evidence"));
        simulation.retained.runs[0].analyses[0] = network_matrix_analysis(false);
        controls.bind(key, &simulation);
        assert!(controls.controls.diagnostic.is_none());
    }

    #[test]
    fn absent_duplicate_or_misaligned_coefficients_are_refused() {
        let mut missing = network_matrix_analysis(false);
        missing.waveforms.pop();
        assert!(!structure_is_renderable(&missing));
        let mut duplicate = network_matrix_analysis(false);
        duplicate
            .data
            .waveforms
            .push(duplicate.data.waveforms[0].clone());
        assert!(!structure_is_renderable(&duplicate));
        let mut wrong_grid = network_matrix_analysis(false);
        wrong_grid.waveforms[0].x = vec![1e6, 3e6].into();
        assert!(!structure_is_renderable(&wrong_grid));
    }

    #[test]
    fn translated_blocks_and_large_port_indexes_keep_their_identity() {
        let mut periodic = network_matrix_analysis(false);
        periodic.analysis_type = AnalysisType::Psp;
        let mut second = periodic.waveforms.clone();
        for waveform in &mut second {
            let complex = waveform.complex.as_mut().unwrap();
            complex.source_name.push_str("[k=+1,m=-1]");
            waveform.name = format!("|{}|", complex.source_name);
        }
        periodic.waveforms.extend(second);
        assert!(structure_is_renderable(&periodic));
        assert_eq!(resolve(&periodic).unwrap().blocks().len(), 2);
    }

    #[test]
    fn mixed_mode_needs_complete_equal_reference_pairs() {
        let mut analysis = network_matrix_analysis(true);
        let Some(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
            ..
        }) = &mut analysis.family_metadata
        else {
            unreachable!()
        };
        reference_impedances_ohm[0] = 75.0;
        assert!(!structure_is_renderable(&analysis));
    }

    #[test]
    fn exact_export_round_trips_complex_components() {
        let analysis = network_matrix_analysis(false);
        let matrix = resolve(&analysis).unwrap();
        let table = matrix.exact_table(&analysis, 0, 1).unwrap();
        for (index, row) in table.rows.iter().enumerate() {
            let complex = analysis.waveforms[matrix.blocks()[0].cells()[index]]
                .complex
                .as_ref()
                .unwrap();
            assert_eq!(
                row[6].parse::<f64>().unwrap().to_bits(),
                complex.real.read_whole().unwrap()[1].to_bits()
            );
            assert_eq!(
                row[7].parse::<f64>().unwrap().to_bits(),
                complex.imag.read_whole().unwrap()[1].to_bits()
            );
        }
        use crate::quantity::AngleDisplay;
        assert!(formatted(0.0, 0.0, "", 2, AngleDisplay::Degrees, 7).contains("phase undefined"));
        // The phase reads in the unit Angle display chose, degrees to the
        // hundredth against their number and radians at the reader's four
        // digits; a negative part with a true minus.
        assert_eq!(
            formatted(0.0, 1.0, "", 1, AngleDisplay::Degrees, 4),
            "1.000 ∠ 90.00\u{b0}"
        );
        assert_eq!(
            formatted(0.0, 1.0, "", 1, AngleDisplay::Radians, 4),
            "1.000 ∠ 1.571 rad"
        );
        assert_eq!(
            formatted(0.5, -0.25, "", 0, AngleDisplay::Degrees, 4),
            "0.5000 \u{2212} 0.2500j"
        );
        assert_eq!(
            formatted(0.1, 0.0, "", 2, AngleDisplay::Degrees, 4),
            "\u{2212}20.00 dB ∠ 0.00\u{b0}"
        );
        assert!((magnitude_db(1e308, 1e308) - (6160.0 + 10.0 * 2.0_f64.log10())).abs() < 1e-10);
    }

    fn cell_text_rect(shape: &egui::Shape) -> Option<egui::Rect> {
        match shape {
            egui::Shape::Text(text) if text.galley.text().starts_with("0.2500 ") => {
                Some(shape.visual_bounding_rect())
            }
            egui::Shape::Vec(shapes) => shapes.iter().find_map(cell_text_rect),
            _ => None,
        }
    }

    #[test]
    fn double_clicking_a_matrix_cell_opens_its_retained_coefficient_plot() {
        for width in [1280.0, 768.0] {
            let mut app =
                crate::workbench::documents::result_document::test_host::ResultsTestHost::default();
            let mut run = SimulationRun::new(1);
            run.add_analysis(network_matrix_analysis(true));
            app.model.simulation.retained.runs.push(run);
            assert!(app.model.simulation.select_run(0));
            let before = app
                .model
                .simulation
                .active_analysis()
                .unwrap()
                .result_data_digest();
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut frame = |events: Vec<egui::Event>| {
                ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 900.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |ui| {
                        egui::CentralPanel::default().show(ui, |ui| {
                            show(ui, &mut SheetContext::of(&mut app.results_parts()))
                        });
                    },
                )
            };
            let _ = frame(Vec::new());
            let output = frame(Vec::new());
            let position = output
                .shapes
                .iter()
                .find_map(|shape| cell_text_rect(&shape.shape))
                .expect("visible matrix coefficient")
                .center();
            // A click selects the coefficient; the second click opens it.
            for pressed in [true, false, true, false] {
                let _ = frame(vec![
                    egui::Event::PointerMoved(position),
                    egui::Event::PointerButton {
                        pos: position,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ]);
            }
            let _ = frame(Vec::new());
            assert!(app.results.results.network_matrix.open_trace);
            assert_eq!(
                app.results.results.session.polar.quantity.as_deref(),
                Some("|Sdd11|")
            );
            assert_eq!(
                before,
                app.model
                    .simulation
                    .active_analysis()
                    .unwrap()
                    .result_data_digest()
            );

            let cached =
                Arc::downgrade(app.results.results.network_matrix.layout.as_ref().unwrap());
            let discarded = app.model.simulation.retained.runs[0].dataset_id;
            let version = app.model.simulation.view.data_version;
            let controls = &mut app.results.results.network_matrix.controls;
            controls.representation = 2;
            controls.transpose = true;
            controls.heatmap = false;
            controls.floor_db = -120.0;
            controls.diagnostic = Some(first_sample_verdict("retained diagnostic"));
            app.model
                .simulation
                .retained
                .runs
                .push(SimulationRun::new(2));
            app.results
                .results
                .reconcile_retained_datasets(&app.model.simulation);
            assert!(cached.upgrade().is_some());
            assert!(app.results.results.network_matrix.open_trace);

            // Prune while another viewer is active, without revisiting the matrix.
            app.results.results.session.viewer =
                rspice_results::result_presentation::ResultViewer::Table;
            app.model
                .simulation
                .retained
                .runs
                .retain(|run| run.dataset_id != discarded);
            app.results
                .results
                .reconcile_retained_datasets(&app.model.simulation);
            assert_eq!(app.model.simulation.view.data_version, version);
            assert!(
                cached.upgrade().is_none(),
                "discarding the dataset must release its matrix layout without another matrix frame"
            );
            assert!(!app.results.results.network_matrix.open_trace);
            let controls = &app.results.results.network_matrix.controls;
            assert!(controls.diagnostic.is_none());
            assert_eq!((controls.block, controls.sample), (0, 0));
            assert_eq!(controls.representation, 2);
            assert!(controls.transpose);
            assert!(!controls.heatmap);
            assert_eq!(controls.floor_db, -120.0);
        }
    }

    /// A series impedance Zs(f) = 100 Ω + j·f/10 kHz Ω between two 50 Ω
    /// ports, at 1 MHz and 2 MHz: S11 = S22 = Zs/(Zs + 100), S21 = S12 =
    /// 100/(Zs + 100).
    fn series_two_port() -> AnalysisResult {
        use num_complex::Complex64;
        let frequencies = [1e6, 2e6];
        let impedance = |f: f64| Complex64::new(100.0, f / 1e4);
        let mut traces = Vec::new();
        for (name, reflection) in [("S11", true), ("S12", false), ("S21", false), ("S22", true)] {
            let values = frequencies
                .iter()
                .map(|&f| {
                    let z = impedance(f);
                    if reflection {
                        z / (z + 100.0)
                    } else {
                        Complex64::new(100.0, 0.0) / (z + 100.0)
                    }
                })
                .collect::<Vec<_>>();
            traces.push(
                crate::state::RetainedTrace::new(
                    format!("|{name}|"),
                    frequencies.to_vec(),
                    values.iter().map(|value| value.norm()).collect::<Vec<_>>(),
                    "#00aaff",
                )
                .with_complex_components(
                    name,
                    values.iter().map(|value| value.re).collect::<Vec<_>>(),
                    values.iter().map(|value| value.im).collect::<Vec<_>>(),
                ),
            );
        }
        AnalysisResult::new(1, AnalysisType::SParameter, "Network")
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![50.0; 2],
                noise_reference_temperature_kelvin: None,
            })
            .with_waveforms(traces)
    }

    fn texts(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => out.push(text.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| texts(shape, out)),
            _ => {}
        }
    }

    #[test]
    fn a_series_element_reads_in_chain_form_at_the_frequency_the_view_keeps() {
        let mut app =
            crate::workbench::documents::result_document::test_host::ResultsTestHost::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(series_two_port());
        app.model.simulation.retained.runs.push(run);
        assert!(app.model.simulation.select_run(0));
        let authored = {
            let simulation = &app.model.simulation;
            AnalysisPresentationKey::new(
                simulation.active_run().unwrap().dataset_id,
                simulation.active_analysis().unwrap(),
            )
            .authored()
        };
        // The view keeps 1.9 MHz; the sheet shows the nearest sample, 2 MHz.
        app.results
            .results
            .session
            .readout_pins
            .insert((authored, super::super::ResultViewer::NetworkMatrix), 1.9e6);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let frame =
            |app: &mut crate::workbench::documents::result_document::test_host::ResultsTestHost,
             form: view_form::NetworkForm| {
                let mut shown = Vec::new();
                for _ in 0..2 {
                    let output = ctx.run_ui(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(
                                egui::Pos2::ZERO,
                                egui::vec2(1600.0, 900.0),
                            )),
                            ..Default::default()
                        },
                        |ui| {
                            egui::CentralPanel::default().show(ui, |ui| {
                                let mut parts = app.results_parts();
                                let mut context = SheetContext::of(&mut parts);
                                ui.horizontal(|ui| domain_bar(ui, &mut context));
                                context.results.network_matrix.controls.form = form;
                                show(ui, &mut context);
                            });
                        },
                    );
                    shown.clear();
                    for shape in &output.shapes {
                        texts(&shape.shape, &mut shown);
                    }
                }
                shown
            };
        let chain = frame(&mut app, view_form::NetworkForm::Abcd);
        assert_eq!(app.results.results.network_matrix.controls.sample, 1);
        // A = D = 1, B = Zs(2 MHz) = 100 + j200 Ω, C = 0, each part at the
        // reader's four digits and the unit named once.
        assert!(
            chain.iter().any(|text| text == "100.0 + 200.0j Ω"),
            "{chain:?}"
        );
        assert!(chain.iter().any(|text| text == "2.000 MHz"), "{chain:?}");
        let impedance = frame(&mut app, view_form::NetworkForm::Z);
        assert!(
            impedance
                .iter()
                .any(|text| text.starts_with("Z: no impedance matrix")),
            "a series element has no impedance matrix: {impedance:?}"
        );
        // The clipboard takes the form shown, at full precision.
        app.results.results.network_matrix.controls.form = view_form::NetworkForm::Y;
        let copied = exact_values(app.results_reads()).unwrap();
        let y11 = copied
            .lines()
            .find(|line| line.starts_with("Y11\t"))
            .expect("Y11 row");
        let parts = y11.split('\t').collect::<Vec<_>>();
        let expected =
            num_complex::Complex64::new(1.0, 0.0) / num_complex::Complex64::new(100.0, 200.0);
        assert!((parts[6].parse::<f64>().unwrap() - expected.re).abs() < 1e-15);
        assert!((parts[7].parse::<f64>().unwrap() - expected.im).abs() < 1e-15);
    }

    #[test]
    fn renderer_runs_at_desktop_and_tablet_widths_without_changing_evidence() {
        for width in [1280.0, 768.0] {
            let mut app =
                crate::workbench::documents::result_document::test_host::ResultsTestHost::default();
            let mut run = SimulationRun::new(1);
            run.add_analysis(network_matrix_analysis(true));
            app.model.simulation.retained.runs.push(run);
            assert!(app.model.simulation.select_run(0));
            let before = app
                .model
                .simulation
                .active_analysis()
                .unwrap()
                .result_data_digest();
            let egui = egui::Context::default();
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 900.0),
                )),
                ..Default::default()
            };
            crate::ui::Theme::default().apply(&egui);
            let output = egui.run_ui(input, |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    show(ui, &mut SheetContext::of(&mut app.results_parts()))
                });
            });
            assert!(!output.shapes.is_empty());
            assert_eq!(
                before,
                app.model
                    .simulation
                    .active_analysis()
                    .unwrap()
                    .result_data_digest()
            );
        }
    }
}
