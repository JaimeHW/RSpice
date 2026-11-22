//! Durable schematic documentation objects, and SPICE directive notes.
//!
//! Design notes live outside electrical components and the conductor
//! collections, so no note ever changes connectivity. Every kind but one is
//! documentation and never reaches generated SPICE: a SPICE directive note
//! holds cards (`.param`, `.model`, `.ic`, ...) that join every deck
//! generated for the design it is drawn in.

use serde::{Deserialize, Serialize};

use super::label_layout::{
    IMPORTED_TEXT_WORLD, LINE_BOX_EM, LayoutPoint, LayoutRect, PlacedText, TextEdge, text_box,
};

use rspice_design_model::Point;
use rspice_design_model::design_management::{
    SheetTextVariable, SheetTextVariables, expand_text_variables, text_variable_names,
};
pub use rspice_design_model::symbol_geometry::SymbolTextAlign;
use std::borrow::Cow;

/// Maximum authored note source retained in a schematic document.
pub const MAX_DESIGN_NOTE_TEXT_LEN: usize = 4096;
pub const MAX_DESIGN_REVIEW_IDENTITY_LEN: usize = 256;
pub const MAX_DESIGN_REVIEW_MESSAGES: usize = 1024;
pub const MAX_DESIGN_REVIEW_EVIDENCE: usize = 256;
pub const MAX_DESIGN_REVIEW_MESSAGE_LEN: usize = 4096;
pub const MAX_DESIGN_REVIEW_EVIDENCE_LABEL_LEN: usize = 256;
pub const MAX_DESIGN_REVIEW_EVIDENCE_IDENTITY_LEN: usize = 1024;

/// A note's type size at the standard size, in world units. Every surface
/// letters a note at these measures times the note's scale.
pub const DESIGN_NOTE_FONT_WORLD: f32 = 12.0;
/// The distance from one line of a note to the next at the standard size.
pub const DESIGN_NOTE_LINE_ADVANCE_WORLD: f32 = 15.0;
/// One glyph's advance at the standard size: Plex Mono's 0.6 em.
pub const DESIGN_NOTE_GLYPH_WIDTH_WORLD: f32 = 7.2;
/// The top-left corner of a note's first line, from its anchor. It does not
/// scale, so the anchor mark keeps one distance from any note's text.
pub const DESIGN_NOTE_TEXT_OFFSET_WORLD: (f32, f32) = (6.0, 3.0);
/// The sizes a note may be lettered at, in percent of the standard size.
pub const DESIGN_NOTE_TEXT_SIZE_PERCENT: std::ops::RangeInclusive<u16> = 25..=800;
const STANDARD_TEXT_SIZE_PERCENT: u16 = 100;

fn standard_text_size() -> u16 {
    STANDARD_TEXT_SIZE_PERCENT
}

fn is_standard_text_size(percent: &u16) -> bool {
    *percent == STANDARD_TEXT_SIZE_PERCENT
}

fn is_left(align: &SymbolTextAlign) -> bool {
    *align == SymbolTextAlign::Left
}

/// The sizes a note is offered at, in percent of the standard size: smaller
/// print for a side remark, larger for a heading on the sheet.
pub const DESIGN_NOTE_SIZE_CHOICES: [u16; 8] = [50, 75, 100, 125, 150, 200, 300, 400];

/// A note size as the reader is shown it.
pub fn design_note_size_label(percent: u16) -> String {
    format!("{percent}%")
}

/// What a note is: documentation of four kinds, or SPICE directives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignNoteKind {
    #[default]
    PlainText,
    /// SPICE cards that join the netlist of the design the note is drawn in,
    /// line for line, as KiCad's and ADS's directive text on the sheet does.
    SpiceDirective,
    PropertyDisplay,
    RequirementLink,
    ReviewNote,
}

impl DesignNoteKind {
    pub const ALL: [Self; 5] = [
        Self::PlainText,
        Self::SpiceDirective,
        Self::PropertyDisplay,
        Self::RequirementLink,
        Self::ReviewNote,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PlainText => "Plain text",
            Self::SpiceDirective => "SPICE directive",
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
    /// Apply the payload to a caller-owned candidate. The document owner checks
    /// permissions and staleness, validates the candidate, and commits with undo.
    pub fn apply(self, note: &mut DesignNote) -> Result<(), DesignNoteError> {
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

    /// Description retained by the document owner's undo transaction.
    pub fn undo_label(&self) -> &'static str {
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

/// The distance from one line of a lettered note to the next at the standard
/// size, as LTspice spaces its lines.
const LETTERED_NOTE_LINE_ADVANCE_WORLD: f32 = 17.5;

/// The distance from one line of a note lettered at `size_percent` to the
/// next, in world units.
#[must_use]
pub fn lettered_note_line_advance(size_percent: u16) -> f32 {
    LETTERED_NOTE_LINE_ADVANCE_WORLD * f32::from(size_percent) / 100.0
}

/// How a note brought from another tool's drawing is lettered: in RSpice's
/// proportional face at the source's size, every line justified about the
/// note's point as the source justified it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoteLettering {
    /// Which edge of each line stands on the point, along the line.
    pub along: TextEdge,
    /// Which edge of the lines together stands on the point, across them.
    pub across: TextEdge,
    /// Lines centred across stand on the point by the first, the rest
    /// running on past it, rather than by the middle of them all.
    pub by_first_line: bool,
    /// The lines read upward, each to the right of the one before.
    pub upward: bool,
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
    /// The note's lettering in percent of the standard size. A comment
    /// imported from another tool keeps the size its drawing gave it.
    #[serde(
        default = "standard_text_size",
        skip_serializing_if = "is_standard_text_size"
    )]
    pub text_size_percent: u16,
    /// The note draws no anchor mark: lettering brought in from a drawing
    /// that marks none reads as it did there.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub anchor_mark_hidden: bool,
    /// Set when the note came from another tool's drawing and letters as it
    /// did there; `pos` is then the point the drawing stood it on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lettering: Option<NoteLettering>,
    /// Which edge of each line stands on the note's point: the lines read on
    /// from it, are centred on it, or end at it. A note lettered as a drawing
    /// lettered it keeps its alignment in `lettering`.
    #[serde(default, skip_serializing_if = "is_left")]
    pub align: SymbolTextAlign,
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
            text_size_percent: STANDARD_TEXT_SIZE_PERCENT,
            anchor_mark_hidden: false,
            lettering: None,
            align: SymbolTextAlign::Left,
        })
    }

    /// Which edge of each line stands on the note's point.
    pub fn alignment(&self) -> SymbolTextAlign {
        match self.lettering.map(|lettering| lettering.along) {
            None => self.align,
            Some(TextEdge::Start) => SymbolTextAlign::Left,
            Some(TextEdge::Center) => SymbolTextAlign::Center,
            Some(TextEdge::End) => SymbolTextAlign::Right,
        }
    }

    /// The same note with each line aligned `align` on its point.
    #[must_use]
    pub fn with_alignment(mut self, align: SymbolTextAlign) -> Self {
        if let Some(lettering) = self.lettering.as_mut() {
            lettering.along = match align {
                SymbolTextAlign::Left => TextEdge::Start,
                SymbolTextAlign::Center => TextEdge::Center,
                SymbolTextAlign::Right => TextEdge::End,
            };
        } else {
            self.align = align;
        }
        self
    }

    /// Where a line `width` world units long starts, from the note's point
    /// along x: past the anchor mark, centred on the point, or ending short
    /// of the mark.
    pub fn line_start_world(&self, width: f32) -> f32 {
        match self.align {
            SymbolTextAlign::Left => DESIGN_NOTE_TEXT_OFFSET_WORLD.0,
            SymbolTextAlign::Center => -width * 0.5,
            SymbolTextAlign::Right => -DESIGN_NOTE_TEXT_OFFSET_WORLD.0 - width,
        }
    }

    /// The same note lettered as a drawing lettered it.
    pub fn with_lettering(mut self, lettering: NoteLettering) -> Self {
        self.lettering = Some(lettering);
        self
    }

    /// A lettered note's type size, in world units.
    pub fn lettered_size_world(&self) -> f32 {
        f32::from(IMPORTED_TEXT_WORLD) * self.text_scale()
    }

    /// Each line of a lettered note reading `rendered_text`, where it stands
    /// in world units.
    pub fn lettered_lines<'a>(
        &self,
        lettering: NoteLettering,
        rendered_text: &'a str,
    ) -> Vec<(PlacedText, &'a str)> {
        let lines = rendered_text.split('\n').collect::<Vec<_>>();
        let advance = lettered_note_line_advance(self.text_size_percent);
        let reach = self.lettered_size_world() * LINE_BOX_EM * 0.5;
        let beyond_first = (lines.len() - 1) as f32 * advance;
        // The middle of the first line, from the point, across the lines.
        let first = match lettering.across {
            TextEdge::Start => reach,
            TextEdge::Center if lettering.by_first_line => 0.0,
            TextEdge::Center => -beyond_first * 0.5,
            TextEdge::End => -reach - beyond_first,
        };
        let (x, y) = (self.pos.x as f32, self.pos.y as f32);
        lines
            .into_iter()
            .enumerate()
            .map(|(index, line)| {
                let across = first + index as f32 * advance;
                let anchor = if lettering.upward {
                    LayoutPoint::new(x + across, y)
                } else {
                    LayoutPoint::new(x, y + across)
                };
                let placed = PlacedText {
                    anchor,
                    horizontal: lettering.along,
                    vertical: TextEdge::Center,
                    upward: lettering.upward,
                };
                (placed, line)
            })
            .collect()
    }

    /// The box the lines of a lettered note take, in world units.
    pub fn lettered_box(&self, lettering: NoteLettering, rendered_text: &str) -> LayoutRect {
        let size = self.lettered_size_world();
        self.lettered_lines(lettering, rendered_text)
            .into_iter()
            .map(|(placed, line)| text_box(placed, line, size))
            .reduce(|all, line| LayoutRect {
                min: LayoutPoint::new(all.min.x.min(line.min.x), all.min.y.min(line.min.y)),
                max: LayoutPoint::new(all.max.x.max(line.max.x), all.max.y.max(line.max.y)),
            })
            .unwrap_or(LayoutRect {
                min: LayoutPoint::new(self.pos.x as f32, self.pos.y as f32),
                max: LayoutPoint::new(self.pos.x as f32, self.pos.y as f32),
            })
    }

    /// The same note without its anchor mark, or with it.
    pub fn with_anchor_mark_hidden(mut self, hidden: bool) -> Self {
        self.anchor_mark_hidden = hidden;
        self
    }

    /// The same note lettered at `percent` of the standard size, held to
    /// the sizes a note may take.
    pub fn with_text_size_percent(mut self, percent: u16) -> Self {
        self.text_size_percent = percent.clamp(
            *DESIGN_NOTE_TEXT_SIZE_PERCENT.start(),
            *DESIGN_NOTE_TEXT_SIZE_PERCENT.end(),
        );
        self
    }

    /// Whether the note is lettered at the standard size.
    pub fn has_standard_text_size(&self) -> bool {
        is_standard_text_size(&self.text_size_percent)
    }

    /// What a surface multiplies the standard lettering by.
    pub fn text_scale(&self) -> f32 {
        f32::from(self.text_size_percent.clamp(
            *DESIGN_NOTE_TEXT_SIZE_PERCENT.start(),
            *DESIGN_NOTE_TEXT_SIZE_PERCENT.end(),
        )) / f32::from(STANDARD_TEXT_SIZE_PERCENT)
    }

    /// The width and height of the box `rendered_text` stands in, in world
    /// units, from the top-left corner of its first line.
    pub fn text_extent_world(&self, rendered_text: &str) -> (f32, f32) {
        let (lines, longest) = rendered_text
            .split('\n')
            .fold((0_usize, 0_usize), |(lines, longest), line| {
                (lines + 1, longest.max(line.chars().count()))
            });
        let scale = self.text_scale();
        (
            longest.max(1) as f32 * DESIGN_NOTE_GLYPH_WIDTH_WORLD * scale,
            lines.max(1) as f32 * DESIGN_NOTE_LINE_ADVANCE_WORLD * scale,
        )
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

/// One value a Property display note letters: one of its sheet's, or a count
/// of what the schematic holds. The note writes it as `${NAME}`; the note's
/// form picks it from a list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignNoteField {
    Sheet(SheetTextVariable),
    PartCount,
    ConductorCount,
}

impl DesignNoteField {
    /// Every field, in the order the form lists them.
    pub fn all() -> impl Iterator<Item = Self> {
        SheetTextVariable::ALL
            .into_iter()
            .map(Self::Sheet)
            .chain([Self::PartCount, Self::ConductorCount])
    }

    /// The name the note writes the field by, inside `${` and `}`.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sheet(variable) => variable.name(),
            Self::PartCount => "PART_COUNT",
            Self::ConductorCount => "CONDUCTOR_COUNT",
        }
    }

    /// What the form's list calls the field.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sheet(variable) => variable.label(),
            Self::PartCount => "Part count",
            Self::ConductorCount => "Conductor count",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::all().find(|field| field.name() == name)
    }

    /// The field as the note's text writes it.
    pub fn token(self) -> String {
        format!("${{{}}}", self.name())
    }
}

/// The first field a Property display note's `text` names.
pub fn named_property_field(text: &str) -> Option<DesignNoteField> {
    text_variable_names(text)
        .ok()?
        .into_iter()
        .find_map(DesignNoteField::from_name)
}

/// `text` naming `field`, as the form's Field list picks it: in place of the
/// first field the text names, keeping the words around it, or after the
/// text when it names none.
pub fn name_property_field(text: &str, field: DesignNoteField) -> String {
    if let Some(current) = named_property_field(text) {
        return text.replacen(&current.token(), &field.token(), 1);
    }
    let separator = if text.is_empty() || text.ends_with(char::is_whitespace) {
        ""
    } else {
        " "
    };
    format!("{text}{separator}{}", field.token())
}

/// What a Property display note's fields read: its sheet's values and the
/// schematic's counts. Only these named, read-only values are lettered;
/// nothing a note says is evaluated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignNoteRenderContext<'a> {
    pub variables: &'a SheetTextVariables,
    pub component_count: usize,
    pub conductor_count: usize,
}

impl<'a> DesignNoteRenderContext<'a> {
    pub fn for_document(
        document: &super::document::SchematicDocument,
        variables: &'a SheetTextVariables,
    ) -> Self {
        Self {
            variables,
            component_count: document.components.len(),
            conductor_count: document.wires.len() + document.buses.len(),
        }
    }
}

impl DesignNote {
    pub fn rendered_text(&self, context: &DesignNoteRenderContext<'_>) -> String {
        match self.kind {
            DesignNoteKind::PlainText
            | DesignNoteKind::SpiceDirective
            | DesignNoteKind::ReviewNote => self.text.clone(),
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
    NoProperty,
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
    /// A line of a SPICE directive note the netlist cannot take.
    RefusedDirective(super::spice_directive::RefusedDirectiveLine),
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
            Self::NoProperty => formatter.write_str("Pick a field for the note to show."),
            Self::UnknownProperty(name) => write!(
                formatter,
                "No field is called \"{name}\". Pick one from the Field list."
            ),
            Self::UnterminatedProperty => {
                formatter.write_str("A field name is not closed: end it with }.")
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
            Self::RefusedDirective(refused) => write!(formatter, "{refused}."),
        }
    }
}

impl std::error::Error for DesignNoteError {}

/// Normalize bounded authored text before preparing a note or a placement.
pub fn normalize_design_note_text(source: &str) -> Result<String, DesignNoteError> {
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

/// Validate the authored text against its note semantics.
pub fn validate_kind_text(kind: DesignNoteKind, text: &str) -> Result<(), DesignNoteError> {
    match kind {
        DesignNoteKind::PropertyDisplay => validate_property_template(text),
        DesignNoteKind::RequirementLink => {
            if requirement_target(text).is_some() {
                Ok(())
            } else {
                Err(DesignNoteError::InvalidRequirementReference)
            }
        }
        DesignNoteKind::SpiceDirective => {
            match super::spice_directive::refused_directive_lines(text)
                .into_iter()
                .next()
            {
                Some(refused) => Err(DesignNoteError::RefusedDirective(refused)),
                None => Ok(()),
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

/// A Property display note names at least one field, and only fields.
fn validate_property_template(source: &str) -> Result<(), DesignNoteError> {
    let names = text_variable_names(source).map_err(|_| DesignNoteError::UnterminatedProperty)?;
    if names.is_empty() {
        return Err(DesignNoteError::NoProperty);
    }
    match names
        .into_iter()
        .find(|name| DesignNoteField::from_name(name).is_none())
    {
        Some(unknown) => Err(DesignNoteError::UnknownProperty(unknown.to_owned())),
        None => Ok(()),
    }
}

fn render_property_template(source: &str, context: &DesignNoteRenderContext<'_>) -> String {
    expand_text_variables(source, |name| match DesignNoteField::from_name(name)? {
        DesignNoteField::Sheet(variable) => context.variables.value(variable).map(Cow::Borrowed),
        DesignNoteField::PartCount => Some(Cow::Owned(context.component_count.to_string())),
        DesignNoteField::ConductorCount => Some(Cow::Owned(context.conductor_count.to_string())),
    })
    .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_display_resolves_only_its_named_fields() {
        use rspice_design_model::design_management::{
            DrawingSheetTitleFieldId, SchematicSheetFormat,
        };
        let note = DesignNote::new(
            7,
            Point::origin(),
            DesignNoteKind::PropertyDisplay,
            "${TITLE} / ${PART_COUNT} parts",
        )
        .unwrap();
        let variables = SheetTextVariables::new(
            &SchematicSheetFormat::default(),
            &std::collections::BTreeMap::from([(
                DrawingSheetTitleFieldId::SheetTitle,
                "top".to_owned(),
            )]),
        );
        assert_eq!(
            note.rendered_text(&DesignNoteRenderContext {
                variables: &variables,
                component_count: 12,
                conductor_count: 8,
            }),
            "top / 12 parts"
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
        assert_eq!(
            validate_kind_text(DesignNoteKind::PropertyDisplay, "Revision"),
            Err(DesignNoteError::NoProperty)
        );
    }

    /// The form's Field list writes the field into the text: after it when
    /// the text names none, in place of the field it names otherwise, keeping
    /// the words around it.
    #[test]
    fn picking_a_field_names_it_in_the_text() {
        let title = DesignNoteField::from_name("TITLE").unwrap();
        let revision = DesignNoteField::from_name("REVISION").unwrap();
        assert_eq!(name_property_field("", title), "${TITLE}");
        assert_eq!(name_property_field("Rev", revision), "Rev ${REVISION}");
        assert_eq!(
            name_property_field("Rev ${TITLE} issued", revision),
            "Rev ${REVISION} issued"
        );
        assert_eq!(named_property_field("Rev ${REVISION}"), Some(revision));
        assert_eq!(named_property_field("${NOPE}"), None);
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
}
