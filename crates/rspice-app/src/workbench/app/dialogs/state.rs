//! App Dialog State
//!
//! Modal/dialog payload used by `AppState`. Analysis configuration lives
//! in `SimSetupState`, not here.

use crate::workbench::app::{
    ConfigurationSetsDialogState, ConfirmationDialogState, ConnectivityManagerDialogState,
    CreateModelBoundSymbolDialogState, DesignManagementDialogState,
    DesignReviewCommentsDialogState, ProjectReviewDialogState, ProjectRevisionHistoryDialogState,
    SelectionBulkEditDialogState, SymbolImportDialogState, SymbolParameterFormDialogState,
};

mod reviews;

pub(crate) use reviews::{
    PlanRemovalConsequence, PlanRemovalRefusal, PlanRemovalReview, PlanRemovalTarget,
    PlanRemovalTone,
};

/// Governed snap-spacing choices shown by the upgraded schematic toolbar
/// transaction. `Free` disables snapping without replacing the document's
/// retained pitch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum GridSnapSpacingChoice {
    #[default]
    Free,
    Mil25,
    Mil50,
    Metric,
}

impl GridSnapSpacingChoice {
    pub(crate) const ALL: [Self; 4] = [Self::Free, Self::Mil25, Self::Mil50, Self::Metric];

    pub(crate) fn label(self) -> String {
        self.pitch().map_or_else(
            || "Free".to_owned(),
            crate::schematic::view::drawing_sheet::grid_pitch_label,
        )
    }

    pub(crate) const fn pitch(self) -> Option<crate::state::SchematicGridPitch> {
        match self {
            Self::Free => None,
            Self::Mil25 => Some(crate::state::SchematicGridPitch::Mil25),
            Self::Mil50 => Some(crate::state::SchematicGridPitch::Mil50),
            Self::Metric => Some(crate::state::SchematicGridPitch::Metric),
        }
    }

    pub(crate) const fn from_pitch(pitch: crate::state::SchematicGridPitch) -> Self {
        match pitch {
            crate::state::SchematicGridPitch::Mil25 => Self::Mil25,
            crate::state::SchematicGridPitch::Mil50 => Self::Mil50,
            crate::state::SchematicGridPitch::Metric => Self::Metric,
        }
    }
}

mod sessions;

pub use sessions::DialogState;
pub(crate) use sessions::{DatasetExportPicker, HelpCenterPage, RunDetailsDialogState};
