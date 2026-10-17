//! Drawing a printed schematic's connectivity the way the canvas draws it.
//!
//! A bus is three strands with its declaration beside it, and a tap takes
//! the canvas's angled breakout with its bit slice at its end, so nets a bus
//! joins read as joined on paper. A net label carries its attachment mark,
//! an off-sheet connector its direction tab, and a port its direction
//! arrow, so nets joined by a name read as joined too. Geometry comes from
//! `rspice_design::schematic` (`bus_route`, `label_layout`, `port_overlay`),
//! the strokes are the SVG export's nominal widths (D1 section 1) and the
//! lettering its classes (section 11).

use super::*;
use crate::sources::{bus_declaration_text, net_label_text, tap_slice_text};
use rspice_design::schematic::{
    bus::{Bus, BusTap},
    bus_route,
    label_layout::{self, LayoutPoint},
    net_label::NetLabel,
    port_overlay,
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

    /// A world-unit point of the shared layout in this scene.
    fn layout_point(&self, point: LayoutPoint) -> Result<ScenePoint, HardcopyRenderError> {
        let um = |value: f32| (f64::from(value) * SCHEMATIC_UNIT_UM as f64).round() as i64;
        self.signed_micrometre_point(um(point.x), um(point.y))
    }

    /// A net label as the canvas draws it: the filled diamond that marks
    /// where it attaches, its name up and right of the mark, and for an
    /// off-sheet connector the tab whose arrow says which way the signal
    /// goes.
    pub(super) fn schematic_net_label(
        &mut self,
        label: &NetLabel,
    ) -> Result<(), HardcopyRenderError> {
        let ink = SemanticColor::Exact(super::schematic_text::NET_INK);
        let layout = label_layout::net_label_layout(label);
        let (x, y) = (label.pos.x as f32, label.pos.y as f32);
        let half = layout.diamond_half;
        let points = [(x, y - half), (x + half, y), (x, y + half), (x - half, y)]
            .into_iter()
            .map(|(x, y)| self.layout_point(LayoutPoint::new(x, y)))
            .collect::<Result<_, _>>()?;
        self.primitives.push(ScenePrimitive::Polyline {
            points,
            closed: true,
            // A hairline in the fill's ink, so the mark is its own size.
            stroke: StrokeStyle::try_new(
                ink,
                Length::from_micrometres(25),
                StrokePattern::Solid,
                None,
            )?,
            fill: Some(SceneFill::solid(ink)),
        });
        self.schematic_text(&net_label_text(label))?;
        if let Some((tab, direction)) = layout.tab {
            let stroke = StrokeStyle::try_new(
                ink,
                width_um(tab.height() * label_layout::CONNECTOR_TAB_STROKE_PER_HEIGHT),
                StrokePattern::Solid,
                None,
            )?;
            let corners = [
                tab.min,
                LayoutPoint::new(tab.max.x, tab.min.y),
                tab.max,
                LayoutPoint::new(tab.min.x, tab.max.y),
            ]
            .into_iter()
            .map(|corner| self.layout_point(corner))
            .collect::<Result<_, _>>()?;
            self.primitives.push(ScenePrimitive::Polyline {
                points: corners,
                closed: true,
                stroke,
                fill: None,
            });
            for [start, end] in label_layout::connector_tab_marks(tab, direction) {
                self.primitives.push(ScenePrimitive::Line {
                    from: self.layout_point(start)?,
                    to: self.layout_point(end)?,
                    stroke,
                });
            }
        }
        Ok(())
    }

    /// A port's direction arrow over its body, turned and mirrored with the
    /// symbol variant the body was drawn from, as the canvas draws it.
    pub(super) fn port_direction_marks(
        &mut self,
        library: &SymbolLibrary,
        component: &Component,
        stroke: StrokeStyle,
    ) -> Result<(), HardcopyRenderError> {
        let Some((_, rotation)) = library.get_with_rotation_variant(
            component.kind,
            component.rotation.degrees(),
            component.symbol_variant.as_deref(),
        ) else {
            return Ok(());
        };
        let direction = component
            .port_spec()
            .map(|port| port.direction)
            .unwrap_or_default();
        let place = |point| {
            let point = port_overlay::transform_point(
                point,
                rotation,
                component.mirror_h,
                component.mirror_v,
            );
            LayoutPoint::new(
                component.pos.x as f32 + point.x,
                component.pos.y as f32 + point.y,
            )
        };
        for segment in port_overlay::direction_segments(direction) {
            self.primitives.push(ScenePrimitive::Line {
                from: self.layout_point(place(segment.start))?,
                to: self.layout_point(place(segment.end))?,
                stroke,
            });
        }
        Ok(())
    }
}
