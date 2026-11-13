//! Every results sheet's text stands 4.5:1 off what is painted behind it in
//! all four theme combinations: either interface mode over either canvas.
//!
//! The canvas has a theme of its own, so text a sheet paints on the canvas in
//! the interface's inks reads dark on dark under a light interface over the
//! default canvas, and light on light the other way round. Each sheet is
//! drawn whole, as the reader meets it in the Results workspace, on the
//! frame-cost fixture's retained run, and the ground under every run of text
//! is measured off the frame.

use rspice_ui_kit::EngineeringCanvasTheme;
use rspice_ui_kit::tokens::Mode;

use super::super::super::{ResultViewer, ResultsTestHost};
use super::{large_state, select_analysis, state_for};
use crate::state::AnalysisType;

/// The frame every sheet is drawn in: the workspace at a laptop's size.
const FRAME: egui::Vec2 = egui::vec2(1_280.0, 800.0);

/// The four theme combinations a reader can choose.
fn themes() -> impl Iterator<Item = crate::ui::Theme> {
    [Mode::Dark, Mode::Light].into_iter().flat_map(|mode| {
        EngineeringCanvasTheme::ALL.map(|canvas_theme| crate::ui::Theme {
            mode,
            canvas_theme,
            ..crate::ui::Theme::default()
        })
    })
}

/// Every run of text `viewer` paints under 4.5:1 in any of the four theme
/// combinations, drawn whole in the Results workspace over `state`.
fn unreadable(viewer: ResultViewer, mut state: ResultsTestHost) -> Vec<String> {
    assert_eq!(
        super::super::super::viewer_unavailability_reason(state.results_reads(), viewer),
        None,
        "the fixture feeds the {viewer:?} sheet"
    );
    state.results.results.session.viewer = viewer;
    let mut failures = Vec::new();
    for theme in themes() {
        let ctx = egui::Context::default();
        ctx.set_os(egui::os::OperatingSystem::Nix);
        theme.apply(&ctx);
        let mut controller = crate::simulation::SimulationController::new();
        let mut output = None;
        for _ in 0..3 {
            output = Some(ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        super::super::super::show(
                            ui,
                            &mut state.results_parts(),
                            &mut controller,
                            None,
                        );
                    });
                },
            ));
        }
        assert_eq!(
            state.results.results.session.viewer, viewer,
            "the workspace kept the {viewer:?} sheet open"
        );
        let output = output.expect("a frame ran");
        let contrasts = rspice_ui_kit::raster::text_contrasts(&ctx, FRAME, &output);
        assert!(contrasts.len() > 8, "{viewer:?} paints its text");
        failures.extend(id_clashes(&contrasts));
        failures.extend(
            contrasts
                .into_iter()
                .filter(|text| text.ratio < 4.5)
                .map(|text| {
                    format!(
                        "{:?}/{:?}: {:?} {:.2}:1 ({:?} on {:?}) at {:?}",
                        theme.mode,
                        theme.canvas_theme,
                        text.text,
                        text.ratio,
                        text.ink,
                        text.ground,
                        text.rect.min
                    )
                }),
        );
    }
    failures
}

/// egui's warning, painted over the frame, that two widgets took one ID and
/// share their state: two legends that scroll together, say.
fn id_clashes(contrasts: &[rspice_ui_kit::raster::TextContrast]) -> Vec<String> {
    contrasts
        .iter()
        .filter(|text| text.text.contains(" use of "))
        .map(|text| format!("ID clash: {:?} at {:?}", text.text, text.rect.min))
        .collect()
}

fn assert_readable(viewer: ResultViewer, state: ResultsTestHost) {
    let failures = unreadable(viewer, state);
    assert!(
        failures.is_empty(),
        "{viewer:?}: {} runs of text under 4.5:1\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// The fixture run with the analysis of `analysis_type` selected.
fn on(analysis_type: AnalysisType) -> ResultsTestHost {
    let mut state = large_state();
    select_analysis(&mut state, analysis_type);
    state
}

#[test]
fn the_waves_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Waves, state_for(ResultViewer::Waves));
}

#[test]
fn the_bode_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Bode, state_for(ResultViewer::Bode));
}

#[test]
fn the_noise_sheet_reads_in_every_theme() {
    assert_readable(
        ResultViewer::NoiseContrib,
        state_for(ResultViewer::NoiseContrib),
    );
}

#[test]
fn the_fft_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Fft, state_for(ResultViewer::Fft));
}

#[test]
fn the_eye_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Eye, state_for(ResultViewer::Eye));
}

#[test]
fn the_events_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Events, state_for(ResultViewer::Events));
}

#[test]
fn the_operating_point_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Op, state_for(ResultViewer::Op));
}

#[test]
fn the_table_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Table, state_for(ResultViewer::Table));
}

#[test]
fn the_soa_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Soa, state_for(ResultViewer::Soa));
}

#[test]
fn the_sensitivity_sheet_reads_in_every_theme() {
    assert_readable(
        ResultViewer::Contribution,
        state_for(ResultViewer::Contribution),
    );
}

#[test]
fn the_optimization_sheet_reads_in_every_theme() {
    assert_readable(
        ResultViewer::Optimization,
        state_for(ResultViewer::Optimization),
    );
}

#[test]
fn the_manifest_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Manifest, state_for(ResultViewer::Manifest));
}

#[test]
fn the_harmonic_balance_sheet_reads_in_every_theme() {
    assert_readable(
        ResultViewer::HarmonicBalance,
        on(AnalysisType::HarmonicBalance),
    );
}

#[test]
fn the_phase_noise_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::PhaseNoise, on(AnalysisType::Pnoise));
}

#[test]
fn the_smith_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Smith, on(AnalysisType::SParameter));
}

#[test]
fn the_polar_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Polar, on(AnalysisType::SParameter));
}

/// A complete two-port S-parameter sweep at 50 Ω, which the matrix sheet
/// reads.
fn two_port() -> ResultsTestHost {
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, RetainedTrace, SimulationRun,
    };

    let frequency: Vec<f64> = (0..200)
        .map(|index| 10f64.powf(6.0 + f64::from(index) * 0.02))
        .collect();
    let waveforms = ["S11", "S12", "S21", "S22"]
        .into_iter()
        .enumerate()
        .map(|(term, name)| {
            let phase = |f: &f64| f.log10() + term as f64;
            let real: Vec<f64> = frequency.iter().map(|f| phase(f).cos() * 0.4).collect();
            let imag: Vec<f64> = frequency.iter().map(|f| phase(f).sin() * 0.4).collect();
            let magnitude = vec![0.4; frequency.len()];
            RetainedTrace::new(name, frequency.clone(), magnitude, "#00aaff")
                .with_complex_components(name, real, imag)
        })
        .collect();
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::SParameter, "SP")
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                noise_reference_temperature_kelvin: None,
                reference_impedances_ohm: vec![50.0, 50.0],
            })
            .with_waveforms(waveforms),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    assert!(state.model.simulation.select_analysis(0));
    state
}

/// A stability run of a single-pole amplifier with a loop gain of 60 dB,
/// with its loop-gain contour loaded for the Nyquist sheet as a finished run
/// leaves it.
fn stability() -> ResultsTestHost {
    use crate::state::{
        ActiveViewer, AnalysisResult, RetainedTrace, SimulationRun,
        SpecializedViewerCacheProvenance,
    };

    let frequency: Vec<f64> = (0..200)
        .map(|index| 10f64.powf(f64::from(index) * 0.03))
        .collect();
    let gain: Vec<(f64, f64)> = frequency
        .iter()
        .map(|f| {
            let pole = f / 10.0;
            let magnitude = 1_000.0 / (1.0 + pole * pole).sqrt();
            (magnitude, -pole.atan())
        })
        .collect();
    let decibels: Vec<f64> = gain.iter().map(|(m, _)| 20.0 * m.log10()).collect();
    let degrees: Vec<f64> = gain.iter().map(|(_, p)| p.to_degrees()).collect();
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Stb, "STB").with_waveforms(vec![
            RetainedTrace::new("Loop Gain (dB)", frequency.clone(), decibels, "#00aaff")
                .with_unit("dB"),
            RetainedTrace::new("Loop Phase (deg)", frequency.clone(), degrees, "#ffbd2e")
                .with_unit("°"),
        ]),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    assert!(state.model.simulation.select_analysis(0));

    let real: Vec<f64> = gain.iter().map(|(m, p)| m * p.cos()).collect();
    let imaginary: Vec<f64> = gain.iter().map(|(m, p)| m * p.sin()).collect();
    state.results.analysis.nyquist_state.load_data(
        rspice_results_ui::nyquist::NyquistData::from_arrays(
            "L(jω)", &frequency, &real, &imaginary,
        ),
    );
    let owner = {
        let simulation = &state.model.simulation;
        let run = simulation.active_run().expect("an active run");
        let analysis = simulation.active_analysis().expect("an active analysis");
        SpecializedViewerCacheProvenance::for_analysis(run.dataset_id, analysis)
    };
    state
        .results
        .analysis
        .bind_specialized_viewer_cache(ActiveViewer::Nyquist, owner);
    state
}

#[test]
fn the_nyquist_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Nyquist, stability());
}

#[test]
fn the_network_matrix_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::NetworkMatrix, two_port());
}

#[test]
fn the_distortion_curves_read_in_every_theme() {
    assert_readable(ResultViewer::Bode, on(AnalysisType::Disto));
}

/// A completed Monte Carlo run of three hundred trials measuring a gain and
/// a bandwidth, which feeds the distribution sheets.
fn monte_carlo() -> ResultsTestHost {
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, SimulationRunLifecycle,
        SimulationRunProvenance,
    };
    use rspice_results::family_metadata::MonteCarloVariableMetadata;

    let variable = |name: &str, centre: f64, spread: f64| {
        let samples: Vec<f64> = (0..300)
            .map(|trial| {
                centre + spread * (f64::from(trial) * 0.618_034).fract().mul_add(2.0, -1.0)
            })
            .collect();
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        let variance = samples
            .iter()
            .map(|sample| (sample - mean).powi(2))
            .sum::<f64>()
            / samples.len() as f64;
        MonteCarloVariableMetadata {
            mean_confidence: None,
            name: name.to_owned(),
            mean,
            std_dev: variance.sqrt(),
            min: samples.iter().copied().fold(f64::INFINITY, f64::min),
            max: samples.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            samples,
        }
    };
    let analysis = AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
        AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 7,
            runs_requested: 300,
            runs_completed: 300,
            failures: 0,
            all_converged: true,
            variables: vec![
                variable("gain_dc", 40.0, 1.5),
                variable("bandwidth", 1.0e6, 2.0e5),
            ],
            member_measurements: Vec::new(),
        },
    );
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(analysis);
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("a provenance");
    run.mark_running().expect("running");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("completed");
    state.model.simulation.complete_run();
    state
}

#[test]
fn the_histogram_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Hist, monte_carlo());
}

#[test]
fn the_scatter_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::Scatter, monte_carlo());
}

#[test]
fn the_box_and_violin_sheet_reads_in_every_theme() {
    assert_readable(ResultViewer::BoxViolin, monte_carlo());
}

/// A result document's pane draws its header, its kept measurements and its
/// viewer's own controls on the canvas-filled pane. Every run of text there,
/// the active pane's title included, reads in all four theme combinations:
/// the title sat on the accent wash over the canvas, dark on dark under a
/// light interface, and the histogram's control labels on the canvas itself.
#[test]
fn a_result_document_pane_reads_in_every_theme() {
    use rspice_results::document_creation::ResultDocumentFamily;
    for (family, viewer_id, state) in [
        (
            ResultDocumentFamily::StatisticsAndYield,
            "viewer-histogram",
            monte_carlo(),
        ),
        (
            ResultDocumentFamily::WaveformWorksheet,
            "viewer-waveform",
            completed(state_for(ResultViewer::Waves)),
        ),
    ] {
        let failures = unreadable_document(family, viewer_id, state);
        assert!(
            failures.is_empty(),
            "{viewer_id}: {} runs of text under 4.5:1\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}

/// `state` with its run finished, as a result document requires.
fn completed(mut state: ResultsTestHost) -> ResultsTestHost {
    use crate::state::{SimulationRunLifecycle, SimulationRunProvenance};
    let run = &mut state.model.simulation.retained.runs[0];
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("a provenance");
    run.mark_running().expect("running");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("completed");
    state
}

/// A single-pane result document of `viewer_id` over `state`, drawn whole in
/// each of the four theme combinations.
fn document_frames(
    family: rspice_results::document_creation::ResultDocumentFamily,
    viewer_id: &str,
    mut state: ResultsTestHost,
) -> Vec<(crate::ui::Theme, egui::Context, egui::FullOutput)> {
    use super::super::super::create_document;
    let document = {
        let mut parts = state.results_parts();
        create_document::open(&mut parts);
        parts.create_result_document.family_id = family.id().to_owned();
        parts.create_result_document.viewer_id = viewer_id.to_owned();
        parts.create_result_document.layout_id =
            rspice_results::document_creation::ResultDocumentLayout::SinglePane
                .id()
                .to_owned();
        create_document::commit(&mut parts).expect("the document is created")
    };
    themes()
        .map(|theme| {
            let ctx = egui::Context::default();
            ctx.set_os(egui::os::OperatingSystem::Nix);
            theme.apply(&ctx);
            let mut controller = crate::simulation::SimulationController::new();
            let mut output = None;
            for _ in 0..3 {
                output = Some(ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            super::super::super::persistent_document::show(
                                ui,
                                &mut state.results_parts(),
                                &mut controller,
                                None,
                                document,
                            );
                        });
                    },
                ));
            }
            (theme, ctx, output.expect("a frame ran"))
        })
        .collect()
}

/// [`unreadable`] for a single-pane result document of `viewer_id`.
fn unreadable_document(
    family: rspice_results::document_creation::ResultDocumentFamily,
    viewer_id: &str,
    state: ResultsTestHost,
) -> Vec<String> {
    let mut failures = Vec::new();
    for (theme, ctx, output) in document_frames(family, viewer_id, state) {
        let contrasts = rspice_ui_kit::raster::text_contrasts(&ctx, FRAME, &output);
        assert!(contrasts.len() > 8, "{viewer_id} paints its text");
        failures.extend(id_clashes(&contrasts));
        failures.extend(
            contrasts
                .into_iter()
                .filter(|text| text.ratio < 4.5)
                .map(|text| {
                    format!(
                        "{:?}/{:?}: {:?} {:.2}:1 ({:?} on {:?}) at {:?}",
                        theme.mode,
                        theme.canvas_theme,
                        text.text,
                        text.ratio,
                        text.ink,
                        text.ground,
                        text.rect.min
                    )
                }),
        );
    }
    failures
}

/// The pictures for review, written under `RSPICE_PICTURES` when it is set:
/// the histogram and waveform document panes in each of the four theme
/// combinations, named `<pane>-<interface>-ui-<canvas>-canvas.png`.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_document_pane_pictures() {
    use rspice_results::document_creation::ResultDocumentFamily;
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    for (name, family, viewer_id, state) in [
        (
            "histogram-document",
            ResultDocumentFamily::StatisticsAndYield,
            "viewer-histogram",
            monte_carlo(),
        ),
        (
            "waveform-document",
            ResultDocumentFamily::WaveformWorksheet,
            "viewer-waveform",
            completed(state_for(ResultViewer::Waves)),
        ),
    ] {
        for (theme, ctx, output) in document_frames(family, viewer_id, state) {
            let picture = rspice_ui_kit::raster::rasterize(&ctx, FRAME, &output);
            let interface = format!("{:?}", theme.mode).to_lowercase();
            let canvas = format!("{:?}", theme.canvas_theme).to_lowercase();
            std::fs::write(
                format!("{directory}/{name}-{interface}-ui-{canvas}-canvas.png"),
                picture.png(picture.height()),
            )
            .expect("the picture is written");
        }
    }
}
