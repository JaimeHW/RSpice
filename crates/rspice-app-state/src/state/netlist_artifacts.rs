//! What generation and validation established about the project's deck.
//!
//! The generated deck, the inputs it was generated from, and the receipt or
//! failure of the latest validation of the visible source are facts about
//! the design.
//! The netlist editor shows them, and run readiness, export, cross-probe, the
//! document bar and the inspector read them, so they are kept apart from the
//! editor's own interaction state.
//!
//! Runtime state only: none of it is persisted. A project change resets it
//! together with the editor.

use crate::product::ContentDigest;

/// Runtime evidence that the exact visible deck passed the same preparation
/// contract used by execution. The receipt is invalid as soon as the visible
/// content digest changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistValidationReceipt {
    pub visible_content_digest: ContentDigest,
    pub executable_source_digest: ContentDigest,
    pub prepared_snapshot_digest: ContentDigest,
    pub project_revision: u64,
    pub task_count: usize,
    pub advisory_count: usize,
}

/// The generated deck, the generation inputs it is bound to, and the latest
/// validation of the visible source.
#[derive(Debug, Clone, Default)]
pub struct NetlistArtifacts {
    /// Canonical immutable document, including generated provenance,
    /// dependency identities, source map, outline, and validation state.
    pub generated_document: Option<crate::state::NetlistDocument>,
    /// Runtime-generated primary artifact retained independently from any
    /// project-owned source document.
    pub generated_source: String,
    /// Exact project-input digest used to produce the retained generated
    /// artifact. `None` means the current bytes have no generated authority.
    pub generated_input_digest: Option<ContentDigest>,
    /// Input digest observed this frame. A mismatch with
    /// `generated_input_digest` makes the retained artifact stale and blocks
    /// execution until generation succeeds.
    pub current_generation_input_digest: Option<ContentDigest>,
    /// Diagnostic from the latest failed generation attempt.
    pub generation_error: Option<String>,
    /// Receipt for the latest exact visible source validation.
    pub validation: Option<NetlistValidationReceipt>,
    /// Exact validation failure retained until the bytes change or validation
    /// succeeds.
    pub validation_error: Option<String>,
}
