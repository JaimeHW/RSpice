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
    /// Whether the detail region states the rule's own words.
    pub(in crate::workbench) explain: bool,
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
            explain: false,
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

    fn read_narrowly(state: &mut crate::workbench::app_state::AppState) {
        let view = &mut state.problems_view;
        view.text = "ground".to_owned();
        view.producer = Some("rspice.design.rules".to_owned());
        view.waived = false;
        view.severities.toggle(DiagnosticLevel::Hint);
        view.order_by(ProblemColumn::Code);
        view.grouping = ProblemGrouping::Code;
        view.collapse("DRC-FLOATING-NODE", true);
        view.explain = true;
    }

    /// Closing the project, or replacing it with another, forgets how the
    /// last one's findings were being read: the next project's findings came
    /// from somewhere else, and a filter carried over would hide conditions
    /// nobody there chose to hide.
    #[test]
    fn closing_or_replacing_the_project_forgets_the_reading_position() {
        use crate::workbench::workflows::project_workflow::{
            close_project_discard, create_new_project,
        };

        let mut state = crate::workbench::app_state::AppState::default();
        read_narrowly(&mut state);
        assert!(close_project_discard(&mut state));
        assert_eq!(state.problems_view, ProblemsView::default());

        read_narrowly(&mut state);
        create_new_project(&mut state);
        assert_eq!(state.problems_view, ProblemsView::default());
    }
}
