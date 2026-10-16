//! The table itself: its column measurements, its header, and its rows.
//!
//! Every width here is measured in the face the cells are printed in. The
//! severity column is measured against the longest severity cell the page can
//! print, the rule and producer columns against the longest code and producer
//! identity the listed rows actually carry, and what is left is shared between
//! the three columns of prose. Nothing is a pixel count somebody typed: the
//! first version of this page fixed two columns at 58 px and 62 px — eight
//! characters — and could not print a location at all.

use egui::{Sense, Stroke, Ui, Vec2};

use super::super::{
    CONSOLE_COLUMN_GAP, CONSOLE_FONT_SIZE, CONSOLE_MARK_GUTTER, CONSOLE_MARK_SIDE,
    CONSOLE_ROW_MIN_HEIGHT, announced_row, paint_row_mark,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Problem, paint_status_mark};
use crate::workbench::app_state::problems_view::{ProblemColumn, ProblemsView};

use super::arrange::Group;

/// The longest severity cell the page can print. The severity column is
/// measured against this, so no severity word is ever cut.
pub(super) const WIDEST_SEVERITY_CELL: &str = "Waived Critical";

/// Shares of the room left after the mark gutter, the severity column and the
/// gaps. The rule and producer columns take what their content needs up to
/// their share — both are identities a reader has to be able to read whole,
/// so their share is generous and the prose columns elide instead. The three
/// columns of prose split what remains.
const CODE_MAX_SHARE: f32 = 0.22;
const PRODUCER_MAX_SHARE: f32 = 0.22;
const SUBJECT_SHARE: f32 = 0.30;
const LOCATION_SHARE: f32 = 0.24;

/// The header row's own height. One line, like every row under it.
pub(super) const HEADER_HEIGHT: f32 = CONSOLE_ROW_MIN_HEIGHT + 4.0;

/// Where each column starts and how wide it is, for one pass of the page.
pub(super) struct Columns {
    severity: f32,
    code: f32,
    subject: f32,
    producer: f32,
    location: f32,
    message: f32,
    /// Width of one monospace character, which is what makes eliding a cell
    /// exact instead of iterative: in a monospace face the number of
    /// characters that fit is a division.
    advance: f32,
    ellipsis: f32,
}

impl Columns {
    /// Measure the columns against the font, the room, and the widest code and
    /// producer identity the listed rows carry.
    pub(super) fn measure(ui: &Ui, widest_code: usize, widest_producer: usize) -> Self {
        let advance = text_width(ui, "M", FontWeight::Regular).max(1.0);
        let ellipsis = text_width(ui, "…", FontWeight::Regular);
        let severity = text_width(ui, WIDEST_SEVERITY_CELL, FontWeight::Regular);
        // The body's scrollbar is allowed for whether or not it is showing.
        // A scroll area's own width includes its bar and changes the frame it
        // appears, so columns measured against it would shift under the reader
        // as the list grew past the viewport — and the header, which is drawn
        // outside the area, would stop lining up with the rows inside it.
        let scrollbar = ui.spacing().scroll.bar_width + ui.spacing().scroll.bar_inner_margin;
        let room = (ui.available_width()
            - scrollbar
            - CONSOLE_MARK_GUTTER
            - severity
            - CONSOLE_COLUMN_GAP * 5.0)
            .max(advance);
        // A column that cannot print its own heading says nothing, so each
        // measured column asks for at least its heading.
        let code = text_width(ui, ProblemColumn::Code.label(), FontWeight::SemiBold)
            .max(widest_code as f32 * advance)
            .min(room * CODE_MAX_SHARE);
        let producer = text_width(ui, ProblemColumn::Producer.label(), FontWeight::SemiBold)
            .max(widest_producer as f32 * advance)
            .min(room * PRODUCER_MAX_SHARE);
        let prose = (room - code - producer).max(advance);
        let subject = prose * SUBJECT_SHARE;
        let location = prose * LOCATION_SHARE;
        Self {
            severity,
            code,
            subject,
            producer,
            location,
            message: (prose - subject - location).max(advance),
            advance,
            ellipsis,
        }
    }

    pub(super) const fn width(&self, column: ProblemColumn) -> f32 {
        match column {
            ProblemColumn::Severity => self.severity,
            ProblemColumn::Code => self.code,
            ProblemColumn::Subject => self.subject,
            ProblemColumn::Producer => self.producer,
            ProblemColumn::Location => self.location,
            ProblemColumn::Message => self.message,
        }
    }

    /// How far into a row a column starts, measured from the row's left edge.
    pub(super) fn offset(&self, column: ProblemColumn) -> f32 {
        let mut x = CONSOLE_MARK_GUTTER;
        for earlier in ProblemColumn::ALL {
            if earlier == column {
                break;
            }
            x += self.width(earlier) + CONSOLE_COLUMN_GAP;
        }
        x
    }
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

/// The column header row. Each cell orders the table by its own column.
///
/// Returns the column a reader activated this frame. Every cell is a real
/// control: it announces as a button with the order it would apply said in
/// words, takes keyboard focus, and activates on Enter and Space as well as
/// on a click — a direction glyph alone cannot be heard.
pub(super) fn header(ui: &mut Ui, columns: &Columns, view: &ProblemsView) -> Option<ProblemColumn> {
    let tokens = Tokens::get(ui.ctx());
    let (_, rect) = ui.allocate_space(Vec2::new(ui.available_width(), HEADER_HEIGHT));
    // One step up from the body's own inset, in both themes: the console body
    // is already painted in `bg_inset`, so a header painted in it would be a
    // band nobody can see.
    ui.painter().rect_filled(rect, 0.0, tokens.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, tokens.color.border),
    );
    let mut activated = None;
    for column in ProblemColumn::ALL {
        let left = rect.left() + columns.offset(column);
        let cell = egui::Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(
                (left + columns.width(column)).min(rect.right()),
                rect.bottom(),
            ),
        );
        let sorted = view.sort == Some(column);
        let response = ui.interact(
            cell,
            egui::Id::new(("workbench.problems.header", column.label())),
            Sense::click(),
        );
        let announcement = header_announcement(column, view);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &announcement)
        });
        if response.clicked() {
            activated = Some(column);
        }
        if response.hovered() {
            ui.painter().rect_filled(cell, 0.0, tokens.color.bg_hover);
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        let ink = if sorted {
            tokens.color.text
        } else {
            tokens.color.text_faint
        };
        // The direction mark sits inside the cell, after the heading, so a
        // narrow column cuts the word rather than losing the order it is read
        // in.
        let mark_room = if sorted { CONSOLE_COLUMN_GAP } else { 0.0 };
        let label = elide_tail(column.label(), (cell.width() - mark_room).max(0.0), columns);
        let width = text_width(ui, &label, FontWeight::SemiBold);
        ui.painter().text(
            egui::pos2(cell.left(), cell.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::mono(CONSOLE_FONT_SIZE, FontWeight::SemiBold),
            ink,
        );
        if sorted {
            paint_sort_direction(
                ui,
                egui::pos2(cell.left() + width + 4.0, cell.center().y),
                view.descending,
                tokens.color.accent,
            );
        }
        if theme::focus_is_visible(ui.ctx(), &response) {
            theme::paint_focus_ring(ui, &response, cell);
        }
    }
    activated
}

/// What a header cell says it will do, and what order it is already reading
/// in. Severity reads worst first when it is ascending, because that is the
/// order a reader asks for by clicking it.
fn header_announcement(column: ProblemColumn, view: &ProblemsView) -> String {
    if view.sort != Some(column) {
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
    let half_width = 3.5;
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
    /// A problem a waiver covers: its shape, without its colour.
    Waived(Problem),
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
/// `current` is the current row's identity and rectangle, where it is drawn:
/// it is announced as the focused list's active row, and it carries the focus
/// ring that Tab shows.
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
    // frames the row the keys will move, or the body while none is current.
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
/// Painted rather than built from widgets, so the row declares itself: a row
/// that can be followed announces as a button, a row that cannot is still text
/// a screen reader reads, and both say whether they are the current row.
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
    // announces as text rather than as a button and leaves the pointer alone.
    let response = ui.interact(rect, id, Sense::CLICK);
    let announced = match cells.mark {
        RowMark::Problem(problem) => Some(problem),
        RowMark::Waived(_) | RowMark::None => None,
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
    let selected = cells.selected;
    let target = cells.target;
    response.widget_info(|| {
        let mut info = egui::WidgetInfo::labeled(
            if target {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            label.clone(),
        );
        info.selected = Some(selected);
        info
    });
    if cells.selected {
        ui.painter()
            .rect_filled(rect, 0.0, tokens.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, tokens.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
    }
    match cells.mark {
        RowMark::Problem(problem) => paint_row_mark(ui, rect.left(), rect.top(), Some(problem)),
        RowMark::Waived(problem) => paint_status_mark(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    rect.left() + CONSOLE_MARK_SIDE * 0.5,
                    rect.top() + CONSOLE_ROW_MIN_HEIGHT * 0.5,
                ),
                Vec2::splat(CONSOLE_MARK_SIDE),
            ),
            problem.mark(),
            tokens.color.text_faint,
        ),
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
        let width = columns.width(column);
        // A path is read from its tail: the file and the object at the end of
        // it are what say where to go, and the directories above them are not,
        // so those columns drop their head. A sentence is read from its start.
        let printed = match column {
            ProblemColumn::Subject | ProblemColumn::Producer | ProblemColumn::Location => {
                elide_head(text, width, columns)
            }
            ProblemColumn::Severity | ProblemColumn::Code | ProblemColumn::Message => {
                elide_tail(text, width, columns)
            }
        };
        ui.painter().text(
            egui::pos2(rect.left() + columns.offset(column), center_y),
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

/// A group's own header: what it groups on, what it holds, and whether it is
/// open. Returns true when the reader asked to collapse or expand it.
///
/// Like a row it is not a keyboard stop of its own; from the keyboard, Left
/// and Right close and open the current finding's group.
pub(super) fn group_header(ui: &mut Ui, columns: &Columns, group: &Group) -> bool {
    let tokens = Tokens::get(ui.ctx());
    let (_, rect) = ui.allocate_space(Vec2::new(ui.available_width(), CONSOLE_ROW_MIN_HEIGHT));
    let response = ui.interact(
        rect,
        egui::Id::new(("workbench.problems.group", group.label.as_str())),
        Sense::CLICK,
    );
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
    ui.painter().rect_filled(rect, 0.0, tokens.color.bg_panel_2);
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    paint_chevron(
        ui,
        egui::pos2(rect.left() + CONSOLE_MARK_GUTTER * 0.5, rect.center().y),
        group.collapsed,
        tokens.color.text_dim,
    );
    let label_left = rect.left() + CONSOLE_MARK_GUTTER;
    let summary_width = text_width(ui, &summary, FontWeight::Regular);
    let label_room = (rect.width() - CONSOLE_MARK_GUTTER - summary_width - CONSOLE_COLUMN_GAP)
        .max(columns.advance);
    ui.painter().text(
        egui::pos2(label_left, rect.center().y),
        egui::Align2::LEFT_CENTER,
        elide_head(&group.label, label_room, columns),
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::SemiBold),
        tokens.color.text,
    );
    ui.painter().text(
        egui::pos2(
            label_left + label_room + CONSOLE_COLUMN_GAP,
            rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        summary,
        theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular),
        tokens.color.text_dim,
    );
    response.clicked()
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
            severity: 70.0,
            code: 70.0,
            subject: 70.0,
            producer: 70.0,
            location: 70.0,
            message: 70.0,
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

    /// What [`Columns::measure`] decides at one page width, in the face the
    /// page prints in, with the room it was measured against.
    struct Measured {
        columns: Columns,
        /// The page's width less the scrollbar it allows for.
        room: f32,
        advance: f32,
        code_heading: f32,
    }

    fn measured(width: f32, widest_code: usize, widest_producer: usize) -> Measured {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut measured = None;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width, 300.0),
            )),
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let scrollbar =
                    ui.spacing().scroll.bar_width + ui.spacing().scroll.bar_inner_margin;
                measured = Some(Measured {
                    columns: Columns::measure(ui, widest_code, widest_producer),
                    room: ui.available_width() - scrollbar,
                    advance: text_width(ui, "M", FontWeight::Regular),
                    code_heading: text_width(ui, ProblemColumn::Code.label(), FontWeight::SemiBold),
                });
            });
        });
        measured.expect("the panel ran")
    }

    /// At every width, the six columns and their gaps end exactly where the
    /// scrollbar's allowance begins: nothing runs under the bar, and nothing
    /// is left over at the end of the row.
    #[test]
    fn the_columns_fill_the_row_short_of_the_scrollbar() {
        for width in [640.0, 900.0, 1400.0] {
            let measured = measured(width, 12, 18);
            let columns = &measured.columns;
            let end =
                columns.offset(ProblemColumn::Message) + columns.width(ProblemColumn::Message);
            assert!(
                (end - measured.room).abs() < 0.5,
                "at {width} px the row ends at {end} rather than at {}",
                measured.room,
            );
            for pair in ProblemColumn::ALL.windows(2) {
                let gap =
                    columns.offset(pair[1]) - (columns.offset(pair[0]) + columns.width(pair[0]));
                assert!(
                    (gap - CONSOLE_COLUMN_GAP).abs() < 0.01,
                    "{} and {} are {gap} apart at {width} px",
                    pair[0].label(),
                    pair[1].label(),
                );
            }
            assert!(
                columns.width(ProblemColumn::Code) + 0.01 >= measured.code_heading,
                "the rule column prints its own heading at {width} px",
            );
        }
    }

    /// With room to spare the identity columns print their widest cell whole;
    /// on a narrow page they stop at their share and the prose columns keep
    /// room to say something.
    #[test]
    fn identity_columns_fit_their_widest_cell_until_room_runs_out() {
        let wide = measured(1400.0, 12, 18);
        assert!(wide.columns.width(ProblemColumn::Code) + 0.01 >= 12.0 * wide.advance);
        assert!(wide.columns.width(ProblemColumn::Producer) + 0.01 >= 18.0 * wide.advance);

        let narrow = measured(640.0, 40, 40);
        assert!(
            narrow.columns.width(ProblemColumn::Producer) < 40.0 * narrow.advance,
            "a producer identity wider than its share is cut, not given the row",
        );
        assert!(
            narrow.columns.width(ProblemColumn::Message) >= 10.0 * narrow.advance,
            "the message keeps room for ten characters at 640 px",
        );
    }
}
