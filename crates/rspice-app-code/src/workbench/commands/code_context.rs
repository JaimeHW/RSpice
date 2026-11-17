//! Whether Compile has a Verilog-A source to compile, and why not.
//!
//! Menus, shortcuts, the command palette, and direct toolbar dispatch must
//! agree on it. The check deliberately fails for incomplete, stale, or
//! internally inconsistent state instead of guessing from a visible label or
//! file extension.

use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::documents::code_workspace::{
    CodeWorkspaceRuntimeState, selected_veriloga_editor_snapshot_in,
};

/// Whether Compile has something to compile, wherever it is offered from.
///
/// A Verilog-A cell view is a design document, so while one is the active
/// Design document it is compiled from any workspace — the Models toolbar and
/// menu included — and the command takes the reader to it. With no Verilog-A
/// view active there is nothing to compile.
#[must_use]
pub(crate) fn resolve_veriloga_compile(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
) -> bool {
    if model.safe_mode.project_read_only()
        || !platform.project_lifecycle.is_open()
        || !model.workspace.content.active_view_is_hdl_source()
        || code.veriloga.pending.is_some()
    {
        return false;
    }
    let Some(selected) = selected_veriloga_editor_snapshot_in(model, code)
        .ok()
        .filter(|selected| !selected.selection_was_invalid())
    else {
        return false;
    };
    model
        .workspace
        .content
        .project_sources
        .get_bundle(selected.bundle_id())
        .is_some_and(|bundle| {
            bundle.file_content(selected.active_path()).is_some()
                && !bundle.root().content().trim().is_empty()
        })
}

/// Why [`resolve_veriloga_compile`] finds nothing to compile, checked in its
/// own order, so a grey Compile row names the reason the compile would have
/// been refused.
pub(crate) fn veriloga_compile_refusal(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
) -> &'static str {
    if model.safe_mode.project_read_only() {
        "the project is open read-only"
    } else if !platform.project_lifecycle.is_open() {
        "no project is open"
    } else if !model.workspace.content.active_view_is_hdl_source() {
        "open a Verilog-A cell view in Design"
    } else if code.veriloga.pending.is_some() {
        "a Verilog-A compile is already running"
    } else {
        "the Verilog-A cell view has no source to compile"
    }
}
