//! The selected-net highlight carried across hierarchy levels, as
//! Virtuoso's hierarchical net probe carries it.
//!
//! Under "Selected net across hierarchy" a highlight belongs to an
//! occurrence, the instance path on screen, not to a cell view: one master
//! placed twice has one editor session, and its two placements must not
//! share what it lights. Leaving an occurrence records its highlight under
//! its path. Entering one clears the session's highlight and lights what
//! the step carried in, or else what that path held: descending carries the
//! parent net into the child's port net the instance terminal binds it to,
//! and ascending carries a port net out to the parent net that terminal
//! reaches.

use std::collections::HashSet;
use std::sync::Arc;

use rspice_design::connectivity::summary::{DesignNet, projection_nets};

use crate::state::SchematicNetHighlighting;
use crate::workbench::app_state::AppState;

/// What a step between occurrences carries.
pub(crate) enum NetCarry {
    /// Into the child placed as `instance` in the active cell view.
    Descend(String),
    /// Up one level, out of the instance the active cell view is placed as.
    Ascend,
    /// Nothing: a jump of several levels, or a new root.
    Nothing,
}

impl AppState {
    fn carries_net_highlight(&self) -> bool {
        self.ui.schematic_visibility.net_highlighting
            == SchematicNetHighlighting::SelectedNetAcrossHierarchy
    }

    /// The key the active occurrence's highlight is held under.
    fn occurrence_highlight_key(&self) -> String {
        let content = &self.workspace.content;
        format!(
            "{}{}",
            content.active_occurrence_or_root().root.key(),
            content.occurrence_path()
        )
    }

    /// The nets of `cell_view_key` as the design projection resolves them.
    fn projected_nets(&self, cell_view_key: &str) -> Arc<Vec<DesignNet>> {
        match self.workspace.design_projection(
            &self.library_manager,
            &self.workspace.content.active_view,
            &self.schematic,
        ) {
            Ok(projection) => {
                projection_nets(self.library_manager.catalog(), &projection, cell_view_key)
            }
            Err(_) => Arc::new(Vec::new()),
        }
    }

    /// Light the active cell view's net named `name`. Returns whether one
    /// was found.
    pub(crate) fn light_net_named(&mut self, name: &str) -> bool {
        let nets = self.projected_nets(&self.workspace.content.active_view.key());
        let Some(net) = nets.iter().find(|net| net.name.eq_ignore_ascii_case(name)) else {
            return false;
        };
        let wires: HashSet<u64> = net.wire_ids.iter().copied().collect();
        self.schematic
            .session
            .editor
            .net_highlight
            .highlight_named_wires(net.name.clone(), wires);
        true
    }

    /// Before the active occurrence is left: record what it lights, and
    /// return the net the occurrence `carry` reaches should light.
    pub(crate) fn leave_occurrence_net_highlight(&mut self, carry: NetCarry) -> Option<String> {
        if !self.carries_net_highlight() {
            return None;
        }
        let key = self.occurrence_highlight_key();
        let highlight = &self.schematic.session.editor.net_highlight;
        let lit = highlight
            .active
            .then(|| highlight.selected_net_name.clone())
            .flatten();
        match &lit {
            Some(name) => self
                .workbench
                .occurrence_net_highlights
                .insert(key, name.clone()),
            None => self.workbench.occurrence_net_highlights.remove(&key),
        };
        let lit = lit?;
        match carry {
            NetCarry::Descend(instance) => self.child_port_net(&lit, &instance),
            NetCarry::Ascend => self.parent_net(&lit),
            NetCarry::Nothing => None,
        }
    }

    /// After an occurrence is entered: light what was carried in, or what
    /// the occurrence held when it was last left, and nothing else.
    pub(crate) fn enter_occurrence_net_highlight(&mut self, carried: Option<String>) {
        if !self.carries_net_highlight() {
            return;
        }
        self.schematic.session.editor.net_highlight.clear();
        let key = self.occurrence_highlight_key();
        let Some(name) =
            carried.or_else(|| self.workbench.occurrence_net_highlights.get(&key).cloned())
        else {
            return;
        };
        if self.light_net_named(&name) {
            self.workbench.occurrence_net_highlights.insert(key, name);
        }
    }

    /// The child's port net that `net` drives through `instance`'s terminal.
    fn child_port_net(&self, net: &str, instance: &str) -> Option<String> {
        let document = self.schematic.document();
        let component = document
            .components
            .iter()
            .find(|component| component.name.eq_ignore_ascii_case(instance))?;
        let nets = self.projected_nets(&self.workspace.content.active_view.key());
        nets.iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(net))?
            .terminals
            .iter()
            .find(|terminal| terminal.component_id == component.id)
            .map(|terminal| terminal.pin.clone())
    }

    /// The parent net the active cell view's port `net` reaches through the
    /// instance terminal the occurrence entered by.
    fn parent_net(&self, net: &str) -> Option<String> {
        let child = self.projected_nets(&self.workspace.content.active_view.key());
        let port = child
            .iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(net) && candidate.is_port())?;
        let occurrence = self.workspace.content.active_occurrence_or_root();
        let (last, above) = occurrence.steps.split_last()?;
        let parent = above.last().map_or(&occurrence.root, |step| &step.master);
        self.projected_nets(&parent.key())
            .iter()
            .find(|candidate| {
                candidate.terminals.iter().any(|terminal| {
                    terminal.reference.eq_ignore_ascii_case(&last.instance_name)
                        && terminal.pin.eq_ignore_ascii_case(&port.name)
                })
            })
            .map(|candidate| candidate.name.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::state::SchematicNetHighlighting;
    use crate::workbench::app_state::AppState;
    use crate::workbench::examples::hierarchy_reference::{self, AMP_INSTANCES};

    fn lit(state: &AppState) -> Option<String> {
        let highlight = &state.schematic.session.editor.net_highlight;
        highlight
            .active
            .then(|| highlight.selected_net_name.clone())
            .flatten()
    }

    /// Light `in` in `top`, descend into X1 and its `IN` port lights;
    /// ascend and `in` is lit again; descend into X2, whose `IN` is driven
    /// by `stage1`, and nothing is lit, though X1 and X2 share one session
    /// (audit U-12).
    #[test]
    fn the_highlight_follows_one_occurrence_and_never_its_twin() {
        let reference = hierarchy_reference::build();
        let amp = reference.amp_schematic.clone();
        let mut state = reference.state;
        state.open_workspace_view(reference.top.clone());
        state.ui.schematic_visibility.net_highlighting =
            SchematicNetHighlighting::SelectedNetAcrossHierarchy;
        assert!(state.light_net_named("in"), "top has a net named in");

        state.descend_into_instance(Some(AMP_INSTANCES[0].to_owned()), amp.clone());
        assert_eq!(lit(&state).as_deref(), Some("IN"), "X1's port is lit");
        assert!(
            !state
                .schematic
                .session
                .editor
                .net_highlight
                .highlighted_wires
                .is_empty(),
            "the port net's wires are lit"
        );

        state.ascend_workspace_level();
        assert_eq!(lit(&state).as_deref(), Some("in"), "back up, in is lit");

        state.descend_into_instance(Some(AMP_INSTANCES[1].to_owned()), amp);
        assert_eq!(
            lit(&state),
            None,
            "X2 shares X1's master, not its highlight"
        );
    }

    /// Under the plain policy a step carries nothing and resets nothing.
    #[test]
    fn the_plain_policy_carries_nothing() {
        let reference = hierarchy_reference::build();
        let amp = reference.amp_schematic.clone();
        let mut state = reference.state;
        state.open_workspace_view(reference.top.clone());
        assert!(state.light_net_named("in"));
        state.descend_into_instance(Some(AMP_INSTANCES[0].to_owned()), amp);
        assert_eq!(lit(&state), None);
    }
}
