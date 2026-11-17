//! What a Verify table remembers between frames, held by the page that draws
//! it, and the order its rows are shown in.
//!
//! The page keeps one [`TableState`] in its session state and hands it to the
//! table each frame. The selection names a row by its index in the page's own
//! row order and, when the page keys its rows, by that key too: a sort moves
//! where a row is shown, never its index, and a refreshed run may reorder the
//! rows, so the key is what finds the selected row again.
//!
//! The shown order is worked out here, with no painting: the rows the filter
//! keeps, sorted inside their group when a column is sorted, each group led by
//! its head unless nothing in it is left to show.

use std::cmp::Ordering;
use std::collections::BTreeSet;

/// A row's identity that outlives its position in the page's row order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RowKey {
    Number(u64),
    Text(String),
}

impl From<u64> for RowKey {
    fn from(key: u64) -> Self {
        Self::Number(key)
    }
}

impl From<String> for RowKey {
    fn from(key: String) -> Self {
        Self::Text(key)
    }
}

impl From<&str> for RowKey {
    fn from(key: &str) -> Self {
        Self::Text(key.to_owned())
    }
}

/// The selected row, and the cell in it when the table selects cells.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    /// The row's index in the page's own row order.
    pub row: usize,
    /// The row's key, when the page keys its rows.
    pub key: Option<RowKey>,
    /// The selected cell's column, when the table selects cells.
    pub column: Option<usize>,
}

/// Which way a sorted column runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    /// Smallest first, words from A, a status worst first.
    Ascending,
    Descending,
}

/// The column the rows are sorted by, and which way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sort {
    pub column: usize,
    pub direction: SortDirection,
}

/// What one table remembers between frames.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TableState {
    pub selection: Option<Selection>,
    /// `None` shows the rows in the page's own order.
    pub sort: Option<Sort>,
    /// How far the columns right of the frozen ones are scrolled, in points.
    pub scroll_x: f32,
    /// Rows none of whose cells contain this, ignoring case, are hidden.
    pub filter: String,
    /// The labels of the folded groups.
    pub folded: BTreeSet<String>,
}

impl TableState {
    /// The selected row's index in the page's own row order.
    pub fn selected_row(&self) -> Option<usize> {
        self.selection.as_ref().map(|selection| selection.row)
    }

    /// The selected row's key, when the page keys its rows.
    pub fn selected_key(&self) -> Option<&RowKey> {
        self.selection.as_ref()?.key.as_ref()
    }

    /// The selected cell's column, when the table selects cells.
    pub fn selected_column(&self) -> Option<usize> {
        self.selection.as_ref()?.column
    }

    /// Select `row`, known by `key` when the page keys its rows.
    pub fn select(&mut self, row: usize, key: Option<RowKey>) {
        self.selection = Some(Selection {
            row,
            key,
            column: None,
        });
    }

    /// Whether the group labelled `label` is folded.
    pub fn is_folded(&self, label: &str) -> bool {
        self.folded.contains(label)
    }

    /// Find the selected row again by its key after the rows changed: its
    /// index follows the key, and a key no row carries any more drops the
    /// selection. A selection without a key keeps its index.
    pub(super) fn follow_keys(&mut self, keys: &[RowKey]) {
        let Some(selection) = &mut self.selection else {
            return;
        };
        let Some(key) = &selection.key else {
            return;
        };
        if keys.get(selection.row) == Some(key) {
            return;
        }
        match keys.iter().position(|candidate| candidate == key) {
            Some(row) => selection.row = row,
            None => self.selection = None,
        }
    }

    /// A head click: sort by `column`, then reverse it, then return to the
    /// page's own order.
    pub(super) fn cycle_sort(&mut self, column: usize) {
        self.sort = match self.sort {
            Some(Sort {
                column: sorted,
                direction: SortDirection::Ascending,
            }) if sorted == column => Some(Sort {
                column,
                direction: SortDirection::Descending,
            }),
            Some(Sort {
                column: sorted,
                direction: SortDirection::Descending,
            }) if sorted == column => None,
            _ => Some(Sort {
                column,
                direction: SortDirection::Ascending,
            }),
        };
    }

    /// Name the sorted and the selected column by `to`'s numbering, as a
    /// table whose columns show in another order than the page's does; a
    /// column `to` has no place for is dropped.
    pub(super) fn remap_columns(&mut self, to: impl Fn(usize) -> Option<usize>) {
        self.sort = self.sort.and_then(|sort| {
            Some(Sort {
                column: to(sort.column)?,
                ..sort
            })
        });
        if let Some(selection) = self.selection.as_mut() {
            selection.column = selection.column.and_then(&to);
        }
    }

    /// Fold the group labelled `label`, or open it again.
    pub(super) fn toggle_fold(&mut self, label: &str) {
        if !self.folded.remove(label) {
            self.folded.insert(label.to_owned());
        }
    }
}

/// A group head the page puts between its rows: the rows from `first_row` up
/// to the next group's first row belong to it. Groups are listed in row
/// order; rows before the first group belong to none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub label: String,
    pub first_row: usize,
}

impl Group {
    pub fn new(label: impl Into<String>, first_row: usize) -> Self {
        Self {
            label: label.into(),
            first_row,
        }
    }
}

/// What a cell is sorted by.
#[derive(Debug, Clone, PartialEq)]
pub enum SortKey {
    Number(f64),
    /// Compared ignoring case.
    Text(String),
    /// A status's severity, higher for worse: an ascending sort puts the
    /// worst first.
    Status(u8),
    /// Nothing to sort by: after every other key, whichever way the column
    /// runs.
    Missing,
}

impl SortKey {
    /// Where a key of this kind sorts against a key of another kind.
    fn kind(&self) -> u8 {
        match self {
            Self::Status(_) => 0,
            Self::Number(_) => 1,
            Self::Text(_) => 2,
            Self::Missing => 3,
        }
    }

    /// The order of two keys in a column running `direction`.
    pub(super) fn order(&self, other: &Self, direction: SortDirection) -> Ordering {
        let missing = |key: &Self| match key {
            Self::Missing => true,
            Self::Number(value) => value.is_nan(),
            Self::Text(_) | Self::Status(_) => false,
        };
        match (missing(self), missing(other)) {
            (true, true) => return Ordering::Equal,
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            (false, false) => {}
        }
        let ascending = match (self, other) {
            (Self::Number(left), Self::Number(right)) => left.total_cmp(right),
            (Self::Text(left), Self::Text(right)) => left
                .to_lowercase()
                .cmp(&right.to_lowercase())
                .then_with(|| left.cmp(right)),
            (Self::Status(left), Self::Status(right)) => right.cmp(left),
            _ => self.kind().cmp(&other.kind()),
        };
        match direction {
            SortDirection::Ascending => ascending,
            SortDirection::Descending => ascending.reverse(),
        }
    }
}

/// One line of the table body, in the order it is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Line {
    /// A group's head: the group's index and how many of its rows the filter
    /// keeps.
    Group { group: usize, rows: usize },
    /// A row, by its index in the page's own order.
    Row(usize),
}

/// The lines a table of `rows` shows: the rows `keep` keeps, in the page's
/// order or, when `order` is given, sorted by it inside their group (the sort
/// is stable, so equal rows keep the page's order); each group led by its
/// head, its rows hidden while it is folded, and a group with no row left to
/// show not shown at all.
pub(super) fn arrange(
    rows: usize,
    groups: &[Group],
    folded: &BTreeSet<String>,
    keep: impl Fn(usize) -> bool,
    order: Option<&dyn Fn(usize, usize) -> Ordering>,
) -> Vec<Line> {
    let mut lines = Vec::with_capacity(rows + groups.len());
    let mut segment = |range: std::ops::Range<usize>, group: Option<usize>| {
        let mut kept = range.filter(|row| keep(*row)).collect::<Vec<_>>();
        if let Some(order) = order {
            kept.sort_by(|left, right| order(*left, *right));
        }
        if let Some(group) = group {
            if kept.is_empty() {
                return;
            }
            lines.push(Line::Group {
                group,
                rows: kept.len(),
            });
            if folded.contains(&groups[group].label) {
                return;
            }
        }
        lines.extend(kept.into_iter().map(Line::Row));
    };
    let first = groups
        .first()
        .map_or(rows, |group| group.first_row.min(rows));
    segment(0..first, None);
    for (index, group) in groups.iter().enumerate() {
        let start = group.first_row.min(rows);
        let end = groups
            .get(index + 1)
            .map_or(rows, |next| next.first_row.min(rows))
            .max(start);
        segment(start..end, Some(index));
    }
    lines
}

/// A key that moves the selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Step {
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
}

/// The row a step from `current` lands on, among the rows `shown` in their
/// shown order; `page` rows make a page. A selection that is not shown steps
/// onto the first row shown, or the last for End.
pub(super) fn step(
    shown: &[usize],
    current: Option<usize>,
    step: Step,
    page: usize,
) -> Option<usize> {
    let last = shown.len().checked_sub(1)?;
    let page = page.max(1);
    let Some(at) = current.and_then(|row| shown.iter().position(|shown| *shown == row)) else {
        return Some(if step == Step::End {
            shown[last]
        } else {
            shown[0]
        });
    };
    let to = match step {
        Step::Up => at.saturating_sub(1),
        Step::Down => (at + 1).min(last),
        Step::PageUp => at.saturating_sub(page),
        Step::PageDown => (at + page).min(last),
        Step::Home => 0,
        Step::End => last,
    };
    Some(shown[to])
}
