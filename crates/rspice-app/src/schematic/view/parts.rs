//! What the schematic canvas reads and writes while it takes a gesture.
//!
//! The canvas answers a pointer or key over the design feature's state, the
//! model, the session, the runs whose preflight a probe invalidates, and the
//! results workspace that a probe and a requirement link write into. Whether
//! a modal dialog was open is lent as it stood when the parts were taken: the
//! canvas opens nothing modal before it reads either fact.
//!
//! What only asks or paints, such as which rows the canvas menu offers or
//! the scene, reads [`CanvasFacts`]: the same parts, shared.

use crate::state::{BusDeclaration, BusParseError, CellViewRef, Point};
use crate::workbench::{
    app::{
        SymbolInPlaceRefusal, deny_read_only_edit, hierarchy_master_for_component,
        place_floating_paste, reconcile_conductor_route, selected_hierarchy_master,
        selected_instance_veriloga_source, selected_symbol_in_place_target, start_canvas_bus,
        start_canvas_wire, sync_active_schematic, veriloga_master_of_component,
        veriloga_source_for_component,
    },
    app_state::{AppState, DragType, Model, model_design_view},
    commands::vocabulary::command_registry,
    documents::result_document::ResultsParts,
    slots::{DesignState, PlatformState, ResultsState, RunsState},
    state::WorkbenchSession,
};
use rspice_design::drc::{DrcLocation, DrcSeverity};
use rspice_schematic_editor::view::design_view::DesignView;
use rspice_schematic_editor::view::selection_drag::ActiveSelectionDrag;

/// The design slot, the model, the session, the runs, the results slot and
/// the platform state the canvas reads and writes, beside the two modal
/// facts it reads.
pub(crate) struct CanvasParts<'a> {
    pub(crate) design: &'a mut DesignState,
    pub(crate) model: &'a mut Model,
    pub(crate) workbench: &'a mut WorkbenchSession,
    pub(crate) runs: &'a mut RunsState,
    pub(crate) results: &'a mut ResultsState,
    pub(crate) platform: &'a PlatformState,
    application_modal: bool,
    dialog_modal: bool,
}

/// The canvas's parts of the application's state, with its modal facts as
/// they stand now.
pub(crate) fn canvas_parts(state: &mut AppState) -> CanvasParts<'_> {
    let application_modal = state.application_modal_open();
    let dialog_modal = state.dialog_modal_open();
    CanvasParts {
        design: &mut state.design,
        model: &mut state.model,
        workbench: &mut state.workbench,
        runs: &mut state.runs,
        results: &mut state.results,
        platform: &state.platform,
        application_modal,
        dialog_modal,
    }
}

impl AsRef<Model> for CanvasParts<'_> {
    fn as_ref(&self) -> &Model {
        self.model
    }
}

/// What the canvas's answers and its paint read: the design slot, the
/// model, the runs (whose slot facts say which check results are current)
/// and the results slot (the selected trace and the cursor a transient is
/// annotated at).
#[derive(Clone, Copy)]
pub(crate) struct CanvasFacts<'a> {
    pub(crate) design: &'a DesignState,
    pub(crate) model: &'a Model,
    pub(crate) runs: &'a RunsState,
    pub(crate) results: &'a ResultsState,
}

/// The canvas's facts from the application's state.
pub(crate) fn canvas_facts(state: &AppState) -> CanvasFacts<'_> {
    CanvasFacts {
        design: &state.design,
        model: &state.model,
        runs: &state.runs,
        results: &state.results,
    }
}

impl AsRef<Model> for CanvasFacts<'_> {
    fn as_ref(&self) -> &Model {
        self.model
    }
}

impl<'a> CanvasFacts<'a> {
    pub(crate) fn schematic_edit_read_only(&self) -> bool {
        self.model.schematic_edit_read_only()
    }

    pub(crate) fn active_view_read_only(&self) -> bool {
        self.model.active_view_read_only()
    }

    /// Whether a selection drag is under way.
    pub(crate) fn schematic_drag_in_progress(&self) -> bool {
        self.design.interaction.drag.in_progress(self.model)
    }

    /// The design the canvas shows: the model's, with the review markers
    /// the design slot asks for.
    pub(crate) fn design_view(&self) -> DesignView<'a> {
        model_design_view(self.model, self.design.schematic_visibility.review_markers)
    }

    pub(crate) fn veriloga_master_of_component(&self, component_id: u64) -> Option<CellViewRef> {
        veriloga_master_of_component(self.model, component_id)
    }

    pub(crate) fn selected_hierarchy_master(&self) -> Option<(String, CellViewRef)> {
        selected_hierarchy_master(self.model)
    }

    pub(crate) fn selected_veriloga_source(&self) -> Option<CellViewRef> {
        selected_instance_veriloga_source(self.model)
    }

    pub(crate) fn selected_symbol_in_place_target(
        &self,
    ) -> Result<(u64, CellViewRef), SymbolInPlaceRefusal> {
        selected_symbol_in_place_target(self.model)
    }

    /// Why the selected instance cannot be shown in the generated deck, or
    /// `None` when it can.
    pub(crate) fn selected_instance_netlist_block(&self) -> Option<&'static str> {
        self.model.selected_instance_generated_line().err()
    }

    /// The current finding's other objects in `cell_view`, as the runs'
    /// slot facts let the check store answer.
    pub(crate) fn current_finding_related_marks(
        &self,
        cell_view: &CellViewRef,
    ) -> Option<(DrcSeverity, Vec<DrcLocation>)> {
        self.model.current_finding_related_marks(
            cell_view,
            self.runs
                .slot_facts(self.design.schematic_visibility.review_markers),
        )
    }
}

impl CanvasParts<'_> {
    /// What the canvas's answers read, from these parts.
    pub(crate) fn facts(&self) -> CanvasFacts<'_> {
        CanvasFacts {
            design: self.design,
            model: self.model,
            runs: self.runs,
            results: self.results,
        }
    }

    /// Whether anything that holds the application modal was open.
    pub(crate) fn application_modal_open(&self) -> bool {
        self.application_modal
    }

    /// Whether anything the dialog gate counts was open.
    pub(crate) fn dialog_modal_open(&self) -> bool {
        self.dialog_modal
    }

    /// A refusal or transient feedback on the status bar.
    pub(crate) fn prompt(&mut self, text: impl Into<String>) {
        self.workbench.prompt(text);
    }

    pub(crate) fn schematic_edit_read_only(&self) -> bool {
        self.model.schematic_edit_read_only()
    }

    pub(crate) fn active_view_read_only(&self) -> bool {
        self.model.active_view_read_only()
    }

    /// [`sync_active_schematic`] over these parts.
    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        sync_active_schematic(self.model, self.workbench);
    }

    /// [`deny_read_only_edit`] over these parts.
    pub(crate) fn deny_read_only_edit(&mut self) -> bool {
        deny_read_only_edit(self.model, self.workbench)
    }

    /// The design the canvas shows: the model's, with the review markers
    /// the design slot asks for.
    pub(crate) fn design_view(&self) -> DesignView<'_> {
        model_design_view(self.model, self.design.schematic_visibility.review_markers)
    }

    pub(crate) fn hierarchy_master_for_component(
        &self,
        component_id: u64,
    ) -> Option<(String, CellViewRef)> {
        hierarchy_master_for_component(self.model, component_id)
    }

    pub(crate) fn veriloga_source_for_component(&self, component_id: u64) -> Option<CellViewRef> {
        veriloga_source_for_component(self.model, component_id)
    }

    pub(crate) fn open_selected_instance_master(&mut self) {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .open_selected_instance_master();
    }

    pub(crate) fn open_veriloga_source_for_component(&mut self, component_id: u64) -> bool {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .open_veriloga_source_for_component(component_id)
    }

    pub(crate) fn edit_selected_symbol_in_place(&mut self) -> bool {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .edit_selected_symbol_in_place()
    }

    pub(crate) fn ascend_workspace_level(&mut self) {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .ascend_workspace_level();
    }

    /// The parts the results workspace's commands write.
    pub(crate) fn results_parts(&mut self) -> ResultsParts<'_> {
        self.results.parts(
            self.model,
            self.workbench,
            self.platform,
            &mut self.runs.preflight,
            command_registry(),
        )
    }

    /// Land the floating copy where it sits now.
    pub(crate) fn place_floating_paste(&mut self) -> bool {
        place_floating_paste(&mut self.design.floating_paste, self.model, self.workbench)
    }

    pub(crate) fn prepare_schematic_drag(&mut self) {
        self.design.interaction.drag.prepare(self.model);
    }

    pub(crate) fn begin_schematic_drag(
        &mut self,
        position: (i32, i32),
        kind: DragType,
        ctx: &egui::Context,
    ) -> bool {
        self.design
            .interaction
            .drag
            .begin(self.model, self.workbench, position, kind, ctx)
    }

    pub(crate) fn schematic_drag_snapshot(
        &self,
        ctx: &egui::Context,
    ) -> Option<ActiveSelectionDrag> {
        self.design
            .interaction
            .drag
            .snapshot(self.model, self.workbench, ctx)
    }

    pub(crate) fn reconcile_schematic_drag(
        &mut self,
        ctx: &egui::Context,
        after_canvas: bool,
    ) -> bool {
        self.design
            .interaction
            .drag
            .reconcile(self.model, self.workbench, ctx, after_canvas)
    }

    pub(crate) fn reconcile_conductor_route(&mut self, ctx: &egui::Context) -> bool {
        reconcile_conductor_route(self.model, self.workbench, ctx)
    }

    pub(crate) fn start_canvas_wire(&mut self, ctx: &egui::Context, position: Point) {
        start_canvas_wire(self.model, self.workbench, ctx, position);
    }

    pub(crate) fn start_canvas_bus(
        &mut self,
        ctx: &egui::Context,
        position: Point,
        declaration: Option<BusDeclaration>,
    ) -> Result<(), BusParseError> {
        start_canvas_bus(self.model, self.workbench, ctx, position, declaration)
    }
}
