//! The glyph a ground symbol draws.

use serde::{Deserialize, Serialize};

use super::component_type::ComponentType;

/// Half the top edge of LTspice's ground flag, which is also how far its apex
/// hangs below the net point: LTspice draws an open triangle 36 units across
/// and 18 deep, which the importer draws at five quarters.
const LTSPICE_REACH: f32 = 22.5;

/// Half the top edge of QSPICE's ground, and its depth: QSPICE draws a
/// stemless triangle 165 units across and 82.5 deep, which the importer draws
/// at a tenth.
const QSPICE_REACH: f32 = 8.25;

/// How a ground symbol draws. RSpice's own ground hangs shrinking bars below
/// a stem. A ground flag imported from another tool keeps that tool's glyph,
/// an open triangle hung from the net point, so whatever the source placed
/// clear of its ground stays clear of it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroundGlyph {
    #[default]
    Native,
    Ltspice,
    Qspice,
}

impl GroundGlyph {
    /// Every glyph a ground imported from another tool draws.
    pub const IMPORTED: [Self; 2] = [Self::Ltspice, Self::Qspice];

    pub fn is_native(&self) -> bool {
        *self == Self::Native
    }

    /// The triangle an imported glyph draws, in schematic units about the
    /// ground symbol's position before it is turned: its top edge centred on
    /// the terminal, its apex below. None for RSpice's own glyph.
    pub fn outline(self) -> Option<[(f32, f32); 3]> {
        let reach = match self {
            Self::Native => return None,
            Self::Ltspice => LTSPICE_REACH,
            Self::Qspice => QSPICE_REACH,
        };
        let top = ComponentType::Ground
            .terminal_offsets()
            .first()
            .map_or(0.0, |(_, terminal)| terminal.y as f32);
        Some([(-reach, top), (reach, top), (0.0, top + reach)])
    }

    /// Half the width and half the height of the box about the position
    /// that holds the glyph, never smaller than the ground symbol's own box.
    /// None for RSpice's own glyph.
    pub fn half_extent(self) -> Option<(f32, f32)> {
        let outline = self.outline()?;
        let (width, height) = ComponentType::Ground.symbol_dimensions();
        Some(outline.iter().fold(
            (width as f32 * 0.5, height as f32 * 0.5),
            |(half_width, half_height), &(x, y)| {
                (half_width.max(x.abs()), half_height.max(y.abs()))
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use rspice_design_model::Point;

    use super::*;
    use crate::schematic::component::Component;

    /// A ground stores no glyph while it draws RSpice's own, so native
    /// drawings store as they did; an imported ground keeps its tool's.
    #[test]
    fn only_an_imported_ground_stores_its_glyph() {
        let mut ground = Component::new(1, ComponentType::Ground, Point::new(0, 10));
        let native = serde_json::to_string(&ground).expect("serializes");
        assert!(!native.contains("ground_glyph"), "{native}");
        ground.ground_glyph = GroundGlyph::Ltspice;
        let imported = serde_json::to_string(&ground).expect("serializes");
        let read: Component = serde_json::from_str(&imported).expect("reads back");
        assert_eq!(read.ground_glyph, GroundGlyph::Ltspice);
    }

    /// An imported ground's box holds its whole triangle: LTspice's reaches
    /// past RSpice's own ground across and below; QSPICE's stays inside it.
    #[test]
    fn an_imported_grounds_box_holds_its_glyph() {
        let mut ground = Component::new(1, ComponentType::Ground, Point::new(0, 10));
        assert_eq!(ground.symbol_dimensions(), (20, 20));
        ground.ground_glyph = GroundGlyph::Qspice;
        assert_eq!(ground.symbol_dimensions(), (20, 20));
        ground.ground_glyph = GroundGlyph::Ltspice;
        let (width, height) = ground.symbol_dimensions();
        let outline = GroundGlyph::Ltspice.outline().expect("an imported glyph");
        assert!(
            outline
                .iter()
                .all(|&(x, y)| { x.abs() <= width as f32 * 0.5 && y.abs() <= height as f32 * 0.5 })
        );
        ground.symbol_variant = Some("earth".to_owned());
        assert_eq!(ground.symbol_dimensions(), (20, 20));
    }
}
