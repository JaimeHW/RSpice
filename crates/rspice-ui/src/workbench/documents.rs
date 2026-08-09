//! The document engines the central surface hosts.
//!
//! Each owns one kind of authored or derived document — netlist source, a
//! result viewer, Verilog-A and automation code, a device model, a model
//! correlation study, a visualization. They render inside a surface but do
//! not own navigation, chrome, or the route: a surface decides which of these
//! is showing, and these decide what it looks like.
//!
//! `visualization_family` is the taxonomy the studio and the result viewers
//! agree on, which is why it sits with them rather than with the design
//! system.

pub(crate) mod canonical_diagnostics;
pub(crate) mod code_workspace;
pub(crate) mod model_correlation;
pub(crate) mod model_editor;
pub(crate) mod netlist_document;
pub(crate) mod result_document;
pub(crate) mod text_document_model;
pub(crate) mod text_editor_commands;
pub(crate) mod virtual_text_editor;
pub(crate) mod visualization_family;
pub(crate) mod visualization_studio;
