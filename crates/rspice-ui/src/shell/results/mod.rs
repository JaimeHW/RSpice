//! The Results workspace.
//!
//! Ports `design/volta-results-workspace.html`: a docbar with run context,
//! viewer tabs and viewer-local controls; a document well carrying the
//! active viewer (waveform strips, Bode, FFT, eye, histogram, and the
//! Nyquist/Smith/pole-zero diagnostics); and a right panel that swaps to
//! the active viewer's instrument readout (cursors, margins, harmonics, eye
//! metrics, distribution stats).

mod bode;
mod eye;
mod fft;
mod hist;
mod nyquist;
mod pz;
mod smith;
mod strip;
mod waves;

pub(crate) use waves::toggle_visibility;

use std::collections::HashSet;

use egui::Ui;
use serde::{Deserialize, Serialize};

use crate::common::{AppState, RSpiceApp};
use crate::simulation::SimulationController;
use crate::simulation::controller::DerivedViewerLoadState;
use crate::state::WaveformData;
use crate::ui::plot::{CursorPair, DecimationCache};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, docbar};
use crate::common::app::ActiveViewer;

/// The result viewers, in tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ResultViewer {
    /// Stacked waveform strips, one per analysis.
    #[default]
    Waves,
    /// Loop-gain stability view with margin markers.
    Bode,
    /// Spectrum with harmonic markers.
    Fft,
    /// Eye diagram with compliance mask.
    Eye,
    /// Monte-Carlo distribution.
    Hist,
    /// Legacy Nyquist surface (pre-redesign chrome).
    Nyquist,
    /// Legacy Smith chart surface.
    Smith,
    /// Legacy pole-zero surface.
    PoleZero,
}

impl ResultViewer {
    /// Tab label (mono, uppercase by design).
    pub fn label(self) -> &'static str {
        match self {
            ResultViewer::Waves => "WAVES",
            ResultViewer::Bode => "BODE",
            ResultViewer::Fft => "FFT",
            ResultViewer::Eye => "EYE",
            ResultViewer::Hist => "HIST",
            ResultViewer::Nyquist => "NYQ",
            ResultViewer::Smith => "SMITH",
            ResultViewer::PoleZero => "PZ",
        }
    }

    /// Status-bar summary fragment.
    fn status(self) -> &'static str {
        match self {
            ResultViewer::Waves => "waves",
            ResultViewer::Bode => "bode",
            ResultViewer::Fft => "fft",
            ResultViewer::Eye => "eye",
            ResultViewer::Hist => "mc",
            ResultViewer::Nyquist => "nyquist",
            ResultViewer::Smith => "smith",
            ResultViewer::PoleZero => "pole-zero",
        }
    }

    /// The legacy viewer this tab corresponds to, for capability checks and
    /// the derived-data loader.
    fn legacy(self) -> Option<ActiveViewer> {
        match self {
            ResultViewer::Fft => Some(ActiveViewer::Fft),
            ResultViewer::Eye => Some(ActiveViewer::EyeDiagram),
            ResultViewer::Hist => Some(ActiveViewer::Histogram),
            ResultViewer::Nyquist => Some(ActiveViewer::Nyquist),
            ResultViewer::Smith => Some(ActiveViewer::SmithChart),
            ResultViewer::PoleZero => Some(ActiveViewer::PoleZero),
            ResultViewer::Waves | ResultViewer::Bode => None,
        }
    }

    const PRIMARY: [ResultViewer; 5] = [
        ResultViewer::Waves,
        ResultViewer::Bode,
        ResultViewer::Fft,
        ResultViewer::Eye,
        ResultViewer::Hist,
    ];
    const LEGACY: [ResultViewer; 3] = [
        ResultViewer::Nyquist,
        ResultViewer::Smith,
        ResultViewer::PoleZero,
    ];
}

/// User zoom/pan override for one plot. `None` per axis means automatic
/// fit-to-data; gestures in the plot engine populate the ranges and a
/// double-click (or the strip's FIT action) clears them.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlotView {
    /// X range override (data space).
    pub x: Option<(f64, f64)>,
    /// Left-Y range override.
    pub y: Option<(f64, f64)>,
    /// Right-Y range override (dual-axis strips).
    pub y_right: Option<(f64, f64)>,
}

impl PlotView {
    /// Whether any axis is zoomed away from the automatic view.
    pub fn is_zoomed(&self) -> bool {
        self.x.is_some() || self.y.is_some() || self.y_right.is_some()
    }

    /// Fold one frame's gesture result into the override.
    pub fn apply(&mut self, change: &crate::ui::plot::ViewChange) {
        if change.reset {
            *self = Self::default();
            return;
        }
        if let Some(x) = change.x {
            self.x = Some(x);
        }
        if let Some(y) = change.y {
            self.y = Some(y);
        }
        if let Some(y_right) = change.y_right {
            self.y_right = Some(y_right);
        }
    }
}

/// Per-session results-workspace state. Only the viewer selection persists;
/// caches and cursors are transient.
#[derive(Debug, Clone, Default)]
pub struct ResultsState {
    /// Active viewer tab.
    pub viewer: ResultViewer,
    /// A/B cursors (data-space X of the strip they live on).
    pub cursors: CursorPair,
    /// Which strip (analysis index) the cursors were placed on.
    pub cursor_strip: Option<usize>,
    /// Decimation envelope cache.
    pub cache: DecimationCache,
    /// Derived dB/phase series cache.
    pub derived: DerivedSeries,
    /// Strips hidden via the strip-close action.
    pub hidden_strips: HashSet<usize>,
    /// Strip currently maximized via the strip action, if any.
    pub maximized_strip: Option<usize>,
    /// Cached FFT display arrays for the active spectrum revision.
    pub fft_series: Option<FftSeries>,
    /// Cached Bode margins + extremes for the active data version.
    pub bode: Option<BodeDerived>,
    /// Cached Nyquist stability numbers for the active data version.
    pub nyquist: Option<nyquist::NyquistDerived>,
    /// Baked EYE density texture for the active eye revision and size.
    pub eye_texture: Option<EyeTexture>,
    /// `simulation.data_version` last seen by the workspace; when it
    /// advances, cursors are cleared so they never report stale data.
    seen_version: u64,
    /// Zoom/pan overrides per plot, keyed by (viewer, strip index). Survives
    /// re-runs on purpose — keeping the zoomed window across parameter
    /// tweaks is how engineers compare iterations.
    pub views: std::collections::HashMap<(ResultViewer, usize), PlotView>,
    /// User expression traces per waves strip (analysis index), evaluated by
    /// the calculator against that analysis' waveforms. Persisted with the
    /// session (visibility resets to on).
    pub exprs: std::collections::HashMap<usize, Vec<ExprTrace>>,
    /// The inline expression editor, when open (one strip at a time).
    pub expr_editor: Option<ExprEditor>,
    /// Evaluated expression series, keyed by (analysis index, expression);
    /// refreshed when the simulation data version advances.
    pub expr_cache: std::collections::HashMap<(usize, String), ExprSeries>,
    /// Pinned data point per XY viewer (trace slot, point index) — the
    /// Smith/Nyquist/PZ click-to-pin readout.
    pub rf_pin: std::collections::HashMap<ResultViewer, (usize, usize)>,
    /// Display AC phase traces unwrapped into a continuous curve instead of
    /// wrapped to ±180°. Applies to the BODE phase trace and the waves
    /// strips' phase traces; the margin math always reads the raw wrapped
    /// arrays. Transient — not persisted with the session.
    pub phase_continuous: bool,
    /// Screen rect of the document well (docbar excluded) from the last
    /// rendered frame — the crop window for viewer PNG export. Transient.
    pub well_rect: Option<egui::Rect>,
}

/// One user expression trace on a waves strip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExprTrace {
    /// The calculator expression as typed ("V(out)/V(in)").
    pub text: String,
    /// Legend-chip visibility.
    #[serde(default = "default_visible")]
    pub visible: bool,
}

fn default_visible() -> bool {
    true
}

/// State of the inline expression editor under a strip header.
#[derive(Debug, Clone, Default)]
pub struct ExprEditor {
    /// The strip (analysis index) the editor is attached to.
    pub analysis_index: usize,
    /// Text being edited.
    pub text: String,
    /// Last evaluation error, shown inline.
    pub error: Option<String>,
    /// Request keyboard focus on the next frame (set when opened).
    pub want_focus: bool,
}

/// One evaluated expression series (owned arrays, cheap to clone).
#[derive(Debug, Clone)]
pub struct ExprSeries {
    /// `simulation.data_version` the series was computed against.
    pub version: u64,
    /// Evaluation result, or the error to show on the strip.
    pub series: Result<(std::sync::Arc<[f64]>, std::sync::Arc<[f64]>), String>,
    /// Finite (min, max) of y, for the automatic fit.
    pub y_extremes: Option<(f64, f64)>,
}

impl ResultsState {
    /// Clear cursors (Esc, Clear action).
    pub fn clear_cursors(&mut self) {
        self.cursors.clear();
        self.cursor_strip = None;
    }

    /// The zoom/pan override for one plot (copy; default = automatic view).
    pub fn plot_view(&self, viewer: ResultViewer, index: usize) -> PlotView {
        self.views.get(&(viewer, index)).copied().unwrap_or_default()
    }

    /// Mutable zoom/pan override for one plot.
    pub fn plot_view_mut(&mut self, viewer: ResultViewer, index: usize) -> &mut PlotView {
        self.views.entry((viewer, index)).or_default()
    }

    /// Drop the zoom/pan override for one plot (FIT action).
    pub fn reset_plot_view(&mut self, viewer: ResultViewer, index: usize) {
        self.views.remove(&(viewer, index));
    }
}

/// Frequency/magnitude-dB arrays derived from the active FFT, keyed on the
/// FFT state's spectrum revision — the FFT can be recomputed (window, size,
/// source) without a simulation data-version bump, and keying on the data's
/// allocation address could serve stale arrays after a reallocation.
#[derive(Debug, Clone)]
pub struct FftSeries {
    pub(crate) revision: u64,
    pub(crate) frequency: std::sync::Arc<[f64]>,
    pub(crate) magnitude_db: std::sync::Arc<[f64]>,
    /// Cached finite (lo, hi) of the magnitude within view: (x1 bits, lo, hi).
    pub(crate) y_extremes: Option<(u64, f64, f64)>,
}

/// GPU density image for the EYE viewer, baked once per (data revision,
/// plot size, trace color) — per frame the eye then costs one textured
/// quad instead of restroking every folded acquisition.
#[derive(Clone)]
pub struct EyeTexture {
    pub(crate) revision: u64,
    pub(crate) size: [usize; 2],
    pub(crate) color: egui::Color32,
    pub(crate) handle: egui::TextureHandle,
}

impl std::fmt::Debug for EyeTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EyeTexture")
            .field("revision", &self.revision)
            .field("size", &self.size)
            .finish_non_exhaustive()
    }
}

/// Stability numbers and axis extremes computed from the active AC curves.
/// BODE's center view and right panel share one cached compute, keyed on
/// the data version and the resolved magnitude waveform.
#[derive(Debug, Clone, Copy)]
pub struct BodeDerived {
    pub(crate) version: u64,
    pub(crate) analysis_index: usize,
    pub(crate) mag_index: usize,
    /// DC (lowest-frequency) gain in dB.
    pub(crate) adc_db: Option<f64>,
    /// Unity-gain frequency (Hz).
    pub(crate) ugf: Option<f64>,
    /// Phase margin (deg) at the UGF.
    pub(crate) pm_deg: Option<f64>,
    /// Frequency where phase crosses −180° (Hz).
    pub(crate) f180: Option<f64>,
    /// Gain margin (dB) at f180.
    pub(crate) gm_db: Option<f64>,
    /// −3 dB bandwidth (Hz).
    pub(crate) f3db: Option<f64>,
    /// Finite (min, max) of the gain curve.
    pub(crate) gain_extremes: (f64, f64),
    /// Finite (min, max) of the phase curve, when phase data exists.
    pub(crate) phase_extremes: Option<(f64, f64)>,
}

/// Finite (min, max) of a slice, if any finite values exist.
pub(super) fn finite_extremes(values: &[f64]) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &v in values {
        if v.is_finite() {
            lo = lo.min(v);
            hi = hi.max(v);
        }
    }
    (lo <= hi).then_some((lo, hi))
}

/// Cache of series derived from waveform data (dB conversions), cleared when
/// the simulation data version changes.
#[derive(Debug, Clone, Default)]
pub struct DerivedSeries {
    version: u64,
    map: std::collections::HashMap<u64, std::sync::Arc<[f64]>>,
    /// Cached finite (min, max) per series key — axis autoranges must not
    /// rescan millions of samples per frame.
    ranges: std::collections::HashMap<u64, Option<(f64, f64)>>,
    /// Cached windowed (min, max, rms) measurements, keyed by
    /// (series key, window-start bits, window-end bits).
    stats: std::collections::HashMap<(u64, u64, u64), Option<(f64, f64, f64)>>,
}

impl DerivedSeries {
    fn ensure_version(&mut self, version: u64) {
        if self.version != version {
            self.map.clear();
            self.ranges.clear();
            self.stats.clear();
            self.version = version;
        }
    }

    /// Fetch or compute the cached finite (min, max) of a series.
    pub fn range_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> Option<(f64, f64)>,
    ) -> Option<(f64, f64)> {
        *self.ranges.entry(key).or_insert_with(build)
    }

    /// Fetch or compute cached windowed (min, max, rms) measurements.
    pub fn stats_or(
        &mut self,
        key: (u64, u64, u64),
        build: impl FnOnce() -> Option<(f64, f64, f64)>,
    ) -> Option<(f64, f64, f64)> {
        *self.stats.entry(key).or_insert_with(build)
    }

    /// Fetch or build a derived series under `key`.
    pub fn get_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> std::sync::Arc<[f64]>,
    ) -> std::sync::Arc<[f64]> {
        if let Some(hit) = self.map.get(&key) {
            return std::sync::Arc::clone(hit);
        }
        let series = build();
        self.map.insert(key, std::sync::Arc::clone(&series));
        series
    }

    /// 20·log₁₀ of a linear-magnitude series, cached under `key`.
    pub fn db(&mut self, key: u64, magnitude: &[f64]) -> std::sync::Arc<[f64]> {
        self.get_or(key, || {
            magnitude
                .iter()
                .map(|&m| 20.0 * m.max(1e-30).log10())
                .collect()
        })
    }

    /// Key-space bit separating unwrapped-phase entries from the dB entries,
    /// which share the `(analysis << 32 | waveform)` key convention.
    const UNWRAP_KEY_BIT: u64 = 1 << 62;

    /// Continuous (unwrapped) copy of a ±180°-wrapped phase-degree series,
    /// cached under `key` like `db`.
    pub fn unwrapped(&mut self, key: u64, phase_deg: &[f64]) -> std::sync::Arc<[f64]> {
        self.get_or(Self::UNWRAP_KEY_BIT | key, || {
            crate::analysis::calculator::functions::unwrap_phase_deg(phase_deg).into()
        })
    }

    /// Finite (min, max) of the unwrapped series for `key`, cached alongside
    /// the per-trace ranges.
    pub fn unwrapped_range(&mut self, key: u64, phase_deg: &[f64]) -> Option<(f64, f64)> {
        let series = self.unwrapped(key, phase_deg);
        self.range_or(Self::UNWRAP_KEY_BIT | key, || finite_extremes(&series))
    }
}

// ---------------------------------------------------------------------------
// shared right-panel furniture
// ---------------------------------------------------------------------------

/// A results stat table: dimmed names left, mono values right, key rows
/// accent-highlighted (at most one or two per table, by design).
/// Forward map for the XY (equal-aspect) viewers: data → screen, matching
/// the plot engine's linear mapping.
pub(super) fn xy_screen_pos(
    plot_rect: egui::Rect,
    point: (f64, f64),
    x_range: (f64, f64),
    y_range: (f64, f64),
) -> egui::Pos2 {
    let fx = (point.0 - x_range.0) / (x_range.1 - x_range.0);
    let fy = (point.1 - y_range.0) / (y_range.1 - y_range.0);
    egui::pos2(
        plot_rect.left() + (fx as f32) * plot_rect.width(),
        plot_rect.bottom() - (fy as f32) * plot_rect.height(),
    )
}

/// One row of a point readout card.
pub(super) type CardRow = (String, String);

/// The floating point-readout card the XY viewers show on hover/pin:
/// colored mono title + k/v rows, anchored beside the point and clamped
/// to the plot. Painted, not laid out — it floats over the chart.
pub(super) fn point_card(
    ui: &Ui,
    bounds: egui::Rect,
    anchor: egui::Pos2,
    title: &str,
    title_color: egui::Color32,
    rows: &[CardRow],
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let painter = ui.painter();

    let title_galley = painter.layout_no_wrap(
        title.to_owned(),
        theme::mono(11.0, FontWeight::Medium),
        title_color,
    );
    let mut key_width = 0.0f32;
    let mut value_width = 0.0f32;
    let galleys: Vec<_> = rows
        .iter()
        .map(|(k, v)| {
            let kg = painter.layout_no_wrap(
                k.clone(),
                theme::mono(11.0, FontWeight::Regular),
                c.text_dim,
            );
            let vg =
                painter.layout_no_wrap(v.clone(), theme::mono(11.0, FontWeight::Regular), c.text);
            key_width = key_width.max(kg.size().x);
            value_width = value_width.max(vg.size().x);
            (kg, vg)
        })
        .collect();

    let (pad_x, pad_y, gap, line_h) = (9.0, 6.0, 12.0, 16.0);
    let width = (key_width + gap + value_width)
        .max(title_galley.size().x)
        + pad_x * 2.0;
    let height = pad_y * 2.0 + line_h * (rows.len() as f32 + 1.0);

    let mut origin = anchor + egui::vec2(14.0, -height - 8.0);
    if origin.x + width > bounds.right() - 4.0 {
        origin.x = anchor.x - width - 14.0;
    }
    origin.y = origin
        .y
        .clamp(bounds.top() + 4.0, (bounds.bottom() - height - 4.0).max(bounds.top() + 4.0));

    let rect = egui::Rect::from_min_size(origin, egui::vec2(width, height));
    painter.rect(
        rect,
        t.radius,
        c.bg_elevated,
        egui::Stroke::new(1.0, c.border_strong),
    );
    painter.galley(
        egui::pos2(origin.x + pad_x, origin.y + pad_y),
        title_galley,
        title_color,
    );
    for (i, (kg, vg)) in galleys.into_iter().enumerate() {
        let y = origin.y + pad_y + line_h * (i as f32 + 1.0);
        painter.galley(egui::pos2(origin.x + pad_x, y), kg, c.text_dim);
        painter.galley(
            egui::pos2(origin.x + pad_x + key_width + gap + value_width - vg.size().x, y),
            vg,
            c.text,
        );
    }
}

pub(super) fn stat_table(ui: &mut Ui, rows: &[(&str, String, bool)]) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    for (i, (name, value, highlight)) in rows.iter().enumerate() {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 25.0), egui::Sense::hover());
        if !ui.is_rect_visible(rect) {
            continue;
        }
        let painter = ui.painter();
        painter.text(
            egui::pos2(rect.left() + 12.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            *name,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            if *highlight { c.text } else { c.text_dim },
        );
        painter.text(
            egui::pos2(rect.right() - 12.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            value,
            theme::mono(tokens::FS_1, FontWeight::Regular),
            if *highlight { c.accent } else { c.text },
        );
        if i + 1 < rows.len() {
            painter.hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
        }
    }
}

/// A faint explanatory note under a right-panel section.
pub(super) fn panel_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::none()
        .inner_margin(egui::Margin {
            left: 12.0,
            right: 12.0,
            top: 4.0,
            bottom: 10.0,
        })
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
}

/// Parse a `#rrggbb` trace color, falling back to the palette cycle.
pub fn trace_color(hex: &str, fallback: egui::Color32) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        if let Ok(value) = u32::from_str_radix(hex, 16) {
            return egui::Color32::from_rgb(
                ((value >> 16) & 0xff) as u8,
                ((value >> 8) & 0xff) as u8,
                (value & 0xff) as u8,
            );
        }
    }
    fallback
}

/// Resolve a waveform's display color from its stored hex + palette fallback.
pub fn waveform_color(waveform: &WaveformData, index: usize, t: &Tokens) -> egui::Color32 {
    trace_color(&waveform.color, t.color.traces[index % t.color.traces.len()])
}

/// Centered faint hint on an empty document well.
pub fn well_hint(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.available_rect_before_wrap();
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        theme::sans(tokens::FS_2, FontWeight::Regular),
        t.color.text_faint,
    );
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the Results workspace center view (docbar + active viewer).
pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let data_version = app.state.simulation.data_version;
    let results = &mut app.state.shell.results;
    if results.seen_version != data_version {
        results.seen_version = data_version;
        results.clear_cursors();
        // Pinned XY readouts index into the old run's point arrays;
        // a same-shape new run would silently relabel them.
        results.rf_pin.clear();
    }
    results.cache.ensure_version(data_version);
    results.derived.ensure_version(data_version);

    show_docbar(ui, &mut app.state);

    let t = Tokens::get(ui.ctx());
    // The document well backdrop; viewers paint on top. The rect doubles
    // as the crop window for viewer PNG export.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, t.color.canvas_bg);
    app.state.shell.results.well_rect = Some(well);

    if !app.state.simulation.has_results() {
        well_hint(ui, "No results yet — run a simulation (F5)");
        return;
    }

    let viewer = app.state.shell.results.viewer;
    match viewer {
        ResultViewer::Waves => waves::show(ui, &mut app.state),
        ResultViewer::Bode => bode::show(ui, &mut app.state),
        ResultViewer::Fft => {
            if ensure_derived(ui, app, ActiveViewer::Fft) {
                fft::show(ui, &mut app.state);
            }
        }
        ResultViewer::Eye => {
            if ensure_derived(ui, app, ActiveViewer::EyeDiagram) {
                eye::show(ui, &mut app.state);
            }
        }
        ResultViewer::Hist => hist::show(ui, &mut app.state),
        ResultViewer::Nyquist => nyquist::show(ui, &mut app.state),
        ResultViewer::Smith => smith::show(ui, &mut app.state),
        ResultViewer::PoleZero => pz::show(ui, &mut app.state),
    }
}

fn show_docbar(ui: &mut Ui, state: &mut AppState) {
    docbar(ui, |ui| {
        run_selector(ui, state);
        viewer_tabs(ui, state);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            ui.menu_button("Export…", |ui| {
                if ui.button("Waveform data (CSV)…").clicked() {
                    state.shell.export_csv_requested = true;
                    ui.close_menu();
                }
                if ui.button("Viewer image (PNG)…").clicked() {
                    state.shell.export_png_requested = true;
                    ui.close_menu();
                }
            });
            // Viewer-local controls.
            match state.shell.results.viewer {
                ResultViewer::Waves => {
                    let results = &mut state.shell.results;
                    if !results.hidden_strips.is_empty() {
                        let n = results.hidden_strips.len();
                        if chip(ui, &format!("{n} hidden"), true)
                            .on_hover_text("Restore closed strips")
                            .clicked()
                        {
                            results.hidden_strips.clear();
                        }
                    }
                    let on = results.cursors.any();
                    if chip(ui, "cursors A/B", on)
                        .on_hover_text("Click a plot to place A, click again for B; Esc clears")
                        .clicked()
                    {
                        results.clear_cursors();
                    }
                }
                ResultViewer::Fft => {
                    let label = state
                        .analysis
                        .fft_state
                        .data
                        .as_ref()
                        .map(|d| format!("{} · {}", d.window.display_name(), d.fft_size));
                    if let Some(label) = label {
                        ui.label(
                            egui::RichText::new(label)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(Tokens::get(ui.ctx()).color.text_faint),
                        );
                    }
                }
                ResultViewer::Eye => {
                    let mask_on = state.analysis.eye_diagram_state.show_mask;
                    if chip(ui, "mask", mask_on).clicked() {
                        state.analysis.eye_diagram_state.show_mask = !mask_on;
                    }
                }
                _ => {}
            }
        });
    });
}

fn run_selector(ui: &mut Ui, state: &mut AppState) {
    let label = state
        .simulation
        .active_run()
        .map_or_else(|| "no runs".to_owned(), |run| format!("run #{}", run.id));
    let runs: Vec<String> = state
        .simulation
        .runs
        .iter()
        .map(|run| {
            format!(
                "run #{} · {} · {:.1} s",
                run.id,
                if run.success { "ok" } else { "failed" },
                run.elapsed_time
            )
        })
        .collect();

    if let Some(index) =
        crate::ui::widgets::select(ui, "volta.results.run", &label, &runs, 150.0)
    {
        state.simulation.select_run(index);
        state.shell.results.clear_cursors();
    }
}

fn viewer_tabs(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.spacing_mut().item_spacing.x = 2.0;

    let current = state.shell.results.viewer;
    let mut clicked: Option<ResultViewer> = None;

    for viewer in ResultViewer::PRIMARY {
        if viewer_tab(ui, viewer, current == viewer, true) {
            clicked = Some(viewer);
        }
    }
    // Legacy surfaces, gated on data availability and visually set apart.
    let (sep, _) = ui.allocate_exact_size(egui::vec2(13.0, 24.0), egui::Sense::hover());
    ui.painter().vline(
        sep.center().x,
        egui::Rangef::new(sep.center().y - 7.0, sep.center().y + 7.0),
        egui::Stroke::new(1.0, t.color.border),
    );
    for viewer in ResultViewer::LEGACY {
        let available = viewer
            .legacy()
            .is_some_and(|legacy| state.viewer_is_available(legacy));
        if viewer_tab(ui, viewer, current == viewer, available) {
            clicked = Some(viewer);
        }
    }
    if let Some(viewer) = clicked {
        state.shell.results.viewer = viewer;
    }
}

/// One viewer tab, per the design: a 24 px chip with 11 px side padding,
/// letterspaced mono label, hover fill, and an accent wash when active
/// (inverted in the Graphite direction).
fn viewer_tab(ui: &mut Ui, viewer: ResultViewer, active: bool, enabled: bool) -> bool {
    use crate::ui::theme::mix;

    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let mut job = egui::text::LayoutJob::default();
    job.append(
        viewer.label(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
            color: egui::Color32::PLACEHOLDER,
            extra_letter_spacing: 0.05 * tokens::FS_0,
            ..Default::default()
        },
    );
    let galley = ui.fonts(|f| f.layout_job(job));

    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(galley.size().x + 22.0, 24.0),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    if !ui.is_rect_visible(rect) {
        return false;
    }

    let hover = ui.ctx().animate_bool_with_time(
        response.id,
        enabled && !active && response.hovered(),
        0.16,
    );
    let graphite = t.direction == crate::ui::Direction::Graphite;
    let (fill, text_color) = if active {
        if graphite {
            (c.text, c.bg_panel)
        } else {
            (c.accent_dim, c.accent)
        }
    } else if !enabled {
        (egui::Color32::TRANSPARENT, c.text_faint)
    } else {
        (
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
            mix(c.text_dim, c.text, hover),
        )
    };

    let painter = ui.painter();
    if fill != egui::Color32::TRANSPARENT {
        painter.rect_filled(rect, t.radius, fill);
    }
    painter.galley(
        egui::pos2(rect.left() + 11.0, rect.center().y - galley.size().y * 0.5),
        galley,
        text_color,
    );

    if !enabled {
        response.on_hover_text("No data for this viewer in the active run");
        return false;
    }
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
        && !active
}

/// Gate FFT/eye rendering on the controller's derived-data loader. Returns
/// `true` when the viewer can render.
fn ensure_derived(ui: &mut Ui, app: &mut RSpiceApp, viewer: ActiveViewer) -> bool {
    let Some(analysis_type) = app
        .state
        .simulation
        .active_analysis()
        .map(|analysis| analysis.analysis_type)
    else {
        well_hint(ui, "Select an analysis with transient data");
        return false;
    };
    if !SimulationController::analysis_supports_transient_derivation(analysis_type) {
        well_hint(ui, "The active analysis has no usable time-domain source");
        return false;
    }
    match app
        .simulation_controller
        .ensure_transient_viewer_data(&mut app.state, viewer)
    {
        DerivedViewerLoadState::Ready => true,
        DerivedViewerLoadState::Loading => {
            well_hint(ui, "Preparing derived data from the active transient…");
            ui.ctx().request_repaint();
            false
        }
        DerivedViewerLoadState::Unavailable => {
            well_hint(ui, "The active analysis does not contain a usable source");
            false
        }
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Render the Results context right panel for the active viewer.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    match state.shell.results.viewer {
        ResultViewer::Waves => waves::right_panel(ui, state),
        ResultViewer::Bode => bode::right_panel(ui, state),
        ResultViewer::Fft => fft::right_panel(ui, state),
        ResultViewer::Eye => eye::right_panel(ui, state),
        ResultViewer::Hist => hist::right_panel(ui, state),
        ResultViewer::Nyquist => nyquist::right_panel(ui, state),
        ResultViewer::Smith => smith::right_panel(ui, state),
        ResultViewer::PoleZero => pz::right_panel(ui, state),
    }
}

/// Status-bar summary for the results workspace.
pub fn status_summary(state: &AppState) -> String {
    let viewer = state.shell.results.viewer;
    match state.simulation.active_run() {
        Some(run) => format!("{} · run #{}", viewer.status(), run.id),
        None => viewer.status().to_owned(),
    }
}
