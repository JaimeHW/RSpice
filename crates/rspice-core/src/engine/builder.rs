//! Circuit builder - constructs CircuitData from Netlist
//!
//! This module handles the conversion from parsed netlist elements
//! to the runtime circuit representation.

#![allow(clippy::needless_range_loop)]
use super::behavioral_expr::prepare_behavioral_expression;
use super::{Engine, SimulationError, extract_dc_value};
use crate::device::JfetChannelModel;
use crate::netlist::{ElementKind, flatten_netlist};
use crate::{CircuitData, Netlist};
#[cfg(feature = "veriloga")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "veriloga")]
use std::io::{Read, Write};
#[cfg(feature = "veriloga")]
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
#[cfg(feature = "veriloga")]
use std::sync::RwLock;
#[cfg(feature = "veriloga")]
use std::time::{Duration, Instant};

mod model_resolution;
use model_resolution::*;
mod behavioral;
mod builtin_models;
mod transmission_lines;
mod xspice_ports;
use behavioral::*;
use builtin_models::*;
use transmission_lines::*;
use xspice_ports::*;
#[cfg(feature = "veriloga")]
mod veriloga_cache;
#[cfg(feature = "veriloga")]
pub use veriloga_cache::{
    VerilogACacheEntry, VerilogACachePruneReport, VerilogACacheStats, clear_veriloga_cache,
    prune_veriloga_cache, register_precompiled_veriloga_model,
    register_precompiled_veriloga_model_with_dependencies, veriloga_cache_entries,
    veriloga_cache_stats,
};
#[cfg(feature = "veriloga")]
use veriloga_cache::{normalize_model_key, resolve_cached_or_compile_veriloga};

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
                    value_expr,
                    model,
                    instance_params,
                } => {
                    if let Some(expression) = value_expr.as_deref()
                        && model.is_none()
                        && expression_references_circuit_state(expression)
                    {
                        add_behavioral_resistor(
                            &mut circuit,
                            netlist,
                            element,
                            expression,
                            instance_params,
                            self.config.temperature,
                        )?;
                        continue;
                    }

                    let resistance = resolve_resistor_instance_value(
                        netlist,
                        &element.name,
                        *value,
                        value_expr.as_deref(),
                        model.as_deref(),
                        instance_params,
                        self.config.temperature,
                    )?;
                    let small_signal_resistance = resolve_resistor_small_signal_value(
                        &element.name,
                        resistance,
                        instance_params,
                    )?;
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    circuit.resistors.add_with_small_signal(
                        element.name.clone(),
                        np,
                        nn,
                        resistance,
                        small_signal_resistance,
                    );
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
                        | crate::netlist::SourceSpec::DcTransient { .. }
                        | crate::netlist::SourceSpec::DcAcTransient { .. }
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
                    let transient_spec = match spec {
                        crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::DcTransient { .. }
                        | crate::netlist::SourceSpec::DcAcTransient { .. }
                        | crate::netlist::SourceSpec::Exp { .. } => Some(spec.clone()),
                        _ => None,
                    };
                    circuit.current_sources.add_with_ac_and_spec(
                        element.name.clone(),
                        np,
                        nn,
                        dc_value,
                        ac_mag,
                        ac_phase,
                        transient_spec,
                    );
                }
                ElementKind::Diode { model } => {
                    let anode = circuit.get_or_create_node(&element.nodes[0]);
                    let cathode = circuit.get_or_create_node(&element.nodes[1]);
                    let mut diode = crate::device::Diode::new(element.name.clone(), anode, cathode);

                    // Look up model and apply parameters
                    if let Some(device_model) = find_model_def(netlist, model) {
                        ensure_model_type(
                            "Diode",
                            &element.name,
                            model,
                            device_model,
                            &["D", "DIODE"],
                        )?;
                        let params_map = model_params_upper_map(&device_model.params);
                        diode = diode.with_model_params(&params_map);
                    } else if let Some(params_map) =
                        builtin_diode_model_map().get(&model.to_uppercase())
                    {
                        diode = diode.with_model_params(params_map);
                        log::debug!(
                            "Applied embedded diode fallback model '{}' to {}",
                            model,
                            element.name
                        );
                    } else {
                        return Err(SimulationError::Circuit(format!(
                            "Diode '{}' references unknown model '{}'",
                            element.name, model
                        )));
                    }

                    circuit.diodes.add(diode);
                }
                ElementKind::Bjt {
                    model,
                    bjt_type,
                    instance_params,
                } => {
                    let collector = circuit.get_or_create_node(&element.nodes[0]);
                    let base = circuit.get_or_create_node(&element.nodes[1]);
                    let emitter = circuit.get_or_create_node(&element.nodes[2]);
                    let substrate = element
                        .nodes
                        .get(3)
                        .map(|n| circuit.get_or_create_node(n))
                        .unwrap_or(0);

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
                        // Normalize keys so model cards remain case-insensitive.
                        let params_map = model_params_upper_map(&device_model.params);
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
                    } else {
                        return Err(SimulationError::Circuit(format!(
                            "BJT '{}' references unknown model '{}'",
                            element.name, model
                        )));
                    }

                    bjt = bjt.with_instance_params(instance_params);
                    bjt.set_temperature(self.config.temperature);
                    bjt.set_substrate_node(substrate);

                    circuit.bjts.add(bjt);
                }
                ElementKind::Mosfet {
                    model,
                    mos_type: _mos_type,
                    instance_params,
                } => {
                    // Resolve NMOS/PMOS from model card when available.
                    let model_def = find_binned_model_def(netlist, model, instance_params);
                    let resolved_mos_type = if let Some(device_model) = model_def {
                        resolve_mos_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "MOSFET '{}' references model '{}' with incompatible type '{}'; expected NMOS or PMOS",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else if model.eq_ignore_ascii_case("NMOS") {
                        crate::netlist::MosType::Nmos
                    } else if model.eq_ignore_ascii_case("PMOS") {
                        crate::netlist::MosType::Pmos
                    } else {
                        return Err(SimulationError::Circuit(format!(
                            "MOSFET '{}' references unknown model '{}'",
                            element.name, model
                        )));
                    };

                    let params_map =
                        model_def.map(|device_model| model_params_upper_map(&device_model.params));
                    let level = params_map
                        .as_ref()
                        .and_then(|params| params.get("LEVEL").copied())
                        .unwrap_or(1.0) as i32;

                    let bulk_node_name = if is_bsimsoi_level(level) && element.nodes.len() > 4 {
                        &element.nodes[4]
                    } else {
                        &element.nodes[3]
                    };

                    let drain_external = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source_external = circuit.get_or_create_node(&element.nodes[2]);
                    let bulk = circuit.get_or_create_node(bulk_node_name);

                    let sheet_resistance = params_map
                        .as_ref()
                        .and_then(|params| params.get("RSH").copied())
                        .filter(|value| value.is_finite() && *value > 0.0)
                        .unwrap_or(0.0);
                    let default_squares = if sheet_resistance > 0.0 { 1.0 } else { 0.0 };
                    let drain_squares = instance_param(
                        instance_params,
                        &["NRD", "NRD_SQ", "NRDS", "DRAIN_SQUARES"],
                    )
                    .unwrap_or(default_squares)
                    .max(0.0);
                    let source_squares = instance_param(
                        instance_params,
                        &["NRS", "NRS_SQ", "NRSS", "SOURCE_SQUARES"],
                    )
                    .unwrap_or(default_squares)
                    .max(0.0);

                    let drain_resistance = sheet_resistance * drain_squares;
                    let source_resistance = sheet_resistance * source_squares;
                    let drain = if drain_resistance > 0.0 {
                        let internal =
                            circuit.get_or_create_node(&format!("{}.__rd.internal", element.name));
                        circuit.resistors.add(
                            format!("{}.__rd", element.name),
                            drain_external,
                            internal,
                            drain_resistance,
                        );
                        internal
                    } else {
                        drain_external
                    };
                    let source = if source_resistance > 0.0 {
                        let internal =
                            circuit.get_or_create_node(&format!("{}.__rs.internal", element.name));
                        circuit.resistors.add(
                            format!("{}.__rs", element.name),
                            source_external,
                            internal,
                            source_resistance,
                        );
                        internal
                    } else {
                        source_external
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
                    if let Some(params_map) = params_map.as_ref() {
                        mosfet = mosfet.with_level(level);

                        // Apply all model parameters (VTO, KP, GAMMA, KC, NC, etc.)
                        mosfet = mosfet.with_params(params_map);
                    }

                    mosfet = mosfet.with_instance_params(instance_params);

                    circuit.mosfets.add(mosfet);
                }
                ElementKind::Jfet {
                    model,
                    jfet_type: _jfet_type,
                    instance_params,
                } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);

                    // Resolve NJF/PJF from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let model_order = netlist
                        .models
                        .iter()
                        .position(|m| m.name.eq_ignore_ascii_case(model))
                        .unwrap_or(usize::MAX);
                    let resolved_jfet_type = if let Some(device_model) = model_def {
                        resolve_jfet_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "JFET '{}' references model '{}' with incompatible type '{}'; expected NJF or PJF",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else if model.eq_ignore_ascii_case("NJF") {
                        crate::netlist::JfetType::Njf
                    } else if model.eq_ignore_ascii_case("PJF") {
                        crate::netlist::JfetType::Pjf
                    } else {
                        return Err(SimulationError::Circuit(format!(
                            "JFET '{}' references unknown model '{}'",
                            element.name, model
                        )));
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
                        let params_map = model_params_upper_map(&device_model.params);
                        jfet = jfet.with_model_params(&params_map);
                    }
                    jfet = jfet.with_instance_params(instance_params);
                    jfet.set_model_order(model_order);

                    // Realistic extrinsic JFET series resistances (RD/RS) are modeled by
                    // inserting explicit linear resistors and connecting the intrinsic JFET
                    // to generated internal drain/source nodes.
                    // ngspice stamps model RD/RS as conductance scaled by the
                    // instance area/multiplicity. Explicit resistors therefore
                    // use the reciprocal equivalent, R / scale.
                    let resistance_scale = jfet_extrinsic_resistance_scale(&jfet);
                    let rd = scaled_extrinsic_resistance(jfet.params.rd, resistance_scale);
                    let rs = scaled_extrinsic_resistance(jfet.params.rs, resistance_scale);

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
                // MESFET (GaAs FET) families share the JFET device container,
                // with model selection below preserving the ngspice equations.
                ElementKind::Mesfet {
                    model,
                    mesfet_type: _mesfet_type,
                    instance_params,
                } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);
                    // Resolve NMF/PMF from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let model_order = netlist
                        .models
                        .iter()
                        .position(|m| m.name.eq_ignore_ascii_case(model))
                        .unwrap_or(usize::MAX);
                    let params_map =
                        model_def.map(|device_model| model_params_upper_map(&device_model.params));
                    let use_hfet_defaults = model_def
                        .map(|device_model| {
                            device_model.model_type.eq_ignore_ascii_case("NHFET")
                                || device_model.model_type.eq_ignore_ascii_case("PHFET")
                        })
                        .unwrap_or_else(|| {
                            model.eq_ignore_ascii_case("NHFET")
                                || model.eq_ignore_ascii_case("PHFET")
                        });
                    let resolved_mesfet_type = if let Some(device_model) = model_def {
                        resolve_mesfet_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "MESFET '{}' references model '{}' with incompatible type '{}'; expected NMF or PMF",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else if model.eq_ignore_ascii_case("NMF") {
                        crate::netlist::MesfetType::Nmf
                    } else if model.eq_ignore_ascii_case("PMF") {
                        crate::netlist::MesfetType::Pmf
                    } else if model.eq_ignore_ascii_case("NHFET") {
                        crate::netlist::MesfetType::Nmf
                    } else if model.eq_ignore_ascii_case("PHFET") {
                        crate::netlist::MesfetType::Pmf
                    } else {
                        return Err(SimulationError::Circuit(format!(
                            "MESFET '{}' references unknown model '{}'",
                            element.name, model
                        )));
                    };

                    let jfet_base = match resolved_mesfet_type {
                        crate::netlist::MesfetType::Nmf => {
                            crate::device::Jfet::njf(&element.name, drain, gate, source)
                        }
                        crate::netlist::MesfetType::Pmf => {
                            crate::device::Jfet::pjf(&element.name, drain, gate, source)
                        }
                    };
                    let mut jfet = if use_hfet_defaults {
                        jfet_base.enable_hfet_model()
                    } else if params_map
                        .as_ref()
                        .and_then(|params| params.get("LEVEL").copied())
                        .is_some_and(is_physical_mesa_mesfet_level)
                    {
                        jfet_base.enable_mesa_model()
                    } else {
                        jfet_base.enable_legacy_mesfet_model()
                    };

                    // Look up model and apply parameters
                    if let Some(params_map) = params_map.as_ref() {
                        jfet = jfet.with_model_params(&params_map);
                    }
                    jfet = jfet.with_instance_params(instance_params);
                    jfet.set_model_order(model_order);

                    // ngspice stamps model RD/RS as conductance scaled by the
                    // instance area/multiplicity. Explicit resistors therefore
                    // use the reciprocal equivalent, R / scale.
                    let resistance_scale = jfet_extrinsic_resistance_scale(&jfet);
                    let rd = scaled_extrinsic_resistance(jfet.params.rd, resistance_scale);
                    let rs = scaled_extrinsic_resistance(jfet.params.rs, resistance_scale);

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
                    let branch = circuit.allocate_branch_named(&element.name);
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
                ElementKind::BehavioralVoltage {
                    expression,
                    tc1,
                    tc2,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    let prepared_expression = prepare_temperature_scaled_behavioral_expression(
                        expression,
                        &netlist.params,
                        self.config.temperature,
                        netlist.options.tnom.unwrap_or(27.0),
                        *tc1,
                        *tc2,
                    )
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Behavioral source '{}': {}",
                            element.name, e
                        ))
                    })?;

                    let bvs = crate::device::BehavioralVoltageSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        branch,
                        &prepared_expression,
                    )
                    .map_err(SimulationError::Circuit)?;
                    circuit.behavioral_sources.add_voltage(bvs);
                }
                ElementKind::BehavioralCurrent {
                    expression,
                    tc1,
                    tc2,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let prepared_expression = prepare_temperature_scaled_behavioral_expression(
                        expression,
                        &netlist.params,
                        self.config.temperature,
                        netlist.options.tnom.unwrap_or(27.0),
                        *tc1,
                        *tc2,
                    )
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Behavioral source '{}': {}",
                            element.name, e
                        ))
                    })?;

                    let bcs = crate::device::BehavioralCurrentSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        &prepared_expression,
                    )
                    .map_err(SimulationError::Circuit)?;
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
                            let resolved = match value {
                                crate::netlist::ParametricValue::Resolved(v) => *v,
                                crate::netlist::ParametricValue::Expression(expr) => {
                                    crate::netlist::expr::eval_expression(expr, &netlist.params)
                                        .map_err(|e| {
                                            SimulationError::Circuit(format!(
                                                "Failed to resolve Verilog-A parameter '{}': {}",
                                                name, e
                                            ))
                                        })?
                                }
                            };
                            let _ = device.set_parameter(name, resolved);
                        }
                        device.set_temperature(self.config.temperature);
                        circuit.add_veriloga_device(device);
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
                    let params_map = model_params_upper_map(&model_def.params);

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
                    let params_map = model_params_upper_map(&model_def.params);

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

                    if let Some(model_name) = model.as_deref() {
                        if let Some(model_def) = find_model_def(netlist, model_name) {
                            if model_def.model_type.eq_ignore_ascii_case("CPL") {
                                build_cpl_multiconductor_line(
                                    &mut circuit,
                                    netlist,
                                    element,
                                    model_name,
                                )?;
                                continue;
                            }

                            ensure_model_type(
                                "Transmission line",
                                &element.name,
                                model_name,
                                model_def,
                                &["LTRA", "TXL"],
                            )?;
                        } else if z0.is_none() {
                            return Err(SimulationError::Circuit(format!(
                                "Transmission line '{}' references unknown model '{}'",
                                element.name, model_name
                            )));
                        }
                    }

                    let model_params = model
                        .as_deref()
                        .and_then(|name| resolve_tline_model_params(netlist, name));

                    let p1p = circuit.get_or_create_node(&element.nodes[0]);
                    let p1n = circuit.get_or_create_node(&element.nodes[1]);
                    let p2p = circuit.get_or_create_node(&element.nodes[2]);
                    let p2n = circuit.get_or_create_node(&element.nodes[3]);

                    let freq_eff = (*freq).or(model_params.and_then(|m| m.freq));
                    let nl_eff = (*nl).or(model_params.and_then(|m| m.nl));
                    // Keep scalar LTRA/TXL instances on the delayed-wave device path.
                    // A synthesized RLGC ladder is useful for diagnostics, but it is
                    // not behaviorally equivalent to ngspice's transmission-line models
                    // and can be substantially slower on the regression decks.
                    let synthesize_distributed_rlgc = false;

                    let delay = (*td)
                        .or_else(|| {
                            if let (Some(f), Some(n)) = (freq_eff, nl_eff) {
                                if f > 0.0 { Some(n / f) } else { None }
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
                    let loss_time_constant = model_params.and_then(tline_model_loss_time_constant);
                    let compact_reltol = model_params
                        .and_then(|p| p.compactrel)
                        .unwrap_or_else(|| self.voltage_reltol());
                    let compact_abstol = model_params
                        .and_then(|p| p.compactabs)
                        .unwrap_or_else(|| self.voltage_abstol());
                    let dc_series_resistance = model_params
                        .and_then(|p| {
                            let r = p.r?;
                            if !r.is_finite() || r <= 0.0 {
                                return None;
                            }
                            let len = p.len.unwrap_or(1.0);
                            if !len.is_finite() || len <= 0.0 {
                                return None;
                            }
                            Some(r * len)
                        })
                        .unwrap_or(0.0);
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
                        tline.set_dc_series_resistance(dc_series_resistance);
                        if let Some(params) = model_params
                            && let (Some(l), Some(c), Some(len)) = (params.l, params.c, params.len)
                        {
                            let r = params.r.unwrap_or(0.0);
                            let g = params.g.unwrap_or(0.0);
                            tline.set_distributed_rlgc_with_compaction(
                                r,
                                l,
                                g,
                                c,
                                len,
                                compact_reltol,
                                compact_abstol,
                            );
                            if let Some(step_hint) = tline.distributed_rlgc_max_safe_step() {
                                circuit.tighten_transient_max_step_hint(step_hint);
                            }
                        }
                        if let Some(att) = attenuation {
                            tline.set_attenuation(att);
                        }
                        if let Some(tau) = loss_time_constant {
                            tline.set_loss_time_constant(tau);
                        }
                        circuit.tlines.push(tline);
                    };

                    if element.nodes.len() == 4 {
                        if synthesize_distributed_rlgc {
                            build_scalar_rlgc_line(
                                &mut circuit,
                                &element.name,
                                p1p,
                                p1n,
                                p2p,
                                p2n,
                                model_params.expect("distributed RLGC synthesis requires model"),
                            )?;
                        } else {
                            push_tline(&mut circuit, element.name.clone(), p1p, p1n, p2p, p2n);
                        }
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
                            let conductor_name = format!("{}#{}", element.name, conductor_idx + 1);
                            if synthesize_distributed_rlgc {
                                build_scalar_rlgc_line(
                                    &mut circuit,
                                    &conductor_name,
                                    near,
                                    0,
                                    far,
                                    0,
                                    model_params
                                        .expect("distributed RLGC synthesis requires model"),
                                )?;
                            } else {
                                push_tline(&mut circuit, conductor_name, near, 0, far, 0);
                            }
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
                    let resolved_model = resolve_xspice_model_instance(
                        netlist,
                        &circuit.xspice_registry,
                        model,
                        params,
                    )
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Failed to resolve XSPICE model '{}' for element {}: {}",
                            model, element.name, e
                        ))
                    })?;

                    let ports_spec = resolved_model.code_model.ports().to_vec();
                    let mut connections: Vec<crate::xspice::PortConnection> =
                        Vec::with_capacity(ports.len());
                    for (port_idx, port) in ports.iter().enumerate() {
                        let port_spec = ports_spec.get(port_idx).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "XSPICE element '{}' provides more connections ({}) than model '{}' ports ({})",
                                element.name,
                                ports.len(),
                                resolved_model.code_model.name(),
                                ports_spec.len()
                            ))
                        })?;
                        let connection = coerce_xspice_connection(&mut circuit, port_spec, port)?;
                        connections.push(connection);
                    }

                    let mut instance = crate::xspice::XspiceInstance::new(
                        element.name.clone(),
                        resolved_model.code_model.clone(),
                        connections,
                        &resolved_model.numeric_params,
                        &resolved_model.string_params,
                    )
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Failed to create XSPICE instance '{}': {}",
                            element.name, e
                        ))
                    })?;

                    // Allocate MNA branch variables for voltage-driven XSPICE outputs.
                    // This allows stamping exact branch equations (like independent/controlled V sources)
                    // instead of approximating these ports as nodal current injections.
                    let ports_spec = instance.ports().to_vec();
                    for (port_idx, port_spec) in ports_spec.iter().enumerate() {
                        let is_output =
                            matches!(port_spec.direction, crate::xspice::PortDirection::Out);
                        let is_voltage_port = matches!(
                            port_spec.default_type,
                            crate::xspice::PortType::Voltage
                                | crate::xspice::PortType::DifferentialVoltage
                        );
                        if !is_output || !is_voltage_port {
                            continue;
                        }

                        let connection = instance.connection_at(port_idx);
                        let is_connected_analog = matches!(
                            connection,
                            Some(crate::xspice::PortConnection::Analog(_))
                                | Some(crate::xspice::PortConnection::Differential(_, _))
                        );
                        if !is_connected_analog {
                            continue;
                        }

                        let branch_name = format!("{}#{}", element.name, port_spec.name);
                        let branch_ordinal = circuit.allocate_branch_named(&branch_name);
                        instance
                            .set_output_branch(port_idx, branch_ordinal)
                            .map_err(|e| {
                                SimulationError::Circuit(format!(
                                    "Failed to assign branch for XSPICE instance '{}' port '{}': {}",
                                    element.name, port_spec.name, e
                                ))
                            })?;
                    }

                    circuit.xspice_instances.push(instance);
                    log::debug!(
                        "Created XSPICE instance {}: model={}, ports={}",
                        element.name,
                        model,
                        ports.len()
                    );
                }
            }
        }

        // Ensure ground reference exists
        // If no node "0" was specified, auto-select a reference node
        circuit.ensure_ground_reference();

        // Resolve behavioral source expression references after final node IDs
        // are stabilized (including any automatic ground remap).
        circuit
            .bind_behavioral_references()
            .map_err(|e| SimulationError::Circuit(e.to_string()))?;

        // Resolve all pending control element references after final node count
        // is established (required for current-controlled switch branch indexing).
        circuit
            .resolve_control_elements()
            .map_err(|e| SimulationError::Circuit(e.to_string()))?;

        let junction_gmin = self
            .config
            .convergence_config
            .gmin_initial
            .max(self.config.convergence_config.gmin_target)
            .max(0.0);
        for mos in &mut circuit.mosfets.devices {
            mos.set_junction_gmin(junction_gmin);
        }

        Ok(circuit)
    }
}
