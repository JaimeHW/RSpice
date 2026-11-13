//! File ▸ Export ▸ Netlist and View Netlist over the session: the deck the
//! code feature generates, prompted, recorded and saved through the
//! session. These take the session whole, so they stay in the shell when
//! the code feature moves into a crate of its own.

use super::netlist_export::{
    GeneratedNetlist, build_menu_netlist_in, netlist_failure_text, record_netlist_warnings_in,
};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub(crate) fn action_export_netlist_with_io(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let netlist_content =
        if state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
            if state.model.netlist_artifacts.generated_source.is_empty() {
                state.prompt("There is no generated netlist to export.");
                return;
            }
            if state.model.netlist_artifacts.generation_error.is_some()
                || state.model.netlist_artifacts.generated_input_digest
                    != state
                        .model
                        .netlist_artifacts
                        .current_generation_input_digest
            {
                state.prompt("The generated netlist is out of date; regenerate it to export.");
                return;
            }
            state.model.netlist_artifacts.generated_source.clone()
        } else {
            if state.model.schematic.document().components.is_empty() {
                state.prompt("No circuit to export. Add components first.");
                return;
            }
            match build_menu_netlist(state) {
                Ok(generated) => {
                    record_netlist_warnings(state, generated.warnings);
                    generated.source
                }
                Err(reasons) => {
                    state.report_failure("Netlist export failed", reasons.join("; "));
                    return;
                }
            }
        };

    let default_name = format!(
        "{}.{}",
        state.model.export_file_stem("circuit"),
        format.extension()
    );

    let filter_name = match format {
        crate::io::NetlistFormat::Spectre => "Spectre Netlist",
        crate::io::NetlistFormat::Spice => "SPICE Netlist",
    };

    let extension = format.extension();

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Netlist",
        default_name: &default_name,
        filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, extension);

            // The deck names its sources relative to where it was saved, so
            // only now can it be written.
            let (netlist_content, sources) = match format {
                crate::io::NetlistFormat::Spectre => {
                    let deck_stem = path
                        .file_stem()
                        .map_or_else(|| "netlist".into(), |stem| stem.to_string_lossy());
                    let export = super::netlist_compat::spice_to_ahdl_export(
                        &netlist_content,
                        &state.model.workspace,
                        &deck_stem,
                    );
                    (export.netlist, export.sources)
                }
                _ => (netlist_content, Vec::new()),
            };
            if !sources.is_empty() && !io.saved_paths_are_reopenable() {
                state.prompt(
                    "This netlist uses project Verilog-A sources; export it from the Netlist \
                     page as a self-contained bundle.",
                );
                return;
            }
            let folder = path.parent().map(std::path::Path::to_path_buf);
            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &netlist_content))
                .and_then(|()| {
                    sources.iter().try_for_each(|source| {
                        let target = folder
                            .as_deref()
                            .unwrap_or(std::path::Path::new(""))
                            .join(&source.relative_path);
                        let destination = io.observe_destination(&target)?;
                        io.write_text_file_observed(&destination, &source.content)
                    })
                });
            match export {
                Ok(()) => {
                    let detail = (!sources.is_empty())
                        .then(|| format!("{} Verilog-A source files beside it", sources.len()));
                    state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            filter_name,
                            &path,
                            detail,
                            io,
                        ),
                    );
                }
                Err(e) => state.report_failure("Netlist export failed", e.to_string()),
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => state.report_failure("Netlist export failed", e.to_string()),
    }
}

pub(crate) fn action_view_netlist(state: &mut AppState) {
    if state.model.schematic.document().components.is_empty() {
        state.prompt("No circuit to netlist. Add components first.");
        return;
    }

    let netlist_content = match build_menu_netlist(state) {
        Ok(generated) => {
            record_netlist_warnings(state, generated.warnings);
            generated.source
        }
        Err(reasons) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                netlist_failure_text("Netlist generation", &reasons),
            );
            return;
        }
    };

    // The Netlist workspace this command opens shows the deck itself.
    let total_lines = netlist_content.lines().count();
    state.model.simulation.source.netlist_content = netlist_content;
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!("Generated the netlist ({total_lines} lines)."),
    );
}

/// Generate the design's deck.
///
/// Nothing here writes the Console. A command that asked for the deck records
/// the outcome; the Netlist page states it where the deck would be. The error
/// is every reason the generator refused, in its order.
pub(crate) fn build_menu_netlist(state: &AppState) -> Result<GeneratedNetlist, Vec<String>> {
    build_menu_netlist_in(&state.model)
}

/// Record a generated deck's warnings, once each, as the Console's account of
/// what the generator substituted or ignored.
pub(crate) fn record_netlist_warnings(state: &mut AppState, warnings: Vec<String>) {
    record_netlist_warnings_in(&mut state.model, warnings);
}
