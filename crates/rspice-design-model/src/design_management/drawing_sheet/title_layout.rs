//! Where a title block letters each field: the cells its template draws, the
//! rules between them, and every line of lettering placed and cut to its
//! cell.
//!
//! A template's layout is a set of cells. Each is a rectangle in the block's
//! upright frame — it may span rows — with an optional caption, lettered over
//! its value or inline before it, and the values it letters, each at its own
//! ISO 3098 height. The canvas, the plot and the sheet previews all draw
//! the cells laid out here rather than working out a grid of their own, so a
//! field cannot sit in one place on screen and another on paper, or be cut
//! short on one and whole on the other.
//!
//! The rules are the cells' own edges: every edge that is not the block's
//! outline is ruled once, and edges of one width that meet end to end are one
//! rule. No rule runs across the block on its own. Each rule and the outline
//! carry the width the block's standard prints them at, so every surface
//! draws the same weights.
//!
//! The layout depends on which fields a sheet shows and never on what they
//! hold, so a cell does not move while a value is typed into it, and a
//! preview that has no values draws the same form the sheet does. Lettering
//! never shrinks to fit: a value or caption longer than its cell holds is cut
//! with an ellipsis, a cell / view path at its start, since the names of the
//! cell and the view are at its end.
//!
//! A cut falls only between grapheme clusters, so a mark is never parted
//! from the letter it sits on. Latin text is measured here, since IBM Plex
//! Mono advances every character it letters alike; the width of any other
//! text is asked of a [`DrawingSheetTitleMeasure`], which the canvas and
//! print answer with the shaped width of the faces that letter it, so both
//! cut a value at the same place. The cut is made in the order the value was
//! typed, and the ellipsis after it stands at the end of the line as it is
//! read. A value whose paragraph is read right to left, such as Arabic or
//! Hebrew, is set against the right of its cell.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use super::*;

/// Width of a title block's outline on paper: ISO 128-20's wide line, the
/// width ISO 5457 gives the frame around the drawing.
pub const DRAWING_SHEET_TITLE_OUTLINE_UM: u64 = 700;

/// Width of the rules between a title block's cells on paper: ISO 128-20's
/// narrow line, half the outline's width.
pub const DRAWING_SHEET_TITLE_RULE_UM: u64 = 350;

/// The width a line of title-block lettering is set at.
///
/// The layout draws no glyphs, so the width of text it cannot count is
/// asked of whoever letters it. [`DrawingSheetTitleCells`] answers without
/// faces.
pub trait DrawingSheetTitleMeasure {
    /// Width on paper of `text` lettered at the ISO 3098 height
    /// `lettering_um`, in the heavier weight when `emphasised`.
    fn width_um(&self, text: &str, lettering_um: u64, emphasised: bool) -> u64;
}

/// The width of a line counted without faces: one IBM Plex Mono advance for
/// each grapheme cluster, and two for an East Asian wide or fullwidth one.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DrawingSheetTitleCells;

impl DrawingSheetTitleMeasure for DrawingSheetTitleCells {
    fn width_um(&self, text: &str, lettering_um: u64, _emphasised: bool) -> u64 {
        let cells = text
            .graphemes(true)
            .map(|cluster| cluster.width().clamp(1, 2) as u64)
            .sum::<u64>();
        cells * drawing_sheet_mono_advance_um(lettering_um)
    }
}

/// The edge of its run a line of title-block lettering is set against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawingSheetTitleAlign {
    /// The left, at the line's `x_um`: a caption, and a value whose
    /// paragraph is read left to right.
    Left,
    /// The right, at the line's `x_um + width_um`: a value whose paragraph
    /// is read right to left.
    Right,
}

impl DrawingSheetTitleAlign {
    /// The edge a value's lines are set against: the right for a paragraph
    /// whose first strongly directional character is read right to left
    /// (UAX #9 rules P2 and P3), and the left otherwise.
    #[must_use]
    pub fn of(value: &str) -> Self {
        match unicode_bidi::get_base_direction(value) {
            unicode_bidi::Direction::Rtl => Self::Right,
            unicode_bidi::Direction::Ltr | unicode_bidi::Direction::Mixed => Self::Left,
        }
    }
}

/// What a value line of a title cell letters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawingSheetTitleText {
    /// The field's resolved value.
    Field(DrawingSheetTitleFieldId),
    /// Wording the form prints itself, whatever the sheet holds.
    Fixed(&'static str),
}

/// Where a cell letters its caption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawingSheetTitleCaptionPlacement {
    /// On a line of its own above the value.
    Over,
    /// On the value's line, before it.
    Inline,
}

/// A cell's caption, placed in the block's upright frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawingSheetTitleCaption {
    pub text: &'static str,
    pub placement: DrawingSheetTitleCaptionPlacement,
    /// Where the lettering starts, from the block's left edge.
    pub x_um: u64,
    /// The point the line is seated on, from the block's top edge: the
    /// canvas centres the line on it and the plot puts the baseline 3/8 of
    /// an em below it (see [`drawing_sheet_mono_baseline_offset_um`]).
    pub seat_um: u64,
    /// How far the lettering may run.
    pub width_um: u64,
    /// ISO 3098 height of the capitals.
    pub lettering_um: u64,
}

impl DrawingSheetTitleCaption {
    /// Characters the caption's width holds at its height.
    #[must_use]
    pub const fn capacity(self) -> usize {
        chars_across(self.width_um, self.lettering_um)
    }

    /// The caption as the cell letters it.
    #[must_use]
    pub fn letter(self) -> String {
        let room = Room::new(
            self.width_um,
            self.lettering_um,
            false,
            &DrawingSheetTitleCells,
        );
        elide_title_text(self.text, &room)
    }
}

/// A value a cell letters, placed in the block's upright frame. A long value
/// wraps at its spaces onto up to `max_lines` lines, each `line_pitch_um`
/// below the last, and what the last line cannot hold is cut.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawingSheetTitleValue {
    pub text: DrawingSheetTitleText,
    /// Where the lettering starts, from the block's left edge.
    pub x_um: u64,
    /// The point the first line is seated on, from the block's top edge.
    pub seat_um: u64,
    /// How far each line may run.
    pub width_um: u64,
    /// ISO 3098 height of the capitals.
    pub lettering_um: u64,
    /// Whether the value is set in the heavier weight.
    pub emphasised: bool,
    pub max_lines: u8,
    pub line_pitch_um: u64,
    /// Whether the value is lettered in capitals whatever case it is
    /// supplied in: wording RSpice supplies on a form whose standard letters
    /// in capitals, such as `1 OF 3` on an ASME sheet.
    pub upper_case: bool,
}

impl DrawingSheetTitleValue {
    /// The field the value letters, if it letters one.
    #[must_use]
    pub const fn field(self) -> Option<DrawingSheetTitleFieldId> {
        match self.text {
            DrawingSheetTitleText::Field(field) => Some(field),
            DrawingSheetTitleText::Fixed(_) => None,
        }
    }

    /// Latin characters one line holds at the value's height. IBM Plex Mono
    /// advances every glyph alike, so this is a measurement, and it holds on
    /// every renderer that letters the height.
    #[must_use]
    pub const fn capacity(self) -> usize {
        chars_across(self.width_um, self.lettering_um)
    }

    /// `value` as the cell letters it, line by line, with the width of text
    /// other than Latin counted by [`DrawingSheetTitleCells`]; see
    /// [`Self::letter_measured`].
    #[must_use]
    pub fn letter(self, value: &str) -> Vec<String> {
        self.letter_measured(value, &DrawingSheetTitleCells)
    }

    /// `value` as the cell letters it, line by line: whole when it fits, and
    /// otherwise wrapped at its spaces and cut with an ellipsis where the
    /// last line runs out. A cell / view path on one line is cut at its
    /// start instead (see [`elide_title_path`]). `measure` gives the width
    /// of text other than Latin.
    #[must_use]
    pub fn letter_measured(
        self,
        value: &str,
        measure: &dyn DrawingSheetTitleMeasure,
    ) -> Vec<String> {
        self.wrap(value, measure).0
    }

    /// Whether the cell cuts `value` short, measured as [`Self::letter`]
    /// measures it.
    #[must_use]
    pub fn elides(self, value: &str) -> bool {
        self.elides_measured(value, &DrawingSheetTitleCells)
    }

    /// Whether the cell cuts `value` short, measured as
    /// [`Self::letter_measured`] measures it.
    #[must_use]
    pub fn elides_measured(self, value: &str, measure: &dyn DrawingSheetTitleMeasure) -> bool {
        self.wrap(value, measure).1
    }

    fn wrap(self, value: &str, measure: &dyn DrawingSheetTitleMeasure) -> (Vec<String>, bool) {
        let value = if self.upper_case {
            std::borrow::Cow::Owned(value.to_uppercase())
        } else {
            std::borrow::Cow::Borrowed(value)
        };
        let room = Room::new(self.width_um, self.lettering_um, self.emphasised, measure);
        if self.field() == Some(DrawingSheetTitleFieldId::CellView) && self.max_lines <= 1 {
            let cut = !room.holds(&value);
            return (vec![elide_title_path(&value, &room)], cut);
        }
        wrap_title_text(&value, &room, usize::from(self.max_lines.max(1)))
    }
}

/// One cell of a title block, in the block's upright frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetTitleCell {
    /// Distance from the block's left edge to the cell's left rule.
    pub x_um: u64,
    /// Distance from the block's top edge to the cell's top rule.
    pub y_um: u64,
    pub width_um: u64,
    pub height_um: u64,
    pub caption: Option<DrawingSheetTitleCaption>,
    /// What the cell letters, top to bottom. A cell with none is a box the
    /// form leaves to be filled in by hand, or empty.
    pub values: Vec<DrawingSheetTitleValue>,
}

impl DrawingSheetTitleCell {
    /// The field the cell is for: the one its first value letters.
    #[must_use]
    pub fn field(&self) -> Option<DrawingSheetTitleFieldId> {
        self.values.first().and_then(|value| value.field())
    }

    /// The value that letters `field`.
    #[must_use]
    pub fn value(&self, field: DrawingSheetTitleFieldId) -> Option<DrawingSheetTitleValue> {
        self.values
            .iter()
            .copied()
            .find(|value| value.field() == Some(field))
    }

    /// The caption as the cell letters it, or nothing for a cell without one.
    #[must_use]
    pub fn letter_caption(&self) -> String {
        self.caption
            .map(DrawingSheetTitleCaption::letter)
            .unwrap_or_default()
    }
}

/// A rule between cells: a straight segment in the block's upright frame,
/// horizontal or vertical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DrawingSheetTitleRule {
    pub from_x_um: u64,
    pub from_y_um: u64,
    pub to_x_um: u64,
    pub to_y_um: u64,
    /// Width of the line on paper.
    pub width_um: u64,
}

/// What a line of title-block lettering is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawingSheetTitleLetteringRole {
    Caption,
    Value,
}

/// One line of lettering a title block draws, placed in the block's upright
/// frame and already cut to its cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetTitleLettering {
    pub text: String,
    /// Where the line's run starts, from the block's left edge.
    pub x_um: u64,
    /// How far the run reaches.
    pub width_um: u64,
    /// Which edge of the run the line is set against.
    pub align: DrawingSheetTitleAlign,
    /// See [`DrawingSheetTitleCaption::seat_um`].
    pub seat_um: u64,
    pub lettering_um: u64,
    pub role: DrawingSheetTitleLetteringRole,
    pub emphasised: bool,
    /// Index of the cell in [`DrawingSheetTitleLayout::cells`] it is
    /// lettered in, and clipped to.
    pub cell: usize,
}

/// A title block's form: its cells and the rules between them. Distances are
/// from the top-left corner of the block, which always stands upright.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetTitleLayout {
    pub width_um: u64,
    pub height_um: u64,
    /// Where the cells start: the width a managed logo reserves at the
    /// block's left edge, or zero. The logo's own rule is the reserve's.
    pub grid_x_um: u64,
    pub cells: Vec<DrawingSheetTitleCell>,
    /// Every rule between cells, each drawn once.
    pub rules: Vec<DrawingSheetTitleRule>,
    /// Width of the block's outline on paper.
    pub outline_um: u64,
    /// Space between a cell's rules and the lettering inside them; each
    /// line's character budget already leaves it on both sides.
    pub padding_um: u64,
}

impl DrawingSheetTitleLayout {
    /// The cells of an ISO 7200 block or a flowed grid: the outline is ISO
    /// 128-20's wide line and every rule its narrow line.
    pub(super) fn from_cells(
        width_um: u64,
        height_um: u64,
        grid_x_um: u64,
        cells: Vec<DrawingSheetTitleCell>,
    ) -> Self {
        let mut layout = Self {
            width_um,
            height_um,
            grid_x_um,
            rules: Vec::new(),
            cells,
            outline_um: DRAWING_SHEET_TITLE_OUTLINE_UM,
            padding_um: DRAWING_SHEET_TITLE_CELL_PADDING_UM,
        };
        layout.rules = cell_rules(&layout, |_| DRAWING_SHEET_TITLE_RULE_UM);
        layout
    }

    /// The cells of a form whose standard rules some divisions heavier than
    /// others: `rule_um` gives the width of each stretch of a cell's edge,
    /// handed over as a rule of zero width, and stretches of one width that
    /// meet end to end are one rule.
    pub(super) fn from_weighted_cells(
        width_um: u64,
        height_um: u64,
        cells: Vec<DrawingSheetTitleCell>,
        outline_um: u64,
        padding_um: u64,
        rule_um: impl Fn(DrawingSheetTitleRule) -> u64,
    ) -> Self {
        let mut layout = Self {
            width_um,
            height_um,
            grid_x_um: 0,
            rules: Vec::new(),
            cells,
            outline_um,
            padding_um,
        };
        layout.rules = cell_rules(&layout, rule_um);
        layout
    }

    #[must_use]
    pub const fn grid_width_um(&self) -> u64 {
        self.width_um.saturating_sub(self.grid_x_um)
    }

    /// The cell that letters `field`.
    #[must_use]
    pub fn cell(&self, field: DrawingSheetTitleFieldId) -> Option<&DrawingSheetTitleCell> {
        self.cells.iter().find(|cell| cell.value(field).is_some())
    }

    /// The value line that letters `field`.
    #[must_use]
    pub fn value(&self, field: DrawingSheetTitleFieldId) -> Option<DrawingSheetTitleValue> {
        self.cells.iter().find_map(|cell| cell.value(field))
    }

    /// Every line the block letters with `fields`, the resolved values of the
    /// sheet, with the width of text other than Latin counted by
    /// [`DrawingSheetTitleCells`]; see [`Self::lettering_measured`].
    #[must_use]
    pub fn lettering(
        &self,
        fields: &[ResolvedDrawingSheetTitleField],
    ) -> Vec<DrawingSheetTitleLettering> {
        self.lettering_measured(fields, &DrawingSheetTitleCells)
    }

    /// Every line the block letters with `fields`, the resolved values of the
    /// sheet, in cell order: each cell's caption, then its values, each cut
    /// to its cell with `measure` giving the width of text other than Latin.
    /// A field with no value letters nothing, so its cell is a box to fill
    /// in. Every line of a value read right to left is set against the right
    /// of its run.
    #[must_use]
    pub fn lettering_measured(
        &self,
        fields: &[ResolvedDrawingSheetTitleField],
        measure: &dyn DrawingSheetTitleMeasure,
    ) -> Vec<DrawingSheetTitleLettering> {
        let mut lettering = Vec::new();
        for (index, cell) in self.cells.iter().enumerate() {
            if let Some(caption) = cell.caption {
                lettering.push(DrawingSheetTitleLettering {
                    text: caption.letter(),
                    x_um: caption.x_um,
                    width_um: caption.width_um,
                    align: DrawingSheetTitleAlign::Left,
                    seat_um: caption.seat_um,
                    lettering_um: caption.lettering_um,
                    role: DrawingSheetTitleLetteringRole::Caption,
                    emphasised: false,
                    cell: index,
                });
            }
            for value in &cell.values {
                let text = match value.text {
                    DrawingSheetTitleText::Fixed(text) => text,
                    DrawingSheetTitleText::Field(field) => fields
                        .iter()
                        .find(|resolved| resolved.id == field)
                        .map_or("", |resolved| resolved.value.as_str()),
                };
                if text.trim().is_empty() {
                    continue;
                }
                let align = DrawingSheetTitleAlign::of(text);
                let mut seat_um = value.seat_um;
                for line in value.letter_measured(text, measure) {
                    lettering.push(DrawingSheetTitleLettering {
                        text: line,
                        x_um: value.x_um,
                        width_um: value.width_um,
                        align,
                        seat_um,
                        lettering_um: value.lettering_um,
                        role: DrawingSheetTitleLetteringRole::Value,
                        emphasised: value.emphasised,
                        cell: index,
                    });
                    seat_um += value.line_pitch_um;
                }
            }
        }
        lettering
    }

    /// The fields among `fields` the block cuts short, measured as
    /// [`Self::lettering`] measures them; see [`Self::cut_fields_measured`].
    #[must_use]
    pub fn cut_fields(
        &self,
        fields: &[ResolvedDrawingSheetTitleField],
    ) -> Vec<DrawingSheetTitleFieldId> {
        self.cut_fields_measured(fields, &DrawingSheetTitleCells)
    }

    /// The fields among `fields`, the resolved values of the sheet, that the
    /// block cuts short, with `measure` giving the width of text other than
    /// Latin. Lettering never shrinks to fit, so each of them prints with an
    /// ellipsis, and the place that checks a sheet's title values says so
    /// beside it with [`DRAWING_SHEET_TITLE_VALUE_CUT_ADVISORY`].
    #[must_use]
    pub fn cut_fields_measured(
        &self,
        fields: &[ResolvedDrawingSheetTitleField],
        measure: &dyn DrawingSheetTitleMeasure,
    ) -> Vec<DrawingSheetTitleFieldId> {
        fields
            .iter()
            .filter(|field| {
                self.value(field.id)
                    .is_some_and(|value| value.elides_measured(&field.value, measure))
            })
            .map(|field| field.id)
            .collect()
    }
}

/// What a sheet says of a title value its block cuts short.
pub const DRAWING_SHEET_TITLE_VALUE_CUT_ADVISORY: &str =
    "Longer than its cell in this sheet's title block holds; it prints cut short.";

/// Lay out the title block `geometry` places on the sheet `format` authors,
/// or `None` when it carries none.
#[must_use]
pub fn drawing_sheet_title_layout(
    format: &SchematicSheetFormat,
    geometry: &DrawingSheetGeometry,
) -> Option<DrawingSheetTitleLayout> {
    let block = geometry.title_block?;
    let template = geometry.effective_title_block_template;
    let (width_um, height_um) = (block.width_um, block.height_um);
    match template {
        DrawingSheetTitleBlockTemplate::Standard => {
            Some(iso_7200_form_layout(format, width_um, height_um))
        }
        DrawingSheetTitleBlockTemplate::AsmeY141 => format
            .asme_title_block_form()
            .map(|form| asme_y14_1_form_layout(format, form)),
        DrawingSheetTitleBlockTemplate::OrganizationManaged => {
            flowed_title_layout(format, template, width_um, height_um)
        }
        DrawingSheetTitleBlockTemplate::None => None,
    }
}

/// Characters lettered at `lettering_um` that fit across `width_um`, and at
/// least one: lettering never shrinks to fit, so a cell too narrow for one
/// character still letters an ellipsis rather than smaller text.
const fn chars_across(width_um: u64, lettering_um: u64) -> usize {
    let chars = width_um / drawing_sheet_mono_advance_um(lettering_um);
    if chars == 0 { 1 } else { chars as usize }
}

/// The ellipsis a cut line ends in, or a cut path starts with.
const ELLIPSIS: char = '\u{2026}';

/// The run one line of lettering may fill, and how the width of what it
/// letters is taken.
struct Room<'a> {
    width_um: u64,
    lettering_um: u64,
    emphasised: bool,
    measure: &'a dyn DrawingSheetTitleMeasure,
}

impl<'a> Room<'a> {
    /// A run `width_um` wide, and never narrower than one character:
    /// lettering never shrinks to fit, so a cell too narrow for one character
    /// still letters one, or an ellipsis, rather than smaller text.
    fn new(
        width_um: u64,
        lettering_um: u64,
        emphasised: bool,
        measure: &'a dyn DrawingSheetTitleMeasure,
    ) -> Self {
        Self {
            width_um: width_um.max(drawing_sheet_mono_advance_um(lettering_um)),
            lettering_um,
            emphasised,
            measure,
        }
    }

    /// Whether `text` fits the run. Latin text and the ellipsis are counted,
    /// since IBM Plex Mono advances each of them alike; the width of anything
    /// else is the measure's.
    fn holds(&self, text: &str) -> bool {
        let width_um = if text
            .chars()
            .all(|character| character.is_ascii() || character == ELLIPSIS)
        {
            text.chars().count() as u64 * drawing_sheet_mono_advance_um(self.lettering_um)
        } else {
            self.measure
                .width_um(text, self.lettering_um, self.emphasised)
        };
        width_um <= self.width_um
    }
}

/// Where `text` may be cut: the byte offset of every boundary between two of
/// its grapheme clusters, in order.
fn cut_points(text: &str) -> Vec<usize> {
    text.grapheme_indices(true)
        .skip(1)
        .map(|(at, _)| at)
        .collect()
}

/// `text` whole when it fits `room`, and otherwise as much of its start as
/// fits with an ellipsis after it, cut between clusters. The ellipsis ends
/// the line as it is read: on the right of a paragraph read left to right,
/// and on the left of one read right to left.
fn elide_title_text(text: &str, room: &Room<'_>) -> String {
    if room.holds(text) {
        return text.to_owned();
    }
    let kept = |at: usize| format!("{}{ELLIPSIS}", &text[..at]);
    let cuts = cut_points(text);
    // A line only widens as clusters are added to it.
    let fitting = cuts.partition_point(|&at| room.holds(&kept(at)));
    fitting
        .checked_sub(1)
        .map_or_else(|| ELLIPSIS.to_string(), |last| kept(cuts[last]))
}

/// `path`, a `/`-separated cell / view path such as `lib/cell/view`, cut to
/// `room` from its start: an ellipsis and as much of its end as fits, since
/// that is where the cell and view are named. Where the first name kept
/// would be only part of one and the end still holds two whole names, the
/// cut moves to the separator after it, so `…/top/schematic` rather than
/// `…t/top/schematic`.
fn elide_title_path(path: &str, room: &Room<'_>) -> String {
    if room.holds(path) {
        return path.to_owned();
    }
    let kept = |from: usize| format!("{ELLIPSIS}{}", &path[from..]);
    let cuts = cut_points(path);
    // The shorter the end kept, the more surely it fits.
    let first = cuts.partition_point(|&from| !room.holds(&kept(from)));
    let Some(&dropped) = cuts.get(first) else {
        return ELLIPSIS.to_string();
    };
    let tail = &path[dropped..];
    let starts_whole = path[..dropped].ends_with('/');
    let tail = match tail.find('/') {
        Some(at) if !starts_whole && tail[at + 1..].contains('/') => &tail[at..],
        _ => tail,
    };
    format!("{ELLIPSIS}{tail}")
}

/// `text` broken at its spaces into at most `max_lines` lines that fit
/// `room`, and whether any of it is cut. A word wider than a line is broken
/// where the line ends, between clusters, and what the last line cannot hold
/// is cut with an ellipsis.
fn wrap_title_text(text: &str, room: &Room<'_>, max_lines: usize) -> (Vec<String>, bool) {
    let whole = room.holds(text);
    if max_lines <= 1 || whole {
        return (vec![elide_title_text(text, room)], !whole);
    }
    let mut lines = Vec::new();
    let mut rest = text.trim();
    while !rest.is_empty() {
        if lines.len() + 1 == max_lines || room.holds(rest) {
            lines.push(elide_title_text(rest, room));
            return (lines, !room.holds(rest));
        }
        // Break at the last space the line holds, or where it is full; a
        // first cluster wider than the line still takes a line of its own.
        let cuts = cut_points(rest);
        let fitting = cuts.partition_point(|&at| room.holds(&rest[..at]));
        let fits = fitting.checked_sub(1).map_or_else(
            || cuts.first().copied().unwrap_or(rest.len()),
            |last| cuts[last],
        );
        let head = &rest[..fits];
        let next = &rest[fits..];
        let (line, after) = if next.starts_with(' ') {
            (head, next)
        } else {
            head.rfind(' ')
                .map_or((head, next), |space| (&rest[..space], &rest[space..]))
        };
        lines.push(line.trim_end().to_owned());
        rest = after.trim_start();
    }
    (lines, false)
}

/// The rules of `layout`'s cells: every edge that is not the outline of the
/// block or of its field area, each at the width `rule_um` gives it, merged
/// so that edges of one width meeting end to end, or shared by two cells, are
/// one rule drawn once.
fn cell_rules(
    layout: &DrawingSheetTitleLayout,
    rule_um: impl Fn(DrawingSheetTitleRule) -> u64,
) -> Vec<DrawingSheetTitleRule> {
    let (width_um, height_um, grid_x_um) = (layout.width_um, layout.height_um, layout.grid_x_um);
    // (position across, width, start, end) for each orientation.
    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();
    let rule = |from: (u64, u64), to: (u64, u64)| DrawingSheetTitleRule {
        from_x_um: from.0,
        from_y_um: from.1,
        to_x_um: to.0,
        to_y_um: to.1,
        width_um: 0,
    };
    for cell in &layout.cells {
        let (left, top) = (cell.x_um, cell.y_um);
        let (right, bottom) = (left + cell.width_um, top + cell.height_um);
        for y in [top, bottom] {
            if y > 0 && y < height_um {
                horizontal.push((y, rule_um(rule((left, y), (right, y))), left, right));
            }
        }
        for x in [left, right] {
            if x > grid_x_um && x < width_um {
                vertical.push((x, rule_um(rule((x, top), (x, bottom))), top, bottom));
            }
        }
    }
    let mut rules = merge_runs(horizontal)
        .into_iter()
        .map(|(y, width_um, from, to)| DrawingSheetTitleRule {
            width_um,
            ..rule((from, y), (to, y))
        })
        .collect::<Vec<_>>();
    rules.extend(
        merge_runs(vertical)
            .into_iter()
            .map(|(x, width_um, from, to)| DrawingSheetTitleRule {
                width_um,
                ..rule((x, from), (x, to))
            }),
    );
    rules
}

/// Collinear segments of one width merged wherever they overlap or meet.
fn merge_runs(mut runs: Vec<(u64, u64, u64, u64)>) -> Vec<(u64, u64, u64, u64)> {
    runs.retain(|&(_, _, from, to)| to > from);
    runs.sort_unstable();
    let mut merged: Vec<(u64, u64, u64, u64)> = Vec::with_capacity(runs.len());
    for (at, width_um, from, to) in runs {
        match merged.last_mut() {
            Some(last) if last.0 == at && last.1 == width_um && from <= last.3 => {
                last.3 = last.3.max(to);
            }
            _ => merged.push((at, width_um, from, to)),
        }
    }
    merged
}
