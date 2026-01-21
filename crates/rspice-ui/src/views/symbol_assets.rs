//! Symbol Assets Management
//!
//! Manages loading, caching, and parsing of SVG assets for schematic components.
//! Assets are baked into the binary using `include_str!` for robust, zero-latency access.

use crate::state::ComponentType;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Represents a parsed SVG asset
#[derive(Debug, Clone)]
pub struct SvgAsset {
    /// The inner SVG content (paths, groups, etc.) without the <svg> wrapper
    pub content: String,
    /// The viewBox xmin, ymin, width, height
    pub view_box: (f64, f64, f64, f64),
    /// Manual scale correction factor (default 1.0)
    pub scale: f64,
    /// Manual stroke width scaling factor (default 1.0)
    pub stroke_scale: f64,
    /// Manual X horizontal offset (default 0.0) - in SVG units
    pub x_offset: f64,
    /// Manual Y vertical offset (default 0.0) - in SVG units
    pub y_offset: f64,
    /// Terminal X offset in grid units (default 0) - adjusts logical connection points
    pub terminal_x_offset: i32,
    /// Terminal Y offset in grid units (default 0) - adjusts logical connection points
    pub terminal_y_offset: i32,
}

/// Static storage for parsed assets
static ASSETS: Lazy<HashMap<ComponentType, SvgAsset>> = Lazy::new(load_assets);

/// Get the SVG asset for a specific component type, if available
pub fn get_component_svg(kind: ComponentType) -> Option<&'static SvgAsset> {
    ASSETS.get(&kind)
}

/// Get terminal position adjustments in grid units for a component type
/// Returns (x_offset, y_offset) to be added to the base terminal positions
pub fn get_terminal_offsets(kind: ComponentType) -> (i32, i32) {
    ASSETS
        .get(&kind)
        .map(|asset| (asset.terminal_x_offset, asset.terminal_y_offset))
        .unwrap_or((0, 0))
}

/// Get the symbol bounding box dimensions (width, height) for rotation=0.
/// These dimensions match the NATIVE SVG orientation:
/// - Resistor SVG is horizontal (wide×short), displayed horizontal at rot=0
/// - When rotated 90°/270°, the schematic code will swap width/height
pub fn get_symbol_bounds(kind: ComponentType) -> (f64, f64) {
    // Bounds for rotation=0 (native SVG orientation)
    // Format: (width, height) in pixels at 1x zoom
    match kind {
        // Passives - native SVG orientations
        ComponentType::Resistor => (32.0, 12.0), // Native horizontal (wide×short)
        ComponentType::Capacitor => (24.0, 18.0), // Native horizontal (plates side-by-side)
        ComponentType::Inductor => (36.0, 14.0), // Native horizontal (coils inline)

        // Semiconductors - typically vertical native orientation
        ComponentType::Diode => (28.0, 14.0), // Native horizontal
        ComponentType::NpnBjt | ComponentType::PnpBjt => (28.0, 38.0),
        ComponentType::Nmos | ComponentType::Pmos => (30.0, 40.0),
        ComponentType::Njfet | ComponentType::Pjfet => (28.0, 38.0),

        // Sources - circular, relatively symmetric
        ComponentType::VoltageSource => (28.0, 44.0),
        ComponentType::CurrentSource => (28.0, 44.0),
        ComponentType::Vcvs | ComponentType::Vccs | ComponentType::Ccvs | ComponentType::Cccs => {
            (32.0, 40.0)
        }

        // Other
        ComponentType::Ground => (22.0, 14.0), // Wide, short

        // Fallback for any other types
        _ => (32.0, 32.0),
    }
}

/// Load and parse all assets
fn load_assets() -> HashMap<ComponentType, SvgAsset> {
    let mut m = HashMap::new();

    // Helper to load and parse an asset with optional scale correction
    let mut load = |kind: ComponentType,
                    raw: &str,
                    scale: f64,
                    stroke_scale: f64,
                    x_offset: f64,
                    y_offset: f64,
                    terminal_x_offset: i32,
                    terminal_y_offset: i32| {
        if let Some(mut asset) = parse_svg(raw) {
            asset.scale = scale;
            asset.stroke_scale = stroke_scale;
            asset.x_offset = x_offset;
            asset.y_offset = y_offset;
            asset.terminal_x_offset = terminal_x_offset;
            asset.terminal_y_offset = terminal_y_offset;
            m.insert(kind, asset);
        } else {
            log::warn!("Failed to parse SVG asset for {:?}", kind);
        }
    };

    // --- BJT ---
    // Shift BJTs left significantly
    load(
        ComponentType::NpnBjt,
        include_str!("../../assets/component_symbols/bjt_npn.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0, // terminal offsets
    );
    load(
        ComponentType::PnpBjt,
        include_str!("../../assets/component_symbols/bjt_pnp.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );

    // --- MOSFET ---
    load(
        ComponentType::Nmos,
        include_str!("../../assets/component_symbols/mos_n_chan_enh_discrete.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );
    load(
        ComponentType::Pmos,
        include_str!("../../assets/component_symbols/mos_p_chan_enh_discrete.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );

    // --- JFET ---
    load(
        ComponentType::Njfet,
        include_str!("../../assets/component_symbols/jfet_n_chan.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );
    load(
        ComponentType::Pjfet,
        include_str!("../../assets/component_symbols/jfet_p_chan.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );

    // --- Sources ---
    load(
        ComponentType::VoltageSource,
        include_str!("../../assets/component_symbols/v_src_dc.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );
    load(
        ComponentType::CurrentSource,
        include_str!("../../assets/component_symbols/i_src.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );

    // Dependent Sources
    // Note: Dependent source assets are currently missing from the new set.
    // Fallback to code rendering.
    /*
    let dep_v = include_str!("../../assets/component_symbols/dependent_dc_v_src.svg");
    load(ComponentType::Vcvs, dep_v, 0.0);
    load(ComponentType::Ccvs, dep_v, 0.0);

    let dep_i = include_str!("../../assets/component_symbols/dependent_dc_i_src.svg");
    load(ComponentType::Vccs, dep_i, 0.0);
    load(ComponentType::Cccs, dep_i, 0.0);
    */

    // --- Passives ---
    load(
        ComponentType::Resistor,
        include_str!("../../assets/component_symbols/resistor.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );
    load(
        ComponentType::Inductor,
        include_str!("../../assets/component_symbols/inductor.svg"),
        1.0, // Scale (default, SVG is now properly sized)
        1.0, // Stroke scale (thicker lines)
        0.0, // X offset (not needed, SVG is centered)
        0.0, // Y offset
        0,   // Terminal offset X
        0,   // Terminal offset Y
    );
    load(
        ComponentType::Capacitor,
        include_str!("../../assets/component_symbols/cap_unpolarized.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );
    load(
        ComponentType::Diode,
        include_str!("../../assets/component_symbols/diode.svg"),
        1.0,
        1.0,
        0.0,
        0.0,
        0,
        0,
    );
    /*
    load(
        ComponentType::Ground,
        include_str!("../../assets/component_symbols/ground.svg"),
        0.0,
    );
    */

    // Note: Some assets (Ground, Dependent Sources) are missing and will fallback.

    // Note: OpAmps are present in assets but not in ComponentType enum currently.
    // If ComponentType is updated in the future, add them here.

    m
}

/// Simple SVG parser to extract viewBox and inner content
///
/// robustly handles:
/// - viewBox attribute (with or without quotes, commas or spaces)
/// - stripping the outer <svg> tags
/// - xml declaration
fn parse_svg(raw: &str) -> Option<SvgAsset> {
    // 1. Extract viewBox
    // Replace hardcoded black colors with currentColor for theming
    let processed = raw
        .replace("#000000", "currentColor")
        .replace("stroke:black", "stroke:currentColor")
        .replace("fill:black", "fill:currentColor")
        .replace("stroke-width:", "data-sw:"); // Disable internal stroke-width to allow inheritance

    let view_box = parse_viewbox(&processed)?;

    // 2. Extract inner content
    // Find the first > that closes the <svg> tag
    let start_idx = processed.find("<svg")?;
    let content_start = processed[start_idx..].find('>')? + start_idx + 1;

    // Find the last </svg>
    let content_end = processed.rfind("</svg")?;

    if content_start >= content_end {
        return None;
    }

    let inner_content = processed[content_start..content_end].trim().to_string();

    Some(SvgAsset {
        content: inner_content,
        view_box,
        scale: 1.0,
        stroke_scale: 1.0,
        x_offset: 0.0,
        y_offset: 0.0,
        terminal_x_offset: 0,
        terminal_y_offset: 0,
    })
}

fn parse_viewbox(raw: &str) -> Option<(f64, f64, f64, f64)> {
    // Simple parser for viewBox="min-x min-y width height"
    // Case insensitive find
    let lower = raw.to_lowercase();
    let key = "viewbox=\"";
    let start = lower.find(key)?;
    let remainder = &lower[start + key.len()..];
    let end = remainder.find('"')?;
    let value = &remainder[..end];

    // Split by comma or whitespace
    let parts: Vec<&str> = value
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();

    if parts.len() != 4 {
        return None;
    }

    let min_x = parts[0].parse().ok()?;
    let min_y = parts[1].parse().ok()?;
    let width = parts[2].parse().ok()?;
    let height = parts[3].parse().ok()?;

    Some((min_x, min_y, width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_svg_simple() {
        let raw =
            r#"<svg width="20" height="20" viewBox="0 0 100 100"><path d="M0 0 L10 10"/></svg>"#;
        let asset = parse_svg(raw).expect("Should parse simple SVG");

        assert_eq!(asset.view_box, (0.0, 0.0, 100.0, 100.0));
        assert!(asset.content.contains("<path d=\"M0 0 L10 10\"/>"));
    }

    #[test]
    fn test_parse_svg_with_header() {
        let raw = r#"<?xml version="1.0" encoding="UTF-8"?><svg viewBox="-10 -10 20 20"><circle r="5"/></svg>"#;
        let asset = parse_svg(raw).expect("Should parse SVG with header");

        assert_eq!(asset.view_box, (-10.0, -10.0, 20.0, 20.0));
        assert!(asset.content.contains("<circle r=\"5\"/>"));
    }

    #[test]
    fn test_load_npn_bjt() {
        // This test verifies that the actual asset file exists and is parseable
        let asset = get_component_svg(ComponentType::NpnBjt);
        assert!(asset.is_some(), "NpnBjt asset should be loaded");

        if let Some(a) = asset {
            // Check that values are reasonable
            assert!(a.view_box.2 > 0.0); // width > 0
            assert!(a.view_box.3 > 0.0); // height > 0
            assert!(!a.content.is_empty());
        }
    }

    #[test]
    fn test_fallback_for_missing_asset() {
        // Ground is known to be missing from assets, so it should return None
        // prompting the simple SVG fallback path.
        let asset = get_component_svg(ComponentType::Ground);
        assert!(
            asset.is_none(),
            "Ground should not have an asset loaded (fallback expected)"
        );
    }
}
