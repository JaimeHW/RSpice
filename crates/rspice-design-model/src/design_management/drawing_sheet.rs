//! Exact, durable drawing-sheet authority for governed schematics.
//!
//! Physical values are integer micrometres. `SchematicSheetSize` remains the
//! compatibility projection consumed by hardcopy; `authored_size` preserves
//! the complete standard identity or custom snapshot.

use super::*;
use serde::de::Error as _;

pub const DRAWING_SHEET_MIN_EDGE_UM: u64 = 50_800;
pub const DRAWING_SHEET_MAX_EDGE_UM: u64 = 2_540_000;
pub const DRAWING_SHEET_MAX_ASPECT_RATIO: u64 = 20;
pub const DRAWING_SHEET_MAX_BLEED_UM: u64 = 25_400;
pub const DRAWING_SHEET_MIN_DRAWING_EDGE_UM: u64 = 25_000;
pub const DRAWING_SHEET_MIN_ZONE_COUNT: u8 = 2;
pub const DRAWING_SHEET_MAX_ZONE_COUNT: u8 = 24;
pub const MAX_DRAWING_SHEET_PROJECT_PRESETS: usize = 4_096;
pub const MAX_DRAWING_SHEET_PRESET_NAME_CHARS: usize = 48;
pub const MAX_DRAWING_SHEET_PRESET_IMPORT_RECEIPTS: usize = 4_096;
pub const MAX_DRAWING_SHEET_PRESET_IMPORT_CANDIDATES: usize = MAX_DRAWING_SHEET_PROJECT_PRESETS;
pub const MAX_DRAWING_SHEET_TRANSACTION_RECEIPTS: usize = 4_096;
pub const MAX_DRAWING_SHEET_REVISION_BYTES: usize = 64;
pub const MAX_DRAWING_SHEET_CHANGE_REFERENCE_BYTES: usize = 256;
pub const DRAWING_SHEET_MANAGED_TEMPLATE_SCHEMA_VERSION: u32 = 1;
pub const DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS: u16 = 10_000;
pub const MAX_DRAWING_SHEET_MANAGED_LOGO_PRIMITIVES: usize = 64;
pub const MAX_DRAWING_SHEET_MANAGED_LOGO_POINTS_PER_PRIMITIVE: usize = 256;
pub const MAX_DRAWING_SHEET_MANAGED_LOGO_POINTS: usize = 4_096;

/// Compatibility projection consumed by existing hardcopy code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SchematicSheetSize {
    Standard {
        size: SchematicPageSize,
    },
    Custom {
        name: String,
        portrait_width_um: u64,
        portrait_height_um: u64,
    },
}

impl Default for SchematicSheetSize {
    fn default() -> Self {
        Self::Standard {
            size: SchematicPageSize::A4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetStandard {
    IsoA5,
    IsoA4,
    IsoA3,
    IsoA2,
    IsoA1,
    IsoA0,
    AnsiA,
    AnsiB,
    AnsiC,
    AnsiD,
    AnsiE,
    ArchA,
    ArchB,
    ArchC,
    ArchD,
    ArchE,
    JisB5,
    JisB4,
    JisB3,
    JisB2,
}

impl DrawingSheetStandard {
    pub const ALL: [Self; 20] = [
        Self::IsoA5,
        Self::IsoA4,
        Self::IsoA3,
        Self::IsoA2,
        Self::IsoA1,
        Self::IsoA0,
        Self::AnsiA,
        Self::AnsiB,
        Self::AnsiC,
        Self::AnsiD,
        Self::AnsiE,
        Self::ArchA,
        Self::ArchB,
        Self::ArchC,
        Self::ArchD,
        Self::ArchE,
        Self::JisB5,
        Self::JisB4,
        Self::JisB3,
        Self::JisB2,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::IsoA5 => "ISO A5",
            Self::IsoA4 => "ISO A4",
            Self::IsoA3 => "ISO A3",
            Self::IsoA2 => "ISO A2",
            Self::IsoA1 => "ISO A1",
            Self::IsoA0 => "ISO A0",
            Self::AnsiA => "ANSI A",
            Self::AnsiB => "ANSI B",
            Self::AnsiC => "ANSI C",
            Self::AnsiD => "ANSI D",
            Self::AnsiE => "ANSI E",
            Self::ArchA => "ARCH A",
            Self::ArchB => "ARCH B",
            Self::ArchC => "ARCH C",
            Self::ArchD => "ARCH D",
            Self::ArchE => "ARCH E",
            Self::JisB5 => "JIS B5",
            Self::JisB4 => "JIS B4",
            Self::JisB3 => "JIS B3",
            Self::JisB2 => "JIS B2",
        }
    }

    #[must_use]
    pub const fn series(self) -> DrawingSheetStandardSeries {
        match self {
            Self::IsoA5 | Self::IsoA4 | Self::IsoA3 | Self::IsoA2 | Self::IsoA1 | Self::IsoA0 => {
                DrawingSheetStandardSeries::Iso
            }
            Self::AnsiA | Self::AnsiB | Self::AnsiC | Self::AnsiD | Self::AnsiE => {
                DrawingSheetStandardSeries::Ansi
            }
            Self::ArchA | Self::ArchB | Self::ArchC | Self::ArchD | Self::ArchE => {
                DrawingSheetStandardSeries::Architectural
            }
            Self::JisB5 | Self::JisB4 | Self::JisB3 | Self::JisB2 => {
                DrawingSheetStandardSeries::Jis
            }
        }
    }

    /// Normative portrait dimensions in integer micrometres.
    #[must_use]
    pub const fn portrait_dimensions_um(self) -> (u64, u64) {
        match self {
            Self::IsoA5 => (148_000, 210_000),
            Self::IsoA4 => (210_000, 297_000),
            Self::IsoA3 => (297_000, 420_000),
            Self::IsoA2 => (420_000, 594_000),
            Self::IsoA1 => (594_000, 841_000),
            Self::IsoA0 => (841_000, 1_189_000),
            Self::AnsiA => (215_900, 279_400),
            Self::AnsiB => (279_400, 431_800),
            Self::AnsiC => (431_800, 558_800),
            Self::AnsiD => (558_800, 863_600),
            Self::AnsiE => (863_600, 1_117_600),
            Self::ArchA => (228_600, 304_800),
            Self::ArchB => (304_800, 457_200),
            Self::ArchC => (457_200, 609_600),
            Self::ArchD => (609_600, 914_400),
            Self::ArchE => (914_400, 1_219_200),
            Self::JisB5 => (182_000, 257_000),
            Self::JisB4 => (257_000, 364_000),
            Self::JisB3 => (364_000, 515_000),
            Self::JisB2 => (515_000, 728_000),
        }
    }

    #[must_use]
    pub const fn default_margins(self) -> DrawingSheetMargins {
        match self.series() {
            DrawingSheetStandardSeries::Iso | DrawingSheetStandardSeries::Jis => {
                DrawingSheetMargins {
                    top_um: 10_000,
                    right_um: 10_000,
                    bottom_um: 10_000,
                    left_um: 20_000,
                }
            }
            DrawingSheetStandardSeries::Ansi => DrawingSheetMargins {
                top_um: 12_700,
                right_um: 12_700,
                bottom_um: 12_700,
                left_um: 19_050,
            },
            DrawingSheetStandardSeries::Architectural => DrawingSheetMargins {
                top_um: 12_700,
                right_um: 12_700,
                bottom_um: 12_700,
                left_um: 25_400,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetStandardSeries {
    Iso,
    Ansi,
    Architectural,
    Jis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomDrawingSheetSnapshot {
    pub preset_id: Option<String>,
    pub name: String,
    pub portrait_width_um: u64,
    pub portrait_height_um: u64,
    /// Keeps a captured preset publishable after its source is unavailable.
    #[serde(default)]
    pub source_preset_unavailable: bool,
}

impl CustomDrawingSheetSnapshot {
    fn validate(&self) -> Result<(), DesignManagementError> {
        validate_name("custom sheet format", &self.name)?;
        validate_drawing_sheet_preset_label(&self.name)?;
        if let Some(preset_id) = &self.preset_id {
            validate_name("drawing sheet preset id", preset_id)?;
        }
        validate_physical_dimensions(
            self.portrait_width_um,
            self.portrait_height_um,
            "custom sheet dimensions",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum AuthoredDrawingSheetSize {
    Standard {
        standard: DrawingSheetStandard,
    },
    Custom {
        snapshot: CustomDrawingSheetSnapshot,
    },
}

impl Default for AuthoredDrawingSheetSize {
    fn default() -> Self {
        Self::Standard {
            standard: DrawingSheetStandard::IsoA4,
        }
    }
}

impl AuthoredDrawingSheetSize {
    #[must_use]
    pub const fn portrait_dimensions_um(&self) -> (u64, u64) {
        match self {
            Self::Standard { standard } => standard.portrait_dimensions_um(),
            Self::Custom { snapshot } => (snapshot.portrait_width_um, snapshot.portrait_height_um),
        }
    }

    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::Standard { standard } => standard.label(),
            Self::Custom { snapshot } => &snapshot.name,
        }
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        match self {
            Self::Standard { .. } => Ok(()),
            Self::Custom { snapshot } => snapshot.validate(),
        }
    }

    fn legacy_projection(&self) -> SchematicSheetSize {
        match self {
            Self::Standard {
                standard: DrawingSheetStandard::IsoA4,
            } => SchematicSheetSize::Standard {
                size: SchematicPageSize::A4,
            },
            Self::Standard {
                standard: DrawingSheetStandard::IsoA3,
            } => SchematicSheetSize::Standard {
                size: SchematicPageSize::A3,
            },
            Self::Standard {
                standard: DrawingSheetStandard::AnsiA,
            } => SchematicSheetSize::Standard {
                size: SchematicPageSize::UsLetter,
            },
            Self::Standard {
                standard: DrawingSheetStandard::AnsiB,
            } => SchematicSheetSize::Standard {
                size: SchematicPageSize::UsLedger,
            },
            Self::Standard { standard } => {
                let (portrait_width_um, portrait_height_um) = standard.portrait_dimensions_um();
                SchematicSheetSize::Custom {
                    name: standard.label().to_owned(),
                    portrait_width_um,
                    portrait_height_um,
                }
            }
            Self::Custom { snapshot } => SchematicSheetSize::Custom {
                name: snapshot.name.clone(),
                portrait_width_um: snapshot.portrait_width_um,
                portrait_height_um: snapshot.portrait_height_um,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetDisplayUnit {
    #[default]
    Millimetres,
    Centimetres,
    Inches,
}

impl DrawingSheetDisplayUnit {
    #[must_use]
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Millimetres => "mm",
            Self::Centimetres => "cm",
            Self::Inches => "in",
        }
    }

    /// Canonical numeric formatter shared by inspectors, dialogs, and print.
    #[must_use]
    pub fn format_um(self, value_um: u64) -> String {
        format_micrometres(value_um, self)
    }

    /// Canonical oriented-size formatter including multiplication sign/unit.
    #[must_use]
    pub fn format_size_um(self, width_um: u64, height_um: u64) -> String {
        format!(
            "{} × {} {}",
            self.format_um(width_um),
            self.format_um(height_um),
            self.suffix()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetMargins {
    pub top_um: u64,
    pub right_um: u64,
    pub bottom_um: u64,
    pub left_um: u64,
}

impl Default for DrawingSheetMargins {
    fn default() -> Self {
        DrawingSheetStandard::IsoA4.default_margins()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetBorderTemplate {
    #[default]
    Standard,
    Plain,
    None,
    OrganizationManaged,
}

impl DrawingSheetBorderTemplate {
    #[must_use]
    pub const fn band_width_um(self) -> u64 {
        match self {
            Self::Standard => 5_000,
            Self::Plain => 2_500,
            Self::None => 0,
            Self::OrganizationManaged => 6_000,
        }
    }

    #[must_use]
    pub const fn supports_zones(self) -> bool {
        !matches!(self, Self::None)
    }

    #[must_use]
    pub const fn default_marks(self) -> DrawingSheetMarks {
        match self {
            Self::Standard => DrawingSheetMarks {
                registration: true,
                folding: false,
            },
            Self::Plain | Self::None => DrawingSheetMarks {
                registration: false,
                folding: false,
            },
            Self::OrganizationManaged => DrawingSheetMarks {
                registration: true,
                folding: true,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetZoneMode {
    #[default]
    Automatic,
    Custom,
    None,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetZoneLabels {
    #[default]
    AlphaNumeric,
    NumericAlpha,
    Coordinates,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetZoneEdges {
    #[default]
    All,
    TopAndLeft,
    BottomAndRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetZones {
    pub mode: DrawingSheetZoneMode,
    pub custom_columns: Option<u8>,
    pub custom_rows: Option<u8>,
    pub labels: DrawingSheetZoneLabels,
    pub edges: DrawingSheetZoneEdges,
}

impl Default for DrawingSheetZones {
    fn default() -> Self {
        Self {
            mode: DrawingSheetZoneMode::Automatic,
            custom_columns: None,
            custom_rows: None,
            labels: DrawingSheetZoneLabels::AlphaNumeric,
            edges: DrawingSheetZoneEdges::All,
        }
    }
}

impl DrawingSheetZones {
    fn validate(self, border: DrawingSheetBorderTemplate) -> Result<(), DesignManagementError> {
        if !border.supports_zones() && self.mode != DrawingSheetZoneMode::None {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet zones without a border",
            ));
        }
        match self.mode {
            DrawingSheetZoneMode::Automatic | DrawingSheetZoneMode::None => {
                if self.custom_columns.is_some() || self.custom_rows.is_some() {
                    return Err(DesignManagementError::NumericRange(
                        "inactive custom drawing sheet zones",
                    ));
                }
            }
            DrawingSheetZoneMode::Custom => {
                validate_zone_count(self.custom_columns)?;
                validate_zone_count(self.custom_rows)?;
            }
        }
        Ok(())
    }
}

fn validate_zone_count(value: Option<u8>) -> Result<(), DesignManagementError> {
    if value.is_some_and(|count| {
        (DRAWING_SHEET_MIN_ZONE_COUNT..=DRAWING_SHEET_MAX_ZONE_COUNT).contains(&count)
    }) {
        Ok(())
    } else {
        Err(DesignManagementError::NumericRange(
            "custom drawing sheet zone count",
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetMarks {
    #[serde(default)]
    pub registration: bool,
    #[serde(default)]
    pub folding: bool,
}

impl Default for DrawingSheetMarks {
    fn default() -> Self {
        DrawingSheetBorderTemplate::Standard.default_marks()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetTitleBlockTemplate {
    #[default]
    Compact,
    Standard,
    Wide,
    OrganizationManaged,
    None,
}

impl DrawingSheetTitleBlockTemplate {
    #[must_use]
    pub const fn dimensions_um(self) -> Option<(u64, u64)> {
        match self {
            Self::Compact => Some((130_000, 32_000)),
            Self::Standard => Some((180_000, 45_000)),
            Self::Wide => Some((230_000, 45_000)),
            Self::OrganizationManaged => Some((180_000, 50_000)),
            Self::None => None,
        }
    }

    /// Minimum authored paper width for the requested engineering template.
    ///
    /// This is a printed-artifact constraint, not a viewport constraint. A
    /// template that does not fit is visibly substituted with the compact
    /// block while the requested template remains authored on the sheet.
    #[must_use]
    pub const fn minimum_sheet_width_um(self) -> u64 {
        match self {
            Self::Compact | Self::None => 0,
            Self::Standard | Self::OrganizationManaged => 297_000,
            Self::Wide => 420_000,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetTitleBlockAnchor {
    #[default]
    BottomRight,
    BottomLeft,
    BottomStrip,
    TopRight,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetTitleBlockRotation {
    #[default]
    Upright,
    Clockwise90,
    CounterClockwise90,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum DrawingSheetScale {
    NotToScale,
    Ratio {
        drawing_units: u32,
        reality_units: u32,
    },
}

impl Default for DrawingSheetScale {
    fn default() -> Self {
        Self::Ratio {
            drawing_units: 1,
            reality_units: 1,
        }
    }
}

impl DrawingSheetScale {
    fn validate(self) -> Result<(), DesignManagementError> {
        match self {
            Self::Ratio {
                drawing_units: 0, ..
            }
            | Self::Ratio {
                reality_units: 0, ..
            } => Err(DesignManagementError::NumericRange("drawing sheet scale")),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetTitleFieldId {
    Project,
    CellView,
    SheetTitle,
    Page,
    Revision,
    Format,
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
    pub const ALL: [Self; 14] = [
        Self::Project,
        Self::CellView,
        Self::SheetTitle,
        Self::Page,
        Self::Revision,
        Self::Format,
        Self::Scale,
        Self::DrawnBy,
        Self::CheckedBy,
        Self::ApprovedBy,
        Self::Date,
        Self::Organization,
        Self::DocumentId,
        Self::Classification,
    ];

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
    /// and hardcopy. Renderers may change case for typography, but not the
    /// field's wording or engineering meaning.
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
            Self::Project | Self::CellView | Self::Page | Self::Revision | Self::Format => {
                DrawingSheetTitleFieldPolicy {
                    required_visible: true,
                    value_authority: DrawingSheetTitleFieldValueAuthority::Automatic,
                }
            }
            Self::SheetTitle => DrawingSheetTitleFieldPolicy {
                required_visible: true,
                value_authority: DrawingSheetTitleFieldValueAuthority::Authored,
            },
            Self::Date | Self::Scale => DrawingSheetTitleFieldPolicy {
                required_visible: false,
                value_authority: DrawingSheetTitleFieldValueAuthority::Automatic,
            },
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

/// One normalized point in a bounded organization logo. Coordinates use a
/// 0..=10,000 basis so the same exact geometry can be projected to canvas,
/// preview, SVG/PDF, raster output, and printer pages without embedding an
/// executable or externally linked asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetManagedLogoPoint {
    x: u16,
    y: u16,
}

impl DrawingSheetManagedLogoPoint {
    pub fn try_new(x: u16, y: u16) -> Result<Self, DesignManagementError> {
        if x > DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS
            || y > DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS
        {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet managed logo point",
            ));
        }
        Ok(Self { x, y })
    }

    #[must_use]
    pub const fn x(self) -> u16 {
        self.x
    }

    #[must_use]
    pub const fn y(self) -> u16 {
        self.y
    }
}

/// A deliberately small, monochrome vector primitive for an organization
/// mark. Filled primitives must be closed convex polygons; this keeps every
/// renderer deterministic and prevents SVG/script/font/raster payloads from
/// entering governed sheet state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetManagedLogoPrimitive {
    points: Vec<DrawingSheetManagedLogoPoint>,
    closed: bool,
    filled: bool,
}

impl DrawingSheetManagedLogoPrimitive {
    pub fn try_new(
        points: Vec<DrawingSheetManagedLogoPoint>,
        closed: bool,
        filled: bool,
    ) -> Result<Self, DesignManagementError> {
        let primitive = Self {
            points,
            closed,
            filled,
        };
        primitive.validate()?;
        Ok(primitive)
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        let minimum = if self.closed { 3 } else { 2 };
        if !(minimum..=MAX_DRAWING_SHEET_MANAGED_LOGO_POINTS_PER_PRIMITIVE)
            .contains(&self.points.len())
            || (self.filled && !self.closed)
            || self.points.iter().any(|point| {
                point.x > DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS
                    || point.y > DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS
            })
            || self.points.windows(2).any(|pair| pair[0] == pair[1])
            || (self.closed && self.points.first() == self.points.last())
            || (self.filled && !managed_logo_polygon_is_convex(&self.points))
        {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet managed logo primitive",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn points(&self) -> &[DrawingSheetManagedLogoPoint] {
        &self.points
    }

    #[must_use]
    pub const fn closed(&self) -> bool {
        self.closed
    }

    #[must_use]
    pub const fn filled(&self) -> bool {
        self.filled
    }
}

fn managed_logo_polygon_is_convex(points: &[DrawingSheetManagedLogoPoint]) -> bool {
    let mut direction = 0i32;
    for index in 0..points.len() {
        let a = points[index];
        let b = points[(index + 1) % points.len()];
        let c = points[(index + 2) % points.len()];
        let ab_x = i32::from(b.x) - i32::from(a.x);
        let ab_y = i32::from(b.y) - i32::from(a.y);
        let bc_x = i32::from(c.x) - i32::from(b.x);
        let bc_y = i32::from(c.y) - i32::from(b.y);
        let cross = ab_x * bc_y - ab_y * bc_x;
        if cross == 0 {
            continue;
        }
        let observed = cross.signum();
        if direction != 0 && direction != observed {
            return false;
        }
        direction = observed;
    }
    direction != 0
}

/// Digest-covered, self-contained organization mark and its reserved title
/// block width. The point and primitive budgets are strict denial-of-service
/// bounds for project loading and worker transfer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetManagedLogo {
    alternative_text: String,
    reserved_width_um: u64,
    primitives: Vec<DrawingSheetManagedLogoPrimitive>,
}

impl DrawingSheetManagedLogo {
    pub fn try_new(
        alternative_text: impl Into<String>,
        reserved_width_um: u64,
        primitives: Vec<DrawingSheetManagedLogoPrimitive>,
    ) -> Result<Self, DesignManagementError> {
        let logo = Self {
            alternative_text: alternative_text.into(),
            reserved_width_um,
            primitives,
        };
        logo.validate()?;
        Ok(logo)
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        validate_name(
            "drawing sheet managed logo alternative text",
            &self.alternative_text,
        )?;
        let total_points = self
            .primitives
            .iter()
            .try_fold(0usize, |total, primitive| {
                primitive.validate()?;
                total.checked_add(primitive.points.len()).ok_or(
                    DesignManagementError::NumericRange("drawing sheet managed logo point count"),
                )
            })?;
        if self.primitives.is_empty()
            || self.primitives.len() > MAX_DRAWING_SHEET_MANAGED_LOGO_PRIMITIVES
            || total_points > MAX_DRAWING_SHEET_MANAGED_LOGO_POINTS
        {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet managed logo complexity",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn alternative_text(&self) -> &str {
        &self.alternative_text
    }

    #[must_use]
    pub const fn reserved_width_um(&self) -> u64 {
        self.reserved_width_um
    }

    #[must_use]
    pub fn primitives(&self) -> &[DrawingSheetManagedLogoPrimitive] {
        &self.primitives
    }
}

/// Exact organization-managed title-block contract captured with a sheet or
/// preset. A digest-authenticated snapshot prevents a later policy update
/// from silently changing released drawings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetManagedTemplateSnapshot {
    schema_version: u32,
    template_id: String,
    revision: String,
    width_um: u64,
    height_um: u64,
    rows: u8,
    field_order: Vec<DrawingSheetTitleFieldId>,
    locked_fields: Vec<DrawingSheetTitleFieldId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    logo: Option<DrawingSheetManagedLogo>,
    content_digest: ContentDigest,
}

#[derive(Serialize)]
struct DrawingSheetManagedTemplateDigestMaterial<'a> {
    schema_version: u32,
    template_id: &'a str,
    revision: &'a str,
    width_um: u64,
    height_um: u64,
    rows: u8,
    field_order: &'a [DrawingSheetTitleFieldId],
    locked_fields: &'a [DrawingSheetTitleFieldId],
    logo: Option<&'a DrawingSheetManagedLogo>,
}

impl Default for DrawingSheetManagedTemplateSnapshot {
    fn default() -> Self {
        Self::try_new(
            "rspice.organization-title-block",
            "1",
            180_000,
            50_000,
            5,
            DrawingSheetTitleFieldId::ALL.to_vec(),
            vec![DrawingSheetTitleFieldId::Classification],
        )
        .expect("the built-in organization title-block snapshot is valid")
    }
}

impl DrawingSheetManagedTemplateSnapshot {
    pub fn try_new(
        template_id: impl Into<String>,
        revision: impl Into<String>,
        width_um: u64,
        height_um: u64,
        rows: u8,
        field_order: Vec<DrawingSheetTitleFieldId>,
        locked_fields: Vec<DrawingSheetTitleFieldId>,
    ) -> Result<Self, DesignManagementError> {
        Self::try_new_with_logo(
            template_id,
            revision,
            width_um,
            height_um,
            rows,
            field_order,
            locked_fields,
            None,
        )
    }

    // One argument per independently authored field of the snapshot this
    // builds. A parameter struct here would be the same fields a second time,
    // with nothing to validate until it was unpacked again.
    #[allow(clippy::too_many_arguments)]
    pub fn try_new_with_logo(
        template_id: impl Into<String>,
        revision: impl Into<String>,
        width_um: u64,
        height_um: u64,
        rows: u8,
        field_order: Vec<DrawingSheetTitleFieldId>,
        locked_fields: Vec<DrawingSheetTitleFieldId>,
        logo: Option<DrawingSheetManagedLogo>,
    ) -> Result<Self, DesignManagementError> {
        let mut snapshot = Self {
            schema_version: DRAWING_SHEET_MANAGED_TEMPLATE_SCHEMA_VERSION,
            template_id: template_id.into(),
            revision: revision.into(),
            width_um,
            height_um,
            rows,
            field_order,
            locked_fields,
            logo,
            content_digest: ContentDigest::from_bytes([0; 32]),
        };
        snapshot.validate_shape()?;
        snapshot.content_digest = snapshot.compute_content_digest()?;
        Ok(snapshot)
    }

    pub fn validate(&self) -> Result<(), DesignManagementError> {
        self.validate_shape()?;
        if self.compute_content_digest()? != self.content_digest {
            return Err(DesignManagementError::InvalidText {
                field: "drawing sheet managed template digest",
                value: self.content_digest.to_string(),
            });
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<(), DesignManagementError> {
        if self.schema_version != DRAWING_SHEET_MANAGED_TEMPLATE_SCHEMA_VERSION {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet managed template schema",
            ));
        }
        validate_name("drawing sheet managed template identity", &self.template_id)?;
        validate_value(
            "drawing sheet managed template revision",
            &self.revision,
            false,
        )?;
        if !(50_000..=600_000).contains(&self.width_um)
            || !(20_000..=200_000).contains(&self.height_um)
            || !(2..=8).contains(&self.rows)
        {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet managed template geometry",
            ));
        }
        let expected = DrawingSheetTitleFieldId::ALL
            .into_iter()
            .collect::<BTreeSet<_>>();
        let actual = self.field_order.iter().copied().collect::<BTreeSet<_>>();
        if self.field_order.len() != DrawingSheetTitleFieldId::ALL.len() || actual != expected {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet managed template field order",
            ));
        }
        let locked = self.locked_fields.iter().copied().collect::<BTreeSet<_>>();
        if locked.len() != self.locked_fields.len()
            || self.locked_fields.iter().any(|field| {
                field.policy().value_authority != DrawingSheetTitleFieldValueAuthority::Authored
            })
        {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet managed template locked fields",
            ));
        }
        if let Some(logo) = &self.logo {
            logo.validate()?;
            if !(10_000..=self.width_um.saturating_sub(50_000)).contains(&logo.reserved_width_um) {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet managed logo reserved width",
                ));
            }
        }
        Ok(())
    }

    fn compute_content_digest(&self) -> Result<ContentDigest, DesignManagementError> {
        digest(
            "rspice-drawing-sheet-managed-template/v1",
            &DrawingSheetManagedTemplateDigestMaterial {
                schema_version: self.schema_version,
                template_id: &self.template_id,
                revision: &self.revision,
                width_um: self.width_um,
                height_um: self.height_um,
                rows: self.rows,
                field_order: &self.field_order,
                locked_fields: &self.locked_fields,
                logo: self.logo.as_ref(),
            },
        )
    }

    #[must_use]
    pub fn template_id(&self) -> &str {
        &self.template_id
    }

    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }

    #[must_use]
    pub const fn dimensions_um(&self) -> (u64, u64) {
        (self.width_um, self.height_um)
    }

    #[must_use]
    pub const fn rows(&self) -> usize {
        self.rows as usize
    }

    #[must_use]
    pub fn field_order(&self) -> &[DrawingSheetTitleFieldId] {
        &self.field_order
    }

    #[must_use]
    pub fn locks_field(&self, field: DrawingSheetTitleFieldId) -> bool {
        self.locked_fields.contains(&field)
    }

    #[must_use]
    pub const fn logo(&self) -> Option<&DrawingSheetManagedLogo> {
        self.logo.as_ref()
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }
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
        .iter()
        .copied()
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetTitleBlock {
    pub template: DrawingSheetTitleBlockTemplate,
    pub anchor: DrawingSheetTitleBlockAnchor,
    pub rotation: DrawingSheetTitleBlockRotation,
    pub offset_x_um: i64,
    pub offset_y_um: i64,
    pub scale: DrawingSheetScale,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_template: Option<DrawingSheetManagedTemplateSnapshot>,
    pub fields: BTreeMap<DrawingSheetTitleFieldId, DrawingSheetTitleFieldState>,
}

impl Default for DrawingSheetTitleBlock {
    fn default() -> Self {
        let fields = DrawingSheetTitleFieldId::ALL
            .into_iter()
            .map(|field| (field, DrawingSheetTitleFieldState::default()))
            .collect::<BTreeMap<_, _>>();
        Self {
            template: DrawingSheetTitleBlockTemplate::Compact,
            anchor: DrawingSheetTitleBlockAnchor::BottomRight,
            rotation: DrawingSheetTitleBlockRotation::Upright,
            offset_x_um: 0,
            offset_y_um: 0,
            scale: DrawingSheetScale::default(),
            managed_template: None,
            fields,
        }
    }
}

impl DrawingSheetTitleBlock {
    fn normalize_managed_template(&mut self) {
        if self.template == DrawingSheetTitleBlockTemplate::OrganizationManaged {
            self.managed_template.get_or_insert_with(Default::default);
        } else {
            self.managed_template = None;
        }
    }

    fn validate(&self) -> Result<(), DesignManagementError> {
        self.scale.validate()?;
        match (&self.template, &self.managed_template) {
            (DrawingSheetTitleBlockTemplate::OrganizationManaged, Some(snapshot)) => {
                snapshot.validate()?;
            }
            (DrawingSheetTitleBlockTemplate::OrganizationManaged, None) => {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet managed template snapshot",
                ));
            }
            (_, Some(_)) => {
                return Err(DesignManagementError::NumericRange(
                    "drawing sheet managed template ownership",
                ));
            }
            (_, None) => {}
        }
        if self.fields.len() != DrawingSheetTitleFieldId::ALL.len()
            || DrawingSheetTitleFieldId::ALL
                .into_iter()
                .any(|field| !self.fields.contains_key(&field))
        {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet title field set",
            ));
        }
        for (field, state) in &self.fields {
            validate_value("drawing sheet title field", &state.value, true)?;
            let policy = field.policy();
            if policy.required_visible && !state.visible {
                return Err(DesignManagementError::NumericRange(
                    "required drawing sheet title field visibility",
                ));
            }
            if policy.value_authority == DrawingSheetTitleFieldValueAuthority::Automatic
                && !state.value.is_empty()
            {
                return Err(DesignManagementError::InvalidText {
                    field: "automatic drawing sheet title field",
                    value: state.value.clone(),
                });
            }
        }
        if self.template == DrawingSheetTitleBlockTemplate::None
            && (self.offset_x_um != 0 || self.offset_y_um != 0)
        {
            return Err(DesignManagementError::NumericRange(
                "title block offset without a template",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
pub(super) fn civil_date_from_unix_days(days: i64) -> String {
    // Proleptic Gregorian conversion adapted from Howard Hinnant's
    // `civil_from_days`; integral and locale-independent on every target.
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetInheritance {
    #[default]
    Explicit,
    ProjectDefault,
    UserDefault,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicSheetFormat {
    pub size: SchematicSheetSize,
    pub orientation: SchematicPageOrientation,
    pub authored_size: AuthoredDrawingSheetSize,
    pub display_unit: DrawingSheetDisplayUnit,
    pub margins: DrawingSheetMargins,
    pub bleed_um: u64,
    pub border: DrawingSheetBorderTemplate,
    pub zones: DrawingSheetZones,
    pub marks: DrawingSheetMarks,
    pub title_block: DrawingSheetTitleBlock,
    pub inheritance: DrawingSheetInheritance,
}

/// Canonical editable payload for Page Setup.
///
/// It deliberately omits the legacy `size` projection. Applying this draft
/// derives that projection from `authored_size` and validates the complete
/// physical layout before returning a new immutable value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchematicSheetFormatDraft {
    pub authored_size: AuthoredDrawingSheetSize,
    pub orientation: SchematicPageOrientation,
    pub display_unit: DrawingSheetDisplayUnit,
    pub margins: DrawingSheetMargins,
    pub bleed_um: u64,
    pub border: DrawingSheetBorderTemplate,
    pub zones: DrawingSheetZones,
    pub marks: DrawingSheetMarks,
    pub title_block: DrawingSheetTitleBlock,
    pub inheritance: DrawingSheetInheritance,
}

impl From<&SchematicSheetFormat> for SchematicSheetFormatDraft {
    fn from(format: &SchematicSheetFormat) -> Self {
        Self {
            authored_size: format.authored_size.clone(),
            orientation: format.orientation,
            display_unit: format.display_unit,
            margins: format.margins,
            bleed_um: format.bleed_um,
            border: format.border,
            zones: format.zones,
            marks: format.marks,
            title_block: format.title_block.clone(),
            inheritance: format.inheritance,
        }
    }
}

impl SchematicSheetFormatDraft {
    /// Apply a border template together with its normative mark defaults.
    pub fn apply_border_template(&mut self, border: DrawingSheetBorderTemplate) {
        self.border = border;
        self.marks = border.default_marks();
        if border.supports_zones() {
            if self.zones.mode == DrawingSheetZoneMode::None {
                self.zones = DrawingSheetZones::default();
            }
        } else {
            self.zones = DrawingSheetZones::none();
        }
    }
}

impl Default for SchematicSheetFormat {
    fn default() -> Self {
        Self::from_standard(
            DrawingSheetStandard::IsoA4,
            SchematicPageOrientation::Landscape,
        )
    }
}

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct SchematicSheetFormatWire {
    size: SchematicSheetSize,
    orientation: SchematicPageOrientation,
    authored_size: PresentField<AuthoredDrawingSheetSize>,
    display_unit: DrawingSheetDisplayUnit,
    margins: PresentField<DrawingSheetMargins>,
    bleed_um: u64,
    border: DrawingSheetBorderTemplate,
    zones: PresentField<DrawingSheetZones>,
    marks: DrawingSheetMarks,
    title_block: PresentField<DrawingSheetTitleBlock>,
    inheritance: DrawingSheetInheritance,
}

/// Records whether a field existed without changing its serialized shape.
///
/// `Option<T>` is not suitable for this wire role: self-describing formats
/// such as JSON accept a direct `T`, while RON correctly requires `Some(T)`.
/// `SchematicSheetFormat` has always serialized these fields directly, so a
/// wire `Option<T>` makes RON sessions written by RSpice unreadable on the
/// next launch. Missing legacy fields still default to `None`; present fields
/// deserialize through the exact durable type that the serializer emitted.
struct PresentField<T>(Option<T>);

impl<T> Default for PresentField<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<'de, T> Deserialize<'de> for PresentField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|value| Self(Some(value)))
    }
}

impl Default for SchematicSheetFormatWire {
    fn default() -> Self {
        Self {
            size: SchematicSheetSize::default(),
            orientation: SchematicPageOrientation::default(),
            authored_size: PresentField::default(),
            display_unit: DrawingSheetDisplayUnit::default(),
            margins: PresentField::default(),
            bleed_um: 0,
            border: DrawingSheetBorderTemplate::default(),
            zones: PresentField::default(),
            marks: DrawingSheetBorderTemplate::Standard.default_marks(),
            title_block: PresentField::default(),
            inheritance: DrawingSheetInheritance::default(),
        }
    }
}

impl<'de> Deserialize<'de> for SchematicSheetFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = SchematicSheetFormatWire::deserialize(deserializer)?;
        let authored_size = wire
            .authored_size
            .0
            .unwrap_or_else(|| authored_size_from_legacy(&wire.size));
        let standard = match &authored_size {
            AuthoredDrawingSheetSize::Standard { standard } => Some(*standard),
            AuthoredDrawingSheetSize::Custom { .. } => None,
        };
        let legacy_title_block = wire.title_block.0.is_none();
        let mut format = Self {
            size: wire.size,
            orientation: wire.orientation,
            authored_size,
            display_unit: wire.display_unit,
            margins: wire.margins.0.unwrap_or_else(|| {
                standard.map_or_else(DrawingSheetMargins::default, |value| {
                    value.default_margins()
                })
            }),
            bleed_um: wire.bleed_um,
            border: wire.border,
            zones: wire.zones.0.unwrap_or_default(),
            marks: wire.marks,
            title_block: wire.title_block.0.unwrap_or_default(),
            inheritance: wire.inheritance,
        };
        // Builds predating structured scale authority mirrored the scale into
        // the generic title-field string. Normalize that redundant legacy
        // copy before validation; renderers resolve Scale from `scale`.
        format
            .title_block
            .fields
            .entry(DrawingSheetTitleFieldId::Scale)
            .or_default()
            .value
            .clear();
        format.title_block.normalize_managed_template();
        if legacy_title_block {
            format.fit_default_title_block();
        }
        format.validate().map_err(D::Error::custom)?;
        Ok(format)
    }
}

impl SchematicSheetFormat {
    pub fn content_digest(&self) -> Result<ContentDigest, DesignManagementError> {
        self.validate()?;
        digest("rspice-drawing-sheet-format/v1", self)
    }

    /// Atomically materialize and validate a complete Page Setup draft.
    pub fn try_from_draft(
        mut draft: SchematicSheetFormatDraft,
    ) -> Result<Self, DesignManagementError> {
        draft.title_block.normalize_managed_template();
        let format = Self {
            size: draft.authored_size.legacy_projection(),
            orientation: draft.orientation,
            authored_size: draft.authored_size,
            display_unit: draft.display_unit,
            margins: draft.margins,
            bleed_um: draft.bleed_um,
            border: draft.border,
            zones: draft.zones,
            marks: draft.marks,
            title_block: draft.title_block,
            inheritance: draft.inheritance,
        };
        format.validate()?;
        Ok(format)
    }

    /// Clone, edit, materialize, and validate without partially mutating the
    /// current format when the edit is invalid.
    pub fn try_update(
        &self,
        edit: impl FnOnce(&mut SchematicSheetFormatDraft),
    ) -> Result<Self, DesignManagementError> {
        let mut draft = SchematicSheetFormatDraft::from(self);
        edit(&mut draft);
        Self::try_from_draft(draft)
    }

    /// Remove values whose durable authority is the project. Field
    /// visibility remains part of the format snapshot, while the values are
    /// resolved from [`DrawingSheetProjectSettings`] at presentation time.
    #[must_use]
    pub fn without_project_owned_title_values(&self) -> Self {
        let mut format = self.clone();
        clear_project_owned_title_values(&mut format);
        debug_assert!(format.validate().is_ok());
        format
    }

    /// Produce the reusable physical-format payload stored by custom-size
    /// preset authorities and portable packages.
    ///
    /// Presets own geometry, frame/zones, title-block layout, declared scale,
    /// and field visibility. They never own values authored by a sheet or
    /// project. Clearing every field value here also prevents responsibility
    /// text from crossing projects through an export package.
    #[must_use]
    pub fn as_reusable_drawing_sheet_preset(&self) -> Self {
        let mut format = self.clone();
        clear_all_title_values(&mut format);
        debug_assert!(format.validate().is_ok());
        format
    }

    /// Apply this physical/frame format to an existing sheet without
    /// overwriting title-field values owned by that target sheet. Page Setup
    /// owns the declared scale, so its value follows the applied format while
    /// the target retains the Scale field's visibility choice.
    #[must_use]
    pub fn with_target_sheet_title_fields(&self, target: &Self) -> Self {
        let mut format = self.clone();
        format.title_block.fields = target.title_block.fields.clone();
        // Scale's value is resolved exclusively from the structured
        // `title_block.scale`; only its target-owned visibility is retained.
        format
            .title_block
            .fields
            .entry(DrawingSheetTitleFieldId::Scale)
            .or_default()
            .value
            .clear();
        clear_project_owned_title_values(&mut format);
        debug_assert!(format.validate().is_ok());
        format
    }

    /// Remove values that identify a particular authored sheet before a
    /// format is stored as a default template. Responsibility fields remain
    /// useful as explicit defaults; only the unique sheet title is cleared.
    #[must_use]
    pub fn as_drawing_sheet_default(&self) -> Self {
        let mut format = self.without_project_owned_title_values();
        format
            .title_block
            .fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .expect("the canonical title-field set contains Sheet title")
            .value
            .clear();
        debug_assert!(format.validate().is_ok());
        format
    }

    #[must_use]
    pub fn standard(size: SchematicPageSize, orientation: SchematicPageOrientation) -> Self {
        let standard = match size {
            SchematicPageSize::A4 => DrawingSheetStandard::IsoA4,
            SchematicPageSize::A3 => DrawingSheetStandard::IsoA3,
            SchematicPageSize::UsLetter => DrawingSheetStandard::AnsiA,
            SchematicPageSize::UsLedger => DrawingSheetStandard::AnsiB,
        };
        Self::from_standard(standard, orientation)
    }

    #[must_use]
    pub fn from_standard(
        standard: DrawingSheetStandard,
        orientation: SchematicPageOrientation,
    ) -> Self {
        let authored_size = AuthoredDrawingSheetSize::Standard { standard };
        let mut format = Self {
            size: authored_size.legacy_projection(),
            orientation,
            authored_size,
            display_unit: DrawingSheetDisplayUnit::Millimetres,
            margins: standard.default_margins(),
            bleed_um: 0,
            border: DrawingSheetBorderTemplate::Standard,
            zones: DrawingSheetZones::default(),
            marks: DrawingSheetBorderTemplate::Standard.default_marks(),
            title_block: DrawingSheetTitleBlock::default(),
            inheritance: DrawingSheetInheritance::Explicit,
        };
        format.fit_default_title_block();
        debug_assert!(format.validate().is_ok());
        format
    }

    pub fn try_custom(
        name: impl Into<String>,
        width_um: u64,
        height_um: u64,
        orientation: SchematicPageOrientation,
    ) -> Result<Self, DesignManagementError> {
        let mut dimensions = [width_um, height_um];
        dimensions.sort_unstable();
        let snapshot = CustomDrawingSheetSnapshot {
            preset_id: None,
            name: normalize_text(&name.into()),
            portrait_width_um: dimensions[0],
            portrait_height_um: dimensions[1],
            source_preset_unavailable: false,
        };
        snapshot.validate()?;
        let authored_size = AuthoredDrawingSheetSize::Custom { snapshot };
        let framed = dimensions[0] >= 80_000 && dimensions[1] >= 80_000;
        let mut format = Self {
            size: authored_size.legacy_projection(),
            orientation,
            authored_size,
            display_unit: DrawingSheetDisplayUnit::Millimetres,
            margins: if framed {
                DrawingSheetMargins::default()
            } else {
                DrawingSheetMargins::zero()
            },
            bleed_um: 0,
            border: if framed {
                DrawingSheetBorderTemplate::Standard
            } else {
                DrawingSheetBorderTemplate::None
            },
            zones: if framed {
                DrawingSheetZones::default()
            } else {
                DrawingSheetZones::none()
            },
            marks: if framed {
                DrawingSheetBorderTemplate::Standard.default_marks()
            } else {
                DrawingSheetBorderTemplate::None.default_marks()
            },
            title_block: DrawingSheetTitleBlock::default(),
            inheritance: DrawingSheetInheritance::Explicit,
        };
        format.fit_default_title_block();
        format.validate()?;
        Ok(format)
    }

    pub fn validate(&self) -> Result<(), DesignManagementError> {
        self.authored_size.validate()?;
        let (width, height) = self.portrait_dimensions_um();
        validate_physical_dimensions(width, height, "drawing sheet dimensions")?;
        if self.size != self.authored_size.legacy_projection() {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet compatibility projection",
            ));
        }
        if self.bleed_um > DRAWING_SHEET_MAX_BLEED_UM {
            return Err(DesignManagementError::NumericRange("drawing sheet bleed"));
        }
        self.zones.validate(self.border)?;
        self.title_block.validate()?;
        self.geometry()?;
        Ok(())
    }

    /// Validate the title value required by a durable authored sheet.
    ///
    /// Reusable defaults and presets intentionally carry no sheet-owned
    /// values, so this contract is applied by `DesignSheet`, not by generic
    /// format validation.
    pub fn validate_authored_sheet_title(&self) -> Result<(), DesignManagementError> {
        let title = self
            .title_block
            .fields
            .get(&DrawingSheetTitleFieldId::SheetTitle)
            .ok_or(DesignManagementError::NumericRange(
                "authored drawing sheet title field",
            ))?;
        if !title.visible || title.value.trim().is_empty() {
            return Err(DesignManagementError::InvalidText {
                field: "authored drawing sheet title",
                value: title.value.clone(),
            });
        }
        Ok(())
    }

    #[must_use]
    pub const fn portrait_dimensions_um(&self) -> (u64, u64) {
        self.authored_size.portrait_dimensions_um()
    }

    #[must_use]
    pub const fn oriented_dimensions_um(&self) -> (u64, u64) {
        let (width, height) = self.portrait_dimensions_um();
        match self.orientation {
            SchematicPageOrientation::Portrait => (width, height),
            SchematicPageOrientation::Landscape => (height, width),
        }
    }

    /// Title-block template actually painted on this physical sheet.
    ///
    /// The requested choice is never silently rewritten. If its printed
    /// footprint cannot fit the current paper width, the compact block is
    /// substituted and `title_block_substituted` reports that fact to Page
    /// Setup and inspection surfaces.
    #[must_use]
    pub fn effective_title_block_template(&self) -> DrawingSheetTitleBlockTemplate {
        let requested = self.title_block.template;
        let Some((template_width_um, _)) = self.title_block_dimensions_um(requested) else {
            return DrawingSheetTitleBlockTemplate::None;
        };
        let (paper_width_um, _) = self.oriented_dimensions_um();
        let fits = paper_width_um >= requested.minimum_sheet_width_um()
            && paper_width_um >= template_width_um.saturating_add(40_000);
        if fits {
            requested
        } else {
            DrawingSheetTitleBlockTemplate::Compact
        }
    }

    #[must_use]
    pub fn title_block_substituted(&self) -> bool {
        self.effective_title_block_template() != self.title_block.template
    }

    /// Exact dimensions for the requested/effective template. Managed
    /// templates read their captured snapshot; built-ins read the stable
    /// product contract.
    #[must_use]
    pub fn title_block_dimensions_um(
        &self,
        template: DrawingSheetTitleBlockTemplate,
    ) -> Option<(u64, u64)> {
        if template == DrawingSheetTitleBlockTemplate::OrganizationManaged {
            self.title_block
                .managed_template
                .as_ref()
                .map(DrawingSheetManagedTemplateSnapshot::dimensions_um)
        } else {
            template.dimensions_um()
        }
    }

    #[must_use]
    pub fn title_block_rows(&self, template: DrawingSheetTitleBlockTemplate) -> Option<usize> {
        if template == DrawingSheetTitleBlockTemplate::OrganizationManaged {
            self.title_block
                .managed_template
                .as_ref()
                .map(DrawingSheetManagedTemplateSnapshot::rows)
        } else {
            drawing_sheet_title_block_rows(template)
        }
    }

    #[must_use]
    pub fn title_block_field_order(&self) -> &[DrawingSheetTitleFieldId] {
        self.title_block
            .managed_template
            .as_ref()
            .filter(|_| {
                self.title_block.template == DrawingSheetTitleBlockTemplate::OrganizationManaged
            })
            .map_or(&DrawingSheetTitleFieldId::ALL, |snapshot| {
                snapshot.field_order()
            })
    }

    /// Captured organization logo for the exact effective title contract.
    /// A substituted compact block never inherits managed artwork that no
    /// longer has an authored reserve in its grid.
    #[must_use]
    pub fn title_block_logo(
        &self,
        template: DrawingSheetTitleBlockTemplate,
    ) -> Option<&DrawingSheetManagedLogo> {
        (template == DrawingSheetTitleBlockTemplate::OrganizationManaged)
            .then_some(self.title_block.managed_template.as_ref())
            .flatten()
            .and_then(DrawingSheetManagedTemplateSnapshot::logo)
    }

    #[must_use]
    pub fn display(&self) -> String {
        let (width, height) = self.oriented_dimensions_um();
        format!(
            "{} {} · {}",
            self.authored_size.label(),
            self.orientation.label(),
            self.display_unit.format_size_um(width, height)
        )
    }

    pub fn geometry(&self) -> Result<DrawingSheetGeometry, DesignManagementError> {
        let (paper_width, paper_height) = self.oriented_dimensions_um();
        let paper = DrawingSheetRect::new(0, 0, paper_width, paper_height);
        let bleed_diameter = self
            .bleed_um
            .checked_mul(2)
            .ok_or(DesignManagementError::NumericRange("drawing sheet bleed"))?;
        let bleed = DrawingSheetRect::new(
            -(self.bleed_um as i64),
            -(self.bleed_um as i64),
            paper_width
                .checked_add(bleed_diameter)
                .ok_or(DesignManagementError::NumericRange("drawing sheet bleed"))?,
            paper_height
                .checked_add(bleed_diameter)
                .ok_or(DesignManagementError::NumericRange("drawing sheet bleed"))?,
        );
        let printable = paper.inset(
            self.margins.left_um,
            self.margins.top_um,
            self.margins.right_um,
            self.margins.bottom_um,
            "drawing sheet margins",
        )?;
        let border_band_um = self.border.band_width_um();
        let drawing_area = printable.inset(
            border_band_um,
            border_band_um,
            border_band_um,
            border_band_um,
            "drawing sheet border",
        )?;
        if drawing_area.width_um < DRAWING_SHEET_MIN_DRAWING_EDGE_UM
            || drawing_area.height_um < DRAWING_SHEET_MIN_DRAWING_EDGE_UM
        {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet drawing area",
            ));
        }
        let effective_title_block_template = self.effective_title_block_template();
        let title_block = self.title_block_rect(drawing_area, effective_title_block_template)?;
        let zones = match self.zones.mode {
            DrawingSheetZoneMode::None => None,
            DrawingSheetZoneMode::Automatic => Some(DrawingSheetZoneGrid {
                columns: automatic_zone_count(paper_width),
                rows: automatic_zone_count(paper_height),
                labels: self.zones.labels,
                edges: self.zones.edges,
            }),
            DrawingSheetZoneMode::Custom => Some(DrawingSheetZoneGrid {
                columns: self.zones.custom_columns.unwrap_or_default(),
                rows: self.zones.custom_rows.unwrap_or_default(),
                labels: self.zones.labels,
                edges: self.zones.edges,
            }),
        };
        Ok(DrawingSheetGeometry {
            bleed,
            paper,
            printable,
            drawing_area,
            content_area: drawing_area,
            title_block,
            zones,
            border_band_um,
            effective_title_block_template,
        })
    }

    fn title_block_rect(
        &self,
        drawing_area: DrawingSheetRect,
        template: DrawingSheetTitleBlockTemplate,
    ) -> Result<Option<DrawingSheetRect>, DesignManagementError> {
        let Some((template_width, template_height)) = self.title_block_dimensions_um(template)
        else {
            return Ok(None);
        };
        let (rotated_width, height) = match self.title_block.rotation {
            DrawingSheetTitleBlockRotation::Upright => (template_width, template_height),
            DrawingSheetTitleBlockRotation::Clockwise90
            | DrawingSheetTitleBlockRotation::CounterClockwise90 => {
                (template_height, template_width)
            }
        };
        let requested_width =
            if self.title_block.anchor == DrawingSheetTitleBlockAnchor::BottomStrip {
                drawing_area.width_um
            } else {
                rotated_width
            };
        // Small supported standards (for example ISO A5 portrait) may not
        // carry the full compact footprint after margins and the zone band.
        // Match the approved mockup contract by clipping the printed block to
        // the drawing area instead of overlapping or rejecting the sheet.
        let width = requested_width.min(drawing_area.width_um);
        let height = height.min(drawing_area.height_um);
        if width == 0 || height == 0 {
            Ok(None)
        } else {
            self.position_title_block(drawing_area, width, height)
                .map(Some)
        }
    }

    fn position_title_block(
        &self,
        drawing_area: DrawingSheetRect,
        width: u64,
        height: u64,
    ) -> Result<DrawingSheetRect, DesignManagementError> {
        let right = drawing_area.right_um()?;
        let bottom = drawing_area.bottom_um()?;
        let (base_x, base_y) = match self.title_block.anchor {
            DrawingSheetTitleBlockAnchor::BottomRight => {
                (right - width as i64, bottom - height as i64)
            }
            DrawingSheetTitleBlockAnchor::BottomLeft
            | DrawingSheetTitleBlockAnchor::BottomStrip => {
                (drawing_area.x_um, bottom - height as i64)
            }
            DrawingSheetTitleBlockAnchor::TopRight => (right - width as i64, drawing_area.y_um),
        };
        let rect = DrawingSheetRect::new(
            base_x.checked_add(self.title_block.offset_x_um).ok_or(
                DesignManagementError::NumericRange("drawing sheet title block offset"),
            )?,
            base_y.checked_add(self.title_block.offset_y_um).ok_or(
                DesignManagementError::NumericRange("drawing sheet title block offset"),
            )?,
            width,
            height,
        );
        if !drawing_area.contains(rect)? {
            return Err(DesignManagementError::NumericRange(
                "drawing sheet title block placement",
            ));
        }
        Ok(rect)
    }

    fn fit_default_title_block(&mut self) {
        if self.title_block.template == DrawingSheetTitleBlockTemplate::None
            || self.geometry().is_ok()
        {
            return;
        }
        self.title_block.rotation = DrawingSheetTitleBlockRotation::Clockwise90;
        if self.geometry().is_ok() {
            return;
        }
        self.title_block = DrawingSheetTitleBlock::none();
    }
}

impl DrawingSheetMargins {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            top_um: 0,
            right_um: 0,
            bottom_um: 0,
            left_um: 0,
        }
    }
}

impl DrawingSheetZones {
    #[must_use]
    pub const fn none() -> Self {
        Self {
            mode: DrawingSheetZoneMode::None,
            custom_columns: None,
            custom_rows: None,
            labels: DrawingSheetZoneLabels::AlphaNumeric,
            edges: DrawingSheetZoneEdges::All,
        }
    }
}

impl DrawingSheetTitleBlock {
    #[must_use]
    pub fn none() -> Self {
        Self {
            template: DrawingSheetTitleBlockTemplate::None,
            ..Self::default()
        }
    }
}

fn authored_size_from_legacy(size: &SchematicSheetSize) -> AuthoredDrawingSheetSize {
    match size {
        SchematicSheetSize::Standard { size } => AuthoredDrawingSheetSize::Standard {
            standard: match size {
                SchematicPageSize::A4 => DrawingSheetStandard::IsoA4,
                SchematicPageSize::A3 => DrawingSheetStandard::IsoA3,
                SchematicPageSize::UsLetter => DrawingSheetStandard::AnsiA,
                SchematicPageSize::UsLedger => DrawingSheetStandard::AnsiB,
            },
        },
        SchematicSheetSize::Custom {
            name,
            portrait_width_um,
            portrait_height_um,
        } => AuthoredDrawingSheetSize::Custom {
            snapshot: CustomDrawingSheetSnapshot {
                preset_id: None,
                name: name.clone(),
                portrait_width_um: *portrait_width_um,
                portrait_height_um: *portrait_height_um,
                source_preset_unavailable: false,
            },
        },
    }
}

fn validate_physical_dimensions(
    width_um: u64,
    height_um: u64,
    field: &'static str,
) -> Result<(), DesignManagementError> {
    if !(DRAWING_SHEET_MIN_EDGE_UM..=DRAWING_SHEET_MAX_EDGE_UM).contains(&width_um)
        || !(DRAWING_SHEET_MIN_EDGE_UM..=DRAWING_SHEET_MAX_EDGE_UM).contains(&height_um)
        || width_um > height_um
        || height_um > width_um.saturating_mul(DRAWING_SHEET_MAX_ASPECT_RATIO)
    {
        Err(DesignManagementError::NumericRange(field))
    } else {
        Ok(())
    }
}

fn automatic_zone_count(edge_um: u64) -> u8 {
    const TARGET_ZONE_PITCH_UM: u64 = 140_000;
    let rounded = edge_um
        .saturating_add(TARGET_ZONE_PITCH_UM / 2)
        .checked_div(TARGET_ZONE_PITCH_UM)
        .unwrap_or(1)
        .clamp(1, (DRAWING_SHEET_MAX_ZONE_COUNT / 2) as u64);
    (rounded as u8) * 2
}

fn format_micrometres(value: u64, unit: DrawingSheetDisplayUnit) -> String {
    let (denominator, precision): (u64, usize) = match unit {
        DrawingSheetDisplayUnit::Millimetres => (1_000, 3),
        DrawingSheetDisplayUnit::Centimetres => (10_000, 4),
        DrawingSheetDisplayUnit::Inches => (25_400, 4),
    };
    let whole = value / denominator;
    let remainder = value % denominator;
    if remainder == 0 {
        return whole.to_string();
    }
    let scale = 10_u64.pow(precision as u32);
    let rounded = (remainder.saturating_mul(scale) + denominator / 2) / denominator;
    let (whole, fraction) = if rounded == scale {
        (whole + 1, 0)
    } else {
        (whole, rounded)
    };
    if fraction == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{fraction:0precision$}")
            .trim_end_matches('0')
            .to_owned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetRect {
    pub x_um: i64,
    pub y_um: i64,
    pub width_um: u64,
    pub height_um: u64,
}

impl DrawingSheetRect {
    const fn new(x_um: i64, y_um: i64, width_um: u64, height_um: u64) -> Self {
        Self {
            x_um,
            y_um,
            width_um,
            height_um,
        }
    }

    fn inset(
        self,
        left_um: u64,
        top_um: u64,
        right_um: u64,
        bottom_um: u64,
        field: &'static str,
    ) -> Result<Self, DesignManagementError> {
        let horizontal = left_um
            .checked_add(right_um)
            .filter(|value| *value < self.width_um)
            .ok_or(DesignManagementError::NumericRange(field))?;
        let vertical = top_um
            .checked_add(bottom_um)
            .filter(|value| *value < self.height_um)
            .ok_or(DesignManagementError::NumericRange(field))?;
        Ok(Self::new(
            self.x_um
                .checked_add(left_um as i64)
                .ok_or(DesignManagementError::NumericRange(field))?,
            self.y_um
                .checked_add(top_um as i64)
                .ok_or(DesignManagementError::NumericRange(field))?,
            self.width_um - horizontal,
            self.height_um - vertical,
        ))
    }

    fn right_um(self) -> Result<i64, DesignManagementError> {
        self.x_um
            .checked_add(self.width_um as i64)
            .ok_or(DesignManagementError::NumericRange(
                "drawing sheet rectangle",
            ))
    }

    fn bottom_um(self) -> Result<i64, DesignManagementError> {
        self.y_um
            .checked_add(self.height_um as i64)
            .ok_or(DesignManagementError::NumericRange(
                "drawing sheet rectangle",
            ))
    }

    fn contains(self, other: Self) -> Result<bool, DesignManagementError> {
        Ok(other.x_um >= self.x_um
            && other.y_um >= self.y_um
            && other.right_um()? <= self.right_um()?
            && other.bottom_um()? <= self.bottom_um()?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetZoneGrid {
    pub columns: u8,
    pub rows: u8,
    pub labels: DrawingSheetZoneLabels,
    pub edges: DrawingSheetZoneEdges,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetGeometry {
    pub bleed: DrawingSheetRect,
    pub paper: DrawingSheetRect,
    pub printable: DrawingSheetRect,
    pub drawing_area: DrawingSheetRect,
    pub content_area: DrawingSheetRect,
    pub title_block: Option<DrawingSheetRect>,
    pub zones: Option<DrawingSheetZoneGrid>,
    pub border_band_um: u64,
    pub effective_title_block_template: DrawingSheetTitleBlockTemplate,
}

/// Authored title-grid rows for each durable title-block template.
///
/// All canvas, preview, setup, and publication paths consume this single
/// contract so a field never moves between cells or gains a different text
/// budget at output time.
#[must_use]
pub const fn drawing_sheet_title_block_rows(
    template: DrawingSheetTitleBlockTemplate,
) -> Option<usize> {
    match template {
        DrawingSheetTitleBlockTemplate::Compact => Some(3),
        DrawingSheetTitleBlockTemplate::Standard | DrawingSheetTitleBlockTemplate::Wide => Some(4),
        DrawingSheetTitleBlockTemplate::OrganizationManaged => Some(5),
        DrawingSheetTitleBlockTemplate::None => None,
    }
}

/// Conservative physical character capacity of one authored title-grid cell.
///
/// The budget is expressed in integer micrometres and is therefore stable
/// across monitor DPI, canvas zoom, SVG/PDF export, and raster publication.
#[must_use]
pub fn drawing_sheet_title_cell_capacity(
    format: &SchematicSheetFormat,
    geometry: &DrawingSheetGeometry,
    visible_field_count: usize,
) -> Option<usize> {
    let block = geometry.title_block?;
    let rows = format.title_block_rows(geometry.effective_title_block_template)?;
    let columns = visible_field_count.max(1).div_ceil(rows).max(1) as u64;
    let authored_width_um = match format.title_block.rotation {
        DrawingSheetTitleBlockRotation::Upright => block.width_um,
        DrawingSheetTitleBlockRotation::Clockwise90
        | DrawingSheetTitleBlockRotation::CounterClockwise90 => block.height_um,
    };
    let logo_reserve_um = format
        .title_block_logo(geometry.effective_title_block_template)
        .map_or(0, DrawingSheetManagedLogo::reserved_width_um);
    let cell_width_um = authored_width_um.saturating_sub(logo_reserve_um) / columns;
    Some(((cell_width_um.saturating_sub(4_000)) / 1_400).max(4) as usize)
}

pub(super) mod presets;
pub use presets::*;
