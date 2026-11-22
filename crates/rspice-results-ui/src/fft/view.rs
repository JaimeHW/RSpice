//! FFT — the derived spectrum of the active transient, rendered as a thin
//! trace with harmonic markers, its phase on a strip of its own when asked;
//! the figures, the harmonics and the peaks live in the right panel.

use std::sync::Arc;

use egui::Ui;

use super::FftState;
use crate::presentation::{PlotView, frequency_readout, with_table_state};
use crate::waves::ReadoutPolicy;
use rspice_results::fft::settings::SpectrumScale;
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::fmt_si_significant;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, sample_at};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::table::{Align, Cell, DataTable, SortKey};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use crate::presentation::well_hint;
use crate::strip::{self, LegendChip};

mod bar;
pub(crate) mod columns;
pub use bar::{RecordCursors, domain_bar};
use columns::{Fitting, HUNDREDTH};

/// The arrays the active FFT draws, keyed on the FFT state's spectrum
/// revision and the scale they are drawn in — the FFT can be recomputed
/// (window, size, source) without a simulation data-version bump, and keying
/// on the data's allocation address could serve stale arrays after a
/// reallocation.
#[derive(Debug, Clone)]
pub struct FftSeries {
    revision: u64,
    scale: SpectrumScale,
    frequency: std::sync::Arc<[f64]>,
    /// The ordinate in the drawn scale.
    level: std::sync::Arc<[f64]>,
    /// Each bin's phase, in degrees.
    phase: std::sync::Arc<[f64]>,
    /// Cached finite (lo, hi) of the level within view: (x1 bits, lo, hi).
    y_extremes: Option<(u64, f64, f64)>,
}

/// Local presentation outcomes for the host to apply after drawing.
#[derive(Default)]
pub struct FftViewResponse {
    pub plot: Option<plot::PlotResponse>,
    /// Fit clears all pinned panes for this FFT plot in the host document.
    pub fit_requested: bool,
    /// Retained samples scanned for axis bounds this frame; zero on a cache hit.
    pub range_samples_read: usize,
}

/// Spectrum arrays + the analysis summary, with the arrays cached on the
/// FFT state's spectrum revision.
struct FftModel {
    revision: u64,
    source: String,
    /// The transform length: how many samples of the record it read.
    fft_size: usize,
    resolution_bandwidth: Option<f64>,
    normalization: crate::fft::data::SpectrumNormalization,
    scale: SpectrumScale,
    frequency: Arc<[f64]>,
    level: Arc<[f64]>,
    phase: Arc<[f64]>,
    /// The fundamental's frequency and its level in the drawn scale.
    fundamental: Option<(f64, f64)>,
    /// Each measured harmonic: order, frequency and dBc.
    harmonics: Vec<(usize, f64, f64)>,
}

/// One bin's level in `scale`: dB of the unit, the magnitude itself, or the
/// power spectral density in dB of the unit per root hertz, an rms density
/// whatever the amplitude convention, over the window's noise bandwidth.
fn scaled_level(
    magnitude: f64,
    scale: SpectrumScale,
    normalization: crate::fft::data::SpectrumNormalization,
    resolution_bandwidth: Option<f64>,
) -> f64 {
    match scale {
        SpectrumScale::Magnitude => magnitude,
        SpectrumScale::Db => 20.0 * magnitude.log10(),
        SpectrumScale::Psd => {
            let rms = match normalization {
                crate::fft::data::SpectrumNormalization::Peak => {
                    magnitude * std::f64::consts::FRAC_1_SQRT_2
                }
                crate::fft::data::SpectrumNormalization::Rms => magnitude,
            };
            resolution_bandwidth.map_or(f64::NAN, |bandwidth| {
                20.0 * rms.log10() - 10.0 * bandwidth.log10()
            })
        }
    }
}

fn build_model(fft: &FftState, series: &mut Option<FftSeries>) -> Option<FftModel> {
    if !fft.has_data() {
        return None;
    }
    let data = fft.data.as_ref()?;
    let analysis = fft.analysis.as_ref()?;
    if data.points.is_empty() {
        return None;
    }
    let bandwidth = resolution_bandwidth(data);
    // Display arrays cached on the spectrum revision and scale: a recompute
    // (window, size, source) replaces the entry rather than growing a map,
    // and a reused allocation can never serve stale arrays.
    let revision = fft.spectrum_revision();
    let scale = fft.scale;
    let series = match series {
        Some(series) if series.revision == revision && series.scale == scale => series,
        _ => series.insert(FftSeries {
            revision,
            scale,
            frequency: data.points.iter().map(|p| p.frequency).collect(),
            level: data
                .points
                .iter()
                .map(|p| scaled_level(p.magnitude, scale, data.normalization, bandwidth))
                .collect(),
            phase: data.points.iter().map(|p| p.phase.to_degrees()).collect(),
            y_extremes: None,
        }),
    };
    let frequency = Arc::clone(&series.frequency);
    let level = Arc::clone(&series.level);
    let phase = Arc::clone(&series.phase);

    let fundamental_db = analysis.fundamental_frequency.zip(analysis.fundamental_db);
    // Every measured harmonic: order, frequency, dBc relative to the
    // fundamental.
    let harmonics = match fundamental_db {
        Some((f0, db0)) if f0 > 0.0 => analysis
            .harmonics
            .iter()
            .filter(|harmonic| harmonic.above_floor)
            .map(|harmonic| (harmonic.order, harmonic.frequency, harmonic.level_db - db0))
            .collect(),
        Some(_) | None => Vec::new(),
    };
    let fundamental = fundamental_db.map(|(f0, db0)| {
        let magnitude = 10.0_f64.powf(db0 / 20.0);
        (
            f0,
            scaled_level(magnitude, scale, data.normalization, bandwidth),
        )
    });

    Some(FftModel {
        revision,
        fft_size: data.fft_size,
        source: fft
            .source_cache
            .as_ref()
            .map_or_else(|| data.name.clone(), |source| source.name.clone()),
        resolution_bandwidth: bandwidth,
        normalization: data.normalization,
        scale,
        frequency,
        level,
        phase,
        fundamental,
        harmonics,
    })
}

/// The quantity a canonical SPICE source name measures: `V` for a voltage,
/// `A` for a current, nothing for anything else.
fn source_quantity(source: &str) -> &'static str {
    let source = source.trim();
    if source
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
    }
}

/// Reference-aware logarithmic amplitude unit for the retained source.
///
/// `FftSourceCache` currently retains the source identity rather than a typed
/// unit, so recognize only canonical SPICE quantity expressions. Unknown and
/// derived names deliberately fall back to a generic dB label instead of
/// making a false voltage assertion.
fn spectrum_level_unit(
    source: &str,
    normalization: crate::fft::data::SpectrumNormalization,
) -> &'static str {
    match (source_quantity(source), normalization) {
        ("V", crate::fft::data::SpectrumNormalization::Peak) => "dBV pk",
        ("V", crate::fft::data::SpectrumNormalization::Rms) => "dBV rms",
        // Not "dBA": in every other instrument that prints it, dBA is an
        // A-weighted sound level. A current spectrum referenced to one ampere
        // has to say so.
        ("A", crate::fft::data::SpectrumNormalization::Peak) => "dB re 1 A pk",
        ("A", crate::fft::data::SpectrumNormalization::Rms) => "dB re 1 A rms",
        (_, crate::fft::data::SpectrumNormalization::Peak) => "dB pk",
        (_, crate::fft::data::SpectrumNormalization::Rms) => "dB rms",
    }
}

/// The unit the ordinate is drawn in, for `scale`.
fn ordinate_unit(
    source: &str,
    normalization: crate::fft::data::SpectrumNormalization,
    scale: SpectrumScale,
) -> &'static str {
    match (scale, source_quantity(source)) {
        (SpectrumScale::Db, _) => spectrum_level_unit(source, normalization),
        (SpectrumScale::Magnitude, quantity) => quantity,
        (SpectrumScale::Psd, "V") => "dBV/√Hz",
        (SpectrumScale::Psd, "A") => "dB re 1 A/√Hz",
        (SpectrumScale::Psd, _) => "dB/√Hz",
    }
}

/// The abscissa has no positive extent, so there is no spectrum to rule.
///
/// Both refusals used to print "Degenerate spectrum", which named the fault
/// twice and told the reader nothing about which of the two they had. They
/// have different causes and different remedies.
const NO_FREQUENCY_SPAN: &str = "No frequency span — the transform produced no positive bin";
/// No level in view is a number, so there is no ordinate to rule.
const NO_FINITE_LEVEL: &str = "No level to plot — the spectrum in this band is undefined";
/// Every bin in view is exactly zero: the record itself is a flat zero.
const ZERO_SIGNAL: &str = "The signal is zero — its spectrum has no level to plot";

/// The record the transform read, as the Spectrum table states it.
///
/// `fft_size` is the length of the transform, not the count of points on
/// screen: a 1024-sample transform plots 513 bins. Calling it "points" beside
/// a plotted spectrum invited exactly the reading it is not.
fn record_length(fft_size: usize) -> String {
    format!("{fft_size} samples")
}

fn trace_cache_key(revision: u64) -> u64 {
    0x0FF7_0001_0000_0000_u64 ^ revision.rotate_left(17)
}

fn resolution_bandwidth(data: &crate::fft::data::FftData) -> Option<f64> {
    let bandwidth = data.resolution_bandwidth();
    (bandwidth.is_finite() && bandwidth > 0.0).then_some(bandwidth)
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// The share of the sheet's height the phase strip takes when it is drawn.
const PHASE_SHARE: f32 = 0.34;

/// Render the spectrum; its cursor reads at the reader's significant digits
/// and by the reader's cursor interpolation.
#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut Ui,
    fft: &mut FftState,
    series: &mut Option<FftSeries>,
    view: PlotView,
    cache: &mut plot::DecimationCache,
    quantity_policy: &QuantityPresentationPolicy,
    presentation: ReadoutPolicy,
    pin: &mut Option<f64>,
) -> FftViewResponse {
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let mut out = FftViewResponse::default();
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let Some(model) = build_model(fft, series) else {
        if let Some(error) = fft.last_error.as_ref() {
            well_hint(ui, &format!("Spectrum unavailable — {error}"));
        } else {
            well_hint(ui, "No spectrum yet — the FFT runs on the active transient");
        }
        return out;
    };
    let level_unit = ordinate_unit(&model.source, model.normalization, model.scale);

    // The legend names the quantity that is plotted, which is the signal the
    // transform read. Nothing follows the kind: the window is the bar's
    // select and the transform length is the Spectrum table's first row.
    let legend = [LegendChip {
        name: &model.source,
        color: c.traces[0],
        on: true,
    }];
    let header = strip::StripHeader::new("FFT", "", &legend)
        .zoomed(view.is_zoomed())
        .maximizable(false)
        .show(ui);
    if header.fit_clicked {
        out.fit_requested = true;
    }

    // X: zoom to ~11 harmonics when a fundamental is known; a logarithmic
    // axis starts at the first bin above DC.
    let data_max = *model.frequency.last().unwrap_or(&1.0);
    let x1 = match model.fundamental {
        Some((f0, _)) if f0 > 0.0 && !fft.log_frequency => (f0 * 11.0).min(data_max),
        _ => data_max,
    };
    if !matches!(x1.partial_cmp(&0.0), Some(std::cmp::Ordering::Greater)) {
        well_hint(ui, NO_FREQUENCY_SPAN);
        return out;
    }
    let first_bin = model.frequency.get(1).copied().unwrap_or(x1 / 2.0);

    // Y: from the data within view, floored sensibly; the scan is cached on
    // (spectrum revision, scale, x1) so it never repeats per frame.
    let x1_bits = x1.to_bits();
    let extremes = match series.as_mut() {
        Some(series) => match series.y_extremes {
            Some((bits, lo, hi)) if bits == x1_bits => Some((lo, hi)),
            _ => {
                let end = model.frequency.partition_point(|&f| f <= x1).max(1);
                out.range_samples_read = end;
                let computed = rspice_results::measurements::finite_extremes(&model.level[..end]);
                if let Some((lo, hi)) = computed {
                    series.y_extremes = Some((x1_bits, lo, hi));
                }
                computed
            }
        },
        None => None,
    };
    // A magnitude of zero is finite, so a silent record drawn as magnitudes
    // has extremes; it has no level to plot all the same.
    let extremes = extremes.filter(|&(_, hi)| model.scale != SpectrumScale::Magnitude || hi > 0.0);
    let Some((lo, hi)) = extremes else {
        let end = model.frequency.partition_point(|&f| f <= x1).max(1);
        let silent = model.level[..end.min(model.level.len())]
            .iter()
            .all(|level| *level == f64::NEG_INFINITY || *level == 0.0);
        well_hint(ui, if silent { ZERO_SIGNAL } else { NO_FINITE_LEVEL });
        return out;
    };
    // A magnitude stands on zero; a level takes the plot's fit headroom.
    let (y_lo, y_hi) = view.y.unwrap_or_else(|| match model.scale {
        SpectrumScale::Magnitude => (0.0, plot::padded_fit((0.0, hi)).1),
        SpectrumScale::Db | SpectrumScale::Psd => plot::padded_fit((lo.max(-200.0), hi)),
    });
    let y = Axis::linear_with(y_lo, y_hi, level_unit, 7);

    let (frequency_scale, frequency_offset, frequency_unit) =
        quantity_policy.frequency_axis_transform();
    let (x_lo, x_hi) = view.x.unwrap_or(if fft.log_frequency {
        (first_bin, x1)
    } else {
        (0.0, x1)
    });
    let (x_axis, x_scale) = if fft.log_frequency {
        (
            Axis::log_decades(x_lo.max(first_bin), x_hi, "Hz"),
            XScale::Log10,
        )
    } else {
        (Axis::linear(x_lo, x_hi, "Hz"), XScale::Linear)
    };
    let x_axis = x_axis.with_display_transform(frequency_scale, frequency_offset, frequency_unit);
    let mut spec = PlotSpec::new(x_axis.clone(), x_scale, y).accessible_name("FFT magnitude plot");
    spec.traces.push(
        Trace::new(&model.frequency, &model.level, c.traces[0])
            .thin()
            .cache_key(trace_cache_key(model.revision)),
    );

    if let Some((f0, level0)) = model.fundamental {
        spec.markers.push(plot::Marker {
            x: f0,
            y: level0,
            color: c.accent,
            label: format!("f₀ {}", fmt_si_significant(level0, level_unit, digits)),
            drop_line: false,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }
    for (i, &(order, f, dbc)) in model.harmonics.iter().enumerate() {
        let level = sample_at(&model.frequency, &model.level, f);
        spec.markers.push(plot::Marker {
            x: f,
            y: level,
            color: c.traces[2],
            label: format!("HD{order} {}", fmt_si_significant(dbc, "dBc", digits)),
            drop_line: false,
            // Stagger alternating tags so neighbours never collide.
            label_dy: if i % 2 == 0 { -14.0 } else { 26.0 },
            shape: plot::MarkerShape::Point,
        });
    }

    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![
            (
                "f".to_owned(),
                frequency_readout(quantity_policy, x, digits),
            ),
            (
                "level".to_owned(),
                fmt_si_significant(
                    presentation.cursor_sample(&model.frequency, &model.level, x),
                    level_unit,
                    digits,
                ),
            ),
        ];
        if fft.phase {
            rows.push((
                "phase".to_owned(),
                fmt_si_significant(sample_at(&model.frequency, &model.phase, x), "°", digits),
            ));
        }
        rows
    };

    if fft.phase {
        let level_height = (ui.available_height() * (1.0 - PHASE_SHARE)).max(120.0);
        let response = ui
            .allocate_ui(egui::vec2(ui.available_width(), level_height), |ui| {
                plot::show(ui, &spec, cache, None, Some(&readout))
            })
            .inner;
        crate::chrome::pinned_readout::follow(ui, &response, spec.x_scale, pin, &readout);
        let mut phase_spec =
            PlotSpec::new(x_axis, x_scale, Axis::linear_with(-180.0, 180.0, "°", 5))
                .accessible_name("FFT phase plot");
        phase_spec.traces.push(
            Trace::new(&model.frequency, &model.phase, c.traces[1])
                .thin()
                .cache_key(trace_cache_key(model.revision) ^ 0x5048),
        );
        plot::show(ui, &phase_spec, cache, None, Some(&readout));
        out.plot = Some(response);
        return out;
    }
    let response = plot::show(ui, &spec, cache, None, Some(&readout));
    crate::chrome::pinned_readout::follow(ui, &response, spec.x_scale, pin, &readout);
    out.plot = Some(response);
    out
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Why THD, SNR and SINAD are absent, in the few words a details row holds,
/// and the whole reason for its hover.
fn absent_thd(coverage: crate::fft::data::HarmonicCoverage) -> Option<(String, String)> {
    match coverage {
        crate::fft::data::HarmonicCoverage::InsufficientBandwidth {
            analyzed_through, ..
        } => Some((
            format!("— Nyquist at HD{analyzed_through}"),
            format!(
                "The record's Nyquist frequency falls before the harmonics asked for: only \
                 HD2 through HD{analyzed_through} lie below it, so THD, SNR and SINAD are \
                 withheld. Sample faster or ask for fewer harmonics."
            ),
        )),
        crate::fft::data::HarmonicCoverage::InsufficientResolution { failed_order, .. } => Some((
            format!("— HD{failed_order} unresolved"),
            format!(
                "HD{failed_order} falls inside the carrier's main lobe, so it cannot be \
                     told from the carrier and THD, SNR and SINAD are withheld. Lengthen the \
                     record or choose a narrower window."
            ),
        )),
        crate::fft::data::HarmonicCoverage::NoFundamental { .. } => Some((
            "— no fundamental".to_owned(),
            "The spectrum has no tone to measure the harmonics against.".to_owned(),
        )),
        crate::fft::data::HarmonicCoverage::Complete { .. } => None,
    }
}

/// The spectral peaks rising this far over the mean noise floor are listed.
const PEAK_RISE_DB: f64 = 20.0;

/// Each local maximum of the spectrum rising [`PEAK_RISE_DB`] over the noise
/// floor, loudest first: (frequency, level in dB).
fn spectral_peaks(fft: &FftState) -> Vec<(f64, f64)> {
    let (Some(data), Some(analysis)) = (fft.data.as_ref(), fft.analysis.as_ref()) else {
        return Vec::new();
    };
    let Some(floor) = analysis.noise_floor_db else {
        return Vec::new();
    };
    let points = &data.points;
    let mut peaks: Vec<(f64, f64)> = (1..points.len().saturating_sub(1))
        .filter(|&bin| {
            let magnitude = points[bin].magnitude;
            magnitude > points[bin - 1].magnitude && magnitude >= points[bin + 1].magnitude
        })
        .map(|bin| (points[bin].frequency, points[bin].magnitude_db()))
        .filter(|(_, level)| level.is_finite() && *level > floor + PEAK_RISE_DB)
        .collect();
    peaks.sort_by(|a, b| b.1.total_cmp(&a.1));
    peaks
}

/// The figures, the harmonic table and the peak table, at the reader's
/// `significant_digits`.
pub fn right_panel(
    ui: &mut Ui,
    fft: &FftState,
    series: &mut Option<FftSeries>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    // The plot well already says the FFT has not run; a section header over
    // an explanation of that is the same sentence a second time.
    let Some(model) = build_model(fft, series) else {
        return;
    };
    let Some(analysis) = fft.analysis.as_ref() else {
        return;
    };
    section_header(ui, "Spectrum", None);
    let level_unit = spectrum_level_unit(&model.source, model.normalization);
    let quantity = source_quantity(&model.source);

    let digits = significant_digits;
    let f = |v: Option<f64>| {
        v.map_or("—".to_owned(), |v| {
            frequency_readout(quantity_policy, v, digits)
        })
    };
    let db = |v: Option<f64>| v.map_or("—".to_owned(), |v| fmt_si_significant(v, "dB", digits));
    let absent = absent_thd(analysis.harmonic_coverage);
    let short = |figure: Option<f64>, unit: &str| match (figure, &absent) {
        (Some(value), _) => fmt_si_significant(value, unit, digits),
        (None, Some((reason, _))) => reason.clone(),
        (None, None) => "—".to_owned(),
    };

    let amplitude = analysis.fundamental_db.map_or("—".to_owned(), |value| {
        fmt_si_significant(value, level_unit, digits)
    });
    let dc = fft.source_cache.as_ref().map_or("—".to_owned(), |source| {
        fmt_si_significant(source.dc_level, quantity, digits)
    });
    let noise_floor = match (analysis.noise_floor_db, analysis.fundamental_db) {
        (Some(noise), Some(fundamental)) => fmt_si_significant(noise - fundamental, "dBc", digits),
        _ => "—".to_owned(),
    };

    // The legend names the source and the window and normalization are
    // selects in the sheet bar, so the panel states the record the transform
    // read and what is derived from it.
    let rows = [
        ("Record", record_length(model.fft_size), false),
        ("Resolution BW", f(model.resolution_bandwidth), false),
        ("Fundamental", f(analysis.fundamental_frequency), false),
        ("Signal", amplitude, false),
        ("DC", dc, false),
    ];
    crate::presentation::stat_table(ui, &rows);
    // A figure withheld says why in a few words, and in full on hover; THD
    // names on hover the harmonics under the floor it leaves out.
    let thd = ui
        .scope(|ui| {
            crate::presentation::stat_table(
                ui,
                &[
                    ("THD", short(analysis.thd_percent, "%"), true),
                    ("THD", short(analysis.thd_db, "dB"), false),
                ],
            );
        })
        .response;
    let noise = ui
        .scope(|ui| {
            crate::presentation::stat_table(
                ui,
                &[
                    ("SNR", short(analysis.snr_db, "dB"), false),
                    ("SINAD", short(analysis.sinad_db, "dB"), false),
                ],
            );
        })
        .response;
    let left_out = analysis
        .harmonics
        .iter()
        .filter(|harmonic| !harmonic.above_floor)
        .map(|harmonic| format!("HD{}", harmonic.order))
        .collect::<Vec<_>>();
    match &absent {
        Some((_, reason)) => {
            thd.on_hover_text(reason.as_str());
            noise.on_hover_text(reason.as_str());
        }
        None if !left_out.is_empty() => {
            thd.on_hover_text(format!(
                "{} below the floor, not counted",
                left_out.join(", ")
            ));
        }
        None => {}
    }
    crate::presentation::stat_table(
        ui,
        &[
            ("SFDR", db(analysis.sfdr_db), false),
            (
                "ENOB",
                analysis.enob_bits().map_or("—".to_owned(), |bits| {
                    fmt_si_significant(bits, "bit", digits)
                }),
                false,
            ),
            ("Noise floor", noise_floor, false),
        ],
    );

    // The tables print at the transform's resolution: a frequency to the
    // decade under its bin, a level to the hundredth of a decibel.
    let resolution = fft
        .data
        .as_ref()
        .map_or(f64::NAN, crate::fft::data::FftData::frequency_resolution);
    if analysis.fundamental_frequency.is_some() && !analysis.harmonics.is_empty() {
        section_header(ui, "Harmonics", None);
        harmonic_table(ui, analysis, resolution, quantity_policy, level_unit);
    }
    let peaks = spectral_peaks(fft);
    if !peaks.is_empty() {
        section_header(ui, "Peaks", None);
        peak_table(
            ui,
            &peaks,
            analysis.fundamental_db,
            resolution,
            quantity_policy,
            level_unit,
        );
    }
}

/// The head of a level column: the unit, spelled out on hover.
fn level_head(unit: &str, what: &str) -> (String, Option<String>) {
    (unit.to_owned(), Some(format!("{what}, in {unit}")))
}

/// What a harmonic under the noise floor reads in place of a level.
const BELOW_FLOOR: &str = "below floor";

/// Every measured harmonic: its order, its frequency, its order times the
/// fundamental's, and its level and its level under the fundamental, each
/// read at the harmonic's own bin. A harmonic that does not rise out of the
/// noise floor says so, with the floor on hover, rather than print a noise
/// bin's level. The table scrolls; nothing is left off.
fn harmonic_table(
    ui: &mut Ui,
    analysis: &crate::fft::data::SpectrumAnalysis,
    resolution: f64,
    quantity_policy: &QuantityPresentationPolicy,
    level_unit: &str,
) {
    let harmonics = &analysis.harmonics;
    let db0 = analysis.fundamental_db.unwrap_or(0.0);
    let frequency = columns::frequencies(
        &harmonics.iter().map(|h| h.frequency).collect::<Vec<_>>(),
        columns::decade_step(resolution),
        quantity_policy,
    );
    let level = columns::at_resolution(
        &harmonics
            .iter()
            .map(|h| h.above_floor.then_some(h.level_db))
            .collect::<Vec<_>>(),
        HUNDREDTH,
        level_unit,
    );
    let dbc = columns::at_resolution(
        &harmonics
            .iter()
            .map(|h| h.above_floor.then_some(h.level_db - db0))
            .collect::<Vec<_>>(),
        HUNDREDTH,
        "dBc",
    );
    let floor = analysis.noise_floor_db.map_or_else(
        || "The harmonic does not rise out of the noise floor".to_owned(),
        |floor| {
            let floor = columns::at_resolution(&[Some(floor - db0)], HUNDREDTH, "dBc");
            format!(
                "The harmonic does not rise out of the noise floor, at {} dBc",
                floor.cells[0].as_deref().unwrap_or("\u{2014}")
            )
        },
    );
    let orders = harmonics
        .iter()
        .map(|h| format!("HD{}", h.order))
        .collect::<Vec<_>>();
    let text = |cells: &[Option<String>], absent: &str| {
        cells
            .iter()
            .map(|cell| {
                (
                    cell.clone().unwrap_or_else(|| absent.to_owned()),
                    cell.is_some(),
                )
            })
            .collect::<Vec<_>>()
    };
    let (level_title, level_tooltip) = level_head(&level.unit, "Level");
    let columns = columns::fitted(
        ui,
        vec![
            Fitting {
                head: "HD".to_owned(),
                tooltip: Some("Harmonic order".to_owned()),
                align: Align::Left,
                cells: orders.iter().map(|order| (order.clone(), true)).collect(),
            },
            Fitting {
                head: frequency.unit.clone(),
                tooltip: Some(format!("Frequency, in {}", frequency.unit)),
                align: Align::Right,
                cells: text(&frequency.cells, ""),
            },
            Fitting {
                head: level_title,
                tooltip: level_tooltip,
                align: Align::Right,
                cells: text(&level.cells, "\u{2014}"),
            },
            Fitting {
                head: "dBc".to_owned(),
                tooltip: Some("Level under the fundamental, in dBc".to_owned()),
                align: Align::Right,
                cells: text(&dbc.cells, BELOW_FLOOR),
            },
        ],
    );
    with_table_state(ui, "fft.harmonics", |ui, state| {
        DataTable::new("rspice.results.fft.harmonics", &columns, harmonics.len())
            .state(state)
            .show(ui, |index| {
                let harmonic = &harmonics[index];
                #[allow(clippy::cast_precision_loss)]
                let order = harmonic.order as f64;
                let dbc_key = SortKey::Number(harmonic.level_db - db0);
                vec![
                    Cell::number(orders[index].clone()).sort_by(SortKey::Number(order)),
                    Cell::number(frequency.cells[index].clone().unwrap_or_default())
                        .sort_by(SortKey::Number(harmonic.frequency)),
                    level.cells[index]
                        .clone()
                        .map_or_else(Cell::absent, |level| {
                            Cell::number(level).sort_by(SortKey::Number(harmonic.level_db))
                        }),
                    match dbc.cells[index].clone() {
                        Some(dbc) => Cell::number(dbc).sort_by(dbc_key),
                        None => Cell::text(BELOW_FLOOR).hint(floor.clone()).sort_by(dbc_key),
                    },
                ]
            });
    });
}

/// Every spectral peak rising over the noise floor, loudest first, at the
/// transform's resolution.
fn peak_table(
    ui: &mut Ui,
    peaks: &[(f64, f64)],
    carrier_db: Option<f64>,
    resolution: f64,
    quantity_policy: &QuantityPresentationPolicy,
    level_unit: &str,
) {
    let frequency = columns::frequencies(
        &peaks
            .iter()
            .map(|(frequency, _)| *frequency)
            .collect::<Vec<_>>(),
        columns::decade_step(resolution),
        quantity_policy,
    );
    let level = columns::at_resolution(
        &peaks
            .iter()
            .map(|(_, level)| Some(*level))
            .collect::<Vec<_>>(),
        HUNDREDTH,
        level_unit,
    );
    let dbc = columns::at_resolution(
        &peaks
            .iter()
            .map(|(_, level)| carrier_db.map(|carrier| level - carrier))
            .collect::<Vec<_>>(),
        HUNDREDTH,
        "dBc",
    );
    let text = |cells: &[Option<String>]| {
        cells
            .iter()
            .map(|cell| (cell.clone().unwrap_or_else(|| "\u{2014}".to_owned()), true))
            .collect::<Vec<_>>()
    };
    let (level_title, level_tooltip) = level_head(&level.unit, "Level");
    let columns = columns::fitted(
        ui,
        vec![
            Fitting {
                head: frequency.unit.clone(),
                tooltip: Some(format!("Frequency, in {}", frequency.unit)),
                align: Align::Right,
                cells: text(&frequency.cells),
            },
            Fitting {
                head: level_title,
                tooltip: level_tooltip,
                align: Align::Right,
                cells: text(&level.cells),
            },
            Fitting {
                head: "dBc".to_owned(),
                tooltip: Some("Level under the fundamental, in dBc".to_owned()),
                align: Align::Right,
                cells: text(&dbc.cells),
            },
        ],
    );
    with_table_state(ui, "fft.peaks", |ui, state| {
        DataTable::new("rspice.results.fft.peaks", &columns, peaks.len())
            .state(state)
            .show(ui, |index| {
                let (at, level_db) = peaks[index];
                let cell = |cells: &[Option<String>], key: f64| {
                    cells[index].clone().map_or_else(Cell::absent, |text| {
                        Cell::number(text).sort_by(SortKey::Number(key))
                    })
                };
                vec![
                    cell(&frequency.cells, at),
                    cell(&level.cells, level_db),
                    cell(&dbc.cells, level_db - carrier_db.unwrap_or(0.0)),
                ]
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fft::WindowFunction;
    use crate::fft::data::{FftData, SpectrumNormalization};

    #[test]
    fn rectangular_resolution_bandwidth_equals_bin_width() {
        let data = FftData::from_time_domain_with_normalization(
            "V(out)",
            &[0.0; 64],
            6_400.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Rms,
        )
        .expect("finite qualified rectangular FFT fixture");
        assert!((resolution_bandwidth(&data).unwrap() - 100.0).abs() < 1.0e-12);
    }

    /// The resolution bandwidth a windowed spectrum reports is the bin width
    /// times the window's equivalent noise bandwidth.
    ///
    /// The ENBW is derived here from Blackman-Harris's own defining cosine-sum
    /// coefficients: for `w = a0 − a1·cos(2πx) + a2·cos(4πx) − a3·cos(6πx)`,
    /// Parseval gives a mean square of `a0² + ½·Σ_{k≥1} aₖ²`, so
    /// `ENBW = (a0² + ½·Σ aₖ²) / a0²` bins — 2.0044 for these coefficients.
    ///
    /// This assertion used to be `1 bin < bandwidth < 2.1 bins`, which is a
    /// bracket rather than a value: every window in the viewer except the
    /// flat-top satisfies it, so it could not tell Blackman-Harris from Hann,
    /// and it passed at 1.01 bins — a spectrum with the ENBW correction
    /// missing altogether.
    #[test]
    fn window_resolution_bandwidth_includes_equivalent_noise_bandwidth() {
        // Long enough that the symmetric window's O(1/N) offset from the
        // periodic closed form is two orders of magnitude inside the
        // tolerance; the bin width stays 1 Hz either way.
        let data = FftData::from_time_domain_with_normalization(
            "V(out)",
            &[0.0; 8192],
            8192.0,
            WindowFunction::BlackmanHarris,
            SpectrumNormalization::Rms,
        )
        .expect("finite qualified window FFT fixture");
        let a = [0.35875_f64, 0.48829, 0.14128, 0.01168];
        let enbw_bins =
            (a[0] * a[0] + 0.5 * a[1..].iter().map(|c| c * c).sum::<f64>()) / (a[0] * a[0]);

        let bandwidth = resolution_bandwidth(&data).unwrap();
        let expected = enbw_bins * data.frequency_resolution();
        assert!(
            (bandwidth / expected - 1.0).abs() < 5.0e-3,
            "resolution bandwidth {bandwidth} vs {expected} ({enbw_bins:.4} bins \
             x {} Hz)",
            data.frequency_resolution()
        );
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

    /// The record's length is the transform length, not the count of points
    /// drawn — a 1024-sample transform plots 513 bins. The sheet once called
    /// it "points", which is the one reading the number does not have.
    #[test]
    fn the_transform_length_is_never_called_a_count_of_plotted_points() {
        let record = record_length(1024);
        assert_eq!(record, "1024 samples");
        assert!(
            !record.contains("point"),
            "the record calls the transform length a count of plotted points: {record}"
        );
    }

    /// One spectrum, rendered.
    ///
    /// The strip announces itself through AccessKit — the legend chip names
    /// its trace, and `well_hint` publishes a refusal as a live status — so
    /// the tree is where what the sheet *said* can be read back without
    /// rasterizing it.
    fn announced(points: Vec<crate::fft::data::FftPoint>, name: &str) -> Vec<String> {
        let mut fft = FftState::default();
        let mut series = None;
        let mut cache = plot::DecimationCache::default();
        let mut data =
            crate::fft::spectrum_fixture(name, &[], &[], &[], 8.0, SpectrumNormalization::Peak);
        data.points = points;
        data.fft_size = 1024;
        data.window = WindowFunction::Hanning;
        fft.data = Some(data);
        // The viewer accepts only the complete FFT transaction.  A curve
        // without its matching metrics is deliberately fail-closed, so this
        // rendering fixture must provide both halves even though these tests
        // exercise only the curve-level empty-state diagnostics.
        fft.analysis = Some(crate::fft::data::SpectrumAnalysis::default());
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        show(
                            ui,
                            &mut fft,
                            &mut series,
                            PlotView::default(),
                            &mut cache,
                            &QuantityPresentationPolicy::default(),
                            ReadoutPolicy::default(),
                            &mut None,
                        )
                    });
            },
        )
        .platform_output
        .accesskit_update
        .expect("spectrum accessibility tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect()
    }

    /// The legend names what is plotted. It named the unit instead, which
    /// the ordinate axis already carries — so the chip repeated the axis and
    /// the sheet never said which signal the spectrum came from.
    #[test]
    fn the_legend_names_the_transformed_signal_rather_than_repeating_the_axis() {
        let announced = announced(
            vec![
                crate::fft::data::FftPoint {
                    frequency: 1.0,
                    magnitude: 1.0,
                    phase: 0.0,
                },
                crate::fft::data::FftPoint {
                    frequency: 2.0,
                    magnitude: 0.5,
                    phase: 0.0,
                },
            ],
            "V(out)",
        );
        assert!(
            announced
                .iter()
                .any(|label| label == "V(out) trace selection"),
            "the legend never named the transformed signal: {announced:?}"
        );
        assert!(
            !announced
                .iter()
                .any(|label| label.ends_with("trace selection") && label.contains("dB")),
            "the legend repeated the ordinate axis: {announced:?}"
        );
    }

    /// Two different refusals read as one.
    ///
    /// "Degenerate spectrum" was printed both when the abscissa had no
    /// positive extent and when every level in view was non-finite. They are
    /// different faults with different remedies, and the reader was given no
    /// way to tell which one they were looking at. Two distinct constants do
    /// not settle that on their own — both sites could still name one of
    /// them, or name each other's — so each fault is driven through the sheet
    /// and read back off what the sheet announced.
    #[test]
    fn the_two_degenerate_spectra_are_told_apart() {
        // Every bin sits at zero, so the abscissa has no positive extent.
        let no_span = announced(
            vec![crate::fft::data::FftPoint {
                frequency: 0.0,
                magnitude: 1.0,
                phase: 0.0,
            }],
            "V(out)",
        );
        assert!(
            no_span.iter().any(|label| label == NO_FREQUENCY_SPAN),
            "{no_span:?}"
        );
        assert!(
            !no_span.iter().any(|label| label == NO_FINITE_LEVEL),
            "the abscissa's fault was reported as the ordinate's: {no_span:?}"
        );

        // A real band, with no finite level anywhere in it.
        let no_level = announced(
            vec![
                crate::fft::data::FftPoint {
                    frequency: 1.0,
                    magnitude: f64::NAN,
                    phase: 0.0,
                },
                crate::fft::data::FftPoint {
                    frequency: 2.0,
                    magnitude: f64::NAN,
                    phase: 0.0,
                },
            ],
            "V(out)",
        );
        assert!(
            no_level.iter().any(|label| label == NO_FINITE_LEVEL),
            "{no_level:?}"
        );
        assert!(
            !no_level.iter().any(|label| label == NO_FREQUENCY_SPAN),
            "the ordinate's fault was reported as the abscissa's: {no_level:?}"
        );

        // A record that is zero throughout says so, not that its levels
        // are undefined.
        let silent = announced(
            vec![
                crate::fft::data::FftPoint {
                    frequency: 1.0,
                    magnitude: 0.0,
                    phase: 0.0,
                },
                crate::fft::data::FftPoint {
                    frequency: 2.0,
                    magnitude: 0.0,
                    phase: 0.0,
                },
            ],
            "V(net1)",
        );
        assert!(
            silent.iter().any(|label| label == ZERO_SIGNAL),
            "{silent:?}"
        );
    }

    /// A coherent 64 Hz tone over 8192 samples a second, with each harmonic
    /// through HD40 a thousandth of the carrier, set to measure `harmonics`.
    fn rich_tone(harmonics: usize) -> FftState {
        let samples = (0..8192)
            .map(|n| {
                let phase = std::f64::consts::TAU * 64.0 * n as f64 / 8192.0;
                phase.sin()
                    + (2..=40)
                        .map(|order| 1.0e-3 * (order as f64 * phase).sin())
                        .sum::<f64>()
            })
            .collect();
        let mut fft = FftState::default();
        fft.load_prepared_input(crate::fft::pipeline::PreparedFftInput {
            name: "V(out)".to_owned(),
            samples,
            sample_rate: 8192.0,
            original_count: 8192,
            decimation_factor: 1,
            dc_level: 0.0,
            record: crate::fft::pipeline::FftTimeWindow::new(0.0, 1.0),
        })
        .expect("a spectrum");
        let settings = rspice_results::fft::settings::SpectrumSettings {
            harmonics,
            ..fft.settings()
        };
        fft.adopt_settings(&settings);
        assert!(fft.has_data(), "{:?}", fft.last_error);
        fft
    }

    /// What the details pane announced for `fft`, drawn on a tall screen so
    /// no table row scrolls out of view.
    fn details(fft: &FftState) -> Vec<String> {
        let mut series = None;
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(480.0, 4000.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    right_panel(
                        ui,
                        fft,
                        &mut series,
                        &QuantityPresentationPolicy::default(),
                        4,
                    );
                });
            },
        )
        .platform_output
        .accesskit_update
        .expect("details accessibility tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().or(node.value()).map(str::to_owned))
        .collect()
    }

    /// Every harmonic measured is a row of the harmonic table. The pane
    /// listed the first six and drew markers on four, so asking for forty
    /// showed a tenth of them.
    #[test]
    fn every_harmonic_measured_is_listed() {
        let fft = rich_tone(40);
        let rows = details(&fft);
        let listed: Vec<&String> = rows
            .iter()
            .filter(|row| row.starts_with("HD: HD"))
            .collect();
        assert_eq!(listed.len(), 39, "{listed:?}");
        let last = listed
            .iter()
            .find(|row| row.starts_with("HD: HD40,"))
            .expect("HD40 listed");
        // HD40 stands at forty times 64 Hz, and every row of a column prints
        // at the one precision the 1 Hz bin resolves.
        assert!(
            last.ends_with("kHz: 2.560, dBV rms: −63.01, dBc: −60"),
            "{last}"
        );
        // Each harmonic rises over the floor, so each is a peak as well.
        let peaks = rows.iter().filter(|row| row.starts_with("kHz: ")).count();
        assert_eq!(peaks, 40, "{rows:?}");
    }

    /// A THD the record cannot measure says why in the row, in a few words.
    #[test]
    fn a_withheld_thd_says_why_in_its_row() {
        // HD200 of a 64 Hz tone is far past the 4096 Hz Nyquist frequency.
        let fft = rich_tone(200);
        assert!(
            fft.analysis
                .as_ref()
                .is_some_and(|a| a.thd_percent.is_none())
        );
        let rows = details(&fft);
        let withheld: Vec<&String> = rows.iter().filter(|row| row.starts_with('—')).collect();
        assert!(
            withheld
                .iter()
                .any(|row| row.starts_with("— Nyquist at HD")),
            "{withheld:?}"
        );
    }

    /// A density reads the rms level over the window's noise bandwidth,
    /// whatever the amplitude convention: one volt peak over 100 Hz is
    /// 20·log10(1/√2) − 20 dBV/√Hz.
    #[test]
    fn a_density_reads_the_rms_level_over_the_noise_bandwidth() {
        let peak = scaled_level(
            1.0,
            SpectrumScale::Psd,
            SpectrumNormalization::Peak,
            Some(100.0),
        );
        let rms = scaled_level(
            std::f64::consts::FRAC_1_SQRT_2,
            SpectrumScale::Psd,
            SpectrumNormalization::Rms,
            Some(100.0),
        );
        assert!((peak + 23.010_299_956_639_81).abs() < 1.0e-12, "{peak}");
        assert!((rms - peak).abs() < 1.0e-12);
        assert_eq!(
            ordinate_unit("V(out)", SpectrumNormalization::Peak, SpectrumScale::Psd),
            "dBV/√Hz"
        );
    }
}
