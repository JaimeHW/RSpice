//! BODE — the active run's AC response promoted to a full-bleed stability
//! view: gain and phase with unity-gain, phase-margin and gain-margin
//! markers, and the stability table in the right panel.
//!
//! All margins are computed from the simulated curves — markers and tables
//! read the same numbers by construction.

use std::sync::Arc;

use egui::Ui;

use crate::common::AppState;
use crate::state::{SharedWaveformValues, ac_bode_summary_for_run};
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si, sample_at};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, section_header};

use super::strip::{self, LegendChip};
use super::{BodeDerived, well_hint};

/// The AC signal pair and its computed stability numbers.
struct BodeModel {
    signal: String,
    frequency: SharedWaveformValues,
    gain_db: SharedWaveformValues,
    /// The phase trace as displayed: raw ±180°-wrapped samples, or the
    /// unwrapped series when the continuous toggle is on. The margins are
    /// always computed from the raw arrays.
    phase_deg: Option<SharedWaveformValues>,
    /// Finite (min, max) of the displayed phase curve.
    phase_extremes: Option<(f64, f64)>,
    margins: BodeDerived,
}

fn build_model(state: &mut AppState) -> Option<BodeModel> {
    let simulation = &state.simulation;
    let run = simulation.active_run()?;
    let summary = ac_bode_summary_for_run(run)?;
    let analysis = &run.analyses[summary.analysis_index];
    let mag = &analysis.waveforms[summary.mag_index];
    let phase = summary
        .phase_index
        .zip(summary.phase_deg.as_ref())
        .map(|(phase_index, phase)| (phase_index, Arc::clone(phase)));

    let gain_db = state.shell.results.derived.db(
        (summary.analysis_index as u64) << 32 | summary.mag_index as u64,
        &mag.y,
    );
    let frequency = Arc::clone(&summary.frequency);

    // Margins + extremes from the curves, cached on (data version, resolved
    // magnitude waveform) — the crossings and folds are O(points) and both
    // panels read them every frame.
    let version = simulation.data_version;
    let margins = match state.shell.results.bode {
        Some(d)
            if d.version == version
                && d.analysis_index == summary.analysis_index
                && d.mag_index == summary.mag_index =>
        {
            d
        }
        _ => {
            let metrics = summary.metrics;
            let d = BodeDerived {
                version,
                analysis_index: summary.analysis_index,
                mag_index: summary.mag_index,
                adc_db: metrics.adc_db,
                ugf: metrics.ugf,
                pm_deg: metrics.pm_deg,
                f180: metrics.f180,
                gm_db: metrics.gm_db,
                f3db: metrics.f3db,
                gain_extremes: metrics.gain_extremes,
                phase_extremes: metrics.phase_extremes,
            };
            state.shell.results.bode = Some(d);
            d
        }
    };

    // Displayed phase: optionally unwrapped into a continuous curve. The
    // margin computation above reads the raw wrapped arrays on purpose —
    // only the displayed trace (and its axis range) changes.
    let (phase_deg, phase_extremes) = match &phase {
        Some((phase_index, raw)) if state.shell.results.phase_continuous => {
            let key = (summary.analysis_index as u64) << 32 | *phase_index as u64;
            let derived = &mut state.shell.results.derived;
            let series = derived.unwrapped(key, raw);
            let extremes = derived.unwrapped_range(key, raw);
            (Some(series), extremes)
        }
        Some((_, raw)) => (Some(Arc::clone(raw)), margins.phase_extremes),
        None => (None, None),
    };

    Some(BodeModel {
        signal: summary.signal,
        frequency,
        gain_db,
        phase_deg,
        phase_extremes,
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
    let Some(model) = build_model(state) else {
        well_hint(
            ui,
            "No AC analysis in the active run — enable ac and re-run",
        );
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
    let view = state.shell.results.plot_view(super::ResultViewer::Bode, 0);
    let header = strip::StripHeader::new(
        "STB",
        &format!("{} · margins from the simulated curves", model.signal),
        &legend,
    )
    .zoomed(view.is_zoomed())
    .show(ui);
    if header.fit_clicked {
        state
            .shell
            .results
            .reset_plot_view(super::ResultViewer::Bode, 0);
    }

    // Phase-wrap selector, mirroring the FFT window-selector row: display
    // only — the margins always read the raw wrapped arrays. The choice
    // also drives the waves strips' phase traces. `continuous` is captured
    // before the chips so this frame's trace keys match the model that was
    // just built; a click re-renders next frame.
    let continuous = state.shell.results.phase_continuous;
    if model.phase_deg.is_some() {
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("phase")
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(c.text_dim),
            );
            if chip(ui, "wrapped ±180°", !continuous)
                .on_hover_text("Phase as simulated, wrapped to ±180°")
                .clicked()
            {
                state.shell.results.phase_continuous = false;
            }
            if chip(ui, "continuous", continuous)
                .on_hover_text("Unwrap the displayed phase into a continuous curve")
                .clicked()
            {
                state.shell.results.phase_continuous = true;
            }
        });
        ui.add_space(2.0);
    }

    let x0 = model
        .frequency
        .iter()
        .copied()
        .find(|&f| f > 0.0)
        .unwrap_or(1.0);
    let x1 = *model.frequency.last().unwrap_or(&1.0);
    if !matches!(x1.partial_cmp(&x0), Some(std::cmp::Ordering::Greater)) {
        well_hint(ui, "Degenerate frequency axis");
        return;
    }
    let (x0, x1) = view.x.unwrap_or((x0, x1));

    let (g_min, g_max) = model.margins.gain_extremes;
    let pad = ((g_max - g_min) * 0.1).max(3.0);
    let (y0, y1) = view
        .y
        .unwrap_or(((g_min - pad).min(-10.0), (g_max + pad).max(10.0)));
    let y = Axis::linear(y0, y1, "dB");

    let mut spec = PlotSpec::new(Axis::log_decades(x0, x1, "Hz"), XScale::Log10, y)
        .accessible_name("Bode plot");
    spec.ref_lines.push(plot::RefLine { y: 0.0 });

    if let Some(phase) = &model.phase_deg {
        // Displayed extremes — the unwrapped curve can leave ±180°, and the
        // 45° lattice below handles the wider range.
        let (p_min, p_max) = model.phase_extremes.unwrap_or((-180.0, 0.0));
        let axis = match view.y_right {
            // Zoomed: plain linear ticks instead of the 45° lattice.
            Some((z0, z1)) => Axis::linear_with(z0, z1, "°", 5),
            None => {
                let p0 = ((p_min.min(-180.0)) / 45.0).floor() * 45.0;
                let p1 = (p_max.max(0.0) / 45.0).ceil() * 45.0;
                let ticks: Vec<f64> = (0..=((p1 - p0) / 45.0) as i64)
                    .map(|i| p0 + i as f64 * 45.0)
                    .collect();
                Axis::with_ticks(p0, p1, "°", &ticks)
            }
        };
        spec.y_right = Some((axis, c.traces[2]));
        spec.traces.push(
            Trace::new(&model.frequency, phase, c.traces[2])
                .right()
                .dashed()
                // Wrapped and continuous series decimate differently.
                .cache_key(if continuous { 0xB0DE_0003 } else { 0xB0DE_0002 }),
        );
    }
    spec.traces
        .push(Trace::new(&model.frequency, &model.gain_db, c.traces[0]).cache_key(0xB0DE_0001));

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

    let response = plot::show(
        ui,
        &spec,
        &mut state.shell.results.cache,
        None,
        Some(&readout),
    );
    if response.view.any() {
        state
            .shell
            .results
            .plot_view_mut(super::ResultViewer::Bode, 0)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// The stability readout.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Stability", None);
    let Some(model) = build_model(state) else {
        super::panel_note(ui, "No AC analysis in the active run.");
        return;
    };
    let m = model.margins;

    let fmt_opt =
        |v: Option<f64>, f: &dyn Fn(f64) -> String| -> String { v.map_or("—".to_owned(), f) };
    let rows = [
        (
            "Phase margin",
            fmt_opt(m.pm_deg, &|v| format!("{v:.1}°")),
            true,
        ),
        (
            "Gain margin",
            fmt_opt(m.gm_db, &|v| format!("{v:.1} dB")),
            true,
        ),
        (
            "Unity-gain freq",
            fmt_opt(m.ugf, &|v| fmt_si(v, "Hz", 1)),
            false,
        ),
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
        super::panel_note(
            ui,
            "Margins measured on the simulated curves; the plot markers show the same values.",
        );
    }
}
