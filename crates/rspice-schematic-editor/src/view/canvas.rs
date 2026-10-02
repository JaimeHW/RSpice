//! Stable canvas interaction, focus, and accessibility presentation.

use super::{design_view::DesignView, scene};
use crate::session::{
    EditorSession,
    selection::{SchematicKeyboardFocus, SchematicSelectionFilter},
    tool::Tool,
};
use egui::{Rect, Response, Sense, Ui, WidgetInfo, WidgetType};
use rspice_ui_kit::accessibility::counted;

const SCHEMATIC_CANVAS_INTERACTION_ID: &str = "rspice-schematic-canvas-interaction";

/// Current design and editor state used by the canvas's accessible description.
pub struct CanvasAccessibilityView<'a> {
    pub design: DesignView<'a>,
    pub editor: &'a EditorSession,
    pub keyboard_focus: Option<SchematicKeyboardFocus>,
    pub filter: SchematicSelectionFilter,
    pub traversal_enabled: bool,
}

pub fn contains_pointer(ctx: &egui::Context) -> bool {
    ctx.pointer_hover_pos().is_some_and(|pointer| {
        ctx.read_response(egui::Id::new(SCHEMATIC_CANVAS_INTERACTION_ID))
            .is_some_and(|response| response.rect.contains(pointer))
    })
}

/// Transfer keyboard ownership to the single active schematic canvas. Modal
/// schematic commands call this when they arm so arrow/Enter entry is
/// immediately available without consuming keys from unrelated controls.
pub fn request_focus(ctx: &egui::Context) {
    ctx.memory_mut(|memory| {
        memory.request_focus(egui::Id::new(SCHEMATIC_CANVAS_INTERACTION_ID));
    });
    let key = focus_request_id(ctx);
    let frame = ctx.cumulative_frame_nr();
    ctx.data_mut(|data| data.insert_temp(key, frame));
    ctx.request_repaint();
}

fn focus_request_id(ctx: &egui::Context) -> egui::Id {
    egui::Id::new((
        SCHEMATIC_CANVAS_INTERACTION_ID,
        "focus-request",
        ctx.viewport_id(),
    ))
}

fn apply_focus_request(ui: &Ui, focus_blocked: impl FnOnce() -> bool) {
    let ctx = ui.ctx();
    let key = focus_request_id(ctx);
    let Some(requested_frame) = ctx.data(|data| data.get_temp::<u64>(key)) else {
        return;
    };
    // egui retains the dismissed modal's input floor for one more frame.
    // Retry through its retirement, without retaining a request across tasks
    // or taking focus from a new dialog or drawer.
    let cancelled =
        ctx.cumulative_frame_nr().saturating_sub(requested_frame) > 2 || focus_blocked();
    let available = ui.is_visible()
        && ui.is_enabled()
        && ui.memory(|memory| memory.is_above_modal_layer(ui.layer_id()));
    if cancelled || available {
        ctx.data_mut(|data| data.remove::<u64>(key));
        if !cancelled {
            ctx.memory_mut(|memory| {
                memory.request_focus(egui::Id::new(SCHEMATIC_CANVAS_INTERACTION_ID))
            });
        }
    } else {
        ctx.request_repaint();
    }
}

/// Allocate the canvas response after honoring any pending host focus request.
/// `focus_blocked` is queried only while a current request can still be applied.
pub fn interact(ui: &Ui, available: Rect, focus_blocked: impl FnOnce() -> bool) -> Response {
    apply_focus_request(ui, focus_blocked);
    ui.interact(
        available,
        egui::Id::new(SCHEMATIC_CANVAS_INTERACTION_ID),
        Sense::click_and_drag(),
    )
}

fn accessibility_label() -> &'static str {
    "Schematic canvas"
}

pub fn accessibility_description(view: &CanvasAccessibilityView<'_>, shortcuts: &str) -> String {
    let tool = if view.editor.tool.is_place_tool() {
        format!("Place {}", view.editor.tool.display_name())
    } else {
        view.editor.tool.display_name().to_owned()
    };
    let traversal_instruction = if view.editor.tool == Tool::DocumentationShape {
        " Arrow keys move the exact shape cursor. Space places a point. Enter completes a legal polygon or places the current point. Backspace removes the last point. Escape cancels."
    } else if !view.traversal_enabled || !keyboard_navigation_has_objects(view) {
        ""
    } else {
        " Arrow keys select the nearest eligible schematic object in each direction."
    };
    format!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}.{traversal_instruction} Active tool: {}.{shortcuts}",
        counted(
            view.design.document.components.len(),
            "component",
            "components"
        ),
        counted(view.design.document.wires.len(), "wire", "wires"),
        counted(view.design.document.buses.len(), "bus", "buses"),
        counted(view.design.document.bus_taps.len(), "bus tap", "bus taps"),
        counted(
            view.design.document.junctions.len(),
            "junction",
            "junctions"
        ),
        counted(
            view.design.document.net_labels.len(),
            "net label",
            "net labels"
        ),
        counted(
            view.design.document.design_notes.len(),
            "design note",
            "design notes"
        ),
        counted(
            view.design.document.documentation_shapes.len(),
            "documentation shape",
            "documentation shapes"
        ),
        counted(
            view.design.document.probes.len(),
            "probe flag",
            "probe flags"
        ),
        tool,
    )
}

pub fn selection_accessibility_status(view: &CanvasAccessibilityView<'_>) -> String {
    if let Some(component) = view.editor.selection.single_component().and_then(|id| {
        view.design
            .document
            .components
            .iter()
            .find(|component| component.id == id)
    }) {
        let name = component.name.trim();
        let name = if name.is_empty() { "unnamed" } else { name };
        let value = component.value.trim();
        let value = if !value.is_empty() {
            value
        } else {
            component.library_cell.as_ref().map_or_else(
                || component.kind.display_name(),
                |binding| {
                    binding
                        .module_name
                        .as_deref()
                        .unwrap_or(binding.cell.as_str())
                },
            )
        };
        return format!("Selected instance {name}, {value}.");
    }

    if let Some(focus) = view.keyboard_focus.filter(|focus| {
        scene::keyboard_focus_matches_selection(&view.design, &view.editor.selection, *focus)
    }) {
        return format!("Selected {}.", keyboard_focus_label(view, focus));
    }

    match view.editor.selection.count() {
        0 => "No schematic object selected.".to_owned(),
        1 => "One schematic object selected.".to_owned(),
        count => format!("{count} schematic objects selected."),
    }
}

fn keyboard_navigation_has_objects(view: &CanvasAccessibilityView<'_>) -> bool {
    let filter = view.filter;
    (filter.instances && !view.design.document.components.is_empty())
        || (filter.wires
            && (!view.design.document.wires.is_empty()
                || !view.design.document.buses.is_empty()
                || !view.design.document.bus_taps.is_empty()
                || !view.design.document.junctions.is_empty()))
        || (filter.labels && !view.design.document.net_labels.is_empty())
        || (filter.annotations
            && (!view.design.document.design_notes.is_empty()
                || !view.design.document.documentation_shapes.is_empty()
                || !view.design.document.probes.is_empty()))
}

fn keyboard_focus_label(
    view: &CanvasAccessibilityView<'_>,
    focus: SchematicKeyboardFocus,
) -> String {
    let id = match focus {
        SchematicKeyboardFocus::Component(id)
        | SchematicKeyboardFocus::Wire(id)
        | SchematicKeyboardFocus::Bus(id)
        | SchematicKeyboardFocus::BusTap(id)
        | SchematicKeyboardFocus::Junction(id)
        | SchematicKeyboardFocus::NetLabel(id)
        | SchematicKeyboardFocus::Probe(id)
        | SchematicKeyboardFocus::DesignNote(id)
        | SchematicKeyboardFocus::DocumentationShape(id) => id,
    };
    match focus {
        SchematicKeyboardFocus::Component(_) => format!("component {id}"),
        SchematicKeyboardFocus::Wire(_) => format!("wire {id}"),
        SchematicKeyboardFocus::Bus(_) => format!("bus {id}"),
        SchematicKeyboardFocus::BusTap(_) => format!("bus tap {id}"),
        SchematicKeyboardFocus::Junction(_) => format!("junction {id}"),
        SchematicKeyboardFocus::NetLabel(_) => view
            .design
            .document
            .net_labels
            .iter()
            .find(|label| label.id == id)
            .map_or_else(
                || format!("net label {id}"),
                |label| format!("net label {}", label.name),
            ),
        SchematicKeyboardFocus::Probe(_) => view
            .design
            .document
            .probes
            .iter()
            .find(|probe| probe.id == id)
            .map_or_else(
                || format!("probe {id}"),
                |probe| format!("probe {}", probe.reference),
            ),
        SchematicKeyboardFocus::DesignNote(_) => format!("design note {id}"),
        SchematicKeyboardFocus::DocumentationShape(_) => {
            format!("documentation shape {id}")
        }
    }
}

/// Publish accessible canvas/selection state and paint the final focus ring.
pub fn finish(
    ui: &Ui,
    response: &Response,
    available: Rect,
    view: &CanvasAccessibilityView<'_>,
    shortcuts: &str,
) {
    let accessibility_label = accessibility_label();
    let accessibility_description = accessibility_description(view, shortcuts);
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Image, ui.is_enabled(), accessibility_label)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Canvas);
        node.set_label(accessibility_label);
        node.set_description(accessibility_description);
    });
    let selection_status = selection_accessibility_status(view);
    let selection_status_response = ui.interact(
        egui::Rect::from_min_size(available.min, egui::Vec2::splat(1.0)),
        response.id.with("selection-status"),
        egui::Sense::hover(),
    );
    ui.ctx()
        .accesskit_node_builder(selection_status_response.id, |node| {
            node.set_role(egui::accesskit::Role::Status);
            node.set_label(selection_status);
            node.set_live(egui::accesskit::Live::Polite);
        });
    // The canvas takes keyboard focus for tool, nudge, and routing shortcuts,
    // so it owes a visible focus ring like every other custom click target.
    // Painted last so the schematic artwork does not cover it.
    rspice_ui_kit::theme::paint_focus_ring(ui, response, available);
}
