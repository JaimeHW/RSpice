use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::state::ComponentType;

use super::error::SymbolError;
use super::parser::parse_svg;
use super::pins::add_default_pins;
use super::render::{BakedSymbol, bake_symbol};
use super::types::Symbol;

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
    baked: RefCell<HashMap<(usize, i32, bool, bool), Rc<BakedSymbol>>>,
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
            rotation_degrees.rem_euclid(360),
            mirror_h,
            mirror_v,
        );
        if let Some(hit) = self.baked.borrow().get(&key) {
            return Rc::clone(hit);
        }
        let baked = Rc::new(bake_symbol(symbol, rotation_degrees, mirror_h, mirror_v));
        self.baked.borrow_mut().insert(key, Rc::clone(&baked));
        baked
    }

    /// Load all embedded SVG symbols from the assets directory.
    /// Returns the library with all symbols loaded, or an error if any fail.
    pub fn load_embedded() -> Result<Self, SymbolError> {
        let mut library = Self::new();

        library.embedded_assets = Self::load_all_embedded_assets()?;

        // Map ComponentType to default SVG filename.
        let default_mappings: &[(ComponentType, &str, &str)] = &[
            (ComponentType::Resistor, "resistor.svg", "Resistor"),
            (ComponentType::Capacitor, "cap_unpolarized.svg", "Capacitor"),
            (ComponentType::Inductor, "inductor.svg", "Inductor"),
            (
                ComponentType::Transformer,
                "transformer_symmetrical.svg",
                "Transformer",
            ),
            (
                ComponentType::SaturableInductor,
                "inductor.svg",
                "Saturable Inductor",
            ),
            (
                ComponentType::VoltageSource,
                "v_src_dc.svg",
                "Voltage Source",
            ),
            (
                ComponentType::VoltageSourceAc,
                "v_src_ac_vertical.svg",
                "AC Voltage Source",
            ),
            (
                ComponentType::VoltageSourceSin,
                "v_src_ac_vertical.svg",
                "Sinusoidal Voltage Source",
            ),
            (
                ComponentType::VoltageSourcePulse,
                "v_src_dc.svg",
                "Pulse Voltage Source",
            ),
            (
                ComponentType::VoltageSourcePwl,
                "v_src_dc.svg",
                "PWL Voltage Source",
            ),
            (
                ComponentType::VoltageSourceExp,
                "v_src_dc.svg",
                "Exponential Voltage Source",
            ),
            (
                ComponentType::VoltageSourceSffm,
                "v_src_dc.svg",
                "SFFM Voltage Source",
            ),
            (ComponentType::CurrentSource, "i_src.svg", "Current Source"),
            (
                ComponentType::CurrentSourceAc,
                "i_src.svg",
                "AC Current Source",
            ),
            (
                ComponentType::CurrentSourcePulse,
                "i_src.svg",
                "Pulse Current Source",
            ),
            (
                ComponentType::CurrentSourceSin,
                "i_src.svg",
                "Sinusoidal Current Source",
            ),
            (
                ComponentType::CurrentSourcePwl,
                "i_src.svg",
                "PWL Current Source",
            ),
            (
                ComponentType::CurrentSourceExp,
                "i_src.svg",
                "Exponential Current Source",
            ),
            (
                ComponentType::CurrentSourceNoise,
                "i_src.svg",
                "Noise Current Source",
            ),
            (ComponentType::Ground, "ground_signal.svg", "Ground"),
            (ComponentType::Diode, "diode.svg", "Diode"),
            (
                ComponentType::Nmos,
                "mos_n_chan_enh_no_substrate.svg",
                "NMOS",
            ),
            (
                ComponentType::Pmos,
                "mos_p_chan_enh_no_substrate.svg",
                "PMOS",
            ),
            (ComponentType::Njfet, "jfet_n_chan.svg", "N-JFET"),
            (ComponentType::Pjfet, "jfet_p_chan.svg", "P-JFET"),
            (
                ComponentType::NVdmos,
                "mos_n_chan_enh_body_diode_discrete.svg",
                "N-VDMOS",
            ),
            (
                ComponentType::PVdmos,
                "mos_p_chan_enh_body_diode_discrete.svg",
                "P-VDMOS",
            ),
            (ComponentType::NpnBjt, "bjt_npn.svg", "NPN BJT"),
            (ComponentType::PnpBjt, "bjt_pnp.svg", "PNP BJT"),
        ];

        for (component_type, filename, name) in default_mappings {
            let symbol = library.prepare_symbol(*component_type, filename, name, false)?;
            library.symbols.insert(*component_type, symbol);
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

        add_default_pins(&mut symbol, component_type);
        Ok(symbol)
    }

    /// Load an embedded SVG file by name

    /// Get a symbol by component type (O(1) lookup)
    pub fn get(&self, component_type: ComponentType) -> Option<&Symbol> {
        self.symbols.get(&component_type)
    }

    /// Get a parsed embedded asset by filename.
    pub fn get_asset(&self, filename: &str) -> Option<&Symbol> {
        self.embedded_assets.get(filename)
    }

    /// Return all parsed embedded asset filenames.
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

    /// Check if a symbol exists for the given component type
    pub fn contains(&self, component_type: ComponentType) -> bool {
        self.symbols.contains_key(&component_type)
    }

    /// Check whether a parsed embedded asset exists by filename.
    pub fn contains_asset(&self, filename: &str) -> bool {
        self.embedded_assets.contains_key(filename)
    }

    /// Get all loaded component types
    pub fn loaded_types(&self) -> Vec<ComponentType> {
        self.symbols.keys().copied().collect()
    }

    /// Number of loaded symbols
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Number of parsed embedded SVG asset files.
    pub fn asset_count(&self) -> usize {
        self.embedded_assets.len()
    }

    /// Check if library is empty
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}
