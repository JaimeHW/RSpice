//! The project feature: the Project workspace, the project launcher and the
//! Library/Cellview Manager.
//!
//! What the workspace keeps between frames is declared here, beneath the
//! session aggregate that holds the project slot and the wire that persists
//! it, so both name it without reaching up into the surfaces.

pub(crate) mod configuration_state;
pub(crate) mod workspace_state;
