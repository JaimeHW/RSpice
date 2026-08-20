//! Presentation primitives shared by the plan-manager shell and the child
//! dialogs promoted out of it.
//!
//! Five of the shell's inline editor modes become their own dialogs, one file
//! per lane, in parallel. Without a shared kit each lane grows its own records
//! table, its own two-column split and its own status treatment, and the
//! catalog ends up with five tables that disagree about column widths and
//! three chips that disagree about what "active" looks like.
//!
//! Nothing here reaches application state. Every primitive takes laid-out
//! values and returns what was clicked or how wide it came out, so a lane
//! cannot reach the whole application through the kit and the module's
//! whole-application mutable-access surface does not grow with it.
//!
//! The geometry is the part worth stating. A records table is given a track and
//! must fit it: the fixed columns are authored widths, one column is elastic,
//! and [`table_minimum_width`] is the narrowest track the set can be painted
//! in. The split refuses to go two-column below that width rather than handing
//! the table a track it would overflow, which is why the breakpoint is derived
//! from the table rather than authored beside it.

use egui::{Align, Align2, Color32, Layout, Rect, Sense, Stroke, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::design_system::elide_text;

/// Gap between two column tracks.
const CELL_GAP: f32 = 8.0;
/// Inset between the table's border and its first column.
const TABLE_PADDING: i8 = 6;
/// The table frame's hairline, counted on both edges.
const TABLE_BORDER: f32 = 1.0;
const HEAD_HEIGHT: f32 = 20.0;
/// The two stacked lines of an identity cell: a 12-point name over an 11-point
/// identity, laid out from the top of the cell.
const IDENTITY_LINE: f32 = 16.0;
/// Two lines: the plan's name over its stable identity.
///
/// It has to contain both galleys. Set under their combined height, the name's
/// ascender is shaved off by the cell's own clip rect — which is a defect the
/// eye reads as a font problem rather than a layout one.
///
/// This dialog may not scroll, so a row's height is spent out of a fixed budget
/// every catalog entry draws from. That also makes it the unit the surface's
/// headroom is measured in, which is why it is visible to the module.
pub(super) const ROW_HEIGHT: f32 = 2.0 * IDENTITY_LINE;

/// Inset between a split column's edge and its content, both sides.
const COLUMN_PADDING: i8 = 8;
/// Height a split column is laid out in before it sizes to its content. Large
/// enough that no arrangement reaches it, finite because an infinite `Rect`
/// propagates NaN through egui's layout arithmetic.
const UNBOUNDED_COLUMN: f32 = 10_000.0;
/// The split's internal hairline.
const DIVIDER: f32 = 1.0;
/// Inset below the columns, inside the split's own surface.
///
/// It closes the surface under its rule, and it absorbs a shortfall in the
/// enclosing dialog body: that body's scroll viewport comes out under the
/// content it was measured from, so the last line of the tallest column lands
/// just outside it and is cut mid-glyph.
///
/// The shortfall is the body's arithmetic and not this content's height, which
/// is why a constant closes it: it measured the same two points whether the
/// aside held nine rows or eleven, and the same eleven points whether its rows
/// are 22 or 24 tall — the surface shrinks, the dialog shrinks with it, and the
/// gap between what the body reports and what it paints does not move. Eleven of
/// the sixteen are the aside's closing status block, whose bottom margin the
/// body does not count.
const SPLIT_BOTTOM_INSET: f32 = 16.0;
/// The hairline around a bordered surface, counted on both edges.
///
/// An `egui::Frame`'s total margin is its inner margin plus its stroke width
/// plus its outer margin, so a bordered surface hands its content two points
/// less than it was given. Solving a column set against the width before the
/// border is how a table that fits on paper overflows by exactly two points.
const SURFACE_BORDER: f32 = 1.0;
/// `minmax(0, 1.65fr) minmax(260px, .85fr)` — the authored records/aside
/// proportion, which is roughly two to one and deliberately not equal columns.
const RECORDS_FRACTION: f32 = 1.65 / 2.5;
/// The authored `minmax(260px, …)` floor on the aside's content.
const ASIDE_MINIMUM: f32 = 260.0;

/// How one column of a records table is sized.
pub(super) enum ColumnTrack {
    /// Exactly this wide at every dialog width. Every column whose values have
    /// a known longest form — a count, a revision, a lifecycle word — is fixed,
    /// so the numeric columns do not move when the dialog resizes.
    Fixed(f32),
    /// At least this wide, and absorbs whatever the track has left over. Only
    /// the identity column is elastic: it is the one cell whose content has no
    /// bound, so it is the one that should get the slack.
    Elastic(f32),
}

pub(super) struct TableColumn {
    pub(super) heading: &'static str,
    pub(super) track: ColumnTrack,
}

/// One row's interactive state and the whole fact it announces.
///
/// The row is the interactive unit — the entire row selects, as in the authored
/// table — so the row carries one accessibility node stating every fact its
/// cells paint. Per-cell nodes would announce seven unlabelled numbers with
/// nothing tying them to the plan they belong to.
pub(super) struct TableRow {
    pub(super) selected: bool,
    pub(super) announced: String,
}

/// The narrowest outer width this column set can be painted in.
pub(super) fn table_minimum_width(columns: &[TableColumn]) -> f32 {
    let content = columns
        .iter()
        .map(|column| match column.track {
            ColumnTrack::Fixed(width) | ColumnTrack::Elastic(width) => width,
        })
        .sum::<f32>()
        + CELL_GAP * columns.len().saturating_sub(1) as f32;
    content + 2.0 * (f32::from(TABLE_PADDING) + TABLE_BORDER)
}

/// Solve the column widths for one content track.
///
/// The surplus over the authored minimum goes to the elastic columns, split
/// evenly, so the solved widths always sum with the gaps to exactly the track.
/// Below the minimum the authored widths are returned unchanged: a caller that
/// gets here has skipped the [`table_minimum_width`] check, and shrinking the
/// numeric columns to fit would trade one unreadable table for another.
fn column_widths(columns: &[TableColumn], track: f32) -> Vec<f32> {
    let gaps = CELL_GAP * columns.len().saturating_sub(1) as f32;
    let authored = columns
        .iter()
        .map(|column| match column.track {
            ColumnTrack::Fixed(width) | ColumnTrack::Elastic(width) => width,
        })
        .sum::<f32>();
    let elastic = columns
        .iter()
        .filter(|column| matches!(column.track, ColumnTrack::Elastic(_)))
        .count();
    let surplus = (track - gaps - authored).max(0.0);
    let share = if elastic == 0 {
        0.0
    } else {
        surplus / elastic as f32
    };
    columns
        .iter()
        .map(|column| match column.track {
            ColumnTrack::Fixed(width) => width,
            ColumnTrack::Elastic(width) => width + share,
        })
        .collect()
}

/// Left-to-right cell rects across one row.
fn cell_rects(row: Rect, widths: &[f32]) -> Vec<Rect> {
    let mut x = row.left();
    widths
        .iter()
        .map(|width| {
            let rect = Rect::from_min_size(pos2(x, row.top()), vec2(*width, row.height()));
            x += width + CELL_GAP;
            rect
        })
        .collect()
}

/// Whether a table's rows are a control or a statement.
///
/// Both tables paint the same columns, and only the caller knows whether a
/// click means anything. A read-only table that borrowed the selectable one's
/// treatment painted a hover fill under the pointer and published every row as
/// a `SelectableLabel`, so the surface offered an affordance that did nothing
/// and a screen reader offered a selection that could not be made. That is a
/// promise the surface cannot keep, and it is invisible to a fit gate: the
/// geometry is identical either way.
enum RowSense {
    /// Rows are the control. The pointer gets a hover fill, the row announces
    /// itself as selectable and reports its own state, and a click is returned.
    Selects,
    /// Rows are a statement. No hover fill, no click, and the row announces as
    /// a label — which still carries the whole fact, because a cell is elided
    /// to its column and the values alone name no row.
    ReadOnly,
}

/// Paint a fixed-width records table whose rows select, and report which row
/// was clicked.
///
/// `cell` is called once per cell with a `Ui` clipped to that cell's track, so
/// a cell painter cannot widen the table by painting a long value — it is
/// elided instead, and the row's node still announces the whole fact. The table
/// therefore occupies exactly the track it was given, which is what keeps the
/// enclosing dialog from growing a horizontal scrollbar.
pub(super) fn records_table(
    ui: &mut Ui,
    id_salt: &str,
    columns: &[TableColumn],
    rows: &[TableRow],
    cell: impl FnMut(&mut Ui, usize, usize),
) -> Option<usize> {
    table(ui, id_salt, columns, rows, RowSense::Selects, cell)
}

/// Paint the same table for a surface that only states what it holds.
///
/// There is no return value to discard: a read-only table reports no click, so
/// a caller cannot come to depend on one and the next reader does not have to
/// work out why the result was being thrown away.
pub(super) fn read_only_records_table(
    ui: &mut Ui,
    id_salt: &str,
    columns: &[TableColumn],
    rows: &[TableRow],
    cell: impl FnMut(&mut Ui, usize, usize),
) {
    table(ui, id_salt, columns, rows, RowSense::ReadOnly, cell);
}

fn table(
    ui: &mut Ui,
    id_salt: &str,
    columns: &[TableColumn],
    rows: &[TableRow],
    sense: RowSense,
    mut cell: impl FnMut(&mut Ui, usize, usize),
) -> Option<usize> {
    let selects = matches!(sense, RowSense::Selects);
    let t = Tokens::get(ui.ctx());
    let mut clicked = None;
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(TABLE_BORDER, t.color.border))
        .inner_margin(egui::Margin::same(TABLE_PADDING))
        .show(ui, |ui| {
            let track = ui.available_width();
            let widths = column_widths(columns, track);
            ui.spacing_mut().item_spacing.y = 0.0;

            let (head, _) = ui.allocate_exact_size(vec2(track, HEAD_HEIGHT), Sense::hover());
            let heading_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
            for (rect, column) in cell_rects(head, &widths).into_iter().zip(columns) {
                let text = elide_text(ui, column.heading, &heading_font, rect.width());
                ui.painter().text(
                    rect.left_center(),
                    Align2::LEFT_CENTER,
                    text,
                    heading_font.clone(),
                    t.color.text_dim,
                );
            }
            ui.painter().hline(
                head.x_range(),
                head.bottom(),
                Stroke::new(1.0, t.color.border),
            );

            for (index, row) in rows.iter().enumerate() {
                let (rect, response) = ui.allocate_exact_size(
                    vec2(track, ROW_HEIGHT),
                    if selects {
                        Sense::click()
                    } else {
                        Sense::hover()
                    },
                );
                if row.selected {
                    ui.painter().rect_filled(rect, 0.0, t.color.bg_active);
                    ui.painter().vline(
                        rect.left() + 1.0,
                        rect.y_range(),
                        Stroke::new(2.0, t.color.accent),
                    );
                } else if selects && response.hovered() {
                    ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                }
                response.widget_info(|| {
                    if selects {
                        egui::WidgetInfo::selected(
                            egui::WidgetType::SelectableLabel,
                            ui.is_enabled(),
                            row.selected,
                            &row.announced,
                        )
                    } else {
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Label,
                            ui.is_enabled(),
                            &row.announced,
                        )
                    }
                });
                for (column, cell_rect) in cell_rects(rect, &widths).into_iter().enumerate() {
                    let mut child = ui.new_child(
                        egui::UiBuilder::new()
                            .id_salt((id_salt, index, column))
                            .max_rect(cell_rect)
                            .layout(Layout::left_to_right(Align::Center)),
                    );
                    child.set_clip_rect(cell_rect.intersect(ui.clip_rect()));
                    cell(&mut child, index, column);
                }
                if selects && response.clicked() {
                    clicked = Some(index);
                }
            }
        });
    clicked
}

/// The plan's name over its stable identity, both elided to the cell.
pub(super) fn cell_identity(ui: &mut Ui, name: &str, identity: &str) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.max_rect();
    let name_font = theme::sans(tokens::FS_1, FontWeight::SemiBold);
    let identity_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let name_text = elide_text(ui, name, &name_font, rect.width());
    let identity_text = elide_text(ui, identity, &identity_font, rect.width());
    // Anchored to the top of the cell, one line each. Centring the pair on the
    // cell's midline put the name's box above the cell and the identity's below
    // it, so both were shaved by the cell's clip rect.
    ui.painter().text(
        pos2(rect.left(), rect.top()),
        Align2::LEFT_TOP,
        name_text,
        name_font,
        t.color.text,
    );
    ui.painter().text(
        pos2(rect.left(), rect.top() + IDENTITY_LINE),
        Align2::LEFT_TOP,
        identity_text,
        identity_font,
        t.color.text_faint,
    );
}

/// One exact value, mono, elided to the cell.
pub(super) fn cell_value(ui: &mut Ui, value: &str, tone: Color32) {
    let rect = ui.max_rect();
    let font = theme::mono(tokens::FS_1, FontWeight::Regular);
    let text = elide_text(ui, value, &font, rect.width());
    ui.painter()
        .text(rect.left_center(), Align2::LEFT_CENTER, text, font, tone);
}

/// A sentence in a table cell, elided to it.
///
/// [`cell_value`] is the sibling and is mono, which is right for an identity, a
/// count or a forecast — values read down a column and compared digit by digit.
/// A cell holding a clause is read as words, and mono spends a third more width
/// per character to do it: the widest cell of the create route's outcome table
/// is 230 points in mono and 179 in sans, and the difference is whether that
/// table fits its dialog at all.
pub(super) fn cell_prose(ui: &mut Ui, text: &str, tone: Color32) {
    let rect = ui.max_rect();
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let shown = elide_text(ui, text, &font, rect.width());
    ui.painter()
        .text(rect.left_center(), Align2::LEFT_CENTER, shown, font, tone);
}

/// The width one column must have to hold both its heading and its value whole.
///
/// A table whose columns are measured from what they actually carry cannot elide
/// a cell, which is the contract the create route is held to: it asserts that
/// nothing it paints ends in an ellipsis. Authored widths cannot make that
/// promise — the values are built from the reader's own choices, so their length
/// is not known where a constant would be written.
pub(super) fn prose_column_width(ui: &Ui, heading: &str, value: &str) -> f32 {
    let heading_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let value_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let width = |text: &str, font: egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font, Color32::WHITE)
            .size()
            .x
    };
    width(heading, heading_font)
        .max(width(value, value_font))
        .ceil()
}

/// The three lifecycle states a plan can be in, which are the only three
/// `active` and `archived` can express between them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LifecycleTone {
    /// The single editable plan. One per catalog, by construction.
    Active,
    /// Retained and not archived — a plan the reader can switch to.
    Available,
    /// Recoverably retired.
    Archived,
}

impl LifecycleTone {
    fn color(self, t: &Tokens) -> Color32 {
        match self {
            Self::Active => t.color.accent,
            Self::Available => t.color.ok,
            Self::Archived => t.color.text_faint,
        }
    }
}

/// A lifecycle word in its state's tone.
///
/// Flat toned mono text, not a boxed pill: the authored `row-status` treatment
/// is exactly this, and a bordered chip repeated down every row of a records
/// table is the decorative-card density this product bar rules out. The tone
/// carries the state; the box would carry nothing.
pub(super) fn lifecycle_chip(ui: &mut Ui, label: &str, tone: LifecycleTone) {
    let t = Tokens::get(ui.ctx());
    let rect = ui.max_rect();
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let text = elide_text(ui, label, &font, rect.width());
    ui.painter().text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        text,
        font,
        tone.color(&t),
    );
}

/// A lifecycle word to sit at the trailing edge of a section head.
pub(super) struct HeadStatus<'label> {
    pub(super) label: &'label str,
    pub(super) tone: LifecycleTone,
}

/// A section heading, optionally closed by a status word at its trailing edge.
///
/// `workflows::workflow_section_heading` is the plain sibling and paints the
/// same rule, font and height, but it takes the full available width and
/// returns nothing, so there is no rect left to place a trailing status in.
/// This is the module's one section-head painter for that reason rather than a
/// second treatment standing beside it.
pub(super) fn section_head(ui: &mut Ui, label: &str, status: Option<HeadStatus<'_>>) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), HEAD_HEIGHT), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    // The label is painted before the status so that paint order is reading
    // order: a reader working through the surface left to right meets the
    // heading and then its state, and so does anything reading the paint back.
    let status_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let status_width = status.as_ref().map_or(0.0, |status| {
        ui.painter()
            .layout_no_wrap(
                status.label.to_owned(),
                status_font.clone(),
                t.color.text_dim,
            )
            .size()
            .x
            + CELL_GAP
    });
    let font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let text = elide_text(ui, label, &font, (rect.width() - status_width).max(0.0));
    ui.painter().text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        text,
        font,
        t.color.text,
    );
    if let Some(status) = status {
        ui.painter().text(
            rect.right_center(),
            Align2::RIGHT_CENTER,
            status.label,
            status_font,
            status.tone.color(&t),
        );
    }
}

/// Height of one detail row.
///
/// `design_system::property_row` is the same row and the same columns, and it
/// takes the design system's control height — which a dialog raises to the
/// 44-point touch target on every viewport 820 points wide or narrower. That is
/// the right rule for a control and the wrong one for a statement: these rows
/// are read, never operated, and at the touch height eight of them cost 352
/// points of an aside that has 418 to hold its whole contents. So the detail
/// list keeps the compact row at every width, and the actions below it — which
/// *are* controls — keep the touch height they need.
const DETAIL_ROW_HEIGHT: f32 = 22.0;
/// Horizontal inset inside a detail row, matching `property_row`'s.
const DETAIL_ROW_PAD: f32 = 10.0;
/// Gap between a detail row's label and value columns, matching
/// `property_row`'s.
const DETAIL_ROW_GAP: f32 = 8.0;
/// Share of a detail row the label takes, matching `property_row`'s.
const DETAIL_LABEL_FRACTION: f32 = 0.4;
/// Width the status mark and its gap take before the value.
const DETAIL_MARK_WIDTH: f32 = 17.0;

/// A label beside its value, on one line, at a height a touch viewport does not
/// change.
///
/// Both halves are elided to their column and the whole pair is published to the
/// accessibility tree, so a shortened value is still reachable in full — the
/// same contract `property_row` keeps, and what the module's elision gate
/// checks.
pub(super) fn detail_row(
    ui: &mut Ui,
    label: &str,
    value: &str,
    tone: Color32,
    mark: Option<crate::workbench::design_system::StatusMark>,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let inner = (width - 2.0 * DETAIL_ROW_PAD).max(1.0);
    let columns = (inner - DETAIL_ROW_GAP.min(inner)).max(1.0);
    let label_column = columns * DETAIL_LABEL_FRACTION;
    let value_column = (columns - label_column).max(1.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, DETAIL_ROW_HEIGHT), Sense::hover());

    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let value_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mark_width = if mark.is_some() {
        DETAIL_MARK_WIDTH
    } else {
        0.0
    };
    let label_text = elide_text(ui, label, &label_font, label_column);
    let value_text = elide_text(ui, value, &value_font, (value_column - mark_width).max(1.0));
    let value_left = rect.left() + DETAIL_ROW_PAD + label_column + DETAIL_ROW_GAP;
    ui.painter().text(
        pos2(rect.left() + DETAIL_ROW_PAD, rect.center().y),
        Align2::LEFT_CENTER,
        label_text,
        label_font,
        t.color.text_dim,
    );
    if let Some(mark) = mark {
        crate::workbench::design_system::paint_status_mark(
            ui.painter(),
            Rect::from_center_size(pos2(value_left + 5.5, rect.center().y), vec2(11.0, 11.0)),
            mark,
            tone,
        );
    }
    ui.painter().text(
        pos2(value_left + mark_width, rect.center().y),
        Align2::LEFT_CENTER,
        value_text,
        value_font,
        tone,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        node.set_value(value);
    });
    response.on_hover_text(format!("{label}: {value}"));
}

/// Equal-width notes side by side inside one bordered surface, each a caption
/// over a wrapped body.
///
/// These state a boundary of the transaction the surface performs, so they are
/// two answers to one question and belong beside each other rather than
/// stacked in the reading flow of the records they qualify.
pub(super) fn note_grid(ui: &mut Ui, notes: &[(&str, &str)]) {
    if notes.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let stroke = Stroke::new(1.0, t.color.border_strong);
    let surface = ui.available_width() - 2.0 * SURFACE_BORDER;
    let column =
        ((surface - DIVIDER * notes.len().saturating_sub(1) as f32) / notes.len() as f32).floor();
    let mut seams = Vec::new();
    let frame = egui::Frame::new().stroke(stroke).show(ui, |ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        ui.horizontal_top(|ui| {
            for (index, (caption, body)) in notes.iter().enumerate() {
                if index > 0 {
                    seams.push(ui.cursor().left() + DIVIDER / 2.0);
                    ui.add_space(DIVIDER);
                }
                ui.allocate_ui_with_layout(vec2(column, 0.0), Layout::top_down(Align::Min), |ui| {
                    ui.set_width(column);
                    egui::Frame::new()
                        .inner_margin(egui::Margin::same(COLUMN_PADDING))
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 4.0;
                            ui.label(
                                egui::RichText::new(*caption)
                                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                    .color(t.color.text),
                            );
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(*body)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_dim),
                                )
                                .wrap(),
                            );
                        });
                });
            }
        });
    });
    for x in seams {
        ui.painter().vline(x, frame.response.rect.y_range(), stroke);
    }
}

/// Which of the split's two columns is being painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SplitColumn {
    Records,
    Aside,
}

/// The tracks the split resolved for one available width.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SplitGeometry {
    /// Whether the columns are stacked rather than side by side.
    pub(super) stacked: bool,
    /// Content width handed to the records column, inside its padding.
    pub(super) records: f32,
    /// Content width handed to the aside, inside its padding.
    pub(super) aside: f32,
}

/// Resolve the split for one available width.
///
/// Two columns only when both sides clear their minimum content width;
/// otherwise the columns stack and each gets the whole width. The records
/// minimum is the caller's, which is how the breakpoint stays a property of
/// what the records column actually contains instead of a number authored
/// beside it and left to drift.
pub(super) fn split_tracks(available: f32, records_minimum: f32) -> SplitGeometry {
    let inset = 2.0 * f32::from(COLUMN_PADDING);
    // The split runs edge to edge inside a flush dialog body and carries only a
    // closing rule, so unlike a boxed surface it loses nothing to side borders.
    let surface = available;
    let usable = surface - DIVIDER;
    let records_floor = records_minimum + inset;
    let aside_floor = ASIDE_MINIMUM + inset;
    if usable >= records_floor + aside_floor {
        // Keep the authored proportion when it clears both floors. At the
        // exact breakpoint it can miss one floor by a point even though the
        // two minimum tracks fit; clamp that point into the narrower side
        // instead of needlessly stacking the surface.
        let records = (usable * RECORDS_FRACTION)
            .floor()
            .clamp(records_floor, usable - aside_floor);
        let aside = usable - records;
        SplitGeometry {
            stacked: false,
            records: records - inset,
            aside: aside - inset,
        }
    } else {
        SplitGeometry {
            stacked: true,
            records: surface - inset,
            aside: surface - inset,
        }
    }
}

/// The asymmetric two-column dialog surface: records left, aside right.
///
/// `hardcopy::render::dialog_split` is the equal-column sibling. It is a real
/// two-column-with-breakpoint primitive and draws the same divider, but it
/// composes with `Ui::columns`, which can only halve the width — and the
/// authored proportion here is roughly two to one, because a seven-column
/// records table and an eleven-row property aside do not want the same track.
/// It also lives in an allowlisted file belonging to another program, so this
/// is a second implementation on purpose rather than by omission.
pub(super) fn manager_split(
    ui: &mut Ui,
    records_minimum: f32,
    mut column: impl FnMut(&mut Ui, SplitColumn),
) {
    let t = Tokens::get(ui.ctx());
    let stroke = Stroke::new(1.0, t.color.border_strong);
    let geometry = split_tracks(ui.available_width(), records_minimum);
    let inset = 2.0 * f32::from(COLUMN_PADDING);
    let mut seam = None;
    // No surrounding box. Inside a flush dialog body the authored split runs
    // edge to edge and carries a closing rule instead of a border, so a boxed
    // surface here would draw two hairlines down the dialog's own edges.
    let frame = egui::Frame::NONE.show(ui, |ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        if geometry.stacked {
            let first = padded_column(ui, geometry.records + inset, |ui| {
                column(ui, SplitColumn::Records);
            });
            seam = Some((false, first.bottom()));
            padded_column(ui, geometry.aside + inset, |ui| {
                column(ui, SplitColumn::Aside);
            });
        } else {
            ui.horizontal_top(|ui| {
                let first = padded_column(ui, geometry.records + inset, |ui| {
                    column(ui, SplitColumn::Records);
                });
                seam = Some((true, first.right() + DIVIDER / 2.0));
                ui.add_space(DIVIDER);
                padded_column(ui, geometry.aside + inset, |ui| {
                    column(ui, SplitColumn::Aside);
                });
            });
        }
        ui.add_space(SPLIT_BOTTOM_INSET);
    });
    // The internal seam, then the rule that closes the surface.
    match seam {
        Some((true, x)) => {
            ui.painter().vline(x, frame.response.rect.y_range(), stroke);
        }
        Some((false, y)) => {
            ui.painter().hline(frame.response.rect.x_range(), y, stroke);
        }
        None => {}
    }
    ui.painter().hline(
        frame.response.rect.x_range(),
        frame.response.rect.bottom(),
        stroke,
    );
}

/// One padded column of the split, sized to `width`. Returns its outer rect.
///
/// Vertical item spacing is zero, and deliberately. A property row already
/// carries its own row height, so an inter-item gap here double-spaces it: the
/// eleven-row aside was paying an extra eight points fifteen times over, which
/// is 120 points of a 511-point body budget spent on nothing. Callers add space
/// where a gap is meant.
fn padded_column(ui: &mut Ui, width: f32, content: impl FnOnce(&mut Ui)) -> Rect {
    // The column is given its own tall `max_rect` rather than the height that
    // happens to be left in the enclosing body.
    //
    // `allocate_ui_with_layout` bounds a child to the available height and
    // clips it there. Inside a dialog body that reserves a viewport, that made
    // the taller of the two columns unable to exceed it: the aside's last
    // property row was cut off mid-glyph while the dialog then sized itself to
    // the cut-down content and left empty screen below. Sizing the column to
    // its content instead lets the surface state its real height, which is what
    // the fit gate is then entitled to check against the viewport.
    let top_left = ui.cursor().min;
    let mut column = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(Rect::from_min_size(top_left, vec2(width, UNBOUNDED_COLUMN)))
            .layout(Layout::top_down(Align::Min)),
    );
    column.set_width(width);
    let rect = egui::Frame::new()
        .inner_margin(egui::Margin::same(COLUMN_PADDING))
        .show(&mut column, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            content(ui);
        })
        .response
        .rect;
    ui.advance_cursor_after_rect(rect);
    rect
}

/// Lay `count` equal tracks side by side, and call `column` for each.
///
/// This is how a property list trades width for height. The same eleven rows are
/// 308 points tall in one track and a third of that in three, so a detail panel
/// that has been given the full dialog width should spend it rather than run off
/// the bottom of a surface that is not allowed to scroll.
pub(super) fn equal_columns(ui: &mut Ui, count: usize, mut column: impl FnMut(&mut Ui, usize)) {
    if count == 0 {
        return;
    }
    let track =
        ((ui.available_width() - CELL_GAP * count.saturating_sub(1) as f32) / count as f32).floor();
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        for index in 0..count {
            if index > 0 {
                ui.add_space(CELL_GAP);
            }
            ui.allocate_ui_with_layout(vec2(track, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.set_width(track);
                column(ui, index);
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The manager's real column set, not a copy of it. A fixture that only
    /// resembled production would let the two drift, and the number these tests
    /// are about — the width below which the split can no longer stay
    /// two-column — is a property of the shipped widths or of nothing.
    fn seven_columns() -> &'static [TableColumn] {
        &super::super::PLAN_COLUMNS
    }

    /// The solved widths fill the track exactly. A sum under the track leaves a
    /// ragged right edge; a sum over it is the horizontal overflow the authored
    /// reference has and this table must not.
    #[test]
    fn solved_column_widths_fill_the_track_exactly_and_never_exceed_it() {
        let columns = seven_columns();
        let content_minimum =
            table_minimum_width(columns) - 2.0 * (f32::from(TABLE_PADDING) + TABLE_BORDER);
        for track in [
            content_minimum,
            content_minimum + 1.0,
            610.0,
            778.0,
            1_400.0,
        ] {
            let widths = column_widths(columns, track);
            let used = widths.iter().sum::<f32>() + CELL_GAP * (widths.len() - 1) as f32;
            assert!(
                (used - track).abs() < 0.001,
                "columns solved to {used} in a {track} track"
            );
        }

        // Below the minimum the authored widths stand rather than shrinking to
        // fit. A caller that gets here has skipped the minimum check, and a
        // silently squeezed numeric column would hide that rather than show it.
        let squeezed = column_widths(columns, content_minimum - 40.0);
        let used = squeezed.iter().sum::<f32>() + CELL_GAP * (squeezed.len() - 1) as f32;
        assert!(
            (used - content_minimum).abs() < 0.001,
            "a track under the minimum resolved to {used} instead of holding \
             the authored {content_minimum}"
        );
    }

    /// Only the elastic column moves. The numeric columns are read by position
    /// across rows, so they must not shift when the dialog is resized.
    #[test]
    fn only_the_elastic_column_absorbs_the_surplus() {
        let columns = seven_columns();
        let narrow = column_widths(columns, 560.0);
        let wide = column_widths(columns, 900.0);
        assert!(wide[0] > narrow[0], "the identity column takes the slack");
        assert_eq!(
            &narrow[1..],
            &wide[1..],
            "a fixed column changed width with the dialog"
        );
    }

    /// The breakpoint is the table's, not a number authored beside it: the
    /// split goes two-column exactly when the records track clears the table's
    /// own minimum and the aside clears the authored floor.
    #[test]
    fn the_split_never_hands_the_records_column_less_than_the_table_needs() {
        let minimum = table_minimum_width(seven_columns());
        let mut transition = None;
        for step in 0..=900 {
            let available = 400.0 + step as f32;
            let geometry = split_tracks(available, minimum);
            if geometry.stacked {
                assert_eq!(
                    geometry.records, geometry.aside,
                    "a stacked column gets the whole width"
                );
                assert!(
                    transition.is_none(),
                    "the split went back to stacked at {available} after \
                     resolving two columns"
                );
            } else {
                transition = transition.or(Some(available));
                assert!(
                    geometry.records >= minimum,
                    "two columns at {available} left the table {} short",
                    minimum - geometry.records
                );
                assert!(geometry.aside >= ASIDE_MINIMUM);
                let inset = 2.0 * f32::from(COLUMN_PADDING);
                let laid_out = geometry.records + geometry.aside + 2.0 * inset + DIVIDER;
                assert!(
                    laid_out <= available + 0.001,
                    "the two tracks and the divider need {laid_out} of {available}"
                );
            }
        }
        let transition = transition.expect("a wide enough dialog resolves two columns");
        // 820 is `WideWorkflow`'s `narrow_max_width`: at or below it the dialog
        // stops being a fixed-width panel and becomes the whole viewport. It is
        // not a floor on the dialog's width — a narrower window makes a narrower
        // dialog — so this asserts the column set stays two-column across every
        // landscape width from 820 up. Portrait widths below it stack, which is
        // correct there and is gated separately against a portrait viewport.
        assert!(
            transition <= 820.0,
            "the split cannot stay two-column until {transition}, so a \
             full-viewport landscape window at 820 would stack — and stacking \
             costs more height than a 640-point viewport has. Trim the column \
             set rather than let the surface fall back to it."
        );
    }

    /// The records column is the wider one. Equal columns were the reason the
    /// existing `dialog_split` could not serve this surface.
    #[test]
    fn the_records_column_is_wider_than_the_aside() {
        let geometry = split_tracks(978.0, table_minimum_width(seven_columns()));
        assert!(!geometry.stacked);
        assert!(
            geometry.records > geometry.aside * 1.5,
            "records {} against aside {}: the authored proportion is roughly \
             two to one, not equal columns",
            geometry.records,
            geometry.aside
        );
    }

    /// Three states, three tones, no two the same — a lifecycle word whose tone
    /// repeated another state's would state less than the word alone.
    #[test]
    fn every_lifecycle_state_has_its_own_tone() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let t = Tokens::get(&ctx);
        let tones = [
            LifecycleTone::Active.color(&t),
            LifecycleTone::Available.color(&t),
            LifecycleTone::Archived.color(&t),
        ];
        for (index, tone) in tones.iter().enumerate() {
            for other in &tones[index + 1..] {
                assert_ne!(tone, other, "two lifecycle states share one tone");
            }
        }
        assert_eq!(
            LifecycleTone::Active.color(&t),
            t.color.accent,
            "the active plan is the accent state"
        );
    }
}
