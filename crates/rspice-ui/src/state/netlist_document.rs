//! The project-owned netlist source document.
//!
//! `ProjectWorkspace` stores a `NetlistDocument` directly, so the document
//! model, its include-directive outline, and its find/replace live here rather
//! than beside the Code & Automation surface that edits them. None of this
//! module touches egui: it owns source text, provenance, validation identity,
//! and document transitions, which must behave identically on native and in
//! the browser.

mod document;
mod outline;
mod sealed;
mod search;

pub use document::{
    DependencyMetadata, DependencyResolution, DependencySourceAuthority, DiagnosticSeverity,
    DocumentOwnership, GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry,
    GenerationInput, NetlistDocument, NetlistDocumentId, SourceLocator, ValidationDiagnostic,
    content_digest,
};
pub use outline::{
    NetlistSourceIndex, OutlineEntry, OutlineEntryKind, OutlineSection, OutlineSectionKind,
};
pub(crate) use outline::{card_tokens, card_tokens_with_columns, parse_include_directives};
pub(crate) use sealed::expand_retained_netlist_dependencies;
pub(crate) use search::find_all_in_source_range_bounded_filter;
pub use search::{
    BoundedFindMatches, FindDirection, FindError, FindMatch, FindOptions,
    find_all_in_source_bounded, replace_source_ranges,
};
