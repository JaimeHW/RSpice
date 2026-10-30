//! The results feature observed through a whole application: the commands
//! that reach the Results workspace and its return edges, and the cases that
//! read or write another feature's state on the way.

mod availability;
mod commands;
mod document_edit;
pub(super) mod histogram_publication;
mod inspector;
mod navigation;
mod op_inspector;
mod persistent_document;
mod result_import_adapters;
mod retained_memo;
mod specs;
mod viewer_capabilities;
mod waveform_export;
