//! Following a console line's anchor.
//!
//! Split from the page because it is a different concern from painting a
//! line: everything here is about whether a line's target still exists and
//! what happens when the reader commits to it. A run's line opens that run's
//! log on the page itself; any other anchor leaves the page for the drawing or
//! symbol it names, which is the session's to do once the page has drawn.

use egui::Ui;

use crate::diagnostics::LogAnchor;
use crate::workbench::framework::workbench_host::ConsoleHost;
use crate::workbench::state::{ConsoleChannel, ConsoleView};

/// Follow `anchor`, from a double-click, Enter or the context menu.
///
/// A jump off the page is handed back through `jump`: the page draws from the
/// session immutably, and navigating mutates it. Returns whether following
/// leaves the page. An anchor whose target is gone does nothing; the line said
/// so under the pointer before the reader committed.
pub fn follow(
    state: &impl ConsoleHost,
    view: &mut ConsoleView,
    anchor: &LogAnchor,
    jump: &mut Option<LogAnchor>,
) -> bool {
    if state.log_anchor_refusal(anchor).is_some() {
        return false;
    }
    match anchor {
        LogAnchor::ResultRun { run_sequence } => {
            view.open(ConsoleChannel::Run(*run_sequence));
            false
        }
        other => {
            *jump = Some(other.clone());
            true
        }
    }
}

/// The context menu's item for a line's anchor. Returns whether choosing it
/// leaves the page.
pub(super) fn menu(
    ui: &mut Ui,
    state: &impl ConsoleHost,
    view: &mut ConsoleView,
    anchor: &LogAnchor,
    jump: &mut Option<LogAnchor>,
) -> bool {
    let label = match anchor {
        LogAnchor::ResultRun { .. } => "Open run log",
        LogAnchor::Symbol { .. } => "Show in symbol view",
        LogAnchor::Schematic { .. } | LogAnchor::Simulation { .. } => "Show on schematic",
    };
    let refusal = state.log_anchor_refusal(anchor);
    let item = ui.add_enabled(refusal.is_none(), egui::Button::new(label));
    if let Some(refusal) = refusal {
        item.on_disabled_hover_text(refusal);
        return false;
    }
    if !item.clicked() {
        return false;
    }
    ui.close();
    follow(state, view, anchor, jump)
}

/// What following this line's anchor will do, in the words of the surface it
/// lands on.
pub(super) fn hint(anchor: &LogAnchor) -> String {
    match anchor {
        LogAnchor::Schematic { .. } => "Double-click to show on the schematic".to_owned(),
        LogAnchor::Symbol { pin_name, .. } => {
            format!("Double-click to show pin {pin_name} in the symbol view")
        }
        LogAnchor::Simulation { nets, devices } => {
            let count = nets.len() + devices.len();
            let objects = if count == 1 { "object" } else { "objects" };
            format!("Double-click to mark the {count} {objects} this run named on the schematic")
        }
        LogAnchor::ResultRun { run_sequence } => {
            format!("Double-click to open the log of Run {run_sequence}")
        }
    }
}
