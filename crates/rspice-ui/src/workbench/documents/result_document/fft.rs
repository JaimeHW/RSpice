//! FFT — the derived spectrum of the active transient, rendered as a thin
//! trace with harmonic markers; THD/SNR and the harmonics table live in the
//! right panel.

use std::sync::Arc;

use egui::Ui;

use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, sample_at};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::strip::{self, LegendChip};
use super::well_hint;

/// Spectrum arrays + the analysis summary, with the arrays cached on the
/// FFT state's spectrum revision.
struct FftModel {
    revision: u64,
    subtitle: String,
    source: String,
    window: crate::analysis::fft::WindowFunction,
    fft_size: usize,
    resolution_bandwidth: Option<f64>,
    normalization: crate::analysis::fft::data::SpectrumNormalization,
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
    let series = &mut state.ui.results.fft_series;
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
        revision,
        subtitle: spectrum_subtitle(
            fft.source_cache
                .as_ref()
                .map_or(data.name.as_str(), |source| source.name.as_str()),
            data.window,
            data.fft_size,
        ),
        source: fft
            .source_cache
            .as_ref()
            .map_or_else(|| data.name.clone(), |source| source.name.clone()),
        window: data.window,
        fft_size: data.fft_size,
        resolution_bandwidth: resolution_bandwidth(data),
        normalization: data.normalization,
        frequency,
        magnitude_db,
        fundamental,
        harmonics,
    })
}

/// Reference-aware logarithmic amplitude unit for the retained source.
///
/// `FftSourceCache` currently retains the source identity rather than a typed
/// unit, so recognize only canonical SPICE quantity expressions. Unknown and
/// derived names deliberately fall back to a generic dB label instead of
/// making a false voltage assertion.
fn spectrum_level_unit(
    source: &str,
    normalization: crate::analysis::fft::data::SpectrumNormalization,
) -> &'static str {
    let source = source.trim();
    let quantity = if source
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
    {
        "V"
    } else if source
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("I("))
    {
        "A"
    } else {
        ""
    };
    match (quantity, normalization) {
        ("V", crate::analysis::fft::data::SpectrumNormalization::Peak) => "dBV pk",
        ("V", crate::analysis::fft::data::SpectrumNormalization::Rms) => "dBV rms",
        // Not "dBA": in every other instrument that prints it, dBA is an
        // A-weighted sound level. A current spectrum referenced to one ampere
        // has to say so.
        ("A", crate::analysis::fft::data::SpectrumNormalization::Peak) => "dB re 1 A pk",
        ("A", crate::analysis::fft::data::SpectrumNormalization::Rms) => "dB re 1 A rms",
        (_, crate::analysis::fft::data::SpectrumNormalization::Peak) => "dB pk",
        (_, crate::analysis::fft::data::SpectrumNormalization::Rms) => "dB rms",
    }
}

/// The abscissa has no positive extent, so there is no spectrum to rule.
///
/// Both refusals used to print "Degenerate spectrum", which named the fault
/// twice and told the reader nothing about which of the two they had. They
/// have different causes and different remedies.
const NO_FREQUENCY_SPAN: &str = "No frequency span — the transform produced no positive bin";
/// Every level in view is non-finite, so there is no ordinate to rule.
const NO_FINITE_LEVEL: &str = "No finite level in view — every magnitude in this band is NaN";

/// The strip subtitle: what was transformed, how, and at what length.
///
/// `fft_size` is the length of the transform, not the count of points on
/// screen: a 1024-point transform plots 513 bins. Calling it "points" beside
/// a plotted spectrum invited exactly the reading it is not.
fn spectrum_subtitle(
    source: &str,
    window: crate::analysis::fft::WindowFunction,
    fft_size: usize,
) -> String {
    format!(
        "{source} · {} · {fft_size}-point transform",
        window.display_name().to_lowercase()
    )
}

/// What the legend chip beside the trace names.
///
/// The quantity that is plotted, which is the signal the transform read. The
/// chip carried the level unit, which the ordinate axis already states — so
/// the legend repeated the axis and the sheet never named its own source.
fn legend_name<'a>(source: &'a str, level_unit: &'a str) -> &'a str {
    let _ = level_unit;
    source
}

/// How the panel labels the transform length.
fn transform_size_row(fft_size: usize) -> (&'static str, String) {
    ("Transform size", format!("{fft_size} samples"))
}

fn trace_cache_key(revision: u64) -> u64 {
    0x0FF7_0001_0000_0000_u64 ^ revision.rotate_left(17)
}

fn resolution_bandwidth(data: &crate::analysis::fft::data::FftData) -> Option<f64> {
    let bandwidth = data.resolution_bandwidth();
    (bandwidth.is_finite() && bandwidth > 0.0).then_some(bandwidth)
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the spectrum.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let Some(model) = build_model(state) else {
        well_hint(ui, "No spectrum yet — the FFT runs on the active transient");
        return;
    };
    let level_unit = spectrum_level_unit(&model.source, model.normalization);

    let legend = [LegendChip {
        name: legend_name(&model.source, level_unit),
        color: c.traces[0],
        on: true,
    }];
    let view = state.ui.results.plot_view(super::ResultViewer::Fft, 0);
    let header = strip::StripHeader::new("FFT", &model.subtitle, &legend)
        .zoomed(view.is_zoomed())
        .show(ui);
    if header.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::Fft, 0);
    }

    // Window and amplitude normalization are transform decisions. Both
    // recompute from the cached uniformly sampled source and therefore stay
    // coupled to the provenance shown by the inspector.
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
        ui.add_enabled_ui(state.analysis.fft_state.source_cache.is_some(), |ui| {
            let current = state.analysis.fft_state.window;
            egui::ComboBox::from_id_salt("rspice.fft.window")
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
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("normalization")
                .font(crate::ui::theme::mono(
                    crate::ui::tokens::FS_0,
                    crate::ui::theme::FontWeight::Medium,
                ))
                .color(c.text_dim),
        );
        let current = state.analysis.fft_state.normalization;
        egui::ComboBox::from_id_salt("rspice.fft.normalization")
            .selected_text(current.display_name())
            .show_ui(ui, |ui| {
                for &normalization in crate::analysis::fft::data::SpectrumNormalization::all() {
                    if ui
                        .selectable_label(normalization == current, normalization.display_name())
                        .clicked()
                    {
                        state.analysis.fft_state.set_normalization(normalization);
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
        well_hint(ui, NO_FREQUENCY_SPAN);
        return;
    }

    // Y: from the data within view, floored sensibly; the scan is cached on
    // (spectrum revision, x1) so it never repeats per frame.
    let x1_bits = x1.to_bits();
    let extremes = match state.ui.results.fft_series.as_mut() {
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
        well_hint(ui, NO_FINITE_LEVEL);
        return;
    };
    let (y_lo, y_hi) = view.y.unwrap_or(((lo - 8.0).max(-200.0), hi + 12.0));
    let y = Axis::linear_with(y_lo, y_hi, level_unit, 7);

    let (x_lo, x_hi) = view.x.unwrap_or((0.0, x1));
    let (frequency_scale, frequency_offset, frequency_unit) =
        quantity_policy.frequency_axis_transform();
    let x_axis = Axis::linear(x_lo, x_hi, "Hz").with_display_transform(
        frequency_scale,
        frequency_offset,
        frequency_unit,
    );
    let mut spec = PlotSpec::new(x_axis, XScale::Linear, y).accessible_name("FFT magnitude plot");
    spec.left_margin = 60.0;
    spec.traces.push(
        Trace::new(&model.frequency, &model.magnitude_db, c.traces[0])
            .thin()
            .cache_key(trace_cache_key(model.revision)),
    );

    if let Some((f0, db0)) = model.fundamental {
        spec.markers.push(plot::Marker {
            x: f0,
            y: db0,
            color: c.accent,
            label: format!("f₀ {:.1} {level_unit}", db0),
            drop_line: false,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }
    for (i, &(order, f, dbc)) in model.harmonics.iter().take(4).enumerate() {
        let level = sample_at(&model.frequency, &model.magnitude_db, f);
        spec.markers.push(plot::Marker {
            x: f,
            y: level,
            color: c.traces[2],
            label: format!("HD{order} {dbc:.1} dBc"),
            drop_line: false,
            // Stagger alternating tags so neighbours never collide.
            label_dy: if i % 2 == 0 { -14.0 } else { 26.0 },
            shape: plot::MarkerShape::Point,
        });
    }

    let readout = |x: f64| -> Vec<(String, String)> {
        vec![
            ("f".to_owned(), quantity_policy.format_frequency(x, 1)),
            (
                "level".to_owned(),
                format!(
                    "{:.1} {}",
                    sample_at(&model.frequency, &model.magnitude_db, x),
                    level_unit
                ),
            ),
        ]
    };

    let response = plot::show(ui, &spec, &mut state.ui.results.cache, None, Some(&readout));
    super::record_drawn_axes(&mut state.ui.results, super::ResultViewer::Fft, &response);
    if response.view.any() {
        state
            .ui
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
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let Some(model) = build_model(state) else {
        super::panel_note(ui, "Spectrum metrics appear once the FFT is computed.");
        return;
    };
    let Some(analysis) = state.analysis.fft_state.analysis.clone() else {
        super::panel_note(ui, "Spectrum metrics appear once the FFT is computed.");
        return;
    };
    let level_unit = spectrum_level_unit(&model.source, model.normalization);

    let f = |v: Option<f64>, digits: usize| {
        v.map_or("—".to_owned(), |v| {
            quantity_policy.format_frequency(v, digits)
        })
    };
    let db = |v: Option<f64>| v.map_or("—".to_owned(), |v| format!("{v:.1} dB"));

    let enob = analysis
        .sinad_db
        .map(|sinad| format!("{:.1} bit", (sinad - 1.76) / 6.02));
    let thd = analysis.thd_percent.map_or("—".to_owned(), |value| {
        format!("{value:.3} % · {} harmonics", analysis.harmonics.len())
    });
    let amplitude = analysis
        .fundamental_db
        .map_or("—".to_owned(), |value| format!("{value:.1} {level_unit}"));
    let resolution_bandwidth = model.resolution_bandwidth.map_or("—".to_owned(), |value| {
        quantity_policy.format_frequency(value, 3)
    });
    let noise_floor = match (analysis.noise_floor_db, analysis.fundamental_db) {
        (Some(noise), Some(fundamental)) => format!("{:.1} dBc", noise - fundamental),
        _ => "—".to_owned(),
    };

    let (transform_label, transform_value) = transform_size_row(model.fft_size);
    let rows = [
        ("Source", model.source, false),
        ("Window", model.window.display_name().to_owned(), false),
        (transform_label, transform_value, false),
        ("Resolution BW", resolution_bandwidth, false),
        (
            "Normalization",
            model.normalization.display_name().to_owned(),
            false,
        ),
        ("Fundamental", f(analysis.fundamental_frequency, 1), false),
        ("Amplitude", amplitude, false),
        ("THD", thd, true),
        ("SNR", db(analysis.snr_db), false),
        ("SINAD", db(analysis.sinad_db), false),
        ("SFDR", db(analysis.sfdr_db), false),
        ("ENOB", enob.unwrap_or_else(|| "—".to_owned()), false),
        ("Noise floor", noise_floor, false),
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
                    format!("HD{order} · {}", quantity_policy.format_frequency(f, 0)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::fft::WindowFunction;
    use crate::analysis::fft::data::{FftData, SpectrumNormalization};

    #[test]
    fn rectangular_resolution_bandwidth_equals_bin_width() {
        let data = FftData::from_time_domain_with_normalization(
            "V(out)",
            &[0.0; 64],
            6_400.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Rms,
        );
        assert!((resolution_bandwidth(&data).unwrap() - 100.0).abs() < 1.0e-12);
    }

    #[test]
    fn window_resolution_bandwidth_includes_equivalent_noise_bandwidth() {
        let data = FftData::from_time_domain_with_normalization(
            "V(out)",
            &[0.0; 1024],
            1024.0,
            WindowFunction::BlackmanHarris,
            SpectrumNormalization::Rms,
        );
        let bandwidth = resolution_bandwidth(&data).unwrap();
        assert!(bandwidth > data.frequency_resolution());
        assert!(bandwidth < 2.1 * data.frequency_resolution());
    }

    #[test]
    fn spectrum_level_units_never_call_current_or_unknown_sources_voltage() {
        assert_eq!(
            spectrum_level_unit("V(out)", SpectrumNormalization::Peak),
            "dBV pk"
        );
        assert_eq!(
            spectrum_level_unit("i(VSENSE)", SpectrumNormalization::Rms),
            "dB re 1 A rms"
        );
        assert_eq!(
            spectrum_level_unit("derived_expression", SpectrumNormalization::Peak),
            "dB pk"
        );
    }

    #[test]
    fn spectrum_revisions_never_share_a_trace_cache_identity() {
        assert_ne!(trace_cache_key(41), trace_cache_key(42));
    }

    /// The number beside a spectrum is the transform length, not the count
    /// of points drawn — a 1024-point transform plots 513 bins. Both the
    /// strip and the panel called it "points", which is the one reading the
    /// number does not have.
    #[test]
    fn the_transform_length_is_never_called_a_count_of_plotted_points() {
        let subtitle = spectrum_subtitle("V(out)", WindowFunction::BlackmanHarris, 1024);
        assert!(subtitle.contains("1024"), "{subtitle}");
        assert!(
            !subtitle.contains("points"),
            "the strip calls the transform length a count of plotted points: {subtitle}"
        );

        let (label, value) = transform_size_row(1024);
        assert_ne!(
            label, "Points",
            "the panel calls the transform length a count of plotted points"
        );
        assert!(value.contains("1024"), "{value}");
    }

    /// The legend names what is plotted. It named the unit instead, which
    /// the ordinate axis already carries — so the chip repeated the axis and
    /// the sheet never said which signal the spectrum came from.
    #[test]
    fn the_legend_names_the_transformed_signal_rather_than_repeating_the_axis() {
        assert_eq!(legend_name("V(out)", "dBV rms"), "V(out)");
    }

    /// Two different refusals read as one.
    ///
    /// "Degenerate spectrum" was printed both when the abscissa had no
    /// positive extent and when every level in view was non-finite. They are
    /// different faults with different remedies, and the reader was given no
    /// way to tell which one they were looking at.
    #[test]
    fn the_two_degenerate_spectra_are_told_apart() {
        assert_ne!(NO_FREQUENCY_SPAN, NO_FINITE_LEVEL);
    }
}
