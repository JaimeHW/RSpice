//! Exact, frequency-indexed network matrices from retained power-wave data.
//!
//! A periodic sideband block is kept separate from every other block. Missing
//! cells, ambiguous names, and differing grids cannot become zero coefficients.

use rspice_results_ui::presentation::well_hint;
use std::sync::Arc;

use egui::Ui;
#[cfg(test)]
use rspice_results::network_matrix::channel_reference;
use rspice_results::network_matrix::{NetworkLayout, resolve};
use rspice_results_ui::network_matrix as view;
#[cfg(test)]
use view::{formatted, magnitude_db};

use super::{AnalysisPresentationKey, AppState, SheetContext};
use crate::state::AnalysisResult;
#[cfg(test)]
use crate::state::AnalysisType;

#[derive(Debug, Clone)]
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

impl Default for NetworkMatrixState {
    fn default() -> Self {
        Self {
            source: None,
            controls: view::NetworkMatrixControls::default(),
            open_trace: false,
            layout: None,
        }
    }
}

impl NetworkMatrixState {
    fn bind(&mut self, key: AnalysisPresentationKey, simulation: &crate::state::SimulationState) {
        let source = (key, simulation.runs.revision(), simulation.data_version);
        if self.source.as_ref() != Some(&source) {
            self.source = Some(source);
            self.controls.reset_source();
            self.open_trace = false;
            self.layout = None;
        }
    }
}

pub(super) fn structure_is_renderable(analysis: &AnalysisResult) -> bool {
    let Some(matrix) = resolve(analysis) else {
        return false;
    };
    super::frame_work::note(super::frame_work::DatasetWalk::SParameterTraceScan);
    matrix.samples_are_finite_and_aligned(analysis)
}

/// What the tab strip says about this sheet, in the sheet's own words.
///
/// The structural verdict comes from the workspace memo rather than a fresh
/// walk of every retained coefficient.
pub(super) fn availability(state: &AppState) -> super::ViewerAvailability {
    if state.simulation.active_run().is_some_and(|run| {
        state.simulation.active_analysis().is_some_and(|analysis| {
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

pub(super) use view::right_panel;

pub(super) fn show(ui: &mut Ui, context: &mut SheetContext<'_>) {
    let Some(run) = context.simulation.active_run() else {
        well_hint(ui, "Run a network analysis to inspect its matrix");
        return;
    };
    let Some(analysis) = context.simulation.active_analysis() else {
        return;
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
            analysis.validate_retained_evidence().is_ok()
        });
    if !valid {
        well_hint(
            ui,
            "The retained network matrix is incomplete or has invalid coefficient or frequency evidence",
        );
        return;
    }
    let controls = &mut context.results.network_matrix;
    controls.bind(key, context.simulation);
    // Matrix topology can be large even when only a few rows are visible.
    // Resolve names and block membership once per retained source generation;
    // subsequent frames borrow its index inventory and only read visible cells.
    if controls.layout.is_none() {
        let Some(matrix) = resolve(analysis) else {
            well_hint(
                ui,
                "Requires a complete complex network matrix and its physical port references",
            );
            return;
        };
        controls.layout = Some(Arc::new(matrix));
    }
    let matrix = controls.layout.as_ref().expect("resolved layout").clone();
    if let Some(coefficient) = view::show(
        ui,
        analysis,
        &matrix,
        &mut controls.controls,
        |block, sample| matrix_csv(&matrix, analysis, block, sample),
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
                        .max_height(200.0)
                        .show(ui, |ui| super::polar::right_panel(ui, context));
                });
                super::polar::show(ui, context);
            });
    }
    context.results.network_matrix.open_trace = open_trace;
}

fn matrix_csv(
    matrix: &NetworkLayout,
    analysis: &AnalysisResult,
    block_index: usize,
    sample: usize,
) -> String {
    let table = matrix
        .exact_table(analysis, block_index, sample)
        .expect("resolved complex coefficient sample");
    rspice_formats::result_csv::encode_network_matrix_csv(&table)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::state::{AnalysisResultFamilyMetadata, SimulationRun, WaveformData};

    pub(crate) fn fixture(mixed: bool) -> AnalysisResult {
        let mut traces = Vec::new();
        let ports = if mixed { 4 } else { 2 };
        for row in 0..ports {
            for column in 0..ports {
                let name = if mixed {
                    format!(
                        "S{}{}{}{}",
                        if row < 2 { "d" } else { "c" },
                        if column < 2 { "d" } else { "c" },
                        row % 2 + 1,
                        column % 2 + 1
                    )
                } else {
                    format!("S{}{}", row + 1, column + 1)
                };
                let value = if row == column { 0.25 } else { 0.01 };
                traces.push(
                    WaveformData::new(
                        format!("|{name}|"),
                        vec![1e6, 2e6],
                        vec![value; 2],
                        "#00aaff",
                    )
                    .with_complex_components(
                        name,
                        vec![value; 2],
                        vec![0.0; 2],
                    ),
                );
            }
        }
        AnalysisResult::new(1, AnalysisType::SParameter, "Network")
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![50.0; ports],
                noise_reference_temperature_kelvin: None,
            })
            .with_waveforms(traces)
    }

    #[test]
    fn complete_physical_and_mixed_matrices_have_exact_references_and_values() {
        for mixed in [false, true] {
            let analysis = fixture(mixed);
            assert!(structure_is_renderable(&analysis));
            let matrix = resolve(&analysis).unwrap();
            let csv = matrix_csv(&matrix, &analysis, 0, 1);
            assert!(csv.contains("2.00000000000000000e6"));
            let tables = rspice_results::network_matrix::report_tables(&analysis).unwrap();
            assert_eq!(tables.len(), 1);
            assert_eq!(tables[0].rows.len(), 2 * matrix.references().len().pow(2));
            assert_eq!(
                channel_reference(0, matrix.references(), mixed),
                if mixed { 100.0 } else { 50.0 }
            );
            if mixed {
                assert_eq!(channel_reference(2, matrix.references(), mixed), 25.0);
            }
        }
    }

    #[test]
    fn replacing_evidence_cannot_reuse_a_sampled_diagnostic() {
        let analysis = fixture(false);
        let mut simulation = crate::state::SimulationState::default();
        let mut run = SimulationRun::new(1);
        let key = AnalysisPresentationKey::new(run.dataset_id, &analysis);
        run.add_analysis(analysis);
        simulation.runs.push(run);
        let mut controls = NetworkMatrixState::default();
        controls.bind(key, &simulation);
        controls.controls.diagnostic = Some((0, 0, "old evidence".to_owned()));
        controls.bind(key, &simulation.clone());
        assert!(controls.controls.diagnostic.is_some());
        simulation.data_version = simulation.data_version.wrapping_add(1);
        controls.bind(key, &simulation);
        assert!(controls.controls.diagnostic.is_none());
        controls.controls.diagnostic = Some((0, 0, "old evidence".to_owned()));
        simulation.runs[0].analyses[0] = fixture(false);
        controls.bind(key, &simulation);
        assert!(controls.controls.diagnostic.is_none());
    }

    #[test]
    fn absent_duplicate_or_misaligned_coefficients_are_refused() {
        let mut missing = fixture(false);
        missing.waveforms.pop();
        assert!(!structure_is_renderable(&missing));
        let mut duplicate = fixture(false);
        duplicate
            .data
            .waveforms
            .push(duplicate.data.waveforms[0].clone());
        assert!(!structure_is_renderable(&duplicate));
        let mut wrong_grid = fixture(false);
        wrong_grid.waveforms[0].x = vec![1e6, 3e6].into();
        assert!(!structure_is_renderable(&wrong_grid));
    }

    #[test]
    fn translated_blocks_and_large_port_indexes_keep_their_identity() {
        let mut periodic = fixture(false);
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
        let mut analysis = fixture(true);
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
        let analysis = fixture(false);
        let matrix = resolve(&analysis).unwrap();
        let table = matrix.exact_table(&analysis, 0, 1).unwrap();
        for (index, row) in table.rows.iter().enumerate() {
            let complex = analysis.waveforms[matrix.blocks()[0].cells()[index]]
                .complex
                .as_ref()
                .unwrap();
            assert_eq!(
                row[6].parse::<f64>().unwrap().to_bits(),
                complex.real[1].to_bits()
            );
            assert_eq!(
                row[7].parse::<f64>().unwrap().to_bits(),
                complex.imag[1].to_bits()
            );
        }
        assert!(formatted(0.0, 0.0, 2).contains("phase undefined"));
        assert!((magnitude_db(1e308, 1e308) - (6160.0 + 10.0 * 2.0_f64.log10())).abs() < 1e-10);
    }

    fn cell_text_rect(shape: &egui::Shape) -> Option<egui::Rect> {
        match shape {
            egui::Shape::Text(text) if text.galley.text().starts_with("2.50000000e-1") => {
                Some(shape.visual_bounding_rect())
            }
            egui::Shape::Vec(shapes) => shapes.iter().find_map(cell_text_rect),
            _ => None,
        }
    }

    #[test]
    fn clicking_a_matrix_cell_opens_its_retained_coefficient_plot() {
        for width in [1280.0, 768.0] {
            let mut app = crate::workbench::AppState::default();
            let mut run = SimulationRun::new(1);
            run.add_analysis(fixture(true));
            app.simulation.runs.push(run);
            assert!(app.simulation.select_run(0));
            let before = app
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
                        egui::CentralPanel::default()
                            .show(ui, |ui| show(ui, &mut SheetContext::of(&mut app)));
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
            for pressed in [true, false] {
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
            assert!(app.ui.results.network_matrix.open_trace);
            assert_eq!(
                app.ui.results.session.polar.quantity.as_deref(),
                Some("|Sdd11|")
            );
            assert_eq!(
                before,
                app.simulation
                    .active_analysis()
                    .unwrap()
                    .result_data_digest()
            );
        }
    }

    #[test]
    fn renderer_runs_at_desktop_and_tablet_widths_without_changing_evidence() {
        for width in [1280.0, 768.0] {
            let mut app = crate::workbench::AppState::default();
            let mut run = SimulationRun::new(1);
            run.add_analysis(fixture(true));
            app.simulation.runs.push(run);
            assert!(app.simulation.select_run(0));
            let before = app
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
                egui::CentralPanel::default()
                    .show(ui, |ui| show(ui, &mut SheetContext::of(&mut app)));
            });
            assert!(!output.shapes.is_empty());
            assert_eq!(
                before,
                app.simulation
                    .active_analysis()
                    .unwrap()
                    .result_data_digest()
            );
        }
    }

    #[test]
    #[ignore = "writes images for manual layout inspection"]
    fn render_network_matrix_previews() {
        let destination = std::path::PathBuf::from(
            std::env::var("RSPICE_NETWORK_MATRIX_QA_DIR")
                .expect("set the preview output directory"),
        );
        std::fs::create_dir_all(&destination).unwrap();
        for (width, open_plot) in [(1280, false), (768, false), (768, true)] {
            let mut app = crate::workbench::AppState::default();
            let mut run = SimulationRun::new(1);
            run.add_analysis(fixture(true));
            app.simulation.runs.push(run);
            assert!(app.simulation.select_run(0));
            if open_plot {
                let run = app.simulation.active_run().unwrap();
                let analysis = app.simulation.active_analysis().unwrap();
                let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
                app.ui.results.network_matrix.bind(key, &app.simulation);
                app.ui.results.network_matrix.open_trace = true;
                app.ui.results.session.polar.quantity = Some("|Sdd11|".to_owned());
            }
            let canvas =
                crate::ui::raster::render(egui::vec2(width as f32, 640.0), |ui, background| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(background).inner_margin(12.0))
                        .show(ui, |ui| show(ui, &mut SheetContext::of(&mut app)));
                });
            std::fs::write(
                destination.join(format!(
                    "network-matrix-{width}{}.png",
                    if open_plot { "-plot" } else { "" }
                )),
                canvas.png(640),
            )
            .unwrap();
        }
    }
}
