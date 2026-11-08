//! Lettering a printed schematic where the canvas letters it.
//!
//! The positions are the shared layout's (`crate::sources::schematic_text`);
//! this sets them in the type and ink the SVG export uses (D1 section 11),
//! hung off their anchors by Plex's metrics, so a printed page and an
//! exported one read the same.

use super::*;
use crate::sources::{SchematicText, SchematicTextClass, SemanticComponent, instance_texts};
use rspice_design::resolved_symbol::ResolvedCellSymbol;
use rspice_design::schematic::label_layout::{self, LayoutPoint, PlacedText, TextEdge};
use rspice_design::symbol_generation::fit_pin_name;

/// The print palette's text inks: the light screen tokens the SVG export
/// letters a sheet in, whatever the theme.
pub(in crate::render) const NAME_INK: Rgb8 = Rgb8::new(0x1F, 0x25, 0x28);
pub(in crate::render) const VALUE_INK: Rgb8 = Rgb8::new(0x53, 0x59, 0x5D);
/// Net names, bus declarations and tap slices.
pub(in crate::render) const NET_INK: Rgb8 = Rgb8::new(0x7D, 0x52, 0x00);

/// A world-unit coordinate in exact micrometres.
fn world_um(value: f32) -> i64 {
    (f64::from(value) * SCHEMATIC_UNIT_UM as f64).round() as i64
}

impl SchematicTextClass {
    const fn font(self) -> SceneFont {
        match self {
            Self::Name => SceneFont::SansSemibold,
            Self::Value => SceneFont::Sans,
            Self::Slice | Self::NetName => SceneFont::MonospaceMedium,
        }
    }

    const fn ink(self) -> Rgb8 {
        match self {
            Self::Name => NAME_INK,
            Self::Value => VALUE_INK,
            Self::Slice | Self::NetName => NET_INK,
        }
    }
}

impl SemanticSceneCompiler<'_> {
    /// An instance's name and value, outside its body for every turn and
    /// mirror, as the canvas sets them.
    pub(super) fn instance_labels(
        &mut self,
        semantic: &SemanticComponent,
    ) -> Result<(), HardcopyRenderError> {
        for text in instance_texts(semantic) {
            self.schematic_text(&text)?;
        }
        Ok(())
    }

    /// One run of schematic text, a line per line, hung off its world anchor
    /// the way the canvas hangs a galley: justified by its anchor edge, each
    /// baseline Plex's ascent below its line's top, and a run read upward
    /// turned a quarter counter-clockwise about its anchor.
    pub(super) fn schematic_text(
        &mut self,
        text: &SchematicText,
    ) -> Result<(), HardcopyRenderError> {
        self.placed_lines(
            text.placed,
            &text.text,
            text.class.font(),
            text.size_world(),
            text.class.ink(),
        )
    }

    /// `text` hung off its anchor, each line on the baseline the canvas sets
    /// it on.
    fn placed_lines(
        &mut self,
        placed: PlacedText,
        text: &str,
        font: SceneFont,
        size: f32,
        ink: Rgb8,
    ) -> Result<(), HardcopyRenderError> {
        let anchor = match placed.horizontal {
            TextEdge::Start => TextAnchor::Start,
            TextEdge::Center => TextAnchor::Middle,
            TextEdge::End => TextAnchor::End,
        };
        let lines = text.lines().collect::<Vec<_>>();
        let LayoutPoint { x, y } = placed.anchor;
        let rotation = if placed.upward {
            SceneTextRotation::CounterClockwise90
        } else {
            SceneTextRotation::Upright
        };
        for (line, baseline) in lines.iter().zip(label_layout::text_baselines(
            y,
            placed.vertical,
            size,
            lines.len(),
        )) {
            // Turned about the anchor, a line's baseline lies right of it.
            let (x, y) = if placed.upward {
                (x + (baseline - y), y)
            } else {
                (x, baseline)
            };
            let origin = self.signed_micrometre_point(world_um(x), world_um(y))?;
            self.add_text_placed(
                origin,
                line,
                font,
                world_um(size).unsigned_abs(),
                SemanticColor::Exact(ink),
                (anchor, rotation),
            )?;
        }
        Ok(())
    }

    /// The names a placed part's symbol letters at its pins, where the canvas
    /// letters them.
    pub(super) fn symbol_pin_names(
        &mut self,
        document: &SymbolDocument,
        component: &Component,
    ) -> Result<(), HardcopyRenderError> {
        let symbol = ResolvedCellSymbol::from_authored_document(document.clone(), &[]);
        let lettered = document.pin_name_size.is_some();
        for pin in symbol.connectable_pins().filter(|pin| !pin.name_hidden) {
            let turned = component.transform_point(symbol.pin_label_anchor(pin));
            let hang = component.pin_name_hang(pin);
            let placed = PlacedText {
                anchor: LayoutPoint::new(
                    (component.pos.x + turned.x) as f32,
                    (component.pos.y + turned.y) as f32,
                ),
                horizontal: hang.horizontal,
                vertical: hang.vertical,
                upward: hang.upward,
            };
            let (name, font) = if lettered {
                (pin.name.clone(), SceneFont::Sans)
            } else {
                (fit_pin_name(&pin.name), SceneFont::Monospace)
            };
            self.placed_lines(placed, &name, font, document.pin_name_world(), VALUE_INK)?;
        }
        Ok(())
    }
}
