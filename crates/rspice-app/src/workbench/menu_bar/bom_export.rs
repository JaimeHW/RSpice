//! File ▸ Export ▸ Bill of materials: the purchased parts of the design
//! below the open cellview, as CSV, one line per value and part number. A
//! part inside a cell is counted once per occurrence, under its
//! hierarchical reference (`X1/R3`).

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use rspice_design::hierarchy::HierarchySource;
use rspice_design::schematic::bom::{bom_csv, bom_lines};

const FAILURE: &str = "Bill of materials export failed";

pub(crate) fn action_export_bill_of_materials_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let active = state.model.workspace.content.active_view.clone();
    let projection = match state.model.workspace.design_projection(
        &state.model.library_manager,
        &active,
        &state.model.schematic,
    ) {
        Ok(projection) => projection,
        Err(error) => {
            state.report_failure(FAILURE, error.to_string());
            return;
        }
    };
    let hierarchy =
        HierarchySource::from_design_projection(state.model.library_manager.catalog(), &projection);
    let parts = hierarchy.bill_of_materials_parts(state.model.schematic.document(), &active);
    let lines = bom_lines(
        parts
            .iter()
            .map(|(reference, component)| (reference.as_str(), *component)),
    );
    if lines.is_empty() {
        state.prompt("This design has no purchased parts to list.");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point};
    use crate::workbench::workflows::export_workflow::ExportWorkflowIo;
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};

    #[derive(Debug, Default)]
    struct CaptureIo {
        csv: RefCell<String>,
    }

    impl ExportWorkflowIo for CaptureIo {
        fn show_save_dialog(
            &self,
            _config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            Ok(Some(PathBuf::from("design-bom.csv")))
        }

        fn write_text_file(&self, _path: &Path, contents: &str) -> Result<(), String> {
            self.csv.replace(contents.to_owned());
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Ok(())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            false
        }
    }

    /// The export reads the open design through its projection: the parts
    /// it buys are listed, and the simulator's source is not.
    #[test]
    fn the_export_lists_the_purchased_parts_of_the_open_design() {
        let mut app = crate::workbench::RSpiceApp::test_instance();
        let schematic = &mut app.state.model.schematic;
        schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
        schematic.add_component(ComponentType::VoltageSource, Point::new(200, 0));
        let io = CaptureIo::default();

        action_export_bill_of_materials_with_io(&mut app.state, &io);

        let csv = io.csv.borrow();
        assert_eq!(
            csv.split("\r\n").skip(1).collect::<Vec<_>>(),
            ["R1,1,1k,,,,,,,", ""],
            "{csv}"
        );
    }
}
