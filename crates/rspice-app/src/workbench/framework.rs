//! What the parts of the workbench share to work with one another without
//! reaching into one another's state.

pub(crate) mod command_registry;
pub(crate) mod dialogs;
pub(crate) mod feature;
pub(crate) mod host;
pub(crate) mod requests;
pub(crate) mod workbench_host;
