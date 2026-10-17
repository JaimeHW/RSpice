//! Where the reader is standing in the list of conditions.
//!
//! The Problems grid's order, its filters and its groups are a reading
//! position over the canonical store, not evidence about the design. They live
//! beside the store rather than inside it because the store decides what is
//! true and this decides what is shown — and they live in the session rather
//! than in the page because a page that keeps its own state cannot be tested
//! without being drawn.
//!
//! Nothing here is persisted. A filter is how a reader is reading the findings
//! in front of them this session; restoring it into a session whose findings
//! came from somewhere else would hide conditions nobody chose to hide.

use std::collections::BTreeSet;

use super::diagnostic_store::DiagnosticLevel;

/// One column of the grid: what it says, and what ordering by it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::workbench) enum ProblemColumn {
    Severity,
    Code,
    Subject,
    Producer,
    Location,
    Message,
}

impl ProblemColumn {
    /// The columns in the order the grid prints them.
    pub(in crate::workbench) const ALL: [Self; 6] = [
        Self::Severity,
        Self::Code,
        Self::Subject,
        Self::Producer,
        Self::Location,
        Self::Message,
    ];

    pub(in crate::workbench) const fn label(self) -> &'static str {
        match self {
            Self::Severity => "Severity",
            Self::Code => "Rule",
            Self::Subject => "Subject",
            Self::Producer => "Producer",
            Self::Location => "Location",
            Self::Message => "Message",
        }
    }
}

/// What the rows are grouped by, where they are grouped at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::workbench) enum ProblemGrouping {
    #[default]
    None,
    Producer,
    Code,
    Subject,
}

impl ProblemGrouping {
    pub(in crate::workbench) const ALL: [Self; 4] =
        [Self::None, Self::Producer, Self::Code, Self::Subject];

    pub(in crate::workbench) const fn label(self) -> &'static str {
        match self {
            Self::None => "No grouping",
            Self::Producer => "By producer",
            Self::Code => "By rule",
            Self::Subject => "By subject",
        }
    }

    pub(in crate::workbench) const fn is_grouped(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Which severities the grid lists.
///
/// A set rather than a minimum rung: a reader chasing one warning does not
/// want the four errors above it, and a severity ladder as a slider cannot
/// express that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::workbench) struct SeverityFilter {
    listed: [bool; DiagnosticLevel::ALL.len()],
}

impl Default for SeverityFilter {
    fn default() -> Self {
        Self {
            listed: [true; DiagnosticLevel::ALL.len()],
        }
    }
}

impl SeverityFilter {
    pub(in crate::workbench) const fn lists(self, level: DiagnosticLevel) -> bool {
        self.listed[level.rung()]
    }

    pub(in crate::workbench) fn toggle(&mut self, level: DiagnosticLevel) {
        let rung = level.rung();
        self.listed[rung] = !self.listed[rung];
    }
}

/// The reading position itself.
///
/// Visible to the crate because [`super::AppState`] holds one and the session
/// aggregate is built by name outside the shell; its fields and vocabulary are
/// the shell's own, and nothing below it reads them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProblemsView {
    /// The column the rows are ordered by. `None` keeps the order the
    /// producers reported in, which is also the order the badge counts them
    /// in and the only order that costs nothing to produce.
    pub(in crate::workbench) sort: Option<ProblemColumn>,
    pub(in crate::workbench) descending: bool,
    pub(in crate::workbench) severities: SeverityFilter,
    /// The producer the grid is narrowed to, by its exact identity. A partial
    /// identity is one a reader cannot check, so this is never a prefix.
    pub(in crate::workbench) producer: Option<String>,
    /// Text matched against the rule code, the subject and the message.
    pub(in crate::workbench) text: String,
    /// Whether findings a waiver covers are listed. On by default: a waiver is
    /// a decision somebody recorded, and a page that hides those decisions by
    /// default is a page that cannot be audited.
    pub(in crate::workbench) waived: bool,
    pub(in crate::workbench) grouping: ProblemGrouping,
    /// The groups the reader has collapsed, by the value they group on.
    pub(in crate::workbench) collapsed: BTreeSet<String>,
    /// The group header the keyboard stands on, by the value it groups on.
    ///
    /// Only while no finding is current: the current finding is the store's,
    /// shared with the canvas tour, and a header is not a finding. Standing on
    /// a header is how a collapsed group is reached and opened again from the
    /// keyboard.
    pub(in crate::workbench) current_group: Option<String>,
    /// Whether the detail region states the rule's own words.
    pub(in crate::workbench) explain: bool,
    /// The waiver being written, where the reader asked to waive a finding
    /// and has not yet recorded or abandoned it.
    pub(in crate::workbench) waiving: Option<WaiverDraft>,
}

/// A waiver being written in the detail region: the finding it is for and
/// the reason typed so far.
///
/// It belongs to one finding. Standing on another lets it go, so a reason
/// typed for one finding is never recorded against the next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) struct WaiverDraft {
    pub(in crate::workbench) finding: uuid::Uuid,
    pub(in crate::workbench) reason: String,
    /// Whether the reason field has yet to take the keyboard: true on the
    /// frame the draft opens, so the reader types straight into it.
    pub(in crate::workbench) focus: bool,
}

impl WaiverDraft {
    pub(in crate::workbench) const fn new(finding: uuid::Uuid) -> Self {
        Self {
            finding,
            reason: String::new(),
            focus: true,
        }
    }
}

/// The reading position a session starts from: every severity, waivers
/// included, nothing grouped, in the order the producers reported.
///
/// Written out rather than derived, because a derived default would list no
/// waived findings — and a session aggregate built field by field elsewhere in
/// the crate would then hide recorded decisions that the session the reader is
/// in shows.
impl Default for ProblemsView {
    fn default() -> Self {
        Self {
            sort: None,
            descending: false,
            severities: SeverityFilter::default(),
            producer: None,
            text: String::new(),
            waived: true,
            grouping: ProblemGrouping::None,
            collapsed: BTreeSet::new(),
            current_group: None,
            explain: false,
            waiving: None,
        }
    }
}

impl ProblemsView {
    /// Order by `column`, or reverse the order when it is already the key.
    ///
    /// Clicking the sorted column again is how every table in this product
    /// reverses, and the first click on a new column must not inherit the
    /// previous column's direction.
    pub(in crate::workbench) fn order_by(&mut self, column: ProblemColumn) {
        if self.sort == Some(column) {
            self.descending = !self.descending;
        } else {
            self.sort = Some(column);
            self.descending = false;
        }
    }

    /// Clear every filter, so every finding is listed — what the strip's
    /// "List everything" does, and what a surface that sends the reader here
    /// to read particular findings does first, so a filter left from earlier
    /// cannot hide them. The order, the grouping, which groups are collapsed
    /// and the header the keyboard stands on are how the list is read rather
    /// than what it holds, and a collapsed group still counts its findings in
    /// its header, so they stay.
    ///
    /// Destructured, so a filter added to the position has to be cleared here
    /// or said not to be one.
    pub(in crate::workbench) fn clear_filters(&mut self) {
        let Self {
            sort: _,
            descending: _,
            severities,
            producer,
            text,
            waived,
            grouping: _,
            collapsed: _,
            current_group: _,
            explain: _,
            waiving: _,
        } = self;
        *severities = SeverityFilter::default();
        *producer = None;
        text.clear();
        *waived = true;
    }

    /// Whether anything is narrowing the list.
    pub(in crate::workbench) fn is_filtered(&self) -> bool {
        !self.text.is_empty()
            || self.producer.is_some()
            || !self.waived
            || DiagnosticLevel::ALL
                .iter()
                .any(|level| !self.severities.lists(*level))
    }

    /// Whether the rows can be drawn straight from the listing, without an
    /// order of their own being built first.
    pub(in crate::workbench) fn lists_everything_as_reported(&self) -> bool {
        self.sort.is_none() && !self.grouping.is_grouped() && !self.is_filtered()
    }

    /// Whether arranging the rows under this position reads what a row says
    /// — its subject, location or message — rather than only what its facet
    /// states. Those are the arrangements that cost a spelled-out row each.
    pub(in crate::workbench) fn reads_row_text(&self) -> bool {
        !self.text.is_empty()
            || self.grouping == ProblemGrouping::Subject
            || matches!(
                self.sort,
                Some(ProblemColumn::Subject | ProblemColumn::Location | ProblemColumn::Message)
            )
    }

    /// Whether two positions list the same rows, in the same order and the
    /// same groups. Everything but where the keyboard stands and the detail
    /// region's own state.
    ///
    /// Destructured rather than compared field by field, so a field added
    /// here has to be placed on one side of that line or the other.
    pub(in crate::workbench) fn arranges_like(&self, other: &Self) -> bool {
        let Self {
            sort,
            descending,
            severities,
            producer,
            text,
            waived,
            grouping,
            collapsed,
            current_group: _,
            explain: _,
            waiving: _,
        } = self;
        *sort == other.sort
            && *descending == other.descending
            && *severities == other.severities
            && *producer == other.producer
            && *text == other.text
            && *waived == other.waived
            && *grouping == other.grouping
            && *collapsed == other.collapsed
    }

    pub(in crate::workbench) fn collapse(&mut self, group: &str, collapsed: bool) {
        if collapsed {
            self.collapsed.insert(group.to_owned());
        } else {
            self.collapsed.remove(group);
        }
    }

    pub(in crate::workbench) fn is_collapsed(&self, group: &str) -> bool {
        self.collapsed.contains(group)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_session_starts_listing_everything_in_the_order_it_was_reported() {
        let view = ProblemsView::default();
        assert!(view.lists_everything_as_reported());
        assert!(!view.is_filtered());
        for level in DiagnosticLevel::ALL {
            assert!(view.severities.lists(level), "{level:?} must be listed");
        }
        assert!(view.waived, "a recorded waiver is listed, not hidden");
    }

    /// The second click on a column reverses it; the first click on another
    /// column does not inherit the direction the previous one was read in.
    #[test]
    fn ordering_by_the_sorted_column_reverses_it() {
        let mut view = ProblemsView::default();
        view.order_by(ProblemColumn::Severity);
        assert_eq!(view.sort, Some(ProblemColumn::Severity));
        assert!(!view.descending);
        view.order_by(ProblemColumn::Severity);
        assert!(view.descending);
        view.order_by(ProblemColumn::Code);
        assert_eq!(view.sort, Some(ProblemColumn::Code));
        assert!(!view.descending, "a new column starts ascending");
    }

    #[test]
    fn narrowing_any_one_thing_counts_as_filtered() {
        let mut severity = ProblemsView::default();
        severity.severities.toggle(DiagnosticLevel::Hint);
        assert!(severity.is_filtered());
        assert!(!severity.lists_everything_as_reported());

        let mut producer = ProblemsView::default();
        producer.producer = Some("rspice.design.rules".to_owned());
        assert!(producer.is_filtered());

        let mut text = ProblemsView::default();
        text.text = "ground".to_owned();
        assert!(text.is_filtered());

        let mut waived = ProblemsView::default();
        waived.waived = false;
        assert!(waived.is_filtered());

        let mut grouped = ProblemsView::default();
        grouped.grouping = ProblemGrouping::Producer;
        assert!(
            !grouped.is_filtered(),
            "grouping shows the same rows in groups; it narrows nothing"
        );
        assert!(!grouped.lists_everything_as_reported());
    }

    /// Clearing the filters lists everything and keeps how it is read.
    #[test]
    fn clearing_the_filters_lists_everything_and_keeps_the_order_and_groups() {
        let mut view = ProblemsView::default();
        view.text = "ground".to_owned();
        view.producer = Some("rspice.design.rules".to_owned());
        view.waived = false;
        view.severities.toggle(DiagnosticLevel::Hint);
        view.order_by(ProblemColumn::Code);
        view.order_by(ProblemColumn::Code);
        view.grouping = ProblemGrouping::Code;
        view.collapse("DRC-FLOATING-NODE", true);
        view.explain = true;

        view.clear_filters();
        assert!(!view.is_filtered());
        assert_eq!(
            view,
            ProblemsView {
                sort: Some(ProblemColumn::Code),
                descending: true,
                grouping: ProblemGrouping::Code,
                collapsed: ["DRC-FLOATING-NODE".to_owned()].into(),
                explain: true,
                ..ProblemsView::default()
            },
        );
    }
}
