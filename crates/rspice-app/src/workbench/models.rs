//! The models feature: the Models & PDKs workspace, the device-model editor,
//! measurement correlation and unsigned PDK authoring.
//!
//! What the workspace keeps between frames is declared here, beneath the
//! session aggregate that holds the models slot and the wire that persists
//! it, so both name it without reaching up into the surfaces.

pub(crate) mod workspace_state;
