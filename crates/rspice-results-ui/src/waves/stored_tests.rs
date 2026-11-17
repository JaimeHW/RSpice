//! A run whose samples stay in a results store draws as any other: its strips are built, fitted
//! and drawn frame after frame, and each stored column is read once, on the first frame that
//! needs its values, and kept by every model and frame after it.

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::pane::{PanePlot, pane_auto_y, show_plot};
use super::stack::pane_y_range;
use super::*;
use crate::derived::DerivedSeries;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::executed_deck::ExecutedDeckArchive;
use rspice_results::waveform::{
    ColumnSource, RetainedTrace, StoredWindow, WaveformValues, WindowLead,
};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::{plot, raster};

/// A stored column in memory that counts how often it is read.
struct Counted {
    name: &'static str,
    values: Vec<f64>,
    reads: AtomicUsize,
}

impl ColumnSource for Counted {
    fn name(&self) -> &str {
        self.name
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }
}

fn counted(name: &'static str, values: Vec<f64>) -> Arc<Counted> {
    Arc::new(Counted {
        name,
        values,
        reads: AtomicUsize::new(0),
    })
}

fn stored(column: &Arc<Counted>) -> WaveformValues {
    let window = StoredWindow {
        first: 0,
        rows: column.values.len() as u64,
        lead: WindowLead::None,
    };
    WaveformValues::stored(column.clone(), window).expect("the window fits the column")
}

fn models(
    run: &rspice_results::run::SimulationRun<AnalysisResult<RetainedTrace>>,
) -> Vec<StripModel> {
    let mut models = build_models(
        &[run],
        &ExecutedDeckArchive::default(),
        &mut DerivedSeries::default(),
        &Tokens::default(),
        ProjectionOptions {
            phase_continuous: false,
            complex_display: Default::default(),
            ac: Default::default(),
            selection: None,
            hidden_family_traces: &HashSet::new(),
        },
        |_| None,
    );
    extent::resolve_x_ranges(&mut models);
    models
}

#[test]
fn a_stored_trace_is_read_once_and_kept_however_many_frames_draw_it() {
    const ROWS: usize = 20_000;
    let time = counted("time", (0..ROWS).map(|row| row as f64 * 1.0e-9).collect());
    let out = counted(
        "V(out)",
        (0..ROWS).map(|row| (row as f64 * 1.0e-3).sin()).collect(),
    );
    let mut run = rspice_results::run::SimulationRun::new(
        1,
        0.0,
        rspice_results::run::ExecutionTarget::LocalDesktop,
    );
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran", 0.0).with_waveforms(vec![
            RetainedTrace::new("V(out)", stored(&time), stored(&out), "#fff"),
        ]),
    );
    let reads = || {
        (
            time.reads.load(Ordering::SeqCst),
            out.reads.load(Ordering::SeqCst),
        )
    };
    assert_eq!(reads(), (0, 0), "building the run reads nothing");

    let mut cache = plot::DecimationCache::default();
    let mut derived = DerivedSeries::default();
    // Two generations of models, as a visibility toggle or a new overlay rebuilds them, each
    // drawn for several frames with a cursor whose readout reads the trace.
    for generation in 0..2 {
        let models = models(&run);
        let model = &models[0];
        assert!(
            model.traces[0].y.is_stored(),
            "the model shares the stored values"
        );
        let (x0, x1) = model.x_range.expect("the strip has an X range");
        for frame in 0..4 {
            raster::render(egui::vec2(640.0, 320.0), |ui, _| {
                cache.ensure_version(generation);
                for pane in model.unit_panes() {
                    let range = pane_y_range(&mut derived, model, &pane.traces, |_| {});
                    let auto_y = pane_auto_y(range, None, &[]).expect("the pane has a range");
                    show_plot(
                        ui,
                        &mut cache,
                        model,
                        &pane,
                        PanePlot {
                            x_range: (x0, x1),
                            auto_y,
                            y_view: None,
                            log_y: false,
                            left_margin: 64.0,
                            readout: ReadoutPolicy::default(),
                            quantity: Default::default(),
                            minor_grid: false,
                            trace_width: 1.8,
                            horizontal_cursors: [None, None],
                            horizontal_cursor_interactive: false,
                            cursors: Some(plot::CursorPair {
                                a: Some(x0 + (x1 - x0) * 0.25),
                                b: Some(x0 + (x1 - x0) * (0.5 + 0.1 * f64::from(frame))),
                            }),
                            specification_limits: Vec::new(),
                            markers: Vec::new(),
                            expressions: &[],
                            family_envelopes: &[],
                            find_nearest: true,
                        },
                    );
                }
            });
            assert_eq!(
                reads(),
                (1, 1),
                "generation {generation}, frame {frame}: each stored column is read once and kept"
            );
        }
    }
}
