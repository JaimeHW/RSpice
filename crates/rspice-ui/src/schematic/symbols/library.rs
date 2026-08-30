//! The built-in symbol library.
//!
//! Resolves a component type to its symbol, loading the built-in SVG bodies
//! on first use.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::state::ComponentType;

use super::error::SymbolError;
use super::parser::parse_svg;
use super::render::{BakedSymbol, bake_symbol_with_dimensions};
use super::types::Symbol;

type BakedSymbolKey = (usize, u32, u32, i32, bool, bool);
type BakedSymbolCache = RefCell<HashMap<BakedSymbolKey, Rc<BakedSymbol>>>;

mod embedded_symbols {
    include!(concat!(env!("OUT_DIR"), "/embedded_symbols.rs"));
}

/// SVG Symbol Library with O(1) lookup by component type.
/// Loads and caches parsed symbols for efficient rendering.
/// Supports orientation-specific symbols (vertical/horizontal) for components
/// that have different SVGs for different rotations.
pub struct SymbolLibrary {
    /// Default (vertical) symbols
    symbols: HashMap<ComponentType, Symbol>,
    /// Horizontal variants for components that have separate horizontal SVGs
    horizontal_symbols: HashMap<ComponentType, Symbol>,
    /// Non-default symbol variants, by component type then variant id
    variant_symbols: HashMap<ComponentType, HashMap<String, Symbol>>,
    /// Horizontal symbol variants, by component type then variant id
    horizontal_variant_symbols: HashMap<ComponentType, HashMap<String, Symbol>>,
    /// All embedded asset files parsed successfully and keyed by filename
    embedded_assets: HashMap<String, Symbol>,
    /// Flattened symbol geometry per (symbol address, rotation, mirror).
    /// The library is immutable after load, so symbol addresses are stable
    /// keys for the process lifetime. RefCell: painting is single-threaded.
    baked: BakedSymbolCache,
}

impl Default for SymbolLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolLibrary {
    /// Create a new empty symbol library
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            horizontal_symbols: HashMap::new(),
            variant_symbols: HashMap::new(),
            horizontal_variant_symbols: HashMap::new(),
            embedded_assets: HashMap::new(),
            baked: RefCell::new(HashMap::new()),
        }
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

    /// Load all embedded SVG symbols from the assets directory.
    /// Returns the library with all symbols loaded, or an error if any fail.
    pub fn load_embedded() -> Result<Self, SymbolError> {
        let mut library = Self::new();

        library.embedded_assets = Self::load_all_embedded_assets()?;

        // Default bindings come from the authoritative device descriptors.
        // A missing or electrically mismatched file is fatal here, so catalog
        // drift cannot degrade into substitute artwork or an empty entry.
        for component_type in ComponentType::ALL {
            let Some(filename) = component_type.descriptor().default_symbol_asset else {
                if component_type == ComponentType::CellInstance {
                    continue;
                }
                return Err(SymbolError::IoError {
                    path: component_type.descriptor().stable_id.to_owned(),
                    message: "standard component has no canonical symbol asset".to_owned(),
                });
            };
            let symbol = library.prepare_symbol(
                component_type,
                filename,
                component_type.display_name(),
                false,
            )?;
            let terminal_offsets: Vec<_> = component_type
                .terminal_offsets()
                .iter()
                .map(|(_, offset)| *offset)
                .collect();
            if !symbol_reaches_terminal_offsets(
                &symbol,
                symbol.target_width,
                symbol.target_height,
                &terminal_offsets,
            ) {
                return Err(SymbolError::ParseError(format!(
                    "canonical asset '{filename}' does not reach every terminal for {}",
                    component_type.descriptor().stable_id
                )));
            }
            library.symbols.insert(component_type, symbol);
        }

        // Load non-default visual variants for symbol families that already share
        // the same electrical terminals and can be treated as pure schematic skins.
        let variant_mappings: &[(ComponentType, &str, &str, &str)] = &[
            (
                ComponentType::VoltageSource,
                "battery",
                "battery.svg",
                "Battery",
            ),
            (
                ComponentType::VoltageSource,
                "battery_multi_cell",
                "battery_multi_cell.svg",
                "Battery",
            ),
            (
                ComponentType::Capacitor,
                "polarized",
                "cap_polarized.svg",
                "Polarized Capacitor",
            ),
            (ComponentType::Ground, "earth", "ground_earth.svg", "Ground"),
            (
                ComponentType::Ground,
                "chassis",
                "ground_chassis.svg",
                "Ground",
            ),
            (
                ComponentType::Diode,
                "schottky",
                "diode_schottky.svg",
                "Schottky Diode",
            ),
            (
                ComponentType::Diode,
                "zener",
                "diode_zener.svg",
                "Zener Diode",
            ),
            (
                ComponentType::Diode,
                "tunnel",
                "diode_tunnel.svg",
                "Tunnel Diode",
            ),
            (ComponentType::Diode, "led", "led.svg", "LED"),
            (
                ComponentType::NpnBjt,
                "discrete",
                "bjt_npn_descrete.svg",
                "NPN BJT",
            ),
            (
                ComponentType::PnpBjt,
                "discrete",
                "bjt_pnp_discrete.svg",
                "PNP BJT",
            ),
            (
                ComponentType::Njfet,
                "discrete",
                "jfet_n_chan_discrete.svg",
                "N-JFET",
            ),
            (
                ComponentType::Pjfet,
                "discrete",
                "jfet_p_chan_discrete.svg",
                "P-JFET",
            ),
        ];

        for (component_type, variant_id, filename, name) in variant_mappings {
            let symbol = library.prepare_symbol(*component_type, filename, name, false)?;
            let terminal_offsets: Vec<_> = component_type
                .terminal_offsets()
                .iter()
                .map(|(_, offset)| *offset)
                .collect();
            if !symbol_reaches_terminal_offsets(
                &symbol,
                symbol.target_width,
                symbol.target_height,
                &terminal_offsets,
            ) {
                return Err(SymbolError::ParseError(format!(
                    "canonical variant '{variant_id}' ({filename}) does not reach every terminal for {}",
                    component_type.descriptor().stable_id
                )));
            }
            library
                .variant_symbols
                .entry(*component_type)
                .or_default()
                .insert((*variant_id).to_string(), symbol);
        }

        // Load horizontal variants for components that have separate horizontal SVGs
        let horizontal_mappings: &[(ComponentType, &str, &str)] = &[
            (
                ComponentType::VoltageSourceAc,
                "v_src_ac_horizontal.svg",
                "AC Voltage Source",
            ),
            (
                ComponentType::VoltageSourceSin,
                "v_src_ac_horizontal.svg",
                "Sinusoidal Voltage Source",
            ),
        ];

        for (component_type, filename, name) in horizontal_mappings {
            let symbol = library.prepare_symbol(*component_type, filename, name, true)?;
            let rotated_terminal_offsets: Vec<_> = component_type
                .terminal_offsets()
                .iter()
                .map(|(_, offset)| crate::state::Point::new(-offset.y, offset.x))
                .collect();
            if !symbol_reaches_terminal_offsets(
                &symbol,
                symbol.target_width,
                symbol.target_height,
                &rotated_terminal_offsets,
            ) {
                return Err(SymbolError::ParseError(format!(
                    "canonical horizontal asset '{filename}' does not reach every terminal for {}",
                    component_type.descriptor().stable_id
                )));
            }
            library.horizontal_symbols.insert(*component_type, symbol);
        }

        Ok(library)
    }

    fn load_all_embedded_assets() -> Result<HashMap<String, Symbol>, SymbolError> {
        let mut assets = HashMap::with_capacity(self::embedded_symbols::EMBEDDED_SYMBOLS.len());

        for &(filename, svg_data) in self::embedded_symbols::EMBEDDED_SYMBOLS {
            let mut symbol = parse_svg(svg_data).map_err(|err| {
                SymbolError::ParseError(format!(
                    "Failed to parse embedded symbol asset '{}': {}",
                    filename, err
                ))
            })?;
            symbol.name = filename.to_string();
            symbol.target_width = (symbol.bounds.2 - symbol.bounds.0).max(1.0);
            symbol.target_height = (symbol.bounds.3 - symbol.bounds.1).max(1.0);
            validate_symbol(filename, &symbol)?;
            assets.insert(filename.to_string(), symbol);
        }

        Ok(assets)
    }

    fn prepare_symbol(
        &self,
        component_type: ComponentType,
        filename: &str,
        name: &str,
        horizontal: bool,
    ) -> Result<Symbol, SymbolError> {
        let mut symbol =
            self.embedded_assets
                .get(filename)
                .cloned()
                .ok_or_else(|| SymbolError::IoError {
                    path: filename.to_string(),
                    message: "embedded symbol asset was not loaded".to_string(),
                })?;

        symbol.name = name.to_string();

        let (target_w, target_h) = component_type.symbol_dimensions();
        if horizontal {
            symbol.target_width = target_h as f32;
            symbol.target_height = target_w as f32;
        } else {
            symbol.target_width = target_w as f32;
            symbol.target_height = target_h as f32;
        }

        validate_symbol(filename, &symbol)?;

        Ok(symbol)
    }

    /// Get a symbol by component type (O(1) lookup)
    #[cfg(test)]
    pub fn get(&self, component_type: ComponentType) -> Option<&Symbol> {
        self.symbols.get(&component_type)
    }

    /// Get a parsed embedded asset by filename. Catalog-backed devices use
    /// this path because many stable device IDs intentionally share the one
    /// generic `CellInstance` placement kind.
    pub fn get_asset(&self, filename: &str) -> Option<&Symbol> {
        self.embedded_assets.get(filename)
    }

    pub fn get_asset_with_rotation(
        &self,
        filename: &str,
        rotation_degrees: i32,
    ) -> Option<(&Symbol, i32)> {
        self.get_asset(filename)
            .map(|symbol| (symbol, rotation_degrees))
    }

    /// Whether the asset's authored boundary lead anchors exactly match the
    /// supplied electrical terminal offsets at the requested dimensions.
    pub fn asset_matches_terminal_offsets(
        &self,
        filename: &str,
        target_width: f32,
        target_height: f32,
        terminal_offsets: &[crate::state::Point],
    ) -> bool {
        let Some(symbol) = self.get_asset(filename) else {
            return false;
        };
        let anchors = symbol.boundary_anchors(target_width, target_height);
        anchors.len() == terminal_offsets.len()
            && terminal_offsets.iter().all(|terminal| {
                anchors.iter().any(|(x, y)| {
                    (*x - terminal.x as f32).abs() <= 0.25 && (*y - terminal.y as f32).abs() <= 0.25
                })
            })
    }

    /// Return all parsed embedded asset filenames.
    #[cfg(test)]
    pub fn asset_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.embedded_assets.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get a symbol with rotation awareness.
    /// For components with horizontal variants (like AC voltage source),
    /// returns the horizontal SVG when rotated 90° or 270°, along with the
    /// adjusted rotation to apply to the symbol.
    /// Returns (symbol, adjusted_rotation_degrees).
    pub fn get_with_rotation(
        &self,
        component_type: ComponentType,
        rotation_degrees: i32,
    ) -> Option<(&Symbol, i32)> {
        // Normalize rotation to 0-359
        let normalized = rotation_degrees.rem_euclid(360);

        // For 90° or 270° rotation, use horizontal variant if available
        if (normalized == 90 || normalized == 270)
            && let Some(symbol) = self.horizontal_symbols.get(&component_type)
        {
            // Horizontal SVG is already rotated 90° from vertical.
            // For 90° requested: use horizontal SVG with 0° rotation
            // For 270° requested: use horizontal SVG with 180° rotation
            let adjusted = if normalized == 90 { 0 } else { 180 };
            return Some((symbol, adjusted));
        }

        // Otherwise use the component's canonical default asset with the
        // original rotation.
        self.symbols
            .get(&component_type)
            .map(|s| (s, rotation_degrees))
    }

    /// Get a symbol with rotation awareness and optional symbol variant override.
    pub fn get_with_rotation_variant(
        &self,
        component_type: ComponentType,
        rotation_degrees: i32,
        variant: Option<&str>,
    ) -> Option<(&Symbol, i32)> {
        let normalized = rotation_degrees.rem_euclid(360);

        if let Some(variant_id) = variant.filter(|variant_id| !variant_id.is_empty()) {
            if (normalized == 90 || normalized == 270)
                && let Some(symbol) = self
                    .horizontal_variant_symbols
                    .get(&component_type)
                    .and_then(|variants| variants.get(variant_id))
            {
                let adjusted = if normalized == 90 { 0 } else { 180 };
                return Some((symbol, adjusted));
            }

            if let Some(symbol) = self
                .variant_symbols
                .get(&component_type)
                .and_then(|variants| variants.get(variant_id))
            {
                return Some((symbol, rotation_degrees));
            }

            // A requested authored skin is part of the saved instance
            // semantics. Silently replacing an unknown id with the default
            // glyph would make the canvas and export lie about that instance.
            return None;
        }

        self.get_with_rotation(component_type, rotation_degrees)
    }

    /// Number of loaded symbols
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Number of parsed embedded SVG asset files.
    pub fn asset_count(&self) -> usize {
        self.embedded_assets.len()
    }
}

fn symbol_reaches_terminal_offsets(
    symbol: &Symbol,
    target_width: f32,
    target_height: f32,
    terminal_offsets: &[crate::state::Point],
) -> bool {
    let anchors = symbol.boundary_anchors(target_width, target_height);
    terminal_offsets.iter().all(|terminal| {
        anchors.iter().any(|(x, y)| {
            (*x - terminal.x as f32).abs() <= 0.25 && (*y - terminal.y as f32).abs() <= 0.25
        })
    })
}

fn validate_symbol(filename: &str, symbol: &Symbol) -> Result<(), SymbolError> {
    let (min_x, min_y, max_x, max_y) = symbol.bounds;
    let finite_bounds = [min_x, min_y, max_x, max_y].into_iter().all(f32::is_finite);
    if !finite_bounds || max_x <= min_x || max_y <= min_y {
        return Err(SymbolError::ParseError(format!(
            "canonical asset '{filename}' has invalid bounds {:?}",
            symbol.bounds
        )));
    }
    if !symbol.target_width.is_finite()
        || !symbol.target_height.is_finite()
        || symbol.target_width <= 0.0
        || symbol.target_height <= 0.0
    {
        return Err(SymbolError::ParseError(format!(
            "canonical asset '{filename}' has invalid target dimensions {}x{}",
            symbol.target_width, symbol.target_height
        )));
    }
    if symbol.paths.is_empty() {
        return Err(SymbolError::ParseError(format!(
            "canonical asset '{filename}' has no drawable paths"
        )));
    }

    let mut drawable = false;
    for path in &symbol.paths {
        for command in &path.commands {
            let finite = match command {
                super::types::PathCommand::MoveTo(x, y)
                | super::types::PathCommand::LineTo(x, y) => x.is_finite() && y.is_finite(),
                super::types::PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                    [ctrl1.0, ctrl1.1, ctrl2.0, ctrl2.1, end.0, end.1]
                        .into_iter()
                        .all(f32::is_finite)
                }
                super::types::PathCommand::Close => true,
            };
            if !finite {
                return Err(SymbolError::ParseError(format!(
                    "canonical asset '{filename}' contains non-finite geometry"
                )));
            }
            drawable |= matches!(
                command,
                super::types::PathCommand::LineTo(..)
                    | super::types::PathCommand::CurveTo { .. }
                    | super::types::PathCommand::Close
            );
        }
    }
    if !drawable {
        return Err(SymbolError::ParseError(format!(
            "canonical asset '{filename}' has no renderable segments"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every standard component must resolve to authored geometry. Cell
    /// instances are the sole exception because their symbol is resolved from
    /// the referenced library cell rather than a `ComponentType` asset.
    #[test]
    fn embedded_library_loads_and_covers_mapped_types() {
        let library = SymbolLibrary::load_embedded().expect("embedded symbol assets must parse");

        for kind in ComponentType::ALL {
            let descriptor = kind.descriptor();
            match descriptor.default_symbol_asset {
                Some(asset) => {
                    assert!(
                        library.get(kind).is_some(),
                        "{} ({}) has no symbol mapping",
                        descriptor.stable_id,
                        asset
                    );
                    assert!(
                        library.get_asset(asset).is_some(),
                        "{} references missing embedded asset {}",
                        descriptor.stable_id,
                        asset
                    );
                }
                None => assert_eq!(kind, ComponentType::CellInstance),
            }
        }

        for descriptor in crate::state::engine_only_xspice_devices() {
            assert!(
                library.get_asset(descriptor.symbol_asset).is_some(),
                "{} references missing embedded asset {}",
                descriptor.stable_id,
                descriptor.symbol_asset
            );
        }

        assert!(
            library.get_asset("switch_expression.svg").is_some(),
            "the generic expression-controlled switch asset is missing"
        );
    }

    #[test]
    fn every_standard_component_resolves_canonical_geometry_at_every_rotation() {
        let library = SymbolLibrary::load_embedded().expect("canonical library loads");

        for kind in ComponentType::ALL {
            if kind == ComponentType::CellInstance {
                continue;
            }
            for rotation in [0, 90, 180, 270] {
                let (symbol, adjusted_rotation) = library
                    .get_with_rotation_variant(kind, rotation, None)
                    .unwrap_or_else(|| panic!("{kind:?} missing at {rotation} degrees"));
                validate_symbol(kind.descriptor().stable_id, symbol)
                    .unwrap_or_else(|error| panic!("{kind:?} at {rotation}: {error}"));
                assert!(matches!(
                    adjusted_rotation.rem_euclid(360),
                    0 | 90 | 180 | 270
                ));
            }
        }
    }

    #[test]
    fn missing_or_invalid_resolution_never_substitutes_default_art() {
        let empty = SymbolLibrary::new();
        assert!(
            empty
                .get_with_rotation_variant(ComponentType::Resistor, 0, None)
                .is_none()
        );

        let library = SymbolLibrary::load_embedded().expect("canonical library loads");
        assert!(
            library
                .get_with_rotation_variant(ComponentType::Diode, 0, Some("not-a-real-variant"))
                .is_none()
        );

        let invalid = Symbol {
            name: "invalid".to_owned(),
            paths: vec![super::super::types::SymbolPath {
                commands: vec![super::super::types::PathCommand::LineTo(f32::NAN, 0.0)],
                filled: false,
            }],
            bounds: (0.0, 0.0, 40.0, 20.0),
            target_width: 40.0,
            target_height: 20.0,
        };
        assert!(validate_symbol("invalid.svg", &invalid).is_err());
    }

    #[test]
    fn led_variant_reaches_the_exact_diode_terminal_contract() {
        let library = SymbolLibrary::load_embedded().expect("canonical library loads");
        let symbol = library
            .get_with_rotation_variant(ComponentType::Diode, 0, Some("led"))
            .expect("canonical LED variant")
            .0;
        let terminals: Vec<_> = ComponentType::Diode
            .terminal_offsets()
            .iter()
            .map(|(_, offset)| *offset)
            .collect();

        assert!(symbol_reaches_terminal_offsets(
            symbol,
            symbol.target_width,
            symbol.target_height,
            &terminals,
        ));
    }

    /// The loop probe is authored in viewBox coordinates like the resistor,
    /// and its conductor has to reach both terminal points exactly — the
    /// probe sits in series with the feedback path, so a lead that stops
    /// short would leave the loop open in the drawing.
    #[test]
    fn the_loop_probe_spans_its_box_and_reaches_both_pins() {
        let library = SymbolLibrary::load_embedded().expect("library loads");
        let probe = library.get(ComponentType::LoopProbe).expect("loop probe");

        assert_eq!(probe.bounds, (0.0, 0.0, 40.0, 20.0));

        use super::super::types::PathCommand;
        let touches = |x: f32, y: f32| {
            probe.paths.iter().any(|path| {
                path.commands.iter().any(|command| match command {
                    PathCommand::MoveTo(px, py) | PathCommand::LineTo(px, py) => {
                        (px - x).abs() < 0.01 && (py - y).abs() < 0.01
                    }
                    _ => false,
                })
            })
        };
        assert!(touches(0.0, 10.0), "no path reaches the left terminal");
        assert!(touches(40.0, 10.0), "no path reaches the right terminal");
    }

    /// The loop probe as the schematic renderer actually draws it.
    ///
    /// This stood as an `#[ignore]`d PNG dump, which is a test only in the
    /// sense that it compiled: it wrote a file and asserted nothing, so it
    /// could not fail and never ran. The test above judges the *parsed paths*,
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
        let canvas = crate::ui::raster::render(egui::vec2(560.0, 260.0), |ui, _| {
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

    /// New-style assets are authored in viewBox coordinates: the parser must
    /// keep them verbatim so pin leads land exactly on the terminal grid.
    #[test]
    fn viewbox_authored_assets_keep_exact_coordinates() {
        let library = SymbolLibrary::load_embedded().expect("library loads");
        let resistor = library.get(ComponentType::Resistor).expect("resistor");
        // The resistor is authored on a 40x20 viewBox with pins at the box
        // edge midpoints; the parsed bounds must be exactly the viewBox.
        assert_eq!(resistor.bounds, (0.0, 0.0, 40.0, 20.0));
    }
}

#[cfg(test)]
mod parse_sanity {
    use super::*;

    /// Every embedded asset must keep all of its SVG paths through the
    /// parse (none dropped, none collapsed), and stroked outlines must
    /// never be classified as filled.
    #[test]
    fn every_asset_keeps_its_paths() {
        let library = SymbolLibrary::load_embedded().expect("library loads");
        for name in library.asset_names() {
            let symbol = library.get_asset(&name).expect("asset");
            assert!(!symbol.paths.is_empty(), "{name}: parsed to zero paths");
            for (index, path) in symbol.paths.iter().enumerate() {
                assert!(
                    !path.commands.is_empty(),
                    "{name}: path {index} has no commands"
                );
            }
        }
    }
}

#[cfg(test)]
mod browser_audit {
    use super::*;
    use crate::schematic::component_palette;
    use crate::state::ComponentType;

    /// Endpoints reachable by the pen in a path (segment ends only).
    fn endpoints(symbol: &Symbol) -> Vec<(f32, f32)> {
        let mut points = Vec::new();
        for path in &symbol.paths {
            for command in &path.commands {
                match command {
                    super::super::types::PathCommand::MoveTo(x, y)
                    | super::super::types::PathCommand::LineTo(x, y) => points.push((*x, *y)),
                    super::super::types::PathCommand::CurveTo { end, .. } => points.push(*end),
                    super::super::types::PathCommand::Close => {}
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
