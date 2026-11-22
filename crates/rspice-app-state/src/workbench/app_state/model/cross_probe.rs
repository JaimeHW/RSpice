//! Where a result signal is drawn on the open cell view: the cross-probe map
//! a run retained, read only while it belongs to the drawing as it is now.
//!
//! The run keys its net geometry by sheet, so a signal is looked up on the
//! sheet the canvas shows first, and a net drawn only on another sheet brings
//! that sheet forward. A drawing edited since the run has no current map,
//! and nothing is selected from a stale one.

use rspice_schematic_editor::view::cross_probe::{self, LocateSignalError};
use rspice_simulation::netlist_gen::SheetNetGeometry;

use super::Model;
use crate::state::SheetId;

impl Model {
    /// Select a result signal only through the map belonging to the active
    /// drawing: on the sheet in view where it draws the net, else on the
    /// first sheet that does, which comes forward.
    ///
    /// Only a Results surface asks this, so no canvas gesture can be under
    /// way on the sheet being left.
    pub fn select_signal_conductor(&mut self, signal: &str) -> Result<String, LocateSignalError> {
        let located = self.select_signal_conductor_on_active_sheet(signal);
        let Err(LocateSignalError::UnknownNet(leaf)) = &located else {
            return located;
        };
        let Some(sheet) = self.other_sheet_drawing_net(leaf) else {
            return located;
        };
        if self.activate_sheet(sheet, |_| {}).is_err() {
            return located;
        }
        self.select_signal_conductor_on_active_sheet(signal)
    }

    /// The first sheet in catalog order, other than the one in view, whose
    /// retained geometry draws the net `leaf`.
    fn other_sheet_drawing_net(&self, leaf: &str) -> Option<SheetId> {
        if !self.result_mapping_is_current() {
            return None;
        }
        let active = self.active_sheet_id();
        self.multi_sheet_catalog()?
            .sheets()
            .iter()
            .map(|sheet| sheet.id())
            .filter(|id| Some(*id) != active)
            .find(|id| self.sheet_draws_net(Some(*id), leaf))
    }

    /// Whether the retained geometry of the open drawing draws the net
    /// `leaf` on any of its sheets, which is what a locate can reach.
    pub fn drawing_draws_net(&self, leaf: &str) -> bool {
        self.sheet_draws_net(self.active_sheet_id(), leaf)
            || self.multi_sheet_catalog().is_some_and(|catalog| {
                catalog
                    .sheets()
                    .iter()
                    .any(|sheet| self.sheet_draws_net(Some(sheet.id()), leaf))
            })
    }

    fn sheet_draws_net(&self, sheet: Option<SheetId>, leaf: &str) -> bool {
        self.simulation
            .source
            .cross_probe
            .on_sheet(sheet)
            .net_to_points
            .iter()
            .any(|(name, points)| name.eq_ignore_ascii_case(leaf) && !points.is_empty())
    }

    fn select_signal_conductor_on_active_sheet(
        &mut self,
        signal: &str,
    ) -> Result<String, LocateSignalError> {
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
