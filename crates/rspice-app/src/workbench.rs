//! The RSpice application shell: the frame loop, the application state that
//! holds every feature's slot, and the one dispatcher between them.
//!
//! Each feature lives in its own crate (`rspice-app-design`,
//! `rspice-app-results`, `rspice-app-code` and the rest) with its surface,
//! navigator, inspector, dialogs and commands, and the containers they draw
//! into (chrome, docks, routing, the command registry) live in
//! `rspice-app-workbench`. What stays here is what only the whole
//! application can do:
//!
//! - `app` — the eframe application, [`RSpiceApp`], and its frame loop, with
//!   the dialogs and actions that still reach the whole session.
//! - `app_state` — [`AppState`], everything a session owns.
//! - `slots` — the state each feature owns, one slot per feature, and the
//!   shell's own.
//! - `features` — the features the shell registers, and the context each one
//!   is lent from the application state.
//! - `dispatch` — the one dispatcher every command goes through, and the
//!   answers to what a feature's call raised, taken straight after it.
//! - `commands` — the shell's own commands, their vocabulary, and the edit
//!   target the routed edit commands follow.
//! - `shell_host` — which owner draws at each contribution point.
//! - `frame` — one frame: chrome, docks and the central surface.
//! - `menu_bar` — the behaviour behind the application menus.
//! - `session` and `workflows` — the transactions over the whole project,
//!   and the multi-step operations a command runs.
//! - `diagnostic_routes` — following a finding to the object it is about.
//! - `chrome` — the status bar's platform segment.
//! - `logging` and `system_contrast` — the native logger's defaults, and the
//!   contrast theme Windows runs.
//! - `state` and `examples` — the workbench session's layout tests, and the
//!   hierarchy fixture and example-circuit tests the suites build from.
//!
//! Its interior layering is ordered and ratcheted by
//! `tests/app_conventions.rs`, not by this file.
pub(crate) mod app;
pub(crate) mod app_state;
pub(crate) mod chrome {
    pub(crate) mod status_bar {
        pub(in crate::workbench) mod shell;
    }
}
pub(crate) mod commands;
pub(crate) mod diagnostic_routes;
mod dispatch;
// The bundled circuits Help's Examples dialog opens as projects are model
// logic in `rspice_app_state::workbench::examples`. The test fixtures beside
// them build through the application's own APIs, so they stay here.
#[cfg(test)]
pub(crate) mod examples {
    /// The one hierarchical project the hierarchy suites build from. It is a
    /// fixture like the flat example circuits, but a whole project rather
    /// than a schematic, so it is authored through the application's own APIs
    /// instead of being drawn.
    pub(crate) mod hierarchy_reference;

    mod netlist_integration_tests;
}
pub(crate) mod features;
pub(crate) mod frame;
pub(crate) mod logging;
pub(crate) mod menu_bar;
pub(crate) mod system_contrast;
/// The shell's transactions over the whole project, run where a feature's
/// work asks for one.
pub(crate) mod session {
    pub(crate) mod netlist_import_transactions;
    pub(crate) mod project_transactions;
}
pub(crate) mod shell_host;
pub(crate) mod slots;
pub(crate) mod state;
pub(crate) mod workflows;

pub use app::RSpiceApp;
pub use app_state::AppState;
pub(crate) use frame::{show, show_embedded_secondary, show_secondary};
