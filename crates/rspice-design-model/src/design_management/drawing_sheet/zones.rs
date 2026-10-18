//! Where a sheet's zones lie and what each is called.
//!
//! Every surface that names a zone — the band lettered on the canvas and on
//! the plot, the status bar's cursor readout, an overflow advisory — reads it
//! from [`DrawingSheetZoneLayout`], so a reference quoted from one surface
//! finds the same field on every other.
//!
//! The two drawing-format standards RSpice follows lay zones out differently:
//!
//! * ISO 5457 (IS 10711:2001, 4.4 and Table 2) measures 50 mm fields from the
//!   axes of symmetry of the trimmed sheet, the corner fields taking up the
//!   difference. Rows are lettered from the top, columns numbered from the
//!   left.
//! * ASME Y14.1-2012 (Table 4-1, Figs 4-1 to 4-3) divides each edge of the
//!   trimmed sheet into equal zones counted from the lower right corner:
//!   numbers run right to left, letters bottom to top. Because the zones are
//!   counted over the whole sheet, the first and last zones inside the border
//!   are shorter by the margin.

use super::*;

/// ISO 5457 4.4: "The length of the fields is 50 mm".
const ISO_5457_FIELD_UM: u64 = 50_000;

/// ISO 5457 Table 2: grid fields along the long and the short side.
const ISO_5457_TABLE_2: [(DrawingSheetStandard, u8, u8); 5] = [
    (DrawingSheetStandard::IsoA0, 24, 16),
    (DrawingSheetStandard::IsoA1, 16, 12),
    (DrawingSheetStandard::IsoA2, 12, 8),
    (DrawingSheetStandard::IsoA3, 8, 6),
    (DrawingSheetStandard::IsoA4, 6, 4),
];

/// ASME Y14.1 Table 4-1 Note (1): zones are at least 1 in long.
pub const DRAWING_SHEET_ASME_MIN_ZONE_UM: u64 = 25_400;
/// ASME Y14.1 Table 4-1 Note (1): zones are at most 5.50 in long.
pub const DRAWING_SHEET_ASME_MAX_ZONE_UM: u64 = 139_700;

/// ASME Y14.1 Table 4-1, by sheet edge: the recommended number of zones along
/// an edge of that length. The table is keyed by size and orientation, but
/// every row gives an edge of a given length the same count, so it reads as
/// one count per edge.
const ASME_Y14_1_TABLE_4_1: [(u64, u8); 6] = [
    (215_900, 2),   // 8.50 in: 2 × 4.25
    (279_400, 2),   // 11.00 in: 2 × 5.50
    (431_800, 4),   // 17.00 in: 4 × 4.25
    (558_800, 4),   // 22.00 in: 4 × 5.50
    (863_600, 8),   // 34.00 in: 8 × 4.25
    (1_117_600, 8), // 44.00 in: 8 × 5.50
];

/// Letters a zone may be lettered with, in order. ISO 5457 4.4 omits I and O,
/// which read as the numerals one and zero; ASME Y14.1 states no omission and
/// letters no edge past H, so the same alphabet serves both.
const ZONE_LETTERS: &[u8; 24] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";

/// The letters naming the zone at `index`, counting from zero. After Z the
/// sequence continues AA, AB, … as a drawing's revision letters do, so a zone
/// reference never becomes a bare number that could be read as the other
/// axis.
#[must_use]
pub fn drawing_sheet_zone_letters(index: usize) -> String {
    let base = ZONE_LETTERS.len();
    let mut remaining = index;
    let mut letters = Vec::new();
    loop {
        letters.push(ZONE_LETTERS[remaining % base]);
        if remaining < base {
            break;
        }
        remaining = remaining / base - 1;
    }
    letters.reverse();
    String::from_utf8(letters).expect("zone letters are ASCII")
}

/// Fields ISO 5457's 50 mm-from-centre rule lays along an edge.
///
/// Fields stand 50 mm apart on both sides of the edge's centre, and a corner
/// field shorter than half a field merges with its neighbour. That is the
/// rule Table 2 follows: it reproduces every count in the table (A1's long
/// side would otherwise carry a 0.5 mm corner field), and it extends the
/// table to every other size drawn on an ISO 5457 frame.
#[must_use]
pub fn drawing_sheet_iso_5457_field_count(edge_um: u64) -> u8 {
    let fields_each_side = (edge_um / 2).saturating_sub(ISO_5457_FIELD_UM / 2) / ISO_5457_FIELD_UM;
    u8::try_from(fields_each_side * 2 + 2).unwrap_or(u8::MAX)
}

/// Zones ASME Y14.1 recommends along an edge of the given length.
///
/// An ANSI edge takes its Table 4-1 count. Any other edge, an ARCH sheet's or
/// a custom one's, takes the count of the ANSI edge closest to it and then
/// the nearest count that keeps every zone inside Note (1)'s 1 to 5.50 in.
#[must_use]
pub fn drawing_sheet_asme_zone_count(edge_um: u64) -> u8 {
    let (_, closest) = ASME_Y14_1_TABLE_4_1
        .into_iter()
        .min_by_key(|(table_edge_um, _)| table_edge_um.abs_diff(edge_um))
        .expect("Table 4-1 has rows");
    let mut count = u64::from(closest);
    if edge_um > count * DRAWING_SHEET_ASME_MAX_ZONE_UM {
        count = edge_um.div_ceil(DRAWING_SHEET_ASME_MAX_ZONE_UM);
    } else if edge_um < count * DRAWING_SHEET_ASME_MIN_ZONE_UM {
        count = (edge_um / DRAWING_SHEET_ASME_MIN_ZONE_UM).max(1);
    }
    u8::try_from(count).unwrap_or(u8::MAX)
}

/// One zone along one edge of the frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetZoneField {
    /// Where the zone starts, in sheet micrometres from the paper's top left,
    /// clipped to the drawing frame.
    pub start_um: i64,
    /// Where the zone ends, clipped likewise.
    pub end_um: i64,
    /// What the band letters the zone: a number or letters.
    pub label: String,
}

impl DrawingSheetZoneField {
    /// The centre of the zone, where the band letters it.
    #[must_use]
    pub const fn centre_um(&self) -> i64 {
        self.start_um + (self.end_um - self.start_um) / 2
    }

    const fn contains(&self, position_um: i64) -> bool {
        position_um >= self.start_um && position_um <= self.end_um
    }
}

/// A sheet's zones: the fields along the horizontal edges (left to right),
/// numbered, and along the vertical edges (top to bottom), lettered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetZoneLayout {
    pub columns: Vec<DrawingSheetZoneField>,
    pub rows: Vec<DrawingSheetZoneField>,
    pub edges: DrawingSheetZoneEdges,
}

impl DrawingSheetZoneLayout {
    /// The zone reference of a point in sheet micrometres, such as `B3`, or
    /// `None` outside the drawing frame.
    #[must_use]
    pub fn reference_at(&self, x_um: i64, y_um: i64) -> Option<String> {
        let column = self.columns.iter().find(|field| field.contains(x_um))?;
        let row = self.rows.iter().find(|field| field.contains(y_um))?;
        Some(format!("{}{}", row.label, column.label))
    }

    /// Positions between neighbouring columns, left to right.
    pub fn column_boundaries_um(&self) -> impl Iterator<Item = i64> + '_ {
        self.columns.iter().skip(1).map(|field| field.start_um)
    }

    /// Positions between neighbouring rows, top to bottom.
    pub fn row_boundaries_um(&self) -> impl Iterator<Item = i64> + '_ {
        self.rows.iter().skip(1).map(|field| field.start_um)
    }

    /// Which band edges letter the zone labels: top, right, bottom, left.
    #[must_use]
    pub const fn lettered_edges(&self) -> (bool, bool, bool, bool) {
        match self.edges {
            DrawingSheetZoneEdges::All => (true, true, true, true),
            DrawingSheetZoneEdges::TopAndRight => (true, true, false, false),
        }
    }
}

/// Which axis of the sheet a set of zones runs along.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ZoneAxis {
    /// Columns, along the top and bottom edges.
    Horizontal,
    /// Rows, along the left and right edges.
    Vertical,
}

/// Zones a sheet of `size` carries along its oriented width and height: ISO 5457 Table 2 and its 50 mm rule, or ASME Y14.1 Table
/// 4-1 and its Note (1), by the size's frame convention.
#[must_use]
pub fn drawing_sheet_automatic_zone_counts(
    size: &AuthoredDrawingSheetSize,
    orientation: SchematicPageOrientation,
) -> (u8, u8) {
    let (short_um, long_um) = size.portrait_dimensions_um();
    let (width_um, height_um) = match orientation {
        SchematicPageOrientation::Portrait => (short_um, long_um),
        SchematicPageOrientation::Landscape => (long_um, short_um),
    };
    match size.frame_convention() {
        DrawingSheetFrameConvention::Iso5457 => {
            let table = match size {
                AuthoredDrawingSheetSize::Standard { standard } => ISO_5457_TABLE_2
                    .into_iter()
                    .find(|(listed, _, _)| listed == standard),
                AuthoredDrawingSheetSize::Custom { .. } => None,
            };
            match table {
                Some((_, long, short)) if width_um >= height_um => (long, short),
                Some((_, long, short)) => (short, long),
                None => (
                    drawing_sheet_iso_5457_field_count(width_um),
                    drawing_sheet_iso_5457_field_count(height_um),
                ),
            }
        }
        DrawingSheetFrameConvention::AsmeY141 => (
            drawing_sheet_asme_zone_count(width_um),
            drawing_sheet_asme_zone_count(height_um),
        ),
    }
}

impl SchematicSheetFormat {
    /// Zones the sheet carries along its oriented width and height.
    pub(super) fn automatic_zone_counts(&self) -> (u8, u8) {
        drawing_sheet_automatic_zone_counts(&self.authored_size, self.orientation)
    }

    /// The sheet's zones as every surface draws and quotes them, or `None`
    /// when it has none.
    #[must_use]
    pub fn zone_layout(&self, geometry: &DrawingSheetGeometry) -> Option<DrawingSheetZoneLayout> {
        let grid = geometry.zones?;
        Some(DrawingSheetZoneLayout {
            columns: self.zone_fields(geometry, grid, ZoneAxis::Horizontal),
            rows: self.zone_fields(geometry, grid, ZoneAxis::Vertical),
            edges: grid.edges,
        })
    }

    fn zone_fields(
        &self,
        geometry: &DrawingSheetGeometry,
        grid: DrawingSheetZoneGrid,
        axis: ZoneAxis,
    ) -> Vec<DrawingSheetZoneField> {
        let (count, edge_um, frame_start, frame_length) = match axis {
            ZoneAxis::Horizontal => (
                grid.columns,
                geometry.paper.width_um,
                geometry.drawing_area.x_um,
                geometry.drawing_area.width_um,
            ),
            ZoneAxis::Vertical => (
                grid.rows,
                geometry.paper.height_um,
                geometry.drawing_area.y_um,
                geometry.drawing_area.height_um,
            ),
        };
        let count = usize::from(count.max(1));
        let frame_end = frame_start + frame_length as i64;
        let convention = self.frame_convention();
        let boundaries = zone_boundaries_um(convention, count, edge_um);
        let mut fields = Vec::with_capacity(count);
        for index in 0..count {
            let start_um = if index == 0 {
                frame_start
            } else {
                boundaries[index - 1].clamp(frame_start, frame_end)
            };
            let end_um = boundaries.get(index).map_or(frame_end, |boundary| {
                (*boundary).clamp(frame_start, frame_end)
            });
            if end_um <= start_um {
                continue;
            }
            // ISO counts from the top left; ASME from the lower right.
            let ordinal = match convention {
                DrawingSheetFrameConvention::Iso5457 => index,
                DrawingSheetFrameConvention::AsmeY141 => count - 1 - index,
            };
            // Both standards number along the horizontal edges and letter
            // along the vertical ones.
            fields.push(DrawingSheetZoneField {
                start_um,
                end_um,
                label: match axis {
                    ZoneAxis::Horizontal => (ordinal + 1).to_string(),
                    ZoneAxis::Vertical => drawing_sheet_zone_letters(ordinal),
                },
            });
        }
        fields
    }
}

/// The `count - 1` positions between neighbouring zones along an edge of
/// `edge_um`, ascending, in sheet micrometres. Both standards measure them
/// over the trimmed sheet, so the first and last may fall in the margin;
/// their zones then end at the frame.
fn zone_boundaries_um(
    convention: DrawingSheetFrameConvention,
    count: usize,
    edge_um: u64,
) -> Vec<i64> {
    let edge_um = edge_um as i64;
    let count_i64 = count as i64;
    match convention {
        DrawingSheetFrameConvention::Iso5457 => {
            // 50 mm fields from the axis of symmetry outwards.
            let centre = edge_um / 2;
            let each_side = (count_i64 - 2) / 2;
            (-each_side..=each_side)
                .map(|step| centre + step * ISO_5457_FIELD_UM as i64)
                .collect()
        }
        DrawingSheetFrameConvention::AsmeY141 => {
            // Equal zones over the trimmed sheet, Table 4-1 Note (1),
            // measured from the lower right corner the zones count from.
            (1..count_i64)
                .rev()
                .map(|index| edge_um - edge_um * index / count_i64)
                .collect()
        }
    }
}
