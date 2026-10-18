//! The selection context menu for the schematic canvas.
//!
//! Right-click, Shift+F10 and a touch long-press all open the same command
//! contract. Every visible command is backed by a real schematic or results
//! operation; a command that cannot be taken right now stays visibly disabled
//! with an explanation, and only one that could never apply to the kind of
//! object selected is left out (`menu_entries`).

use egui::{Context, Response};

use crate::state::{Point, Tool, ViewType};
use crate::workbench::app_state::{AppState, ContextTarget};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::Workspace;
use crate::workbench::{
    ResultViewer,
    app::{
        StimulusLinkMode, commit_readoption, new_veriloga_device_available,
        open_new_veriloga_device_form, open_replace_instance_dialog, open_stimulus_definition,
        open_stimulus_link, replace_instance_available,
    },
};

use super::SchematicSymbolContext;
use super::coordinates::{screen_to_grid, screen_to_schematic};
use super::interaction::{PointerHit, PointerTarget, pointer_target};
use super::selection_transform::{SelectionTransform, transform_selection_on_active_sheet};
use super::sheet_visibility::retain_selection_on_active_sheet;
use super::viewport::Viewport;

use super::selection_layout::{
    SelectionLayoutCommand, apply_selection_layout, selection_layout_availability,
};
use super::shelf_drag::{PlacementTransform, something_floats, turn_what_floats};
use rspice_schematic_editor::view::context_menu::{
    self as context_surface, ArrangeAction, ContextAction, ContextIcon, ContextMenuBinding,
    ContextMenuRequest, ContextMenuView,
};
use rspice_schematic_editor::view::pick_chooser::ChooserRow;
use rspice_schematic_editor::view::selection_drag::target_is_selected;
use rspice_schematic_editor::view::tool_input::ToolPointerView;

mod stimulus;

#[derive(Debug, Clone, Copy)]
struct ContextCommand {
    action: ContextAction,
    icon: ContextIcon,
    label: &'static str,
    shortcut_command: Option<Command>,
}

type ContextEntry = context_surface::ContextEntry<ContextCommand>;

const CONTEXT_ENTRIES: &[ContextEntry] = &[
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Properties,
        icon: ContextIcon::Sliders,
        label: "Object properties…",
        shortcut_command: Some(Command::ObjectProperties),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Rotate,
        icon: ContextIcon::Rotate,
        label: "Rotate 90°",
        shortcut_command: Some(Command::RotateSelection),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Mirror,
        icon: ContextIcon::Mirror,
        label: "Mirror",
        shortcut_command: Some(Command::MirrorSelectionHorizontal),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::MirrorVertical,
        icon: ContextIcon::MirrorVertical,
        label: "Mirror vertically",
        shortcut_command: Some(Command::MirrorSelectionVertical),
    }),
    // The Design menu's Arrange submenu, as a page of this menu: a phone
    // shows no menu bar, and Align and Distribute have no key.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Arrange,
        icon: ContextIcon::Arrange,
        label: "Arrange",
        shortcut_command: None,
    }),
    // The stimulus-library group, which exists only in a placed source's menu
    // and states the verbs that source's standing with the library calls for;
    // see `stimulus`.
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::AdoptStimulus,
        icon: ContextIcon::Waveform,
        label: "Adopt stimulus definition…",
        shortcut_command: None,
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ReadoptStimulus,
        icon: ContextIcon::Waveform,
        label: "Re-adopt library revision",
        shortcut_command: None,
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::SaveStimulus,
        icon: ContextIcon::Waveform,
        label: "Save as stimulus definition…",
        shortcut_command: None,
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::OpenStimulusDefinition,
        icon: ContextIcon::Waveform,
        label: "Open in Stimulus Library",
        shortcut_command: None,
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Cut,
        icon: ContextIcon::Copy,
        label: "Cut selection",
        shortcut_command: Some(Command::Cut),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Copy,
        icon: ContextIcon::Copy,
        label: "Copy selection",
        shortcut_command: Some(Command::Copy),
    }),
    // Where the menu was opened, which is the point a reader right-clicks
    // to say "here".
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Paste,
        icon: ContextIcon::Copy,
        label: "Paste here",
        shortcut_command: Some(Command::Paste),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Duplicate,
        icon: ContextIcon::Copy,
        label: "Duplicate",
        shortcut_command: Some(Command::Duplicate),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Delete,
        icon: ContextIcon::Trash,
        label: "Delete",
        shortcut_command: Some(Command::Delete),
    }),
    ContextEntry::Separator,
    // The instance-master slot: what the menu's subject has for a master.
    // An instance descends into its schematic; a Verilog-A instance's master
    // is a module, which opens as its cell view (the Descend shortcut does
    // the same); the empty canvas has no master and offers a new one.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::DescendHierarchy,
        icon: ContextIcon::Hierarchy,
        label: "Descend into selected instance",
        shortcut_command: Some(Command::DescendHierarchyDirect),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::OpenVerilogAView,
        icon: ContextIcon::VerilogA,
        label: "Open Verilog-A view",
        shortcut_command: Some(Command::DescendHierarchyDirect),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::NewVerilogADevice,
        icon: ContextIcon::VerilogA,
        label: "New Verilog-A device…",
        shortcut_command: Some(Command::NewVerilogADevice),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::UpdateInstanceInterface,
        icon: ContextIcon::Hierarchy,
        label: "Update instance interface",
        shortcut_command: Some(Command::UpdateInstanceInterface),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ReplaceInstance,
        icon: ContextIcon::Hierarchy,
        label: "Replace instance…",
        shortcut_command: Some(Command::ReplaceInstance),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::CreateHierarchy,
        icon: ContextIcon::Hierarchy,
        label: "Create hierarchy from selection…",
        shortcut_command: Some(Command::CreateHierarchy),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::CreateSymbolFromPorts,
        icon: ContextIcon::Hierarchy,
        label: "Create symbol from schematic ports…",
        shortcut_command: None,
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::PageSetup,
        icon: ContextIcon::Sheet,
        label: "Page setup…",
        shortcut_command: Some(Command::PageSetup),
    }),
    // Fitting the drawing sheet is not here. `Command::ZoomFit` already has a
    // toolbar button on this canvas, a status-bar route, a mobile canvas
    // control and the `F` key; a selection menu that fits without scrolling is
    // worth more than a sixth route to it.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::FitContent,
        icon: ContextIcon::Fit,
        label: "Fit schematic content",
        shortcut_command: Some(Command::FitSchematicContent),
    }),
    // Only on the empty canvas's menu: a finger has no Z key, and the next
    // drag after this row frames the view.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ZoomBox,
        icon: ContextIcon::ZoomBox,
        label: "Zoom box",
        shortcut_command: Some(Command::ZoomBox),
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ShowInNetlist,
        icon: ContextIcon::Code,
        label: "Show in netlist",
        shortcut_command: Some(Command::ShowInNetlist),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Probe,
        icon: ContextIcon::Probe,
        label: "Add voltage or current probe…",
        shortcut_command: Some(Command::PlaceProbe),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::OperatingPoint,
        icon: ContextIcon::Waveform,
        label: "Open operating point",
        shortcut_command: Some(Command::ResultViewer(ResultViewer::Op)),
    }),
];

/// The one independent source the menu is about, when that is all that is
/// selected.
fn clicked_source(state: &AppState) -> Option<&crate::state::Component> {
    let id = state
        .schematic
        .session
        .editor
        .selection
        .single_component()?;
    state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .filter(|component| {
            crate::simulation::stimulus_realize::is_independent_source(component.kind)
        })
}

/// What one menu is about, which decides what fills its instance-master slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuSubject {
    /// One placed independent source: the stimulus group, and no master rows.
    Source,
    /// An instance whose master is a Verilog-A view.
    VerilogAInstance,
    /// The canvas itself: no object was under the pointer.
    Canvas,
    /// Any other instance, or a wire.
    Object,
}

fn menu_subject(state: &AppState, target: ContextTarget) -> MenuSubject {
    if clicked_source(state).is_some() {
        return MenuSubject::Source;
    }
    match target {
        ContextTarget::Canvas => MenuSubject::Canvas,
        ContextTarget::Component(id) if state.veriloga_master_of_component(id).is_some() => {
            MenuSubject::VerilogAInstance
        }
        ContextTarget::Component(_) | ContextTarget::Wire(_) => MenuSubject::Object,
    }
}

/// The Arrange page: the Design menu's Arrange submenu, group by group.
const ARRANGE_ENTRIES: &[ContextEntry] = &[
    arrange(ArrangeAction::RotateClockwise, ContextIcon::Rotate),
    arrange(ArrangeAction::RotateCounterClockwise, ContextIcon::Rotate),
    arrange(ArrangeAction::MirrorHorizontal, ContextIcon::Mirror),
    arrange(ArrangeAction::MirrorVertical, ContextIcon::MirrorVertical),
    ContextEntry::Separator,
    arrange(ArrangeAction::AlignLeft, ContextIcon::Arrange),
    arrange(ArrangeAction::AlignCenter, ContextIcon::Arrange),
    arrange(ArrangeAction::AlignRight, ContextIcon::Arrange),
    arrange(ArrangeAction::AlignTop, ContextIcon::Arrange),
    arrange(ArrangeAction::AlignMiddle, ContextIcon::Arrange),
    arrange(ArrangeAction::AlignBottom, ContextIcon::Arrange),
    ContextEntry::Separator,
    arrange(ArrangeAction::DistributeHorizontal, ContextIcon::Arrange),
    arrange(ArrangeAction::DistributeVertical, ContextIcon::Arrange),
    ContextEntry::Separator,
    arrange(ArrangeAction::SnapToGrid, ContextIcon::Grid),
];

const fn arrange(action: ArrangeAction, icon: ContextIcon) -> ContextEntry {
    let command = arrange_command(action);
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ArrangeCommand(action),
        icon,
        label: command.spec().label,
        shortcut_command: Some(command),
    })
}

/// The product command an Arrange row stands for.
const fn arrange_command(action: ArrangeAction) -> Command {
    match action {
        ArrangeAction::RotateClockwise => Command::RotateSelection,
        ArrangeAction::RotateCounterClockwise => Command::RotateSelectionCounterClockwise,
        ArrangeAction::MirrorHorizontal => Command::MirrorSelectionHorizontal,
        ArrangeAction::MirrorVertical => Command::MirrorSelectionVertical,
        ArrangeAction::AlignLeft => Command::AlignSelectionLeft,
        ArrangeAction::AlignCenter => Command::AlignSelectionCenter,
        ArrangeAction::AlignRight => Command::AlignSelectionRight,
        ArrangeAction::AlignTop => Command::AlignSelectionTop,
        ArrangeAction::AlignMiddle => Command::AlignSelectionMiddle,
        ArrangeAction::AlignBottom => Command::AlignSelectionBottom,
        ArrangeAction::DistributeHorizontal => Command::DistributeSelectionHorizontal,
        ArrangeAction::DistributeVertical => Command::DistributeSelectionVertical,
        ArrangeAction::SnapToGrid => Command::SnapSelectionToGrid,
    }
}

/// The selection layout an Arrange row asks for, when it is not a turn.
fn arrange_layout(action: ArrangeAction) -> Option<SelectionLayoutCommand> {
    use SelectionLayoutCommand as Layout;
    Some(match action {
        ArrangeAction::AlignLeft => Layout::AlignLeft,
        ArrangeAction::AlignCenter => Layout::AlignCenter,
        ArrangeAction::AlignRight => Layout::AlignRight,
        ArrangeAction::AlignTop => Layout::AlignTop,
        ArrangeAction::AlignMiddle => Layout::AlignMiddle,
        ArrangeAction::AlignBottom => Layout::AlignBottom,
        ArrangeAction::DistributeHorizontal => Layout::DistributeHorizontal,
        ArrangeAction::DistributeVertical => Layout::DistributeVertical,
        ArrangeAction::RotateClockwise
        | ArrangeAction::RotateCounterClockwise
        | ArrangeAction::MirrorHorizontal
        | ArrangeAction::MirrorVertical
        | ArrangeAction::SnapToGrid => return None,
    })
}

/// The turn an Arrange row asks for, when it is not a layout.
const fn arrange_turn(action: ArrangeAction) -> Option<(SelectionTransform, PlacementTransform)> {
    Some(match action {
        ArrangeAction::RotateClockwise => (
            SelectionTransform::RotateClockwise,
            PlacementTransform::RotateClockwise,
        ),
        ArrangeAction::MirrorHorizontal => (
            SelectionTransform::MirrorHorizontal,
            PlacementTransform::ToggleHorizontalMirror,
        ),
        ArrangeAction::MirrorVertical => (
            SelectionTransform::MirrorVertical,
            PlacementTransform::ToggleVerticalMirror,
        ),
        ArrangeAction::RotateCounterClockwise => (
            SelectionTransform::RotateCounterClockwise,
            PlacementTransform::RotateCounterClockwise,
        ),
        _ => return None,
    })
}

/// Turn what floats, the placement ghost or what Move carries, as R and M
/// would; with nothing floating, turn the selection on the sheet.
fn turn(state: &mut AppState, symbol_context: &SchematicSymbolContext, action: ArrangeAction) {
    let Some((selection, floating)) = arrange_turn(action) else {
        return;
    };
    if !turn_what_floats(state, floating) {
        transform_selection_on_active_sheet(state, symbol_context, selection);
    }
}

/// Whether the catalog's instance-master slot holds `action` for `subject`.
fn master_slot_shows(action: ContextAction, subject: MenuSubject) -> bool {
    match action {
        ContextAction::DescendHierarchy => subject == MenuSubject::Object,
        ContextAction::OpenVerilogAView => subject == MenuSubject::VerilogAInstance,
        ContextAction::NewVerilogADevice | ContextAction::ZoomBox => subject == MenuSubject::Canvas,
        ContextAction::UpdateInstanceInterface | ContextAction::ReplaceInstance => {
            matches!(subject, MenuSubject::Object | MenuSubject::VerilogAInstance)
        }
        _ => true,
    }
}

/// The entries the menu is made of, for what it was opened on.
///
/// The catalog is one list, and almost all of it is every target's: a verb that
/// cannot be taken right now stays where it is, disabled, and says why. What is
/// left out is a verb that could never apply to the *kind* of object the menu
/// is about. Only the instance-master slot varies. An instance offers its
/// master's three rows; a Verilog-A instance's Descend row opens its module's
/// view; the empty canvas, which has no master, offers a new Verilog-A device
/// in their place. A placed source gets the stimulus-library group instead of
/// the master rows, which is also what lets the group in without turning the
/// menu into a scroller. A rule with nothing left under it goes too.
fn menu_entries(state: &AppState, target: ContextTarget) -> Vec<ContextEntry> {
    let subject = menu_subject(state, target);
    let mut entries: Vec<ContextEntry> = Vec::with_capacity(CONTEXT_ENTRIES.len());
    for entry in CONTEXT_ENTRIES {
        let shown = match entry {
            ContextEntry::Separator => {
                matches!(entries.last(), Some(ContextEntry::Command(_)))
            }
            ContextEntry::Command(command) if stimulus::owns(command.action) => {
                stimulus::shown(command.action, state)
            }
            ContextEntry::Command(command) => master_slot_shows(command.action, subject),
        };
        if shown {
            entries.push(*entry);
        }
    }
    if matches!(entries.last(), Some(ContextEntry::Separator)) {
        entries.pop();
    }
    entries
}

pub(super) fn handle_context_menu(
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    _routing_was_active: bool,
    symbol_context: &SchematicSymbolContext,
) {
    retain_selection_on_active_sheet(state);
    #[cfg(target_arch = "wasm32")]
    let browser_keyboard_open =
        crate::workbench::browser::accessibility::take_schematic_context_menu_request();
    #[cfg(not(target_arch = "wasm32"))]
    let browser_keyboard_open = false;
    let opening = context_surface::opening(response, browser_keyboard_open);
    if !opening.needs_contents() {
        context_surface::close(response);
        forget_choices(&response.ctx);
        state.dialogs.interaction.context_target = None;
        return;
    }
    let anchor = if opening.secondary_clicked {
        capture_pointer_target(response, state, viewport, symbol_context)
    } else {
        if opening.keyboard_open {
            forget_choices(&response.ctx);
            let (target, click_pos) = keyboard_target(state, viewport, response.rect.center());
            state.dialogs.interaction.context_target = Some((target, (click_pos.x, click_pos.y)));
        }
        None
    };
    let Some(view) = context_menu_view(state, &response.ctx) else {
        context_surface::close(response);
        return;
    };
    let output = context_surface::show(response, opening, anchor, &view);
    match output.request {
        Some(ContextMenuRequest {
            binding,
            action: ContextAction::Choose(target),
        }) => choose_target(state, &view, &binding, target),
        Some(request) => apply_context_request(state, request, symbol_context),
        None => {}
    }
    if !output.open {
        forget_choices(&response.ctx);
        state.dialogs.interaction.context_target = None;
    }
}

fn choices_id() -> egui::Id {
    egui::Id::new("rspice.schematic.context_menu.choices")
}

fn forget_choices(ctx: &Context) {
    ctx.data_mut(|data| data.remove::<Vec<ChooserRow>>(choices_id()));
}

fn capture_pointer_target(
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) -> Option<egui::Pos2> {
    let pos = response.interact_pointer_pos()?;
    let grid_pos = screen_to_grid(viewport, state.schematic.document().grid_size, pos);
    let hit_pos = screen_to_schematic(viewport, pos);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    let target = if response.long_touched() {
        let target = touch_target(&response.ctx, state, viewport, symbol_context, pos);
        select_context_target(state, target)
    } else {
        forget_choices(&response.ctx);
        select_pointer_target(
            state,
            PointerHit::new(grid_pos, hit_pos),
            hit_radius,
            symbol_context,
            &response.ctx,
            viewport,
            pos,
        )
    };
    state.dialogs.interaction.context_target = Some((target, (grid_pos.x, grid_pos.y)));
    Some(pos)
}

/// What a long press opens the menu on: of the objects under the finger,
/// the one already selected, else the topmost. When two or more are there
/// the menu lists them first, so a finger can take the one underneath; a
/// second long press on the same spot then opens on that one (D1 sections
/// 9 and 10, audit G-9).
fn touch_target(
    ctx: &Context,
    state: &AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    position: egui::Pos2,
) -> Option<PointerTarget> {
    let query = ToolPointerView {
        design: super::schematic_design_view(state),
        snap_engine: &state.schematic.session.editor.snap_engine,
        can_edit: !state.schematic_edit_read_only(),
    }
    .selection_query(viewport, position);
    let candidates =
        super::pick_chooser::candidates_at(ctx, state, query, symbol_context, viewport);
    let choices = if candidates.len() >= 2 {
        super::pick_chooser::chooser_rows(state, &candidates)
    } else {
        Vec::new()
    };
    ctx.data_mut(|data| data.insert_temp(choices_id(), choices));
    let selection = &state.schematic.session.editor.selection;
    candidates
        .iter()
        .copied()
        .find(|target| target_is_selected(selection, *target))
        .or_else(|| candidates.first().copied())
}

/// Make one of the objects under the long press the selection.
fn choose_target(
    state: &mut AppState,
    view: &ContextMenuView,
    binding: &ContextMenuBinding,
    target: PointerTarget,
) {
    if context_menu_binding(state).as_ref() != Some(binding)
        || state.application_modal_open()
        || !view.choices.iter().any(|choice| choice.target == target)
    {
        return;
    }
    let request = rspice_schematic_editor::requests::EditorRequest {
        source: super::requests::editor_request_source(state),
        selection: state.schematic.session.editor.selection.clone(),
        action: rspice_schematic_editor::requests::EditorAction::SelectPointer {
            target: Some(target),
            additive: false,
            alt_held: false,
        },
    };
    super::requests::apply_editor_request(state, request);
}

fn select_pointer_target(
    state: &mut AppState,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) -> ContextTarget {
    let target = pointer_target(
        state,
        hit,
        hit_radius,
        symbol_context,
        ctx,
        viewport,
        pointer_pos,
    );
    select_context_target(state, target)
}

/// Select what the menu opens on, unless it is already in the selection,
/// and name it as the menu's target. A net highlight stays lit on an
/// instance the menu can descend into, so Descend carries it.
fn select_context_target(state: &mut AppState, target: Option<PointerTarget>) -> ContextTarget {
    let Some(target) = target else {
        return ContextTarget::Canvas;
    };
    if !super::requests::picks_instance_to_descend(state, Some(target)) {
        state.schematic.session.editor.net_highlight.clear();
    }
    match target {
        PointerTarget::Component(id) => {
            if !state.schematic.session.editor.selection.has_component(id) {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_component(id);
            }
            ContextTarget::Component(id)
        }
        PointerTarget::DesignNote(id) => {
            if !state.schematic.session.editor.selection.has_design_note(id) {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_design_note(id);
            }
            ContextTarget::Canvas
        }
        // A probe selects like any other annotation. `ContextTarget` has no
        // probe case, so the menu opens against the canvas.
        PointerTarget::Probe(id) => {
            if !state.schematic.session.editor.selection.has_probe(id) {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_probe(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::DocumentationShape(id) => {
            if !state
                .schematic
                .session
                .editor
                .selection
                .has_documentation_shape(id)
            {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_documentation_shape(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::NetLabel(id) => {
            if !state.schematic.session.editor.selection.has_net_label(id) {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_net_label(id);
            }
            // ContextTarget has no net-label variant; stable Selection identity
            // remains authoritative for properties and lifecycle actions.
            ContextTarget::Canvas
        }
        PointerTarget::BusTap(id) => {
            if !state.schematic.session.editor.selection.has_bus_tap(id) {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_bus_tap(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::Junction(position) => {
            if !state
                .schematic
                .session
                .editor
                .selection
                .has_junction(position)
            {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_junction(position);
            }
            ContextTarget::Canvas
        }
        PointerTarget::Bus(id) => {
            if !state.schematic.session.editor.selection.has_bus(id) {
                state.schematic.session.editor.selection.select_only_bus(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::Wire(id) => {
            if !state.schematic.session.editor.selection.has_wire(id) {
                state
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_wire(id);
            }
            ContextTarget::Wire(id)
        }
    }
}

fn point_midpoint(first: Point, last: Point) -> Point {
    Point::new(
        ((i64::from(first.x) + i64::from(last.x)) / 2) as i32,
        ((i64::from(first.y) + i64::from(last.y)) / 2) as i32,
    )
}

fn keyboard_target(
    state: &AppState,
    viewport: &Viewport,
    fallback_screen_pos: egui::Pos2,
) -> (ContextTarget, Point) {
    if let Some(id) = state.schematic.session.editor.selection.single_bus_tap()
        && let Some(tap) = state
            .schematic
            .document()
            .bus_taps
            .iter()
            .find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, tap.connection_point);
    }
    if let Some(id) = state.schematic.session.editor.selection.single_net_label()
        && let Some(label) = state
            .schematic
            .document()
            .net_labels
            .iter()
            .find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, label.pos);
    }
    if let Some(id) = state.schematic.session.editor.selection.single_bus()
        && let Some(bus) = state
            .schematic
            .document()
            .buses
            .iter()
            .find(|item| item.id == id)
        && let (Some(first), Some(last)) = (bus.points.first(), bus.points.last())
    {
        return (ContextTarget::Canvas, point_midpoint(*first, *last));
    }
    if let Some(id) = state.schematic.session.editor.selection.single_component()
        && let Some(component) = state
            .schematic
            .document()
            .components
            .iter()
            .find(|item| item.id == id)
    {
        return (ContextTarget::Component(id), component.pos);
    }
    if let Some(id) = state.schematic.session.editor.selection.single_wire()
        && let Some(wire) = state
            .schematic
            .document()
            .wires
            .iter()
            .find(|item| item.id == id)
        && let (Some(first), Some(last)) = (wire.points.first(), wire.points.last())
    {
        return (ContextTarget::Wire(id), point_midpoint(*first, *last));
    }
    if let Some(point) = state.schematic.session.editor.selection.single_junction() {
        return (ContextTarget::Canvas, point);
    }
    if let Some(id) = state
        .schematic
        .session
        .editor
        .selection
        .components
        .iter()
        .copied()
        .min()
        && let Some(component) = state
            .schematic
            .document()
            .components
            .iter()
            .find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, component.pos);
    }
    if let Some((x, y)) = state.dialogs.interaction.last_click_pos {
        return (ContextTarget::Canvas, Point::new(x, y));
    }
    (
        ContextTarget::Canvas,
        screen_to_grid(
            viewport,
            state.schematic.document().grid_size,
            fallback_screen_pos,
        ),
    )
}

fn selection_summary(state: &AppState, target: ContextTarget) -> String {
    let selection = &state.schematic.session.editor.selection;
    let count = selection.components.len()
        + selection.wires.len()
        + selection.wire_segments.len()
        + selection.wire_vertices.len()
        + selection.junctions.len()
        + selection.buses.len()
        + selection.bus_taps.len()
        + selection.net_labels.len()
        + selection.probes.len()
        + selection.design_notes.len()
        + selection.documentation_shapes.len();
    let path = format!("/{}", state.workspace.content.active_view.display_path());
    if count > 1 {
        return format!("{count} selected objects · {path}");
    }
    if let Some(id) = selection.single_bus_tap()
        && let Some(tap) = state
            .schematic
            .document()
            .bus_taps
            .iter()
            .find(|tap| tap.id == id)
    {
        return format!("bus tap · {} · {path}", tap.slice);
    }
    if let Some(id) = selection.single_bus()
        && let Some(bus) = state
            .schematic
            .document()
            .buses
            .iter()
            .find(|bus| bus.id == id)
    {
        return format!(
            "bus · {} · {path}",
            bus.declaration
                .as_ref()
                .map_or_else(|| "unnamed".to_owned(), ToString::to_string)
        );
    }
    if let Some(id) = selection.single_net_label()
        && let Some(label) = state
            .schematic
            .document()
            .net_labels
            .iter()
            .find(|label| label.id == id)
    {
        return format!(
            "net label · {} · {path}",
            if label.name.trim().is_empty() {
                "<unnamed>"
            } else {
                &label.name
            }
        );
    }
    if let Some(id) = selection.single_probe()
        && let Some(probe) = state
            .schematic
            .document()
            .probes
            .iter()
            .find(|probe| probe.id == id)
    {
        return format!("probe · {} · {path}", probe.reference);
    }
    if let Some(id) = selection.single_design_note()
        && let Some(note) = state
            .schematic
            .document()
            .design_notes
            .iter()
            .find(|note| note.id == id)
    {
        return format!("{} · {} · {path}", note.kind.label(), note.text);
    }
    if let Some(id) = selection.single_documentation_shape()
        && let Some(shape) = state
            .schematic
            .document()
            .documentation_shapes
            .iter()
            .find(|shape| shape.id == id)
    {
        return format!(
            "{} \u{b7} drawing / documentation \u{b7} {path}",
            shape.kind().label()
        );
    }
    let target = selection
        .single_component()
        .map(ContextTarget::Component)
        .or_else(|| selection.single_wire().map(ContextTarget::Wire))
        .unwrap_or(target);
    match target {
        ContextTarget::Component(id) => state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .map(|component| {
                let value = if component.value.trim().is_empty() {
                    component.library_cell.as_ref().map_or_else(
                        || component.kind.display_name(),
                        |binding| {
                            binding
                                .module_name
                                .as_deref()
                                .unwrap_or(binding.cell.as_str())
                        },
                    )
                } else {
                    component.value.trim()
                };
                format!("{} · {value} · {path}", component.name)
            })
            .unwrap_or_else(|| format!("Schematic object · {path}")),
        ContextTarget::Wire(id) => {
            let net = state
                .schematic
                .document()
                .wires
                .iter()
                .find(|wire| wire.id == id)
                .and_then(|wire| {
                    wire.points.iter().find_map(|point| {
                        state.simulation.source.cross_probe.net_at_in(
                            &state.workspace.content.active_view,
                            state.schematic.topology_version(),
                            super::sheet_visibility::active_sheet_id(state),
                            *point,
                        )
                    })
                })
                .map_or("wire", String::as_str);
            format!("wire · {net} · {path}")
        }
        ContextTarget::Canvas if count == 1 => format!("Selected schematic object · {path}"),
        ContextTarget::Canvas => format!("No object selected · {path}"),
    }
}

fn action_availability(action: ContextAction, state: &AppState) -> (bool, &'static str) {
    let selection = &state.schematic.session.editor.selection;
    // Components, complete wires, and explicit junctions are clipboard and
    // deletion objects. Wire segments and vertices remain edit handles.
    let has_live_component = state
        .schematic
        .document()
        .components
        .iter()
        .any(|component| selection.has_component(component.id));
    let has_live_wire = state
        .schematic
        .document()
        .wires
        .iter()
        .any(|wire| selection.has_wire(wire.id));
    let has_live_junction = state
        .schematic
        .document()
        .junctions
        .iter()
        .any(|junction| selection.has_junction(junction.pos));
    let has_live_bus = state
        .schematic
        .document()
        .buses
        .iter()
        .any(|bus| selection.has_bus(bus.id));
    let has_live_bus_tap = state
        .schematic
        .document()
        .bus_taps
        .iter()
        .any(|tap| selection.has_bus_tap(tap.id));
    let has_live_net_label = state
        .schematic
        .document()
        .net_labels
        .iter()
        .any(|label| selection.has_net_label(label.id));
    let has_live_probe = state
        .schematic
        .document()
        .probes
        .iter()
        .any(|probe| selection.has_probe(probe.id));
    let has_live_design_note = state
        .schematic
        .document()
        .design_notes
        .iter()
        .any(|note| selection.has_design_note(note.id));
    let has_live_documentation_shape = state
        .schematic
        .document()
        .documentation_shapes
        .iter()
        .any(|shape| selection.has_documentation_shape(shape.id));
    let has_copyable_object = has_live_component
        || has_live_wire
        || has_live_junction
        || has_live_bus
        || has_live_bus_tap
        || has_live_net_label
        || has_live_probe
        || has_live_design_note
        || has_live_documentation_shape;
    let all_whole_object_ids_are_live = selection.components.iter().all(|id| {
        state
            .schematic
            .document()
            .components
            .iter()
            .any(|component| component.id == *id)
    }) && selection.wires.iter().all(|id| {
        state
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.id == *id)
    }) && selection.buses.iter().all(|id| {
        state
            .schematic
            .document()
            .buses
            .iter()
            .any(|bus| bus.id == *id)
    }) && selection.bus_taps.iter().all(|id| {
        state
            .schematic
            .document()
            .bus_taps
            .iter()
            .any(|tap| tap.id == *id)
    }) && selection.net_labels.iter().all(|id| {
        state
            .schematic
            .document()
            .net_labels
            .iter()
            .any(|label| label.id == *id)
    }) && selection.probes.iter().all(|id| {
        state
            .schematic
            .document()
            .probes
            .iter()
            .any(|probe| probe.id == *id)
    }) && selection.design_notes.iter().all(|id| {
        state
            .schematic
            .document()
            .design_notes
            .iter()
            .any(|note| note.id == *id)
    }) && selection.documentation_shapes.iter().all(|id| {
        state
            .schematic
            .document()
            .documentation_shapes
            .iter()
            .any(|shape| shape.id == *id)
    });
    let all_junctions_are_live = selection.junctions.iter().all(|selected| {
        state
            .schematic
            .document()
            .junctions
            .iter()
            .any(|junction| junction.pos == selected.pos)
    });
    let has_wire_sub_object =
        !selection.wire_segments.is_empty() || !selection.wire_vertices.is_empty();
    let copyable_objects_only = has_copyable_object
        && all_whole_object_ids_are_live
        && all_junctions_are_live
        && !has_wire_sub_object;
    // Junction-only paste requires a separately chosen valid intersection, so
    // fixed-offset Duplicate intentionally stays unavailable for that case.
    let duplicable_objects_only = (has_live_component
        || has_live_wire
        || has_live_bus
        || has_live_bus_tap
        || has_live_net_label
        || has_live_probe
        || has_live_design_note
        || has_live_documentation_shape)
        && all_whole_object_ids_are_live
        && all_junctions_are_live
        && !has_wire_sub_object;
    let deletable_objects_only = (has_copyable_object || has_live_junction)
        && all_whole_object_ids_are_live
        && all_junctions_are_live
        && !has_wire_sub_object;
    let writable = !state.schematic.session.read_only && !state.active_view_read_only();
    match action {
        ContextAction::Properties => (
            crate::workbench::app::selected_object_properties_available(state),
            "Select one editable component, bus, bus tap, net label, design note, or documentation shape to open its properties",
        ),
        // While a part is being placed, or Move carries the selection, these
        // turn what floats, as R and M do (G-12: a finger has no R or M).
        ContextAction::Rotate | ContextAction::Mirror | ContextAction::MirrorVertical => (
            writable && (has_copyable_object || something_floats(state)),
            "Select at least one editable object",
        ),
        ContextAction::Arrange => (
            ARRANGE_ENTRIES.iter().any(|entry| {
                matches!(entry, ContextEntry::Command(command)
                    if action_availability(command.action, state).0)
            }),
            "Select at least one editable object",
        ),
        // The Design menu's availability for the same command.
        ContextAction::ArrangeCommand(ArrangeAction::SnapToGrid) => (
            !state.schematic_edit_read_only()
                && !state.schematic.session.editor.selection.is_empty(),
            "Select at least one editable object",
        ),
        ContextAction::ArrangeCommand(arrangement) => match arrange_layout(arrangement) {
            Some(layout) => match selection_layout_availability(state, layout) {
                Ok(()) => (true, ""),
                Err(_) => (
                    false,
                    "Select at least two editable objects to line up or space out",
                ),
            },
            None => (
                writable && (has_copyable_object || something_floats(state)),
                "Select at least one editable object",
            ),
        },
        ContextAction::ZoomBox => (
            matches!(
                state.workspace.content.active_view_type(),
                ViewType::Schematic | ViewType::Testbench
            ),
            "The zoom box frames a schematic or testbench canvas",
        ),
        ContextAction::AdoptStimulus
        | ContextAction::ReadoptStimulus
        | ContextAction::SaveStimulus
        | ContextAction::OpenStimulusDefinition => stimulus::availability(action, state),
        ContextAction::Cut => (
            writable && copyable_objects_only,
            "Select at least one editable component, wire, bus, tap, junction, net label, probe, design note, or documentation shape",
        ),
        ContextAction::Copy => (
            copyable_objects_only,
            "Select at least one component, wire, bus, tap, junction, net label, probe, design note, or documentation shape",
        ),
        ContextAction::Paste => (
            writable && !state.ui.schematic_clipboard.is_empty(),
            "Copy or cut something to paste first",
        ),
        ContextAction::Duplicate => (
            writable && duplicable_objects_only,
            "Select at least one editable component, wire, bus, tap, net label, probe, design note, or documentation shape",
        ),
        ContextAction::Delete => (
            writable && deletable_objects_only,
            "Select at least one editable component, wire, bus, tap, junction, net label, probe, or design note",
        ),
        ContextAction::DescendHierarchy => (
            state.selected_hierarchy_master().is_some(),
            "Select one instance with a resolved schematic master",
        ),
        ContextAction::OpenVerilogAView => (
            state.selected_veriloga_source().is_some(),
            "Select one Verilog-A instance whose view owns its module's source",
        ),
        ContextAction::NewVerilogADevice => (
            new_veriloga_device_available(state),
            "Make a writable design library available",
        ),
        ContextAction::UpdateInstanceInterface => (
            writable
                && state.schematic.selected_instance_interface_is_stale(
                    &state.library_manager,
                    &state.workspace.content.schematic_buffers,
                ),
            "Select one instance whose master interface changed after it was placed",
        ),
        // The command owns whether a replacement can be made at all — one
        // instance, editable, and a different ready master that preserves the
        // connected terminal contract. Restating any of that here would offer
        // the row where the dialog would immediately refuse.
        ContextAction::ReplaceInstance => (
            replace_instance_available(state),
            "Select one editable instance that a different ready master can stand in for",
        ),
        ContextAction::CreateHierarchy => (
            crate::workbench::app::create_hierarchy_available(state),
            "Select one or more complete editable instances and no partial objects",
        ),
        ContextAction::CreateSymbolFromPorts => (
            writable
                && !state.schematic.interface_ports().is_empty()
                && state
                    .library_manager
                    .libraries_sorted()
                    .iter()
                    .any(|library| !library.read_only),
            "Add at least one schematic port and make a writable design library available",
        ),
        ContextAction::PageSetup => (
            writable
                && matches!(
                    state.workspace.content.active_view_type(),
                    ViewType::Schematic | ViewType::Testbench
                ),
            "Page setup requires a writable schematic or testbench",
        ),
        ContextAction::FitContent => (
            matches!(
                state.workspace.content.active_view_type(),
                ViewType::Schematic | ViewType::Testbench
            ),
            "Fit is available on a schematic or testbench canvas",
        ),
        // The locator is the one owner of why a jump cannot be made, so this
        // row explains itself with the reason the command would report.
        ContextAction::ShowInNetlist => {
            let blocked = state.selected_instance_netlist_block();
            (
                blocked.is_none(),
                blocked.unwrap_or("Select one instance the generated netlist states"),
            )
        }
        ContextAction::Probe => (
            writable,
            "The active schematic view is read-only; reopen it in an editable context to place a probe",
        ),
        ContextAction::OperatingPoint => (
            operating_point_available(state),
            "Run a DC operating-point analysis with device OP reporting first",
        ),
        // A choice row is not a catalog command; `choose_target` checks it
        // against the rows the menu showed.
        ContextAction::Choose(_) => (false, "Long-press the objects to choose among them"),
    }
}

fn execute_context_action(
    action: ContextAction,
    state: &mut AppState,
    click_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    retain_selection_on_active_sheet(state);
    match action {
        ContextAction::Properties => {
            crate::workbench::app::open_selected_object_properties(state);
        }
        ContextAction::Rotate => {
            turn(state, symbol_context, ArrangeAction::RotateClockwise);
        }
        ContextAction::Mirror => {
            turn(state, symbol_context, ArrangeAction::MirrorHorizontal);
        }
        ContextAction::MirrorVertical => {
            turn(state, symbol_context, ArrangeAction::MirrorVertical);
        }
        // A page turn, which the menu takes itself.
        ContextAction::Arrange => {}
        ContextAction::ArrangeCommand(ArrangeAction::SnapToGrid) => {
            super::grid_snap_selection::run_snap_selection_to_grid(state, symbol_context);
        }
        ContextAction::ArrangeCommand(arrangement) => match arrange_layout(arrangement) {
            Some(layout) => match apply_selection_layout(state, symbol_context, layout) {
                Ok(true) => {}
                Ok(false) => {
                    state.prompt(format!("{} produced no geometry change.", layout.label()));
                }
                Err(error) => {
                    state.prompt(format!("{} was not applied: {error}.", layout.label()));
                }
            },
            None => turn(state, symbol_context, arrangement),
        },
        ContextAction::ZoomBox => {
            state.schematic.session.editor.zoom_box.arm();
            state.prompt("Drag a box to zoom to it.");
        }
        ContextAction::AdoptStimulus
        | ContextAction::ReadoptStimulus
        | ContextAction::SaveStimulus
        | ContextAction::OpenStimulusDefinition => stimulus::execute(action, state),
        ContextAction::Cut => {
            state.cut_schematic_selection();
        }
        ContextAction::Copy => {
            state.copy_active_schematic_selection();
        }
        ContextAction::Paste => {
            let refusal = match state.paste_schematic_clipboard_at(click_pos) {
                Ok(true) => None,
                Ok(false) => Some("Nothing was pasted.".to_owned()),
                Err(reason) => Some(reason),
            };
            if let Some(refusal) = refusal {
                state.prompt(refusal);
            }
        }
        ContextAction::Duplicate => {
            let step = state.schematic_grid_step();
            state.duplicate_schematic_selection_at(click_pos + step);
        }
        ContextAction::Delete => {
            state.delete_schematic_selection();
        }
        ContextAction::DescendHierarchy | ContextAction::OpenVerilogAView => {
            state.open_selected_instance_master();
        }
        ContextAction::NewVerilogADevice => {
            open_new_veriloga_device_form(state);
        }
        ContextAction::UpdateInstanceInterface => {
            let outcome = state.schematic.update_selected_instance_interface(
                &state.library_manager,
                &state.workspace.content.schematic_buffers,
            );
            state.prompt(match outcome {
                Ok(summary) => summary,
                Err(error) => error.to_string(),
            });
        }
        ContextAction::ReplaceInstance => open_replace_instance_dialog(state),
        ContextAction::CreateHierarchy => {
            crate::workbench::app::open_create_hierarchy_dialog(state);
        }
        ContextAction::CreateSymbolFromPorts => {
            crate::workbench::app::open_create_model_bound_symbol_dialog(state);
        }
        ContextAction::PageSetup => {
            crate::workbench::app::open_drawing_sheet_setup_for_state(state);
        }
        ContextAction::FitContent => {
            state.schematic.session.editor.needs_fit = true;
            state.schematic.session.editor.needs_drawing_sheet_fit = false;
        }
        ContextAction::ShowInNetlist => state.show_selected_instance_in_netlist(),
        ContextAction::Probe => state.schematic.arm_tool(Tool::Probe),
        ContextAction::OperatingPoint => open_operating_point(state),
        ContextAction::Choose(_) => {}
    }
}

fn open_operating_point(state: &mut AppState) {
    // The inspector renders the *selected* analysis, so the hop selects the
    // operating point before it routes. Without this a run of [OP, TRAN] with
    // the transient selected landed on "not a DC operating-point result" —
    // the workspace and the viewer were right and the one thing that decides
    // what they show was left wherever the reader had put it.
    let clicked = clicked_instance_name(state);
    let target = operating_point_analysis_index(state, clicked.as_deref());
    if let Some(index) = target {
        state.simulation.select_analysis(index);
    }
    // The device under the pointer is what this hop is about. The Op inspector
    // reads one device-name selection — its docbar filter — so the hop writes
    // that, exactly as the reverse direction writes the schematic selection.
    //
    // Written unconditionally, including as an empty filter: a device the
    // selected report does not hold used to leave the *previous* device's
    // filter in place, so the hop opened on some other instance's row and read
    // as if it had worked.
    state.ui.results.session.op_filter = clicked
        .filter(|name| {
            target.is_some_and(|index| analysis_reports_device(state, index, name.as_str()))
        })
        .unwrap_or_default();
    state.ui.results.session.viewer = ResultViewer::Op;
    state.workbench.activate(Workspace::Results);
}

/// The clicked instance's exact engine name, when exactly one is selected: the
/// element the engine knows it by, inside the occurrence the tab is editing.
///
/// The name comparison downstream is the one
/// [`crate::workbench::documents::result_document::op_inspector`] uses to walk
/// the other way, so the two directions agree on what "this device" means.
fn clicked_instance_name(state: &AppState) -> Option<String> {
    let id = state
        .schematic
        .session
        .editor
        .selection
        .single_component()?;
    let element = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)?
        .engine_element_name();
    let occurrence = state.workspace.content.occurrence_path();
    if occurrence.is_root() {
        return Some(element);
    }
    occurrence.engine_name_for(&element).ok()
}

/// The run's operating point, preferring one whose device report holds `device`.
///
/// A run may hold several, and the one worth landing on is the one that can
/// answer the question the click asked. Falling back to the first keeps the hop
/// working for a device no report names — the inspector then opens unfiltered,
/// which is a whole report rather than an empty one.
fn operating_point_analysis_index(state: &AppState, device: Option<&str>) -> Option<usize> {
    let run = state.simulation.active_run()?;
    let is_operating_point = |analysis: &crate::state::AnalysisResult| {
        analysis.analysis_type == crate::state::AnalysisType::DcOp
    };
    if let Some(device) = device
        && let Some(index) = run.analyses.iter().position(|analysis| {
            is_operating_point(analysis)
                && analysis.device_op.as_ref().is_some_and(|report| {
                    report
                        .entries
                        .iter()
                        .any(|entry| entry.name.eq_ignore_ascii_case(device))
                })
        })
    {
        return Some(index);
    }
    run.analyses.iter().position(is_operating_point)
}

/// Whether one analysis of the active run reports `name`.
fn analysis_reports_device(state: &AppState, index: usize, name: &str) -> bool {
    state.simulation.active_run().is_some_and(|run| {
        run.analyses.get(index).is_some_and(|analysis| {
            analysis.device_op.as_ref().is_some_and(|report| {
                report
                    .entries
                    .iter()
                    .any(|entry| entry.name.eq_ignore_ascii_case(name))
            })
        })
    })
}

fn operating_point_available(state: &AppState) -> bool {
    state.simulation.active_run().is_some_and(|run| {
        run.analyses.iter().any(|analysis| {
            analysis
                .device_op
                .as_ref()
                .is_some_and(|report| !report.is_empty())
        })
    })
}

fn context_menu_binding(state: &AppState) -> Option<ContextMenuBinding> {
    let (target, (x, y)) = state.dialogs.interaction.context_target?;
    Some(ContextMenuBinding {
        source: super::requests::editor_request_source(state),
        selection: state.schematic.session.editor.selection.clone(),
        tool: state.schematic.session.editor.tool,
        target,
        click_pos: Point::new(x, y),
    })
}

fn context_menu_view(state: &AppState, ctx: &Context) -> Option<ContextMenuView> {
    let binding = context_menu_binding(state)?;
    let resolve = |entries: &[ContextEntry]| {
        entries
            .iter()
            .map(|entry| resolve_entry(state, ctx, *entry))
            .collect::<Vec<_>>()
    };
    Some(ContextMenuView {
        summary: selection_summary(state, binding.target),
        entries: resolve(&menu_entries(state, binding.target)),
        arrange: resolve(ARRANGE_ENTRIES),
        binding,
        choices: ctx
            .data(|data| data.get_temp::<Vec<ChooserRow>>(choices_id()))
            .unwrap_or_default(),
    })
}

/// One catalog entry as the menu draws it: available or not and why, with
/// its key.
fn resolve_entry(
    state: &AppState,
    ctx: &Context,
    entry: ContextEntry,
) -> context_surface::ContextEntry {
    let ContextEntry::Command(command) = entry else {
        return context_surface::ContextEntry::Separator;
    };
    let (enabled, disabled_reason) = action_availability(command.action, state);
    let shortcut = command
        .shortcut_command
        .map_or_else(String::new, |product_command| {
            state.ui.preferences.shortcuts().resolved_label(
                product_command,
                crate::workbench::app_state::runtime_command_platform(ctx),
                ctx.os(),
            )
        });
    context_surface::ContextEntry::Command(context_surface::ContextCommand {
        action: command.action,
        icon: command.icon,
        label: command.label,
        shortcut,
        enabled,
        disabled_reason,
    })
}

fn apply_context_request(
    state: &mut AppState,
    request: ContextMenuRequest,
    symbol_context: &SchematicSymbolContext,
) {
    let offered = |entries: &[ContextEntry]| {
        entries.iter().any(|entry| {
            matches!(entry, ContextEntry::Command(command) if command.action == request.action)
        })
    };
    if context_menu_binding(state).as_ref() != Some(&request.binding)
        || state.application_modal_open()
        || !action_availability(request.action, state).0
        || !(offered(&menu_entries(state, request.binding.target)) || offered(ARRANGE_ENTRIES))
    {
        return;
    }
    execute_context_action(
        request.action,
        state,
        request.binding.click_pos,
        symbol_context,
    );
}

#[cfg(test)]
mod tests;
