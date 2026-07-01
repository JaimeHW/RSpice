//! Circuit builder - constructs CircuitData from Netlist
//!
//! This module handles the conversion from parsed netlist elements
//! to the runtime circuit representation.

#![allow(clippy::needless_range_loop)]
use super::{Engine, JfetLevel2Model, SimulationError, SpiceDialect, extract_dc_value};
use crate::device::{JfetChannelModel, MosBodyJunctionModel};
use crate::netlist::expr::prepare_behavioral_expression;
use crate::netlist::{
    Element, ElementKind, SourceSpec, XYCE_DEFAULT_ZERO_RESISTANCE_TOL, XspicePort,
    flatten_netlist_with_models, reduce_supernode_topology,
};
use crate::{CircuitData, Netlist};
#[cfg(feature = "veriloga")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "veriloga")]
use std::io::Read;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
use std::io::Write;
#[cfg(feature = "veriloga")]
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
#[cfg(feature = "veriloga")]
use std::sync::RwLock;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
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
    register_precompiled_veriloga_model_with_dependencies,
    register_precompiled_veriloga_runtime_with_dependencies, veriloga_cache_entries,
    veriloga_cache_stats,
};
#[cfg(feature = "veriloga")]
use veriloga_cache::{normalize_model_key, resolve_cached_or_compile_veriloga};

mod model_policy;
use model_policy::*;
mod advanced_mos;
use advanced_mos::{Bsim3v3SharedModel, Bsim4v8SharedModel};
#[cfg(feature = "veriloga-builtins")]
mod generated_model_routing;
#[cfg(feature = "veriloga-builtins")]
use generated_model_routing::{
    try_route_generated_bjt_model, try_route_generated_diode_model, try_route_generated_mos_model,
    try_route_generated_resistor_model,
};

fn validate_source_file_inputs(
    source_name: &str,
    spec: &crate::netlist::SourceSpec,
) -> Result<(), SimulationError> {
    use crate::netlist::SourceSpec;

    match spec {
        SourceSpec::PwlFile {
            path,
            time_scale,
            value_scale,
            time_offset,
            value_offset,
        } => crate::circuit::VoltageSources::load_pwl_waveform_cached(
            path,
            *time_scale,
            *value_scale,
            *time_offset,
            *value_offset,
        )
        .map(|_| ())
        .map_err(|error| SimulationError::Circuit(format!("source '{source_name}': {error}"))),
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            validate_source_file_inputs(source_name, transient)
        }
        _ => Ok(()),
    }
}

fn xtradev_scalar_terminal_node(port: &XspicePort) -> Option<&str> {
    match port {
        XspicePort::Analog(name) | XspicePort::Conductance(name) => Some(name),
        _ => None,
    }
}

fn xtradev_two_terminal_nodes(
    element_name: &str,
    model_name: &str,
    model_type: &str,
    ports: &[XspicePort],
) -> Result<(String, String), SimulationError> {
    match ports {
        [XspicePort::DifferentialVoltage { pos, neg }]
        | [XspicePort::DifferentialCurrent { pos, neg }]
        | [XspicePort::DifferentialConductance { pos, neg }] => Ok((pos.clone(), neg.clone())),
        [first, second] => {
            let pos = xtradev_scalar_terminal_node(first);
            let neg = xtradev_scalar_terminal_node(second);
            match (pos, neg) {
                (Some(pos), Some(neg)) => Ok((pos.to_string(), neg.to_string())),
                _ => Err(SimulationError::Circuit(format!(
                    "XSPICE xtradev {model_type} instance '{element_name}' model '{model_name}' \
                     expects one differential terminal pair or two bare analog nodes"
                ))),
            }
        }
        _ => Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {model_type} instance '{element_name}' model '{model_name}' \
             expects one differential terminal pair or two bare analog nodes"
        ))),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XspiceMeterKind {
    Capacitance,
    Inductance,
}

fn xspice_meter_kind(model_name: &str) -> Option<XspiceMeterKind> {
    if model_name.eq_ignore_ascii_case("cmeter") {
        Some(XspiceMeterKind::Capacitance)
    } else if model_name.eq_ignore_ascii_case("lmeter") {
        Some(XspiceMeterKind::Inductance)
    } else {
        None
    }
}

fn xspice_meter_input_node<'a>(
    element_name: &str,
    model_name: &str,
    ports: &'a [XspicePort],
) -> Result<&'a str, SimulationError> {
    match ports.first() {
        Some(XspicePort::Analog(node)) => Ok(node),
        Some(XspicePort::DifferentialVoltage { pos, .. }) => Ok(pos),
        Some(other) => Err(SimulationError::Circuit(format!(
            "XSPICE xtradev meter instance '{element_name}' model '{model_name}' expects first \
             port to be voltage or differential voltage input, got {other:?}"
        ))),
        None => Err(SimulationError::Circuit(format!(
            "XSPICE xtradev meter instance '{element_name}' model '{model_name}' requires an input port"
        ))),
    }
}

fn xspice_meter_node_incident(nodes: &[String], target: &str) -> bool {
    nodes.iter().any(|node| node.eq_ignore_ascii_case(target))
}

fn xspice_meter_element_incident(element: &Element, target: &str) -> Result<bool, SimulationError> {
    if xspice_meter_node_incident(&element.nodes, target) {
        return Ok(true);
    }

    let ElementKind::Xspice { model, ports, .. } = &element.kind else {
        return Ok(false);
    };
    let (pos, neg) = xtradev_two_terminal_nodes(&element.name, model, "reactive", ports)?;
    Ok(pos.eq_ignore_ascii_case(target) || neg.eq_ignore_ascii_case(target))
}

fn xspice_meter_zero_dc_voltage_source(spec: &SourceSpec) -> bool {
    match spec {
        SourceSpec::Dc(value) => *value == 0.0,
        SourceSpec::DcAc { dc_value, .. } => *dc_value == 0.0,
        _ => false,
    }
}

fn xspice_meter_zero_source_other_node<'a>(element: &'a Element, target: &str) -> Option<&'a str> {
    let ElementKind::VoltageSource(spec) = &element.kind else {
        return None;
    };
    if !xspice_meter_zero_dc_voltage_source(spec) || element.nodes.len() < 2 {
        return None;
    }
    if element.nodes[0].eq_ignore_ascii_case(target) {
        Some(&element.nodes[1])
    } else if element.nodes[1].eq_ignore_ascii_case(target) {
        Some(&element.nodes[0])
    } else {
        None
    }
}

fn xspice_meter_resolved_capacitance(
    netlist: &Netlist,
    element: &Element,
    temperature: f64,
) -> Result<Option<f64>, SimulationError> {
    match &element.kind {
        ElementKind::Capacitor {
            value,
            model,
            instance_params,
            ..
        } => Ok(Some(resolve_capacitor_instance_value(
            netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            temperature,
        )?)),
        ElementKind::Xspice {
            model,
            params,
            expr_params,
            string_params,
            string_expr_params,
            string_vector_params,
            string_vector_expr_params,
            real_vector_params,
            real_vector_expr_params,
            ..
        } => {
            match resolve_native_xtradev_reactive_model(
                netlist,
                model,
                &element.name,
                params,
                expr_params,
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                real_vector_params,
                real_vector_expr_params,
            )? {
                Some(NativeXtradevReactiveModel::Capacitor { capacitance, .. }) => {
                    Ok(Some(capacitance))
                }
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

fn xspice_meter_resolved_inductance(
    netlist: &Netlist,
    element: &Element,
    temperature: f64,
) -> Result<Option<f64>, SimulationError> {
    match &element.kind {
        ElementKind::Inductor {
            value,
            model,
            instance_params,
            ..
        } => Ok(Some(resolve_inductor_instance_value(
            netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            temperature,
        )?)),
        ElementKind::Xspice {
            model,
            params,
            expr_params,
            string_params,
            string_expr_params,
            string_vector_params,
            string_vector_expr_params,
            real_vector_params,
            real_vector_expr_params,
            ..
        } => {
            match resolve_native_xtradev_reactive_model(
                netlist,
                model,
                &element.name,
                params,
                expr_params,
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                real_vector_params,
                real_vector_expr_params,
            )? {
                Some(NativeXtradevReactiveModel::Inductor { inductance, .. }) => {
                    Ok(Some(inductance))
                }
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

fn xspice_meter_parallel_inductance(existing: f64, next: f64) -> f64 {
    1.0 / (1.0 / existing + 1.0 / next)
}

fn xspice_meter_equivalent_capacitance(
    netlist: &Netlist,
    flat_elements: &[Element],
    input_node: &str,
    temperature: f64,
) -> Result<f64, SimulationError> {
    let mut capacitance = 0.0;

    for element in flat_elements {
        if let Some(value) = xspice_meter_resolved_capacitance(netlist, element, temperature)?
            && xspice_meter_element_incident(element, input_node)?
        {
            capacitance += value;
        }
    }

    for zero_source_node in flat_elements
        .iter()
        .filter_map(|element| xspice_meter_zero_source_other_node(element, input_node))
    {
        for element in flat_elements {
            if let Some(value) = xspice_meter_resolved_capacitance(netlist, element, temperature)?
                && xspice_meter_element_incident(element, zero_source_node)?
            {
                capacitance += value;
            }
        }
    }

    Ok(capacitance)
}

fn xspice_meter_equivalent_inductance(
    netlist: &Netlist,
    flat_elements: &[Element],
    input_node: &str,
    temperature: f64,
) -> Result<f64, SimulationError> {
    let mut inductance = 1.0e12;

    for element in flat_elements {
        if let Some(value) = xspice_meter_resolved_inductance(netlist, element, temperature)?
            && xspice_meter_element_incident(element, input_node)?
        {
            inductance = xspice_meter_parallel_inductance(inductance, value);
        }
    }

    for zero_source_node in flat_elements
        .iter()
        .filter_map(|element| xspice_meter_zero_source_other_node(element, input_node))
    {
        for element in flat_elements {
            if let Some(value) = xspice_meter_resolved_inductance(netlist, element, temperature)?
                && xspice_meter_element_incident(element, zero_source_node)?
            {
                inductance = xspice_meter_parallel_inductance(inductance, value);
            }
        }
    }

    Ok(inductance)
}

fn xspice_meter_measured_value(
    netlist: &Netlist,
    flat_elements: &[Element],
    element_name: &str,
    model_name: &str,
    ports: &[XspicePort],
    kind: XspiceMeterKind,
    temperature: f64,
) -> Result<f64, SimulationError> {
    let input_node = xspice_meter_input_node(element_name, model_name, ports)?;
    match kind {
        XspiceMeterKind::Capacitance => {
            xspice_meter_equivalent_capacitance(netlist, flat_elements, input_node, temperature)
        }
        XspiceMeterKind::Inductance => {
            xspice_meter_equivalent_inductance(netlist, flat_elements, input_node, temperature)
        }
    }
}

impl Engine {
    pub(crate) fn resolved_resistor_value(
        &self,
        netlist: &Netlist,
        resistor_name: &str,
    ) -> Result<Option<f64>, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|e| SimulationError::Netlist(format!("Flattening error: {}", e)))?;
        let mut effective_netlist;
        let netlist = if flattened.scoped_models.is_empty() {
            netlist
        } else {
            effective_netlist = netlist.clone();
            effective_netlist.models.extend(flattened.scoped_models);
            &effective_netlist
        };

        let Some(element) = flattened
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(resistor_name))
        else {
            return Ok(None);
        };

        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Ok(None);
        };

        resolve_resistor_instance_value(
            netlist,
            &element.name,
            *value,
            value_expr.as_deref(),
            model.as_deref(),
            instance_params,
            engine.config.temperature,
        )
        .map(Some)
    }

    /// Build circuit from netlist (flattens subcircuits first)
    pub fn build_circuit(&self, netlist: &Netlist) -> Result<CircuitData, SimulationError> {
        let mut circuit = CircuitData::new();
        circuit.b3soi_gmin_scale = if self.config.b3soi_gmin_scaling {
            1.0e-6
        } else {
            1.0
        };

        // Flatten subcircuit instances into top-level elements
        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|e| SimulationError::Netlist(format!("Flattening error: {}", e)))?;
        let mut effective_netlist;
        let netlist = if flattened.scoped_models.is_empty() {
            netlist
        } else {
            effective_netlist = netlist.clone();
            effective_netlist.models.extend(flattened.scoped_models);
            &effective_netlist
        };
        let mut flat_elements = flattened.elements;
        if netlist.options.topology_supernode.unwrap_or(false) {
            let reduction = reduce_supernode_topology(
                flat_elements,
                netlist
                    .options
                    .device_zero_resistance_tol
                    .unwrap_or(XYCE_DEFAULT_ZERO_RESISTANCE_TOL),
            );
            flat_elements = reduction.elements;
        }

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

        // One shared Arc per model: instances share the (megabyte-scale)
        // program and a single JIT compilation
        #[cfg(feature = "veriloga")]
        let mut veriloga_models: HashMap<String, veriloga_cache::CachedVerilogAModel> =
            HashMap::new();

        // One shared BSIM3v3.3 card + temperature block per .model name,
        // with the (W, L) size knots memoized across instances.
        let mut bsim3v3_models: HashMap<String, Bsim3v3SharedModel> = HashMap::new();

        // Likewise for BSIM4 v4.8, keyed on (W, L, NF) size knots.
        let mut bsim4v8_models: HashMap<String, Bsim4v8SharedModel> = HashMap::new();

        // Load and cache Verilog-A models referenced by .VERILOGA directives.
        #[cfg(feature = "veriloga")]
        {
            for include in &netlist.veriloga_includes {
                let entry = resolve_cached_or_compile_veriloga(&include.file_path)?;
                let model = std::sync::Arc::clone(&entry.model);

                let model_key = normalize_model_key(model.name.as_str());
                veriloga_models
                    .entry(model_key)
                    .or_insert_with(|| entry.clone());

                if let Some(alias) = include.model_name.as_deref() {
                    veriloga_models
                        .entry(normalize_model_key(alias))
                        .or_insert_with(|| entry.clone());
                }

                if let Some(stem) = include.file_path.file_stem().and_then(|s| s.to_str()) {
                    veriloga_models
                        .entry(normalize_model_key(stem))
                        .or_insert_with(|| entry.clone());
                }

                log::info!(
                    "Loaded Verilog-A model '{}' from {}",
                    model.name,
                    include.file_path.display()
                );
            }
        }

        warn_floating_nodes(&flat_elements);

        for element in &flat_elements {
            match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    #[cfg(not(feature = "veriloga-builtins"))]
                    let _ = deferred_params;

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

                    #[cfg(feature = "veriloga-builtins")]
                    if let Some(model_name) = model.as_deref()
                        && try_route_generated_resistor_model(
                            &mut circuit,
                            netlist,
                            element,
                            model_name,
                            instance_params,
                            deferred_params,
                            self.config.temperature,
                        )?
                    {
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
                    let zero_resistance_tol = netlist
                        .options
                        .device_zero_resistance_tol
                        .unwrap_or(XYCE_DEFAULT_ZERO_RESISTANCE_TOL)
                        .max(0.0);
                    if resistance.is_finite() && resistance.abs() <= zero_resistance_tol {
                        if !small_signal_resistance.is_finite() {
                            return Err(SimulationError::Circuit(format!(
                                "Resistor '{}' resolved to non-finite branch-form small-signal resistance {}",
                                element.name, small_signal_resistance
                            )));
                        }
                        let branch = circuit.allocate_branch_named(&element.name);
                        circuit.resistor_branches.add(
                            element.name.clone(),
                            np,
                            nn,
                            branch,
                            resistance,
                            small_signal_resistance,
                        );
                        continue;
                    }
                    circuit.resistors.add_with_small_signal(
                        element.name.clone(),
                        np,
                        nn,
                        resistance,
                        small_signal_resistance,
                    );
                    // Per-instance thermal-noise temperature, resnoise.c
                    // semantics: with TEMP given the offset is
                    // temp − CKTtemp + tnom (in Celsius terms, ngspice's
                    // own quirk); otherwise DTEMP is the offset directly.
                    let noise_dtemp = if let Some(temp) = instance_param(instance_params, &["TEMP"])
                    {
                        let temp_k = crate::analysis::temperature::celsius_to_kelvin(temp);
                        let tnom_c = netlist.options.tnom.unwrap_or(27.0);
                        temp_k - self.config.temperature + tnom_c
                    } else {
                        instance_param(instance_params, &["DTEMP"]).unwrap_or(0.0)
                    };
                    if noise_dtemp != 0.0 {
                        circuit
                            .resistors
                            .set_last_noise_temperature_offset(noise_dtemp);
                    }
                    // ngspice `noisy` instance switch (default on): a quiet
                    // resistor produces no noise at all.
                    if let Some(noisy) = instance_param(instance_params, &["NOISY", "NOISE"]) {
                        circuit.resistors.set_last_noisy(noisy != 0.0);
                    }
                    // Model-card flicker noise (resnoise.c), folded with the
                    // effective noise area at build time.
                    if let Some((coefficient, af, ef)) = resolve_resistor_flicker_noise(
                        netlist,
                        model.as_deref(),
                        instance_params,
                        self.config.temperature,
                    )? {
                        circuit
                            .resistors
                            .set_last_flicker_noise(coefficient, af, ef);
                    }
                }
                ElementKind::Capacitor {
                    value,
                    initial_voltage,
                    model,
                    instance_params,
                    ..
                } => {
                    let capacitance = resolve_capacitor_instance_value(
                        netlist,
                        &element.name,
                        *value,
                        model.as_deref(),
                        instance_params,
                        self.config.temperature,
                    )?;
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    if let Some(ic) = *initial_voltage {
                        circuit.capacitors.add_with_ic(
                            element.name.clone(),
                            np,
                            nn,
                            capacitance,
                            ic,
                        );
                    } else {
                        circuit
                            .capacitors
                            .add(element.name.clone(), np, nn, capacitance);
                    }
                }
                ElementKind::Inductor {
                    value,
                    initial_current,
                    model,
                    instance_params,
                    ..
                } => {
                    // Magnetic-core model cards (Jiles-Atherton) route to the
                    // hysteretic inductor; plain L/IND cards and modelless
                    // instances stay linear.
                    let core_model = model.as_deref().and_then(|model_name| {
                        find_model_def(netlist, model_name)
                            .filter(|def| is_magnetic_core_model_type(&def.model_type))
                            .map(|_| model_name)
                    });

                    if let Some(model_name) = core_model {
                        add_jiles_atherton_inductor_element(
                            &mut circuit,
                            netlist,
                            element,
                            *value,
                            model_name,
                            *initial_current,
                        )?;
                        continue;
                    }

                    let inductance = resolve_inductor_instance_value(
                        netlist,
                        &element.name,
                        *value,
                        model.as_deref(),
                        instance_params,
                        self.config.temperature,
                    )?;
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    if let Some(ic) = *initial_current {
                        circuit.inductors.add_with_ic(
                            element.name.clone(),
                            np,
                            nn,
                            branch,
                            inductance,
                            ic,
                        );
                    } else {
                        circuit
                            .inductors
                            .add(element.name.clone(), np, nn, branch, inductance);
                    }
                }
                ElementKind::JilesAthertonInductor {
                    value,
                    model,
                    initial_current,
                } => {
                    add_jiles_atherton_inductor_element(
                        &mut circuit,
                        netlist,
                        element,
                        *value,
                        model,
                        *initial_current,
                    )?;
                }
                ElementKind::VoltageSource(spec) => {
                    validate_source_file_inputs(&element.name, spec)?;
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
                        | crate::netlist::SourceSpec::Exp { .. }
                        | crate::netlist::SourceSpec::Sffm { .. }
                        | crate::netlist::SourceSpec::Am { .. } => Some(spec.clone()),
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
                    validate_source_file_inputs(&element.name, spec)?;
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
                        | crate::netlist::SourceSpec::Exp { .. }
                        | crate::netlist::SourceSpec::Sffm { .. }
                        | crate::netlist::SourceSpec::Am { .. } => Some(spec.clone()),
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
                ElementKind::VoltageSourceDeferred(_) | ElementKind::CurrentSourceDeferred(_) => {
                    return Err(SimulationError::Circuit(format!(
                        "Source '{}' still has unresolved subcircuit parameter scope after flattening",
                        element.name
                    )));
                }
                ElementKind::Diode {
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    #[cfg(not(feature = "veriloga-builtins"))]
                    let _ = deferred_params;

                    #[cfg(feature = "veriloga-builtins")]
                    if try_route_generated_diode_model(
                        &mut circuit,
                        netlist,
                        element,
                        model,
                        instance_params,
                        deferred_params,
                        self.config.temperature,
                    )? {
                        continue;
                    }

                    let anode = circuit.get_or_create_node(&element.nodes[0]);
                    let cathode = circuit.get_or_create_node(&element.nodes[1]);
                    // Model cards start from ngspice's defaults: parameters a
                    // card omits must mean what they mean in SPICE, not
                    // inherit the 1N4148-like convenience values.
                    let mut diode =
                        crate::device::Diode::spice_defaults(element.name.clone(), anode, cathode);

                    // Junction temperature: instance TEMP is absolute (C),
                    // DTEMP offsets the circuit temperature; the model TNOM
                    // (or .options tnom) anchors the legacy SPICE scaling.
                    let tnom_k = crate::analysis::temperature::celsius_to_kelvin(
                        netlist.options.tnom.unwrap_or(27.0),
                    );
                    let temp_k = if let Some(t) = instance_param(instance_params, &["TEMP"]) {
                        crate::analysis::temperature::celsius_to_kelvin(t)
                    } else if let Some(dt) = instance_param(instance_params, &["DTEMP"]) {
                        self.config.temperature + dt
                    } else {
                        self.config.temperature
                    };

                    // Look up model and apply parameters
                    let rs_given;
                    if let Some(device_model) = find_model_def(netlist, model) {
                        ensure_model_type(
                            "Diode",
                            &element.name,
                            model,
                            device_model,
                            &["D", "DIODE"],
                        )?;
                        let params_map = model_params_upper_map(&device_model.params);
                        validate_diode_model_level(
                            &element.name,
                            model,
                            &params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                        rs_given = params_map.contains_key("RS");
                        diode = diode.with_model_params(&params_map);
                    } else if let Some(params_map) =
                        builtin_diode_model_map().get(&model.to_uppercase())
                    {
                        rs_given = params_map.contains_key("RS");
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

                    // Instance scaling: AREA and M/MULT both act as parallel
                    // junction multipliers for the lumped junction (ngspice
                    // DIOload semantics): currents and depletion charge scale
                    // up, series resistance scales down.
                    let area = instance_param(instance_params, &["AREA"]).unwrap_or(1.0);
                    let mult = instance_param(instance_params, &["M", "MULT"]).unwrap_or(1.0);
                    let sidewall_perimeter =
                        instance_param(instance_params, &["PJ"]).unwrap_or(0.0);
                    if !area.is_finite() || area <= 0.0 {
                        return Err(SimulationError::Circuit(format!(
                            "Diode '{}' has invalid AREA={} (must be finite and > 0)",
                            element.name, area
                        )));
                    }
                    if !mult.is_finite() || mult <= 0.0 {
                        return Err(SimulationError::Circuit(format!(
                            "Diode '{}' has invalid multiplicity M={} (must be finite and > 0)",
                            element.name, mult
                        )));
                    }
                    if !sidewall_perimeter.is_finite() || sidewall_perimeter < 0.0 {
                        return Err(SimulationError::Circuit(format!(
                            "Diode '{}' has invalid PJ={} (must be finite and >= 0)",
                            element.name, sidewall_perimeter
                        )));
                    }
                    let junction_scale = area * mult;
                    if junction_scale != 1.0 {
                        diode.apply_junction_scaling(junction_scale);
                    }
                    diode.set_sidewall_perimeter(sidewall_perimeter * mult);
                    diode.multiplicity = mult;
                    diode.set_temperature(temp_k, tnom_k);

                    // Series resistance participates in the solution as an
                    // explicit resistor between the anode and an internal
                    // node (the junction model itself never stamps RS). Only
                    // externalized when the model card provides RS, keeping
                    // decks without RS bit-identical to prior behavior.
                    if rs_given && diode.rs.is_finite() && diode.rs > 0.0 {
                        let aint_name = format!("{}.__aint", element.name);
                        let aint = circuit.get_or_create_node(&aint_name);
                        let rs_name = format!("{}.__rs", element.name);
                        circuit.resistors.add(rs_name, anode, aint, diode.rs);
                        diode.node_anode = aint;
                        diode.rs = 0.0;
                        // dionoise.c heats the RS thermal source by the
                        // instance offset: DTEMP directly, or with TEMP
                        // given, temp − CKTtemp + tnom in Celsius terms
                        // (ngspice's quirk, mirrored).
                        let noise_dtemp = if instance_param(instance_params, &["TEMP"]).is_some() {
                            temp_k - self.config.temperature + netlist.options.tnom.unwrap_or(27.0)
                        } else {
                            temp_k - self.config.temperature
                        };
                        if noise_dtemp != 0.0 {
                            circuit
                                .resistors
                                .set_last_noise_temperature_offset(noise_dtemp);
                        }
                    }

                    circuit.diodes.add(diode);
                }
                ElementKind::Bjt {
                    model,
                    bjt_type,
                    instance_params,
                    deferred_params,
                } => {
                    #[cfg(not(feature = "veriloga-builtins"))]
                    let _ = deferred_params;

                    let collector = circuit.get_or_create_node(&element.nodes[0]);
                    let base = circuit.get_or_create_node(&element.nodes[1]);
                    let emitter = circuit.get_or_create_node(&element.nodes[2]);
                    let fourth_terminal = element
                        .nodes
                        .get(3)
                        .map(|n| circuit.get_or_create_node(n))
                        .unwrap_or(0);
                    let fifth_terminal = element
                        .nodes
                        .get(4)
                        .map(|n| circuit.get_or_create_node(n))
                        .unwrap_or(0);
                    let bjt_level;
                    // Resolve polarity from model card when available.
                    let model_def = find_model_def(netlist, model);
                    #[cfg(feature = "veriloga-builtins")]
                    if try_route_generated_bjt_model(
                        &mut circuit,
                        netlist,
                        element,
                        model,
                        model_def,
                        instance_params,
                        deferred_params,
                        self.config.temperature,
                    )? {
                        continue;
                    }

                    let resolved_bjt_type = if let Some(device_model) = model_def {
                        resolve_bjt_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "BJT '{}' references model '{}' with incompatible type '{}'; expected NPN, PNP, or LPNP",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *bjt_type
                    };

                    if let Some(device_model) = model_def {
                        let params_map = model_params_upper_map(&device_model.params);
                        if is_lpnp_bjt_model_type(&device_model.model_type) {
                            let level = params_map.get("LEVEL").copied().unwrap_or(1.0);
                            if !legacy_gummel_poon_bjt_level(level) {
                                let descriptor = bjt_level_descriptor(level);
                                return Err(SimulationError::Circuit(format!(
                                    "BJT '{}': model '{}' uses LPNP with {descriptor}; \
                                     LPNP is a legacy Gummel-Poon lateral-PNP alias and is \
                                     supported only for no LEVEL or LEVEL=0/1/2 until separate \
                                     reference-backed validation exists",
                                    element.name, model
                                )));
                            }
                        }
                        validate_bjt_model_level(
                            &element.name,
                            model,
                            &params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                    }

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
                        bjt_level = params_map.get("LEVEL").copied();
                        validate_bjt_model_level(
                            &element.name,
                            model,
                            &params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                        bjt = bjt.with_params(&params_map);
                    } else if let Some(params_map) =
                        builtin_bjt_model_map().get(&model.to_uppercase())
                    {
                        bjt_level = params_map.get("LEVEL").copied();
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
                    bjt.refresh_noise_temperature_offset(
                        self.config.temperature,
                        netlist.options.tnom.unwrap_or(27.0),
                    );
                    let xyce_vbic_external_dt = self.config.spice_dialect == SpiceDialect::Xyce
                        && bjt.uses_vbic_dynamic_charges()
                        && bjt_level.is_some_and(|level| bjt_level_matches(level, 11.0))
                        && fourth_terminal != 0;
                    let substrate = if xyce_vbic_external_dt {
                        0
                    } else {
                        fourth_terminal
                    };
                    let external_thermal = if xyce_vbic_external_dt {
                        fourth_terminal
                    } else {
                        fifth_terminal
                    };
                    bjt.set_substrate_node(substrate);
                    if bjt.uses_vbic_dynamic_charges() && external_thermal != 0 {
                        bjt.set_vbic_external_thermal_node(external_thermal);
                    }

                    // Legacy GP: externalize the constant collector,
                    // emitter, and base resistances onto real internal
                    // nodes (the diode/JFET/MOSFET pattern), so their
                    // thermal noise rides the resistor walk and junction
                    // noise injects at the true internal terminals.
                    // Values are taken after model, instance, and
                    // temperature application, and the zeroed device
                    // fields collapse the matching internal states, so
                    // the solved system is identical. Only the
                    // bias-dependent base part (qb-modulated, ngspice
                    // BJTgx, nonzero when RBM < RB) stays folded.
                    // VBIC instances solve their internal states as MNA
                    // unknowns (ngspice vbicsetup.c topology); allocate the
                    // non-collapsed internal nodes now so the matrix builder
                    // reserves the coupled block.
                    if bjt.uses_vbic_dynamic_charges() {
                        bjt.assign_vbic_internal_nodes(|suffix| {
                            circuit.get_or_create_node(&format!(
                                "{}.__{}.internal",
                                element.name, suffix
                            ))
                        });
                    } else if bjt.uses_legacy_gummel_poon() {
                        if bjt.rcx.is_finite() && bjt.rcx > 0.0 {
                            let cint_name = format!("{}.__cint", element.name);
                            let cint = circuit.get_or_create_node(&cint_name);
                            let rc_name = format!("{}.__rc", element.name);
                            circuit.resistors.add(rc_name, collector, cint, bjt.rcx);
                            bjt.node_collector = cint;
                            bjt.clear_collector_series_resistance();
                            if bjt.noise_temperature_offset != 0.0 {
                                circuit.resistors.set_last_noise_temperature_offset(
                                    bjt.noise_temperature_offset,
                                );
                            }
                        }
                        if bjt.re.is_finite() && bjt.re > 0.0 {
                            let eint_name = format!("{}.__eint", element.name);
                            let eint = circuit.get_or_create_node(&eint_name);
                            let re_name = format!("{}.__re", element.name);
                            circuit.resistors.add(re_name, emitter, eint, bjt.re);
                            bjt.node_emitter = eint;
                            bjt.clear_emitter_series_resistance();
                            if bjt.noise_temperature_offset != 0.0 {
                                circuit.resistors.set_last_noise_temperature_offset(
                                    bjt.noise_temperature_offset,
                                );
                            }
                        }
                        // The constant base part is RBM, which ngspice
                        // defaults to RB (bjttemp.c) so the folded remainder
                        // is zero for common cards. Junction limiting moves
                        // with the topology: the device update applies
                        // pnjlim to its junction state against the previous
                        // iterate (bjtload.c's discipline at the prime
                        // nodes), and the engine-side external scale clamp
                        // skips GP devices.
                        if bjt.rbx.is_finite() && bjt.rbx > 0.0 {
                            let bint_name = format!("{}.__bint", element.name);
                            let bint = circuit.get_or_create_node(&bint_name);
                            let rb_name = format!("{}.__rb", element.name);
                            circuit.resistors.add(rb_name, base, bint, bjt.rbx);
                            bjt.node_base = bint;
                            bjt.clear_base_constant_resistance();
                            if bjt.noise_temperature_offset != 0.0 {
                                circuit.resistors.set_last_noise_temperature_offset(
                                    bjt.noise_temperature_offset,
                                );
                            }
                        }
                    }

                    circuit.bjts.add(bjt);
                }
                ElementKind::Mosfet {
                    model,
                    mos_type: _mos_type,
                    compact_syntax,
                    instance_params,
                    deferred_params,
                } => {
                    // Resolve NMOS/PMOS from model card when available.
                    let model_def = find_binned_model_def(netlist, model, instance_params);
                    #[cfg(feature = "veriloga-builtins")]
                    if try_route_generated_mos_model(
                        &mut circuit,
                        netlist,
                        element,
                        model,
                        model_def,
                        *compact_syntax,
                        instance_params,
                        deferred_params,
                        self.config.temperature,
                    )? {
                        continue;
                    }

                    let params_map =
                        model_def.map(|device_model| model_params_upper_map(&device_model.params));
                    let resolved_mos_type = if let Some(device_model) = model_def {
                        resolve_mos_type_from_model(&device_model.model_type)
                            .or_else(|| {
                                params_map.as_ref().and_then(|params| {
                                    resolve_vdmos_type_from_model(&device_model.model_type, params)
                                })
                            })
                            .ok_or_else(|| {
                                SimulationError::Circuit(format!(
                                    "MOSFET '{}' references model '{}' with incompatible type '{}'; expected NMOS, PMOS, or VDMOS",
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
                    let level = match (params_map.as_ref(), model_def) {
                        (Some(params), Some(device_model)) => checked_integer_model_level(
                            "MOSFET",
                            &element.name,
                            model,
                            params,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?,
                        _ => None,
                    }
                    .unwrap_or(1);
                    let is_vdmos_compatible = level == 18
                        || model_def.is_some_and(|def| is_vdmos_model_type(&def.model_type));

                    if *compact_syntax
                        && !is_vdmos_compatible
                        && known_advanced_mos_level_without_native(level)
                    {
                        return Err(missing_advanced_mos_builtin_error(
                            &element.name,
                            model,
                            level,
                        ));
                    }

                    if *compact_syntax && !is_vdmos_compatible {
                        return Err(SimulationError::Circuit(format!(
                            "MOSFET '{}': compact three-terminal syntax `M D G S model` is only supported for VDMOS-compatible models; ordinary MOSFETs require an explicit bulk node.",
                            element.name
                        )));
                    }

                    // BSIMSOI variants are distinct devices with their own SOI node
                    // topology and charge model. Route each native level to its port:
                    // 55 -> FD (fully depleted), 56 -> DD (dynamic depletion),
                    // 57 -> PD (partially depleted). Xyce LEVEL=10 (BSIMSOI3)
                    // uses SOIMOD to select the same native family.
                    if is_bsimsoi_level(level) {
                        if let Some(params_map) = params_map.as_ref() {
                            let device_model = model_def
                                .expect("native BSIMSOI params map derives from model card");
                            reject_deferred_native_mos_model_params(
                                &element.name,
                                model,
                                "BSIMSOI",
                                params_map,
                                &device_model.expr_params,
                                &device_model.string_params,
                            )?;
                            let native_level =
                                native_bsimsoi_level_for(level, params_map, instance_params)
                                    .map_err(|err| {
                                        SimulationError::Circuit(format!(
                                            "MOSFET '{}': model '{}' {err}",
                                            element.name, model
                                        ))
                                    })?
                                    .expect("is_bsimsoi_level must map to a native SOI level");
                            match native_level {
                                55 => {
                                    Self::build_b3soi_fd(
                                        &mut circuit,
                                        element,
                                        resolved_mos_type,
                                        model,
                                        params_map,
                                        instance_params,
                                        deferred_params,
                                        self.config.temperature,
                                    )?;
                                    continue;
                                }
                                56 => {
                                    Self::build_b3soi_dd(
                                        &mut circuit,
                                        element,
                                        resolved_mos_type,
                                        model,
                                        params_map,
                                        instance_params,
                                        deferred_params,
                                        self.config.temperature,
                                    )?;
                                    continue;
                                }
                                57 => {
                                    Self::build_b3soi_pd(
                                        &mut circuit,
                                        element,
                                        resolved_mos_type,
                                        model,
                                        params_map,
                                        instance_params,
                                        deferred_params,
                                        self.config.temperature,
                                    )?;
                                    continue;
                                }
                                _ => {}
                            }
                        }
                    }

                    // BSIM3v3.3: LEVEL=8/49 (ngspice) and BSIM3-shaped
                    // LEVEL=9 (Xyce) cards route to the native port. ngspice
                    // also uses LEVEL=9 for MOS9, so BestAvailable dispatch
                    // keeps MOS9-shaped cards on the Berkeley MOS path below.
                    if matches!(level, 8 | 49)
                        && let (Some(params_map), Some(device_model)) =
                            (params_map.as_ref(), model_def)
                        && let Some(version_family) =
                            bsim3_level8_49_version_family(params_map, &device_model.string_params)
                    {
                        match version_family {
                            Bsim3VersionFamily::V32(version) => {
                                return Err(SimulationError::Circuit(format!(
                                    "MOSFET '{}': BSIM3v32 LEVEL={level} VERSION={version} \
                                     requires a distinct native BSIM3v32 port; RSpice's \
                                     BSIM3v3.3 native evaluator must not be used as a \
                                     VERSION={version} compatibility fallback",
                                    element.name
                                )));
                            }
                            Bsim3VersionFamily::UnsupportedPre33(version) => {
                                return Err(SimulationError::Circuit(format!(
                                    "MOSFET '{}': unsupported BSIM3 pre-3.3 LEVEL={level} \
                                     VERSION={version}; supported native BSIM3v3.3 cards use \
                                     VERSION>=3.3, while BSIM3v32 VERSION=3.2/3.22/3.23/3.24 \
                                     requires a distinct native port",
                                    element.name
                                )));
                            }
                            Bsim3VersionFamily::V33OrLater => {}
                        }
                    }
                    // One shared model card + temperature block per .model;
                    // size knots are memoized across instances exactly as
                    // ngspice reuses pSizeDependParamKnot.
                    let level9_is_bsim3 = matches!((level, params_map.as_ref(), model_def), (9, Some(params), Some(device_model))
                    if level9_selects_bsim3(
                        params,
                        &device_model.expr_params,
                        &device_model.string_params,
                        self.config.spice_dialect,
                    ));
                    if (matches!(level, 8 | 49) || level9_is_bsim3)
                        && let Some(params_map) = params_map.as_ref()
                    {
                        let device_model =
                            model_def.expect("native BSIM3 params map derives from model card");
                        let bsim3_params = native_bsim3_model_params_upper_map(
                            &element.name,
                            model,
                            params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                        let model_key = device_model.name.clone();
                        let tnom_default_k = crate::analysis::temperature::celsius_to_kelvin(
                            netlist.options.tnom.unwrap_or(27.0),
                        );
                        Self::build_bsim3v3(
                            &mut circuit,
                            element,
                            resolved_mos_type,
                            &model_key,
                            &bsim3_params,
                            instance_params,
                            deferred_params,
                            self.config.temperature,
                            tnom_default_k,
                            &mut bsim3v3_models,
                        )?;
                        continue;
                    }

                    // BSIM4 v4.8: LEVEL=14/54 cards route to the native port
                    // with the same sharing scheme (the size knots carry NF).
                    // Supported external resistance modes are lowered before
                    // the intrinsic device is registered; unsupported selectors
                    // surface the module's typed construction error.
                    if matches!(level, 14 | 54)
                        && let Some(params_map) = params_map.as_ref()
                    {
                        let device_model =
                            model_def.expect("native BSIM4 params map derives from model card");
                        let bsim4_params = native_bsim4_model_params_upper_map(
                            &element.name,
                            model,
                            params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                        let model_key = device_model.name.clone();
                        let tnom_default_k = crate::analysis::temperature::celsius_to_kelvin(
                            netlist.options.tnom.unwrap_or(27.0),
                        );
                        Self::build_bsim4v8(
                            &mut circuit,
                            element,
                            resolved_mos_type,
                            &model_key,
                            &bsim4_params,
                            instance_params,
                            self.config.temperature,
                            tnom_default_k,
                            &mut bsim4v8_models,
                        )?;
                        continue;
                    }

                    // EKV 2.6 LEVEL=260 has a native validated runtime when
                    // generated Verilog-A builtins are not enabled. Feature
                    // builds with the generated EKV builtin route before this
                    // point.
                    if native_ekv26_level(level)
                        && let Some(params_map) = params_map.as_ref()
                    {
                        let device_model =
                            model_def.expect("native EKV26 params map derives from model card");
                        let model_key = device_model.name.clone();
                        reject_deferred_native_mos_model_params(
                            &element.name,
                            &model_key,
                            "EKV26",
                            params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                        Self::build_ekv26(
                            &mut circuit,
                            element,
                            resolved_mos_type,
                            &model_key,
                            params_map,
                            instance_params,
                            self.config.temperature,
                        )?;
                        continue;
                    }

                    // EKV3 LEVEL=301. The native support is deliberately
                    // narrow: VA-Models NMOS150 ekv3_rf DC plus the Xyce
                    // VANOISE regression slice. Unsupported EKV3 surfaces fail
                    // closed rather than using simplified MOS.
                    if native_ekv3_level(level)
                        && let Some(params_map) = params_map.as_ref()
                    {
                        let device_model =
                            model_def.expect("native EKV3 params map derives from model card");
                        let model_key = device_model.name.clone();
                        reject_deferred_native_mos_model_params(
                            &element.name,
                            &model_key,
                            "EKV3",
                            params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                        Self::build_ekv3(
                            &mut circuit,
                            element,
                            resolved_mos_type,
                            &model_key,
                            params_map,
                            instance_params,
                            self.config.temperature,
                        )?;
                        continue;
                    }

                    // Native VDMOS accepts both compatibility fronts:
                    // Xyce MOS LEVEL=18 (`.model ... NMOS/PMOS level=18`)
                    // and ngspice VDMOS (`.model ... VDMOS nchan/pchan`).
                    if is_vdmos_compatible && let Some(params_map) = params_map.as_ref() {
                        let device_model =
                            model_def.expect("native VDMOS params map derives from model card");
                        let model_key = device_model.name.clone();
                        reject_deferred_native_mos_model_params(
                            &element.name,
                            &model_key,
                            "VDMOS",
                            params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                        Self::build_vdmos(
                            &mut circuit,
                            element,
                            resolved_mos_type,
                            params_map,
                            instance_params,
                            self.config.temperature,
                            crate::analysis::temperature::celsius_to_kelvin(
                                netlist.options.tnom.unwrap_or(27.0),
                            ),
                        )?;
                        continue;
                    }

                    // Levels without a native implementation must not fall
                    // through to the simplified short-channel approximation
                    // silently: a BSIM3/BSIM4 card evaluated with ~15 honored
                    // parameters yields plausible-looking but wrong currents,
                    // which is strictly worse than an error.
                    if !native_bulk_mos_level(level) {
                        if known_advanced_mos_level_without_native(level) {
                            return Err(missing_advanced_mos_builtin_error(
                                &element.name,
                                model,
                                level,
                            ));
                        }
                        let descriptor = mos_level_descriptor(level);
                        return Err(SimulationError::Circuit(format!(
                            "MOSFET '{}': model '{}' requests {} which has no native \
                             implementation. Supported levels: 1, 2, 3, 6 (Berkeley \
                            MOS1/MOS2/MOS3/MOS6), 4/5 (legacy BSIM1/BSIM2), \
                             9 (ngspice MOS9), 8/49 plus Xyce-style 9 \
                             (BSIM3v3.3), 14/54 (BSIM4 v4.8), 10/55/56/57 \
                             (native BSIMSOI), 18 (native VDMOS), 260 (EKV26), \
                             and 301 (EKV3). \
                            Unsupported MOS levels must fail closed until native support \
                            and reference-backed validation are added.",
                            element.name, model, descriptor
                        )));
                    }

                    let bulk_node_name = &element.nodes[3];

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
                    mosfet.set_body_junction_model(match self.config.spice_dialect {
                        SpiceDialect::Xyce => MosBodyJunctionModel::XyceClassicLinearizedReverse,
                        SpiceDialect::BestAvailable | SpiceDialect::Ngspice => {
                            MosBodyJunctionModel::NgspiceReverseClamp
                        }
                    });

                    // Look up model and apply parameters including LEVEL
                    if let Some(params_map) = params_map.as_ref() {
                        mosfet = mosfet.with_level(level);

                        // Apply all model parameters (VTO, KP, GAMMA, KC, NC, etc.)
                        mosfet = mosfet.with_params(params_map);
                    }

                    mosfet = mosfet.with_instance_params(instance_params);

                    // Device temperature: instance TEMP is absolute (C),
                    // DTEMP offsets the circuit temperature; model TNOM
                    // (or .options tnom) anchors the scaling.
                    let tnom_k = params_map
                        .as_ref()
                        .and_then(|params| params.get("TNOM").copied())
                        .filter(|v| v.is_finite())
                        .map(crate::analysis::temperature::celsius_to_kelvin)
                        .unwrap_or_else(|| {
                            crate::analysis::temperature::celsius_to_kelvin(
                                netlist.options.tnom.unwrap_or(27.0),
                            )
                        });
                    let temp_k = if let Some(t) = instance_param(instance_params, &["TEMP"]) {
                        crate::analysis::temperature::celsius_to_kelvin(t)
                    } else if let Some(dt) = instance_param(instance_params, &["DTEMP"]) {
                        self.config.temperature + dt
                    } else {
                        self.config.temperature
                    };
                    mosfet.set_temperature(temp_k, tnom_k);

                    // mos1noi.c heats every thermal source by the instance
                    // offset: DTEMP directly, or temp − CKTtemp + tnom in
                    // Celsius terms when TEMP is given (ngspice's quirk).
                    mosfet.noise_temperature_offset =
                        if instance_param(instance_params, &["TEMP"]).is_some() {
                            temp_k - self.config.temperature + netlist.options.tnom.unwrap_or(27.0)
                        } else {
                            temp_k - self.config.temperature
                        };

                    // Drain/source ohmic resistances, mos1temp.c precedence:
                    // RD (or RS) when given, else RSH times the diffusion
                    // squares. ngspice stamps the conductance at internal
                    // prime nodes scaled by the multiplicity; the explicit
                    // resistor uses the reciprocal equivalent R/m, and the
                    // repointed device terminals make junction noise and
                    // limiting act at the true internal nodes.
                    // Legacy BSIM1/BSIM2 instances keep their historical
                    // terminal topology; their sheet-resistance handling is
                    // part of the bsim parity program.
                    let multiplicity = mosfet.multiplicity.max(1e-12);
                    let resistances_apply = !mosfet.uses_legacy_bsim();
                    let drain_r = if !resistances_apply {
                        0.0
                    } else if mosfet.rd_model > 0.0 {
                        mosfet.rd_model
                    } else if mosfet.rsh > 0.0 {
                        mosfet.rsh * mosfet.nrd.max(0.0)
                    } else {
                        0.0
                    };
                    if drain_r > 0.0 {
                        let dint_name = format!("{}.__dint", element.name);
                        let dint = circuit.get_or_create_node(&dint_name);
                        let rd_name = format!("{}.__rd", element.name);
                        circuit
                            .resistors
                            .add(rd_name, drain, dint, drain_r / multiplicity);
                        mosfet.node_drain = dint;
                        if mosfet.noise_temperature_offset != 0.0 {
                            circuit
                                .resistors
                                .set_last_noise_temperature_offset(mosfet.noise_temperature_offset);
                        }
                    }
                    let source_r = if !resistances_apply {
                        0.0
                    } else if mosfet.rs_model > 0.0 {
                        mosfet.rs_model
                    } else if mosfet.rsh > 0.0 {
                        mosfet.rsh * mosfet.nrs.max(0.0)
                    } else {
                        0.0
                    };
                    if source_r > 0.0 {
                        let sint_name = format!("{}.__sint", element.name);
                        let sint = circuit.get_or_create_node(&sint_name);
                        let rs_name = format!("{}.__rs", element.name);
                        circuit
                            .resistors
                            .add(rs_name, source, sint, source_r / multiplicity);
                        mosfet.node_source = sint;
                        if mosfet.noise_temperature_offset != 0.0 {
                            circuit
                                .resistors
                                .set_last_noise_temperature_offset(mosfet.noise_temperature_offset);
                        }
                    }

                    circuit.mosfets.add(mosfet);
                }
                ElementKind::Jfet {
                    model,
                    jfet_type: _jfet_type,
                    instance_params,
                    ..
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
                        let level = checked_integer_model_level(
                            "JFET",
                            &element.name,
                            model,
                            &params_map,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?;
                        if let Some(level) = level
                            && !matches!(level, 1 | 2)
                        {
                            return Err(SimulationError::Circuit(format!(
                                "JFET '{}' model '{}' requests unsupported LEVEL={level}; supported native JFET levels are legacy LEVEL=1/no LEVEL and ngspice/Xyce LEVEL=2",
                                element.name, model
                            )));
                        }
                        if level == Some(2) {
                            jfet = match self.config.resolved_jfet_level2_model() {
                                JfetLevel2Model::DialectDefault => unreachable!(
                                    "resolved_jfet_level2_model must return a concrete selector"
                                ),
                                JfetLevel2Model::ParkerSkellern => jfet.enable_jfet2_model(),
                                JfetLevel2Model::XyceModifiedShockley => {
                                    jfet.enable_xyce_jfet2_model()
                                }
                            };
                        } else if self.config.spice_dialect == SpiceDialect::Xyce {
                            jfet = jfet.enable_xyce_jfet1_model();
                        }
                        jfet = jfet.with_model_params(&params_map);
                    }
                    jfet = jfet.with_instance_params(instance_params);
                    jfet.set_analysis_temperature(self.config.temperature);
                    jfet.set_model_order(model_order);

                    // jfetnoi.c heats the thermal sources by the instance
                    // offset; resolve it once for the channel source and the
                    // externalized resistors below.
                    jfet.noise_dtemp = jfet.noise_temperature_offset(
                        self.config.temperature,
                        netlist.options.tnom.unwrap_or(27.0),
                    );

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
                        if jfet.noise_dtemp != 0.0 {
                            circuit
                                .resistors
                                .set_last_noise_temperature_offset(jfet.noise_dtemp);
                        }
                    }
                    if rs > 0.0 {
                        let sint_name = format!("{}.__sint", element.name);
                        let sint = circuit.get_or_create_node(&sint_name);
                        let rs_name = format!("{}.__rs", element.name);
                        circuit.resistors.add(rs_name, source, sint, rs);
                        jfet.source = sint;
                        jfet.params.rs = 0.0;
                        if jfet.noise_dtemp != 0.0 {
                            circuit
                                .resistors
                                .set_last_noise_temperature_offset(jfet.noise_dtemp);
                        }
                    }

                    circuit.jfets.push(jfet);
                }
                // MESFET (GaAs FET) families share the JFET device container,
                // with model selection below preserving the ngspice equations.
                ElementKind::Mesfet {
                    model,
                    mesfet_type: _mesfet_type,
                    instance_params,
                    ..
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
                    let mesfet_level = match (params_map.as_ref(), model_def) {
                        (Some(params), Some(device_model)) => checked_integer_model_level(
                            "MESFET",
                            &element.name,
                            model,
                            params,
                            &device_model.expr_params,
                            &device_model.string_params,
                        )?,
                        _ => None,
                    };
                    if let Some(level) = mesfet_level
                        && !matches!(level, 0..=6)
                    {
                        return Err(SimulationError::Circuit(format!(
                            "MESFET '{}' model '{}' requests unsupported LEVEL={level}; \
                             supported native MESFET/HFET levels are no LEVEL/LEVEL=0/1 \
                             (legacy MESFET), LEVEL=2/3/4 (MESA), LEVEL=5 (HFET1), \
                             and LEVEL=6 (HFET2)",
                            element.name, model
                        )));
                    }
                    // ngspice selects the HFET-family equations either by the
                    // NHFET/PHFET model type or by NMF/PMF with LEVEL=5/6
                    // (the z-device level map: 1 = MES, 2-4 = MESA,
                    // 5 = HFET1, 6 = HFET2).
                    let card_is_hfet_level =
                        mesfet_level.is_some_and(|level| matches!(level, 5 | 6));
                    let use_hfet_defaults = model_def
                        .map(|device_model| {
                            device_model.model_type.eq_ignore_ascii_case("NHFET")
                                || device_model.model_type.eq_ignore_ascii_case("PHFET")
                        })
                        .unwrap_or_else(|| {
                            model.eq_ignore_ascii_case("NHFET")
                                || model.eq_ignore_ascii_case("PHFET")
                        })
                        || card_is_hfet_level;
                    let resolved_mesfet_type = if let Some(device_model) = model_def {
                        resolve_mesfet_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "MESFET '{}' references model '{}' with incompatible type '{}'; expected NMF, PMF, NHFET, or PHFET",
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
                    } else if mesfet_level.is_some_and(|level| matches!(level, 2..=4)) {
                        jfet_base.enable_mesa_model()
                    } else {
                        jfet_base.enable_legacy_mesfet_model()
                    };

                    // Look up model and apply parameters
                    if let Some(params_map) = params_map.as_ref() {
                        jfet = jfet.with_model_params(params_map);
                    }
                    jfet = jfet.with_instance_params(instance_params);
                    jfet.set_analysis_temperature(self.config.temperature);
                    jfet.set_model_order(model_order);

                    // jfetnoi.c heats the thermal sources by the instance
                    // offset; resolve it once for the channel source and the
                    // externalized resistors below.
                    jfet.noise_dtemp = jfet.noise_temperature_offset(
                        self.config.temperature,
                        netlist.options.tnom.unwrap_or(27.0),
                    );

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
                        if jfet.noise_dtemp != 0.0 {
                            circuit
                                .resistors
                                .set_last_noise_temperature_offset(jfet.noise_dtemp);
                        }
                    }
                    if rs > 0.0 {
                        let sint_name = format!("{}.__sint", element.name);
                        let sint = circuit.get_or_create_node(&sint_name);
                        let rs_name = format!("{}.__rs", element.name);
                        circuit.resistors.add(rs_name, source, sint, rs);
                        jfet.source = sint;
                        jfet.params.rs = 0.0;
                        if jfet.noise_dtemp != 0.0 {
                            circuit
                                .resistors
                                .set_last_noise_temperature_offset(jfet.noise_dtemp);
                        }
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

                    let mut bvs = crate::device::BehavioralVoltageSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        branch,
                        &prepared_expression,
                    )
                    .map_err(SimulationError::Circuit)?;
                    bvs.set_temperature(crate::analysis::temperature::kelvin_to_celsius(
                        self.config.temperature,
                    ));
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

                    let mut bcs = crate::device::BehavioralCurrentSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        &prepared_expression,
                    )
                    .map_err(SimulationError::Circuit)?;
                    bcs.set_temperature(crate::analysis::temperature::kelvin_to_celsius(
                        self.config.temperature,
                    ));
                    circuit.behavioral_sources.add_current(bcs);
                }
                // Flattened tree leaves external subcircuit-backed devices here
                // (for example, Verilog-A model instances).
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } => {
                    #[cfg(not(any(feature = "veriloga-builtins", feature = "veriloga")))]
                    let _ = params;

                    #[cfg(feature = "veriloga-builtins")]
                    {
                        if let Some(mut device) =
                            crate::device::veriloga_generated::instantiate_builtin(
                                subckt_name,
                                &element.name,
                                &element.nodes,
                                params,
                                &netlist.params,
                                &mut circuit,
                            )?
                        {
                            device.set_temperature(self.config.temperature);
                            circuit.add_generated_veriloga_device(device);
                            continue;
                        }
                    }

                    #[cfg(feature = "veriloga")]
                    {
                        if let Some(entry) = veriloga_models.get(&normalize_model_key(subckt_name))
                        {
                            let model = &entry.model;
                            if element.nodes.len() > model.num_terminals {
                                return Err(SimulationError::Circuit(format!(
                                    "Verilog-A instance '{}' expects at most {} terminals for model '{}', found {}",
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

                            #[cfg(feature = "veriloga-native")]
                            let mut device = {
                                let canonical_ir = entry.canonical_ir.as_deref().ok_or_else(|| {
                                    SimulationError::Circuit(format!(
                                        "Verilog-A device '{}' native JIT requires canonical IR for model '{}' (no interpreter fallback)",
                                        element.name, model.name
                                    ))
                                })?;
                                crate::device::veriloga::VerilogADevice::try_new_with_canonical_ir(
                                    element.name.clone(),
                                    std::sync::Arc::clone(model),
                                    canonical_ir,
                                    &node_ids,
                                )
                            }
                            .map_err(|err| {
                                SimulationError::Circuit(format!(
                                    "Verilog-A device '{}' parameter default resolution failed: {}",
                                    element.name, err
                                ))
                            })?;

                            #[cfg(not(feature = "veriloga-native"))]
                            let mut device = crate::device::veriloga::VerilogADevice::try_new(
                                element.name.clone(),
                                std::sync::Arc::clone(model),
                                &node_ids,
                            )
                            .map_err(|err| {
                                SimulationError::Circuit(format!(
                                    "Verilog-A device '{}' parameter default resolution failed: {}",
                                    element.name, err
                                ))
                            })?;

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

                            // Allocate system unknowns for branch currents of
                            // potential (voltage) contributions.
                            if device.num_branch_unknowns() > 0 {
                                let mut branch_nodes =
                                    Vec::with_capacity(device.num_branch_unknowns());
                                for idx in 0..device.num_branch_unknowns() {
                                    let node_name = format!("{}.__br{}", element.name, idx + 1);
                                    branch_nodes.push(circuit.get_or_create_node(&node_name));
                                }
                                device.set_branch_current_indices(&branch_nodes);
                            }

                            for (name, value) in params {
                                let resolved = match value {
                                    crate::netlist::ParametricValue::Resolved(v) => *v,
                                    crate::netlist::ParametricValue::Expression(expr) => {
                                        crate::netlist::expr::eval_expression(
                                            expr,
                                            &netlist.params,
                                        )
                                        .map_err(|e| {
                                            SimulationError::Circuit(format!(
                                                "Failed to resolve Verilog-A parameter '{}': {}",
                                                name, e
                                            ))
                                        })?
                                    }
                                    crate::netlist::ParametricValue::String(_)
                                    | crate::netlist::ParametricValue::StringExpression(_) => {
                                        return Err(SimulationError::Circuit(format!(
                                            "Verilog-A parameter '{}' expects a numeric value, got string value",
                                            name
                                        )));
                                    }
                                };
                                // `m=` on an instance whose model does not
                                // declare an m parameter is the standard
                                // parallel-multiplicity ($mfactor); models
                                // declaring their own m keep handling it
                                if !device.set_parameter(name, resolved)
                                    && name.eq_ignore_ascii_case("m")
                                {
                                    device.set_multiplicity(resolved);
                                }
                            }
                            // Dependent parameter defaults must see the instance
                            // overrides applied above
                            device.try_resolve_parameter_defaults().map_err(|err| {
                                SimulationError::Circuit(format!(
                                    "Verilog-A device '{}' parameter default resolution failed: {}",
                                    element.name, err
                                ))
                            })?;
                            device.set_temperature(self.config.temperature);
                            circuit.add_veriloga_device(device);
                            continue;
                        }
                    }

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
                    let params_map = resolve_supported_model_params_upper_map(
                        netlist,
                        model_def,
                        "Voltage-controlled switch",
                        &element.name,
                        model,
                        VSWITCH_MODEL_PARAMS,
                        self.config.temperature,
                    )?;

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
                    let params_map = resolve_supported_model_params_upper_map(
                        netlist,
                        model_def,
                        "Current-controlled switch",
                        &element.name,
                        model,
                        ISWITCH_MODEL_PARAMS,
                        self.config.temperature,
                    )?;

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
                ElementKind::GenericSwitch {
                    model,
                    control_expression,
                    initial_state,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);

                    let model_def = find_model_def(netlist, model).ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Generic switch '{}' references unknown model '{}'",
                            element.name, model
                        ))
                    })?;
                    ensure_model_type(
                        "Generic switch",
                        &element.name,
                        model,
                        model_def,
                        &["SW", "SWITCH"],
                    )?;
                    let params_map = resolve_supported_model_params_upper_map(
                        netlist,
                        model_def,
                        "Generic switch",
                        &element.name,
                        model,
                        GENERIC_SWITCH_MODEL_PARAMS,
                        self.config.temperature,
                    )?;

                    let mut sw = crate::device::GenericSwitch::new(
                        element.name.clone(),
                        np,
                        nn,
                        control_expression,
                    )
                    .map_err(SimulationError::Circuit)?
                    .with_params(&params_map);
                    if sw.has_solution_references() {
                        return Err(SimulationError::Circuit(format!(
                            "Generic switch '{}' CONTROL expression references circuit nodes or branch currents; native SWITCH CONTROL currently supports time/constant expressions only",
                            element.name
                        )));
                    }
                    if let Some(state) = initial_state {
                        sw = sw.with_initial_state(map_switch_state(*state));
                    }
                    circuit.generic_switches.push(sw);
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

                    let model_params = if let Some(name) = model.as_deref() {
                        resolve_tline_model_params(netlist, name)?
                    } else {
                        None
                    };

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

                    let txl_lossless_branch = model_params
                        .map(|params| params.uses_txl_lossless_branch())
                        .unwrap_or(false);
                    let attenuation = model_params.and_then(|p| {
                        if txl_lossless_branch {
                            None
                        } else {
                            tline_model_attenuation(p, z0_eff)
                        }
                    });
                    let loss_time_constant = model_params.and_then(|p| {
                        if txl_lossless_branch {
                            None
                        } else {
                            tline_model_loss_time_constant(p)
                        }
                    });
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
                                      p2n: usize,
                                      allow_native_txl: bool| {
                        let mut tline = crate::device::TransmissionLine::new(
                            name.clone(),
                            p1p,
                            p1n,
                            p2p,
                            p2n,
                            z0_eff,
                            delay,
                        );
                        tline.freq = freq_eff;
                        tline.nl = nl_eff;
                        tline.set_dc_series_resistance(dc_series_resistance);
                        if let Some(params) = model_params {
                            tline.set_ltra_breakpoint_tolerances(
                                params.rel.unwrap_or(1.0),
                                params.abs.unwrap_or(1.0),
                            );
                            if !params.is_txl() {
                                match params.ltra_interpolation {
                                    LtraInterpolationMode::Linear => {
                                        tline.set_ltra_linear_interpolation()
                                    }
                                    LtraInterpolationMode::Quadratic => {
                                        tline.set_ltra_quadratic_interpolation()
                                    }
                                    LtraInterpolationMode::Mixed => {
                                        tline.set_ltra_mixed_interpolation()
                                    }
                                }
                            }
                        }
                        let native_txl = if allow_native_txl
                            && !txl_lossless_branch
                            && let Some(params) = model_params
                            && params.is_txl()
                            && let (Some(r), Some(l), Some(g), Some(c), Some(len)) =
                                (params.r, params.l, params.g, params.c, params.len)
                        {
                            tline.enable_txl_runtime(r, l, g, c, len)
                        } else {
                            false
                        };
                        if native_txl {
                            let branch1 = circuit.allocate_branch_named(&format!("{}#ibr1", name));
                            let branch2 = circuit.allocate_branch_named(&format!("{}#ibr2", name));
                            tline.set_txl_branch_ordinals(branch1, branch2);
                        }
                        if !native_txl
                            && !txl_lossless_branch
                            && let Some(params) = model_params
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
                        if !native_txl
                            && tline.has_distributed_rlgc()
                            && let Some(params) = model_params
                            && !params.is_txl()
                        {
                            let branch1 = circuit.allocate_branch_named(&format!("{}#ibr1", name));
                            let branch2 = circuit.allocate_branch_named(&format!("{}#ibr2", name));
                            tline.set_ltra_branch_ordinals(branch1, branch2);
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
                            push_tline(
                                &mut circuit,
                                element.name.clone(),
                                p1p,
                                p1n,
                                p2p,
                                p2n,
                                true,
                            );
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
                                push_tline(&mut circuit, conductor_name, near, 0, far, 0, false);
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
                    expr_params,
                    string_params,
                    string_expr_params,
                    string_vector_params,
                    string_vector_expr_params,
                    real_vector_params,
                    real_vector_expr_params,
                } => {
                    let xspice_ramp_active =
                        self.config.ramptime.is_finite() && self.config.ramptime > 0.0;
                    if !xspice_ramp_active
                        && let Some(native_model) = resolve_native_xtradev_reactive_model(
                            netlist,
                            model,
                            &element.name,
                            params,
                            expr_params,
                            string_params,
                            string_expr_params,
                            string_vector_params,
                            string_vector_expr_params,
                            real_vector_params,
                            real_vector_expr_params,
                        )?
                    {
                        let lowered_to_native = match native_model {
                            NativeXtradevReactiveModel::Capacitor {
                                capacitance,
                                initial_voltage,
                            } => {
                                let (pos, neg) = xtradev_two_terminal_nodes(
                                    &element.name,
                                    model,
                                    "capacitoric",
                                    ports,
                                )?;
                                let np = circuit.get_or_create_node(&pos);
                                let nn = circuit.get_or_create_node(&neg);
                                if let Some(ic) = initial_voltage {
                                    circuit.capacitors.add_with_ic(
                                        element.name.clone(),
                                        np,
                                        nn,
                                        capacitance,
                                        ic,
                                    );
                                } else {
                                    circuit.capacitors.add(
                                        element.name.clone(),
                                        np,
                                        nn,
                                        capacitance,
                                    );
                                }
                                true
                            }
                            // ngspice inductoric is an XSPICE gd current-output
                            // model at DC/AC, not a native SPICE inductor.
                            NativeXtradevReactiveModel::Inductor { .. } => false,
                        };
                        if lowered_to_native {
                            log::debug!(
                                "Lowered XSPICE xtradev instance {} model={} to native reactive device",
                                element.name,
                                model
                            );
                            continue;
                        }
                    }

                    let resolved_model = resolve_xspice_model_instance(
                        netlist,
                        &circuit.xspice_registry,
                        model,
                        params,
                        expr_params,
                        string_params,
                        string_expr_params,
                        string_vector_params,
                        string_vector_expr_params,
                        real_vector_params,
                        real_vector_expr_params,
                    )
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Failed to resolve XSPICE model '{}' for element {}: {}",
                            model, element.name, e
                        ))
                    })?;

                    let mut numeric_params = resolved_model.numeric_params.clone();
                    if let Some(kind) = xspice_meter_kind(resolved_model.code_model.name()) {
                        let measured = xspice_meter_measured_value(
                            netlist,
                            &flat_elements,
                            &element.name,
                            model,
                            ports,
                            kind,
                            self.config.temperature,
                        )?;
                        numeric_params.push((
                            crate::xspice::models::XTRADEV_METER_MEASURED_VALUE_PARAM.to_string(),
                            measured,
                        ));
                    }

                    let ports_spec = resolved_model.code_model.ports().to_vec();
                    let connections = coerce_xspice_connections(
                        &mut circuit,
                        &ports_spec,
                        ports,
                        &element.name,
                        resolved_model.code_model.name(),
                    )?;

                    let mut instance = crate::xspice::XspiceInstance::new_with_string_vectors(
                        element.name.clone(),
                        resolved_model.code_model.clone(),
                        connections,
                        &numeric_params,
                        &resolved_model.string_params,
                        &resolved_model.string_vector_params,
                        &resolved_model.real_vector_params,
                        &resolved_model.integer_vector_params,
                    )
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Failed to create XSPICE instance '{}': {}",
                            element.name, e
                        ))
                    })?;

                    instance.set_temperature(self.config.temperature);
                    instance.set_ramptime(self.config.ramptime);

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

                        let connection = instance.connection_at(port_idx).cloned();
                        match connection {
                            Some(crate::xspice::PortConnection::Analog(_))
                            | Some(crate::xspice::PortConnection::Differential(_, _)) => {
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
                            Some(crate::xspice::PortConnection::AnalogVector(nodes)) => {
                                for element_idx in 0..nodes.len() {
                                    let branch_name = format!(
                                        "{}#{}[{}]",
                                        element.name, port_spec.name, element_idx
                                    );
                                    let branch_ordinal =
                                        circuit.allocate_branch_named(&branch_name);
                                    instance
                                        .set_output_vector_branch(
                                            port_idx,
                                            element_idx,
                                            branch_ordinal,
                                        )
                                        .map_err(|e| {
                                            SimulationError::Circuit(format!(
                                                "Failed to assign branch for XSPICE instance '{}' port '{}[{}]': {}",
                                                element.name, port_spec.name, element_idx, e
                                            ))
                                        })?;
                                }
                            }
                            Some(crate::xspice::PortConnection::TypedAnalogVector(elements)) => {
                                for (element_idx, element_connection) in elements.iter().enumerate()
                                {
                                    let needs_voltage_branch = matches!(
                                        element_connection,
                                        crate::xspice::AnalogInputConnection::Node(_)
                                            | crate::xspice::AnalogInputConnection::Differential(
                                                _,
                                                _
                                            )
                                    );
                                    if !needs_voltage_branch {
                                        continue;
                                    }
                                    let branch_name = format!(
                                        "{}#{}[{}]",
                                        element.name, port_spec.name, element_idx
                                    );
                                    let branch_ordinal =
                                        circuit.allocate_branch_named(&branch_name);
                                    instance
                                        .set_output_vector_branch(
                                            port_idx,
                                            element_idx,
                                            branch_ordinal,
                                        )
                                        .map_err(|e| {
                                            SimulationError::Circuit(format!(
                                                "Failed to assign branch for XSPICE instance '{}' port '{}[{}]': {}",
                                                element.name, port_spec.name, element_idx, e
                                            ))
                                        })?;
                                }
                            }
                            _ => {}
                        }
                    }

                    instance.init().map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Failed to initialize XSPICE instance '{}': {}",
                            element.name, e
                        ))
                    })?;

                    circuit.add_xspice_instance(instance);
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
        circuit
            .resolve_xspice_branch_references()
            .map_err(|e| SimulationError::Circuit(e.to_string()))?;

        // Resolve K couplings into mutual-coupling overlays now that every
        // inductor and its branch ordinal exist. The standalone inductors keep
        // their full self-inductance stamps; each pair contributes ONLY the
        // mutual terms (see CoupledInductorPair). K cards with 3+ inductors
        // couple every pair with the same k (ngspice semantics).
        let couplings = std::mem::take(&mut circuit.couplings);
        for coupling in &couplings {
            if coupling.inductor_names.len() < 2 {
                return Err(SimulationError::Circuit(format!(
                    "coupling {} names fewer than two inductors",
                    coupling.name
                )));
            }
            let mut indices = Vec::with_capacity(coupling.inductor_names.len());
            for lname in &coupling.inductor_names {
                let idx = circuit
                    .inductors
                    .names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(lname))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "coupling {} references unknown inductor {}",
                            coupling.name, lname
                        ))
                    })?;
                indices.push(idx);
            }
            for a in 0..indices.len() {
                for b in (a + 1)..indices.len() {
                    let (i, j) = (indices[a], indices[b]);
                    let device = crate::device::CoupledInductorPair::new(
                        coupling.name.clone(),
                        circuit.inductors.node_pos[i],
                        circuit.inductors.node_neg[i],
                        circuit.inductors.inductances[i],
                        circuit.inductors.node_pos[j],
                        circuit.inductors.node_neg[j],
                        circuit.inductors.inductances[j],
                        coupling.coefficient,
                    );
                    circuit.add_coupled_inductor_pair(
                        circuit.inductors.branch_indices[i],
                        circuit.inductors.branch_indices[j],
                        device,
                    );
                }
            }
        }
        circuit.couplings = couplings;

        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        for mos in &mut circuit.mosfets.devices {
            mos.set_junction_gmin(junction_gmin);
        }
        for jfet in &mut circuit.jfets {
            jfet.set_junction_gmin(junction_gmin);
        }
        for dev in &mut circuit.bsim3v3.devices {
            dev.set_eval_gmin(junction_gmin);
        }
        for dev in &mut circuit.bsim4v8.devices {
            dev.set_eval_gmin(junction_gmin);
        }
        let b3soi_gmin = junction_gmin * circuit.b3soi_gmin_scale.max(0.0);
        for dev in &mut circuit.b3soi.devices {
            dev.set_eval_gmin(b3soi_gmin);
        }
        for dev in &mut circuit.b3soi_fd.devices {
            dev.set_eval_gmin(b3soi_gmin);
        }
        for dev in &mut circuit.b3soi_pd.devices {
            dev.set_eval_gmin(b3soi_gmin);
        }

        Ok(circuit)
    }
}
