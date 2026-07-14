use rspice_core::Value;
use rspice_core::netlist::{ElementKind, SourceSpec};

pub(super) fn apply_voltage_corner(
    netlist: &mut rspice_core::Netlist,
    corner_voltage: Value,
    nominal_voltage: Value,
) -> Result<(), String> {
    if !corner_voltage.is_finite() || corner_voltage <= 0.0 {
        return Err("Corner voltage must be a positive finite value".to_string());
    }
    if !nominal_voltage.is_finite() || nominal_voltage <= 0.0 {
        return Err("Corner nominal voltage must be a positive finite value".to_string());
    }
    let scale = corner_voltage / nominal_voltage;

    let mut candidate_indices = Vec::new();
    for (idx, element) in netlist.elements.iter().enumerate() {
        let Some(neg) = element.nodes.get(1) else {
            continue;
        };
        if !is_ground_node(neg) {
            continue;
        }
        if let ElementKind::VoltageSource(spec) = &element.kind
            && dc_value_from_source(spec).is_some()
        {
            candidate_indices.push(idx);
        }
    }

    if candidate_indices.is_empty() {
        for (idx, element) in netlist.elements.iter().enumerate() {
            if let ElementKind::VoltageSource(spec) = &element.kind
                && dc_value_from_source(spec).is_some()
            {
                candidate_indices.push(idx);
            }
        }
    }

    for idx in candidate_indices {
        let Some(element) = netlist.elements.get_mut(idx) else {
            continue;
        };
        if let ElementKind::VoltageSource(spec) = &mut element.kind
            && let Some(dc) = dc_value_from_source(spec)
        {
            let _ = set_dc_value_for_source(spec, dc * scale);
        }
    }

    Ok(())
}

pub(super) fn infer_nominal_supply_voltage(netlist: &rspice_core::Netlist) -> Option<Value> {
    let mut ground_referenced = Vec::new();
    let mut all_sources = Vec::new();

    for element in &netlist.elements {
        if let ElementKind::VoltageSource(spec) = &element.kind
            && let Some(dc) = dc_value_from_source(spec)
        {
            let abs_dc = dc.abs();
            if abs_dc <= 1e-15 {
                continue;
            }
            all_sources.push(abs_dc);
            if element
                .nodes
                .get(1)
                .map(|name| is_ground_node(name))
                .unwrap_or(false)
            {
                ground_referenced.push(abs_dc);
            }
        }
    }

    if !ground_referenced.is_empty() {
        return ground_referenced.into_iter().max_by(|a, b| a.total_cmp(b));
    }
    all_sources.into_iter().max_by(|a, b| a.total_cmp(b))
}

fn is_ground_node(node: &str) -> bool {
    let n = node.trim();
    n == "0" || n.eq_ignore_ascii_case("gnd") || n.eq_ignore_ascii_case("ground")
}

fn dc_value_from_source(spec: &SourceSpec) -> Option<Value> {
    match spec {
        SourceSpec::Dc(v) => Some(*v),
        SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}

fn set_dc_value_for_source(spec: &mut SourceSpec, value: Value) -> bool {
    match spec {
        SourceSpec::Dc(v) => {
            *v = value;
            true
        }
        SourceSpec::DcAc { dc_value, .. } => {
            *dc_value = value;
            true
        }
        _ => false,
    }
}
