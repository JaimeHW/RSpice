//! Parsed symbol artwork with disposable egui drawing caches.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use super::render::{BakedSymbol, bake_symbol_with_dimensions};
use rspice_design::symbol_artwork::{Symbol, SymbolError, SymbolLibrary as ArtworkLibrary};

type BakedSymbolKey = (usize, u32, u32, i32, bool, bool);
type BakedSymbolCache = RefCell<HashMap<BakedSymbolKey, Rc<BakedSymbol>>>;

/// Immutable authored artwork and the geometry cached for interactive painting.
pub struct SymbolLibrary {
    artwork: ArtworkLibrary,
    baked: BakedSymbolCache,
}

impl Deref for SymbolLibrary {
    type Target = ArtworkLibrary;

    fn deref(&self) -> &Self::Target {
        &self.artwork
    }
}

impl SymbolLibrary {
    pub fn load_embedded() -> Result<Self, SymbolError> {
        Ok(Self {
            artwork: ArtworkLibrary::load_embedded()?,
            baked: RefCell::new(HashMap::new()),
        })
    }

    /// Flattened (baked) geometry for a symbol under the given orientation,
    /// cached for the process lifetime.
    pub fn baked(
        &self,
        symbol: &Symbol,
        rotation_degrees: i32,
        mirror_h: bool,
        mirror_v: bool,
    ) -> Rc<BakedSymbol> {
        let key = (
            symbol as *const Symbol as usize,
            symbol.target_width.to_bits(),
            symbol.target_height.to_bits(),
            rotation_degrees.rem_euclid(360),
            mirror_h,
            mirror_v,
        );
        if let Some(hit) = self.baked.borrow().get(&key) {
            return Rc::clone(hit);
        }
        let baked = Rc::new(bake_symbol_with_dimensions(
            symbol,
            symbol.target_width,
            symbol.target_height,
            rotation_degrees,
            mirror_h,
            mirror_v,
        ));
        self.baked.borrow_mut().insert(key, Rc::clone(&baked));
        baked
    }

    /// Placement-sized baked geometry for an immutable embedded asset.
    pub fn baked_asset(
        &self,
        filename: &str,
        target_width: f32,
        target_height: f32,
        rotation_degrees: i32,
        mirror_h: bool,
        mirror_v: bool,
    ) -> Option<Rc<BakedSymbol>> {
        let symbol = self.get_asset(filename)?;
        let key = (
            symbol as *const Symbol as usize,
            target_width.to_bits(),
            target_height.to_bits(),
            rotation_degrees.rem_euclid(360),
            mirror_h,
            mirror_v,
        );
        if let Some(hit) = self.baked.borrow().get(&key) {
            return Some(Rc::clone(hit));
        }
        let baked = Rc::new(bake_symbol_with_dimensions(
            symbol,
            target_width,
            target_height,
            rotation_degrees,
            mirror_h,
            mirror_v,
        ));
        self.baked.borrow_mut().insert(key, Rc::clone(&baked));
        Some(baked)
    }

    #[cfg(test)]
    fn get(
        &self,
        kind: rspice_design::schematic::component_type::ComponentType,
    ) -> Option<&Symbol> {
        self.artwork
            .get_with_rotation(kind, 0)
            .map(|(symbol, _)| symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::component_type::ComponentType;

    /// The loop probe as the schematic renderer actually draws it.
    ///
    /// This stood as an `#[ignore]`d PNG dump, which is a test only in the
    /// sense that it compiled: it wrote a file and asserted nothing, so it
    /// could not fail and never ran. The design artwork tests judge the *parsed paths*,
    /// which leaves the whole of `draw_symbol_with_dimensions` — the transform
    /// from viewBox coordinates to the placement it was given — unjudged.
    ///
    /// So the render is kept and the claims the artwork makes are asserted off
    /// it. The probe is a 0 V source: Tian's method measures the loop without
    /// opening it, and the drawing has to say so, which means the conductor
    /// runs terminal to terminal with no gap and the injection plane is marked
    /// beside it rather than cut into it. A conductor that broke would draw
    /// exactly like a source, at the one place in the schematic where the
    /// difference is the entire analysis.
    ///
    /// The resistor is drawn beside it at a second placement, so a transform
    /// that ignored the centre it was handed fails here too.
    #[test]
    fn the_rendered_loop_probe_is_an_unbroken_conductor_through_a_marked_plane() {
        /// Where a viewBox point lands, for a 40x20 symbol drawn at `scale`.
        fn at(centre: egui::Pos2, scale: f32, x: f32, y: f32) -> egui::Pos2 {
            egui::pos2(centre.x + (x - 20.0) * scale, centre.y + (y - 10.0) * scale)
        }

        const SCALE: f32 = 5.0;
        let probe_centre = egui::pos2(280.0, 70.0);
        let resistor_centre = egui::pos2(280.0, 180.0);

        let library = SymbolLibrary::load_embedded().expect("library loads");
        let canvas = rspice_ui_kit::raster::render(egui::vec2(560.0, 260.0), |ui, _| {
            let painter = ui.painter().clone();
            let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(240, 240, 240));
            for (centre, kind) in [
                (probe_centre, ComponentType::LoopProbe),
                (resistor_centre, ComponentType::Resistor),
            ] {
                let symbol = library.get(kind).expect("symbol");
                super::super::render::draw_symbol_with_dimensions(
                    &painter, symbol, 40.0, 20.0, centre, SCALE, 0, false, false, stroke,
                );
            }
        });

        // A 3x3 window, because a 2-point stroke is two pixels wide and the
        // tessellator feathers its edges: sampling one pixel would be asking
        // about antialiasing rather than about the shape.
        let inked = |point: egui::Pos2| {
            let window = egui::Rect::from_center_size(point, egui::vec2(3.0, 3.0));
            let pixels: Vec<_> = canvas.pixels_in(window).collect();
            assert!(
                !pixels.is_empty(),
                "the sample window at {point:?} fell outside the canvas"
            );
            pixels.iter().any(|pixel| *pixel != canvas.background())
        };

        // The conductor, terminal to terminal, with no gap anywhere along it.
        // Stepped in whole viewBox units, which is finer than any feature of
        // the artwork and coarse enough not to be an assertion about the
        // rasterizer's filtering.
        let mut gaps = Vec::new();
        for step in 0..=40 {
            let x = step as f32;
            if !inked(at(probe_centre, SCALE, x, 10.0)) {
                gaps.push(x);
            }
        }
        assert!(
            gaps.is_empty(),
            "the loop probe's conductor is broken at viewBox x {gaps:?}; a probe drawn with a \
             gap is drawn as a source, and it is a short at every operating point"
        );

        // The injection plane, marked across the conductor and reaching past
        // the circle at both ends.
        assert!(
            inked(at(probe_centre, SCALE, 20.0, 2.0)),
            "the transverse bar does not reach above the circle"
        );
        assert!(
            inked(at(probe_centre, SCALE, 20.0, 18.0)),
            "the transverse bar does not reach below the circle"
        );

        // The circle itself, sampled where neither the conductor nor the bar
        // can account for the ink: 45 degrees off centre.
        let offset = 6.0 / std::f32::consts::SQRT_2;
        assert!(
            inked(at(probe_centre, SCALE, 20.0 + offset, 10.0 - offset)),
            "the probe circle does not render"
        );

        // Nothing between the two symbols, so the probe is inside the box it
        // declares rather than merely overlapping it.
        assert!(
            !inked(egui::pos2(
                probe_centre.x,
                probe_centre.y.midpoint(resistor_centre.y)
            )),
            "something painted outside the symbol boxes"
        );

        // And the second symbol was drawn at the second placement, which is
        // what proves the transform reads the centre it is handed.
        assert!(
            inked(at(resistor_centre, SCALE, 0.0, 10.0)),
            "the resistor did not reach its own left terminal"
        );
        assert!(
            inked(at(resistor_centre, SCALE, 40.0, 10.0)),
            "the resistor did not reach its own right terminal"
        );
    }
}

#[cfg(test)]
mod browser_audit {
    use super::*;
    use crate::component_palette;
    use rspice_design::schematic::component_type::ComponentType;

    /// Endpoints reachable by the pen in a path (segment ends only).
    fn endpoints(symbol: &Symbol) -> Vec<(f32, f32)> {
        let mut points = Vec::new();
        for path in &symbol.paths {
            for command in &path.commands {
                match command {
                    rspice_design::symbol_artwork::PathCommand::MoveTo(x, y)
                    | rspice_design::symbol_artwork::PathCommand::LineTo(x, y) => {
                        points.push((*x, *y))
                    }
                    rspice_design::symbol_artwork::PathCommand::CurveTo { end, .. } => {
                        points.push(*end)
                    }
                    rspice_design::symbol_artwork::PathCommand::Close => {}
                }
            }
        }
        points
    }

    /// Every component the browser offers must resolve to a symbol whose
    /// artwork actually reaches each of its terminal grid points — the
    /// mechanical definition of "the pins line up".
    #[test]
    fn every_palette_entry_has_an_aligned_symbol() {
        let library = SymbolLibrary::load_embedded().expect("library loads");

        for section in component_palette() {
            for entry in section.entries {
                let kind: ComponentType = entry.kind;
                let symbol = library
                    .get(kind)
                    .unwrap_or_else(|| panic!("{:?} ({}) has no symbol", kind, entry.label));

                let (vb_w, vb_h) = (symbol.bounds.2, symbol.bounds.3);
                let (target_w, target_h) = kind.symbol_dimensions();
                // Map grid-unit terminal offsets into viewBox coordinates.
                let scale_x = vb_w / target_w as f32;
                let scale_y = vb_h / target_h as f32;
                let (cx, cy) = (vb_w * 0.5, vb_h * 0.5);

                let points = endpoints(symbol);
                for (pin, offset) in kind.terminal_offsets() {
                    let expected = (
                        cx + offset.x as f32 * scale_x,
                        cy + offset.y as f32 * scale_y,
                    );
                    let reached = points.iter().any(|(x, y)| {
                        (x - expected.0).abs() <= 0.75 && (y - expected.1).abs() <= 0.75
                    });
                    assert!(
                        reached,
                        "{:?} ({}): pin '{}' at viewBox ({:.1},{:.1}) is not reached by the artwork",
                        kind, entry.label, pin, expected.0, expected.1
                    );
                }
            }
        }
    }
}
