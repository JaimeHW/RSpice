//! The nets of the open cell view as the design resolves them: the one the
//! selection names, and showing one of them on the drawing.

use std::collections::HashSet;
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

    /// Keep the net highlight on its net after an edit: a wire drawn onto the
    /// lit net lights, and a cut leaves lit only the part that is still that
    /// net. Reads the nets only when the document changed since the light was
    /// last read.
    pub fn follow_net_highlight(&mut self) {
        let version = (
            self.schematic.topology_version(),
            self.schematic.content_version(),
        );
        if self
            .schematic
            .session
            .editor
            .net_highlight
            .is_followed_at(version)
        {
            return;
        }
        let nets = self.active_design_nets();
        self.schematic
            .session
            .editor
            .net_highlight
            .follow(&nets, version);
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

    /// The one net the selection on the drawing names: a net the navigator
    /// or the inspector selected whole, a port, or wires, junctions and net
    /// labels that all lie on one net. `None` for an instance, for nothing
    /// and for a selection spanning two nets.
    #[must_use]
    pub fn selected_design_net(&self) -> Option<String> {
        let editor = &self.schematic.session.editor;
        if editor.net_highlight.selected_net_name.is_some()
            && let Some(name) = self.selected_whole_net(&self.active_design_nets())
        {
            return Some(name);
        }
        if let Some(component_id) = editor.selection.single_component() {
            return self
                .schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == component_id)?
                .port_spec()
                .map(|port| port.name);
        }
        if !conductor_selection(&editor.selection) {
            return None;
        }
        self.selected_conductor_net(&self.active_design_nets())
    }

    /// The net a navigator or inspector row selected, while the selection
    /// and the light are still exactly what it set: its wires, or the
    /// instances on it when it has none, and nothing else.
    #[must_use]
    pub fn selected_whole_net(&self, nets: &[DesignNet]) -> Option<String> {
        let editor = &self.schematic.session.editor;
        let name = editor.net_highlight.selected_net_name.as_deref()?;
        let net = nets
            .iter()
            .find(|net| net.name.eq_ignore_ascii_case(name))?;
        if editor.net_highlight.highlighted_wires != net.wire_ids.iter().copied().collect() {
            return None;
        }
        let selection = &editor.selection;
        let no_other_classes = selection.wire_segments.is_empty()
            && selection.wire_vertices.is_empty()
            && selection.junctions.is_empty()
            && selection.buses.is_empty()
            && selection.bus_taps.is_empty()
            && selection.net_labels.is_empty()
            && selection.design_notes.is_empty()
            && selection.documentation_shapes.is_empty()
            && selection.probes.is_empty();
        let exact_wires = selection.wires == net.wire_ids.iter().copied().collect();
        let exact_components = if net.wire_ids.is_empty() {
            selection.components
                == net
                    .terminals
                    .iter()
                    .map(|terminal| terminal.component_id)
                    .collect()
        } else {
            selection.components.is_empty()
        };
        (no_other_classes && exact_wires && exact_components).then(|| net.name.clone())
    }

    /// The one net the selected wires, junctions and net labels lie on.
    fn selected_conductor_net(&self, nets: &[DesignNet]) -> Option<String> {
        let selection = &self.schematic.session.editor.selection;
        let document = self.schematic.document();
        let mut resolved: Option<String> = None;
        let mut accept = |candidate: &str| -> bool {
            match resolved.as_deref() {
                Some(existing) => existing.eq_ignore_ascii_case(candidate),
                None => {
                    resolved = Some(candidate.to_owned());
                    true
                }
            }
        };
        for label in &document.net_labels {
            if selection.net_labels.contains(&label.id) && !accept(&label.name) {
                return None;
            }
        }
        let mut wire_ids = selection.wires.iter().copied().collect::<HashSet<_>>();
        wire_ids.extend(
            selection
                .wire_segments
                .iter()
                .map(|segment| segment.wire_id),
        );
        wire_ids.extend(selection.wire_vertices.iter().map(|vertex| vertex.wire_id));
        for wire_id in wire_ids {
            let Some(net) = nets.iter().find(|net| net.wire_ids.contains(&wire_id)) else {
                continue;
            };
            if !accept(&net.name) {
                return None;
            }
        }
        for junction in &selection.junctions {
            let mut names = document
                .wires
                .iter()
                .filter(|wire| wire.contains_point(junction.pos))
                .filter_map(|wire| {
                    nets.iter()
                        .find(|net| net.wire_ids.contains(&wire.id))
                        .map(|net| net.name.as_str())
                });
            if let Some(name) = names.next()
                && (!accept(name) || names.any(|candidate| !accept(candidate)))
            {
                return None;
            }
        }
        resolved
    }
}

/// Whether the selection is conductors alone: wires, their segments and
/// vertices, junctions and net labels, with no instance or other object.
fn conductor_selection(selection: &crate::state::Selection) -> bool {
    let has_conductor = !selection.wires.is_empty()
        || !selection.wire_segments.is_empty()
        || !selection.wire_vertices.is_empty()
        || !selection.junctions.is_empty()
        || !selection.net_labels.is_empty();
    has_conductor
        && selection.components.is_empty()
        && selection.buses.is_empty()
        && selection.bus_taps.is_empty()
        && selection.design_notes.is_empty()
        && selection.documentation_shapes.is_empty()
        && selection.probes.is_empty()
}
