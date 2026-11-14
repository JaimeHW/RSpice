//! The project launcher's composition over the whole application.

use egui::{Context, Popup};
use std::path::Path;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::services_access::open_comparison;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::lifecycle::recovery::{
    open_diagnostics_folder, refresh_catalog_if_requested,
};
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::workspace_state::ProjectLauncherPage;
use crate::workbench::session::project_transactions::start_local_safe_mode;
use crate::workbench::state::Workspace;
use crate::workbench::tools::project_launcher::{
    LAUNCHER_TOUCH_MAX_WIDTH, LauncherAction, LauncherControlHeightOverride, LauncherPass,
    launcher, restore_launcher_focus, show_discard_confirmation,
};
use crate::workbench::{RSpiceApp, features};

fn handle_launcher_netlist_drop(ctx: &Context, app: &mut RSpiceApp) -> bool {
    let dropped = ctx.input(|input| input.raw.dropped_files.clone());
    if dropped.is_empty() {
        return false;
    }
    if dropped.len() != 1 {
        app.state
            .prompt("Drop one SPICE deck at a time; each opens as one netlist project.");
        return false;
    }

    let file = &dropped[0];
    let source_path = file.path.clone();
    let display_name = (!file.name.trim().is_empty())
        .then(|| file.name.clone())
        .or_else(|| {
            source_path
                .as_deref()
                .and_then(Path::file_name)
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "dropped-netlist.spice".to_owned());
    let supported = Path::new(&display_name)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "cir" | "sp" | "spice" | "net" | "ckt"
            )
        });
    if !supported {
        app.state
            .prompt("The project launcher accepts .cir, .sp, .spice, .net, and .ckt SPICE decks.");
        return false;
    }

    let bytes = if let Some(bytes) = file.bytes.as_ref() {
        Ok(bytes.to_vec())
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            source_path
                .as_deref()
                .ok_or_else(|| "Dropped file has neither bytes nor a native path.".to_owned())
                .and_then(|path| std::fs::read(path).map_err(|error| error.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err("The browser did not provide the dropped file's contents.".to_owned())
        }
    };
    match bytes {
        Ok(bytes) => {
            if app
                .state
                .stage_dropped_netlist_project(bytes, source_path, display_name)
            {
                dismiss_launcher(app);
                true
            } else {
                false
            }
        }
        Err(error) => {
            app.state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Dropped SPICE source could not be read: {error}"),
            );
            false
        }
    }
}

/// The launcher, drawn in the project feature's context, with what the
/// reader asked of it carried out by the shell as soon as it has drawn and
/// before its focus is handed back.
pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp) {
    if !app.state.project.project_launcher_open {
        show_discard_confirmation(ctx, &mut app.state.platform);
        return;
    }

    if handle_launcher_netlist_drop(ctx, app) {
        return;
    }

    refresh_catalog_if_requested(
        &mut app.state.platform,
        &app.state.model,
        app.file_workflow_io.as_ref(),
    );

    let screen = ctx.content_rect();
    let large_targets = app.state.workbench.coarse_pointer
        || screen.width() <= LAUNCHER_TOUCH_MAX_WIDTH
        || ctx.input(|input| input.has_touch_screen());
    let _control_height = LauncherControlHeightOverride::new(ctx, large_targets);
    let LauncherPass {
        action,
        focus_state_id,
        surface_id,
        modal_layer,
    } = launcher(ctx, &mut features::cx(app), screen, large_targets);

    if let Some(action) = action {
        match action {
            LauncherAction::Close => dismiss_launcher(app),
            LauncherAction::Browse => {
                dismiss_launcher(app);
                app.execute_command_key(ProjectCommand::OpenProject.key());
            }
            LauncherAction::NewProject => {
                dismiss_launcher(app);
                app.execute_command_key(ProjectCommand::NewProject.key());
            }
            LauncherAction::OpenNetlist => {
                dismiss_launcher(app);
                app.execute_command_key(ids::OPEN_NETLIST);
            }
            LauncherAction::Open(recent) => {
                dismiss_launcher(app);
                app.open_recent_file(recent);
            }
            LauncherAction::Page(page) => {
                app.state.project.project_launcher_page = page;
                if page == ProjectLauncherPage::Recovery {
                    app.state.platform.recovery_catalog.request_refresh();
                }
            }
            LauncherAction::Recover(candidate) => match open_comparison(&mut app.state, &candidate)
            {
                Ok(()) => dismiss_launcher(app),
                Err(error) => app.state.platform.recovery_catalog.warning(error),
            },
            LauncherAction::RequestDiscard(candidate) => {
                app.state.platform.recovery_catalog.pending_discard = Some(candidate);
            }
            LauncherAction::OpenDiagnosticsFolder => {
                match open_diagnostics_folder(&app.state.model, &app.state.platform) {
                    Ok(path) => app
                        .state
                        .prompt(format!("Opened diagnostic folder: {}", path.display())),
                    Err(error) => {
                        app.state.record(
                            LogSeverity::Error,
                            LogSource::Studio,
                            format!("Diagnostic folder could not be opened: {error}"),
                        );
                    }
                }
            }
            LauncherAction::StartSafeMode(options) => {
                match start_local_safe_mode(&mut app.state, options) {
                    Ok(()) => app.state.project.project_launcher_open = false,
                    Err(error) => app.state.platform.recovery_catalog.warning(error),
                }
            }
        }
    }
    if !app.state.project.project_launcher_open {
        Popup::close_all(ctx);
        restore_launcher_focus(ctx, focus_state_id, surface_id, modal_layer);
    }
    show_discard_confirmation(ctx, &mut app.state.platform);
}

/// Close the modal presentation and leave its canonical route in the same
/// transaction. Keeping those two state owners synchronized prevents a
/// browser-back traversal from immediately reopening a dismissed launcher.
pub(crate) fn dismiss_launcher(app: &mut RSpiceApp) {
    app.state.project.project_launcher_open = false;
    if app.state.workbench.current_route().surface_id()
        != crate::workbench::SurfaceId::ProjectLauncher
    {
        return;
    }
    // With nothing open, every workspace but the no-project landing would
    // present the bootstrap placeholder as the reader's work, so a dismissal
    // never returns to one.
    let project_open = app.state.platform.project_lifecycle.is_open();
    if project_open
        && app
            .state
            .workbench
            .navigate_back(crate::workbench::RouteTransitionSource::User)
            .is_some()
    {
        return;
    }
    // One replace transition. A back traversal followed by a push is not
    // atomic in browsers: the delayed `popstate` would clear the push and
    // restore the launcher's predecessor.
    let workspace = if project_open {
        app.state.workbench.workspace
    } else {
        Workspace::Project
    };
    let fallback = crate::workbench::SurfaceRoute::surface(
        crate::workbench::SurfaceId::from_workspace(workspace),
    );
    if let Err(error) = app
        .state
        .workbench
        .replace_route(fallback, crate::workbench::RouteTransitionSource::User)
    {
        app.state.workbench.record_route_diagnostic(format!(
            "The Project Launcher closed, but its fallback workspace could not be restored: {error}"
        ));
    }
}
