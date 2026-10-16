//! The fields a drawing sheet's title block can carry: what each is called,
//! who owns its value, whether a sheet must show it, and how a sheet's
//! authored set resolves into the values every renderer letters.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetTitleFieldId {
    Project,
    CellView,
    SheetTitle,
    Page,
    Revision,
    /// ISO 7200's *Paper size*: the sheet's designation, such as `A4`.
    ///
    /// Stored as `format`, the name it was saved under before it was briefly
    /// withdrawn, so the sheets and sealed templates that name it keep
    /// loading.
    #[serde(rename = "format")]
    PaperSize,
    /// Retired. ISO 7200 places a drawing's scale outside its title block,
    /// and a schematic is not drawn to one, so the cell only ever said 1:1
    /// about a circuit that has no size. The variant remains only so sheets
    /// authored before it was withdrawn still load — it is absent from
    /// [`Self::ALL`], so nothing offers, resolves, or renders it,
    /// deserialization drops it from the field set, and a sealed organization
    /// template keeps it in an order nothing reads it from.
    Scale,
    DrawnBy,
    CheckedBy,
    ApprovedBy,
    Date,
    Organization,
    DocumentId,
    Classification,
    /// ISO 7200's *Document type*: what kind of document the sheet is. It
    /// reads [`DRAWING_SHEET_DEFAULT_DOCUMENT_TYPE`] until a sheet says
    /// otherwise.
    DocumentType,
    /// ISO 7200's *Document status*: where the document stands in its
    /// release, read from the project's document control. Revision and Date
    /// carry an index and a date; the status is lettered here instead of in
    /// either of them.
    DocumentStatus,
}

/// What a schematic sheet is, in the name IEC 61082 gives it: the Document
/// type a sheet letters until it is given another.
pub const DRAWING_SHEET_DEFAULT_DOCUMENT_TYPE: &str = "Circuit diagram";

impl DrawingSheetTitleFieldId {
    pub const ALL: [Self; 15] = [
        Self::Project,
        Self::CellView,
        Self::SheetTitle,
        Self::DocumentType,
        Self::Page,
        Self::Revision,
        Self::PaperSize,
        Self::DrawnBy,
        Self::CheckedBy,
        Self::ApprovedBy,
        Self::Date,
        Self::DocumentStatus,
        Self::Organization,
        Self::DocumentId,
        Self::Classification,
    ];

    /// Fields withdrawn from the product that older sheets may still carry.
    ///
    /// Deserialization drops these from the field set rather than refusing
    /// the sheet, so retiring a field costs no saved work. They are never
    /// written back.
    pub const RETIRED: [Self; 1] = [Self::Scale];

    /// Editable title-block values whose authority is the project rather than
    /// one sheet. Visibility remains sheet-owned, but the value is resolved
    /// from [`DrawingSheetProjectSettings::title_block_field_values`] on every
    /// canvas and hardcopy page.
    pub const PROJECT_OWNED: [Self; 3] =
        [Self::Organization, Self::DocumentId, Self::Classification];

    #[must_use]
    pub const fn is_project_owned(self) -> bool {
        matches!(
            self,
            Self::Organization | Self::DocumentId | Self::Classification
        )
    }

    /// Canonical human-facing wording shared by canvas, dialogs, preview,
    /// and hardcopy. Every renderer letters it exactly as written, case
    /// included, so a caption reads the same wherever the sheet is shown.
    #[must_use]
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::CellView => "Cell / view",
            Self::SheetTitle => "Sheet title",
            Self::Page => "Page",
            Self::Revision => "Revision",
            Self::PaperSize => "Paper size",
            Self::Scale => "Scale",
            Self::DrawnBy => "Drawn by",
            Self::CheckedBy => "Checked by",
            Self::ApprovedBy => "Approved by",
            Self::Date => "Date",
            Self::Organization => "Organization",
            Self::DocumentId => "Document ID",
            Self::Classification => "Classification",
            Self::DocumentType => "Document type",
            Self::DocumentStatus => "Document status",
        }
    }

    /// Whether every renderer letters the field's value in the heavier
    /// weight. ISO 7200's figures letter values at regular weight and
    /// emphasise only the identification number, and a caption already stands
    /// a step of the series below its value, so only the document identifier
    /// is set apart.
    #[must_use]
    pub const fn emphasised(self) -> bool {
        matches!(self, Self::DocumentId)
    }

    /// What the field reads when neither the sheet nor the authority behind
    /// it supplies a value.
    #[must_use]
    pub const fn default_value(self) -> &'static str {
        match self {
            Self::DocumentType => DRAWING_SHEET_DEFAULT_DOCUMENT_TYPE,
            _ => "",
        }
    }

    #[must_use]
    pub const fn policy(self) -> DrawingSheetTitleFieldPolicy {
        match self {
            Self::CellView | Self::Page | Self::Revision => DrawingSheetTitleFieldPolicy {
                required_visible: true,
                value_authority: DrawingSheetTitleFieldValueAuthority::Automatic,
            },
            Self::SheetTitle => DrawingSheetTitleFieldPolicy {
                required_visible: true,
                value_authority: DrawingSheetTitleFieldValueAuthority::Authored,
            },
            // The project name is the leading segment of `Cell / view` in a
            // single-library workspace, so the sheet must be free to carry
            // one or the other rather than always both.
            Self::Project | Self::Date | Self::DocumentStatus | Self::Scale | Self::PaperSize => {
                DrawingSheetTitleFieldPolicy {
                    required_visible: false,
                    value_authority: DrawingSheetTitleFieldValueAuthority::Automatic,
                }
            }
            Self::DrawnBy
            | Self::CheckedBy
            | Self::ApprovedBy
            | Self::Organization
            | Self::DocumentId
            | Self::Classification
            | Self::DocumentType => DrawingSheetTitleFieldPolicy {
                required_visible: false,
                value_authority: DrawingSheetTitleFieldValueAuthority::Authored,
            },
        }
    }

    /// Whether a new sheet shows this field.
    ///
    /// ISO 7200 makes the creator and the approval person mandatory, so
    /// Drawn by and Approved by start on the sheet, empty until someone
    /// signs. Checked by is not one of the standard's fields, Classification
    /// is set by a release policy, and Paper size is optional and restates
    /// the sheet the reader is holding, so those wait in Page Setup until a
    /// sheet asks for them: every column a sheet does not spend is width the
    /// remaining cells keep.
    #[must_use]
    pub const fn visible_by_default(self) -> bool {
        !matches!(
            self,
            Self::CheckedBy | Self::Classification | Self::PaperSize
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawingSheetTitleFieldPolicy {
    pub required_visible: bool,
    pub value_authority: DrawingSheetTitleFieldValueAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawingSheetTitleFieldValueAuthority {
    Automatic,
    Authored,
}

/// One visible title-field value after ownership and migration fallback rules
/// have been resolved. This semantic value is renderer-independent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDrawingSheetTitleField {
    pub id: DrawingSheetTitleFieldId,
    pub value: String,
    pub authority: DrawingSheetTitleFieldValueAuthority,
}

/// Resolve the exact visible title fields for canvas and publication.
/// `authority_values` supplies automatic values plus canonical project-owned
/// authored values; ordinary authored values remain in the sheet format.
#[must_use]
pub fn resolve_drawing_sheet_title_fields(
    format: &SchematicSheetFormat,
    authority_values: &BTreeMap<DrawingSheetTitleFieldId, String>,
) -> Vec<ResolvedDrawingSheetTitleField> {
    format
        .title_block_field_order()
        .into_iter()
        .filter_map(|id| {
            let authored = format.title_block.fields.get(&id)?;
            authored.visible.then(|| {
                let authority = id.policy().value_authority;
                let governed = authority_values
                    .get(&id)
                    .map(String::as_str)
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or(id.default_value());
                let value = match authority {
                    DrawingSheetTitleFieldValueAuthority::Automatic => governed,
                    DrawingSheetTitleFieldValueAuthority::Authored if id.is_project_owned() => {
                        if governed.trim().is_empty() {
                            authored.value.as_str()
                        } else {
                            governed
                        }
                    }
                    DrawingSheetTitleFieldValueAuthority::Authored
                        if authored.value.trim().is_empty() =>
                    {
                        governed
                    }
                    DrawingSheetTitleFieldValueAuthority::Authored => authored.value.as_str(),
                };
                ResolvedDrawingSheetTitleField {
                    id,
                    value: if value.trim().is_empty() {
                        "—".to_owned()
                    } else {
                        value.to_owned()
                    },
                    authority,
                }
            })
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetTitleFieldState {
    pub value: String,
    pub visible: bool,
}

impl Default for DrawingSheetTitleFieldState {
    fn default() -> Self {
        Self {
            value: String::new(),
            visible: true,
        }
    }
}
