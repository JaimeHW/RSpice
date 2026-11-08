//! Shared operating-point report facts and quantity vocabulary.

use super::{
    OperatingPointAnnotationEvidence, OperatingPointDeviceDetailEvidence,
    OperatingPointProcessEvidence,
};

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

/// Unit of a reported operating-point quantity, or `""` for one without.
///
/// A label means one physical quantity whichever family reports it, so only
/// the family's standing is read: a family outside the fixed vocabulary is a
/// compiled Verilog-A model, and all such a model reports are the currents at
/// its terminals, under names its ports decide.
pub fn device_param_unit(family: &str, name: &str) -> &'static str {
    use rspice_core::op_label::{OP_LABELS, OpLabel};

    if OpLabel::is_intrinsic_voltage_name(name) {
        return "V";
    }
    if !OP_LABELS.iter().any(|label| *label == family) {
        return "A";
    }
    match name {
        "id" | "ig" | "is" | "ic" | "ib" | "ie" | "igs" | "igd" | "idiode" | "idsw" => "A",
        "vd" | "vgs" | "vds" | "vbs" | "vbe" | "vce" | "vth" | "vdsat" | "output_vdsat"
        | "vdsw" => "V",
        "gm" | "gds" | "gd" | "gdsw" | "gmb" | "gmbs" | "gpi" | "gmu" | "go" => "S",
        "cd" | "cdsw" | "c" | "cpi" | "cmu" => "F",
        "qbe" | "qbc" | "qd" => "C",
        "r" => "\u{3a9}",
        "power" => "W",
        "temp" => "\u{b0}C",
        _ => "",
    }
}

/// Device identities admitted by the retained operating-point save policy.
#[derive(Clone)]
pub struct RetainedDeviceDetail {
    pub policy: OperatingPointDeviceDetailEvidence,
    pub selected: Vec<String>,
    pub violations: Vec<String>,
}

/// Whether a retained report row belongs to the recorded device-detail scope.
pub fn retained_detail_allows(
    entry_name: &str,
    retained_detail: Option<&RetainedDeviceDetail>,
) -> bool {
    let Some(detail) = retained_detail else {
        // Legacy reports predate explicit detail metadata. Their existing rows
        // remain exact retained evidence, so hiding them would discard data.
        return true;
    };
    match detail.policy {
        OperatingPointDeviceDetailEvidence::AllDevices => true,
        OperatingPointDeviceDetailEvidence::SelectedAndViolations => {
            (detail.selected.is_empty() && detail.violations.is_empty())
                || contains_identity(&detail.selected, entry_name)
                || contains_identity(&detail.violations, entry_name)
        }
        OperatingPointDeviceDetailEvidence::ViolationsOnly => {
            contains_identity(&detail.violations, entry_name)
        }
        OperatingPointDeviceDetailEvidence::None => false,
    }
}
fn contains_identity(identities: &[String], candidate: &str) -> bool {
    identities
        .iter()
        .any(|identity| identity.eq_ignore_ascii_case(candidate))
}

/// Bare retained identity inside a voltage, current, or power accessor.
pub fn signal_leaf(name: &str) -> &str {
    let name = name.trim();
    if name.len() >= 3
        && name.as_bytes().get(1) == Some(&b'(')
        && name.ends_with(')')
        && matches!(name.as_bytes()[0].to_ascii_uppercase(), b'V' | b'I' | b'P')
    {
        name[2..name.len() - 1].trim()
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// The vocabulary's device families and operating regions, which name no
    /// quantity.
    const NOT_QUANTITIES: &[&str] = &[
        "MOSFET",
        "MOS9",
        "BSIM3",
        "BSIM4",
        "B3SOIFD",
        "B3SOIDD",
        "B3SOIPD",
        "EKV26",
        "EKV3",
        "VDMOS",
        "BJT",
        "DIODE",
        "RESISTOR",
        "CAPACITOR",
        "JFET",
        "JFET2",
        "JFET2_XYCE",
        "MESFET",
        "HFET1",
        "HFET2",
        "NONLINEAR_CORE",
        "VERILOGA",
        "cutoff",
        "linear",
        "triode",
        "saturation",
        "quasi-saturation",
        "subthreshold",
        "body-diode",
        "active",
        "reverse",
        "forward",
    ];
    /// A current gain is a ratio.
    const UNITLESS: &[&str] = &["beta"];
    /// A nonlinear core's state, reported in Xyce's convention: its unit
    /// follows the core's LEVEL and BHSIUNITS, which the label does not carry.
    const UNIT_SET_BY_THE_MODEL: &[&str] = &["m", "h", "b"];

    #[test]
    fn every_reported_quantity_is_classified() {
        use rspice_core::op_label::OP_LABELS;
        for label in OP_LABELS {
            let label = label.as_str();
            if NOT_QUANTITIES.contains(&label) {
                continue;
            }
            let unit = device_param_unit("MOSFET", label);
            if UNITLESS.contains(&label) || UNIT_SET_BY_THE_MODEL.contains(&label) {
                assert_eq!(unit, "", "{label} was listed as having no fixed unit");
            } else {
                assert_ne!(
                    unit, "",
                    "{label} has no unit: name one in device_param_unit, or list \
                     it as unitless with the reason"
                );
            }
        }
    }

    #[test]
    fn a_quantity_states_the_physics_of_its_label() {
        for (family, name, unit) in [
            ("DIODE", "cd", "F"),
            ("DIODE", "cdsw", "F"),
            ("DIODE", "temp", "\u{b0}C"),
            ("BSIM4", "output_vdsat", "V"),
            ("VDMOS", "power", "W"),
            ("RESISTOR", "r", "\u{3a9}"),
            ("CAPACITOR", "c", "F"),
            ("MOSFET", "vgs_intrinsic", "V"),
            ("BJT", "beta", ""),
        ] {
            assert_eq!(device_param_unit(family, name), unit, "{family} {name}");
        }
    }

    #[test]
    fn a_compiled_model_reports_terminal_currents_under_its_ports_names() {
        assert_eq!(device_param_unit("vamp", "iout"), "A");
        assert_eq!(device_param_unit("psp104", "id"), "A");
    }

    #[test]
    fn detail_policy_never_expands_beyond_retained_rows() {
        let selected = RetainedDeviceDetail {
            policy: OperatingPointDeviceDetailEvidence::SelectedAndViolations,
            selected: vec!["M1".to_owned()],
            violations: vec!["M3".to_owned()],
        };
        assert!(retained_detail_allows("m1", Some(&selected)));
        assert!(retained_detail_allows("M3", Some(&selected)));
        assert!(!retained_detail_allows("M2", Some(&selected)));

        let none = RetainedDeviceDetail {
            policy: OperatingPointDeviceDetailEvidence::None,
            selected: Vec::new(),
            violations: Vec::new(),
        };
        assert!(!retained_detail_allows("M1", Some(&none)));
    }
}
