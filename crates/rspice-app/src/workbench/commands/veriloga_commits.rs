//! Committing the code feature's two Verilog-A forms, answered for its
//! requests: File > Import Verilog-A/AMS and New Verilog-A device….
//!
//! The code feature makes the cells over the model and its own state
//! (app/dialogs/library). What it made is a cell view, which is a Design
//! document, so opening it reaches past the code feature. The shell does that
//! part here, in this order: it opens the view, brings Design forward, puts a
//! new device's starter receipt in front, and records what was made. The two
//! stay methods on the app so that the code feature's tests reach them the
//! way the shell's arms do.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::CellViewRef;
use crate::workbench::RSpiceApp;
use crate::workbench::app::dialogs::library::{new_veriloga_device, veriloga_import};
use crate::workbench::state::Workspace;

impl RSpiceApp {
    /// Create the import form's cells and open the first in Design.
    pub(in crate::workbench) fn commit_veriloga_import(&mut self) -> Result<CellViewRef, String> {
        let state = &mut self.state;
        let imported = veriloga_import::publish_veriloga_import(
            &mut state.model,
            &mut state.code.code_workspace.veriloga,
        )?;
        // A cell view is a Design document; the first module opens there.
        state.open_workspace_view(imported.first.clone());
        state.workbench.activate(Workspace::Design);
        state.record(LogSeverity::Info, LogSource::Studio, imported.record);
        Ok(imported.first)
    }

    /// Create the drafted device's cell and open its module in Design.
    pub(in crate::workbench) fn commit_new_veriloga_device(
        &mut self,
    ) -> Result<CellViewRef, String> {
        let state = &mut self.state;
        let created = new_veriloga_device::publish_new_veriloga_device(
            &mut state.model,
            &mut state.code.new_veriloga_device,
            &mut state.code.code_workspace.veriloga,
            &state.workbench.messages(),
        )?;
        // The module is a view of the cell, so it opens as a Design document.
        state.open_workspace_view(created.reference.clone());
        state.workbench.activate(Workspace::Design);
        state.code.code_workspace.veriloga.receipt = Some(created.receipt);
        state.record(LogSeverity::Info, LogSource::Studio, created.record);
        Ok(created.reference)
    }
}
