//! The draft of one edit to a sheet's format, and its validation.
//!
//! The draft holds the sheet's format as the sheet panel edits it: the size a
//! menu chose and the edges it gives the sheet as drawn, the frame and its
//! marks, the title block and its fields. It holds no printer, pagination,
//! fit or tiling state: Print and Export own output media. A draft becomes a
//! durable format only through [`DrawingSheetDraft::validate`], which checks
//! it whole.

use std::collections::BTreeMap;

use crate::state::{
    AuthoredDrawingSheetSize, CustomDrawingSheetSnapshot, DrawingSheetBorderTemplate,
    DrawingSheetDisplayUnit, DrawingSheetFrameConvention, DrawingSheetInheritance,
    DrawingSheetManagedTemplateSnapshot, DrawingSheetMargins, DrawingSheetMarks,
    DrawingSheetPresetScope, DrawingSheetStandard, DrawingSheetStandardSeries,
    DrawingSheetTitleBlock, DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId,
    DrawingSheetTitleFieldState, DrawingSheetZoneMode, DrawingSheetZones, RetiredExportBleed,
    SchematicPageOrientation, SchematicSheetFormat, SchematicSheetFormatDraft,
};

/// What the sheet's size is drawn from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SheetSizeChoice {
    Standard(DrawingSheetStandard),
    /// A saved custom size, the project's or the person's own.
    CapturedPreset {
        id: String,
        name: String,
        scope: DrawingSheetPresetScope,
    },
    /// A one-off size the sheet states its own edges for.
    Custom,
}

impl SheetSizeChoice {
    pub(crate) const fn standard(&self) -> Option<DrawingSheetStandard> {
        match self {
            Self::Standard(standard) => Some(*standard),
            Self::CapturedPreset { .. } | Self::Custom => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DrawingSheetDraft {
    pub(crate) size: SheetSizeChoice,
    /// The series a custom size keeps the frame of: the one the sheet's
    /// size came from, or the last standard size chosen.
    pub(crate) custom_started_from: Option<DrawingSheetStandardSeries>,
    pub(crate) custom_name: String,
    /// The sheet's edges as it is drawn, in its orientation.
    pub(crate) width_um: u64,
    pub(crate) height_um: u64,
    pub(crate) orientation: SchematicPageOrientation,
    pub(crate) display_unit: DrawingSheetDisplayUnit,
    pub(crate) save_custom_preset: bool,
    pub(crate) custom_preset_scope: DrawingSheetPresetScope,
    /// ASME Y14.1 4.1.3's binding margin. The margins are otherwise the
    /// standard's, and nothing here edits them.
    pub(crate) binding_margin: bool,
    /// The export bleed the sheet was saved with, if it was: never edited
    /// here, carried so the sheet keeps it for its first print setup.
    pub(crate) retired_export_bleed: RetiredExportBleed,
    pub(crate) border: DrawingSheetBorderTemplate,
    pub(crate) zone_mode: DrawingSheetZoneMode,
    pub(crate) centring_marks: bool,
    pub(crate) title_block: DrawingSheetTitleBlockTemplate,
    pub(crate) managed_title_template: Option<DrawingSheetManagedTemplateSnapshot>,
    pub(crate) title_fields: BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
    pub(crate) inheritance: DrawingSheetInheritance,
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
        Self {
            size,
            custom_started_from: format.authored_size.series_for_custom_size(),
            custom_name,
            width_um,
            height_um,
            orientation: format.orientation,
            display_unit: format.display_unit,
            save_custom_preset: false,
            custom_preset_scope: DrawingSheetPresetScope::Project,
            binding_margin: format.binding_margin,
            retired_export_bleed: format.retired_export_bleed,
            border: format.border,
            zone_mode: format.zones.mode,
            centring_marks: format.marks.centring,
            title_block: format.title_block.template,
            managed_title_template: format.title_block.managed_template.clone(),
            title_fields: format.title_block.fields.clone(),
            inheritance: format.inheritance,
        }
    }

    pub(crate) fn set_orientation(&mut self, orientation: SchematicPageOrientation) {
        if self.orientation == orientation {
            return;
        }
        self.orientation = orientation;
        std::mem::swap(&mut self.width_um, &mut self.height_um);
    }

    pub(crate) fn apply_size_choice(&mut self, choice: SheetSizeChoice) {
        let previous = self.size.standard();
        self.size = choice.clone();
        let Some(standard) = choice.standard() else {
            if choice == SheetSizeChoice::Custom {
                self.inheritance = DrawingSheetInheritance::Explicit;
            }
            return;
        };
        let previous_convention = self.frame_convention_of(previous);
        self.custom_started_from =
            AuthoredDrawingSheetSize::Standard { standard }.series_for_custom_size();
        self.save_custom_preset = false;
        self.custom_preset_scope = DrawingSheetPresetScope::Project;
        let format = SchematicSheetFormat::from_standard(standard, self.orientation);
        (self.width_um, self.height_um) = format.oriented_dimensions_um();
        // Only an ASME Y14.1 format is bound.
        self.binding_margin &= format.can_bind();
        // Marks the author left at the old size's default follow the new
        // size's; a choice they made stays as it is.
        if self.centring_marks == previous_convention.default_marks(self.border).centring {
            self.centring_marks = format
                .frame_convention()
                .default_marks(self.border)
                .centring;
        }
        // A block left at the old format's standard one follows the new
        // format's, and the ASME form leaves a sheet that is no longer in its
        // format; a block the author chose otherwise stays.
        let convention = format.frame_convention();
        if self.title_block == DrawingSheetTitleBlockTemplate::for_convention(previous_convention)
            || (self.title_block == DrawingSheetTitleBlockTemplate::AsmeY141
                && convention != DrawingSheetFrameConvention::AsmeY141)
        {
            self.title_block = DrawingSheetTitleBlockTemplate::for_convention(convention);
        }
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

    /// Whether the sheet carries its standard's frame.
    pub(crate) fn standard_frame(&self) -> bool {
        self.border == DrawingSheetBorderTemplate::Standard
    }

    /// The size the draft describes, as the model would store it.
    fn authored_size(&self, portrait: [u64; 2]) -> AuthoredDrawingSheetSize {
        let snapshot = |preset_id: Option<String>, name: String| CustomDrawingSheetSnapshot {
            preset_id,
            name,
            portrait_width_um: portrait[0],
            portrait_height_um: portrait[1],
            source_preset_unavailable: false,
            started_from: self.custom_started_from,
        };
        match &self.size {
            SheetSizeChoice::Standard(standard) => AuthoredDrawingSheetSize::Standard {
                standard: *standard,
            },
            SheetSizeChoice::CapturedPreset { id, name, .. } => AuthoredDrawingSheetSize::Custom {
                snapshot: snapshot(Some(id.clone()), name.clone()),
            },
            SheetSizeChoice::Custom => AuthoredDrawingSheetSize::Custom {
                snapshot: snapshot(
                    None,
                    if self.custom_name.trim().is_empty() {
                        ONE_OFF_CUSTOM_SIZE.to_owned()
                    } else {
                        self.custom_name.trim().to_owned()
                    },
                ),
            },
        }
    }

    /// The margins the draft's sheet takes from its standard, or none when it
    /// has no frame.
    fn margins(&self) -> DrawingSheetMargins {
        if !self.standard_frame() {
            return DrawingSheetMargins::zero();
        }
        let convention = self.frame_convention();
        convention
            .margins(self.binding_margin && convention == DrawingSheetFrameConvention::AsmeY141)
    }

    /// Re-apply the managed presentation authority.
    ///
    /// Taking a preset or following a default may replace the rest of the
    /// draft, but it must never replace an organization-owned title-block
    /// definition captured from the authored sheet.
    pub(crate) fn enforce_managed_authority(&mut self, authority: &Self) {
        if authority.title_block == DrawingSheetTitleBlockTemplate::OrganizationManaged {
            self.title_block = authority.title_block;
        }
    }

    /// The frame of the sheet's standard, or none, with the zones and marks
    /// each starts with. A sheet without a frame is not bound.
    pub(crate) fn set_border_template(&mut self, border: DrawingSheetBorderTemplate) {
        self.border = border;
        self.zone_mode = match border {
            DrawingSheetBorderTemplate::Standard => DrawingSheetZoneMode::Automatic,
            DrawingSheetBorderTemplate::None => DrawingSheetZoneMode::None,
        };
        self.binding_margin &= border == DrawingSheetBorderTemplate::Standard;
        self.centring_marks = self.frame_convention().default_marks(border).centring;
    }

    /// The format the draft describes, checked whole, or every reason it is
    /// refused, in one sentence after another.
    pub(crate) fn validate(&self) -> Result<SchematicSheetFormat, String> {
        let mut problems = Vec::new();
        let (width, height) = (self.width_um, self.height_um);
        if !(50_800..=2_540_000).contains(&width) || !(50_800..=2_540_000).contains(&height) {
            problems.push("Sheet edges must be between 50.8 mm and 2540 mm.".to_owned());
        } else if width.max(height) > width.min(height).saturating_mul(20) {
            problems.push("The sheet aspect ratio cannot exceed 20:1.".to_owned());
        }
        let margins = self.margins();
        if width.saturating_sub(margins.left_um + margins.right_um)
            < crate::state::DRAWING_SHEET_MIN_DRAWING_EDGE_UM
            || height.saturating_sub(margins.top_um + margins.bottom_um)
                < crate::state::DRAWING_SHEET_MIN_DRAWING_EDGE_UM
        {
            problems.push(
                "The sheet must leave at least 25 mm of drawing area inside its standard's \
                 margins on each edge."
                    .to_owned(),
            );
        }
        if self
            .title_fields
            .get(&DrawingSheetTitleFieldId::SheetTitle)
            .is_none_or(|field| !field.visible || field.value.trim().is_empty())
        {
            problems.push(format!(
                "{} is required and cannot be empty.",
                DrawingSheetTitleFieldId::SheetTitle.display_label()
            ));
        }
        if self.size == SheetSizeChoice::Custom && self.save_custom_preset {
            let name = self.custom_name.trim();
            if name.is_empty() || name == ONE_OFF_CUSTOM_SIZE {
                problems.push("Name the custom size before saving it.".to_owned());
            } else if name.chars().count() > 48
                || name.chars().any(|character| {
                    matches!(
                        character,
                        '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
                    )
                })
            {
                problems.push(
                    "Use at most 48 characters and no filesystem-reserved characters.".to_owned(),
                );
            }
        }
        if !problems.is_empty() {
            return Err(problems.join(" "));
        }
        let mut portrait = [width, height];
        portrait.sort_unstable();
        SchematicSheetFormat::try_from_draft(SchematicSheetFormatDraft {
            authored_size: self.authored_size(portrait),
            orientation: self.orientation,
            display_unit: self.display_unit,
            binding_margin: self.binding_margin,
            border: self.border,
            zones: DrawingSheetZones {
                mode: self.zone_mode,
            },
            marks: DrawingSheetMarks {
                centring: self.centring_marks,
            },
            title_block: DrawingSheetTitleBlock {
                template: self.title_block,
                managed_template: self.managed_title_template.clone(),
                fields: self.title_fields.clone(),
            },
            inheritance: self.inheritance,
            retired_export_bleed: self.retired_export_bleed,
        })
        .map_err(|error| error.to_string())
    }
}

/// What a one-off custom size is called until it is named.
const ONE_OFF_CUSTOM_SIZE: &str = "One-off custom size";

/// A one-line note when `size` stands in an orientation its standard does
/// not use: ISO 5457 draws A0 to A3 in landscape only.
pub(crate) fn standard_orientation_advisory(
    size: &AuthoredDrawingSheetSize,
    orientation: SchematicPageOrientation,
) -> Option<&'static str> {
    let landscape_only = matches!(
        size,
        AuthoredDrawingSheetSize::Standard {
            standard: DrawingSheetStandard::IsoA0
                | DrawingSheetStandard::IsoA1
                | DrawingSheetStandard::IsoA2
                | DrawingSheetStandard::IsoA3
        }
    );
    (landscape_only && orientation == SchematicPageOrientation::Portrait)
        .then_some("ISO 5457 uses A0–A3 in landscape.")
}

/// The size menu's label for `standard` on the sheet `format` describes: its
/// designation and oriented size in that sheet's unit. Every surface offering
/// the same choice names it exactly so.
pub(crate) fn standard_size_choice_label(
    standard: DrawingSheetStandard,
    format: &SchematicSheetFormat,
) -> String {
    format_choice_label(
        standard.label(),
        standard.portrait_dimensions_um(),
        format.orientation,
        format.display_unit,
    )
}

/// The size menu's label for the size `format` already has, custom sizes
/// included.
pub(crate) fn current_size_choice_label(format: &SchematicSheetFormat) -> String {
    match &format.authored_size {
        AuthoredDrawingSheetSize::Standard { standard } => {
            standard_size_choice_label(*standard, format)
        }
        AuthoredDrawingSheetSize::Custom { snapshot } => format_choice_label(
            &snapshot.name,
            format.portrait_dimensions_um(),
            format.orientation,
            format.display_unit,
        ),
    }
}

/// `name` with the oriented size its portrait edges make, in `unit`.
fn format_choice_label(
    name: &str,
    portrait: (u64, u64),
    orientation: SchematicPageOrientation,
    unit: DrawingSheetDisplayUnit,
) -> String {
    let (width, height) = match orientation {
        SchematicPageOrientation::Landscape => (portrait.1, portrait.0),
        SchematicPageOrientation::Portrait => portrait,
    };
    format!(
        "{name} · {} × {} {}",
        format_sheet_length(width, unit),
        format_sheet_length(height, unit),
        sheet_unit_suffix(unit)
    )
}

/// The symbol a sheet's lengths are written with in `unit`.
pub(crate) const fn sheet_unit_suffix(unit: DrawingSheetDisplayUnit) -> &'static str {
    match unit {
        DrawingSheetDisplayUnit::Millimetres => "mm",
        DrawingSheetDisplayUnit::Centimetres => "cm",
        DrawingSheetDisplayUnit::Inches => "in",
    }
}

/// How many micrometres one `unit` is, and the decimals that state a length
/// in it to the micrometre.
const fn unit_scale(unit: DrawingSheetDisplayUnit) -> (u64, u32) {
    match unit {
        DrawingSheetDisplayUnit::Millimetres => (1_000, 3),
        DrawingSheetDisplayUnit::Centimetres => (10_000, 4),
        DrawingSheetDisplayUnit::Inches => (25_400, 5),
    }
}

/// A length typed in the sheet's `unit`, in micrometres.
pub(crate) fn parse_sheet_length(
    value: &str,
    unit: DrawingSheetDisplayUnit,
) -> Result<u64, String> {
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
    let (scale_um, _) = unit_scale(unit);
    let whole_um = whole
        .parse::<u64>()
        .ok()
        .and_then(|whole| whole.checked_mul(scale_um))
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

/// `value_um` written in the sheet's `unit`, to the micrometre it is stored
/// to and no further.
pub(crate) fn format_sheet_length(value_um: u64, unit: DrawingSheetDisplayUnit) -> String {
    let (scale_um, digits) = unit_scale(unit);
    let whole = value_um / scale_um;
    let remainder = value_um % scale_um;
    if remainder == 0 {
        return whole.to_string();
    }
    let multiplier = 10_u64.pow(digits);
    let rounded = (remainder.saturating_mul(multiplier) + scale_um / 2) / scale_um;
    let width = digits as usize;
    format!("{whole}.{rounded:0width$}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
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
            parse_sheet_length("215.9", DrawingSheetDisplayUnit::Millimetres).unwrap(),
            215_900
        );
        assert_eq!(
            parse_sheet_length("8.5", DrawingSheetDisplayUnit::Inches).unwrap(),
            215_900
        );
        assert_eq!(
            format_sheet_length(215_900, DrawingSheetDisplayUnit::Millimetres),
            "215.9"
        );
        assert_eq!(
            format_sheet_length(215_900, DrawingSheetDisplayUnit::Inches),
            "8.5"
        );
    }

    #[test]
    fn draft_validation_rejects_a_size_its_standards_margins_leave_no_room_in() {
        let mut draft = valid_draft();
        draft.size = SheetSizeChoice::Custom;
        draft.width_um = 52_000;
        let refused = draft.validate().unwrap_err();
        assert!(refused.contains("25 mm of drawing area"), "{refused}");
        // Without a frame a sheet has no margins, and the same size is
        // drawing area edge to edge.
        draft.set_border_template(DrawingSheetBorderTemplate::None);
        assert!(draft.validate().is_ok());
    }

    #[test]
    fn orientation_swaps_drawn_edges_without_changing_the_format_identity() {
        let mut draft = valid_draft();
        let before = (draft.width_um, draft.height_um);
        draft.set_orientation(SchematicPageOrientation::Portrait);
        assert_eq!((draft.width_um, draft.height_um), (before.1, before.0));
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
        let turned = draft.validate().unwrap();
        assert_eq!(turned, expected);
        // ISO 5457 leaves A4 portrait a 180 mm drawing space, which holds ISO
        // 7200's 180 mm block upright.
        assert!(!turned.title_block_substituted());
    }

    #[test]
    fn iso_5457_is_noted_for_a_size_it_uses_only_in_landscape() {
        let size = |standard| AuthoredDrawingSheetSize::Standard { standard };
        let portrait = SchematicPageOrientation::Portrait;
        assert_eq!(
            standard_orientation_advisory(&size(DrawingSheetStandard::IsoA3), portrait),
            Some("ISO 5457 uses A0–A3 in landscape.")
        );
        assert_eq!(
            standard_orientation_advisory(
                &size(DrawingSheetStandard::IsoA3),
                SchematicPageOrientation::Landscape
            ),
            None
        );
        // ISO 5457 Amd 1 lets A4 stand either way.
        assert_eq!(
            standard_orientation_advisory(&size(DrawingSheetStandard::IsoA4), portrait),
            None
        );
    }

    #[test]
    fn a_new_ansi_sheet_takes_table_4_1_zones_and_leaves_the_arrows_off() {
        let mut draft = valid_draft();
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiA));
        assert!(!draft.centring_marks);
        let zones = draft.validate().unwrap().geometry().unwrap();
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
        let format = draft.validate().unwrap();
        assert_eq!(format.margins().left_um, 19_050);
        assert_eq!(format.margins().right_um, 12_700);
        assert_eq!(draft.margins(), format.margins());
        // An ISO sheet has no binding margin to keep.
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::IsoA3));
        assert!(!draft.binding_margin);
        assert_eq!(draft.validate().unwrap().margins().left_um, 20_000);
    }

    #[test]
    fn a_custom_size_keeps_the_frame_of_the_size_it_was_started_from() {
        let mut draft = valid_draft();
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiD));
        draft.apply_size_choice(SheetSizeChoice::Custom);
        draft.width_um = 800_000;
        assert_eq!(
            draft.validate().unwrap().frame_convention(),
            DrawingSheetFrameConvention::AsmeY141
        );
        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::IsoA3));
        draft.apply_size_choice(SheetSizeChoice::Custom);
        assert_eq!(
            draft.validate().unwrap().frame_convention(),
            DrawingSheetFrameConvention::Iso5457
        );
    }

    #[test]
    fn a_frame_change_normalizes_the_zones_and_marks_it_carries() {
        let mut draft = valid_draft();
        draft.set_border_template(DrawingSheetBorderTemplate::None);
        assert_eq!(draft.zone_mode, DrawingSheetZoneMode::None);
        assert!(!draft.centring_marks);
        assert!(draft.validate().is_ok());

        draft.set_border_template(DrawingSheetBorderTemplate::Standard);
        assert_eq!(draft.zone_mode, DrawingSheetZoneMode::Automatic);
        assert!(draft.centring_marks);
        assert!(draft.validate().is_ok());
    }

    #[test]
    fn a_managed_title_authority_survives_replacement_drafts() {
        let mut authority = valid_draft();
        authority.title_block = DrawingSheetTitleBlockTemplate::OrganizationManaged;

        let mut replacement = valid_draft();
        replacement.set_border_template(DrawingSheetBorderTemplate::None);
        replacement.title_block = DrawingSheetTitleBlockTemplate::None;
        replacement
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value = "Edited title".to_owned();
        replacement.enforce_managed_authority(&authority);

        // Only the title block is managed; the frame stays as edited.
        assert_eq!(replacement.border, DrawingSheetBorderTemplate::None);
        assert_eq!(
            replacement.title_block,
            DrawingSheetTitleBlockTemplate::OrganizationManaged
        );
        assert_eq!(
            replacement.title_fields[&DrawingSheetTitleFieldId::SheetTitle].value,
            "Edited title"
        );
    }

    #[test]
    fn a_one_off_custom_size_needs_a_name_only_when_it_is_saved() {
        let mut draft = valid_draft();
        draft.size = SheetSizeChoice::Custom;
        draft.custom_name.clear();
        let one_off = draft.validate().unwrap();
        assert_eq!(one_off.authored_size.label(), "One-off custom size");

        draft.save_custom_preset = true;
        assert_eq!(
            draft.validate().unwrap_err(),
            "Name the custom size before saving it."
        );
        draft.custom_name = "Lab/panel".to_owned();
        assert!(
            draft
                .validate()
                .unwrap_err()
                .contains("no filesystem-reserved characters")
        );
        draft.custom_name = "Lab panel".to_owned();
        assert!(draft.validate().is_ok());
    }

    #[test]
    fn validation_states_every_reason_a_draft_is_refused() {
        let mut draft = valid_draft();
        draft.width_um = 10_000;
        draft
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value
            .clear();

        let refused = draft.validate().unwrap_err();

        assert!(refused.contains("between 50.8 mm and 2540 mm"), "{refused}");
        assert!(refused.contains("Title is required"), "{refused}");
    }
}
