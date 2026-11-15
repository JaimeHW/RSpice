//! Contract-driven, responsive RSpice application workbench.
//!
//! This is the sole owner of application chrome and top-level task layout.
//! Document engines such as the schematic canvas and precision result viewers
//! render inside its central surface but do not own navigation or chrome.
//!
//! # What the shell is made of
//!
//! The declarations below are one sorted list. They were previously three
//! blocks accreted by the `common` merge and the `panels` fold, with a thread
//! helper wedged between the first two and 470 lines of frame rendering after
//! the last — so the module root was the least useful place to look for the
//! shell's shape. Rendering now lives in [`frame`], platform branching in
//! [`platform`].
//!
//! Five kinds of view container appear here, and the distinction is load
//! bearing:
//!
//! - `chrome` — the frame around everything: title bar, activity rail,
//!   document bar, toolbar, status bar. Always present, never a document.
//! - `docks` — context panels that flank the central surface (navigator,
//!   inspector, console, drawers). Dismissible; they never own a document.
//! - `surfaces` — the canonical task owners that fill the central area. One
//!   is active at a time, and it is what the current route names.
//! - `tools` — application-modal tools that float above the active surface
//!   (project launcher, jobs list, calculator, notification centre). They
//!   own neither a route nor a document: they open over whatever is beneath,
//!   do one job, and close.
//! - `app/dialogs` — modal dialogs. They block the surface beneath until
//!   dismissed, which is what separates them from a `tools` float.
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
