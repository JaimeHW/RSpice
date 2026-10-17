//! Lettering a printed schematic where the canvas letters it.
//!
//! The positions are the shared layout's (`crate::sources::schematic_text`);
//! this sets them in the type and ink the SVG export uses (D1 section 11),
//! hung off their anchors by Plex's metrics, so a printed page and an
//! exported one read the same.

use super::*;
use crate::sources::{SchematicText, SchematicTextClass, SemanticComponent, instance_texts};
use rspice_design::schematic::label_layout::{self, LayoutPoint, TextEdge};

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
    /// the way the canvas hangs a galley: upright, justified by its anchor
    /// edge, each baseline Plex's ascent below its line's top.
    pub(super) fn schematic_text(
        &mut self,
        text: &SchematicText,
    ) -> Result<(), HardcopyRenderError> {
        let size = text.class.size_world();
        let anchor = match text.placed.horizontal {
            TextEdge::Start => TextAnchor::Start,
            TextEdge::Center => TextAnchor::Middle,
            TextEdge::End => TextAnchor::End,
        };
        let lines = text.text.lines().collect::<Vec<_>>();
        let LayoutPoint { x, y } = text.placed.anchor;
        for (line, baseline) in lines.iter().zip(label_layout::text_baselines(
            y,
            text.placed.vertical,
            size,
            lines.len(),
        )) {
            let origin = self.signed_micrometre_point(world_um(x), world_um(baseline))?;
            self.add_text_placed(
                origin,
                line,
                text.class.font(),
                world_um(size).unsigned_abs(),
                SemanticColor::Exact(text.class.ink()),
                (anchor, SceneTextRotation::Upright),
            )?;
        }
        Ok(())
    }
}
