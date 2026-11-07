//! File ▸ Export ▸ Bill of materials: the schematic's purchased parts as CSV,
//! one line per value and part number.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use rspice_design::schematic::bom::{bom_csv, bom_lines};

const FAILURE: &str = "Bill of materials export failed";

pub(crate) fn action_export_bill_of_materials_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let document = state.model.schematic.document();
    let lines = bom_lines(
        document
            .components
            .iter()
            .map(|component| (component.name.as_str(), component)),
    );
    if lines.is_empty() {
        state.prompt("This schematic has no purchased parts to list.");
        return;
    }
    let csv = bom_csv(&lines);
    let default_name = format!("{}-bom.csv", state.model.export_file_stem("circuit"));
    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Bill of Materials",
        default_name: &default_name,
        filter_name: "CSV",
        filter_extensions: &["csv"],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, "csv");
            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &csv));
            match export {
                Ok(()) => {
                    state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "Bill of materials",
                            &path,
                            None,
                            io,
                        ),
                    );
                }
                Err(error) => state.report_failure(FAILURE, error.to_string()),
            }
        }
        Ok(None) => {}
        Err(error) => state.report_failure(FAILURE, error.to_string()),
    }
}
