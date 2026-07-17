//! Canonical state and behavior for the Code & Automation workspace.
//!
//! The visual surface composes these contracts; it does not own source text,
//! provenance, validation identity, or document transitions. Keeping those
//! concerns here makes native and browser behavior identical and prevents an
//! asynchronous file or validation response from being applied to newer text.

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
pub use outline::{
    IncludeDirective, IncludeKind, NetlistOutline, OutlineEntry, OutlineEntryKind, OutlineSection,
    OutlineSectionKind,
};
pub use search::{
    FindDirection, FindError, FindMatch, FindOptions, ReplaceOutcome, ReplaceScope,
    find_all_in_source, find_in_source, replace_in_source,
};
