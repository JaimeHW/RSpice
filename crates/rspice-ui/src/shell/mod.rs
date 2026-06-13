//! The IDE shell.
//!
//! Composes the design system (`crate::ui`) and the application state into
//! the full IDE chrome, top to bottom:
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────┐
//! │ menubar    File Edit View Create Check Simulate …        │
//! ├──────────────────────────────────────────────────────────┤
//! │ toolbar    tools · zoom · history · checks   corner ▾ Run│
//! ├──────────────────────────────────────────────────────────┤
//! │ wtabs      Library  Schematic·cell  Netlist  Sim  Results│
//! ├────────────┬────────────────────────────────┬────────────┤
//! │ left panel │        center view             │ right panel│
//! ├────────────┴────────────────────────────────┴────────────┤
//! │ console    Console  Errors  Warnings              ⌫  ▾   │
//! ├──────────────────────────────────────────────────────────┤
//! │ statusbar  x/y · grid · sel        corner · engine · zoom│
//! └──────────────────────────────────────────────────────────┘
//! ```
//!
//! The shell renders chrome and dispatches into the action layer; document
//! and simulation state stay in `crate::state` / `crate::simulation`.

mod console;
pub(crate) mod menubar;
pub(crate) mod panels;
pub mod results;
mod state;
mod statusbar;
mod toolbar;
mod views;
mod wtabs;

pub use results::{ResultViewer, ResultsState};
pub use state::{
    ConsoleUiState, GridStyle, NavMode, RailTab, ShellState, ShellStateSer, WorkspaceView,
};

use egui::Context;

use crate::common::RSpiceApp;

/// Render one frame of the IDE shell.
///
/// Panel order matters in egui: top bars first, then bottom bars (status bar
/// outermost), side panels, and the central view last.
pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    menubar::show(ctx, app);
    toolbar::show(ctx, &mut app.state);
    wtabs::show(ctx, &mut app.state);
    statusbar::show(ctx, &mut app.state);
    console::show(ctx, &mut app.state);
    panels::show(ctx, &mut app.state, app.symbol_library.as_ref());
    views::show(ctx, app);

    // Deferred requests that need IO backends owned by the app.
    if std::mem::take(&mut app.state.shell.export_csv_requested) {
        crate::common::menu_bar::action_export_csv_with_io(
            &mut app.state,
            app.export_workflow_io.as_ref(),
        );
    }

    // The canvas reports hover coordinates only on frames it renders; clear
    // them otherwise so the status bar doesn't show stale positions.
    if app.state.shell.view != WorkspaceView::Schematic {
        app.state.shell.canvas_hover = None;
    }

    app.state.shell.toasts.show(ctx);
}
