//! The net highlight carried across a hierarchy descent, through the whole application:
//! the hierarchy reference example builds its design as an AppState.

use crate::state::SchematicNetHighlighting;
use crate::workbench::app_state::AppState;
use crate::workbench::examples::hierarchy_reference::{self, AMP_INSTANCES};

fn lit(state: &AppState) -> Option<String> {
    let highlight = &state.model.schematic.session.editor.net_highlight;
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
    state.design.schematic_visibility.net_highlighting =
        SchematicNetHighlighting::SelectedNetAcrossHierarchy;
    assert!(state.light_net_named("in"), "top has a net named in");

    state
        .navigation()
        .descend_into_instance(Some(AMP_INSTANCES[0].to_owned()), amp.clone());
    assert_eq!(lit(&state).as_deref(), Some("IN"), "X1's port is lit");
    assert!(
        !state
            .model
            .schematic
            .session
            .editor
            .net_highlight
            .highlighted_wires
            .is_empty(),
        "the port net's wires are lit"
    );

    state.navigation().ascend_workspace_level();
    assert_eq!(lit(&state).as_deref(), Some("in"), "back up, in is lit");

    state
        .navigation()
        .descend_into_instance(Some(AMP_INSTANCES[1].to_owned()), amp);
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
    state
        .navigation()
        .descend_into_instance(Some(AMP_INSTANCES[0].to_owned()), amp);
    assert_eq!(lit(&state), None);
}
