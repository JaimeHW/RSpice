//! Runtime draft and validation for the authored drawing-sheet studio.
//!
//! The draft deliberately mirrors the mockup's permanent sheet vocabulary.
//! It contains no printer, pagination, fit, or tiling state. Conversion to the
//! durable design-management model is isolated in `commit.rs` so additions to
//! that model cannot leak output-media concepts back into this workflow.

use crate::state::{
    AuthoredDrawingSheetSize, CustomDrawingSheetSnapshot, DrawingSheetBorderTemplate,
    DrawingSheetDisplayUnit, DrawingSheetInheritance, DrawingSheetManagedTemplateSnapshot,
    DrawingSheetMargins, DrawingSheetMarks, DrawingSheetPreset, DrawingSheetPresetScope,
    DrawingSheetScale, DrawingSheetStandard, DrawingSheetTitleBlock, DrawingSheetTitleBlockAnchor,
    DrawingSheetTitleBlockRotation, DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId,
    DrawingSheetTitleFieldState, DrawingSheetZoneEdges, DrawingSheetZoneLabels,
    DrawingSheetZoneMode, DrawingSheetZones, SchematicPageOrientation, SchematicSheetFormat,
    SchematicSheetFormatDraft, SheetId,
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
    Plain,
    None,
    Organization,
}

impl BorderTemplateChoice {
    pub(crate) const ALL: [Self; 4] = [Self::Standard, Self::Plain, Self::None, Self::Organization];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Standard => "Standard border with zones",
            Self::Plain => "Plain border",
            Self::None => "No border",
            Self::Organization => "Organization border · managed",
        }
    }

    pub(crate) const fn summary(self) -> &'static str {
        match self {
            Self::Standard => "Ruled frame, zone band and corner references.",
            Self::Plain => "Ruled frame only; reviews quote sheet coordinates.",
            Self::None => "Paper and margins only; zone references are unavailable.",
            Self::Organization => "Managed frame with registration and folding marks.",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ZoneModeChoice {
    #[default]
    Automatic,
    Custom,
    None,
}

impl ZoneModeChoice {
    pub(crate) const ALL: [Self; 3] = [Self::Automatic, Self::Custom, Self::None];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Automatic => "Automatic",
            Self::Custom => "Custom",
            Self::None => "None",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ZoneEdgesChoice {
    #[default]
    All,
    TopLeft,
    BottomRight,
}

impl ZoneEdgesChoice {
    pub(crate) const ALL: [Self; 3] = [Self::All, Self::TopLeft, Self::BottomRight];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::All => "All four edges",
            Self::TopLeft => "Top and left",
            Self::BottomRight => "Bottom and right",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ZoneLabelsChoice {
    #[default]
    AlphaNumeric,
    NumericAlpha,
    Coordinates,
}

impl ZoneLabelsChoice {
    pub(crate) const ALL: [Self; 3] = [Self::AlphaNumeric, Self::NumericAlpha, Self::Coordinates];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::AlphaNumeric => "Rows A…Y · columns 1…24",
            Self::NumericAlpha => "Rows 1…24 · columns A…Y",
            Self::Coordinates => "Coordinates only",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum TitleBlockTemplateChoice {
    #[default]
    Compact,
    Standard,
    Wide,
    Organization,
    None,
}

impl TitleBlockTemplateChoice {
    pub(crate) const ALL: [Self; 5] = [
        Self::Compact,
        Self::Standard,
        Self::Wide,
        Self::Organization,
        Self::None,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Compact => "RSpice compact",
            Self::Standard => "RSpice standard",
            Self::Wide => "RSpice wide",
            Self::Organization => "Organization block · managed",
            Self::None => "No title block",
        }
    }

    pub(crate) const fn summary(self) -> &'static str {
        match self {
            Self::Compact => "Identity, revision and page; suited to A4 and ANSI A.",
            Self::Standard => "Full engineering identity, responsibility, date and scale.",
            Self::Wide => "Standard block plus organization and classification bands.",
            Self::Organization => "Organization-owned labels, order and mark.",
            Self::None => "The authored sheet carries no title block.",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum TitleBlockAnchorChoice {
    #[default]
    BottomRight,
    BottomLeft,
    BottomStrip,
    TopRight,
}

impl TitleBlockAnchorChoice {
    pub(crate) const ALL: [Self; 4] = [
        Self::BottomRight,
        Self::BottomLeft,
        Self::BottomStrip,
        Self::TopRight,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::BottomRight => "Bottom right",
            Self::BottomLeft => "Bottom left",
            Self::BottomStrip => "Bottom strip · full width",
            Self::TopRight => "Top right",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum TitleBlockRotationChoice {
    #[default]
    Upright,
    Clockwise,
    CounterClockwise,
}

impl TitleBlockRotationChoice {
    pub(crate) const ALL: [Self; 3] = [Self::Upright, Self::Clockwise, Self::CounterClockwise];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Upright => "Upright",
            Self::Clockwise => "Rotated 90° CW",
            Self::CounterClockwise => "Rotated 90° CCW",
        }
    }

    pub(crate) const fn note(self) -> &'static str {
        match self {
            Self::Upright => "Reads from the bottom edge of the sheet.",
            Self::Clockwise => "Reads from the right edge of the sheet.",
            Self::CounterClockwise => "Reads from the left edge of the sheet.",
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
            Self::CurrentSheet => "Write only the exact active drawing-sheet record.",
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
    pub(crate) custom_name: String,
    pub(crate) width: String,
    pub(crate) height: String,
    pub(crate) orientation: SchematicPageOrientation,
    pub(crate) display_unit: SheetDisplayUnit,
    pub(crate) save_custom_preset: bool,
    pub(crate) custom_preset_scope: DrawingSheetPresetScope,
    pub(crate) margins_linked: bool,
    pub(crate) margin_top: String,
    pub(crate) margin_right: String,
    pub(crate) margin_bottom: String,
    pub(crate) margin_left: String,
    pub(crate) bleed: String,
    pub(crate) border: BorderTemplateChoice,
    pub(crate) zone_mode: ZoneModeChoice,
    pub(crate) zone_columns: u8,
    pub(crate) zone_rows: u8,
    pub(crate) zone_edges: ZoneEdgesChoice,
    pub(crate) zone_labels: ZoneLabelsChoice,
    pub(crate) registration_marks: bool,
    pub(crate) folding_marks: bool,
    pub(crate) title_block: TitleBlockTemplateChoice,
    pub(crate) title_block_anchor: TitleBlockAnchorChoice,
    pub(crate) title_block_rotation: TitleBlockRotationChoice,
    pub(crate) title_block_offset_x: String,
    pub(crate) title_block_offset_y: String,
    pub(crate) declared_scale: String,
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
        let geometry = format.geometry().ok();
        let (zone_columns, zone_rows) = geometry
            .and_then(|geometry| geometry.zones.map(|zones| (zones.columns, zones.rows)))
            .unwrap_or((4, 4));
        Self {
            size,
            custom_name,
            width: format_um(width_um, display_unit),
            height: format_um(height_um, display_unit),
            orientation: format.orientation,
            display_unit,
            save_custom_preset: false,
            custom_preset_scope: DrawingSheetPresetScope::Project,
            margins_linked: format.margins.top_um == format.margins.right_um
                && format.margins.top_um == format.margins.bottom_um
                && format.margins.top_um == format.margins.left_um,
            margin_top: format_um(format.margins.top_um, display_unit),
            margin_right: format_um(format.margins.right_um, display_unit),
            margin_bottom: format_um(format.margins.bottom_um, display_unit),
            margin_left: format_um(format.margins.left_um, display_unit),
            bleed: format_um(format.bleed_um, display_unit),
            border: border_from_model(format.border),
            zone_mode: zone_mode_from_model(format.zones.mode),
            zone_columns,
            zone_rows,
            zone_edges: zone_edges_from_model(format.zones.edges),
            zone_labels: zone_labels_from_model(format.zones.labels),
            registration_marks: format.marks.registration,
            folding_marks: format.marks.folding,
            title_block: title_template_from_model(format.title_block.template),
            title_block_anchor: title_anchor_from_model(format.title_block.anchor),
            title_block_rotation: title_rotation_from_model(format.title_block.rotation),
            title_block_offset_x: format_signed_um(format.title_block.offset_x_um, display_unit),
            title_block_offset_y: format_signed_um(format.title_block.offset_y_um, display_unit),
            declared_scale: scale_from_model(format.title_block.scale),
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
            parse_length_um(&self.margin_top, current)?,
            parse_length_um(&self.margin_right, current)?,
            parse_length_um(&self.margin_bottom, current)?,
            parse_length_um(&self.margin_left, current)?,
            parse_length_um(&self.bleed, current)?,
        ];
        let offset_x = parse_signed_length_um(&self.title_block_offset_x, current)?;
        let offset_y = parse_signed_length_um(&self.title_block_offset_y, current)?;
        self.width = format_um(physical[0], next);
        self.height = format_um(physical[1], next);
        self.margin_top = format_um(physical[2], next);
        self.margin_right = format_um(physical[3], next);
        self.margin_bottom = format_um(physical[4], next);
        self.margin_left = format_um(physical[5], next);
        self.bleed = format_um(physical[6], next);
        self.title_block_offset_x = format_signed_um(offset_x, next);
        self.title_block_offset_y = format_signed_um(offset_y, next);
        self.display_unit = next;
        Ok(())
    }

    pub(crate) fn set_orientation(&mut self, orientation: SchematicPageOrientation) {
        if self.orientation == orientation {
            return;
        }
        self.orientation = orientation;
        std::mem::swap(&mut self.width, &mut self.height);
        // Custom zone counts are authored horizontal/vertical divisions, not
        // derived page-edge labels. Automatic mode, however, follows the
        // oriented physical edges at the canonical ~140 mm even pitch.
        if self.zone_mode == ZoneModeChoice::Automatic
            && let (Ok(width_um), Ok(height_um)) = (
                parse_length_um(&self.width, self.display_unit),
                parse_length_um(&self.height, self.display_unit),
            )
        {
            self.zone_columns = automatic_zone_divisions(width_um);
            self.zone_rows = automatic_zone_divisions(height_um);
        }
    }

    pub(crate) fn apply_size_choice(&mut self, choice: SheetSizeChoice) {
        self.size = choice.clone();
        let Some(standard) = choice.standard() else {
            if choice == SheetSizeChoice::Custom {
                self.inheritance = DrawingSheetInheritance::Explicit;
            }
            return;
        };
        self.save_custom_preset = false;
        self.custom_preset_scope = DrawingSheetPresetScope::Project;
        let format = SchematicSheetFormat::from_standard(standard, self.orientation);
        let (width_um, height_um) = format.oriented_dimensions_um();
        self.width = format_um(width_um, self.display_unit);
        self.height = format_um(height_um, self.display_unit);
        let margins = standard.default_margins();
        self.margin_top = format_um(margins.top_um, self.display_unit);
        self.margin_right = format_um(margins.right_um, self.display_unit);
        self.margin_bottom = format_um(margins.bottom_um, self.display_unit);
        self.margin_left = format_um(margins.left_um, self.display_unit);
        if self.zone_mode == ZoneModeChoice::Automatic {
            self.zone_columns = automatic_zone_divisions(width_um);
            self.zone_rows = automatic_zone_divisions(height_um);
        }
    }

    pub(crate) fn link_margins_from(&mut self, edge: MarginEdge) {
        if self.margins_linked {
            let value = match edge {
                MarginEdge::Top => self.margin_top.clone(),
                MarginEdge::Right => self.margin_right.clone(),
                MarginEdge::Bottom => self.margin_bottom.clone(),
                MarginEdge::Left => self.margin_left.clone(),
            };
            self.margin_top.clone_from(&value);
            self.margin_right.clone_from(&value);
            self.margin_bottom.clone_from(&value);
            self.margin_left = value;
        }
    }

    /// Re-apply the two independently managed presentation authorities.
    ///
    /// Selecting a preset or following a default may replace the rest of the
    /// draft, but it must never replace an organization-owned border/frame or
    /// title-block definition captured from the authored sheet.
    pub(crate) fn enforce_managed_authority(&mut self, authority: &Self) {
        if authority.border == BorderTemplateChoice::Organization {
            self.border = authority.border;
            self.zone_mode = authority.zone_mode;
        }
        if authority.title_block == TitleBlockTemplateChoice::Organization {
            self.title_block = authority.title_block;
        }
    }

    pub(crate) fn set_border_template(&mut self, border: BorderTemplateChoice) {
        self.border = border;
        match border {
            BorderTemplateChoice::Standard => {
                self.zone_mode = ZoneModeChoice::Automatic;
                self.registration_marks = true;
                self.folding_marks = false;
            }
            BorderTemplateChoice::Plain | BorderTemplateChoice::None => {
                self.zone_mode = ZoneModeChoice::None;
                self.registration_marks = false;
                self.folding_marks = false;
            }
            BorderTemplateChoice::Organization => {
                self.zone_mode = ZoneModeChoice::Automatic;
                self.registration_marks = true;
                self.folding_marks = true;
            }
        }
    }

    pub(crate) fn set_title_block_template(&mut self, template: TitleBlockTemplateChoice) {
        self.title_block = template;
        if template == TitleBlockTemplateChoice::None {
            self.title_block_offset_x = "0".to_owned();
            self.title_block_offset_y = "0".to_owned();
            self.title_block_rotation = TitleBlockRotationChoice::Upright;
        }
    }

    pub(crate) fn set_title_block_anchor(&mut self, anchor: TitleBlockAnchorChoice) {
        self.title_block_anchor = anchor;
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
        let top_um = parse("margin-top", &self.margin_top, &mut problems);
        let right_um = parse("margin-right", &self.margin_right, &mut problems);
        let bottom_um = parse("margin-bottom", &self.margin_bottom, &mut problems);
        let left_um = parse("margin-left", &self.margin_left, &mut problems);
        let bleed_um = parse("bleed", &self.bleed, &mut problems);
        let offset_x_um = parse_signed_length_um(&self.title_block_offset_x, self.display_unit)
            .map_err(|message| {
                problems.push(FieldProblem {
                    field: "title-offset-x",
                    message,
                });
            })
            .ok();
        let offset_y_um = parse_signed_length_um(&self.title_block_offset_y, self.display_unit)
            .map_err(|message| {
                problems.push(FieldProblem {
                    field: "title-offset-y",
                    message,
                });
            })
            .ok();

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
            if let (Some(top), Some(right), Some(bottom), Some(left)) =
                (top_um, right_um, bottom_um, left_um)
            {
                let border_band = border_to_model(self.border).band_width_um();
                let horizontal_reserved = left
                    .saturating_add(right)
                    .saturating_add(border_band.saturating_mul(2));
                let vertical_reserved = top
                    .saturating_add(bottom)
                    .saturating_add(border_band.saturating_mul(2));
                if width.saturating_sub(horizontal_reserved)
                    < crate::state::DRAWING_SHEET_MIN_DRAWING_EDGE_UM
                    || height.saturating_sub(vertical_reserved)
                        < crate::state::DRAWING_SHEET_MIN_DRAWING_EDGE_UM
                {
                    problems.push(FieldProblem {
                        field: "margins",
                        message: format!(
                            "Margins and the {} mm frame band must leave at least 25 mm of drawing area on each edge.",
                            border_band as f64 / 1_000.0
                        ),
                    });
                }
            }
        }
        if self
            .title_fields
            .get(&DrawingSheetTitleFieldId::SheetTitle)
            .is_none_or(|field| !field.visible || field.value.trim().is_empty())
        {
            problems.push(FieldProblem {
                field: "title-fields",
                message: "Sheet title is required and cannot be empty.".to_owned(),
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
        if self.zone_mode == ZoneModeChoice::Custom
            && (!(2..=24).contains(&self.zone_columns) || !(2..=24).contains(&self.zone_rows))
        {
            problems.push(FieldProblem {
                field: "zones",
                message: "Custom zone rows and columns must each be between 2 and 24.".to_owned(),
            });
        }
        if parse_scale(&self.declared_scale).is_none() {
            problems.push(FieldProblem {
                field: "scale",
                message: "Declared scale must be NTS or a positive ratio such as 1:1 or 1:2.5."
                    .to_owned(),
            });
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
        let authored_size = match &self.size {
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
                },
            },
        };
        let declared_scale = parse_scale(&self.declared_scale).expect("validated scale");
        let mut title_fields = self.title_fields.clone();
        title_fields
            .get_mut(&DrawingSheetTitleFieldId::Scale)
            .expect("the canonical title-field set contains Scale")
            .value
            .clear();
        let page_format = SchematicSheetFormat::try_from_draft(SchematicSheetFormatDraft {
            authored_size,
            orientation: self.orientation,
            display_unit: display_unit_to_model(self.display_unit),
            margins: DrawingSheetMargins {
                top_um: top_um.expect("validated top margin"),
                right_um: right_um.expect("validated right margin"),
                bottom_um: bottom_um.expect("validated bottom margin"),
                left_um: left_um.expect("validated left margin"),
            },
            bleed_um: bleed_um.expect("validated bleed"),
            border: border_to_model(self.border),
            zones: DrawingSheetZones {
                mode: zone_mode_to_model(self.zone_mode),
                custom_columns: (self.zone_mode == ZoneModeChoice::Custom)
                    .then_some(self.zone_columns),
                custom_rows: (self.zone_mode == ZoneModeChoice::Custom).then_some(self.zone_rows),
                labels: zone_labels_to_model(self.zone_labels),
                edges: zone_edges_to_model(self.zone_edges),
            },
            marks: DrawingSheetMarks {
                registration: self.registration_marks,
                folding: self.folding_marks,
            },
            title_block: DrawingSheetTitleBlock {
                template: title_template_to_model(self.title_block),
                anchor: title_anchor_to_model(self.title_block_anchor),
                rotation: title_rotation_to_model(self.title_block_rotation),
                offset_x_um: offset_x_um.expect("validated horizontal title offset"),
                offset_y_um: offset_y_um.expect("validated vertical title offset"),
                scale: declared_scale,
                managed_template: self.managed_title_template.clone(),
                fields: title_fields,
            },
            inheritance: self.inheritance,
        })
        .map_err(|error| {
            let message = error.to_string();
            if message.contains("drawing sheet title block placement")
                || message.contains("drawing sheet title block offset")
            {
                return vec![
                    FieldProblem {
                        field: "title-offset-x",
                        message: message.clone(),
                    },
                    FieldProblem {
                        field: "title-offset-y",
                        message,
                    },
                ];
            }
            let field = if message.contains("drawing sheet margin")
                || message.contains("drawing sheet border")
                || message.contains("drawing sheet drawing area")
            {
                "margins"
            } else {
                "drawing-sheet"
            };
            vec![FieldProblem { field, message }]
        })?;
        Ok(ValidatedDrawingSheetDraft {
            page_format,
            width_um,
            height_um,
            margin_top_um: top_um.expect("validated top margin"),
            margin_right_um: right_um.expect("validated right margin"),
            margin_bottom_um: bottom_um.expect("validated bottom margin"),
            margin_left_um: left_um.expect("validated left margin"),
            bleed_um: bleed_um.expect("validated bleed"),
            title_block_offset_x_um: offset_x_um.expect("validated horizontal title offset"),
            title_block_offset_y_um: offset_y_um.expect("validated vertical title offset"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MarginEdge {
    Top,
    Right,
    Bottom,
    Left,
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
    pub(crate) margin_top_um: u64,
    pub(crate) margin_right_um: u64,
    pub(crate) margin_bottom_um: u64,
    pub(crate) margin_left_um: u64,
    pub(crate) bleed_um: u64,
    pub(crate) title_block_offset_x_um: i64,
    pub(crate) title_block_offset_y_um: i64,
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
    pub(crate) eyebrow: String,
    pub(crate) document_name: String,
    pub(crate) sheet_name: String,
    pub(crate) sheet_count: usize,
    pub(crate) writable_sheet_count: usize,
    pub(crate) managed_sheet_names: Vec<String>,
    pub(crate) available_presets: Vec<DrawingSheetPreset>,
    pub(crate) project_default: SchematicSheetFormat,
    pub(crate) restore_margins: DrawingSheetMargins,
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
    pub(crate) commit_error: Option<String>,
    pub(crate) pending_support: Option<DrawingSheetSupportRequest>,
    pub(crate) support_suspended: bool,
}

impl Default for DrawingSheetSetupState {
    fn default() -> Self {
        let draft = DrawingSheetDraft::from_format(&SchematicSheetFormat::default());
        Self {
            open: false,
            section: DrawingSheetSection::Format,
            eyebrow: "DRAWING SHEET".to_owned(),
            document_name: "Active document".to_owned(),
            sheet_name: "Active sheet".to_owned(),
            sheet_count: 1,
            writable_sheet_count: 1,
            managed_sheet_names: Vec::new(),
            available_presets: Vec::new(),
            project_default: SchematicSheetFormat::default(),
            restore_margins: SchematicSheetFormat::default().margins,
            authority: None,
            baseline: draft.clone(),
            last_valid_format: SchematicSheetFormat::default(),
            draft,
            staged_project_title_values: None,
            staged_document_control: None,
            authority_error: None,
            commit_error: None,
            pending_support: None,
            support_suspended: false,
        }
    }
}

impl DrawingSheetSetupState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn is_dirty(&self) -> bool {
        self.draft != self.baseline || self.staged_document_control.is_some()
    }

    pub(crate) fn restore_opened_values(&mut self) {
        self.draft.clone_from(&self.baseline);
        self.staged_project_title_values = None;
        self.staged_document_control = None;
        self.capture_restore_margins_from_draft();
        self.commit_error = None;
    }

    /// Restore only the four page margins from the selected format authority.
    ///
    /// This intentionally leaves dimensions, orientation, preset identity,
    /// frame/zones, bleed and title-block state untouched.
    pub(crate) fn restore_selected_margins(&mut self) {
        let margins = match &self.draft.size {
            SheetSizeChoice::Standard(standard) => standard.default_margins(),
            SheetSizeChoice::CapturedPreset { id, .. } => self
                .available_presets
                .iter()
                .find(|preset| preset.id == *id)
                .map(|preset| preset.format.margins)
                .unwrap_or(self.restore_margins),
            SheetSizeChoice::Custom => self.restore_margins,
        };
        self.draft.margin_top = format_um(margins.top_um, self.draft.display_unit);
        self.draft.margin_right = format_um(margins.right_um, self.draft.display_unit);
        self.draft.margin_bottom = format_um(margins.bottom_um, self.draft.display_unit);
        self.draft.margin_left = format_um(margins.left_um, self.draft.display_unit);
    }

    pub(crate) fn capture_restore_margins_from_draft(&mut self) {
        let unit = self.draft.display_unit;
        if let (Ok(top_um), Ok(right_um), Ok(bottom_um), Ok(left_um)) = (
            parse_length_um(&self.draft.margin_top, unit),
            parse_length_um(&self.draft.margin_right, unit),
            parse_length_um(&self.draft.margin_bottom, unit),
            parse_length_um(&self.draft.margin_left, unit),
        ) {
            self.restore_margins = DrawingSheetMargins {
                top_um,
                right_um,
                bottom_um,
                left_um,
            };
        }
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

pub(crate) fn parse_signed_length_um(value: &str, unit: SheetDisplayUnit) -> Result<i64, String> {
    let normalized = value.trim();
    let (negative, magnitude) = normalized
        .strip_prefix('-')
        .map_or((false, normalized), |value| (true, value));
    let magnitude = parse_length_um(magnitude, unit)?;
    let magnitude =
        i64::try_from(magnitude).map_err(|_| "The physical offset is too large.".to_owned())?;
    Ok(if negative { -magnitude } else { magnitude })
}

pub(crate) fn format_signed_um(value_um: i64, unit: SheetDisplayUnit) -> String {
    let magnitude = value_um.unsigned_abs();
    let formatted = format_um(magnitude, unit);
    if value_um < 0 {
        format!("-{formatted}")
    } else {
        formatted
    }
}

fn display_unit_from_model(unit: DrawingSheetDisplayUnit) -> SheetDisplayUnit {
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
        DrawingSheetBorderTemplate::Plain => BorderTemplateChoice::Plain,
        DrawingSheetBorderTemplate::None => BorderTemplateChoice::None,
        DrawingSheetBorderTemplate::OrganizationManaged => BorderTemplateChoice::Organization,
    }
}

fn border_to_model(value: BorderTemplateChoice) -> DrawingSheetBorderTemplate {
    match value {
        BorderTemplateChoice::Standard => DrawingSheetBorderTemplate::Standard,
        BorderTemplateChoice::Plain => DrawingSheetBorderTemplate::Plain,
        BorderTemplateChoice::None => DrawingSheetBorderTemplate::None,
        BorderTemplateChoice::Organization => DrawingSheetBorderTemplate::OrganizationManaged,
    }
}

fn zone_mode_from_model(value: DrawingSheetZoneMode) -> ZoneModeChoice {
    match value {
        DrawingSheetZoneMode::Automatic => ZoneModeChoice::Automatic,
        DrawingSheetZoneMode::Custom => ZoneModeChoice::Custom,
        DrawingSheetZoneMode::None => ZoneModeChoice::None,
    }
}

fn zone_mode_to_model(value: ZoneModeChoice) -> DrawingSheetZoneMode {
    match value {
        ZoneModeChoice::Automatic => DrawingSheetZoneMode::Automatic,
        ZoneModeChoice::Custom => DrawingSheetZoneMode::Custom,
        ZoneModeChoice::None => DrawingSheetZoneMode::None,
    }
}

fn zone_edges_from_model(value: DrawingSheetZoneEdges) -> ZoneEdgesChoice {
    match value {
        DrawingSheetZoneEdges::All => ZoneEdgesChoice::All,
        DrawingSheetZoneEdges::TopAndLeft => ZoneEdgesChoice::TopLeft,
        DrawingSheetZoneEdges::BottomAndRight => ZoneEdgesChoice::BottomRight,
    }
}

fn zone_edges_to_model(value: ZoneEdgesChoice) -> DrawingSheetZoneEdges {
    match value {
        ZoneEdgesChoice::All => DrawingSheetZoneEdges::All,
        ZoneEdgesChoice::TopLeft => DrawingSheetZoneEdges::TopAndLeft,
        ZoneEdgesChoice::BottomRight => DrawingSheetZoneEdges::BottomAndRight,
    }
}

fn zone_labels_from_model(value: DrawingSheetZoneLabels) -> ZoneLabelsChoice {
    match value {
        DrawingSheetZoneLabels::AlphaNumeric => ZoneLabelsChoice::AlphaNumeric,
        DrawingSheetZoneLabels::NumericAlpha => ZoneLabelsChoice::NumericAlpha,
        DrawingSheetZoneLabels::Coordinates => ZoneLabelsChoice::Coordinates,
    }
}

fn zone_labels_to_model(value: ZoneLabelsChoice) -> DrawingSheetZoneLabels {
    match value {
        ZoneLabelsChoice::AlphaNumeric => DrawingSheetZoneLabels::AlphaNumeric,
        ZoneLabelsChoice::NumericAlpha => DrawingSheetZoneLabels::NumericAlpha,
        ZoneLabelsChoice::Coordinates => DrawingSheetZoneLabels::Coordinates,
    }
}

fn title_template_from_model(value: DrawingSheetTitleBlockTemplate) -> TitleBlockTemplateChoice {
    match value {
        DrawingSheetTitleBlockTemplate::Compact => TitleBlockTemplateChoice::Compact,
        DrawingSheetTitleBlockTemplate::Standard => TitleBlockTemplateChoice::Standard,
        DrawingSheetTitleBlockTemplate::Wide => TitleBlockTemplateChoice::Wide,
        DrawingSheetTitleBlockTemplate::OrganizationManaged => {
            TitleBlockTemplateChoice::Organization
        }
        DrawingSheetTitleBlockTemplate::None => TitleBlockTemplateChoice::None,
    }
}

fn title_template_to_model(value: TitleBlockTemplateChoice) -> DrawingSheetTitleBlockTemplate {
    match value {
        TitleBlockTemplateChoice::Compact => DrawingSheetTitleBlockTemplate::Compact,
        TitleBlockTemplateChoice::Standard => DrawingSheetTitleBlockTemplate::Standard,
        TitleBlockTemplateChoice::Wide => DrawingSheetTitleBlockTemplate::Wide,
        TitleBlockTemplateChoice::Organization => {
            DrawingSheetTitleBlockTemplate::OrganizationManaged
        }
        TitleBlockTemplateChoice::None => DrawingSheetTitleBlockTemplate::None,
    }
}

fn title_anchor_from_model(value: DrawingSheetTitleBlockAnchor) -> TitleBlockAnchorChoice {
    match value {
        DrawingSheetTitleBlockAnchor::BottomRight => TitleBlockAnchorChoice::BottomRight,
        DrawingSheetTitleBlockAnchor::BottomLeft => TitleBlockAnchorChoice::BottomLeft,
        DrawingSheetTitleBlockAnchor::BottomStrip => TitleBlockAnchorChoice::BottomStrip,
        DrawingSheetTitleBlockAnchor::TopRight => TitleBlockAnchorChoice::TopRight,
    }
}

fn title_anchor_to_model(value: TitleBlockAnchorChoice) -> DrawingSheetTitleBlockAnchor {
    match value {
        TitleBlockAnchorChoice::BottomRight => DrawingSheetTitleBlockAnchor::BottomRight,
        TitleBlockAnchorChoice::BottomLeft => DrawingSheetTitleBlockAnchor::BottomLeft,
        TitleBlockAnchorChoice::BottomStrip => DrawingSheetTitleBlockAnchor::BottomStrip,
        TitleBlockAnchorChoice::TopRight => DrawingSheetTitleBlockAnchor::TopRight,
    }
}

fn title_rotation_from_model(value: DrawingSheetTitleBlockRotation) -> TitleBlockRotationChoice {
    match value {
        DrawingSheetTitleBlockRotation::Upright => TitleBlockRotationChoice::Upright,
        DrawingSheetTitleBlockRotation::Clockwise90 => TitleBlockRotationChoice::Clockwise,
        DrawingSheetTitleBlockRotation::CounterClockwise90 => {
            TitleBlockRotationChoice::CounterClockwise
        }
    }
}

fn title_rotation_to_model(value: TitleBlockRotationChoice) -> DrawingSheetTitleBlockRotation {
    match value {
        TitleBlockRotationChoice::Upright => DrawingSheetTitleBlockRotation::Upright,
        TitleBlockRotationChoice::Clockwise => DrawingSheetTitleBlockRotation::Clockwise90,
        TitleBlockRotationChoice::CounterClockwise => {
            DrawingSheetTitleBlockRotation::CounterClockwise90
        }
    }
}

fn scale_from_model(value: DrawingSheetScale) -> String {
    match value {
        DrawingSheetScale::NotToScale => "NTS".to_owned(),
        DrawingSheetScale::Ratio {
            drawing_units,
            reality_units,
        } => format!("{drawing_units}:{reality_units}"),
    }
}

fn parse_scale(value: &str) -> Option<DrawingSheetScale> {
    let value = value.trim();
    if value.eq_ignore_ascii_case("NTS") {
        return Some(DrawingSheetScale::NotToScale);
    }
    let (drawing, reality) = value.split_once(':')?;
    let (drawing_numerator, drawing_denominator) = parse_positive_decimal(drawing)?;
    let (reality_numerator, reality_denominator) = parse_positive_decimal(reality)?;
    let drawing_units = drawing_numerator.checked_mul(reality_denominator)?;
    let reality_units = reality_numerator.checked_mul(drawing_denominator)?;
    let divisor = gcd_u64(drawing_units, reality_units);
    let drawing_units = u32::try_from(drawing_units / divisor).ok()?;
    let reality_units = u32::try_from(reality_units / divisor).ok()?;
    Some(DrawingSheetScale::Ratio {
        drawing_units,
        reality_units,
    })
}

fn parse_positive_decimal(value: &str) -> Option<(u64, u64)> {
    let normalized = value.trim().replace(',', ".");
    let normalized = normalized.strip_prefix('+').unwrap_or(&normalized);
    let (whole, fraction) = normalized
        .split_once('.')
        .map_or((normalized, ""), |parts| parts);
    if whole.is_empty()
        || !whole.bytes().all(|byte| byte.is_ascii_digit())
        || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        || fraction.len() > 6
    {
        return None;
    }
    let denominator = 10_u64.checked_pow(fraction.len() as u32)?;
    let whole = whole.parse::<u64>().ok()?;
    let fraction = if fraction.is_empty() {
        0
    } else {
        fraction.parse::<u64>().ok()?
    };
    let numerator = whole.checked_mul(denominator)?.checked_add(fraction)?;
    (numerator > 0).then_some((numerator, denominator))
}

fn gcd_u64(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn automatic_zone_divisions(edge_um: u64) -> u8 {
    let edge_mm = edge_um as f64 / 1_000.0;
    let nearest = (edge_mm / 140.0).round() as i32;
    (2 * nearest).clamp(2, 24) as u8
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
    fn draft_validation_rejects_collapsed_printable_area() {
        let mut draft = valid_draft();
        draft.margin_left = "200".to_owned();
        draft.margin_right = "200".to_owned();
        let problems = draft.validate().unwrap_err();
        assert!(problems.iter().any(|problem| problem.field == "margins"));
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
        })
        .unwrap();
        assert_eq!(draft.validate().unwrap().page_format, expected);
    }

    #[test]
    fn orientation_keeps_authored_custom_zone_counts() {
        let mut draft = valid_draft();
        draft.zone_mode = ZoneModeChoice::Custom;
        draft.zone_columns = 7;
        draft.zone_rows = 5;
        draft.set_orientation(SchematicPageOrientation::Portrait);
        assert_eq!(
            (draft.zone_columns, draft.zone_rows),
            (7, 5),
            "custom zone divisions remain authored through orientation changes"
        );
    }

    #[test]
    fn size_changes_keep_authored_custom_zone_counts() {
        let mut draft = valid_draft();
        draft.zone_mode = ZoneModeChoice::Custom;
        draft.zone_columns = 7;
        draft.zone_rows = 5;

        draft.apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiE));

        assert_eq!(
            (draft.zone_columns, draft.zone_rows),
            (7, 5),
            "custom zone divisions remain authored through size changes"
        );
    }

    #[test]
    fn border_changes_normalize_disabled_zone_and_mark_controls() {
        let mut draft = valid_draft();
        draft.set_border_template(BorderTemplateChoice::None);
        assert_eq!(draft.zone_mode, ZoneModeChoice::None);
        assert!(!draft.registration_marks);
        assert!(!draft.folding_marks);
        assert!(draft.validate().is_ok());

        draft.set_border_template(BorderTemplateChoice::Organization);
        assert_eq!(draft.zone_mode, ZoneModeChoice::Automatic);
        assert!(draft.registration_marks);
        assert!(draft.folding_marks);
        assert!(draft.validate().is_ok());
    }

    #[test]
    fn title_block_changes_normalize_inaccessible_values() {
        let mut draft = valid_draft();
        draft.title_block_offset_x = "-4".to_owned();
        draft.title_block_offset_y = "2".to_owned();
        draft.title_block_rotation = TitleBlockRotationChoice::Clockwise;
        draft.set_title_block_template(TitleBlockTemplateChoice::None);
        assert_eq!(draft.title_block_offset_x, "0");
        assert_eq!(draft.title_block_offset_y, "0");
        assert_eq!(
            draft.title_block_rotation,
            TitleBlockRotationChoice::Upright
        );
        assert!(draft.validate().is_ok());

        draft.set_title_block_template(TitleBlockTemplateChoice::Compact);
        draft.title_block_rotation = TitleBlockRotationChoice::CounterClockwise;
        draft.set_title_block_anchor(TitleBlockAnchorChoice::BottomStrip);
        assert_eq!(
            draft.title_block_rotation,
            TitleBlockRotationChoice::CounterClockwise
        );
        assert!(draft.validate().is_ok());
    }

    #[test]
    fn declared_scale_is_the_single_canonical_scale_value() {
        let mut draft = valid_draft();
        draft.declared_scale = "2:5".to_owned();
        draft
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::Scale)
            .unwrap()
            .value = "stale".to_owned();

        let format = draft.validate().unwrap().page_format;

        assert_eq!(
            format.title_block.scale,
            DrawingSheetScale::Ratio {
                drawing_units: 2,
                reality_units: 5,
            }
        );
        assert_eq!(
            format.title_block.fields[&DrawingSheetTitleFieldId::Scale].value,
            ""
        );
    }

    #[test]
    fn decimal_declared_scale_is_preserved_as_an_exact_rational_ratio() {
        let mut draft = valid_draft();
        draft.declared_scale = "1:2.5".to_owned();

        let format = draft.validate().unwrap().page_format;

        assert_eq!(
            format.title_block.scale,
            DrawingSheetScale::Ratio {
                drawing_units: 2,
                reality_units: 5,
            }
        );
        assert_eq!(scale_from_model(format.title_block.scale), "2:5");
    }

    #[test]
    fn linked_margins_propagate_the_edge_that_was_edited() {
        let mut draft = valid_draft();
        draft.margins_linked = true;
        draft.margin_right = "17.25".to_owned();

        draft.link_margins_from(MarginEdge::Right);

        assert_eq!(draft.margin_top, "17.25");
        assert_eq!(draft.margin_right, "17.25");
        assert_eq!(draft.margin_bottom, "17.25");
        assert_eq!(draft.margin_left, "17.25");
    }

    #[test]
    fn managed_border_and_title_authorities_survive_replacement_drafts() {
        let mut authority = valid_draft();
        authority.set_border_template(BorderTemplateChoice::Organization);
        authority.set_title_block_template(TitleBlockTemplateChoice::Organization);
        authority.title_block_anchor = TitleBlockAnchorChoice::TopRight;
        authority.declared_scale = "1:2.5".to_owned();

        let mut replacement = valid_draft();
        replacement.set_border_template(BorderTemplateChoice::None);
        replacement.set_title_block_template(TitleBlockTemplateChoice::None);
        replacement
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value = "Edited title".to_owned();
        replacement.enforce_managed_authority(&authority);

        assert_eq!(replacement.border, BorderTemplateChoice::Organization);
        assert_eq!(
            replacement.title_block,
            TitleBlockTemplateChoice::Organization
        );
        assert_eq!(
            replacement.title_block_anchor,
            TitleBlockAnchorChoice::BottomRight
        );
        assert_eq!(replacement.declared_scale, "1:1");
        assert_eq!(
            replacement.title_fields[&DrawingSheetTitleFieldId::SheetTitle].value,
            "Edited title"
        );
    }

    #[test]
    fn one_off_custom_margin_restore_does_not_reset_geometry_or_frame() {
        let mut state = DrawingSheetSetupState::default();
        state.draft.size = SheetSizeChoice::Custom;
        state.draft.width = "333.125".to_owned();
        state.draft.height = "777.25".to_owned();
        state.draft.border = BorderTemplateChoice::Plain;
        state.restore_margins = DrawingSheetMargins {
            top_um: 12_000,
            right_um: 13_000,
            bottom_um: 14_000,
            left_um: 15_000,
        };
        state.draft.margin_top = "1".to_owned();
        state.draft.margin_right = "2".to_owned();
        state.draft.margin_bottom = "3".to_owned();
        state.draft.margin_left = "4".to_owned();

        state.restore_selected_margins();

        assert_eq!(state.draft.width, "333.125");
        assert_eq!(state.draft.height, "777.25");
        assert_eq!(state.draft.border, BorderTemplateChoice::Plain);
        assert_eq!(state.draft.margin_top, "12");
        assert_eq!(state.draft.margin_right, "13");
        assert_eq!(state.draft.margin_bottom, "14");
        assert_eq!(state.draft.margin_left, "15");
    }

    #[test]
    fn missing_captured_preset_restores_its_opened_margin_snapshot() {
        let mut state = DrawingSheetSetupState::default();
        state.draft.size = SheetSizeChoice::CapturedPreset {
            id: "missing-captured-preset".to_owned(),
            name: "Retained snapshot".to_owned(),
            scope: DrawingSheetPresetScope::Project,
        };
        state.available_presets.clear();
        state.restore_margins = DrawingSheetMargins {
            top_um: 11_000,
            right_um: 12_000,
            bottom_um: 13_000,
            left_um: 14_000,
        };
        state.draft.margin_top = "1".to_owned();
        state.draft.margin_right = "2".to_owned();
        state.draft.margin_bottom = "3".to_owned();
        state.draft.margin_left = "4".to_owned();

        state.restore_selected_margins();

        assert_eq!(state.draft.margin_top, "11");
        assert_eq!(state.draft.margin_right, "12");
        assert_eq!(state.draft.margin_bottom, "13");
        assert_eq!(state.draft.margin_left, "14");
    }

    #[test]
    fn restore_opened_values_also_restores_the_opened_margin_authority() {
        let mut state = DrawingSheetSetupState::default();
        let mut opened = valid_draft();
        opened.size = SheetSizeChoice::Custom;
        opened.margin_top = "21".to_owned();
        opened.margin_right = "22".to_owned();
        opened.margin_bottom = "23".to_owned();
        opened.margin_left = "24".to_owned();
        state.baseline = opened;
        state
            .draft
            .apply_size_choice(SheetSizeChoice::Standard(DrawingSheetStandard::AnsiA));
        state.restore_margins = DrawingSheetStandard::AnsiA.default_margins();

        state.restore_opened_values();
        state.draft.margin_top = "1".to_owned();
        state.draft.margin_right = "2".to_owned();
        state.draft.margin_bottom = "3".to_owned();
        state.draft.margin_left = "4".to_owned();
        state.restore_selected_margins();

        assert_eq!(state.draft.size, SheetSizeChoice::Custom);
        assert_eq!(state.draft.margin_top, "21");
        assert_eq!(state.draft.margin_right, "22");
        assert_eq!(state.draft.margin_bottom, "23");
        assert_eq!(state.draft.margin_left, "24");
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
        draft.margin_top = "10".to_owned();
        draft.margin_right = "10".to_owned();
        draft.margin_bottom = "10".to_owned();
        draft.margin_left = "10".to_owned();
        draft.border = BorderTemplateChoice::Standard;
        assert!(
            draft
                .validate()
                .unwrap_err()
                .iter()
                .any(|problem| problem.field == "margins")
        );
    }

    #[test]
    fn validation_retains_every_field_problem_for_complete_surface_feedback() {
        let mut draft = valid_draft();
        draft.width = "not-a-length".to_owned();
        draft.declared_scale = "not-a-scale".to_owned();
        draft
            .title_fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value
            .clear();

        let problems = draft.validate().unwrap_err();

        assert!(problems.iter().any(|problem| problem.field == "width"));
        assert!(problems.iter().any(|problem| problem.field == "scale"));
        assert!(
            problems
                .iter()
                .any(|problem| problem.field == "title-fields")
        );
    }
}
