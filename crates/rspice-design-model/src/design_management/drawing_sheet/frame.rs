//! The frame a drawing-format standard draws around a sheet: the border line,
//! the zone band with its ticks and labels, centring marks, the size
//! designation and trimming marks.
//!
//! A sheet follows one of two standards, chosen by the series its size comes
//! from (see [`DrawingSheetFrameConvention`]). The frame is laid out here once,
//! in sheet micrometres, as [`DrawingSheetFrameArtwork`]; the canvas, the plot
//! and Page Setup's preview each paint that list rather than working out a
//! frame of their own, so the screen previews the print line for line.

use super::*;

/// ISO 5457 4.2: the frame is a continuous line 0.7 mm wide.
const ISO_5457_FRAME_LINE_UM: u64 = 700;
/// ISO 5457 4.4: grid reference lines are continuous lines 0.35 mm wide. The
/// grid reference border that closes the band is drawn at the same width.
const ISO_5457_GRID_LINE_UM: u64 = 350;
/// ISO 5457 4.3: centring marks are continuous lines 0.7 mm wide.
const ISO_5457_CENTRING_LINE_UM: u64 = 700;
/// ISO 5457 4.3: centring marks run from the grid reference border to
/// "10 mm beyond the drawing frame".
const ISO_5457_CENTRING_BEYOND_FRAME_UM: u64 = 10_000;
/// ISO 5457 4.5: each trimming mark is two overlapping 10 mm × 5 mm
/// rectangles.
const ISO_5457_TRIM_MARK_UM: (u64, u64) = (10_000, 5_000);
/// Lettering of the size designation in the bottom border (ISO 5457 3.1),
/// the height 4.4 gives the grid references beside it.
const ISO_5457_SIZE_DESIGNATION_UM: u64 = 3_500;
/// ISO 5457 4.4: grid references are lettered 3.5 mm high.
const ISO_5457_ZONE_LETTERING_UM: u64 = 3_500;
/// The grid reference band between the frame and the grid reference border.
/// Figure 4 names the border without the text dimensioning it; 5 mm from the
/// trimmed edge is where references to the standard draw it.
const ISO_5457_GRID_BAND_UM: u64 = 5_000;

/// ASME Y14.1 5.6(a): the border line is thick; ASME Y14.2 makes a thick line
/// at least 0.6 mm.
const ASME_Y14_1_BORDER_LINE_UM: u64 = 600;
/// ASME Y14.1 5.6(b)(3): zone markers are thin; ASME Y14.2 makes a thin line
/// at least 0.3 mm.
const ASME_Y14_1_ZONE_LINE_UM: u64 = 300;
/// ASME Y14.2: zone letters and numerals in the margin are at least 0.24 in.
const ASME_Y14_1_ZONE_LETTERING_UM: u64 = 6_000;
/// ASME Y14.1 Fig 5-5: a microfilm arrow on A, B and C sizes is 0.25 in long
/// with a 0.15 in head; on D and larger, 0.40 in with a 0.20 in head. Both
/// heads are 60° at the point.
const ASME_Y14_1_ARROW_SMALL_UM: (u64, u64) = (6_350, 3_810);
const ASME_Y14_1_ARROW_LARGE_UM: (u64, u64) = (10_160, 5_080);
/// The shorter edge from which ASME Y14.1 Fig 5-5 sizes arrows as D size.
const ASME_Y14_1_D_SIZE_LONG_EDGE_UM: u64 = 863_600;

/// ISO 5457 4.2: the left border, which is also the filing margin, is 20 mm
/// including the frame, and every other border 10 mm.
const ISO_5457_FILING_BORDER_UM: u64 = 20_000;
const ISO_5457_BORDER_UM: u64 = 10_000;
/// ASME Y14.1 Table 4-1: a 0.50 in format margin on every side of every flat
/// size.
const ASME_Y14_1_FORMAT_MARGIN_UM: u64 = 12_700;
/// ASME Y14.1 4.1.3 lets the margins of a bound document grow; RSpice's
/// binding margin is 0.75 in on the left.
pub const DRAWING_SHEET_ASME_BINDING_MARGIN_UM: u64 = 19_050;

/// The drawing-format standard a sheet's frame, zones and marks follow.
///
/// It is not stored: a standard size takes its series' standard, ISO A and
/// JIS B sheets ISO 5457 and ANSI and ARCH sheets the ASME Y14.1 format, and
/// a custom size the standard of the series it was started from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DrawingSheetFrameConvention {
    /// ISO 5457 (identical to IS 10711:2001).
    Iso5457,
    /// ASME Y14.1-2012. ARCH sheets are drawn in this format without being
    /// ASME sizes.
    AsmeY141,
}

impl DrawingSheetFrameConvention {
    #[must_use]
    pub const fn for_series(series: DrawingSheetStandardSeries) -> Self {
        match series {
            DrawingSheetStandardSeries::Iso | DrawingSheetStandardSeries::Jis => Self::Iso5457,
            DrawingSheetStandardSeries::Ansi | DrawingSheetStandardSeries::Architectural => {
                Self::AsmeY141
            }
        }
    }

    /// The format's name wherever RSpice reports it.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Iso5457 => "ISO 5457",
            Self::AsmeY141 => "ASME Y14.1 format",
        }
    }

    /// Width of the frame (ISO) or border line (ASME).
    #[must_use]
    pub const fn frame_line_um(self) -> u64 {
        match self {
            Self::Iso5457 => ISO_5457_FRAME_LINE_UM,
            Self::AsmeY141 => ASME_Y14_1_BORDER_LINE_UM,
        }
    }

    /// Width of the zone lines and of the band's outer border.
    #[must_use]
    pub const fn zone_line_um(self) -> u64 {
        match self {
            Self::Iso5457 => ISO_5457_GRID_LINE_UM,
            Self::AsmeY141 => ASME_Y14_1_ZONE_LINE_UM,
        }
    }

    /// Height the standard letters zone references at. Every surface letters
    /// a reference at this height, centred in the band, which each standard
    /// makes wide enough to hold it.
    #[must_use]
    pub const fn zone_lettering_um(self) -> u64 {
        match self {
            Self::Iso5457 => ISO_5457_ZONE_LETTERING_UM,
            Self::AsmeY141 => ASME_Y14_1_ZONE_LETTERING_UM,
        }
    }

    /// The standard's margins from the trimmed edge to the frame: ISO 5457
    /// 4.2's 20 mm filing border on the left and 10 mm elsewhere, or ASME
    /// Y14.1 Table 4-1's 0.50 in on every side, widened on the left for a
    /// bound drawing.
    #[must_use]
    pub const fn margins(self, binding: bool) -> DrawingSheetMargins {
        match self {
            Self::Iso5457 => DrawingSheetMargins {
                top_um: ISO_5457_BORDER_UM,
                right_um: ISO_5457_BORDER_UM,
                bottom_um: ISO_5457_BORDER_UM,
                left_um: ISO_5457_FILING_BORDER_UM,
            },
            Self::AsmeY141 => DrawingSheetMargins {
                top_um: ASME_Y14_1_FORMAT_MARGIN_UM,
                right_um: ASME_Y14_1_FORMAT_MARGIN_UM,
                bottom_um: ASME_Y14_1_FORMAT_MARGIN_UM,
                left_um: if binding {
                    DRAWING_SHEET_ASME_BINDING_MARGIN_UM
                } else {
                    ASME_Y14_1_FORMAT_MARGIN_UM
                },
            },
        }
    }
}

/// Where a sheet's frame puts its zone band and drawing area. ISO 5457 sets
/// the frame at the margins with the grid reference band outside it, closed
/// by the grid reference border; ASME Y14.1 sets the border line at the
/// margins with the zones in the margin between it and the trimmed edge.
#[derive(Debug, Clone, Copy)]
pub(super) struct DrawingSheetFrameLayout {
    pub(super) zone_band: DrawingSheetRect,
    pub(super) drawing_area: DrawingSheetRect,
    pub(super) band_um: u64,
}

/// What a piece of frame artwork belongs to, so each surface can show or
/// withhold it with the matching layer or output option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawingSheetFrameLayer {
    /// The frame or border line, and ISO's grid reference border.
    Border,
    /// Zone ticks across the band.
    Zones,
    /// Zone numbers and letters in the band.
    ZoneLabels,
    /// ISO centring marks, or ASME microfilm arrows.
    CentringMarks,
    /// The size designation in ISO's bottom border.
    SizeDesignation,
    /// Trimming marks at the paper corners, drawn only on output.
    TrimMarks,
}

/// One piece of frame artwork, in sheet micrometres from the paper's top
/// left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawingSheetFrameMark {
    Line {
        from: (i64, i64),
        to: (i64, i64),
        width_um: u64,
    },
    Rect {
        rect: DrawingSheetRect,
        width_um: u64,
    },
    FilledRect {
        rect: DrawingSheetRect,
    },
    /// A filled convex polygon.
    FilledPolygon {
        points: Vec<(i64, i64)>,
    },
    /// One line of upright text centred on `centre`, its capitals
    /// `lettering_um` high.
    Text {
        centre: (i64, i64),
        text: String,
        lettering_um: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetFrameItem {
    pub layer: DrawingSheetFrameLayer,
    pub mark: DrawingSheetFrameMark,
}

/// Everything a sheet's frame draws, in painting order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DrawingSheetFrameArtwork {
    pub items: Vec<DrawingSheetFrameItem>,
}

impl DrawingSheetFrameArtwork {
    /// The marks of one layer.
    pub fn layer(
        &self,
        layer: DrawingSheetFrameLayer,
    ) -> impl Iterator<Item = &DrawingSheetFrameMark> + '_ {
        self.items
            .iter()
            .filter(move |item| item.layer == layer)
            .map(|item| &item.mark)
    }

    fn push(&mut self, layer: DrawingSheetFrameLayer, mark: DrawingSheetFrameMark) {
        self.items.push(DrawingSheetFrameItem { layer, mark });
    }

    fn line(
        &mut self,
        layer: DrawingSheetFrameLayer,
        from: (i64, i64),
        to: (i64, i64),
        width_um: u64,
    ) {
        self.push(layer, DrawingSheetFrameMark::Line { from, to, width_um });
    }
}

impl DrawingSheetFrameConvention {
    /// The marks a border starts with on this format. ISO 5457's centring
    /// marks belong to its frame; ASME Y14.1 5.7 makes its microfilm arrows
    /// optional, and a new sheet leaves them off. A sheet with no border has
    /// no marks.
    #[must_use]
    pub const fn default_marks(self, border: DrawingSheetBorderTemplate) -> DrawingSheetMarks {
        DrawingSheetMarks {
            centring: matches!(
                (self, border),
                (Self::Iso5457, DrawingSheetBorderTemplate::Standard)
            ),
        }
    }
}

impl AuthoredDrawingSheetSize {
    /// The drawing-format standard a sheet of this size is framed to.
    #[must_use]
    pub const fn frame_convention(&self) -> DrawingSheetFrameConvention {
        match self {
            Self::Standard { standard } => {
                DrawingSheetFrameConvention::for_series(standard.series())
            }
            Self::Custom { snapshot } => match snapshot.started_from {
                Some(series) => DrawingSheetFrameConvention::for_series(series),
                None => DrawingSheetFrameConvention::Iso5457,
            },
        }
    }
}

impl SchematicSheetFormat {
    /// The drawing-format standard this sheet's frame and zones follow.
    #[must_use]
    pub const fn frame_convention(&self) -> DrawingSheetFrameConvention {
        self.authored_size.frame_convention()
    }

    pub(super) fn frame_layout(
        &self,
        paper: DrawingSheetRect,
        printable: DrawingSheetRect,
    ) -> Result<DrawingSheetFrameLayout, DesignManagementError> {
        if self.border == DrawingSheetBorderTemplate::None {
            return Ok(DrawingSheetFrameLayout {
                zone_band: printable,
                drawing_area: printable,
                band_um: 0,
            });
        }
        let margins = self.margins();
        Ok(match self.frame_convention() {
            DrawingSheetFrameConvention::Iso5457 => {
                let band_um = ISO_5457_GRID_BAND_UM;
                DrawingSheetFrameLayout {
                    zone_band: paper.inset(
                        margins.left_um - band_um,
                        margins.top_um - band_um,
                        margins.right_um - band_um,
                        margins.bottom_um - band_um,
                        "drawing sheet margins",
                    )?,
                    drawing_area: printable,
                    band_um,
                }
            }
            // The whole margin is the zone band; a binding margin widens it
            // on the left alone.
            DrawingSheetFrameConvention::AsmeY141 => DrawingSheetFrameLayout {
                zone_band: paper,
                drawing_area: printable,
                band_um: ASME_Y14_1_FORMAT_MARGIN_UM,
            },
        })
    }

    /// Everything this sheet's frame draws.
    #[must_use]
    pub fn frame_artwork(&self, geometry: &DrawingSheetGeometry) -> DrawingSheetFrameArtwork {
        let mut artwork = DrawingSheetFrameArtwork::default();
        let convention = self.frame_convention();
        let framed = self.border != DrawingSheetBorderTemplate::None;
        let iso_frame = framed && convention == DrawingSheetFrameConvention::Iso5457;
        let paper = geometry.paper;
        let band = geometry.zone_band;
        let area = geometry.drawing_area;
        let (paper_right, paper_bottom) = far_corner(paper);
        let (band_right, band_bottom) = far_corner(band);
        let (area_right, area_bottom) = far_corner(area);
        let centre = (paper_right / 2, paper_bottom / 2);
        if framed {
            artwork.push(
                DrawingSheetFrameLayer::Border,
                DrawingSheetFrameMark::Rect {
                    rect: area,
                    width_um: convention.frame_line_um(),
                },
            );
            let zones = self.zone_layout(geometry);
            // The grid reference border closes the band the zones are
            // lettered in; a sheet without zones has no band to close.
            if iso_frame && zones.is_some() {
                artwork.push(
                    DrawingSheetFrameLayer::Border,
                    DrawingSheetFrameMark::Rect {
                        rect: band,
                        width_um: ISO_5457_GRID_LINE_UM,
                    },
                );
            }
            if let Some(zones) = zones {
                self.zone_artwork(&mut artwork, geometry, &zones);
            }
            match convention {
                DrawingSheetFrameConvention::Iso5457 if self.marks.centring => {
                    let beyond = ISO_5457_CENTRING_BEYOND_FRAME_UM as i64;
                    for (from, at_frame, to) in [
                        (
                            (centre.0, band.y_um),
                            (centre.0, area.y_um),
                            (centre.0, area.y_um + beyond),
                        ),
                        (
                            (centre.0, band_bottom),
                            (centre.0, area_bottom),
                            (centre.0, area_bottom - beyond),
                        ),
                        (
                            (band.x_um, centre.1),
                            (area.x_um, centre.1),
                            (area.x_um + beyond, centre.1),
                        ),
                        (
                            (band_right, centre.1),
                            (area_right, centre.1),
                            (area_right - beyond, centre.1),
                        ),
                    ] {
                        // A mark that would run into the title block stops at
                        // the frame rather than rule through its cells.
                        let into_block = geometry
                            .title_block
                            .is_some_and(|block| segment_meets_rect(at_frame, to, block));
                        artwork.line(
                            DrawingSheetFrameLayer::CentringMarks,
                            from,
                            if into_block { at_frame } else { to },
                            ISO_5457_CENTRING_LINE_UM,
                        );
                    }
                }
                DrawingSheetFrameConvention::AsmeY141 if self.marks.centring => {
                    self.asme_microfilm_arrows(&mut artwork, geometry);
                }
                DrawingSheetFrameConvention::Iso5457 | DrawingSheetFrameConvention::AsmeY141 => {}
            }
            if iso_frame {
                self.iso_size_designation(&mut artwork, geometry);
            }
        }
        if iso_frame {
            let (long, short) = ISO_5457_TRIM_MARK_UM;
            let (long_i, short_i) = (long as i64, short as i64);
            for (x, y) in [
                (paper.x_um, paper.y_um),
                (paper_right - long_i, paper.y_um),
                (paper.x_um, paper_bottom - short_i),
                (paper_right - long_i, paper_bottom - short_i),
            ] {
                artwork.push(
                    DrawingSheetFrameLayer::TrimMarks,
                    DrawingSheetFrameMark::FilledRect {
                        rect: DrawingSheetRect::new(x, y, long, short),
                    },
                );
            }
            for (x, y) in [
                (paper.x_um, paper.y_um),
                (paper_right - short_i, paper.y_um),
                (paper.x_um, paper_bottom - long_i),
                (paper_right - short_i, paper_bottom - long_i),
            ] {
                artwork.push(
                    DrawingSheetFrameLayer::TrimMarks,
                    DrawingSheetFrameMark::FilledRect {
                        rect: DrawingSheetRect::new(x, y, short, long),
                    },
                );
            }
        } else {
            let arm = (paper.width_um.min(paper.height_um) / 8).min(4_000) as i64;
            for (x, y, dx, dy) in [
                (paper.x_um, paper.y_um, 1_i64, 1_i64),
                (paper_right, paper.y_um, -1, 1),
                (paper.x_um, paper_bottom, 1, -1),
                (paper_right, paper_bottom, -1, -1),
            ] {
                let width_um = convention.zone_line_um();
                artwork.line(
                    DrawingSheetFrameLayer::TrimMarks,
                    (x, y),
                    (x + dx * arm, y),
                    width_um,
                );
                artwork.line(
                    DrawingSheetFrameLayer::TrimMarks,
                    (x, y),
                    (x, y + dy * arm),
                    width_um,
                );
            }
        }
        artwork
    }

    fn zone_artwork(
        &self,
        artwork: &mut DrawingSheetFrameArtwork,
        geometry: &DrawingSheetGeometry,
        zones: &DrawingSheetZoneLayout,
    ) {
        let convention = self.frame_convention();
        let band = geometry.zone_band;
        let area = geometry.drawing_area;
        let (band_right, band_bottom) = far_corner(band);
        let (area_right, area_bottom) = far_corner(area);
        let width_um = convention.zone_line_um();
        // The band carries every boundary on all four sides; the standard
        // decides only where the references are lettered.
        for x in zones.column_boundaries_um() {
            artwork.line(
                DrawingSheetFrameLayer::Zones,
                (x, band.y_um),
                (x, area.y_um),
                width_um,
            );
            artwork.line(
                DrawingSheetFrameLayer::Zones,
                (x, area_bottom),
                (x, band_bottom),
                width_um,
            );
        }
        for y in zones.row_boundaries_um() {
            artwork.line(
                DrawingSheetFrameLayer::Zones,
                (band.x_um, y),
                (area.x_um, y),
                width_um,
            );
            artwork.line(
                DrawingSheetFrameLayer::Zones,
                (area_right, y),
                (band_right, y),
                width_um,
            );
        }
        let (top, right, bottom, left) = zones.lettered_edges();
        let lettering_um = convention.zone_lettering_um();
        let mid = |a: i64, b: i64| a + (b - a) / 2;
        let mut label = |centre: (i64, i64), text: &str| {
            artwork.push(
                DrawingSheetFrameLayer::ZoneLabels,
                DrawingSheetFrameMark::Text {
                    centre,
                    text: text.to_owned(),
                    lettering_um,
                },
            );
        };
        for column in &zones.columns {
            if top {
                label(
                    (column.centre_um(), mid(band.y_um, area.y_um)),
                    &column.label,
                );
            }
            if bottom {
                label(
                    (column.centre_um(), mid(area_bottom, band_bottom)),
                    &column.label,
                );
            }
        }
        for row in &zones.rows {
            if left {
                label((mid(band.x_um, area.x_um), row.centre_um()), &row.label);
            }
            if right {
                label((mid(area_right, band_right), row.centre_um()), &row.label);
            }
        }
    }

    /// ASME Y14.1 5.7.1 and Fig 5-5: a microfilm arrow centred in each format
    /// margin, pointing at the border line.
    fn asme_microfilm_arrows(
        &self,
        artwork: &mut DrawingSheetFrameArtwork,
        geometry: &DrawingSheetGeometry,
    ) {
        let paper = geometry.paper;
        let area = geometry.drawing_area;
        let (paper_right, paper_bottom) = far_corner(paper);
        let (area_right, area_bottom) = far_corner(area);
        let long_edge = paper.width_um.max(paper.height_um);
        let (length, head) = if long_edge >= ASME_Y14_1_D_SIZE_LONG_EDGE_UM {
            ASME_Y14_1_ARROW_LARGE_UM
        } else {
            ASME_Y14_1_ARROW_SMALL_UM
        };
        // A 60° point leaves the head as wide as 2 tan 30° of its length.
        let half_width = (head * 577 / 1_000) as i64;
        let centre = (paper_right / 2, paper_bottom / 2);
        // (tip, outward unit direction, margin depth)
        for (tip, outward, margin) in [
            (
                (centre.0, area.y_um),
                (0_i64, -1_i64),
                area.y_um - paper.y_um,
            ),
            ((centre.0, area_bottom), (0, 1), paper_bottom - area_bottom),
            ((area.x_um, centre.1), (-1, 0), area.x_um - paper.x_um),
            ((area_right, centre.1), (1, 0), paper_right - area_right),
        ] {
            let length = (length as i64).min(margin);
            let head = (head as i64).min(length);
            let base = (tip.0 + outward.0 * head, tip.1 + outward.1 * head);
            let across = (outward.1, outward.0);
            artwork.push(
                DrawingSheetFrameLayer::CentringMarks,
                DrawingSheetFrameMark::FilledPolygon {
                    points: vec![
                        tip,
                        (
                            base.0 + across.0 * half_width,
                            base.1 + across.1 * half_width,
                        ),
                        (
                            base.0 - across.0 * half_width,
                            base.1 - across.1 * half_width,
                        ),
                    ],
                },
            );
            artwork.line(
                DrawingSheetFrameLayer::CentringMarks,
                base,
                (tip.0 + outward.0 * length, tip.1 + outward.1 * length),
                ASME_Y14_1_ZONE_LINE_UM,
            );
        }
    }

    /// ISO 5457 3.1: the size designation in the bottom border at the right
    /// corner, lettered in the strip between the grid reference border and
    /// the trimmed edge so it never meets a grid reference.
    fn iso_size_designation(
        &self,
        artwork: &mut DrawingSheetFrameArtwork,
        geometry: &DrawingSheetGeometry,
    ) {
        let AuthoredDrawingSheetSize::Standard { standard } = &self.authored_size else {
            return;
        };
        let text = standard.paper_size();
        let (paper_right, paper_bottom) = far_corner(geometry.paper);
        let (_, band_bottom) = far_corner(geometry.zone_band);
        let strip_um = paper_bottom - band_bottom;
        let lettering_um = ISO_5457_SIZE_DESIGNATION_UM;
        if strip_um < (lettering_um * 12 / 10) as i64 {
            return;
        }
        let width_um =
            (drawing_sheet_mono_advance_um(lettering_um) * text.chars().count() as u64) as i64;
        // Clear of the trimming mark that occupies the corner by 2 mm.
        let right = paper_right - ISO_5457_TRIM_MARK_UM.0 as i64 - 2_000;
        artwork.push(
            DrawingSheetFrameLayer::SizeDesignation,
            DrawingSheetFrameMark::Text {
                centre: (right - width_um / 2, band_bottom + strip_um / 2),
                text: text.to_owned(),
                lettering_um,
            },
        );
    }
}

/// Whether an axis-aligned segment reaches inside a rectangle, its edges
/// included.
fn segment_meets_rect(from: (i64, i64), to: (i64, i64), rect: DrawingSheetRect) -> bool {
    let (right, bottom) = far_corner(rect);
    from.0.min(to.0) <= right
        && from.0.max(to.0) >= rect.x_um
        && from.1.min(to.1) <= bottom
        && from.1.max(to.1) >= rect.y_um
}

/// A rectangle's right and bottom edges.
const fn far_corner(rect: DrawingSheetRect) -> (i64, i64) {
    (
        rect.x_um + rect.width_um as i64,
        rect.y_um + rect.height_um as i64,
    )
}
