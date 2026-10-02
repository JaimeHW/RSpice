//! Canonical component artwork fitted to compact previews.

use super::{SymbolLibrary, draw_symbol};
use crate::view::drawing;
use rspice_design::schematic::component_type::ComponentType;
use rspice_design_model::port::PortDirection;

/// Paint a component symbol centered in `rect` — used by the component
/// browser's preview pane. Pure presentation: no state access.
///
/// Uses the same canonical SVG symbol the canvas renders, scaled to fit the
/// preview rect. Missing or invalid resolution paints an explicit error state.
pub fn draw_symbol_preview(
    painter: &egui::Painter,
    rect: egui::Rect,
    kind: ComponentType,
    color: egui::Color32,
    symbol_library: Option<&SymbolLibrary>,
) {
    let stroke = egui::Stroke::new(1.6, color);

    if let Some((symbol, rotation)) =
        symbol_library.and_then(|library| library.get_with_rotation_variant(kind, 0, None))
    {
        if let Some(fit) = symbol_preview_scale(rect, symbol.target_width, symbol.target_height) {
            draw_symbol(
                painter,
                symbol,
                rect.center(),
                fit,
                rotation,
                false,
                false,
                stroke,
            );
            if kind == ComponentType::Port {
                drawing::draw_port_direction_overlay(
                    painter,
                    rect.center(),
                    fit,
                    rotation,
                    false,
                    false,
                    PortDirection::default(),
                    stroke,
                );
            }
        } else {
            drawing::draw_symbol_resolution_error(
                painter,
                rect.center(),
                1.0,
                kind,
                "invalid canonical bounds",
            );
        }
    } else {
        drawing::draw_symbol_resolution_error(
            painter,
            rect.center(),
            1.0,
            kind,
            "missing canonical SVG",
        );
    }
}

fn symbol_preview_scale(rect: egui::Rect, target_width: f32, target_height: f32) -> Option<f32> {
    let size = rect.size();
    if !rect.is_finite()
        || size.x <= 0.0
        || size.y <= 0.0
        || !target_width.is_finite()
        || !target_height.is_finite()
        || target_width <= 0.0
        || target_height <= 0.0
    {
        return None;
    }

    // A proportional inset keeps tiny cells usable instead of allowing the
    // fixed browser padding to consume their entire drawing area. Larger
    // previews retain the established six-point maximum breathing room.
    let inset = (size.x.min(size.y) * 0.12).min(6.0);
    let available_width = size.x - 2.0 * inset;
    let available_height = size.y - 2.0 * inset;
    let fit = (available_width / target_width).min(available_height / target_height);
    (fit.is_finite() && fit > 0.0).then_some(fit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_preview_scale_is_positive_adaptive_and_finite() {
        let tiny = symbol_preview_scale(
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(2.0)),
            40.0,
            20.0,
        )
        .expect("a small positive preview remains drawable");
        let roomy = symbol_preview_scale(
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(100.0, 60.0)),
            40.0,
            20.0,
        )
        .expect("a normal preview fits");

        assert!(tiny.is_finite() && tiny > 0.0);
        assert!(roomy.is_finite() && roomy > tiny);
        assert_eq!(
            symbol_preview_scale(
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::ZERO),
                40.0,
                20.0,
            ),
            None
        );
        assert_eq!(
            symbol_preview_scale(
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(f32::INFINITY, 20.0),),
                40.0,
                20.0,
            ),
            None
        );
        assert_eq!(
            symbol_preview_scale(
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(20.0)),
                f32::NAN,
                20.0,
            ),
            None
        );
    }
}
