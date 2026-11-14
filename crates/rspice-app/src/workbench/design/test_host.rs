//! What the design feature's own tests run against: the design feature's
//! state, the shared model, the workbench session and the runs.
//!
//! Each field carries the name `AppState` gives the same value, so a test
//! body reads the same against either and only its constructor differs. A
//! test that reads another slot, the session's other parts or the command
//! registry, or draws a frame, runs against the whole application instead.

use rspice_app_state::workbench::workspace::restore_session_model_library_manager;

use crate::diagnostics::LogAnchor;
use crate::properties::PropertyEditorSchema;
use crate::schematic::view::{CanvasFacts, CanvasParts, canvas_facts_lent, canvas_parts_lent};
use crate::state::pdk_config::PdkConfig;
use crate::state::{CellViewRef, SchematicState, SymbolDocument, SymbolEditorMetadata};
use crate::workbench::app::actions::workspace::symbol_document::{
    can_undo_active_symbol_document, redo_active_symbol_document, refuse_unsavable_active_symbol,
    store_active_symbol_document, store_active_symbol_editor_bundle, undo_active_symbol_document,
};
use crate::workbench::app::actions::workspace::{
    NavigationParts, new_schematic_document, sync_active_schematic,
};
use crate::workbench::app_state::diagnostic_store::{BatchStanding, DiagnosticBatch};
use crate::workbench::app_state::{Model, SlotFacts, prompt_text};
use crate::workbench::framework::command_registry::Registry;
use crate::workbench::framework::feature::harness::{fresh_session_code, fresh_session_results};
use crate::workbench::slots::{DesignState, RunsState};
use crate::workbench::state::WorkbenchSession;

/// The design feature's state, the shared model, the workbench session and
/// the runs, under the names `AppState` gives them.
#[derive(Clone)]
pub(crate) struct DesignTestHost {
    pub(crate) design: Box<DesignState>,
    pub(crate) model: Model,
    pub(crate) workbench: Box<WorkbenchSession>,
    pub(crate) runs: Box<RunsState>,
}

impl Default for DesignTestHost {
    /// What a new session starts with, as `AppState::default()` builds these
    /// four: the model over the empty PDK configuration a test session reads,
    /// the design state with the property schema and the model's snap grid,
    /// a new workbench session and new runs.
    fn default() -> Self {
        let model = Model::session_start(
            PdkConfig::default(),
            None,
            restore_session_model_library_manager,
        );
        let mut design = Box::<DesignState>::default();
        design.property_registry = PropertyEditorSchema::new();
        design.schematic_snap.grid_size = model
            .schematic
            .document()
            .document_policy
            .grid_pitch
            .canvas_grid_size();
        Self {
            design,
            model,
            workbench: Box::default(),
            runs: Box::default(),
        }
    }
}

impl DesignTestHost {
    /// `AppState::sync_active_schematic_to_workspace`, the same one line:
    /// [`sync_active_schematic`] over the model and the session.
    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        sync_active_schematic(&mut self.model, &mut self.workbench);
    }

    /// `AppState::open_workspace_view`, the same one line:
    /// [`NavigationParts::open_workspace_view`] over [`Self::navigation`].
    pub(crate) fn open_workspace_view(&mut self, reference: CellViewRef) {
        self.navigation().open_workspace_view(reference);
    }

    /// `AppState::navigation`, the same one line: the design state's
    /// navigation parts over the model, the session and the runs.
    pub(crate) fn navigation(&mut self) -> NavigationParts<'_> {
        self.design
            .navigation_parts(&mut self.model, &mut self.workbench, &self.runs)
    }

    /// `AppState::new_schematic_document`, the same one line.
    pub(crate) fn new_schematic_document(&self) -> SchematicState {
        new_schematic_document(
            &self.model,
            &self.design.schematic_snap,
            self.design.schematic_routing_mode,
        )
    }

    /// `AppState::jump_to_log_anchor`, the same one line.
    pub(crate) fn jump_to_log_anchor(&mut self, anchor: LogAnchor) {
        self.navigation().navigate_to_log_anchor(anchor);
    }

    /// `AppState::store_active_symbol_document`, the same one line.
    pub(crate) fn store_active_symbol_document(
        &mut self,
        document: &SymbolDocument,
    ) -> Result<(), String> {
        store_active_symbol_document(&mut self.design.symbol, &mut self.model, document)
    }

    /// `AppState::store_active_symbol_editor_bundle`, the same one line.
    pub(crate) fn store_active_symbol_editor_bundle(
        &mut self,
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
    ) -> Result<(), String> {
        store_active_symbol_editor_bundle(
            &mut self.design.symbol,
            &mut self.model,
            &mut self.workbench,
            document,
            metadata,
        )
    }

    /// `AppState::refuse_unsavable_active_symbol`, the same one line.
    pub(crate) fn refuse_unsavable_active_symbol(&self) -> Option<String> {
        refuse_unsavable_active_symbol(&self.model)
    }

    /// `AppState::can_undo_active_symbol_document`, the same one line.
    pub(crate) fn can_undo_active_symbol_document(&self) -> bool {
        can_undo_active_symbol_document(&self.design.symbol, &self.model)
    }

    /// `AppState::undo_active_symbol_document`, the same one line.
    pub(crate) fn undo_active_symbol_document(&mut self) -> Result<bool, String> {
        undo_active_symbol_document(&mut self.design.symbol, &mut self.model)
    }

    /// `AppState::redo_active_symbol_document`, the same one line.
    pub(crate) fn redo_active_symbol_document(&mut self) -> Result<bool, String> {
        redo_active_symbol_document(&mut self.design.symbol, &mut self.model)
    }

    /// `AppState::slot_facts`, the same one line: what model code reads of
    /// the runs and the design state.
    pub(crate) fn slot_facts(&self) -> SlotFacts {
        self.runs
            .slot_facts(self.design.schematic_visibility.review_markers)
    }

    /// `AppState::diagnostic_batch_standing`, the same one line.
    pub(crate) fn diagnostic_batch_standing(&self, batch: &DiagnosticBatch) -> BatchStanding {
        self.model
            .diagnostic_batch_standing(batch, self.slot_facts())
    }

    /// `AppState::prompt_text`, the same one line.
    pub(crate) fn prompt_text(&self) -> Option<&str> {
        prompt_text(&self.workbench)
    }
}

/// The canvas's parts of the host, as `canvas_parts` takes them of the
/// whole application. The host holds no other feature's slot, so no foreign
/// dialog is open: both modal facts are lent as closed, and a dialog of the
/// design feature's own or one the surface asked for still counts. The
/// results workspace and the code feature answer as a fresh session's do.
pub(crate) fn canvas_parts(host: &mut DesignTestHost) -> CanvasParts<'_> {
    canvas_parts_lent(
        &mut host.design,
        &mut host.model,
        &mut host.workbench,
        &mut host.runs,
        fresh_session_results(),
        fresh_session_code(),
        no_commands(),
        false,
        false,
    )
}

/// The canvas's facts from the host, as `canvas_facts` reads them from the
/// whole application, with the results workspace and the code feature
/// answering as a fresh session's do.
pub(crate) fn canvas_facts(host: &DesignTestHost) -> CanvasFacts<'_> {
    canvas_facts_lent(
        &host.design,
        &host.model,
        &host.workbench,
        &host.runs,
        fresh_session_results(),
        fresh_session_code(),
        no_commands(),
    )
}

/// The commands the host's parts carry: none. A shortcut label reads the
/// application's registry, so a test that reads one runs through the whole
/// application in feature_tests/design.
fn no_commands() -> &'static Registry {
    static NONE: std::sync::LazyLock<Registry> =
        std::sync::LazyLock::new(|| Registry::new(&[], Vec::new(), &[]));
    &NONE
}

/// The shared model, as `AppState` lends it.
impl AsRef<Model> for DesignTestHost {
    fn as_ref(&self) -> &Model {
        &self.model
    }
}
