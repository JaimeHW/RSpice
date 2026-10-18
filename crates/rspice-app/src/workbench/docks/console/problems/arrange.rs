//! What the grid lists, in what order, and in what groups.
//!
//! One pass over the canonical list answers every question the page asks of
//! it: which findings the filters admit, what each filter toggle would list if
//! the reader moved it, which producers are present, and how wide each column
//! fitted to its content has to be. They are answered together because they
//! are all answers about the same rows, and because the list is sized for a
//! million records — a page that walked it once per question would walk it six
//! times a frame. Measuring the subject and location spells each listed row
//! out, which is a cost only a build pays.
//!
//! That pass, and the order and groups built from it, are kept between frames
//! and rebuilt only when the list or the reading position changed. Each frame
//! takes a stamp of what the arrangement was built from: the store's rows by
//! each batch's fingerprint and standing, without a walk, and the editors'
//! findings by folding their [`RowFacet`]s, which spell nothing out. A frame
//! whose stamp and position match the last build visits no stored row, sorts
//! nothing and spells out no row. A position that reads row text — a text
//! filter, an order by severity, subject, location or message, groups by
//! subject — also folds the editors' row text into the stamp, because one of
//! their rows can be republished saying something else under the same
//! identity; a stored batch's fingerprint already covers what its rows say.
//! Any other position measures the editors' rows as the build found them: an
//! editor mints a finding's identity from its message and the revision it
//! validated, so a row that says something else is a new identity, and the
//! facets fold that.
//!
//! The rows are always ordered. The order producers happened to report in
//! says nothing a reader can use, so a page opens worst first.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;
use crate::workbench::app_state::diagnostic_store::listing::{
    DiagnosticListing, DiagnosticRow, RowFacet,
};
use crate::workbench::app_state::problems_view::{ProblemColumn, ProblemGrouping, ProblemsView};

/// One printed line of the body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Line {
    /// A group header, by index into the arrangement's groups.
    Group(usize),
    /// A finding, by its index in the canonical listing.
    Row(usize),
}

/// What one group holds, in the terms the design is judged by.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct GroupCounts {
    pub(super) errors: usize,
    pub(super) advisories: usize,
    pub(super) stale: usize,
    pub(super) waived: usize,
}

impl GroupCounts {
    fn record(&mut self, facet: &RowFacet<'_>) {
        match (facet.current, facet.waived) {
            (false, _) => self.stale += 1,
            (true, true) => self.waived += 1,
            (true, false) if facet.level.is_error() => self.errors += 1,
            (true, false) => self.advisories += 1,
        }
    }

    /// The counts in the words a group header prints, worst first, naming only
    /// what the group actually holds.
    pub(super) fn summary(self) -> String {
        let mut stated = Vec::with_capacity(4);
        for (count, singular, plural) in [
            (self.errors, "error", "errors"),
            (self.advisories, "advisory", "advisories"),
            (self.stale, "stale", "stale"),
            (self.waived, "waived", "waived"),
        ] {
            if count > 0 {
                stated.push(format!(
                    "{count} {}",
                    if count == 1 { singular } else { plural }
                ));
            }
        }
        // A group exists because a finding is in it, so at least one of the
        // four counts above is non-zero.
        stated.join(" · ")
    }
}

/// One group of findings.
#[derive(Debug, Clone)]
pub(super) struct Group {
    /// The value the group is grouped on, in full.
    pub(super) label: String,
    pub(super) counts: GroupCounts,
    pub(super) collapsed: bool,
}

/// What each filter control would list, and how wide the measured columns
/// have to be to print what is listed.
#[derive(Debug, Clone)]
pub(super) struct Tally {
    /// Per severity rung, in [`DiagnosticLevel::ALL`] order: the findings that
    /// rung would list, with every *other* filter exactly as it stands. That
    /// is what makes the number beside a toggle a description of the toggle
    /// rather than of the list.
    severities: [usize; DiagnosticLevel::ALL.len()],
    /// The waived findings the waiver toggle would list, under the other
    /// filters as they stand.
    pub(super) waived: usize,
    /// Every producer present, with what narrowing to it would list.
    pub(super) producers: Vec<(String, usize)>,
    /// The longest cell each column fitted to its content prints among the
    /// listed rows, in characters, in [`super::grid::FITTED`] order. The face
    /// is monospace, so a character count is what the column measurement
    /// needs.
    pub(super) widest: [usize; super::grid::FITTED.len()],
}

impl Tally {
    pub(super) const fn severity(&self, level: DiagnosticLevel) -> usize {
        self.severities[level.rung()]
    }
}

/// What the listing held when an arrangement was built from it.
///
/// A digest rather than a copy, so a frame checks it without holding a second
/// list. It covers every input the build reads: each finding's identity,
/// severity, standing and waiver in reported order — the identity is minted
/// from the producer and the rule, so those are covered by it — and, where
/// the reading position reads them, what each row says.
///
/// The store's rows are stated by the listing itself, by each batch's
/// fingerprint and standing, so a frame does not walk them; only the editors'
/// findings, which are read in place and carry no fingerprint the list can
/// see, are folded row by row.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Stamp {
    rows: usize,
    stored: Vec<(u64, bool)>,
    facets: u64,
    text: u64,
}

/// The rows the page prints, with the groups and counts that describe them.
pub(super) struct Arrangement {
    /// Every finding the filters admit, in print order, a group's findings
    /// together and collapsed groups included.
    order: Vec<usize>,
    /// The printed lines, built only when the rows are grouped; ungrouped,
    /// `order` is the printed lines.
    lines: Vec<Line>,
    groups: Vec<Group>,
    /// Where each group's findings start in `order`.
    group_starts: Vec<usize>,
    /// Findings the filters admit, whether or not their group is collapsed.
    pub(super) listed: usize,
    pub(super) tally: Tally,
    /// The reading position it was built for.
    view: ProblemsView,
    /// The text filter, lowercased once rather than once a row.
    needle: String,
    stamp: Option<Stamp>,
}

/// The arrangement for this frame, and which listing index the current
/// finding is when the filters list it.
///
/// A current finding the filters exclude is still the current finding — the
/// canvas may be standing on it — but it is not a row on this page, so the
/// page draws no selection and describes nothing.
pub(super) fn arrange(
    ctx: &egui::Context,
    listed: &DiagnosticListing<'_>,
    view: &ProblemsView,
    current: Option<uuid::Uuid>,
) -> (Arc<Arrangement>, Option<usize>) {
    let reads_text = view.reads_row_text();
    let mut facets = std::collections::hash_map::DefaultHasher::new();
    let mut text = std::collections::hash_map::DefaultHasher::new();
    let rows = listed.len();
    for index in listed.stored_len()..rows {
        let Some(facet) = listed.facet(index) else {
            continue;
        };
        (facet.id, facet.level.rung(), facet.current, facet.waived).hash(&mut facets);
        if reads_text && let Some(row) = listed.row(index) {
            (&row.subject, &row.location, row.message).hash(&mut text);
        }
    }
    let stamp = Stamp {
        rows,
        stored: listed.stored_stamp(),
        facets: facets.finish(),
        text: text.finish(),
    };

    let id = egui::Id::new("workbench.problems.arrangement");
    let arrangement = match ctx.data(|data| data.get_temp::<Arc<Arrangement>>(id)) {
        Some(built) if built.stamp.as_ref() == Some(&stamp) && built.view.arranges_like(view) => {
            built
        }
        _ => {
            let mut built = Arrangement::build(listed, view);
            built.stamp = Some(stamp.clone());
            let built = Arc::new(built);
            ctx.data_mut(|data| data.insert_temp(id, Arc::clone(&built)));
            built
        }
    };
    // Where the current finding is, found by a walk only when the finding or
    // the list moved: a frame in which neither did reads it back.
    let found = egui::Id::new("workbench.problems.current-row");
    let standing = match ctx.data(|data| data.get_temp::<CurrentRow>(found)) {
        Some(row) if row.stamp == stamp && row.finding == current => row.index,
        _ => {
            let index = current.and_then(|current| {
                (0..rows).find(|index| {
                    listed
                        .facet(*index)
                        .is_some_and(|facet| facet.id == current)
                })
            });
            ctx.data_mut(|data| {
                data.insert_temp(
                    found,
                    CurrentRow {
                        stamp,
                        finding: current,
                        index,
                    },
                );
            });
            index
        }
    };
    let current = standing.filter(|index| arrangement.admits(listed, *index));
    (arrangement, current)
}

/// The listing index of the current finding, and the list and finding it was
/// found for.
#[derive(Debug, Clone)]
struct CurrentRow {
    stamp: Stamp,
    finding: Option<uuid::Uuid>,
    index: Option<usize>,
}

impl Arrangement {
    pub(super) fn build(listed: &DiagnosticListing<'_>, view: &ProblemsView) -> Self {
        let needle = view.text.to_lowercase();
        let total = listed.len();
        let mut severities = [0_usize; DiagnosticLevel::ALL.len()];
        let mut waived = 0;
        let mut producers: BTreeMap<&str, usize> = BTreeMap::new();
        let mut widest = [0_usize; super::grid::FITTED.len()];
        let mut admitted: Vec<usize> = Vec::new();
        let mut count = 0;

        for index in 0..total {
            let Some(facet) = listed.facet(index) else {
                continue;
            };
            if !text_admits(listed, index, &needle) {
                continue;
            }
            let severity_ok = view.severities.lists(facet.level);
            let producer_ok = producer_admits(view, &facet);
            let waiver_ok = view.waived || !facet.waived;

            if producer_ok && waiver_ok {
                severities[facet.level.rung()] += 1;
            }
            if severity_ok && producer_ok && facet.waived {
                waived += 1;
            }
            if severity_ok && waiver_ok {
                *producers.entry(facet.producer).or_default() += 1;
            }
            if severity_ok && producer_ok && waiver_ok {
                count += 1;
                // What a listed row prints, measured: the rule and producer
                // from the facet, and the severity word, subject and location
                // from the row spelled out. Only a rebuild spells them.
                if let Some(row) = listed.row(index) {
                    let cells = [
                        super::severity_cell(&row).chars().count(),
                        facet.code.chars().count(),
                        row.subject.chars().count(),
                        facet.producer.chars().count(),
                        row.location.chars().count(),
                    ];
                    for (widest, cell) in widest.iter_mut().zip(cells) {
                        *widest = (*widest).max(cell);
                    }
                }
                admitted.push(index);
            }
        }

        let mut arrangement = Self {
            order: Vec::new(),
            lines: Vec::new(),
            groups: Vec::new(),
            group_starts: Vec::new(),
            listed: count,
            tally: Tally {
                severities,
                waived,
                producers: producers
                    .into_iter()
                    .map(|(producer, count)| (producer.to_owned(), count))
                    .collect(),
                widest,
            },
            view: view.clone(),
            needle,
            stamp: None,
        };

        admitted = ordered(listed, admitted, view.sort, view.descending);
        if view.grouping.is_grouped() {
            let grouped = group(listed, &admitted, view.grouping, view);
            arrangement.order = grouped.order;
            arrangement.lines = grouped.lines;
            arrangement.groups = grouped.groups;
            arrangement.group_starts = grouped.starts;
        } else {
            arrangement.order = admitted;
        }
        arrangement
    }

    /// Whether the filters admit one finding, asked of a single row without
    /// rebuilding: it is how the current finding is found to be a row on this
    /// page or not.
    fn admits(&self, listed: &DiagnosticListing<'_>, index: usize) -> bool {
        listed.facet(index).is_some_and(|facet| {
            self.view.severities.lists(facet.level)
                && producer_admits(&self.view, &facet)
                && (self.view.waived || !facet.waived)
                && text_admits(listed, index, &self.needle)
        })
    }

    /// Printed lines, group headers included.
    pub(super) fn lines(&self) -> usize {
        if self.view.grouping.is_grouped() {
            self.lines.len()
        } else {
            self.order.len()
        }
    }

    pub(super) fn line(&self, index: usize) -> Option<Line> {
        if self.view.grouping.is_grouped() {
            self.lines.get(index).copied()
        } else {
            self.order.get(index).copied().map(Line::Row)
        }
    }

    pub(super) fn group(&self, index: usize) -> Option<&Group> {
        self.groups.get(index)
    }

    /// Which group a listed finding is in, by index, when the rows are
    /// grouped.
    pub(super) fn group_of(&self, listing_index: usize) -> Option<usize> {
        if !self.view.grouping.is_grouped() {
            return None;
        }
        let position = self.order.iter().position(|row| *row == listing_index)?;
        self.group_starts
            .partition_point(|start| *start <= position)
            .checked_sub(1)
    }

    /// The group whose value is `label`, by index.
    pub(super) fn group_named(&self, label: &str) -> Option<usize> {
        self.groups.iter().position(|group| group.label == label)
    }

    /// The printed line a group's header is on.
    pub(super) fn header_line(&self, group: usize) -> Option<usize> {
        self.lines
            .iter()
            .position(|line| *line == Line::Group(group))
    }

    /// Whether any group is collapsed, so that what is listed is more than
    /// what is printed.
    pub(super) fn hides_collapsed_rows(&self) -> bool {
        self.groups.iter().any(|group| group.collapsed)
    }

    /// Every finding the filters admit, in print order, collapsed groups
    /// included: what "listed" counts.
    pub(super) fn listed_rows(&self) -> impl Iterator<Item = usize> + '_ {
        self.order.iter().copied()
    }

    /// Which printed line a finding is on, when it is printed at all.
    pub(super) fn line_of(&self, listing_index: usize) -> Option<usize> {
        if self.view.grouping.is_grouped() {
            self.lines
                .iter()
                .position(|line| *line == Line::Row(listing_index))
        } else {
            self.order.iter().position(|row| *row == listing_index)
        }
    }

    /// The printed line `step` lines on from line `from`, wrapping at either
    /// end, or the first or last line when the keyboard stands nowhere.
    ///
    /// Group headers are lines like any other: a collapsed group is reached,
    /// and opened again, from its header. Every move is arithmetic on line
    /// numbers, so no list is walked to make one.
    pub(super) fn step_line(&self, from: Option<usize>, step: isize) -> Option<usize> {
        let count = self.lines();
        if count == 0 {
            return None;
        }
        Some(match from {
            Some(line) if line < count => (line as isize + step)
                .rem_euclid(count as isize)
                .unsigned_abs(),
            _ if step >= 0 => 0,
            _ => count - 1,
        })
    }

    /// The printed line `lines` lines on from line `from` in the direction of
    /// travel, stopping at the first or last line rather than wrapping: a page
    /// key that wrapped would carry the reader from the bottom of the list to
    /// its top.
    pub(super) fn page_line(
        &self,
        from: Option<usize>,
        lines: usize,
        forward: bool,
    ) -> Option<usize> {
        let last = self.lines().checked_sub(1)?;
        Some(match from {
            Some(line) if forward => line.saturating_add(lines).min(last),
            Some(line) => line.saturating_sub(lines).min(last),
            None if forward => 0,
            None => last,
        })
    }
}

fn producer_admits(view: &ProblemsView, facet: &RowFacet<'_>) -> bool {
    view.producer
        .as_deref()
        .is_none_or(|producer| producer == facet.producer)
}

/// Whether a row answers the reader's text. Only a row a text filter has to
/// read is spelled out.
fn text_admits(listed: &DiagnosticListing<'_>, index: usize, needle: &str) -> bool {
    needle.is_empty()
        || listed
            .row(index)
            .is_some_and(|row| row_matches(&row, needle))
}

/// Whether a row answers the reader's text, which is matched against the rule
/// it is an instance of, the thing it is about and what it says. Not the
/// producer: that has its own control, and a dotted identity matches
/// everything a reader is likely to type.
fn row_matches(row: &DiagnosticRow<'_>, needle: &str) -> bool {
    row.code.to_lowercase().contains(needle)
        || row.subject.to_lowercase().contains(needle)
        || row.message.to_lowercase().contains(needle)
}

/// Order the admitted rows by one column, stably.
///
/// Keys are materialized before the sort rather than spelled during it: a
/// comparator that formats a subject would format it `n log n` times.
fn ordered(
    listed: &DiagnosticListing<'_>,
    admitted: Vec<usize>,
    column: ProblemColumn,
    descending: bool,
) -> Vec<usize> {
    match column {
        ProblemColumn::Severity => by_severity(listed, &admitted, descending),
        ProblemColumn::Code | ProblemColumn::Producer => {
            let mut keyed: Vec<(String, usize)> = admitted
                .iter()
                .filter_map(|index| {
                    listed.facet(*index).map(|facet| {
                        let key = match column {
                            ProblemColumn::Producer => facet.producer,
                            _ => facet.code,
                        };
                        (key.to_lowercase(), *index)
                    })
                })
                .collect();
            sort_keyed(&mut keyed, descending, |left, right| natural(left, right));
            undecorate(keyed)
        }
        ProblemColumn::Subject | ProblemColumn::Location | ProblemColumn::Message => {
            let mut keyed: Vec<(String, usize)> = admitted
                .iter()
                .filter_map(|index| {
                    listed.row(*index).map(|row| {
                        let key = match column {
                            ProblemColumn::Subject => row.subject.to_lowercase(),
                            ProblemColumn::Location => row.location.to_lowercase(),
                            _ => row.message.to_lowercase(),
                        };
                        (key, *index)
                    })
                })
                .collect();
            sort_keyed(&mut keyed, descending, |left, right| natural(left, right));
            undecorate(keyed)
        }
    }
}

/// Order the admitted rows worst first, or least severe first when
/// `descending`.
///
/// Ascending is worst first, which is the lowest rung: the order a page opens
/// in, and the header says which way it is reading in words rather than
/// leaving a triangle to imply it. Findings of one severity are ordered as a
/// reader goes looking for them — by rule, then by what they are about, then
/// by where — so two findings never swap places because their producers
/// reported them the other way round. Only the severity is reversed: inside
/// one severity the order reads the same either way.
///
/// The rule is a facet; the subject and the location have to be spelled out,
/// so they are, only for findings that share a severity and a rule.
fn by_severity(listed: &DiagnosticListing<'_>, admitted: &[usize], descending: bool) -> Vec<usize> {
    let mut keyed: Vec<(usize, String, usize)> = admitted
        .iter()
        .filter_map(|index| {
            listed
                .facet(*index)
                .map(|facet| (facet.level.rung(), facet.code.to_lowercase(), *index))
        })
        .collect();
    keyed.sort_by(|left, right| {
        let rung = left.0.cmp(&right.0);
        let rung = if descending { rung.reverse() } else { rung };
        rung.then_with(|| natural(&left.1, &right.1))
    });
    let mut order = Vec::with_capacity(keyed.len());
    let mut start = 0;
    while start < keyed.len() {
        let (rung, code) = (keyed[start].0, &keyed[start].1);
        let end = start
            + keyed[start..]
                .iter()
                .take_while(|key| key.0 == rung && key.1 == *code)
                .count();
        if end - start == 1 {
            order.push(keyed[start].2);
        } else {
            let mut run: Vec<(String, String, usize)> = keyed[start..end]
                .iter()
                .filter_map(|(_, _, index)| {
                    listed.row(*index).map(|row| {
                        (
                            row.subject.to_lowercase(),
                            row.location.to_lowercase(),
                            *index,
                        )
                    })
                })
                .collect();
            run.sort_by(|left, right| {
                natural(&left.0, &right.0).then_with(|| natural(&left.1, &right.1))
            });
            order.extend(run.into_iter().map(|(_, _, index)| index));
        }
        start = end;
    }
    order
}

/// Sort a key list stably in the direction the reader asked for.
///
/// The direction is applied to the comparison, not to the sorted result:
/// reversing a stable ascending sort would reverse equal keys as well, and
/// equal keys keep the order their producers reported them in either way.
fn sort_keyed<K>(keyed: &mut [(K, usize)], descending: bool, compare: impl Fn(&K, &K) -> Ordering) {
    keyed.sort_by(|left, right| {
        let ordering = compare(&left.0, &right.0);
        if descending {
            ordering.reverse()
        } else {
            ordering
        }
    });
}

fn undecorate<K>(keyed: Vec<(K, usize)>) -> Vec<usize> {
    keyed.into_iter().map(|(_, index)| index).collect()
}

/// Compare two cells the way a reader counts: a run of digits is a number, so
/// `line 9` comes before `line 10`, `line 2 · column 9` before
/// `line 2 · column 10`, and `R9` before `R10`. Everything else compares
/// character by character.
///
/// This is what makes the location column sort by line and then column: a
/// location is printed as its structure, and reading its numbers as numbers
/// is reading that structure back.
fn natural(left: &str, right: &str) -> Ordering {
    let mut left = left.chars().peekable();
    let mut right = right.chars().peekable();
    loop {
        match (left.peek().copied(), right.peek().copied()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(l), Some(r)) if l.is_ascii_digit() && r.is_ascii_digit() => {
                let ordering = number(&mut left, &mut right);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            (Some(l), Some(r)) => {
                if l != r {
                    return l.cmp(&r);
                }
                left.next();
                right.next();
            }
        }
    }
}

/// Compare the digit runs both sides are standing on, by value, and consume
/// them. Leading zeros carry no value; a longer run of significant digits is
/// the larger number; equal lengths compare at their first differing digit.
fn number(
    left: &mut std::iter::Peekable<std::str::Chars<'_>>,
    right: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Ordering {
    while left.next_if_eq(&'0').is_some() {}
    while right.next_if_eq(&'0').is_some() {}
    let mut first_difference = Ordering::Equal;
    loop {
        match (
            left.next_if(char::is_ascii_digit),
            right.next_if(char::is_ascii_digit),
        ) {
            (Some(l), Some(r)) => {
                if first_difference == Ordering::Equal {
                    first_difference = l.cmp(&r);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return first_difference,
        }
    }
}

/// The admitted rows, gathered into groups.
struct Grouped {
    /// Every listed row in print order, collapsed groups included.
    order: Vec<usize>,
    /// The printed lines: each group's header, then its rows unless it is
    /// collapsed.
    lines: Vec<Line>,
    groups: Vec<Group>,
    /// Where each group's rows start in `order`.
    starts: Vec<usize>,
}

/// Gather the admitted rows into groups, keeping each group's rows in the
/// order the arrangement put them in.
fn group(
    listed: &DiagnosticListing<'_>,
    admitted: &[usize],
    grouping: ProblemGrouping,
    view: &ProblemsView,
) -> Grouped {
    let mut gathered: BTreeMap<String, (GroupCounts, Vec<usize>)> = BTreeMap::new();
    for index in admitted {
        let Some(facet) = listed.facet(*index) else {
            continue;
        };
        let label = match grouping {
            ProblemGrouping::Producer => facet.producer.to_owned(),
            ProblemGrouping::Code => facet.code.to_owned(),
            ProblemGrouping::Subject | ProblemGrouping::None => listed
                .row(*index)
                .map(|row| row.subject)
                .unwrap_or_default(),
        };
        let entry = gathered.entry(label).or_default();
        entry.0.record(&facet);
        entry.1.push(*index);
    }

    let mut grouped = Grouped {
        order: Vec::with_capacity(admitted.len()),
        lines: Vec::with_capacity(admitted.len() + gathered.len()),
        groups: Vec::with_capacity(gathered.len()),
        starts: Vec::with_capacity(gathered.len()),
    };
    for (label, (counts, rows)) in gathered {
        let collapsed = view.is_collapsed(&label);
        grouped.lines.push(Line::Group(grouped.groups.len()));
        if !collapsed {
            grouped.lines.extend(rows.iter().copied().map(Line::Row));
        }
        grouped.starts.push(grouped.order.len());
        grouped.order.extend(rows);
        grouped.groups.push(Group {
            label,
            counts,
            collapsed,
        });
    }
    grouped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_in_a_cell_compare_by_value() {
        let mut locations = vec![
            "line 10",
            "line 9",
            "line 2 · column 10",
            "line 2 · column 9",
            "",
            "line 100",
        ];
        locations.sort_by(|left, right| natural(left, right));
        assert_eq!(
            locations,
            [
                "",
                "line 2 · column 9",
                "line 2 · column 10",
                "line 9",
                "line 10",
                "line 100",
            ],
        );
        assert_eq!(natural("r007", "r7"), Ordering::Equal);
        assert_eq!(natural("r9", "r10"), Ordering::Less);
        assert_eq!(natural("r10a", "r10b"), Ordering::Less);
        assert_eq!(natural("a", "a0"), Ordering::Less);
    }

    /// Equal keys keep the order their producers reported them in, in both
    /// directions. Reversing an ascending sort would reverse them as well.
    #[test]
    fn equal_keys_keep_their_reported_order_either_way() {
        let keyed = || vec![(1, 10), (0, 11), (1, 12), (0, 13), (1, 14)];
        let mut ascending = keyed();
        sort_keyed(&mut ascending, false, Ord::cmp);
        assert_eq!(undecorate(ascending), [11, 13, 10, 12, 14]);
        let mut descending = keyed();
        sort_keyed(&mut descending, true, Ord::cmp);
        assert_eq!(undecorate(descending), [10, 12, 14, 11, 13]);
    }
}
