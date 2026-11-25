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
use crate::state::{Point, SheetId};

/// What a result signal was shown as on the drawing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalPlace {
    /// A net's conductor, spelled as the design spells it.
    Conductor(String),
    /// The part a current flows through, by the name the drawing gives it.
    Part(String),
    /// The pin of a part a current is read at, by the names the drawing and
    /// the part's symbol give them.
    Pin { part: String, pin: String },
}

impl SignalPlace {
    /// What the status line says once `signal` is shown.
    pub fn message(&self, signal: &str) -> String {
        match self {
            Self::Conductor(net) => format!("Selected conductor {net} from {signal}."),
            Self::Part(part) => format!("Selected {part} from {signal}."),
            Self::Pin { part, pin } => format!("Selected pin {pin} of {part} from {signal}."),
        }
    }
}

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
        self.current_sheet_net_geometry(sheet)
            .is_some_and(|geometry| {
                geometry
                    .net_to_points
                    .iter()
                    .any(|(name, points)| name.eq_ignore_ascii_case(leaf) && !points.is_empty())
            })
    }

    fn select_signal_conductor_on_active_sheet(
        &mut self,
        signal: &str,
    ) -> Result<String, LocateSignalError> {
        let occurrence = self.workspace.content.occurrence_path();
        let sheet = self.active_sheet_id();
        let topology = self.schematic.topology_version();
        let key = self.workspace.content.active_schematic_reference().key();
        let (document, editor) = self.schematic.document_and_editor();
        let view = cross_probe::CrossProbeView {
            document,
            sheet_catalog: self.workspace.content.design_management.sheet_catalog(&key),
            occurrence: &occurrence,
            net_to_points: self
                .simulation
                .source
                .cross_probe
                .on_sheet_at(
                    &self.workspace.content.active_view,
                    &occurrence,
                    topology,
                    sheet,
                )
                .map(|geometry| &geometry.net_to_points),
        };
        cross_probe::select_signal_conductor(&view, editor, signal)
    }

    /// Show a result signal where the drawing in front carries it: a net's
    /// conductor, the part a current flows through, or, for a current read at
    /// one pin, that pin of its part. The sheet that draws it comes forward.
    /// Read only through a map current for the drawing at this occurrence;
    /// a signal read in another occurrence says which.
    pub fn select_signal_on_drawing(
        &mut self,
        signal: &str,
    ) -> Result<SignalPlace, LocateSignalError> {
        if cross_probe::probed_device(signal).is_none() {
            return self
                .select_signal_conductor(signal)
                .map(SignalPlace::Conductor);
        }
        let (component, _, name) = self.locate_signal_device(signal)?;
        if let Some(sheet) = self
            .multi_sheet_catalog()
            .and_then(|catalog| catalog.sheet_for_object(component))
            && self.active_sheet_id() != Some(sheet)
            && self.activate_sheet(sheet, |_| {}).is_err()
        {
            return Err(LocateSignalError::UnknownDevice(name));
        }
        let symbols = self.schematic_symbol_context();
        let Some(part) = self
            .schematic
            .document()
            .components
            .iter()
            .find(|part| part.id == component)
            .cloned()
        else {
            return Err(LocateSignalError::UnknownDevice(name));
        };
        let terminals = symbols.named_terminal_points(&part);
        let pin = cross_probe::probed_pin(signal)
            .and_then(|pin| cross_probe::drawn_pin(&terminals, pin))
            .cloned();
        let occurrence = self.workspace.content.occurrence_path();
        let sheet = self.active_sheet_id();
        let topology = self.schematic.topology_version();
        let key = self.workspace.content.active_schematic_reference().key();
        let (document, editor) = self.schematic.document_and_editor();
        let view = cross_probe::CrossProbeView {
            document,
            sheet_catalog: self.workspace.content.design_management.sheet_catalog(&key),
            occurrence: &occurrence,
            net_to_points: self
                .simulation
                .source
                .cross_probe
                .on_sheet_at(
                    &self.workspace.content.active_view,
                    &occurrence,
                    topology,
                    sheet,
                )
                .map(|geometry| &geometry.net_to_points),
        };
        cross_probe::select_signal_part(&view, editor, &part, pin.as_ref().map(|(_, at)| *at));
        Ok(match pin {
            Some((pin, _)) => SignalPlace::Pin { part: name, pin },
            None => SignalPlace::Part(name),
        })
    }

    /// The scope `signal` is read in when it is not the occurrence the tab
    /// in front is editing, as the engine spells it. `None` when the signal
    /// is read here, or names nothing the schematic draws.
    pub fn signal_read_elsewhere(&self, signal: &str) -> Option<crate::state::InstancePath> {
        let scope = cross_probe::signal_scope(signal)?;
        (!scope.same_engine_scope(&self.workspace.content.occurrence_path())).then_some(scope)
    }

    /// The drawn part a current trace is read through, on the open drawing
    /// at the occurrence in front: its id, where it sits and the name the
    /// drawing gives it. Read only through a map current for the drawing, by
    /// [`cross_probe::locate_signal_device`]'s rules.
    pub fn locate_signal_device(
        &self,
        signal: &str,
    ) -> Result<(u64, Point, String), LocateSignalError> {
        let occurrence = self.workspace.content.occurrence_path();
        let key = self.workspace.content.active_schematic_reference().key();
        let view = cross_probe::CrossProbeView {
            document: self.schematic.document(),
            sheet_catalog: self.workspace.content.design_management.sheet_catalog(&key),
            occurrence: &occurrence,
            net_to_points: self
                .active_sheet_net_geometry()
                .map(|geometry| &geometry.net_to_points),
        };
        cross_probe::locate_signal_device(&view, signal)
            .map(|component| (component.id, component.pos, component.name.clone()))
    }

    /// Whether the retained cross-probe map belongs to the open cell as it is
    /// drawn right now, at the occurrence the tab is editing: the design root,
    /// or a block descended into.
    pub fn result_mapping_is_current(&self) -> bool {
        self.simulation.source.cross_probe.is_current_at(
            &self.workspace.content.active_view,
            &self.workspace.content.occurrence_path(),
            self.schematic.topology_version(),
        )
    }

    /// The net geometry the active sheet draws, in its own coordinates, while
    /// the retained map is current for it.
    pub fn active_sheet_net_geometry(&self) -> Option<&SheetNetGeometry> {
        self.current_sheet_net_geometry(self.active_sheet_id())
    }

    /// The net geometry `sheet` of the open drawing draws, while the retained
    /// map is current for the drawing at the occurrence in front.
    fn current_sheet_net_geometry(&self, sheet: Option<SheetId>) -> Option<&SheetNetGeometry> {
        self.simulation.source.cross_probe.on_sheet_at(
            &self.workspace.content.active_view,
            &self.workspace.content.occurrence_path(),
            self.schematic.topology_version(),
            sheet,
        )
    }
}
