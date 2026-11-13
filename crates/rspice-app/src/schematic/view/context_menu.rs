//! The selection context menu for the schematic canvas.
//!
//! Right-click, Shift+F10 and a touch long-press all open the same command
//! contract. Every visible command is backed by a real schematic or results
//! operation; a command that cannot be taken right now stays visibly disabled
//! with an explanation, and only one that could never apply to the kind of
//! object selected is left out (`menu_entries`).

use egui::{Context, Response};

use crate::state::command_catalog::CommandKey;
use crate::state::{Point, Tool, ViewType};
use crate::workbench::app_state::ContextTarget;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::Workspace;
use crate::workbench::{
    app::{
        StimulusLinkMode, copy_active_schematic_selection, cut_schematic_selection,
        delete_schematic_selection, duplicate_schematic_selection_at,
        new_veriloga_device_available_in, open_replace_instance_dialog, open_stimulus_link,
        paste_schematic_clipboard_at, replace_instance_available, schematic_grid_step,
        show_stimulus_definition,
    },
    design::commands::DesignCommand,
    framework::command_registry::ids,
    framework::requests::RevealSheetSection,
};

use super::SchematicSymbolContext;
use super::coordinates::{screen_to_grid, screen_to_schematic};
use super::interaction::{PointerHit, PointerTarget, pointer_target};
use super::parts::{CanvasFacts, CanvasParts};
use super::requests::DesignSurfaceRequest;
use super::selection_transform::{SelectionTransform, transform_selection_on_active_sheet};
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
    /// The command whose shortcut the row shows.
    shortcut_command: Option<CommandKey>,
}

type ContextEntry = context_surface::ContextEntry<ContextCommand>;

const CONTEXT_ENTRIES: &[ContextEntry] = &[
    // What can be started on a bare sheet: only the empty canvas offers them.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::PlaceInstance,
        icon: ContextIcon::Instance,
        label: "Place instance…",
        shortcut_command: Some(DesignCommand::PlaceInstance.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::PlaceWire,
        icon: ContextIcon::Wire,
        label: "Draw wire",
        shortcut_command: Some(DesignCommand::PlaceWire.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::PlaceLabel,
        icon: ContextIcon::Label,
        label: "Place net label",
        shortcut_command: Some(DesignCommand::PlaceLabel.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::PlaceText,
        icon: ContextIcon::Text,
        label: "Place text or note…",
        shortcut_command: Some(DesignCommand::PlaceText.key()),
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Properties,
        icon: ContextIcon::Sliders,
        label: "Object properties…",
        shortcut_command: Some(DesignCommand::ObjectProperties.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Rotate,
        icon: ContextIcon::Rotate,
        label: "Rotate 90°",
        shortcut_command: Some(DesignCommand::RotateSelection.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Mirror,
        icon: ContextIcon::Mirror,
        label: "Mirror",
        shortcut_command: Some(DesignCommand::MirrorSelectionHorizontal.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::MirrorVertical,
        icon: ContextIcon::MirrorVertical,
        label: "Mirror vertically",
        shortcut_command: Some(DesignCommand::MirrorSelectionVertical.key()),
    }),
    // The Design menu's Arrange submenu, as a page of this menu: a phone
    // shows no menu bar, and Align and Distribute have no key.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Arrange,
        icon: ContextIcon::Arrange,
        label: "Arrange",
        shortcut_command: None,
    }),
    // Only in the menu of a selection that holds a resistor, capacitor or
    // inductor; see `menu_entries`.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::SnapToStandardValue,
        icon: ContextIcon::Grid,
        label: "Snap to standard value",
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
        shortcut_command: Some(Command::Cut.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Copy,
        icon: ContextIcon::Copy,
        label: "Copy selection",
        shortcut_command: Some(Command::Copy.key()),
    }),
    // Where the menu was opened, which is the point a reader right-clicks
    // to say "here".
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Paste,
        icon: ContextIcon::Copy,
        label: "Paste here",
        shortcut_command: Some(Command::Paste.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Duplicate,
        icon: ContextIcon::Copy,
        label: "Duplicate",
        shortcut_command: Some(Command::Duplicate.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Delete,
        icon: ContextIcon::Trash,
        label: "Delete",
        shortcut_command: Some(Command::Delete.key()),
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
        shortcut_command: Some(DesignCommand::DescendHierarchyDirect.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::OpenVerilogAView,
        icon: ContextIcon::VerilogA,
        label: "Open Verilog-A view",
        shortcut_command: Some(DesignCommand::DescendHierarchyDirect.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::EditSymbolInPlace,
        icon: ContextIcon::Hierarchy,
        label: "Edit symbol in place",
        shortcut_command: Some(DesignCommand::EditSymbolInPlace.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::NewVerilogADevice,
        icon: ContextIcon::VerilogA,
        label: "New Verilog-A device…",
        shortcut_command: Some(ids::NEW_VERILOGA_DEVICE),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::UpdateInstanceInterface,
        icon: ContextIcon::Hierarchy,
        label: "Update instance interface",
        shortcut_command: Some(DesignCommand::UpdateInstanceInterface.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ReplaceInstance,
        icon: ContextIcon::Hierarchy,
        label: "Replace instance…",
        shortcut_command: Some(DesignCommand::ReplaceInstance.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::CreateHierarchy,
        icon: ContextIcon::Hierarchy,
        label: "Create hierarchy from selection…",
        shortcut_command: Some(DesignCommand::CreateHierarchy.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::CreateSymbolFromPorts,
        icon: ContextIcon::Hierarchy,
        label: "Create symbol from schematic ports…",
        shortcut_command: None,
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::SheetProperties,
        icon: ContextIcon::Sheet,
        label: "Sheet properties",
        shortcut_command: Some(ids::SHEET_PROPERTIES),
    }),
    // Fitting the drawing sheet is not here. `Command::ZoomFit` already has a
    // toolbar button on this canvas, a status-bar route, a mobile canvas
    // control and the `F` key; a selection menu that fits without scrolling is
    // worth more than a sixth route to it.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::FitContent,
        icon: ContextIcon::Fit,
        label: "Fit schematic content",
        shortcut_command: Some(DesignCommand::FitSchematicContent.key()),
    }),
    // Only on the empty canvas's menu: a finger has no Z key, and the next
    // drag after this row frames the view.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ZoomBox,
        icon: ContextIcon::ZoomBox,
        label: "Zoom box",
        shortcut_command: Some(Command::ZoomBox.key()),
    }),
    ContextEntry::Separator,
    ContextEntry::Command(ContextCommand {
        action: ContextAction::ShowInNetlist,
        icon: ContextIcon::Code,
        label: "Show in netlist",
        shortcut_command: Some(ids::SHOW_IN_NETLIST),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::Probe,
        icon: ContextIcon::Probe,
        label: "Add voltage or current probe…",
        shortcut_command: Some(DesignCommand::PlaceProbe.key()),
    }),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::OperatingPoint,
        icon: ContextIcon::Waveform,
        label: "Open operating point",
        shortcut_command: Some(ids::RESULT_OPERATING_POINT),
    }),
];

/// The one independent source the menu is about, when that is all that is
/// selected.
fn clicked_source<'a>(state: &CanvasFacts<'a>) -> Option<&'a crate::state::Component> {
    let id = state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_component()?;
    state
        .model
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

fn menu_subject(state: &CanvasFacts<'_>, target: ContextTarget) -> MenuSubject {
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

/// A click on empty sheet with nothing selected.
fn is_bare_canvas(state: &CanvasFacts<'_>, target: ContextTarget) -> bool {
    menu_subject(state, target) == MenuSubject::Canvas
        && state.model.schematic.session.editor.selection.is_empty()
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
        label: command.declared_spec().label,
        shortcut_command: Some(command.key()),
    })
}

/// The product command an Arrange row stands for.
const fn arrange_command(action: ArrangeAction) -> DesignCommand {
    match action {
        ArrangeAction::RotateClockwise => DesignCommand::RotateSelection,
        ArrangeAction::RotateCounterClockwise => DesignCommand::RotateSelectionCounterClockwise,
        ArrangeAction::MirrorHorizontal => DesignCommand::MirrorSelectionHorizontal,
        ArrangeAction::MirrorVertical => DesignCommand::MirrorSelectionVertical,
        ArrangeAction::AlignLeft => DesignCommand::AlignSelectionLeft,
        ArrangeAction::AlignCenter => DesignCommand::AlignSelectionCenter,
        ArrangeAction::AlignRight => DesignCommand::AlignSelectionRight,
        ArrangeAction::AlignTop => DesignCommand::AlignSelectionTop,
        ArrangeAction::AlignMiddle => DesignCommand::AlignSelectionMiddle,
        ArrangeAction::AlignBottom => DesignCommand::AlignSelectionBottom,
        ArrangeAction::DistributeHorizontal => DesignCommand::DistributeSelectionHorizontal,
        ArrangeAction::DistributeVertical => DesignCommand::DistributeSelectionVertical,
        ArrangeAction::SnapToGrid => DesignCommand::SnapSelectionToGrid,
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
fn turn(
    state: &mut CanvasParts<'_>,
    symbol_context: &SchematicSymbolContext,
    action: ArrangeAction,
) {
    let Some((selection, floating)) = arrange_turn(action) else {
        return;
    };
    if !turn_what_floats(state, floating) {
        transform_selection_on_active_sheet(
            state.model,
            state.workbench,
            symbol_context,
            selection,
        );
    }
}

/// The rows a menu opened on the bare canvas, with nothing selected, leaves
/// out: they act on a selection there is none of, so they could only ever be
/// disabled there.
fn selection_only(action: ContextAction, state: &CanvasFacts<'_>) -> bool {
    match action {
        ContextAction::Properties
        | ContextAction::Rotate
        | ContextAction::Mirror
        | ContextAction::MirrorVertical
        | ContextAction::Arrange
        | ContextAction::Cut
        | ContextAction::Copy
        | ContextAction::Duplicate
        | ContextAction::Delete
        | ContextAction::CreateHierarchy
        | ContextAction::ShowInNetlist => true,
        ContextAction::Paste => state.design.schematic_clipboard.is_empty(),
        _ => false,
    }
}

/// Whether the catalog's instance-master slot holds `action` for `subject`.
fn master_slot_shows(action: ContextAction, subject: MenuSubject) -> bool {
    match action {
        ContextAction::DescendHierarchy => subject == MenuSubject::Object,
        ContextAction::OpenVerilogAView => subject == MenuSubject::VerilogAInstance,
        // A symbol from the sheet's own ports is the sheet's verb, so it is
        // on the sheet's menu; an instance's menu offers its symbol instead.
        ContextAction::NewVerilogADevice
        | ContextAction::ZoomBox
        | ContextAction::CreateSymbolFromPorts
        | ContextAction::PlaceInstance
        | ContextAction::PlaceWire
        | ContextAction::PlaceLabel
        | ContextAction::PlaceText => subject == MenuSubject::Canvas,
        ContextAction::UpdateInstanceInterface
        | ContextAction::ReplaceInstance
        | ContextAction::EditSymbolInPlace => {
            matches!(subject, MenuSubject::Object | MenuSubject::VerilogAInstance)
        }
        _ => true,
    }
}

/// Whether the selection holds a part whose value a series rounds: a menu
/// opened on anything else has no use for the row.
fn selection_holds_a_standard_value(state: &CanvasFacts<'_>) -> bool {
    let selection = &state.model.schematic.session.editor.selection;
    state
        .model
        .schematic
        .document()
        .components
        .iter()
        .any(|part| {
            selection.components.contains(&part.id)
                && super::standard_value_snap::takes_standard_value(part.kind)
        })
}

/// The entries the menu is made of, for what it was opened on.
///
/// The catalog is one list, and almost all of it is every target's: a verb that
/// cannot be taken right now stays where it is, disabled, and says why. What is
/// left out is a verb that could never apply to the *kind* of object the menu
/// is about. Only the instance-master slot varies. An instance offers its
/// master's four rows; a Verilog-A instance's Descend row opens its module's
/// view; the empty canvas, which has no master, offers a new Verilog-A device
/// in their place, and a symbol from its own ports. A placed source gets the
/// stimulus-library group instead of the master rows, which is also what lets
/// the group in without turning the menu into a scroller. A rule with nothing
/// left under it goes too.
fn menu_entries(state: &CanvasFacts<'_>, target: ContextTarget) -> Vec<ContextEntry> {
    let subject = menu_subject(state, target);
    let bare_canvas = is_bare_canvas(state, target);
    let mut entries: Vec<ContextEntry> = Vec::with_capacity(CONTEXT_ENTRIES.len());
    for entry in CONTEXT_ENTRIES {
        if let ContextEntry::Command(command) = entry
            && bare_canvas
            && selection_only(command.action, state)
        {
            continue;
        }
        let shown = match entry {
            ContextEntry::Separator => {
                matches!(entries.last(), Some(ContextEntry::Command(_)))
            }
            ContextEntry::Command(command) if stimulus::owns(command.action) => {
                stimulus::shown(command.action, state)
            }
            ContextEntry::Command(command)
                if command.action == ContextAction::SnapToStandardValue =>
            {
                selection_holds_a_standard_value(state)
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
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
    _routing_was_active: bool,
    symbol_context: &SchematicSymbolContext,
) {
    state.model.retain_selection_on_active_sheet();
    #[cfg(target_arch = "wasm32")]
    let browser_keyboard_open =
        crate::workbench::browser::accessibility::take_schematic_context_menu_request();
    #[cfg(not(target_arch = "wasm32"))]
    let browser_keyboard_open = false;
    let opening = context_surface::opening(response, browser_keyboard_open);
    if !opening.needs_contents() {
        context_surface::close(response);
        forget_choices(&response.ctx);
        state.design.interaction.context_target = None;
        return;
    }
    let anchor = if opening.secondary_clicked {
        capture_pointer_target(response, state, viewport, symbol_context)
    } else {
        if opening.keyboard_open {
            forget_choices(&response.ctx);
            let (target, click_pos) =
                keyboard_target(&state.facts(), viewport, response.rect.center());
            state.design.interaction.context_target = Some((target, (click_pos.x, click_pos.y)));
        }
        None
    };
    let Some(view) = context_menu_view(&state.facts(), &response.ctx) else {
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
        state.design.interaction.context_target = None;
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
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) -> Option<egui::Pos2> {
    let pos = response.interact_pointer_pos()?;
    let grid_pos = screen_to_grid(viewport, state.model.schematic.document().grid_size, pos);
    let hit_pos = screen_to_schematic(viewport, pos);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    let target = if response.long_touched() {
        let target = touch_target(&response.ctx, &state.facts(), viewport, symbol_context, pos);
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
    state.design.interaction.context_target = Some((target, (grid_pos.x, grid_pos.y)));
    Some(pos)
}

/// What a long press opens the menu on: of the objects under the finger,
/// the one already selected, else the topmost. When two or more are there
/// the menu lists them first, so a finger can take the one underneath; a
/// second long press on the same spot then opens on that one (D1 sections
/// 9 and 10, audit G-9).
fn touch_target(
    ctx: &Context,
    state: &CanvasFacts<'_>,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    position: egui::Pos2,
) -> Option<PointerTarget> {
    let query = ToolPointerView {
        design: state.design_view(),
        snap_engine: &state.model.schematic.session.editor.snap_engine,
        can_edit: !state.schematic_edit_read_only(),
    }
    .selection_query(viewport, position);
    let candidates = super::pick_chooser::candidates_at(
        ctx,
        state.model,
        state.design,
        query,
        symbol_context,
        viewport,
    );
    let choices = if candidates.len() >= 2 {
        super::pick_chooser::chooser_rows(state.model, &candidates)
    } else {
        Vec::new()
    };
    ctx.data_mut(|data| data.insert_temp(choices_id(), choices));
    let selection = &state.model.schematic.session.editor.selection;
    candidates
        .iter()
        .copied()
        .find(|target| target_is_selected(selection, *target))
        .or_else(|| candidates.first().copied())
}

/// Make one of the objects under the long press the selection.
fn choose_target(
    state: &mut CanvasParts<'_>,
    view: &ContextMenuView,
    binding: &ContextMenuBinding,
    target: PointerTarget,
) {
    if context_menu_binding(&state.facts()).as_ref() != Some(binding)
        || state.application_modal_open()
        || !view.choices.iter().any(|choice| choice.target == target)
    {
        return;
    }
    let request = rspice_schematic_editor::requests::EditorRequest {
        source: super::requests::editor_request_source(state),
        selection: state.model.schematic.session.editor.selection.clone(),
        action: rspice_schematic_editor::requests::EditorAction::SelectPointer {
            target: Some(target),
            additive: false,
            alt_held: false,
        },
    };
    super::requests::apply_editor_request(state, request);
}

fn select_pointer_target(
    state: &mut CanvasParts<'_>,
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
fn select_context_target(
    state: &mut CanvasParts<'_>,
    target: Option<PointerTarget>,
) -> ContextTarget {
    let Some(target) = target else {
        return ContextTarget::Canvas;
    };
    if !super::requests::picks_instance_to_descend(state.model, Some(target)) {
        state.model.schematic.session.editor.net_highlight.clear();
    }
    match target {
        PointerTarget::Component(id) => {
            if !state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_component(id)
            {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_component(id);
            }
            ContextTarget::Component(id)
        }
        PointerTarget::DesignNote(id) => {
            if !state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_design_note(id)
            {
                state
                    .model
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
            if !state.model.schematic.session.editor.selection.has_probe(id) {
                state
                    .model
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
                .model
                .schematic
                .session
                .editor
                .selection
                .has_documentation_shape(id)
            {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_documentation_shape(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::NetLabel(id) => {
            if !state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_net_label(id)
            {
                state
                    .model
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
            if !state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_bus_tap(id)
            {
                state
                    .model
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
                .model
                .schematic
                .session
                .editor
                .selection
                .has_junction(position)
            {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_junction(position);
            }
            ContextTarget::Canvas
        }
        PointerTarget::Bus(id) => {
            if !state.model.schematic.session.editor.selection.has_bus(id) {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_bus(id);
            }
            ContextTarget::Canvas
        }
        PointerTarget::Wire(id) => {
            if !state.model.schematic.session.editor.selection.has_wire(id) {
                state
                    .model
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
    state: &CanvasFacts<'_>,
    viewport: &Viewport,
    fallback_screen_pos: egui::Pos2,
) -> (ContextTarget, Point) {
    if let Some(id) = state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_bus_tap()
        && let Some(tap) = state
            .model
            .schematic
            .document()
            .bus_taps
            .iter()
            .find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, tap.connection_point);
    }
    if let Some(id) = state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_net_label()
        && let Some(label) = state
            .model
            .schematic
            .document()
            .net_labels
            .iter()
            .find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, label.pos);
    }
    if let Some(id) = state.model.schematic.session.editor.selection.single_bus()
        && let Some(bus) = state
            .model
            .schematic
            .document()
            .buses
            .iter()
            .find(|item| item.id == id)
        && let (Some(first), Some(last)) = (bus.points.first(), bus.points.last())
    {
        return (ContextTarget::Canvas, point_midpoint(*first, *last));
    }
    if let Some(id) = state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_component()
        && let Some(component) = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|item| item.id == id)
    {
        return (ContextTarget::Component(id), component.pos);
    }
    if let Some(id) = state.model.schematic.session.editor.selection.single_wire()
        && let Some(wire) = state
            .model
            .schematic
            .document()
            .wires
            .iter()
            .find(|item| item.id == id)
        && let (Some(first), Some(last)) = (wire.points.first(), wire.points.last())
    {
        return (ContextTarget::Wire(id), point_midpoint(*first, *last));
    }
    if let Some(point) = state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_junction()
    {
        return (ContextTarget::Canvas, point);
    }
    if let Some(id) = state
        .model
        .schematic
        .session
        .editor
        .selection
        .components
        .iter()
        .copied()
        .min()
        && let Some(component) = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .find(|item| item.id == id)
    {
        return (ContextTarget::Canvas, component.pos);
    }
    if let Some((x, y)) = state.design.interaction.last_click_pos {
        return (ContextTarget::Canvas, Point::new(x, y));
    }
    (
        ContextTarget::Canvas,
        screen_to_grid(
            viewport,
            state.model.schematic.document().grid_size,
            fallback_screen_pos,
        ),
    )
}

/// What the menu's header names, which on a bare canvas is nothing.
fn menu_summary(state: &CanvasFacts<'_>, target: ContextTarget) -> Option<String> {
    (!is_bare_canvas(state, target)).then(|| selection_summary(state, target))
}

fn selection_summary(state: &CanvasFacts<'_>, target: ContextTarget) -> String {
    let selection = &state.model.schematic.session.editor.selection;
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
    let path = format!(
        "/{}",
        state.model.workspace.content.active_view.display_path()
    );
    if count > 1 {
        return format!("{count} selected objects · {path}");
    }
    if let Some(id) = selection.single_bus_tap()
        && let Some(tap) = state
            .model
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
            .model
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
            .model
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
            .model
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
            .model
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
            .model
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
            .model
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
                .model
                .schematic
                .document()
                .wires
                .iter()
                .find(|wire| wire.id == id)
                .and_then(|wire| {
                    wire.points.iter().find_map(|point| {
                        state.model.simulation.source.cross_probe.net_at_in(
                            &state.model.workspace.content.active_view,
                            state.model.schematic.topology_version(),
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

fn action_availability(action: ContextAction, state: &CanvasFacts<'_>) -> (bool, &'static str) {
    let selection = &state.model.schematic.session.editor.selection;
    // Components, complete wires, and explicit junctions are clipboard and
    // deletion objects. Wire segments and vertices remain edit handles.
    let has_live_component = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .any(|component| selection.has_component(component.id));
    let has_live_wire = state
        .model
        .schematic
        .document()
        .wires
        .iter()
        .any(|wire| selection.has_wire(wire.id));
    let has_live_junction = state
        .model
        .schematic
        .document()
        .junctions
        .iter()
        .any(|junction| selection.has_junction(junction.pos));
    let has_live_bus = state
        .model
        .schematic
        .document()
        .buses
        .iter()
        .any(|bus| selection.has_bus(bus.id));
    let has_live_bus_tap = state
        .model
        .schematic
        .document()
        .bus_taps
        .iter()
        .any(|tap| selection.has_bus_tap(tap.id));
    let has_live_net_label = state
        .model
        .schematic
        .document()
        .net_labels
        .iter()
        .any(|label| selection.has_net_label(label.id));
    let has_live_probe = state
        .model
        .schematic
        .document()
        .probes
        .iter()
        .any(|probe| selection.has_probe(probe.id));
    let has_live_design_note = state
        .model
        .schematic
        .document()
        .design_notes
        .iter()
        .any(|note| selection.has_design_note(note.id));
    let has_live_documentation_shape = state
        .model
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
            .model
            .schematic
            .document()
            .components
            .iter()
            .any(|component| component.id == *id)
    }) && selection.wires.iter().all(|id| {
        state
            .model
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.id == *id)
    }) && selection.buses.iter().all(|id| {
        state
            .model
            .schematic
            .document()
            .buses
            .iter()
            .any(|bus| bus.id == *id)
    }) && selection.bus_taps.iter().all(|id| {
        state
            .model
            .schematic
            .document()
            .bus_taps
            .iter()
            .any(|tap| tap.id == *id)
    }) && selection.net_labels.iter().all(|id| {
        state
            .model
            .schematic
            .document()
            .net_labels
            .iter()
            .any(|label| label.id == *id)
    }) && selection.probes.iter().all(|id| {
        state
            .model
            .schematic
            .document()
            .probes
            .iter()
            .any(|probe| probe.id == *id)
    }) && selection.design_notes.iter().all(|id| {
        state
            .model
            .schematic
            .document()
            .design_notes
            .iter()
            .any(|note| note.id == *id)
    }) && selection.documentation_shapes.iter().all(|id| {
        state
            .model
            .schematic
            .document()
            .documentation_shapes
            .iter()
            .any(|shape| shape.id == *id)
    });
    let all_junctions_are_live = selection.junctions.iter().all(|selected| {
        state
            .model
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
    let writable = !state.model.schematic.session.read_only && !state.active_view_read_only();
    match action {
        ContextAction::Properties => (
            crate::workbench::app::selected_object_properties_available_in(
                &state.design.property_registry,
                state.model,
            ),
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
                && !state.model.schematic.session.editor.selection.is_empty(),
            "Select at least one editable object",
        ),
        ContextAction::ArrangeCommand(arrangement) => match arrange_layout(arrangement) {
            Some(layout) => match selection_layout_availability(
                state.model,
                state.design.schematic_visibility.review_markers,
                layout,
            ) {
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
                state.model.workspace.content.active_view_type(),
                ViewType::Schematic | ViewType::Testbench
            ),
            "The zoom box frames a schematic or testbench canvas",
        ),
        ContextAction::PlaceInstance
        | ContextAction::PlaceWire
        | ContextAction::PlaceLabel
        | ContextAction::PlaceText => (
            writable,
            "The active schematic view is read-only; reopen it in an editable context to add to it",
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
            writable && !state.design.schematic_clipboard.is_empty(),
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
        ContextAction::EditSymbolInPlace => match state.selected_symbol_in_place_target() {
            Ok(_) => (true, ""),
            Err(refusal) => (false, refusal.sentence()),
        },
        ContextAction::NewVerilogADevice => (
            new_veriloga_device_available_in(state.model),
            "Make a writable design library available",
        ),
        ContextAction::UpdateInstanceInterface => (
            writable
                && state.model.schematic.selected_instance_interface_is_stale(
                    &state.model.library_manager,
                    &state.model.workspace.content.schematic_buffers,
                ),
            "Select one instance whose master interface changed after it was placed",
        ),
        // The command owns whether a replacement can be made at all — one
        // instance, editable, and a different ready master that preserves the
        // connected terminal contract. Restating any of that here would offer
        // the row where the dialog would immediately refuse.
        ContextAction::ReplaceInstance => (
            replace_instance_available(&state.design.property_registry, state.model),
            "Select one editable instance that a different ready master can stand in for",
        ),
        ContextAction::CreateHierarchy => (
            state.model.create_hierarchy_available(),
            "Select one or more complete editable instances and no partial objects",
        ),
        ContextAction::CreateSymbolFromPorts => (
            writable
                && !state.model.schematic.interface_ports().is_empty()
                && state
                    .model
                    .library_manager
                    .libraries_sorted()
                    .iter()
                    .any(|library| !library.read_only),
            "Add at least one schematic port and make a writable design library available",
        ),
        ContextAction::SheetProperties => (
            writable
                && matches!(
                    state.model.workspace.content.active_view_type(),
                    ViewType::Schematic | ViewType::Testbench
                ),
            "Sheet properties require a writable schematic or testbench",
        ),
        ContextAction::FitContent => (
            matches!(
                state.model.workspace.content.active_view_type(),
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
        ContextAction::SnapToStandardValue => {
            let blocked = super::standard_value_snap::snap_block_reason(state.model);
            (blocked.is_none(), blocked.unwrap_or_default())
        }
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
    state: &mut CanvasParts<'_>,
    click_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    state.model.retain_selection_on_active_sheet();
    match action {
        ContextAction::Properties => {
            crate::workbench::app::open_selected_object_properties_in(
                &mut state.design.property_form_parts(),
                state.model,
                state.workbench,
            );
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
            super::grid_snap_selection::run_snap_selection_to_grid(
                state.model,
                state.workbench,
                symbol_context,
            );
        }
        ContextAction::ArrangeCommand(arrangement) => match arrange_layout(arrangement) {
            Some(layout) => match apply_selection_layout(
                state.model,
                state.workbench,
                state.design.schematic_visibility.review_markers,
                symbol_context,
                layout,
            ) {
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
            state.model.schematic.session.editor.zoom_box.arm();
            state.prompt("Drag a box to zoom to it.");
        }
        ContextAction::PlaceInstance => {
            state.workbench.navigator_visible = true;
            state.workbench.drawer = Some(crate::workbench::state::Drawer::Navigator);
            state.design.show_component_shelf();
            state
                .workbench
                .outbox
                .push(crate::workbench::framework::requests::FocusPlacementSearch);
        }
        ContextAction::PlaceWire => state.model.schematic.arm_tool(Tool::Wire),
        ContextAction::PlaceLabel => state.model.schematic.arm_tool(Tool::Label),
        ContextAction::PlaceText => {
            let source = crate::workbench::app::schematic_editor_request_source(state);
            state.design.design_note.open(source);
        }
        ContextAction::AdoptStimulus
        | ContextAction::ReadoptStimulus
        | ContextAction::SaveStimulus
        | ContextAction::OpenStimulusDefinition => stimulus::execute(action, state),
        ContextAction::Cut => {
            cut_schematic_selection(
                &mut state.design.schematic_clipboard,
                state.model,
                state.workbench,
            );
        }
        ContextAction::Copy => {
            copy_active_schematic_selection(&mut state.design.schematic_clipboard, state.model);
        }
        ContextAction::Paste => {
            let refusal = match paste_schematic_clipboard_at(
                state.design.floating_copy_parts(),
                state.model,
                state.workbench,
                click_pos,
            ) {
                Ok(true) => None,
                Ok(false) => Some("Nothing was pasted.".to_owned()),
                Err(reason) => Some(reason),
            };
            if let Some(refusal) = refusal {
                state.prompt(refusal);
            }
        }
        ContextAction::Duplicate => {
            let step = schematic_grid_step(state.model);
            duplicate_schematic_selection_at(
                state.design.floating_copy_parts(),
                state.model,
                state.workbench,
                click_pos + step,
            );
        }
        ContextAction::Delete => {
            delete_schematic_selection(state.model, state.workbench);
        }
        ContextAction::DescendHierarchy | ContextAction::OpenVerilogAView => {
            state.open_selected_instance_master();
        }
        ContextAction::EditSymbolInPlace => {
            state.edit_selected_symbol_in_place();
        }
        ContextAction::NewVerilogADevice => {
            state
                .workbench
                .outbox
                .push(DesignSurfaceRequest::NewVerilogADevice);
        }
        ContextAction::UpdateInstanceInterface => {
            let outcome = state.model.schematic.update_selected_instance_interface(
                &state.model.library_manager,
                &state.model.workspace.content.schematic_buffers,
            );
            state.prompt(match outcome {
                Ok(summary) => summary,
                Err(error) => error.to_string(),
            });
        }
        ContextAction::ReplaceInstance => open_replace_instance_dialog(
            &mut state.design.replace_instance,
            &state.design.property_registry,
            state.model,
            state.workbench,
        ),
        ContextAction::CreateHierarchy => {
            crate::workbench::app::open_create_hierarchy_dialog(
                &mut state.design.create_hierarchy,
                state.model,
                state.workbench,
            );
        }
        ContextAction::CreateSymbolFromPorts => {
            state
                .workbench
                .outbox
                .push(DesignSurfaceRequest::CreateSymbolFromPorts);
        }
        ContextAction::SheetProperties => show_sheet_properties(state),
        ContextAction::FitContent => {
            state.model.schematic.session.editor.needs_fit = true;
            state.model.schematic.session.editor.needs_drawing_sheet_fit = false;
        }
        ContextAction::ShowInNetlist => show_in_netlist(state),
        ContextAction::Probe => state.model.schematic.arm_tool(Tool::Probe),
        ContextAction::OperatingPoint => open_operating_point(state),
        ContextAction::SnapToStandardValue => {
            super::standard_value_snap::run_snap_to_standard_value(state.model, state.workbench);
        }
        ContextAction::Choose(_) => {}
    }
}

/// Show the selected instance's card in the generated deck. The Netlist
/// workspace opens at the click, as it did, so the rest of the frame reads
/// the route it read. The shell opens the deck at the card once the surface
/// has drawn. An instance the deck does not state changes nothing.
fn show_in_netlist(state: &mut CanvasParts<'_>) {
    if state.facts().selected_instance_netlist_block().is_some() {
        return;
    }
    state.workbench.activate(Workspace::Netlist);
    state
        .workbench
        .outbox
        .push(DesignSurfaceRequest::ShowInNetlist);
}

/// Show the open sheet's properties: with nothing selected the inspector is
/// the sheet's, so the selection is cleared and the inspector is opened
/// where it is closed, before the scene is drawn, and the sheets feature is
/// asked to bring its Sheet section into view. Only an editable sheet is
/// shown so; anywhere else nothing changes.
fn show_sheet_properties(state: &mut CanvasParts<'_>) {
    if !(state.workbench.shows_design_surface() && state.model.drawing_sheet_writable()) {
        return;
    }
    state.model.schematic.session.editor.selection.clear();
    state.model.schematic.session.editor.net_highlight.clear();
    state.workbench.inspector_visible = true;
    state.workbench.drawer = Some(crate::workbench::state::Drawer::Inspector);
    state.workbench.outbox.push(RevealSheetSection);
}

fn open_operating_point(state: &mut CanvasParts<'_>) {
    // The inspector renders the *selected* analysis, so the hop selects the
    // operating point before it routes. Without this a run of [OP, TRAN] with
    // the transient selected landed on "not a DC operating-point result" —
    // the workspace and the viewer were right and the one thing that decides
    // what they show was left wherever the reader had put it.
    let clicked = clicked_instance_name(&state.facts());
    let target = operating_point_analysis_index(&state.facts(), clicked.as_deref());
    if let Some(index) = target {
        state.model.simulation.select_analysis(index);
    }
    // The device under the pointer is what this hop is about. The Op inspector
    // reads one device-name selection — its docbar filter — so the hop asks
    // for that, exactly as the reverse direction writes the schematic
    // selection. The results workspace writes it, with its viewer, once the
    // surface has drawn.
    //
    // Asked unconditionally, including as an empty filter: a device the
    // selected report does not hold used to leave the *previous* device's
    // filter in place, so the hop opened on some other instance's row and read
    // as if it had worked.
    let device = clicked
        .filter(|name| {
            target
                .is_some_and(|index| analysis_reports_device(&state.facts(), index, name.as_str()))
        })
        .unwrap_or_default();
    state
        .workbench
        .outbox
        .push(DesignSurfaceRequest::ShowOperatingPoint { device });
    state.workbench.activate(Workspace::Results);
}

/// The clicked instance's exact engine name, when exactly one is selected: the
/// element the engine knows it by, inside the occurrence the tab is editing.
///
/// The name comparison downstream is the one
/// [`crate::workbench::documents::result_document::op_inspector`] uses to walk
/// the other way, so the two directions agree on what "this device" means.
fn clicked_instance_name(state: &CanvasFacts<'_>) -> Option<String> {
    let id = state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_component()?;
    let element = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)?
        .engine_element_name();
    let occurrence = state.model.workspace.content.occurrence_path();
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
fn operating_point_analysis_index(state: &CanvasFacts<'_>, device: Option<&str>) -> Option<usize> {
    let run = state.model.simulation.active_run()?;
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
fn analysis_reports_device(state: &CanvasFacts<'_>, index: usize, name: &str) -> bool {
    state.model.simulation.active_run().is_some_and(|run| {
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

fn operating_point_available(state: &CanvasFacts<'_>) -> bool {
    state.model.simulation.active_run().is_some_and(|run| {
        run.analyses.iter().any(|analysis| {
            analysis
                .device_op
                .as_ref()
                .is_some_and(|report| !report.is_empty())
        })
    })
}

fn context_menu_binding(state: &CanvasFacts<'_>) -> Option<ContextMenuBinding> {
    let (target, (x, y)) = state.design.interaction.context_target?;
    Some(ContextMenuBinding {
        source: super::requests::editor_request_source(state),
        selection: state.model.schematic.session.editor.selection.clone(),
        tool: state.model.schematic.session.editor.tool,
        target,
        click_pos: Point::new(x, y),
    })
}

fn context_menu_view(state: &CanvasFacts<'_>, ctx: &Context) -> Option<ContextMenuView> {
    let binding = context_menu_binding(state)?;
    let resolve = |entries: &[ContextEntry]| {
        entries
            .iter()
            .map(|entry| resolve_entry(state, ctx, *entry))
            .collect::<Vec<_>>()
    };
    Some(ContextMenuView {
        summary: menu_summary(state, binding.target),
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
    state: &CanvasFacts<'_>,
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
            state.model.preferences.shortcuts().resolved_label(
                &crate::workbench::commands::vocabulary::CommandRegistry,
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
    state: &mut CanvasParts<'_>,
    request: ContextMenuRequest,
    symbol_context: &SchematicSymbolContext,
) {
    let offered = |entries: &[ContextEntry]| {
        entries.iter().any(|entry| {
            matches!(entry, ContextEntry::Command(command) if command.action == request.action)
        })
    };
    if context_menu_binding(&state.facts()).as_ref() != Some(&request.binding)
        || state.application_modal_open()
        || !action_availability(request.action, &state.facts()).0
        || !(offered(&menu_entries(&state.facts(), request.binding.target))
            || offered(ARRANGE_ENTRIES))
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
