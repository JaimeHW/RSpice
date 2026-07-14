use super::*;
use crate::device::veriloga_generated::{builtins, instantiate_builtin};
use crate::netlist::{Element, ModelDef, ParametricValue};

#[derive(Clone, Copy)]
struct GeneratedTarget {
    model_name: &'static str,
    pass_level_parameter: bool,
    consumes_geometry_bin_metadata: bool,
}

impl GeneratedTarget {
    const fn new(model_name: &'static str) -> Self {
        Self {
            model_name,
            pass_level_parameter: false,
            consumes_geometry_bin_metadata: false,
        }
    }

    const fn with_level_parameter(model_name: &'static str) -> Self {
        Self {
            model_name,
            pass_level_parameter: true,
            consumes_geometry_bin_metadata: false,
        }
    }

    const fn consuming_geometry_bin_metadata(model_name: &'static str) -> Self {
        Self {
            model_name,
            pass_level_parameter: false,
            consumes_geometry_bin_metadata: true,
        }
    }

    fn is_available(self) -> bool {
        builtins::node_count(self.model_name).is_some()
    }
}

pub(super) fn try_route_generated_resistor_model(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &Element,
    model_name: &str,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
    temperature: f64,
) -> Result<bool, SimulationError> {
    let model_def = find_model_def(netlist, model_name);
    let target = generated_resistor_target(model_name, model_def)?;
    let Some(target) = target.filter(|target| target.is_available()) else {
        return Ok(false);
    };

    let mut nodes = element.nodes.clone();
    let expected = expected_terminal_count(element, target)?;
    if nodes.len() > expected {
        return Err(generated_terminal_error(
            element,
            target,
            nodes.len(),
            expected,
        ));
    }
    if target.model_name.eq_ignore_ascii_case("R3_CMC") && nodes.len() < expected {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}': generated Verilog-A model '{}' requires explicit control and thermal terminals; instantiate it as an X-device instead of a two-terminal R element",
            element.name, target.model_name
        )));
    }
    while nodes.len() < expected {
        nodes.push("0".to_string());
    }

    add_generated_instance(
        circuit,
        netlist,
        element,
        target,
        &nodes,
        model_def,
        instance_params,
        deferred_params,
        temperature,
    )?;
    Ok(true)
}

pub(super) fn try_route_generated_diode_model(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &Element,
    model_name: &str,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
    temperature: f64,
) -> Result<bool, SimulationError> {
    let model_def = find_model_def(netlist, model_name);
    let target = generated_diode_target(model_name, model_def)?;
    let Some(target) = target.filter(|target| target.is_available()) else {
        return Ok(false);
    };

    let nodes = exact_or_ground_padded_nodes(element, target, 0)?;
    add_generated_instance(
        circuit,
        netlist,
        element,
        target,
        &nodes,
        model_def,
        instance_params,
        deferred_params,
        temperature,
    )?;
    Ok(true)
}

pub(super) fn try_route_generated_bjt_model(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &Element,
    model_name: &str,
    model_def: Option<&ModelDef>,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
    temperature: f64,
) -> Result<bool, SimulationError> {
    let target = generated_bjt_target(model_name, model_def, element.nodes.len())?;
    let Some(target) = target.filter(|target| target.is_available()) else {
        return Ok(false);
    };

    let expected = expected_terminal_count(element, target)?;
    let max_implicit_ground_nodes = expected.saturating_sub(3);
    let nodes = exact_or_ground_padded_nodes(element, target, max_implicit_ground_nodes)?;
    add_generated_instance(
        circuit,
        netlist,
        element,
        target,
        &nodes,
        model_def,
        instance_params,
        deferred_params,
        temperature,
    )?;
    Ok(true)
}

pub(super) fn try_route_generated_mos_model(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &Element,
    model_name: &str,
    model_def: Option<&ModelDef>,
    compact_syntax: bool,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
    temperature: f64,
) -> Result<bool, SimulationError> {
    let target = generated_mos_target(model_name, model_def)?;
    let Some(target) = target.filter(|target| target.is_available()) else {
        return Ok(false);
    };
    let expected = expected_terminal_count(element, target)?;
    let nodes = if compact_syntax {
        compact_generated_mos_nodes(element, target, expected)?
    } else {
        let max_implicit_ground_nodes = expected.saturating_sub(4);
        exact_or_ground_padded_nodes(element, target, max_implicit_ground_nodes)?
    };
    add_generated_instance(
        circuit,
        netlist,
        element,
        target,
        &nodes,
        model_def,
        instance_params,
        deferred_params,
        temperature,
    )?;
    Ok(true)
}

fn compact_generated_mos_nodes(
    element: &Element,
    target: GeneratedTarget,
    expected: usize,
) -> Result<Vec<String>, SimulationError> {
    if expected == 3 {
        return compact_generated_mos_prefix_nodes(element, target, 3, expected);
    }

    if match_normalized(target.model_name, &["ANGELOV_GAN"]) && expected == 5 {
        return compact_generated_mos_prefix_nodes(element, target, 3, expected);
    }

    if match_normalized(target.model_name, &["EPFL_HEMT_10A"]) && expected == 5 {
        return compact_generated_mos_prefix_nodes(element, target, 4, expected);
    }

    Err(SimulationError::Circuit(format!(
        "MOSFET '{}': generated Verilog-A model '{}' requires explicit MOS terminals; compact three-terminal syntax is not accepted for this model",
        element.name, target.model_name
    )))
}

fn compact_generated_mos_prefix_nodes(
    element: &Element,
    target: GeneratedTarget,
    prefix_len: usize,
    expected: usize,
) -> Result<Vec<String>, SimulationError> {
    if element.nodes.len() < prefix_len {
        return Err(generated_terminal_error(
            element,
            target,
            element.nodes.len(),
            prefix_len,
        ));
    }
    let mut nodes = element.nodes[..prefix_len].to_vec();
    nodes.resize(expected, "0".to_string());
    Ok(nodes)
}

fn generated_resistor_target(
    model_name: &str,
    model_def: Option<&ModelDef>,
) -> Result<Option<GeneratedTarget>, SimulationError> {
    let type_name = model_def
        .map(|model| model.model_type.as_str())
        .unwrap_or(model_name);
    let exact = if match_normalized(type_name, &["R2_CMC", "R2"]) {
        Some(GeneratedTarget::with_level_parameter("R2_CMC"))
    } else if match_normalized(type_name, &["R2_ET_CMC", "R2_ET"]) {
        Some(GeneratedTarget::with_level_parameter("R2_ET_CMC"))
    } else if match_normalized(type_name, &["R3_CMC", "R3"]) {
        Some(GeneratedTarget::with_level_parameter("R3_CMC"))
    } else {
        None
    };
    if exact.is_some() {
        return Ok(exact);
    }

    let Some(model) = model_def else {
        return Ok(None);
    };
    if !is_resistor_model_type(&model.model_type) {
        return Ok(None);
    }
    Ok(match checked_model_level("Resistor", model_name, model)? {
        Some(1002) => Some(GeneratedTarget::with_level_parameter("R2_CMC")),
        Some(1003) => Some(GeneratedTarget::with_level_parameter("R3_CMC")),
        _ => None,
    })
}

fn generated_diode_target(
    model_name: &str,
    model_def: Option<&ModelDef>,
) -> Result<Option<GeneratedTarget>, SimulationError> {
    let type_name = model_def
        .map(|model| model.model_type.as_str())
        .unwrap_or(model_name);
    if match_normalized(type_name, &["DIODE_CMC"]) {
        return Ok(Some(GeneratedTarget::with_level_parameter("DIODE_CMC")));
    }
    if match_normalized(type_name, &["JUNCAP200"]) {
        return Ok(Some(GeneratedTarget::with_level_parameter("JUNCAP200")));
    }

    let Some(model) = model_def else {
        return Ok(None);
    };
    if !matches_model_type(&model.model_type, &["D", "DIODE"]) {
        return Ok(None);
    }
    Ok(match checked_model_level("Diode", model_name, model)? {
        Some(200) => Some(GeneratedTarget::with_level_parameter("JUNCAP200")),
        Some(2002) => Some(GeneratedTarget::with_level_parameter("DIODE_CMC")),
        _ => None,
    })
}

fn generated_bjt_target(
    model_name: &str,
    model_def: Option<&ModelDef>,
    instance_terminal_count: usize,
) -> Result<Option<GeneratedTarget>, SimulationError> {
    let type_name = model_def
        .map(|model| model.model_type.as_str())
        .unwrap_or(model_name);
    let explicit = match_normalized(type_name, &["HICUML0VA", "HICUML0"])
        .then_some(GeneratedTarget::new("hicumL0va"))
        .or_else(|| {
            match_normalized(type_name, &["HICUML2VA", "HICUML2"])
                .then_some(GeneratedTarget::new("hicumL2va"))
        })
        .or_else(|| {
            match_normalized(type_name, &["BJT505_VA", "MEXTRAM505"])
                .then_some(GeneratedTarget::new("bjt505_va"))
        })
        .or_else(|| {
            match_normalized(type_name, &["BJT505T_VA", "MEXTRAM505T"])
                .then_some(GeneratedTarget::new("bjt505t_va"))
        })
        .or_else(|| {
            match_normalized(type_name, &["BJTD505_VA"])
                .then_some(GeneratedTarget::new("bjtd505_va"))
        })
        .or_else(|| {
            match_normalized(type_name, &["BJTD505T_VA"])
                .then_some(GeneratedTarget::new("bjtd505t_va"))
        })
        .or_else(|| {
            match_normalized(type_name, &["VBIC", "VBIC13", "VBIC13_4T", "VBIC1P3"])
                .then_some(GeneratedTarget::new("vbic13_4t"))
        })
        .or_else(|| {
            match_normalized(type_name, &["VBIC_4T_ET_CF"])
                .then_some(GeneratedTarget::new("vbic_4T_et_cf"))
        });
    if explicit.is_some() {
        return Ok(explicit);
    }

    let Some(model) = model_def else {
        return Ok(None);
    };
    if resolve_bjt_type_from_model(&model.model_type).is_none() {
        return Ok(None);
    }
    Ok(match checked_model_level("BJT", model_name, model)? {
        Some(8 | 234) => Some(GeneratedTarget::new("hicumL2va")),
        Some(230) => Some(GeneratedTarget::new("hicumL0va")),
        Some(504) if instance_terminal_count >= 4 => Some(GeneratedTarget::new("bjt505_va")),
        Some(504) => Some(GeneratedTarget::new("bjtd505_va")),
        Some(505) if instance_terminal_count >= 4 => Some(GeneratedTarget::new("bjt505t_va")),
        Some(505) => Some(GeneratedTarget::new("bjtd505t_va")),
        Some(11) if instance_terminal_count >= 4 => Some(GeneratedTarget::new("vbic13_3t_et")),
        Some(11) => Some(GeneratedTarget::new("vbic13")),
        Some(4 | 9 | 12 | 13) => Some(GeneratedTarget::new("vbic13_4t")),
        _ => None,
    })
}

fn generated_mos_target(
    model_name: &str,
    model_def: Option<&ModelDef>,
) -> Result<Option<GeneratedTarget>, SimulationError> {
    let type_name = model_def
        .map(|model| model.model_type.as_str())
        .unwrap_or(model_name);
    let explicit = generated_mos_target_by_type(type_name);
    if explicit.is_some() {
        return Ok(explicit);
    }

    let Some(model) = model_def else {
        return Ok(None);
    };
    if resolve_mos_type_from_model(&model.model_type).is_none()
        && !matches_model_type(&model.model_type, &["VDMOS"])
    {
        return Ok(None);
    }
    Ok(match checked_model_level("MOSFET", model_name, model)? {
        Some(104) => Some(GeneratedTarget::with_level_parameter("PSP104VA")),
        Some(10 | 55 | 56 | 57) => Some(GeneratedTarget::new("bsimsoi_va")),
        Some(107 | 108 | 110 | 111) => Some(GeneratedTarget::consuming_geometry_bin_metadata(
            "bsimcmg_va",
        )),
        Some(260) => Some(GeneratedTarget::new("ekv_va")),
        Some(70470) => Some(GeneratedTarget::new("bsimsoi__18c250bc")),
        Some(1000) => Some(GeneratedTarget::with_level_parameter("mosvar")),
        Some(2002) => Some(GeneratedTarget::new("mvsg_cmc")),
        Some(10240) => Some(GeneratedTarget::new("l_utsoi__832ce87d")),
        _ => None,
    })
}

fn generated_mos_target_by_type(model_type: &str) -> Option<GeneratedTarget> {
    match_normalized(model_type, &["BSIMBULK"])
        .then_some(GeneratedTarget::new("bsimbulk"))
        .or_else(|| {
            match_normalized(model_type, &["BSIMCMG", "BSIMCMG_VA"]).then_some(
                GeneratedTarget::consuming_geometry_bin_metadata("bsimcmg_va"),
            )
        })
        .or_else(|| {
            match_normalized(model_type, &["BSIMIMG"]).then_some(GeneratedTarget::new("bsimimg"))
        })
        .or_else(|| {
            match_normalized(model_type, &["BSIMSOI_VA", "BSIMSOI46", "BSIMSOI461"])
                .then_some(GeneratedTarget::new("bsimsoi_va"))
        })
        .or_else(|| {
            match_normalized(
                model_type,
                &["BSIMSOI", "BSIM-SOI", "BSIMSOI4", "BSIMSOI47"],
            )
            .then_some(GeneratedTarget::new("bsimsoi__18c250bc"))
        })
        .or_else(|| {
            match_normalized(model_type, &["BSIMSOI100", "BSIMSOI__E2AFF994"])
                .then_some(GeneratedTarget::new("bsimsoi__e2aff994"))
        })
        .or_else(|| {
            match_normalized(model_type, &["PSP104", "PSP104VA"])
                .then_some(GeneratedTarget::with_level_parameter("PSP104VA"))
        })
        .or_else(|| {
            match_normalized(model_type, &["PSP104TVA"])
                .then_some(GeneratedTarget::with_level_parameter("PSP104TVA"))
        })
        .or_else(|| {
            match_normalized(model_type, &["PSPNQS104VA", "PSP104NQS"])
                .then_some(GeneratedTarget::with_level_parameter("PSPNQS104VA"))
        })
        .or_else(|| {
            match_normalized(model_type, &["L_UTSOI", "LUTSOI", "L_UTSOI__832CE87D"])
                .then_some(GeneratedTarget::new("l_utsoi__832ce87d"))
        })
        .or_else(|| {
            match_normalized(model_type, &["L_UTSOI_NQS", "L_UTSOI__485E0AC9"])
                .then_some(GeneratedTarget::new("l_utsoi__485e0ac9"))
        })
        .or_else(|| {
            match_normalized(model_type, &["MVSG", "MVSG_CMC"])
                .then_some(GeneratedTarget::new("mvsg_cmc"))
        })
        .or_else(|| {
            match_normalized(model_type, &["MOSVAR"])
                .then_some(GeneratedTarget::with_level_parameter("mosvar"))
        })
        .or_else(|| {
            match_normalized(model_type, &["EKV", "EKV26", "EKV26VA", "EKV_VA"])
                .then_some(GeneratedTarget::new("ekv_va"))
        })
        .or_else(|| {
            match_normalized(model_type, &["HISIMHV", "HISIMHV_VA"])
                .then_some(GeneratedTarget::new("hisimhv_va"))
        })
        .or_else(|| {
            match_normalized(model_type, &["HISIMSOI", "HISIMSOI_VA"])
                .then_some(GeneratedTarget::new("hisimsoi_va__5be18005"))
        })
        .or_else(|| {
            match_normalized(model_type, &["HISIMSOTB", "HISIMSOTB_VA"])
                .then_some(GeneratedTarget::new("hisimsotb_va"))
        })
        .or_else(|| {
            match_normalized(model_type, &["ASMHEMT"]).then_some(GeneratedTarget::new("asmhemt"))
        })
        .or_else(|| {
            match_normalized(model_type, &["ANGELOV"]).then_some(GeneratedTarget::new("angelov"))
        })
        .or_else(|| {
            match_normalized(model_type, &["ANGELOV_GAN", "ANGELOVGAN"])
                .then_some(GeneratedTarget::new("angelov_gan"))
        })
        .or_else(|| {
            match_normalized(model_type, &["EPFL_HEMT", "EPFL_HEMT_10A", "EPFLHEMT"])
                .then_some(GeneratedTarget::new("EPFL_HEMT_10a"))
        })
}

fn add_generated_instance(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &Element,
    target: GeneratedTarget,
    nodes: &[String],
    model_def: Option<&ModelDef>,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
    temperature: f64,
) -> Result<(), SimulationError> {
    let params = generated_params(target, model_def, instance_params, deferred_params)?;
    let Some(mut device) = instantiate_builtin(
        target.model_name,
        &element.name,
        nodes,
        &params,
        &netlist.params,
        circuit,
    )?
    else {
        return Ok(());
    };
    device.set_temperature(temperature);
    circuit.add_generated_veriloga_device(device);
    Ok(())
}

fn generated_params(
    target: GeneratedTarget,
    model_def: Option<&ModelDef>,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
) -> Result<Vec<(String, ParametricValue)>, SimulationError> {
    let model_count = model_def
        .map(|model| model.params.len() + model.expr_params.len() + model.string_params.len())
        .unwrap_or(0);
    let mut params =
        Vec::with_capacity(model_count + instance_params.len() + deferred_params.len());
    if let Some(model) = model_def {
        for (name, value) in &model.params {
            if should_pass_model_param(target, name) {
                params.push((name.clone(), ParametricValue::Resolved(*value)));
            }
        }
        for (name, expr) in &model.expr_params {
            if should_pass_model_param(target, name) {
                params.push((name.clone(), ParametricValue::Expression(expr.clone())));
            }
        }
        for (name, value) in &model.string_params {
            if should_pass_model_param(target, name) {
                return Err(SimulationError::Circuit(format!(
                    "Generated Verilog-A model '{}' cannot route non-numeric model parameter {}=\"{}\" from .model '{}'",
                    target.model_name, name, value, model.name
                )));
            }
        }
    }
    append_generated_mos_polarity_param(target, model_def, &mut params);
    append_generated_bjt_polarity_param(target, model_def, &mut params);
    append_generated_instance_params(target, &mut params, instance_params, deferred_params)?;
    Ok(params)
}

fn should_pass_model_param(target: GeneratedTarget, name: &str) -> bool {
    !(target.consumes_geometry_bin_metadata && is_model_binning_metadata(name))
        && (target.pass_level_parameter || !name.eq_ignore_ascii_case("LEVEL"))
}

fn is_model_binning_metadata(name: &str) -> bool {
    ["LMIN", "LMAX", "WMIN", "WMAX", "NFINMIN", "NFINMAX"]
        .iter()
        .any(|metadata| name.eq_ignore_ascii_case(metadata))
}

fn append_generated_mos_polarity_param(
    target: GeneratedTarget,
    model_def: Option<&ModelDef>,
    params: &mut Vec<(String, ParametricValue)>,
) {
    if !needs_inferred_generated_mos_type(target.model_name) {
        return;
    }
    if params
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("TYPE"))
    {
        return;
    }
    let Some(model) = model_def else {
        return;
    };
    let Some(mos_type) = resolve_mos_type_from_model(&model.model_type) else {
        return;
    };
    let type_value = match mos_type {
        crate::netlist::MosType::Nmos => 1.0,
        crate::netlist::MosType::Pmos => -1.0,
    };
    params.push(("TYPE".to_string(), ParametricValue::Resolved(type_value)));
}

fn needs_inferred_generated_mos_type(model_name: &str) -> bool {
    match_normalized(model_name, &["EKV_VA", "BSIMSOI_VA", "BSIMCMG_VA"])
}

fn append_generated_bjt_polarity_param(
    target: GeneratedTarget,
    model_def: Option<&ModelDef>,
    params: &mut Vec<(String, ParametricValue)>,
) {
    if !needs_inferred_generated_bjt_type(target.model_name) {
        return;
    }
    if params
        .iter()
        .any(|(name, _)| matches_model_type(name, &["TYPE", "NPN", "PNP"]))
    {
        return;
    }
    let Some(model) = model_def else {
        return;
    };
    let Some(bjt_type) = resolve_bjt_type_from_model(&model.model_type) else {
        return;
    };
    let type_value = match bjt_type {
        crate::netlist::BjtType::Npn => 1.0,
        crate::netlist::BjtType::Pnp => -1.0,
    };
    params.push(("type".to_string(), ParametricValue::Resolved(type_value)));
}

fn needs_inferred_generated_bjt_type(model_name: &str) -> bool {
    match_normalized(
        model_name,
        &["VBIC13", "VBIC13_3T_ET", "VBIC13_4T", "VBIC_4T_ET_CF"],
    )
}

fn append_generated_instance_params(
    target: GeneratedTarget,
    params: &mut Vec<(String, ParametricValue)>,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
) -> Result<(), SimulationError> {
    if !is_generated_bjt_target(target.model_name) {
        for (name, value) in instance_params {
            if is_internal_routing_param(name) {
                continue;
            }
            params.push((name.clone(), ParametricValue::Resolved(*value)));
        }
        for (name, expr) in deferred_params {
            params.push((name.clone(), ParametricValue::Expression(expr.clone())));
        }
        return Ok(());
    }

    let mut multiplier = 1.0;
    let mut multiplier_given = false;
    let mut multiplier_exprs = Vec::new();
    for (name, value) in instance_params {
        if is_internal_routing_param(name) {
            continue;
        } else if is_bjt_multiplier_param(name) {
            if !value.is_finite() || *value <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Generated Verilog-A BJT model '{}' instance parameter {}={} must be finite and > 0",
                    target.model_name, name, value
                )));
            }
            multiplier *= *value;
            multiplier_given = true;
        } else {
            params.push((name.clone(), ParametricValue::Resolved(*value)));
        }
    }
    for (name, expr) in deferred_params {
        if is_bjt_multiplier_param(name) {
            multiplier_exprs.push(expr.clone());
            multiplier_given = true;
        } else {
            params.push((name.clone(), ParametricValue::Expression(expr.clone())));
        }
    }

    if multiplier_given {
        if multiplier_exprs.is_empty() {
            params.push(("m".to_string(), ParametricValue::Resolved(multiplier)));
        } else {
            let expr = if multiplier == 1.0 {
                multiplier_exprs
                    .into_iter()
                    .map(|expr| format!("({expr})"))
                    .collect::<Vec<_>>()
                    .join("*")
            } else {
                format!(
                    "{multiplier}*{}",
                    multiplier_exprs
                        .into_iter()
                        .map(|expr| format!("({expr})"))
                        .collect::<Vec<_>>()
                        .join("*")
                )
            };
            params.push(("m".to_string(), ParametricValue::Expression(expr)));
        }
    }
    Ok(())
}

fn is_internal_routing_param(name: &str) -> bool {
    // Parser-owned metadata controls compatibility resolution and is not a
    // Verilog-A instance parameter. Generated devices must see only canonical
    // model/instance parameters from the user's netlist.
    name.eq_ignore_ascii_case(crate::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER)
}

fn is_generated_bjt_target(model_name: &str) -> bool {
    match_normalized(
        model_name,
        &[
            "HICUML0VA",
            "HICUML2VA",
            "BJT505_VA",
            "BJT505T_VA",
            "BJTD505_VA",
            "BJTD505T_VA",
            "VBIC13_4T",
            "VBIC_4T_ET_CF",
        ],
    )
}

fn is_bjt_multiplier_param(name: &str) -> bool {
    matches_model_type(name, &["AREA", "M", "MULT"])
}

fn exact_or_ground_padded_nodes(
    element: &Element,
    target: GeneratedTarget,
    max_implicit_ground_nodes: usize,
) -> Result<Vec<String>, SimulationError> {
    let expected = expected_terminal_count(element, target)?;
    if element.nodes.len() > expected {
        return Err(generated_terminal_error(
            element,
            target,
            element.nodes.len(),
            expected,
        ));
    }
    let missing = expected - element.nodes.len();
    if missing > max_implicit_ground_nodes {
        return Err(SimulationError::Circuit(format!(
            "{} '{}': generated Verilog-A model '{}' expects {} terminals, found {}; missing terminals cannot be inferred for .model routing",
            element_kind_name(element),
            element.name,
            target.model_name,
            expected,
            element.nodes.len()
        )));
    }
    let mut nodes = element.nodes.clone();
    nodes.extend(std::iter::repeat_n("0".to_string(), missing));
    Ok(nodes)
}

fn expected_terminal_count(
    element: &Element,
    target: GeneratedTarget,
) -> Result<usize, SimulationError> {
    builtins::node_count(target.model_name).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "{} '{}': generated Verilog-A model '{}' is not available in the built-in registry",
            element_kind_name(element),
            element.name,
            target.model_name
        ))
    })
}

fn generated_terminal_error(
    element: &Element,
    target: GeneratedTarget,
    found: usize,
    expected: usize,
) -> SimulationError {
    SimulationError::Circuit(format!(
        "{} '{}': generated Verilog-A model '{}' expects {} terminals, found {}",
        element_kind_name(element),
        element.name,
        target.model_name,
        expected,
        found
    ))
}

fn checked_model_level(
    element_kind: &str,
    model_name: &str,
    model: &ModelDef,
) -> Result<Option<i32>, SimulationError> {
    let params = model_params_upper_map(&model.params);
    checked_integer_model_level(
        element_kind,
        model_name,
        &model.name,
        &params,
        &model.expr_params,
        &model.string_params,
    )
}

fn element_kind_name(element: &Element) -> &'static str {
    match element.kind {
        ElementKind::Resistor { .. } => "Resistor",
        ElementKind::Diode { .. } => "Diode",
        ElementKind::Bjt { .. } => "BJT",
        ElementKind::Mosfet { .. } => "MOSFET",
        _ => "Device",
    }
}

fn is_resistor_model_type(model_type: &str) -> bool {
    matches_model_type(model_type, &["R", "RES", "RESISTOR"])
}

fn matches_model_type(model_type: &str, expected: &[&str]) -> bool {
    expected
        .iter()
        .any(|name| model_type.eq_ignore_ascii_case(name))
}

fn match_normalized(value: &str, candidates: &[&str]) -> bool {
    let normalized = normalize_generated_key(value);
    candidates
        .iter()
        .any(|candidate| normalized == normalize_generated_key(candidate))
}

fn normalize_generated_key(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_uppercase)
        .collect()
}
