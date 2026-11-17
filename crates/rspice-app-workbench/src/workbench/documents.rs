//! The document engines the central surface hosts.
//!
//! Each owns one kind of authored or derived document — netlist source, a
//! result viewer, Verilog-A code, a device model, a model correlation study,
//! a visualization. They render inside a surface but do not own navigation,
//! chrome, or the route: a surface decides which of these is showing, and
//! these decide what it looks like.

/// A project-owned source-tree transaction requested by a document navigator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeSourceFileAction {
    New,
    Rename,
    Move,
    Duplicate,
    Delete,
}
