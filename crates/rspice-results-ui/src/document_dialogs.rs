//! The small forms the result viewer opens over a project result document.
//!
//! Each form is a [`rspice_ui_kit::widgets::CommandForm`]: a few labelled
//! rows, Cancel, and one primary action. The draft a form edits lives here, in
//! the host's runtime state. The choices a form offers and the transaction it
//! commits stay with the host, which resolves both against the retained
//! datasets every frame.

pub mod add_pane;
pub mod compare;

/// The one result-document form open at a time.
#[derive(Debug, Clone, PartialEq)]
pub enum ResultDocumentDialog {
    AddPane(add_pane::AddPaneDraft),
    /// Compare datasets also opens over a quick view, where its outcome is
    /// shown and not kept.
    Compare(compare::CompareDraft),
}

/// Option labels for a select row, and the label of the selected entry.
fn labels_and_selection<K: PartialEq + Copy>(
    entries: &[(K, String)],
    selected: Option<K>,
) -> (Vec<String>, String) {
    let labels = entries.iter().map(|(_, label)| label.clone()).collect();
    let current = selected
        .and_then(|key| entries.iter().find(|(candidate, _)| *candidate == key))
        .map_or_else(String::new, |(_, label)| label.clone());
    (labels, current)
}
