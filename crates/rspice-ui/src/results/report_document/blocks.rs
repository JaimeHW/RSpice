//! The block kinds a report page can hold, and what makes each well-formed.
//!
//! Tables, datasheets, requirements, specifications, and review notes each
//! carry their own shape rule, and all of them are checked before a block is
//! accepted into a page — a ragged table or a requirement without a
//! disposition is rejected at edit time, so a report can never be rendered
//! from a block that was never valid.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FigureSizing {
    FitWidth,
    FitPage,
    Natural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportTemplate {
    #[default]
    ReleaseVerification42,
    DesignReview,
    ModelQualification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportOutputFormats {
    pub pdf_a: bool,
    pub html_bundle: bool,
    pub canonical_json: bool,
    pub selected_csv: bool,
}

impl ReportOutputFormats {
    #[must_use]
    pub const fn has_any(self) -> bool {
        self.pdf_a || self.html_bundle || self.canonical_json || self.selected_csv
    }

    pub(crate) const fn is_default(&self) -> bool {
        self.pdf_a && self.html_bundle && self.canonical_json && !self.selected_csv
    }

    pub(super) fn validate(self) -> Result<(), ReportError> {
        if self.has_any() {
            Ok(())
        } else {
            Err(ReportError::InvalidValue {
                field: "report-document.output-formats",
                message: "at least one report output format must remain enabled".to_owned(),
            })
        }
    }
}

impl Default for ReportOutputFormats {
    fn default() -> Self {
        Self {
            pdf_a: true,
            html_bundle: true,
            canonical_json: true,
            selected_csv: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportPublicationTemplate {
    #[default]
    OrganizationVerificationReport,
    CustomerDatasheet,
    InternalReviewMemo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportPublicationPageSize {
    #[default]
    A4Portrait,
    UsLetterPortrait,
    A3Landscape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportDraftMarking {
    #[default]
    WatermarkWhileGatesOpen,
    NeverWatermark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportPageNumbering {
    #[default]
    SectionPageOfTotal,
    ContinuousPageNumbers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportTablePrecision {
    #[default]
    SevenSignificantDigits,
    FullStoredF64,
    MatchSourceDisplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReportPublicationProfile {
    pub template: ReportPublicationTemplate,
    pub page_size: ReportPublicationPageSize,
    pub draft_marking: ReportDraftMarking,
    pub numbering: ReportPageNumbering,
    pub table_precision: ReportTablePrecision,
}

impl ReportPublicationProfile {
    pub(crate) fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportPageUpdatePolicy {
    #[default]
    RefreshLinkedAutomatically,
    FreezeSelectedRevision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportPageInclusion {
    #[default]
    Included,
    ExcludedFromDraft,
    AppendixOnly,
}

impl ReportPageInclusion {
    pub(crate) const fn is_default(&self) -> bool {
        matches!(self, Self::Included)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case", tag = "mode")]
pub enum ReportPageEvidenceBinding {
    #[default]
    Unbound,
    ExactDataset {
        binding: DatasetBinding,
    },
    LatestAcceptedRun,
}

impl ReportPageEvidenceBinding {
    pub(crate) const fn is_default(&self) -> bool {
        matches!(self, Self::Unbound)
    }

    #[must_use]
    pub const fn exact_dataset(self) -> Option<DatasetBinding> {
        match self {
            Self::ExactDataset { binding } => Some(binding),
            Self::Unbound | Self::LatestAcceptedRun => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportBlockedGateTextPolicy {
    #[default]
    VerbatimFromSource,
    SummarizeWithLink,
}

impl ReportBlockedGateTextPolicy {
    pub(crate) const fn is_default(&self) -> bool {
        matches!(self, Self::VerbatimFromSource)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotFigureBlock {
    pub caption: String,
    pub alternative_text: String,
    pub sizing: FigureSizing,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "value")]
pub enum TableCell {
    Empty,
    Text(String),
    Number { value: f64, unit: Option<String> },
    Integer(i64),
    Boolean(bool),
}

impl TableCell {
    pub(super) fn validate(&self) -> Result<(), ReportError> {
        match self {
            Self::Text(value) => validate_text("table.cell.text", value, 16_384, true),
            Self::Number { value, unit } => {
                if !value.is_finite() {
                    return Err(ReportError::InvalidValue {
                        field: "table.cell.number",
                        message: "numeric table values must be finite".to_owned(),
                    });
                }
                if let Some(unit) = unit {
                    validate_label("table.cell.unit", unit, 64)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableColumn {
    pub key: String,
    pub heading: String,
    pub unit: Option<String>,
}

impl TableColumn {
    pub(super) fn validate(&self) -> Result<(), ReportError> {
        validate_token("table.column.key", &self.key, 128)?;
        validate_label("table.column.heading", &self.heading, 256)?;
        if let Some(unit) = &self.unit {
            validate_label("table.column.unit", unit, 64)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTableBlock {
    pub title: String,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<Vec<TableCell>>,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasheetField {
    pub key: String,
    pub label: String,
    pub value: String,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasheetBlock {
    pub title: String,
    pub fields: Vec<DatasheetField>,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequirementDisposition {
    NotEvaluated,
    Passed,
    Failed,
    Waived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementEntry {
    pub requirement_id: String,
    pub statement: String,
    pub disposition: RequirementDisposition,
    pub evidence_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementsBlock {
    pub title: String,
    pub entries: Vec<RequirementEntry>,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecificationDisposition {
    NotEvaluated,
    InSpecification,
    OutOfSpecification,
    Informational,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecificationEntry {
    pub expression: String,
    pub limit: String,
    pub measured: Option<String>,
    pub disposition: SpecificationDisposition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecificationsBlock {
    pub title: String,
    pub entries: Vec<SpecificationEntry>,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProseStyle {
    Body,
    ExecutiveSummary,
    Method,
    Conclusion,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProseBlock {
    pub style: ProseStyle,
    pub markdown: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewNoteStatus {
    Open,
    Addressed,
    Accepted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewNoteBlock {
    pub author: String,
    pub status: ReviewNoteStatus,
    pub message: String,
    pub created_at_unix_ms: u64,
    pub resolved_at_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBlock {
    pub title: String,
    pub summary: String,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "block-type", content = "content")]
pub enum ReportBlockKind {
    PlotFigure(PlotFigureBlock),
    DataTable(DataTableBlock),
    Datasheet(DatasheetBlock),
    Requirements(RequirementsBlock),
    Specifications(SpecificationsBlock),
    Prose(ProseBlock),
    ReviewNote(ReviewNoteBlock),
    Evidence(EvidenceBlock),
}

impl ReportBlockKind {
    #[must_use]
    pub const fn reference(&self) -> Option<&ReportReferenceMode> {
        match self {
            Self::PlotFigure(block) => Some(&block.reference),
            Self::DataTable(block) => Some(&block.reference),
            Self::Datasheet(block) => Some(&block.reference),
            Self::Requirements(block) => Some(&block.reference),
            Self::Specifications(block) => Some(&block.reference),
            Self::Evidence(block) => Some(&block.reference),
            Self::Prose(_) | Self::ReviewNote(_) => None,
        }
    }

    pub(super) fn set_reference(
        &mut self,
        reference: ReportReferenceMode,
    ) -> Result<(), ReportError> {
        match self {
            Self::PlotFigure(block) => block.reference = reference,
            Self::DataTable(block) => block.reference = reference,
            Self::Datasheet(block) => block.reference = reference,
            Self::Requirements(block) => block.reference = reference,
            Self::Specifications(block) => block.reference = reference,
            Self::Evidence(block) => block.reference = reference,
            Self::Prose(_) | Self::ReviewNote(_) => {
                return Err(ReportError::BlockHasNoExternalReference);
            }
        }
        self.validate()
    }

    pub(super) fn validate(&self) -> Result<(), ReportError> {
        match self {
            Self::PlotFigure(block) => {
                validate_label("plot-figure.caption", &block.caption, 2_048)?;
                validate_text(
                    "plot-figure.alternative-text",
                    &block.alternative_text,
                    8_192,
                    false,
                )?;
                block.reference.validate()?;
                if !matches!(
                    block.reference.snapshot().source,
                    ReportSourceId::VisualizationDocument { .. }
                ) {
                    return Err(ReportError::InvalidReferenceKind {
                        block: "plot-figure",
                        expected: "visualization-document",
                    });
                }
            }
            Self::DataTable(block) => validate_data_table(block)?,
            Self::Datasheet(block) => validate_datasheet(block)?,
            Self::Requirements(block) => validate_requirements(block)?,
            Self::Specifications(block) => validate_specifications(block)?,
            Self::Prose(block) => {
                validate_text("prose.markdown", &block.markdown, MAX_TEXT_BYTES, false)?;
            }
            Self::ReviewNote(block) => validate_review_note(block)?,
            Self::Evidence(block) => {
                validate_label("evidence.title", &block.title, 512)?;
                validate_text("evidence.summary", &block.summary, 65_536, false)?;
                block.reference.validate()?;
                if !matches!(
                    block.reference.snapshot().source,
                    ReportSourceId::VerificationEvidence { .. }
                ) {
                    return Err(ReportError::InvalidReferenceKind {
                        block: "evidence",
                        expected: "verification-evidence",
                    });
                }
            }
        }
        Ok(())
    }
}

pub(super) fn validate_data_table(block: &DataTableBlock) -> Result<(), ReportError> {
    validate_label("data-table.title", &block.title, 512)?;
    block.reference.validate()?;
    if block.columns.is_empty() || block.columns.len() > MAX_TABLE_COLUMNS {
        return Err(ReportError::InvalidValue {
            field: "data-table.columns",
            message: format!("a table requires 1 to {MAX_TABLE_COLUMNS} columns"),
        });
    }
    if block.rows.len() > MAX_TABLE_ROWS
        || block
            .columns
            .len()
            .checked_mul(block.rows.len())
            .is_none_or(|cells| cells > MAX_TABLE_CELLS)
    {
        return Err(ReportError::InvalidValue {
            field: "data-table.rows",
            message: format!(
                "a table may contain at most {MAX_TABLE_ROWS} rows and {MAX_TABLE_CELLS} cells"
            ),
        });
    }
    let mut keys = HashSet::with_capacity(block.columns.len());
    for column in &block.columns {
        column.validate()?;
        if !keys.insert(column.key.as_str()) {
            return Err(ReportError::DuplicateKey(column.key.clone()));
        }
    }
    for row in &block.rows {
        if row.len() != block.columns.len() {
            return Err(ReportError::InvalidValue {
                field: "data-table.rows",
                message: "every row must contain exactly one cell per declared column".to_owned(),
            });
        }
        for cell in row {
            cell.validate()?;
        }
    }
    Ok(())
}

pub(super) fn validate_datasheet(block: &DatasheetBlock) -> Result<(), ReportError> {
    validate_label("datasheet.title", &block.title, 512)?;
    block.reference.validate()?;
    if block.fields.is_empty() || block.fields.len() > MAX_STRUCTURED_ENTRIES {
        return Err(ReportError::InvalidValue {
            field: "datasheet.fields",
            message: format!("a datasheet requires 1 to {MAX_STRUCTURED_ENTRIES} fields"),
        });
    }
    let mut keys = HashSet::with_capacity(block.fields.len());
    for field in &block.fields {
        validate_token("datasheet.field.key", &field.key, 128)?;
        validate_label("datasheet.field.label", &field.label, 256)?;
        validate_text("datasheet.field.value", &field.value, 16_384, true)?;
        if let Some(unit) = &field.unit {
            validate_label("datasheet.field.unit", unit, 64)?;
        }
        if !keys.insert(field.key.as_str()) {
            return Err(ReportError::DuplicateKey(field.key.clone()));
        }
    }
    Ok(())
}

pub(super) fn validate_requirements(block: &RequirementsBlock) -> Result<(), ReportError> {
    validate_label("requirements.title", &block.title, 512)?;
    block.reference.validate()?;
    if block.entries.is_empty() || block.entries.len() > MAX_STRUCTURED_ENTRIES {
        return Err(ReportError::InvalidValue {
            field: "requirements.entries",
            message: format!("a requirements block requires 1 to {MAX_STRUCTURED_ENTRIES} entries"),
        });
    }
    let mut identities = HashSet::with_capacity(block.entries.len());
    for entry in &block.entries {
        validate_token("requirement.id", &entry.requirement_id, 256)?;
        validate_text("requirement.statement", &entry.statement, 65_536, false)?;
        if let Some(label) = &entry.evidence_label {
            validate_label("requirement.evidence-label", label, 512)?;
        }
        if !identities.insert(entry.requirement_id.as_str()) {
            return Err(ReportError::DuplicateKey(entry.requirement_id.clone()));
        }
    }
    Ok(())
}

pub(super) fn validate_specifications(block: &SpecificationsBlock) -> Result<(), ReportError> {
    validate_label("specifications.title", &block.title, 512)?;
    block.reference.validate()?;
    if block.entries.is_empty() || block.entries.len() > MAX_STRUCTURED_ENTRIES {
        return Err(ReportError::InvalidValue {
            field: "specifications.entries",
            message: format!(
                "a specifications block requires 1 to {MAX_STRUCTURED_ENTRIES} entries"
            ),
        });
    }
    let mut expressions = HashSet::with_capacity(block.entries.len());
    for entry in &block.entries {
        validate_text("specification.expression", &entry.expression, 4_096, true)?;
        validate_text("specification.limit", &entry.limit, 4_096, true)?;
        if let Some(measured) = &entry.measured {
            validate_text("specification.measured", measured, 4_096, true)?;
        }
        if !expressions.insert(entry.expression.as_str()) {
            return Err(ReportError::DuplicateKey(entry.expression.clone()));
        }
    }
    Ok(())
}

pub(super) fn validate_review_note(block: &ReviewNoteBlock) -> Result<(), ReportError> {
    validate_label("review-note.author", &block.author, 256)?;
    validate_text("review-note.message", &block.message, 65_536, false)?;
    match (block.status, block.resolved_at_unix_ms) {
        (ReviewNoteStatus::Open, None) => {}
        (ReviewNoteStatus::Addressed | ReviewNoteStatus::Accepted, Some(resolved))
            if resolved >= block.created_at_unix_ms => {}
        (ReviewNoteStatus::Open, Some(_)) => {
            return Err(ReportError::InvalidValue {
                field: "review-note.resolved-at",
                message: "an open review note must not carry a resolution timestamp".to_owned(),
            });
        }
        _ => {
            return Err(ReportError::InvalidValue {
                field: "review-note.resolved-at",
                message:
                    "addressed and accepted notes require a resolution timestamp at or after creation"
                        .to_owned(),
            });
        }
    }
    Ok(())
}

pub(super) fn validate_dataset_bindings(bindings: &[DatasetBinding]) -> Result<(), ReportError> {
    if bindings.len() > MAX_DATASET_BINDINGS {
        return Err(ReportError::InvalidValue {
            field: "reference.dataset-bindings",
            message: format!("at most {MAX_DATASET_BINDINGS} dataset bindings are permitted"),
        });
    }
    let mut identities = HashSet::with_capacity(bindings.len());
    for binding in bindings {
        if !identities.insert(binding.dataset_id) {
            return Err(ReportError::DuplicateDatasetBinding(binding.dataset_id));
        }
    }
    Ok(())
}

pub(super) fn validate_label(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), ReportError> {
    if value.is_empty()
        || value != value.trim()
        || value.len() > maximum_bytes
        || value.chars().any(char::is_control)
    {
        return Err(ReportError::InvalidValue {
            field,
            message: format!(
                "value must be trimmed, non-blank, contain no control characters, and not exceed {maximum_bytes} bytes"
            ),
        });
    }
    Ok(())
}

pub(super) fn validate_text(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
    single_line: bool,
) -> Result<(), ReportError> {
    if value.trim().is_empty() || value.len() > maximum_bytes {
        return Err(ReportError::InvalidValue {
            field,
            message: format!("text must be non-blank and not exceed {maximum_bytes} bytes"),
        });
    }
    let invalid_control = value
        .chars()
        .any(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'));
    if invalid_control || (single_line && value.contains(['\n', '\r'])) {
        return Err(ReportError::InvalidValue {
            field,
            message: "text contains a control character not permitted in this field".to_owned(),
        });
    }
    Ok(())
}

pub(super) fn validate_token(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), ReportError> {
    if value.is_empty()
        || value.len() > maximum_bytes
        || value != value.trim()
        || value.chars().any(|character| {
            character.is_control() || character.is_whitespace() || matches!(character, '/' | '\\')
        })
    {
        return Err(ReportError::InvalidValue {
            field,
            message: format!(
                "token must be trimmed, non-blank, path-neutral, contain no whitespace or control characters, and not exceed {maximum_bytes} bytes"
            ),
        });
    }
    Ok(())
}
