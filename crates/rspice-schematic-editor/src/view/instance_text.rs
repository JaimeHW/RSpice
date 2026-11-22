//! A part's name and value on the canvas: where each stands, the box it
//! takes, and which one the pointer is on.
//!
//! The two texts belong to their part and are not objects of their own: a
//! click on either takes the part, a drag moves the text alone, a turn of the
//! part turns them with it, and a double-click edits the text where it
//! stands. Virtuoso keeps instance labels out of the selection by default
//! for the same reason: a stray click on a label should not leave the part
//! behind.

use super::{
    design_view::DesignView, lod, resolved_symbol_render::resolved_symbol_world_bounds,
    symbol_context::SchematicSymbolContext, viewport::Viewport,
};
use crate::source_labels::component_value_label_cached;
use egui::{Pos2, Rect};
use rspice_design::{
    resolved_symbol::{ResolvedCellSymbol, ResolvedSymbolSource},
    schematic::{
        component::Component,
        component_display::SchematicParameterLabelVisibility,
        component_type::ComponentType,
        label_layout::{
            self, INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD, LayoutRect, PlacedText, ValueClass,
        },
        label_position::InstanceText,
        value_label::{cell_instance_value, value_pieces},
    },
};
use std::borrow::Borrow;

/// How far outside its lettering a name or value still takes a click, in
/// points. Kept small: the text often sits against its own body, and a press
/// on the body moves the part.
const TEXT_PICK_SLOP: f32 = 1.0;

/// A part's name or value under the pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstanceTextHit {
    pub component: u64,
    pub text: InstanceText,
}

/// The authored symbol whose anchors `component`'s name and value stand at,
/// as the canvas draws the part; `None` for a part the shared layout letters
/// clear of its body.
pub fn lettering_symbol<'a>(
    component: &'a Component,
    symbols: &'a SchematicSymbolContext,
) -> Option<&'a ResolvedCellSymbol> {
    if !component.is_drawn_from_a_symbol_document() {
        return None;
    }
    // A built-in artwork letters its texts as a catalog part does.
    if component.source_symbol().is_none() && component.builtin_artwork().is_some() {
        return None;
    }
    let symbol = component.source_symbol().or_else(|| {
        (component.kind == ComponentType::CellInstance)
            .then(|| symbols.resolved_symbol(component))
            .flatten()
    })?;
    (symbol.source() == ResolvedSymbolSource::Authored
        && resolved_symbol_world_bounds(component, symbol).is_some())
    .then_some(symbol)
}

/// Where `component` letters `text` on the canvas, in world units.
pub fn placement(
    component: &Component,
    symbols: &SchematicSymbolContext,
    text: InstanceText,
) -> PlacedText {
    label_layout::instance_text_placement(
        component,
        lettering_symbol(component, symbols).map(ResolvedCellSymbol::document),
        text,
    )
}

/// The box `text` takes on the canvas in world units, and its type size,
/// while the canvas letters it under `labels`; `None` for a text the canvas
/// does not draw. A value lettered in an imported drawing's own pieces is
/// not one text, and is left to its part.
pub fn lettered_box(
    component: &Component,
    symbols: &SchematicSymbolContext,
    labels: SchematicParameterLabelVisibility,
    text: InstanceText,
) -> Option<(LayoutRect, f32)> {
    let symbol = lettering_symbol(component, symbols).map(ResolvedCellSymbol::document);
    if symbol.is_none()
        && (component.kind == ComponentType::Ground
            || labels == SchematicParameterLabelVisibility::Hidden)
    {
        return None;
    }
    let placed = label_layout::instance_text_placement(component, symbol, text);
    let boxed = |lettering: &str, size: f32| {
        (!lettering.trim().is_empty())
            .then(|| (label_layout::text_box(placed, lettering, size), size))
    };
    match text {
        InstanceText::Name => {
            let shown = component.display_mode.show_name(labels)
                && !symbol.is_some_and(|document| document.name_hidden);
            shown
                .then(|| {
                    boxed(
                        &component.name,
                        component.name_label_pos.size_world(INSTANCE_NAME_WORLD),
                    )
                })
                .flatten()
        }
        InstanceText::Value => {
            let shown = component.display_mode.show_value(labels)
                && !symbol.is_some_and(|document| document.value_hidden);
            if !shown {
                return None;
            }
            let class = label_layout::instance_value_class(component);
            if value_pieces(component).is_some() && (symbol.is_some() || class == ValueClass::Value)
            {
                return None;
            }
            let size = match class {
                ValueClass::NetName if symbol.is_none() => class.size_world(),
                _ => component.value_label_pos.size_world(INSTANCE_VALUE_WORLD),
            };
            if symbol.is_some() {
                boxed(cell_instance_value(component), size)
            } else {
                boxed(&component_value_label_cached(component), size)
            }
        }
    }
}

/// The names and values of `components` under `pointer`, topmost part first
/// and each part's name before its value: the first only, unless `all`. A
/// text too small to read at this zoom is not drawn and takes nothing.
pub fn instance_texts_at<C: Borrow<Component>>(
    view: &DesignView<'_>,
    components: &[C],
    symbols: &SchematicSymbolContext,
    viewport: &Viewport,
    pixels_per_point: f32,
    pointer: Pos2,
    all: bool,
) -> Vec<InstanceTextHit> {
    let mut found = Vec::new();
    // Instances paint in document order, so the last one is on top.
    for component in components.iter().rev().map(Borrow::borrow) {
        for text in [InstanceText::Name, InstanceText::Value] {
            let Some((world, size)) = lettered_box(component, symbols, view.parameter_labels, text)
            else {
                continue;
            };
            if lod::annotation_size(size, viewport.zoom, pixels_per_point).is_none() {
                continue;
            }
            if screen_rect(viewport, world)
                .expand(TEXT_PICK_SLOP)
                .contains(pointer)
            {
                found.push(InstanceTextHit {
                    component: component.id,
                    text,
                });
                if !all {
                    return found;
                }
            }
        }
    }
    found
}

/// A world box on the screen.
pub fn screen_rect(viewport: &Viewport, world: LayoutRect) -> Rect {
    let at = |x: f32, y: f32| {
        Pos2::new(
            viewport.bounds.min.x + viewport.offset.x + x * viewport.zoom,
            viewport.bounds.min.y + viewport.offset.y + y * viewport.zoom,
        )
    };
    Rect::from_min_max(at(world.min.x, world.min.y), at(world.max.x, world.max.y))
}
