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
        // Dataset-native sheets are evidence the bound dataset either carries
        // or it does not, never one of a family's plot modes. No family claims
        // or excludes them; `viewer_availability` is their only gate.
        let Some(document_id) = viewer.viewer_document_id() else {
            return true;
        };
        if viewer_document(document_id).is_some_and(|document| self.includes(document)) {
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
    TwoLinkedPanes,
    SinglePane,
    EngineeringGrid,
    FreeformReviewPage,
}

impl ResultDocumentLayout {
    pub const ALL: [Self; 4] = [
        Self::TwoLinkedPanes,
        Self::SinglePane,
        Self::EngineeringGrid,
        Self::FreeformReviewPage,
    ];

    pub const fn id(self) -> &'static str {
        match self {
            Self::TwoLinkedPanes => "two-linked-panes",
            Self::SinglePane => "single-pane",
            Self::EngineeringGrid => "engineering-grid-2x2",
            Self::FreeformReviewPage => "freeform-review-page",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::TwoLinkedPanes => "Two linked panes",
            Self::SinglePane => "Single pane",
            Self::EngineeringGrid => "2 × 2 engineering sheet",
            Self::FreeformReviewPage => "Freeform review page",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "two-linked-panes" => Some(Self::TwoLinkedPanes),
            "single-pane" => Some(Self::SinglePane),
            "engineering-grid-2x2" => Some(Self::EngineeringGrid),
            "freeform-review-page" => Some(Self::FreeformReviewPage),
            _ => None,
        }
    }

    pub const fn pane_count(self) -> usize {
        match self {
            Self::SinglePane | Self::FreeformReviewPage => 1,
            Self::TwoLinkedPanes => 2,
            Self::EngineeringGrid => 4,
        }
    }

    pub const fn page_layout(self) -> PageLayout {
        match self {
            Self::SinglePane => PageLayout::SinglePane,
            Self::TwoLinkedPanes => PageLayout::Columns,
            Self::EngineeringGrid => PageLayout::Grid { columns: 2 },
            // The review template owns free placement; Rows is its
            // deterministic initial flow before the user moves objects.
            Self::FreeformReviewPage => PageLayout::Rows,
        }
    }

    pub const fn template_id(self) -> &'static str {
        match self {
            Self::FreeformReviewPage => "review-freeform",
            Self::SinglePane | Self::TwoLinkedPanes | Self::EngineeringGrid => "engineering-dark",
        }
    }
}

#[cfg(test)]
mod tests;
