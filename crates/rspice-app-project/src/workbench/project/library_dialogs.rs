//! The Library/Cellview Manager's dialogs the project feature draws in its
//! own context: what each opens over the feature's state and the model, and
//! what each commits through the model's library catalog.

pub mod cell_ops;
pub mod deletion;
pub mod library_files;
pub mod library_ops;
pub mod new_cell;
pub mod new_view;
pub mod properties;
pub mod rename_view;
pub mod transfer;

pub(super) use new_view::new_view_target;

/// What a delete review says beside its buttons when Undo takes it back.
const UNDOABLE: &str = "This can be undone with Undo.";

/// What a refused delete review says beside its buttons, in words short
/// enough for the room a phone leaves there; its body states why, whole.
fn refused(object: &str) -> String {
    format!("The {object} cannot be deleted.")
}
