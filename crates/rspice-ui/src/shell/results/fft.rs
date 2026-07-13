//! FFT — the derived spectrum of the active transient, rendered as a thin
//! trace with harmonic markers; THD/SNR and the harmonics table live in the
//! right panel.

use std::sync::Arc;

use egui::Ui;

use crate::common::AppState;
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si, sample_at};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;

use super::strip::{self, LegendChip};
use super::well_hint;

/// Spectrum arrays + the analysis summary, with the arrays cached on the
/// FFT state's spectrum revision.
struct FftModel {
    subtitle: String,
    frequency: Arc<[f64]>,
    magnitude_db: Arc<[f64]>,
    fundamental: Option<(f64, f64)>,
    harmonics: Vec<(usize, f64, f64)>,
}

fn build_model(state: &mut AppState) -> Option<FftModel> {
    let fft = &state.analysis.fft_state;
    let data = fft.data.as_ref()?;
    if data.points.is_empty() {
        return None;
    }
    // Display arrays cached on the spectrum revision: a recompute (window,
    // size, source) replaces the entry rather than growing a map, and a
    // reused allocation can never serve stale arrays.
    let revision = fft.spectrum_revision();
    let series = &mut state.shell.results.fft_series;
    let series = match series {
        Some(series) if series.revision == revision => series,
        _ => series.insert(super::FftSeries {
            revision,
            frequency: data.points.iter().map(|p| p.frequency).collect(),
            magnitude_db: data.points.iter().map(|p| p.magnitude_db()).collect(),
            y_extremes: None,
        }),
    };
    let frequency = Arc::clone(&series.frequency);
    let magnitude_db = Arc::clone(&series.magnitude_db);

    let analysis = fft.analysis.as_ref();
    let fundamental = analysis.and_then(|a| Some((a.fundamental_frequency?, a.fundamental_db?)));
    // Harmonic list: order, frequency, dBc relative to the fundamental.
    let harmonics = match (analysis, fundamental) {
        (Some(a), Some((f0, db0))) if f0 > 0.0 => a
            .harmonics
            .iter()
            .filter(|(f, _)| *f > f0 * 1.5)
            .map(|&(f, db)| (((f / f0).round() as usize).max(2), f, db - db0))
            .take(6)
            .collect(),
        _ => Vec::new(),
    };

    Some(FftModel {
        subtitle: format!(
            "{} · {} · {} pts",
            data.name,
            data.window.display_name().to_lowercase(),
            data.fft_size
        ),
        frequency,
        magnitude_db,
        fundamental,
        harmonics,
    })
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the spectrum.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let Some(model) = build_model(state) else {
        well_hint(ui, "No spectrum yet — the FFT runs on the active transient");
        return;
    };

    let legend = [LegendChip {
        name: "dBV",
        color: c.traces[0],
        on: true,
    }];
    let view = state.shell.results.plot_view(super::ResultViewer::Fft, 0);
    let header = strip::StripHeader::new("FFT", &model.subtitle, &legend)
        .zoomed(view.is_zoomed())
        .show(ui);
    if header.fit_clicked {
        state
            .shell
            .results
            .reset_plot_view(super::ResultViewer::Fft, 0);
    }

    // Window-function selector: the leakage/resolution tradeoff is an
    // analysis decision, so it lives on the strip, not in a buried dialog.
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("window")
                .font(crate::ui::theme::mono(
                    crate::ui::tokens::FS_0,
                    crate::ui::theme::FontWeight::Medium,
                ))
                .color(c.text_dim),
        );
        let current = state.analysis.fft_state.window;
        egui::ComboBox::from_id_salt("volta.fft.window")
            .selected_text(current.display_name())
            .show_ui(ui, |ui| {
                for &window in crate::analysis::fft::WindowFunction::all() {
                    if ui
                        .selectable_label(window == current, window.display_name())
                        .clicked()
                    {
                        state.analysis.fft_state.set_window(window);
                    }
                }
            });
    });
    ui.add_space(2.0);

    // X: linear; zoom to ~11 harmonics when a fundamental is known.
    let data_max = *model.frequency.last().unwrap_or(&1.0);
    let x1 = match model.fundamental {
        Some((f0, _)) if f0 > 0.0 => (f0 * 11.0).min(data_max),
        _ => data_max,
    };
    if !matches!(x1.partial_cmp(&0.0), Some(std::cmp::Ordering::Greater)) {
        well_hint(ui, "Degenerate spectrum");
        return;
    }

    // Y: from the data within view, floored sensibly; the scan is cached on
    // (spectrum revision, x1) so it never repeats per frame.
    let x1_bits = x1.to_bits();
    let extremes = match state.shell.results.fft_series.as_mut() {
        Some(series) => match series.y_extremes {
            Some((bits, lo, hi)) if bits == x1_bits => Some((lo, hi)),
            _ => {
                let end = model.frequency.partition_point(|&f| f <= x1).max(1);
                let computed = super::finite_extremes(&model.magnitude_db[..end]);
                if let Some((lo, hi)) = computed {
                    series.y_extremes = Some((x1_bits, lo, hi));
                }
                computed
            }
        },
        None => None,
    };
    let Some((lo, hi)) = extremes else {
        well_hint(ui, "Degenerate spectrum");
        return;
    };
    let (y_lo, y_hi) = view.y.unwrap_or(((lo - 8.0).max(-200.0), hi + 12.0));
    let y = Axis::linear_with(y_lo, y_hi, "dBV", 7);

    let (x_lo, x_hi) = view.x.unwrap_or((0.0, x1));
    let mut spec = PlotSpec::new(Axis::linear(x_lo, x_hi, "Hz"), XScale::Linear, y)
        .accessible_name("FFT magnitude plot");
    spec.left_margin = 60.0;
    spec.traces.push(
        Trace::new(&model.frequency, &model.magnitude_db, c.traces[0])
            .thin()
            .cache_key(0xFF7_0001),
    );

    if let Some((f0, db0)) = model.fundamental {
        spec.markers.push(plot::Marker {
            x: f0,
            y: db0,
            side: plot::YSide::Left,
            color: c.accent,
            label: format!("f₀ {:.1} dBV", db0),
            drop_line: false,
            label_dy: 0.0,
        });
    }
    for (i, &(order, f, dbc)) in model.harmonics.iter().take(4).enumerate() {
        let level = sample_at(&model.frequency, &model.magnitude_db, f);
        spec.markers.push(plot::Marker {
            x: f,
            y: level,
            side: plot::YSide::Left,
            color: c.traces[2],
            label: format!("HD{order} {dbc:.1} dBc"),
            drop_line: false,
            // Stagger alternating tags so neighbours never collide.
            label_dy: if i % 2 == 0 { -14.0 } else { 26.0 },
        });
    }

    let readout = |x: f64| -> Vec<(String, String)> {
        vec![
            ("f".to_owned(), fmt_si(x, "Hz", 1)),
            (
                "level".to_owned(),
                format!(
                    "{:.1} dBV",
                    sample_at(&model.frequency, &model.magnitude_db, x)
                ),
            ),
        ]
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
            .plot_view_mut(super::ResultViewer::Fft, 0)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// THD/SNR summary + harmonics table.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Spectrum", None);
    let Some(analysis) = state.analysis.fft_state.analysis.clone() else {
        super::panel_note(ui, "Spectrum metrics appear once the FFT is computed.");
        return;
    };

    let f = |v: Option<f64>, unit: &str, digits: usize| {
        v.map_or("—".to_owned(), |v| fmt_si(v, unit, digits))
    };
    let db = |v: Option<f64>| v.map_or("—".to_owned(), |v| format!("{v:.1} dB"));

    let enob = analysis
        .sinad_db
        .map(|sinad| format!("{:.1} bit", (sinad - 1.76) / 6.02));
    let thd = analysis
        .thd_percent
        .map_or("—".to_owned(), |v| format!("{v:.3} %"));

    let rows = [
        (
            "Fundamental",
            f(analysis.fundamental_frequency, "Hz", 1),
            false,
        ),
        (
            "Amplitude",
            db(analysis.fundamental_db).replace(" dB", " dBV"),
            false,
        ),
        ("THD", thd, true),
        ("SNR", db(analysis.snr_db), false),
        ("SINAD", db(analysis.sinad_db), false),
        ("SFDR", db(analysis.sfdr_db), false),
        ("ENOB", enob.unwrap_or_else(|| "—".to_owned()), false),
        (
            "Noise floor",
            db(analysis.noise_floor_db).replace(" dB", " dBV"),
            false,
        ),
    ];
    super::stat_table(ui, &rows);

    if let Some(f0) = analysis.fundamental_frequency {
        let db0 = analysis.fundamental_db.unwrap_or(0.0);
        let harmonic_rows: Vec<(String, String)> = analysis
            .harmonics
            .iter()
            .filter(|(f, _)| *f > f0 * 1.5)
            .take(6)
            .map(|&(f, level)| {
                let order = ((f / f0).round() as usize).max(2);
                (
                    format!("HD{order} · {}", fmt_si(f, "Hz", 0)),
                    format!("{:.1} dBc", level - db0),
                )
            })
            .collect();
        if !harmonic_rows.is_empty() {
            section_header(ui, "Harmonics", None);
            let rows: Vec<(&str, String, bool)> = harmonic_rows
                .iter()
                .map(|(k, v)| (k.as_str(), v.clone(), false))
                .collect();
            super::stat_table(ui, &rows);
        }
    }
}
