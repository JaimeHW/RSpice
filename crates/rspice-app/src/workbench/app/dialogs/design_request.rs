//! What the design feature's dialogs hand the shell.

use crate::workbench::app::dialogs::hierarchy::descend::{
    DescendHierarchy, answer_descend_hierarchy,
};
use crate::workbench::app::open_property_editor;
use crate::workbench::app_state::AppState;
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
}

impl Request for DesignRequest {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Carry out what a design dialog handed over, as the dialog once did in
/// place.
///
/// The shell answers these for now, because what they do still reaches the
/// whole session. Design carries each out itself once it sits on its own
/// parts.
pub(crate) fn answer_design_request(state: &mut AppState, request: DesignRequest) {
    match request {
        DesignRequest::Descend(choice) => answer_descend_hierarchy(state, choice),
        DesignRequest::ReopenPropertyEditor(component_id) => {
            open_property_editor(state, component_id);
        }
    }
}
