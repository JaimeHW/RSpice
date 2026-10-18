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
    /// ISO 7200's *Language code*: the ISO 639 code of the language the
    /// document is written in, such as `en`. Optional, and kept by the
    /// project, since every sheet of a document is written in one language.
    Language,
    /// ASME Y14.1's design activity identification: the CAGE code of the
    /// activity whose drawing number the sheet carries. Optional, kept by the
    /// project, and lettered only in the ASME form's DAI block.
    CageCode,
    /// The engineer who signs ASME Y14.1's block E, beside the drafter, the
    /// checker and the approver.
    Engineer,
    /// The dates block E's signatures were made on, each beside its name,
    /// written `YYYY-MM-DD`. Kept with the names, by the sheet.
    DrawnDate,
    CheckedDate,
    EngineerDate,
    /// Lettered from the document's date of issue while the sheet gives it
    /// none.
    ApprovedDate,
    /// The contract number ASME Y14.1 6.2 puts in block E when one applies.
    /// Optional, and kept by the project.
    ContractNumber,
}

/// What a schematic sheet is, in the name IEC 61082 gives it: the Document
/// type a sheet letters until it is given another.
pub const DRAWING_SHEET_DEFAULT_DOCUMENT_TYPE: &str = "Circuit diagram";

impl DrawingSheetTitleFieldId {
    /// Every field a sheet carries. A field added since the first of these
    /// goes last, so a block that pours its fields in this order keeps the
    /// order a saved sheet was drawn with.
    pub const ALL: [Self; 23] = [
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
        Self::Language,
        Self::CageCode,
        Self::Engineer,
        Self::DrawnDate,
        Self::CheckedDate,
        Self::EngineerDate,
        Self::ApprovedDate,
        Self::ContractNumber,
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
    pub const PROJECT_OWNED: [Self; 6] = [
        Self::Organization,
        Self::DocumentId,
        Self::Classification,
        Self::Language,
        Self::CageCode,
        Self::ContractNumber,
    ];

    #[must_use]
    pub const fn is_project_owned(self) -> bool {
        matches!(
            self,
            Self::Organization
                | Self::DocumentId
                | Self::Classification
                | Self::Language
                | Self::CageCode
                | Self::ContractNumber
        )
    }

    /// Whether the field is one of block E's signature dates, a calendar
    /// date written `YYYY-MM-DD` and checked as the document's date is.
    #[must_use]
    pub const fn is_signature_date(self) -> bool {
        matches!(
            self,
            Self::DrawnDate | Self::CheckedDate | Self::EngineerDate | Self::ApprovedDate
        )
    }

    /// What an empty entry for the field shows as a hint where it is typed:
    /// the form a date is written in, or the value the field reads until it
    /// is given one.
    #[must_use]
    pub const fn entry_hint(self) -> &'static str {
        if self.is_signature_date() {
            "YYYY-MM-DD"
        } else {
            self.default_value()
        }
    }

    /// The field's name wherever RSpice offers it for editing — the
    /// inspector, Page Setup and the title-block fields dialog — in the term
    /// ISO 7200 gives it, so the panel names a field as the printed form
    /// does. The ISO form's own captions are the same terms, abbreviated
    /// where its figures abbreviate them (`Rev.`, `Lang.`).
    #[must_use]
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::CellView => "Cell / view",
            Self::SheetTitle => "Title",
            Self::Page => "Sheet",
            Self::Revision => "Revision",
            Self::PaperSize => "Paper size",
            Self::Scale => "Scale",
            Self::DrawnBy => "Created by",
            Self::CheckedBy => "Checked by",
            Self::ApprovedBy => "Approved by",
            Self::Date => "Date of issue",
            Self::Organization => "Legal owner",
            Self::DocumentId => "Identification number",
            Self::Classification => "Classification",
            Self::DocumentType => "Document type",
            Self::DocumentStatus => "Document status",
            Self::Language => "Language",
            Self::CageCode => "CAGE code",
            Self::Engineer => "Engineer",
            Self::DrawnDate => "Created date",
            Self::CheckedDate => "Checked date",
            Self::EngineerDate => "Engineer date",
            Self::ApprovedDate => "Approved date",
            Self::ContractNumber => "Contract number",
        }
    }

    /// The caption a flowed grid — the compact and organization blocks —
    /// letters over the field. These blocks keep the wording they were
    /// drawn with before the ISO 7200 form, so a saved sheet on one of them
    /// prints as it always did, and an organization's sealed block keeps
    /// the labels it was released with. Every renderer letters it exactly
    /// as written, case included.
    #[must_use]
    pub const fn grid_caption(self) -> &'static str {
        match self {
            Self::SheetTitle => "Sheet title",
            Self::Page => "Page",
            Self::DrawnBy => "Drawn by",
            Self::Date => "Date",
            Self::Organization => "Organization",
            Self::DocumentId => "Document ID",
            other => other.display_label(),
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

    /// Who supplies the field's value, and whether every block must show it.
    ///
    /// Only the sheet number and the title must stand on every block. ISO
    /// 7200's form adds its own mandatory fields while a sheet is on it (see
    /// [`DrawingSheetTitleBlockTemplate::requires_field`]); the cell / view
    /// is the form's supplementary title and the revision index is optional
    /// there, so neither is required of a block.
    #[must_use]
    pub const fn policy(self) -> DrawingSheetTitleFieldPolicy {
        match self {
            Self::Page => DrawingSheetTitleFieldPolicy {
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
            Self::CellView
            | Self::Revision
            | Self::Project
            | Self::Date
            | Self::DocumentStatus
            | Self::Scale
            | Self::PaperSize => DrawingSheetTitleFieldPolicy {
                required_visible: false,
                value_authority: DrawingSheetTitleFieldValueAuthority::Automatic,
            },
            Self::DrawnBy
            | Self::CheckedBy
            | Self::ApprovedBy
            | Self::Organization
            | Self::DocumentId
            | Self::Classification
            | Self::DocumentType
            | Self::Language
            | Self::CageCode
            | Self::Engineer
            | Self::DrawnDate
            | Self::CheckedDate
            | Self::EngineerDate
            | Self::ApprovedDate
            | Self::ContractNumber => DrawingSheetTitleFieldPolicy {
                required_visible: false,
                value_authority: DrawingSheetTitleFieldValueAuthority::Authored,
            },
        }
    }

    /// Whether a new sheet shows this field.
    ///
    /// ISO 7200 makes the creator and the approval person mandatory, so
    /// Created by and Approved by start on the sheet, empty until someone
    /// signs. Checked by is not one of the standard's fields, Classification
    /// is set by a release policy, the language code is optional, Paper size
    /// is optional and restates the sheet the reader is holding, and in a
    /// single-library workspace the project's name is already the leading
    /// segment of Cell / view, so a sheet showing Project as well letters
    /// that name twice. Those wait in Page Setup until a sheet asks for
    /// them: every cell a sheet does not spend is width the cells beside it
    /// keep. The CAGE code, the engineer, the signature dates and the
    /// contract number belong to the ASME form, which always carries their
    /// cells, so no other block shows them unless a sheet asks.
    #[must_use]
    pub const fn visible_by_default(self) -> bool {
        !matches!(
            self,
            Self::Project
                | Self::CheckedBy
                | Self::Classification
                | Self::PaperSize
                | Self::Language
                | Self::CageCode
                | Self::Engineer
                | Self::DrawnDate
                | Self::CheckedDate
                | Self::EngineerDate
                | Self::ApprovedDate
                | Self::ContractNumber
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
///
/// `value` is empty for an authored field nobody has filled in, and every
/// renderer letters such a cell with its caption alone.
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
            format.title_field_shown(id).then(|| {
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
                    value: match authority {
                        _ if !value.trim().is_empty() => value.to_owned(),
                        // ASME Y14.35 letters a dash in REV for a drawing's
                        // initial release, so on the ASME form a revision not
                        // yet issued is a blank cell: a dash there would read
                        // as released.
                        DrawingSheetTitleFieldValueAuthority::Automatic
                            if id == DrawingSheetTitleFieldId::Revision
                                && format.title_block.template
                                    == DrawingSheetTitleBlockTemplate::AsmeY141 =>
                        {
                            String::new()
                        }
                        // A value RSpice supplies and does not have yet, such
                        // as the date of a document never issued, reads as
                        // not yet known.
                        DrawingSheetTitleFieldValueAuthority::Automatic => "—".to_owned(),
                        // A title block is a form: an empty authored cell is a
                        // box to fill in or sign on the print. A dash there
                        // would read as an answer, such as no approval needed.
                        DrawingSheetTitleFieldValueAuthority::Authored => String::new(),
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
