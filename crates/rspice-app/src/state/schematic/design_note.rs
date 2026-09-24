//! Durable, non-electrical schematic documentation objects.
//!
//! Design notes deliberately live outside [`super::Component`] and the
//! conductor collections.  They therefore participate in document editing
//! and export without ever changing connectivity or generated SPICE.

use serde::{Deserialize, Serialize};

use super::Point;
use super::SchematicState;

/// Maximum authored note source retained in a schematic document.
pub const MAX_DESIGN_NOTE_TEXT_LEN: usize = 4096;
pub const MAX_DESIGN_REVIEW_IDENTITY_LEN: usize = 256;
pub const MAX_DESIGN_REVIEW_MESSAGES: usize = 1024;
pub const MAX_DESIGN_REVIEW_EVIDENCE: usize = 256;
pub const MAX_DESIGN_REVIEW_MESSAGE_LEN: usize = 4096;
pub const MAX_DESIGN_REVIEW_EVIDENCE_LABEL_LEN: usize = 256;
pub const MAX_DESIGN_REVIEW_EVIDENCE_IDENTITY_LEN: usize = 1024;

/// The four documentation semantics exposed by the mockup-owned placement
/// transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignNoteKind {
    #[default]
    PlainText,
    PropertyDisplay,
    RequirementLink,
    ReviewNote,
}

impl DesignNoteKind {
    pub const ALL: [Self; 4] = [
        Self::PlainText,
        Self::PropertyDisplay,
        Self::RequirementLink,
        Self::ReviewNote,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PlainText => "Plain text",
            Self::PropertyDisplay => "Property display",
            Self::RequirementLink => "Requirement link",
            Self::ReviewNote => "Review note",
        }
    }
}

/// Non-electrical drawing layer used by design notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignNoteLayer {
    #[default]
    DrawingAnnotation,
}

impl DesignNoteLayer {
    pub const fn label(self) -> &'static str {
        match self {
            Self::DrawingAnnotation => "drawing / annotation",
        }
    }
}

/// Governed review lifecycle retained with review-note objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignReviewState {
    Open,
    Resolved,
}

impl DesignReviewState {
    pub const ALL: [Self; 2] = [Self::Open, Self::Resolved];

    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Resolved => "resolved",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::Resolved => "Resolved",
        }
    }
}

/// Activation target carried by a requirement-link note. Web links are
/// deliberately restricted to HTTP(S); every other accepted value is an
/// internal project specification reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementTarget<'a> {
    ExternalUri(&'a str),
    ProjectSpecification(&'a str),
}

/// Stable review metadata. A new review note always starts open; resolving it
/// is an explicit document edit and never inferred from text changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignReviewMessage {
    pub message_id: String,
    pub author: String,
    pub body: String,
    pub created_unix_ms: u64,
}

impl DesignReviewMessage {
    pub fn new(
        message_id: impl Into<String>,
        author: impl Into<String>,
        body: impl Into<String>,
        created_unix_ms: u64,
    ) -> Result<Self, DesignNoteError> {
        let message = Self {
            message_id: canonical_review_identity(message_id.into())?,
            author: canonical_review_identity(author.into())?,
            body: canonical_review_message(body.into())?,
            created_unix_ms,
        };
        message.validate()?;
        Ok(message)
    }

    fn validate(&self) -> Result<(), DesignNoteError> {
        validate_review_identity(&self.message_id)?;
        validate_review_identity(&self.author)?;
        let canonical_body = canonical_review_message(self.body.clone())?;
        if canonical_body != self.body {
            return Err(DesignNoteError::NonCanonicalReviewField);
        }
        Ok(())
    }
}

/// Immutable evidence reference attached to a design-review thread. Evidence
/// bytes remain owned by their source result or project artifact; the review
/// record retains only the stable identity and optional content digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignReviewEvidence {
    pub evidence_id: String,
    pub label: String,
    pub source_identity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_digest: Option<String>,
}

impl DesignReviewEvidence {
    pub fn new(
        evidence_id: impl Into<String>,
        label: impl Into<String>,
        source_identity: impl Into<String>,
        content_digest: Option<String>,
    ) -> Result<Self, DesignNoteError> {
        let evidence = Self {
            evidence_id: canonical_review_identity(evidence_id.into())?,
            label: canonical_review_evidence_label(label.into())?,
            source_identity: canonical_review_evidence_identity(source_identity.into())?,
            content_digest: content_digest.map(canonical_review_identity).transpose()?,
        };
        evidence.validate()?;
        Ok(evidence)
    }

    fn validate(&self) -> Result<(), DesignNoteError> {
        validate_review_identity(&self.evidence_id)?;
        if canonical_review_evidence_label(self.label.clone())? != self.label
            || canonical_review_evidence_identity(self.source_identity.clone())?
                != self.source_identity
            || self.content_digest.as_ref().is_some_and(|digest| {
                canonical_review_identity(digest.clone()).map_or(true, |value| value != *digest)
            })
        {
            return Err(DesignNoteError::NonCanonicalReviewField);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignReviewRecord {
    pub record_id: String,
    pub state: DesignReviewState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchored_revision: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<DesignReviewMessage>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<DesignReviewEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_note: Option<String>,
}

impl DesignReviewRecord {
    fn new(record_id: String) -> Self {
        Self {
            record_id,
            state: DesignReviewState::Open,
            assignee: None,
            anchored_revision: None,
            messages: Vec::new(),
            evidence: Vec::new(),
            resolution_note: None,
        }
    }

    fn validate(&self) -> Result<(), DesignNoteError> {
        validate_review_identity(&self.record_id)?;
        if self.assignee.as_ref().is_some_and(|value| {
            canonical_review_identity(value.clone()).map_or(true, |canonical| canonical != *value)
        }) || self.anchored_revision.as_ref().is_some_and(|value| {
            canonical_review_identity(value.clone()).map_or(true, |canonical| canonical != *value)
        }) || self.resolution_note.as_ref().is_some_and(|value| {
            canonical_review_message(value.clone()).map_or(true, |canonical| canonical != *value)
        }) {
            return Err(DesignNoteError::NonCanonicalReviewField);
        }
        if self.messages.len() > MAX_DESIGN_REVIEW_MESSAGES {
            return Err(DesignNoteError::TooManyReviewMessages);
        }
        if self.evidence.len() > MAX_DESIGN_REVIEW_EVIDENCE {
            return Err(DesignNoteError::TooMuchReviewEvidence);
        }
        let mut message_ids = std::collections::HashSet::with_capacity(self.messages.len());
        for message in &self.messages {
            message.validate()?;
            if !message_ids.insert(message.message_id.as_str()) {
                return Err(DesignNoteError::DuplicateReviewIdentity);
            }
        }
        let mut evidence_ids = std::collections::HashSet::with_capacity(self.evidence.len());
        for evidence in &self.evidence {
            evidence.validate()?;
            if !evidence_ids.insert(evidence.evidence_id.as_str()) {
                return Err(DesignNoteError::DuplicateReviewIdentity);
            }
        }
        if self.state == DesignReviewState::Open && self.resolution_note.is_some() {
            return Err(DesignNoteError::InvalidReviewResolution);
        }
        Ok(())
    }
}

/// Complete, typed update applied to one review note as one schematic undo
/// transaction. Every payload is validated before the live document changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesignReviewMutation {
    Assign {
        assignee: Option<String>,
    },
    AnchorRevision {
        revision: Option<String>,
    },
    Reply {
        author: String,
        body: String,
        created_unix_ms: u64,
    },
    AttachEvidence {
        label: String,
        source_identity: String,
        content_digest: Option<String>,
    },
    Resolve {
        author: String,
        note: String,
        created_unix_ms: u64,
    },
    Reopen,
}

impl DesignReviewMutation {
    fn apply(self, note: &mut DesignNote) -> Result<(), DesignNoteError> {
        match self {
            Self::Assign { assignee } => note.assign_review(assignee),
            Self::AnchorRevision { revision } => note.anchor_review_to_revision(revision),
            Self::Reply {
                author,
                body,
                created_unix_ms,
            } => note
                .append_review_message(author, body, created_unix_ms)
                .map(|_| ()),
            Self::AttachEvidence {
                label,
                source_identity,
                content_digest,
            } => note
                .attach_review_evidence(label, source_identity, content_digest)
                .map(|_| ()),
            Self::Resolve {
                author,
                note: resolution_note,
                created_unix_ms,
            } => note.resolve_review(author, resolution_note, created_unix_ms),
            Self::Reopen => note.set_review_state(DesignReviewState::Open),
        }
    }

    fn undo_label(&self) -> &'static str {
        match self {
            Self::Assign { .. } => "assign design review",
            Self::AnchorRevision { .. } => "anchor design review revision",
            Self::Reply { .. } => "reply to design review",
            Self::AttachEvidence { .. } => "attach design review evidence",
            Self::Resolve { .. } => "resolve design review",
            Self::Reopen => "reopen design review",
        }
    }
}

/// A durable authored documentation object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignNote {
    pub id: u64,
    pub pos: Point,
    pub kind: DesignNoteKind,
    pub text: String,
    pub layer: DesignNoteLayer,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<DesignReviewRecord>,
}

/// Frozen, validated one-shot placement payload. The UI also binds the
/// application epochs and active cell/view so a delayed click cannot mutate a
/// different document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingDesignNotePlacement {
    pub kind: DesignNoteKind,
    pub text: String,
    pub layer: DesignNoteLayer,
    pub topology_version: u64,
    /// Exact non-electrical document content frozen when the tool is armed.
    /// Topology versioning deliberately ignores notes, so this closes the
    /// concurrent note-edit gap without treating documentation as electrical.
    pub expected_design_notes: Vec<DesignNote>,
    pub document_authority: Option<DesignNotePlacementAuthority>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignNotePlacementAuthority {
    pub design_execution_epoch: u64,
    pub active_schematic_epoch: u64,
    pub view_path: String,
}

impl PendingDesignNotePlacement {
    pub fn new(
        kind: DesignNoteKind,
        text: impl Into<String>,
        topology_version: u64,
        expected_design_notes: &[DesignNote],
    ) -> Result<Self, DesignNoteError> {
        let text = normalize_design_note_text(&text.into())?;
        validate_kind_text(kind, &text)?;
        Ok(Self {
            kind,
            text,
            layer: DesignNoteLayer::DrawingAnnotation,
            topology_version,
            expected_design_notes: expected_design_notes.to_vec(),
            document_authority: None,
        })
    }

    pub fn with_document_authority(
        mut self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: String,
    ) -> Self {
        self.document_authority = Some(DesignNotePlacementAuthority {
            design_execution_epoch,
            active_schematic_epoch,
            view_path,
        });
        self
    }
}

impl DesignNote {
    pub fn new(
        id: u64,
        pos: Point,
        kind: DesignNoteKind,
        text: impl Into<String>,
    ) -> Result<Self, DesignNoteError> {
        let text = normalize_design_note_text(&text.into())?;
        validate_kind_text(kind, &text)?;
        let review = (kind == DesignNoteKind::ReviewNote)
            .then(|| DesignReviewRecord::new(format!("NOTE-{id:04}")));
        Ok(Self {
            id,
            pos,
            kind,
            text,
            layer: DesignNoteLayer::DrawingAnnotation,
            review,
        })
    }

    pub fn validate(&self) -> Result<(), DesignNoteError> {
        let normalized = normalize_design_note_text(&self.text)?;
        if normalized != self.text {
            return Err(DesignNoteError::NonCanonicalText);
        }
        validate_kind_text(self.kind, &self.text)?;
        match (self.kind, &self.review) {
            (DesignNoteKind::ReviewNote, Some(review))
                if review.record_id == format!("NOTE-{:04}", self.id) =>
            {
                review.validate()
            }
            (DesignNoteKind::ReviewNote, _) => Err(DesignNoteError::InvalidReviewRecord),
            (_, None) => Ok(()),
            (_, Some(_)) => Err(DesignNoteError::UnexpectedReviewRecord),
        }
    }

    /// Apply a validated property edit while preserving stable identity and
    /// review lifecycle state.
    pub fn update(
        &mut self,
        kind: DesignNoteKind,
        text: impl Into<String>,
    ) -> Result<(), DesignNoteError> {
        let text = normalize_design_note_text(&text.into())?;
        validate_kind_text(kind, &text)?;
        self.kind = kind;
        self.text = text;
        self.review = if kind == DesignNoteKind::ReviewNote {
            self.review
                .take()
                .or_else(|| Some(DesignReviewRecord::new(format!("NOTE-{:04}", self.id))))
        } else {
            None
        };
        self.validate()
    }

    pub fn translate(&mut self, delta: Point) {
        self.pos.x = self.pos.x.saturating_add(delta.x);
        self.pos.y = self.pos.y.saturating_add(delta.y);
    }

    pub fn set_review_state(&mut self, state: DesignReviewState) -> Result<(), DesignNoteError> {
        let Some(review) = self.review.as_mut() else {
            return Err(DesignNoteError::InvalidReviewRecord);
        };
        review.state = state;
        if state == DesignReviewState::Open {
            review.resolution_note = None;
        }
        self.validate()
    }

    pub fn assign_review(
        &mut self,
        assignee: Option<impl Into<String>>,
    ) -> Result<(), DesignNoteError> {
        let Some(review) = self.review.as_mut() else {
            return Err(DesignNoteError::InvalidReviewRecord);
        };
        review.assignee = assignee
            .map(Into::into)
            .map(canonical_review_identity)
            .transpose()?;
        self.validate()
    }

    pub fn anchor_review_to_revision(
        &mut self,
        revision: Option<impl Into<String>>,
    ) -> Result<(), DesignNoteError> {
        let Some(review) = self.review.as_mut() else {
            return Err(DesignNoteError::InvalidReviewRecord);
        };
        review.anchored_revision = revision
            .map(Into::into)
            .map(canonical_review_identity)
            .transpose()?;
        self.validate()
    }

    pub fn append_review_message(
        &mut self,
        author: impl Into<String>,
        body: impl Into<String>,
        created_unix_ms: u64,
    ) -> Result<String, DesignNoteError> {
        let Some(review) = self.review.as_mut() else {
            return Err(DesignNoteError::InvalidReviewRecord);
        };
        if review.messages.len() >= MAX_DESIGN_REVIEW_MESSAGES {
            return Err(DesignNoteError::TooManyReviewMessages);
        }
        let ordinal = review
            .messages
            .len()
            .checked_add(1)
            .ok_or(DesignNoteError::TooManyReviewMessages)?;
        let message_id = format!("{}-MSG-{ordinal:04}", review.record_id);
        review.messages.push(DesignReviewMessage::new(
            message_id.clone(),
            author,
            body,
            created_unix_ms,
        )?);
        self.validate()?;
        Ok(message_id)
    }

    pub fn attach_review_evidence(
        &mut self,
        label: impl Into<String>,
        source_identity: impl Into<String>,
        content_digest: Option<String>,
    ) -> Result<String, DesignNoteError> {
        let Some(review) = self.review.as_mut() else {
            return Err(DesignNoteError::InvalidReviewRecord);
        };
        if review.evidence.len() >= MAX_DESIGN_REVIEW_EVIDENCE {
            return Err(DesignNoteError::TooMuchReviewEvidence);
        }
        let ordinal = review
            .evidence
            .len()
            .checked_add(1)
            .ok_or(DesignNoteError::TooMuchReviewEvidence)?;
        let evidence_id = format!("{}-EVID-{ordinal:04}", review.record_id);
        review.evidence.push(DesignReviewEvidence::new(
            evidence_id.clone(),
            label,
            source_identity,
            content_digest,
        )?);
        self.validate()?;
        Ok(evidence_id)
    }

    pub fn resolve_review(
        &mut self,
        author: impl Into<String>,
        resolution_note: impl Into<String>,
        created_unix_ms: u64,
    ) -> Result<(), DesignNoteError> {
        let author = author.into();
        let resolution_note = canonical_review_message(resolution_note.into())?;
        self.append_review_message(author, resolution_note.clone(), created_unix_ms)?;
        let Some(review) = self.review.as_mut() else {
            return Err(DesignNoteError::InvalidReviewRecord);
        };
        review.state = DesignReviewState::Resolved;
        review.resolution_note = Some(resolution_note);
        self.validate()
    }

    pub fn requirement_target(&self) -> Option<RequirementTarget<'_>> {
        (self.kind == DesignNoteKind::RequirementLink)
            .then(|| requirement_target(&self.text))
            .flatten()
    }
}

/// Context used to resolve safe property-display tokens. Only documented,
/// read-only schematic properties are accepted; arbitrary expressions are
/// never evaluated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignNoteRenderContext<'a> {
    pub view_path: &'a str,
    pub component_count: usize,
    pub conductor_count: usize,
}

impl<'a> DesignNoteRenderContext<'a> {
    pub fn for_schematic(view_path: &'a str, schematic: &SchematicState) -> Self {
        Self {
            view_path,
            component_count: schematic.components.len(),
            conductor_count: schematic.wires.len() + schematic.buses.len(),
        }
    }
}

impl DesignNote {
    pub fn rendered_text(&self, context: &DesignNoteRenderContext<'_>) -> String {
        match self.kind {
            DesignNoteKind::PlainText | DesignNoteKind::ReviewNote => self.text.clone(),
            DesignNoteKind::RequirementLink => format!("REQ · {}", self.text),
            DesignNoteKind::PropertyDisplay => render_property_template(&self.text, context),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesignNoteError {
    EmptyText,
    TextTooLong,
    InvalidControlCharacter,
    NonCanonicalText,
    UnknownProperty(String),
    UnterminatedProperty,
    InvalidRequirementReference,
    InvalidReviewRecord,
    UnexpectedReviewRecord,
    EmptyReviewField,
    ReviewIdentityTooLong,
    ReviewMessageTooLong,
    ReviewEvidenceLabelTooLong,
    ReviewEvidenceIdentityTooLong,
    NonCanonicalReviewField,
    DuplicateReviewIdentity,
    TooManyReviewMessages,
    TooMuchReviewEvidence,
    InvalidReviewResolution,
    ReviewRecordNotFound,
    InvalidLayer,
    ReadOnly,
    StaleDocument,
}

impl std::fmt::Display for DesignNoteError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyText => formatter.write_str("Text is required."),
            Self::TextTooLong => write!(
                formatter,
                "Text must be {MAX_DESIGN_NOTE_TEXT_LEN} characters or fewer."
            ),
            Self::InvalidControlCharacter => {
                formatter.write_str("Text contains an unsupported control character.")
            }
            Self::NonCanonicalText => {
                formatter.write_str("Text has uncommitted leading or trailing whitespace.")
            }
            Self::UnknownProperty(name) => write!(
                formatter,
                "Unknown property \"{name}\". Supported properties are view, component_count, and conductor_count."
            ),
            Self::UnterminatedProperty => {
                formatter.write_str("Property displays must close every ${property} token.")
            }
            Self::InvalidRequirementReference => formatter.write_str(
                "Requirement links must be a stable identifier or URI without whitespace.",
            ),
            Self::InvalidReviewRecord => {
                formatter.write_str("The review note does not have its required stable record.")
            }
            Self::UnexpectedReviewRecord => {
                formatter.write_str("Only review notes may carry governed review lifecycle data.")
            }
            Self::EmptyReviewField => {
                formatter.write_str("Review identities, messages, and evidence must not be empty.")
            }
            Self::ReviewIdentityTooLong => write!(
                formatter,
                "Review identity exceeds {MAX_DESIGN_REVIEW_IDENTITY_LEN} characters."
            ),
            Self::ReviewMessageTooLong => write!(
                formatter,
                "Review message exceeds {MAX_DESIGN_REVIEW_MESSAGE_LEN} characters."
            ),
            Self::ReviewEvidenceLabelTooLong => write!(
                formatter,
                "Review evidence label exceeds {MAX_DESIGN_REVIEW_EVIDENCE_LABEL_LEN} characters."
            ),
            Self::ReviewEvidenceIdentityTooLong => write!(
                formatter,
                "Review evidence identity exceeds {MAX_DESIGN_REVIEW_EVIDENCE_IDENTITY_LEN} characters."
            ),
            Self::NonCanonicalReviewField => formatter
                .write_str("Review metadata contains uncommitted whitespace or line endings."),
            Self::DuplicateReviewIdentity => {
                formatter.write_str("Review messages and evidence must have unique identities.")
            }
            Self::TooManyReviewMessages => write!(
                formatter,
                "Review thread exceeds {MAX_DESIGN_REVIEW_MESSAGES} messages."
            ),
            Self::TooMuchReviewEvidence => write!(
                formatter,
                "Review thread exceeds {MAX_DESIGN_REVIEW_EVIDENCE} evidence references."
            ),
            Self::InvalidReviewResolution => formatter
                .write_str("An open review cannot retain a resolved-state disposition note."),
            Self::ReviewRecordNotFound => {
                formatter.write_str("The selected design-review record no longer exists.")
            }
            Self::InvalidLayer => {
                formatter.write_str("Design notes must remain on drawing / annotation.")
            }
            Self::ReadOnly => formatter.write_str("The active schematic is read-only."),
            Self::StaleDocument => formatter
                .write_str("The schematic changed after the design-note contract was prepared."),
        }
    }
}

impl SchematicState {
    pub fn validate_pending_design_note(
        &self,
        pending: &PendingDesignNotePlacement,
    ) -> Result<(), DesignNoteError> {
        if self.read_only {
            return Err(DesignNoteError::ReadOnly);
        }
        if pending.topology_version != self.topology_version() {
            return Err(DesignNoteError::StaleDocument);
        }
        if pending.expected_design_notes != self.design_notes {
            return Err(DesignNoteError::StaleDocument);
        }
        if pending.layer != DesignNoteLayer::DrawingAnnotation {
            return Err(DesignNoteError::InvalidLayer);
        }
        let normalized = normalize_design_note_text(&pending.text)?;
        if normalized != pending.text {
            return Err(DesignNoteError::NonCanonicalText);
        }
        validate_kind_text(pending.kind, &pending.text)
    }

    /// Commit one design note as one undo transaction. The object is
    /// intentionally excluded from electrical connectivity collections.
    pub fn place_pending_design_note(
        &mut self,
        pos: Point,
        pending: PendingDesignNotePlacement,
    ) -> Result<u64, DesignNoteError> {
        self.validate_pending_design_note(&pending)?;
        let id = self.next_id();
        let mut note = DesignNote::new(id, pos, pending.kind, pending.text)?;
        if pending.kind == DesignNoteKind::ReviewNote {
            let anchor = self
                .validated_revisions
                .records()
                .last()
                .map(|revision| revision.revision_digest().to_string());
            note.anchor_review_to_revision(anchor)?;
        }
        let changed = self.with_undo("place design note", |schematic| {
            schematic.design_notes.push(note);
            schematic.selection.clear();
            schematic.selection.select_design_note(id);
            schematic.is_dirty = true;
        });
        if changed {
            Ok(id)
        } else {
            Err(DesignNoteError::ReadOnly)
        }
    }

    /// Apply one review mutation against an exact note snapshot. This closes
    /// delayed-dialog and cross-document races without treating comments as
    /// electrical topology changes.
    pub fn apply_design_review_mutation(
        &mut self,
        note_id: u64,
        expected_design_notes: &[DesignNote],
        mutation: DesignReviewMutation,
    ) -> Result<(), DesignNoteError> {
        if self.read_only {
            return Err(DesignNoteError::ReadOnly);
        }
        if self.design_notes != expected_design_notes {
            return Err(DesignNoteError::StaleDocument);
        }
        let index = self
            .design_notes
            .iter()
            .position(|note| note.id == note_id)
            .ok_or(DesignNoteError::ReviewRecordNotFound)?;
        let mut candidate = self.design_notes[index].clone();
        mutation.clone().apply(&mut candidate)?;
        candidate.validate()?;
        let label = mutation.undo_label();
        let changed = self.with_undo(label, |schematic| {
            schematic.design_notes[index] = candidate;
            schematic.is_dirty = true;
        });
        if changed {
            Ok(())
        } else {
            Err(DesignNoteError::ReadOnly)
        }
    }
}

impl std::error::Error for DesignNoteError {}

fn normalize_design_note_text(source: &str) -> Result<String, DesignNoteError> {
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    let normalized = normalized.trim().to_owned();
    if normalized.is_empty() {
        return Err(DesignNoteError::EmptyText);
    }
    if normalized.chars().count() > MAX_DESIGN_NOTE_TEXT_LEN {
        return Err(DesignNoteError::TextTooLong);
    }
    if normalized
        .chars()
        .any(|character| character.is_control() && character != '\n' && character != '\t')
    {
        return Err(DesignNoteError::InvalidControlCharacter);
    }
    Ok(normalized)
}

fn canonical_review_identity(source: String) -> Result<String, DesignNoteError> {
    canonical_review_single_line(source, MAX_DESIGN_REVIEW_IDENTITY_LEN).map_err(
        |error| match error {
            CanonicalReviewFieldError::Empty => DesignNoteError::EmptyReviewField,
            CanonicalReviewFieldError::TooLong => DesignNoteError::ReviewIdentityTooLong,
            CanonicalReviewFieldError::Control => DesignNoteError::InvalidControlCharacter,
        },
    )
}

fn validate_review_identity(source: &str) -> Result<(), DesignNoteError> {
    if canonical_review_identity(source.to_owned())? == source {
        Ok(())
    } else {
        Err(DesignNoteError::NonCanonicalReviewField)
    }
}

fn canonical_review_message(source: String) -> Result<String, DesignNoteError> {
    let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
    let normalized = normalized.trim().to_owned();
    if normalized.is_empty() {
        return Err(DesignNoteError::EmptyReviewField);
    }
    if normalized.chars().count() > MAX_DESIGN_REVIEW_MESSAGE_LEN {
        return Err(DesignNoteError::ReviewMessageTooLong);
    }
    if normalized
        .chars()
        .any(|character| character.is_control() && character != '\n' && character != '\t')
    {
        return Err(DesignNoteError::InvalidControlCharacter);
    }
    Ok(normalized)
}

fn canonical_review_evidence_label(source: String) -> Result<String, DesignNoteError> {
    canonical_review_single_line(source, MAX_DESIGN_REVIEW_EVIDENCE_LABEL_LEN).map_err(|error| {
        match error {
            CanonicalReviewFieldError::Empty => DesignNoteError::EmptyReviewField,
            CanonicalReviewFieldError::TooLong => DesignNoteError::ReviewEvidenceLabelTooLong,
            CanonicalReviewFieldError::Control => DesignNoteError::InvalidControlCharacter,
        }
    })
}

fn canonical_review_evidence_identity(source: String) -> Result<String, DesignNoteError> {
    canonical_review_single_line(source, MAX_DESIGN_REVIEW_EVIDENCE_IDENTITY_LEN).map_err(|error| {
        match error {
            CanonicalReviewFieldError::Empty => DesignNoteError::EmptyReviewField,
            CanonicalReviewFieldError::TooLong => DesignNoteError::ReviewEvidenceIdentityTooLong,
            CanonicalReviewFieldError::Control => DesignNoteError::InvalidControlCharacter,
        }
    })
}

#[derive(Debug, Clone, Copy)]
enum CanonicalReviewFieldError {
    Empty,
    TooLong,
    Control,
}

fn canonical_review_single_line(
    source: String,
    max_chars: usize,
) -> Result<String, CanonicalReviewFieldError> {
    let normalized = source.trim().to_owned();
    if normalized.is_empty() {
        return Err(CanonicalReviewFieldError::Empty);
    }
    if normalized.chars().count() > max_chars {
        return Err(CanonicalReviewFieldError::TooLong);
    }
    if normalized.chars().any(char::is_control) {
        return Err(CanonicalReviewFieldError::Control);
    }
    Ok(normalized)
}

fn validate_kind_text(kind: DesignNoteKind, text: &str) -> Result<(), DesignNoteError> {
    match kind {
        DesignNoteKind::PropertyDisplay => parse_property_template(text).map(|_| ()),
        DesignNoteKind::RequirementLink => {
            if requirement_target(text).is_some() {
                Ok(())
            } else {
                Err(DesignNoteError::InvalidRequirementReference)
            }
        }
        DesignNoteKind::PlainText | DesignNoteKind::ReviewNote => Ok(()),
    }
}

fn requirement_target(source: &str) -> Option<RequirementTarget<'_>> {
    if source.starts_with("https://") || source.starts_with("http://") {
        let authority_and_path = source.split_once("://")?.1;
        let authority = authority_and_path
            .split(['/', '?', '#'])
            .next()
            .unwrap_or_default();
        let valid = !authority.is_empty()
            && !authority.starts_with('.')
            && !authority.ends_with('.')
            && source
                .chars()
                .all(|character| !character.is_whitespace() && !character.is_control());
        return valid.then_some(RequirementTarget::ExternalUri(source));
    }
    let valid = source.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ':' | '/' | '#')
    });
    valid.then_some(RequirementTarget::ProjectSpecification(source))
}

fn parse_property_template(source: &str) -> Result<Vec<(&str, &str)>, DesignNoteError> {
    let mut tokens = Vec::new();
    let mut remainder = source;
    while let Some(start) = remainder.find("${") {
        let after_start = &remainder[start + 2..];
        let Some(end) = after_start.find('}') else {
            return Err(DesignNoteError::UnterminatedProperty);
        };
        let property = &after_start[..end];
        if !matches!(property, "view" | "component_count" | "conductor_count") {
            return Err(DesignNoteError::UnknownProperty(property.to_owned()));
        }
        tokens.push((&remainder[..start], property));
        remainder = &after_start[end + 1..];
    }
    if tokens.is_empty() {
        return Err(DesignNoteError::UnknownProperty(source.to_owned()));
    }
    tokens.push((remainder, ""));
    Ok(tokens)
}

fn render_property_template(source: &str, context: &DesignNoteRenderContext<'_>) -> String {
    let Ok(tokens) = parse_property_template(source) else {
        return source.to_owned();
    };
    let mut rendered = String::new();
    for (literal, property) in tokens {
        rendered.push_str(literal);
        match property {
            "view" => rendered.push_str(context.view_path),
            "component_count" => rendered.push_str(&context.component_count.to_string()),
            "conductor_count" => rendered.push_str(&context.conductor_count.to_string()),
            "" => {}
            _ => unreachable!("validated property token"),
        }
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProjectId;
    use crate::state::{ValidatedRevisionRequest, ValidationFindingCounts};

    #[test]
    fn property_display_resolves_only_documented_read_only_tokens() {
        let note = DesignNote::new(
            7,
            Point::origin(),
            DesignNoteKind::PropertyDisplay,
            "${view} / ${component_count} parts",
        )
        .unwrap();
        assert_eq!(
            note.rendered_text(&DesignNoteRenderContext {
                view_path: "user/top/schematic",
                component_count: 12,
                conductor_count: 8,
            }),
            "user/top/schematic / 12 parts"
        );
        assert!(matches!(
            DesignNote::new(
                8,
                Point::origin(),
                DesignNoteKind::PropertyDisplay,
                "${shell}"
            ),
            Err(DesignNoteError::UnknownProperty(_))
        ));
    }

    #[test]
    fn review_record_is_stable_across_edits_and_removed_on_type_change() {
        let mut note = DesignNote::new(
            42,
            Point::origin(),
            DesignNoteKind::ReviewNote,
            "Check this bias path",
        )
        .unwrap();
        assert_eq!(note.review.as_ref().unwrap().record_id, "NOTE-0042");
        note.update(DesignNoteKind::ReviewNote, "Review updated")
            .unwrap();
        assert_eq!(note.review.as_ref().unwrap().record_id, "NOTE-0042");
        note.update(DesignNoteKind::PlainText, "Documentation")
            .unwrap();
        assert!(note.review.is_none());
    }

    #[test]
    fn legacy_review_records_migrate_to_an_empty_durable_thread() {
        let note: DesignNote = serde_json::from_value(serde_json::json!({
            "id": 42,
            "pos": { "x": 1, "y": 2 },
            "kind": "review_note",
            "text": "Confirm the bias model",
            "layer": "drawing_annotation",
            "review": {
                "record_id": "NOTE-0042",
                "state": "open"
            }
        }))
        .unwrap();

        note.validate().unwrap();
        let review = note.review.unwrap();
        assert!(review.assignee.is_none());
        assert!(review.anchored_revision.is_none());
        assert!(review.messages.is_empty());
        assert!(review.evidence.is_empty());
        assert!(review.resolution_note.is_none());
    }

    #[test]
    fn review_assign_reply_evidence_and_resolution_are_durable_and_validated() {
        let mut note = DesignNote::new(
            42,
            Point::origin(),
            DesignNoteKind::ReviewNote,
            "Confirm the bias model",
        )
        .unwrap();
        note.assign_review(Some("M. Chen")).unwrap();
        note.anchor_review_to_revision(Some("revision-0007"))
            .unwrap();
        note.append_review_message("J. Whitfield", "Correlation attached.", 101)
            .unwrap();
        note.attach_review_evidence(
            "Run 38 / OPA189 correlation",
            "result://run-38/measurement/input-bias",
            Some("sha256:0123456789abcdef".to_owned()),
        )
        .unwrap();
        note.resolve_review("M. Chen", "Confirmed against vendor maximum.", 102)
            .unwrap();

        note.validate().unwrap();
        let review = note.review.as_ref().unwrap();
        assert_eq!(review.assignee.as_deref(), Some("M. Chen"));
        assert_eq!(review.anchored_revision.as_deref(), Some("revision-0007"));
        assert_eq!(review.messages.len(), 2);
        assert_eq!(review.evidence.len(), 1);
        assert_eq!(review.state, DesignReviewState::Resolved);
        assert_eq!(
            review.resolution_note.as_deref(),
            Some("Confirmed against vendor maximum.")
        );

        note.set_review_state(DesignReviewState::Open).unwrap();
        assert!(note.review.as_ref().unwrap().resolution_note.is_none());
    }

    #[test]
    fn review_mutations_are_atomic_stale_guarded_and_undoable() {
        let mut schematic = SchematicState::default();
        schematic.design_notes.push(
            DesignNote::new(
                42,
                Point::origin(),
                DesignNoteKind::ReviewNote,
                "Confirm the bias model",
            )
            .unwrap(),
        );
        let expected = schematic.design_notes.clone();
        schematic
            .apply_design_review_mutation(
                42,
                &expected,
                DesignReviewMutation::Assign {
                    assignee: Some("M. Chen".to_owned()),
                },
            )
            .unwrap();
        assert_eq!(
            schematic.design_notes[0]
                .review
                .as_ref()
                .unwrap()
                .assignee
                .as_deref(),
            Some("M. Chen")
        );
        assert!(schematic.undo());
        assert!(
            schematic.design_notes[0]
                .review
                .as_ref()
                .unwrap()
                .assignee
                .is_none()
        );

        schematic.design_notes[0].text = "Concurrent edit".to_owned();
        let before = schematic.design_notes.clone();
        assert_eq!(
            schematic.apply_design_review_mutation(
                42,
                &expected,
                DesignReviewMutation::Reply {
                    author: "J. Whitfield".to_owned(),
                    body: "This must not commit.".to_owned(),
                    created_unix_ms: 103,
                },
            ),
            Err(DesignNoteError::StaleDocument)
        );
        assert_eq!(schematic.design_notes, before);
    }

    #[test]
    fn review_fields_reject_controls_and_duplicate_identities() {
        let mut note = DesignNote::new(
            42,
            Point::origin(),
            DesignNoteKind::ReviewNote,
            "Confirm the bias model",
        )
        .unwrap();
        assert_eq!(
            note.assign_review(Some("Owner\nInjected")),
            Err(DesignNoteError::InvalidControlCharacter)
        );
        note.append_review_message("Owner", "First", 1).unwrap();
        let duplicate = note.review.as_ref().unwrap().messages[0].clone();
        note.review.as_mut().unwrap().messages.push(duplicate);
        assert_eq!(
            note.validate(),
            Err(DesignNoteError::DuplicateReviewIdentity)
        );
    }

    #[test]
    fn requirement_links_reject_ambiguous_whitespace() {
        assert!(
            DesignNote::new(
                1,
                Point::origin(),
                DesignNoteKind::RequirementLink,
                "REQ-19"
            )
            .is_ok()
        );
        assert!(matches!(
            DesignNote::new(
                1,
                Point::origin(),
                DesignNoteKind::RequirementLink,
                "REQ 19"
            ),
            Err(DesignNoteError::InvalidRequirementReference)
        ));
    }

    #[test]
    fn placement_is_non_electrical_and_one_undo_record() {
        let mut schematic = SchematicState::default();
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::PlainText,
            "Bias network",
            schematic.topology_version(),
            &schematic.design_notes,
        )
        .unwrap();
        let topology = schematic.topology_version();
        let id = schematic
            .place_pending_design_note(Point::new(4, 7), pending)
            .unwrap();
        assert_eq!(schematic.design_notes.len(), 1);
        assert_eq!(schematic.design_notes[0].id, id);
        assert!(schematic.components.is_empty());
        assert!(schematic.wires.is_empty());
        assert_eq!(schematic.topology_version(), topology);
        assert!(schematic.can_undo());
        assert!(schematic.undo());
        assert!(schematic.design_notes.is_empty());
    }

    #[test]
    fn placed_review_note_anchors_to_latest_validated_revision() {
        let mut schematic = SchematicState::default();
        let validation_receipt_digest = schematic.validated_design_content_digest().unwrap();
        schematic
            .append_validated_revision(ValidatedRevisionRequest {
                project_id: ProjectId::new().to_string(),
                project_revision: 1,
                view_identity: "user/top/schematic".to_owned(),
                revision_note: "Validated review anchor".to_owned(),
                author: "Test engineer".to_owned(),
                validation_receipt_digest,
                finding_counts: ValidationFindingCounts::default(),
                dependencies: Vec::new(),
                advisory_dispositions: Vec::new(),
            })
            .unwrap();
        let expected_anchor = schematic.validated_revisions.records()[0]
            .revision_digest()
            .to_string();
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::ReviewNote,
            "Confirm the retained model binding",
            schematic.topology_version(),
            &schematic.design_notes,
        )
        .unwrap();
        schematic
            .place_pending_design_note(Point::new(4, 7), pending)
            .unwrap();
        assert_eq!(
            schematic.design_notes[0]
                .review
                .as_ref()
                .and_then(|review| review.anchored_revision.as_deref()),
            Some(expected_anchor.as_str())
        );
    }

    #[test]
    fn requirement_targets_accept_project_ids_and_safe_http_uris() {
        let project = DesignNote::new(
            1,
            Point::origin(),
            DesignNoteKind::RequirementLink,
            "REQ-19",
        )
        .unwrap();
        assert_eq!(
            project.requirement_target(),
            Some(RequirementTarget::ProjectSpecification("REQ-19"))
        );
        let uri = DesignNote::new(
            2,
            Point::origin(),
            DesignNoteKind::RequirementLink,
            "https://tracker.example/item?id=19&from=schematic%20note",
        )
        .unwrap();
        assert!(matches!(
            uri.requirement_target(),
            Some(RequirementTarget::ExternalUri(_))
        ));
        assert!(
            DesignNote::new(
                3,
                Point::origin(),
                DesignNoteKind::RequirementLink,
                "javascript:alert(1)",
            )
            .is_err()
        );
    }

    #[test]
    fn armed_note_contract_rejects_non_electrical_document_drift() {
        let mut schematic = SchematicState::default();
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::PlainText,
            "Bias network",
            schematic.topology_version(),
            &schematic.design_notes,
        )
        .unwrap();
        schematic.design_notes.push(
            DesignNote::new(
                99,
                Point::new(1, 1),
                DesignNoteKind::PlainText,
                "Concurrent note",
            )
            .unwrap(),
        );

        assert_eq!(
            schematic.validate_pending_design_note(&pending),
            Err(DesignNoteError::StaleDocument)
        );
    }
}
