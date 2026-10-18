//! ASME Y14.1's title block as a designed form.
//!
//! ASME Y14.1-2012 5.1 puts the title block of Fig 5-1 or Fig 5-2 on sheet
//! one, 6.1 in the lower right corner of the format, and 4.3 lets computer
//! output drop the figures' line styles but not the sizes or the relative
//! positions of the headings and their contents. Fig 5-1 serves A, B and C
//! sizes and Fig 5-2 D and larger; both are dimensioned in inches, every
//! division measured from the block's right edge:
//!
//! ```text
//! Fig 5-1 (A-C), 6.25 x 2.00 in            Fig 5-2 (D-E), 7.62 x 2.50 in
//! |<- 2.00 ->|<-------- 4.25 -------->|     |<- 2.50 ->|<-------- 5.12 -------->|
//! | E        | A                 0.62 |     | E        | A                 0.62 |
//! |          | B  TITLE          0.76 |     |          | B  TITLE          1.13 |
//! |          | SIZE|DAI |DWG NO |REV  |     |          | SIZE|DAI |DWG NO |REV  |
//! |          | .38 |1.12| 2.37  |.38  |     |          | .37 |1.25| 3.00  |.50  |
//! |          | SCALE|WEIGHT|SHEET 0.25|     |          | SCALE|WEIGHT|SHEET 0.25|
//! |          | 1.00 | 1.50 | 1.75     |     |          | 1.37 | 2.00 | 1.75     |
//! ```
//!
//! Block E, which the figures leave undivided, is drawn as a NAME | DATE
//! grid: a contract number and the column headings over the A block, then
//! one row each for the people who draw, check, engineer and approve the
//! sheet. The blocks F and G the figures dash in as optional are absorbed
//! into E, as 6.2 allows.
//!
//! An ARCH sheet's size designation, such as `ARCH D`, does not fit the
//! figures' 0.38 in SIZE block at the height ASME Y14.2 letters it. ASME
//! Y14.1 does not define the ARCH series; RSpice draws its sheets in the
//! format with the block widened at its left by what the designation needs,
//! taken by SIZE and SCALE, so every division the figures measure from the
//! right edge stands where they put it. 5.3 makes every block size a minimum.
//!
//! The fields RSpice carries that have no ASME block (project, cell / view,
//! document status, classification and language) stand in a strip of their
//! own directly above the block, as 10 places additional data blocks, and
//! never inside blocks A to L. The strip appears only while the sheet shows
//! one of them. The figures have no document type, and neither does the
//! form.
//!
//! Lettering follows ASME Y14.2 (the heights are known from secondary
//! restatements of its Table 1): headings 0.10 in (2.5 mm), entries 0.12 in
//! (3 mm), and the title, size, DAI, drawing number and revision 0.12 in on
//! A to C and 0.24 in (6 mm) on D and larger. Headings are the figures' own
//! capitals. The bottom row is 0.25 in high, so each heading there stands
//! before its value on one line, as the figures letter `SCALE` and `SHEET`.
//!
//! 5.6 asks for thick lines on the outline and the block's main divisions
//! and thin ones on its minor subdivisions. RSpice reads the main divisions
//! as the rules that cross the block: under A, under B, under the drawing
//! number's row, and the rule beside E. Every rule inside a row and every
//! rule inside E is a minor subdivision.

use super::*;

use DrawingSheetTitleFieldId as Field;

/// A length the figures give in hundredths of an inch.
const fn hundredths(count: u64) -> u64 {
    count * 254
}

/// ASME Y14.2's thick line, at least 0.6 mm: the outline and the main
/// divisions.
const THICK_UM: u64 = 600;
/// ASME Y14.2's thin line, at least 0.3 mm: the minor subdivisions.
const THIN_UM: u64 = 300;
/// Block headings, 0.10 in.
const HEADING_UM: u64 = 2_500;
/// Every other entry, 0.12 in.
const ENTRY_UM: u64 = 3_000;
/// The title, size, DAI, drawing number and revision on D and larger sizes,
/// 0.24 in.
const KEY_LARGE_UM: u64 = 6_000;
/// Space between a rule and the entry beside it. Block E's 0.90 in name
/// column holds eight characters only at this padding.
const PADDING_UM: u64 = 1_000;
/// Space between a rule and the heading beside it: Fig 5-2's 0.37 in SIZE
/// block holds its four-letter heading at 2.5 mm only this close to its
/// rules. The face's own side bearings keep the capitals off them.
const HEADING_PADDING_UM: u64 = 400;
/// Least space between a rule and the capitals below it, and between the
/// descenders of a line and the rule under it.
const TOP_CLEAR_UM: u64 = 600;
const FOOT_CLEAR_UM: u64 = 500;

/// Which of the two figures a sheet's block follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsmeY141TitleBlockSize {
    /// Fig 5-1, for A, B and C sizes: 6.25 × 2.00 in.
    Small,
    /// Fig 5-2, for D and larger sizes: 7.62 × 2.50 in.
    Large,
}

/// The ASME form a sheet carries: its figure, whether its SIZE block is
/// widened for an ARCH designation, and whether the supplementary strip
/// stands above it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsmeY141TitleBlockForm {
    pub size: AsmeY141TitleBlockSize,
    pub architectural: bool,
    pub supplementary: bool,
}

impl AsmeY141TitleBlockForm {
    /// Width of SIZE: the figure's, or on an ARCH sheet enough for `ARCH C`
    /// at 3 mm on Fig 5-1 and `ARCH E1` at 6 mm on Fig 5-2.
    const fn size_block_um(self) -> u64 {
        match (self.size, self.architectural) {
            (AsmeY141TitleBlockSize::Small, false) => hundredths(38),
            (AsmeY141TitleBlockSize::Large, false) => hundredths(37),
            (AsmeY141TitleBlockSize::Small, true) => hundredths(70),
            (AsmeY141TitleBlockSize::Large, true) => hundredths(160),
        }
    }

    /// What an ARCH sheet's SIZE block adds to the figure's width.
    const fn widened_um(self) -> u64 {
        let figure = AsmeY141TitleBlockForm {
            architectural: false,
            ..self
        };
        self.size_block_um() - figure.size_block_um()
    }

    /// Width of block E, the approvals.
    const fn approvals_um(self) -> u64 {
        match self.size {
            AsmeY141TitleBlockSize::Small => hundredths(200),
            AsmeY141TitleBlockSize::Large => hundredths(250),
        }
    }

    /// Width of the part of the block right of E: 4.25 or 5.12 in, and an
    /// ARCH sheet's widening.
    const fn right_part_um(self) -> u64 {
        let figure = match self.size {
            AsmeY141TitleBlockSize::Small => hundredths(425),
            AsmeY141TitleBlockSize::Large => hundredths(512),
        };
        figure + self.widened_um()
    }

    #[must_use]
    pub const fn width_um(self) -> u64 {
        self.approvals_um() + self.right_part_um()
    }

    /// Height of the figure's block: 2.00 or 2.50 in.
    #[must_use]
    pub const fn figure_height_um(self) -> u64 {
        match self.size {
            AsmeY141TitleBlockSize::Small => hundredths(200),
            AsmeY141TitleBlockSize::Large => hundredths(250),
        }
    }

    /// Height of the supplementary strip, when it stands above the block:
    /// the drawing number row's 0.37 in, which seats a heading over an
    /// entry at 3 mm.
    #[must_use]
    pub const fn supplementary_um(self) -> u64 {
        if self.supplementary {
            hundredths(37)
        } else {
            0
        }
    }

    /// Height of the block and the strip above it.
    #[must_use]
    pub const fn height_um(self) -> u64 {
        self.figure_height_um() + self.supplementary_um()
    }

    #[must_use]
    pub const fn dimensions_um(self) -> (u64, u64) {
        (self.width_um(), self.height_um())
    }

    /// The form's name wherever RSpice offers or reports it. An ARCH sheet
    /// is drawn in the format without being an ASME size, so its block is
    /// named for the format rather than claimed as the standard's.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match (self.size, self.architectural) {
            (AsmeY141TitleBlockSize::Small, false) => "ASME Y14.1 · 6.25 × 2.00 in",
            (AsmeY141TitleBlockSize::Large, false) => "ASME Y14.1 · 7.62 × 2.50 in",
            (AsmeY141TitleBlockSize::Small, true) => "ASME Y14.1 format · 6.57 × 2.00 in",
            (AsmeY141TitleBlockSize::Large, true) => "ASME Y14.1 format · 8.85 × 2.50 in",
        }
    }

    /// Height of the title, size, DAI, drawing number and revision.
    const fn key_lettering_um(self) -> u64 {
        match self.size {
            AsmeY141TitleBlockSize::Small => ENTRY_UM,
            AsmeY141TitleBlockSize::Large => KEY_LARGE_UM,
        }
    }

    /// Height of each row right of E, from the top: A, B, the drawing
    /// number's row and the bottom row.
    const fn row_heights_um(self) -> [u64; 4] {
        match self.size {
            AsmeY141TitleBlockSize::Small => [
                hundredths(62),
                hundredths(76),
                hundredths(37),
                hundredths(25),
            ],
            AsmeY141TitleBlockSize::Large => [
                hundredths(62),
                hundredths(113),
                hundredths(50),
                hundredths(25),
            ],
        }
    }

    /// Widths of SIZE, DAI, DWG NO and REV.
    const fn number_row_um(self) -> [u64; 4] {
        match self.size {
            AsmeY141TitleBlockSize::Small => [
                self.size_block_um(),
                hundredths(112),
                hundredths(237),
                hundredths(38),
            ],
            AsmeY141TitleBlockSize::Large => [
                self.size_block_um(),
                hundredths(125),
                hundredths(300),
                hundredths(50),
            ],
        }
    }

    /// Widths of SCALE, WEIGHT and SHEET.
    const fn bottom_row_um(self) -> [u64; 3] {
        match self.size {
            AsmeY141TitleBlockSize::Small => [
                hundredths(100) + self.widened_um(),
                hundredths(150),
                hundredths(175),
            ],
            AsmeY141TitleBlockSize::Large => [
                hundredths(137) + self.widened_um(),
                hundredths(200),
                hundredths(175),
            ],
        }
    }

    /// Widths of block E's NAME and DATE columns. A date column of 1.10 in
    /// holds `YYYY-MM-DD` at 3 mm.
    const fn approval_columns_um(self) -> [u64; 2] {
        match self.size {
            AsmeY141TitleBlockSize::Small => [hundredths(90), hundredths(110)],
            AsmeY141TitleBlockSize::Large => [hundredths(140), hundredths(110)],
        }
    }

    /// Height of each of block E's four signature rows: what is left of the
    /// block below the contract number and the column headings, which
    /// together stand as tall as block A beside them.
    const fn signature_row_um(self) -> u64 {
        (self.figure_height_um() - hundredths(62)) / 4
    }

    /// Widths of the supplementary strip's PROJECT, CELL / VIEW, STATUS,
    /// CLASSIFICATION and LANG cells. STATUS holds the longest status at
    /// 3 mm, CLASSIFICATION its heading at 2.5 mm, LANG an ISO 639 code;
    /// CELL / VIEW takes the rest, and an ARCH block's widening.
    const fn supplementary_cells_um(self) -> [u64; 5] {
        let [project, cell_view] = match self.size {
            AsmeY141TitleBlockSize::Small => [hundredths(100), hundredths(200)],
            AsmeY141TitleBlockSize::Large => [hundredths(125), hundredths(312)],
        };
        [
            project,
            cell_view + self.widened_um(),
            hundredths(150),
            hundredths(125),
            hundredths(50),
        ]
    }
}

/// The fields with no ASME block, which the form letters in a strip of their
/// own directly above the block, as ASME Y14.1 10 places additional data,
/// with each heading over its entry. Each is optional; a hidden one gives
/// its width to the cell on its left, or on its right when it starts the
/// strip, so the rules that remain stand where the strip puts them.
pub const ASME_Y14_1_SUPPLEMENTARY_FIELDS: [DrawingSheetTitleFieldId; 5] = [
    Field::Project,
    Field::CellView,
    Field::DocumentStatus,
    Field::Classification,
    Field::Language,
];

/// Each supplementary field's heading, and whether RSpice's wording for it
/// is lettered in capitals.
const SUPPLEMENTARY_HEADINGS: [(&str, bool); 5] = [
    ("PROJECT", false),
    ("CELL / VIEW", false),
    ("STATUS", true),
    ("CLASSIFICATION", false),
    ("LANG", false),
];

/// The fields on the ASME form, in the order it is read: block E, then A,
/// B, the drawing number's row and the bottom row.
pub const ASME_Y14_1_FORM_FIELDS: [DrawingSheetTitleFieldId; 16] = [
    Field::ContractNumber,
    Field::DrawnBy,
    Field::DrawnDate,
    Field::CheckedBy,
    Field::CheckedDate,
    Field::Engineer,
    Field::EngineerDate,
    Field::ApprovedBy,
    Field::ApprovedDate,
    Field::Organization,
    Field::SheetTitle,
    Field::PaperSize,
    Field::CageCode,
    Field::DocumentId,
    Field::Revision,
    Field::Page,
];

/// The capitals ASME Y14.35 letters revisions with: A to Y, less I, O, Q,
/// S and X, which read as numerals or other letters, and Z.
const fn y14_35_revision_letter(letter: char) -> bool {
    letter.is_ascii_uppercase() && !matches!(letter, 'I' | 'O' | 'Q' | 'S' | 'X' | 'Z')
}

/// What ASME Y14.35 advises against in a revision, if `revision` does it:
/// anything but capitals from its sequence, A to Y and then AA, AB and on,
/// or more than two of them. A dash, the initial issue, is none of these.
/// An advisory, not a refusal: a document already issued under such a
/// revision keeps it. (ASME Y14.35's sequence is known here from a
/// secondary summary of the standard.)
#[must_use]
pub fn drawing_sheet_asme_revision_advisory(revision: &str) -> Option<&'static str> {
    let revision = revision.trim();
    if revision.is_empty() || revision == "-" {
        None
    } else if !revision.chars().all(y14_35_revision_letter) {
        Some("ASME Y14.35 letters a revision in capitals from A, without I, O, Q, S, X or Z.")
    } else if revision.chars().count() > 2 {
        Some("ASME Y14.35 gives a revision two letters at most.")
    } else {
        None
    }
}

impl DrawingSheetTitleBlockTemplate {
    /// What the standard behind this block advises against in `revision`:
    /// ASME Y14.35 on the ASME form, ISO 7200 on every other block.
    #[must_use]
    pub fn revision_advisory(self, revision: &str) -> Option<&'static str> {
        match self {
            Self::AsmeY141 => drawing_sheet_asme_revision_advisory(revision),
            Self::Compact | Self::Standard | Self::OrganizationManaged | Self::None => {
                drawing_sheet_revision_advisory(revision)
            }
        }
    }
}

impl DrawingSheetStandard {
    /// The drawing size designation ASME Y14.1's SIZE block letters: Table
    /// 4-1's bare letter for an ANSI sheet, and the series named for an ARCH
    /// sheet, which the standard does not define.
    #[must_use]
    pub const fn asme_size_designation(self) -> Option<&'static str> {
        match self {
            Self::AnsiA => Some("A"),
            Self::AnsiB => Some("B"),
            Self::AnsiC => Some("C"),
            Self::AnsiD => Some("D"),
            Self::AnsiE => Some("E"),
            Self::ArchA => Some("ARCH A"),
            Self::ArchB => Some("ARCH B"),
            Self::ArchC => Some("ARCH C"),
            Self::ArchD => Some("ARCH D"),
            Self::ArchE => Some("ARCH E"),
            Self::ArchE1 => Some("ARCH E1"),
            Self::IsoA5
            | Self::IsoA4
            | Self::IsoA3
            | Self::IsoA2
            | Self::IsoA1
            | Self::IsoA0
            | Self::JisB5
            | Self::JisB4
            | Self::JisB3
            | Self::JisB2 => None,
        }
    }

    /// The figure ASME Y14.1 gives this size's title block: Fig 5-1 for A
    /// to C and Fig 5-2 for D and larger. RSpice gives the ARCH series the
    /// figure of the ANSI size it stands among.
    const fn asme_title_block_size(self) -> AsmeY141TitleBlockSize {
        match self {
            Self::AnsiD | Self::AnsiE | Self::ArchD | Self::ArchE | Self::ArchE1 => {
                AsmeY141TitleBlockSize::Large
            }
            _ => AsmeY141TitleBlockSize::Small,
        }
    }
}

/// The long edge from which a custom sheet takes Fig 5-2's block: an ANSI
/// D sheet's 34 in.
const ASME_LARGE_BLOCK_LONG_EDGE_UM: u64 = 863_600;

impl SchematicSheetFormat {
    /// The ASME form this sheet's size calls for, before the drawing area is
    /// asked whether it holds it.
    fn preferred_asme_title_block(&self) -> AsmeY141TitleBlockForm {
        let (size, series) = match &self.authored_size {
            AuthoredDrawingSheetSize::Standard { standard } => {
                (standard.asme_title_block_size(), Some(standard.series()))
            }
            AuthoredDrawingSheetSize::Custom { snapshot } => {
                let long_edge = snapshot.portrait_height_um;
                (
                    if long_edge >= ASME_LARGE_BLOCK_LONG_EDGE_UM {
                        AsmeY141TitleBlockSize::Large
                    } else {
                        AsmeY141TitleBlockSize::Small
                    },
                    snapshot.started_from,
                )
            }
        };
        AsmeY141TitleBlockForm {
            size,
            architectural: series == Some(DrawingSheetStandardSeries::Architectural),
            supplementary: ASME_Y14_1_SUPPLEMENTARY_FIELDS
                .into_iter()
                .any(|field| self.title_field_shown(field)),
        }
    }

    /// The ASME form the sheet carries: its size's figure, or Fig 5-1 where
    /// the drawing area cannot hold Fig 5-2, upright in the lower right
    /// corner either way. `None` when the drawing area holds neither.
    #[must_use]
    pub fn asme_title_block_form(&self) -> Option<AsmeY141TitleBlockForm> {
        let preferred = self.preferred_asme_title_block();
        let (area_width_um, area_height_um) = self.drawing_area_extent_um()?;
        [
            preferred,
            AsmeY141TitleBlockForm {
                size: AsmeY141TitleBlockSize::Small,
                ..preferred
            },
        ]
        .into_iter()
        .find(|form| {
            let (width_um, height_um) = form.dimensions_um();
            width_um <= area_width_um && height_um <= area_height_um
        })
    }

    /// The template's name as this sheet's Page Setup offers it: the ASME
    /// form with the size of the block its sheet carries.
    #[must_use]
    pub fn title_block_template_label(
        &self,
        template: DrawingSheetTitleBlockTemplate,
    ) -> &'static str {
        match template {
            DrawingSheetTitleBlockTemplate::AsmeY141 => self
                .asme_title_block_form()
                .unwrap_or_else(|| self.preferred_asme_title_block())
                .label(),
            other => other.label(),
        }
    }

    /// What the SIZE block letters: the designation of a standard size, or
    /// the name a custom size was given.
    fn asme_size_text(&self) -> DrawingSheetTitleText {
        match &self.authored_size {
            AuthoredDrawingSheetSize::Standard { standard } => {
                standard.asme_size_designation().map_or(
                    DrawingSheetTitleText::Field(Field::PaperSize),
                    |designation| DrawingSheetTitleText::Fixed(designation),
                )
            }
            AuthoredDrawingSheetSize::Custom { .. } => {
                DrawingSheetTitleText::Field(Field::PaperSize)
            }
        }
    }
}

/// Where lines are seated in a cell, in the block's frame. A seat is the
/// point the canvas centres a line on and the plot sets its baseline 3/8 of
/// an em below (see [`drawing_sheet_mono_baseline_offset_um`]).
mod seat {
    use super::*;

    /// A line whose capitals start [`TOP_CLEAR_UM`] below `top_um`.
    pub(super) const fn under(top_um: u64, lettering_um: u64) -> u64 {
        top_um + TOP_CLEAR_UM + drawing_sheet_mono_em_um(lettering_um) * 7 / 20
    }

    /// A line whose descenders stop [`FOOT_CLEAR_UM`] above `bottom_um`.
    pub(super) const fn over(bottom_um: u64, lettering_um: u64) -> u64 {
        bottom_um - FOOT_CLEAR_UM - drawing_sheet_mono_em_um(lettering_um) * 12 / 20
    }

    /// A line on the baseline that centres capitals `cap_um` high in the
    /// stretch from `top_um` to `bottom_um`: lines of two heights on one
    /// baseline, such as an inline heading and its value.
    pub(super) const fn on_centred_baseline(
        top_um: u64,
        bottom_um: u64,
        cap_um: u64,
        lettering_um: u64,
    ) -> u64 {
        let baseline_um = top_um + (bottom_um - top_um + cap_um) / 2;
        baseline_um - drawing_sheet_mono_baseline_offset_um(lettering_um)
    }

    /// The first of `lines` lines, `pitch_um` apart, centred between `top_um`
    /// and `bottom_um` from the first line's capitals to the last line's
    /// descenders.
    pub(super) const fn first_of_centred(
        top_um: u64,
        bottom_um: u64,
        lettering_um: u64,
        lines: u64,
        pitch_um: u64,
    ) -> u64 {
        let em_um = drawing_sheet_mono_em_um(lettering_um);
        let rise_um = em_um * 7 / 20;
        let ink_um = rise_um + pitch_um * (lines - 1) + em_um * 12 / 20;
        let room_um = bottom_um - top_um;
        let slack_um = if room_um > ink_um {
            room_um - ink_um
        } else {
            0
        };
        top_um + slack_um / 2 + rise_um
    }
}

/// ISO 3098's least baseline spacing, 19/10 of the lettering height, which
/// RSpice keeps on every form.
const fn line_pitch_um(lettering_um: u64) -> u64 {
    lettering_um * 19 / 10
}

/// A cell under construction.
struct Cell {
    x_um: u64,
    y_um: u64,
    width_um: u64,
    height_um: u64,
}

impl Cell {
    const fn bottom_um(&self) -> u64 {
        self.y_um + self.height_um
    }

    const fn lettered_um(&self) -> u64 {
        self.width_um.saturating_sub(PADDING_UM * 2)
    }

    /// A heading over the cell's contents.
    fn heading_over(&self, text: &'static str) -> DrawingSheetTitleCaption {
        DrawingSheetTitleCaption {
            text,
            placement: DrawingSheetTitleCaptionPlacement::Over,
            x_um: self.x_um + HEADING_PADDING_UM,
            seat_um: seat::under(self.y_um, HEADING_UM),
            width_um: self.width_um.saturating_sub(HEADING_PADDING_UM * 2),
            lettering_um: HEADING_UM,
        }
    }

    /// A value under a heading, its last line's descenders clear of the rule
    /// below.
    fn value_under_heading(
        &self,
        text: DrawingSheetTitleText,
        lettering_um: u64,
    ) -> DrawingSheetTitleValue {
        DrawingSheetTitleValue {
            text,
            x_um: self.x_um + PADDING_UM,
            seat_um: seat::over(self.bottom_um(), lettering_um),
            width_um: self.lettered_um(),
            lettering_um,
            emphasised: matches!(text, DrawingSheetTitleText::Field(field) if field.emphasised()),
            max_lines: 1,
            line_pitch_um: 0,
            upper_case: false,
        }
    }

    /// Up to `lines` lines centred in the stretch from `top_um` to the
    /// cell's foot.
    fn value_centred(
        &self,
        field: DrawingSheetTitleFieldId,
        top_um: u64,
        lettering_um: u64,
        lines: u8,
    ) -> DrawingSheetTitleValue {
        let pitch_um = line_pitch_um(lettering_um);
        DrawingSheetTitleValue {
            text: DrawingSheetTitleText::Field(field),
            x_um: self.x_um + PADDING_UM,
            seat_um: seat::first_of_centred(
                top_um,
                self.bottom_um() - FOOT_CLEAR_UM,
                lettering_um,
                u64::from(lines),
                pitch_um,
            ),
            width_um: self.lettered_um(),
            lettering_um,
            emphasised: field.emphasised(),
            max_lines: lines,
            line_pitch_um: pitch_um,
            upper_case: false,
        }
    }

    /// A heading lettered before its value on one line, both on the
    /// baseline that centres the value's capitals in the cell.
    fn heading_inline(&self, text: &'static str) -> DrawingSheetTitleCaption {
        let width_um = drawing_sheet_mono_advance_um(HEADING_UM) * text.len() as u64;
        DrawingSheetTitleCaption {
            text,
            placement: DrawingSheetTitleCaptionPlacement::Inline,
            x_um: self.x_um + PADDING_UM,
            seat_um: seat::on_centred_baseline(self.y_um, self.bottom_um(), ENTRY_UM, HEADING_UM),
            width_um: width_um.min(self.lettered_um()),
            lettering_um: HEADING_UM,
        }
    }

    /// The value after an inline heading, one heading space after it: Fig
    /// 5-1's 1.00 in SCALE block holds `SCALE NONE` at that spacing.
    fn value_after(
        &self,
        heading: DrawingSheetTitleCaption,
        text: DrawingSheetTitleText,
    ) -> DrawingSheetTitleValue {
        let x_um = heading.x_um + heading.width_um + drawing_sheet_mono_advance_um(HEADING_UM);
        DrawingSheetTitleValue {
            text,
            x_um,
            seat_um: seat::on_centred_baseline(self.y_um, self.bottom_um(), ENTRY_UM, ENTRY_UM),
            width_um: (self.x_um + self.width_um - PADDING_UM).saturating_sub(x_um),
            lettering_um: ENTRY_UM,
            emphasised: false,
            max_lines: 1,
            line_pitch_um: 0,
            upper_case: true,
        }
    }

    fn finish(
        self,
        caption: Option<DrawingSheetTitleCaption>,
        values: Vec<DrawingSheetTitleValue>,
    ) -> DrawingSheetTitleCell {
        DrawingSheetTitleCell {
            x_um: self.x_um,
            y_um: self.y_um,
            width_um: self.width_um,
            height_um: self.height_um,
            caption,
            values,
        }
    }
}

/// The people block E names, top to bottom, with each row's heading.
const SIGNATURES: [(&str, DrawingSheetTitleFieldId, DrawingSheetTitleFieldId); 4] = [
    ("DRAWN", Field::DrawnBy, Field::DrawnDate),
    ("CHECKED", Field::CheckedBy, Field::CheckedDate),
    ("ENGR", Field::Engineer, Field::EngineerDate),
    ("APPROVED", Field::ApprovedBy, Field::ApprovedDate),
];

/// Lay out the ASME form `form` for the sheet `format` authors.
pub(super) fn asme_y14_1_form_layout(
    format: &SchematicSheetFormat,
    form: AsmeY141TitleBlockForm,
) -> DrawingSheetTitleLayout {
    let mut cells = approval_cells(form);
    let x0_um = form.approvals_um();
    let right_um = form.right_part_um();
    let [a_um, b_um, number_um, bottom_um] = form.row_heights_um();
    let key_um = form.key_lettering_um();

    // A: the name of the design activity, with no heading.
    let block_a = Cell {
        x_um: x0_um,
        y_um: 0,
        width_um: right_um,
        height_um: a_um,
    };
    let organization = block_a.value_centred(Field::Organization, 0, ENTRY_UM, 2);
    cells.push(block_a.finish(None, vec![organization]));

    // B: the title, under its heading.
    let block_b = Cell {
        x_um: x0_um,
        y_um: a_um,
        width_um: right_um,
        height_um: b_um,
    };
    let heading = block_b.heading_over("TITLE");
    let below_heading_um =
        heading.seat_um + drawing_sheet_mono_baseline_offset_um(HEADING_UM) + TOP_CLEAR_UM;
    let title = block_b.value_centred(Field::SheetTitle, below_heading_um, key_um, 2);
    cells.push(block_b.finish(Some(heading), vec![title]));

    // SIZE | DAI | DWG NO | REV, each heading over its entry.
    let number_y_um = a_um + b_um;
    let mut x_um = x0_um;
    for (width_um, (heading, text)) in form.number_row_um().into_iter().zip([
        ("SIZE", format.asme_size_text()),
        ("DAI", DrawingSheetTitleText::Field(Field::CageCode)),
        ("DWG NO", DrawingSheetTitleText::Field(Field::DocumentId)),
        ("REV", DrawingSheetTitleText::Field(Field::Revision)),
    ]) {
        let cell = Cell {
            x_um,
            y_um: number_y_um,
            width_um,
            height_um: number_um,
        };
        let caption = cell.heading_over(heading);
        let value = cell.value_under_heading(text, key_um);
        cells.push(cell.finish(Some(caption), vec![value]));
        x_um += width_um;
    }

    // SCALE | WEIGHT | SHEET, each heading before its entry. A schematic is
    // not drawn to a scale, which Y14.1 6.2 enters as NONE; the weight is
    // left blank, as a schematic has none.
    let bottom_y_um = number_y_um + number_um;
    let mut x_um = x0_um;
    for (width_um, (heading, text)) in form.bottom_row_um().into_iter().zip([
        ("SCALE", Some(DrawingSheetTitleText::Fixed("NONE"))),
        ("WEIGHT", None),
        ("SHEET", Some(DrawingSheetTitleText::Field(Field::Page))),
    ]) {
        let cell = Cell {
            x_um,
            y_um: bottom_y_um,
            width_um,
            height_um: bottom_um,
        };
        let caption = cell.heading_inline(heading);
        let values = text
            .map(|text| cell.value_after(caption, text))
            .into_iter()
            .collect();
        cells.push(cell.finish(Some(caption), values));
        x_um += width_um;
    }

    // The block stands under the supplementary strip, whose foot is the
    // block's outline.
    let top_um = form.supplementary_um();
    let mut all = supplementary_cells(format, form);
    all.extend(cells.into_iter().map(|cell| lowered(cell, top_um)));
    let main_divisions = [a_um, a_um + b_um, bottom_y_um].map(|y_um| top_um + y_um);
    DrawingSheetTitleLayout::from_weighted_cells(
        form.width_um(),
        form.height_um(),
        all,
        THICK_UM,
        HEADING_PADDING_UM,
        |rule| {
            let main = if rule.from_y_um == rule.to_y_um {
                rule.from_y_um == top_um
                    || (rule.from_x_um >= x0_um && main_divisions.contains(&rule.from_y_um))
            } else {
                rule.from_x_um == x0_um && rule.from_y_um >= top_um
            };
            if main { THICK_UM } else { THIN_UM }
        },
    )
}

/// `cell` moved `by_um` down the block, with everything it letters.
fn lowered(mut cell: DrawingSheetTitleCell, by_um: u64) -> DrawingSheetTitleCell {
    cell.y_um += by_um;
    if let Some(caption) = &mut cell.caption {
        caption.seat_um += by_um;
    }
    for value in &mut cell.values {
        value.seat_um += by_um;
    }
    cell
}

/// The supplementary strip's cells for the fields the sheet shows: each
/// hidden field gives its width to the shown cell on its left, or on its
/// right when it starts the strip. None when the sheet shows none.
fn supplementary_cells(
    format: &SchematicSheetFormat,
    form: AsmeY141TitleBlockForm,
) -> Vec<DrawingSheetTitleCell> {
    if !form.supplementary {
        return Vec::new();
    }
    let shown = ASME_Y14_1_SUPPLEMENTARY_FIELDS.map(|field| format.title_field_shown(field));
    let mut widths = form.supplementary_cells_um();
    for index in 0..widths.len() {
        if shown[index] {
            continue;
        }
        let given = std::mem::take(&mut widths[index]);
        let neighbour = (0..index)
            .rev()
            .find(|left| shown[*left])
            .or_else(|| (index + 1..widths.len()).find(|right| shown[*right]));
        if let Some(neighbour) = neighbour {
            widths[neighbour] += given;
        }
    }
    let mut x_um = 0;
    let mut cells = Vec::new();
    for (index, field) in ASME_Y14_1_SUPPLEMENTARY_FIELDS.into_iter().enumerate() {
        if !shown[index] {
            continue;
        }
        let (heading, upper_case) = SUPPLEMENTARY_HEADINGS[index];
        let cell = Cell {
            x_um,
            y_um: 0,
            width_um: widths[index],
            height_um: form.supplementary_um(),
        };
        let caption = cell.heading_over(heading);
        let value = DrawingSheetTitleValue {
            upper_case,
            ..cell.value_under_heading(DrawingSheetTitleText::Field(field), ENTRY_UM)
        };
        x_um += cell.width_um;
        cells.push(cell.finish(Some(caption), vec![value]));
    }
    cells
}

/// Block E: the contract number, the NAME and DATE headings, and a row for
/// each person who signs.
fn approval_cells(form: AsmeY141TitleBlockForm) -> Vec<DrawingSheetTitleCell> {
    let [name_um, date_um] = form.approval_columns_um();
    let width_um = name_um + date_um;
    let contract_um = hundredths(37);
    let headings_um = hundredths(25);
    let row_um = form.signature_row_um();
    let mut cells = Vec::with_capacity(3 + SIGNATURES.len() * 2);

    let contract = Cell {
        x_um: 0,
        y_um: 0,
        width_um,
        height_um: contract_um,
    };
    let caption = contract.heading_over("CONTRACT NO");
    let number = contract.value_under_heading(
        DrawingSheetTitleText::Field(Field::ContractNumber),
        ENTRY_UM,
    );
    cells.push(contract.finish(Some(caption), vec![number]));
    for (x_um, cell_um, heading) in [(0, name_um, "NAME"), (name_um, date_um, "DATE")] {
        let cell = Cell {
            x_um,
            y_um: contract_um,
            width_um: cell_um,
            height_um: headings_um,
        };
        let caption = DrawingSheetTitleCaption {
            seat_um: seat::on_centred_baseline(cell.y_um, cell.bottom_um(), HEADING_UM, HEADING_UM),
            ..cell.heading_inline(heading)
        };
        cells.push(cell.finish(Some(caption), Vec::new()));
    }
    for (row, (heading, name, date)) in SIGNATURES.into_iter().enumerate() {
        let y_um = contract_um + headings_um + row_um * row as u64;
        let name_cell = Cell {
            x_um: 0,
            y_um,
            width_um: name_um,
            height_um: row_um,
        };
        let caption = name_cell.heading_over(heading);
        let name = name_cell.value_under_heading(DrawingSheetTitleText::Field(name), ENTRY_UM);
        cells.push(name_cell.finish(Some(caption), vec![name]));
        // The date stands on the name's line, under the DATE heading.
        let date_cell = Cell {
            x_um: name_um,
            y_um,
            width_um: date_um,
            height_um: row_um,
        };
        let date = date_cell.value_under_heading(DrawingSheetTitleText::Field(date), ENTRY_UM);
        cells.push(date_cell.finish(None, vec![date]));
    }
    cells
}
