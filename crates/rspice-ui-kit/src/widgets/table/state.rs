//! What a table remembers between frames, held by the page that draws it,
//! and the order its rows are shown in.
//!
//! The page keeps one [`TableState`] in its session state and hands it to the
//! table each frame. The selection names a row by its index in the page's own
//! row order and, when the page keys its rows, by that key too: a sort moves
//! where a row is shown, never its index, and refreshed data may reorder the
//! rows, so the key is what finds the selected row again. A table that
//! selects several rows keeps a lead, the row the keyboard moves from and
//! acts on, beside the others, and an anchor Shift extends from.
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
    /// The lead: the selected row the keyboard moves from and acts on, and
    /// its cell when the table selects cells.
    pub selection: Option<Selection>,
    /// The rows selected beside the lead, in the order they joined.
    pub others: Vec<Selection>,
    /// The row Shift extends the selection from: the last one a plain or a
    /// toggling press chose.
    pub anchor: Option<Selection>,
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

    /// Every selected row's index in the page's own order, the lead first.
    pub fn selected_rows(&self) -> Vec<usize> {
        self.selected().map(|selection| selection.row).collect()
    }

    /// Every selected row's key, the lead first, when the page keys its rows.
    pub fn selected_keys(&self) -> Vec<&RowKey> {
        self.selected()
            .filter_map(|selection| selection.key.as_ref())
            .collect()
    }

    /// Whether row `row`, in the page's own order, is selected.
    pub fn is_selected(&self, row: usize) -> bool {
        self.selected().any(|selection| selection.row == row)
    }

    fn selected(&self) -> impl Iterator<Item = &Selection> {
        self.selection.iter().chain(&self.others)
    }

    /// Select `row` alone, known by `key` when the page keys its rows.
    pub fn select(&mut self, row: usize, key: Option<RowKey>) {
        let _ = self.select_only(Selection {
            row,
            key,
            column: None,
        });
    }

    /// Make `selection` the only selected row and the anchor. Returns
    /// whether the selection changed.
    pub(super) fn select_only(&mut self, selection: Selection) -> bool {
        let changed = self.selection.as_ref() != Some(&selection) || !self.others.is_empty();
        self.anchor = Some(Selection {
            column: None,
            ..selection.clone()
        });
        self.selection = Some(selection);
        self.others.clear();
        changed
    }

    /// Make `selection` the lead and keep every other selected row, as a
    /// context press on a row of a selection does. Returns whether the
    /// selection changed.
    pub(super) fn lead_to(&mut self, selection: Selection) -> bool {
        if !self.is_selected(selection.row) {
            return self.select_only(selection);
        }
        let before = self.selection.clone();
        if let Some(lead) = self.selection.take() {
            self.others.push(lead);
        }
        self.others.retain(|other| other.row != selection.row);
        self.selection = Some(selection);
        before != self.selection
    }

    /// A toggling press, Ctrl or the leading mark: a selected row leaves the
    /// selection, any other joins it as the lead; either way it becomes the
    /// anchor. Returns whether the selection changed, which it always does.
    pub(super) fn toggle(&mut self, selection: Selection) -> bool {
        self.anchor = Some(Selection {
            column: None,
            ..selection.clone()
        });
        if self
            .selection
            .as_ref()
            .is_some_and(|lead| lead.row == selection.row)
        {
            self.selection = self.others.pop();
        } else if let Some(at) = self
            .others
            .iter()
            .position(|other| other.row == selection.row)
        {
            self.others.remove(at);
        } else {
            if let Some(lead) = self.selection.take() {
                self.others.push(lead);
            }
            self.selection = Some(selection);
        }
        true
    }

    /// An extending press, Shift: `range` (the rows from the anchor to the
    /// pressed one, in the order they show) becomes the selection, added to
    /// it when `add`, with `lead` as the lead. The anchor stays. Returns
    /// whether the selection changed.
    pub(super) fn extend(&mut self, lead: Selection, range: Vec<Selection>, add: bool) -> bool {
        let before = (self.selection.clone(), self.others.clone());
        let mut others = if add {
            self.selected().cloned().collect()
        } else {
            Vec::new()
        };
        for selection in range {
            if !others
                .iter()
                .any(|other: &Selection| other.row == selection.row)
            {
                others.push(selection);
            }
        }
        others.retain(|other| other.row != lead.row);
        if self.anchor.is_none() {
            self.anchor = Some(Selection {
                column: None,
                ..lead.clone()
            });
        }
        self.selection = Some(lead);
        self.others = others;
        before != (self.selection.clone(), self.others.clone())
    }

    /// Whether the group labelled `label` is folded.
    pub fn is_folded(&self, label: &str) -> bool {
        self.folded.contains(label)
    }

    /// Find the selected rows again by their keys after the rows changed:
    /// each index follows its key, and a key no row carries any more drops
    /// that row from the selection; a lead that goes hands the lead to the
    /// row that joined last. A selection without a key keeps its index.
    pub(super) fn follow_keys(&mut self, keys: &[RowKey]) {
        /// Whether `selection` still names a row, moved to where it is now.
        fn follow(selection: &mut Selection, keys: &[RowKey]) -> bool {
            let Some(key) = &selection.key else {
                return true;
            };
            if keys.get(selection.row) == Some(key) {
                return true;
            }
            match keys.iter().position(|candidate| candidate == key) {
                Some(row) => {
                    selection.row = row;
                    true
                }
                None => false,
            }
        }
        self.others.retain_mut(|other| follow(other, keys));
        if let Some(anchor) = &mut self.anchor
            && !follow(anchor, keys)
        {
            self.anchor = None;
        }
        if let Some(lead) = &mut self.selection
            && !follow(lead, keys)
        {
            self.selection = self.others.pop();
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

/// How long a pause between typed letters starts a new type-ahead search.
pub(super) const TYPE_AHEAD_PAUSE: f64 = 1.0;

/// The row type-ahead moves to, among the rows `shown` in their shown order:
/// the next whose words, `words(row)` in lower case, start with what was
/// `typed` (lower case too). One letter, or one letter typed again and
/// again, moves on past the current row and cycles among the rows it
/// starts; more letters refine from the current row, so it stays while it
/// still matches.
pub(super) fn type_ahead(
    shown: &[usize],
    current: Option<usize>,
    typed: &str,
    mut words: impl FnMut(usize) -> String,
) -> Option<usize> {
    let first = typed.chars().next()?;
    let at = current.and_then(|row| shown.iter().position(|shown| *shown == row));
    let (needle, start) = if typed.chars().all(|letter| letter == first) {
        (first.to_string(), at.map_or(0, |at| at + 1))
    } else {
        (typed.to_owned(), at.unwrap_or(0))
    };
    (0..shown.len())
        .map(|step| shown[(start + step) % shown.len()])
        .find(|row| words(*row).starts_with(&needle))
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
