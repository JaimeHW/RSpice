//! Operating Point Annotation Module
//!
//! Commercial-grade OP value overlay for schematic display.
//! Matches Cadence Spectre's operating point annotation capabilities.
//!
//! # Architecture
//!
//! - `OpAnnotationConfig`: Configuration for which parameters to display
//! - `OpAnnotation`: Formatted parameter values for a single device
//! - `OpAnnotationRenderer`: Renders annotations on the schematic canvas
//!
//! # Device Parameters
//!
//! **MOSFETs**: Vgs, Vds, Vth, Id, gm, gds, region
//! **BJTs**: Vbe, Vce, Ic, β, gm
//! **Diodes**: Vd, Id
//! **Passives**: Power dissipation, current

use crate::simulation::results::{DcOpResult, DeviceOpPoint};
use egui::{Color32, FontId, Pos2, Rect, Ui};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// OP Annotation Configuration
// =============================================================================

/// Configuration for operating point annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpAnnotationConfig {
    /// Whether annotations are enabled.
    pub enabled: bool,
    /// Parameters to show for MOSFETs.
    pub mosfet_params: Vec<OpParameter>,
    /// Parameters to show for BJTs.
    pub bjt_params: Vec<OpParameter>,
    /// Parameters to show for diodes.
    pub diode_params: Vec<OpParameter>,
    /// Parameters to show for resistors.
    pub resistor_params: Vec<OpParameter>,
    /// Parameters to show for capacitors.
    pub capacitor_params: Vec<OpParameter>,
    /// Font size for annotations.
    pub font_size: f32,
    /// Text color.
    pub text_color: Color32,
    /// Background color (semi-transparent).
    pub bg_color: Color32,
    /// Position offset from component center.
    pub offset: (f32, f32),
    /// Maximum number of parameters to show per device.
    pub max_params: usize,
    /// Number of significant digits.
    pub precision: usize,
}

impl Default for OpAnnotationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mosfet_params: vec![
                OpParameter::Vgs,
                OpParameter::Vds,
                OpParameter::Id,
                OpParameter::Gm,
                OpParameter::Region,
            ],
            bjt_params: vec![
                OpParameter::Vbe,
                OpParameter::Vce,
                OpParameter::Ic,
                OpParameter::Beta,
            ],
            diode_params: vec![OpParameter::Vd, OpParameter::Id],
            resistor_params: vec![OpParameter::Current, OpParameter::Power],
            capacitor_params: vec![OpParameter::Voltage],
            font_size: 10.0,
            text_color: Color32::from_rgb(0, 100, 200),
            bg_color: Color32::from_rgba_unmultiplied(255, 255, 240, 200),
            offset: (15.0, -15.0),
            max_params: 5,
            precision: 3,
        }
    }
}

impl OpAnnotationConfig {
    /// Create a new configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable annotations.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable annotations.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Toggle annotations.
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Get parameters for a device type.
    pub fn params_for_device(&self, device_type: &str) -> &[OpParameter] {
        match device_type.to_uppercase().as_str() {
            "M" | "MOSFET" | "NMOS" | "PMOS" => &self.mosfet_params,
            "Q" | "BJT" | "NPN" | "PNP" => &self.bjt_params,
            "D" | "DIODE" => &self.diode_params,
            "R" | "RESISTOR" => &self.resistor_params,
            "C" | "CAPACITOR" => &self.capacitor_params,
            _ => &[],
        }
    }

    /// Create a compact MOSFET-only config.
    pub fn mosfet_compact() -> Self {
        Self {
            mosfet_params: vec![OpParameter::Id, OpParameter::Vgs, OpParameter::Region],
            ..Default::default()
        }
    }

    /// Create a detailed config showing all parameters.
    pub fn detailed() -> Self {
        Self {
            mosfet_params: vec![
                OpParameter::Vgs,
                OpParameter::Vds,
                OpParameter::Vth,
                OpParameter::Id,
                OpParameter::Gm,
                OpParameter::Gds,
                OpParameter::Region,
            ],
            bjt_params: vec![
                OpParameter::Vbe,
                OpParameter::Vce,
                OpParameter::Ic,
                OpParameter::Ib,
                OpParameter::Beta,
                OpParameter::Gm,
            ],
            max_params: 7,
            ..Default::default()
        }
    }
}

// =============================================================================
// OP Parameter Types
// =============================================================================

/// Operating point parameter types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OpParameter {
    // MOSFET parameters
    /// Gate-source voltage
    Vgs,
    /// Drain-source voltage
    Vds,
    /// Gate-drain voltage
    Vgd,
    /// Threshold voltage
    Vth,
    /// Drain current
    Id,
    /// Transconductance
    Gm,
    /// Output conductance
    Gds,
    /// Gate-source capacitance
    Cgs,
    /// Gate-drain capacitance
    Cgd,
    /// Operating region
    Region,

    // BJT parameters
    /// Base-emitter voltage
    Vbe,
    /// Collector-emitter voltage
    Vce,
    /// Base-collector voltage
    Vbc,
    /// Collector current
    Ic,
    /// Base current
    Ib,
    /// Emitter current
    Ie,
    /// DC current gain
    Beta,

    // Diode parameters
    /// Forward voltage
    Vd,

    // Common parameters
    /// Current (for any device)
    Current,
    /// Voltage (across device)
    Voltage,
    /// Power dissipation
    Power,
}

impl OpParameter {
    /// Get the display name for this parameter.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Vgs => "Vgs",
            Self::Vds => "Vds",
            Self::Vgd => "Vgd",
            Self::Vth => "Vth",
            Self::Id => "Id",
            Self::Gm => "gm",
            Self::Gds => "gds",
            Self::Cgs => "Cgs",
            Self::Cgd => "Cgd",
            Self::Region => "Region",
            Self::Vbe => "Vbe",
            Self::Vce => "Vce",
            Self::Vbc => "Vbc",
            Self::Ic => "Ic",
            Self::Ib => "Ib",
            Self::Ie => "Ie",
            Self::Beta => "β",
            Self::Vd => "Vd",
            Self::Current => "I",
            Self::Voltage => "V",
            Self::Power => "P",
        }
    }

    /// Get the SPICE-style parameter name.
    pub fn spice_name(&self) -> &'static str {
        match self {
            Self::Vgs => "vgs",
            Self::Vds => "vds",
            Self::Vgd => "vgd",
            Self::Vth => "vth",
            Self::Id => "id",
            Self::Gm => "gm",
            Self::Gds => "gds",
            Self::Cgs => "cgs",
            Self::Cgd => "cgd",
            Self::Region => "region",
            Self::Vbe => "vbe",
            Self::Vce => "vce",
            Self::Vbc => "vbc",
            Self::Ic => "ic",
            Self::Ib => "ib",
            Self::Ie => "ie",
            Self::Beta => "betadc",
            Self::Vd => "vd",
            Self::Current => "i",
            Self::Voltage => "v",
            Self::Power => "p",
        }
    }

    /// Get the unit for this parameter.
    pub fn unit(&self) -> &'static str {
        match self {
            Self::Vgs
            | Self::Vds
            | Self::Vgd
            | Self::Vth
            | Self::Vbe
            | Self::Vce
            | Self::Vbc
            | Self::Vd
            | Self::Voltage => "V",
            Self::Id | Self::Ic | Self::Ib | Self::Ie | Self::Current => "A",
            Self::Gm | Self::Gds => "S",
            Self::Cgs | Self::Cgd => "F",
            Self::Power => "W",
            Self::Beta => "",
            Self::Region => "",
        }
    }

    /// Check if this is a voltage parameter.
    pub fn is_voltage(&self) -> bool {
        matches!(
            self,
            Self::Vgs
                | Self::Vds
                | Self::Vgd
                | Self::Vth
                | Self::Vbe
                | Self::Vce
                | Self::Vbc
                | Self::Vd
                | Self::Voltage
        )
    }

    /// Check if this is a current parameter.
    pub fn is_current(&self) -> bool {
        matches!(
            self,
            Self::Id | Self::Ic | Self::Ib | Self::Ie | Self::Current
        )
    }
}

// =============================================================================
// OP Annotation
// =============================================================================

/// Operating point annotation for a single device.
#[derive(Debug, Clone, Default)]
pub struct OpAnnotation {
    /// Device instance name.
    pub device_name: String,
    /// Device type (M, Q, D, R, etc.).
    pub device_type: String,
    /// Formatted parameter lines.
    pub lines: Vec<String>,
    /// Raw parameter values.
    pub values: HashMap<OpParameter, f64>,
}

impl OpAnnotation {
    /// Create a new annotation from device operating point data.
    pub fn from_device_op(
        device_name: &str,
        device_op: &DeviceOpPoint,
        config: &OpAnnotationConfig,
    ) -> Self {
        let device_type = device_op.device_type.to_uppercase();
        let params = config.params_for_device(&device_type);

        let mut annotation = Self {
            device_name: device_name.to_string(),
            device_type: device_type.clone(),
            lines: Vec::new(),
            values: HashMap::new(),
        };

        for param in params.iter().take(config.max_params) {
            if let Some(value) = device_op.parameters.get(param.spice_name()) {
                annotation.values.insert(*param, *value);

                let formatted = if matches!(param, OpParameter::Region) {
                    format_region(*value as i32)
                } else if matches!(param, OpParameter::Beta) {
                    format!("{}={:.1}", param.display_name(), value)
                } else {
                    format!(
                        "{}={}",
                        param.display_name(),
                        format_engineering(*value, param.unit(), config.precision)
                    )
                };

                annotation.lines.push(formatted);
            }
        }

        annotation
    }

    /// Check if annotation has any parameters.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Get the number of parameter lines.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get a specific parameter value.
    pub fn get_value(&self, param: OpParameter) -> Option<f64> {
        self.values.get(&param).copied()
    }
}

/// Format region code to string.
fn format_region(region: i32) -> String {
    let region_str = match region {
        0 => "Off",
        1 => "Linear",
        2 => "Saturation",
        3 => "Subthreshold",
        _ => "Unknown",
    };
    format!("Region={}", region_str)
}

/// Format a value with engineering notation.
fn format_engineering(value: f64, unit: &str, precision: usize) -> String {
    let abs_value = value.abs();

    if abs_value == 0.0 {
        return format!("0{}", unit);
    }

    let (scaled, prefix) = if abs_value >= 1e12 {
        (value / 1e12, "T")
    } else if abs_value >= 1e9 {
        (value / 1e9, "G")
    } else if abs_value >= 1e6 {
        (value / 1e6, "M")
    } else if abs_value >= 1e3 {
        (value / 1e3, "k")
    } else if abs_value >= 1.0 {
        (value, "")
    } else if abs_value >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "µ")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_value >= 1e-12 {
        (value * 1e12, "p")
    } else if abs_value >= 1e-15 {
        (value * 1e15, "f")
    } else {
        (value * 1e18, "a")
    };

    format!("{:.prec$}{}{}", scaled, prefix, unit, prec = precision)
}

// =============================================================================
// OP Annotation Extractor
// =============================================================================

/// Extract annotations from DC operating point results.
pub fn extract_op_annotations(
    dc_op: &DcOpResult,
    config: &OpAnnotationConfig,
) -> HashMap<String, OpAnnotation> {
    let mut annotations = HashMap::new();

    for (device_name, device_op) in &dc_op.device_ops {
        let annotation = OpAnnotation::from_device_op(device_name, device_op, config);
        if !annotation.is_empty() {
            annotations.insert(device_name.clone(), annotation);
        }
    }

    annotations
}

/// Get annotation for a specific device.
pub fn get_device_annotation(
    device_name: &str,
    dc_op: &DcOpResult,
    config: &OpAnnotationConfig,
) -> Option<OpAnnotation> {
    dc_op
        .device_ops
        .get(device_name)
        .map(|op| OpAnnotation::from_device_op(device_name, op, config))
}

// =============================================================================
// OP Annotation Renderer
// =============================================================================

/// Renderer for operating point annotations on schematic.
#[derive(Debug, Clone, Default)]
pub struct OpAnnotationRenderer {
    /// Configuration.
    pub config: OpAnnotationConfig,
    /// Cached annotations (device name → annotation).
    pub annotations: HashMap<String, OpAnnotation>,
}

impl OpAnnotationRenderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with configuration.
    pub fn with_config(config: OpAnnotationConfig) -> Self {
        Self {
            config,
            annotations: HashMap::new(),
        }
    }

    /// Update annotations from DC OP result.
    pub fn update_from_dc_op(&mut self, dc_op: &DcOpResult) {
        self.annotations = extract_op_annotations(dc_op, &self.config);
    }

    /// Clear all annotations.
    pub fn clear(&mut self) {
        self.annotations.clear();
    }

    /// Check if there are any annotations.
    pub fn has_annotations(&self) -> bool {
        !self.annotations.is_empty()
    }

    /// Get annotation for a device.
    pub fn get(&self, device_name: &str) -> Option<&OpAnnotation> {
        self.annotations.get(device_name)
    }

    /// Render a single annotation at a position.
    pub fn render_at(&self, ui: &mut Ui, annotation: &OpAnnotation, position: Pos2) {
        if !self.config.enabled || annotation.is_empty() {
            return;
        }

        let offset_pos = Pos2::new(
            position.x + self.config.offset.0,
            position.y + self.config.offset.1,
        );

        let painter = ui.painter();
        let font_id = FontId::proportional(self.config.font_size);
        let galley = painter.layout_no_wrap(
            annotation.lines.join("\n"),
            font_id.clone(),
            self.config.text_color,
        );

        // Background rectangle
        let text_rect = Rect::from_min_size(offset_pos, galley.size());
        let bg_rect = text_rect.expand(2.0);
        painter.rect_filled(bg_rect, 2.0, self.config.bg_color);

        // Text
        painter.galley(offset_pos, galley, self.config.text_color);
    }

    /// Get the number of annotations.
    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // OpAnnotationConfig Tests
    // =========================================================================

    #[test]
    fn test_config_default() {
        let config = OpAnnotationConfig::default();

        assert!(!config.enabled);
        assert!(!config.mosfet_params.is_empty());
        assert!(!config.bjt_params.is_empty());
        assert_eq!(config.precision, 3);
        assert_eq!(config.max_params, 5);
    }

    #[test]
    fn test_config_new() {
        let config = OpAnnotationConfig::new();
        assert!(!config.enabled);
    }

    #[test]
    fn test_config_enable_disable() {
        let mut config = OpAnnotationConfig::new();

        config.enable();
        assert!(config.enabled);

        config.disable();
        assert!(!config.enabled);
    }

    #[test]
    fn test_config_toggle() {
        let mut config = OpAnnotationConfig::new();

        config.toggle();
        assert!(config.enabled);

        config.toggle();
        assert!(!config.enabled);
    }

    #[test]
    fn test_config_params_for_mosfet() {
        let config = OpAnnotationConfig::default();

        let params = config.params_for_device("M");
        assert!(!params.is_empty());

        let params = config.params_for_device("MOSFET");
        assert!(!params.is_empty());

        let params = config.params_for_device("nmos");
        assert!(!params.is_empty());
    }

    #[test]
    fn test_config_params_for_bjt() {
        let config = OpAnnotationConfig::default();

        let params = config.params_for_device("Q");
        assert!(!params.is_empty());

        let params = config.params_for_device("NPN");
        assert!(!params.is_empty());
    }

    #[test]
    fn test_config_params_for_diode() {
        let config = OpAnnotationConfig::default();
        let params = config.params_for_device("D");
        assert!(!params.is_empty());
    }

    #[test]
    fn test_config_params_for_unknown() {
        let config = OpAnnotationConfig::default();
        let params = config.params_for_device("UNKNOWN");
        assert!(params.is_empty());
    }

    #[test]
    fn test_config_mosfet_compact() {
        let config = OpAnnotationConfig::mosfet_compact();
        assert_eq!(config.mosfet_params.len(), 3);
    }

    #[test]
    fn test_config_detailed() {
        let config = OpAnnotationConfig::detailed();
        assert!(config.mosfet_params.len() > 5);
        assert_eq!(config.max_params, 7);
    }

    // =========================================================================
    // OpParameter Tests
    // =========================================================================

    #[test]
    fn test_parameter_display_name() {
        assert_eq!(OpParameter::Vgs.display_name(), "Vgs");
        assert_eq!(OpParameter::Id.display_name(), "Id");
        assert_eq!(OpParameter::Gm.display_name(), "gm");
        assert_eq!(OpParameter::Beta.display_name(), "β");
    }

    #[test]
    fn test_parameter_spice_name() {
        assert_eq!(OpParameter::Vgs.spice_name(), "vgs");
        assert_eq!(OpParameter::Id.spice_name(), "id");
        assert_eq!(OpParameter::Beta.spice_name(), "betadc");
    }

    #[test]
    fn test_parameter_unit() {
        assert_eq!(OpParameter::Vgs.unit(), "V");
        assert_eq!(OpParameter::Id.unit(), "A");
        assert_eq!(OpParameter::Gm.unit(), "S");
        assert_eq!(OpParameter::Beta.unit(), "");
    }

    #[test]
    fn test_parameter_is_voltage() {
        assert!(OpParameter::Vgs.is_voltage());
        assert!(OpParameter::Vds.is_voltage());
        assert!(OpParameter::Vbe.is_voltage());
        assert!(!OpParameter::Id.is_voltage());
        assert!(!OpParameter::Gm.is_voltage());
    }

    #[test]
    fn test_parameter_is_current() {
        assert!(OpParameter::Id.is_current());
        assert!(OpParameter::Ic.is_current());
        assert!(OpParameter::Ib.is_current());
        assert!(!OpParameter::Vgs.is_current());
        assert!(!OpParameter::Gm.is_current());
    }

    // =========================================================================
    // Format Engineering Tests
    // =========================================================================

    #[test]
    fn test_format_engineering_zero() {
        assert_eq!(format_engineering(0.0, "V", 3), "0V");
    }

    #[test]
    fn test_format_engineering_milli() {
        let s = format_engineering(1e-3, "A", 3);
        assert!(s.contains("m"));
        assert!(s.contains("A"));
    }

    #[test]
    fn test_format_engineering_micro() {
        let s = format_engineering(1e-6, "A", 3);
        assert!(s.contains("µ"));
    }

    #[test]
    fn test_format_engineering_nano() {
        let s = format_engineering(1e-9, "A", 3);
        assert!(s.contains("n"));
    }

    #[test]
    fn test_format_engineering_kilo() {
        let s = format_engineering(1e3, "Ω", 3);
        assert!(s.contains("k"));
    }

    #[test]
    fn test_format_engineering_mega() {
        let s = format_engineering(1e6, "Hz", 3);
        assert!(s.contains("M"));
    }

    #[test]
    fn test_format_engineering_negative() {
        let s = format_engineering(-1e-3, "A", 3);
        assert!(s.contains("-"));
    }

    // =========================================================================
    // Format Region Tests
    // =========================================================================

    #[test]
    fn test_format_region_off() {
        assert_eq!(format_region(0), "Region=Off");
    }

    #[test]
    fn test_format_region_linear() {
        assert_eq!(format_region(1), "Region=Linear");
    }

    #[test]
    fn test_format_region_saturation() {
        assert_eq!(format_region(2), "Region=Saturation");
    }

    #[test]
    fn test_format_region_subthreshold() {
        assert_eq!(format_region(3), "Region=Subthreshold");
    }

    #[test]
    fn test_format_region_unknown() {
        assert_eq!(format_region(99), "Region=Unknown");
    }

    // =========================================================================
    // OpAnnotation Tests
    // =========================================================================

    #[test]
    fn test_annotation_from_device_op() {
        let mut device_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: HashMap::new(),
        };
        device_op.parameters.insert("vgs".to_string(), 0.6);
        device_op.parameters.insert("vds".to_string(), 1.2);
        device_op.parameters.insert("id".to_string(), 1e-3);

        let config = OpAnnotationConfig::default();
        let annotation = OpAnnotation::from_device_op("M1", &device_op, &config);

        assert_eq!(annotation.device_name, "M1");
        assert_eq!(annotation.device_type, "M");
        assert!(!annotation.is_empty());
        assert!(annotation.line_count() >= 3);
    }

    #[test]
    fn test_annotation_empty() {
        let device_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: HashMap::new(),
        };

        let config = OpAnnotationConfig::default();
        let annotation = OpAnnotation::from_device_op("M1", &device_op, &config);

        assert!(annotation.is_empty());
        assert_eq!(annotation.line_count(), 0);
    }

    #[test]
    fn test_annotation_get_value() {
        let mut device_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: HashMap::new(),
        };
        device_op.parameters.insert("vgs".to_string(), 0.6);

        let config = OpAnnotationConfig::default();
        let annotation = OpAnnotation::from_device_op("M1", &device_op, &config);

        assert!((annotation.get_value(OpParameter::Vgs).unwrap() - 0.6).abs() < 1e-9);
        assert!(annotation.get_value(OpParameter::Gm).is_none());
    }

    #[test]
    fn test_annotation_max_params() {
        let mut device_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: HashMap::new(),
        };
        device_op.parameters.insert("vgs".to_string(), 0.6);
        device_op.parameters.insert("vds".to_string(), 1.2);
        device_op.parameters.insert("id".to_string(), 1e-3);
        device_op.parameters.insert("gm".to_string(), 10e-3);
        device_op.parameters.insert("gds".to_string(), 1e-6);
        device_op.parameters.insert("region".to_string(), 2.0);

        let mut config = OpAnnotationConfig::default();
        config.max_params = 3;

        let annotation = OpAnnotation::from_device_op("M1", &device_op, &config);
        assert!(annotation.line_count() <= 3);
    }

    // =========================================================================
    // Extract Annotations Tests
    // =========================================================================

    #[test]
    fn test_extract_op_annotations() {
        let mut dc_op = DcOpResult::default();

        let mut m1_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: HashMap::new(),
        };
        m1_op.parameters.insert("vgs".to_string(), 0.6);
        m1_op.parameters.insert("id".to_string(), 1e-3);
        dc_op.device_ops.insert("M1".to_string(), m1_op);

        let config = OpAnnotationConfig::default();
        let annotations = extract_op_annotations(&dc_op, &config);

        assert_eq!(annotations.len(), 1);
        assert!(annotations.contains_key("M1"));
    }

    #[test]
    fn test_extract_op_annotations_empty() {
        let dc_op = DcOpResult::default();
        let config = OpAnnotationConfig::default();
        let annotations = extract_op_annotations(&dc_op, &config);

        assert!(annotations.is_empty());
    }

    #[test]
    fn test_get_device_annotation() {
        let mut dc_op = DcOpResult::default();

        let mut m1_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: HashMap::new(),
        };
        m1_op.parameters.insert("vgs".to_string(), 0.6);
        dc_op.device_ops.insert("M1".to_string(), m1_op);

        let config = OpAnnotationConfig::default();

        let annotation = get_device_annotation("M1", &dc_op, &config);
        assert!(annotation.is_some());

        let annotation = get_device_annotation("M2", &dc_op, &config);
        assert!(annotation.is_none());
    }

    // =========================================================================
    // OpAnnotationRenderer Tests
    // =========================================================================

    #[test]
    fn test_renderer_new() {
        let renderer = OpAnnotationRenderer::new();
        assert!(!renderer.has_annotations());
        assert_eq!(renderer.annotation_count(), 0);
    }

    #[test]
    fn test_renderer_with_config() {
        let mut config = OpAnnotationConfig::default();
        config.enable();

        let renderer = OpAnnotationRenderer::with_config(config);
        assert!(renderer.config.enabled);
    }

    #[test]
    fn test_renderer_update_from_dc_op() {
        let mut dc_op = DcOpResult::default();

        let mut m1_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: HashMap::new(),
        };
        m1_op.parameters.insert("vgs".to_string(), 0.6);
        dc_op.device_ops.insert("M1".to_string(), m1_op);

        let mut renderer = OpAnnotationRenderer::new();
        renderer.update_from_dc_op(&dc_op);

        assert!(renderer.has_annotations());
        assert_eq!(renderer.annotation_count(), 1);
        assert!(renderer.get("M1").is_some());
    }

    #[test]
    fn test_renderer_clear() {
        let mut dc_op = DcOpResult::default();
        let m1_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: {
                let mut p = HashMap::new();
                p.insert("vgs".to_string(), 0.6);
                p
            },
        };
        dc_op.device_ops.insert("M1".to_string(), m1_op);

        let mut renderer = OpAnnotationRenderer::new();
        renderer.update_from_dc_op(&dc_op);
        assert!(renderer.has_annotations());

        renderer.clear();
        assert!(!renderer.has_annotations());
    }

    #[test]
    fn test_renderer_get() {
        let mut dc_op = DcOpResult::default();
        let m1_op = DeviceOpPoint {
            device_type: "M".to_string(),
            parameters: {
                let mut p = HashMap::new();
                p.insert("vgs".to_string(), 0.6);
                p
            },
        };
        dc_op.device_ops.insert("M1".to_string(), m1_op);

        let mut renderer = OpAnnotationRenderer::new();
        renderer.update_from_dc_op(&dc_op);

        assert!(renderer.get("M1").is_some());
        assert!(renderer.get("M2").is_none());
    }

    // =========================================================================
    // BJT Annotation Tests
    // =========================================================================

    #[test]
    fn test_bjt_annotation() {
        let mut device_op = DeviceOpPoint {
            device_type: "Q".to_string(),
            parameters: HashMap::new(),
        };
        device_op.parameters.insert("vbe".to_string(), 0.7);
        device_op.parameters.insert("vce".to_string(), 2.0);
        device_op.parameters.insert("ic".to_string(), 1e-3);
        device_op.parameters.insert("betadc".to_string(), 100.0);

        let config = OpAnnotationConfig::default();
        let annotation = OpAnnotation::from_device_op("Q1", &device_op, &config);

        assert_eq!(annotation.device_type, "Q");
        assert!(!annotation.is_empty());
        assert!(annotation.get_value(OpParameter::Vbe).is_some());
    }

    // =========================================================================
    // Diode Annotation Tests
    // =========================================================================

    #[test]
    fn test_diode_annotation() {
        let mut device_op = DeviceOpPoint {
            device_type: "D".to_string(),
            parameters: HashMap::new(),
        };
        device_op.parameters.insert("vd".to_string(), 0.65);
        device_op.parameters.insert("id".to_string(), 1e-3);

        let config = OpAnnotationConfig::default();
        let annotation = OpAnnotation::from_device_op("D1", &device_op, &config);

        assert_eq!(annotation.device_type, "D");
        assert!(!annotation.is_empty());
    }
}
