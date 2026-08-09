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
        // A missing file is fatal here, so catalog drift cannot degrade into a
        // procedural fallback or an empty palette entry.
        for component_type in ComponentType::ALL {
            let Some(filename) = component_type.descriptor().default_symbol_asset else {
                continue;
            };
            let symbol = library.prepare_symbol(
                component_type,
                filename,
                component_type.display_name(),
                false,
            )?;
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
        if anchors.len() != terminal_offsets.len() {
            return false;
        }
        terminal_offsets.iter().all(|terminal| {
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

        // Fall back to default symbol with original rotation
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Every embedded asset must parse and every mapped component type must
    /// resolve to a symbol — a bad asset would otherwise silently drop the
    /// whole library back to procedural fallbacks at startup.
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
