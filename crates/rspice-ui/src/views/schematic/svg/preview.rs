//! Component Preview SVG Rendering
//!
//! Renders a ghost preview of a component during placement mode,
//! showing where the component will be placed when clicked.

use dioxus::prelude::*;
use crate::state::{ComponentType, Point, Rotation};
use crate::theme::Theme;
use crate::views::symbol_assets;

/// Preview SVG component - ghost preview for component placement
///
/// Shows a semi-transparent preview of the component at the cursor position
/// with a dashed circle indicator and the component symbol.
///
/// # Props
/// - `kind` - Type of component to preview
/// - `pos` - Current cursor grid position
/// - `grid_size` - Grid size in pixels
/// - `rotation` - Current preview rotation
#[component]
pub fn PreviewSvg(kind: ComponentType, pos: Point, grid_size: i32, rotation: Rotation) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let (cx, cy) = pos.to_pixels(grid_size);
    let rot_deg = rotation.degrees();
    
    let asset = symbol_assets::get_component_svg(kind);
    
    rsx! {
        g { transform: "translate({cx},{cy}) rotate({rot_deg})", opacity: "0.6",
            // Preview indicator circle with dashed border
            circle { cx: "0", cy: "0", r: "20", fill: "{th.accent_primary()}30", stroke: "{th.accent_primary()}", stroke_dasharray: "4,2" }
            
            if let Some(svg) = asset {
                 {
                   let (vx, vy, vw, vh) = svg.view_box;
                   // Standard component size is 4 grid blocks (e.g. 40px for 10px grid)
                   let target_size = (grid_size as f64) * 4.0;
                   // Apply manual scale correction (same as CompSvg)
                   let scale = (target_size / vw.max(vh)) * svg.scale;
                   // Incorporate offsets into center calculation (same as CompSvg)
                   let center_x = vx + vw / 2.0 - svg.x_offset;
                   let center_y = vy + vh / 2.0 - svg.y_offset;
                   // Stroke width correction (same as CompSvg)
                   let base_scale = scale / svg.scale;

                   rsx! {
                       g {
                           transform: "scale({scale}) translate({-center_x}, {-center_y})",
                           dangerous_inner_html: "{svg.content}",
                           stroke: "{th.accent_primary()}",
                           fill: "{th.accent_primary()}",
                           stroke_width: "{1.5 * svg.stroke_scale / base_scale}",
                       }
                   }
                }
            } else {
                // Fallback to hardcoded path
                path { d: "{symbol_path(kind)}", stroke: "{th.accent_primary()}", stroke_width: "2", fill: "none" }
            }
        }
    }
}

/// Returns the SVG path data for a component symbol (fallback rendering)
///
/// This is used when no SVG asset is available for a component type.
/// Each component has a carefully designed path that maintains
/// proper proportions and connection points.
pub fn symbol_path(k: ComponentType) -> &'static str {
    match k {
        ComponentType::Resistor => "M-20 0 L-15 0 L-12-8 L-6 8 L0-8 L6 8 L12-8 L15 0 L20 0",
        ComponentType::Capacitor => "M-20 0 L-4 0 M-4-12 L-4 12 M4-12 L4 12 M4 0 L20 0",
        ComponentType::Inductor => "M-20 0 C-15 0-15-10-10-10 C-5-10-5 0 0 0 C5 0 5-10 10-10 C15-10 15 0 20 0",
        ComponentType::CoupledInductor => "M-15-10 C-10-10-10-20-5-20 C0-20 0-10 5-10 C10-10 10-20 15-20 M-15 10 C-10 10-10 0-5 0 C0 0 0 10 5 10 C10 10 10 0 15 0 M0-6 L0 6",
        ComponentType::Diode => "M-20 0 L-8 0 M-8-10 L-8 10 L8 0 Z M8-10 L8 10 M8 0 L20 0",
        ComponentType::Ground => "M0-20 L0 0 M-12 0 L12 0 M-8 5 L8 5 M-4 10 L4 10",
        ComponentType::VoltageSource | ComponentType::VoltageSourceAc | ComponentType::VoltageSourcePulse | ComponentType::VoltageSourceSin 
        | ComponentType::VoltageSourcePwl | ComponentType::VoltageSourceExp | ComponentType::VoltageSourceSffm => 
            "M0-20 L0-12 M0 12 L0 20 M0 0 m-12 0 a12 12 0 1 0 24 0 a12 12 0 1 0-24 0 M-4-4 L4-4 M0-8 L0 0",
        ComponentType::CurrentSource | ComponentType::CurrentSourceAc | ComponentType::CurrentSourcePulse | ComponentType::CurrentSourceSin
        | ComponentType::CurrentSourcePwl | ComponentType::CurrentSourceExp | ComponentType::CurrentSourceNoise =>
            "M0-20 L0-12 M0 12 L0 20 M0 0 m-12 0 a12 12 0 1 0 24 0 a12 12 0 1 0-24 0 M0-6 L0 6 M-3 3 L0 6 L3 3",
        ComponentType::NpnBjt => "M-20 0 L-12 0 M0 0 m-12 0 a12 12 0 1 0 24 0 a12 12 0 1 0-24 0 M-4-8 L-4 8 M-4-4 L8-10 L10-20 M-4 4 L8 10 L10 20 M4 7 L8 10 L5 11",
        ComponentType::PnpBjt => "M-20 0 L-12 0 M0 0 m-12 0 a12 12 0 1 0 24 0 a12 12 0 1 0-24 0 M-4-8 L-4 8 M-4-4 L8-10 L10-20 M-4 4 L8 10 L10 20 M-1 6 L-4 4 L-1 2",
        ComponentType::Nmos => "M-20 0 L-8 0 M-8-12 L-8 12 M-4-10 L-4-3 M-4 3 L-4 10 M-4-7 L10-7 L10-20 M-4 7 L10 7 L10 20 M-4 0 L10 0 M6-3 L10 0 L6 3",
        ComponentType::Pmos => "M-20 0 L-12 0 M-8 0 m-3 0 a3 3 0 1 0 6 0 a3 3 0 1 0-6 0 M-5-12 L-5 12 M-1-10 L-1-3 M-1 3 L-1 10 M-1-7 L10-7 L10-20 M-1 7 L10 7 L10 20 M-1 0 L10 0",
        ComponentType::Njfet => "M-20 0 L-6 0 M-6-10 L-6 10 M-6-5 L10-5 L10-20 M-6 5 L10 5 L10 20 M4 2 L10 0 L4-2",
        ComponentType::Pjfet => "M-20 0 L-6 0 M-6-10 L-6 10 M-6-5 L10-5 L10-20 M-6 5 L10 5 L10 20 M-2 2 L-6 0 L-2-2",
        // Power electronics - VDMOS uses same symbols as MOSFET
        ComponentType::NVdmos => "M-20 0 L-8 0 M-8-12 L-8 12 M-4-10 L-4-3 M-4 3 L-4 10 M-4-7 L10-7 L10-20 M-4 7 L10 7 L10 20 M-4 0 L10 0 M6-3 L10 0 L6 3",
        ComponentType::PVdmos => "M-20 0 L-12 0 M-8 0 m-3 0 a3 3 0 1 0 6 0 a3 3 0 1 0-6 0 M-5-12 L-5 12 M-1-10 L-1-3 M-1 3 L-1 10 M-1-7 L10-7 L10-20 M-1 7 L10 7 L10 20 M-1 0 L10 0",
        // Saturable inductor - inductor with core lines
        ComponentType::SaturableInductor => "M-20 0 C-15 0-15-10-10-10 C-5-10-5 0 0 0 C5 0 5-10 10-10 C15-10 15 0 20 0 M-12-14 L12-14 M-12-16 L12-16",
        ComponentType::Vcvs | ComponentType::Vccs | ComponentType::Ccvs | ComponentType::Cccs => 
            "M0-15 L12 0 L0 15 L-12 0 Z M-20-10 L-12-5 M-20 10 L-12 5 M12-5 L20-10 M12 5 L20 10",
        ComponentType::XspiceGain | ComponentType::XspiceLimiter | ComponentType::XspiceIntegrator | ComponentType::XspiceDifferentiator =>
            "M-15-12 L15-12 L15 12 L-15 12 Z M-20 0 L-15 0 M15 0 L20 0",
        ComponentType::XspiceSummer =>
            "M-10-15 L15 0 L-10 15 Z M-20-10 L-10-5 M-20 10 L-10 5 M15 0 L20 0",
        ComponentType::XspiceMultiplier | ComponentType::XspiceDivider =>
            "M-12-12 L12-12 L12 12 L-12 12 Z M-20-10 L-12-6 M-20 10 L-12 6 M12 0 L20 0",
        ComponentType::XspiceInverter =>
            "M-10-12 L10 0 L-10 12 Z M12 0 m-3 0 a3 3 0 1 0 6 0 a3 3 0 1 0-6 0 M-20 0 L-10 0 M15 0 L20 0",
        ComponentType::XspiceBuffer =>
            "M-10-12 L10 0 L-10 12 Z M-20 0 L-10 0 M10 0 L20 0",
        ComponentType::XspiceAndGate =>
            "M-10-12 L-10 12 L2 12 A12 12 0 0 0 2-12 Z M-20-10 L-10-10 M-20 10 L-10 10 M14 0 L20 0",
        ComponentType::XspiceOrGate =>
            "M-12-12 Q-6 0-12 12 Q0 10 6 12 Q14 0 6-12 Q0-10-12-12 M-20-10 L-9-10 M-20 10 L-9 10 M14 0 L20 0",
        ComponentType::XspiceNandGate =>
            "M-10-12 L-10 12 L2 12 A12 12 0 0 0 2-12 Z M14 0 m-3 0 a3 3 0 1 0 6 0 a3 3 0 1 0-6 0 M-20-10 L-10-10 M-20 10 L-10 10 M17 0 L20 0",
        ComponentType::XspiceNorGate =>
            "M-12-12 Q-6 0-12 12 Q0 10 6 12 Q14 0 6-12 Q0-10-12-12 M14 0 m-3 0 a3 3 0 1 0 6 0 a3 3 0 1 0-6 0 M-20-10 L-9-10 M-20 10 L-9 10 M17 0 L20 0",
        ComponentType::XspiceXorGate =>
            "M-12-12 Q-6 0-12 12 Q0 10 6 12 Q14 0 6-12 Q0-10-12-12 M-15-12 Q-9 0-15 12 M-20-10 L-10-10 M-20 10 L-10 10 M14 0 L20 0",
        ComponentType::XspiceTristate =>
            "M-10-12 L10 0 L-10 12 Z M-20 0 L-10 0 M10 0 L20 0 M0-20 L0-6",
        ComponentType::XspiceDFlipFlop | ComponentType::XspiceSrLatch =>
            "M-15-15 L15-15 L15 15 L-15 15 Z M-20-10 L-15-10 M-20 10 L-15 10 M15-10 L20-10 M15 10 L20 10 M-15 7 L-12 10 L-15 13",
        ComponentType::XspiceJkFlipFlop =>
            "M-15-20 L15-20 L15 20 L-15 20 Z M-20-10 L-15-10 M-20 0 L-15 0 M-20 10 L-15 10 M15-10 L20-10 M15 10 L20 10 M-15-3 L-12 0 L-15 3",
        ComponentType::XspiceAdcBridge | ComponentType::XspiceDacBridge =>
            "M-15-12 L15-12 L15 12 L-15 12 Z M-20 0 L-15 0 M15 0 L20 0 M-6 0 L6 0 M3-3 L6 0 L3 3",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_component_types_have_paths() {
        // Verify all component types have valid symbol paths
        let component_types = [
            ComponentType::Resistor,
            ComponentType::Capacitor,
            ComponentType::Inductor,
            ComponentType::CoupledInductor,
            ComponentType::Diode,
            ComponentType::Ground,
            ComponentType::VoltageSource,
            ComponentType::VoltageSourceAc,
            ComponentType::VoltageSourcePulse,
            ComponentType::VoltageSourceSin,
            ComponentType::CurrentSource,
            ComponentType::NpnBjt,
            ComponentType::PnpBjt,
            ComponentType::Nmos,
            ComponentType::Pmos,
            ComponentType::Njfet,
            ComponentType::Pjfet,
            ComponentType::Vcvs,
            ComponentType::Vccs,
            ComponentType::Ccvs,
            ComponentType::Cccs,
        ];
        
        for ct in component_types {
            let path = symbol_path(ct);
            assert!(!path.is_empty(), "Component {:?} has empty path", ct);
            assert!(path.contains('M') || path.contains('m'), "Path for {:?} should start with M command", ct);
        }
    }

    #[test]
    fn test_resistor_path_structure() {
        let path = symbol_path(ComponentType::Resistor);
        
        // Resistor path should have zigzag pattern (multiple L commands for vertices)
        assert!(path.starts_with("M-20 0"));
        assert!(path.contains('L'));
        assert!(path.ends_with("L20 0"));
    }

    #[test]
    fn test_ground_symbol_path() {
        let path = symbol_path(ComponentType::Ground);
        
        // Ground has vertical line and horizontal bars
        assert!(path.contains("M0-20 L0 0")); // Vertical line
        assert!(path.contains("M-12 0 L12 0")); // Top bar
    }

    #[test]
    fn test_capacitor_path_structure() {
        let path = symbol_path(ComponentType::Capacitor);
        
        // Capacitor has two vertical bars and leads
        assert!(path.contains("-4-12"));
        assert!(path.contains("4-12"));
    }

    #[test]
    fn test_rotation_degrees() {
        assert_eq!(Rotation::R0.degrees(), 0);
        assert_eq!(Rotation::R90.degrees(), 90);
        assert_eq!(Rotation::R180.degrees(), 180);
        assert_eq!(Rotation::R270.degrees(), 270);
    }

    #[test]
    fn test_preview_position_conversion() {
        let pos = Point::new(5, 3);
        let grid_size = 10;
        
        let (cx, cy) = pos.to_pixels(grid_size);
        
        assert_eq!(cx, 50.0);
        assert_eq!(cy, 30.0);
    }

    #[test]
    fn test_controlled_sources_same_path() {
        // All controlled sources share the same diamond symbol
        let vcvs = symbol_path(ComponentType::Vcvs);
        let vccs = symbol_path(ComponentType::Vccs);
        let ccvs = symbol_path(ComponentType::Ccvs);
        let cccs = symbol_path(ComponentType::Cccs);
        
        assert_eq!(vcvs, vccs);
        assert_eq!(vccs, ccvs);
        assert_eq!(ccvs, cccs);
    }

    #[test]
    fn test_voltage_sources_same_path() {
        // All voltage sources share the same circle symbol
        let vs = symbol_path(ComponentType::VoltageSource);
        let vs_ac = symbol_path(ComponentType::VoltageSourceAc);
        let vs_pulse = symbol_path(ComponentType::VoltageSourcePulse);
        let vs_sin = symbol_path(ComponentType::VoltageSourceSin);
        
        assert_eq!(vs, vs_ac);
        assert_eq!(vs_ac, vs_pulse);
        assert_eq!(vs_pulse, vs_sin);
    }
}
