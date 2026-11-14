//! What the schematic canvas reads and writes while it takes a gesture.
//!
//! The canvas answers a pointer or key over the design feature's state, the
//! model, the session and the runs whose preflight a probe invalidates. It
//! reads the results workspace. What it asks of another feature's workspace
//! (a probe's trace, the operating-point hop, a requirement link, and the
//! menu's Verilog-A device form, model-bound symbol and netlist card) waits
//! in the session's outbox until the shell answers it, once the surface has
//! drawn. Whether a modal dialog was open is lent as it stood when the parts
//! were taken. A dialog the surface asked for counts as open from the
//! moment it asked, though the shell opens it only once the surface has
//! drawn: [`canvas_application_modal_open`] is that one fact.
//!
//! A page of the design surface draws over a [`CanvasFrame`]: the same
//! parts for the whole pass, with the modal facts lent as it began, from
//! which each step takes its own parts.
//!
//! What only asks or paints, such as which rows the canvas menu offers or
//! the scene, reads [`CanvasFacts`]: the same parts, shared.
//!
//! The symbol canvas, in its own view or in place on a parent sheet, takes
//! its gestures over the same parts.

use crate::state::{
    BusDeclaration, BusParseError, CellViewRef, Point, PortSpec, SymbolDocument,
    SymbolEditorMetadata,
};
use crate::workbench::{
    app::{
        SymbolInPlace, SymbolInPlaceRefusal, active_symbol_save_checks, deny_read_only_edit,
        finish_symbol_parent_edit, hierarchy_master_for_component, in_place_placement,
        in_place_session, in_place_session_mut, place_floating_paste,
        publish_active_symbol_revision, reconcile_conductor_route, record_symbol_edit,
        selected_hierarchy_master, selected_instance_veriloga_source,
        selected_symbol_in_place_target, start_canvas_bus, start_canvas_wire,
        store_symbol_canvas_frame, symbol_editor_request_source, sync_active_schematic,
        veriloga_master_of_component, veriloga_source_for_component,
    },
    app_state::{AppState, DragType, Model, model_design_view},
    design::{DesignFeature, own_modal_dialog_open},
    framework::{feature::Cx, ports::ResultsPort},
    slots::{DesignState, RunsState},
    state::WorkbenchSession,
};
use rspice_design::drc::{DrcLocation, DrcSeverity};
use rspice_design::symbol::publication::SymbolSaveCheck;
use rspice_results_ui::selection::SelectedResultTrace;
use rspice_schematic_editor::symbol_editor::interaction::SymbolRequestSource;
use rspice_schematic_editor::symbol_editor::placement::SymbolPlacement;
use rspice_schematic_editor::view::design_view::DesignView;
use rspice_schematic_editor::view::selection_drag::ActiveSelectionDrag;

use super::requests::{DesignSurfaceRequest, SelectProbedTrace};

/// The design slot, the model, the session and the runs the canvas reads
/// and writes, beside the results workspace and the two modal facts it
/// reads.
pub(crate) struct CanvasParts<'a> {
    pub(crate) design: &'a mut DesignState,
    pub(crate) model: &'a mut Model,
    pub(crate) workbench: &'a mut WorkbenchSession,
    pub(crate) runs: &'a mut RunsState,
    pub(crate) results: &'a dyn ResultsPort,
    application_modal: bool,
    dialog_modal: bool,
}

/// The canvas's parts of the application's state, with its modal facts as
/// they stand now.
pub(crate) fn canvas_parts(state: &mut AppState) -> CanvasParts<'_> {
    let application_modal = canvas_application_modal_open(state);
    let dialog_modal = state.dialog_modal_open() || asked_dialog_waits(&state.workbench);
    CanvasParts {
        design: &mut state.design,
        model: &mut state.model,
        workbench: &mut state.workbench,
        runs: &mut state.runs,
        results: &*state.results,
        application_modal,
        dialog_modal,
    }
}

/// The canvas's parts of the design feature's context, with its modal facts
/// as the shell lent them.
pub(crate) fn canvas_parts_in<'a>(cx: &'a mut Cx<'_, DesignFeature>) -> CanvasParts<'a> {
    let application_modal_open = cx.application_modal_open();
    let dialog_modal_open = cx.dialog_modal_open();
    canvas_parts_lent(
        &mut *cx.state,
        &mut *cx.model,
        &mut *cx.wb,
        &mut *cx.runs,
        cx.peers.results(),
        application_modal_open,
        dialog_modal_open,
    )
}

/// The canvas's parts over the design slot, the model, the session, the
/// runs and the results workspace, with the two modal facts lent as
/// `application_modal_open` and `dialog_modal_open`: a modal dialog of the
/// design feature's own, or one the surface asked for that waits, counts as
/// well.
pub(crate) fn canvas_parts_lent<'a>(
    state: &'a mut DesignState,
    model: &'a mut Model,
    wb: &'a mut WorkbenchSession,
    runs: &'a mut RunsState,
    results: &'a dyn ResultsPort,
    application_modal_open: bool,
    dialog_modal_open: bool,
) -> CanvasParts<'a> {
    let application_modal =
        application_modal_open || own_modal_dialog_open(state, false) || asked_dialog_waits(wb);
    let dialog_modal =
        dialog_modal_open || own_modal_dialog_open(state, true) || asked_dialog_waits(wb);
    CanvasParts {
        design: &mut *state,
        model: &mut *model,
        workbench: &mut *wb,
        runs: &mut *runs,
        results,
        application_modal,
        dialog_modal,
    }
}

/// [`canvas_application_modal_open`] in the design feature's context: the
/// fact the shell lent it, a modal dialog of the feature's own that has
/// opened since, or a dialog the surface asked for that waits.
pub(crate) fn canvas_application_modal_open_in(cx: &Cx<'_, DesignFeature>) -> bool {
    cx.application_modal_open()
        || own_modal_dialog_open(cx.state, false)
        || asked_dialog_waits(cx.wb)
}

/// What a page of the design surface draws over for one pass: the parts the
/// canvas reads and writes, with the two modal facts as they stood when the
/// pass began.
///
/// Each step of the pass takes its own parts ([`Self::parts`]), whose modal
/// facts are the lent ones, a modal dialog of the design feature's own, or
/// one the surface asked for that waits, read as the step takes them. The
/// surface closes no modal dialog and opens another feature's only by
/// asking, so a step reads what the whole application would tell it.
pub(crate) struct CanvasFrame<'a> {
    pub(crate) design: &'a mut DesignState,
    pub(crate) model: &'a mut Model,
    pub(crate) workbench: &'a mut WorkbenchSession,
    pub(crate) runs: &'a mut RunsState,
    pub(crate) results: &'a dyn ResultsPort,
    application_modal: bool,
    dialog_modal: bool,
}

/// The canvas frame over the application's state, with its modal facts as
/// they stand now.
#[cfg(test)]
pub(crate) fn canvas_frame(state: &mut AppState) -> CanvasFrame<'_> {
    let application_modal = state.application_modal_open();
    let dialog_modal = state.dialog_modal_open();
    CanvasFrame {
        design: &mut state.design,
        model: &mut state.model,
        workbench: &mut state.workbench,
        runs: &mut state.runs,
        results: &*state.results,
        application_modal,
        dialog_modal,
    }
}

/// The canvas frame over the design feature's context, with its modal facts
/// as the shell lent them.
pub(crate) fn canvas_frame_in<'a>(cx: &'a mut Cx<'_, DesignFeature>) -> CanvasFrame<'a> {
    let application_modal_open = cx.application_modal_open();
    let dialog_modal_open = cx.dialog_modal_open();
    canvas_frame_lent(
        &mut *cx.state,
        &mut *cx.model,
        &mut *cx.wb,
        &mut *cx.runs,
        cx.peers.results(),
        application_modal_open,
        dialog_modal_open,
    )
}

/// The canvas frame over the design slot, the model, the session, the runs
/// and the results workspace, with the two modal facts lent as
/// `application_modal_open` and `dialog_modal_open`.
pub(crate) fn canvas_frame_lent<'a>(
    state: &'a mut DesignState,
    model: &'a mut Model,
    wb: &'a mut WorkbenchSession,
    runs: &'a mut RunsState,
    results: &'a dyn ResultsPort,
    application_modal_open: bool,
    dialog_modal_open: bool,
) -> CanvasFrame<'a> {
    let application_modal = application_modal_open;
    let dialog_modal = dialog_modal_open;
    CanvasFrame {
        design: &mut *state,
        model: &mut *model,
        workbench: &mut *wb,
        runs: &mut *runs,
        results,
        application_modal,
        dialog_modal,
    }
}

impl CanvasFrame<'_> {
    /// The parts one step of the pass takes, with the modal facts as the
    /// step begins.
    pub(crate) fn parts(&mut self) -> CanvasParts<'_> {
        let application_modal = self.application_modal_open();
        let dialog_modal = self.dialog_modal_open();
        CanvasParts {
            design: &mut *self.design,
            model: &mut *self.model,
            workbench: &mut *self.workbench,
            runs: &mut *self.runs,
            results: self.results,
            application_modal,
            dialog_modal,
        }
    }

    /// What the canvas's answers and its paint read, from this frame.
    pub(crate) fn facts(&self) -> CanvasFacts<'_> {
        CanvasFacts {
            design: &*self.design,
            model: &*self.model,
            runs: &*self.runs,
            results: self.results,
            probed_trace: probed_trace(&*self.workbench),
        }
    }

    /// Whether anything that holds the application modal is open, as the
    /// surface reads it.
    pub(crate) fn application_modal_open(&self) -> bool {
        self.application_modal
            || own_modal_dialog_open(self.design, false)
            || asked_dialog_waits(self.workbench)
    }

    /// Whether anything the dialog gate counts is open, as the surface
    /// reads it.
    pub(crate) fn dialog_modal_open(&self) -> bool {
        self.dialog_modal
            || own_modal_dialog_open(self.design, true)
            || asked_dialog_waits(self.workbench)
    }

    /// A refusal or transient feedback on the status bar.
    pub(crate) fn prompt(&mut self, text: impl Into<String>) {
        self.workbench.prompt(text);
    }
}

/// Whether the design surface reads a modal dialog as open: one that is,
/// or one it asked another workspace to open that waits for the shell.
/// Every dialog the surface asks for counts for the dialog gate too.
pub(crate) fn canvas_application_modal_open(state: &AppState) -> bool {
    state.application_modal_open() || asked_dialog_waits(&state.workbench)
}

/// Whether a dialog the design surface asked for waits in `workbench`'s
/// outbox for the shell to open it.
fn asked_dialog_waits(workbench: &WorkbenchSession) -> bool {
    workbench
        .outbox
        .peek::<DesignSurfaceRequest>()
        .is_some_and(DesignSurfaceRequest::opens_dialog)
}

impl AsRef<Model> for CanvasParts<'_> {
    fn as_ref(&self) -> &Model {
        self.model
    }
}

/// What the canvas's answers and its paint read: the design slot, the
/// model, the runs (whose slot facts say which check results are current)
/// and the results workspace (the selected trace and the cursor a transient
/// is annotated at), with the trace a probe selected that waits for it.
#[derive(Clone, Copy)]
pub(crate) struct CanvasFacts<'a> {
    pub(crate) design: &'a DesignState,
    pub(crate) model: &'a Model,
    pub(crate) runs: &'a RunsState,
    pub(crate) results: &'a dyn ResultsPort,
    probed_trace: Option<&'a SelectedResultTrace>,
}

/// The canvas's facts from the application's state.
#[cfg(test)]
pub(crate) fn canvas_facts(state: &AppState) -> CanvasFacts<'_> {
    CanvasFacts {
        design: &state.design,
        model: &state.model,
        runs: &state.runs,
        results: &*state.results,
        probed_trace: probed_trace(&state.workbench),
    }
}

/// The canvas's facts from the design feature's context.
pub(crate) fn canvas_facts_in<'a>(cx: &'a Cx<'_, DesignFeature>) -> CanvasFacts<'a> {
    canvas_facts_lent(cx.state, cx.model, cx.wb, cx.runs, cx.peers.results())
}

/// The canvas's facts from the design slot, the model, the session, the
/// runs and the results workspace.
pub(crate) fn canvas_facts_lent<'a>(
    state: &'a DesignState,
    model: &'a Model,
    wb: &'a WorkbenchSession,
    runs: &'a RunsState,
    results: &'a dyn ResultsPort,
) -> CanvasFacts<'a> {
    CanvasFacts {
        design: state,
        model,
        runs,
        results,
        probed_trace: probed_trace(wb),
    }
}

/// The trace a probe selected that waits in `workbench`'s outbox for the
/// results workspace to select it.
fn probed_trace(workbench: &WorkbenchSession) -> Option<&SelectedResultTrace> {
    workbench
        .outbox
        .peek::<SelectProbedTrace>()
        .map(|SelectProbedTrace(trace)| trace)
}

impl AsRef<Model> for CanvasFacts<'_> {
    fn as_ref(&self) -> &Model {
        self.model
    }
}

impl<'a> CanvasFacts<'a> {
    /// The selected trace, while the model's active run still holds it. A
    /// trace a probe selected that still waits for the results workspace
    /// comes first, as it would once the workspace has selected it.
    pub(crate) fn selected_trace(&self) -> Option<&'a SelectedResultTrace> {
        match self.probed_trace {
            Some(probed) => {
                let run = self.model.simulation.active_run()?;
                probed.resolve(run).map(|_| probed)
            }
            None => self.results.selected_trace(self.model),
        }
    }

    pub(crate) fn schematic_edit_read_only(&self) -> bool {
        self.model.schematic_edit_read_only()
    }

    pub(crate) fn active_view_read_only(&self) -> bool {
        self.model.active_view_read_only()
    }

    /// The in-place session the open symbol belongs to.
    pub(crate) fn symbol_in_place(&self) -> Option<&'a SymbolInPlace> {
        let design = self.design;
        in_place_session(&design.symbol.in_place, self.model)
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
            probed_trace: probed_trace(self.workbench),
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

    /// [`symbol_editor_request_source`] over these parts.
    pub(crate) fn symbol_editor_request_source(&self) -> SymbolRequestSource {
        symbol_editor_request_source(self.model, self.workbench)
    }

    /// [`active_symbol_save_checks`] over these parts.
    pub(crate) fn active_symbol_save_checks(
        &self,
        document: &SymbolDocument,
        ports: &[PortSpec],
    ) -> Vec<SymbolSaveCheck> {
        active_symbol_save_checks(self.model, document, ports)
    }

    /// [`publish_active_symbol_revision`] over these parts.
    pub(crate) fn publish_active_symbol_revision(
        &mut self,
        document: &SymbolDocument,
        metadata: &mut SymbolEditorMetadata,
        revision_note: &str,
    ) -> Result<u64, String> {
        publish_active_symbol_revision(
            &mut self.design.symbol,
            self.model,
            self.workbench,
            document,
            metadata,
            revision_note,
        )
    }

    /// [`finish_symbol_parent_edit`] over these parts.
    pub(crate) fn finish_symbol_parent_edit(&mut self) {
        finish_symbol_parent_edit(&mut self.design.symbol, self.model);
    }

    /// [`record_symbol_edit`] over these parts.
    pub(crate) fn record_symbol_edit(&mut self, before: &SymbolDocument) {
        record_symbol_edit(&mut self.design.symbol, self.model, before);
    }

    /// [`store_symbol_canvas_frame`] over these parts.
    pub(crate) fn store_symbol_canvas_frame(
        &mut self,
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
    ) -> Result<(), String> {
        store_symbol_canvas_frame(
            &mut self.design.symbol,
            self.model,
            self.workbench,
            document,
            metadata,
        )
    }

    /// The in-place session the open symbol belongs to.
    pub(crate) fn symbol_in_place(&self) -> Option<&SymbolInPlace> {
        in_place_session(&self.design.symbol.in_place, self.model)
    }

    /// [`CanvasParts::symbol_in_place`], to write to.
    pub(crate) fn symbol_in_place_mut(&mut self) -> Option<&mut SymbolInPlace> {
        in_place_session_mut(&mut self.design.symbol.in_place, self.model)
    }

    /// Where the edited instance puts its symbol on the parent.
    pub(crate) fn symbol_in_place_placement(&self) -> Option<SymbolPlacement> {
        in_place_placement(self.symbol_in_place()?, self.model)
    }

    pub(crate) fn save_and_leave_symbol_in_place(&mut self) {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .save_and_leave_symbol_in_place();
    }

    pub(crate) fn discard_and_leave_symbol_in_place(&mut self) {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .discard_and_leave_symbol_in_place();
    }

    pub(crate) fn stay_in_symbol_in_place(&mut self) {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .stay_in_symbol_in_place();
    }

    pub(crate) fn symbol_in_place_saved(&mut self) {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .symbol_in_place_saved();
    }

    pub(crate) fn symbol_in_place_save_cancelled(&mut self) {
        self.design
            .navigation_parts(self.model, self.workbench, self.runs)
            .symbol_in_place_save_cancelled();
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
