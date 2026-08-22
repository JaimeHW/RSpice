//! Where a reported location sends the reader.
//!
//! One resolution path for every route into the Netlist workspace: a Problems
//! row, an inspector action, and the simulation-preflight report all name a
//! location the same way and land the same way. Split from the document
//! module so the routing is one readable unit rather than two functions
//! buried between the document projections.

// `AppState` comes through the parent rather than from `crate::workbench`:
// `app_state` is being retired, and a submodule of it does not need its own
// edge to it — the same route `comparison` takes.
use super::{
    AppState, CodeWorkspacePage, canonical_root_document, open_netlist_dependency_from_root,
};

/// Open the exact source location owned by a canonical diagnostic. Problems,
/// inspector actions, and future validation reports all route through this
/// function so included-source diagnostics cannot be misrepresented as root
/// buffer lines.
pub(crate) fn open_diagnostic_location(
    state: &mut AppState,
    diagnostic_id: uuid::Uuid,
) -> Result<(), String> {
    let diagnostic = state
        .ui
        .netlist
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.canonical.diagnostic_id == diagnostic_id)
        .cloned()
        .ok_or_else(|| "The selected diagnostic is no longer available.".to_owned())?;
    if !diagnostic.is_current() {
        return Err(
            "The selected diagnostic belongs to stale source; validate the current document first."
                .to_owned(),
        );
    }

    let included_source = diagnostic
        .line
        .is_none()
        .then_some(diagnostic.source_path.as_deref())
        .flatten();
    open_source_location(
        state,
        included_source,
        diagnostic.line.or(diagnostic.source_line),
    )
}

/// Open one source location in the Netlist workspace.
///
/// `included_source`, when present, names a file inside the root document's
/// authenticated include closure; it is resolved through that closure so an
/// included-source location cannot be misrepresented as a root-buffer line,
/// and an unresolvable one refuses rather than landing somewhere plausible.
///
/// Problems rows and the simulation-preflight report both route here. That is
/// the point: a location has to mean the same thing wherever it was reported
/// from, and the preflight report used to have no route to the netlist at all.
pub(crate) fn open_source_location(
    state: &mut AppState,
    included_source: Option<&std::path::Path>,
    line: Option<usize>,
) -> Result<(), String> {
    let root = state
        .ui
        .netlist
        .active_dependency_root
        .unwrap_or(state.ui.netlist.active_document);
    if let Some(source_path) = included_source {
        let dependency_identity = canonical_root_document(state, root)
            .and_then(|document| {
                document.dependencies().iter().find(|dependency| {
                    let locator = dependency.locator();
                    locator.native_origin().is_some_and(|origin| {
                        std::path::Path::new(origin) == source_path
                    }) || std::path::Path::new(locator.logical_identity()) == source_path
                        || std::path::Path::new(locator.locator()) == source_path
                })
            })
            .map(|dependency| dependency.locator().logical_identity().to_owned())
            .ok_or_else(|| {
                format!(
                    "Diagnostic source {} is no longer present in the authenticated include closure.",
                    source_path.display()
                )
            })?;
        open_netlist_dependency_from_root(state, root, &dependency_identity)?;
    }

    state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    if let Some(line) = line {
        state.ui.netlist.cursor_line = line;
        state.ui.netlist.requested_line = Some(line.saturating_add(1));
    }
    Ok(())
}
