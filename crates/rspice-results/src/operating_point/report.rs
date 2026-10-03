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
