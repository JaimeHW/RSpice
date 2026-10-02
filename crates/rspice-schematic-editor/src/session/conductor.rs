//! Source ownership of an unfinished wire or bus route.

use super::{EditorSession, tool::Tool};
use crate::requests::EditorRequestSource;
use rspice_design_model::cell_view::CellViewRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConductorSource {
    pub document: EditorRequestSource,
    pub view: CellViewRef,
    /// Host window identity also distinguishes embedded windows in one viewport.
    pub window: u64,
    pub viewport: egui::ViewportId,
}

impl ConductorSource {
    fn matches(&self, current: &Self) -> bool {
        // The active-buffer epoch advances during passive window projection.
        // A route belongs to its retained per-document session and revision;
        // replacing that session drops its source along with the draft.
        self.document.project == current.document.project
            && self.document.document == current.document.document
            && self.document.occurrence == current.document.occurrence
            && self.document.design_epoch == current.document.design_epoch
            && self.document.content_version == current.document.content_version
            && self.document.topology_version == current.document.topology_version
            && self.document.symbol_revision == current.document.symbol_revision
            && self.document.sheet == current.document.sheet
            && self.view == current.view
            && self.window == current.window
            && self.viewport == current.viewport
    }
}

impl EditorSession {
    /// Bind a newly started conductor draft to the host's current source.
    pub fn bind_conductor_source(&mut self, source: ConductorSource) {
        self.conductor_source = Some(source);
    }

    /// Retire a stale draft before its triggering input can begin another edit.
    pub fn reconcile_conductor_source(
        &mut self,
        current: &ConductorSource,
        can_edit: bool,
    ) -> bool {
        let active = self.wire_drawing.active || self.bus_drawing.active;
        if !active {
            self.conductor_source = None;
            return false;
        }
        let tool_matches = match self.tool {
            Tool::Wire => self.wire_drawing.active && !self.bus_drawing.active,
            Tool::Bus => self.bus_drawing.active && !self.wire_drawing.active,
            _ => false,
        };
        let current = self
            .conductor_source
            .as_ref()
            .is_some_and(|source| source.matches(current));
        if !can_edit || !tool_matches || !current {
            self.cancel_routing_gestures();
            return true;
        }
        false
    }
}
