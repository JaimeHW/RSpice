//! File ▸ Export ▸ Netlist. The export writes the deck the run executes, with
//! every enabled analysis's card and its Verilog-A beside it, so the file runs
//! in the CLI as the design runs here. With a deck in front of the Netlist
//! workspace it writes a copy of that deck instead. It records in the
//! model's Console, and a refusal is prompted on the session's status bar.

use super::netlist_export::record_netlist_warnings_in;
use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::Model;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use crate::workbench::{MessageCatalog, MessageId, WorkbenchSession};

/// Export a copy of `deck` as it is in the editor, edits not yet saved
/// included, to the file the reader picks: in the deck's own encoding and
/// line separators, its include lines as written. The deck itself stays
/// where it is and keeps its file.
pub fn action_export_deck_with_io(
    model: &mut Model,
    deck: &crate::workbench::documents::deck_files::OpenDeck,
    io: &(impl ExportWorkflowIo + ?Sized),
    messages: MessageCatalog,
) {
    let failed = messages.text(MessageId::NetlistDeckExportFailed);
    let file = &deck.root;
    let name = file.name();
    let extension = std::path::Path::new(&name)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("cir")
        .to_owned();
    let bytes = match file.encoding.encode(&file.line_ending.write(&file.text)) {
        Ok(bytes) => bytes,
        Err(error) => {
            model.report_failure(failed, error);
            return;
        }
    };
    let picked = io.show_save_dialog(SaveDialogConfig {
        title: &messages.text(MessageId::NetlistExportDeckTitle),
        default_name: &name,
        filter_name: &messages.text(MessageId::NetlistDeckFilter),
        filter_extensions: &[extension.as_str()],
    });
    let mut path = match picked {
        Ok(Some(path)) => path,
        Ok(None) => return,
        Err(error) => {
            model.report_failure(failed, error);
            return;
        }
    };
    crate::io::file_exchange::ensure_file_extension(&mut path, &extension);
    let written = io
        .observe_destination(&path)
        .and_then(|destination| io.write_bytes_file_observed(&destination, &bytes, "text/plain"));
    match written {
        Ok(()) => {
            model.record(
                LogSeverity::Info,
                LogSource::Studio,
                crate::workbench::workflows::export_workflow::export_completion_message(
                    &messages.text(MessageId::NetlistExportedDeck),
                    &path,
                    None,
                    io,
                ),
            );
        }
        Err(error) => model.report_failure(failed, error),
    }
}

pub fn action_export_netlist_with_io(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    // The deck the run executes, with every enabled analysis's card, so the
    // file runs in the CLI as the design runs here.
    let messages = wb.messages();
    let failed = messages.text(MessageId::NetlistExportFailedTitle);
    if model.schematic.document().components.is_empty() {
        wb.prompt(messages.text(MessageId::NetlistExportNoCircuit));
        return;
    }
    let exported =
        match crate::simulation::controller::SimulationController::prepare_design_netlist_for_export(
            model,
        ) {
            Ok(exported) => exported,
            Err(error) => {
                model.report_failure(failed, error.to_string());
                return;
            }
        };
    record_netlist_warnings_in(model, exported.warnings.clone());

    let extension = "spice";
    let default_name = format!("{}.{extension}", model.export_file_stem("circuit"));
    let filter_name = messages.text(MessageId::NetlistNetlistFilter);

    match io.show_save_dialog(SaveDialogConfig {
        title: &messages.text(MessageId::NetlistExportNetlistTitle),
        default_name: &default_name,
        filter_name: &filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, extension);

            // The deck names its sources relative to where it was saved, so
            // only now can it be written.
            let deck_stem = path
                .file_stem()
                .map_or_else(|| "netlist".into(), |stem| stem.to_string_lossy());
            let export = super::netlist_compat::spice_export(
                &exported.netlist,
                &model.workspace,
                &exported.model_library_veriloga,
                &deck_stem,
            );
            let (netlist_content, sources) = (export.netlist, export.sources);
            if !sources.is_empty() && !io.saved_paths_are_reopenable() {
                wb.prompt(messages.text(MessageId::NetlistExportVerilogANeedsZip));
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
                    let detail = (!sources.is_empty()).then(|| {
                        messages.counted(
                            sources.len(),
                            MessageId::NetlistExportVerilogABesideSingular,
                            MessageId::NetlistExportVerilogABeside,
                        )
                    });
                    model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            &filter_name,
                            &path,
                            detail,
                            io,
                        ),
                    );
                }
                Err(e) => model.report_failure(failed, e.to_string()),
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => model.report_failure(failed, e.to_string()),
    }
}
