//! The document engines the central surface hosts.
//!
//! Each owns one kind of authored or derived document — netlist source, a
//! result viewer, Verilog-A and automation code, a device model, a model
//! correlation study, a visualization. They render inside a surface but do
//! not own navigation, chrome, or the route: a surface decides which of these
//! is showing, and these decide what it looks like.

/// The document the Code & Automation workspace currently presents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeWorkspacePage {
    #[default]
    Netlist,
    VerilogA,
    Automation,
}

impl CodeWorkspacePage {
    pub const ALL: [Self; 3] = [Self::Netlist, Self::VerilogA, Self::Automation];
}

/// A project-owned source-tree transaction requested by a document navigator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeSourceFileAction {
    New,
    Rename,
    Move,
    Duplicate,
    Delete,
}

pub(crate) mod code_workspace;
pub(crate) mod model_correlation;
pub(crate) mod model_editor;
pub(crate) mod netlist_document;
pub(crate) mod result_document;
pub(crate) mod text_document_model;
pub(crate) mod text_editor_commands;
pub(crate) mod virtual_text_editor;
pub(crate) mod visualization_studio;
