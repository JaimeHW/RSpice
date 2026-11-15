//! The symbol canvas's context menu: its rows, the keys they show and what
//! each one runs. It is drawn by the schematic canvas's own menu surface.
//!
//! Every row is a command the menus and keys run, under the same key, or a
//! key the shape being drawn answers to on the canvas. A right-click while a
//! shape is being drawn opens the drawing's rows and leaves the shape as it
//! was.

use egui::{Context, Response};
use rspice_schematic_editor::symbol_editor::SymbolViewport;
use rspice_schematic_editor::symbol_editor::context_menu::{
    self as symbol_menu, SymbolMenuAction, SymbolMenuBinding, SymbolMenuTarget,
};
use rspice_schematic_editor::symbol_editor::interaction::{SymbolCanvasInput, SymbolCanvasRequest};
use rspice_schematic_editor::symbol_editor::session::SymbolTool;
use rspice_schematic_editor::view::context_menu::{
    self as context_surface, ContextCommand, ContextEntry, ContextIcon, ContextMenuRequest,
    ContextMenuView,
};

use super::interaction::apply_canvas_request;
use crate::schematic::view::CanvasParts;
use crate::state::command_catalog::CommandKey;
use crate::state::{SymbolDocument, SymbolEditorMetadata};
use crate::workbench::app::actions::{duplicate_selected_symbol_shape, paste_symbol_shape};
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Execute;

/// The key a row shows: its command's, as the menus show it, or the key the
/// shape being drawn answers to on the canvas.
#[derive(Debug, Clone, Copy)]
enum RowKey {
    Command(CommandKey),
    Canvas(egui::Key),
}

#[derive(Debug, Clone, Copy)]
struct Row {
    action: SymbolMenuAction,
    icon: ContextIcon,
    label: &'static str,
    key: RowKey,
}

const fn row(
    action: SymbolMenuAction,
    icon: ContextIcon,
    label: &'static str,
    key: CommandKey,
) -> ContextEntry<Row> {
    ContextEntry::Command(Row {
        action,
        icon,
        label,
        key: RowKey::Command(key),
    })
}

/// A selection's rows, in the schematic menu's order.
const SELECTION_ROWS: &[ContextEntry<Row>] = &[
    row(
        SymbolMenuAction::Properties,
        ContextIcon::Sliders,
        "Object properties…",
        DesignCommand::ObjectProperties.key(),
    ),
    row(
        SymbolMenuAction::Rotate,
        ContextIcon::Rotate,
        "Rotate 90°",
        DesignCommand::RotateSelection.key(),
    ),
    row(
        SymbolMenuAction::Mirror,
        ContextIcon::Mirror,
        "Mirror",
        DesignCommand::MirrorSelectionHorizontal.key(),
    ),
    ContextEntry::Separator,
    row(
        SymbolMenuAction::Cut,
        ContextIcon::Copy,
        "Cut selection",
        ids::CUT,
    ),
    row(
        SymbolMenuAction::Copy,
        ContextIcon::Copy,
        "Copy selection",
        ids::COPY,
    ),
    row(
        SymbolMenuAction::Duplicate,
        ContextIcon::Copy,
        "Duplicate",
        ids::DUPLICATE,
    ),
    row(
        SymbolMenuAction::Delete,
        ContextIcon::Trash,
        "Delete",
        ids::DELETE,
    ),
];

/// The bare canvas's rows: what can be put down there, and the view.
const CANVAS_ROWS: &[ContextEntry<Row>] = &[
    // Where the menu was opened, which is the point a reader right-clicks
    // to say "here".
    row(
        SymbolMenuAction::Paste,
        ContextIcon::Copy,
        "Paste here",
        ids::PASTE,
    ),
    ContextEntry::Separator,
    row(
        SymbolMenuAction::Tool(SymbolTool::PlacePin),
        ContextIcon::SymbolPin,
        "Place pin",
        DesignCommand::SymbolPinTool.key(),
    ),
    row(
        SymbolMenuAction::Tool(SymbolTool::Line),
        ContextIcon::Line,
        "Draw line",
        DesignCommand::SymbolPolylineTool.key(),
    ),
    row(
        SymbolMenuAction::Tool(SymbolTool::Rectangle),
        ContextIcon::Rectangle,
        "Draw rectangle",
        DesignCommand::SymbolRectangleTool.key(),
    ),
    row(
        SymbolMenuAction::Tool(SymbolTool::Circle),
        ContextIcon::Circle,
        "Draw circle",
        DesignCommand::SymbolCircleTool.key(),
    ),
    row(
        SymbolMenuAction::Tool(SymbolTool::Arc),
        ContextIcon::Arc,
        "Draw arc",
        DesignCommand::SymbolArcTool.key(),
    ),
    row(
        SymbolMenuAction::Tool(SymbolTool::Polygon),
        ContextIcon::Polygon,
        "Draw polygon",
        DesignCommand::SymbolPolygonTool.key(),
    ),
    row(
        SymbolMenuAction::Tool(SymbolTool::Text),
        ContextIcon::Label,
        "Place text",
        DesignCommand::SymbolTextTool.key(),
    ),
    ContextEntry::Separator,
    row(
        SymbolMenuAction::Fit,
        ContextIcon::Fit,
        "Fit symbol",
        ids::ZOOM_FIT,
    ),
    row(
        SymbolMenuAction::Grid,
        ContextIcon::Grid,
        "Show grid",
        DesignCommand::CycleGrid.key(),
    ),
];

/// The shape being drawn's rows: Enter, Backspace and Escape, as rows.
const DRAWING_ROWS: &[ContextEntry<Row>] = &[
    ContextEntry::Command(Row {
        action: SymbolMenuAction::Finish,
        icon: ContextIcon::Check,
        label: "Finish",
        key: RowKey::Canvas(egui::Key::Enter),
    }),
    ContextEntry::Command(Row {
        action: SymbolMenuAction::RemoveLastPoint,
        icon: ContextIcon::Back,
        label: "Remove last point",
        key: RowKey::Canvas(egui::Key::Backspace),
    }),
    row(
        SymbolMenuAction::Cancel,
        ContextIcon::Close,
        "Cancel",
        ids::CANCEL,
    ),
];

fn rows(target: SymbolMenuTarget) -> &'static [ContextEntry<Row>] {
    match target {
        SymbolMenuTarget::Selection => SELECTION_ROWS,
        SymbolMenuTarget::Canvas => CANVAS_ROWS,
        SymbolMenuTarget::Drawing => DRAWING_ROWS,
    }
}

/// Open, draw and answer the canvas's context menu for this frame.
pub(super) fn handle_context_menu(
    state: &mut CanvasParts<'_>,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    viewport: SymbolViewport,
    response: &Response,
) {
    let opening = context_surface::opening(response, false);
    if !opening.needs_contents() {
        context_surface::close::<SymbolMenuBinding>(response);
        state.design.symbol.editor.menu = None;
        return;
    }
    let anchor = if opening.secondary_clicked {
        let pressed = response
            .interact_pointer_pos()
            .or_else(|| response.hover_pos());
        if let Some(pos) = pressed {
            let target = symbol_menu::pointer_target(
                &mut state.design.symbol.editor,
                document,
                metadata,
                viewport,
                pos,
            );
            state.design.symbol.editor.menu = Some((target, viewport.screen_to_world(pos)));
        }
        pressed
    } else {
        if opening.keyboard_open {
            let editor = &mut state.design.symbol.editor;
            let at = editor
                .pointer
                .unwrap_or_else(|| viewport.screen_to_world(viewport.rect.center()));
            editor.menu = Some((symbol_menu::keyboard_target(editor), at));
        }
        None
    };
    let Some(view) = menu_view(state, document, &response.ctx) else {
        context_surface::close::<SymbolMenuBinding>(response);
        state.design.symbol.editor.menu = None;
        return;
    };
    let output = context_surface::show(response, opening, anchor, &view);
    if let Some(request) = output.request {
        apply_menu_request(state, document, metadata, viewport, request);
    }
    if !output.open {
        state.design.symbol.editor.menu = None;
    }
}

/// What the open menu is bound to: the symbol in front, its selection and
/// tool, and what the menu was opened on and where.
fn menu_binding(state: &CanvasParts<'_>) -> Option<SymbolMenuBinding> {
    let editor = &state.design.symbol.editor;
    let (target, click_pos) = editor.menu?;
    Some(SymbolMenuBinding {
        source: state.symbol_editor_request_source(),
        selection: editor.effective_selection(),
        tool: editor.tool,
        target,
        click_pos,
    })
}

fn menu_view(
    state: &CanvasParts<'_>,
    document: &SymbolDocument,
    ctx: &Context,
) -> Option<ContextMenuView<SymbolMenuBinding, SymbolMenuAction>> {
    let binding = menu_binding(state)?;
    let entries = rows(binding.target)
        .iter()
        .map(|entry| match entry {
            ContextEntry::Separator => ContextEntry::Separator,
            ContextEntry::Command(row) => ContextEntry::Command(resolve_row(state, ctx, *row)),
        })
        .collect();
    Some(ContextMenuView {
        summary: symbol_menu::summary(document, &binding),
        binding,
        entries,
        choices: Vec::new(),
        arrange: Vec::new(),
    })
}

/// One row as the menu draws it: available or not and why, with its key.
fn resolve_row(
    state: &CanvasParts<'_>,
    ctx: &Context,
    row: Row,
) -> ContextCommand<SymbolMenuAction> {
    let (enabled, disabled_reason) = availability(state, row.action);
    let shortcut = match row.key {
        RowKey::Command(key) => state.model.preferences.shortcuts().resolved_label(
            state.commands,
            key,
            crate::workbench::app_state::runtime_command_platform(ctx),
            ctx.os(),
        ),
        RowKey::Canvas(key) => key.name().to_owned(),
    };
    let label = match row.action {
        SymbolMenuAction::Grid if state.design.symbol.editor.show_grid => "Hide grid",
        _ => row.label,
    };
    ContextCommand {
        action: row.action,
        icon: row.icon,
        label,
        shortcut,
        enabled,
        disabled_reason,
    }
}

/// Whether `action` can run now, and why not when it cannot: what its
/// command answers, for the commands.
fn availability(state: &CanvasParts<'_>, action: SymbolMenuAction) -> (bool, &'static str) {
    let editor = &state.design.symbol.editor;
    let selection = editor.effective_selection();
    let read_only = state.model.active_view_read_only();
    let refusal = match action {
        SymbolMenuAction::Properties => (selection.pins.len() + selection.shapes.len() != 1)
            .then_some("select one inspectable object"),
        SymbolMenuAction::Rotate
        | SymbolMenuAction::Mirror
        | SymbolMenuAction::Cut
        | SymbolMenuAction::Duplicate
        | SymbolMenuAction::Delete => {
            if read_only {
                Some("the symbol is read-only")
            } else {
                selection.is_empty().then_some("select an editable object")
            }
        }
        SymbolMenuAction::Copy => selection.is_empty().then_some("select an object"),
        SymbolMenuAction::Paste => {
            if read_only {
                Some("the symbol is read-only")
            } else {
                editor
                    .clipboard
                    .is_empty()
                    .then_some("the clipboard is empty")
            }
        }
        SymbolMenuAction::Tool(_) => read_only.then_some("the symbol is read-only"),
        SymbolMenuAction::Fit | SymbolMenuAction::Grid | SymbolMenuAction::Cancel => None,
        SymbolMenuAction::Finish => {
            (!matches!(editor.tool, SymbolTool::Line | SymbolTool::Polygon)
                || editor.pending_polyline.is_empty())
            .then_some("only a line or a polygon being drawn is finished")
        }
        SymbolMenuAction::RemoveLastPoint => {
            if editor.text_draft.is_some() {
                Some("text is being typed")
            } else {
                (!editor.has_draft()).then_some("nothing is being drawn")
            }
        }
    };
    (refusal.is_none(), refusal.unwrap_or(""))
}

fn apply_menu_request(
    state: &mut CanvasParts<'_>,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    viewport: SymbolViewport,
    request: ContextMenuRequest<SymbolMenuBinding, SymbolMenuAction>,
) {
    let offered = rows(request.binding.target)
        .iter()
        .find_map(|entry| match entry {
            ContextEntry::Command(row) if row.action == request.action => Some(*row),
            _ => None,
        });
    let Some(row) = offered else {
        return;
    };
    if menu_binding(state).as_ref() != Some(&request.binding)
        || state.application_modal_open()
        || !availability(state, request.action).0
    {
        return;
    }
    let click = request.binding.click_pos;
    let here = Some((f64::from(click.x), f64::from(click.y)));
    let modal_open = state.application_modal_open();
    match request.action {
        SymbolMenuAction::Paste => paste_symbol_shape(
            &mut state.design.symbol,
            state.model,
            state.workbench,
            here,
            None,
            modal_open,
        ),
        SymbolMenuAction::Duplicate => duplicate_selected_symbol_shape(
            &mut state.design.symbol,
            state.model,
            state.workbench,
            here,
            None,
            modal_open,
        ),
        SymbolMenuAction::Finish | SymbolMenuAction::RemoveLastPoint => {
            let finish = request.action == SymbolMenuAction::Finish;
            let editor = &state.design.symbol.editor;
            let request = SymbolCanvasRequest {
                source: state.symbol_editor_request_source(),
                tool: editor.tool,
                selection: editor.effective_selection(),
                viewport,
                input: SymbolCanvasInput {
                    pointer: viewport.world_to_screen(editor.pointer.unwrap_or(click)),
                    finish,
                    remove_last_point: !finish,
                    ..SymbolCanvasInput::default()
                },
            };
            apply_canvas_request(state, document, metadata, request);
        }
        _ => {
            if let RowKey::Command(key) = row.key {
                state.workbench.outbox.push(Execute(key));
            }
        }
    }
}
