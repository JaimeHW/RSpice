//! File ▸ Export ▸ Netlist and View Netlist. The export writes the deck the
//! run executes, with every enabled analysis's card and its Verilog-A beside
//! it, so the file runs in the CLI as the design runs here; View Netlist shows
//! the design deck the code feature generates. Both record in the model's
//! Console, and a refusal is prompted on the session's status bar.

use super::netlist_export::{
    build_menu_netlist_in, netlist_failure_text, record_netlist_warnings_in,
};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::WorkbenchSession;
use crate::workbench::app_state::Model;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub fn action_export_netlist_with_io(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    // The deck the run executes, with every enabled analysis's card, so the
    // file runs in the CLI as the design runs here.
    if model.schematic.document().components.is_empty() {
        wb.prompt("No circuit to export. Add components first.");
        return;
    }
    let exported =
        match crate::simulation::controller::SimulationController::prepare_design_netlist_for_export(
            model,
        ) {
            Ok(exported) => exported,
            Err(error) => {
                model.report_failure("Netlist export failed", error.to_string());
                return;
            }
        };
    record_netlist_warnings_in(model, exported.warnings.clone());

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
            let deck_stem = path
                .file_stem()
                .map_or_else(|| "netlist".into(), |stem| stem.to_string_lossy());
            let export = match format {
                crate::io::NetlistFormat::Spectre => super::netlist_compat::spice_to_ahdl_export(
                    &exported.netlist,
                    &model.workspace,
                    &exported.model_library_veriloga,
                    &deck_stem,
                ),
                crate::io::NetlistFormat::Spice => super::netlist_compat::spice_export(
                    &exported.netlist,
                    &model.workspace,
                    &exported.model_library_veriloga,
                    &deck_stem,
                ),
            };
            let (netlist_content, sources) = (export.netlist, export.sources);
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
