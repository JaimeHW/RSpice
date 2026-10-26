//! Where a result signal is drawn on the open cell view: the cross-probe map
//! a run retained, read only while it belongs to the drawing as it is now.
//!
//! The run keys its net geometry by sheet, so a signal is looked up on the
//! sheet the canvas shows. A drawing edited since the run has no current map,
//! and nothing is selected from a stale one.

use rspice_schematic_editor::view::cross_probe::{self, LocateSignalError};
use rspice_simulation::netlist_gen::SheetNetGeometry;

use super::Model;

impl Model {
    /// Select a result signal only through the map belonging to the active
    /// drawing.
    pub fn select_signal_conductor(&mut self, signal: &str) -> Result<String, LocateSignalError> {
        let current = self.result_mapping_is_current();
        let occurrence = self.workspace.content.occurrence_path();
        let sheet = self.active_sheet_id();
        let key = self.workspace.content.active_schematic_reference().key();
        let (document, editor) = self.schematic.document_and_editor();
        let view = cross_probe::CrossProbeView {
            document,
            sheet_catalog: self.workspace.content.design_management.sheet_catalog(&key),
            occurrence: &occurrence,
            net_to_points: current.then_some(
                &self
                    .simulation
                    .source
                    .cross_probe
                    .on_sheet(sheet)
                    .net_to_points,
            ),
        };
        cross_probe::select_signal_conductor(&view, editor, signal)
    }

    /// Whether the retained cross-probe map belongs to the open cell as it is
    /// drawn right now.
    pub fn result_mapping_is_current(&self) -> bool {
        self.simulation.source.cross_probe.is_current_for(
            &self.workspace.content.active_view,
            self.schematic.topology_version(),
        )
    }

    /// The net geometry the active sheet draws, in its own coordinates.
    pub fn active_sheet_net_geometry(&self) -> &SheetNetGeometry {
        self.simulation
            .source
            .cross_probe
            .on_sheet(self.active_sheet_id())
    }
}
