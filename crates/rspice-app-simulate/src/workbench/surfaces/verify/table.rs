//! The one result table every Verify page reads its rows in.
//!
//! It is the grid of an ADE results view, not a card: 24 px heads and rows,
//! heads as written (sentence case, no tracking), names in sans, numbers in
//! mono and right-aligned so their digits line up, hairlines between rows and
//! columns. The only colour in a row is a verdict cell's — a failure in the
//! error tone, a value near its limit in the warning tone, a pass neutral — and
//! the accent wash of the row the reader selected. A problem the reader must
//! notice in a cell is a mark in front of neutral words, never coloured words.
//!
//! The first column's words start on the panel inset and the last column's end
//! on it, so a table's text lines up with the toolbar title above it. Columns
//! that do not fit scroll sideways rather than squeezing their numbers; the
//! leading frozen columns stay put while the rest scroll under them, and band
//! rows over the head name a run of columns once. Only the rows and columns in
//! view are laid out.
//!
//! A page that hands the table a [`TableState`] gets the rest of a results
//! grid: a selection the keyboard moves, a sort a head click sets, foldable
//! group heads, a row filter, and Ctrl+C. A page that hands it none gets the
//! rows it asked for, in its own order, and reports clicks.

mod body;
mod geometry;
mod head;
mod state;
#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::BTreeSet;

use egui::{Key, Modifiers, Rect, Sense, Ui, pos2, vec2};

use crate::ui::theme;
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Problem, paint_problem_mark};
use crate::workbench::design_system::elide_text;

pub use geometry::Span;
pub use state::{Group, RowKey, Selection, Sort, SortDirection, SortKey, TableState};

use geometry::Columns;
use state::{Line, Step, arrange, step};

/// Height of a table's head band, and of each band row over it.
pub(super) const TABLE_HEAD_H: f32 = 24.0;
/// Height of one table row.
pub(super) const TABLE_ROW_H: f32 = 24.0;
/// Where the first column's words start and the last column's end: the panel
/// inset, so a table's text lines up with the toolbar title over it.
pub(super) const TABLE_EDGE_INSET: f32 = crate::workbench::design_system::PANEL_INSET;
/// Space between a cell's words and the rule on either side of it.
const CELL_PAD: f32 = 8.0;
/// A problem mark in a cell, and the advance from its left edge to the words:
/// the kit's own, so a marked cell matches every other marked line.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};

/// The head and row heights: 24 px under a fine pointer, the touch target the
/// preferences ask for under a coarse one.
pub(super) fn line_heights(t: &Tokens) -> (f32, f32) {
    if t.metrics.is_touch() {
        let target = t.metrics.touch_target();
        (target, target)
    } else {
        (TABLE_HEAD_H, TABLE_ROW_H)
    }
}

/// Which edge of its cell a column's words sit on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Align {
    Left,
    Right,
}

/// One column: its head, how wide it is, and where its words sit.
#[derive(Debug, Clone)]
pub struct Column {
    pub(super) head: String,
    /// What the head stands for, shown when the pointer rests on it.
    pub(super) tooltip: Option<String>,
    /// The column's width, or its least width when it takes a share of what
    /// the fixed columns leave.
    width: f32,
    /// Share of the spare width; zero for a fixed column.
    flex: f32,
    pub(super) align: Align,
    /// A head click sorts by this column.
    sortable: bool,
}

impl Column {
    fn with(head: impl Into<String>, width: f32, flex: f32, align: Align) -> Self {
        Self {
            head: head.into(),
            tooltip: None,
            width,
            flex,
            align,
            sortable: true,
        }
    }

    /// A column of names that takes the width the fixed columns leave.
    pub(super) fn name(head: impl Into<String>, min_width: f32) -> Self {
        Self::with(head, min_width, 1.0, Align::Left)
    }

    /// A fixed-width column of right-aligned numbers.
    pub(super) fn number(head: impl Into<String>, width: f32) -> Self {
        Self::with(head, width, 0.0, Align::Right)
    }

    /// A fixed-width column of left-aligned words: a limit, a verdict.
    pub(super) fn words(head: impl Into<String>, width: f32) -> Self {
        Self::with(head, width, 0.0, Align::Left)
    }

    pub(super) fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// A head click leaves the rows' order alone.
    pub fn unsortable(mut self) -> Self {
        self.sortable = false;
        self
    }
}

/// The widths `columns` take across `available` points: fixed columns at their
/// width, the spare shared among the flexible ones by their share. When the
/// fixed widths and the flexible minimums exceed `available`, every column
/// keeps its least width and the table scrolls sideways.
pub(super) fn column_widths(columns: &[Column], available: f32) -> Vec<f32> {
    let least: f32 = columns.iter().map(|column| column.width).sum();
    let shares: f32 = columns.iter().map(|column| column.flex).sum();
    let spare = (available - least).max(0.0);
    let mut widths = columns
        .iter()
        .map(|column| {
            column.width
                + if shares > 0.0 {
                    spare * column.flex / shares
                } else {
                    0.0
                }
        })
        .collect::<Vec<_>>();
    if shares <= 0.0
        && let Some(last) = widths.last_mut()
    {
        *last += spare;
    }
    widths
}

/// A result's judgment of one value.
///
/// A failure is painted in the error tone and a value near its limit in the
/// warning tone. A pass is neutral: success is the absence of a problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Near,
    Fail,
}

impl Verdict {
    /// The word a pass/fail column states.
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Pass => "Pass",
            Self::Near => "Near",
            Self::Fail => "Fail",
        }
    }

    /// How bad it is, for a status sort: the higher, the worse.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Pass => 0,
            Self::Near => 1,
            Self::Fail => 3,
        }
    }
}

/// How a cell's words are inked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ink {
    Plain,
    /// The metadata register: a value the run does not hold.
    Faint,
    Verdict(Verdict),
    /// A problem's mark in front of neutral words.
    Problem(Problem),
}

/// One cell's words and how they are set.
#[derive(Debug, Clone)]
pub struct Cell {
    pub(super) text: String,
    mono: bool,
    ink: Ink,
    /// The cell answers a click by opening for entry.
    editable: bool,
    /// What the cell says when the pointer rests on it.
    hint: Option<String>,
    /// What a sort by its column orders it by, when not its words.
    sort: Option<SortKey>,
}

impl Cell {
    fn with(text: impl Into<String>, mono: bool, ink: Ink) -> Self {
        Self {
            text: text.into(),
            mono,
            ink,
            editable: false,
            hint: None,
            sort: None,
        }
    }

    /// Words in sans.
    pub(super) fn text(text: impl Into<String>) -> Self {
        Self::with(text, false, Ink::Plain)
    }

    /// A number in mono.
    pub(super) fn number(text: impl Into<String>) -> Self {
        Self::with(text, true, Ink::Plain)
    }

    /// A value the run does not hold: an em dash in the faint register.
    pub(super) fn absent() -> Self {
        Self::with("\u{2014}", true, Ink::Faint)
    }

    /// Nothing at all: a cell a row has no business filling.
    pub(super) fn empty() -> Self {
        Self::text("")
    }

    /// The same words, judged: coloured only when the verdict is not a pass.
    pub(super) fn judged(mut self, verdict: Verdict) -> Self {
        self.ink = Ink::Verdict(verdict);
        self
    }

    /// A pass/fail column's word for `verdict`.
    pub(super) fn verdict(verdict: Verdict) -> Self {
        Self::text(verdict.label()).judged(verdict)
    }

    /// Neutral words behind a problem's mark.
    pub(super) fn problem(text: impl Into<String>, problem: Problem) -> Self {
        Self::with(text, false, Ink::Problem(problem))
    }

    /// The same words, opened for entry by a click.
    pub(super) fn editable(mut self) -> Self {
        self.editable = true;
        self
    }

    /// The same words, saying `hint` when the pointer rests on them.
    pub(super) fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// The same words, sorted by `key`: the value a formatted number stands
    /// for, a status's severity.
    pub fn sort_by(mut self, key: SortKey) -> Self {
        self.sort = Some(key);
        self
    }

    /// What a sort orders this cell by: the key it was given; else a judged
    /// cell's severity; else its words, and nothing for a cell with no value.
    fn sort_key(&self) -> SortKey {
        if let Some(key) = &self.sort {
            return key.clone();
        }
        match self.ink {
            Ink::Verdict(verdict) => SortKey::Status(verdict.severity()),
            Ink::Faint => SortKey::Missing,
            _ if self.text.trim().is_empty() => SortKey::Missing,
            _ => SortKey::Text(self.text.clone()),
        }
    }

    /// Whether the cell's words contain `needle`, which is lower case.
    fn contains(&self, needle: &str) -> bool {
        self.text.to_lowercase().contains(needle)
    }
}

/// The one cell open for entry: where it is and what has been typed.
pub struct CellEditor<'a> {
    pub(super) row: usize,
    pub(super) column: usize,
    pub(super) text: &'a mut String,
    /// Set on the frame the cell opens: the field takes the keyboard, with
    /// its words selected so the first keystroke replaces them.
    pub(super) focus: bool,
}

/// How an open cell was closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditOutcome {
    /// Enter, Tab, or the field let go of the keyboard: apply what was typed.
    Commit,
    /// Escape: leave the value as it was.
    Cancel,
}

/// A cell, by its row in the page's own order and, when the reader pointed
/// at one, its column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellRef {
    pub row: usize,
    pub column: Option<usize>,
}

/// The reader asked for a row's context menu: the page draws it at
/// `position`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextRequest {
    pub cell: CellRef,
    pub position: egui::Pos2,
}

/// What a table reported back this frame.
#[derive(Debug, Clone, Copy, Default)]
pub struct TableResponse {
    /// The row the reader clicked.
    pub(super) clicked: Option<usize>,
    /// The editable cell the reader opened, by a click, Enter, F2 or Tab, as
    /// (row, column).
    pub(super) edit: Option<(usize, usize)>,
    /// How the open cell was closed this frame, if it was.
    pub(super) edited: Option<EditOutcome>,
    /// The row, and the cell, the reader double-clicked.
    pub double_clicked: Option<CellRef>,
    /// The reader asked for a row's context menu.
    pub context_menu: Option<ContextRequest>,
    /// The selection in the table's state changed this frame.
    pub selection_changed: bool,
    /// The table has the keyboard: a key the page gives a meaning to, such
    /// as Delete on the selected row, is the page's to read.
    pub focused: bool,
}

/// One table: its columns, its rows, and where it may grow.
pub struct DataTable<'a> {
    id: &'a str,
    columns: &'a [Column],
    rows: usize,
    selected: Option<usize>,
    clickable: bool,
    max_height: f32,
    editor: Option<CellEditor<'a>>,
    state: Option<&'a mut TableState>,
    keys: Option<&'a [RowKey]>,
    frozen: usize,
    bands: &'a [Vec<Span>],
    groups: &'a [Group],
    cell_selection: bool,
}

/// Where the body is seen from this frame, in screen points: the viewport's
/// left edge and width, and how far the scrolled columns are scrolled.
#[derive(Debug, Clone, Copy)]
struct View {
    left: f32,
    width: f32,
    offset: f32,
}

impl View {
    /// The frozen columns' strip and the scrolled columns' strip over `y`.
    fn regions(&self, geometry: &Columns, y: egui::Rangef) -> (Rect, Rect) {
        let edge = self.left + geometry.freeze_edge().min(self.width);
        (
            Rect::from_x_y_ranges(self.left..=edge, y),
            Rect::from_x_y_ranges(edge..=self.left + self.width, y),
        )
    }

    /// Column `column`'s cell over `y`, and the strip it shows in.
    fn cell(&self, geometry: &Columns, column: usize, y: egui::Rangef) -> (Rect, Rect) {
        let left = self.left + geometry.left_at(column, self.offset);
        let rect = Rect::from_x_y_ranges(left..=left + geometry.width(column), y);
        let (frozen, scrolled) = self.regions(geometry, y);
        (
            rect,
            if column < geometry.frozen() {
                frozen
            } else {
                scrolled
            },
        )
    }
}

/// What the table remembers about its own view between frames, which the page
/// has no reason to keep: where the body was scrolled to and how tall it was,
/// for Page Up and Down and for bringing the selection into view.
#[derive(Debug, Clone, Copy, Default)]
struct ViewMemory {
    offset_y: f32,
    height: f32,
    /// The sort changed: bring the selection into view on the next frame.
    reveal: bool,
}

impl ViewMemory {
    fn load(ctx: &egui::Context, id: egui::Id) -> Self {
        ctx.data(|data| data.get_temp::<Self>(id))
            .unwrap_or_default()
    }

    fn store(self, ctx: &egui::Context, id: egui::Id) {
        ctx.data_mut(|data| data.insert_temp(id, self));
    }
}

/// What the keyboard asked of a focused table this frame.
#[derive(Debug, Clone, Copy, Default)]
struct Keys {
    step: Option<Step>,
    /// Left or right by one cell.
    across: Option<isize>,
    open: bool,
    copy: bool,
    menu: bool,
}

impl Keys {
    fn read(ui: &Ui) -> Self {
        ui.input_mut(|input| {
            let mut take = |key| input.consume_key(Modifiers::NONE, key);
            let step = [
                (Key::ArrowUp, Step::Up),
                (Key::ArrowDown, Step::Down),
                (Key::PageUp, Step::PageUp),
                (Key::PageDown, Step::PageDown),
                (Key::Home, Step::Home),
                (Key::End, Step::End),
            ]
            .into_iter()
            .fold(None, |found, (key, step)| {
                let pressed = take(key);
                found.or(pressed.then_some(step))
            });
            let left = take(Key::ArrowLeft);
            let right = take(Key::ArrowRight);
            let across = match (left, right) {
                (true, false) => Some(-1),
                (false, true) => Some(1),
                _ => None,
            };
            let enter = take(Key::Enter);
            let f2 = take(Key::F2);
            let menu = input.consume_key(Modifiers::SHIFT, Key::F10);
            let copy_event = input
                .events
                .iter()
                .any(|event| matches!(event, egui::Event::Copy));
            input
                .events
                .retain(|event| !matches!(event, egui::Event::Copy));
            let copy_key = input.consume_key(Modifiers::COMMAND, Key::C);
            Self {
                step,
                across,
                open: enter || f2,
                copy: copy_event || copy_key,
                menu,
            }
        })
    }
}

/// The rows of a table with every row's cells read: a sort and a filter look
/// at all of them. Rows are read on first use otherwise, so a table that only
/// shows its rows reads only the rows in view.
struct Rows<'f> {
    read: Vec<Option<Vec<Cell>>>,
    row_at: &'f mut dyn FnMut(usize) -> Vec<Cell>,
}

impl<'f> Rows<'f> {
    fn new(rows: usize, row_at: &'f mut dyn FnMut(usize) -> Vec<Cell>) -> Self {
        Self {
            read: vec![None; rows],
            row_at,
        }
    }

    fn get(&mut self, row: usize) -> &[Cell] {
        let slot = &mut self.read[row];
        if slot.is_none() {
            *slot = Some((self.row_at)(row));
        }
        slot.as_deref().unwrap_or_default()
    }

    /// The cells a frame already read for `row`, without reading it.
    fn known(&self, row: usize) -> Option<&[Cell]> {
        self.read.get(row)?.as_deref()
    }
}

impl<'a> DataTable<'a> {
    pub(super) fn new(id: &'a str, columns: &'a [Column], rows: usize) -> Self {
        Self {
            id,
            columns,
            rows,
            selected: None,
            clickable: false,
            max_height: f32::INFINITY,
            editor: None,
            state: None,
            keys: None,
            frozen: 0,
            bands: &[],
            groups: &[],
            cell_selection: false,
        }
    }

    /// The cell open for entry, painted as a field in its place.
    pub(super) fn editing(mut self, editor: Option<CellEditor<'a>>) -> Self {
        self.editor = editor;
        self
    }

    /// The tallest the table may be, head included; its rows scroll inside.
    pub(super) fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    /// The page's memory of this table: its selection, sort, filter, folded
    /// groups and sideways scroll. Rows answer a click, and the table takes
    /// the keyboard.
    pub fn state(mut self, state: &'a mut TableState) -> Self {
        self.state = Some(state);
        self.clickable = true;
        self
    }

    /// Each row's key, in the page's row order, so the selection finds its
    /// row again when the rows change.
    pub fn keys(mut self, keys: &'a [RowKey]) -> Self {
        self.keys = Some(keys);
        self
    }

    /// The first `columns` stay put while the rest scroll sideways.
    pub fn freeze(mut self, columns: usize) -> Self {
        self.frozen = columns;
        self
    }

    /// Band rows over the head, top first, each a list of spans.
    pub fn bands(mut self, bands: &'a [Vec<Span>]) -> Self {
        self.bands = bands;
        self
    }

    /// Group heads between the rows, in row order.
    pub fn groups(mut self, groups: &'a [Group]) -> Self {
        self.groups = groups;
        self
    }

    /// A click, and Left and Right, select a cell within the selected row.
    pub fn cell_selection(mut self) -> Self {
        self.cell_selection = true;
        self
    }

    pub(super) fn show(
        mut self,
        ui: &mut Ui,
        mut row_at: impl FnMut(usize) -> Vec<Cell>,
    ) -> TableResponse {
        let t = Tokens::get(ui.ctx());
        let (head_h, row_h) = line_heights(&t);
        let mut editor = self.editor.take();
        let mut state = self.state.take();
        let mut response = TableResponse::default();
        if let (Some(state), Some(keys)) = (state.as_deref_mut(), self.keys) {
            state.follow_keys(keys);
        }
        let mut rows = Rows::new(self.rows, &mut row_at);

        // Which rows show, in what order.
        let filter = state
            .as_deref()
            .map(|state| state.filter.trim().to_lowercase())
            .filter(|filter| !filter.is_empty());
        let sort = state
            .as_deref()
            .and_then(|state| state.sort)
            .filter(|sort| {
                self.columns
                    .get(sort.column)
                    .is_some_and(|column| column.sortable)
            });
        if filter.is_some() || sort.is_some() {
            for row in 0..self.rows {
                let _ = rows.get(row);
            }
        }
        let sort_keys = sort.map(|sort| {
            let keys = (0..self.rows)
                .map(|row| {
                    rows.known(row)
                        .and_then(|cells| cells.get(sort.column))
                        .map_or(SortKey::Missing, Cell::sort_key)
                })
                .collect::<Vec<_>>();
            (keys, sort.direction)
        });
        let order = sort_keys.as_ref().map(|(keys, direction)| {
            move |left: usize, right: usize| keys[left].order(&keys[right], *direction)
        });
        let no_folds = BTreeSet::new();
        let lines = arrange(
            self.rows,
            self.groups,
            state.as_deref().map_or(&no_folds, |state| &state.folded),
            |row| {
                filter.as_deref().is_none_or(|needle| {
                    rows.known(row)
                        .is_some_and(|cells| cells.iter().any(|cell| cell.contains(needle)))
                })
            },
            order
                .as_ref()
                .map(|order| order as &dyn Fn(usize, usize) -> Ordering),
        );
        let shown = lines
            .iter()
            .filter_map(|line| match line {
                Line::Row(row) => Some(*row),
                Line::Group { .. } => None,
            })
            .collect::<Vec<_>>();

        // Where the columns go.
        let visible = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        let available = visible.width().max(1.0);
        let bands_h = TABLE_HEAD_H * self.bands.len() as f32;
        let body_max = (self.max_height - bands_h - head_h).max(row_h);
        let lines_h = row_h * lines.len() as f32;
        let bar = if lines_h > body_max + 0.5 {
            ui.spacing().scroll.allocated_width()
        } else {
            0.0
        };
        let viewport_w = (available - bar).max(1.0);
        let least: f32 = self.columns.iter().map(|column| column.width).sum();
        let scrolls_x = least > viewport_w + 0.5;
        let geometry = Columns::new(
            column_widths(self.columns, viewport_w.max(least)),
            self.frozen,
            viewport_w,
        );

        // The keyboard, before anything is painted, so a move shows at once.
        let table_id = ui.id().with((self.id, "table"));
        let mut memory = ViewMemory::load(ui.ctx(), table_id);
        let mut scroll_x = state.as_deref().map(|state| state.scroll_x);
        let mut reveal = std::mem::take(&mut memory.reveal)
            .then(|| state.as_deref().and_then(TableState::selected_row))
            .flatten();
        let focused = state.is_some() && ui.memory(|memory| memory.has_focus(table_id));
        response.focused = focused;
        let mut keyed_menu = false;
        if let Some(state) = state.as_deref_mut()
            && focused
        {
            ui.memory_mut(|memory| {
                memory.set_focus_lock_filter(
                    table_id,
                    egui::EventFilter {
                        horizontal_arrows: true,
                        vertical_arrows: true,
                        tab: false,
                        escape: false,
                    },
                );
            });
            let keys = Keys::read(ui);
            if keys.step.is_some() || keys.across.is_some() {
                // The arrows moved the selection, not the keyboard focus:
                // the frame the table took focus it had no filter yet.
                ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
            }
            if let Some(step_by) = keys.step {
                let page = ((memory.height.max(row_h) / row_h).floor() as usize).max(1);
                if let Some(row) = step(&shown, state.selected_row(), step_by, page) {
                    let column = state.selected_column();
                    response.selection_changed |= self.select(state, row, column);
                    reveal = Some(row);
                }
            }
            if let (Some(by), true) = (keys.across, self.cell_selection)
                && let Some(selection) = state.selection.as_mut()
                && !self.columns.is_empty()
            {
                let last = self.columns.len() - 1;
                let column = selection
                    .column
                    .map_or(0, |column| column.saturating_add_signed(by).min(last));
                response.selection_changed |= selection.column != Some(column);
                selection.column = Some(column);
                scroll_x = Some(geometry.reveal(column, state.scroll_x, viewport_w));
            }
            if keys.open
                && let Some(selection) = &state.selection
                && shown.contains(&selection.row)
            {
                let cells = rows.get(selection.row);
                let editable =
                    |column: &usize| cells.get(*column).is_some_and(|cell| cell.editable);
                let column = selection
                    .column
                    .filter(editable)
                    .or_else(|| (0..cells.len()).find(editable));
                if let Some(column) = column {
                    response.edit = Some((selection.row, column));
                    scroll_x = Some(geometry.reveal(column, state.scroll_x, viewport_w));
                    reveal = Some(selection.row);
                }
            }
            if keys.copy
                && let Some(row) = state.selected_row()
            {
                let text = self.copied_text(rows.get(row));
                ui.ctx().copy_text(text);
            }
            keyed_menu = keys.menu && state.selection.is_some();
        }
        let reveal_y = reveal
            .and_then(|row| lines.iter().position(|line| *line == Line::Row(row)))
            .and_then(|line| {
                let top = line as f32 * row_h;
                let height = if memory.height > 0.0 {
                    memory.height
                } else {
                    body_max
                };
                if top < memory.offset_y {
                    Some(top)
                } else if top + row_h > memory.offset_y + height {
                    Some(top + row_h - height)
                } else {
                    None
                }
            });

        // The head, painted after the body has settled the scroll. The body
        // sits flush under it; the page's own spacing follows the body.
        let spacing = ui.spacing().item_spacing;
        ui.spacing_mut().item_spacing.y = 0.0;
        let (head_block, head_response) =
            ui.allocate_exact_size(vec2(available, bands_h + head_h), Sense::hover());
        ui.spacing_mut().item_spacing = spacing;
        let viewport_left = head_block.left();

        // The body.
        let mut scroll = egui::ScrollArea::new([scrolls_x, true])
            .id_salt((self.id, "rows"))
            .max_height(body_max)
            .auto_shrink([false, true]);
        if !scrolls_x {
            scroll = scroll.horizontal_scroll_offset(0.0);
        } else if let Some(offset) = scroll_x {
            scroll = scroll.horizontal_scroll_offset(offset);
        }
        if let Some(offset) = reveal_y {
            scroll = scroll.vertical_scroll_offset(offset);
        }
        let selected = match state.as_deref() {
            Some(state) => state.selection.clone(),
            None => self.selected.map(|row| Selection {
                row,
                key: None,
                column: None,
            }),
        };
        let mut folds = Vec::new();
        let mut take_focus = false;
        let output = scroll.show_viewport(ui, |ui, viewport| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            ui.set_min_width(geometry.content_width());
            ui.set_height(lines_h);
            let total = lines.len();
            let mut first = (viewport.min.y / row_h).floor() as usize;
            let mut last = (viewport.max.y / row_h).ceil() as usize + 1;
            if last > total {
                let span = last.saturating_sub(first);
                last = total;
                first = total.saturating_sub(span);
            }
            let view = View {
                left: viewport_left,
                width: viewport_w,
                offset: viewport.min.x,
            };
            let top = ui.max_rect().top();
            let band = Rect::from_x_y_ranges(
                ui.max_rect().x_range(),
                top + first as f32 * row_h..=top + last as f32 * row_h,
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(band), |ui| {
                ui.skip_ahead_auto_ids(first);
                for line in &lines[first..last] {
                    match *line {
                        Line::Group { group, rows: count } => {
                            let label = &self.groups[group].label;
                            let head = body::GroupHead {
                                label,
                                rows: count,
                                folded: state
                                    .as_deref()
                                    .is_some_and(|state| state.is_folded(label)),
                                foldable: state.is_some(),
                            };
                            if self.group_head(ui, view, &geometry, row_h, head) {
                                folds.push(label.clone());
                            }
                        }
                        Line::Row(index) => {
                            let cells = rows.get(index);
                            let open = editor.as_mut().filter(|editor| editor.row == index);
                            let marked =
                                selected.as_ref().filter(|selection| selection.row == index);
                            let row = self.row(
                                ui,
                                body::RowPlace {
                                    index,
                                    view,
                                    geometry: &geometry,
                                    height: row_h,
                                    selected: marked.is_some(),
                                    selected_column: marked.and_then(|selection| selection.column),
                                    stateful: state.is_some(),
                                },
                                cells,
                                open,
                            );
                            take_focus |=
                                self.answer(&mut response, state.as_deref_mut(), index, row);
                            if keyed_menu && selected.as_ref().is_some_and(|s| s.row == index) {
                                response.context_menu = Some(ContextRequest {
                                    cell: CellRef {
                                        row: index,
                                        column: selected.as_ref().and_then(|s| s.column),
                                    },
                                    position: pos2(
                                        viewport_left + TABLE_EDGE_INSET,
                                        row.rect.bottom(),
                                    ),
                                });
                            }
                        }
                    }
                }
            });
            view
        });
        let view = View {
            offset: if scrolls_x {
                output.state.offset.x
            } else {
                0.0
            },
            ..output.inner
        };
        memory.offset_y = output.state.offset.y;
        memory.height = output.inner_rect.height();
        if let Some(state) = state.as_deref_mut() {
            state.scroll_x = view.offset;
            for label in &folds {
                state.toggle_fold(label);
            }
        }

        if let Some(column) = self.head(
            ui,
            head::HeadPlace {
                block: head_block,
                response: &head_response,
                height: head_h,
                view,
                geometry: &geometry,
                sort,
                sortable: state.is_some(),
            },
        ) && let Some(state) = state.as_deref_mut()
        {
            state.cycle_sort(column);
            memory.reveal = true;
        }
        if geometry.frozen() > 0 {
            let edge = viewport_left + geometry.freeze_edge();
            // The edge stops under the last row: below the rows there are no
            // columns for it to divide.
            let rows_bottom = (output.inner_rect.top() + output.content_size.y
                - output.state.offset.y)
                .min(output.inner_rect.bottom());
            ui.painter().vline(
                edge,
                head_block.top()..=rows_bottom,
                egui::Stroke::new(1.0, t.color.border_strong),
            );
        }
        if state.is_some() {
            self.focus_target(
                ui,
                table_id,
                head_block.union(output.inner_rect),
                selected.as_ref(),
            );
            // Taken after the focus target has been seen, so a press that
            // lands on a row is not read as a press elsewhere.
            if take_focus {
                ui.memory_mut(|memory| memory.request_focus(table_id));
            }
        }
        memory.store(ui.ctx(), table_id);
        response
    }

    /// Select `row`, keeping `column` when cells are selected. Returns
    /// whether the selection changed.
    fn select(&self, state: &mut TableState, row: usize, column: Option<usize>) -> bool {
        let selection = Selection {
            row,
            key: self.keys.and_then(|keys| keys.get(row).cloned()),
            column: if self.cell_selection { column } else { None },
        };
        let changed = state.selection.as_ref() != Some(&selection);
        state.selection = Some(selection);
        changed
    }

    /// Fold one row's report into the table's, and move the selection to
    /// where the reader pointed. Returns whether the table takes the
    /// keyboard: a row was pressed, or an open cell closed.
    fn answer(
        &self,
        response: &mut TableResponse,
        state: Option<&mut TableState>,
        index: usize,
        row: body::RowResponse,
    ) -> bool {
        if row.clicked || row.edit.is_some() {
            response.clicked = Some(index);
        }
        if let Some(column) = row.edit {
            response.edit = Some((index, column));
        }
        response.edited = response.edited.or(row.edited);
        if row.double_clicked {
            response.double_clicked = Some(CellRef {
                row: index,
                column: row.column,
            });
        }
        if let Some(position) = row.context {
            response.context_menu = Some(ContextRequest {
                cell: CellRef {
                    row: index,
                    column: row.column,
                },
                position,
            });
        }
        let Some(state) = state else {
            return false;
        };
        let pressed = row.clicked || row.edit.is_some() || row.context.is_some();
        if pressed {
            let column = row.edit.or(row.column);
            response.selection_changed |= self.select(state, index, column);
        }
        // A closed cell hands the keyboard back to the table.
        pressed || (row.edited.is_some() && row.edit.is_none())
    }

    /// The keyboard's place in a table with state: one stop for the whole
    /// table, where Tab lands and the arrow keys act, framed by the selected
    /// row while focus is shown.
    fn focus_target(&self, ui: &Ui, table_id: egui::Id, rect: Rect, selected: Option<&Selection>) {
        let focus = ui.interact(rect, table_id, Sense::focusable_noninteractive());
        focus.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                format!(
                    "Table, {} rows{}",
                    self.rows,
                    selected.map_or_else(String::new, |selection| format!(
                        ", row {} selected",
                        selection.row + 1
                    ))
                ),
            )
        });
        ui.ctx().accesskit_node_builder(focus.id, |node| {
            node.set_role(egui::accesskit::Role::Grid);
        });
        if theme::focus_is_visible(ui.ctx(), &focus) {
            theme::paint_focus_ring(ui, &focus, rect);
        }
    }

    /// The selected row as tab-separated text under the heads of the columns
    /// that have one.
    fn copied_text(&self, cells: &[Cell]) -> String {
        let named = self
            .columns
            .iter()
            .enumerate()
            .filter(|(_, column)| !column.head.trim().is_empty())
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let heads = named
            .iter()
            .map(|index| self.columns[*index].head.as_str())
            .collect::<Vec<_>>()
            .join("\t");
        let values = named
            .iter()
            .map(|index| {
                cells
                    .get(*index)
                    .filter(|cell| cell.ink != Ink::Faint)
                    .map_or("", |cell| cell.text.as_str())
            })
            .collect::<Vec<_>>()
            .join("\t");
        format!("{heads}\n{values}")
    }

    /// The horizontal span a column's words may occupy: the cell less its
    /// padding, with the table's outer columns held to the panel inset.
    fn text_span(&self, column: usize, cell: Rect) -> egui::Rangef {
        let left = if column == 0 {
            TABLE_EDGE_INSET
        } else {
            CELL_PAD
        };
        let right = if column + 1 == self.columns.len() {
            TABLE_EDGE_INSET
        } else {
            CELL_PAD
        };
        egui::Rangef::new(
            cell.left() + left,
            (cell.right() - right).max(cell.left() + left),
        )
    }
}

/// How one cell's words are set.
struct Words<'a> {
    text: &'a str,
    font: egui::FontId,
    align: Align,
    color: egui::Color32,
    problem: Option<Problem>,
}

/// Paint one cell's words on its line, elided to `span` and clipped to
/// `clip`, and return where the words landed.
fn paint_words(
    ui: &Ui,
    cell: Rect,
    clip: Rect,
    span: egui::Rangef,
    words: Words<'_>,
) -> Option<Rect> {
    let Words {
        text,
        font,
        align,
        color,
        problem,
    } = words;
    if text.is_empty() {
        return None;
    }
    let mark_room = if problem.is_some() { MARK_ADVANCE } else { 0.0 };
    let shown = elide_text(ui, text, &font, (span.span() - mark_room).max(0.0));
    let galley = ui.painter().layout_no_wrap(shown, font, color);
    let text_width = galley.size().x;
    let left = match align {
        Align::Left => span.min + mark_room,
        Align::Right => span.max - text_width,
    };
    let top = cell.center().y - galley.size().y * 0.5;
    let painter = ui
        .painter()
        .with_clip_rect(cell.intersect(clip).intersect(ui.clip_rect()));
    if let Some(problem) = problem {
        paint_problem_mark(
            &painter,
            Rect::from_center_size(
                pos2(left - mark_room + MARK_SIDE * 0.5, cell.center().y),
                egui::Vec2::splat(MARK_SIDE),
            ),
            problem,
        );
    }
    let landed = Rect::from_min_size(pos2(left, top), galley.size());
    painter.galley(landed.min, galley, color);
    Some(landed)
}
