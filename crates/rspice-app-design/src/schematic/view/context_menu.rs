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
use crate::workbench::app::dialogs::hierarchy::subcircuit_source;
use crate::workbench::app::dialogs::selection::shared_properties;
use crate::workbench::app_state::ContextTarget;
use crate::workbench::state::Workspace;
use crate::workbench::{
    app::{
        StimulusLinkMode, copy_active_schematic_selection, cut_schematic_selection,
        delete_schematic_selection, duplicate_schematic_selection_at, open_stimulus_link,
        paste_schematic_clipboard_at, replace_instance_available, replace_instance_refusal,
        schematic_grid_step, show_stimulus_definition,
    },
    design::commands::DesignCommand,
    framework::command_registry::{WorkbenchCommand, ids},
    framework::feature::Execute,
    framework::requests::RevealSheetSection,
};

use super::SchematicSymbolContext;
use super::coordinates::{screen_to_grid, screen_to_schematic};
use super::interaction::{PointerHit, PointerTarget, pointer_target};
use super::parts::{CanvasFacts, CanvasParts};
use super::requests::DesignSurfaceRequest;
use super::viewport::Viewport;

use super::selection_layout::{SelectionLayoutCommand, selection_layout_availability};
use super::shelf_drag::something_floats;
use rspice_schematic_editor::view::context_menu::{
    self as context_surface, ArrangeAction, ContextAction, ContextIcon, ContextMenuBinding,
    ContextMenuRequest, ContextMenuView,
};
use rspice_schematic_editor::view::pick_chooser::ChooserRow;
use rspice_schematic_editor::view::selection_drag::target_is_selected;
use rspice_schematic_editor::view::tool_input::ToolPointerView;

mod stimulus;

#[derive(Debug, Clone, Copy)]
pub struct ContextCommand {
    pub action: ContextAction,
    icon: Option<ContextIcon>,
    pub label: &'static str,
    /// The command whose shortcut the row shows.
    pub shortcut_command: Option<CommandKey>,
}

pub type ContextEntry = context_surface::ContextEntry<ContextCommand>;

/// A row that runs `command`, under the name and with the key the command
/// declares, so it reads as the menu bar's row does.
const fn design_row(
    action: ContextAction,
    command: DesignCommand,
    icon: Option<ContextIcon>,
) -> ContextEntry {
    ContextEntry::Command(ContextCommand {
        action,
        icon,
        label: command.declared_spec().label,
        shortcut_command: Some(command.key()),
    })
}

/// A row that runs the workbench's `command`, under its declared name.
const fn workbench_row(
    action: ContextAction,
    command: WorkbenchCommand,
    icon: Option<ContextIcon>,
) -> ContextEntry {
    ContextEntry::Command(ContextCommand {
        action,
        icon,
        label: command.declared_spec().label,
        shortcut_command: Some(command.key()),
    })
}

/// A row under the name the menu bar gives `shortcut_command`, or a row
/// of this menu's own when it has none.
const fn named_row(
    action: ContextAction,
    label: &'static str,
    shortcut_command: Option<CommandKey>,
) -> ContextEntry {
    ContextEntry::Command(ContextCommand {
        action,
        icon: None,
        label,
        shortcut_command,
    })
}

// A row wears the glyph of the toolbar button that runs the same command,
// and a row with no such button wears none, as the menu bar's rows do; so
// no glyph stands for two actions anywhere.
const CONTEXT_ENTRIES: &[ContextEntry] = &[
    // The wire or bus being drawn: Enter, Backspace and Escape, as rows. A
    // right-click while routing offers these alone.
    named_row(ContextAction::FinishRoute, "Finish", None),
    named_row(ContextAction::RemoveRoutePoint, "Remove last point", None),
    // The touch strip's Cancel.
    ContextEntry::Command(ContextCommand {
        action: ContextAction::CancelRoute,
        icon: Some(ContextIcon::Close),
        label: "Cancel",
        shortcut_command: Some(ids::CANCEL),
    }),
    ContextEntry::Separator,
    // What can be started on a bare sheet: only the empty canvas offers them.
    design_row(
        ContextAction::PlaceInstance,
        DesignCommand::PlaceInstance,
        Some(ContextIcon::Instance),
    ),
    design_row(
        ContextAction::PlaceWire,
        DesignCommand::PlaceWire,
        Some(ContextIcon::Wire),
    ),
    design_row(
        ContextAction::PlaceLabel,
        DesignCommand::PlaceLabel,
        Some(ContextIcon::Label),
    ),
    design_row(
        ContextAction::PlaceText,
        DesignCommand::PlaceText,
        Some(ContextIcon::Text),
    ),
    ContextEntry::Separator,
    // What starts where the menu was opened on a wire, a junction or a pin
    // end: the places a wire or a label is started from.
    named_row(ContextAction::DrawWireFromHere, "Draw wire from here", None),
    named_row(ContextAction::NameNet, "Name this net…", None),
    ContextEntry::Separator,
    design_row(
        ContextAction::Properties,
        DesignCommand::ObjectProperties,
        None,
    ),
    design_row(
        ContextAction::Rotate,
        DesignCommand::RotateSelection,
        Some(ContextIcon::Rotate),
    ),
    design_row(
        ContextAction::Mirror,
        DesignCommand::MirrorSelectionHorizontal,
        Some(ContextIcon::Mirror),
    ),
    design_row(
        ContextAction::MirrorVertical,
        DesignCommand::MirrorSelectionVertical,
        Some(ContextIcon::MirrorVertical),
    ),
    // The Design menu's Arrange submenu, as a page of this menu: a phone
    // shows no menu bar, and Align and Distribute have no key.
    named_row(ContextAction::Arrange, "Arrange", None),
    // Only in the menu of a selection that holds a resistor, capacitor or
    // inductor; see `menu_entries`.
    design_row(
        ContextAction::SnapToStandardValue,
        DesignCommand::SnapToStandardValue,
        None,
    ),
    // Only in the menu of a selection that holds a part whose name or value
    // has been moved; see `menu_entries`.
    design_row(
        ContextAction::ResetLabelPosition,
        DesignCommand::ResetLabelPosition,
        None,
    ),
    // The stimulus-library group, which exists only in a placed source's menu
    // and states the verbs that source's standing with the library calls for;
    // see `stimulus`.
    ContextEntry::Separator,
    named_row(
        ContextAction::AdoptStimulus,
        "Adopt stimulus definition…",
        None,
    ),
    named_row(
        ContextAction::ReadoptStimulus,
        "Re-adopt library revision",
        None,
    ),
    named_row(
        ContextAction::SaveStimulus,
        "Save as stimulus definition…",
        None,
    ),
    named_row(
        ContextAction::OpenStimulusDefinition,
        "Open in Stimulus Library",
        None,
    ),
    ContextEntry::Separator,
    workbench_row(ContextAction::Cut, WorkbenchCommand::Cut, None),
    workbench_row(ContextAction::Copy, WorkbenchCommand::Copy, None),
    // Floats the clipboard where the menu was opened, which is the point a
    // reader right-clicks to say "here"; Edit ▸ Paste floats it at the
    // pointer, which is on the menu by then.
    named_row(ContextAction::Paste, "Paste here", Some(ids::PASTE)),
    workbench_row(
        ContextAction::Duplicate,
        WorkbenchCommand::Duplicate,
        Some(ContextIcon::Duplicate),
    ),
    workbench_row(
        ContextAction::Delete,
        WorkbenchCommand::Delete,
        Some(ContextIcon::Trash),
    ),
    ContextEntry::Separator,
    // The instance-master slot: what the menu's subject has for a master.
    // An instance descends into its schematic; a Verilog-A instance's master
    // is a module, which opens as its cell view (the Descend shortcut does
    // the same); the empty canvas has no master and offers a new one.
    design_row(
        ContextAction::DescendHierarchy,
        DesignCommand::DescendHierarchyDirect,
        Some(ContextIcon::Descend),
    ),
    ContextEntry::Command(ContextCommand {
        action: ContextAction::OpenVerilogAView,
        icon: Some(ContextIcon::Descend),
        label: "Open Verilog-A view",
        shortcut_command: Some(DesignCommand::DescendHierarchyDirect.key()),
    }),
    design_row(
        ContextAction::EditSymbolInPlace,
        DesignCommand::EditSymbolInPlace,
        None,
    ),
    named_row(
        ContextAction::NewVerilogADevice,
        "New Verilog-A device…",
        Some(ids::NEW_VERILOGA_DEVICE),
    ),
    design_row(
        ContextAction::UpdateInstanceInterface,
        DesignCommand::UpdateInstanceInterface,
        None,
    ),
    design_row(
        ContextAction::UpdateAllInstanceInterfaces,
        DesignCommand::UpdateAllInstanceInterfaces,
        None,
    ),
    design_row(
        ContextAction::ReplaceInstance,
        DesignCommand::ReplaceInstance,
        None,
    ),
    design_row(
        ContextAction::CreateHierarchy,
        DesignCommand::CreateHierarchy,
        None,
    ),
    named_row(
        ContextAction::CreateSymbolFromPorts,
        "Create symbol from schematic ports…",
        None,
    ),
    ContextEntry::Separator,
    named_row(
        ContextAction::SheetProperties,
        "Sheet properties",
        Some(ids::SHEET_PROPERTIES),
    ),
    // Fitting the drawing sheet is not here. `WorkbenchCommand::ZoomFit` already has a
    // toolbar button on this canvas, a status-bar route, a mobile canvas
    // control and the `F` key; a selection menu that fits without scrolling is
    // worth more than a sixth route to it.
    design_row(
        ContextAction::FitContent,
        DesignCommand::FitSchematicContent,
        None,
    ),
    // Only on the empty canvas's menu: a finger has no Z key, and the next
    // drag after this row frames the view.
    workbench_row(ContextAction::ZoomBox, WorkbenchCommand::ZoomBox, None),
    ContextEntry::Separator,
    named_row(
        ContextAction::ShowInNetlist,
        "Show in netlist",
        Some(ids::SHOW_IN_NETLIST),
    ),
    design_row(
        ContextAction::Probe,
        DesignCommand::PlaceProbe,
        Some(ContextIcon::Probe),
    ),
    named_row(
        ContextAction::OperatingPoint,
        "Operating-point table",
        Some(ids::RESULT_OPERATING_POINT),
    ),
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

/// What one menu is about, which decides its rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuSubject {
    /// A wire or bus being drawn.
    Route,
    /// The canvas itself: nothing was under the pointer, and nothing is
    /// selected.
    Canvas,
    /// One placed independent source: the stimulus group, and no master rows.
    Source,
    /// An instance whose master is a Verilog-A view.
    VerilogAInstance,
    /// Any other instance.
    Instance,
    /// The end of a part's pin.
    Pin,
    /// A wire, or a piece of one.
    Wire,
    Junction,
    NetLabel,
    /// A bus or one of its taps.
    Bus,
    Probe,
    /// A design note or a documentation shape.
    Annotation,
    /// Two or more objects, which the menu acts on together.
    Selection,
}

fn menu_subject(state: &CanvasFacts<'_>, target: ContextTarget) -> MenuSubject {
    if route_is_live(state) {
        return MenuSubject::Route;
    }
    if let ContextTarget::Pin(_) = target {
        return MenuSubject::Pin;
    }
    let selection = &state.model.schematic.session.editor.selection;
    if selection.count() > 1 {
        return MenuSubject::Selection;
    }
    if clicked_source(state).is_some() {
        return MenuSubject::Source;
    }
    let instance = |id| {
        if state.veriloga_master_of_component(id).is_some() {
            MenuSubject::VerilogAInstance
        } else {
            MenuSubject::Instance
        }
    };
    match target {
        ContextTarget::Component(id) => instance(id),
        ContextTarget::Pin(_) => MenuSubject::Pin,
        ContextTarget::Wire(_) => MenuSubject::Wire,
        // A right-click selects what it lands on, and names a part or a
        // wire as the target; anything else is named by what it selected.
        ContextTarget::Canvas => {
            if let Some(id) = selection.single_component() {
                instance(id)
            } else if selection.single_net_label().is_some() {
                MenuSubject::NetLabel
            } else if selection.single_bus().is_some() || selection.single_bus_tap().is_some() {
                MenuSubject::Bus
            } else if selection.single_junction().is_some() {
                MenuSubject::Junction
            } else if selection.single_probe().is_some() {
                MenuSubject::Probe
            } else if selection.single_design_note().is_some()
                || selection.single_documentation_shape().is_some()
            {
                MenuSubject::Annotation
            } else if selection.has_any_wire_selection() {
                MenuSubject::Wire
            } else {
                MenuSubject::Canvas
            }
        }
    }
}

/// Whether a wire or bus is being drawn.
fn route_is_live(state: &CanvasFacts<'_>) -> bool {
    let editor = &state.model.schematic.session.editor;
    editor.wire_drawing.active || editor.bus_drawing.active
}

/// The Arrange page: the Design menu's Arrange submenu, group by group.
pub const ARRANGE_ENTRIES: &[ContextEntry] = &[
    arrange(ArrangeAction::RotateClockwise, Some(ContextIcon::Rotate)),
    arrange(ArrangeAction::RotateCounterClockwise, None),
    arrange(ArrangeAction::MirrorHorizontal, Some(ContextIcon::Mirror)),
    arrange(
        ArrangeAction::MirrorVertical,
        Some(ContextIcon::MirrorVertical),
    ),
    ContextEntry::Separator,
    arrange(ArrangeAction::AlignLeft, None),
    arrange(ArrangeAction::AlignCenter, None),
    arrange(ArrangeAction::AlignRight, None),
    arrange(ArrangeAction::AlignTop, None),
    arrange(ArrangeAction::AlignMiddle, None),
    arrange(ArrangeAction::AlignBottom, None),
    ContextEntry::Separator,
    arrange(ArrangeAction::DistributeHorizontal, None),
    arrange(ArrangeAction::DistributeVertical, None),
    ContextEntry::Separator,
    arrange(ArrangeAction::SnapToGrid, None),
];

const fn arrange(action: ArrangeAction, icon: Option<ContextIcon>) -> ContextEntry {
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

/// Whether the row `action` belongs to a menu about `subject`: the verbs that
/// apply to that kind of object, and only those. The empty canvas offers what
/// can be started on a sheet and the sheet's own verbs; an object offers its
/// own, the edits common to every object, then what it has for a master or a
/// signal. A row that applies to the kind but cannot run right now stays, off,
/// and says why.
fn row_belongs(action: ContextAction, subject: MenuSubject) -> bool {
    use MenuSubject as S;
    let route_row = matches!(
        action,
        ContextAction::FinishRoute | ContextAction::RemoveRoutePoint | ContextAction::CancelRoute
    );
    if subject == S::Route || route_row {
        return subject == S::Route && route_row;
    }
    let instance = matches!(subject, S::Instance | S::VerilogAInstance);
    let part = instance || subject == S::Source;
    let turns = part || matches!(subject, S::NetLabel | S::Annotation | S::Selection);
    match action {
        ContextAction::PlaceInstance
        | ContextAction::PlaceWire
        | ContextAction::PlaceLabel
        | ContextAction::PlaceText
        | ContextAction::Paste
        | ContextAction::NewVerilogADevice
        | ContextAction::CreateSymbolFromPorts
        | ContextAction::SheetProperties
        | ContextAction::FitContent
        | ContextAction::ZoomBox => subject == S::Canvas,
        ContextAction::Properties => turns || subject == S::Bus,
        ContextAction::Rotate
        | ContextAction::Mirror
        | ContextAction::MirrorVertical
        | ContextAction::Arrange => turns,
        ContextAction::SnapToStandardValue | ContextAction::ResetLabelPosition => {
            part || subject == S::Selection
        }
        ContextAction::AdoptStimulus
        | ContextAction::ReadoptStimulus
        | ContextAction::SaveStimulus
        | ContextAction::OpenStimulusDefinition => subject == S::Source,
        ContextAction::DrawWireFromHere | ContextAction::NameNet => {
            matches!(subject, S::Wire | S::Junction | S::Pin)
        }
        ContextAction::Cut | ContextAction::Copy | ContextAction::Delete => {
            !matches!(subject, S::Canvas | S::Pin)
        }
        // A junction alone has nowhere to be copied to at a fixed offset.
        ContextAction::Duplicate => !matches!(subject, S::Canvas | S::Junction | S::Pin),
        ContextAction::DescendHierarchy => subject == S::Instance,
        ContextAction::OpenVerilogAView => subject == S::VerilogAInstance,
        ContextAction::EditSymbolInPlace
        | ContextAction::UpdateInstanceInterface
        | ContextAction::ReplaceInstance => instance,
        // A repair of the whole drawing, offered on the sheet and on an
        // instance beside the repair of that one.
        ContextAction::UpdateAllInstanceInterfaces => instance || subject == S::Canvas,
        ContextAction::CreateHierarchy => part || subject == S::Selection,
        ContextAction::ShowInNetlist => part,
        // A probe goes on a part or a wire; the canvas's own probe is the tool.
        ContextAction::Probe => part || matches!(subject, S::Wire | S::Pin),
        ContextAction::OperatingPoint => part || subject == S::Canvas,
        ContextAction::ArrangeCommand(_)
        | ContextAction::Choose(_)
        | ContextAction::FinishRoute
        | ContextAction::RemoveRoutePoint
        | ContextAction::CancelRoute => false,
    }
}

/// The open drawing's instances whose master's interface moved on, by the
/// answer the deck and the Design command give.
fn stale_instances(state: &CanvasFacts<'_>) -> Vec<String> {
    state.model.schematic.stale_instance_interfaces(
        &state.model.library_manager,
        &state.model.workspace.content.schematic_buffers,
    )
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
/// The catalog is one list, in one order, and each menu takes the rows that
/// belong to its subject (`row_belongs`). Some rows are offered only where
/// they have something to act on: Snap to standard value on a resistor,
/// capacitor or inductor; Reset label position on a part whose text moved;
/// the stimulus group on a source, by its standing with the library; Update
/// all instances where one is stale; Paste here with something to paste. A
/// rule with nothing left under it goes too.
pub fn menu_entries(state: &CanvasFacts<'_>, target: ContextTarget) -> Vec<ContextEntry> {
    let subject = menu_subject(state, target);
    let mut entries: Vec<ContextEntry> = Vec::with_capacity(CONTEXT_ENTRIES.len());
    for entry in CONTEXT_ENTRIES {
        let shown = match entry {
            ContextEntry::Separator => {
                matches!(entries.last(), Some(ContextEntry::Command(_)))
            }
            ContextEntry::Command(command) if !row_belongs(command.action, subject) => false,
            ContextEntry::Command(command) => match command.action {
                action if stimulus::owns(action) => stimulus::shown(action, state),
                ContextAction::SnapToStandardValue => selection_holds_a_standard_value(state),
                ContextAction::ResetLabelPosition => {
                    super::label_reset::selection_holds_placed_text(state.model)
                }
                // A repair, so offered only where there is something to repair.
                ContextAction::UpdateAllInstanceInterfaces => !stale_instances(state).is_empty(),
                // Pasting needs something to paste.
                ContextAction::Paste => !state.design.schematic_clipboard.is_empty(),
                _ => true,
            },
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
    routing_was_active: bool,
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
        context_surface::close::<ContextMenuBinding>(response);
        forget_choices(&response.ctx);
        state.design.interaction.context_target = None;
        return;
    }
    let anchor = if opening.secondary_clicked {
        capture_pointer_target(
            response,
            state,
            viewport,
            symbol_context,
            routing_was_active,
        )
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
        context_surface::close::<ContextMenuBinding>(response);
        return;
    };
    let output = context_surface::show(response, opening, anchor, &view);
    match output.request {
        Some(ContextMenuRequest {
            binding,
            action: ContextAction::Choose(target),
        }) => choose_target(state, &view, &binding, target),
        Some(request) => apply_context_request(state, request, symbol_context, &response.ctx),
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

/// Name what the menu opens on and where, and select it. A right-click
/// while routing opens the route's menu and selects nothing: the route is
/// what the menu is about.
fn capture_pointer_target(
    response: &Response,
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    routing: bool,
) -> Option<egui::Pos2> {
    let pos = response.interact_pointer_pos()?;
    let grid_pos = screen_to_grid(viewport, state.model.schematic.document().grid_size, pos);
    let hit_pos = screen_to_schematic(viewport, pos);
    let hit_radius = (6.0 / viewport.zoom.max(0.1)).ceil() as i32;
    let target = if routing {
        forget_choices(&response.ctx);
        ContextTarget::Canvas
    } else if response.long_touched() {
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
    // On a part's pin end the menu is the pin's, opened at the pin.
    let (target, at) = match target {
        ContextTarget::Component(id) => {
            pin_end_near(&state.facts(), symbol_context, id, hit_pos, hit_radius)
                .map_or((target, grid_pos), |pin| (ContextTarget::Pin(id), pin))
        }
        _ => (target, grid_pos),
    };
    state.design.interaction.context_target = Some((target, (at.x, at.y)));
    Some(pos)
}

/// The end of one of part `id`'s pins within `radius` of `hit`.
fn pin_end_near(
    state: &CanvasFacts<'_>,
    symbol_context: &SchematicSymbolContext,
    id: u64,
    hit: Point,
    radius: i32,
) -> Option<Point> {
    let component = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)?;
    symbol_context
        .terminal_points(component)
        .into_iter()
        .find(|pin| {
            i64::from(pin.x).abs_diff(i64::from(hit.x)) <= u64::from(radius.unsigned_abs())
                && i64::from(pin.y).abs_diff(i64::from(hit.y)) <= u64::from(radius.unsigned_abs())
        })
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
/// and name it as the menu's target. Opened on empty sheet, the menu is the
/// sheet's, so nothing stays selected for its rows to act on elsewhere. A net
/// highlight stays lit, so the menu can act on the lit net and Descend
/// carries it into an instance.
pub fn select_context_target(
    state: &mut CanvasParts<'_>,
    target: Option<PointerTarget>,
) -> ContextTarget {
    let Some(target) = target else {
        state.model.schematic.session.editor.selection.clear();
        return ContextTarget::Canvas;
    };
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
    match menu_subject(state, target) {
        MenuSubject::Canvas => None,
        MenuSubject::Route if state.model.schematic.session.editor.bus_drawing.active => {
            Some("Bus being drawn".to_owned())
        }
        MenuSubject::Route => Some("Wire being drawn".to_owned()),
        _ => Some(selection_summary(state, target)),
    }
}

pub fn selection_summary(state: &CanvasFacts<'_>, target: ContextTarget) -> String {
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
        ContextTarget::Component(id) | ContextTarget::Pin(id) => state
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

/// Why a row that would change the drawing is off on a view that cannot be
/// changed.
const READ_ONLY_VIEW: &str =
    "The active schematic view is read-only; reopen it in an editable context to change it";

/// Whether `action` changes the drawing in front, so that a read-only view
/// is the first reason it cannot run.
fn changes_the_view(action: ContextAction) -> bool {
    matches!(
        action,
        ContextAction::Rotate
            | ContextAction::Mirror
            | ContextAction::MirrorVertical
            | ContextAction::Arrange
            | ContextAction::ArrangeCommand(_)
            | ContextAction::SnapToStandardValue
            | ContextAction::ResetLabelPosition
            | ContextAction::Cut
            | ContextAction::Paste
            | ContextAction::Duplicate
            | ContextAction::Delete
            | ContextAction::UpdateInstanceInterface
            | ContextAction::UpdateAllInstanceInterfaces
            | ContextAction::ReplaceInstance
            | ContextAction::CreateHierarchy
            | ContextAction::SheetProperties
            | ContextAction::PlaceInstance
            | ContextAction::PlaceWire
            | ContextAction::PlaceLabel
            | ContextAction::PlaceText
            | ContextAction::DrawWireFromHere
            | ContextAction::NameNet
    )
}

pub fn action_availability(action: ContextAction, state: &CanvasFacts<'_>) -> (bool, &'static str) {
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
    // A read-only view is why every row that would change it is off, whatever
    // the selection or the clipboard holds.
    if !writable && changes_the_view(action) {
        return (false, READ_ONLY_VIEW);
    }
    match action {
        // The command's own test: one object opens its form, several parts
        // the form for what they share.
        ContextAction::Properties => (
            crate::workbench::app::selected_object_properties_available_in(
                &state.design.property_registry,
                state.model,
            ) || shared_properties::shared_properties_available(
                &state.design.property_registry,
                state.model,
            ),
            "Select one editable component, bus, bus tap, net label, design note, or documentation shape, or several parts, to open their properties",
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
                Err(refusal) => (false, refusal.message()),
            },
            None => (
                writable && (has_copyable_object || something_floats(state)),
                "Select at least one editable object",
            ),
        },
        ContextAction::ZoomBox => (
            matches!(
                state.model.workspace.content.active_view_type(),
                Some(ViewType::Schematic | ViewType::Testbench)
            ),
            "The zoom box frames a schematic or testbench canvas",
        ),
        ContextAction::PlaceInstance
        | ContextAction::PlaceWire
        | ContextAction::PlaceLabel
        | ContextAction::PlaceText => (true, ""),
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
            "Select at least one editable component, wire, bus, tap, junction, net label, probe, design note, or documentation shape",
        ),
        ContextAction::DescendHierarchy => (
            state.selected_hierarchy_master().is_some()
                || subcircuit_source::selected_instance_subcircuit(state.model).is_some(),
            "Select one instance with a resolved schematic or subcircuit master",
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
            state.code.new_veriloga_device_available(state.model),
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
        ContextAction::UpdateAllInstanceInterfaces => (
            writable && !stale_instances(state).is_empty(),
            "Every instance matches its master's current interface",
        ),
        // The command owns whether a replacement can be made at all — one
        // instance, editable, and a different ready master that preserves the
        // connected terminal contract. Restating any of that here would offer
        // the row where the dialog would immediately refuse.
        ContextAction::ReplaceInstance => {
            if !replace_instance_available(&state.design.property_registry, state.model) {
                (
                    false,
                    "Select one editable instance that a different ready master can stand in for",
                )
            } else {
                match replace_instance_refusal(&state.design.property_registry, state.model) {
                    None => (true, ""),
                    Some(refusal) => (false, refusal.sentence()),
                }
            }
        }
        ContextAction::CreateHierarchy => (
            state.model.create_hierarchy_available(),
            "Select one or more editable instances, with no port, bus, note or piece of a wire",
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
                    Some(ViewType::Schematic | ViewType::Testbench)
                ),
            "Sheet properties require a writable schematic or testbench",
        ),
        ContextAction::FitContent => (
            matches!(
                state.model.workspace.content.active_view_type(),
                Some(ViewType::Schematic | ViewType::Testbench)
            ),
            "Fit is available on a schematic or testbench canvas",
        ),
        // The model is the one owner of why the jump cannot be taken, so this
        // row explains itself with the reason the command would report.
        ContextAction::ShowInNetlist => {
            let blocked = state.selected_instance_netlist_block();
            (
                blocked.is_none(),
                blocked.unwrap_or("Select one instance the generated netlist states"),
            )
        }
        // A probe plots what a run holds, so a read-only view offers it too.
        ContextAction::Probe => (true, ""),
        ContextAction::SnapToStandardValue => {
            let blocked = super::standard_value_snap::snap_block_reason(state.model);
            (blocked.is_none(), blocked.unwrap_or_default())
        }
        ContextAction::ResetLabelPosition => {
            let blocked = super::label_reset::reset_block_reason(state.model);
            (blocked.is_none(), blocked.unwrap_or_default())
        }
        ContextAction::OperatingPoint => (
            operating_point_available(state),
            "Run a DC operating-point analysis with device OP reporting first",
        ),
        ContextAction::DrawWireFromHere | ContextAction::NameNet => (true, ""),
        ContextAction::FinishRoute => (
            route_points(state) >= 2,
            "Click a second point to finish the route",
        ),
        ContextAction::RemoveRoutePoint | ContextAction::CancelRoute => {
            (route_is_live(state), "No wire or bus is being drawn")
        }
        // A choice row is not a catalog command; `choose_target` checks it
        // against the rows the menu showed.
        ContextAction::Choose(_) => (false, "Long-press the objects to choose among them"),
    }
}

pub fn execute_context_action(
    action: ContextAction,
    state: &mut CanvasParts<'_>,
    click_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    state.model.retain_selection_on_active_sheet();
    if let Some(command) = row_command(action) {
        state.workbench.outbox.push(Execute(command));
        return;
    }
    match action {
        // A page turn, which the menu takes itself.
        ContextAction::Arrange => {}
        ContextAction::FinishRoute => finish_route(state),
        ContextAction::RemoveRoutePoint => {
            state.model.schematic.retract_route();
        }
        ContextAction::CancelRoute => state.model.schematic.cancel_interaction_step(),
        ContextAction::NameNet => {
            let at = conductor_point(&state.facts(), click_pos);
            let dialog_modal_open = state.dialog_modal_open();
            crate::workbench::app::open_net_label_placement_in(
                &mut state.design.net_label_placement,
                state.model,
                state.workbench,
                dialog_modal_open,
                at,
            );
        }
        // Started by `apply_context_request`, which holds the frame's
        // context a route is bound to.
        ContextAction::DrawWireFromHere => {}
        ContextAction::ZoomBox => {
            state.model.schematic.session.editor.zoom_box.arm();
            state.prompt("Drag a box to zoom to it.");
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
            // A subcircuit has no drawing to descend into; its text opens.
            let subcircuit = (action == ContextAction::DescendHierarchy
                && crate::workbench::app::actions::workspace::selected_hierarchy_master(
                    state.model,
                )
                .is_none())
            .then(|| subcircuit_source::selected_instance_subcircuit(state.model))
            .flatten();
            match subcircuit {
                Some(instance) => {
                    if let Err(refusal) = subcircuit_source::open_subcircuit_source(
                        &mut state.design.subcircuit_source,
                        state.model,
                        instance,
                    ) {
                        state.prompt(refusal);
                    }
                }
                None => state.open_selected_instance_master(),
            }
        }
        ContextAction::NewVerilogADevice => {
            state
                .workbench
                .outbox
                .push(DesignSurfaceRequest::NewVerilogADevice);
        }
        ContextAction::CreateSymbolFromPorts => {
            state
                .workbench
                .outbox
                .push(DesignSurfaceRequest::CreateSymbolFromPorts);
        }
        ContextAction::SheetProperties => show_sheet_properties(state, None),
        ContextAction::ShowInNetlist => show_in_netlist(state),
        ContextAction::Probe => {
            let target = state
                .design
                .interaction
                .context_target
                .map_or(ContextTarget::Canvas, |(target, _)| target);
            super::interaction::probe_context_target(state, target, click_pos, symbol_context);
        }
        ContextAction::OperatingPoint => open_operating_point(state),
        // A row its command runs, raised above, or a choice, which
        // `choose_target` takes.
        ContextAction::Properties
        | ContextAction::Rotate
        | ContextAction::Mirror
        | ContextAction::MirrorVertical
        | ContextAction::ArrangeCommand(_)
        | ContextAction::SnapToStandardValue
        | ContextAction::ResetLabelPosition
        | ContextAction::PlaceInstance
        | ContextAction::PlaceWire
        | ContextAction::PlaceLabel
        | ContextAction::PlaceText
        | ContextAction::EditSymbolInPlace
        | ContextAction::UpdateInstanceInterface
        | ContextAction::UpdateAllInstanceInterfaces
        | ContextAction::ReplaceInstance
        | ContextAction::CreateHierarchy
        | ContextAction::FitContent
        | ContextAction::Choose(_) => {}
    }
}

/// The command a row runs where the menus, the toolbar and the keys run the
/// same one. The row raises it, so it goes through the dispatcher as theirs
/// do: an open inspector field is committed first, a read-only view refuses
/// it in the same words, and "." repeats it. The rows that move the view
/// (Descend, Show in netlist, Sheet properties, the zoom box) and the rows
/// that act at the click (Paste here, Duplicate, Probe) keep their own.
const fn row_command(action: ContextAction) -> Option<CommandKey> {
    Some(match action {
        ContextAction::Properties => DesignCommand::ObjectProperties.key(),
        ContextAction::Rotate => DesignCommand::RotateSelection.key(),
        ContextAction::Mirror => DesignCommand::MirrorSelectionHorizontal.key(),
        ContextAction::MirrorVertical => DesignCommand::MirrorSelectionVertical.key(),
        ContextAction::ArrangeCommand(arrangement) => arrange_command(arrangement).key(),
        ContextAction::SnapToStandardValue => DesignCommand::SnapToStandardValue.key(),
        ContextAction::ResetLabelPosition => DesignCommand::ResetLabelPosition.key(),
        ContextAction::PlaceInstance => DesignCommand::PlaceInstance.key(),
        ContextAction::PlaceWire => DesignCommand::PlaceWire.key(),
        ContextAction::PlaceLabel => DesignCommand::PlaceLabel.key(),
        ContextAction::PlaceText => DesignCommand::PlaceText.key(),
        ContextAction::EditSymbolInPlace => DesignCommand::EditSymbolInPlace.key(),
        ContextAction::UpdateInstanceInterface => DesignCommand::UpdateInstanceInterface.key(),
        ContextAction::UpdateAllInstanceInterfaces => {
            DesignCommand::UpdateAllInstanceInterfaces.key()
        }
        ContextAction::ReplaceInstance => DesignCommand::ReplaceInstance.key(),
        ContextAction::CreateHierarchy => DesignCommand::CreateHierarchy.key(),
        ContextAction::FitContent => DesignCommand::FitSchematicContent.key(),
        _ => return None,
    })
}

/// Where on what the menu was opened on a wire or a label starts: the point
/// of the right-clicked wire nearest the click, the selected junction, or
/// the pin end the menu was opened at.
fn conductor_point(state: &CanvasFacts<'_>, click_pos: Point) -> Point {
    let target = state
        .design
        .interaction
        .context_target
        .map(|(target, _)| target);
    if let Some(ContextTarget::Wire(id)) = target {
        return state
            .model
            .schematic
            .document()
            .wires
            .iter()
            .find(|wire| wire.id == id)
            .and_then(|wire| wire.closest_point(click_pos))
            .map_or(click_pos, |(point, _)| point);
    }
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_junction()
        .filter(|_| target == Some(ContextTarget::Canvas))
        .unwrap_or(click_pos)
}

/// The points the route being drawn has placed.
fn route_points(state: &CanvasFacts<'_>) -> usize {
    let editor = &state.model.schematic.session.editor;
    if editor.bus_drawing.active {
        editor.bus_drawing.points.len()
    } else if editor.wire_drawing.active {
        editor.wire_drawing.points.len()
    } else {
        0
    }
}

/// End the route where it stands, as Enter does: the menu's binding has
/// already held the drawing to the one the route began on.
fn finish_route(state: &mut CanvasParts<'_>) {
    if state.model.schematic.session.editor.wire_drawing.active {
        super::sheet_visibility::finish_wire_on_active_sheet(state.model);
    } else if state.model.schematic.session.editor.bus_drawing.active
        && let Err(error) = state.model.schematic.finish_bus()
    {
        state.prompt(format!("Bus not committed: {error}"));
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
    // The jump moves to the Netlist workspace itself once it has located the
    // instance, and stays here when it cannot.
    state
        .workbench
        .outbox
        .push(DesignSurfaceRequest::ShowInNetlist);
}

/// Show the open sheet's properties: with nothing selected the inspector is
/// the sheet's, so the selection is cleared and the inspector is opened
/// where it is closed, before the scene is drawn, and the sheets feature is
/// asked to bring its Sheet section into view, with the cursor in `focus`'s
/// field where it names one. Only an editable sheet is shown so; anywhere
/// else nothing changes.
pub(super) fn show_sheet_properties(
    state: &mut CanvasParts<'_>,
    focus: Option<crate::state::DrawingSheetTitleFieldId>,
) {
    if !(state.workbench.shows_design_surface() && state.model.drawing_sheet_writable()) {
        return;
    }
    state.model.schematic.session.editor.selection.clear();
    state.model.schematic.session.editor.net_highlight.clear();
    state.workbench.inspector_visible = true;
    state.workbench.drawer = Some(crate::workbench::state::Drawer::Inspector);
    state.workbench.outbox.push(RevealSheetSection { focus });
}

pub fn open_operating_point(state: &mut CanvasParts<'_>) {
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

pub fn context_menu_binding(state: &CanvasFacts<'_>) -> Option<ContextMenuBinding> {
    let (target, (x, y)) = state.design.interaction.context_target?;
    Some(ContextMenuBinding {
        source: super::requests::editor_request_source(state),
        selection: state.model.schematic.session.editor.selection.clone(),
        tool: state.model.schematic.session.editor.tool,
        target,
        click_pos: Point::new(x, y),
    })
}

pub fn context_menu_view(state: &CanvasFacts<'_>, ctx: &Context) -> Option<ContextMenuView> {
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

/// The key a row with no command of its own answers to on the canvas.
const fn canvas_key(action: ContextAction) -> Option<egui::Key> {
    match action {
        ContextAction::FinishRoute => Some(egui::Key::Enter),
        ContextAction::RemoveRoutePoint => Some(egui::Key::Backspace),
        _ => None,
    }
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
    let shortcut = command.shortcut_command.map_or_else(
        || canvas_key(command.action).map_or_else(String::new, |key| key.name().to_owned()),
        |product_command| {
            state.model.preferences.shortcuts().resolved_label(
                state.commands,
                product_command,
                crate::workbench::app_state::runtime_command_platform(ctx),
                ctx.os(),
            )
        },
    );
    context_surface::ContextEntry::Command(context_surface::ContextCommand {
        action: command.action,
        icon: command.icon,
        label: command.label,
        shortcut,
        enabled,
        disabled_reason,
    })
}

pub fn apply_context_request(
    state: &mut CanvasParts<'_>,
    request: ContextMenuRequest,
    symbol_context: &SchematicSymbolContext,
    ctx: &Context,
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
    if request.action == ContextAction::DrawWireFromHere {
        let at = conductor_point(&state.facts(), request.binding.click_pos);
        state.model.schematic.arm_tool(Tool::Wire);
        state.start_canvas_wire(ctx, at);
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
pub(crate) mod tests;
