//! The name an export of the open design proposes, read from the model alone,
//! so the schematic picture and the netlist deck propose the same name.

use super::Model;

impl Model {
    /// The name an export proposes before its extension: the schematic file's
    /// own name when it is one, otherwise the cell it exports, with the view
    /// where the cell has more than one kind ("amp_testbench"); `fallback` only
    /// when there is neither. Inside a project there is no schematic file, and
    /// every export used to propose the fallback.
    pub fn export_file_stem(&self, fallback: &str) -> String {
        if let Some(stem) = self
            .schematic
            .session
            .current_file
            .as_ref()
            .and_then(|path| path.file_stem())
        {
            return stem.to_string_lossy().into_owned();
        }
        let reference = self.workspace.content.active_schematic_reference();
        match (reference.cell.trim(), reference.view.trim()) {
            ("", _) => fallback.to_owned(),
            (cell, "" | "schematic") => cell.to_owned(),
            (cell, view) => format!("{cell}_{view}"),
        }
    }
}
