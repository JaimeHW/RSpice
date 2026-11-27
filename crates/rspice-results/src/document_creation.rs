//! Result document families and initial page layouts.

use crate::result_presentation::ResultViewer;
use crate::viewer_catalog::{
    ResultCreationFamilyDefinition, ViewerDocumentDefinition, result_creation_family,
    viewer_document,
};
use crate::visualization_document::PageLayout;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultDocumentFamily {
    WaveformWorksheet,
    FrequencyAndStability,
    RfAndNetwork,
    StatisticsAndYield,
    DigitalAndAmsEvents,
    VerificationAndOptimization,
    FieldsAndPhysical,
    Photonics,
    ReportPage,
}

impl ResultDocumentFamily {
    pub const ALL: [Self; 9] = [
        Self::WaveformWorksheet,
        Self::FrequencyAndStability,
        Self::RfAndNetwork,
        Self::StatisticsAndYield,
        Self::DigitalAndAmsEvents,
        Self::VerificationAndOptimization,
        Self::FieldsAndPhysical,
        Self::Photonics,
        Self::ReportPage,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::WaveformWorksheet => "waveform-worksheet",
            Self::FrequencyAndStability => "frequency-stability",
            Self::RfAndNetwork => "rf-network",
            Self::StatisticsAndYield => "statistics-yield",
            Self::DigitalAndAmsEvents => "digital-ams-events",
            Self::VerificationAndOptimization => "verification-optimization",
            Self::FieldsAndPhysical => "fields-physical",
            Self::Photonics => "photonics",
            Self::ReportPage => "report-page",
        }
    }

    fn definition(self) -> &'static ResultCreationFamilyDefinition {
        result_creation_family(self.id()).expect("every Rust family must exist in the contract")
    }

    pub fn label(self) -> &'static str {
        self.definition().label
    }

    pub fn description(self) -> &'static str {
        self.definition().description
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|family| family.id() == id)
    }

    /// Resolve the family a persistent page belongs to. Pages are titled with
    /// the family label at creation; a page the user has renamed, or one
    /// imported from another build, resolves to `None` and is scoped by its own
    /// retained panes instead.
    pub fn from_label(label: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|family| family.label() == label)
    }

    /// Which workspace sheets this family's docbar offers.
    ///
    /// Every viewer document [`Self::includes`] admits has to be offered here:
    /// that is the same list the Create dialog binds a new document's first
    /// pane from, so a docbar refusing it would strand the document RSpice had
    /// just built with no reachable sheet. What follows are quick modes — they
    /// read the bound dataset through a different sheet without introducing a
    /// pane type the family does not compose.
    pub fn offers_sheet(self, viewer: ResultViewer) -> bool {
        if viewer_document(viewer.viewer_document_id())
            .is_some_and(|document| self.includes(document))
        {
            return true;
        }
        match self {
            // Exact samples and the scalar DC gains behind them are how a
            // waveform review is checked; neither adds a pane to the sheet.
            Self::WaveformWorksheet => {
                matches!(viewer, ResultViewer::TransferFunction | ResultViewer::Table)
            }
            _ => false,
        }
    }

    pub fn includes(self, viewer: &ViewerDocumentDefinition) -> bool {
        // A report page can embed any compatible viewer; the canonical family
        // intentionally has no fixed viewer list. Every other family consumes
        // exact generated membership.
        self == Self::ReportPage || self.definition().viewer_ids.contains(&viewer.id)
    }
}

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

    /// Two panes that share an X stack, so the reader reads one X down both.
    pub const fn page_layout(self) -> PageLayout {
        match self {
            Self::SinglePane => PageLayout::SinglePane,
            Self::TwoLinkedPanes => PageLayout::Rows,
            Self::EngineeringGrid => PageLayout::Grid { columns: 2 },
        }
    }
}

#[cfg(test)]
mod tests;
