//! Circuit builder - constructs CircuitData from Netlist
//!
//! This module handles the conversion from parsed netlist elements
//! to the runtime circuit representation.

use super::{extract_dc_value, Engine, SimulationError};
use crate::netlist::{flatten_netlist, ElementKind};
use crate::{CircuitData, Netlist};
#[cfg(feature = "veriloga")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "veriloga")]
use std::io::{Read, Write};
#[cfg(feature = "veriloga")]
use std::path::{Path, PathBuf};
#[cfg(all(test, feature = "veriloga"))]
use std::sync::Mutex;
use std::sync::OnceLock;
#[cfg(feature = "veriloga")]
use std::sync::RwLock;
#[cfg(feature = "veriloga")]
use std::time::{Duration, Instant};

mod model_resolution;
use model_resolution::*;
#[cfg(feature = "veriloga")]
mod veriloga_cache;
#[cfg(feature = "veriloga")]
pub use veriloga_cache::{
    clear_veriloga_cache, prune_veriloga_cache, register_precompiled_veriloga_model,
    register_precompiled_veriloga_model_with_dependencies, veriloga_cache_entries,
    veriloga_cache_stats, VerilogACacheEntry, VerilogACachePruneReport, VerilogACacheStats,
};
#[cfg(feature = "veriloga")]
use veriloga_cache::{normalize_model_key, resolve_cached_or_compile_veriloga};

/// Embedded transistor model library used for fallback model resolution.
const BUILTIN_TRANSISTOR_LIB: &str = include_str!("../../../../models/spice/transistor.lib");

/// Lazily parsed builtin BJT model parameter map (MODEL_NAME -> params).
fn builtin_bjt_model_map() -> &'static HashMap<String, HashMap<String, f64>> {
    static BJT_MODELS: OnceLock<HashMap<String, HashMap<String, f64>>> = OnceLock::new();
    BJT_MODELS.get_or_init(|| {
        let mut map = HashMap::new();
        let Ok(netlist) = crate::netlist::parse_netlist(BUILTIN_TRANSISTOR_LIB) else {
            log::warn!("Failed to parse embedded transistor library for BJT fallback models");
            return map;
        };

        for model in netlist.models {
            if model.model_type.eq_ignore_ascii_case("NPN")
                || model.model_type.eq_ignore_ascii_case("PNP")
            {
                map.insert(
                    model.name.to_uppercase(),
                    model.params.into_iter().collect(),
                );
            }
        }
        map
    })
}

impl Engine {
    /// Build circuit from netlist (flattens subcircuits first)
    pub fn build_circuit(&self, netlist: &Netlist) -> Result<CircuitData, SimulationError> {
        let mut circuit = CircuitData::new();

        // Flatten subcircuit instances into top-level elements
        let flat_elements = flatten_netlist(netlist)
            .map_err(|e| SimulationError::Netlist(format!("Flattening error: {}", e)))?;

        // Debug: log all elements
        log::info!("Building circuit with {} elements:", flat_elements.len());
        for element in &flat_elements {
            log::info!(
                "  Element: {} nodes={:?} kind={:?}",
                element.name,
                element.nodes,
                element.kind
            );
        }

        #[cfg(feature = "veriloga")]
        let mut veriloga_models: HashMap<String, rspice_veriloga::CompiledModel> = HashMap::new();

        // Load and cache Verilog-A models referenced by .VERILOGA directives.
        #[cfg(feature = "veriloga")]
        {
            for include in &netlist.veriloga_includes {
                let model = resolve_cached_or_compile_veriloga(&include.file_path)?;

                let model_key = normalize_model_key(model.name.as_str());
                veriloga_models
                    .entry(model_key)
                    .or_insert_with(|| model.clone());

                if let Some(alias) = include.model_name.as_deref() {
                    veriloga_models
                        .entry(normalize_model_key(alias))
                        .or_insert_with(|| model.clone());
                }

                if let Some(stem) = include.file_path.file_stem().and_then(|s| s.to_str()) {
                    veriloga_models
                        .entry(normalize_model_key(stem))
                        .or_insert_with(|| model.clone());
                }

                log::info!(
                    "Loaded Verilog-A model '{}' from {}",
                    model.name,
                    include.file_path.display()
                );
            }
        }

        for element in &flat_elements {
            match &element.kind {
                ElementKind::Resistor {
                    value,
                    model,
                    instance_params,
                } => {
                    let resistance = resolve_resistor_instance_value(
                        netlist,
                        &element.name,
                        *value,
                        model.as_deref(),
                        instance_params,
                    )?;
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    circuit
                        .resistors
                        .add(element.name.clone(), np, nn, resistance);
                }
                ElementKind::Capacitor { value, .. } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    circuit.capacitors.add(element.name.clone(), np, nn, *value);
                }
                ElementKind::Inductor { value, .. } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    circuit
                        .inductors
                        .add(element.name.clone(), np, nn, branch, *value);
                }
                ElementKind::JilesAthertonInductor {
                    value,
                    model,
                    initial_current,
                } => {
                    if !value.is_finite() || *value <= 0.0 {
                        return Err(SimulationError::Circuit(format!(
                            "Jiles-Atherton inductor '{}' has invalid inductance value {}",
                            element.name, value
                        )));
                    }

                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);

                    let model_def = find_model_def(netlist, model).ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Jiles-Atherton inductor '{}' references unknown model '{}'",
                            element.name, model
                        ))
                    })?;
                    ensure_model_type(
                        "Jiles-Atherton inductor",
                        &element.name,
                        model,
                        model_def,
                        &["CORE", "JA", "JILES", "JILESATHERTON"],
                    )?;

                    let params = resolve_jiles_atherton_model_params(model_def, *value)?;
                    let mut ja = crate::device::passive::JilesAthertonInductor::new(
                        element.name.clone(),
                        np,
                        nn,
                    )
                    .with_params(params);
                    if let Some(ic) = *initial_current {
                        ja.set_initial_current(ic);
                    }

                    let effective_l = ja.effective_inductance();
                    let runtime_l = if effective_l.is_finite() && effective_l > 0.0 {
                        effective_l
                    } else {
                        *value
                    };

                    if let Some(ic) = *initial_current {
                        circuit.inductors.add_with_ic(
                            element.name.clone(),
                            np,
                            nn,
                            branch,
                            runtime_l,
                            ic,
                        );
                    } else {
                        circuit
                            .inductors
                            .add(element.name.clone(), np, nn, branch, runtime_l);
                    }

                    let inductor_index = circuit.inductors.len().saturating_sub(1);
                    circuit.add_jiles_atherton_inductor(inductor_index, branch, ja);
                }
                ElementKind::VoltageSource(spec) => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    let dc_value = extract_dc_value(spec);
                    let (ac_mag, ac_phase) = super::extract_ac_value(spec);
                    log::debug!(
                        "VoltageSource {}: DC={}, AC_mag={}, AC_phase={}, spec={:?}",
                        element.name,
                        dc_value,
                        ac_mag,
                        ac_phase,
                        spec
                    );
                    // Clone spec for transient analysis if it's a time-varying source
                    let transient_spec = match spec {
                        crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::Exp { .. } => Some(spec.clone()),
                        _ => None,
                    };
                    circuit.voltage_sources.add_with_ac_and_spec(
                        element.name.clone(),
                        np,
                        nn,
                        branch,
                        dc_value,
                        ac_mag,
                        ac_phase,
                        transient_spec,
                    );
                }
                ElementKind::CurrentSource(spec) => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let dc_value = extract_dc_value(spec);
                    let (ac_mag, ac_phase) = super::extract_ac_value(spec);
                    circuit.current_sources.add_with_ac(
                        element.name.clone(),
                        np,
                        nn,
                        dc_value,
                        ac_mag,
                        ac_phase,
                    );
                }
                ElementKind::Diode { model } => {
                    let anode = circuit.get_or_create_node(&element.nodes[0]);
                    let cathode = circuit.get_or_create_node(&element.nodes[1]);
                    let mut diode = crate::device::Diode::new(element.name.clone(), anode, cathode);

                    // Look up model and apply parameters
                    let model_def = find_model_def(netlist, model);
                    if let Some(device_model) = model_def {
                        ensure_model_type(
                            "Diode",
                            &element.name,
                            model,
                            device_model,
                            &["D", "DIODE"],
                        )?;
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();
                        diode = diode.with_model_params(&params_map);
                    }

                    circuit.diodes.add(diode);
                }
                ElementKind::Bjt { model, bjt_type } => {
                    let collector = circuit.get_or_create_node(&element.nodes[0]);
                    let base = circuit.get_or_create_node(&element.nodes[1]);
                    let emitter = circuit.get_or_create_node(&element.nodes[2]);

                    // Resolve polarity from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let resolved_bjt_type = if let Some(device_model) = model_def {
                        resolve_bjt_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "BJT '{}' references model '{}' with incompatible type '{}'; expected NPN or PNP",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *bjt_type
                    };

                    let mut bjt = match resolved_bjt_type {
                        crate::netlist::BjtType::Npn => crate::device::Bjt::new_npn(
                            element.name.clone(),
                            collector,
                            base,
                            emitter,
                        ),
                        crate::netlist::BjtType::Pnp => crate::device::Bjt::new_pnp(
                            element.name.clone(),
                            collector,
                            base,
                            emitter,
                        ),
                    };

                    // Look up model and apply parameters
                    if let Some(device_model) = model_def {
                        // Convert Vec<(String, f64)> to HashMap for with_params
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();
                        bjt = bjt.with_params(&params_map);
                    } else if let Some(params_map) =
                        builtin_bjt_model_map().get(&model.to_uppercase())
                    {
                        // Fallback to embedded transistor library models when no
                        // explicit .MODEL card is present in the parsed netlist.
                        bjt = bjt.with_params(params_map);
                        log::debug!(
                            "Applied embedded BJT fallback model '{}' to {}",
                            model,
                            element.name
                        );
                    }

                    circuit.bjts.add(bjt);
                }
                ElementKind::Mosfet { model, mos_type } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);
                    let bulk = circuit.get_or_create_node(&element.nodes[3]);

                    // Resolve NMOS/PMOS from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let resolved_mos_type = if let Some(device_model) = model_def {
                        resolve_mos_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "MOSFET '{}' references model '{}' with incompatible type '{}'; expected NMOS or PMOS",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *mos_type
                    };

                    let mut mosfet = match resolved_mos_type {
                        crate::netlist::MosType::Nmos => crate::device::Mosfet::new_nmos(
                            element.name.clone(),
                            drain,
                            gate,
                            source,
                            bulk,
                        ),
                        crate::netlist::MosType::Pmos => crate::device::Mosfet::new_pmos(
                            element.name.clone(),
                            drain,
                            gate,
                            source,
                            bulk,
                        ),
                    };

                    // Look up model and apply parameters including LEVEL
                    if let Some(device_model) = model_def {
                        // Convert Vec<(String, f64)> to HashMap for with_params
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();

                        // Extract LEVEL from params (default to 1)
                        let level = params_map.get("LEVEL").copied().unwrap_or(1.0) as i32;
                        mosfet = mosfet.with_level(level);

                        // Apply all model parameters (VTO, KP, GAMMA, KC, NC, etc.)
                        mosfet = mosfet.with_params(&params_map);
                    }

                    circuit.mosfets.add(mosfet);
                }
                ElementKind::Jfet { model, jfet_type } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);

                    // Resolve NJF/PJF from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let resolved_jfet_type = if let Some(device_model) = model_def {
                        resolve_jfet_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "JFET '{}' references model '{}' with incompatible type '{}'; expected NJF or PJF",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *jfet_type
                    };

                    let mut jfet = match resolved_jfet_type {
                        crate::netlist::JfetType::Njf => {
                            crate::device::Jfet::njf(&element.name, drain, gate, source)
                        }
                        crate::netlist::JfetType::Pjf => {
                            crate::device::Jfet::pjf(&element.name, drain, gate, source)
                        }
                    };

                    // Look up model and apply parameters
                    if let Some(device_model) = model_def {
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();
                        jfet = jfet.with_model_params(&params_map);
                    }

                    // Realistic extrinsic JFET series resistances (RD/RS) are modeled by
                    // inserting explicit linear resistors and connecting the intrinsic JFET
                    // to generated internal drain/source nodes.
                    let rd = if jfet.params.rd.is_finite() && jfet.params.rd > 0.0 {
                        jfet.params.rd
                    } else {
                        0.0
                    };
                    let rs = if jfet.params.rs.is_finite() && jfet.params.rs > 0.0 {
                        jfet.params.rs
                    } else {
                        0.0
                    };

                    if rd > 0.0 {
                        let dint_name = format!("{}.__dint", element.name);
                        let dint = circuit.get_or_create_node(&dint_name);
                        let rd_name = format!("{}.__rd", element.name);
                        circuit.resistors.add(rd_name, drain, dint, rd);
                        jfet.drain = dint;
                        jfet.params.rd = 0.0;
                    }
                    if rs > 0.0 {
                        let sint_name = format!("{}.__sint", element.name);
                        let sint = circuit.get_or_create_node(&sint_name);
                        let rs_name = format!("{}.__rs", element.name);
                        circuit.resistors.add(rs_name, source, sint, rs);
                        jfet.source = sint;
                        jfet.params.rs = 0.0;
                    }

                    circuit.jfets.push(jfet);
                }
                // MESFET (GaAs FET) - treat as JFET for now since physics are similar
                ElementKind::Mesfet { model, mesfet_type } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);
                    // MESFET uses similar equations to JFET - treat as N-channel JFET

                    // Resolve NMF/PMF from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let resolved_mesfet_type = if let Some(device_model) = model_def {
                        resolve_mesfet_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "MESFET '{}' references model '{}' with incompatible type '{}'; expected NMF or PMF",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *mesfet_type
                    };

                    let mut jfet = match resolved_mesfet_type {
                        crate::netlist::MesfetType::Nmf => {
                            crate::device::Jfet::njf(&element.name, drain, gate, source)
                        }
                        crate::netlist::MesfetType::Pmf => {
                            crate::device::Jfet::pjf(&element.name, drain, gate, source)
                        }
                    };

                    // Look up model and apply parameters
                    if let Some(device_model) = model_def {
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();
                        jfet = jfet.with_model_params(&params_map);
                    }

                    // Apply the same RD/RS extrinsic-node expansion for MESFET aliases.
                    let rd = if jfet.params.rd.is_finite() && jfet.params.rd > 0.0 {
                        jfet.params.rd
                    } else {
                        0.0
                    };
                    let rs = if jfet.params.rs.is_finite() && jfet.params.rs > 0.0 {
                        jfet.params.rs
                    } else {
                        0.0
                    };

                    if rd > 0.0 {
                        let dint_name = format!("{}.__dint", element.name);
                        let dint = circuit.get_or_create_node(&dint_name);
                        let rd_name = format!("{}.__rd", element.name);
                        circuit.resistors.add(rd_name, drain, dint, rd);
                        jfet.drain = dint;
                        jfet.params.rd = 0.0;
                    }
                    if rs > 0.0 {
                        let sint_name = format!("{}.__sint", element.name);
                        let sint = circuit.get_or_create_node(&sint_name);
                        let rs_name = format!("{}.__rs", element.name);
                        circuit.resistors.add(rs_name, source, sint, rs);
                        jfet.source = sint;
                        jfet.params.rs = 0.0;
                    }

                    circuit.jfets.push(jfet);
                }
                // Controlled sources
                ElementKind::Vcvs {
                    gain,
                    control_nodes,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(&control_nodes.0);
                    let cn = circuit.get_or_create_node(&control_nodes.1);
                    let branch = circuit.allocate_branch();
                    circuit
                        .vcvs
                        .add(element.name.clone(), np, nn, cp, cn, branch, *gain);
                }
                ElementKind::Vccs {
                    transconductance,
                    control_nodes,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(&control_nodes.0);
                    let cn = circuit.get_or_create_node(&control_nodes.1);
                    circuit
                        .vccs
                        .add(element.name.clone(), np, nn, cp, cn, *transconductance);
                }
                ElementKind::Cccs {
                    gain,
                    control_element,
                } => {
                    // CCCS needs the branch of a controlling voltage source
                    // Register for deferred resolution after all elements are added
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cccs_idx = circuit.cccs.len();
                    // Add with placeholder branch (will be resolved later)
                    circuit.cccs.add(element.name.clone(), np, nn, 0, *gain);
                    circuit.add_cccs_pending(cccs_idx, control_element.clone());
                }
                ElementKind::Ccvs {
                    transresistance,
                    control_element,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    let ccvs_idx = circuit.ccvs.len();
                    // Add with placeholder control branch (will be resolved later)
                    circuit
                        .ccvs
                        .add(element.name.clone(), np, nn, branch, 0, *transresistance);
                    circuit.add_ccvs_pending(ccvs_idx, control_element.clone());
                }
                // Behavioral sources
                ElementKind::BehavioralVoltage { expression } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);

                    let bvs = crate::device::BehavioralVoltageSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        branch,
                        expression,
                    );
                    circuit.behavioral_sources.add_voltage(bvs);
                }
                ElementKind::BehavioralCurrent { expression } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);

                    let bcs = crate::device::BehavioralCurrentSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        expression,
                    );
                    circuit.behavioral_sources.add_current(bcs);
                }
                // Flattened tree leaves external subcircuit-backed devices here
                // (for example, Verilog-A model instances).
                #[cfg(feature = "veriloga")]
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } => {
                    if let Some(model) = veriloga_models.get(&normalize_model_key(subckt_name)) {
                        if element.nodes.len() != model.num_terminals {
                            return Err(SimulationError::Circuit(format!(
                                "Verilog-A instance '{}' expects {} terminals for model '{}', found {}",
                                element.name,
                                model.num_terminals,
                                subckt_name,
                                element.nodes.len()
                            )));
                        }

                        let mut node_ids = Vec::with_capacity(model.num_terminals);
                        for node_name in &element.nodes {
                            node_ids.push(if node_name.eq_ignore_ascii_case("0") {
                                0
                            } else {
                                circuit.get_or_create_node(node_name)
                            });
                        }

                        let mut device = crate::device::veriloga::VerilogADevice::new(
                            element.name.clone(),
                            model.clone(),
                            &node_ids,
                        );

                        // Allocate global circuit node indices for internal Verilog-A nodes.
                        if device.num_internal_nodes() > 0 {
                            let mut internal_nodes =
                                Vec::with_capacity(device.num_internal_nodes());
                            for idx in 0..device.num_internal_nodes() {
                                let node_name = format!("{}.__int{}", element.name, idx + 1);
                                internal_nodes.push(circuit.get_or_create_node(&node_name));
                            }
                            device.set_internal_node_indices(&internal_nodes);
                        }

                        for (name, value) in params {
                            let _ = device.set_parameter(name, *value);
                        }
                        device.set_temperature(self.config.temperature);
                        circuit.veriloga_devices.add(device);
                        continue;
                    }

                    return Err(SimulationError::Circuit(format!(
                        "Unresolved subcircuit instance '{}' referencing '{}'",
                        element.name, subckt_name
                    )));
                }
                #[cfg(not(feature = "veriloga"))]
                ElementKind::Subcircuit { subckt_name, .. } => {
                    return Err(SimulationError::Circuit(format!(
                        "Unresolved subcircuit instance '{}' referencing '{}'",
                        element.name, subckt_name
                    )));
                }

                // New element types
                ElementKind::VSwitch {
                    control_pos,
                    control_neg,
                    model,
                    initial_state,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(control_pos);
                    let cn = circuit.get_or_create_node(control_neg);

                    let model_def = find_model_def(netlist, model).ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Voltage-controlled switch '{}' references unknown model '{}'",
                            element.name, model
                        ))
                    })?;
                    ensure_model_type(
                        "Voltage-controlled switch",
                        &element.name,
                        model,
                        model_def,
                        &["SW", "VSWITCH", "VSW"],
                    )?;
                    let params_map: std::collections::HashMap<String, f64> =
                        model_def.params.iter().cloned().collect();

                    let mut sw = crate::device::VoltageSwitch::new(
                        element.name.clone(),
                        np,
                        nn, // Switch terminals
                        cp,
                        cn, // Control terminals
                    )
                    .with_params(&params_map);
                    if let Some(state) = initial_state {
                        sw = sw.with_initial_state(map_switch_state(*state));
                    }
                    circuit.vswitches.push(sw);
                }
                ElementKind::ISwitch {
                    control_element,
                    model,
                    initial_state,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);

                    let model_def = find_model_def(netlist, model).ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Current-controlled switch '{}' references unknown model '{}'",
                            element.name, model
                        ))
                    })?;
                    ensure_model_type(
                        "Current-controlled switch",
                        &element.name,
                        model,
                        model_def,
                        &["CSW", "ISWITCH", "ISW"],
                    )?;
                    let params_map: std::collections::HashMap<String, f64> =
                        model_def.params.iter().cloned().collect();

                    let mut sw = crate::device::CurrentSwitch::new(
                        element.name.clone(),
                        np,
                        nn,
                        control_element.clone(), // Control source name
                    )
                    .with_params(&params_map);
                    if let Some(state) = initial_state {
                        sw = sw.with_initial_state(map_switch_state(*state));
                    }
                    let iswitch_idx = circuit.iswitches.len();
                    circuit.iswitches.push(sw);
                    circuit.add_iswitch_pending(iswitch_idx, control_element.clone());
                }
                ElementKind::TransmissionLine {
                    z0,
                    td,
                    freq,
                    nl,
                    model,
                } => {
                    if element.nodes.len() < 4 {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' requires 4 nodes",
                            element.name
                        )));
                    }

                    let p1p = circuit.get_or_create_node(&element.nodes[0]);
                    let p1n = circuit.get_or_create_node(&element.nodes[1]);
                    let p2p = circuit.get_or_create_node(&element.nodes[2]);
                    let p2n = circuit.get_or_create_node(&element.nodes[3]);

                    if let (Some(model_name), Some(model_def)) = (
                        model.as_deref(),
                        model
                            .as_deref()
                            .and_then(|name| find_model_def(netlist, name)),
                    ) {
                        ensure_model_type(
                            "Transmission line",
                            &element.name,
                            model_name,
                            model_def,
                            &["LTRA", "TXL"],
                        )?;
                    }

                    let model_params = model
                        .as_deref()
                        .and_then(|name| resolve_tline_model_params(netlist, name));

                    if model.is_some() && model_params.is_none() && z0.is_none() {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' references unknown model '{}'",
                            element.name,
                            model.as_deref().unwrap_or_default()
                        )));
                    }

                    let freq_eff = (*freq).or(model_params.and_then(|m| m.freq));
                    let nl_eff = (*nl).or(model_params.and_then(|m| m.nl));

                    let delay = (*td)
                        .or_else(|| {
                            if let (Some(f), Some(n)) = (freq_eff, nl_eff) {
                                if f > 0.0 {
                                    Some(n / f)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .or(model_params.and_then(|m| m.td))
                        .unwrap_or(1e-9);

                    let z0_eff = (*z0).or(model_params.and_then(|m| m.z0)).unwrap_or(50.0);
                    if z0_eff <= 0.0 || !z0_eff.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' has invalid Z0={}",
                            element.name, z0_eff
                        )));
                    }
                    if delay <= 0.0 || !delay.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' has invalid TD={}",
                            element.name, delay
                        )));
                    }

                    let attenuation = model_params.and_then(|p| tline_model_attenuation(p, z0_eff));
                    let push_tline = |circuit: &mut CircuitData,
                                      name: String,
                                      p1p: usize,
                                      p1n: usize,
                                      p2p: usize,
                                      p2n: usize| {
                        let mut tline = crate::device::TransmissionLine::new(
                            name, p1p, p1n, p2p, p2n, z0_eff, delay,
                        );
                        tline.freq = freq_eff;
                        tline.nl = nl_eff;
                        if let Some(att) = attenuation {
                            tline.set_attenuation(att);
                        }
                        circuit.tlines.push(tline);
                    };

                    if element.nodes.len() == 4 {
                        push_tline(&mut circuit, element.name.clone(), p1p, p1n, p2p, p2n);
                    } else {
                        if element.nodes.len() % 2 != 0 {
                            return Err(SimulationError::Circuit(format!(
                                "Multiconductor transmission line '{}' requires an even number of nodes, found {}",
                                element.name,
                                element.nodes.len()
                            )));
                        }

                        let conductors = element.nodes.len() / 2;
                        if conductors < 2 {
                            return Err(SimulationError::Circuit(format!(
                                "Multiconductor transmission line '{}' requires at least two conductors",
                                element.name
                            )));
                        }

                        for conductor_idx in 0..conductors {
                            let near = circuit.get_or_create_node(&element.nodes[conductor_idx]);
                            let far = circuit
                                .get_or_create_node(&element.nodes[conductor_idx + conductors]);
                            push_tline(
                                &mut circuit,
                                format!("{}#{}", element.name, conductor_idx + 1),
                                near,
                                0,
                                far,
                                0,
                            );
                        }
                    }
                }
                ElementKind::Coupling {
                    inductors,
                    coefficient,
                } => {
                    // Store coupling for later resolution
                    circuit.couplings.push(crate::device::InductorCoupling::new(
                        element.name.clone(),
                        inductors.clone(),
                        *coefficient,
                    ));
                }

                // XSPICE code model instances
                ElementKind::Xspice {
                    model,
                    ports,
                    params,
                } => {
                    // Convert parsed XspicePort to PortConnection with resolved node IDs
                    let mut connections: Vec<crate::xspice::PortConnection> = Vec::new();
                    for port in ports {
                        let connection = match port {
                            crate::netlist::XspicePort::Analog(name) => {
                                let node = if name.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(name)
                                };
                                crate::xspice::PortConnection::Analog(node)
                            }
                            crate::netlist::XspicePort::Digital(name) => {
                                let node = if name.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(name)
                                };
                                crate::xspice::PortConnection::Digital(node)
                            }
                            crate::netlist::XspicePort::AnalogVector(names) => {
                                let nodes: Vec<usize> = names
                                    .iter()
                                    .map(|n| {
                                        if n.eq_ignore_ascii_case("0") {
                                            0
                                        } else {
                                            circuit.get_or_create_node(n)
                                        }
                                    })
                                    .collect();
                                crate::xspice::PortConnection::AnalogVector(nodes)
                            }
                            crate::netlist::XspicePort::DigitalVector(names) => {
                                let nodes: Vec<usize> = names
                                    .iter()
                                    .map(|n| {
                                        if n.eq_ignore_ascii_case("0") {
                                            0
                                        } else {
                                            circuit.get_or_create_node(n)
                                        }
                                    })
                                    .collect();
                                crate::xspice::PortConnection::DigitalVector(nodes)
                            }
                            crate::netlist::XspicePort::DifferentialVoltage { pos, neg }
                            | crate::netlist::XspicePort::DifferentialCurrent { pos, neg } => {
                                let pos_node = if pos.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(pos)
                                };
                                let neg_node = if neg.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(neg)
                                };
                                crate::xspice::PortConnection::Differential(pos_node, neg_node)
                            }
                            crate::netlist::XspicePort::Null => crate::xspice::PortConnection::Null,
                        };
                        connections.push(connection);
                    }

                    // Look up the model in the registry and create instance
                    if let Some(code_model) = circuit.xspice_registry.get(model) {
                        match crate::xspice::XspiceInstance::new(
                            element.name.clone(),
                            code_model.clone(),
                            connections,
                            params,
                        ) {
                            Ok(instance) => {
                                circuit.xspice_instances.push(instance);
                                log::debug!(
                                    "Created XSPICE instance {}: model={}, ports={}",
                                    element.name,
                                    model,
                                    ports.len()
                                );
                            }
                            Err(e) => {
                                log::warn!(
                                    "Failed to create XSPICE instance {}: {}",
                                    element.name,
                                    e
                                );
                            }
                        }
                    } else {
                        log::warn!(
                            "Unknown XSPICE model '{}' for element {}",
                            model,
                            element.name
                        );
                    }
                }
            }
        }

        // Ensure ground reference exists
        // If no node "0" was specified, auto-select a reference node
        circuit.ensure_ground_reference();

        // Resolve all pending control element references after final node count
        // is established (required for current-controlled switch branch indexing).
        circuit
            .resolve_control_elements()
            .map_err(|e| SimulationError::Circuit(e.to_string()))?;

        Ok(circuit)
    }
}

#[cfg(all(test, feature = "veriloga"))]
mod veriloga_cache_tests;
