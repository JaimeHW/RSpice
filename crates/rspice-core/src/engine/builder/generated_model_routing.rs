use super::version_metadata::parse_dotted_version_metadata;
use super::*;
use crate::device::veriloga_builtins::{
    BuiltinParameterAssignment, GeneratedTerminalCurrentAlias, builtins, instantiate_builtin_scoped,
};
use crate::netlist::{Element, ModelDef, ParametricValue};
use rspice_veriloga_runtime::{GeneratedParameterOrigin, GeneratedVerilogAParameterScope};

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
    spice_dialect: crate::config::SpiceDialect,
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
        spice_dialect,
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
    spice_dialect: crate::config::SpiceDialect,
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
        spice_dialect,
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
    spice_dialect: crate::config::SpiceDialect,
    temperature: f64,
    voltage_limiting_enabled: bool,
) -> Result<bool, SimulationError> {
    let target = generated_bjt_target(model_name, model_def, element.nodes.len(), spice_dialect)?;
    let Some(target) = target else {
        return Ok(false);
    };
    if !voltage_limiting_enabled {
        return Err(SimulationError::Circuit(format!(
            "DEVICE.VOLTLIM=0 is not implemented for generated BJT '{}' model '{}' family '{}'; only native legacy Gummel-Poon BJT limiting can currently be disabled",
            element.name, model_name, target.model_name
        )));
    }
    if !target.is_available() {
        return Err(SimulationError::Circuit(format!(
            "BJT '{}': model '{}' selects generated Verilog-A model '{}' under {:?} compatibility, but that exact model is not available in this build",
            element.name, model_name, target.model_name, spice_dialect
        )));
    }

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
        spice_dialect,
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
    spice_dialect: crate::config::SpiceDialect,
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
        spice_dialect,
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

/// Resolve explicit generated BJT types and dialect-owned Q-level devices.
///
/// Xyce 7.10 registers its ADMS VBIC 1.3 devices directly as Q LEVEL=11
/// (three electrical terminals, with an optional external thermal terminal)
/// and Q LEVEL=12 (four electrical terminals). Selecting those modules is
/// dialect semantics, not an opportunistic fallback. A build lacking that
/// exact generated module must fail explicitly rather than silently selecting
/// another BJT implementation. ngspice and BestAvailable selectors remain on
/// the native BJT policy, while explicit module types remain available in
/// every dialect.
fn generated_bjt_target(
    model_name: &str,
    model_def: Option<&ModelDef>,
    instance_terminal_count: usize,
    spice_dialect: crate::config::SpiceDialect,
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
    if spice_dialect != crate::config::SpiceDialect::Xyce
        || is_lpnp_bjt_model_type(&model.model_type)
        || resolve_bjt_type_from_model(&model.model_type).is_none()
    {
        return Ok(None);
    }

    Ok(match checked_model_level("BJT", model_name, model)? {
        Some(11) if instance_terminal_count >= 4 => Some(GeneratedTarget::new("vbic13_3t_et")),
        Some(11) => Some(GeneratedTarget::new("vbic13")),
        Some(12) => Some(GeneratedTarget::new("vbic13_4t")),
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
        // Xyce registers the canonical BSIM-SOI 4.6.1 ADMS device as MOS
        // LEVEL=70. LEVEL=10/57 select Xyce's native BSIM3-SOI front, while
        // 55/56/57 are RSpice's native per-family compatibility selectors;
        // they must not be shadowed merely because this generated artifact is
        // compiled into the same binary.
        Some(70) => Some(GeneratedTarget::new("bsimsoi_va")),
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
        // EKV3 302.00. Deliberately a type spelling and not a LEVEL: LEVEL=301
        // stays on the native 150 nm slice, which is what the VA-Models/Xyce
        // cards in the wild select. Without an entry here the generated model
        // would be reachable only by bare `X` instantiation, which refuses
        // model-scope parameters -- that is, only at its 262 defaults.
        .or_else(|| {
            match_normalized(model_type, &["EKV3", "EKV3_RF"])
                .then_some(GeneratedTarget::new("ekv3_rf"))
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
    spice_dialect: crate::config::SpiceDialect,
    temperature: f64,
) -> Result<(), SimulationError> {
    let keywords = generated_card_keywords(
        target,
        element,
        instance_params,
        deferred_params,
        &netlist.params,
    )?;
    let params = generated_params(
        target,
        model_def,
        instance_params,
        deferred_params,
        spice_dialect,
    )?;
    let Some(mut device) = instantiate_builtin_scoped(
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
    device
        .set_terminal_current_aliases(generated_card_current_aliases(target, element))
        .map_err(SimulationError::Circuit)?;
    device.set_temperature(temperature);
    device.set_initially_off(keywords.initial_off);
    circuit.add_generated_veriloga_device(device);
    Ok(())
}

/// SPICE instance keywords a card carries that are solver directives rather
/// than Verilog-A parameters.
///
/// A netlist that reaches a generated implementation must mean what it means on
/// the native one. These are the keys where the two vocabularies do not
/// overlap: no Verilog-A module declares them, so forwarding them as parameters
/// could only ever fail, and dropping them would change the answer in silence.
#[derive(Clone, Copy, Debug, Default)]
struct GeneratedCardKeywords {
    initial_off: bool,
}

/// What the generated route does with one instance-tail key.
enum GeneratedInstanceKeyRole {
    /// A parameter the selected module declares; forward it unchanged.
    Parameter,
    /// The `OFF` keyword, honoured by the generated-device adapter.
    InitiallyOff,
    /// Card semantics this generated module cannot express. The payload is the
    /// user-facing reason, which must name what the deck actually wrote.
    Unsupported(String),
}

/// The `IC=` vector components every junction-device card expands into.
///
/// The user writes `IC=0.7,3`; the parser splits it into these per-component
/// keys. They are read only by the `UIC` transient startup, which seeds native
/// device branch voltages directly, and a generated module has no such entry
/// point — so an error here has to name `IC=` and not the component key the
/// deck never contained.
const GENERATED_INSTANCE_IC_KEYS: &[&str] = &[
    "IC", "IC_VDS", "IC_VGS", "IC_VBS", "IC_VES", "IC_VPS", "IC_VBE", "IC_VCE",
];

/// Instance temperature keys the native junction-device cards accept.
const GENERATED_INSTANCE_TEMPERATURE_KEYS: &[&str] = &["TEMP", "DTEMP"];

fn generated_instance_key_role(target: GeneratedTarget, name: &str) -> GeneratedInstanceKeyRole {
    // `OFF` is a bare keyword in the card grammar and a solver directive in
    // every SPICE that has one. It is never a module parameter, so it is
    // claimed before the declaration lookup rather than after it.
    if matches_model_type(name, &["OFF"]) {
        return GeneratedInstanceKeyRole::InitiallyOff;
    }
    let declared = builtins::parameter_scope(target.model_name, name);
    if matches!(
        declared,
        Some(GeneratedVerilogAParameterScope::Instance | GeneratedVerilogAParameterScope::Dual)
    ) {
        return GeneratedInstanceKeyRole::Parameter;
    }
    if matches_model_type(name, GENERATED_INSTANCE_IC_KEYS) {
        return GeneratedInstanceKeyRole::Unsupported(format!(
            "instance IC= seeds a native device's branch voltages for a UIC transient start, and generated Verilog-A model '{}' has no such entry point; remove IC= from the card or use .IC on the nodes",
            target.model_name
        ));
    }
    if matches_model_type(name, GENERATED_INSTANCE_TEMPERATURE_KEYS) {
        let reason = if declared.is_some() {
            "declares it only for model-card assignment"
        } else {
            "does not declare it"
        };
        return GeneratedInstanceKeyRole::Unsupported(format!(
            "instance {} sets one device's operating temperature, and generated Verilog-A model '{}' {}; set the temperature on the .MODEL card or with .OPTIONS TEMP instead",
            name.to_ascii_uppercase(),
            target.model_name,
            reason
        ));
    }
    // Everything else is a parameter name. An undeclared one still fails, but
    // it fails at the instantiation site with the module and the exact key the
    // deck wrote, which is the message that key deserves.
    GeneratedInstanceKeyRole::Parameter
}

/// Read the card-level instance keywords off one element's tail.
///
/// This runs before parameter lowering so an unsupported keyword is reported
/// against the card the user wrote, with the element named, rather than
/// surfacing later as an unknown module parameter.
fn generated_card_keywords(
    target: GeneratedTarget,
    element: &Element,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
    param_ctx: &crate::netlist::ParamContext,
) -> Result<GeneratedCardKeywords, SimulationError> {
    let mut keywords = GeneratedCardKeywords::default();
    let unsupported = |reason: String| {
        SimulationError::Circuit(format!(
            "{} '{}': {}",
            element_kind_name(element),
            element.name,
            reason
        ))
    };
    for (name, value) in instance_params {
        if is_internal_routing_param(name) {
            continue;
        }
        match generated_instance_key_role(target, name) {
            GeneratedInstanceKeyRole::Parameter => {}
            GeneratedInstanceKeyRole::InitiallyOff => keywords.initial_off |= *value != 0.0,
            GeneratedInstanceKeyRole::Unsupported(reason) => {
                return Err(unsupported(reason));
            }
        }
    }
    for (name, expr) in deferred_params {
        match generated_instance_key_role(target, name) {
            GeneratedInstanceKeyRole::Parameter => {}
            GeneratedInstanceKeyRole::InitiallyOff => {
                let value =
                    crate::netlist::expr::eval_expression(expr, param_ctx).map_err(|error| {
                        SimulationError::Circuit(format!(
                            "{} '{}': cannot resolve instance keyword OFF={}: {}",
                            element_kind_name(element),
                            element.name,
                            expr,
                            error
                        ))
                    })?;
                keywords.initial_off |= value != 0.0;
            }
            GeneratedInstanceKeyRole::Unsupported(reason) => {
                return Err(unsupported(reason));
            }
        }
    }
    Ok(keywords)
}

const DIODE_CURRENT_ALIASES: &[GeneratedTerminalCurrentAlias] = &[GeneratedTerminalCurrentAlias {
    parameter: "id",
    terminal: "a",
}];
const EXTERNAL_E_BULK_CURRENT_ALIASES: &[GeneratedTerminalCurrentAlias] =
    &[GeneratedTerminalCurrentAlias {
        parameter: "ib",
        terminal: "e",
    }];
const BSIMIMG_CURRENT_ALIASES: &[GeneratedTerminalCurrentAlias] = &[
    GeneratedTerminalCurrentAlias {
        parameter: "ig",
        terminal: "fg",
    },
    GeneratedTerminalCurrentAlias {
        parameter: "ib",
        terminal: "bg",
    },
];

fn generated_card_current_aliases(
    target: GeneratedTarget,
    element: &Element,
) -> &'static [GeneratedTerminalCurrentAlias] {
    match element.kind {
        ElementKind::Diode { .. } => DIODE_CURRENT_ALIASES,
        ElementKind::Mosfet { .. } if match_normalized(target.model_name, &["BSIMIMG"]) => {
            BSIMIMG_CURRENT_ALIASES
        }
        ElementKind::Mosfet { .. }
            if match_normalized(
                target.model_name,
                &[
                    "BSIMCMG_VA",
                    "BSIMSOI_VA",
                    "HISIMSOI_VA__5BE18005",
                    "HISIMSOI_VA__242BC21D",
                    "HISIMSOI_VA__38074D06",
                    "HISIMSOTB_VA",
                ],
            ) =>
        {
            EXTERNAL_E_BULK_CURRENT_ALIASES
        }
        _ => &[],
    }
}

fn generated_params(
    target: GeneratedTarget,
    model_def: Option<&ModelDef>,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
    spice_dialect: crate::config::SpiceDialect,
) -> Result<Vec<BuiltinParameterAssignment>, SimulationError> {
    let model_count = model_def
        .map(|model| model.params.len() + model.expr_params.len() + model.string_params.len())
        .unwrap_or(0);
    let mut params =
        Vec::with_capacity(model_count + instance_params.len() + deferred_params.len());
    if let Some(model) = model_def {
        for (name, value) in &model.params {
            if should_pass_model_param(target, name) {
                if let Some(assignment) = generated_model_card_assignment(
                    target,
                    name,
                    ParametricValue::Resolved(*value),
                    spice_dialect,
                )? {
                    params.push(assignment);
                }
            }
        }
        for (name, expr) in &model.expr_params {
            if should_pass_model_param(target, name) {
                if let Some(assignment) = generated_model_card_assignment(
                    target,
                    name,
                    ParametricValue::Expression(expr.clone()),
                    spice_dialect,
                )? {
                    params.push(assignment);
                }
            }
        }
        for (name, value) in &model.string_params {
            if should_pass_model_param(target, name) {
                let Some(scope) = generated_model_parameter_scope(target, name, spice_dialect)?
                else {
                    continue;
                };
                if name.eq_ignore_ascii_case("VERSION") {
                    let version = parse_dotted_version_metadata(value).ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Generated Verilog-A model '{}' cannot route invalid VERSION=\"{}\" from .model '{}'; VERSION metadata must be a finite dotted numeric version such as 4.6.1",
                            target.model_name, value, model.name
                        ))
                    })?;
                    params.push(generated_model_card_assignment_for_scope(
                        target,
                        name,
                        ParametricValue::Resolved(version),
                        spice_dialect,
                        scope,
                    )?);
                    continue;
                }
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

fn generated_model_card_assignment(
    target: GeneratedTarget,
    name: &str,
    value: ParametricValue,
    spice_dialect: crate::config::SpiceDialect,
) -> Result<Option<BuiltinParameterAssignment>, SimulationError> {
    if generated_model_card_param_is_exact_zero_inert(target, name, &value) {
        return Ok(None);
    }
    let Some(scope) = generated_model_parameter_scope(target, name, spice_dialect)? else {
        return Ok(None);
    };
    Ok(Some(generated_model_card_assignment_for_scope(
        target,
        name,
        value,
        spice_dialect,
        scope,
    )?))
}

fn generated_model_card_param_is_exact_zero_inert(
    target: GeneratedTarget,
    name: &str,
    value: &ParametricValue,
) -> bool {
    target.model_name.eq_ignore_ascii_case("ekv_va")
        && matches!(value, ParametricValue::Resolved(value) if *value == 0.0)
        && matches_model_type(name, &["FNOIMOD", "NOIA", "CGSO", "CGDO", "CGBO"])
}

fn generated_model_parameter_scope(
    target: GeneratedTarget,
    name: &str,
    spice_dialect: crate::config::SpiceDialect,
) -> Result<Option<GeneratedVerilogAParameterScope>, SimulationError> {
    let Some(scope) = builtins::parameter_scope(target.model_name, name) else {
        if spice_dialect == crate::config::SpiceDialect::Xyce {
            // Xyce warns and ignores model-card names that the selected ADMS
            // device does not declare. Preserve that compatibility behavior
            // without inventing aliases or weakening canonical assignments.
            log::warn!(
                "Generated Verilog-A model '{}' has no declared model parameter '{}'; parameter ignored in Xyce compatibility mode",
                target.model_name,
                name
            );
            return Ok(None);
        }
        return Err(SimulationError::Circuit(format!(
            "Generated Verilog-A model '{}' has no unambiguous declaration-scope metadata for model-card parameter '{}'",
            target.model_name, name
        )));
    };
    Ok(Some(scope))
}

fn generated_model_card_assignment_for_scope(
    target: GeneratedTarget,
    name: &str,
    value: ParametricValue,
    spice_dialect: crate::config::SpiceDialect,
    scope: GeneratedVerilogAParameterScope,
) -> Result<BuiltinParameterAssignment, SimulationError> {
    let origin = match scope {
        GeneratedVerilogAParameterScope::Model | GeneratedVerilogAParameterScope::Dual => {
            GeneratedParameterOrigin::ModelCard
        }
        GeneratedVerilogAParameterScope::Instance
            if spice_dialect == crate::config::SpiceDialect::Xyce =>
        {
            // Xyce accepts canonical instance parameters on a .MODEL card as
            // per-instance defaults. These assignments precede explicit
            // instance-card assignments below, so the explicit value retains
            // normal SPICE override precedence.
            GeneratedParameterOrigin::Instance
        }
        GeneratedVerilogAParameterScope::Instance => {
            return Err(SimulationError::Circuit(format!(
                "Generated Verilog-A model '{}' parameter '{}' is declared instance-only; model-card defaults for instance parameters require the Xyce compatibility dialect",
                target.model_name, name
            )));
        }
    };
    Ok(BuiltinParameterAssignment::new(name, value, origin))
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
    params: &mut Vec<BuiltinParameterAssignment>,
) {
    if !needs_inferred_generated_mos_type(target.model_name) {
        return;
    }
    if params
        .iter()
        .any(|assignment| assignment.name.eq_ignore_ascii_case("TYPE"))
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
    params.push(BuiltinParameterAssignment::model_card(
        "TYPE",
        ParametricValue::Resolved(type_value),
    ));
}

fn needs_inferred_generated_mos_type(model_name: &str) -> bool {
    match_normalized(model_name, &["EKV_VA", "BSIMSOI_VA", "BSIMCMG_VA"])
}

fn append_generated_bjt_polarity_param(
    target: GeneratedTarget,
    model_def: Option<&ModelDef>,
    params: &mut Vec<BuiltinParameterAssignment>,
) {
    if !needs_inferred_generated_bjt_type(target.model_name) {
        return;
    }
    if params
        .iter()
        .any(|assignment| matches_model_type(&assignment.name, &["TYPE", "NPN", "PNP"]))
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
    params.push(BuiltinParameterAssignment::model_card(
        "type",
        ParametricValue::Resolved(type_value),
    ));
}

fn needs_inferred_generated_bjt_type(model_name: &str) -> bool {
    match_normalized(
        model_name,
        &["VBIC13", "VBIC13_3T_ET", "VBIC13_4T", "VBIC_4T_ET_CF"],
    )
}

fn append_generated_instance_params(
    target: GeneratedTarget,
    params: &mut Vec<BuiltinParameterAssignment>,
    instance_params: &[(String, f64)],
    deferred_params: &[(String, String)],
) -> Result<(), SimulationError> {
    if !is_generated_bjt_target(target.model_name) {
        for (name, value) in instance_params {
            if is_internal_routing_param(name) || is_generated_card_keyword(target, name) {
                continue;
            }
            params.push(BuiltinParameterAssignment::instance(
                name,
                ParametricValue::Resolved(*value),
            ));
        }
        for (name, expr) in deferred_params {
            if is_generated_card_keyword(target, name) {
                continue;
            }
            params.push(BuiltinParameterAssignment::instance(
                name,
                ParametricValue::Expression(expr.clone()),
            ));
        }
        return Ok(());
    }

    let mut multiplier = 1.0;
    let mut multiplier_given = false;
    let mut multiplier_exprs = Vec::new();
    for (name, value) in instance_params {
        if is_internal_routing_param(name) || is_generated_card_keyword(target, name) {
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
            params.push(BuiltinParameterAssignment::instance(
                name,
                ParametricValue::Resolved(*value),
            ));
        }
    }
    for (name, expr) in deferred_params {
        if is_generated_card_keyword(target, name) {
            continue;
        } else if is_bjt_multiplier_param(name) {
            multiplier_exprs.push(expr.clone());
            multiplier_given = true;
        } else {
            params.push(BuiltinParameterAssignment::instance(
                name,
                ParametricValue::Expression(expr.clone()),
            ));
        }
    }

    if multiplier_given {
        if multiplier_exprs.is_empty() {
            params.push(BuiltinParameterAssignment::instance(
                "m",
                ParametricValue::Resolved(multiplier),
            ));
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
            params.push(BuiltinParameterAssignment::instance(
                "m",
                ParametricValue::Expression(expr),
            ));
        }
    }
    Ok(())
}

/// Whether the adapter, and not the generated module, owns this key.
///
/// [`generated_card_keywords`] has already read it off the card and rejected
/// the ones the module cannot honour, so anything still classified as card
/// semantics here must not also reach `apply_parameters`.
fn is_generated_card_keyword(target: GeneratedTarget, name: &str) -> bool {
    matches!(
        generated_instance_key_role(target, name),
        GeneratedInstanceKeyRole::InitiallyOff
    )
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

#[cfg(test)]
mod exact_zero_compatibility_tests {
    use super::*;

    #[test]
    fn ekv_exact_zero_inert_parameters_are_narrowly_scoped() {
        let target = GeneratedTarget::new("ekv_va");
        for name in ["FNOIMOD", "NOIA", "CGSO", "CGDO", "CGBO"] {
            assert!(generated_model_card_param_is_exact_zero_inert(
                target,
                name,
                &ParametricValue::Resolved(0.0)
            ));
        }

        assert!(!generated_model_card_param_is_exact_zero_inert(
            target,
            "FNOIMOD",
            &ParametricValue::Resolved(1.0)
        ));
        assert!(!generated_model_card_param_is_exact_zero_inert(
            target,
            "FNOIMOD",
            &ParametricValue::Expression("0".to_string())
        ));
        assert!(!generated_model_card_param_is_exact_zero_inert(
            target,
            "UNKNOWN",
            &ParametricValue::Resolved(0.0)
        ));
        assert!(!generated_model_card_param_is_exact_zero_inert(
            GeneratedTarget::new("vbic13"),
            "FNOIMOD",
            &ParametricValue::Resolved(0.0)
        ));
    }
}

#[cfg(all(test, feature = "veriloga-model-ekv-va"))]
mod card_keyword_tests {
    use super::*;

    const EKV: GeneratedTarget = GeneratedTarget::new("ekv_va");

    fn mos_card(tail: &str) -> (Netlist, Element) {
        let netlist = Netlist::parse(&format!(
            "generated EKV26 instance tail admission\n\
             M1 d g 0 0 n w=10u l=1u{tail}\n\
             .MODEL n NMOS LEVEL=260\n\
             .END\n"
        ))
        .expect("EKV26 instance-tail fixture parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("M1"))
            .expect("fixture contains M1")
            .clone();
        (netlist, element)
    }

    fn card_tail(element: &Element) -> (&[(String, f64)], &[(String, String)]) {
        let ElementKind::Mosfet {
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            panic!("M1 is a MOSFET");
        };
        (instance_params, deferred_params)
    }

    fn keywords(tail: &str) -> Result<GeneratedCardKeywords, SimulationError> {
        let (netlist, element) = mos_card(tail);
        let (instance_params, deferred_params) = card_tail(&element);
        generated_card_keywords(
            EKV,
            &element,
            instance_params,
            deferred_params,
            &netlist.params,
        )
    }

    #[test]
    fn off_is_read_off_the_card_and_never_lowered_to_a_module_parameter() {
        assert!(
            !keywords("")
                .expect("a bare card carries no keywords")
                .initial_off
        );
        assert!(
            keywords(" OFF")
                .expect("the generated route accepts OFF")
                .initial_off
        );
        assert!(
            !keywords(" OFF=0")
                .expect("an explicit OFF=0 is accepted")
                .initial_off,
            "OFF=0 leaves the instance active, as it does on the native route"
        );

        let (netlist, element) = mos_card(" OFF");
        let (instance_params, deferred_params) = card_tail(&element);
        let assignments = generated_params(
            EKV,
            find_model_def(&netlist, "n"),
            instance_params,
            deferred_params,
            crate::config::SpiceDialect::BestAvailable,
        )
        .expect("OFF does not reach the module's parameter table");
        assert!(
            assignments
                .iter()
                .all(|assignment| !assignment.name.eq_ignore_ascii_case("OFF")),
            "OFF is a solver directive; lowering it as a parameter is what made \
             a standard SPICE deck fail on this route"
        );
    }

    #[test]
    fn card_keywords_the_module_cannot_honour_are_refused_by_name() {
        for (tail, fragments) in [
            (" IC=0.2,0.3", ["M1", "IC=", "ekv_va"]),
            (" TEMP=85", ["M1", "TEMP", "ekv_va"]),
            (" DTEMP=10", ["M1", "DTEMP", "ekv_va"]),
        ] {
            let message = keywords(tail)
                .expect_err("ekv_va cannot honour this card keyword")
                .to_string();
            for fragment in fragments {
                assert!(
                    message.contains(fragment),
                    "'{tail}' must be refused naming '{fragment}', got: {message}"
                );
            }
        }
    }

    #[test]
    fn an_undeclared_instance_parameter_still_fails_naming_the_module_and_the_key() {
        let (netlist, element) = mos_card(" nrd=3");
        let (instance_params, deferred_params) = card_tail(&element);
        generated_card_keywords(
            EKV,
            &element,
            instance_params,
            deferred_params,
            &netlist.params,
        )
        .expect("NRD is a parameter name, not a card keyword");
        let params = generated_params(
            EKV,
            find_model_def(&netlist, "n"),
            instance_params,
            deferred_params,
            crate::config::SpiceDialect::BestAvailable,
        )
        .expect("NRD lowers as an ordinary instance parameter");

        let mut circuit = CircuitData::new();
        let nodes = ["d", "g", "0", "0"].map(str::to_string);
        let message = instantiate_builtin_scoped(
            "ekv_va",
            "M1",
            &nodes,
            &params,
            &netlist.params,
            &mut circuit,
        )
        .expect_err("ekv_va declares no NRD")
        .to_string();
        assert!(
            message.contains("NRD") && message.contains("ekv_va"),
            "an undeclared parameter must name the module and the key, got: {message}"
        );
    }
}

#[cfg(all(test, feature = "veriloga-model-vbic13"))]
mod tests {
    use super::*;

    fn vbic_scope_fixture() -> (Netlist, Vec<(String, f64)>, Vec<(String, String)>) {
        let netlist = Netlist::parse(
            "generated VBIC parameter-scope lowering\n\
             Q1 c b e model sw_noise=1\n\
             .MODEL model NPN LEVEL=11 sw_noise=0\n\
             .END\n",
        )
        .expect("VBIC scope fixture parses");
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("Q1"))
            .expect("fixture contains Q1");
        let ElementKind::Bjt {
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            panic!("Q1 is a BJT");
        };
        (
            netlist.clone(),
            instance_params.clone(),
            deferred_params.clone(),
        )
    }

    #[test]
    fn xyce_model_instance_defaults_keep_explicit_instance_precedence() {
        let (netlist, instance_params, deferred_params) = vbic_scope_fixture();
        let model = find_model_def(&netlist, "model").expect("fixture model exists");
        let assignments = generated_params(
            GeneratedTarget::new("VBIC13"),
            Some(model),
            &instance_params,
            &deferred_params,
            crate::config::SpiceDialect::Xyce,
        )
        .expect("Xyce lowers model-card instance defaults");
        let sw_noise = assignments
            .iter()
            .filter(|assignment| assignment.name.eq_ignore_ascii_case("SW_NOISE"))
            .collect::<Vec<_>>();
        assert_eq!(sw_noise.len(), 2);
        assert_eq!(sw_noise[0].origin, GeneratedParameterOrigin::Instance);
        assert_eq!(sw_noise[1].origin, GeneratedParameterOrigin::Instance);
        assert!(matches!(sw_noise[0].value, ParametricValue::Resolved(0.0)));
        assert!(matches!(sw_noise[1].value, ParametricValue::Resolved(1.0)));
    }

    #[test]
    fn non_xyce_model_instance_defaults_fail_closed() {
        let (netlist, instance_params, deferred_params) = vbic_scope_fixture();
        let model = find_model_def(&netlist, "model").expect("fixture model exists");
        for dialect in [
            crate::config::SpiceDialect::BestAvailable,
            crate::config::SpiceDialect::Ngspice,
        ] {
            let error = generated_params(
                GeneratedTarget::new("VBIC13"),
                Some(model),
                &instance_params,
                &deferred_params,
                dialect,
            )
            .expect_err("non-Xyce model-card instance defaults must fail closed");
            let message = error.to_string();
            assert!(
                message.contains("SW_NOISE")
                    && message.contains("instance-only")
                    && message.contains("Xyce"),
                "unexpected scope error: {message}"
            );
        }
    }

    #[test]
    fn xyce_ignores_only_undeclared_generated_model_parameters() {
        let netlist = Netlist::parse(
            "generated VBIC unknown model-card compatibility\n\
             Q1 c b e model\n\
             .MODEL model NPN LEVEL=11 rb=4 rc=5 legacy_label=\"ignored\" rbx=2\n\
             .END\n",
        )
        .expect("VBIC compatibility fixture parses");
        let model = find_model_def(&netlist, "model").expect("fixture model exists");
        let assignments = generated_params(
            GeneratedTarget::new("VBIC13"),
            Some(model),
            &[],
            &[],
            crate::config::SpiceDialect::Xyce,
        )
        .expect("Xyce ignores undeclared ADMS model parameters");

        assert!(
            assignments
                .iter()
                .all(|assignment| !matches_model_type(&assignment.name, &["RB", "RC"])),
            "undeclared legacy BJT parameters must not be fabricated as canonical VBIC parameters"
        );
        let rbx = assignments
            .iter()
            .find(|assignment| assignment.name.eq_ignore_ascii_case("RBX"))
            .expect("declared canonical RBX remains assigned");
        assert_eq!(rbx.origin, GeneratedParameterOrigin::ModelCard);
        assert!(matches!(rbx.value, ParametricValue::Resolved(2.0)));

        let error = generated_params(
            GeneratedTarget::new("VBIC13"),
            Some(model),
            &[],
            &[],
            crate::config::SpiceDialect::BestAvailable,
        )
        .expect_err("non-Xyce routing must continue to reject undeclared parameters");
        let message = error.to_string();
        assert!(
            message.contains("RB") || message.contains("RC"),
            "unexpected undeclared-parameter error: {message}"
        );
    }

    #[test]
    fn xyce_q_levels_select_the_exact_registered_vbic_variant() {
        for (level, terminals, expected) in [
            (11, 3, "vbic13"),
            (11, 4, "vbic13_3t_et"),
            (12, 4, "vbic13_4t"),
        ] {
            let netlist = Netlist::parse(&format!(
                "Xyce VBIC level routing\nQ1 c b e model\n.MODEL model NPN LEVEL={level}\n.END\n"
            ))
            .expect("VBIC level fixture parses");
            let model = find_model_def(&netlist, "model").expect("fixture model exists");
            let target = generated_bjt_target(
                "model",
                Some(model),
                terminals,
                crate::config::SpiceDialect::Xyce,
            )
            .expect("Xyce level resolves")
            .expect("Xyce level selects a generated model");
            assert_eq!(target.model_name, expected);
        }
    }

    #[test]
    fn non_xyce_and_unregistered_q_levels_do_not_select_generated_vbic() {
        for (level, dialect) in [
            (11, crate::config::SpiceDialect::BestAvailable),
            (11, crate::config::SpiceDialect::Ngspice),
            (13, crate::config::SpiceDialect::Xyce),
        ] {
            let netlist = Netlist::parse(&format!(
                "non-Xyce VBIC level routing\nQ1 c b e model\n.MODEL model NPN LEVEL={level}\n.END\n"
            ))
            .expect("VBIC level fixture parses");
            let model = find_model_def(&netlist, "model").expect("fixture model exists");
            assert!(
                generated_bjt_target("model", Some(model), 3, dialect)
                    .expect("level classification succeeds")
                    .is_none(),
                "LEVEL={level} under {dialect:?} must not select a generated VBIC implementation"
            );
        }
    }
}
