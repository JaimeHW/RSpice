//! What generation and validation established about the project's deck.
//!
//! The generated deck, the inputs it was generated from, and the receipt for
//! the latest validation of the visible source are facts about the design.
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
pub(crate) struct NetlistValidationReceipt {
    pub(crate) visible_content_digest: ContentDigest,
    pub(crate) executable_source_digest: ContentDigest,
    pub(crate) prepared_snapshot_digest: ContentDigest,
    pub(crate) project_revision: u64,
    pub(crate) task_count: usize,
    pub(crate) advisory_count: usize,
}

/// The generated deck, the generation inputs it is bound to, and the latest
/// validation of the visible source.
#[derive(Debug, Clone, Default)]
pub(crate) struct NetlistArtifacts {
    /// Canonical immutable document, including generated provenance,
    /// dependency identities, source map, outline, and validation state.
    pub(crate) generated_document: Option<crate::state::NetlistDocument>,
    /// Runtime-generated primary artifact retained independently from any
    /// project-owned source document.
    pub(crate) generated_source: String,
    /// Exact project-input digest used to produce the retained generated
    /// artifact. `None` means the current bytes have no generated authority.
    pub(crate) generated_input_digest: Option<ContentDigest>,
    /// Input digest observed this frame. A mismatch with
    /// `generated_input_digest` makes the retained artifact stale and blocks
    /// execution until generation succeeds.
    pub(crate) current_generation_input_digest: Option<ContentDigest>,
    /// Diagnostic from the latest failed generation attempt.
    pub(crate) generation_error: Option<String>,
    /// Receipt for the latest exact visible source validation.
    pub(crate) validation: Option<NetlistValidationReceipt>,
}
