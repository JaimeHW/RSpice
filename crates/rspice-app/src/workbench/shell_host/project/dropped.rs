//! A file dropped on the window opens through the route File > Open takes
//! for it, wherever it lands: the launcher, the landing or any workspace. A
//! project opens with the review of the unsaved work it would replace; a
//! schematic opens on the start screen or comes into the open project as a
//! cell view; a SPICE deck opens as a netlist project or comes into the open
//! one. What a page takes itself, a Verilog-A source anywhere and a deck on
//! the Netlist page, is left to that page.

use std::path::Path;

use egui::{Context, DroppedFile};

use crate::workbench::RSpiceApp;
use crate::workbench::shell_host::project::launcher::dismiss_launcher;
use rspice_app_state::diagnostics::{LogSeverity, LogSource};
use rspice_app_state::workbench::workspace::Workspace;

/// What a dropped file is, by its extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dropped {
    Project,
    Schematic,
    Deck,
    /// Nothing the shell opens: a page may take it.
    Other,
}

fn kind_of(file: &DroppedFile) -> Dropped {
    let extension = Path::new(&name_of(file))
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    match extension.as_deref() {
        Some("rspiceproj") => Dropped::Project,
        Some("rsch") => Dropped::Schematic,
        Some("cir" | "sp" | "spice" | "net" | "ckt") => Dropped::Deck,
        _ => Dropped::Other,
    }
}

/// The dropped file's name: the one the drop carries, else its path's.
fn name_of(file: &DroppedFile) -> String {
    (!file.name.trim().is_empty())
        .then(|| file.name.clone())
        .or_else(|| {
            file.path
                .as_deref()
                .and_then(Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_default()
}

/// Open what was dropped on the window this frame, if the shell opens it,
/// before any page draws: a page that takes drops never sees a project or a
/// schematic as one of its own files.
pub(crate) fn open_dropped_file(ctx: &Context, app: &mut RSpiceApp) {
    let dropped = ctx.input(|input| input.raw.dropped_files.clone());
    if dropped.is_empty() || app.state.application_modal_open() {
        return;
    }
    let kinds: Vec<Dropped> = dropped.iter().map(kind_of).collect();
    let project_open = app.state.platform.project_lifecycle.is_open();
    let start_screen = !project_open || app.state.project.project_launcher_open;
    if kinds.iter().all(|kind| *kind == Dropped::Other) {
        // The start screen has no page that takes a file of its own.
        if start_screen {
            app.state.prompt(
                "RSpice opens .rspiceproj projects, .rsch schematics and .cir, .sp, .spice, \
                 .net and .ckt SPICE decks.",
            );
        }
        return;
    }
    let netlist_page = project_open && app.state.workbench.workspace == Workspace::Netlist;
    if netlist_page
        && kinds
            .iter()
            .all(|kind| matches!(kind, Dropped::Deck | Dropped::Other))
    {
        return;
    }
    // From here the drop is the shell's, and no page reads it.
    ctx.input_mut(|input| input.raw.dropped_files.clear());
    let [file] = dropped.as_slice() else {
        app.state
            .prompt("Drop one project, schematic or SPICE deck at a time.");
        return;
    };
    match kinds[0] {
        Dropped::Project => open_project(app, file),
        Dropped::Schematic => open_schematic(app, file),
        Dropped::Deck => open_deck(app, file),
        Dropped::Other => {}
    }
}

/// A project opens as File > Open recent opens one: its unsaved work
/// reviewed first. In the browser it opens from the bytes the drop handed
/// over, and its first save asks where.
fn open_project(app: &mut RSpiceApp, file: &DroppedFile) {
    #[cfg(not(target_arch = "wasm32"))]
    let (path, unread) = (
        file.path.clone(),
        "The dropped project has no file on this computer to open.",
    );
    #[cfg(target_arch = "wasm32")]
    let (path, unread) = (
        keep_dropped(file),
        "The browser did not hand over the dropped project's contents.",
    );
    let Some(path) = path else {
        app.state.prompt(unread);
        return;
    };
    if app.found_recent_file(&path) {
        dismiss_launcher(app);
        app.open_recent_file(recent(
            rspice_app_services::services::recent_files::RecentKind::Project,
            path,
        ));
    }
}

/// A schematic comes into the open project as a cell view, as File > Open
/// brings one in; on the start screen it opens as a document of its own,
/// its unsaved work reviewed first.
fn open_schematic(app: &mut RSpiceApp, file: &DroppedFile) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let Some(path) = file.path.clone() else {
            app.state
                .prompt("The dropped schematic has no file on this computer to open.");
            return;
        };
        if app.state.platform.project_lifecycle.is_open() {
            let (state, io) = (&mut app.state, app.file_workflow_io.as_ref());
            if state.load_schematic_from_path_with_io(&path, io) {
                state.workbench.activate(Workspace::Design);
            }
        } else if app.found_recent_file(&path) {
            dismiss_launcher(app);
            app.open_recent_file(recent(
                rspice_app_services::services::recent_files::RecentKind::Schematic,
                path,
            ));
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let Some(path) = keep_dropped(file) else {
            app.state
                .prompt("The browser did not hand over the dropped schematic's contents.");
            return;
        };
        if app.state.platform.project_lifecycle.is_open() {
            if let Some(bytes) = take_dropped(&path)
                && app.state.open_dropped_schematic(&name_of(file), &bytes)
            {
                app.state.workbench.activate(Workspace::Design);
            }
        } else {
            dismiss_launcher(app);
            app.open_recent_file(recent(
                rspice_app_services::services::recent_files::RecentKind::Schematic,
                path,
            ));
        }
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    /// The project or schematic last dropped on the browser window, under
    /// the path its open is asked for by, until that open takes it.
    static DROPPED: std::cell::RefCell<Option<(std::path::PathBuf, std::sync::Arc<[u8]>)>> =
        const { std::cell::RefCell::new(None) };
}

/// Keep what the browser handed over for `file` until its open takes it,
/// under a path that names no file: the open reviews unsaved work first, as
/// a recent file's does. `None` when the drop carried no contents.
#[cfg(target_arch = "wasm32")]
fn keep_dropped(file: &DroppedFile) -> Option<std::path::PathBuf> {
    let bytes = file.bytes.clone()?;
    let path = Path::new("dropped").join(name_of(file));
    DROPPED.with(|kept| *kept.borrow_mut() = Some((path.clone(), bytes)));
    Some(path)
}

/// The contents of the file dropped under `path`, once.
#[cfg(target_arch = "wasm32")]
pub(crate) fn take_dropped(path: &Path) -> Option<std::sync::Arc<[u8]>> {
    DROPPED.with(|kept| {
        let mut kept = kept.borrow_mut();
        if kept.as_ref().is_some_and(|(dropped, _)| dropped == path) {
            kept.take().map(|(_, bytes)| bytes)
        } else {
            None
        }
    })
}

/// A deck comes into the open project as the Netlist page brings one in, or
/// opens as a netlist project of its own.
fn open_deck(app: &mut RSpiceApp, file: &DroppedFile) {
    if app.state.platform.project_lifecycle.is_open() && !app.state.project.project_launcher_open {
        app.state.import_dropped_netlist(vec![file.clone()]);
        return;
    }
    let source_path = file.path.clone();
    let bytes = if let Some(bytes) = file.bytes.as_ref() {
        Ok(bytes.to_vec())
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            source_path
                .as_deref()
                .ok_or_else(|| "the drop carried neither its contents nor a file".to_owned())
                .and_then(|path| std::fs::read(path).map_err(|error| error.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err("the browser did not hand over the dropped file's contents".to_owned())
        }
    };
    match bytes {
        Ok(bytes) => {
            let name = name_of(file);
            if app
                .state
                .stage_dropped_netlist_project(bytes, source_path, name)
            {
                dismiss_launcher(app);
            }
        }
        Err(error) => {
            let message = format!("The dropped SPICE deck could not be read: {error}.");
            app.state.prompt(message.clone());
            app.state
                .record(LogSeverity::Error, LogSource::Studio, message);
        }
    }
}

fn recent(
    kind: rspice_app_services::services::recent_files::RecentKind,
    path: std::path::PathBuf,
) -> rspice_app_services::services::recent_files::RecentFile {
    rspice_app_services::services::recent_files::RecentFile {
        kind,
        path,
        opened_at_unix_ms: 0,
        pinned: false,
        missing: false,
    }
}
