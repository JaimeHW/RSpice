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
mod search;

pub use document::{
    DependencyMetadata, DependencyResolution, DiagnosticSeverity, DocumentError, DocumentOwnership,
    GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry, GenerationInput,
    ImportedProvenance, NetlistDocument, NetlistDocumentId, SaveAcknowledgement, SourceLocator,
    SourcePosition, SourceProvenance, TransitionReceipt, ValidationDiagnostic, ValidationReport,
    content_digest,
};
pub(crate) use outline::parse_include_directives;
pub use outline::{
    IncludeDirective, IncludeKind, NetlistOutline, OutlineEntry, OutlineEntryKind, OutlineSection,
    OutlineSectionKind,
};
pub use search::{
    FindDirection, FindError, FindMatch, FindOptions, ReplaceOutcome, ReplaceScope,
    find_all_in_source, find_in_source, replace_in_source,
};
