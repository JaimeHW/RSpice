//! The nets of the open cell view as the design resolves them, and showing
//! one of them on the drawing.

use std::sync::Arc;

use rspice_design::connectivity::summary::{DesignNet, projection_nets};

use super::Model;

impl Model {
    /// The nets of the open cell view as the configured design resolves
    /// them: the names generation writes, so a net named on a generated line
    /// is found among them. An unresolved configuration has none, rather
    /// than names the generator would never write.
    #[must_use]
    pub fn active_design_nets(&self) -> Arc<Vec<DesignNet>> {
        let view = &self.workspace.content.active_view;
        match self
            .workspace
            .design_projection(&self.library_manager, view, &self.schematic)
        {
            Ok(projection) => {
                projection_nets(self.library_manager.catalog(), &projection, &view.key())
            }
            Err(error) => {
                log::warn!("The open cell view has no design projection: {error}");
                Arc::new(Vec::new())
            }
        }
    }

    /// Select the net of this name on the drawing on screen, light it, and
    /// centre the view on it: its label, else its first wire, else the first
    /// instance on it. A net with no wire drawn is selected by the instances
    /// it joins. Whether the open cell view has a net of that name; when it
    /// has none, nothing changes.
    pub fn show_design_net(&mut self, name: &str) -> bool {
        let nets = self.active_design_nets();
        let Some(net) = nets.iter().find(|net| net.name.eq_ignore_ascii_case(name)) else {
            return false;
        };
        let document = self.schematic.document();
        let position = document
            .net_labels
            .iter()
            .find(|label| label.name.eq_ignore_ascii_case(&net.name))
            .map(|label| label.pos)
            .or_else(|| {
                document
                    .wires
                    .iter()
                    .find(|wire| net.wire_ids.contains(&wire.id))
                    .and_then(crate::state::Wire::start)
            })
            .or_else(|| {
                net.terminals.iter().find_map(|terminal| {
                    document
                        .components
                        .iter()
                        .find(|component| component.id == terminal.component_id)
                        .map(|component| component.pos)
                })
            });
        let editor = &mut self.schematic.session.editor;
        editor.selection.clear();
        for wire_id in &net.wire_ids {
            editor.selection.select_wire(*wire_id);
        }
        if net.wire_ids.is_empty() {
            for terminal in &net.terminals {
                editor.selection.select_component(terminal.component_id);
            }
        }
        editor
            .net_highlight
            .highlight_named_wires(&net.name, net.wire_ids.iter().copied().collect());
        editor.center_request = position;
        true
    }
}
