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
    register_precompiled_veriloga_model_with_dependencies, veriloga_cache_entries,
    veriloga_cache_stats,
};
#[cfg(feature = "veriloga")]
use veriloga_cache::{normalize_model_key, resolve_cached_or_compile_veriloga};

/// Construct a Jiles-Atherton (magnetic-core) inductor instance and add it,
/// together with its linear runtime companion, to the circuit.
fn add_jiles_atherton_inductor_element(
    circuit: &mut CircuitData,
    netlist: &Netlist,
    element: &crate::netlist::Element,
    value: f64,
    model: &str,
    initial_current: Option<f64>,
) -> Result<(), SimulationError> {
    if !value.is_finite() || value <= 0.0 {
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

    let params = resolve_jiles_atherton_model_params(model_def, value)?;
    let mut ja =
        crate::device::passive::JilesAthertonInductor::new(element.name.clone(), np, nn)
            .with_params(params);
    if let Some(ic) = initial_current {
        ja.set_initial_current(ic);
    }

    let effective_l = ja.effective_inductance();
    let runtime_l = if effective_l.is_finite() && effective_l > 0.0 {
        effective_l
    } else {
        value
    };

    if let Some(ic) = initial_current {
        circuit
            .inductors
            .add_with_ic(element.name.clone(), np, nn, branch, runtime_l, ic);
    } else {
        circuit
            .inductors
            .add(element.name.clone(), np, nn, branch, runtime_l);
    }

    let inductor_index = circuit.inductors.len().saturating_sub(1);
    circuit.add_jiles_atherton_inductor(inductor_index, branch, ja);

    Ok(())
}

/// `true` when a model card's type names a magnetic-core (Jiles-Atherton)
/// model rather than a linear inductor card.
fn is_magnetic_core_model_type(model_type: &str) -> bool {
    model_type.eq_ignore_ascii_case("CORE")
        || model_type.eq_ignore_ascii_case("JA")
        || model_type.eq_ignore_ascii_case("JILES")
        || model_type.eq_ignore_ascii_case("JILESATHERTON")
}

/// MOS model levels with a native bulk-MOSFET implementation: Berkeley
/// MOS1/MOS2/MOS6 (1/2/6) and the legacy BSIM1/BSIM2 ports (4/5). Levels
/// 8/49 (BSIM3v3.3) and 55-57 (BSIM3-SOI) are routed to dedicated devices
/// before this check applies.
fn native_bulk_mos_level(level: i32) -> bool {
    matches!(level, 1 | 2 | 4 | 5 | 6)
}

/// Warn about nodes with no conductive path to ground: the unconditional
/// matrix gmin keeps such systems numerically solvable, so without this
/// notice a forgotten connection simulates silently with a meaningless bias.
/// Capacitors and current sources do not conduct DC; every other element's
/// terminals are treated as connected, which never produces a false alarm.
fn warn_floating_nodes(flat_elements: &[crate::netlist::Element]) {
    use crate::netlist::ElementKind;
    use std::collections::HashMap;

    let canonical = |node: &str| -> String {
        let lower = node.to_ascii_lowercase();
        if lower == "gnd" { "0".to_string() } else { lower }
    };

    let mut parent: HashMap<String, String> = HashMap::new();
    fn find(parent: &mut HashMap<String, String>, node: &str) -> String {
        let mut current = node.to_string();
        loop {
            let up = parent
                .entry(current.clone())
                .or_insert_with(|| current.clone())
                .clone();
            if up == current {
                return current;
            }
            let grand = parent.get(&up).cloned().unwrap_or_else(|| up.clone());
            parent.insert(current, grand);
            current = up;
        }
    }

    let mut all_nodes: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for element in flat_elements {
        let conducts = !matches!(
            element.kind,
            ElementKind::Capacitor { .. } | ElementKind::CurrentSource(_)
        );
        let nodes: Vec<String> = element.nodes.iter().map(|n| canonical(n)).collect();
        for node in &nodes {
            if seen.insert(node.clone()) {
                all_nodes.push(node.clone());
            }
        }
        if conducts && nodes.len() >= 2 {
            let first = find(&mut parent, &nodes[0]);
            for other in &nodes[1..] {
                let root = find(&mut parent, other);
                if root != first {
                    parent.insert(root, first.clone());
                }
            }
        }
    }

    if !seen.contains("0") {
        // No ground reference at all; other validation owns that report.
        return;
    }
    let ground_root = find(&mut parent, "0");
    let floating: Vec<&String> = all_nodes
        .iter()
        .filter(|node| node.as_str() != "0" && find(&mut parent, node) != ground_root)
        .collect();
    if floating.is_empty() {
        return;
    }
    let shown: Vec<&str> = floating.iter().take(8).map(|s| s.as_str()).collect();
    let suffix = if floating.len() > shown.len() {
        format!(" (and {} more)", floating.len() - shown.len())
    } else {
        String::new()
    };
    log::warn!(
        "node(s) {}{} have no conductive path to ground (capacitors and current sources \
         do not conduct DC); their bias is set by the matrix gmin only and is not \
         physically meaningful",
        shown.join(", "),
        suffix
    );
}

/// Diagnostic descriptor for well-known MOS model levels.
fn mos_level_descriptor(level: i32) -> String {
    match level {
        3 => "LEVEL=3 (MOS3)".to_string(),
        8 | 49 | 53 => format!("LEVEL={level} (BSIM3v3)"),
        14 | 54 => format!("LEVEL={level} (BSIM4)"),
        _ => format!("LEVEL={level}"),
    }
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

        // One shared Arc per model: instances share the (megabyte-scale)
        // program and a single JIT compilation
        #[cfg(feature = "veriloga")]
        let mut veriloga_models: HashMap<String, std::sync::Arc<rspice_veriloga::CompiledModel>> =
            HashMap::new();

        // One shared BSIM3v3.3 card + temperature block per .model name,
        // with the (W, L) size knots memoized across instances.
        let mut bsim3v3_models: HashMap<String, Bsim3v3SharedModel> = HashMap::new();

        // Load and cache Verilog-A models referenced by .VERILOGA directives.
        #[cfg(feature = "veriloga")]
        {
            for include in &netlist.veriloga_includes {
                let model = std::sync::Arc::new(resolve_cached_or_compile_veriloga(
                    &include.file_path,
                )?);

                let model_key = normalize_model_key(model.name.as_str());
                veriloga_models
                    .entry(model_key)
                    .or_insert_with(|| std::sync::Arc::clone(&model));

                if let Some(alias) = include.model_name.as_deref() {
                    veriloga_models
                        .entry(normalize_model_key(alias))
                        .or_insert_with(|| std::sync::Arc::clone(&model));
                }

                if let Some(stem) = include.file_path.file_stem().and_then(|s| s.to_str()) {
                    veriloga_models
                        .entry(normalize_model_key(stem))
                        .or_insert_with(|| std::sync::Arc::clone(&model));
                }

                log::info!(
                    "Loaded Verilog-A model '{}' from {}",
                    model.name,
                    include.file_path.display()
                );
            }
        }

        warn_floating_nodes(&flat_elements);

        // Deduplicates the loud simplified-MOS warnings to one per model card.
        let mut simplified_mos_warned: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for element in &flat_elements {
            match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    ..
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
                    // Per-instance thermal-noise temperature, resnoise.c
                    // semantics: with TEMP given the offset is
                    // temp − CKTtemp + tnom (in Celsius terms, ngspice's
                    // own quirk); otherwise DTEMP is the offset directly.
                    let noise_dtemp = if let Some(temp) =
                        instance_param(instance_params, &["TEMP"])
                    {
                        let temp_k =
                            crate::analysis::temperature::celsius_to_kelvin(temp);
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
                        circuit
                            .capacitors
                            .add_with_ic(element.name.clone(), np, nn, capacitance, ic);
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
                ElementKind::Diode {
                    model,
                    instance_params,
                    ..
                } => {
                    let anode = circuit.get_or_create_node(&element.nodes[0]);
                    let cathode = circuit.get_or_create_node(&element.nodes[1]);
                    // Model cards start from ngspice's defaults: parameters a
                    // card omits must mean what they mean in SPICE, not
                    // inherit the 1N4148-like convenience values.
                    let mut diode =
                        crate::device::Diode::spice_defaults(element.name.clone(), anode, cathode);

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
                    let junction_scale = area * mult;
                    if junction_scale != 1.0 {
                        diode.apply_junction_scaling(junction_scale);
                    }
                    diode.multiplicity = mult;

                    // Junction temperature: instance TEMP is absolute (C),
                    // DTEMP offsets the circuit temperature; the model TNOM
                    // (or .options tnom) anchors the scaling.
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
                            temp_k - self.config.temperature
                                + netlist.options.tnom.unwrap_or(27.0)
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
                    ..
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
                    bjt.refresh_noise_temperature_offset(
                        self.config.temperature,
                        netlist.options.tnom.unwrap_or(27.0),
                    );
                    bjt.set_substrate_node(substrate);

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
                    } else {
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
                    instance_params,
                    ..
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

                    // BSIMSOI variants are distinct devices with their own SOI node
                    // topology and charge model. Route each level to its port:
                    // 55 -> FD (fully depleted), 56 -> DD (dynamic depletion),
                    // 57 -> PD (partially depleted).
                    if is_bsimsoi_level(level) {
                        if let Some(params_map) = params_map.as_ref() {
                            match level {
                                55 => {
                                    Self::build_b3soi_fd(
                                        &mut circuit,
                                        element,
                                        resolved_mos_type,
                                        params_map,
                                        instance_params,
                                        self.config.temperature,
                                    )?;
                                    continue;
                                }
                                56 => {
                                    Self::build_b3soi_dd(
                                        &mut circuit,
                                        element,
                                        resolved_mos_type,
                                        params_map,
                                        instance_params,
                                        self.config.temperature,
                                    )?;
                                    continue;
                                }
                                57 => {
                                    Self::build_b3soi_pd(
                                        &mut circuit,
                                        element,
                                        resolved_mos_type,
                                        params_map,
                                        instance_params,
                                        self.config.temperature,
                                    )?;
                                    continue;
                                }
                                _ => {}
                            }
                        }
                    }

                    // BSIM3v3.3: LEVEL=8/49 cards route to the native port
                    // (LEVEL=54 stays on the BSIM4 rejection below). One
                    // shared model card + temperature block per .model; size
                    // knots are memoized across instances exactly as ngspice
                    // reuses pSizeDependParamKnot.
                    if matches!(level, 8 | 49)
                        && let Some(params_map) = params_map.as_ref()
                    {
                        let model_key =
                            model_def.map_or_else(|| model.clone(), |def| def.name.clone());
                        let tnom_default_k = crate::analysis::temperature::celsius_to_kelvin(
                            netlist.options.tnom.unwrap_or(27.0),
                        );
                        Self::build_bsim3v3(
                            &mut circuit,
                            element,
                            resolved_mos_type,
                            &model_key,
                            params_map,
                            instance_params,
                            self.config.temperature,
                            tnom_default_k,
                            &mut bsim3v3_models,
                        )?;
                        continue;
                    }

                    // Levels without a native implementation must not fall
                    // through to the simplified short-channel approximation
                    // silently: a BSIM3/BSIM4 card evaluated with ~15 honored
                    // parameters yields plausible-looking but wrong currents,
                    // which is strictly worse than an error. LEVEL=3 remains
                    // runnable with a warning — the approximation is of the
                    // same empirical family and the vendored ngspice MOS3
                    // oracle deck passes within suite tolerance — while
                    // BSIM-class levels require an explicit
                    // `.options allow_simplified_mos=1` opt-in.
                    if !native_bulk_mos_level(level) {
                        let descriptor = mos_level_descriptor(level);
                        if level == 3 {
                            if simplified_mos_warned.insert(model.clone()) {
                                log::warn!(
                                    "MOSFET model '{model}' (LEVEL=3): no native MOS3 \
                                     implementation; using a simplified short-channel \
                                     approximation. MOS3-specific parameters (THETA, ETA, \
                                     KAPPA, NFS, VMAX, XJ, DELTA) are not honored."
                                );
                            }
                        } else if netlist.options.allow_simplified_mos == Some(true) {
                            if simplified_mos_warned.insert(model.clone()) {
                                log::warn!(
                                    "MOSFET model '{model}' ({descriptor}): not implemented \
                                     natively; running the simplified short-channel \
                                     approximation because `.options allow_simplified_mos` \
                                     is set. Results will NOT match {descriptor}."
                                );
                            }
                        } else {
                            return Err(SimulationError::Circuit(format!(
                                "MOSFET '{}': model '{}' requests {} which has no native \
                                 implementation. Supported levels: 1, 2, 6 (Berkeley \
                                 MOS1/MOS2/MOS6), 4/5 (legacy BSIM1/BSIM2), 55-57 \
                                 (BSIM3-SOI FD/DD/PD). For BSIM4 accuracy use the bundled \
                                 Verilog-A model (models/veriloga/bsim4.va); to knowingly \
                                 run a simplified ~15-parameter approximation instead, set \
                                 `.options allow_simplified_mos=1`.",
                                element.name, model, descriptor
                            )));
                        }
                    }

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
                            temp_k - self.config.temperature
                                + netlist.options.tnom.unwrap_or(27.0)
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
                            circuit.resistors.set_last_noise_temperature_offset(
                                mosfet.noise_temperature_offset,
                            );
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
                            circuit.resistors.set_last_noise_temperature_offset(
                                mosfet.noise_temperature_offset,
                            );
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
                        // ngspice's NJF/PJF LEVEL=2 is the Parker-Skellern
                        // JFET2 model, which has no port here; running the
                        // level-1 equations silently would be wrong physics.
                        if params_map
                            .get("LEVEL")
                            .copied()
                            .is_some_and(|level| level.is_finite() && level.round() as i32 == 2)
                        {
                            log::warn!(
                                "JFET '{}': LEVEL=2 (Parker-Skellern JFET2) is not \
                                 implemented; using the level-1 Shichman-Hodges model",
                                element.name
                            );
                        }
                        jfet = jfet.with_model_params(&params_map);
                    }
                    jfet = jfet.with_instance_params(instance_params);
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
                    // ngspice selects HFET1 either by the NHFET/PHFET model
                    // type or by NMF/PMF with LEVEL=5 (the z-device level
                    // map: 1 = MES, 2-4 = MESA, 5 = HFET1).
                    let card_is_hfet_level = params_map
                        .as_ref()
                        .and_then(|params| params.get("LEVEL").copied())
                        .is_some_and(|level| {
                            level.is_finite() && level.round() as i32 == 5
                        });
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
                            std::sync::Arc::clone(model),
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
                                    crate::netlist::expr::eval_expression(expr, &netlist.params)
                                        .map_err(|e| {
                                            SimulationError::Circuit(format!(
                                                "Failed to resolve Verilog-A parameter '{}': {}",
                                                name, e
                                            ))
                                        })?
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
                        device.resolve_parameter_defaults();
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

        let junction_gmin = self
            .config
            .convergence_config
            .gmin_initial
            .max(self.config.convergence_config.gmin_target)
            .max(0.0);
        for mos in &mut circuit.mosfets.devices {
            mos.set_junction_gmin(junction_gmin);
        }
        for jfet in &mut circuit.jfets {
            jfet.set_junction_gmin(junction_gmin);
        }
        for dev in &mut circuit.bsim3v3.devices {
            dev.set_eval_gmin(junction_gmin);
        }

        Ok(circuit)
    }

    /// Build and register a BSIMSOI dynamic-depletion (level 56) instance.
    ///
    /// Node topology (b3soiddset.c:975-1037):
    /// - 4-terminal `m d g s e`: floating body. An internal body node
    ///   `<name>.__body.internal` is allocated; `bodyMod = 0`, `float = 1`.
    /// - 5-terminal `m d g s e p`: ideal body tie. The external `p` node *is* the
    ///   body node; `bodyMod = 2`, no internal node.
    ///
    /// The drain/source primes coincide with the external drain/source (no series
    /// R in the supported decks) and there is no self-heating temperature node
    /// (SHMOD=0). Series-R / RSH support and bodyMod==1 are deferred with the
    /// FD/PD siblings.
    fn build_b3soi_dd(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        temperature_kelvin: f64,
    ) -> Result<(), SimulationError> {
        use crate::device::{B3SoiDd, B3SoiDdModel, BodyMode};
        use crate::device::mosfet::b3soi::dd::temp::B3SoiDdGeometry;

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        // `config.temperature` is already in Kelvin (`TEMP_REFERENCE`).
        let temp_k = temperature_kelvin;

        // ngspice TNOM defaults to 27C; the model card may override it (Celsius).
        let tnom_c = params_map.get("TNOM").copied().unwrap_or(27.0);
        let tnom_k = crate::analysis::temperature::celsius_to_kelvin(tnom_c);

        let model = std::sync::Arc::new(B3SoiDdModel::from_params(params_map, is_pmos, tnom_k));

        let node_drain = circuit.get_or_create_node(&element.nodes[0]);
        let node_gate = circuit.get_or_create_node(&element.nodes[1]);
        let node_source = circuit.get_or_create_node(&element.nodes[2]);
        let node_e = circuit.get_or_create_node(&element.nodes[3]);

        let (node_body, node_p, body_mode) = if element.nodes.len() > 4 {
            // Ideal body tie: P is the body node.
            let p = circuit.get_or_create_node(&element.nodes[4]);
            (p, p, BodyMode::TiedIdeal)
        } else {
            // Floating body: allocate an internal body node.
            let body =
                circuit.get_or_create_node(&format!("{}.__body.internal", element.name));
            (body, 0, BodyMode::Floating)
        };

        // Instance geometry (W/L plus optional area/perimeter/squares).
        let l = instance_param(instance_params, &["L"]).unwrap_or(0.0);
        let w = instance_param(instance_params, &["W"]).unwrap_or(0.0);
        let geom = B3SoiDdGeometry {
            l,
            w,
            drain_area: instance_param(instance_params, &["AD"]).unwrap_or(0.0),
            source_area: instance_param(instance_params, &["AS"]).unwrap_or(0.0),
            drain_squares: instance_param(instance_params, &["NRD"]).unwrap_or(0.0),
            source_squares: instance_param(instance_params, &["NRS"]).unwrap_or(0.0),
            drain_perimeter: instance_param(instance_params, &["PD"]).unwrap_or(0.0),
            source_perimeter: instance_param(instance_params, &["PS"]).unwrap_or(0.0),
            body_squares: instance_param(instance_params, &["NRB"]).unwrap_or(0.0),
            rth0: params_map.get("RTH0").copied().unwrap_or(0.0),
            cth0: params_map.get("CTH0").copied().unwrap_or(0.0),
        };

        let mut device = B3SoiDd::new(
            element.name.clone(),
            node_drain,
            node_gate,
            node_source,
            node_e,
            node_body,
            node_p,
            body_mode,
            model,
            geom,
            temp_k,
        )
        .map_err(SimulationError::Circuit)?;

        // DEBUG=-1 runs the device without dynamic charges (ngspice debugMod).
        if let Some(debug) = instance_param(instance_params, &["DEBUG"]) {
            device.set_debug_mod(debug.round() as i32);
        }
        circuit.b3soi.add(device);
        Ok(())
    }

    /// Build and register a BSIMSOI fully-depleted (level 55) instance.
    ///
    /// Node topology (b3soifdset.c): FD never solves the body as a circuit node.
    /// - 4-terminal `m d g s e`: floating body, `bNode = 0` — no internal node is
    ///   created (the body voltage is pinned to `Vbs0eff` in the load).
    /// - 5-terminal `m d g s e b`: body contact present. The external `b` node is
    ///   read for the initial guess but the load still pins `Vbs = Vbs0eff`.
    ///
    /// The drain/source primes coincide with the external drain/source (no series
    /// R in the supported decks) and there is no self-heating node (SHMOD=0).
    fn build_b3soi_fd(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        temperature_kelvin: f64,
    ) -> Result<(), SimulationError> {
        use crate::device::B3SoiFd;
        use crate::device::B3SoiFdModel;
        use crate::device::mosfet::b3soi::fd::BodyMode;
        use crate::device::mosfet::b3soi::fd::temp::B3SoiFdGeometry;

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        let temp_k = temperature_kelvin;
        let tnom_c = params_map.get("TNOM").copied().unwrap_or(27.0);
        let tnom_k = crate::analysis::temperature::celsius_to_kelvin(tnom_c);

        let model = std::sync::Arc::new(B3SoiFdModel::from_params(params_map, is_pmos, tnom_k));

        let node_drain = circuit.get_or_create_node(&element.nodes[0]);
        let node_gate = circuit.get_or_create_node(&element.nodes[1]);
        let node_source = circuit.get_or_create_node(&element.nodes[2]);
        let node_e = circuit.get_or_create_node(&element.nodes[3]);

        let (node_body, body_mode) = if element.nodes.len() > 4 {
            // Body contact: read its node for the initial guess only.
            let b = circuit.get_or_create_node(&element.nodes[4]);
            (b, BodyMode::TiedIdeal)
        } else {
            // Floating body: FD allocates no body node.
            (0, BodyMode::Floating)
        };

        let l = instance_param(instance_params, &["L"]).unwrap_or(0.0);
        let w = instance_param(instance_params, &["W"]).unwrap_or(0.0);
        let geom = B3SoiFdGeometry {
            l,
            w,
            drain_area: instance_param(instance_params, &["AD"]).unwrap_or(0.0),
            source_area: instance_param(instance_params, &["AS"]).unwrap_or(0.0),
            drain_squares: instance_param(instance_params, &["NRD"]).unwrap_or(0.0),
            source_squares: instance_param(instance_params, &["NRS"]).unwrap_or(0.0),
            drain_perimeter: instance_param(instance_params, &["PD"]).unwrap_or(0.0),
            source_perimeter: instance_param(instance_params, &["PS"]).unwrap_or(0.0),
            body_squares: instance_param(instance_params, &["NRB"]).unwrap_or(0.0),
            rth0: params_map.get("RTH0").copied().unwrap_or(0.0),
            cth0: params_map.get("CTH0").copied().unwrap_or(0.0),
        };

        let mut device = B3SoiFd::new(
            element.name.clone(),
            node_drain,
            node_gate,
            node_source,
            node_e,
            node_body,
            body_mode,
            model,
            geom,
            temp_k,
        )
        .map_err(SimulationError::Circuit)?;

        // DEBUG=-1 runs the device without dynamic charges (ngspice debugMod).
        if let Some(debug) = instance_param(instance_params, &["DEBUG"]) {
            device.set_debug_mod(debug.round() as i32);
        }
        circuit.b3soi_fd.add(device);
        Ok(())
    }

    /// Build and register a BSIMSOI partially-depleted (level 57) instance.
    ///
    /// Node topology (b3soipdset.c) matches DD: a 4-terminal `m d g s e` device
    /// has a floating body modeled with an internal node
    /// `<name>.__body.internal` (`bodyMod = 0`); a 5-terminal `m d g s e b`
    /// device is a body tie. With `rbody == rbsh == 0` it is an ideal tie
    /// (`bodyMod = 2`, the external `b` *is* the body node); otherwise it is a
    /// nonideal tie (`bodyMod = 1`) whose body resistor is folded into the body
    /// stamping. The supported PD decks use `rbody = 1`, so 5-terminal `t4` is a
    /// nonideal tie.
    fn build_b3soi_pd(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        temperature_kelvin: f64,
    ) -> Result<(), SimulationError> {
        use crate::device::B3SoiPd;
        use crate::device::B3SoiPdModel;
        use crate::device::mosfet::b3soi::pd::BodyMode;
        use crate::device::mosfet::b3soi::pd::temp::B3SoiPdGeometry;

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        let temp_k = temperature_kelvin;
        let tnom_c = params_map.get("TNOM").copied().unwrap_or(27.0);
        let tnom_k = crate::analysis::temperature::celsius_to_kelvin(tnom_c);

        let model = std::sync::Arc::new(B3SoiPdModel::from_params(params_map, is_pmos, tnom_k));

        let node_drain = circuit.get_or_create_node(&element.nodes[0]);
        let node_gate = circuit.get_or_create_node(&element.nodes[1]);
        let node_source = circuit.get_or_create_node(&element.nodes[2]);
        let node_e = circuit.get_or_create_node(&element.nodes[3]);

        let rbody = params_map.get("RBODY").copied().unwrap_or(0.0);
        let rbsh = params_map.get("RBSH").copied().unwrap_or(0.0);
        let ideal_tie = rbody == 0.0 && rbsh == 0.0;

        let (node_body, node_p, body_mode) = if element.nodes.len() > 4 {
            let b = circuit.get_or_create_node(&element.nodes[4]);
            if ideal_tie {
                // Ideal body tie: the external contact is the body node.
                (b, b, BodyMode::TiedIdeal)
            } else {
                // Nonideal body tie: an internal body node sits behind the body
                // resistor; the external contact is the `p` node.
                let body =
                    circuit.get_or_create_node(&format!("{}.__body.internal", element.name));
                (body, b, BodyMode::TiedResistive)
            }
        } else {
            // Floating body: allocate an internal body node.
            let body = circuit.get_or_create_node(&format!("{}.__body.internal", element.name));
            (body, 0, BodyMode::Floating)
        };

        let l = instance_param(instance_params, &["L"]).unwrap_or(0.0);
        let w = instance_param(instance_params, &["W"]).unwrap_or(0.0);
        let geom = B3SoiPdGeometry {
            l,
            w,
            drain_area: instance_param(instance_params, &["AD"]).unwrap_or(0.0),
            source_area: instance_param(instance_params, &["AS"]).unwrap_or(0.0),
            drain_squares: instance_param(instance_params, &["NRD"]).unwrap_or(0.0),
            source_squares: instance_param(instance_params, &["NRS"]).unwrap_or(0.0),
            drain_perimeter: instance_param(instance_params, &["PD"]).unwrap_or(0.0),
            source_perimeter: instance_param(instance_params, &["PS"]).unwrap_or(0.0),
            body_squares: instance_param(instance_params, &["NRB"]).unwrap_or(0.0),
            rth0: params_map.get("RTH0").copied().unwrap_or(0.0),
            cth0: params_map.get("CTH0").copied().unwrap_or(0.0),
        };

        let mut device = B3SoiPd::new(
            element.name.clone(),
            node_drain,
            node_gate,
            node_source,
            node_e,
            node_body,
            node_p,
            body_mode,
            model,
            geom,
            temp_k,
        )
        .map_err(SimulationError::Circuit)?;

        // DEBUG=-1 runs the device without dynamic charges (ngspice debugMod).
        if let Some(debug) = instance_param(instance_params, &["DEBUG"]) {
            device.set_debug_mod(debug.round() as i32);
        }
        circuit.b3soi_pd.add(device);
        Ok(())
    }

    /// Build and register a native BSIM3v3.3 (MOS level 8/49) instance.
    ///
    /// Topology is the standard 4-terminal bulk MOSFET `m d g s b`. Series
    /// drain/source resistance follows b3temp.c: a conductance of
    /// `1 / (RSH * NRD)` (resp. NRS) exists only when both factors are
    /// positive; it is lowered to an ordinary linear resistor of
    /// `RSH * NRD / M` ohms at an internal prime node, and the device's
    /// drain/source point at the primes (ngspice stamps `m *
    /// drainConductance` between dNode and dNodePrime, b3ld.c:3050).
    #[allow(clippy::too_many_arguments)]
    fn build_bsim3v3(
        circuit: &mut CircuitData,
        element: &crate::netlist::Element,
        mos_type: crate::netlist::MosType,
        model_key: &str,
        params_map: &HashMap<String, f64>,
        instance_params: &[(String, f64)],
        temperature_kelvin: f64,
        tnom_default_k: f64,
        shared: &mut HashMap<String, Bsim3v3SharedModel>,
    ) -> Result<(), SimulationError> {
        use crate::device::mosfet::bsim3v3::{
            Bsim3v3, Bsim3v3Geometry, Bsim3v3Model, Bsim3v3ModelTemp, SizeDepCache,
        };
        use crate::device::Bsim3v3Device;

        let is_pmos = matches!(mos_type, crate::netlist::MosType::Pmos);
        // BSIM3v3.3 has no instance TEMP/DTEMP (b3set.c); every instance
        // evaluates at the circuit temperature, like ngspice's CKTtemp.
        let temp_k = temperature_kelvin;

        let entry = match shared.entry(model_key.to_string()) {
            std::collections::hash_map::Entry::Occupied(occupied) => occupied.into_mut(),
            std::collections::hash_map::Entry::Vacant(vacant) => {
                let model = std::sync::Arc::new(Bsim3v3Model::from_params(
                    params_map,
                    is_pmos,
                    tnom_default_k,
                ));
                // The charge model implements CAPMOD=3 (the BSIM3v3.3
                // default); transient/AC under CAPMOD 0-2 would need the
                // unported charge equations, so reject the card up front
                // rather than failing mid-analysis. XPART<0 (intrinsic
                // charge suppression) remains honored.
                if model.cap_mod != 3 && model.xpart >= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "MOSFET '{}': BSIM3 model '{}' requests CAPMOD={} which is not \
                         implemented (only CAPMOD=3, the BSIM3v3.3 default)",
                        element.name, model_key, model.cap_mod
                    )));
                }
                let model_temp = std::sync::Arc::new(Bsim3v3ModelTemp::new(&model, temp_k));
                vacant.insert(Bsim3v3SharedModel {
                    model,
                    model_temp,
                    size_cache: SizeDepCache::new(),
                })
            }
        };

        let multiplier = instance_param(instance_params, &["M"])
            .filter(|m| m.is_finite() && *m > 0.0)
            .unwrap_or(1.0);
        let defaults = Bsim3v3Geometry::default();
        let geom = Bsim3v3Geometry {
            l: instance_param(instance_params, &["L"]).unwrap_or(defaults.l),
            w: instance_param(instance_params, &["W"]).unwrap_or(defaults.w),
            m: multiplier,
            drain_area: instance_param(instance_params, &["AD"]).unwrap_or(0.0),
            source_area: instance_param(instance_params, &["AS"]).unwrap_or(0.0),
            drain_squares: instance_param(instance_params, &["NRD"])
                .unwrap_or(defaults.drain_squares),
            source_squares: instance_param(instance_params, &["NRS"])
                .unwrap_or(defaults.source_squares),
            drain_perimeter: instance_param(instance_params, &["PD"]).unwrap_or(0.0),
            source_perimeter: instance_param(instance_params, &["PS"]).unwrap_or(0.0),
            delvto: instance_param(instance_params, &["DELVTO", "DELVT0"]).unwrap_or(0.0),
            mulu0: instance_param(instance_params, &["MULU0"]).unwrap_or(1.0),
            ..defaults
        };

        let core = Bsim3v3::new_shared(
            element.name.clone(),
            std::sync::Arc::clone(&entry.model),
            std::sync::Arc::clone(&entry.model_temp),
            &mut entry.size_cache,
            geom,
        )
        .map_err(SimulationError::Circuit)?;

        let drain_external = circuit.get_or_create_node(&element.nodes[0]);
        let gate = circuit.get_or_create_node(&element.nodes[1]);
        let source_external = circuit.get_or_create_node(&element.nodes[2]);
        let bulk = circuit.get_or_create_node(&element.nodes[3]);

        // Internal prime nodes only when the series conductance exists
        // (drain_conductance = 1/(RSH*NRD) > 0, b3temp.c:811-851).
        let drain = if core.inst.drain_conductance > 0.0 {
            let dint = circuit.get_or_create_node(&format!("{}.__dint", element.name));
            circuit.resistors.add(
                format!("{}.__rd", element.name),
                drain_external,
                dint,
                1.0 / (core.inst.drain_conductance * multiplier),
            );
            dint
        } else {
            drain_external
        };
        let source = if core.inst.source_conductance > 0.0 {
            let sint = circuit.get_or_create_node(&format!("{}.__sint", element.name));
            circuit.resistors.add(
                format!("{}.__rs", element.name),
                source_external,
                sint,
                1.0 / (core.inst.source_conductance * multiplier),
            );
            sint
        } else {
            source_external
        };

        circuit.bsim3v3.add(Bsim3v3Device::new(
            element.name.clone(),
            drain,
            gate,
            source,
            bulk,
            multiplier,
            core,
        ));
        Ok(())
    }
}

/// Per-`.model` shared BSIM3v3.3 state: the parsed card, its temperature
/// block, and the (W, L)-keyed size-dependent parameter knots.
struct Bsim3v3SharedModel {
    model: std::sync::Arc<crate::device::Bsim3v3Model>,
    model_temp: std::sync::Arc<crate::device::mosfet::bsim3v3::Bsim3v3ModelTemp>,
    size_cache: crate::device::mosfet::bsim3v3::SizeDepCache,
}
