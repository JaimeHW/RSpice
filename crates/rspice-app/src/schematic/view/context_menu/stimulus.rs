//! The stimulus-library verbs a placed source's menu offers.
//!
//! The menu is about one independent source when exactly one is selected, and
//! only then does this group exist: a verb that could never apply to the kind
//! of object under the pointer is left out, where a verb that cannot be taken
//! *right now* is drawn disabled and says why. The three rows that are about a
//! cell instance's master make room for it on the same terms — a primitive
//! source has no master to descend into, refresh or replace.
//!
//! Every verb goes through the entry point Component Properties uses for the
//! same verb, so the two surfaces cannot link one instance differently. The
//! editor states all four at once; the menu states the three that matter in the
//! instance's present standing, because it has to fit without scrolling:
//! re-adoption takes the place of adopting something else when the library has
//! a revision to offer, and switching definitions stays one step away in the
//! editor.

// The application's entry points come through the menu's own imports: this
// group reaches nothing the menu does not already reach.
use super::{
    AppState, ConsoleMessage, ContextAction, StimulusLinkMode, clicked_source, commit_readoption,
    open_stimulus_definition, open_stimulus_link,
};

/// Whether `action` is one of this group's.
pub(super) const fn owns(action: ContextAction) -> bool {
    matches!(
        action,
        ContextAction::AdoptStimulus
            | ContextAction::ReadoptStimulus
            | ContextAction::SaveStimulus
            | ContextAction::OpenStimulusDefinition
    )
}

/// The definition the selected source names, while the library still holds it.
fn held_definition(state: &AppState) -> Option<&str> {
    let name = clicked_source(state)?
        .stimulus_provenance
        .as_ref()?
        .definition
        .as_str();
    state
        .workspace
        .stimulus_library
        .get(name)
        .is_some()
        .then_some(name)
}

/// Whether the library has a revision to copy back onto the selected source.
fn offers_readoption(state: &AppState) -> bool {
    held_definition(state).is_some()
        && clicked_source(state).is_some_and(|source| {
            state
                .workspace
                .stimulus_library
                .provenance_state(source)
                .offers_readoption()
        })
}

/// Whether this verb is part of the menu for what is selected.
pub(super) fn shown(action: ContextAction, state: &AppState) -> bool {
    if clicked_source(state).is_none() {
        return false;
    }
    match action {
        ContextAction::AdoptStimulus => !offers_readoption(state),
        ContextAction::ReadoptStimulus => offers_readoption(state),
        ContextAction::OpenStimulusDefinition => held_definition(state).is_some(),
        _ => true,
    }
}

/// Whether this verb can be taken now, and what to say when it cannot.
pub(super) fn availability(action: ContextAction, state: &AppState) -> (bool, &'static str) {
    let writable = !state.schematic_edit_read_only();
    match action {
        ContextAction::AdoptStimulus if state.workspace.stimulus_library.is_empty() => (
            false,
            "This project holds no stimulus definitions yet; save this source as one first",
        ),
        ContextAction::OpenStimulusDefinition => (true, ""),
        _ => (
            writable,
            "The active schematic view is read-only; reopen it in an editable context to link \
             this source",
        ),
    }
}

/// Take the verb, through the entry point Component Properties uses for it.
pub(super) fn execute(action: ContextAction, state: &mut AppState) {
    let Some((component_id, definition)) = clicked_source(state).map(|source| {
        (
            source.id,
            source
                .stimulus_provenance
                .as_ref()
                .map(|provenance| provenance.definition.clone()),
        )
    }) else {
        return;
    };
    let outcome = match action {
        ContextAction::AdoptStimulus => {
            open_stimulus_link(state, component_id, StimulusLinkMode::Adopt).map(|()| None)
        }
        ContextAction::SaveStimulus => {
            open_stimulus_link(state, component_id, StimulusLinkMode::Extract).map(|()| None)
        }
        ContextAction::ReadoptStimulus => match definition {
            Some(definition) => commit_readoption(state, component_id, &definition).map(Some),
            None => Ok(None),
        },
        ContextAction::OpenStimulusDefinition => {
            if let Some(definition) = definition {
                open_stimulus_definition(state, &definition);
            }
            Ok(None)
        }
        _ => Ok(None),
    };
    match outcome {
        Ok(Some(line)) => state.push_user_message(ConsoleMessage::info(line)),
        Ok(None) => {}
        Err(refusal) => state.push_user_message(ConsoleMessage::warning(refusal)),
    }
}
