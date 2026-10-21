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

/// Command palette (Ctrl+K) state.
#[derive(Debug, Clone, Default)]
pub struct CommandPaletteState {
    /// Whether the palette is showing.
    pub open: bool,
    /// Filter text.
    pub query: String,
    /// Index into the filtered list.
    pub selected: usize,
    /// Request keyboard focus on the next frame (set when opened).
    pub want_focus: bool,
    /// Optional reviewed scope selected by a task-focused menu entry. This is
    /// consumed when the palette opens; ordinary Ctrl+K starts at All.
    pub(crate) initial_scope: Option<String>,
    /// Commands run from the palette, newest first — leads the empty-query
    /// list under a RECENT header. Survives close/reopen, capped at five.
    pub(crate) recent: Vec<crate::workbench::commands::vocabulary::Command>,
    /// Whether the palette owns the canonical `?surface=command-palette`
    /// route. Contextual test and embedded hosts may still render the palette
    /// without changing application navigation.
    pub(crate) route_owned: bool,
}

impl CommandPaletteState {
    /// Open fresh (empty query, first row selected, focus requested).
    pub fn open(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
        self.want_focus = true;
        self.initial_scope = None;
    }

    pub(crate) fn open_routed(&mut self) {
        self.open();
        self.route_owned = true;
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
        self.route_owned = false;
    }

    pub(crate) fn open_in_scope(&mut self, scope: &str) {
        self.open();
        self.initial_scope = Some(scope.to_owned());
    }
}

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
pub(crate) use sessions::{
    DatasetExportPicker, HelpCenterPage, NewWindowInitialContent, RunDetailsDialogState,
    WindowLayoutChoice, WindowSessionPage, WindowWorkflow,
};
