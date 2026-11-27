//! The page layouts a new result document starts with.

use crate::visualization_document::PageLayout;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultDocumentLayout {
    SinglePane,
    TwoLinkedPanes,
    EngineeringGrid,
}

impl ResultDocumentLayout {
    pub const ALL: [Self; 3] = [
        Self::SinglePane,
        Self::TwoLinkedPanes,
        Self::EngineeringGrid,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::SinglePane => "One pane",
            Self::TwoLinkedPanes => "Two panes, linked X",
            Self::EngineeringGrid => "Four panes, 2 \u{d7} 2",
        }
    }

    pub const fn pane_count(self) -> usize {
        match self {
            Self::SinglePane => 1,
            Self::TwoLinkedPanes => 2,
            Self::EngineeringGrid => 4,
        }
    }

    /// Where the pane at `index` sits on the page, in the reader's words,
    /// when the page has more than one: panes are placed in reading order.
    pub const fn pane_place(self, index: usize) -> Option<&'static str> {
        match (self, index) {
            (Self::TwoLinkedPanes, 0) => Some("Top"),
            (Self::TwoLinkedPanes, 1) => Some("Bottom"),
            (Self::EngineeringGrid, 0) => Some("Top left"),
            (Self::EngineeringGrid, 1) => Some("Top right"),
            (Self::EngineeringGrid, 2) => Some("Bottom left"),
            (Self::EngineeringGrid, 3) => Some("Bottom right"),
            _ => None,
        }
    }

    /// Two panes that share an X stack, so the reader reads one X down both.
    pub const fn page_layout(self) -> PageLayout {
        match self {
            Self::SinglePane => PageLayout::SinglePane,
            Self::TwoLinkedPanes => PageLayout::Rows,
            Self::EngineeringGrid => PageLayout::Grid { columns: 2 },
        }
    }
}
