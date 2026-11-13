//! The design feature: the schematic and symbol canvases, the stimulus
//! library, and the dialogs that edit what they draw.
//!
//! What it has not registered yet, the shell's legacy tables still serve.

pub(crate) mod commands;
pub(crate) mod inspector;
pub(in crate::workbench) mod navigator;
mod routed;
pub(in crate::workbench) mod surface;
pub(crate) mod toolbar;

use egui::Context;
use rspice_schematic_editor::symbol_editor::commands::{
    SymbolPinTransform, SymbolTransform, activate_tool,
};
use rspice_schematic_editor::symbol_editor::session::SymbolTool;

use crate::schematic::view::grid_snap_selection::run_snap_selection_to_grid;
use crate::schematic::view::selection_layout::{
    SelectionLayoutCommand, apply_selection_layout, selection_layout_availability,
};
use crate::schematic::view::selection_transform::{
    SelectionTransform, transform_selection_on_active_sheet,
};
use crate::schematic::view::sheet_visibility::objects_of_kind_on_active_sheet;
use crate::schematic::view::standard_value_snap::{run_snap_to_standard_value, snap_block_reason};
use crate::schematic::view::{highlight_selected_net, selected_net, transient_annotation_status};
use crate::state::{ComponentType, GridStyle, SchematicAnnotationVisibility, Tool};
use crate::workbench::app::actions::property_edit::{
    open_selected_object_properties_in, selected_object_properties_available_in,
};
use crate::workbench::app::actions::workspace::design_checks;
use crate::workbench::app::actions::workspace::symbol_document::{
    active_symbol_pin_finding_count, active_symbol_ports, clear_active_symbol_pin_checks,
    refuse_unsavable_active_symbol, run_active_symbol_pin_checks,
    update_active_symbol_pins_from_contract,
};
use crate::workbench::app::actions::workspace::symbol_in_place::{
    SymbolInPlaceExit, in_place_session, selected_symbol_in_place_target,
};
use crate::workbench::app::actions::workspace::{
    NavigationParts, selected_hierarchy_master, selected_instance_veriloga_source,
};
use crate::workbench::app::actions::{
    activate_symbol_tool, stimulus, transform_selected_symbol_item, transform_selected_symbol_pin,
};
use crate::workbench::app::dialogs::connectivity_manager::{
    ConnectivityManager, open_connectivity_manager,
};
use crate::workbench::app::dialogs::design_review_comments::{
    ReviewComments, open_design_review_comments,
};
use crate::workbench::app::dialogs::engineering_table::{
    EngineeringTable, open_engineering_table_dialog,
};
use crate::workbench::app::dialogs::grid_snap_routing::open_grid_snap_routing_dialog;
use crate::workbench::app::dialogs::hierarchy::create::{
    CreateHierarchy, open_create_hierarchy_dialog,
};
use crate::workbench::app::dialogs::hierarchy::descend;
use crate::workbench::app::dialogs::placement::builtin_xspice::BuiltinXspicePlacement;
use crate::workbench::app::dialogs::placement::bus_tap::open_bus_tap;
use crate::workbench::app::dialogs::placement::pin_port::open_create_pins;
use crate::workbench::app::dialogs::property_dialog::PropertyDialog;
use crate::workbench::app::dialogs::replace_instance::{
    ReplaceInstance, open_replace_instance_dialog, replace_instance_available,
};
use crate::workbench::app::dialogs::selection::commands::{
    schematic_selection_has_live_object, select_schematic_objects_of_kind,
};
use crate::workbench::app::dialogs::selection::{
    array, bulk_edit, move_selection, rename, shared_properties, stretch,
};
use crate::workbench::app::dialogs::{
    create_model_bound_symbol, find_replace, object_properties, placement, rule_setup,
    stimulus_link, symbol_definition, visibility_options,
};
use crate::workbench::app::{
    arm_schematic_tool, join_collinear_wires_on_active_sheet, schematic_editor_request_source,
    turn_floating_paste,
};
use crate::workbench::app_state::SlotFacts;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::dialogs::{DialogKey, Modality};
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureRegistry, FeatureSurface, NoHandles, Reads,
};
use crate::workbench::framework::requests::FocusPlacementSearch;
use crate::workbench::slots::DesignState;
use crate::workbench::state::{Drawer, WorkbenchSession, Workspace};
use commands::DesignCommand;

/// The design feature.
pub(crate) struct DesignFeature;

impl Feature for DesignFeature {
    type State = DesignState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;
    /// The canvas's touch strip offers Rotate, Mirror, Undo, Redo and Fit as
    /// the dispatcher answers them, its guards included.
    const FOREIGN_AVAILABILITY: &'static [crate::state::command_catalog::CommandKey] =
        crate::schematic::view::TOUCH_STRIP_ASKED_COMMANDS;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<DesignCommand>();
        registry.dialogs(DIALOGS);
        registry.routed(routed::ROUTED);
        registry.toolbars(toolbar::TOOLBARS);
        registry.surface(FeatureSurface {
            show: surface::show_surface,
        });
    }
}

impl CommandSet for DesignCommand {
    type Feature = DesignFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    /// The schematic dialogs need a schematic or testbench in front of the
    /// reader; Design rule setup and Check design need only the project,
    /// which the registry already asks for. Rename names what it can act on; Replace instance and
    /// Create hierarchy name the first thing missing: the schematic, an
    /// editable one, then something in it to act on. Find and replace needs
    /// an editable schematic or testbench in front of the reader and names
    /// neither. A placement needs an
    /// editable schematic or testbench in front of the reader.
    ///
    /// Every library verb needs the instrument open, because they all act on
    /// the definition it is reading; New and Validate need only that,
    /// because a library with nothing in it is exactly where New belongs.
    /// Placing is an edit of the schematic, so a read-only view is the one
    /// extra thing it needs beyond a definition to place.
    ///
    /// The select tool, the grid cycle and Check cellview act on a schematic
    /// or a symbol in front, where a symbol checks its pin contract; the grid
    /// styles and Fit content on a schematic alone.
    ///
    /// Moving up the hierarchy needs a schematic in front below its root;
    /// a descent needs a schematic in front, then something to descend
    /// into. Clearing needs a check to have run against the view in front,
    /// and the tour a finding to visit; an annotation needs a schematic.
    ///
    /// A turn or flip with a symbol in front acts on its selection, which
    /// must be editable and hold something; otherwise it, the snaps and the
    /// collinear join need an editable schematic or testbench, and a turn
    /// then needs a live object selected or a copy floating under the
    /// pointer, and Snap selection to grid something selected. Snapping to a
    /// standard value names what blocks it. A layout needs a schematic whose
    /// selection it can lay out, and choosing the series only the project.
    ///
    /// Selecting by kind needs a schematic with an object of the kind on its
    /// active sheet. Move, Stretch, Create array, the symbol swap and the
    /// interface update need an editable schematic or testbench, then a
    /// selection each can act on; the swap and the update name what they
    /// lack. Highlight net needs a schematic, then names what keeps it from
    /// one net. Properties needs one inspectable object: a symbol's one pin
    /// or shape, or a schematic object or instances its forms can open.
    ///
    /// The symbol canvas's tools and its save need an editable symbol in
    /// front; a pin turn also needs one pin selected alone, and the contract
    /// update an interface to reconcile with. Editing a symbol in place
    /// needs a schematic in front and an instance whose symbol can be, and
    /// names what it lacks; leaving needs a symbol edited in place in front.
    fn availability(self, reads: &Reads<'_, DesignFeature>) -> CommandAvailability {
        let schematic = schematic_editor(reads);
        let selected = reads.state.selected_stimulus_definition.is_some();
        let refusal = match self {
            Self::GridSnapRouting
            | Self::VisibilityOptions
            | Self::EngineeringTableView
            | Self::FitSchematicContent => {
                (!schematic).then_some("command is unavailable in this context")
            }
            Self::SelectTool | Self::CycleGrid | Self::CheckCellview => {
                let symbol = routed::symbol_editor(reads.wb.workspace, reads.model);
                (!schematic && !symbol).then_some("command is unavailable in this context")
            }
            Self::GridDots | Self::GridLines | Self::GridOff => {
                (!schematic).then_some("open a schematic or testbench")
            }
            Self::ConnectivityManager => (!schematic).then_some("open a schematic or testbench"),
            Self::SelectionBulkEdit | Self::ReviewComments => {
                (!schematic).then_some("open a schematic project")
            }
            Self::RuleSetup | Self::CheckDesign => None,
            Self::RenameSelection
                if schematic && rename::rename_selection_available(reads.model) =>
            {
                None
            }
            Self::RenameSelection => {
                Some("select one editable component, net label, or declared bus")
            }
            Self::ReplaceInstance | Self::CreateHierarchy if !schematic => {
                Some("open an editable schematic or testbench")
            }
            Self::ReplaceInstance
                if replace_instance_available(&reads.state.property_registry, reads.model) =>
            {
                None
            }
            Self::CreateHierarchy if reads.model.create_hierarchy_available() => None,
            Self::ReplaceInstance | Self::CreateHierarchy
                if reads.model.schematic_edit_read_only() =>
            {
                Some("the active schematic is read-only")
            }
            Self::ReplaceInstance | Self::CreateHierarchy => Some("select an editable object"),
            Self::FindReplace => (!schematic || reads.model.schematic_edit_read_only())
                .then_some("command is unavailable in this context"),
            Self::AscendHierarchy if !schematic => Some("open a schematic or testbench"),
            Self::AscendHierarchy => (reads.model.workspace.content.hierarchy_stack.len() <= 1)
                .then_some("already at top hierarchy"),
            Self::DescendHierarchy | Self::DescendHierarchyDirect if !schematic => {
                Some("open a schematic or testbench")
            }
            Self::DescendHierarchy => selected_hierarchy_master(reads.model)
                .is_none()
                .then_some("select one hierarchical instance"),
            // The direct route opens what the instance is an instance of, as
            // its context-menu row does: a Verilog-A instance's module opens
            // as its cell view.
            Self::DescendHierarchyDirect => (selected_hierarchy_master(reads.model).is_none()
                && selected_instance_veriloga_source(reads.model).is_none())
            .then_some("select one hierarchical or Verilog-A instance"),
            Self::ClearChecks => (!check_results_stand(reads)).then_some("no check has run"),
            Self::NextViolation | Self::PreviousViolation => {
                (!findings_to_visit(reads)).then_some("the last check found no violations")
            }
            Self::AnnotateOperatingPoint | Self::AnnotateTransientVoltages => {
                (!schematic).then_some("open a schematic or testbench")
            }
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
                if routed::symbol_editor(reads.wb.workspace, reads.model) =>
            {
                (reads.model.active_view_read_only()
                    || reads.state.symbol.editor.effective_selection().is_empty())
                .then_some("open an editable schematic or testbench")
            }
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
            | Self::SnapSelectionToGrid
            | Self::SnapToStandardValue
            | Self::JoinCollinearWires
                if !schematic =>
            {
                Some("open an editable schematic or testbench")
            }
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
            | Self::SnapSelectionToGrid
            | Self::JoinCollinearWires
                if reads.model.schematic_edit_read_only() =>
            {
                Some("the active schematic is read-only")
            }
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical => {
                (!schematic_selection_has_live_object(&reads.model.schematic)
                    && reads.state.floating_paste.is_none())
                .then_some("select an editable object")
            }
            Self::SnapSelectionToGrid => reads
                .model
                .schematic
                .session
                .editor
                .selection
                .is_empty()
                .then_some("select an editable object"),
            Self::JoinCollinearWires | Self::SetStandardValueSeries(_) => None,
            Self::SnapToStandardValue => snap_block_reason(reads.model),
            Self::AlignSelectionLeft
            | Self::AlignSelectionCenter
            | Self::AlignSelectionRight
            | Self::AlignSelectionTop
            | Self::AlignSelectionMiddle
            | Self::AlignSelectionBottom
            | Self::DistributeSelectionHorizontal
            | Self::DistributeSelectionVertical => {
                let markers = reads.state.schematic_visibility.review_markers;
                let lays_out = schematic
                    && layout_command(self).is_some_and(|command| {
                        selection_layout_availability(reads.model, markers, command).is_ok()
                    });
                (!lays_out).then_some("command is unavailable in this context")
            }
            Self::SelectOfKind(_) if !schematic => Some("open a schematic or testbench"),
            Self::SelectOfKind(kind) => objects_of_kind_on_active_sheet(reads.model, kind)
                .is_empty()
                .then_some(kind.absent()),
            Self::MoveSelection
            | Self::StretchSelection
            | Self::ArraySelection
            | Self::SwapToRSpiceSymbols
            | Self::UpdateInstanceInterface
                if !schematic =>
            {
                Some("open an editable schematic or testbench")
            }
            Self::MoveSelection
            | Self::StretchSelection
            | Self::ArraySelection
            | Self::SwapToRSpiceSymbols
            | Self::UpdateInstanceInterface
                if reads.model.schematic_edit_read_only() =>
            {
                Some("the active schematic is read-only")
            }
            Self::MoveSelection => (!reads.model.schematic.has_live_movable_selection())
                .then_some("select an editable object"),
            Self::StretchSelection => reads
                .model
                .schematic
                .default_stretch_target()
                .is_none()
                .then_some("select an editable object"),
            Self::ArraySelection => reads
                .model
                .schematic
                .validate_array_source_selection()
                .is_err()
                .then_some("select an editable object"),
            Self::SwapToRSpiceSymbols => (!reads.model.schematic.can_swap_to_rspice_symbols())
                .then_some("no part here draws an imported symbol"),
            // The repair is offered exactly where the deck refuses: the
            // schematic itself answers whether the selected instance still
            // presents its master's interface, so a live row and a netlist
            // defect can never disagree. Its refusal names that condition
            // rather than the one the editing family reports.
            Self::UpdateInstanceInterface => {
                (!reads.model.schematic.selected_instance_interface_is_stale(
                    &reads.model.library_manager,
                    &reads.model.workspace.content.schematic_buffers,
                ))
                .then_some("select one instance whose master interface changed after it was placed")
            }
            Self::HighlightNet if !schematic => Some("open a schematic or testbench"),
            // What the command would refuse with is the reason.
            Self::HighlightNet => selected_net(
                reads.model,
                reads.state.schematic_visibility.net_highlighting,
            )
            .err(),
            Self::ObjectProperties if routed::symbol_editor(reads.wb.workspace, reads.model) => {
                let selection = reads.state.symbol.editor.effective_selection();
                (selection.pins.len() + selection.shapes.len() != 1)
                    .then_some("select one inspectable object")
            }
            Self::ObjectProperties => {
                let registry = &reads.state.property_registry;
                let inspectable = schematic
                    && (selected_object_properties_available_in(registry, reads.model)
                        || shared_properties::shared_properties_available(registry, reads.model));
                (!inspectable).then_some("select one inspectable object")
            }
            Self::SymbolPinTool
            | Self::SymbolPolylineTool
            | Self::SymbolRectangleTool
            | Self::SymbolCircleTool
            | Self::SymbolArcTool
            | Self::SymbolPolygonTool
            | Self::SymbolTextTool
            | Self::SymbolSave => {
                (!editable_symbol(reads)).then_some("command is unavailable in this context")
            }
            Self::SymbolRotatePin | Self::SymbolMirrorPin => {
                let selection = reads.state.symbol.editor.effective_selection();
                let one_pin = selection.pins.len() == 1
                    && selection.shapes.is_empty()
                    && selection.attributes.is_empty();
                (!(editable_symbol(reads) && one_pin))
                    .then_some("command is unavailable in this context")
            }
            // Nothing to reconcile against without a declared interface, and
            // the command must say so by reporting unavailable rather than
            // opening on an empty contract.
            Self::SymbolUpdatePinsFromContract => (!editable_symbol(reads)
                || active_symbol_ports(reads.model).is_empty())
            .then_some("command is unavailable in this context"),
            Self::EditSymbolInPlace => match selected_symbol_in_place_target(reads.model) {
                Ok(_) if schematic => None,
                Ok(_) => Some("select one instance"),
                Err(refusal) => Some(refusal.reason()),
            },
            Self::ExitSymbolInPlace => (!routed::symbol_editor(reads.wb.workspace, reads.model)
                || in_place_session(&reads.state.symbol.in_place, reads.model).is_none())
            .then_some("no symbol is being edited in place"),
            _ if self.places() && !schematic => Some("open an editable schematic or testbench"),
            _ if self.places() && reads.model.schematic_edit_read_only() => {
                Some("the active schematic is read-only")
            }
            Self::PlaceInstance
            | Self::PlaceWire
            | Self::PlaceBus
            | Self::PlaceBusTap
            | Self::PlaceJunction
            | Self::PlaceLabel
            | Self::PlaceOffSheetConnector
            | Self::PlaceProbe
            | Self::PlaceText
            | Self::PlaceShape
            | Self::PlacePin
            | Self::PlaceResistor
            | Self::PlaceCapacitor
            | Self::PlaceInductor
            | Self::PlaceDiode
            | Self::PlaceGround
            | Self::PlaceVoltageSource
            | Self::PlaceCurrentSource => None,
            _ if reads.wb.workspace != Workspace::Stimulus => Some("open the Stimulus Library"),
            Self::NewStimulusDefinition | Self::ValidateStimulusLibrary => None,
            Self::DuplicateStimulusDefinition | Self::DeleteStimulusDefinition => {
                (!selected).then_some("select a stimulus definition")
            }
            Self::PlaceSourceFromDefinition => {
                if reads.model.schematic_edit_read_only() {
                    Some("the active schematic view is read-only")
                } else {
                    (!selected).then_some("select a stimulus definition")
                }
            }
            Self::ApplyStimulusDraft | Self::RevertStimulusDraft => {
                (!stimulus::selected_draft_is_dirty(
                    reads.state.selected_stimulus_definition.as_deref(),
                    &reads.state.stimulus_editor,
                ))
                .then_some("this definition has no draft to apply")
            }
            Self::ShowAdopterOnSchematic => (!stimulus::selected_definition_has_adopter(
                reads.state.selected_stimulus_definition.as_deref(),
                reads.model,
            ))
            .then_some("no placed source has adopted this definition"),
        };
        refusal.map_or(
            CommandAvailability::Available,
            CommandAvailability::Disabled,
        )
    }

    /// The grid styles are one choice: the style the schematic draws is the
    /// one checked. Each annotation is checked while the schematic shows it,
    /// and the standard-value series while the project steps along it.
    fn checked(self, reads: &Reads<'_, DesignFeature>) -> Option<bool> {
        let annotations = reads.state.schematic_visibility.annotations;
        let style = match self {
            Self::GridDots => GridStyle::Dots,
            Self::GridLines => GridStyle::Lines,
            Self::GridOff => GridStyle::Off,
            Self::AnnotateOperatingPoint => {
                return Some(annotations == SchematicAnnotationVisibility::OperatingPoint);
            }
            Self::AnnotateTransientVoltages => {
                return Some(annotations == SchematicAnnotationVisibility::TransientVoltages);
            }
            Self::SetStandardValueSeries(series) => {
                return Some(reads.model.workspace.content.standard_value_series == series);
            }
            _ => return None,
        };
        Some(reads.state.grid == style)
    }

    fn execute(self, cx: &mut Cx<'_, DesignFeature>) {
        match self {
            Self::NewStimulusDefinition => stimulus::new_definition(
                &mut cx.state.selected_stimulus_definition,
                &mut cx.state.stimulus_editor,
                cx.model,
                cx.wb,
            ),
            Self::DuplicateStimulusDefinition => {
                stimulus::duplicate_definition(
                    &mut cx.state.selected_stimulus_definition,
                    cx.model,
                    cx.wb,
                );
            }
            Self::DeleteStimulusDefinition => stimulus::delete_definition(
                &mut cx.state.selected_stimulus_definition,
                &mut cx.state.stimulus_editor,
                cx.model,
            ),
            Self::ApplyStimulusDraft => stimulus::apply_draft(
                &mut cx.state.selected_stimulus_definition,
                &mut cx.state.stimulus_editor,
                cx.model,
                cx.wb,
            ),
            Self::RevertStimulusDraft => stimulus::revert_draft(
                cx.state.selected_stimulus_definition.as_deref(),
                &mut cx.state.stimulus_editor,
                cx.model,
            ),
            Self::ValidateStimulusLibrary => stimulus::validate_library(
                &cx.state.stimulus_editor,
                &cx.state.property_registry,
                cx.model,
            ),
            Self::PlaceSourceFromDefinition => {
                stimulus::place_selected_definition(
                    cx.state.selected_stimulus_definition.as_deref(),
                    &cx.state.stimulus_editor,
                    cx.model,
                    cx.wb,
                );
            }
            Self::GridSnapRouting => {
                open_grid_snap_routing_dialog(
                    &mut cx.state.grid_snap_routing,
                    cx.state.grid,
                    cx.model,
                    cx.wb,
                );
            }
            Self::VisibilityOptions => {
                visibility_options::open_schematic_visibility_options(
                    &mut cx.state.schematic_visibility_dialog,
                    cx.state.schematic_visibility,
                    cx.state.drawing_sheet_layers,
                );
            }
            Self::EngineeringTableView => {
                open_engineering_table_dialog(
                    &mut cx.state.engineering_table,
                    &cx.state.engineering_table_views,
                    cx.model,
                );
            }
            Self::ConnectivityManager => open_connectivity_manager(
                &mut cx.state.connectivity_manager,
                cx.model,
                cx.wb,
                cx.platform,
            ),
            Self::SelectionBulkEdit => bulk_edit::open_selection_bulk_edit_dialog(
                &mut cx.state.selection_bulk_edit,
                &cx.state.schematic_bulk_edit_filters,
                cx.model,
            ),
            Self::RuleSetup => {
                rule_setup::open_rule_setup(&mut cx.state.rule_setup, cx.model, cx.wb, cx.platform)
            }
            Self::ReviewComments => open_design_review_comments(
                &mut cx.state.design_review_comments,
                cx.model,
                cx.platform,
            ),
            Self::RenameSelection => {
                rename::open_selected_object_rename(
                    &mut cx.state.rename_selection,
                    cx.state.object_properties.open || cx.state.tabbed_property_dialog.open,
                    cx.model,
                );
            }
            Self::ReplaceInstance => open_replace_instance_dialog(
                &mut cx.state.replace_instance,
                &cx.state.property_registry,
                cx.model,
                cx.wb,
            ),
            Self::CreateHierarchy => {
                open_create_hierarchy_dialog(&mut cx.state.create_hierarchy, cx.model, cx.wb);
            }
            // A dialog that holds the workbench keeps the form shut. The
            // matches are listed in the Navigator, so it comes forward.
            Self::FindReplace => {
                if !cx.dialog_modal_open() {
                    cx.state.find_replace.open = true;
                    cx.wb.navigator_visible = true;
                    cx.state.show_navigator();
                }
            }
            Self::PlaceInstance => {
                cx.wb.activate(Workspace::Design);
                cx.wb.navigator_visible = true;
                cx.wb.drawer = Some(Drawer::Navigator);
                cx.state.show_component_shelf();
                cx.wb.outbox.push(FocusPlacementSearch);
            }
            Self::PlaceWire => arm_tool(cx, Tool::Wire),
            Self::PlaceBus => arm_tool(cx, Tool::Bus),
            Self::PlaceBusTap => {
                cx.wb.activate(Workspace::Design);
                open_bus_tap(&mut cx.state.bus_tap, cx.model);
            }
            Self::PlaceJunction => arm_tool(cx, Tool::Junction),
            Self::PlaceLabel => arm_tool(cx, Tool::Label),
            Self::PlaceOffSheetConnector => arm_tool(cx, Tool::OffSheetConnector),
            Self::PlaceProbe => arm_tool(cx, Tool::Probe),
            Self::PlaceText => {
                cx.wb.activate(Workspace::Design);
                let source = schematic_editor_request_source(&*cx.model);
                cx.state.design_note.open(source);
            }
            Self::PlaceShape => {
                cx.wb.activate(Workspace::Design);
                let source = schematic_editor_request_source(&*cx.model);
                let expected_shapes = cx.model.schematic.document().documentation_shapes.clone();
                cx.state.documentation_shape.open(source, expected_shapes);
            }
            Self::PlacePin => {
                cx.wb.activate(Workspace::Design);
                open_create_pins(&mut cx.state.pin_port, cx.model);
            }
            Self::PlaceResistor => arm_tool(cx, Tool::Place(ComponentType::Resistor)),
            Self::PlaceCapacitor => arm_tool(cx, Tool::Place(ComponentType::Capacitor)),
            Self::PlaceInductor => arm_tool(cx, Tool::Place(ComponentType::Inductor)),
            Self::PlaceDiode => arm_tool(cx, Tool::Place(ComponentType::Diode)),
            Self::PlaceGround => arm_tool(cx, Tool::Place(ComponentType::Ground)),
            Self::PlaceVoltageSource => arm_tool(cx, Tool::Place(ComponentType::VoltageSource)),
            Self::PlaceCurrentSource => arm_tool(cx, Tool::Place(ComponentType::CurrentSource)),
            Self::SelectTool => {
                if routed::symbol_editor(cx.wb.workspace, cx.model) {
                    activate_tool(&mut cx.state.symbol.editor, SymbolTool::Select, None);
                } else {
                    arm_tool(cx, Tool::Select);
                }
            }
            Self::FitSchematicContent => {
                let editor = &mut cx.model.schematic.session.editor;
                editor.needs_fit = true;
                editor.needs_drawing_sheet_fit = false;
            }
            // G changes only what the canvas shows: snap pitch and target
            // classes stay with Grid, snap and wire routing.
            Self::CycleGrid => {
                if routed::symbol_editor(cx.wb.workspace, cx.model) {
                    let editor = &mut cx.state.symbol.editor;
                    editor.show_grid = !editor.show_grid;
                } else {
                    cx.state.cycle_grid_style();
                }
            }
            Self::GridDots => cx.state.set_grid_style(GridStyle::Dots),
            Self::GridLines => cx.state.set_grid_style(GridStyle::Lines),
            Self::GridOff => cx.state.set_grid_style(GridStyle::Off),
            Self::ShowAdopterOnSchematic => {
                stimulus::show_adopter_on_schematic(
                    cx.state.selected_stimulus_definition.as_deref(),
                    cx.model,
                    cx.wb,
                );
                cx.wb.activate(Workspace::Design);
            }
            Self::AscendHierarchy => navigation(cx).ascend_workspace_level(),
            Self::DescendHierarchy => {
                descend::open_descend_hierarchy_dialog(
                    &mut cx.state.descend_hierarchy,
                    cx.model,
                    cx.wb,
                );
            }
            Self::DescendHierarchyDirect => navigation(cx).open_selected_instance_master(),
            Self::ClearChecks => {
                if routed::symbol_editor(cx.wb.workspace, cx.model) {
                    clear_active_symbol_pin_checks(cx.model);
                } else {
                    let facts = check_facts(cx);
                    cx.model.clear_active_design_check(facts);
                }
                cx.model.design_checks.drc_results = None;
                cx.model.design_checks.drc_checked_version = 0;
            }
            // A symbol in front has its pins checked against its schematic.
            Self::CheckCellview if routed::symbol_editor(cx.wb.workspace, cx.model) => {
                let facts = check_facts(cx);
                run_active_symbol_pin_checks(cx.model, facts);
            }
            Self::CheckCellview => {
                let facts = check_facts(cx);
                design_checks::check_cellview(cx.model, cx.wb, facts);
            }
            Self::CheckDesign => {
                let facts = check_facts(cx);
                design_checks::check_design(cx.model, cx.wb, facts);
            }
            Self::NextViolation => navigation(cx).cycle_violation(1),
            Self::PreviousViolation => navigation(cx).cycle_violation(-1),
            // Off leaves the violations on the drawing: hiding them as well
            // is the visibility dialog's choice.
            Self::AnnotateOperatingPoint => {
                let annotations = &mut cx.state.schematic_visibility.annotations;
                *annotations = if *annotations == SchematicAnnotationVisibility::OperatingPoint {
                    SchematicAnnotationVisibility::ViolationsOnly
                } else {
                    SchematicAnnotationVisibility::OperatingPoint
                };
            }
            // On, the status bar says which instant the badges read.
            Self::AnnotateTransientVoltages => {
                let annotations = &mut cx.state.schematic_visibility.annotations;
                if *annotations == SchematicAnnotationVisibility::TransientVoltages {
                    *annotations = SchematicAnnotationVisibility::ViolationsOnly;
                } else {
                    *annotations = SchematicAnnotationVisibility::TransientVoltages;
                    let status =
                        transient_annotation_status(cx.model, cx.peers.results().cursors().a);
                    cx.wb.prompt(status);
                }
            }
            // A symbol in front turns its own selected pins and shapes.
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
                if routed::symbol_editor(cx.wb.workspace, cx.model) =>
            {
                let modal_open = cx.application_modal_open();
                transform_selected_symbol_item(
                    &mut cx.state.symbol,
                    cx.model,
                    cx.wb,
                    modal_open,
                    symbol_transform(self),
                );
            }
            // A copy floating under the pointer is what these act on.
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical => {
                let transform = selection_transform(self);
                if !turn_floating_paste(&mut cx.state.floating_paste, transform) {
                    let symbols = cx.model.schematic_symbol_context();
                    transform_selection_on_active_sheet(cx.model, cx.wb, &symbols, transform);
                }
            }
            Self::JoinCollinearWires => {
                join_collinear_wires_on_active_sheet(cx.model, cx.wb);
            }
            Self::SnapSelectionToGrid => {
                let symbols = cx.model.schematic_symbol_context();
                run_snap_selection_to_grid(cx.model, cx.wb, &symbols);
            }
            Self::SnapToStandardValue => run_snap_to_standard_value(cx.model, cx.wb),
            Self::SetStandardValueSeries(series) => {
                let workspace = &mut cx.model.workspace.content;
                if workspace.standard_value_series != series {
                    workspace.standard_value_series = series;
                    workspace.project_metadata_dirty = true;
                }
            }
            Self::AlignSelectionLeft
            | Self::AlignSelectionCenter
            | Self::AlignSelectionRight
            | Self::AlignSelectionTop
            | Self::AlignSelectionMiddle
            | Self::AlignSelectionBottom
            | Self::DistributeSelectionHorizontal
            | Self::DistributeSelectionVertical => {
                if let Some(command) = layout_command(self) {
                    lay_out_selection(cx, command);
                }
            }
            Self::SelectOfKind(kind) => {
                select_schematic_objects_of_kind(cx.model, cx.wb, kind);
            }
            // A symbol in front shows its one selected pin or shape in the
            // Inspector.
            Self::ObjectProperties if routed::symbol_editor(cx.wb.workspace, cx.model) => {
                cx.wb.inspector_visible = true;
                cx.wb.drawer = Some(Drawer::Inspector);
            }
            // Several instances: one form for what they share.
            Self::ObjectProperties
                if shared_properties::shared_properties_available(
                    &cx.state.property_registry,
                    cx.model,
                ) =>
            {
                let modal_open = cx.dialog_modal_open();
                shared_properties::open_shared_properties(
                    &mut cx.state.shared_properties,
                    &cx.state.property_registry,
                    modal_open,
                    cx.model,
                    cx.wb,
                );
            }
            Self::ObjectProperties => {
                open_selected_object_properties_in(
                    &mut cx.state.property_form_parts(),
                    cx.model,
                    cx.wb,
                );
            }
            Self::MoveSelection => move_selection::arm_move_selection_in(
                &mut cx.state.move_selection,
                &mut cx.state.stretch_selection,
                &mut cx.state.array_selection,
                cx.model,
            ),
            Self::StretchSelection => stretch::arm_stretch_selection_in(
                &mut cx.state.stretch_selection,
                &mut cx.state.move_selection,
                &mut cx.state.array_selection,
                cx.state.schematic_visibility.wire_routing,
                cx.model,
                cx.wb,
            ),
            Self::ArraySelection => array::open_array_selection_dialog_in(
                &mut cx.state.array_selection,
                &mut cx.state.move_selection,
                &mut cx.state.stretch_selection,
                cx.model,
                cx.wb,
            ),
            Self::SwapToRSpiceSymbols => {
                cx.model.schematic.swap_to_rspice_symbols();
            }
            // The operation reports its own outcome, including the name of any
            // terminal the master dropped, so no route composes a second
            // account of what happened.
            Self::UpdateInstanceInterface => {
                let outcome = cx.model.schematic.update_selected_instance_interface(
                    &cx.model.library_manager,
                    &cx.model.workspace.content.schematic_buffers,
                );
                cx.wb.prompt(match outcome {
                    Ok(summary) => summary,
                    Err(error) => error.to_string(),
                });
            }
            Self::HighlightNet => {
                let highlighting = cx.state.schematic_visibility.net_highlighting;
                let outcome = highlight_selected_net(cx.model, highlighting);
                cx.wb.prompt(outcome);
            }
            Self::SymbolPinTool
            | Self::SymbolPolylineTool
            | Self::SymbolRectangleTool
            | Self::SymbolCircleTool
            | Self::SymbolArcTool
            | Self::SymbolPolygonTool
            | Self::SymbolTextTool => {
                activate_symbol_tool(&mut cx.state.symbol, cx.model, symbol_tool(self));
            }
            Self::SymbolRotatePin | Self::SymbolMirrorPin => {
                let transform = match self {
                    Self::SymbolMirrorPin => SymbolPinTransform::Mirror,
                    _ => SymbolPinTransform::Rotate,
                };
                let modal_open = cx.application_modal_open();
                transform_selected_symbol_pin(
                    &mut cx.state.symbol,
                    cx.model,
                    cx.wb,
                    modal_open,
                    transform,
                );
            }
            Self::SymbolUpdatePinsFromContract => {
                match update_active_symbol_pins_from_contract(&mut cx.state.symbol, cx.model, cx.wb)
                {
                    Ok(text) | Err(text) => cx.wb.prompt(text),
                }
            }
            // The checks run before the dialog opens, so a symbol that cannot
            // be published says which row blocked it instead of presenting a
            // save transaction with a disabled primary control.
            Self::SymbolSave => match refuse_unsavable_active_symbol(cx.model) {
                Some(refusal) => cx.wb.prompt(refusal),
                None => {
                    cx.state.symbol.editor.save_dialog_open = true;
                    cx.state.symbol.editor.save_error = None;
                }
            },
            Self::EditSymbolInPlace => {
                navigation(cx).edit_selected_symbol_in_place();
            }
            Self::ExitSymbolInPlace => {
                navigation(cx).request_symbol_in_place_exit(SymbolInPlaceExit::Parent);
            }
        }
    }
}

/// What moving the canvas to another view or occurrence reads and writes,
/// lent from the feature's context.
fn navigation<'a>(cx: &'a mut Cx<'_, DesignFeature>) -> NavigationParts<'a> {
    cx.state.navigation_parts(cx.model, cx.wb, cx.runs)
}

/// The slots' facts a check reads and publishes against: the runs' with the
/// schematic's review markers, as the session's own are.
fn check_facts(cx: &Cx<'_, DesignFeature>) -> SlotFacts {
    cx.runs
        .slot_facts(cx.state.schematic_visibility.review_markers)
}

/// Whether a check's results stand against the view in front: the symbol's
/// pin findings, or the schematic's rule results or check receipt.
fn check_results_stand(reads: &Reads<'_, DesignFeature>) -> bool {
    if routed::symbol_editor(reads.wb.workspace, reads.model) {
        return active_symbol_pin_finding_count(reads.model).is_some();
    }
    reads.model.design_checks.drc_results.is_some() || {
        let facts = reads
            .runs
            .slot_facts(reads.state.schematic_visibility.review_markers);
        reads
            .model
            .active_design_check_status(facts)
            .last_receipt()
            .is_some()
    }
}

/// Whether the finding tour has somewhere to go: a pin finding against the
/// symbol in front, or a violation in the schematic's current receipt.
fn findings_to_visit(reads: &Reads<'_, DesignFeature>) -> bool {
    if routed::symbol_editor(reads.wb.workspace, reads.model) {
        return active_symbol_pin_finding_count(reads.model).is_some_and(|findings| findings > 0);
    }
    let facts = reads
        .runs
        .slot_facts(reads.state.schematic_visibility.review_markers);
    reads
        .model
        .active_design_check_status(facts)
        .current_receipt()
        .is_some_and(|receipt| !receipt.result.violations().is_empty())
}

/// Arm `tool` on the schematic, as every placement that is a tool does.
fn arm_tool(cx: &mut Cx<'_, DesignFeature>, tool: Tool) {
    arm_schematic_tool(
        &mut cx.state.move_selection,
        &mut cx.state.stretch_selection,
        &mut cx.state.array_selection,
        cx.model,
        cx.wb,
        tool,
    );
}

/// Whether a schematic or testbench is the document in front of the reader
/// in the Design workspace.
/// Whether an editable symbol is in front.
fn editable_symbol(reads: &Reads<'_, DesignFeature>) -> bool {
    routed::symbol_editor(reads.wb.workspace, reads.model) && !reads.model.active_view_read_only()
}

/// The tool a symbol canvas tool command arms.
fn symbol_tool(command: DesignCommand) -> SymbolTool {
    match command {
        DesignCommand::SymbolPolylineTool => SymbolTool::Line,
        DesignCommand::SymbolRectangleTool => SymbolTool::Rectangle,
        DesignCommand::SymbolCircleTool => SymbolTool::Circle,
        DesignCommand::SymbolArcTool => SymbolTool::Arc,
        DesignCommand::SymbolPolygonTool => SymbolTool::Polygon,
        DesignCommand::SymbolTextTool => SymbolTool::Text,
        _ => SymbolTool::PlacePin,
    }
}

fn schematic_editor(reads: &Reads<'_, DesignFeature>) -> bool {
    reads.wb.workspace == Workspace::Design
        && matches!(
            reads.model.workspace.content.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
}

/// What a turn or flip does to the schematic's selection.
fn selection_transform(command: DesignCommand) -> SelectionTransform {
    match command {
        DesignCommand::RotateSelectionCounterClockwise => {
            SelectionTransform::RotateCounterClockwise
        }
        DesignCommand::MirrorSelectionHorizontal => SelectionTransform::MirrorHorizontal,
        DesignCommand::MirrorSelectionVertical => SelectionTransform::MirrorVertical,
        _ => SelectionTransform::RotateClockwise,
    }
}

/// What a turn or flip does to a symbol's selected pins and shapes.
fn symbol_transform(command: DesignCommand) -> SymbolTransform {
    match command {
        DesignCommand::RotateSelectionCounterClockwise => SymbolTransform::RotateCounterClockwise,
        DesignCommand::MirrorSelectionHorizontal => SymbolTransform::MirrorHorizontal,
        DesignCommand::MirrorSelectionVertical => SymbolTransform::MirrorVertical,
        _ => SymbolTransform::Rotate,
    }
}

/// The selection layout an alignment or a distribution asks for.
fn layout_command(command: DesignCommand) -> Option<SelectionLayoutCommand> {
    use SelectionLayoutCommand as Layout;
    Some(match command {
        DesignCommand::AlignSelectionLeft => Layout::AlignLeft,
        DesignCommand::AlignSelectionCenter => Layout::AlignCenter,
        DesignCommand::AlignSelectionRight => Layout::AlignRight,
        DesignCommand::AlignSelectionTop => Layout::AlignTop,
        DesignCommand::AlignSelectionMiddle => Layout::AlignMiddle,
        DesignCommand::AlignSelectionBottom => Layout::AlignBottom,
        DesignCommand::DistributeSelectionHorizontal => Layout::DistributeHorizontal,
        DesignCommand::DistributeSelectionVertical => Layout::DistributeVertical,
        _ => return None,
    })
}

/// Lay the selection out by `command`, saying so where it moved nothing or
/// could not be applied.
fn lay_out_selection(cx: &mut Cx<'_, DesignFeature>, command: SelectionLayoutCommand) {
    let symbols = cx.model.schematic_symbol_context();
    let markers = cx.state.schematic_visibility.review_markers;
    match apply_selection_layout(cx.model, cx.wb, markers, &symbols, command) {
        // The moved selection is the answer to a completed layout.
        Ok(true) => {}
        Ok(false) => cx
            .wb
            .prompt(format!("{} produced no geometry change.", command.label())),
        Err(error) => cx
            .wb
            .prompt(format!("{} was not applied: {error}.", command.label())),
    }
}

/// The dialog steps the feature draws, under the keys the shell's dialog
/// order lists them by and in that order. Each is open while its form is;
/// Import symbol definition while either of its two forms is.
const DIALOGS: &[FeatureDialog<DesignFeature>] = &[
    FeatureDialog {
        key: DialogKey("design.property"),
        is_open: |design| design.tabbed_property_dialog.open,
        render: DesignFeature::render_property_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.bus_tap"),
        is_open: |design| design.bus_tap.open,
        render: DesignFeature::render_bus_tap_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.builtin_xspice_placement"),
        is_open: |design| design.builtin_xspice_placement.open,
        render: DesignFeature::render_builtin_xspice_placement_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.net_label"),
        is_open: |design| design.net_label_placement.open,
        render: DesignFeature::render_net_label_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.grid_snap_routing"),
        is_open: |design| design.grid_snap_routing.open,
        render: DesignFeature::render_grid_snap_routing_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.schematic_visibility"),
        is_open: |design| design.schematic_visibility_dialog.open,
        render: DesignFeature::render_schematic_visibility_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.descend_hierarchy"),
        is_open: |design| design.descend_hierarchy.open,
        render: DesignFeature::render_descend_hierarchy_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.engineering_table"),
        is_open: |design| design.engineering_table.open,
        render: DesignFeature::render_engineering_table_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.pin_port"),
        is_open: |design| design.pin_port.open,
        render: DesignFeature::render_pin_port_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.design_review_comments"),
        is_open: |design| design.design_review_comments.open,
        render: DesignFeature::render_design_review_comments_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.design_note"),
        is_open: |design| design.design_note.open,
        render: DesignFeature::render_design_note_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.documentation_shape"),
        is_open: |design| design.documentation_shape.open,
        render: DesignFeature::render_documentation_shape_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.array_selection"),
        is_open: |design| design.array_selection.open,
        render: DesignFeature::render_array_selection_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.replace_instance"),
        is_open: |design| design.replace_instance.open,
        render: DesignFeature::render_replace_instance_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.stimulus_link"),
        is_open: |design| design.stimulus_link.open,
        render: DesignFeature::render_stimulus_link_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.create_hierarchy"),
        is_open: |design| design.create_hierarchy.open,
        render: DesignFeature::render_create_hierarchy_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.connectivity_manager"),
        is_open: |design| design.connectivity_manager.open,
        render: DesignFeature::render_connectivity_manager_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.selection_bulk_edit"),
        is_open: |design| design.selection_bulk_edit.open,
        render: DesignFeature::render_selection_bulk_edit_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.create_model_bound_symbol"),
        is_open: |design| design.create_model_bound_symbol.open,
        render: DesignFeature::render_create_model_bound_symbol_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.symbol_definition"),
        is_open: |design| design.symbol_import.open || design.symbol_parameter_form.open,
        render: DesignFeature::render_symbol_definition_dialogs,
    },
    FeatureDialog {
        key: DialogKey("design.rule_setup"),
        is_open: |design| design.rule_setup.open,
        render: DesignFeature::render_rule_setup_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.object_properties"),
        is_open: |design| design.object_properties.open,
        render: DesignFeature::render_object_properties_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.rename_selection"),
        is_open: |design| design.rename_selection.open,
        render: DesignFeature::render_rename_selection_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.find_replace"),
        is_open: |design| design.find_replace.open,
        render: DesignFeature::render_find_replace_dialog,
    },
    FeatureDialog {
        key: DialogKey("design.shared_properties"),
        is_open: |design| design.shared_properties.open,
        render: DesignFeature::render_shared_properties_dialog,
    },
];

/// How each of the feature's dialogs holds the workbench while it is open,
/// as the shell's dialog order lists it: the property form holds the
/// application modal, the descent chooser is drawn beside the canvas, and
/// every other dialog holds the dialog gate.
pub(crate) fn dialog_modality(key: DialogKey) -> Modality {
    match key.0 {
        "design.property" => Modality::Application,
        "design.descend_hierarchy" => Modality::Modeless,
        _ => Modality::Dialog,
    }
}

/// Whether one of the feature's own dialogs is open in `design` that holds
/// the application modal or, with `dialog_gate`, the dialog gate.
pub(crate) fn own_modal_dialog_open(design: &DesignState, dialog_gate: bool) -> bool {
    DIALOGS.iter().any(|dialog| {
        let counts = match dialog_modality(dialog.key) {
            Modality::Dialog => true,
            Modality::Application => !dialog_gate,
            Modality::Modeless | Modality::None => false,
        };
        counts && (dialog.is_open)(design)
    })
}

/// The feature's dialog steps, each over the parts of the design slot it
/// owns.
impl DesignFeature {
    /// Place bus tap.
    fn render_bus_tap_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::bus_tap::render_bus_tap_dialog(&mut cx.state.bus_tap, cx.model, ctx);
    }

    /// Net label, or the off-sheet connector form.
    fn render_net_label_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::net_label::render_net_label_dialog(
            &mut cx.state.net_label_placement,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Create pins.
    fn render_pin_port_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::pin_port::render_pin_port_dialog(&mut cx.state.pin_port, cx.model, ctx);
    }

    /// Place text or design note.
    fn render_design_note_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::design_note::render_design_note_dialog(&mut cx.state.design_note, cx.model, ctx);
    }

    /// Draw documentation shape.
    fn render_documentation_shape_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::documentation_shape::render_documentation_shape_dialog(
            &mut cx.state.documentation_shape,
            cx.model,
            ctx,
        );
    }

    /// Create array.
    fn render_array_selection_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        array::render_array_selection_dialog(&mut cx.state.array_selection, cx.model, ctx);
    }

    /// Object properties.
    fn render_object_properties_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        object_properties::render_object_properties_dialog(
            &mut cx.state.object_properties,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Rename.
    fn render_rename_selection_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        rename::render_rename_selection_dialog(
            &mut cx.state.rename_selection,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Find and replace.
    fn render_find_replace_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let edit_in_progress = cx.state.schematic_edit_in_progress(cx.model);
        find_replace::render_find_replace_dialog(
            &mut cx.state.find_replace,
            edit_in_progress,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Properties, for the instances selected together.
    fn render_shared_properties_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        shared_properties::render_shared_properties_dialog(
            &mut cx.state.shared_properties,
            &cx.state.property_registry,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Grid, snap and wire routing.
    fn render_grid_snap_routing_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        cx.state
            .render_grid_snap_routing_dialog(cx.model, cx.wb, ctx);
    }

    /// Display options.
    fn render_schematic_visibility_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        cx.state
            .render_schematic_visibility_dialog(cx.model, cx.wb, ctx);
    }

    /// Descend into hierarchy. A checked choice goes to the shell, which
    /// answers it straight after the dialog's step draws.
    fn render_descend_hierarchy_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        descend::render_descend_hierarchy_dialog(
            &mut cx.state.descend_hierarchy,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Import symbol definition, then the symbol's parameter form.
    fn render_symbol_definition_dialogs(cx: &mut Cx<'_, Self>, ctx: &Context) {
        symbol_definition::render_symbol_definition_dialogs(
            &mut cx.state.symbol_import,
            &mut cx.state.symbol_parameter_form,
            cx.model,
            ctx,
        );
    }

    /// Design review comments, signed with the name the cloud gives the
    /// reviewer.
    fn render_design_review_comments_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        ReviewComments {
            dialog: &mut cx.state.design_review_comments,
            design_note: &mut cx.state.design_note,
            model: cx.model,
            workbench: cx.wb,
            cloud: cx.peers.cloud(),
            session: cx.peers.cloud_session(),
        }
        .render_design_review_comments_dialog(ctx);
    }

    /// Design rule setup. A new deck changes what blocks a run, so an applied
    /// one drops the run prepared under the old deck with its preflight.
    fn render_rule_setup_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let review_markers = cx.state.schematic_visibility.review_markers;
        if rule_setup::render_rule_setup_dialog(
            &mut cx.state.rule_setup,
            review_markers,
            cx.model,
            cx.runs,
            ctx,
        ) {
            rspice_app_runs::workbench::preflight::invalidate(
                &mut cx.runs.preflight,
                cx.handles.run_controller,
            );
        }
    }

    /// Bulk edit, over its draft and the saved filters.
    fn render_selection_bulk_edit_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        bulk_edit::render_selection_bulk_edit_dialog(
            &mut cx.state.selection_bulk_edit,
            &mut cx.state.schematic_bulk_edit_filters,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Engineering table view, over its draft and the views saved on this
    /// device.
    fn render_engineering_table_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        EngineeringTable {
            dialog: &mut cx.state.engineering_table,
            views: &mut cx.state.engineering_table_views,
            model: cx.model,
            platform: cx.platform,
            workbench: cx.wb,
        }
        .render_engineering_table_dialog(ctx);
    }

    /// Adopt a stimulus definition onto a placed source, or save its card as
    /// one, over the draft and the definition the Stimulus Library selects.
    fn render_stimulus_link_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        stimulus_link::render_stimulus_link_dialog(
            &mut cx.state.stimulus_link,
            &mut cx.state.selected_stimulus_definition,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Create model-bound symbol, over its draft. A created symbol asks for
    /// its view to open in the Design workspace.
    fn render_create_model_bound_symbol_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        create_model_bound_symbol::render_create_model_bound_symbol_dialog(
            &mut cx.state.create_model_bound_symbol,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// The schematic component editor, over its draft, the property sheets
    /// and the stimulus link it hands an instance to. A re-adopted stimulus
    /// asks for the editor to reopen on the instance's new card.
    fn render_property_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        PropertyDialog {
            dialog: &mut cx.state.tabbed_property_dialog,
            registry: &mut cx.state.property_registry,
            stimulus_link: &mut cx.state.stimulus_link,
            selected_stimulus_definition: &mut cx.state.selected_stimulus_definition,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_property_dialog(ctx);
    }

    /// The connectivity and bus manager, over its draft. Revealing a finding,
    /// opening the source map and applying reviewed repairs go to the shell.
    fn render_connectivity_manager_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        ConnectivityManager {
            dialog: &mut cx.state.connectivity_manager,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_connectivity_manager_dialog(ctx);
    }

    /// Create hierarchy, over its draft and the model it checks the draft
    /// against. A checked extraction goes to the shell.
    fn render_create_hierarchy_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        CreateHierarchy {
            dialog: &mut cx.state.create_hierarchy,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_create_hierarchy_dialog(ctx);
    }

    /// Replace instance, over its draft, the property sheets its catalog reads,
    /// the model and the session.
    fn render_replace_instance_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        ReplaceInstance {
            dialog: &mut cx.state.replace_instance,
            registry: &cx.state.property_registry,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_replace_instance_dialog(ctx);
    }

    /// The built-in XSPICE placement form, over its draft, the model it arms
    /// the placement on and the session.
    fn render_builtin_xspice_placement_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        BuiltinXspicePlacement {
            dialog: &mut cx.state.builtin_xspice_placement,
            model: cx.model,
            workbench: cx.wb,
        }
        .render_builtin_xspice_placement_dialog(ctx);
    }
}
