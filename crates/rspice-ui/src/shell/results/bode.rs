//! BODE — the active run's AC response promoted to a full-bleed stability
//! view: gain and phase with unity-gain, phase-margin and gain-margin
//! markers, and the stability table in the right panel.
//!
//! All margins are computed from the simulated curves — markers and tables
//! read the same numbers by construction.

use std::sync::Arc;

use egui::Ui;

use crate::common::AppState;
use crate::state::{AnalysisType, SimulationState};
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si, sample_at};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;

use super::strip::{self, LegendChip};
use super::{DerivedSeries, well_hint};

/// The AC signal pair and its computed stability numbers.
struct BodeModel {
    signal: String,
    frequency: Arc<[f64]>,
    gain_db: Arc<[f64]>,
    phase_deg: Option<Arc<[f64]>>,
    margins: Margins,
}

#[derive(Default, Clone, Copy)]
struct Margins {
    /// DC (lowest-frequency) gain in dB.
    adc_db: Option<f64>,
    /// Unity-gain frequency (Hz).
    ugf: Option<f64>,
    /// Phase margin (deg) at the UGF.
    pm_deg: Option<f64>,
    /// Frequency where phase crosses −180° (Hz).
    f180: Option<f64>,
    /// Gain margin (dB) at f180.
    gm_db: Option<f64>,
    /// −3 dB bandwidth (Hz).
    f3db: Option<f64>,
}

/// Find the first crossing of `series` through `level`, interpolated in
/// log-frequency (frequencies ascending, positive).
fn crossing(frequency: &[f64], series: &[f64], level: f64) -> Option<f64> {
    let n = frequency.len().min(series.len());
    for i in 1..n {
        let (y0, y1) = (series[i - 1] - level, series[i] - level);
        if y0 == 0.0 {
            return Some(frequency[i - 1]);
        }
        if y0 * y1 < 0.0 {
            let t = y0 / (y0 - y1);
            let (l0, l1) = (frequency[i - 1].log10(), frequency[i].log10());
            return Some(10f64.powf(l0 + t * (l1 - l0)));
        }
    }
    None
}

fn build_model(simulation: &SimulationState, derived: &mut DerivedSeries) -> Option<BodeModel> {
    let run = simulation.active_run()?;
    let (analysis_index, analysis) = run
        .analyses
        .iter()
        .enumerate()
        .find(|(_, a)| a.analysis_type == AnalysisType::Ac && !a.waveforms.is_empty())?;

    // First magnitude waveform (prefer a visible one) and its phase partner.
    let (mag_index, mag) = analysis
        .waveforms
        .iter()
        .enumerate()
        .filter(|(_, w)| w.name.starts_with('|'))
        .max_by_key(|(_, w)| w.visible)?;
    let base = mag.name.trim_start_matches('|').trim_end_matches('|');
    let phase = analysis
        .waveforms
        .iter()
        .find(|w| w.name == format!("phase({base})"))
        .map(|w| Arc::clone(&w.y));

    let gain_db = derived.db(
        (analysis_index as u64) << 32 | mag_index as u64,
        &mag.y,
    );
    let frequency = Arc::clone(&mag.x);

    // Margins from the curves.
    let mut margins = Margins {
        adc_db: gain_db.first().copied(),
        ..Margins::default()
    };
    margins.ugf = crossing(&frequency, &gain_db, 0.0);
    if let Some(adc) = margins.adc_db {
        margins.f3db = crossing(&frequency, &gain_db, adc - 3.0);
    }
    if let Some(phase) = &phase {
        if let Some(ugf) = margins.ugf {
            margins.pm_deg = Some(180.0 + sample_at(&frequency, phase, ugf));
        }
        margins.f180 = crossing(&frequency, phase, -180.0);
        if let Some(f180) = margins.f180 {
            margins.gm_db = Some(-sample_at(&frequency, &gain_db, f180));
        }
    }

    Some(BodeModel {
        signal: base.to_owned(),
        frequency,
        gain_db,
        phase_deg: phase,
        margins,
    })
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the stability view.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let Some(model) = build_model(&state.simulation, &mut state.shell.results.derived) else {
        well_hint(ui, "No AC analysis in the active run — enable ac and re-run");
        return;
    };

    let legend = [
        LegendChip {
            name: "gain dB20",
            color: c.traces[0],
            on: true,
        },
        LegendChip {
            name: "phase°",
            color: c.traces[2],
            on: model.phase_deg.is_some(),
        },
    ];
    strip::header(
        ui,
        "STB",
        &format!("{} · margins from the simulated curves", model.signal),
        &legend,
        false,
        false,
    );

    let x0 = model
        .frequency
        .iter()
        .copied()
        .find(|&f| f > 0.0)
        .unwrap_or(1.0);
    let x1 = *model.frequency.last().unwrap_or(&1.0);
    if !(x1 > x0) {
        well_hint(ui, "Degenerate frequency axis");
        return;
    }

    let (g_min, g_max) = model
        .gain_db
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
            (lo.min(v), hi.max(v))
        });
    let pad = ((g_max - g_min) * 0.1).max(3.0);
    let y = Axis::linear((g_min - pad).min(-10.0), (g_max + pad).max(10.0), "dB");

    let mut spec = PlotSpec::new(Axis::log_decades(x0, x1, "Hz"), XScale::Log10, y);
    spec.ref_lines.push(plot::RefLine { y: 0.0 });

    if let Some(phase) = &model.phase_deg {
        let (p_min, p_max) = phase
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
                (lo.min(v), hi.max(v))
            });
        let p0 = ((p_min.min(-180.0)) / 45.0).floor() * 45.0;
        let p1 = (p_max.max(0.0) / 45.0).ceil() * 45.0;
        let ticks: Vec<f64> = (0..=((p1 - p0) / 45.0) as i64)
            .map(|i| p0 + i as f64 * 45.0)
            .collect();
        spec.y_right = Some((Axis::with_ticks(p0, p1, "°", &ticks), c.traces[2]));
        spec.traces.push(
            Trace::new(&model.frequency, phase, c.traces[2])
                .right()
                .dashed()
                .cache_key(0xB0DE_0002),
        );
    }
    spec.traces.push(
        Trace::new(&model.frequency, &model.gain_db, c.traces[0]).cache_key(0xB0DE_0001),
    );

    // Margin markers — first-class objects per the design.
    let m = model.margins;
    if let Some(ugf) = m.ugf {
        spec.markers.push(plot::Marker {
            x: ugf,
            y: 0.0,
            side: plot::YSide::Left,
            color: c.accent,
            label: format!("UGF {}", fmt_si(ugf, "Hz", 1)),
            drop_line: true,
            label_dy: 0.0,
        });
        if let (Some(pm), Some(phase)) = (m.pm_deg, &model.phase_deg) {
            spec.markers.push(plot::Marker {
                x: ugf,
                y: sample_at(&model.frequency, phase, ugf),
                side: plot::YSide::Right,
                color: c.traces[2],
                label: format!("PM {pm:.1}°"),
                drop_line: false,
                label_dy: 30.0,
            });
        }
    }
    if let (Some(f180), Some(gm)) = (m.f180, m.gm_db) {
        spec.markers.push(plot::Marker {
            x: f180,
            y: -gm,
            side: plot::YSide::Left,
            color: c.traces[0],
            label: format!("GM {gm:.1} dB"),
            drop_line: true,
            label_dy: 0.0,
        });
    }

    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![
            ("f".to_owned(), fmt_si(x, "Hz", 2)),
            (
                "gain".to_owned(),
                format!("{:.1} dB", sample_at(&model.frequency, &model.gain_db, x)),
            ),
        ];
        if let Some(phase) = &model.phase_deg {
            rows.push((
                "phase".to_owned(),
                format!("{:.1} °", sample_at(&model.frequency, phase, x)),
            ));
        }
        rows
    };

    plot::show(
        ui,
        &spec,
        &mut state.shell.results.cache,
        None,
        Some(&readout),
    );
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// The stability readout.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Stability", None);
    let Some(model) = build_model(&state.simulation, &mut state.shell.results.derived) else {
        super::panel_note(ui, "No AC analysis in the active run.");
        return;
    };
    let m = model.margins;

    let fmt_opt = |v: Option<f64>, f: &dyn Fn(f64) -> String| -> String {
        v.map_or("—".to_owned(), f)
    };
    let rows = [
        ("Phase margin", fmt_opt(m.pm_deg, &|v| format!("{v:.1}°")), true),
        ("Gain margin", fmt_opt(m.gm_db, &|v| format!("{v:.1} dB")), true),
        ("Unity-gain freq", fmt_opt(m.ugf, &|v| fmt_si(v, "Hz", 1)), false),
        ("f₁₈₀", fmt_opt(m.f180, &|v| fmt_si(v, "Hz", 0)), false),
        ("A_dc", fmt_opt(m.adc_db, &|v| format!("{v:.1} dB")), false),
        ("f₋₃dB", fmt_opt(m.f3db, &|v| fmt_si(v, "Hz", 0)), false),
    ];
    super::stat_table(ui, &rows);

    if model.phase_deg.is_none() {
        super::panel_note(
            ui,
            "Phase data unavailable for this run — re-run the AC analysis to compute margins.",
        );
    } else {
        super::panel_note(ui, "Margins measured on the simulated curves; the plot markers show the same values.");
    }
}
