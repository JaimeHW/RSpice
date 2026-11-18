//! What a session keeps of an editor's view of its document: its carets
//! and selections, what is folded, how far it is scrolled and what its find
//! bar searches for. A view taken from one launch is handed back to the
//! editor on the next, which shows the document as it was left.

use egui::Vec2;
use serde::{Deserialize, Serialize};

use super::super::text_document_model::TextSelection;
use super::VirtualEditorSession;
use super::find_bar::FindView;

/// An editor's view of its document, as a session keeps it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EditorView {
    /// Each selection as its anchor and its caret, in characters; the last
    /// is the primary one.
    selections: Vec<(usize, usize)>,
    /// Each folded stretch, as its first line and the line after it.
    folds: Vec<(usize, usize)>,
    /// How far the text is scrolled, across and down, in points.
    scroll: [f32; 2],
    find: FindView,
}

impl EditorView {
    /// The view `session` shows, scrolled by `scroll`.
    pub(super) fn of(session: &VirtualEditorSession, scroll: Vec2) -> Self {
        Self {
            selections: session
                .model
                .selections()
                .iter()
                .map(|selection| (selection.anchor, selection.cursor))
                .collect(),
            folds: session
                .model
                .folds()
                .iter()
                .map(|fold| (fold.start_line, fold.end_line))
                .collect(),
            scroll: [scroll.x, scroll.y],
            find: session.find.view(),
        }
    }

    /// Show this view in `session`, whose text is already the document's.
    /// What no longer fits the text, a caret past its end or a fold past its
    /// last line, is brought inside it or left out.
    pub(super) fn apply(self, session: &mut VirtualEditorSession, editable: bool) {
        session.model.unfold_all();
        for (start, end) in self.folds {
            session.model.fold_lines(start, end);
        }
        if !self.selections.is_empty() {
            session.model.set_selections(
                self.selections
                    .into_iter()
                    .map(|(anchor, cursor)| TextSelection { anchor, cursor })
                    .collect(),
            );
        }
        session.find.restore(self.find, editable);
        session.restore_scroll = Some(Vec2::new(self.scroll[0], self.scroll[1]));
    }

    /// The primary caret, in characters.
    #[must_use]
    pub fn caret(&self) -> Option<usize> {
        self.selections.last().map(|(_, cursor)| *cursor)
    }

    /// How far the text is scrolled down, in points.
    #[must_use]
    pub fn scrolled_down(&self) -> f32 {
        self.scroll[1]
    }

    /// How many stretches are folded.
    #[must_use]
    pub fn folded(&self) -> usize {
        self.folds.len()
    }

    /// What the find bar searches for, while it is open.
    #[must_use]
    pub fn searching_for(&self) -> Option<&str> {
        self.find.open.then_some(self.find.query.as_str())
    }
}

fn queued_view_id(editor_id: egui::Id) -> egui::Id {
    editor_id.with("restored-view")
}

/// Show `view` in the editor `editor_id` the next time it is drawn.
pub(crate) fn queue_view(context: &egui::Context, editor_id: egui::Id, view: EditorView) {
    context.data_mut(|data| data.insert_temp(queued_view_id(editor_id), view));
}

/// The view the editor `editor_id` was asked to show, taken once.
pub(super) fn take_queued_view(context: &egui::Context, editor_id: egui::Id) -> Option<EditorView> {
    context.data_mut(|data| data.remove_temp::<EditorView>(queued_view_id(editor_id)))
}
