//! Runtime draft and validation for the authored drawing-sheet studio.
//!
//! The draft deliberately mirrors the mockup's permanent sheet vocabulary.
//! It contains no printer, pagination, fit, or tiling state. Conversion to the
//! durable design-management model is isolated in `commit.rs` so additions to
//! that model cannot leak output-media concepts back into this workflow.

use crate::state::{
    AuthoredDrawingSheetSize, CustomDrawingSheetSnapshot, DRAWING_SHEET_MIXED_SIZES_ADVISORY,
    DrawingSheetBorderTemplate, DrawingSheetDisplayUnit, DrawingSheetFrameConvention,
    DrawingSheetInheritance, DrawingSheetManagedTemplateSnapshot, DrawingSheetMargins,
    DrawingSheetMarks, DrawingSheetPreset, DrawingSheetPresetScope, DrawingSheetStandard,
    DrawingSheetStandardSeries, DrawingSheetTitleBlock, DrawingSheetTitleBlockTemplate,
    DrawingSheetTitleFieldId, DrawingSheetTitleFieldState, DrawingSheetZoneMode, DrawingSheetZones,
    SchematicPageOrientation, SchematicSheetFormat, SchematicSheetFormatDraft, SheetId,
    drawing_sheet_automatic_zone_counts, drawing_sheet_sizes_mixed,
};
use crate::workbench::app::SchematicEditAuthority;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) enum DrawingSheetSection {
    #[default]
    Format,
    Margins,
    Frame,
    TitleBlock,
    Scope,
}

impl DrawingSheetSection {
    pub(crate) const ALL: [Self; 5] = [
        Self::Format,
        Self::Margins,
        Self::Frame,
        Self::TitleBlock,
        Self::Scope,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Format => "Size & orientation",
            Self::Margins => "Margins & bleed",
            Self::Frame => "Frame & zones",
            Self::TitleBlock => "Title block",
            Self::Scope => "Scope & inheritance",
        }
    }

    pub(crate) const fn number(self) -> &'static str {
        match self {
            Self::Format => "01",
            Self::Margins => "02",
            Self::Frame => "03",
            Self::TitleBlock => "04",
            Self::Scope => "05",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum SheetDisplayUnit {
    #[default]
    Millimetres,
    Centimetres,
    Inches,
}

impl SheetDisplayUnit {
    pub(crate) const ALL: [Self; 3] = [Self::Millimetres, Self::Centimetres, Self::Inches];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Millimetres => "millimetres (mm)",
            Self::Centimetres => "centimetres (cm)",
            Self::Inches => "inches (in)",
        }
    }

    pub(crate) const fn suffix(self) -> &'static str {
        match self {
            Self::Millimetres => "mm",
            Self::Centimetres => "cm",
            Self::Inches => "in",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SheetSizeChoice {
    Standard(DrawingSheetStandard),
    CapturedPreset {
        id: String,
        name: String,
        scope: DrawingSheetPresetScope,
    },
    Custom,
}

impl Default for SheetSizeChoice {
    fn default() -> Self {
        Self::Standard(DrawingSheetStandard::IsoA4)
    }
}

impl SheetSizeChoice {
    pub(crate) fn label(&self) -> &str {
        match self {
            Self::Standard(standard) => standard.label(),
            Self::CapturedPreset { name, .. } => name,
            Self::Custom => "One-off custom size…",
        }
    }

    pub(crate) const fn standard(&self) -> Option<DrawingSheetStandard> {
        match self {
            Self::Standard(standard) => Some(*standard),
            Self::CapturedPreset { .. } | Self::Custom => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum BorderTemplateChoice {
    #[default]
    Standard,
    None,
}

impl BorderTemplateChoice {
    pub(crate) const ALL: [Self; 2] = [Self::Standard, Self::None];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Standard => "Standard frame",
            Self::None => "No border",
        }
    }

    pub(crate) const fn summary(self) -> &'static str {
        match self {
            Self::Standard => "The frame of the sheet's standard, ISO 5457 or ASME Y14.1.",
            Self::None => "Paper only; no frame, margins or zone references.",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ZoneModeChoice {
    #[default]
    Automatic,
    None,
}

impl ZoneModeChoice {
    pub(crate) const ALL: [Self; 2] = [Self::Automatic, Self::None];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Automatic => "Automatic",
            Self::None => "None",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum TitleBlockTemplateChoice {
    #[default]
    Standard,
    Asme,
    Organization,
    None,
}

impl TitleBlockTemplateChoice {
    pub(crate) const ALL: [Self; 4] = [Self::Standard, Self::Asme, Self::Organization, Self::None];

    /// The template's one name without a sheet to size it by.
    pub(crate) fn label(self) -> &'static str {
        title_template_to_model(self).label()
    }

    /// The template's name on the draft's sheet, as every other surface
    /// gives it: the ASME form with the size of the block that sheet
    /// carries.
    pub(crate) fn label_on(self, draft: &DrawingSheetDraft) -> &'static str {
        let template = title_template_to_model(self);
        match draft.validate() {
            Ok(validated) => validated.page_format.title_block_template_label(template),
            Err(_) => template.label(),
        }
    }

    /// The standard block of a frame convention's format.
    pub(crate) const fn for_convention(convention: DrawingSheetFrameConvention) -> Self {
        match convention {
            DrawingSheetFrameConvention::Iso5457 => Self::Standard,
            DrawingSheetFrameConvention::AsmeY141 => Self::Asme,
        }
    }

    pub(crate) const fn summary(self) -> &'static str {
        match self {
            Self::Standard => {
                "ISO 7200's form: people, legal owner, title, identification and date of issue."
            }
            Self::Asme => {
                "ASME Y14.1's form: approvals, name, title, size, DAI, drawing number, revision and sheet, upright in the lower right corner."
            }
            Self::Organization => "Organization-owned labels, order and mark.",
            Self::None => "The authored sheet carries no title block.",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum PageSetupScope {
    #[default]
    CurrentSheet,
    Document,
    CurrentSheetAndDefault,
}

impl PageSetupScope {
    pub(crate) const ALL: [Self; 3] = [
        Self::CurrentSheet,
        Self::Document,
        Self::CurrentSheetAndDefault,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::CurrentSheet => "This sheet",
            Self::Document => "Every sheet in this document",
            Self::CurrentSheetAndDefault => "This sheet and the project default",
        }
    }

    pub(crate) const fn detail(self) -> &'static str {
        match self {
            Self::CurrentSheet => "Write only the active sheet's drawing-sheet record.",
            Self::Document => "Write every governed sheet in the active cell/view.",
            Self::CurrentSheetAndDefault => {
                "Write this sheet and seed future sheets with the same format."
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DrawingSheetDraft {
    pub(crate) size: SheetSizeChoice,
    /// The series a custom size keeps the frame of: the one the sheet's
    /// size came from, or the last standard size chosen here.
    pub(crate) custom_started_from: Option<DrawingSheetStandardSeries>,
    pub(crate) custom_name: String,
    pub(crate) width: String,
    pub(crate) height: String,
    pub(crate) orientation: SchematicPageOrientation,
    pub(crate) display_unit: SheetDisplayUnit,
    pub(crate) save_custom_preset: bool,
    pub(crate) custom_preset_scope: DrawingSheetPresetScope,
    /// ASME Y14.1 4.1.3's binding margin. The margins are otherwise the
    /// standard's, and nothing here edits them.
    pub(crate) binding_margin: bool,
    pub(crate) bleed: String,
    pub(crate) border: BorderTemplateChoice,
    pub(crate) zone_mode: ZoneModeChoice,
    /// The zones the sheet's standard gives its size and orientation, shown
    /// beside the zone choice.
    pub(crate) zone_columns: u8,
    pub(crate) zone_rows: u8,
    pub(crate) centring_marks: bool,
    pub(crate) title_block: TitleBlockTemplateChoice,
    pub(crate) managed_title_template: Option<DrawingSheetManagedTemplateSnapshot>,
    pub(crate) title_fields:
        std::collections::BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
    pub(crate) inheritance: DrawingSheetInheritance,
    pub(crate) scope: PageSetupScope,
}

impl DrawingSheetDraft {
    pub(crate) fn from_format(format: &SchematicSheetFormat) -> Self {
        let (size, custom_name) = match &format.authored_size {
            AuthoredDrawingSheetSize::Standard { standard } => {
                (SheetSizeChoice::Standard(*standard), String::new())
            }
            AuthoredDrawingSheetSize::Custom { snapshot } => (
                snapshot
                    .preset_id
                    .as_ref()
                    .map_or(SheetSizeChoice::Custom, |id| {
                        SheetSizeChoice::CapturedPreset {
                            id: id.clone(),
                            name: snapshot.name.clone(),
                            // An authored snapshot cannot retain a personal
                            // reference. Any personal preset is copied into
                            // the project before the sheet is committed.
                            scope: DrawingSheetPresetScope::Project,
                        }
                    }),
                snapshot.name.clone(),
            ),
        };
        let (width_um, height_um) = format.oriented_dimensions_um();
        let display_unit = display_unit_from_model(format.display_unit);
        let (zone_columns, zone_rows) =
            drawing_sheet_automatic_zone_counts(&format.authored_size, format.orientation);
        Self {
            size,
            custom_started_from: format.authored_size.series_for_custom_size(),
            custom_name,
            width: format_um(width_um, display_unit),
            height: format_um(height_um, display_unit),
            orientation: format.orientation,
            display_unit,
            save_custom_preset: false,
            custom_preset_scope: DrawingSheetPresetScope::Project,
            binding_margin: format.binding_margin,
            bleed: format_um(format.bleed_um, display_unit),
            border: border_from_model(format.border),
            zone_mode: zone_mode_from_model(format.zones.mode),
            zone_columns,
            zone_rows,
            centring_marks: format.marks.centring,
            title_block: title_template_from_model(format.title_block.template),
            managed_title_template: format.title_block.managed_template.clone(),
            title_fields: format.title_block.fields.clone(),
            inheritance: format.inheritance,
            scope: PageSetupScope::CurrentSheet,
        }
    }

    pub(crate) fn set_display_unit(&mut self, next: SheetDisplayUnit) -> Result<(), String> {
        if self.display_unit == next {
            return Ok(());
        }
        let current = self.display_unit;
        let physical = [
            parse_length_um(&self.width, current)?,
            parse_length_um(&self.height, current)?,
            parse_length_um(&self.bleed, current)?,
        ];
        self.width = format_um(physical[0], next);
        self.height = format_um(physical[1], next);
        self.bleed = format_um(physical[2], next);
        self.display_unit = next;
        Ok(())
    }

    pub(crate) fn set_orientation(&mut self, orientation: SchematicPageOrientation) {
        if self.orientation == orientation {
            return;
        }
        self.orientation = orientation;
        std::mem::swap(&mut self.width, &mut self.height);
        self.refresh_automatic_zone_counts();
    }

    pub(crate) fn apply_size_choice(&mut self, choice: SheetSizeChoice) {
        let previous = self.size.standard();
        self.size = choice.clone();
        let Some(standard) = choice.standard() else {
            if choice == SheetSizeChoice::Custom {
                self.inheritance = DrawingSheetInheritance::Explicit;
            }
            self.refresh_automatic_zone_counts();
            return;
        };
        let previous_convention = self.frame_convention_of(previous);
        self.custom_started_from =
            AuthoredDrawingSheetSize::Standard { standard }.series_for_custom_size();
        self.save_custom_preset = false;
        self.custom_preset_scope = DrawingSheetPresetScope::Project;
        let format = SchematicSheetFormat::from_standard(standard, self.orientation);
        let (width_um, height_um) = format.oriented_dimensions_um();
        self.width = format_um(width_um, self.display_unit);
        self.height = format_um(height_um, self.display_unit);
        // Only an ASME Y14.1 format is bound.
        self.binding_margin &= format.can_bind();
        // Marks the author left at the old size's default follow the new
        // size's; a choice they made stays as it is.
        let border = border_to_model(self.border);
        if self.centring_marks == previous_convention.default_marks(border).centring {
            self.centring_marks = format.frame_convention().default_marks(border).centring;
        }
        // A block left at the old format's standard one follows the new
        // format's, and the ASME form leaves a sheet that is no longer in its
        // format; a block the author chose otherwise stays.
        let convention = format.frame_convention();
        if self.title_block == TitleBlockTemplateChoice::for_convention(previous_convention)
            || (self.title_block == TitleBlockTemplateChoice::Asme
                && convention != DrawingSheetFrameConvention::AsmeY141)
        {
            self.set_title_block_template(TitleBlockTemplateChoice::for_convention(convention));
        }
        self.refresh_automatic_zone_counts();
    }

    /// The frame convention a size choice gives the sheet.
    fn frame_convention_of(
        &self,
        standard: Option<DrawingSheetStandard>,
    ) -> DrawingSheetFrameConvention {
        match standard {
            Some(standard) => DrawingSheetFrameConvention::for_series(standard.series()),
            None => self
                .custom_started_from
                .map_or(DrawingSheetFrameConvention::Iso5457, |series| {
                    DrawingSheetFrameConvention::for_series(series)
                }),
        }
    }

    /// The drawing-format standard the draft's sheet is framed to.
    pub(crate) fn frame_convention(&self) -> DrawingSheetFrameConvention {
        self.frame_convention_of(self.size.standard())
    }

    /// Whether the frame takes its standard's own placement: the standard
    /// border.
    pub(crate) fn standard_frame(&self) -> bool {
        self.border == BorderTemplateChoice::Standard
    }

    /// The size the draft describes, as the model would store it.
    fn authored_size(&self, portrait: [u64; 2]) -> AuthoredDrawingSheetSize {
        match &self.size {
            SheetSizeChoice::Standard(standard) => AuthoredDrawingSheetSize::Standard {
                standard: *standard,
            },
            SheetSizeChoice::CapturedPreset { id, name, .. } => AuthoredDrawingSheetSize::Custom {
                snapshot: CustomDrawingSheetSnapshot {
                    preset_id: Some(id.clone()),
                    name: name.clone(),
                    portrait_width_um: portrait[0],
                    portrait_height_um: portrait[1],
                    source_preset_unavailable: false,
                    started_from: self.custom_started_from,
                },
            },
            SheetSizeChoice::Custom => AuthoredDrawingSheetSize::Custom {
                snapshot: CustomDrawingSheetSnapshot {
                    preset_id: None,
                    name: if self.custom_name.trim().is_empty() {
                        "One-off custom size".to_owned()
                    } else {
                        self.custom_name.trim().to_owned()
                    },
                    portrait_width_um: portrait[0],
                    portrait_height_um: portrait[1],
                    source_preset_unavailable: false,
                    started_from: self.custom_started_from,
                },
            },
        }
    }

    /// Bring the zone counts shown up to the sheet's size, orientation and
    /// standard.
    fn refresh_automatic_zone_counts(&mut self) {
        if let (Ok(width_um), Ok(height_um)) = (
            parse_length_um(&self.width, self.display_unit),
            parse_length_um(&self.height, self.display_unit),
        ) {
            let mut portrait = [width_um, height_um];
            portrait.sort_unstable();
            (self.zone_columns, self.zone_rows) = drawing_sheet_automatic_zone_counts(
                &self.authored_size(portrait),
                self.orientation,
            );
        }
    }

    /// A one-line note on an orientation ISO 5457 does not use.
    pub(crate) fn orientation_advisory(&self) -> Option<&'static str> {
        let landscape_only = matches!(
            self.size.standard(),
            Some(
                DrawingSheetStandard::IsoA0
                    | DrawingSheetStandard::IsoA1
                    | DrawingSheetStandard::IsoA2
                    | DrawingSheetStandard::IsoA3
            )
        );
        (landscape_only && self.orientation == SchematicPageOrientation::Portrait)
            .then_some("ISO 5457 uses A0–A3 in landscape.")
    }

    /// The margins the draft's sheet takes from its standard, or none when it
    /// has no border.
    pub(crate) fn margins(&self) -> DrawingSheetMargins {
        if self.border == BorderTemplateChoice::None {
            return DrawingSheetMargins::zero();
        }
        let convention = self.frame_convention();
        convention
            .margins(self.binding_margin && convention == DrawingSheetFrameConvention::AsmeY141)
    }

    /// Re-apply the managed presentation authority.
    ///
    /// Selecting a preset or following a default may replace the rest of the
    /// draft, but it must never replace an organization-owned title-block
    /// definition captured from the authored sheet.
    pub(crate) fn enforce_managed_authority(&mut self, authority: &Self) {
        if authority.title_block == TitleBlockTemplateChoice::Organization {
            self.title_block = authority.title_block;
        }
    }

    pub(crate) fn set_border_template(&mut self, border: BorderTemplateChoice) {
        self.border = border;
        self.zone_mode = match border {
            BorderTemplateChoice::Standard => ZoneModeChoice::Automatic,
            BorderTemplateChoice::None => ZoneModeChoice::None,
        };
        self.binding_margin &= border == BorderTemplateChoice::Standard;
        self.centring_marks = self
            .frame_convention()
            .default_marks(border_to_model(border))
            .centring;
        self.refresh_automatic_zone_counts();
    }

    /// What the centring-marks option draws on this frame, and a note on it.
    pub(crate) fn centring_marks_wording(&self) -> (&'static str, &'static str) {
        match self.frame_convention() {
            DrawingSheetFrameConvention::Iso5457 => {
                ("Centring marks", "ISO 5457 marks on the axes")
            }
            DrawingSheetFrameConvention::AsmeY141 => {
                ("Microfilm arrows", "ASME Y14.1 Fig 5-5, optional")
            }
        }
    }

    pub(crate) fn set_title_block_template(&mut self, template: TitleBlockTemplateChoice) {
        self.title_block = template;
    }

    /// The title blocks Page Setup offers the draft's sheet: ISO 7200's
    /// form on any sheet, ASME Y14.1's on a sheet in its format or already
    /// on it, and no block. The organization's block is assigned, never
    /// chosen.
    pub(crate) fn offered_title_blocks(
        &self,
        baseline: TitleBlockTemplateChoice,
    ) -> Vec<TitleBlockTemplateChoice> {
        let asme_format = self.frame_convention() == DrawingSheetFrameConvention::AsmeY141;
        TitleBlockTemplateChoice::ALL
            .into_iter()
            .filter(|choice| match choice {
                TitleBlockTemplateChoice::Standard | TitleBlockTemplateChoice::None => true,
                TitleBlockTemplateChoice::Asme => {
                    asme_format || baseline == TitleBlockTemplateChoice::Asme
                }
                TitleBlockTemplateChoice::Organization => false,
            })
            .collect()
    }

    pub(crate) fn validate(&self) -> Result<ValidatedDrawingSheetDraft, Vec<FieldProblem>> {
        let mut problems = Vec::new();
        let parse = |field: &'static str, value: &str, problems: &mut Vec<FieldProblem>| {
            parse_length_um(value, self.display_unit)
                .map_err(|message| problems.push(FieldProblem { field, message }))
                .ok()
        };
        let width_um = parse("width", &self.width, &mut problems);
        let height_um = parse("height", &self.height, &mut problems);
        let bleed_um = parse("bleed", &self.bleed, &mut problems);

        if let (Some(width), Some(height)) = (width_um, height_um) {
            if !(50_800..=2_540_000).contains(&width) || !(50_800..=2_540_000).contains(&height) {
                problems.push(FieldProblem {
                    field: "width",
                    message: "Sheet edges must be between 50.8 mm and 2540 mm.".to_owned(),
                });
            } else {
                let longer = width.max(height);
                let shorter = width.min(height);
                if longer > shorter.saturating_mul(20) {
                    problems.push(FieldProblem {
                        field: "width",
                        message: "The sheet aspect ratio cannot exceed 20:1.".to_owned(),
                    });
                }
            }
            let margins = self.margins();
            let horizontal_reserved = margins.left_um + margins.right_um;
            let vertical_reserved = margins.top_um + margins.bottom_um;
            if width.saturating_sub(horizontal_reserved)
                < crate::state::DRAWING_SHEET_MIN_DRAWING_EDGE_UM
                || height.saturating_sub(vertical_reserved)
                    < crate::state::DRAWING_SHEET_MIN_DRAWING_EDGE_UM
            {
                problems.push(FieldProblem {
                    field: "width",
                    message: "The sheet must leave at least 25 mm of drawing area inside its standard's margins on each edge.".to_owned(),
                });
            }
        }
        if self
            .title_fields
            .get(&DrawingSheetTitleFieldId::SheetTitle)
            .is_none_or(|field| !field.visible || field.value.trim().is_empty())
        {
            problems.push(FieldProblem {
                field: "title-fields",
                message: format!(
                    "{} is required and cannot be empty.",
                    DrawingSheetTitleFieldId::SheetTitle.display_label()
                ),
            });
        }
        if self.size == SheetSizeChoice::Custom && self.save_custom_preset {
            let name = self.custom_name.trim();
            if name.is_empty() {
                problems.push(FieldProblem {
                    field: "custom-name",
                    message: "Enter a durable custom-size name.".to_owned(),
                });
            } else if name.chars().count() > 48
                || name.chars().any(|character| {
                    matches!(
                        character,
                        '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
                    )
                })
            {
                problems.push(FieldProblem {
                    field: "custom-name",
                    message: "Use at most 48 characters and no filesystem-reserved characters."
                        .to_owned(),
                });
            }
        }
        if bleed_um.is_some_and(|bleed| bleed > 25_400) {
            problems.push(FieldProblem {
                field: "bleed",
                message: "Export bleed must be between 0 and 25.4 mm.".to_owned(),
            });
        }

        if !problems.is_empty() {
            return Err(problems);
        }
        let width_um = width_um.expect("validated width");
        let height_um = height_um.expect("validated height");
        let mut portrait = [width_um, height_um];
        portrait.sort_unstable();
        let authored_size = self.authored_size(portrait);
        let page_format = SchematicSheetFormat::try_from_draft(SchematicSheetFormatDraft {
            authored_size,
            orientation: self.orientation,
            display_unit: display_unit_to_model(self.display_unit),
            binding_margin: self.binding_margin,
            bleed_um: bleed_um.expect("validated bleed"),
            border: border_to_model(self.border),
            zones: DrawingSheetZones {
                mode: zone_mode_to_model(self.zone_mode),
            },
            marks: DrawingSheetMarks {
                centring: self.centring_marks,
            },
            title_block: DrawingSheetTitleBlock {
                template: title_template_to_model(self.title_block),
                managed_template: self.managed_title_template.clone(),
                fields: self.title_fields.clone(),
            },
            inheritance: self.inheritance,
        })
        .map_err(|error| {
            let message = error.to_string();
            // The margins are the standard's, so a sheet they leave too
            // little room inside is too small.
            let field = if message.contains("drawing sheet margin")
                || message.contains("drawing sheet border")
                || message.contains("drawing sheet drawing area")
            {
                "width"
            } else {
                "drawing-sheet"
            };
            vec![FieldProblem { field, message }]
        })?;
        Ok(ValidatedDrawingSheetDraft {
            page_format,
            width_um,
            height_um,
            bleed_um: bleed_um.expect("validated bleed"),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldProblem {
    pub(crate) field: &'static str,
    pub(crate) message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ValidatedDrawingSheetDraft {
    pub(crate) page_format: SchematicSheetFormat,
    pub(crate) width_um: u64,
    pub(crate) height_um: u64,
    pub(crate) bleed_um: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct GovernedDrawingSheetAuthority {
    pub(crate) cell_view_key: String,
    pub(crate) catalog_revision: u64,
    pub(crate) sheet_id: SheetId,
    pub(crate) sheet_revision: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct DrawingSheetAuthority {
    pub(crate) edit: SchematicEditAuthority,
    pub(crate) cell_view_key: String,
    pub(crate) design_management_revision: u64,
    pub(crate) personal_preferences_digest: Option<crate::product::ContentDigest>,
    pub(crate) governed: Option<GovernedDrawingSheetAuthority>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DrawingSheetSupportRequest {
    SheetFormatManager,
    TitleBlockFields,
    Defaults,
    CustomSizes,
}

#[derive(Debug, Clone)]
pub(crate) struct DrawingSheetSetupState {
    pub(crate) open: bool,
    pub(crate) section: DrawingSheetSection,
    pub(crate) document_name: String,
    pub(crate) sheet_name: String,
    pub(crate) sheet_count: usize,
    pub(crate) writable_sheet_count: usize,
    /// The paper of every other sheet of the schematic, as each is drawn.
    pub(crate) other_sheet_sizes: Vec<AuthoredDrawingSheetSize>,
    pub(crate) managed_sheet_names: Vec<String>,
    pub(crate) available_presets: Vec<DrawingSheetPreset>,
    pub(crate) project_default: SchematicSheetFormat,
    pub(crate) authority: Option<DrawingSheetAuthority>,
    pub(crate) baseline: DrawingSheetDraft,
    pub(crate) draft: DrawingSheetDraft,
    /// Project-owned title values accepted by the nested field editor. They
    /// remain part of this transaction until Page Setup applies or discards.
    pub(crate) staged_project_title_values:
        Option<std::collections::BTreeMap<DrawingSheetTitleFieldId, String>>,
    /// Project-owned revision/date authority accepted by the nested field
    /// editor. It publishes only when this Page Setup transaction applies.
    pub(crate) staged_document_control: Option<crate::state::DrawingSheetDocumentControl>,
    /// Last geometrically valid format produced by this draft. Invalid
    /// in-progress text never blanks the preview or makes its layout jump.
    pub(crate) last_valid_format: SchematicSheetFormat,
    pub(crate) authority_error: Option<String>,
    pub(crate) pending_support: Option<DrawingSheetSupportRequest>,
    pub(crate) support_suspended: bool,
}

impl Default for DrawingSheetSetupState {
    fn default() -> Self {
        let draft = DrawingSheetDraft::from_format(&SchematicSheetFormat::default());
        Self {
            open: false,
            section: DrawingSheetSection::Format,
            document_name: "Active document".to_owned(),
            sheet_name: "Active sheet".to_owned(),
            sheet_count: 1,
            writable_sheet_count: 1,
            other_sheet_sizes: Vec::new(),
            managed_sheet_names: Vec::new(),
            available_presets: Vec::new(),
            project_default: SchematicSheetFormat::default(),
            authority: None,
            baseline: draft.clone(),
            last_valid_format: SchematicSheetFormat::default(),
            draft,
            staged_project_title_values: None,
            staged_document_control: None,
            authority_error: None,
            pending_support: None,
            support_suspended: false,
        }
    }
}

impl DrawingSheetSetupState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    /// ASME Y14.1's advice when the draft leaves an ANSI or ARCH
    /// schematic's sheets on more than one size. Applying the draft to every
    /// sheet gives them one.
    pub(crate) fn mixed_size_advisory(&self) -> Option<&'static str> {
        if self.draft.scope == PageSetupScope::Document
            || self.draft.frame_convention() != DrawingSheetFrameConvention::AsmeY141
        {
            return None;
        }
        let size = self.draft.validate().ok()?.page_format.authored_size;
        drawing_sheet_sizes_mixed(std::iter::once(&size).chain(&self.other_sheet_sizes))
            .then_some(DRAWING_SHEET_MIXED_SIZES_ADVISORY)
    }

    pub(crate) fn is_dirty(&self) -> bool {
        self.draft != self.baseline || self.staged_document_control.is_some()
    }

    pub(crate) fn restore_opened_values(&mut self) {
        self.draft.clone_from(&self.baseline);
        self.staged_project_title_values = None;
        self.staged_document_control = None;
    }
}

pub(crate) fn parse_length_um(value: &str, unit: SheetDisplayUnit) -> Result<u64, String> {
    let normalized = value.trim().replace(',', ".");
    if normalized.is_empty() {
        return Err("Enter a physical dimension.".to_owned());
    }
    if normalized.starts_with('-') {
        return Err("Physical dimensions cannot be negative.".to_owned());
    }
    let normalized = normalized.strip_prefix('+').unwrap_or(&normalized);
    let (whole, fraction) = normalized.split_once('.').unwrap_or((normalized, ""));
    if whole.is_empty()
        || !whole.bytes().all(|byte| byte.is_ascii_digit())
        || !fraction.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err("Use a decimal physical dimension.".to_owned());
    }
    let scale_um = match unit {
        SheetDisplayUnit::Millimetres => 1_000_u64,
        SheetDisplayUnit::Centimetres => 10_000_u64,
        SheetDisplayUnit::Inches => 25_400_u64,
    };
    let whole = whole
        .parse::<u64>()
        .map_err(|_| "The physical dimension is too large.".to_owned())?;
    let whole_um = whole
        .checked_mul(scale_um)
        .ok_or_else(|| "The physical dimension is too large.".to_owned())?;
    if fraction.len() > 6 {
        return Err("Use no more than six decimal places.".to_owned());
    }
    let fraction_value = if fraction.is_empty() {
        0
    } else {
        fraction
            .parse::<u64>()
            .map_err(|_| "Use a decimal physical dimension.".to_owned())?
    };
    let denominator = 10_u64.pow(fraction.len() as u32);
    let scaled = fraction_value
        .checked_mul(scale_um)
        .ok_or_else(|| "The physical dimension is too large.".to_owned())?;
    whole_um
        .checked_add((scaled + denominator / 2) / denominator)
        .ok_or_else(|| "The physical dimension is too large.".to_owned())
}

pub(crate) fn format_um(value_um: u64, unit: SheetDisplayUnit) -> String {
    let scale_um = match unit {
        SheetDisplayUnit::Millimetres => 1_000_u64,
        SheetDisplayUnit::Centimetres => 10_000_u64,
        SheetDisplayUnit::Inches => 25_400_u64,
    };
    let whole = value_um / scale_um;
    let remainder = value_um % scale_um;
    if remainder == 0 {
        return whole.to_string();
    }
    let digits: u32 = match unit {
        SheetDisplayUnit::Millimetres => 3,
        SheetDisplayUnit::Centimetres => 4,
        SheetDisplayUnit::Inches => 5,
    };
    let multiplier = 10_u64.pow(digits);
    let rounded = (remainder.saturating_mul(multiplier) + scale_um / 2) / scale_um;
    let width = digits as usize;
    format!("{whole}.{rounded:0width$}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

pub(super) fn display_unit_from_model(unit: DrawingSheetDisplayUnit) -> SheetDisplayUnit {
    match unit {
        DrawingSheetDisplayUnit::Millimetres => SheetDisplayUnit::Millimetres,
        DrawingSheetDisplayUnit::Centimetres => SheetDisplayUnit::Centimetres,
        DrawingSheetDisplayUnit::Inches => SheetDisplayUnit::Inches,
    }
}

fn display_unit_to_model(unit: SheetDisplayUnit) -> DrawingSheetDisplayUnit {
    match unit {
        SheetDisplayUnit::Millimetres => DrawingSheetDisplayUnit::Millimetres,
        SheetDisplayUnit::Centimetres => DrawingSheetDisplayUnit::Centimetres,
        SheetDisplayUnit::Inches => DrawingSheetDisplayUnit::Inches,
    }
}

fn border_from_model(value: DrawingSheetBorderTemplate) -> BorderTemplateChoice {
    match value {
        DrawingSheetBorderTemplate::Standard => BorderTemplateChoice::Standard,
        DrawingSheetBorderTemplate::None => BorderTemplateChoice::None,
    }
}

fn border_to_model(value: BorderTemplateChoice) -> DrawingSheetBorderTemplate {
    match value {
        BorderTemplateChoice::Standard => DrawingSheetBorderTemplate::Standard,
        BorderTemplateChoice::None => DrawingSheetBorderTemplate::None,
    }
}

fn zone_mode_from_model(value: DrawingSheetZoneMode) -> ZoneModeChoice {
    match value {
        DrawingSheetZoneMode::Automatic => ZoneModeChoice::Automatic,
        DrawingSheetZoneMode::None => ZoneModeChoice::None,
    }
}

fn zone_mode_to_model(value: ZoneModeChoice) -> DrawingSheetZoneMode {
    match value {
        ZoneModeChoice::Automatic => DrawingSheetZoneMode::Automatic,
        ZoneModeChoice::None => DrawingSheetZoneMode::None,
    }
}

pub(super) fn title_template_from_model(
    value: DrawingSheetTitleBlockTemplate,
) -> TitleBlockTemplateChoice {
    match value {
        DrawingSheetTitleBlockTemplate::Standard => TitleBlockTemplateChoice::Standard,
        DrawingSheetTitleBlockTemplate::AsmeY141 => TitleBlockTemplateChoice::Asme,
        DrawingSheetTitleBlockTemplate::OrganizationManaged => {
            TitleBlockTemplateChoice::Organization
        }
        DrawingSheetTitleBlockTemplate::None => TitleBlockTemplateChoice::None,
    }
}

fn title_template_to_model(value: TitleBlockTemplateChoice) -> DrawingSheetTitleBlockTemplate {
    match value {
        TitleBlockTemplateChoice::Standard => DrawingSheetTitleBlockTemplate::Standard,
        TitleBlockTemplateChoice::Asme => DrawingSheetTitleBlockTemplate::AsmeY141,
        TitleBlockTemplateChoice::Organization => {
            DrawingSheetTitleBlockTemplate::OrganizationManaged
        }
        TitleBlockTemplateChoice::None => DrawingSheetTitleBlockTemplate::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::SchematicPageSize;

    fn valid_draft() -> DrawingSheetDraft {
        let mut draft = DrawingSheetDraft::from_format(&SchematicSheetFormat::default());
        draft
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value = "Sheet 1".to_owned();
        draft
    }

    #[test]
    fn physical_dimensions_round_trip_exactly_across_display_units() {
        assert_eq!(
            parse_length_um("215.9", SheetDisplayUnit::Millimetres).unwrap(),
            215_900
        );
        assert_eq!(
            parse_length_um("8.5", SheetDisplayUnit::Inches).unwrap(),
            215_900
        );
        assert_eq!(format_um(215_900, SheetDisplayUnit::Millimetres), "215.9");
    }

    #[test]
    fn draft_validation_rejects_a_size_its_standards_margins_leave_no_room_in() {
        let mut draft = valid_draft();
        draft.size = SheetSizeChoice::Custom;
        draft.width = "52".to_owned();
        let problems = draft.validate().unwrap_err();
        assert!(problems.iter().any(|problem| problem.field == "width"));
        // Without a border a sheet has no margins, and the same size is
        // drawing area edge to edge.
        draft.set_border_template(BorderTemplateChoice::None);
        assert!(draft.validate().is_ok());
    }

    #[test]
    fn orientation_swaps_drawn_edges_without_changing_the_format_identity() {
        let mut draft = valid_draft();
        let before = (draft.width.clone(), draft.height.clone());
        draft.set_orientation(SchematicPageOrientation::Portrait);
        assert_eq!(
            (draft.width.as_str(), draft.height.as_str()),
            (before.1.as_str(), before.0.as_str())
        );
        let expected = SchematicSheetFormat::standard(
            SchematicPageSize::A4,
            SchematicPageOrientation::Portrait,
        )
        .try_update(|format| {
            format
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                .unwrap()
                .value = "Sheet 1".to_owned();
            // Turning a sheet keeps the block it asked for.
            format.title_block.template = DrawingSheetTitleBlockTemplate::Standard;
        })
        .unwrap();
        let turned = draft.validate().unwrap().page_format;
        assert_eq!(turned, expected);
        // ISO 5457 leaves A4 portrait a 180 mm drawing space, which holds ISO
        // 7200's 180 mm block upright.
        assert!(!turned.title_block_substituted());
    }

    #[test]
    fn page_setup_notes_an_iso_size_the_standard_uses_only_in_landscape() {
        let mut draft = valid_draft();
        assert_eq!(draft.orientation_advisory(), None);
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::IsoA3));
        assert_eq!(draft.orientation_advisory(), None);
        draft.set_orientation(SchematicPageOrientation::Portrait);
        assert_eq!(
            draft.orientation_advisory(),
            Some("ISO 5457 uses A0–A3 in landscape.")
        );
        // ISO 5457 Amd 1 lets A4 stand either way.
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::IsoA4));
        assert_eq!(draft.orientation_advisory(), None);
    }

    #[test]
    fn a_new_ansi_sheet_takes_table_4_1_zones_and_leaves_the_arrows_off() {
        let mut draft = valid_draft();
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiA));
        assert_eq!((draft.zone_columns, draft.zone_rows), (2, 2));
        assert!(!draft.centring_marks);
        let zones = draft.validate().unwrap().page_format.geometry().unwrap();
        assert_eq!(
            zones
                .zones
                .map(|zones| (zones.columns, zones.rows, zones.edges)),
            Some((2, 2, crate::state::DrawingSheetZoneEdges::All))
        );
    }

    #[test]
    fn a_binding_margin_is_an_asme_option_not_its_default() {
        let mut draft = valid_draft();
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiB));
        assert!(!draft.binding_margin);
        draft.binding_margin = true;
        let format = draft.validate().unwrap().page_format;
        assert_eq!(format.margins().left_um, 19_050);
        assert_eq!(format.margins().right_um, 12_700);
        assert_eq!(draft.margins(), format.margins());
        // An ISO sheet has no binding margin to keep.
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::IsoA3));
        assert!(!draft.binding_margin);
        assert_eq!(
            draft.validate().unwrap().page_format.margins().left_um,
            20_000
        );
    }

    #[test]
    fn a_custom_size_keeps_the_frame_of_the_size_it_was_started_from() {
        let mut draft = valid_draft();
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiD));
        draft.apply_size_choice(SheetSizeChoice::Custom);
        draft.width = "800".to_owned();
        assert_eq!(
            draft.validate().unwrap().page_format.frame_convention(),
            crate::state::DrawingSheetFrameConvention::AsmeY141
        );
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::IsoA3));
        draft.apply_size_choice(SheetSizeChoice::Custom);
        assert_eq!(
            draft.validate().unwrap().page_format.frame_convention(),
            crate::state::DrawingSheetFrameConvention::Iso5457
        );
    }

    #[test]
    fn the_zones_shown_follow_the_sizes_and_orientations_standard() {
        let mut draft = valid_draft();
        assert_eq!((draft.zone_columns, draft.zone_rows), (6, 4));
        draft.set_orientation(SchematicPageOrientation::Portrait);
        assert_eq!((draft.zone_columns, draft.zone_rows), (4, 6));
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiE));
        assert_eq!((draft.zone_columns, draft.zone_rows), (8, 8));
        // Zones off and on again, the counts are still the standard's.
        draft.zone_mode = ZoneModeChoice::None;
        draft.set_orientation(SchematicPageOrientation::Landscape);
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiB));
        draft.zone_mode = ZoneModeChoice::Automatic;
        assert_eq!((draft.zone_columns, draft.zone_rows), (4, 2));
    }

    #[test]
    fn border_changes_normalize_disabled_zone_and_mark_controls() {
        let mut draft = valid_draft();
        draft.set_border_template(BorderTemplateChoice::None);
        assert_eq!(draft.zone_mode, ZoneModeChoice::None);
        assert!(!draft.centring_marks);
        assert!(draft.validate().is_ok());

        draft.set_border_template(BorderTemplateChoice::Standard);
        assert_eq!(draft.zone_mode, ZoneModeChoice::Automatic);
        assert!(draft.centring_marks);
        assert!(draft.validate().is_ok());
    }

    #[test]
    fn a_managed_title_authority_survives_replacement_drafts() {
        let mut authority = valid_draft();
        authority.set_title_block_template(TitleBlockTemplateChoice::Organization);

        let mut replacement = valid_draft();
        replacement.set_border_template(BorderTemplateChoice::None);
        replacement.set_title_block_template(TitleBlockTemplateChoice::None);
        replacement
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value = "Edited title".to_owned();
        replacement.enforce_managed_authority(&authority);

        // Only the title block is managed; the border stays as edited.
        assert_eq!(replacement.border, BorderTemplateChoice::None);
        assert_eq!(
            replacement.title_block,
            TitleBlockTemplateChoice::Organization
        );
        assert_eq!(
            replacement.title_fields[&DrawingSheetTitleFieldId::SheetTitle].value,
            "Edited title"
        );
    }

    #[test]
    fn one_off_custom_size_needs_a_name_only_when_it_is_saved_as_a_preset() {
        let mut draft = valid_draft();
        draft.size = SheetSizeChoice::Custom;
        draft.custom_name.clear();
        let one_off = draft.validate().unwrap().page_format;
        assert_eq!(one_off.authored_size.label(), "One-off custom size");

        draft.save_custom_preset = true;
        let problems = draft.validate().unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.field == "custom-name")
        );
    }

    #[test]
    fn required_sheet_title_and_frame_band_are_validated_before_commit() {
        let mut draft = valid_draft();
        draft
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value
            .clear();
        assert!(
            draft
                .validate()
                .unwrap_err()
                .iter()
                .any(|problem| problem.field == "title-fields")
        );

        let mut draft = valid_draft();
        draft.width = "54".to_owned();
        draft.height = "54".to_owned();
        draft.border = BorderTemplateChoice::Standard;
        assert!(
            draft
                .validate()
                .unwrap_err()
                .iter()
                .any(|problem| problem.field == "width")
        );
    }

    #[test]
    fn validation_retains_every_field_problem_for_complete_surface_feedback() {
        let mut draft = valid_draft();
        draft.width = "not-a-length".to_owned();
        draft.bleed = "not-a-bleed".to_owned();
        draft
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value
            .clear();

        let problems = draft.validate().unwrap_err();

        assert!(problems.iter().any(|problem| problem.field == "width"));
        assert!(problems.iter().any(|problem| problem.field == "bleed"));
        assert!(
            problems
                .iter()
                .any(|problem| problem.field == "title-fields")
        );
    }

    /// ASME Y14.1 5.2 recommends one size for every sheet of a drawing:
    /// Page Setup advises, and refuses nothing, when the draft leaves an
    /// ANSI or ARCH schematic's sheets on more than one size.
    #[test]
    fn a_draft_mixing_an_asme_schematics_sizes_is_advised_against() {
        let on = |standard| {
            let mut format =
                SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape);
            format = format
                .try_update(|draft| {
                    draft
                        .title_block
                        .fields
                        .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                        .unwrap()
                        .value = "Sheet 1".to_owned();
                })
                .unwrap();
            DrawingSheetSetupState {
                draft: DrawingSheetDraft::from_format(&format),
                other_sheet_sizes: vec![AuthoredDrawingSheetSize::Standard {
                    standard: DrawingSheetStandard::AnsiD,
                }],
                ..DrawingSheetSetupState::default()
            }
        };
        let mut state = on(DrawingSheetStandard::AnsiB);
        assert_eq!(
            state.mixed_size_advisory(),
            Some(crate::state::DRAWING_SHEET_MIXED_SIZES_ADVISORY)
        );
        assert!(state.draft.validate().is_ok(), "an advisory, not a refusal");
        // Applied to every sheet, the draft gives them one size.
        state.draft.scope = PageSetupScope::Document;
        assert_eq!(state.mixed_size_advisory(), None);
        assert_eq!(on(DrawingSheetStandard::AnsiD).mixed_size_advisory(), None);
        // ISO 5457 makes no such recommendation.
        assert_eq!(on(DrawingSheetStandard::IsoA3).mixed_size_advisory(), None);
    }
}
