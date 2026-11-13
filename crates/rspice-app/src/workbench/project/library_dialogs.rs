//! The Library/Cellview Manager's dialogs the project feature draws in its
//! own context: what each opens over the feature's state and the model, and
//! what each commits through the model's library catalog.

pub(crate) mod cell_ops;
pub(crate) mod deletion;
pub(crate) mod library_ops;
pub(crate) mod new_cell;
pub(crate) mod new_view;
pub(crate) mod rename_view;

pub(super) use new_view::new_view_target;
