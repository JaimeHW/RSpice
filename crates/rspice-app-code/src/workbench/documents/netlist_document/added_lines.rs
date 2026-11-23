//! The Added lines tab: the lines the active plan's root design adds to its
//! generated netlist, edited where they are kept, in the project.
//!
//! Each edit is the project's at once; Ctrl+S saves the project. A line the
//! generated netlist cannot take (an analysis, which belongs to the plan, or
//! an `.end`) is marked as an error on its own line, and Run refuses the plan
//! naming it.

use super::{Model, NetlistDocumentState, NetlistTab};
use crate::state::CellViewRef;

/// The design whose Added lines the tab shows: the active plan's root
/// design, the one the generated netlist is written for. `None` when the
/// project has no design to netlist.
#[must_use]
pub fn added_lines_design(model: &Model) -> Option<CellViewRef> {
    model
        .workspace
        .configuration_execution_projection(
            &model.library_manager,
            &model.workspace.content.active_view,
            &model.schematic,
        )
        .ok()
        .map(|projection| projection.root().clone())
}

/// The Added lines of the active plan's root design, empty when it has none.
#[must_use]
pub fn added_lines_text(model: &Model) -> String {
    added_lines_design(model)
        .and_then(|design| {
            rspice_design::added_lines::added_lines_of(
                &model.workspace.content.added_lines,
                &design,
            )
            .map(str::to_owned)
        })
        .unwrap_or_default()
}

/// Bring the Added lines to the front; Run then runs the plan. Whether the
/// project has a design whose Added lines these are.
pub fn show_added_lines(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    if added_lines_design(model).is_none() {
        return false;
    }
    netlist.run_the_plan();
    let text = added_lines_text(model);
    if netlist.front == NetlistTab::AddedLines
        && netlist.active_document_initialized
        && model.simulation.source.netlist_content == text
    {
        return true;
    }
    netlist.front = NetlistTab::AddedLines;
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content.set(text);
    netlist.requested_line = None;
    netlist.cursor_line = 0;
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    true
}

/// Keep `text`, the editor's Added lines, as the root design's; whether that
/// changed them.
pub fn edit_added_lines(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    text: String,
) -> bool {
    if netlist.front != NetlistTab::AddedLines {
        return false;
    }
    let Some(design) = added_lines_design(model) else {
        return false;
    };
    if !rspice_design::added_lines::set_added_lines(
        &mut model.workspace.content.added_lines,
        &design,
        text.clone(),
    ) {
        return false;
    }
    model.workspace.content.project_metadata_dirty = true;
    model.simulation.source.netlist_content.set(text);
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    super::clear_netlist_edit_journal(netlist);
    true
}

/// The check of the Added lines: every line the generated netlist cannot
/// take, as an error on its own line.
pub(super) fn refusals(text: &str) -> Vec<crate::state::netlist_diagnostics::Diagnostic> {
    rspice_simulation::netlist_gen::added_lines_refusals(text)
        .into_iter()
        .map(|refusal| {
            crate::state::netlist_diagnostics::Diagnostic::error(refusal.message)
                .with_line(Some(refusal.line.saturating_sub(1)))
        })
        .collect()
}
