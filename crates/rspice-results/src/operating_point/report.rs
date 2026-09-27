//! Shared operating-point report facts and quantity vocabulary.

use super::{OperatingPointAnnotationEvidence, OperatingPointProcessEvidence};

/// Selected solve facts shared by operating-point viewers and report encoders.
#[derive(Debug, Clone)]
pub struct OperatingPointReportFacts {
    pub temperature_celsius: f64,
    pub process: OperatingPointProcessEvidence,
    pub point_index: u64,
    pub point_count: u64,
    pub mna_nodes: usize,
    pub mna_branches: usize,
    pub annotation: OperatingPointAnnotationEvidence,
}

/// Retained process-corner label.
pub fn process_label(process: OperatingPointProcessEvidence) -> &'static str {
    match process {
        OperatingPointProcessEvidence::TT => "TT",
        OperatingPointProcessEvidence::SS => "SS",
        OperatingPointProcessEvidence::FF => "FF",
        OperatingPointProcessEvidence::SF => "SF",
        OperatingPointProcessEvidence::FS => "FS",
    }
}

/// Retained annotation-policy label.
pub fn annotation_label(annotation: OperatingPointAnnotationEvidence) -> &'static str {
    match annotation {
        OperatingPointAnnotationEvidence::VoltagesAndCurrents => "voltages and currents",
        OperatingPointAnnotationEvidence::VoltagesOnly => "voltages only",
        OperatingPointAnnotationEvidence::VoltagesAndDeviceOp => "voltages and device OP",
        OperatingPointAnnotationEvidence::None => "not retained",
    }
}

/// Unit of an existing operating-point report quantity; unknown labels stay unspecified.
pub fn device_param_unit(family: &str, name: &str) -> &'static str {
    if rspice_core::op_label::OpLabel::is_intrinsic_voltage_name(name) {
        return "V";
    }
    match (family, name) {
        ("MOSFET", "id") | ("BJT", "ic" | "ib") | ("DIODE", "id") => "A",
        ("MOSFET", "vgs" | "vds" | "vbs" | "vth") | ("BJT", "vbe" | "vce") | ("DIODE", "vd") => "V",
        ("MOSFET", "gm" | "gds" | "gmb") | ("BJT", "gm") | ("DIODE", "gd") => "S",
        _ => "",
    }
}
