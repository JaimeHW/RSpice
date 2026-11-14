//! File ▸ Export ▸ Netlist and View Netlist: the deck the code feature
//! generates, recorded in the model's Console and saved through the export
//! workflow. A refusal is prompted on the session's status bar.

use super::netlist_export::{
    build_menu_netlist_in, netlist_failure_text, record_netlist_warnings_in,
};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::WorkbenchSession;
use crate::workbench::app_state::Model;
use crate::workbench::state::Workspace;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub fn action_export_netlist_with_io(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let netlist_content = if wb.workspace == Workspace::Netlist {
        if model.netlist_artifacts.generated_source.is_empty() {
            wb.prompt("There is no generated netlist to export.");
            return;
        }
        if model.netlist_artifacts.generation_error.is_some()
            || model.netlist_artifacts.generated_input_digest
                != model.netlist_artifacts.current_generation_input_digest
        {
            wb.prompt("The generated netlist is out of date; regenerate it to export.");
            return;
        }
        model.netlist_artifacts.generated_source.clone()
    } else {
        if model.schematic.document().components.is_empty() {
            wb.prompt("No circuit to export. Add components first.");
            return;
        }
        match build_menu_netlist_in(model) {
            Ok(generated) => {
                record_netlist_warnings_in(model, generated.warnings);
                generated.source
            }
            Err(reasons) => {
                model.report_failure("Netlist export failed", reasons.join("; "));
                return;
            }
        }
    };

    let default_name = format!(
        "{}.{}",
        model.export_file_stem("circuit"),
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
                        &model.workspace,
                        &deck_stem,
                    );
                    (export.netlist, export.sources)
                }
                _ => (netlist_content, Vec::new()),
            };
            if !sources.is_empty() && !io.saved_paths_are_reopenable() {
                wb.prompt(
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
                    model.record(
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
                Err(e) => model.report_failure("Netlist export failed", e.to_string()),
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => model.report_failure("Netlist export failed", e.to_string()),
    }
}

pub fn action_view_netlist(model: &mut Model, wb: &mut WorkbenchSession) {
    if model.schematic.document().components.is_empty() {
        wb.prompt("No circuit to netlist. Add components first.");
        return;
    }

    let netlist_content = match build_menu_netlist_in(model) {
        Ok(generated) => {
            record_netlist_warnings_in(model, generated.warnings);
            generated.source
        }
        Err(reasons) => {
            model.record(
                LogSeverity::Error,
                LogSource::Studio,
                netlist_failure_text("Netlist generation", &reasons),
            );
            return;
        }
    };

    // The Netlist workspace this command opens shows the deck itself.
    let total_lines = netlist_content.lines().count();
    model.simulation.source.netlist_content = netlist_content;
    model.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!("Generated the netlist ({total_lines} lines)."),
    );
}
