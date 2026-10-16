//! What the grid lists, in what order, and in what groups.
//!
//! One pass over the canonical list answers every question the page asks of
//! it: which findings the filters admit, what each filter toggle would list if
//! the reader moved it, which producers are present, and how wide the rule and
//! producer columns have to be. They are answered together because they are
//! all answers about the same rows, and because the list is sized for a
//! million records — a page that walked it once per question would walk it six
//! times a frame.
//!
//! The pass reads [`RowFacet`]s, which spell nothing out, so an unfiltered
//! frame allocates nothing per row. A text filter is the one thing a facet
//! cannot answer, because matching a subject means spelling it; that is the
//! only case in which rows are built to decide what is listed.
//!
//! When nothing is filtered, ordered or grouped, no order is built at all: the
//! page draws the listing as its producers reported it, which is the order the
//! badge counts in.

use std::collections::BTreeMap;

use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;
use crate::workbench::app_state::diagnostic_store::listing::{DiagnosticListing, RowFacet};
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
pub(super) struct Tally<'a> {
    /// Per severity rung, in [`DiagnosticLevel::ALL`] order: the findings that
    /// rung would list, with every *other* filter exactly as it stands. That
    /// is what makes the number beside a toggle a description of the toggle
    /// rather than of the list.
    severities: [usize; DiagnosticLevel::ALL.len()],
    /// The waived findings the waiver toggle would list, under the other
    /// filters as they stand.
    pub(super) waived: usize,
    /// Every producer present, with what narrowing to it would list.
    pub(super) producers: Vec<(&'a str, usize)>,
    /// The longest rule code and producer identity among the listed rows, in
    /// characters. The face is monospace, so a character count is what the
    /// column measurement needs.
    pub(super) widest_code: usize,
    pub(super) widest_producer: usize,
}

impl Tally<'_> {
    pub(super) const fn severity(&self, level: DiagnosticLevel) -> usize {
        self.severities[level.rung()]
    }
}

/// The rows the page prints this frame, with the groups and counts that
/// describe them.
pub(super) struct Arrangement<'a> {
    /// The printed lines, built only when the reader asked for an order, a
    /// filter or groups.
    lines: Vec<Line>,
    /// How many findings the listing holds, when they are drawn exactly as
    /// reported and `lines` is therefore empty.
    as_reported: Option<usize>,
    groups: Vec<Group>,
    /// Findings the filters admit, whether or not their group is collapsed.
    pub(super) listed: usize,
    /// Which listing index the current finding is, when the filters list it.
    ///
    /// A current finding the filters exclude is still the current finding —
    /// the canvas may be standing on it — but it is not a row on this page, so
    /// the page draws no selection and describes nothing.
    pub(super) current: Option<usize>,
    pub(super) tally: Tally<'a>,
}

impl<'a> Arrangement<'a> {
    pub(super) fn build(
        listed: &DiagnosticListing<'a>,
        view: &ProblemsView,
        current: Option<uuid::Uuid>,
    ) -> Self {
        let needle = view.text.to_lowercase();
        let as_reported = view.lists_everything_as_reported();
        let total = listed.len();
        let mut severities = [0_usize; DiagnosticLevel::ALL.len()];
        let mut waived = 0;
        let mut producers: BTreeMap<&'a str, usize> = BTreeMap::new();
        let mut widest_code = 0;
        let mut widest_producer = 0;
        let mut admitted: Vec<usize> = Vec::new();
        let mut count = 0;
        let mut standing = None;

        for index in 0..total {
            let Some(facet) = listed.facet(index) else {
                continue;
            };
            let text_ok = needle.is_empty()
                || listed
                    .row(index)
                    .is_some_and(|row| row_matches(&row, &needle));
            if !text_ok {
                continue;
            }
            let severity_ok = view.severities.lists(facet.level);
            let producer_ok = view
                .producer
                .as_deref()
                .is_none_or(|producer| producer == facet.producer);
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
                widest_code = widest_code.max(facet.code.chars().count());
                widest_producer = widest_producer.max(facet.producer.chars().count());
                if current == Some(facet.id) {
                    standing = Some(index);
                }
                if !as_reported {
                    admitted.push(index);
                }
            }
        }

        let tally = Tally {
            severities,
            waived,
            producers: producers.into_iter().collect(),
            widest_code,
            widest_producer,
        };
        if as_reported {
            return Self {
                lines: Vec::new(),
                as_reported: Some(total),
                groups: Vec::new(),
                listed: count,
                current: standing,
                tally,
            };
        }

        if let Some(column) = view.sort {
            admitted = ordered(listed, admitted, column, view.descending);
        }
        let (lines, groups) = match view.grouping {
            ProblemGrouping::None => (admitted.into_iter().map(Line::Row).collect(), Vec::new()),
            grouping => group(listed, &admitted, grouping, view),
        };
        Self {
            lines,
            as_reported: None,
            groups,
            listed: count,
            current: standing,
            tally,
        }
    }

    /// Printed lines, group headers included.
    pub(super) fn lines(&self) -> usize {
        self.as_reported.unwrap_or(self.lines.len())
    }

    pub(super) fn line(&self, index: usize) -> Option<Line> {
        match self.as_reported {
            Some(total) if index < total => Some(Line::Row(index)),
            Some(_) => None,
            None => self.lines.get(index).copied(),
        }
    }

    pub(super) fn group(&self, index: usize) -> Option<&Group> {
        self.groups.get(index)
    }

    /// The listing indices this arrangement prints, in print order.
    pub(super) fn rows(&self) -> impl Iterator<Item = usize> + '_ {
        let reported = self.as_reported.unwrap_or(0);
        (0..reported).chain(self.lines.iter().filter_map(|line| match line {
            Line::Row(index) => Some(*index),
            Line::Group(_) => None,
        }))
    }

    /// Which printed line a finding is on, when it is printed at all.
    pub(super) fn line_of(&self, listing_index: usize) -> Option<usize> {
        match self.as_reported {
            Some(total) => (listing_index < total).then_some(listing_index),
            None => self
                .lines
                .iter()
                .position(|line| *line == Line::Row(listing_index)),
        }
    }

    /// The finding `step` places after `from` in print order, or the first one
    /// when nothing is current. Group headers are stepped over and a collapsed
    /// group's rows are not printed, so neither can be landed on.
    pub(super) fn step(&self, from: Option<usize>, step: isize) -> Option<usize> {
        if let Some(total) = self.as_reported {
            // Drawn as reported, a finding's print position is its own index,
            // so stepping is arithmetic rather than a walk of a list that may
            // hold a million rows.
            if total == 0 {
                return None;
            }
            return Some(match from {
                Some(index) if index < total => (index as isize + step)
                    .rem_euclid(total as isize)
                    .unsigned_abs(),
                _ if step >= 0 => 0,
                _ => total - 1,
            });
        }
        let printed = self.rows().count();
        if printed == 0 {
            return None;
        }
        let standing = from.and_then(|index| self.rows().position(|row| row == index));
        let next = match standing {
            Some(position) => (position as isize + step)
                .rem_euclid(printed as isize)
                .unsigned_abs(),
            None if step >= 0 => 0,
            None => printed - 1,
        };
        self.rows().nth(next)
    }
}

/// Whether a row answers the reader's text, which is matched against the rule
/// it is an instance of, the thing it is about and what it says. Not the
/// producer: that has its own control, and a dotted identity matches
/// everything a reader is likely to type.
fn row_matches(
    row: &crate::workbench::app_state::diagnostic_store::listing::DiagnosticRow<'_>,
    needle: &str,
) -> bool {
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
        ProblemColumn::Severity => {
            // Ascending is worst first. It is the only order a reader asks for
            // by clicking a severity column, and the header says which way it
            // is reading in words rather than leaving a triangle to imply it.
            let mut keyed: Vec<(DiagnosticLevel, usize)> = admitted
                .iter()
                .filter_map(|index| listed.facet(*index).map(|facet| (facet.level, *index)))
                .collect();
            keyed.sort_by(|left, right| right.0.cmp(&left.0));
            finish(keyed, descending)
        }
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
            keyed.sort_by(|left, right| left.0.cmp(&right.0));
            finish(keyed, descending)
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
            keyed.sort_by(|left, right| left.0.cmp(&right.0));
            finish(keyed, descending)
        }
    }
}

/// Undecorate a sorted key list, reversing it where the reader asked for the
/// other direction. Reversing the sorted order rather than sorting backwards
/// keeps equal keys in the order the producers reported them either way.
fn finish<K>(keyed: Vec<(K, usize)>, descending: bool) -> Vec<usize> {
    let mut rows: Vec<usize> = keyed.into_iter().map(|(_, index)| index).collect();
    if descending {
        rows.reverse();
    }
    rows
}

/// Gather the admitted rows into groups, keeping each group's rows in the
/// order the arrangement put them in.
fn group(
    listed: &DiagnosticListing<'_>,
    admitted: &[usize],
    grouping: ProblemGrouping,
    view: &ProblemsView,
) -> (Vec<Line>, Vec<Group>) {
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

    let mut lines = Vec::with_capacity(admitted.len() + gathered.len());
    let mut groups = Vec::with_capacity(gathered.len());
    for (label, (counts, rows)) in gathered {
        let collapsed = view.is_collapsed(&label);
        lines.push(Line::Group(groups.len()));
        if !collapsed {
            lines.extend(rows.into_iter().map(Line::Row));
        }
        groups.push(Group {
            label,
            counts,
            collapsed,
        });
    }
    (lines, groups)
}
