//! Shell state — everything the IDE chrome needs that is not document or
//! simulation data: the active workspace view, console UI state, toast queue,
//! theme selection, and small cross-frame signals (canvas hover coordinates,
//! results badge bookkeeping).

use serde::{Deserialize, Serialize};

use crate::ui::Theme;
use crate::ui::widgets::Toasts;

/// The five top-level workspaces, matching the design's workspace tab strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WorkspaceView {
    /// Library manager (library / cell / view columns).
    Library,
    /// Schematic editor (default).
    #[default]
    Schematic,
    /// Generated netlist viewer.
    Netlist,
    /// Simulation setup (analyses, variables, outputs, corners).
    Simulate,
    /// Results (waveform strips and analysis viewers).
    Results,
}

impl WorkspaceView {
    /// All views in tab order.
    pub const ALL: [WorkspaceView; 5] = [
        WorkspaceView::Library,
        WorkspaceView::Schematic,
        WorkspaceView::Netlist,
        WorkspaceView::Simulate,
        WorkspaceView::Results,
    ];

    /// Tab label (the schematic tab appends the active cell name at render
    /// time).
    pub fn label(self) -> &'static str {
        match self {
            WorkspaceView::Library => "Library",
            WorkspaceView::Schematic => "Schematic",
            WorkspaceView::Netlist => "Netlist",
            WorkspaceView::Simulate => "Simulate",
            WorkspaceView::Results => "Results",
        }
    }

    /// Whether this view shows the contextual side panels.
    pub fn has_side_panels(self) -> bool {
        matches!(
            self,
            WorkspaceView::Schematic | WorkspaceView::Simulate | WorkspaceView::Results
        )
    }
}

/// Console severity filter (the console's tab strip).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ConsoleFilter {
    /// All messages.
    #[default]
    All,
    /// Errors only.
    Errors,
    /// Warnings only.
    Warnings,
    /// The interactive automation console.
    Script,
}

/// Console chrome state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleUiState {
    /// Collapsed to the 29 px header strip.
    pub collapsed: bool,
    /// Expanded height in points.
    pub height: f32,
    /// Active severity filter tab.
    #[serde(skip)]
    pub filter: ConsoleFilter,
    /// Entry indices matching the active filter — the virtualized log's
    /// row → entry map, rebuilt only when the buffer or filter changes.
    #[serde(skip)]
    pub(crate) rows: Vec<u32>,
    /// (filter, buffer revision, buffer length) the row map was built for.
    #[serde(skip)]
    pub(crate) rows_key: Option<(ConsoleFilter, u64, usize)>,
}

/// An in-flight inspector edit session: the inspected component and the
/// pre-edit snapshot. One undo entry commits when the session ends (focus
/// leaves the fields or the selection moves) — not per keystroke.
#[derive(Debug, Clone)]
pub struct InspectorEdit {
    /// Component being edited.
    pub component_id: u64,
    /// Design state captured when the first keystroke landed.
    pub before: crate::state::SchematicSnapshot,
}

impl Default for ConsoleUiState {
    fn default() -> Self {
        Self {
            collapsed: false,
            height: 176.0,
            filter: ConsoleFilter::All,
            rows: Vec::new(),
            rows_key: None,
        }
    }
}

/// Persistent + per-frame state of the IDE shell.
#[derive(Debug, Clone, Default)]
pub struct ShellState {
    /// Active workspace view.
    pub view: WorkspaceView,
    /// Theme selection (direction / mode / density).
    pub theme: Theme,
    /// Console chrome state.
    pub console: ConsoleUiState,
    /// Toast queue.
    pub toasts: Toasts,
    /// Schematic-space cursor position, reported by the canvas each frame the
    /// pointer hovers it; consumed by the status bar.
    pub canvas_hover: Option<(f64, f64)>,
    /// Schematic-space center of the visible canvas, reported each frame the
    /// canvas renders; paste targets fall back to it when the cursor is not
    /// over the canvas (menu-driven paste).
    pub canvas_view_center: Option<(f64, f64)>,
    /// `simulation.data_version` at the last time the Results view was shown;
    /// drives the "new results" badge on the Results tab.
    pub results_seen_version: u64,
    /// Selected process corner name (e.g. "tt") shown in the toolbar and
    /// status bar and applied to model selection.
    pub corner: String,
    /// Selected analysis row in the Simulate view (drives the right-panel
    /// analysis inspector).
    pub selected_analysis: Option<usize>,
    /// Canvas dot-grid visibility.
    pub show_grid: bool,
    /// Hide both contextual side panels (focus mode).
    pub panels_hidden: bool,
    /// One-shot request to focus the component search box (Create ▸ Instance,
    /// `i` shortcut).
    pub focus_cell_search: bool,
    /// Component browser search query.
    pub cell_search: String,
    /// Component browser library filter ("All libs", "primitives", or a
    /// library name).
    pub cell_lib_filter: String,
    /// Selected row in the component browser list.
    pub cell_selected: Option<usize>,
    /// One-shot request to export the visible waveforms as CSV (needs the
    /// app's IO backend, so it is handled at the shell level).
    pub export_csv_requested: bool,
    /// One-shot request to capture the window as a PNG (native: viewport
    /// screenshot + save dialog; the web build offers the browser's own
    /// capture instead).
    pub export_png_requested: bool,
    /// Results workspace state (viewer, cursors, plot caches).
    pub results: super::results::ResultsState,
    /// In-flight inspector edit session, if any.
    pub inspector_edit: Option<InspectorEdit>,
}

impl ShellState {
    /// New shell state with defaults (Instrument dark, compact, schematic
    /// view, "tt" corner, grid on).
    pub fn new() -> Self {
        Self {
            corner: "tt".to_owned(),
            show_grid: true,
            cell_lib_filter: "All libs".to_owned(),
            ..Self::default()
        }
    }

    /// Cycle to the next workspace view (Window ▸ Next workspace tab).
    pub fn cycle_view(&mut self) {
        let idx = WorkspaceView::ALL
            .iter()
            .position(|v| *v == self.view)
            .unwrap_or(0);
        self.view = WorkspaceView::ALL[(idx + 1) % WorkspaceView::ALL.len()];
    }

    /// Restore the default layout (panel visibility, console size).
    pub fn reset_layout(&mut self) {
        self.console = ConsoleUiState::default();
        self.panels_hidden = false;
        self.show_grid = true;
    }
}

/// Serialized subset of [`ShellState`] (layout + theme survive restarts;
/// transient signals do not).
#[derive(Serialize, Deserialize)]
pub struct ShellStateSer {
    #[serde(default)]
    view: WorkspaceView,
    #[serde(default)]
    theme: Theme,
    #[serde(default)]
    console: ConsoleUiState,
    #[serde(default = "default_corner")]
    corner: String,
    #[serde(default = "default_true")]
    show_grid: bool,
    #[serde(default)]
    panels_hidden: bool,
    #[serde(default)]
    result_viewer: super::results::ResultViewer,
    /// User expression traces per waves strip: (analysis index, expression).
    #[serde(default)]
    expr_traces: Vec<(usize, String)>,
}

fn default_corner() -> String {
    "tt".to_owned()
}

fn default_true() -> bool {
    true
}

impl Default for ShellStateSer {
    fn default() -> Self {
        Self::from(&ShellState::new())
    }
}

impl From<&ShellState> for ShellStateSer {
    fn from(shell: &ShellState) -> Self {
        let mut expr_traces: Vec<(usize, String)> = shell
            .results
            .exprs
            .iter()
            .flat_map(|(&analysis, traces)| {
                traces
                    .iter()
                    .map(move |trace| (analysis, trace.text.clone()))
            })
            .collect();
        expr_traces.sort();
        Self {
            view: shell.view,
            theme: shell.theme,
            console: shell.console.clone(),
            corner: shell.corner.clone(),
            show_grid: shell.show_grid,
            panels_hidden: shell.panels_hidden,
            result_viewer: shell.results.viewer,
            expr_traces,
        }
    }
}

impl From<ShellStateSer> for ShellState {
    fn from(ser: ShellStateSer) -> Self {
        let mut shell = Self {
            view: ser.view,
            theme: ser.theme,
            console: ser.console,
            corner: ser.corner,
            show_grid: ser.show_grid,
            panels_hidden: ser.panels_hidden,
            ..Self::new()
        };
        shell.results.viewer = ser.result_viewer;
        for (analysis, text) in ser.expr_traces {
            shell
                .results
                .exprs
                .entry(analysis)
                .or_default()
                .push(super::results::ExprTrace {
                    text,
                    visible: true,
                });
        }
        shell
    }
}
