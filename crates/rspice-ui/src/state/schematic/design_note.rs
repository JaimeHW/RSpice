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
pub struct DesignReviewRecord {
    pub record_id: String,
    pub state: DesignReviewState,
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
        let review = (kind == DesignNoteKind::ReviewNote).then(|| DesignReviewRecord {
            record_id: format!("NOTE-{id:04}"),
            state: DesignReviewState::Open,
        });
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
                Ok(())
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
            self.review.take().or_else(|| {
                Some(DesignReviewRecord {
                    record_id: format!("NOTE-{:04}", self.id),
                    state: DesignReviewState::Open,
                })
            })
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
        let note = DesignNote::new(id, pos, pending.kind, pending.text)?;
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
