//! Component placement and shelf-drag ghosts.

use super::super::{
    drawing::{
        compatible_builtin_xspice_asset, draw_artwork_lead_extensions, draw_port_direction_overlay,
        draw_symbol_resolution_error, port_symbol_stroke,
    },
    resolved_symbol_render::{draw_resolved_symbol, resolved_symbol_world_bounds},
    snap_resolution::resolve_grid_pointer,
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use super::PreviewView;
use crate::{
    session::{EditorSession, tool::Tool},
    symbols::{SymbolLibrary, draw_symbol, draw_symbol_with_dimensions},
};
use egui::{Painter, Response, Stroke};
use rspice_design::{
    resolved_symbol::{ResolvedCellSymbol, ResolvedSymbolSource},
    schematic::{
        component::{Component, LibraryCellInstance},
        component_type::ComponentType,
    },
};
use rspice_design_model::{Point, port::PortSpec};

const COMPONENT_PREVIEW_GHOST_ALPHA: f32 = 0.55;

pub struct ShelfPreview<'a> {
    pub component_type: ComponentType,
    pub binding: Option<&'a LibraryCellInstance>,
}

pub fn pending_library_cell_component(
    editor: &EditorSession,
    grid_pos: Point,
) -> Option<Component> {
    Some(
        Component::new(0, ComponentType::CellInstance, grid_pos)
            .with_rotation(editor.preview_rotation)
            .with_mirror_h(editor.preview_mirror_h)
            .with_library_cell(editor.pending_library_cell.as_ref()?.binding.clone()),
    )
}

pub fn draw_component_preview(
    painter: &Painter,
    response: &Response,
    view: &PreviewView<'_>,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
    symbol_library: Option<&SymbolLibrary>,
) {
    if !view.can_edit {
        return;
    }

    let preview_tool = view.editor.tool;
    let preview_rotation_degrees = view.editor.preview_rotation.degrees();
    let preview_mirror_h = view.editor.preview_mirror_h;

    if let Tool::Place(component_type) = preview_tool
        && let Some(hover_pos) = response.hover_pos()
    {
        let grid_pos = resolve_grid_pointer(
            &view.editor.snap_engine,
            view.design.document.grid_size,
            viewport,
            hover_pos,
        )
        .snapped_position;
        let preview_pos = viewport.schematic_to_screen(grid_pos);

        // Ghost the symbol in dimmed accent until it is placed.
        let preview_stroke = Stroke::new(
            1.0 * viewport.zoom,
            rspice_ui_kit::tokens::active_palette()
                .accent
                .gamma_multiply(COMPONENT_PREVIEW_GHOST_ALPHA),
        );

        if component_type == ComponentType::CellInstance
            && let Some(preview_component) = pending_library_cell_component(view.editor, grid_pos)
        {
            // Artwork first, exactly as the canvas resolves it: the ghost
            // must be the symbol that lands when the pointer is released.
            if let Some((library, filename, width, height)) = symbol_library
                .and_then(|library| compatible_builtin_xspice_asset(&preview_component, library))
                && let Some((symbol, adjusted_rotation)) =
                    library.get_asset_with_rotation(filename, preview_rotation_degrees)
            {
                draw_symbol_with_dimensions(
                    painter,
                    symbol,
                    width,
                    height,
                    preview_pos,
                    viewport.zoom,
                    adjusted_rotation,
                    preview_component.mirror_h,
                    preview_component.mirror_v,
                    preview_stroke,
                );
                draw_artwork_lead_extensions(
                    painter,
                    preview_pos,
                    viewport.zoom,
                    &preview_component,
                    preview_stroke,
                );
            } else if let Some(symbol) = symbol_context.pending_library_symbol()
                && symbol.source() == ResolvedSymbolSource::Authored
                && resolved_symbol_world_bounds(&preview_component, symbol).is_some()
            {
                draw_resolved_symbol(
                    painter,
                    preview_pos,
                    viewport.zoom,
                    &preview_component,
                    symbol,
                    preview_stroke,
                );
            } else {
                draw_symbol_resolution_error(
                    painter,
                    preview_pos,
                    viewport.zoom,
                    component_type,
                    "unresolved cell",
                );
            }
            return;
        }

        if let Some((symbol, adjusted_rotation)) = symbol_library.and_then(|library| {
            library.get_with_rotation_variant(component_type, preview_rotation_degrees, None)
        }) {
            let pending_port_spec =
                view.editor
                    .pending_port_sequence
                    .as_ref()
                    .and_then(|sequence| {
                        Some(PortSpec {
                            name: sequence.next_name()?.to_owned(),
                            direction: sequence.direction,
                        })
                    });
            let symbol_stroke = if component_type == ComponentType::Port {
                port_symbol_stroke(
                    preview_stroke,
                    viewport.zoom,
                    false,
                    pending_port_spec.as_ref(),
                )
            } else {
                preview_stroke
            };
            draw_symbol(
                painter,
                symbol,
                preview_pos,
                viewport.zoom,
                adjusted_rotation,
                preview_mirror_h,
                false,
                symbol_stroke,
            );
            if component_type == ComponentType::Port {
                draw_port_direction_overlay(
                    painter,
                    preview_pos,
                    viewport.zoom,
                    adjusted_rotation,
                    preview_mirror_h,
                    false,
                    view.editor
                        .pending_port_sequence
                        .as_ref()
                        .map(|sequence| sequence.direction)
                        .unwrap_or_default(),
                    symbol_stroke,
                );
            }
        } else {
            draw_symbol_resolution_error(
                painter,
                preview_pos,
                viewport.zoom,
                component_type,
                "missing canonical SVG",
            );
        }
    }
}

/// Paint an uncommitted shelf payload without arming a placement tool.
/// Symbol resolution is lazy so canonical artwork keeps precedence.
pub fn draw_shelf_drag_preview(
    painter: &Painter,
    view: &PreviewView<'_>,
    viewport: &Viewport,
    payload: ShelfPreview<'_>,
    pointer_pos: egui::Pos2,
    symbol_library: Option<&SymbolLibrary>,
    resolve_symbol: impl FnOnce(&LibraryCellInstance) -> Option<ResolvedCellSymbol>,
) {
    if !view.can_edit {
        return;
    }
    let grid_pos = resolve_grid_pointer(
        &view.editor.snap_engine,
        view.design.document.grid_size,
        viewport,
        pointer_pos,
    )
    .snapped_position;
    let preview_pos = viewport.schematic_to_screen(grid_pos);
    let rotation = view.editor.preview_rotation;
    let rotation_degrees = rotation.degrees();
    let mirror_h = view.editor.preview_mirror_h;
    let preview_stroke = Stroke::new(
        viewport.zoom,
        rspice_ui_kit::tokens::active_palette()
            .accent
            .gamma_multiply(COMPONENT_PREVIEW_GHOST_ALPHA),
    );

    if let Some(binding) = payload.binding {
        let component = Component::new(0, ComponentType::CellInstance, grid_pos)
            .with_rotation(rotation)
            .with_mirror_h(mirror_h)
            .with_library_cell(binding.clone());
        // Artwork first, exactly as the canvas resolves it.
        if let Some((library, filename, width, height)) =
            symbol_library.and_then(|library| compatible_builtin_xspice_asset(&component, library))
            && let Some((symbol, adjusted_rotation)) =
                library.get_asset_with_rotation(filename, rotation_degrees)
        {
            draw_symbol_with_dimensions(
                painter,
                symbol,
                width,
                height,
                preview_pos,
                viewport.zoom,
                adjusted_rotation,
                mirror_h,
                false,
                preview_stroke,
            );
            draw_artwork_lead_extensions(
                painter,
                preview_pos,
                viewport.zoom,
                &component,
                preview_stroke,
            );
        } else if let Some(symbol) = resolve_symbol(binding)
            && symbol.source() == ResolvedSymbolSource::Authored
            && resolved_symbol_world_bounds(&component, &symbol).is_some()
        {
            draw_resolved_symbol(
                painter,
                preview_pos,
                viewport.zoom,
                &component,
                &symbol,
                preview_stroke,
            );
        } else {
            draw_symbol_resolution_error(
                painter,
                preview_pos,
                viewport.zoom,
                ComponentType::CellInstance,
                "unresolved cell",
            );
        }
        return;
    }

    let component_type = payload.component_type;
    if let Some((symbol, adjusted_rotation)) = symbol_library.and_then(|library| {
        library.get_with_rotation_variant(component_type, rotation_degrees, None)
    }) {
        draw_symbol(
            painter,
            symbol,
            preview_pos,
            viewport.zoom,
            adjusted_rotation,
            mirror_h,
            false,
            preview_stroke,
        );
    } else {
        draw_symbol_resolution_error(
            painter,
            preview_pos,
            viewport.zoom,
            component_type,
            "missing canonical SVG",
        );
    }
}
