//! The results feature observed through a whole application: the commands
//! that reach the Results workspace and its return edges, and the cases that
//! read or write another feature's state on the way.

mod commands;
mod inspector;
mod navigation;
mod persistent_document;
mod viewer_capabilities;
