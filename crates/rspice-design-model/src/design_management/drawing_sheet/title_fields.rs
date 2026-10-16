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
    /// Retired. The sheet format restated the paper the reader is holding,
    /// in prose, in a cell narrow enough that it rarely fitted; Page Setup
    /// owns that fact and the border itself shows it. The variant remains
    /// only so sheets authored before it was withdrawn still load — it is
    /// absent from [`Self::ALL`], so nothing offers, resolves, or renders
    /// it, deserialization drops it from the field set, and a sealed
    /// organization template keeps it in an order nothing reads it from.
    Format,
    /// Retired. ISO 7200 places a drawing's scale outside its title block,
    /// and a schematic is not drawn to one, so the cell only ever said 1:1
    /// about a circuit that has no size. The variant remains, as
    /// [`Self::Format`] did, so a sheet or sealed template that names it
    /// still loads.
    Scale,
    DrawnBy,
    CheckedBy,
    ApprovedBy,
    Date,
    Organization,
    DocumentId,
    Classification,
}

impl DrawingSheetTitleFieldId {
    pub const ALL: [Self; 12] = [
        Self::Project,
        Self::CellView,
        Self::SheetTitle,
        Self::Page,
        Self::Revision,
        Self::DrawnBy,
        Self::CheckedBy,
        Self::ApprovedBy,
        Self::Date,
        Self::Organization,
        Self::DocumentId,
        Self::Classification,
    ];

    /// Fields withdrawn from the product that older sheets may still carry.
    ///
    /// Deserialization drops these from the field set rather than refusing
    /// the sheet, so retiring a field costs no saved work. They are never
    /// written back.
    pub const RETIRED: [Self; 2] = [Self::Format, Self::Scale];

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
            Self::Format => "Format",
            Self::Scale => "Scale",
            Self::DrawnBy => "Drawn by",
            Self::CheckedBy => "Checked by",
            Self::ApprovedBy => "Approved by",
            Self::Date => "Date",
            Self::Organization => "Organization",
            Self::DocumentId => "Document ID",
            Self::Classification => "Classification",
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
            Self::Project | Self::Date | Self::Scale | Self::Format => {
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
            | Self::Classification => DrawingSheetTitleFieldPolicy {
                required_visible: false,
                value_authority: DrawingSheetTitleFieldValueAuthority::Authored,
            },
        }
    }

    /// Whether a new sheet shows this field.
    ///
    /// The sign-off and classification cells are filled by a release
    /// process, not by drawing the circuit, so a sheet that has never been
    /// released would spend them on em dashes. They stay one click away in
    /// Page Setup, and every field a sheet does show is one the engineer can
    /// read: fewer columns means wider cells and more characters in each.
    #[must_use]
    pub const fn visible_by_default(self) -> bool {
        !matches!(
            self,
            Self::DrawnBy | Self::CheckedBy | Self::ApprovedBy | Self::Classification
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
                    .unwrap_or_default();
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
