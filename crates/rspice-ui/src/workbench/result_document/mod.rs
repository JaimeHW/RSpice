//! The Results workspace.
//!
//! A docbar with run context,
//! viewer tabs and viewer-local controls; a document well carrying the
//! active viewer (waveform strips, Bode, FFT, eye, histogram, and the
//! Nyquist/Smith/pole-zero diagnostics); and a right panel that swaps to
//! the active viewer's instrument readout (cursors, margins, harmonics, eye
//! metrics, distribution stats).

mod bode;
mod eye;
mod fft;
mod hist;
mod noise_contrib;
mod nyquist;
mod op_inspector;
mod pz;
mod smith;
mod specs;

pub(crate) fn open_specification_editor(state: &mut AppState) {
    specs::open_editor(state);
}
mod strip;
mod waves;
pub(crate) use waves::copy_cursor_text;

pub(crate) use waves::toggle_visibility;

use std::collections::HashSet;

use egui::{Ui, WidgetInfo, WidgetType};
use serde::{Deserialize, Serialize};

use crate::common::app::ActiveViewer;
use crate::common::{AppState, RSpiceApp};
use crate::product::DatasetId;
use crate::simulation::SimulationController;
use crate::simulation::controller::DerivedViewerLoadState;
use crate::state::{SharedWaveformValues, WaveformData};
use crate::ui::plot::{CursorPair, DecimationCache};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, docbar};

pub type WaveformSeries = (SharedWaveformValues, SharedWaveformValues);
pub type WaveformSeriesResult = Result<WaveformSeries, String>;
type WindowStatsKey = (u64, u64, u64);
type WindowStats = Option<(f64, f64, f64)>;

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
    /// Per-device operating-point inspector (Spectre-style OP info).
    Op,
    /// Ranked band-integrated noise contributors.
    NoiseContrib,
    /// Measurements × runs matrix against spec bounds.
    Specs,
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
            ResultViewer::Op => "OP",
            ResultViewer::NoiseContrib => "NOISE",
            ResultViewer::Specs => "SPECS",
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
            ResultViewer::Op => "op info",
            ResultViewer::NoiseContrib => "noise contributors",
            ResultViewer::Specs => "specs matrix",
            ResultViewer::Nyquist => "nyquist",
            ResultViewer::Smith => "smith",
            ResultViewer::PoleZero => "pole-zero",
        }
    }

    const PRIMARY: [ResultViewer; 8] = [
        ResultViewer::Waves,
        ResultViewer::Bode,
        ResultViewer::Fft,
        ResultViewer::Eye,
        ResultViewer::Hist,
        ResultViewer::Op,
        ResultViewer::NoiseContrib,
        ResultViewer::Specs,
    ];
    const LEGACY: [ResultViewer; 3] = [
        ResultViewer::Nyquist,
        ResultViewer::Smith,
        ResultViewer::PoleZero,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ViewerAvailability {
    available: bool,
    reason: &'static str,
}

impl ViewerAvailability {
    const fn available(reason: &'static str) -> Self {
        Self {
            available: true,
            reason,
        }
    }

    const fn unavailable(reason: &'static str) -> Self {
        Self {
            available: false,
            reason,
        }
    }
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
    /// Share the same A/B cursor positions across every compatible waveform
    /// strip instead of scoping them to `cursor_strip`.
    pub linked_cursors: bool,
    /// Decimation envelope cache.
    pub cache: DecimationCache,
    /// Derived dB/phase series cache.
    pub derived: DerivedSeries,
    /// Fingerprint-keyed strip-model cache for the waves viewer.
    models: waves::ModelsCache,
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
    /// the calculator against that analysis' waveforms.
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
    /// OP inspector device-name filter (docbar input). Transient.
    pub op_filter: String,
    /// OP inspector sort: (column key, descending). Transient.
    pub op_sort: Option<(String, bool)>,
    /// Open spec-editor rows (None = matrix view). Transient.
    pub spec_drafts: Option<Vec<specs::SpecDraft>>,
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
    pub series: WaveformSeriesResult,
    /// Finite (min, max) of y, for the automatic fit.
    pub y_extremes: Option<(f64, f64)>,
}

impl ResultsState {
    /// Clear cursors (Esc, Clear action).
    pub fn clear_cursors(&mut self) {
        self.cursors.clear();
        self.cursor_strip = None;
    }

    pub fn toggle_linked_cursors(&mut self) {
        self.linked_cursors = !self.linked_cursors;
    }

    /// Clear result UI state that is tied to the active project/design data.
    pub fn clear_project_scoped_state(&mut self) {
        let viewer = self.viewer;
        let phase_continuous = self.phase_continuous;
        *self = Self {
            viewer,
            phase_continuous,
            ..Self::default()
        };
    }

    /// The zoom/pan override for one plot (copy; default = automatic view).
    pub fn plot_view(&self, viewer: ResultViewer, index: usize) -> PlotView {
        self.views
            .get(&(viewer, index))
            .copied()
            .unwrap_or_default()
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
    map: std::collections::HashMap<u64, SharedWaveformValues>,
    /// Cached finite (min, max) per series key — axis autoranges must not
    /// rescan millions of samples per frame.
    ranges: std::collections::HashMap<u64, Option<(f64, f64)>>,
    /// Cached windowed (min, max, rms) measurements, keyed by
    /// (series key, window-start bits, window-end bits).
    stats: std::collections::HashMap<WindowStatsKey, WindowStats>,
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
        build: impl FnOnce() -> SharedWaveformValues,
    ) -> SharedWaveformValues {
        if let Some(hit) = self.map.get(&key) {
            return std::sync::Arc::clone(hit);
        }
        let series = build();
        self.map.insert(key, std::sync::Arc::clone(&series));
        series
    }

    /// 20·log₁₀ of a linear-magnitude series, cached under `key`.
    pub fn db(&mut self, key: u64, magnitude: &[f64]) -> SharedWaveformValues {
        self.get_or(key, || {
            std::sync::Arc::new(
                magnitude
                    .iter()
                    .map(|&m| 20.0 * m.max(1e-30).log10())
                    .collect::<Vec<_>>(),
            )
        })
    }

    /// Key-space bit separating unwrapped-phase entries from the dB entries,
    /// which share the `(analysis << 32 | waveform)` key convention.
    const UNWRAP_KEY_BIT: u64 = 1 << 62;

    /// Continuous (unwrapped) copy of a ±180°-wrapped phase-degree series,
    /// cached under `key` like `db`.
    pub fn unwrapped(&mut self, key: u64, phase_deg: &[f64]) -> SharedWaveformValues {
        self.get_or(Self::UNWRAP_KEY_BIT | key, || {
            std::sync::Arc::new(crate::analysis::calculator::functions::unwrap_phase_deg(
                phase_deg,
            ))
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
    let width = (key_width + gap + value_width).max(title_galley.size().x) + pad_x * 2.0;
    let height = pad_y * 2.0 + line_h * (rows.len() as f32 + 1.0);

    let mut origin = anchor + egui::vec2(14.0, -height - 8.0);
    if origin.x + width > bounds.right() - 4.0 {
        origin.x = anchor.x - width - 14.0;
    }
    origin.y = origin.y.clamp(
        bounds.top() + 4.0,
        (bounds.bottom() - height - 4.0).max(bounds.top() + 4.0),
    );

    let rect = egui::Rect::from_min_size(origin, egui::vec2(width, height));
    painter.rect(
        rect,
        t.radius,
        c.bg_elevated,
        egui::Stroke::new(1.0, c.border_strong),
        egui::StrokeKind::Inside,
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
            egui::pos2(
                origin.x + pad_x + key_width + gap + value_width - vg.size().x,
                y,
            ),
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
    egui::Frame::NONE
        .inner_margin(egui::Margin {
            left: 12,
            right: 12,
            top: 4,
            bottom: 10,
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
    if hex.len() == 6
        && let Ok(value) = u32::from_str_radix(hex, 16)
    {
        return egui::Color32::from_rgb(
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
        );
    }
    fallback
}

/// Resolve a waveform's display color from its stored hex + palette fallback.
pub fn waveform_color(waveform: &WaveformData, index: usize, t: &Tokens) -> egui::Color32 {
    trace_color(
        &waveform.color,
        t.color.traces[index % t.color.traces.len()],
    )
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
    let results = &mut app.state.ui.results;
    if results.seen_version != data_version {
        results.seen_version = data_version;
        results.clear_cursors();
        // Pinned XY readouts index into the old run's point arrays;
        // a same-shape new run would silently relabel them.
        results.rf_pin.clear();
    }
    results.cache.ensure_version(data_version);
    results.derived.ensure_version(data_version);

    reconcile_active_viewer(&mut app.state);

    show_docbar(ui, &mut app.state);

    let t = Tokens::get(ui.ctx());
    // The document well backdrop; viewers paint on top. The rect doubles
    // as the crop window for viewer PNG export.
    let well = ui.available_rect_before_wrap();
    ui.painter().rect_filled(well, 0.0, t.color.canvas_bg);
    app.state.ui.results.well_rect = Some(well);

    if !app.state.simulation.has_results() {
        let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
            crate::workbench::commands::Command::RunSimulation,
            crate::common::app::runtime_command_platform(ui.ctx()),
            ui.ctx().os(),
        );
        let hint = if shortcut.is_empty() {
            "No results yet — run a simulation".to_owned()
        } else {
            format!("No results yet — run a simulation ({shortcut})")
        };
        well_hint(ui, &hint);
        return;
    }

    let viewer = app.state.ui.results.viewer;
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
        ResultViewer::Op => op_inspector::show(ui, &mut app.state),
        ResultViewer::NoiseContrib => noise_contrib::show(ui, &mut app.state),
        ResultViewer::Specs => specs::show(ui, &mut app.state),
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
                    state.ui.export_csv_requested = true;
                    ui.close();
                }
                if ui.button("Viewer image (PNG)…").clicked() {
                    state.ui.export_png_requested = true;
                    ui.close();
                }
            });
            // Viewer-local controls.
            match state.ui.results.viewer {
                ResultViewer::Waves => {
                    let linked_shortcut = state.ui.preferences.shortcuts().resolved_label(
                        crate::workbench::commands::Command::ToggleLinkedCursors,
                        crate::common::app::runtime_command_platform(ui.ctx()),
                        ui.ctx().os(),
                    );
                    let results = &mut state.ui.results;
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
                    let linked_label = "Linked A/B cursors";
                    let linked = results.linked_cursors;
                    let linked_tooltip = if linked {
                        "A/B positions are shared across plots with matching analysis and X-axis domains"
                    } else {
                        "A/B positions are scoped to the active plot"
                    };
                    let linked_tooltip = if linked_shortcut.is_empty() {
                        linked_tooltip.to_owned()
                    } else {
                        format!("{linked_tooltip} · {linked_shortcut}")
                    };
                    let response = chip(ui, linked_label, linked).on_hover_text(linked_tooltip);
                    response.widget_info(|| {
                        WidgetInfo::selected(WidgetType::Button, true, linked, linked_label)
                    });
                    if !linked_shortcut.is_empty() {
                        ui.ctx().accesskit_node_builder(response.id, |node| {
                            node.set_keyboard_shortcut(linked_shortcut.as_str());
                        });
                    }
                    if response.clicked() {
                        results.toggle_linked_cursors();
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
                ResultViewer::Op => {
                    let filter = &mut state.ui.results.op_filter;
                    if !filter.is_empty() && chip(ui, "clear", true).clicked() {
                        filter.clear();
                    }
                    ui.add(
                        egui::TextEdit::singleline(filter)
                            .desired_width(150.0)
                            .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                            .hint_text("filter devices…"),
                    );
                }
                ResultViewer::Specs => {
                    if state.ui.results.spec_drafts.is_some() {
                        if ui.button("Discard").clicked() {
                            state.ui.results.spec_drafts = None;
                        }
                        if ui.button("Apply").clicked() && !specs::apply_drafts(state) {
                            state.push_sim_message(crate::common::app::ConsoleMessage::warning(
                                "Specs not applied — fix the invalid bound first",
                            ));
                        }
                    } else if ui.button("Edit specs…").clicked() {
                        specs::open_editor(state);
                    }
                }
                _ => {}
            }
        });
    });
}

/// The run shelf: run context + overlay control in one docbar popover.
///
/// Grammar per the results-v2 design: the active run reads in the button
/// (with the overlay count when comparing); the popover lists the history —
/// status dot, label, analyses, elapsed — with click-to-activate rows and
/// per-run overlay toggles ("signal owns hue, run owns weight").
fn run_selector(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let overlay_count = state.simulation.overlay_dataset_ids.len();
    let label = match state.simulation.active_run() {
        Some(run) if overlay_count > 0 => format!("run #{} +{overlay_count}", run.id),
        Some(run) => format!("run #{}", run.id),
        None => "no runs".to_owned(),
    };

    let mut select_run: Option<usize> = None;
    let mut toggle_overlay: Option<DatasetId> = None;
    let mut clear_overlays = false;

    ui.menu_button(
        egui::RichText::new(label).font(theme::mono(tokens::FS_1, FontWeight::Regular)),
        |ui| {
            ui.set_min_width(290.0);
            let active_id = state.simulation.active_run().map(|run| run.dataset_id);

            for (index, run) in state.simulation.runs.iter().enumerate() {
                let is_active = Some(run.dataset_id) == active_id;
                let overlaid = state.simulation.is_dataset_overlaid(run.dataset_id);

                ui.horizontal(|ui| {
                    // Status dot: ok / failed.
                    let (dot, _) =
                        ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                    ui.painter().circle_filled(
                        dot.center(),
                        3.0,
                        if run.success { t.color.ok } else { t.color.err },
                    );

                    // Row body: click activates the run.
                    let analyses: Vec<&str> = run
                        .analyses
                        .iter()
                        .map(|analysis| analysis.analysis_type.short_label())
                        .collect();
                    let body = format!(
                        "{} · {} · {:.1} s",
                        run.label,
                        if analyses.is_empty() {
                            "no analyses".to_owned()
                        } else {
                            analyses.join(" ")
                        },
                        run.elapsed_time
                    );
                    let accessibility_body = body.clone();
                    let text = egui::RichText::new(body)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(if is_active {
                            t.color.text
                        } else {
                            t.color.text_dim
                        });
                    let run_response = ui
                        .add(egui::Label::new(text).sense(egui::Sense::click()))
                        .on_hover_text("Activate this run");
                    run_response.widget_info(|| {
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::SelectableLabel,
                            ui.is_enabled(),
                            &accessibility_body,
                        )
                    });
                    ui.ctx().accesskit_node_builder(run_response.id, |node| {
                        node.set_role(egui::accesskit::Role::ListBoxOption);
                        node.set_selected(is_active);
                    });
                    theme::paint_focus_ring(ui, &run_response, run_response.rect);
                    if run_response.clicked() {
                        select_run = Some(index);
                        ui.close();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if is_active {
                            ui.label(
                                egui::RichText::new("active")
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.accent),
                            );
                        } else {
                            let mut on = overlaid;
                            if ui
                                .checkbox(&mut on, "overlay")
                                .on_hover_text(
                                    "Draw this run's traces under the active run \
                                     (same signal color, reduced weight)",
                                )
                                .changed()
                            {
                                toggle_overlay = Some(run.dataset_id);
                            }
                        }
                    });
                });
            }

            if overlay_count > 0 {
                ui.separator();
                if ui.button("Clear overlays").clicked() {
                    clear_overlays = true;
                    ui.close();
                }
            }
        },
    )
    .response
    .on_hover_text("Run history — activate a run or overlay runs for comparison");

    if let Some(index) = select_run {
        state.simulation.select_run(index);
        state.ui.results.clear_cursors();
    }
    if let Some(dataset_id) = toggle_overlay {
        state.simulation.toggle_dataset_overlay(dataset_id);
        state.ui.results.clear_cursors();
    }
    if clear_overlays {
        state.simulation.clear_dataset_overlays();
        state.ui.results.clear_cursors();
    }
}

fn viewer_tabs(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.spacing_mut().item_spacing.x = 2.0;

    let current = state.ui.results.viewer;
    let mut clicked: Option<ResultViewer> = None;

    for viewer in ResultViewer::PRIMARY {
        if viewer_tab(
            ui,
            viewer,
            current == viewer,
            viewer_availability(state, viewer),
        ) {
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
        if viewer_tab(
            ui,
            viewer,
            current == viewer,
            viewer_availability(state, viewer),
        ) {
            clicked = Some(viewer);
        }
    }
    if let Some(viewer) = clicked {
        state.ui.results.viewer = viewer;
    }
}

fn reconcile_active_viewer(state: &mut AppState) {
    if !state.simulation.has_results()
        || viewer_availability(state, state.ui.results.viewer).available
    {
        return;
    }
    if let Some(viewer) = ResultViewer::PRIMARY
        .into_iter()
        .chain(ResultViewer::LEGACY)
        .find(|viewer| viewer_availability(state, *viewer).available)
    {
        state.ui.results.viewer = viewer;
    }
}

fn viewer_availability(state: &AppState, viewer: ResultViewer) -> ViewerAvailability {
    let active_run = state.simulation.active_run();
    match viewer {
        ResultViewer::Waves => {
            if active_run.is_some_and(|run| {
                run.analyses
                    .iter()
                    .any(|analysis| !analysis.waveforms.is_empty())
            }) {
                ViewerAvailability::available("Waveforms are present in the active dataset")
            } else {
                ViewerAvailability::unavailable("Requires waveform data in the active dataset")
            }
        }
        ResultViewer::Bode => {
            if active_run
                .and_then(|run| {
                    crate::state::ac_bode_summary_for_selection(
                        run,
                        state.simulation.active_analysis_idx,
                    )
                })
                .is_some()
            {
                ViewerAvailability::available("AC magnitude response is available")
            } else {
                ViewerAvailability::unavailable(
                    "Requires a usable AC magnitude response in the active dataset",
                )
            }
        }
        ResultViewer::Fft => specialized_availability(state, ActiveViewer::Fft),
        ResultViewer::Eye => specialized_availability(state, ActiveViewer::EyeDiagram),
        ResultViewer::Hist => specialized_availability(state, ActiveViewer::Histogram),
        ResultViewer::Op => {
            if active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    analysis
                        .device_op
                        .as_ref()
                        .is_some_and(|report| !report.is_empty())
                })
            }) {
                ViewerAvailability::available("Device operating-point data is available")
            } else {
                ViewerAvailability::unavailable(
                    "Requires a device operating-point report in the active dataset",
                )
            }
        }
        ResultViewer::NoiseContrib => {
            if active_run.is_some_and(|run| {
                run.analyses.iter().any(|analysis| {
                    analysis
                        .noise_summary
                        .as_ref()
                        .is_some_and(|summary| !summary.rows.is_empty())
                })
            }) {
                ViewerAvailability::available("Noise-contributor data is available")
            } else {
                ViewerAvailability::unavailable("Requires a noise analysis with contributor data")
            }
        }
        ResultViewer::Specs => {
            let has_measurements = state.simulation.runs.iter().any(|run| {
                run.analyses
                    .iter()
                    .any(|analysis| !analysis.measurements.is_empty())
            });
            if !state.workspace.specs.is_empty() || has_measurements {
                ViewerAvailability::available("Specification or measurement data is available")
            } else {
                ViewerAvailability::unavailable("Requires specifications or measured results")
            }
        }
        ResultViewer::Nyquist => specialized_availability(state, ActiveViewer::Nyquist),
        ResultViewer::Smith => specialized_availability(state, ActiveViewer::SmithChart),
        ResultViewer::PoleZero => specialized_availability(state, ActiveViewer::PoleZero),
    }
}

fn specialized_availability(state: &AppState, viewer: ActiveViewer) -> ViewerAvailability {
    let capability = state.viewer_capability(viewer);
    ViewerAvailability {
        available: capability.available,
        reason: capability.reason,
    }
}

/// One viewer tab, per the design: a 24 px chip with 11 px side padding,
/// letterspaced mono label, hover fill, and an accent wash when active.
fn viewer_tab(
    ui: &mut Ui,
    viewer: ResultViewer,
    active: bool,
    availability: ViewerAvailability,
) -> bool {
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
    let galley = ui.fonts_mut(|f| f.layout_job(job));

    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(galley.size().x + 22.0, 24.0),
        if availability.available {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::SelectableLabel,
            availability.available && ui.is_enabled(),
            viewer.label(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        if active {
            node.set_selected(true);
        } else {
            node.set_selected(false);
        }
    });
    if !ui.is_rect_visible(rect) {
        return false;
    }

    let hover = ui.ctx().animate_bool_with_time(
        response.id,
        availability.available && !active && response.hovered(),
        ui.style().animation_time,
    );
    let (fill, text_color) = if active {
        (c.accent_dim, c.accent)
    } else if !availability.available {
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

    theme::paint_focus_ring(ui, &response, rect);

    if !availability.available {
        response.on_hover_text(availability.reason);
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
    match state.ui.results.viewer {
        ResultViewer::Waves => waves::right_panel(ui, state),
        ResultViewer::Bode => bode::right_panel(ui, state),
        ResultViewer::Fft => fft::right_panel(ui, state),
        ResultViewer::Eye => eye::right_panel(ui, state),
        ResultViewer::Hist => hist::right_panel(ui, state),
        ResultViewer::Op => op_inspector::right_panel(ui, state),
        ResultViewer::NoiseContrib => noise_contrib::right_panel(ui, state),
        ResultViewer::Specs => specs::right_panel(ui, state),
        ResultViewer::Nyquist => nyquist::right_panel(ui, state),
        ResultViewer::Smith => smith::right_panel(ui, state),
        ResultViewer::PoleZero => pz::right_panel(ui, state),
    }
}

/// Status-bar summary for the results workspace.
pub fn status_summary(state: &AppState) -> String {
    let viewer = state.ui.results.viewer;
    match state.simulation.active_run() {
        Some(run) => format!("{} · run #{}", viewer.status(), run.id),
        None => viewer.status().to_owned(),
    }
}

#[cfg(test)]
mod availability_tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, WaveformData};

    fn state_with_analysis(analysis: AnalysisResult) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        state
    }

    #[test]
    fn waveform_data_does_not_enable_incompatible_result_viewers() {
        let state = state_with_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![0.0, 1.0, 0.0],
                    "#00aaff",
                ),
            ]),
        );

        assert!(viewer_availability(&state, ResultViewer::Waves).available);
        assert!(!viewer_availability(&state, ResultViewer::Bode).available);
        assert!(!viewer_availability(&state, ResultViewer::Hist).available);
        assert!(!viewer_availability(&state, ResultViewer::NoiseContrib).available);
    }

    #[test]
    fn incompatible_active_viewer_falls_back_to_compatible_view() {
        let mut state = state_with_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new(
                    "V(out)",
                    vec![0.0, 1.0, 2.0],
                    vec![0.0, 1.0, 0.0],
                    "#00aaff",
                ),
            ]),
        );
        state.ui.results.viewer = ResultViewer::Hist;

        reconcile_active_viewer(&mut state);

        assert_eq!(state.ui.results.viewer, ResultViewer::Waves);
    }
}
