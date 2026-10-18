//! The table itself: its column measurements, its header, and its rows.
//!
//! The table runs from one side of the dock to the other. Its header band and
//! every row's fills meet both edges, as a list does in every tool this page
//! is measured against; only the text in them is inset. The first column's
//! text stands where the toolbar's first control does, and every cell's text
//! stands one cell inset inside its cell, so the gutter between one column's
//! widest text and the next column's text is the same everywhere: two insets.
//!
//! Every column but the message is as wide as the widest cell the listed rows
//! give it, in the face the cells are printed in, with its heading as a floor
//! and a share of the room as a ceiling. The message takes what is left.
//! Nothing is a pixel count somebody typed: the first version of this page
//! fixed two columns at 58 px and 62 px — eight characters — and could not
//! print a location at all, and the next gave the subject and the location
//! fixed shares whatever they held, so `Net 'test'` sat in 390 px of air.
//!
//! A column's right edge in the header is a control: dragged, it sets the
//! column's width; double-clicked, or Home pressed on it, it fits the column
//! to what it lists again; the arrows move it a step at a time. A width set
//! there holds for the session.

use egui::{Sense, Stroke, Ui, Vec2};

use super::super::{
    CONSOLE_BODY_PADDING_X, CONSOLE_CELL_INSET, CONSOLE_FONT_SIZE, CONSOLE_MARK_GUTTER,
    CONSOLE_MARK_SIDE, CONSOLE_ROW_MIN_HEIGHT, CONSOLE_TABLE_HEADER_HEIGHT, announced_row,
    paint_header_band, paint_row_fill, paint_row_mark,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Problem, StatusMark, paint_status_mark};
use crate::workbench::app_state::problems_view::{ProblemColumn, ProblemsView};

use super::arrange::Group;

/// The header row's own height.
pub(super) const HEADER_HEIGHT: f32 = CONSOLE_TABLE_HEADER_HEIGHT;

/// The columns fitted to their content: every one but the message, in the
/// order the grid prints them.
pub(super) const FITTED: [ProblemColumn; 5] = [
    ProblemColumn::Severity,
    ProblemColumn::Code,
    ProblemColumn::Subject,
    ProblemColumn::Producer,
    ProblemColumn::Location,
];

/// The order mark's distance from its heading, and its width. Every heading
/// keeps room for it, so ordering by a column never moves the columns.
const SORT_MARK_GAP: f32 = 4.0;
const SORT_MARK_WIDTH: f32 = 7.0;

/// The most of the table's room one column fitted to its content may take.
/// Past it the column cuts its cells, so one long path cannot push the
/// message off the page.
const FITTED_MAX_SHARE: f32 = 0.24;

/// The least the message keeps, in characters. Fitted columns that would
/// leave it less give way, the widest first.
const MESSAGE_MIN_CHARACTERS: f32 = 16.0;

/// The least a column can be dragged to, in characters of its face.
const RESIZED_MIN_CHARACTERS: f32 = 3.0;

/// How far one arrow key moves a column's edge.
const RESIZE_STEP: f32 = 12.0;

/// The widths a reader set by dragging a column's edge, in [`FITTED`] order.
/// `None` is a column fitted to its content. Kept for the session, beside
/// the page's other between-frame state.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Resized([Option<f32>; FITTED.len()]);

impl Resized {
    fn id() -> egui::Id {
        egui::Id::new("workbench.problems.columns")
    }

    fn load(ctx: &egui::Context) -> Self {
        ctx.data(|data| data.get_temp(Self::id()))
            .unwrap_or_default()
    }

    fn store(self, ctx: &egui::Context) {
        ctx.data_mut(|data| data.insert_temp(Self::id(), self));
    }
}

/// Where each column is and how wide it is, for one pass of the page.
pub(super) struct Columns {
    /// The table's left and right edges: the dock's own.
    left: f32,
    right: f32,
    /// Where each column starts, measured from `left`, in
    /// [`ProblemColumn::ALL`] order.
    starts: [f32; 6],
    /// Each column's width, its insets included.
    widths: [f32; 6],
    /// The room the columns share: the table less the lead before the first
    /// column and the scrollbar's allowance after the last.
    room: f32,
    /// Width of one monospace character, which is what makes eliding a cell
    /// exact instead of iterative: in a monospace face the number of
    /// characters that fit is a division.
    advance: f32,
    ellipsis: f32,
}

impl Columns {
    /// Measure the columns against the font, the table's edges, and the widest
    /// cell each fitted column lists, in characters in [`FITTED`] order — the
    /// face is monospace, so a count of characters is a width. The severity
    /// column's widest cell is its word; the mark before it is added here.
    pub(super) fn measure(ui: &Ui, widest: &[usize; FITTED.len()], band: egui::Rangef) -> Self {
        let advance = text_width(ui, "M", FontWeight::Regular).max(1.0);
        let ellipsis = text_width(ui, "…", FontWeight::Regular);
        // The body's scrollbar is allowed for whether or not it is showing.
        // A scroll area's own width includes its bar and changes the frame it
        // appears, so columns measured against it would shift under the reader
        // as the list grew past the viewport — and the header, which is drawn
        // outside the area, would stop lining up with the rows inside it.
        let scrollbar = ui.spacing().scroll.bar_width + ui.spacing().scroll.bar_inner_margin;
        let lead = CONSOLE_BODY_PADDING_X - CONSOLE_CELL_INSET;
        let room = (band.span() - lead - scrollbar).max(advance);
        let resized = Resized::load(ui.ctx());

        let mut floors = [0.0; FITTED.len()];
        let mut widths = [0.0; 6];
        for (index, column) in FITTED.into_iter().enumerate() {
            // A column that cannot print its own heading says nothing, so
            // each asks for at least its heading and the order mark's room.
            let heading = text_width(ui, column.label(), FontWeight::SemiBold)
                + SORT_MARK_GAP
                + SORT_MARK_WIDTH;
            let mark = if column == ProblemColumn::Severity {
                CONSOLE_MARK_GUTTER
            } else {
                0.0
            };
            let content = mark + widest[index] as f32 * advance;
            floors[index] = heading + 2.0 * CONSOLE_CELL_INSET;
            widths[index] = match resized.0[index] {
                Some(width) => width.max(resized_least(advance)),
                None => (content.max(heading) + 2.0 * CONSOLE_CELL_INSET)
                    .min(room * FITTED_MAX_SHARE)
                    .max(floors[index]),
            };
        }
        // Columns that would leave the message less than its least give way,
        // the widest first: one ceiling is lowered over them until the
        // excess is taken, none below its heading. A short column — the
        // severity word, a net's name — keeps its cells whole while a long
        // rule or path is cut; taking from every column in proportion cut
        // "Critical" to "Critic…" to spare a forty-character rule two
        // characters. A width a reader set is theirs, and stays.
        let message_least = MESSAGE_MIN_CHARACTERS * advance + 2.0 * CONSOLE_CELL_INSET;
        let fitted: f32 = widths[..FITTED.len()].iter().sum();
        let excess = fitted + message_least - room;
        let yielding = |index: &usize| resized.0[*index].is_none();
        if excess > 0.0 {
            let under = |ceiling: f32, index: usize| widths[index].min(ceiling).max(floors[index]);
            let taken = |ceiling: f32| -> f32 {
                (0..FITTED.len())
                    .filter(yielding)
                    .map(|index| widths[index] - under(ceiling, index))
                    .sum()
            };
            let (mut low, mut high) = (
                0.0_f32,
                widths[..FITTED.len()]
                    .iter()
                    .copied()
                    .fold(0.0_f32, f32::max),
            );
            for _ in 0..32 {
                let ceiling = (low + high) * 0.5;
                if taken(ceiling) > excess {
                    low = ceiling;
                } else {
                    high = ceiling;
                }
            }
            let capped: [f32; FITTED.len()] = std::array::from_fn(|index| {
                if yielding(&index) {
                    under(high, index)
                } else {
                    widths[index]
                }
            });
            widths[..FITTED.len()].copy_from_slice(&capped);
        }
        let fitted: f32 = widths[..FITTED.len()].iter().sum();
        widths[5] = (room - fitted).max(message_least);

        let mut starts = [0.0; 6];
        let mut x = lead;
        for (start, width) in starts.iter_mut().zip(widths) {
            *start = x;
            x += width;
        }
        Self {
            left: band.min,
            right: band.max,
            starts,
            widths,
            room,
            advance,
            ellipsis,
        }
    }

    const fn index(column: ProblemColumn) -> usize {
        match column {
            ProblemColumn::Severity => 0,
            ProblemColumn::Code => 1,
            ProblemColumn::Subject => 2,
            ProblemColumn::Producer => 3,
            ProblemColumn::Location => 4,
            ProblemColumn::Message => 5,
        }
    }

    /// A column's width, its insets included.
    pub(super) const fn width(&self, column: ProblemColumn) -> f32 {
        self.widths[Self::index(column)]
    }

    /// How far into the table a column's cell starts, from its left edge.
    pub(super) const fn offset(&self, column: ProblemColumn) -> f32 {
        self.starts[Self::index(column)]
    }

    /// How far into the table a column's text starts. The severity word
    /// stands after its mark; every other cell's text at its cell's inset.
    pub(super) fn text_offset(&self, column: ProblemColumn) -> f32 {
        let mark = if column == ProblemColumn::Severity {
            CONSOLE_MARK_GUTTER
        } else {
            0.0
        };
        self.offset(column) + CONSOLE_CELL_INSET + mark
    }

    /// How much text one of a column's cells prints before it is cut.
    pub(super) fn text_room(&self, column: ProblemColumn) -> f32 {
        (self.offset(column) + self.width(column) - CONSOLE_CELL_INSET - self.text_offset(column))
            .max(0.0)
    }

    /// The table's right edge: the dock's.
    pub(super) const fn right(&self) -> f32 {
        self.right
    }
}

/// The least a reader can drag a column to.
fn resized_least(advance: f32) -> f32 {
    RESIZED_MIN_CHARACTERS * advance + 2.0 * CONSOLE_CELL_INSET
}

fn text_width(ui: &Ui, text: &str, weight: FontWeight) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                text.to_owned(),
                theme::mono(CONSOLE_FONT_SIZE, weight),
                egui::Color32::WHITE,
            )
            .size()
            .x
    })
}

/// The column header row. Each cell orders the table by its own column, and
/// each fitted column's right edge resizes it.
///
/// Returns the column a reader activated this frame. Every cell is a real
/// control: it announces as a button with the order it would apply said in
/// words, takes keyboard focus, and activates on Enter and Space as well as
/// on a click — a direction glyph alone cannot be heard.
pub(super) fn header(ui: &mut Ui, columns: &Columns, view: &ProblemsView) -> Option<ProblemColumn> {
    let tokens = Tokens::get(ui.ctx());
    let (_, rect) = ui.allocate_space(Vec2::new(columns.right - columns.left, HEADER_HEIGHT));
    let cells = ProblemColumn::ALL.map(|column| {
        let left = rect.left() + columns.offset(column);
        // The message's cell runs on to the band's end, over the scrollbar's
        // allowance, so the whole band is something to press.
        let right = if column == ProblemColumn::Message {
            rect.right()
        } else {
            (left + columns.width(column)).min(rect.right())
        };
        egui::Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom()),
        )
    });
    // Each cell, then the edge on its right, so Tab reaches a column's edge
    // straight after its heading. A cell senses the pointer short of the
    // edges on either side of it, so the strip over an edge is the edge's to
    // press and drag rather than either cell's.
    let grab = ui.style().interaction.resize_grab_radius_side;
    let before = Resized::load(ui.ctx());
    let mut resized = before;
    let mut responses = Vec::with_capacity(ProblemColumn::ALL.len());
    let mut edges = Vec::with_capacity(FITTED.len());
    for (index, column) in ProblemColumn::ALL.into_iter().enumerate() {
        let cell = cells[index];
        let fitted = FITTED.contains(&column);
        let left = if index == 0 {
            cell.left()
        } else {
            cell.left() + grab
        };
        let right = if fitted {
            cell.right() - grab
        } else {
            cell.right()
        };
        let sensed = egui::Rect::from_x_y_ranges(left..=right.max(left), cell.y_range());
        responses.push(ui.interact(
            sensed,
            egui::Id::new(("workbench.problems.header", column.label())),
            Sense::click(),
        ));
        if fitted {
            edges.push(edge(ui, columns, column, cell.right(), rect, &mut resized));
        }
    }
    if resized != before {
        resized.store(ui.ctx());
        ui.ctx().request_repaint();
    }

    let hovered = responses
        .iter()
        .zip(cells)
        .find(|(response, _)| response.hovered())
        .map(|(_, cell)| cell);
    let dividers: Vec<f32> = cells[1..].iter().map(egui::Rect::left).collect();
    paint_header_band(ui, rect, &dividers, hovered);
    for edge in &edges {
        edge.paint(ui, rect);
    }

    let mut activated = None;
    for (column, response) in ProblemColumn::ALL.into_iter().zip(&responses) {
        let cell = cells[Columns::index(column)];
        let sorted = view.sort == column;
        let announcement = header_announcement(column, view);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &announcement)
        });
        if response.clicked() {
            activated = Some(column);
        }
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        let ink = if sorted {
            tokens.color.text
        } else {
            tokens.color.text_faint
        };
        let left = rect.left() + columns.offset(column) + CONSOLE_CELL_INSET;
        // The direction mark sits inside the cell, after the heading, in the
        // room every heading keeps for it.
        let room =
            (cell.right() - CONSOLE_CELL_INSET - SORT_MARK_GAP - SORT_MARK_WIDTH - left).max(0.0);
        let label = elide_tail(column.label(), room, columns);
        let width = text_width(ui, &label, FontWeight::SemiBold);
        ui.painter().text(
            egui::pos2(left, cell.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::mono(CONSOLE_FONT_SIZE, FontWeight::SemiBold),
            ink,
        );
        if sorted {
            paint_sort_direction(
                ui,
                egui::pos2(
                    left + width + SORT_MARK_GAP + SORT_MARK_WIDTH * 0.5,
                    cell.center().y,
                ),
                view.descending,
                tokens.color.accent,
            );
        }
        if theme::focus_is_visible(ui.ctx(), response) {
            theme::paint_focus_ring(ui, response, cell);
        }
    }
    activated
}

/// One column's right edge in the header, as this frame left it.
struct Edge {
    x: f32,
    response: egui::Response,
}

impl Edge {
    /// Over the band's hairline: a stronger line while the pointer is on the
    /// edge, the accent while it is dragged, and the focus ring when Tab put
    /// the keyboard on it.
    fn paint(&self, ui: &Ui, band: egui::Rect) {
        let tokens = Tokens::get(ui.ctx());
        let stroke = if self.response.dragged() {
            Some(Stroke::new(2.0, tokens.color.accent))
        } else if self.response.hovered() {
            Some(Stroke::new(2.0, tokens.color.border_strong))
        } else {
            None
        };
        if let Some(stroke) = stroke {
            ui.painter().vline(self.x, band.y_range(), stroke);
        }
        if theme::focus_is_visible(ui.ctx(), &self.response) {
            theme::paint_focus_ring(ui, &self.response, self.response.rect);
        }
    }
}

/// The control on `column`'s right edge, at `x`: what dragging it, pressing
/// it twice or keying it does to the column's width, written to `resized`.
fn edge(
    ui: &Ui,
    columns: &Columns,
    column: ProblemColumn,
    x: f32,
    band: egui::Rect,
    resized: &mut Resized,
) -> Edge {
    let index = Columns::index(column);
    let grab = ui.style().interaction.resize_grab_radius_side;
    let rect = egui::Rect::from_x_y_ranges(x - grab..=x + grab, band.y_range());
    let id = egui::Id::new(("workbench.problems.edge", column.label()));
    let response = ui.interact(rect, id, Sense::click_and_drag());
    let label = format!("{} column width", column.label());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Other, ui.is_enabled(), &label)
    });
    let least = resized_least(columns.advance);
    let most = columns.room.max(least);
    let width = columns.width(column);
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::Splitter);
        node.set_numeric_value(f64::from(width));
        node.set_min_numeric_value(f64::from(least));
        node.set_max_numeric_value(f64::from(most));
        node.set_numeric_value_step(f64::from(RESIZE_STEP));
    });
    if response.hovered() || response.dragged() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeColumn);
    }
    let focused = response.has_focus();
    if focused {
        // The arrows are the edge's while it holds the keyboard; egui would
        // otherwise read them as a move to the control beside it.
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                id,
                egui::EventFilter {
                    tab: false,
                    horizontal_arrows: true,
                    vertical_arrows: false,
                    escape: false,
                },
            );
        });
    }
    let keyed =
        |key| focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
    let step = if keyed(egui::Key::ArrowRight) {
        RESIZE_STEP
    } else if keyed(egui::Key::ArrowLeft) {
        -RESIZE_STEP
    } else {
        0.0
    };
    if step != 0.0 {
        // On the first frame the edge holds the keyboard its filter is not
        // yet in force, and egui has read the arrow as a move to a neighbour.
        ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
    }
    if response.double_clicked() || keyed(egui::Key::Home) {
        resized.0[index] = None;
    } else if response.dragged() && response.drag_delta().x != 0.0 {
        let from = resized.0[index].unwrap_or(width);
        resized.0[index] = Some((from + response.drag_delta().x).clamp(least, most));
    } else if step != 0.0 {
        resized.0[index] = Some((width + step).clamp(least, most));
    }
    Edge { x, response }
}

/// What a header cell says it will do, and what order it is already reading
/// in. Severity reads worst first when it is ascending, because that is the
/// order a reader asks for by clicking it.
fn header_announcement(column: ProblemColumn, view: &ProblemsView) -> String {
    if view.sort != column {
        return format!("Sort by {}", column.label());
    }
    let direction = match (column, view.descending) {
        (ProblemColumn::Severity, false) => "worst first",
        (ProblemColumn::Severity, true) => "least severe first",
        (_, false) => "ascending",
        (_, true) => "descending",
    };
    format!("{}, sorted {direction}", column.label())
}

/// The direction mark on the ordered column: a filled triangle, drawn. The
/// bundled face has no arrow glyphs, and a header that renders a tofu box is
/// worse than one with no mark at all.
fn paint_sort_direction(ui: &Ui, center: egui::Pos2, descending: bool, color: egui::Color32) {
    let half_width = SORT_MARK_WIDTH * 0.5;
    let half_height = 2.5;
    let points = if descending {
        vec![
            egui::pos2(center.x - half_width, center.y - half_height),
            egui::pos2(center.x + half_width, center.y - half_height),
            egui::pos2(center.x, center.y + half_height),
        ]
    } else {
        vec![
            egui::pos2(center.x - half_width, center.y + half_height),
            egui::pos2(center.x + half_width, center.y + half_height),
            egui::pos2(center.x, center.y - half_height),
        ]
    };
    ui.painter().add(egui::epaint::Shape::convex_polygon(
        points,
        color,
        Stroke::NONE,
    ));
}

/// What a row's gutter carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RowMark {
    None,
    /// A problem that counts against the design, in its own colour.
    Problem(Problem),
    /// A note: the `i`, in the note tone.
    Info,
    /// A hint: the note's `i`, a step quieter than a note.
    Hint,
    /// A finding a waiver covers: its severity's shape, without its colour.
    Waived(StatusMark),
}

/// One finding, as the grid prints it.
pub(super) struct RowCells<'a> {
    pub(super) mark: RowMark,
    pub(super) severity: &'a str,
    pub(super) code: &'a str,
    pub(super) subject: &'a str,
    pub(super) producer: &'a str,
    pub(super) location: &'a str,
    pub(super) message: &'a str,
    /// Whether the finding describes the design as it stands.
    pub(super) current: bool,
    /// Whether following it leads somewhere the page has checked.
    pub(super) target: bool,
    /// Whether it is the one current row.
    pub(super) selected: bool,
}

/// The identity a row is addressed by, so the current row survives the table
/// being sorted, filtered or regrouped under it.
pub(super) fn row_id(finding: uuid::Uuid) -> egui::Id {
    egui::Id::new(("workbench.problems.row", finding))
}

/// The identity of the table's one keyboard stop.
pub(super) fn body_id() -> egui::Id {
    egui::Id::new("workbench.problems.grid")
}

/// The table's one keyboard stop, laid over the body's `rect`.
///
/// One stop rather than one per row: a reader tabbing through the page passes
/// the table in one press, not one per finding, and moving through it is what
/// the arrows are for. Holding focus here rather than on a row also keeps the
/// lock filter in force from row to row — egui applies a filter only to a
/// widget that held focus the frame before, so a filter set on each row as
/// the arrows reached it arrived a frame late, and a quick second arrow moved
/// egui's own focus past a row.
///
/// `current` is the identity and rectangle of the line the keyboard stands on
/// — a finding's row or a group's header — where it is drawn: it is announced
/// as the focused list's active item, and it carries the focus ring that Tab
/// shows.
pub(super) fn body(
    ui: &mut Ui,
    rect: egui::Rect,
    current: Option<(egui::Id, egui::Rect)>,
) -> egui::Response {
    let id = body_id();
    let response = ui.interact(rect, id, Sense::FOCUSABLE);
    // The table owns the arrows and Escape while it holds focus. Without this
    // egui takes them first — it reads the arrows as a request to move focus
    // to the widget beside, above or below, and Escape as a request to
    // surrender it, both at the start of the pass and therefore before this
    // page can see them. Tab is deliberately left to egui, because Tab is how
    // a reader leaves the table.
    if response.has_focus() {
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                id,
                egui::EventFilter {
                    tab: false,
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    escape: true,
                },
            );
        });
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Other, ui.is_enabled(), "Findings")
    });
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::ListBox);
        if let Some((row, _)) = current {
            node.set_active_descendant(row.accesskit_id());
        }
    });
    // Focus a pointer press gave must not frame anything; focus Tab gave
    // frames the line the keys will move from, or the body while there is
    // none.
    // A row the body's edge cuts is framed where it shows.
    if theme::focus_is_visible(ui.ctx(), &response) {
        let framed = current
            .map(|(_, row)| row.intersect(rect))
            .filter(|row| row.is_positive())
            .unwrap_or(rect);
        theme::paint_focus_ring(ui, &response, framed);
    }
    response
}

/// Paint one finding and return its response.
///
/// Painted rather than built from widgets, so the row declares itself: it is
/// an option of the list the table is, and says whether it is the current one
/// as a list says it — selected — rather than as a toggle pressed or not.
pub(super) fn row(
    ui: &mut Ui,
    columns: &Columns,
    cells: &RowCells<'_>,
    id: egui::Id,
) -> egui::Response {
    let tokens = Tokens::get(ui.ctx());
    let (dim, body) = if cells.current {
        (tokens.color.text_dim, tokens.color.text)
    } else {
        (tokens.color.text_faint, tokens.color.text_faint)
    };
    let (_, rect) = ui.allocate_space(Vec2::new(ui.available_width(), CONSOLE_ROW_MIN_HEIGHT));
    // Every row senses a click because every row can become the current row,
    // and none takes focus: the table is one keyboard stop, held by `body`.
    // What a row that cannot be followed does not do is *look* like a jump: it
    // leaves the pointer alone.
    let response = ui.interact(rect, id, Sense::CLICK);
    let announced = match cells.mark {
        RowMark::Problem(problem) => Some(problem),
        RowMark::Info | RowMark::Hint | RowMark::Waived(_) | RowMark::None => None,
    };
    let label = announced_row(
        announced,
        &[
            cells.severity,
            cells.code,
            cells.subject,
            cells.producer,
            cells.location,
            cells.message,
        ],
    );
    // Declared as a selectable label so the row's words are its name, then
    // given the role and state a list option has; egui's own projection of a
    // selection is a toggle. Every row has the one role, so a row that leads
    // nowhere says so, where the pointer is told by its cursor.
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            label.clone(),
        )
    });
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::ListBoxOption);
        node.set_selected(cells.selected);
        if !cells.target {
            node.set_description("Has no location to go to");
        }
    });
    if cells.selected {
        paint_current(ui, rect, columns.right());
    } else if response.hovered() {
        paint_row_fill(ui, rect, columns.right(), tokens.color.bg_hover);
    }
    let mark_left = rect.left() + columns.offset(ProblemColumn::Severity) + CONSOLE_CELL_INSET;
    let mark = egui::Rect::from_center_size(
        egui::pos2(
            mark_left + CONSOLE_MARK_SIDE * 0.5,
            rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5,
        ),
        Vec2::splat(CONSOLE_MARK_SIDE),
    );
    match cells.mark {
        RowMark::Problem(problem) => paint_row_mark(ui, mark_left, rect.top(), Some(problem)),
        RowMark::Info => {
            paint_status_mark(ui.painter(), mark, StatusMark::Info, tokens.color.info);
        }
        RowMark::Hint => paint_status_mark(ui.painter(), mark, StatusMark::Info, dim),
        RowMark::Waived(shape) => {
            paint_status_mark(ui.painter(), mark, shape, tokens.color.text_faint);
        }
        RowMark::None => {}
    }
    let center_y = rect.center().y;
    for (column, text, weight) in [
        (ProblemColumn::Severity, cells.severity, FontWeight::Regular),
        (ProblemColumn::Code, cells.code, FontWeight::Regular),
        (ProblemColumn::Subject, cells.subject, FontWeight::Medium),
        (ProblemColumn::Producer, cells.producer, FontWeight::Regular),
        (ProblemColumn::Location, cells.location, FontWeight::Regular),
        (ProblemColumn::Message, cells.message, FontWeight::Regular),
    ] {
        if text.is_empty() {
            continue;
        }
        let room = columns.text_room(column);
        // A path is read from its tail: the file and the object at the end of
        // it are what say where to go, and the directories above them are not,
        // so those columns drop their head. A sentence is read from its start.
        let printed = match column {
            ProblemColumn::Subject | ProblemColumn::Producer | ProblemColumn::Location => {
                elide_head(text, room, columns)
            }
            ProblemColumn::Severity | ProblemColumn::Code | ProblemColumn::Message => {
                elide_tail(text, room, columns)
            }
        };
        ui.painter().text(
            egui::pos2(rect.left() + columns.text_offset(column), center_y),
            egui::Align2::LEFT_CENTER,
            printed,
            theme::mono(CONSOLE_FONT_SIZE, weight),
            if column == ProblemColumn::Message {
                body
            } else {
                dim
            },
        );
    }
    if cells.target {
        return response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }
    response
}

/// The mark of the line the keyboard stands on: the selection fill, from the
/// dock's left edge to its right, and the accent edge along the left. The
/// same on a finding's row as on a group's header.
fn paint_current(ui: &Ui, rect: egui::Rect, right: f32) {
    let tokens = Tokens::get(ui.ctx());
    paint_row_fill(ui, rect, right, tokens.color.accent.linear_multiply(0.14));
    // Inside the row, so neither of its two points falls off the dock.
    ui.painter().vline(
        rect.left() + 1.0,
        rect.y_range(),
        Stroke::new(2.0, tokens.color.accent),
    );
}

/// The identity a group's header is addressed by, so it survives the rows
/// under it changing.
pub(super) fn group_id(label: &str) -> egui::Id {
    egui::Id::new(("workbench.problems.group", label))
}

/// A group's own header: what it groups on, what it holds, and whether it is
/// open. `current` is whether the keyboard stands on it.
///
/// Like a row it is not a keyboard stop of its own: it is a line the table's
/// arrows stand on, and Left, Right, Enter and Space close and open it from
/// there. Its chevron stands where a row's mark does, its value where a row's
/// severity does, and its counts end where the message's text ends, short of
/// the scrollbar.
pub(super) fn group_header(
    ui: &mut Ui,
    columns: &Columns,
    group: &Group,
    current: bool,
) -> egui::Response {
    let tokens = Tokens::get(ui.ctx());
    let (_, rect) = ui.allocate_space(Vec2::new(ui.available_width(), CONSOLE_ROW_MIN_HEIGHT));
    let response = ui.interact(rect, group_id(&group.label), Sense::click());
    let summary = group.counts.summary();
    let announcement = format!("{} · {summary}", group.label);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            announcement.clone(),
        )
    });
    // Open or closed is the state the control carries, and a reader who
    // cannot see the chevron has nothing else to hear it from. It is stated as
    // expanded or collapsed, which is what it is, rather than as a toggle
    // pressed or not.
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(!group.collapsed);
    });
    paint_row_fill(ui, rect, columns.right(), tokens.color.bg_panel_2);
    if current {
        paint_current(ui, rect, columns.right());
    } else if response.hovered() {
        paint_row_fill(ui, rect, columns.right(), tokens.color.bg_hover);
    }
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let mark_left = rect.left() + columns.offset(ProblemColumn::Severity) + CONSOLE_CELL_INSET;
    paint_chevron(
        ui,
        egui::pos2(mark_left + CONSOLE_MARK_SIDE * 0.5, rect.center().y),
        group.collapsed,
        tokens.color.text_dim,
    );
    let label_left = rect.left() + columns.text_offset(ProblemColumn::Severity);
    let summary_right = rect.left()
        + columns.text_offset(ProblemColumn::Message)
        + columns.text_room(ProblemColumn::Message);
    let summary_width = text_width(ui, &summary, FontWeight::Regular);
    let label_room = (summary_right - summary_width - 2.0 * CONSOLE_CELL_INSET - label_left)
        .max(columns.advance);
    ui.painter().text(
        egui::pos2(label_left, rect.center().y),
        egui::Align2::LEFT_CENTER,
        elide_head(&group.label, label_room, columns),
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::SemiBold),
        tokens.color.text,
    );
    ui.painter().text(
        egui::pos2(summary_right, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        summary,
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
        tokens.color.text_dim,
    );
    response
}

/// A group's open/closed mark, drawn: the bundled face has no chevron glyph.
fn paint_chevron(ui: &Ui, center: egui::Pos2, collapsed: bool, color: egui::Color32) {
    let half = 3.0;
    let points = if collapsed {
        vec![
            egui::pos2(center.x - half * 0.6, center.y - half),
            egui::pos2(center.x + half * 0.6, center.y),
            egui::pos2(center.x - half * 0.6, center.y + half),
        ]
    } else {
        vec![
            egui::pos2(center.x - half, center.y - half * 0.6),
            egui::pos2(center.x + half, center.y - half * 0.6),
            egui::pos2(center.x, center.y + half * 0.6),
        ]
    };
    ui.painter().add(egui::epaint::Shape::convex_polygon(
        points,
        color,
        Stroke::NONE,
    ));
}

/// Fit `text` into `width`, dropping characters from the front and marking the
/// cut. Exact rather than iterative because the face is monospace.
pub(super) fn elide_head(text: &str, width: f32, columns: &Columns) -> String {
    let characters = text.chars().count();
    if characters as f32 * columns.advance <= width {
        return text.to_owned();
    }
    let fits = fitting_characters(width, columns);
    if fits == 0 {
        return "…".to_owned();
    }
    let tail: String = text.chars().skip(characters.saturating_sub(fits)).collect();
    format!("…{tail}")
}

/// Fit `text` into `width`, dropping characters from the end.
pub(super) fn elide_tail(text: &str, width: f32, columns: &Columns) -> String {
    let characters = text.chars().count();
    if characters as f32 * columns.advance <= width {
        return text.to_owned();
    }
    let fits = fitting_characters(width, columns);
    if fits == 0 {
        return "…".to_owned();
    }
    let head: String = text.chars().take(fits).collect();
    format!("{head}…")
}

/// How many characters fit beside the ellipsis that marks the cut.
fn fitting_characters(width: f32, columns: &Columns) -> usize {
    let room = (width - columns.ellipsis).max(0.0);
    (room / columns.advance).floor().max(0.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fixture whose advance is exactly 7 points, so the arithmetic every
    /// elision does is checkable by hand.
    fn columns() -> Columns {
        Columns {
            left: 0.0,
            right: 600.0,
            starts: [0.0, 70.0, 140.0, 210.0, 280.0, 350.0],
            widths: [70.0; 6],
            room: 600.0,
            advance: 7.0,
            ellipsis: 7.0,
        }
    }

    #[test]
    fn a_cell_that_fits_is_printed_whole() {
        let columns = columns();
        assert_eq!(elide_head("Component M1", 100.0, &columns), "Component M1");
        assert_eq!(elide_tail("Component M1", 100.0, &columns), "Component M1");
    }

    /// A path is read from its tail: the file and the object at the end of it
    /// say where to go, and the directories above them do not.
    #[test]
    fn a_long_path_keeps_its_tail() {
        let columns = columns();
        let elided = elide_head("models/analog/opamp.sp", 70.0, &columns);
        assert!(elided.starts_with('…'), "{elided} must mark the cut");
        assert!(elided.ends_with("opamp.sp"), "{elided} must keep its tail");
        assert_eq!(
            elided.chars().count(),
            10,
            "{elided} must fit the width it was measured against",
        );
    }

    /// A sentence is read from its start.
    #[test]
    fn a_long_sentence_keeps_its_start() {
        let columns = columns();
        let elided = elide_tail("Node has only one connection", 70.0, &columns);
        assert!(elided.ends_with('…'), "{elided} must mark the cut");
        assert!(elided.starts_with("Node has"), "{elided}");
        assert_eq!(elided.chars().count(), 10);
    }

    /// A column too narrow for even one character prints the cut alone rather
    /// than nothing at all.
    #[test]
    fn a_column_with_no_room_prints_the_cut() {
        assert_eq!(elide_head("Component M1", 3.0, &columns()), "…");
        assert_eq!(elide_tail("Component M1", 3.0, &columns()), "…");
    }

    /// What [`Columns::measure`] decides for a table `width` wide, in the face
    /// the page prints in, with the room it was measured against.
    struct Measured {
        columns: Columns,
        /// Where the last column has to end: the table less the scrollbar it
        /// allows for.
        end: f32,
        advance: f32,
        headings: [f32; FITTED.len()],
    }

    fn measured(width: f32, widest: [usize; FITTED.len()], resized: Resized) -> Measured {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        resized.store(&ctx);
        let mut measured = None;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 300.0),
            )),
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ui| {
            let scrollbar = ui.spacing().scroll.bar_width + ui.spacing().scroll.bar_inner_margin;
            let band = ui.available_rect_before_wrap().x_range();
            measured = Some(Measured {
                columns: Columns::measure(ui, &widest, band),
                end: band.span() - scrollbar,
                advance: text_width(ui, "M", FontWeight::Regular),
                headings: FITTED.map(|column| {
                    text_width(ui, column.label(), FontWeight::SemiBold)
                        + SORT_MARK_GAP
                        + SORT_MARK_WIDTH
                }),
            });
        });
        measured.expect("the pass ran")
    }

    /// At every width the columns abut, and the last ends exactly where the
    /// scrollbar's allowance begins: nothing runs under the bar, and nothing
    /// is left over at the end of the row. The first column's text stands at
    /// the page's text inset, where the toolbar's first control does.
    #[test]
    fn the_columns_abut_and_end_where_the_scrollbar_begins() {
        for width in [640.0, 900.0, 1400.0, 1886.0] {
            let measured = measured(width, [7, 12, 10, 19, 18], Resized::default());
            let columns = &measured.columns;
            let end =
                columns.offset(ProblemColumn::Message) + columns.width(ProblemColumn::Message);
            assert!(
                (end - measured.end).abs() < 0.5,
                "at {width} px the row ends at {end} rather than at {}",
                measured.end,
            );
            for pair in ProblemColumn::ALL.windows(2) {
                let gap =
                    columns.offset(pair[1]) - (columns.offset(pair[0]) + columns.width(pair[0]));
                assert!(
                    gap.abs() < 0.01,
                    "{} and {} are {gap} apart at {width} px",
                    pair[0].label(),
                    pair[1].label(),
                );
            }
            assert!(
                (columns.offset(ProblemColumn::Severity) + CONSOLE_CELL_INSET
                    - CONSOLE_BODY_PADDING_X)
                    .abs()
                    < 0.01,
                "the first column's text stands at the page's inset at {width} px",
            );
        }
    }

    /// With room to spare every fitted column is its widest cell and two cell
    /// insets — the same gutter after every column — and the message takes
    /// the rest. The severity column's cell is its mark and its word.
    #[test]
    fn fitted_columns_are_their_widest_cell_and_two_insets() {
        let widest = [7, 12, 10, 19, 18];
        let measured = measured(1886.0, widest, Resized::default());
        for (index, column) in FITTED.into_iter().enumerate() {
            let mark = if column == ProblemColumn::Severity {
                CONSOLE_MARK_GUTTER
            } else {
                0.0
            };
            let content =
                (mark + widest[index] as f32 * measured.advance).max(measured.headings[index]);
            assert!(
                (measured.columns.width(column) - (content + 2.0 * CONSOLE_CELL_INSET)).abs()
                    < 0.01,
                "{} is {} wide for content {content}",
                column.label(),
                measured.columns.width(column),
            );
        }
    }

    /// A column's heading is its floor, and a short column is as wide as its
    /// heading rather than its cells: severity read "Info" alone is still
    /// headed "Severity".
    #[test]
    fn a_heading_wider_than_its_cells_sets_the_width() {
        let measured = measured(1400.0, [4, 1, 1, 1, 1], Resized::default());
        for (index, column) in FITTED.into_iter().enumerate() {
            assert!(
                (measured.columns.width(column)
                    - (measured.headings[index] + 2.0 * CONSOLE_CELL_INSET))
                    .abs()
                    < 0.01,
                "{} is as wide as its heading",
                column.label(),
            );
        }
    }

    /// On a narrow table the fitted columns stop at their share and give way
    /// to the message's least; a column cut there elides rather than taking
    /// the row.
    #[test]
    fn on_a_narrow_table_the_message_keeps_its_least() {
        let narrow = measured(640.0, [15, 40, 40, 40, 40], Resized::default());
        assert!(
            narrow.columns.width(ProblemColumn::Producer) < 40.0 * narrow.advance,
            "a producer identity wider than its share is cut, not given the row",
        );
        assert!(
            narrow.columns.width(ProblemColumn::Message)
                >= MESSAGE_MIN_CHARACTERS * narrow.advance + 2.0 * CONSOLE_CELL_INSET - 0.01,
            "the message keeps room for its least at 640 px",
        );
    }

    /// Where the columns must give way, the widest gives first: the severity
    /// word and a short name keep their cells whole, and the long rule is
    /// what is cut.
    #[test]
    fn on_a_narrow_table_the_widest_column_gives_way_first() {
        let widest = [8, 39, 10, 12, 10];
        let narrow = measured(640.0, widest, Resized::default());
        let message_least = MESSAGE_MIN_CHARACTERS * narrow.advance + 2.0 * CONSOLE_CELL_INSET;
        assert!(
            (narrow.columns.width(ProblemColumn::Message) - message_least).abs() < 0.5,
            "precondition: at 640 px the columns give way to the message",
        );
        let whole = |index: usize| {
            let mark = if index == 0 { CONSOLE_MARK_GUTTER } else { 0.0 };
            (mark + widest[index] as f32 * narrow.advance).max(narrow.headings[index])
                + 2.0 * CONSOLE_CELL_INSET
        };
        for index in [0, 2, 3, 4] {
            assert!(
                (narrow.columns.width(FITTED[index]) - whole(index)).abs() < 0.01,
                "{} keeps its cells whole at 760 px",
                FITTED[index].label(),
            );
        }
        assert!(
            narrow.columns.width(ProblemColumn::Code) < whole(1) - narrow.advance,
            "the rule is cut",
        );
    }

    /// A width a reader dragged a column to is the column's width, whatever it
    /// lists, and the message gives or takes the difference.
    #[test]
    fn a_dragged_width_holds() {
        let mut resized = Resized::default();
        resized.0[1] = Some(300.0);
        let measured = measured(1400.0, [7, 12, 10, 19, 18], resized);
        assert!((measured.columns.width(ProblemColumn::Code) - 300.0).abs() < 0.01);
        let end = measured.columns.offset(ProblemColumn::Message)
            + measured.columns.width(ProblemColumn::Message);
        assert!((end - measured.end).abs() < 0.5);
    }
}
