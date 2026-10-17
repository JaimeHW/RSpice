//! Drawing a printed schematic's connectivity the way the canvas draws it.
//!
//! A bus is three strands with its declaration beside it, and a tap takes
//! the canvas's angled breakout with its bit slice at its end, so nets a bus
//! joins read as joined on paper. Geometry comes from
//! `rspice_design::schematic::bus_route`, the strokes are the SVG export's
//! nominal widths (D1 section 1) and the lettering its classes (section 11).

use super::*;
use crate::sources::{bus_declaration_text, tap_slice_text};
use rspice_design::schematic::{
    bus::{Bus, BusTap},
    bus_route,
};

/// The print palette's conductor ink: the light screen wire token the SVG
/// export draws wires, buses and taps in.
pub(in crate::render) const WIRE_INK: Rgb8 = Rgb8::new(0x4E, 0x34, 0x00);

/// A nominal world-unit width in exact micrometres.
pub(super) fn width_um(world: f32) -> Length {
    Length::from_micrometres((f64::from(world) * SCHEMATIC_UNIT_UM as f64).round() as u64)
}

impl SemanticSceneCompiler<'_> {
    /// A bus's three strands, 4 wu apart and mitred through its corners,
    /// and its declaration beside its last point.
    pub(super) fn schematic_bus(
        &mut self,
        bus: &Bus,
        strand: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        if bus.points.len() >= 2 {
            let centreline = bus
                .points
                .iter()
                .map(|point| {
                    (
                        f64::from(point.x) * SCHEMATIC_UNIT_UM as f64,
                        f64::from(point.y) * SCHEMATIC_UNIT_UM as f64,
                    )
                })
                .collect::<Vec<_>>();
            let offset = f64::from(bus_route::BUS_STRAND_OFFSET_WORLD) * SCHEMATIC_UNIT_UM as f64;
            for offset in [-offset, 0.0, offset] {
                let points = bus_route::offset_polyline(&centreline, offset)
                    .into_iter()
                    .map(|(x, y)| self.signed_micrometre_point(x.round() as i64, y.round() as i64))
                    .collect::<Result<_, _>>()?;
                self.primitives.push(ScenePrimitive::Polyline {
                    points,
                    closed: false,
                    stroke: strand,
                    fill: None,
                });
            }
        }
        if let Some(declaration) = bus_declaration_text(bus) {
            self.schematic_text(&declaration)?;
        }
        Ok(())
    }

    /// A tap along the canvas's route, breakout and all, and the bit it
    /// carries beside its connection point.
    pub(super) fn schematic_bus_tap(
        &mut self,
        tap: &BusTap,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        self.primitives.push(ScenePrimitive::Polyline {
            points: self.schematic_polyline(&bus_route::bus_tap_route_points(tap))?,
            closed: false,
            stroke,
            fill: None,
        });
        self.schematic_text(&tap_slice_text(tap))
    }
}
