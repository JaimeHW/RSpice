//! The one table every page reads its rows in.
//!
//! It is the grid of an ADE results view, not a card: dense heads and rows,
//! heads as written (sentence case, no tracking), names in sans, numbers in
//! mono and right-aligned so their digits line up, hairlines between rows and
//! columns. The only colour in a row is a toned cell's — an error in the error
//! tone, a caution in the warning tone — and the accent wash of the rows the
//! reader selected. A problem the reader must notice in a cell is a mark in
//! front of neutral words, never coloured words.
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
//! group heads, a row filter, type-ahead, Ctrl+C, and a column's right edge
//! that the reader drags to size it; one that asks for
//! [`DataTable::multi_select`] lets the reader select several rows the way a
//! file list does. Enter, F2, Delete and the menu key are reported for the
//! page to act on. Under a coarse pointer such a table puts a mark before
//! each row that a tap toggles, since a finger has no Ctrl to hold, and a
//! press held still on a row asks for its menu. Words a cell cuts short show
//! in full on hover and under that held press. A page that hands it no state
//! gets the rows it asked for, in its own order, and reports clicks.
//!
//! The same table draws a plain list or a tree, for a rail or a library
//! column: [`DataTable::list`] and [`DataTable::tree`] draw no heads, give
//! each row the list row height under the live metrics, and announce the rows
//! as a list's options or a tree's items. A cell may carry a leading icon, a
//! twist and an indent; Left and Right close, open and walk a tree.

mod body;
mod geometry;
mod head;
#[cfg(test)]
mod list_tests;
mod state;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tree_tests;

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::BTreeSet;

use egui::{Key, Modifiers, Rect, Sense, Ui, pos2, vec2};

use crate::icons::Icon;
use crate::panels::elide_text;
use crate::theme;
use crate::tokens::Tokens;
use crate::widgets::{Problem, paint_problem_mark};

pub use geometry::Span;
pub use state::{Group, RowKey, Selection, Sort, SortDirection, SortKey, TableState};

use geometry::Columns;
use state::{Line, Step, TYPE_AHEAD_PAUSE, TreeStep, arrange, step, tree_step, type_ahead};

/// Height of a table's head and of each band row over it at the compact
/// density under a fine pointer, for a page that lines its own bands up with
/// a table's; [`LineHeights`] is what a table takes under the live metrics.
pub const TABLE_HEAD_H: f32 = 24.0;
/// Height of one table row at the compact density under a fine pointer.
pub const TABLE_ROW_H: f32 = 24.0;
/// Where the first column's words start and the last column's end: the panel
/// inset, so a table's text lines up with the toolbar title over it.
pub const TABLE_EDGE_INSET: f32 = crate::panels::PANEL_INSET;
/// Space between a cell's words and the rule on either side of it, which a
/// page sizing a column to its words adds to them.
pub const CELL_PAD: f32 = 8.0;
/// The narrowest the reader can drag a column: room for a letter and an
/// ellipsis between its pads.
const MIN_COLUMN_W: f32 = 2.0 * CELL_PAD + 16.0;
/// The widest a table reads the way a phone does: the width the
/// application's own layout turns to bottom navigation at. A table no wider
/// than this freezes its first column only, and shows its pinned column
/// beside it.
const NARROW_W: f32 = 560.0;
/// A problem mark in a cell, and the advance from its left edge to the words:
/// the kit's own, so a marked cell matches every other marked line.
use crate::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};

/// The heights of a table's lines under the live metrics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineHeights {
    /// The row of column heads.
    pub head: f32,
    /// One row.
    pub row: f32,
    /// One band row over the head, which names a run of columns and is never
    /// touched.
    pub band: f32,
}

impl LineHeights {
    /// A grid's: the density's grid row under a fine pointer; under a coarse
    /// one the touch target the preferences ask for, for every line a finger
    /// presses.
    pub fn grid(t: &Tokens) -> Self {
        let band = t.metrics.grid_row_h;
        if t.metrics.is_touch() {
            let target = t.metrics.touch_target();
            Self {
                head: target,
                row: target,
                band,
            }
        } else {
            Self {
                head: band,
                row: band,
                band,
            }
        }
    }

    /// A list's: no head, and each row as tall as a list row under the live
    /// metrics, which under a coarse pointer is the touch target.
    pub fn list(t: &Tokens) -> Self {
        Self {
            head: 0.0,
            row: if t.metrics.is_touch() {
                t.metrics.touch_target()
            } else {
                t.metrics.row_h
            },
            band: t.metrics.grid_row_h,
        }
    }
}

/// Which edge of its cell a column's words sit on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
}

/// One column: its head, how wide it is, and where its words sit.
#[derive(Debug, Clone)]
pub struct Column {
    head: String,
    /// What the head stands for, shown when the pointer rests on it.
    tooltip: Option<String>,
    /// The column's width, or its least width when it takes a share of what
    /// the fixed columns leave.
    width: f32,
    /// Share of the spare width; zero for a fixed column.
    flex: f32,
    align: Align,
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
    pub fn name(head: impl Into<String>, min_width: f32) -> Self {
        Self::with(head, min_width, 1.0, Align::Left)
    }

    /// A fixed-width column of right-aligned numbers, and of limits, which
    /// are numbers the values beside them are read against.
    pub fn number(head: impl Into<String>, width: f32) -> Self {
        Self::with(head, width, 0.0, Align::Right)
    }

    /// A fixed-width column of left-aligned words: a status, a rule.
    pub fn words(head: impl Into<String>, width: f32) -> Self {
        Self::with(head, width, 0.0, Align::Left)
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// The same column with its words on `align`'s edge: a flexible column
    /// of numbers, say, that shares the spare width with its siblings.
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// The words over the column.
    pub fn head(&self) -> &str {
        &self.head
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
fn column_widths(columns: &[Column], available: f32) -> Vec<f32> {
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

/// The tone a judged cell's words are inked in, which is also what a sort by
/// its column orders it by.
///
/// An error is painted in the error tone and a caution in the warning tone.
/// A neutral judgement is plain: success is the absence of a problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Caution,
    Error,
}

impl Tone {
    /// How bad it is, for a status sort: the higher, the worse.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Neutral => 0,
            Self::Caution => 1,
            Self::Error => 2,
        }
    }
}

/// How a cell's words are inked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ink {
    Plain,
    /// The metadata register: a value the run does not hold.
    Faint,
    Toned(Tone),
    /// A problem's mark in front of neutral words.
    Problem(Problem),
}

/// One cell's words and how they are set.
#[derive(Debug, Clone)]
pub struct Cell {
    pub text: String,
    mono: bool,
    ink: Ink,
    /// The cell answers a click by opening for entry.
    editable: bool,
    /// What the cell says when the pointer rests on it.
    hint: Option<String>,
    /// What a sort by its column orders it by, when not its words.
    sort: Option<SortKey>,
    /// A mark before the words, after any twist.
    icon: Option<Icon>,
    /// How many levels of a tree the words step in.
    indent: u8,
    twist: Twist,
    /// What the row names has unsaved changes: the document tab's neutral
    /// dot follows the words.
    unsaved: bool,
}

/// Whether a cell's row opens and closes in a tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Twist {
    /// No twist, and no room for one.
    None,
    /// No twist, but its room: a row without children among rows with them,
    /// whose words line up with theirs.
    Leaf,
    /// A twist, pointing down when the row is open.
    Branch(bool),
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
            icon: None,
            indent: 0,
            twist: Twist::None,
            unsaved: false,
        }
    }

    /// Words in sans.
    pub fn text(text: impl Into<String>) -> Self {
        Self::with(text, false, Ink::Plain)
    }

    /// A number in mono.
    pub fn number(text: impl Into<String>) -> Self {
        Self::with(text, true, Ink::Plain)
    }

    /// A value the run does not hold: an em dash in the faint register.
    pub fn absent() -> Self {
        Self::with("\u{2014}", true, Ink::Faint)
    }

    /// Nothing at all: a cell a row has no business filling.
    pub fn empty() -> Self {
        Self::text("")
    }

    /// The same words, judged: coloured only when the tone is not neutral,
    /// and sorted by the tone's severity.
    pub fn toned(mut self, tone: Tone) -> Self {
        self.ink = Ink::Toned(tone);
        self
    }

    /// Neutral words behind a problem's mark.
    pub fn problem(text: impl Into<String>, problem: Problem) -> Self {
        Self::with(text, false, Ink::Problem(problem))
    }

    /// The same words, opened for entry by a click.
    pub fn editable(mut self) -> Self {
        self.editable = true;
        self
    }

    /// The same words, saying `hint` when the pointer rests on them.
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// The same words, sorted by `key`: the value a formatted number stands
    /// for, a status's severity.
    pub fn sort_by(mut self, key: SortKey) -> Self {
        self.sort = Some(key);
        self
    }

    /// The same words behind `icon`.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// The same words stepped in `level` levels, as a tree's child is.
    pub fn indent(mut self, level: u8) -> Self {
        self.indent = level;
        self
    }

    /// The same words behind a twist: the row has children, shown while
    /// `open`. A click on the twist, and Left and Right, ask to close or
    /// open it.
    pub fn twist(mut self, open: bool) -> Self {
        self.twist = Twist::Branch(open);
        self
    }

    /// The same words behind a twist's room but no twist: a row without
    /// children whose words line up with its siblings that have them.
    pub fn leaf(mut self) -> Self {
        self.twist = Twist::Leaf;
        self
    }

    /// The same words followed by the document tab's neutral dot: what the
    /// row names has unsaved changes. The dot says nothing is wrong, only
    /// that the saved version differs.
    pub fn unsaved(mut self) -> Self {
        self.unsaved = true;
        self
    }

    /// The row's level in a tree and, when it has children, whether they
    /// show.
    fn branch(&self) -> (u8, Option<bool>) {
        (
            self.indent,
            match self.twist {
                Twist::Branch(open) => Some(open),
                Twist::None | Twist::Leaf => None,
            },
        )
    }

    /// What a sort orders this cell by: the key it was given; else a judged
    /// cell's severity; else its words, and nothing for a cell with no value.
    fn sort_key(&self) -> SortKey {
        if let Some(key) = &self.sort {
            return key.clone();
        }
        match self.ink {
            Ink::Toned(tone) => SortKey::Status(tone.severity()),
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
    pub row: usize,
    pub column: usize,
    pub text: &'a mut String,
    /// Set on the frame the cell opens: the field takes the keyboard, with
    /// its words selected so the first keystroke replaces them.
    pub focus: bool,
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
    pub clicked: Option<usize>,
    /// The editable cell the reader opened, by a click, Enter, F2 or Tab, as
    /// (row, column).
    pub edit: Option<(usize, usize)>,
    /// How the open cell was closed this frame, if it was.
    pub edited: Option<EditOutcome>,
    /// The row, and the cell, the reader double-clicked.
    pub double_clicked: Option<CellRef>,
    /// The reader asked for a row's context menu.
    pub context_menu: Option<ContextRequest>,
    /// The selection in the table's state changed this frame.
    pub selection_changed: bool,
    /// The table has the keyboard: a key the page gives a meaning to beyond
    /// the ones reported here is the page's to read.
    pub focused: bool,
    /// The row whose primary action the reader asked for: Enter on a row
    /// with no editable cell, or a double-click.
    pub activate: Option<usize>,
    /// F2 on a row with no editable cell: the page renames what the lead
    /// row names.
    pub rename: Option<usize>,
    /// Delete while the table has the keyboard and a row is selected: the
    /// page deletes what the selected rows name.
    pub delete: bool,
    /// The row a twist click, Left or Right asked to open (true) or close
    /// (false); the page shows or hides its children.
    pub expand: Option<(usize, bool)>,
    /// Left (-1) or Right (1) while the table has the keyboard, when the key
    /// neither opens, closes nor moves within it: a page whose lists stand
    /// side by side, as a column browser's do, moves to the neighbouring one.
    pub across: Option<isize>,
    /// A row dragged out of a table was let go over this row: this row,
    /// and the one dragged.
    pub dropped: Option<(usize, RowDrag)>,
}

/// What a table is drawn as, and announced as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// Rows of cells under heads.
    Grid,
    /// A plain list.
    List,
    /// A list whose rows nest.
    Tree,
}

/// One table: its columns, its rows, and where it may grow.
pub struct DataTable<'a> {
    id: &'a str,
    /// The columns in the order they show, which is the page's order unless a
    /// narrow table moved its pinned column forward.
    columns: Cow<'a, [Column]>,
    /// The column a narrow table shows directly after the first, in the
    /// page's order.
    pinned: Option<usize>,
    /// The page's column under each shown column, when they differ.
    order: Option<Vec<usize>>,
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
    multi: bool,
    /// The room before the first column's words that the selection marks
    /// take under a coarse pointer; none elsewhere.
    lead_gutter: f32,
    kind: Kind,
    /// The table takes the keyboard this frame.
    focus: bool,
    /// A row can be dragged out under a fine pointer.
    drags: bool,
    /// A row takes a row dragged onto it.
    drops: bool,
    /// Bring the selected row into view this frame.
    reveal: bool,
}

/// A row dragged out of a table: the table, by the name it was built with,
/// and the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowDrag {
    table: egui::Id,
    pub row: usize,
}

impl RowDrag {
    /// Whether the row came from the table built as `DataTable::new(table, ..)`.
    pub fn is_from(&self, table: &str) -> bool {
        self.table == egui::Id::new(table)
    }
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
/// for Page Up and Down and for bringing the selection into view, and the
/// letters typed so far for type-ahead.
#[derive(Debug, Clone, Default)]
struct ViewMemory {
    offset_y: f32,
    height: f32,
    /// The sort changed: bring the selection into view on the next frame.
    reveal: bool,
    /// The letters typed since the last pause, in lower case.
    typed: String,
    /// When the last of them was typed, in the input's seconds.
    typed_at: f64,
    /// When the held press that last asked for a menu began, so one press
    /// asks once.
    held_press: Option<f64>,
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
#[derive(Debug, Clone, Default)]
struct Keys {
    step: Option<Step>,
    /// Shift was held with the step: extend the selection to where it lands.
    extend: bool,
    /// Left or right by one cell.
    across: Option<isize>,
    enter: bool,
    f2: bool,
    delete: bool,
    /// Ctrl+A in a table that selects several rows.
    all: bool,
    copy: bool,
    menu: bool,
    /// The letters typed this frame, for type-ahead.
    typed: String,
}

impl Keys {
    /// Read and take the keys a focused table answers; Shift with a step and
    /// Ctrl+A only when it selects several rows.
    fn read(ui: &Ui, multi: bool) -> Self {
        ui.input_mut(|input| {
            let mut step = None;
            let mut extend = false;
            for (key, to) in [
                (Key::ArrowUp, Step::Up),
                (Key::ArrowDown, Step::Down),
                (Key::PageUp, Step::PageUp),
                (Key::PageDown, Step::PageDown),
                (Key::Home, Step::Home),
                (Key::End, Step::End),
            ] {
                // Shift first: a key taken without modifiers is taken with
                // Shift held too.
                if multi && input.consume_key(Modifiers::SHIFT, key) {
                    step = step.or(Some(to));
                    extend = true;
                }
                if input.consume_key(Modifiers::NONE, key) {
                    step = step.or(Some(to));
                }
            }
            let mut take = |key| input.consume_key(Modifiers::NONE, key);
            let left = take(Key::ArrowLeft);
            let right = take(Key::ArrowRight);
            let across = match (left, right) {
                (true, false) => Some(-1),
                (false, true) => Some(1),
                _ => None,
            };
            let enter = take(Key::Enter);
            let f2 = take(Key::F2);
            let delete = take(Key::Delete);
            let all = multi && input.consume_key(Modifiers::COMMAND, Key::A);
            let menu = input.consume_key(Modifiers::SHIFT, Key::F10);
            let typed = input
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Text(text) => Some(text.as_str()),
                    _ => None,
                })
                .collect::<String>();
            input
                .events
                .retain(|event| !matches!(event, egui::Event::Text(_)));
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
                extend,
                across,
                enter,
                f2,
                delete,
                all,
                copy: copy_event || copy_key,
                menu,
                typed,
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
    pub fn new(id: &'a str, columns: &'a [Column], rows: usize) -> Self {
        Self {
            id,
            columns: Cow::Borrowed(columns),
            pinned: None,
            order: None,
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
            multi: false,
            lead_gutter: 0.0,
            kind: Kind::Grid,
            focus: false,
            drags: false,
            drops: false,
            reveal: false,
        }
    }

    /// The cell open for entry, painted as a field in its place.
    pub fn editing(mut self, editor: Option<CellEditor<'a>>) -> Self {
        self.editor = editor;
        self
    }

    /// The tallest the table may be, head included; its rows scroll inside.
    pub fn max_height(mut self, max_height: f32) -> Self {
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

    /// On a narrow table, column `column` shows directly after the first
    /// column and stays put with it while the rest scroll, so the reader
    /// reads what matters most about a row without scrolling sideways. A
    /// table with bands keeps its order: its spans name runs of columns.
    pub fn pin_when_narrow(mut self, column: usize) -> Self {
        self.pinned = Some(column);
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

    /// The reader may select several rows, as in a file list: Ctrl+click
    /// toggles a row, Shift+click and Shift with the arrow keys extend from
    /// the anchor, Ctrl+A selects every row shown. A context press on a
    /// selected row keeps the selection, so its menu acts on all of them.
    pub fn multi_select(mut self) -> Self {
        self.multi = true;
        self
    }

    /// Draw as a plain list: no heads and no bands, each row as tall as a
    /// list row under the live metrics, announced as a list's options.
    pub fn list(mut self) -> Self {
        self.kind = Kind::List;
        self
    }

    /// Draw as a tree: a list whose rows announce as a tree's items, with
    /// their level and, for a row with a twist, whether it is open.
    pub fn tree(mut self) -> Self {
        self.kind = Kind::Tree;
        self
    }

    /// Take the keyboard this frame, as a column browser's list does when
    /// the reader moves into it from its neighbour. Only a table with state
    /// holds the keyboard.
    pub fn focus(mut self, take: bool) -> Self {
        self.focus = take;
        self
    }

    /// A row pressed and moved under a fine pointer is dragged out, a chip
    /// with its name following the pointer, for another table that takes
    /// drops to receive. A finger's press keeps its long-press menu.
    pub fn drag_rows(mut self) -> Self {
        self.drags = true;
        self
    }

    /// A row takes a row dragged onto it: it is outlined while one is held
    /// over it, and the release is reported as [`TableResponse::dropped`].
    pub fn drops(mut self) -> Self {
        self.drops = true;
        self
    }

    /// Bring the selected row into view this frame, as a keyboard move does:
    /// for a page that chose the row itself, or that changed which rows the
    /// table shows or how tall it is.
    pub fn reveal_selection(mut self, reveal: bool) -> Self {
        self.reveal = reveal;
        self
    }

    pub fn show(
        mut self,
        ui: &mut Ui,
        mut row_at: impl FnMut(usize) -> Vec<Cell>,
    ) -> TableResponse {
        let t = Tokens::get(ui.ctx());
        let LineHeights {
            head: head_h,
            row: row_h,
            band: band_h,
        } = if self.kind == Kind::Grid {
            LineHeights::grid(&t)
        } else {
            self.bands = &[];
            LineHeights::list(&t)
        };
        let mut editor = self.editor.take();
        let mut state = self.state.take();
        let mut response = TableResponse::default();
        if let (Some(state), Some(keys)) = (state.as_deref_mut(), self.keys) {
            state.follow_keys(keys);
        }

        // A narrow table moves its pinned column beside the first. Everything
        // below works in the shown order; the page's memory and the response
        // speak the page's, so they are turned on the way in and back on the
        // way out.
        let visible = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        let available = visible.width().max(1.0);
        let narrow = available <= NARROW_W;
        self.order = self
            .pinned
            .filter(|_| narrow && self.bands.is_empty())
            .and_then(|pinned| pinned_order(self.columns.len(), pinned));
        if let Some(order) = self.order.clone() {
            self.columns = Cow::Owned(
                order
                    .iter()
                    .map(|column| self.columns[*column].clone())
                    .collect(),
            );
            let shown = |column: usize| order.iter().position(|page| *page == column);
            if let Some(state) = state.as_deref_mut() {
                state.remap_columns(shown);
            }
            if let Some(editor) = editor.as_mut() {
                editor.column = shown(editor.column).unwrap_or(editor.column);
            }
        }
        // A finger selects several rows by a mark before each: the first
        // column grows by the mark's room, so its words keep theirs.
        if self.multi && state.is_some() && t.metrics.is_touch() && !self.columns.is_empty() {
            self.lead_gutter = t.metrics.touch_target();
            self.columns.to_mut()[0].width += self.lead_gutter - TABLE_EDGE_INSET;
        }
        // A column the reader sized keeps that width, whatever share of the
        // spare it was given: the flexible columns left share what is over.
        if let Some(state) = state.as_deref()
            && !state.widths.is_empty()
        {
            let sized = (0..self.columns.len())
                .filter_map(|shown| Some((shown, *state.widths.get(&self.page_column(shown))?)))
                .collect::<Vec<_>>();
            let columns = self.columns.to_mut();
            for (shown, width) in sized {
                columns[shown].width = width.max(MIN_COLUMN_W);
                columns[shown].flex = 0.0;
            }
        }
        let order = self.order.clone();
        let mut shown_row = move |row: usize| {
            let cells = row_at(row);
            match &order {
                Some(order) => in_order(cells, order),
                None => cells,
            }
        };
        let mut rows = Rows::new(self.rows, &mut shown_row);

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
        let bands_h = band_h * self.bands.len() as f32;
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
        // A narrow table holds the first column only, and the pinned one
        // beside it: a wider freeze leaves the values a sliver, or none.
        let frozen = if narrow {
            self.frozen.min(1) + usize::from(self.order.is_some())
        } else {
            self.frozen
        };
        let geometry = Columns::new(
            column_widths(&self.columns, viewport_w.max(least)),
            frozen,
            viewport_w,
        );

        // The keyboard, before anything is painted, so a move shows at once.
        let table_id = ui.id().with((self.id, "table"));
        let mut memory = ViewMemory::load(ui.ctx(), table_id);
        let mut scroll_x = state.as_deref().map(|state| state.scroll_x);
        let mut reveal = (std::mem::take(&mut memory.reveal) || self.reveal)
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
            let keys = Keys::read(ui, self.multi);
            if keys.step.is_some() || keys.across.is_some() {
                // The arrows moved the selection, not the keyboard focus:
                // the frame the table took focus it had no filter yet.
                ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
            }
            if let Some(step_by) = keys.step {
                let page = ((memory.height.max(row_h) / row_h).floor() as usize).max(1);
                if let Some(row) = step(&shown, state.selected_row(), step_by, page) {
                    let lead = self.selection_at(row, state.selected_column());
                    response.selection_changed |= if keys.extend {
                        let range = self.range(state, &shown, row);
                        state.extend(lead, range, false)
                    } else {
                        state.select_only(lead)
                    };
                    reveal = Some(row);
                }
            }
            if keys.all
                && let Some(first) = shown.first()
            {
                let lead = state
                    .selected_row()
                    .filter(|row| shown.contains(row))
                    .unwrap_or(*first);
                let lead = self.selection_at(lead, state.selected_column());
                let every = shown
                    .iter()
                    .map(|row| self.selection_at(*row, None))
                    .collect();
                response.selection_changed |= state.extend(lead, every, false);
            }
            if !keys.typed.is_empty() {
                let now = ui.input(|input| input.time);
                if now - memory.typed_at > TYPE_AHEAD_PAUSE {
                    memory.typed.clear();
                }
                memory.typed_at = now;
                memory.typed.push_str(&keys.typed.to_lowercase());
                let column = self.text_column();
                let found = type_ahead(
                    &shown,
                    state.selected_row(),
                    memory.typed.trim_start(),
                    |row| {
                        rows.get(row)
                            .get(column)
                            .map_or_else(String::new, |cell| cell.text.to_lowercase())
                    },
                );
                if let Some(row) = found {
                    let lead = self.selection_at(row, state.selected_column());
                    response.selection_changed |= state.select_only(lead);
                    reveal = Some(row);
                }
            }
            if let (Some(by), false) = (keys.across, self.cell_selection)
                && let Some(lead) = state.selected_row()
            {
                match tree_step(&shown, lead, by, |row| {
                    rows.get(row).first().map_or((0, None), Cell::branch)
                }) {
                    TreeStep::Expand(open) => response.expand = Some((lead, open)),
                    TreeStep::Move(row) => {
                        let lead = self.selection_at(row, None);
                        response.selection_changed |= state.select_only(lead);
                        reveal = Some(row);
                    }
                    TreeStep::Stay => response.across = Some(by),
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
            if (keys.enter || keys.f2)
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
                } else if keys.enter {
                    response.activate = Some(selection.row);
                } else {
                    response.rename = Some(selection.row);
                }
            }
            response.delete = keys.delete && state.selection.is_some();
            if keys.copy && state.selection.is_some() {
                let (named, heads) = self.copied_heads();
                let lines = shown
                    .iter()
                    .filter(|row| state.is_selected(**row))
                    .map(|row| self.copied_line(&named, rows.get(*row)))
                    .collect::<Vec<_>>();
                ui.ctx().copy_text(
                    heads
                        .into_iter()
                        .chain(lines)
                        .collect::<Vec<_>>()
                        .join("\n"),
                );
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
        // Every selected row, the lead among them, as the frame began.
        let chosen = state
            .as_deref()
            .map(|state| state.selected_rows().into_iter().collect::<BTreeSet<_>>())
            .unwrap_or_default();
        let modifiers = ui.input(|input| input.modifiers);
        let mut folds = Vec::new();
        let mut take_focus = false;
        // The table's one stop holds its rows for a reader: made before
        // them, so they find it.
        if state.is_some() {
            ui.ctx().accesskit_node_builder(table_id, |_| ());
        }
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
            // The rows belong to the table's one keyboard stop, so a reader
            // hears them as its rows.
            let mut builder = egui::UiBuilder::new().max_rect(band);
            if state.is_some() {
                builder = builder.accessibility_parent(table_id);
            }
            ui.scope_builder(builder, |ui| {
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
                            let mut row = self.row(
                                ui,
                                body::RowPlace {
                                    index,
                                    view,
                                    geometry: &geometry,
                                    height: row_h,
                                    selected: marked.is_some() || chosen.contains(&index),
                                    selected_column: marked.and_then(|selection| selection.column),
                                    stateful: state.is_some(),
                                },
                                cells,
                                open,
                            );
                            if let Some(drag) = row.dropped {
                                response.dropped = Some((index, drag));
                            }
                            if let Some((start, origin)) = row.held
                                && memory.held_press != Some(start)
                            {
                                memory.held_press = Some(start);
                                row.context = row.context.or(Some(origin));
                            }
                            take_focus |= self.answer(
                                &mut response,
                                state.as_deref_mut(),
                                Pressed {
                                    index,
                                    modifiers,
                                    shown: &shown,
                                },
                                row,
                            );
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

        if self.kind == Kind::Grid {
            let answer = self.head(
                ui,
                head::HeadPlace {
                    block: head_block,
                    response: &head_response,
                    height: head_h,
                    band_height: band_h,
                    view,
                    geometry: &geometry,
                    sort,
                    sortable: state.is_some(),
                },
            );
            if let Some(state) = state.as_deref_mut() {
                if let Some(column) = answer.sort {
                    state.cycle_sort(column);
                    memory.reveal = true;
                }
                if let Some((column, width)) = answer.resize {
                    state
                        .widths
                        .insert(self.page_column(column), width.max(MIN_COLUMN_W));
                    ui.ctx().request_repaint();
                }
            }
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
        if self.order.is_some() {
            let page = |column: usize| self.page_column(column);
            if let Some(state) = state.as_deref_mut() {
                state.remap_columns(|column| Some(page(column)));
            }
            if let Some((_, column)) = response.edit.as_mut() {
                *column = page(*column);
            }
            for cell in [
                response.double_clicked.as_mut(),
                response.context_menu.as_mut().map(|menu| &mut menu.cell),
            ]
            .into_iter()
            .flatten()
            {
                cell.column = cell.column.map(page);
            }
        }
        if state.is_some() {
            self.focus_target(
                ui,
                table_id,
                head_block.union(output.inner_rect),
                selected.as_ref(),
                state
                    .as_deref()
                    .map_or(0, |state| state.selected_rows().len()),
            );
            // Taken after the focus target has been seen, so a press that
            // lands on a row is not read as a press elsewhere.
            if take_focus || self.focus {
                ui.memory_mut(|memory| memory.request_focus(table_id));
            }
        }
        memory.store(ui.ctx(), table_id);
        response
    }

    /// Row `row` as a selection, keeping `column` when cells are selected.
    fn selection_at(&self, row: usize, column: Option<usize>) -> Selection {
        Selection {
            row,
            key: self.keys.and_then(|keys| keys.get(row).cloned()),
            column: if self.cell_selection { column } else { None },
        }
    }

    /// The rows from the anchor to `to`, in the order they show; `to` alone
    /// when the anchor is not shown.
    fn range(&self, state: &TableState, shown: &[usize], to: usize) -> Vec<Selection> {
        let anchor = state
            .anchor
            .as_ref()
            .map(|anchor| anchor.row)
            .or_else(|| state.selected_row())
            .unwrap_or(to);
        let at = |row: usize| shown.iter().position(|shown| *shown == row);
        match (at(anchor), at(to)) {
            (Some(from), Some(to)) => shown[from.min(to)..=from.max(to)]
                .iter()
                .map(|row| self.selection_at(*row, None))
                .collect(),
            _ => vec![self.selection_at(to, None)],
        }
    }

    /// The column type-ahead reads: the first whose words sit on the left,
    /// a name rather than a number.
    fn text_column(&self) -> usize {
        self.columns
            .iter()
            .position(|column| column.align == Align::Left)
            .unwrap_or(0)
    }

    /// Fold one row's report into the table's, and move the selection to
    /// where the reader pointed. Returns whether the table takes the
    /// keyboard: a row was pressed, or an open cell closed.
    fn answer(
        &self,
        response: &mut TableResponse,
        state: Option<&mut TableState>,
        pressed: Pressed<'_>,
        row: body::RowResponse,
    ) -> bool {
        let Pressed {
            index,
            modifiers,
            shown,
        } = pressed;
        if row.clicked || row.edit.is_some() {
            response.clicked = Some(index);
        }
        if let Some(column) = row.edit {
            response.edit = Some((index, column));
        }
        response.edited = response.edited.or(row.edited);
        if let Some(open) = row.twist {
            response.expand = Some((index, open));
        }
        if row.double_clicked {
            response.double_clicked = Some(CellRef {
                row: index,
                column: row.column,
            });
            response.activate = Some(index);
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
        let pressed = row.clicked || row.edit.is_some() || row.context.is_some() || row.marked;
        if pressed {
            let selection = self.selection_at(index, row.edit.or(row.column));
            response.selection_changed |= if row.marked {
                state.toggle(selection)
            } else if !self.multi || row.edit.is_some() {
                state.select_only(selection)
            } else if row.context.is_some() {
                state.lead_to(selection)
            } else if modifiers.shift {
                let range = self.range(state, shown, index);
                state.extend(selection, range, modifiers.command)
            } else if modifiers.command {
                state.toggle(selection)
            } else {
                state.select_only(selection)
            };
        }
        // A closed cell hands the keyboard back to the table.
        pressed || (row.edited.is_some() && row.edit.is_none())
    }

    /// The keyboard's place in a table with state: one stop for the whole
    /// table, where Tab lands and the arrow keys act, framed by the selected
    /// row while focus is shown.
    fn focus_target(
        &self,
        ui: &Ui,
        table_id: egui::Id,
        rect: Rect,
        selected: Option<&Selection>,
        chosen: usize,
    ) {
        let focus = ui.interact(rect, table_id, Sense::focusable_noninteractive());
        focus.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                format!(
                    "{}, {} rows{}",
                    match self.kind {
                        Kind::Grid => "Table",
                        Kind::List => "List",
                        Kind::Tree => "Tree",
                    },
                    self.rows,
                    match (selected, chosen) {
                        (_, 2..) => format!(", {chosen} rows selected"),
                        (Some(selection), _) => format!(", row {} selected", selection.row + 1),
                        (None, _) => String::new(),
                    }
                ),
            )
        });
        ui.ctx().accesskit_node_builder(focus.id, |node| {
            node.set_role(match self.kind {
                Kind::Grid => egui::accesskit::Role::Grid,
                Kind::List => egui::accesskit::Role::ListBox,
                Kind::Tree => egui::accesskit::Role::Tree,
            });
            if self.multi {
                node.set_multiselectable();
            }
        });
        if theme::focus_is_visible(ui.ctx(), &focus) {
            theme::paint_focus_ring(ui, &focus, rect);
        }
    }

    /// The columns Ctrl+C copies, and their heads as one tab-separated line
    /// over the rows: the columns that have a head in a grid; every column,
    /// under no head line, in a list.
    fn copied_heads(&self) -> (Vec<usize>, Option<String>) {
        if self.kind != Kind::Grid {
            return ((0..self.columns.len()).collect(), None);
        }
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
        (named, Some(heads))
    }

    /// One row's words under the `named` columns, tab-separated; a value the
    /// row does not hold is empty.
    fn copied_line(&self, named: &[usize], cells: &[Cell]) -> String {
        named
            .iter()
            .map(|index| {
                cells
                    .get(*index)
                    .filter(|cell| cell.ink != Ink::Faint)
                    .map_or("", |cell| cell.text.as_str())
            })
            .collect::<Vec<_>>()
            .join("\t")
    }

    /// The page's column under shown column `column`.
    fn page_column(&self, column: usize) -> usize {
        self.order
            .as_ref()
            .and_then(|order| order.get(column).copied())
            .unwrap_or(column)
    }

    /// The horizontal span a column's words may occupy: the cell less its
    /// padding, with the table's outer columns held to the panel inset, and
    /// the first column's words after the selection marks when it has them.
    fn text_span(&self, column: usize, cell: Rect) -> egui::Rangef {
        let left = if column == 0 {
            TABLE_EDGE_INSET.max(self.lead_gutter)
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

/// Where a row was pressed: its index, the modifiers held, and the rows in
/// the order they show, which a Shift press extends across.
struct Pressed<'s> {
    index: usize,
    modifiers: Modifiers,
    shown: &'s [usize],
}

/// The page's columns in the order a narrow table shows them, the pinned one
/// directly after the first: the page's column under each shown one. None
/// when the pin names no column past the first.
fn pinned_order(columns: usize, pinned: usize) -> Option<Vec<usize>> {
    (pinned > 0 && pinned < columns).then(|| {
        std::iter::once(0)
            .chain(std::iter::once(pinned))
            .chain((1..columns).filter(|column| *column != pinned))
            .collect()
    })
}

/// A row's cells in the shown order; a cell the row leaves out stays empty.
fn in_order(cells: Vec<Cell>, order: &[usize]) -> Vec<Cell> {
    let mut cells = cells.into_iter().map(Some).collect::<Vec<_>>();
    order
        .iter()
        .map(|column| {
            cells
                .get_mut(*column)
                .and_then(Option::take)
                .unwrap_or_else(Cell::empty)
        })
        .collect()
}

/// How one cell's words are set.
struct Words<'a> {
    text: &'a str,
    font: egui::FontId,
    align: Align,
    color: egui::Color32,
    problem: Option<Problem>,
}

/// `text` cut to `room` where a reader expects a cut: after the last whole
/// item of a list its middle dots set apart, else after the last whole word,
/// and inside a word only when the first is too long for the room alone.
fn elide_at_words(ui: &Ui, text: &str, font: &egui::FontId, room: f32) -> String {
    const ELLIPSIS: char = '\u{2026}';
    let cut = elide_text(ui, text, font, room);
    let Some(kept) = cut.strip_suffix(ELLIPSIS) else {
        return cut;
    };
    let rest = &text[kept.len()..];
    let before = |boundary: &str| {
        if rest.starts_with(boundary) {
            Some(kept)
        } else {
            kept.rfind(boundary).map(|at| &kept[..at])
        }
    };
    let head = before(" \u{b7} ")
        .or_else(|| before(" "))
        .unwrap_or_default()
        .trim_end_matches([',', ';', ':', '\u{b7}', ' ']);
    if head.is_empty() {
        cut
    } else {
        format!("{head}{ELLIPSIS}")
    }
}

/// Where a cell's words landed, and whether they were cut short to fit.
#[derive(Debug, Clone, Copy)]
struct Painted {
    landed: Rect,
    elided: bool,
}

/// Paint one cell's words on its line, elided to `span` and clipped to
/// `clip`, and return where they landed.
fn paint_words(
    ui: &Ui,
    cell: Rect,
    clip: Rect,
    span: egui::Rangef,
    words: Words<'_>,
) -> Option<Painted> {
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
    let shown = elide_at_words(ui, text, &font, (span.span() - mark_room).max(0.0));
    let elided = shown != text;
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
    Some(Painted { landed, elided })
}
