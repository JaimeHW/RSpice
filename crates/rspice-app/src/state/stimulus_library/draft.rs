//! Editing one definition, with the undo stack the workspace keys off.
//!
//! A definition is `saved · rN` until someone edits it, and then it is a draft:
//! a working record beside the saved one, plus the history each edit pushes.
//! Dirty is a comparison — the working record differing from the saved one
//! after normalization — so a field typed back to what it was stops being an
//! edit, and no surface has to remember to clear a flag.
//!
//! Apply is the only thing that publishes: it stamps `rN+1`, adopts the working
//! record as the new saved one, and clears the history, because an undo across
//! a publish would silently un-publish a revision adopters may already be
//! reading as `behind`. Revert is an ordinary edit and stays undoable.

use super::definition::StimulusDefinition;

/// One definition being edited, with its undo history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionDraft {
    saved: StimulusDefinition,
    working: StimulusDefinition,
    history: Vec<StimulusDefinition>,
    cursor: usize,
}

impl DefinitionDraft {
    /// Open a saved definition for editing.
    #[must_use]
    pub fn new(saved: StimulusDefinition) -> Self {
        Self {
            working: saved.clone(),
            history: vec![saved.clone()],
            saved,
            cursor: 0,
        }
    }

    /// The definition as the library holds it.
    #[must_use]
    pub const fn saved(&self) -> &StimulusDefinition {
        &self.saved
    }

    /// The definition as it is being edited.
    #[must_use]
    pub const fn working(&self) -> &StimulusDefinition {
        &self.working
    }

    /// Whether the working record says anything the saved one does not.
    ///
    /// Normalized on both sides, so re-ordering a parameter string or typing a
    /// value back to its original does not leave the draft dirty.
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.working.normalized() != self.saved.normalized()
    }

    /// Whether [`Self::undo`] would do anything.
    #[must_use]
    pub const fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    /// Whether [`Self::redo`] would do anything.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        self.cursor + 1 < self.history.len()
    }

    /// Apply one edit and push it onto the history.
    ///
    /// An edit made after an undo drops the redo tail, which is what every
    /// undo stack does and what keeps the history a line rather than a tree.
    /// An edit that changes nothing pushes nothing, so holding a key down in a
    /// field that is already at its value does not bury the previous state.
    pub fn edit(&mut self, edit: impl FnOnce(&mut StimulusDefinition)) {
        let mut next = self.working.clone();
        edit(&mut next);
        if next.normalized() == self.working.normalized() {
            self.working = next;
            return;
        }
        self.working = next;
        self.history.truncate(self.cursor + 1);
        self.history.push(self.working.clone());
        self.cursor = self.history.len() - 1;
    }

    /// Step one edit back.
    pub fn undo(&mut self) -> bool {
        if !self.can_undo() {
            return false;
        }
        self.cursor -= 1;
        self.working = self.history[self.cursor].clone();
        true
    }

    /// Step one edit forward.
    pub fn redo(&mut self) -> bool {
        if !self.can_redo() {
            return false;
        }
        self.cursor += 1;
        self.working = self.history[self.cursor].clone();
        true
    }

    /// Throw away the edits, as one undoable step.
    pub fn revert(&mut self) {
        let saved = self.saved.clone();
        self.edit(|working| *working = saved);
    }

    /// Publish the working record as the next revision.
    ///
    /// The history is cleared rather than extended: the published revision is
    /// what adopters compare against, and an undo that walked back past it
    /// would leave the library holding a revision number no record explains.
    pub fn apply(&mut self) -> StimulusDefinition {
        let mut published = self.working.clone();
        published.publish_next_revision();
        self.saved = published.clone();
        self.working = published.clone();
        self.history = vec![published.clone()];
        self.cursor = 0;
        published
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ComponentType;

    fn draft() -> DefinitionDraft {
        let mut definition =
            StimulusDefinition::new("sensor_diff_1k", ComponentType::VoltageSourceSin)
                .expect("definition");
        definition.value = "0".to_owned();
        definition.params = "va=3m freq=1k".to_owned();
        DefinitionDraft::new(definition)
    }

    #[test]
    fn a_freshly_opened_draft_is_clean_and_has_nothing_to_undo() {
        let draft = draft();
        assert!(!draft.is_dirty());
        assert!(!draft.can_undo());
        assert!(!draft.can_redo());
    }

    #[test]
    fn undo_and_redo_walk_the_edits_one_at_a_time() {
        let mut draft = draft();
        draft.edit(|working| working.params = "va=6m freq=1k".to_owned());
        draft.edit(|working| working.params = "va=9m freq=1k".to_owned());
        assert!(draft.is_dirty());

        assert!(draft.undo());
        assert_eq!(draft.working().params, "va=6m freq=1k");
        assert!(draft.undo());
        assert_eq!(draft.working().params, "va=3m freq=1k");
        assert!(!draft.is_dirty());
        assert!(!draft.undo());

        assert!(draft.redo());
        assert_eq!(draft.working().params, "va=6m freq=1k");
        assert!(draft.redo());
        assert!(!draft.redo());
        assert_eq!(draft.working().params, "va=9m freq=1k");
    }

    #[test]
    fn an_edit_after_an_undo_drops_the_redo_tail() {
        let mut draft = draft();
        draft.edit(|working| working.params = "va=6m freq=1k".to_owned());
        draft.edit(|working| working.params = "va=9m freq=1k".to_owned());
        draft.undo();
        draft.edit(|working| working.params = "va=12m freq=1k".to_owned());

        assert!(!draft.can_redo());
        assert!(draft.undo());
        assert_eq!(draft.working().params, "va=6m freq=1k");
    }

    #[test]
    fn an_edit_that_changes_nothing_pushes_no_history() {
        let mut draft = draft();
        draft.edit(|working| working.params = "freq=1k va=3m".to_owned());
        assert!(!draft.can_undo());
        assert!(!draft.is_dirty());
    }

    #[test]
    fn revert_is_one_undoable_step() {
        let mut draft = draft();
        draft.edit(|working| working.params = "va=6m freq=1k".to_owned());
        draft.revert();

        assert!(!draft.is_dirty());
        assert!(draft.undo());
        assert_eq!(draft.working().params, "va=6m freq=1k");
    }

    #[test]
    fn apply_publishes_the_next_revision_and_clears_the_history() {
        let mut draft = draft();
        draft.edit(|working| working.params = "va=6m freq=1k".to_owned());
        let published = draft.apply();

        assert_eq!(published.revision(), 2);
        assert_eq!(published.params, "va=6m freq=1k");
        assert!(!draft.is_dirty());
        assert!(!draft.can_undo());
        assert!(!draft.can_redo());
        assert_eq!(draft.saved().revision(), 2);
    }

    #[test]
    fn a_family_switch_is_an_edit_and_undo_restores_the_shape_parameters() {
        let mut draft = draft();
        draft.edit(|working| {
            *working = working.with_family(super::super::definition::StimulusFamily::Pulse);
        });
        assert_eq!(
            draft.working().component_type(),
            ComponentType::VoltageSourcePulse
        );
        assert_eq!(draft.working().params, "");

        assert!(draft.undo());
        assert_eq!(
            draft.working().component_type(),
            ComponentType::VoltageSourceSin
        );
        assert_eq!(draft.working().params, "va=3m freq=1k");
    }
}
