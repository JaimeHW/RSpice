//! What the design feature's dialogs hand the shell.

use crate::workbench::app::dialogs::connectivity_manager::{
    RevealTarget, open_connectivity_source_map, reveal_connectivity_target,
};
use crate::workbench::app::dialogs::hierarchy::create::commit_create_hierarchy;
use crate::workbench::app::dialogs::hierarchy::descend::{
    DescendHierarchy, answer_descend_hierarchy,
};
use crate::workbench::app::{RSpiceApp, open_property_editor};
use crate::workbench::framework::requests::{Drain, Request};

/// What a design dialog hands the shell, which carries it out over the whole
/// session as the dialog once did in place. Each step that raises one raises
/// one, and has it taken straight after it draws, in the frame the reader
/// chose it.
#[derive(Debug, Clone)]
pub(crate) enum DesignRequest {
    /// A checked Descend into hierarchy choice.
    Descend(DescendHierarchy),
    /// Open the component editor again on one instance, whose card a
    /// re-adopted stimulus has just replaced.
    ReopenPropertyEditor(u64),
    /// Reveal one finding of the connectivity report on the canvas.
    RevealConnectivity(RevealTarget),
    /// Open the generated netlist on its source map.
    OpenSourceMap,
    /// Apply the connectivity repairs and policy the manager has under review.
    ApplyConnectivityRepairs,
    /// Extract the selection into the new cell Create hierarchy describes.
    CreateHierarchy,
}

impl Request for DesignRequest {
    const DRAIN: Drain = Drain::AfterContribution;
}

impl RSpiceApp {
    /// Carry out what a design dialog handed over, as the dialog once did in
    /// place.
    ///
    /// The shell answers these for now, because what they do still reaches
    /// the whole session, and applying repairs the simulation controller too.
    /// Design carries each out itself once it sits on its own parts.
    pub(crate) fn answer_design_request(&mut self, request: DesignRequest) {
        match request {
            DesignRequest::Descend(choice) => answer_descend_hierarchy(&mut self.state, choice),
            DesignRequest::ReopenPropertyEditor(component_id) => {
                open_property_editor(&mut self.state, component_id);
            }
            DesignRequest::RevealConnectivity(target) => {
                reveal_connectivity_target(&mut self.state, target);
            }
            DesignRequest::OpenSourceMap => open_connectivity_source_map(&mut self.state),
            DesignRequest::ApplyConnectivityRepairs => {
                if let Err(error) = self.apply_selected_connectivity_repairs() {
                    self.state.design.connectivity_manager.error = Some(error);
                }
            }
            DesignRequest::CreateHierarchy => {
                if let Err(error) = commit_create_hierarchy(&mut self.state) {
                    self.state.design.create_hierarchy.validation_error = Some(error);
                }
            }
        }
    }
}
