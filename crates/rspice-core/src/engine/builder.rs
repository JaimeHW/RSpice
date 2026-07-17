//! Circuit builder - constructs CircuitData from Netlist
//!
//! This module handles the conversion from parsed netlist elements
//! to the runtime circuit representation.

#![allow(clippy::needless_range_loop)]
use super::{Engine, JfetLevel2Model, SimulationError, SpiceDialect, extract_dc_value_with_limits};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::device::{JfetChannelModel, MosBodyJunctionModel};
use crate::netlist::expr::prepare_behavioral_expression;
use crate::netlist::{
    Element, ElementKind, FlattenerConfig, ParseError, ParseWithAbortError, SourceSpec,
    XYCE_DEFAULT_ZERO_RESISTANCE_TOL, XspiceAutoBridgeNodeHint, XspiceAutoBridgeTemplate,
    XspicePort, flatten_netlist_with_models, flatten_netlist_with_models_config_with_abort,
    reduce_supernode_topology,
};
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use crate::{CircuitData, Netlist};
#[cfg(feature = "veriloga")]
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
#[cfg(feature = "veriloga")]
use std::io::Read;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
use std::io::Write;
use std::path::Path;
#[cfg(feature = "veriloga")]
use std::path::PathBuf;
use std::sync::OnceLock;
#[cfg(feature = "veriloga")]
use std::sync::RwLock;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
use std::time::{Duration, Instant};

mod model_resolution;
pub(crate) use model_resolution::XYCE_DEFAULT_CAPACITOR_AGE_DEGRADATION;
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
    register_precompiled_veriloga_runtime_with_dependencies,
    register_project_veriloga_runtime_for_session, veriloga_cache_entries, veriloga_cache_stats,
};
#[cfg(feature = "veriloga")]
use veriloga_cache::{normalize_model_key, resolve_cached_or_compile_veriloga_with_limits};

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

#[inline]
fn check_build_abort(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

fn map_build_parse_error(context: &str, error: ParseWithAbortError) -> SimulationError {
    match error {
        ParseWithAbortError::Aborted => SimulationError::Aborted,
        ParseWithAbortError::Parse(ParseError::ResourceLimit(error)) => {
            SimulationError::ResourceLimit(error)
        }
        ParseWithAbortError::Parse(error) => {
            SimulationError::Netlist(format!("{context} error: {error}"))
        }
    }
}

fn check_circuit_resource_limits(
    engine: &Engine,
    circuit: &CircuitData,
) -> Result<(), SimulationError> {
    ResourceLimitError::ensure(
        ResourceKind::CircuitNodes,
        circuit.num_nodes(),
        engine.config.resource_limits.max_circuit_nodes,
    )?;
    ResourceLimitError::ensure(
        ResourceKind::MatrixUnknowns,
        circuit.matrix_size(),
        engine.config.resource_limits.max_matrix_unknowns,
    )?;
    Ok(())
}

fn validate_source_file_inputs(
    source_name: &str,
    spec: &crate::netlist::SourceSpec,
    resource_limits: ResourceLimits,
) -> Result<(), SimulationError> {
    use crate::netlist::SourceSpec;

    match spec {
        SourceSpec::RfPort { inner, .. } => {
            validate_source_file_inputs(source_name, inner, resource_limits)
        }
        SourceSpec::PwlFile {
            path,
            time_scale,
            value_scale,
            time_offset,
            value_offset,
            repeat_from,
            ..
        } => {
            let waveform = crate::circuit::VoltageSources::load_pwl_waveform_cached_with_limits(
                path,
                *time_scale,
                *value_scale,
                *time_offset,
                *value_offset,
                resource_limits,
            )
            .map_err(|error| match error {
                crate::device::pwl_file::PwlFileError::ResourceLimit(error) => {
                    SimulationError::ResourceLimit(error)
                }
                error => SimulationError::Circuit(format!(
                    "source '{source_name}': failed to load PWL file '{path}': {error}"
                )),
            })?;
            if let Some(repeat_from) = repeat_from
                && *repeat_from >= waveform.last_source_time()
            {
                return Err(SimulationError::Circuit(format!(
                    "source '{source_name}': PWL R must be less than the final PWL time"
                )));
            }
            Ok(())
        }
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            validate_source_file_inputs(source_name, transient, resource_limits)
        }
        _ => Ok(()),
    }
}

fn parse_direct_branch_current_control(expression: &str) -> Option<String> {
    let normalized: String = expression
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace())
        .flat_map(|ch| ch.to_lowercase())
        .collect();
    if !normalized.starts_with("i(") || !normalized.ends_with(')') {
        return None;
    }
    let inner = &normalized[2..normalized.len() - 1];
    (!inner.is_empty()).then(|| inner.to_string())
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
        | [XspicePort::DifferentialConductance { pos, neg }]
        | [XspicePort::DifferentialHybrid { pos, neg }] => Ok((pos.clone(), neg.clone())),
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
        SourceSpec::RfPort { inner, .. } => xspice_meter_zero_dc_voltage_source(inner),
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
    spice_dialect: SpiceDialect,
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
            spice_dialect,
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
    spice_dialect: SpiceDialect,
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
            spice_dialect,
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
    spice_dialect: SpiceDialect,
) -> Result<f64, SimulationError> {
    let mut capacitance = 0.0;

    for element in flat_elements {
        if let Some(value) =
            xspice_meter_resolved_capacitance(netlist, element, temperature, spice_dialect)?
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
            if let Some(value) =
                xspice_meter_resolved_capacitance(netlist, element, temperature, spice_dialect)?
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
    spice_dialect: SpiceDialect,
) -> Result<f64, SimulationError> {
    let mut inductance = 1.0e12;

    for element in flat_elements {
        if let Some(value) =
            xspice_meter_resolved_inductance(netlist, element, temperature, spice_dialect)?
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
            if let Some(value) =
                xspice_meter_resolved_inductance(netlist, element, temperature, spice_dialect)?
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
    spice_dialect: SpiceDialect,
) -> Result<f64, SimulationError> {
    let input_node = xspice_meter_input_node(element_name, model_name, ports)?;
    match kind {
        XspiceMeterKind::Capacitance => xspice_meter_equivalent_capacitance(
            netlist,
            flat_elements,
            input_node,
            temperature,
            spice_dialect,
        ),
        XspiceMeterKind::Inductance => xspice_meter_equivalent_inductance(
            netlist,
            flat_elements,
            input_node,
            temperature,
            spice_dialect,
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XspiceAutoBridgeKind {
    Adc,
    Dac,
    Bidi,
    RealToV,
}

#[derive(Debug, Clone)]
struct PlannedXspiceAutoBridge {
    node: usize,
    kind: XspiceAutoBridgeKind,
    vcc: crate::Value,
    family: Option<String>,
}

#[derive(Debug, Clone)]
struct XspiceAutoBridgeNodeMetadata {
    vcc: Option<crate::Value>,
    family: Option<String>,
}

#[derive(Debug, Clone)]
struct XspiceAutoBridgeFamilyCandidate {
    depth: usize,
    family: String,
}

#[derive(Debug, Clone, Copy)]
enum XspiceAutoBridgeOutputBranch {
    Scalar { port_idx: usize },
    Vector { port_idx: usize, element_idx: usize },
}

#[derive(Debug, Default, Clone, Copy)]
struct XspiceAutoBridgeUsage {
    digital_input: bool,
    digital_output: bool,
    digital_inout: bool,
}

#[derive(Debug, Default, Clone, Copy)]
struct XspiceExplicitDigitalBridgeCoverage {
    adc: bool,
    dac: bool,
    bidi: bool,
}

impl XspiceExplicitDigitalBridgeCoverage {
    fn register(&mut self, kind: XspiceAutoBridgeKind) {
        match kind {
            XspiceAutoBridgeKind::Adc => self.adc = true,
            XspiceAutoBridgeKind::Dac => self.dac = true,
            XspiceAutoBridgeKind::Bidi => self.bidi = true,
            XspiceAutoBridgeKind::RealToV => {}
        }
    }

    fn covers_adc(self) -> bool {
        self.adc || self.bidi
    }

    fn covers_dac(self) -> bool {
        self.dac || self.bidi
    }

    fn covers_bidi(self) -> bool {
        self.bidi || (self.adc && self.dac)
    }
}

impl XspiceAutoBridgeUsage {
    fn register(&mut self, direction: crate::xspice::PortDirection) {
        match direction {
            crate::xspice::PortDirection::In => self.digital_input = true,
            crate::xspice::PortDirection::Out => self.digital_output = true,
            crate::xspice::PortDirection::InOut => self.digital_inout = true,
        }
    }

    fn bridge_kind(
        self,
        coverage: XspiceExplicitDigitalBridgeCoverage,
    ) -> Option<XspiceAutoBridgeKind> {
        if self.digital_inout {
            if coverage.covers_bidi() {
                None
            } else {
                Some(XspiceAutoBridgeKind::Bidi)
            }
        } else if self.digital_input && self.digital_output {
            match (coverage.covers_adc(), coverage.covers_dac()) {
                (true, true) => None,
                (true, false) => Some(XspiceAutoBridgeKind::Dac),
                (false, true) => Some(XspiceAutoBridgeKind::Adc),
                (false, false) => Some(XspiceAutoBridgeKind::Bidi),
            }
        } else if self.digital_input && !coverage.covers_adc() {
            Some(XspiceAutoBridgeKind::Adc)
        } else if self.digital_output && !coverage.covers_dac() {
            Some(XspiceAutoBridgeKind::Dac)
        } else {
            None
        }
    }
}

fn direction_outputs_events(direction: crate::xspice::PortDirection) -> bool {
    matches!(
        direction,
        crate::xspice::PortDirection::Out | crate::xspice::PortDirection::InOut
    )
}

fn insert_non_ground_node(nodes: &mut BTreeSet<usize>, node: usize) {
    if node > 0 {
        nodes.insert(node);
    }
}

fn collect_flat_analog_nodes(
    nodes: &mut BTreeSet<usize>,
    circuit: &CircuitData,
    flat_elements: &[Element],
) {
    for element in flat_elements {
        if matches!(element.kind, ElementKind::Xspice { .. }) {
            continue;
        }
        for node_name in &element.nodes {
            if let Some(node) = circuit.get_node_by_name(node_name) {
                insert_non_ground_node(nodes, node);
            }
        }
    }
}

fn collect_real_connection_nodes(
    nodes: &mut BTreeSet<usize>,
    connection: &crate::xspice::PortConnection,
) {
    use crate::xspice::PortConnection;

    match connection {
        PortConnection::Real(node) => insert_non_ground_node(nodes, *node),
        PortConnection::RealVector(vector) => {
            for node in vector {
                insert_non_ground_node(nodes, *node);
            }
        }
        _ => {}
    }
}

fn collect_analog_connection_nodes(
    nodes: &mut BTreeSet<usize>,
    connection: &crate::xspice::PortConnection,
) {
    use crate::xspice::{AnalogInputConnection, PortConnection};

    match connection {
        PortConnection::Analog(node) => insert_non_ground_node(nodes, *node),
        PortConnection::Differential(pos, neg)
        | PortConnection::CurrentProbe { pos, neg, .. }
        | PortConnection::CurrentOutput { pos, neg }
        | PortConnection::Hybrid { pos, neg, .. } => {
            insert_non_ground_node(nodes, *pos);
            insert_non_ground_node(nodes, *neg);
        }
        PortConnection::AnalogVector(vector) => {
            for node in vector {
                insert_non_ground_node(nodes, *node);
            }
        }
        PortConnection::TypedAnalogVector(vector) => {
            for connection in vector {
                match connection {
                    AnalogInputConnection::Node(node) => insert_non_ground_node(nodes, *node),
                    AnalogInputConnection::Differential(pos, neg)
                    | AnalogInputConnection::CurrentProbe { pos, neg, .. }
                    | AnalogInputConnection::CurrentOutput { pos, neg }
                    | AnalogInputConnection::Hybrid { pos, neg, .. } => {
                        insert_non_ground_node(nodes, *pos);
                        insert_non_ground_node(nodes, *neg);
                    }
                    AnalogInputConnection::BranchCurrent { .. }
                    | AnalogInputConnection::NamedBranchCurrent { .. }
                    | AnalogInputConnection::NamedCurrentSource { .. } => {}
                }
            }
        }
        PortConnection::Digital(_)
        | PortConnection::DigitalInverted(_)
        | PortConnection::Real(_)
        | PortConnection::DigitalVector(_)
        | PortConnection::DigitalVectorMapped(_)
        | PortConnection::RealVector(_)
        | PortConnection::BranchCurrent { .. }
        | PortConnection::NamedBranchCurrent { .. }
        | PortConnection::NamedCurrentSource { .. }
        | PortConnection::Null => {}
    }
}

fn register_explicit_digital_bridge_coverage(
    nodes: &mut BTreeMap<usize, XspiceExplicitDigitalBridgeCoverage>,
    connection: &crate::xspice::PortConnection,
    kind: XspiceAutoBridgeKind,
) {
    use crate::xspice::PortConnection;

    let mut register = |node: usize| {
        if node > 0 {
            nodes.entry(node).or_default().register(kind);
        }
    };

    match connection {
        PortConnection::Digital(node) | PortConnection::DigitalInverted(node) => register(*node),
        PortConnection::DigitalVector(vector) => {
            for node in vector {
                register(*node);
            }
        }
        PortConnection::DigitalVectorMapped(vector) => {
            for connection in vector {
                register(connection.node);
            }
        }
        _ => {}
    }
}

fn register_digital_connection_nodes(
    nodes: &mut BTreeMap<usize, XspiceAutoBridgeUsage>,
    connection: &crate::xspice::PortConnection,
    direction: crate::xspice::PortDirection,
) {
    use crate::xspice::PortConnection;

    let mut register = |node: usize| {
        if node > 0 {
            nodes.entry(node).or_default().register(direction);
        }
    };

    match connection {
        PortConnection::Digital(node) | PortConnection::DigitalInverted(node) => register(*node),
        PortConnection::DigitalVector(vector) => {
            for node in vector {
                register(*node);
            }
        }
        PortConnection::DigitalVectorMapped(vector) => {
            for connection in vector {
                register(connection.node);
            }
        }
        _ => {}
    }
}

fn register_digital_connection_family(
    nodes: &mut BTreeMap<usize, XspiceAutoBridgeFamilyCandidate>,
    connection: &crate::xspice::PortConnection,
    family: Option<&str>,
    depth: usize,
) {
    use crate::xspice::PortConnection;

    let Some(family) = family.map(str::trim).filter(|family| !family.is_empty()) else {
        return;
    };
    let family = family.to_string();
    let mut register = |node: usize| {
        if node == 0 {
            return;
        }
        match nodes.get_mut(&node) {
            Some(candidate) if depth > candidate.depth => {
                candidate.depth = depth;
                candidate.family = family.clone();
            }
            Some(_) => {}
            None => {
                nodes.insert(
                    node,
                    XspiceAutoBridgeFamilyCandidate {
                        depth,
                        family: family.clone(),
                    },
                );
            }
        }
    };

    match connection {
        PortConnection::Digital(node) | PortConnection::DigitalInverted(node) => register(*node),
        PortConnection::DigitalVector(vector) => {
            for node in vector {
                register(*node);
            }
        }
        PortConnection::DigitalVectorMapped(vector) => {
            for connection in vector {
                register(connection.node);
            }
        }
        _ => {}
    }
}

fn xspice_instance_hierarchy_depth(name: &str) -> usize {
    name.bytes().filter(|byte| *byte == b'.').count()
}

fn register_real_output_connection_nodes(
    nodes: &mut BTreeSet<usize>,
    connection: &crate::xspice::PortConnection,
    direction: crate::xspice::PortDirection,
) {
    use crate::xspice::PortConnection;

    if !direction_outputs_events(direction) {
        return;
    }

    match connection {
        PortConnection::Real(node) => insert_non_ground_node(nodes, *node),
        PortConnection::RealVector(vector) => {
            for node in vector {
                insert_non_ground_node(nodes, *node);
            }
        }
        _ => {}
    }
}

fn is_xspice_digital_bridge_model(model_name: &str) -> bool {
    model_name.eq_ignore_ascii_case("adc_bridge")
        || model_name.eq_ignore_ascii_case("dac_bridge")
        || model_name.eq_ignore_ascii_case("bidi_bridge")
}

fn is_xspice_real_bridge_model(model_name: &str) -> bool {
    model_name.eq_ignore_ascii_case("real_to_v") || model_name.eq_ignore_ascii_case("r_to_v")
}

fn explicit_digital_bridge_kind_for_port(
    model_name: &str,
    port: &crate::xspice::PortSpec,
) -> Option<XspiceAutoBridgeKind> {
    use crate::xspice::{PortDirection, PortType};

    if port.default_type != PortType::Digital {
        return None;
    }

    if model_name.eq_ignore_ascii_case("adc_bridge") && port.direction == PortDirection::Out {
        Some(XspiceAutoBridgeKind::Adc)
    } else if model_name.eq_ignore_ascii_case("dac_bridge") && port.direction == PortDirection::In {
        Some(XspiceAutoBridgeKind::Dac)
    } else if model_name.eq_ignore_ascii_case("bidi_bridge")
        && port.direction == PortDirection::InOut
        && port.name.eq_ignore_ascii_case("d")
    {
        Some(XspiceAutoBridgeKind::Bidi)
    } else {
        None
    }
}

fn plan_xspice_auto_bridges(
    circuit: &CircuitData,
    flat_elements: &[Element],
    bridge_metadata: &BTreeMap<usize, XspiceAutoBridgeNodeMetadata>,
    default_vcc: crate::Value,
) -> Vec<PlannedXspiceAutoBridge> {
    let mut analog_nodes = BTreeSet::new();
    let mut digital_nodes = BTreeMap::new();
    let mut digital_node_families = BTreeMap::new();
    let mut real_output_nodes = BTreeSet::new();
    let mut explicit_digital_bridge_coverage = BTreeMap::new();
    let mut explicit_real_bridge_nodes = BTreeSet::new();

    collect_flat_analog_nodes(&mut analog_nodes, circuit, flat_elements);

    for instance in &circuit.xspice_instances {
        let explicit_digital_bridge = is_xspice_digital_bridge_model(instance.model_name());
        let explicit_real_bridge = is_xspice_real_bridge_model(instance.model_name());
        for (port_idx, port) in instance.ports().iter().enumerate() {
            let Some(connection) = instance.connection_at(port_idx) else {
                continue;
            };

            if explicit_digital_bridge {
                if let Some(kind) =
                    explicit_digital_bridge_kind_for_port(instance.model_name(), port)
                {
                    register_explicit_digital_bridge_coverage(
                        &mut explicit_digital_bridge_coverage,
                        connection,
                        kind,
                    );
                }
                continue;
            }
            if explicit_real_bridge {
                collect_real_connection_nodes(&mut explicit_real_bridge_nodes, connection);
                continue;
            }

            collect_analog_connection_nodes(&mut analog_nodes, connection);
            if port.default_type == crate::xspice::PortType::Digital {
                register_digital_connection_nodes(&mut digital_nodes, connection, port.direction);
                register_digital_connection_family(
                    &mut digital_node_families,
                    connection,
                    instance.string_param("family"),
                    xspice_instance_hierarchy_depth(&instance.name),
                );
            } else if port.default_type == crate::xspice::PortType::Real {
                register_real_output_connection_nodes(
                    &mut real_output_nodes,
                    connection,
                    port.direction,
                );
            }
        }
    }

    let mut planned: Vec<PlannedXspiceAutoBridge> = digital_nodes
        .into_iter()
        .filter_map(|(node, usage)| {
            if !analog_nodes.contains(&node) {
                return None;
            }
            let coverage = explicit_digital_bridge_coverage
                .get(&node)
                .copied()
                .unwrap_or_default();
            let metadata = bridge_metadata.get(&node);
            usage
                .bridge_kind(coverage)
                .map(|kind| PlannedXspiceAutoBridge {
                    node,
                    kind,
                    vcc: metadata
                        .and_then(|metadata| metadata.vcc)
                        .unwrap_or(default_vcc),
                    family: digital_node_families
                        .get(&node)
                        .map(|candidate: &XspiceAutoBridgeFamilyCandidate| candidate.family.clone())
                        .or_else(|| metadata.and_then(|metadata| metadata.family.clone())),
                })
        })
        .collect();

    planned.extend(
        real_output_nodes
            .into_iter()
            .filter(|node| {
                analog_nodes.contains(node) && !explicit_real_bridge_nodes.contains(node)
            })
            .map(|node| PlannedXspiceAutoBridge {
                node,
                kind: XspiceAutoBridgeKind::RealToV,
                vcc: 0.0,
                family: None,
            }),
    );

    planned
}

fn xspice_auto_bridge_vcc(netlist: &Netlist) -> crate::Value {
    let param_name = netlist.options.auto_bridge_param_name("d").unwrap_or("vcc");
    netlist
        .params
        .get(param_name)
        .filter(|value| value.is_finite())
        .unwrap_or(3.3)
}

fn xspice_auto_bridge_scoped_metadata(
    circuit: &CircuitData,
    hints: &[XspiceAutoBridgeNodeHint],
) -> BTreeMap<usize, XspiceAutoBridgeNodeMetadata> {
    let mut best_by_node: BTreeMap<usize, (usize, XspiceAutoBridgeNodeMetadata)> = BTreeMap::new();
    for hint in hints {
        let Some(node) = circuit.get_node_by_name(&hint.node) else {
            continue;
        };
        if node == 0 {
            continue;
        }
        let metadata = XspiceAutoBridgeNodeMetadata {
            vcc: hint.vcc,
            family: hint.family.clone(),
        };
        if metadata.vcc.is_none() && metadata.family.is_none() {
            continue;
        }
        match best_by_node.get_mut(&node) {
            Some((depth, existing)) if hint.depth > *depth => {
                *depth = hint.depth;
                *existing = metadata;
            }
            Some(_) => {}
            None => {
                best_by_node.insert(node, (hint.depth, metadata));
            }
        }
    }
    best_by_node
        .into_iter()
        .map(|(node, (_, metadata))| (node, metadata))
        .collect()
}

fn xspice_auto_bridge_node_label(node_names: Option<&[String]>, node: usize) -> String {
    if node == 0 {
        return "0".to_string();
    }

    node_names
        .and_then(|names| names.get(node - 1))
        .cloned()
        .unwrap_or_else(|| node.to_string())
}

fn xspice_auto_bridge_kind_label(kind: XspiceAutoBridgeKind) -> &'static str {
    match kind {
        XspiceAutoBridgeKind::Adc => "analog-to-digital",
        XspiceAutoBridgeKind::Dac => "digital-to-analog",
        XspiceAutoBridgeKind::Bidi => "bidirectional digital/analog",
        XspiceAutoBridgeKind::RealToV => "real-to-voltage",
    }
}

fn xspice_auto_bridge_generated_card(
    bridge: &PlannedXspiceAutoBridge,
    instance_name: &str,
    node_label: &str,
) -> String {
    let vcc = bridge.vcc;
    let half_vcc = vcc / 2.0;
    match bridge.kind {
        XspiceAutoBridgeKind::Adc => format!(
            "{instance_name} [ {node_label} ] [ {node_label} ] adc_bridge(in_low={half_vcc} in_high={half_vcc})"
        ),
        XspiceAutoBridgeKind::Dac => format!(
            "{instance_name} [ {node_label} ] [ {node_label} ] dac_bridge(out_low=0 out_high={vcc})"
        ),
        XspiceAutoBridgeKind::Bidi => format!(
            "{instance_name} [ {node_label} ] [ {node_label} ] null bidi_bridge(out_high={vcc} in_low={half_vcc} in_high={half_vcc})"
        ),
        XspiceAutoBridgeKind::RealToV => {
            format!("{instance_name} {node_label} null {node_label} r_to_v")
        }
    }
}

fn reject_disabled_xspice_auto_bridge(
    circuit: &CircuitData,
    bridges: &[PlannedXspiceAutoBridge],
) -> Result<(), SimulationError> {
    if let Some(bridge) = bridges.first() {
        let node_names = circuit.node_names_sorted();
        let node_label = xspice_auto_bridge_node_label(Some(&node_names), bridge.node);
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge insertion is disabled, but node '{}' is mixed-type and needs a {} bridge",
            node_label,
            xspice_auto_bridge_kind_label(bridge.kind)
        )));
    }
    Ok(())
}

fn xspice_auto_bridge_template_type_name(kind: XspiceAutoBridgeKind) -> &'static str {
    match kind {
        XspiceAutoBridgeKind::Adc | XspiceAutoBridgeKind::Dac | XspiceAutoBridgeKind::Bidi => "d",
        XspiceAutoBridgeKind::RealToV => "real",
    }
}

fn xspice_auto_bridge_template_direction(kind: XspiceAutoBridgeKind) -> &'static str {
    match kind {
        XspiceAutoBridgeKind::Adc => "in",
        XspiceAutoBridgeKind::Dac | XspiceAutoBridgeKind::RealToV => "out",
        XspiceAutoBridgeKind::Bidi => "inout",
    }
}

fn find_xspice_auto_bridge_template<'a>(
    templates: &'a [XspiceAutoBridgeTemplate],
    bridge: &PlannedXspiceAutoBridge,
    family_enabled: bool,
) -> Option<&'a XspiceAutoBridgeTemplate> {
    let type_name = xspice_auto_bridge_template_type_name(bridge.kind);
    let direction = xspice_auto_bridge_template_direction(bridge.kind);

    if family_enabled
        && let Some(family) = bridge
            .family
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
    {
        let family = family.strip_prefix('*').unwrap_or(family);
        if !family.is_empty() {
            let key = format!("auto_bridge_{family}_{type_name}_{direction}");
            if let Some(template) = find_xspice_auto_bridge_template_key(templates, &key) {
                return Some(template);
            }
        }
    }

    let key = format!("auto_bridge_{type_name}_{direction}");
    find_xspice_auto_bridge_template_key(templates, &key)
}

fn find_xspice_auto_bridge_template_key<'a>(
    templates: &'a [XspiceAutoBridgeTemplate],
    key: &str,
) -> Option<&'a XspiceAutoBridgeTemplate> {
    templates
        .iter()
        .rev()
        .find(|template| template.key.eq_ignore_ascii_case(key))
}

fn xspice_auto_bridge_standard_family_template(
    bridge: &PlannedXspiceAutoBridge,
    source_path: Option<&Path>,
) -> Option<XspiceAutoBridgeTemplate> {
    let family = bridge.family.as_deref()?.trim();
    if family.is_empty() || family.starts_with('*') {
        return None;
    }

    let type_name = xspice_auto_bridge_template_type_name(bridge.kind);
    let direction = xspice_auto_bridge_template_direction(bridge.kind);
    let include_file = format!("bridge_{family}_{type_name}_{direction}.subcir");
    let include_path = Path::new(&include_file);
    let include_exists = if include_path.is_absolute() {
        include_path.exists()
    } else if let Some(parent) = source_path.and_then(Path::parent) {
        parent.join(include_path).exists()
    } else {
        include_path.exists()
    };
    if !include_exists {
        return None;
    }

    let key = format!("auto_bridge_{family}_{type_name}_{direction}");
    Some(XspiceAutoBridgeTemplate {
        key,
        setup_card: format!(".include {include_file}"),
        device_card: format!("Xauto_bridge%d %s %s bridge_{family}_{type_name}_{direction} vcc=%g"),
        max_nodes: Some(1),
    })
}

fn xspice_auto_bridge_effective_templates(
    templates: &[XspiceAutoBridgeTemplate],
    bridges: &[PlannedXspiceAutoBridge],
    source_path: Option<&Path>,
    family_enabled: bool,
) -> Vec<XspiceAutoBridgeTemplate> {
    let mut effective_templates = templates.to_vec();
    if !family_enabled {
        return effective_templates;
    }

    let mut seen_standard_keys = BTreeSet::new();
    for bridge in bridges {
        let Some(template) = xspice_auto_bridge_standard_family_template(bridge, source_path)
        else {
            continue;
        };
        if seen_standard_keys.insert(template.key.to_ascii_uppercase()) {
            effective_templates.push(template);
        }
    }
    effective_templates
}

enum XspiceAutoBridgeFormatArg<'a> {
    Int(usize),
    Str(&'a str),
    Float(crate::Value),
}

#[derive(Default)]
struct XspiceAutoBridgePrintfFlags {
    left_justify: bool,
    sign_plus: bool,
    sign_space: bool,
    alternate: bool,
    zero_pad: bool,
}

struct XspiceAutoBridgePrintfSpec {
    placeholder: String,
    flags: XspiceAutoBridgePrintfFlags,
    width: Option<usize>,
    precision: Option<usize>,
    specifier: char,
}

fn parse_xspice_auto_bridge_printf_spec(
    template_key: &str,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<XspiceAutoBridgePrintfSpec, SimulationError> {
    let mut placeholder = String::from("%");
    let mut flags = XspiceAutoBridgePrintfFlags::default();

    loop {
        let Some(&ch) = chars.peek() else {
            return Err(SimulationError::Circuit(format!(
                "XSPICE auto-bridge template '{template_key}' has a trailing '%' in format card"
            )));
        };
        match ch {
            '-' => flags.left_justify = true,
            '+' => flags.sign_plus = true,
            ' ' => flags.sign_space = true,
            '#' => flags.alternate = true,
            '0' => flags.zero_pad = true,
            _ => break,
        }
        placeholder.push(ch);
        chars.next();
    }

    let width = parse_xspice_auto_bridge_printf_digits(chars, &mut placeholder);

    let precision = if chars.peek() == Some(&'.') {
        placeholder.push('.');
        chars.next();
        Some(parse_xspice_auto_bridge_printf_digits(chars, &mut placeholder).unwrap_or(0))
    } else {
        None
    };

    if chars.peek() == Some(&'*') {
        placeholder.push('*');
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge template '{template_key}' uses unsupported dynamic-width format placeholder {placeholder}"
        )));
    }

    match chars.peek().copied() {
        Some('h' | 'l') => {
            let length = chars.next().expect("peeked length modifier");
            placeholder.push(length);
            if chars.peek() == Some(&length) {
                placeholder.push(length);
                chars.next();
            }
        }
        Some('L' | 'j' | 'z' | 't') => {
            let length = chars.next().expect("peeked length modifier");
            placeholder.push(length);
        }
        _ => {}
    }

    let Some(specifier) = chars.next() else {
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge template '{template_key}' has a trailing '%' in format card"
        )));
    };
    placeholder.push(specifier);

    Ok(XspiceAutoBridgePrintfSpec {
        placeholder,
        flags,
        width,
        precision,
        specifier,
    })
}

fn parse_xspice_auto_bridge_printf_digits(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    placeholder: &mut String,
) -> Option<usize> {
    let mut value = 0usize;
    let mut saw_digit = false;
    while let Some(ch) = chars.peek().copied() {
        let Some(digit) = ch.to_digit(10) else {
            break;
        };
        saw_digit = true;
        placeholder.push(ch);
        chars.next();
        value = value.saturating_mul(10).saturating_add(digit as usize);
    }
    saw_digit.then_some(value)
}

fn xspice_auto_bridge_format_len(value: &str) -> usize {
    value.chars().count()
}

fn apply_xspice_auto_bridge_string_width(
    mut value: String,
    spec: &XspiceAutoBridgePrintfSpec,
) -> String {
    let Some(width) = spec.width else {
        return value;
    };
    let len = xspice_auto_bridge_format_len(&value);
    if len >= width {
        return value;
    }

    let padding = " ".repeat(width - len);
    if spec.flags.left_justify {
        value.push_str(&padding);
        value
    } else {
        format!("{padding}{value}")
    }
}

fn xspice_auto_bridge_numeric_prefix_len(value: &str) -> usize {
    let mut prefix_len = match value.as_bytes().first().copied() {
        Some(b'+' | b'-' | b' ') => 1,
        _ => 0,
    };
    let rest = &value[prefix_len..];
    if rest.starts_with("0x") || rest.starts_with("0X") {
        prefix_len += 2;
    }
    prefix_len
}

fn apply_xspice_auto_bridge_numeric_width(
    value: String,
    spec: &XspiceAutoBridgePrintfSpec,
    zero_pad_allowed: bool,
) -> String {
    let Some(width) = spec.width else {
        return value;
    };
    let len = xspice_auto_bridge_format_len(&value);
    if len >= width {
        return value;
    }

    let pad_len = width - len;
    if spec.flags.left_justify {
        let mut padded = value;
        padded.push_str(&" ".repeat(pad_len));
        return padded;
    }

    if spec.flags.zero_pad && zero_pad_allowed {
        let prefix_len = xspice_auto_bridge_numeric_prefix_len(&value);
        let (prefix, body) = value.split_at(prefix_len);
        format!("{prefix}{}{body}", "0".repeat(pad_len))
    } else {
        format!("{}{value}", " ".repeat(pad_len))
    }
}

fn format_xspice_auto_bridge_int(value: usize, spec: &XspiceAutoBridgePrintfSpec) -> String {
    let mut digits = match spec.specifier {
        'o' => format!("{value:o}"),
        'x' => format!("{value:x}"),
        'X' => format!("{value:X}"),
        _ => value.to_string(),
    };

    if spec.precision == Some(0) && value == 0 {
        digits.clear();
    } else if let Some(precision) = spec.precision {
        let len = xspice_auto_bridge_format_len(&digits);
        if len < precision {
            digits = format!("{}{digits}", "0".repeat(precision - len));
        }
    }

    let mut formatted = String::new();
    match spec.specifier {
        'd' | 'i' => {
            if spec.flags.sign_plus {
                formatted.push('+');
            } else if spec.flags.sign_space {
                formatted.push(' ');
            }
        }
        'o' if spec.flags.alternate && !digits.starts_with('0') => formatted.push('0'),
        'x' if spec.flags.alternate && value != 0 => formatted.push_str("0x"),
        'X' if spec.flags.alternate && value != 0 => formatted.push_str("0X"),
        _ => {}
    }
    formatted.push_str(&digits);
    apply_xspice_auto_bridge_numeric_width(formatted, spec, spec.precision.is_none())
}

fn trim_xspice_auto_bridge_float_zeros(value: &mut String) {
    let exponent = value.find('e').or_else(|| value.find('E'));
    let suffix = exponent.map(|index| value.split_off(index));

    if let Some(dot) = value.find('.') {
        while value.ends_with('0') {
            value.pop();
        }
        if value.len() > dot && value.ends_with('.') {
            value.pop();
        }
    }

    if let Some(suffix) = suffix {
        value.push_str(&suffix);
    }
}

fn format_xspice_auto_bridge_general_float(
    value: crate::Value,
    precision: Option<usize>,
    uppercase: bool,
    alternate: bool,
) -> String {
    let precision = precision.unwrap_or(6).max(1);
    if !value.is_finite() {
        let formatted = value.to_string();
        return if uppercase {
            formatted.to_ascii_uppercase()
        } else {
            formatted
        };
    }

    let abs = value.abs();
    let exponent = if abs == 0.0 {
        0
    } else {
        abs.log10().floor() as i32
    };

    let mut formatted = if exponent < -4 || exponent >= precision as i32 {
        let digits_after_decimal = precision.saturating_sub(1);
        if uppercase {
            format!("{:.*E}", digits_after_decimal, value)
        } else {
            format!("{:.*e}", digits_after_decimal, value)
        }
    } else {
        let digits_after_decimal = (precision as i32 - exponent - 1).max(0) as usize;
        format!("{:.*}", digits_after_decimal, value)
    };

    if !alternate {
        trim_xspice_auto_bridge_float_zeros(&mut formatted);
    }
    if uppercase {
        formatted = formatted.to_ascii_uppercase();
    }
    formatted
}

fn format_xspice_auto_bridge_float(
    value: crate::Value,
    spec: &XspiceAutoBridgePrintfSpec,
) -> String {
    let mut formatted = match spec.specifier {
        'e' => format!("{:.*e}", spec.precision.unwrap_or(6), value),
        'E' => format!("{:.*E}", spec.precision.unwrap_or(6), value),
        'f' | 'F' => {
            let mut fixed = format!("{:.*}", spec.precision.unwrap_or(6), value);
            if spec.specifier == 'F' {
                fixed = fixed.to_ascii_uppercase();
            }
            if spec.flags.alternate && spec.precision == Some(0) && !fixed.contains('.') {
                fixed.push('.');
            }
            fixed
        }
        'g' => format_xspice_auto_bridge_general_float(
            value,
            spec.precision,
            false,
            spec.flags.alternate,
        ),
        'G' => format_xspice_auto_bridge_general_float(
            value,
            spec.precision,
            true,
            spec.flags.alternate,
        ),
        _ => value.to_string(),
    };

    if !formatted.starts_with('-') {
        if spec.flags.sign_plus {
            formatted.insert(0, '+');
        } else if spec.flags.sign_space {
            formatted.insert(0, ' ');
        }
    }

    apply_xspice_auto_bridge_numeric_width(formatted, spec, true)
}

fn format_xspice_auto_bridge_str(value: &str, spec: &XspiceAutoBridgePrintfSpec) -> String {
    let formatted = if let Some(precision) = spec.precision {
        value.chars().take(precision).collect()
    } else {
        value.to_string()
    };
    apply_xspice_auto_bridge_string_width(formatted, spec)
}

fn format_xspice_auto_bridge_template_card(
    template_key: &str,
    card: &str,
    args: &[XspiceAutoBridgeFormatArg<'_>],
) -> Result<String, SimulationError> {
    let mut output = String::with_capacity(card.len() + 32);
    let mut chars = card.chars().peekable();
    let mut arg_index = 0usize;

    while let Some(ch) = chars.next() {
        if ch != '%' {
            output.push(ch);
            continue;
        }

        let Some(next) = chars.peek().copied() else {
            return Err(SimulationError::Circuit(format!(
                "XSPICE auto-bridge template '{template_key}' has a trailing '%' in format card"
            )));
        };
        if next == '%' {
            chars.next();
            output.push('%');
            continue;
        }

        let spec = parse_xspice_auto_bridge_printf_spec(template_key, &mut chars)?;

        let Some(arg) = args.get(arg_index) else {
            return Err(SimulationError::Circuit(format!(
                "XSPICE auto-bridge template '{template_key}' has too many {} placeholders",
                spec.placeholder
            )));
        };
        arg_index += 1;

        match (spec.specifier, arg) {
            ('d' | 'i' | 'u' | 'o' | 'x' | 'X', XspiceAutoBridgeFormatArg::Int(value)) => {
                output.push_str(&format_xspice_auto_bridge_int(*value, &spec));
            }
            ('s', XspiceAutoBridgeFormatArg::Str(value)) => {
                output.push_str(&format_xspice_auto_bridge_str(value, &spec));
            }
            ('e' | 'E' | 'f' | 'F' | 'g' | 'G', XspiceAutoBridgeFormatArg::Float(value)) => {
                output.push_str(&format_xspice_auto_bridge_float(*value, &spec));
            }
            ('d' | 'i' | 'u' | 'o' | 'x' | 'X' | 's' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G', _) => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE auto-bridge template '{template_key}' placeholder {} has the wrong argument type",
                    spec.placeholder
                )));
            }
            _ => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE auto-bridge template '{template_key}' uses unsupported format placeholder {}",
                    spec.placeholder
                )));
            }
        }
    }

    Ok(output)
}

fn assign_xspice_output_branches(
    circuit: &mut CircuitData,
    instance: &mut crate::xspice::XspiceInstance,
    instance_name: &str,
) -> Result<(), SimulationError> {
    let ports_spec = instance.ports().to_vec();
    for (port_idx, port_spec) in ports_spec.iter().enumerate() {
        let is_output = matches!(port_spec.direction, crate::xspice::PortDirection::Out);
        let is_voltage_port = matches!(
            port_spec.default_type,
            crate::xspice::PortType::Voltage | crate::xspice::PortType::DifferentialVoltage
        );
        if !is_output || !is_voltage_port {
            continue;
        }

        let connection = instance.connection_at(port_idx).cloned();
        match connection {
            Some(crate::xspice::PortConnection::Analog(_))
            | Some(crate::xspice::PortConnection::Differential(_, _)) => {
                let branch_name = format!("{}#{}", instance_name, port_spec.name);
                let branch_ordinal = circuit.allocate_branch_named(&branch_name);
                instance
                    .set_output_branch(port_idx, branch_ordinal)
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Failed to assign branch for XSPICE instance '{}' port '{}': {}",
                            instance_name, port_spec.name, e
                        ))
                    })?;
            }
            Some(crate::xspice::PortConnection::AnalogVector(nodes)) => {
                for element_idx in 0..nodes.len() {
                    let branch_name =
                        format!("{}#{}[{}]", instance_name, port_spec.name, element_idx);
                    let branch_ordinal = circuit.allocate_branch_named(&branch_name);
                    instance
                        .set_output_vector_branch(port_idx, element_idx, branch_ordinal)
                        .map_err(|e| {
                            SimulationError::Circuit(format!(
                                "Failed to assign branch for XSPICE instance '{}' port '{}[{}]': {}",
                                instance_name, port_spec.name, element_idx, e
                            ))
                        })?;
                }
            }
            Some(crate::xspice::PortConnection::TypedAnalogVector(elements)) => {
                for (element_idx, element_connection) in elements.iter().enumerate() {
                    let needs_voltage_branch = matches!(
                        element_connection,
                        crate::xspice::AnalogInputConnection::Node(_)
                            | crate::xspice::AnalogInputConnection::Differential(_, _)
                    );
                    if !needs_voltage_branch {
                        continue;
                    }
                    let branch_name =
                        format!("{}#{}[{}]", instance_name, port_spec.name, element_idx);
                    let branch_ordinal = circuit.allocate_branch_named(&branch_name);
                    instance
                        .set_output_vector_branch(port_idx, element_idx, branch_ordinal)
                        .map_err(|e| {
                            SimulationError::Circuit(format!(
                                "Failed to assign branch for XSPICE instance '{}' port '{}[{}]': {}",
                                instance_name, port_spec.name, element_idx, e
                            ))
                        })?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn parse_generated_xspice_auto_bridge_deck(
    generated_deck: &str,
    source_path: Option<&Path>,
    resolve_includes: bool,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Netlist, ParseWithAbortError> {
    let options = crate::netlist::NetlistParseOptions {
        resource_limits,
        ..crate::netlist::NetlistParseOptions::default()
    };
    if !resolve_includes {
        return Netlist::parse_with_options_and_abort(generated_deck, options, abort);
    }

    let synthetic_path =
        source_path.unwrap_or_else(|| Path::new("rspice_generated_auto_bridge.cir"));
    Netlist::parse_with_path_and_options_and_abort(generated_deck, synthetic_path, options, abort)
}

fn add_generated_xspice_auto_bridge_resistor(
    circuit: &mut CircuitData,
    generated: &Netlist,
    element: &Element,
    temperature: crate::Value,
    spice_dialect: SpiceDialect,
) -> Result<(), SimulationError> {
    let ElementKind::Resistor {
        value,
        value_expr,
        model,
        instance_params,
        ..
    } = &element.kind
    else {
        return Err(SimulationError::Circuit(format!(
            "Generated XSPICE auto-bridge element '{}' is not a resistor",
            element.name
        )));
    };
    if element.nodes.len() != 2 {
        return Err(SimulationError::Circuit(format!(
            "Generated XSPICE auto-bridge resistor '{}' requires 2 nodes",
            element.name
        )));
    }
    if value_expr
        .as_deref()
        .is_some_and(expression_references_circuit_state)
    {
        return Err(SimulationError::Circuit(format!(
            "Generated XSPICE auto-bridge resistor '{}' cannot use a circuit-state expression",
            element.name
        )));
    }

    let resistance = resolve_resistor_instance_value(
        generated,
        &element.name,
        *value,
        value_expr.as_deref(),
        model.as_deref(),
        instance_params,
        temperature,
        spice_dialect,
    )?;
    let small_signal_resistance =
        resolve_resistor_small_signal_value(&element.name, resistance, instance_params)?;
    let np = circuit.get_or_create_node(&element.nodes[0]);
    let nn = circuit.get_or_create_node(&element.nodes[1]);
    let zero_resistance_tol = generated
        .options
        .device_zero_resistance_tol
        .unwrap_or(XYCE_DEFAULT_ZERO_RESISTANCE_TOL)
        .max(0.0);
    if resistance.is_finite() && resistance.abs() <= zero_resistance_tol {
        if !small_signal_resistance.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "Generated XSPICE auto-bridge resistor '{}' resolved to non-finite branch-form small-signal resistance {}",
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
    } else {
        circuit.resistors.add_with_small_signal(
            element.name.clone(),
            np,
            nn,
            resistance,
            small_signal_resistance,
        );
    }
    Ok(())
}

fn add_generated_xspice_auto_bridge_capacitor(
    circuit: &mut CircuitData,
    generated: &Netlist,
    element: &Element,
    temperature: crate::Value,
    spice_dialect: SpiceDialect,
) -> Result<(), SimulationError> {
    let ElementKind::Capacitor {
        value,
        initial_voltage,
        model,
        instance_params,
        ..
    } = &element.kind
    else {
        return Err(SimulationError::Circuit(format!(
            "Generated XSPICE auto-bridge element '{}' is not a capacitor",
            element.name
        )));
    };
    if element.nodes.len() != 2 {
        return Err(SimulationError::Circuit(format!(
            "Generated XSPICE auto-bridge capacitor '{}' requires 2 nodes",
            element.name
        )));
    }

    let capacitance = resolve_capacitor_instance_value(
        generated,
        &element.name,
        *value,
        model.as_deref(),
        instance_params,
        temperature,
        spice_dialect,
    )?;
    let np = circuit.get_or_create_node(&element.nodes[0]);
    let nn = circuit.get_or_create_node(&element.nodes[1]);
    if let Some(ic) = *initial_voltage {
        if spice_dialect == SpiceDialect::Xyce {
            let branch = circuit.allocate_branch_named(&element.name);
            circuit.capacitors.add_with_ic_branch(
                element.name.clone(),
                np,
                nn,
                capacitance,
                ic,
                branch,
            );
        } else {
            circuit
                .capacitors
                .add_with_ic(element.name.clone(), np, nn, capacitance, ic);
        }
    } else {
        circuit
            .capacitors
            .add(element.name.clone(), np, nn, capacitance);
    }
    Ok(())
}

fn add_generated_xspice_auto_bridge_inductor(
    circuit: &mut CircuitData,
    generated: &Netlist,
    element: &Element,
    temperature: crate::Value,
    spice_dialect: SpiceDialect,
) -> Result<(), SimulationError> {
    let ElementKind::Inductor {
        value,
        initial_current,
        model,
        instance_params,
        ..
    } = &element.kind
    else {
        return Err(SimulationError::Circuit(format!(
            "Generated XSPICE auto-bridge element '{}' is not an inductor",
            element.name
        )));
    };
    if element.nodes.len() != 2 {
        return Err(SimulationError::Circuit(format!(
            "Generated XSPICE auto-bridge inductor '{}' requires 2 nodes",
            element.name
        )));
    }

    let inductance = resolve_inductor_instance_value(
        generated,
        &element.name,
        *value,
        model.as_deref(),
        instance_params,
        temperature,
        spice_dialect,
    )?;
    let np = circuit.get_or_create_node(&element.nodes[0]);
    let nn = circuit.get_or_create_node(&element.nodes[1]);
    let branch = circuit.allocate_branch_named(&element.name);
    if let Some(ic) = *initial_current {
        circuit
            .inductors
            .add_with_ic(element.name.clone(), np, nn, branch, inductance, ic);
    } else {
        circuit
            .inductors
            .add(element.name.clone(), np, nn, branch, inductance);
    }
    Ok(())
}

fn add_generated_xspice_auto_bridge_instance(
    circuit: &mut CircuitData,
    generated: &Netlist,
    element: &Element,
    template_key: &str,
    temperature: crate::Value,
    ramptime: crate::Value,
    digital_delay_type: Option<i64>,
    resource_limits: ResourceLimits,
) -> Result<(), SimulationError> {
    let ElementKind::Xspice {
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
        ..
    } = &element.kind
    else {
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge template '{}' generated non-XSPICE element '{}'",
            template_key, element.name
        )));
    };

    let resolved_model = resolve_xspice_model_instance(
        generated,
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
            "Failed to resolve generated XSPICE auto-bridge model '{}' for template '{}': {}",
            model, template_key, e
        ))
    })?;

    let ports_spec = resolved_model.code_model.ports().to_vec();
    let connections = coerce_xspice_connections(
        circuit,
        &ports_spec,
        ports,
        &element.name,
        resolved_model.code_model.name(),
    )?;

    let mut instance = crate::xspice::XspiceInstance::new_with_string_vectors(
        element.name.clone(),
        resolved_model.code_model.clone(),
        connections,
        &resolved_model.numeric_params,
        &resolved_model.string_params,
        &resolved_model.string_vector_params,
        &resolved_model.real_vector_params,
        &resolved_model.integer_vector_params,
    )
    .map_err(|e| {
        SimulationError::Circuit(format!(
            "Failed to create generated XSPICE auto-bridge '{}': {}",
            element.name, e
        ))
    })?;

    instance.set_temperature(temperature);
    instance.set_ramptime(ramptime);
    instance.set_digital_delay_type(digital_delay_type);
    instance.set_resource_limits(resource_limits);
    assign_xspice_output_branches(circuit, &mut instance, &element.name)?;
    instance.init().map_err(|e| {
        SimulationError::Circuit(format!(
            "Failed to initialize generated XSPICE auto-bridge '{}': {}",
            element.name, e
        ))
    })?;

    circuit.add_xspice_instance(instance);
    Ok(())
}

fn add_generated_xspice_auto_bridge_subcircuit(
    circuit: &mut CircuitData,
    generated: &Netlist,
    template: &XspiceAutoBridgeTemplate,
    temperature: crate::Value,
    ramptime: crate::Value,
    digital_delay_type: Option<i64>,
    spice_dialect: SpiceDialect,
    resource_limits: ResourceLimits,
) -> Result<(), SimulationError> {
    let flattened = flatten_netlist_with_models(generated).map_err(|e| {
        SimulationError::Circuit(format!(
            "Failed to flatten generated XSPICE auto-bridge template '{}': {}",
            template.key, e
        ))
    })?;
    if !flattened.scoped_initial_conditions.is_empty() || !flattened.scoped_node_sets.is_empty() {
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge template '{}' generated subcircuit startup directives; RSpice supports device/model cards in this path",
            template.key
        )));
    }

    let mut effective_generated;
    let generated = if flattened.scoped_models.is_empty() {
        generated
    } else {
        effective_generated = generated.clone();
        effective_generated.models.extend(flattened.scoped_models);
        &effective_generated
    };

    let mut added_xspice = false;
    for element in &flattened.elements {
        match &element.kind {
            ElementKind::Resistor { .. } => {
                add_generated_xspice_auto_bridge_resistor(
                    circuit,
                    generated,
                    element,
                    temperature,
                    spice_dialect,
                )?;
            }
            ElementKind::Capacitor { .. } => {
                add_generated_xspice_auto_bridge_capacitor(
                    circuit,
                    generated,
                    element,
                    temperature,
                    spice_dialect,
                )?;
            }
            ElementKind::Inductor { .. } => {
                add_generated_xspice_auto_bridge_inductor(
                    circuit,
                    generated,
                    element,
                    temperature,
                    spice_dialect,
                )?;
            }
            ElementKind::Xspice { .. } => {
                add_generated_xspice_auto_bridge_instance(
                    circuit,
                    generated,
                    element,
                    &template.key,
                    temperature,
                    ramptime,
                    digital_delay_type,
                    resource_limits,
                )?;
                added_xspice = true;
            }
            _ => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE auto-bridge template '{}' generated unsupported subcircuit element '{}'; RSpice supports R/C/L passives and XSPICE A-device cards in generated bridge subcircuits",
                    template.key, element.name
                )));
            }
        }
    }

    if !added_xspice {
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge template '{}' generated no XSPICE A-device cards",
            template.key
        )));
    }
    Ok(())
}

fn add_template_xspice_auto_bridge(
    circuit: &mut CircuitData,
    bridges: &[&PlannedXspiceAutoBridge],
    template: &XspiceAutoBridgeTemplate,
    source_path: Option<&Path>,
    temperature: crate::Value,
    ramptime: crate::Value,
    digital_delay_type: Option<i64>,
    spice_dialect: SpiceDialect,
    node_names: Option<&[String]>,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    check_build_abort(abort)?;
    let Some(first_bridge) = bridges.first().copied() else {
        return Ok(());
    };

    let owned_node_names;
    let all_node_names: &[String] = match node_names {
        Some(node_names) => node_names,
        None => {
            owned_node_names = circuit.node_names_sorted();
            &owned_node_names
        }
    };
    let node_labels: Vec<String> = bridges
        .iter()
        .map(|bridge| xspice_auto_bridge_node_label(Some(all_node_names), bridge.node))
        .collect();
    let node_list = node_labels.join(" ");

    let setup_card = format_xspice_auto_bridge_template_card(
        &template.key,
        &template.setup_card,
        &[
            XspiceAutoBridgeFormatArg::Float(first_bridge.vcc),
            XspiceAutoBridgeFormatArg::Float(first_bridge.vcc),
            XspiceAutoBridgeFormatArg::Float(first_bridge.vcc),
            XspiceAutoBridgeFormatArg::Float(first_bridge.vcc),
            XspiceAutoBridgeFormatArg::Float(first_bridge.vcc),
        ],
    )?;
    let device_card = format_xspice_auto_bridge_template_card(
        &template.key,
        &template.device_card,
        &[
            XspiceAutoBridgeFormatArg::Int(first_bridge.node),
            XspiceAutoBridgeFormatArg::Str(&node_list),
            XspiceAutoBridgeFormatArg::Str(&node_list),
            XspiceAutoBridgeFormatArg::Float(first_bridge.vcc),
        ],
    )?;

    let setup_trimmed = setup_card.trim_start();
    let setup_is_include = setup_trimmed
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(".inc"));
    if !setup_is_include
        && !setup_trimmed
            .get(..6)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(".model"))
    {
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge template '{}' setup card must be a .model or .include card",
            template.key
        )));
    }

    let device_trimmed = device_card.trim_start();
    if !device_trimmed
        .chars()
        .next()
        .is_some_and(|ch| ch.eq_ignore_ascii_case(&'a') || ch.eq_ignore_ascii_case(&'x'))
    {
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge template '{}' device card must be an XSPICE A-device card or subcircuit X-device card",
            template.key
        )));
    }
    let device_is_subcircuit = device_trimmed
        .chars()
        .next()
        .is_some_and(|ch| ch.eq_ignore_ascii_case(&'x'));

    let generated_deck =
        format!("RSpice generated XSPICE auto bridge\n{setup_card}\n{device_card}\n.end\n");
    let generated = parse_generated_xspice_auto_bridge_deck(
        &generated_deck,
        source_path,
        setup_is_include,
        resource_limits,
        abort,
    )
    .map_err(|error| match error {
        ParseWithAbortError::Aborted => SimulationError::Aborted,
        ParseWithAbortError::Parse(ParseError::ResourceLimit(error)) => {
            SimulationError::ResourceLimit(error)
        }
        ParseWithAbortError::Parse(error) => SimulationError::Circuit(format!(
            "Failed to parse generated XSPICE auto-bridge template '{}': {}",
            template.key, error
        )),
    })?;
    if device_is_subcircuit {
        add_generated_xspice_auto_bridge_subcircuit(
            circuit,
            &generated,
            template,
            temperature,
            ramptime,
            digital_delay_type,
            spice_dialect,
            resource_limits,
        )?;
        log::debug!(
            "Generated XSPICE subcircuit auto-bridge on nodes {} from template {}",
            node_list,
            template.key
        );
        if node_names.is_some() {
            log::info!("Generated XSPICE auto-bridge card: {}", device_card);
        }
        return Ok(());
    }

    if generated.elements.len() != 1 {
        return Err(SimulationError::Circuit(format!(
            "XSPICE auto-bridge template '{}' must generate exactly one XSPICE A-device card",
            template.key
        )));
    }

    let element = &generated.elements[0];
    add_generated_xspice_auto_bridge_instance(
        circuit,
        &generated,
        element,
        &template.key,
        temperature,
        ramptime,
        digital_delay_type,
        resource_limits,
    )?;

    log::debug!(
        "Generated XSPICE auto-bridge {} on nodes {} from template {}",
        element.name,
        node_list,
        template.key
    );
    if node_names.is_some() {
        log::info!("Generated XSPICE auto-bridge card: {}", device_card);
    }
    Ok(())
}

fn add_planned_xspice_auto_bridges(
    circuit: &mut CircuitData,
    bridges: &[PlannedXspiceAutoBridge],
    templates: &[XspiceAutoBridgeTemplate],
    source_path: Option<&Path>,
    family_enabled: bool,
    temperature: crate::Value,
    ramptime: crate::Value,
    digital_delay_type: Option<i64>,
    spice_dialect: SpiceDialect,
    show_generated: bool,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    check_build_abort(abort)?;
    let node_names = show_generated.then(|| circuit.node_names_sorted());
    let effective_templates =
        xspice_auto_bridge_effective_templates(templates, bridges, source_path, family_enabled);
    let templates = effective_templates.as_slice();
    let mut consumed = vec![false; bridges.len()];

    for index in 0..bridges.len() {
        check_build_abort(abort)?;
        if consumed[index] {
            continue;
        }

        let bridge = &bridges[index];
        let Some(template) = find_xspice_auto_bridge_template(templates, bridge, family_enabled)
        else {
            consumed[index] = true;
            add_planned_xspice_auto_bridge(
                circuit,
                bridge,
                &[],
                source_path,
                family_enabled,
                temperature,
                ramptime,
                digital_delay_type,
                spice_dialect,
                node_names.as_deref(),
                resource_limits,
                abort,
            )?;
            continue;
        };

        let max_nodes = template.max_nodes.unwrap_or(bridges.len()).max(1);
        let mut group = Vec::with_capacity(max_nodes.min(bridges.len() - index));
        consumed[index] = true;
        group.push(bridge);

        for candidate_index in index + 1..bridges.len() {
            if candidate_index.is_multiple_of(64) {
                check_build_abort(abort)?;
            }
            if group.len() >= max_nodes {
                break;
            }
            if consumed[candidate_index] {
                continue;
            }
            let candidate = &bridges[candidate_index];
            if xspice_auto_bridge_template_group_compatible(
                templates,
                template,
                bridge,
                candidate,
                family_enabled,
            ) {
                consumed[candidate_index] = true;
                group.push(candidate);
            }
        }

        add_template_xspice_auto_bridge(
            circuit,
            &group,
            template,
            source_path,
            temperature,
            ramptime,
            digital_delay_type,
            spice_dialect,
            node_names.as_deref(),
            resource_limits,
            abort,
        )?;
    }
    Ok(())
}

fn xspice_auto_bridge_template_group_compatible(
    templates: &[XspiceAutoBridgeTemplate],
    template: &XspiceAutoBridgeTemplate,
    first: &PlannedXspiceAutoBridge,
    candidate: &PlannedXspiceAutoBridge,
    family_enabled: bool,
) -> bool {
    if first.kind != candidate.kind || first.vcc != candidate.vcc {
        return false;
    }

    find_xspice_auto_bridge_template(templates, candidate, family_enabled).is_some_and(
        |candidate_template| candidate_template.key.eq_ignore_ascii_case(&template.key),
    )
}

fn add_planned_xspice_auto_bridge(
    circuit: &mut CircuitData,
    bridge: &PlannedXspiceAutoBridge,
    templates: &[XspiceAutoBridgeTemplate],
    source_path: Option<&Path>,
    family_enabled: bool,
    temperature: crate::Value,
    ramptime: crate::Value,
    digital_delay_type: Option<i64>,
    spice_dialect: SpiceDialect,
    node_names: Option<&[String]>,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    use crate::xspice::PortConnection;

    if let Some(template) = find_xspice_auto_bridge_template(templates, bridge, family_enabled) {
        let bridges = [bridge];
        return add_template_xspice_auto_bridge(
            circuit,
            &bridges,
            template,
            source_path,
            temperature,
            ramptime,
            digital_delay_type,
            spice_dialect,
            node_names,
            resource_limits,
            abort,
        );
    }

    let vcc = bridge.vcc;
    let half_vcc = vcc / 2.0;
    let (model_name, instance_name, connections, numeric_params, output_branch) = match bridge.kind
    {
        XspiceAutoBridgeKind::Adc => (
            "adc_bridge",
            format!("__rspice_auto_adc_{}", bridge.node),
            vec![
                PortConnection::AnalogVector(vec![bridge.node]),
                PortConnection::DigitalVector(vec![bridge.node]),
            ],
            vec![
                ("in_low".to_string(), half_vcc),
                ("in_high".to_string(), half_vcc),
            ],
            None,
        ),
        XspiceAutoBridgeKind::Dac => (
            "dac_bridge",
            format!("__rspice_auto_dac_{}", bridge.node),
            vec![
                PortConnection::DigitalVector(vec![bridge.node]),
                PortConnection::AnalogVector(vec![bridge.node]),
            ],
            vec![("out_low".to_string(), 0.0), ("out_high".to_string(), vcc)],
            Some(XspiceAutoBridgeOutputBranch::Vector {
                port_idx: 1,
                element_idx: 0,
            }),
        ),
        XspiceAutoBridgeKind::Bidi => (
            "bidi_bridge",
            format!("__rspice_auto_bidi_{}", bridge.node),
            vec![
                PortConnection::AnalogVector(vec![bridge.node]),
                PortConnection::DigitalVector(vec![bridge.node]),
                PortConnection::Null,
            ],
            vec![
                ("out_high".to_string(), vcc),
                ("in_low".to_string(), half_vcc),
                ("in_high".to_string(), half_vcc),
            ],
            None,
        ),
        XspiceAutoBridgeKind::RealToV => (
            "real_to_v",
            format!("__rspice_auto_real_to_v_{}", bridge.node),
            vec![
                PortConnection::Real(bridge.node),
                PortConnection::Analog(bridge.node),
            ],
            Vec::new(),
            Some(XspiceAutoBridgeOutputBranch::Scalar { port_idx: 1 }),
        ),
    };

    let code_model = circuit.xspice_registry.get(model_name).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Failed to resolve generated XSPICE auto-bridge model '{}'",
            model_name
        ))
    })?;

    let mut instance = crate::xspice::XspiceInstance::new_with_string_vectors(
        instance_name.clone(),
        code_model,
        connections,
        &numeric_params,
        &[],
        &[],
        &[],
        &[],
    )
    .map_err(|e| {
        SimulationError::Circuit(format!(
            "Failed to create generated XSPICE auto-bridge '{}': {}",
            instance_name, e
        ))
    })?;

    instance.set_temperature(temperature);
    instance.set_ramptime(ramptime);
    instance.set_digital_delay_type(digital_delay_type);
    instance.set_resource_limits(resource_limits);

    if let Some(output_branch) = output_branch {
        match output_branch {
            XspiceAutoBridgeOutputBranch::Scalar { port_idx } => {
                let branch_name = format!("{}#out", instance_name);
                let branch_ordinal = circuit.allocate_branch_named(&branch_name);
                instance
                    .set_output_branch(port_idx, branch_ordinal)
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Failed to assign branch for generated XSPICE auto-bridge '{}': {}",
                            instance_name, e
                        ))
                    })?;
            }
            XspiceAutoBridgeOutputBranch::Vector {
                port_idx,
                element_idx,
            } => {
                let branch_name = format!("{}#out[{}]", instance_name, element_idx);
                let branch_ordinal = circuit.allocate_branch_named(&branch_name);
                instance
                    .set_output_vector_branch(port_idx, element_idx, branch_ordinal)
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "Failed to assign branch for generated XSPICE auto-bridge '{}': {}",
                            instance_name, e
                        ))
                    })?;
            }
        }
    }

    instance.init().map_err(|e| {
        SimulationError::Circuit(format!(
            "Failed to initialize generated XSPICE auto-bridge '{}': {}",
            instance_name, e
        ))
    })?;

    log::debug!(
        "Generated XSPICE auto-bridge {} on node {}",
        instance_name,
        bridge.node
    );
    if node_names.is_some() {
        let node_label = xspice_auto_bridge_node_label(node_names, bridge.node);
        log::info!(
            "Generated XSPICE auto-bridge card: {}",
            xspice_auto_bridge_generated_card(bridge, &instance_name, &node_label)
        );
    }
    circuit.add_xspice_instance(instance);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_auto_bridge_decks_inherit_resource_limits() {
        let generated = "generated bridge\n.model adc adc_bridge\n.end\n";
        let mut limits = ResourceLimits::default();
        limits.max_netlist_bytes = generated.len() - 1;

        assert!(matches!(
            parse_generated_xspice_auto_bridge_deck(
                generated,
                None,
                false,
                limits,
                &NoAbort,
            ),
            Err(ParseWithAbortError::Parse(ParseError::ResourceLimit(
                ResourceLimitError {
                    resource: ResourceKind::NetlistBytes,
                    requested,
                    limit,
                }
            ))) if requested == generated.len() && limit == generated.len() - 1
        ));
    }

    struct DiscardingStamper;

    impl crate::device::MatrixStamper for DiscardingStamper {
        fn stamp(
            &mut self,
            _row: crate::circuit::NodeId,
            _col: crate::circuit::NodeId,
            _value: crate::Value,
        ) {
        }

        fn stamp_rhs(&mut self, _index: crate::circuit::NodeId, _value: crate::Value) {}
    }

    #[test]
    fn behavioral_gmin_uses_resolved_engine_device_option() {
        let netlist = Netlist::parse_with_options(
            "resolved behavioral GMIN\nB1 out 0 V={GMIN}\n.END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::netlist::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce behavioral deck parses");
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config.junction_gmin_target = 7.5e-9;

        let mut circuit = Engine::new(config)
            .build_circuit(&netlist)
            .expect("behavioral circuit builds");

        assert_eq!(
            circuit.behavioral_sources.voltage_sources[0].evaluate(&[], 0.0),
            7.5e-9
        );
    }

    #[test]
    fn generic_switch_retains_scoped_runtime_params_and_resolved_context() {
        let netlist = Netlist::parse_with_options(
            "scoped runtime switch control\n\
             .OPTIONS GMIN=2.5E-8\n\
             .MODEL SM SWITCH(RON=2 ROFF=1MEG ON=150 OFF=140)\n\
             X1 out CELL SCALE=2\n\
             .SUBCKT CELL P SCALE=1\n\
             .PARAM CONTROL_VALUE={SCALE*(TEMP+VT+GMIN)}\n\
             S1 P 0 SM CONTROL={CONTROL_VALUE}\n\
             .ENDS\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::netlist::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce scoped generic-switch deck parses");
        let mut config = crate::engine::SimulationConfig::default();
        config.temperature = crate::analysis::temperature::celsius_to_kelvin(80.0);
        config.convergence_config.junction_gmin_target = 9.0e-7;
        let engine = Engine::new(config).resolved_for_netlist(&netlist);
        assert_eq!(
            engine.config.convergence_config.junction_gmin_target, 2.5e-8,
            ".OPTIONS GMIN must override the base engine device-option value"
        );

        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("scoped generic-switch circuit builds");
        assert_eq!(circuit.generic_switches.len(), 1);

        circuit.generic_switches[0].stamp_time_dependent(0.0, &mut DiscardingStamper);
        assert_eq!(
            circuit.generic_switches[0].conductance(),
            0.5,
            "resolved SCALE, TEMP, VT, and GMIN should drive the switch fully on"
        );
    }

    fn xspice_model_count(circuit: &CircuitData, model_name: &str) -> usize {
        circuit
            .xspice_instances
            .iter()
            .filter(|instance| instance.model_name().eq_ignore_ascii_case(model_name))
            .count()
    }

    fn single_xspice_param(circuit: &CircuitData, model_name: &str, param: &str) -> crate::Value {
        let mut matches = circuit
            .xspice_instances
            .iter()
            .filter(|instance| instance.model_name().eq_ignore_ascii_case(model_name));
        let instance = matches
            .next()
            .unwrap_or_else(|| panic!("expected one {model_name} instance"));
        assert!(
            matches.next().is_none(),
            "expected exactly one {model_name} instance"
        );
        instance.param(param)
    }

    fn single_xspice_instance<'a>(
        circuit: &'a CircuitData,
        model_name: &str,
    ) -> &'a crate::xspice::XspiceInstance {
        let mut matches = circuit
            .xspice_instances
            .iter()
            .filter(|instance| instance.model_name().eq_ignore_ascii_case(model_name));
        let instance = matches
            .next()
            .unwrap_or_else(|| panic!("expected one {model_name} instance"));
        assert!(
            matches.next().is_none(),
            "expected exactly one {model_name} instance"
        );
        instance
    }

    #[test]
    fn explicit_adc_does_not_suppress_needed_auto_dac_on_same_node() {
        let netlist = Netlist::parse(
            "\
* explicit adc plus mixed digital-output node
vctrl ain 0 dc 1
aadc [ain] [mix] adc
apull [mix] pullup
rload mix 0 1k
.model adc adc_bridge
.model pullup d_pullup
.end
",
        )
        .expect("deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "adc_bridge"), 1);
        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 1);
        assert_eq!(
            xspice_model_count(&circuit, "dac_bridge"),
            1,
            "explicit adc_bridge only covers analog-to-digital; mixed node 'mix' still needs a generated dac_bridge"
        );
    }

    #[test]
    fn auto_bridge_uses_deepest_xspice_subckt_vcc() {
        let netlist = Netlist::parse(
            "\
* auto bridge uses scoped subckt vcc
.param vcc=3.3
rload mix 0 1k
xcell mix dcell vcc=5
.model pull d_pullup
.subckt dcell y vcc=5
apull [y] pull
.ends
.end
",
        )
        .expect("deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 1);
        assert_eq!(xspice_model_count(&circuit, "dac_bridge"), 1);
        assert_eq!(
            single_xspice_param(&circuit, "dac_bridge", "out_high"),
            5.0,
            "generated dac_bridge should use the deepest connected XSPICE subckt vcc, not the top-level vcc"
        );
    }

    #[test]
    fn auto_bridge_uses_family_specific_template() {
        let netlist = Netlist::parse(
            "\
* auto bridge uses family-specific template
.param vcc=5
rload mix 0 1k
.model pull d_pullup(family=\"74HCT\")
apull [mix] pull
.control
set auto_bridge_d_out = ( \".model generic_dac dac_bridge(out_low = -1 out_high = %g)\" \"ageneric%d [ %s ] [ %s ] generic_dac\" 1 )
set auto_bridge_74HCT_d_out = ( \".model family_dac dac_bridge(out_low = -2 out_high = %g)\" \"afamily%d [ %s ] [ %s ] family_dac\" 1 )
.endc
.end
",
        )
        .expect("deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 1);
        assert_eq!(xspice_model_count(&circuit, "dac_bridge"), 1);
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_low"), -2.0);
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_high"), 5.0);
    }

    #[test]
    fn auto_bridge_uses_standard_family_include_template() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!(
            "rspice-auto-bridge-family-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&temp_dir).expect("create temp dir");
        std::fs::write(
            temp_dir.join("bridge_74HCT_d_out.subcir"),
            "\
.subckt bridge_74HCT_d_out dig ana vcc=5
.model family_dac dac_bridge(out_low = -0.75 out_high = {vcc})
abuf [ dig ] [ ana ] family_dac
.ends
",
        )
        .expect("write family bridge include file");

        let deck_path = temp_dir.join("main.cir");
        let netlist = Netlist::parse_with_path(
            "\
* auto bridge uses standard family include
.param vcc=4.4
rload mix 0 1k
.model pull d_pullup(family=\"74HCT\")
apull [mix] pull
.end
",
            &deck_path,
        )
        .expect("deck parses with source path");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 1);
        assert_eq!(xspice_model_count(&circuit, "dac_bridge"), 1);
        assert_eq!(
            single_xspice_param(&circuit, "dac_bridge", "out_low"),
            -0.75
        );
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_high"), 4.4);

        std::fs::remove_dir_all(temp_dir).expect("remove temp dir");
    }

    #[test]
    fn auto_bridge_no_family_uses_generic_template() {
        let netlist = Netlist::parse(
            "\
* auto bridge skips family-specific template when disabled
.param vcc=5
rload mix 0 1k
.model pull d_pullup(family=\"74HCT\")
apull [mix] pull
.control
set no_auto_bridge_family
set auto_bridge_d_out = ( \".model generic_dac dac_bridge(out_low = -1 out_high = %g)\" \"ageneric%d [ %s ] [ %s ] generic_dac\" 1 )
set auto_bridge_74HCT_d_out = ( \".model family_dac dac_bridge(out_low = -2 out_high = %g)\" \"afamily%d [ %s ] [ %s ] family_dac\" 1 )
.endc
.end
",
        )
        .expect("deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 1);
        assert_eq!(xspice_model_count(&circuit, "dac_bridge"), 1);
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_low"), -1.0);
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_high"), 5.0);
    }

    #[test]
    fn auto_bridge_template_accepts_printf_width_and_precision() {
        let netlist = Netlist::parse(
            "\
* auto bridge template accepts printf modifiers
.param vcc=4.567
rload mix 0 1k
.model pull d_pullup
apull [mix] pull
.control
set auto_bridge_d_out = ( \".model fmt_dac dac_bridge(out_low = 0 out_high = %.2f)\" \"afmt%03d [ %12s ] [ %12s ] fmt_dac\" 1 )
.endc
.end
",
        )
        .expect("deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        let dac = single_xspice_instance(&circuit, "dac_bridge");
        assert_eq!(dac.name, "AFMT001");
        assert_eq!(
            single_xspice_param(&circuit, "dac_bridge", "out_high"),
            4.57
        );
    }

    #[test]
    fn auto_bridge_template_groups_nodes_up_to_max() {
        let netlist = Netlist::parse(
            "\
* auto bridge template max groups nodes into one vector bridge
.param vcc=4
rload0 mix0 0 1k
rload1 mix1 0 1k
.model pull d_pullup
apull0 [mix0] pull
apull1 [mix1] pull
.control
set auto_bridge_d_out = ( \".model grouped_dac dac_bridge(out_low = -0.25 out_high = %g)\" \"agroup%d [ %s ] [ %s ] grouped_dac\" 2 )
.endc
.end
",
        )
        .expect("deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 2);
        assert_eq!(xspice_model_count(&circuit, "dac_bridge"), 1);
        assert_eq!(
            single_xspice_param(&circuit, "dac_bridge", "out_low"),
            -0.25
        );
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_high"), 4.0);

        let dac = single_xspice_instance(&circuit, "dac_bridge");
        assert!(
            matches!(
                dac.connection("in"),
                Some(crate::xspice::PortConnection::DigitalVector(nodes)) if nodes.len() == 2
            ),
            "grouped template should generate a two-bit digital input vector"
        );
        assert!(
            matches!(
                dac.connection("out"),
                Some(crate::xspice::PortConnection::AnalogVector(nodes)) if nodes.len() == 2
            ),
            "grouped template should generate a two-node analog output vector"
        );
    }

    #[test]
    fn auto_bridge_template_include_setup_resolves_model_card() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!(
            "rspice-auto-bridge-include-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&temp_dir).expect("create temp dir");
        std::fs::write(
            temp_dir.join("bridge_models.cir"),
            ".model included_dac dac_bridge(out_low = -0.5 out_high = 4.4)\n",
        )
        .expect("write include file");

        let deck_path = temp_dir.join("main.cir");
        let netlist = Netlist::parse_with_path(
            "\
* auto bridge template include setup
rload mix 0 1k
.model pull d_pullup
apull [mix] pull
.control
set auto_bridge_d_out = ( \".include bridge_models.cir\" \"ainc%d [ %s ] [ %s ] included_dac\" 1 )
.endc
.end
",
            &deck_path,
        )
        .expect("deck parses with source path");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 1);
        assert_eq!(xspice_model_count(&circuit, "dac_bridge"), 1);
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_low"), -0.5);
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_high"), 4.4);

        std::fs::remove_dir_all(temp_dir).expect("remove temp dir");
    }

    #[test]
    fn auto_bridge_template_include_setup_builds_subcircuit_bridge() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!(
            "rspice-auto-bridge-subckt-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&temp_dir).expect("create temp dir");
        std::fs::write(
            temp_dir.join("bridge_sub.cir"),
            "\
.subckt auto_buf dig ana vcc=5
.model auto_dac dac_bridge(out_low = 0 out_high = {vcc})
abuf [ dig ] [ internal ] auto_dac
rint internal ana 100
.ends
",
        )
        .expect("write bridge subckt include file");

        let deck_path = temp_dir.join("main.cir");
        let netlist = Netlist::parse_with_path(
            "\
* auto bridge template subcircuit setup
.param vcc=4.4
rload mix 0 1k
.model pull d_pullup
apull [mix] pull
.control
set auto_bridge_d_out = ( \".include bridge_sub.cir\" \"xauto_buf%d %s %s auto_buf vcc=%g\" 1 )
.endc
.end
",
            &deck_path,
        )
        .expect("deck parses with source path");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 1);
        assert_eq!(xspice_model_count(&circuit, "dac_bridge"), 1);
        assert_eq!(single_xspice_param(&circuit, "dac_bridge", "out_high"), 4.4);
        assert!(
            circuit
                .resistors
                .names
                .iter()
                .any(|name| name.to_ascii_uppercase().contains("RINT")),
            "generated bridge subcircuit should add the included rint resistor"
        );

        std::fs::remove_dir_all(temp_dir).expect("remove temp dir");
    }

    #[test]
    fn auto_bridge_uses_custom_digital_param_name() {
        let netlist = Netlist::parse(
            "\
* auto bridge uses auto_bridge_parm_d
.param vcc=5 vdd=1.8
rload mix 0 1k
.model pull d_pullup
apull [mix] pull
.control
set auto_bridge_parm_d = vdd
.endc
.end
",
        )
        .expect("deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("circuit builds");

        assert_eq!(xspice_model_count(&circuit, "d_pullup"), 1);
        assert_eq!(xspice_model_count(&circuit, "dac_bridge"), 1);
        assert_eq!(
            single_xspice_param(&circuit, "dac_bridge", "out_high"),
            1.8,
            "auto_bridge_parm_d should select vdd instead of the default vcc parameter"
        );
    }

    #[test]
    fn legacy_bsim1_rsh_uses_unit_default_diffusion_squares() {
        let netlist = Netlist::parse(
            "legacy BSIM1 default diffusion squares\n\
             vds d 0 0.05\n\
             vgs g 0 1.8\n\
             m1 d g 0 0 b1 l=10u w=50u\n\
             .model b1 nmos level=4 tox=0.03 vdd=5 rsh=35\n\
             .end\n",
        )
        .expect("legacy BSIM1 deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("legacy BSIM1 circuit builds");
        let drain = circuit.get_node_by_name("d").expect("external drain node");
        let drain_prime = circuit
            .get_node_by_name("m1.__dint")
            .expect("RSH creates a drain prime node");
        let source_prime = circuit
            .get_node_by_name("m1.__sint")
            .expect("RSH creates a source prime node");
        let mosfet = circuit
            .mosfets
            .devices
            .first()
            .expect("one legacy BSIM1 device");

        assert!(mosfet.uses_legacy_bsim());
        assert_eq!(mosfet.node_drain, drain_prime);
        assert_eq!(mosfet.node_source, source_prime);

        for (name, node_pos, node_neg) in [
            ("m1.__rd", drain, drain_prime),
            ("m1.__rs", 0, source_prime),
        ] {
            let index = circuit
                .resistors
                .names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .unwrap_or_else(|| panic!("expected generated resistor {name}"));
            let stamp = circuit.resistors.stamps[index];
            assert_eq!(stamp.pp.row, node_pos, "{name} positive terminal");
            assert_eq!(stamp.nn.row, node_neg, "{name} negative terminal");
            assert!(
                (circuit.resistors.conductances[index] - 1.0 / 35.0).abs() <= 1.0e-15,
                "{name} must implement RSH times the default one diffusion square"
            );
        }
    }

    #[test]
    fn legacy_bsim1_explicit_zero_diffusion_squares_disable_prime_nodes() {
        let netlist = Netlist::parse(
            "legacy BSIM1 zero diffusion squares\n\
             vds d 0 0.05\n\
             vgs g 0 1.8\n\
             m1 d g 0 0 b1 l=10u w=50u nrd=0 nrs=0\n\
             .model b1 nmos level=4 tox=0.03 vdd=5 rsh=35\n\
             .end\n",
        )
        .expect("legacy BSIM1 deck parses");

        let circuit = Engine::default()
            .build_circuit(&netlist)
            .expect("legacy BSIM1 circuit builds");
        let drain = circuit.get_node_by_name("d").expect("external drain node");
        let mosfet = circuit
            .mosfets
            .devices
            .first()
            .expect("one legacy BSIM1 device");

        assert!(mosfet.uses_legacy_bsim());
        assert_eq!(mosfet.node_drain, drain);
        assert_eq!(mosfet.node_source, 0);
        assert!(circuit.get_node_by_name("m1.__dint").is_none());
        assert!(circuit.get_node_by_name("m1.__sint").is_none());
        assert!(
            circuit.resistors.names.iter().all(|name| {
                !name.eq_ignore_ascii_case("m1.__rd") && !name.eq_ignore_ascii_case("m1.__rs")
            }),
            "explicit zero NRD/NRS must suppress both generated resistors"
        );
    }

    #[test]
    fn circuit_builder_honors_cancellation_before_construction() {
        let netlist = Netlist::parse("cancelled build\nV1 in 0 1\nR1 in 0 1k\n.end")
            .expect("cancellation fixture parses");

        assert!(matches!(
            Engine::default()
                .build_circuit_with_abort(&netlist, &crate::abort_signal::ImmediateAbort,),
            Err(SimulationError::Aborted)
        ));
    }
}

impl Engine {
    pub(crate) fn resolved_resistor_parameters(
        &self,
        netlist: &Netlist,
        resistor_name: &str,
    ) -> Result<Option<ResolvedResistorParameters>, SimulationError> {
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

        resolve_resistor_effective_parameters(
            netlist,
            &element.name,
            *value,
            value_expr.as_deref(),
            model.as_deref(),
            instance_params,
            engine.config.temperature,
            engine.config.spice_dialect,
        )
        .map(Some)
    }

    /// Return a canonical effective resistor parameter suitable for device
    /// reporting. Parameters copied from the model are resolved through the
    /// same path used to construct the simulated resistor.
    pub(crate) fn resolved_resistor_parameter(
        &self,
        netlist: &Netlist,
        resistor_name: &str,
        parameter: &str,
    ) -> Result<Option<f64>, SimulationError> {
        let Some(parameters) = self.resolved_resistor_parameters(netlist, resistor_name)? else {
            return Ok(None);
        };
        Ok(match parameter.to_ascii_uppercase().as_str() {
            "R" | "VALUE" | "RES" | "RESISTANCE" => Some(parameters.resistance),
            "W" | "WIDTH" | "DEFW" => Some(parameters.width),
            "TC" | "TC1" => Some(parameters.tc1),
            "TC2" => Some(parameters.tc2),
            "TEMP" | "TEMPER" => Some(parameters.temperature_celsius),
            _ => None,
        })
    }

    pub(crate) fn resolved_inductor_value(
        &self,
        netlist: &Netlist,
        inductor_name: &str,
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
            .find(|element| element.name.eq_ignore_ascii_case(inductor_name))
        else {
            return Ok(None);
        };

        let ElementKind::Inductor {
            value,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Ok(None);
        };

        resolve_inductor_instance_value(
            netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            engine.config.temperature,
            engine.config.spice_dialect,
        )
        .map(Some)
    }

    pub(crate) fn resolved_capacitor_value(
        &self,
        netlist: &Netlist,
        capacitor_name: &str,
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
            .find(|element| element.name.eq_ignore_ascii_case(capacitor_name))
        else {
            return Ok(None);
        };

        let ElementKind::Capacitor {
            value,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Ok(None);
        };

        resolve_capacitor_instance_value(
            netlist,
            &element.name,
            *value,
            model.as_deref(),
            instance_params,
            engine.config.temperature,
            engine.config.spice_dialect,
        )
        .map(Some)
    }

    /// Build a circuit from a netlist, using a non-cancellable compatibility path.
    pub fn build_circuit(&self, netlist: &Netlist) -> Result<CircuitData, SimulationError> {
        self.build_circuit_with_abort(netlist, &NoAbort)
    }

    /// Build a circuit while observing cooperative cancellation throughout
    /// validation, hierarchy flattening, and device instantiation.
    ///
    /// All public cancellable analyses use this entry point so a large or
    /// deeply hierarchical deck cannot make a timeout, UI stop request, or
    /// Python interrupt wait for construction to finish.
    pub fn build_circuit_with_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<CircuitData, SimulationError> {
        self.ensure_valid_configuration()?;
        check_build_abort(abort)?;
        let mut startup_validated;
        let netlist = if netlist.startup_directives.is_empty() {
            netlist
        } else {
            startup_validated = netlist.clone();
            crate::netlist::validate_startup_directives_with_abort(&mut startup_validated, abort)
                .map_err(|error| map_build_parse_error("startup validation", error))?;
            &startup_validated
        };
        crate::netlist::validate_output_symbols_with_abort(netlist, abort)
            .map_err(|error| map_build_parse_error("output validation", error))?;
        check_build_abort(abort)?;
        let mut circuit = CircuitData::new();
        circuit.b3soi_gmin_scale = if self.config.b3soi_gmin_scaling {
            1.0e-6
        } else {
            1.0
        };

        // Flatten subcircuit instances into top-level elements
        let flattened = flatten_netlist_with_models_config_with_abort(
            netlist,
            FlattenerConfig {
                max_depth: self.config.resource_limits.max_hierarchy_depth,
                max_elements: self.config.resource_limits.max_flattened_elements,
                ..FlattenerConfig::default()
            },
            abort,
        )
        .map_err(|error| map_build_parse_error("subcircuit flattening", error))?;
        let mut effective_model_netlist;
        let netlist = if flattened.scoped_models.is_empty() {
            netlist
        } else {
            effective_model_netlist = netlist.clone();
            effective_model_netlist
                .models
                .extend(flattened.scoped_models);
            &effective_model_netlist
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

        log::debug!("Building circuit with {} elements", flat_elements.len());
        if log::log_enabled!(log::Level::Trace) {
            for element in &flat_elements {
                log::trace!(
                    "Element {} nodes={:?} kind={:?}",
                    element.name,
                    element.nodes,
                    element.kind
                );
            }
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
                let entry = resolve_cached_or_compile_veriloga_with_limits(
                    &include.file_path,
                    self.config.resource_limits,
                )?;
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

        for (element_index, element) in flat_elements.iter().enumerate() {
            if element_index.is_multiple_of(64) {
                check_build_abort(abort)?;
                check_circuit_resource_limits(self, &circuit)?;
            }
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

                    let prepared_value_expr = value_expr
                        .as_deref()
                        .map(|expression| {
                            prepare_behavioral_expression(expression, &base_eval_context(netlist))
                                .map_err(|error| {
                                    SimulationError::Circuit(format!(
                                        "Resistor '{}' value expression could not be prepared: {}",
                                        element.name, error
                                    ))
                                })
                        })
                        .transpose()?;
                    let value_expr = prepared_value_expr.as_deref();

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

                    if let Some(expression) = value_expr
                        && expression_references_circuit_state(expression)
                    {
                        add_behavioral_resistor(
                            &mut circuit,
                            netlist,
                            element,
                            expression,
                            model.as_deref(),
                            instance_params,
                            self.config.temperature,
                            self.config.convergence_config.junction_gmin_target,
                            self.config.resource_limits,
                            self.config.spice_dialect,
                        )?;
                        continue;
                    }

                    let resistance = resolve_resistor_instance_value(
                        netlist,
                        &element.name,
                        *value,
                        value_expr,
                        model.as_deref(),
                        instance_params,
                        self.config.temperature,
                        self.config.spice_dialect,
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
                        self.config.spice_dialect,
                    )?;
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    if let Some(ic) = *initial_voltage {
                        if self.config.spice_dialect == SpiceDialect::Xyce {
                            let branch = circuit.allocate_branch_named(&element.name);
                            circuit.capacitors.add_with_ic_branch(
                                element.name.clone(),
                                np,
                                nn,
                                capacitance,
                                ic,
                                branch,
                            );
                        } else {
                            circuit.capacitors.add_with_ic(
                                element.name.clone(),
                                np,
                                nn,
                                capacitance,
                                ic,
                            );
                        }
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
                        self.config.spice_dialect,
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
                    validate_source_file_inputs(&element.name, spec, self.config.resource_limits)?;
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    let dc_value = extract_dc_value_with_limits(spec, self.config.resource_limits);
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
                        crate::netlist::SourceSpec::Distortion { .. }
                        | crate::netlist::SourceSpec::RfPort { .. }
                        | crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::Pat { .. }
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
                    validate_source_file_inputs(&element.name, spec, self.config.resource_limits)?;
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let dc_value = extract_dc_value_with_limits(spec, self.config.resource_limits);
                    let (ac_mag, ac_phase) = super::extract_ac_value(spec);
                    let transient_spec = match spec {
                        crate::netlist::SourceSpec::Distortion { .. }
                        | crate::netlist::SourceSpec::RfPort { .. }
                        | crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::Pat { .. }
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
                            !device_model.string_vector_params.is_empty()
                                || !device_model.real_vector_params.is_empty()
                                || !device_model.real_vector_expr_params.is_empty()
                                || !device_model.integer_vector_params.is_empty(),
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
                    if self.config.spice_dialect == SpiceDialect::Xyce {
                        diode.set_temperature_xyce_7(temp_k, tnom_k);
                    } else {
                        diode.set_temperature(temp_k, tnom_k);
                    }

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
                    bjt.set_xyce_thermal_voltage_constants(
                        self.config.spice_dialect == SpiceDialect::Xyce,
                    );
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
                            Bsim3VersionFamily::LegacyV31Metadata(version) => {
                                if self.config.spice_dialect != SpiceDialect::Xyce {
                                    return Err(SimulationError::Circuit(format!(
                                        "MOSFET '{}': BSIM3 VERSION={version} LEVEL={level} \
                                         requires a distinct native BSIM3v1 port outside Xyce \
                                         B3 compatibility mode; RSpice's BSIM3v3.3 native \
                                         evaluator must not be used as a generic VERSION={version} \
                                         compatibility fallback",
                                        element.name
                                    )));
                                }
                                // Xyce MOSFET_B3 treats VERSION=3.1 as accepted metadata on
                                // its B3 evaluator; it does not switch to ngspice BSIM3v1.
                            }
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

                    // Series RD/RS topology is constructed once below, after
                    // all model and instance parameters have been applied, so
                    // explicit RD/RS precedence, RSH squares, multiplicity,
                    // and noise temperature share one canonical path.
                    let drain = drain_external;
                    let source = source_external;

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

                    // Drain/source ohmic resistances, matching the canonical
                    // prime-node topology used by mos1temp.c and the legacy
                    // BSIM1/BSIM2 b1temp.c/b2temp.c paths: RD (or RS) when
                    // given, else RSH times the diffusion squares. ngspice
                    // stamps the conductance at internal prime nodes scaled
                    // by the multiplicity; the explicit resistor uses the
                    // reciprocal equivalent R/m, and the repointed device
                    // terminals make junction noise and limiting act at the
                    // true internal nodes. BSIM1/BSIM2 default NRD/NRS to one
                    // and therefore participate in this path as well.
                    let multiplicity = mosfet.multiplicity.max(1e-12);
                    let drain_r = if mosfet.rd_model > 0.0 {
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
                    let source_r = if mosfet.rs_model > 0.0 {
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
                    ..
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
                    ..
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
                    ..
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
                    ..
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

                    let mut bvs =
                        crate::device::BehavioralVoltageSource::new_with_source_path_and_limits(
                            element.name.clone(),
                            np,
                            nn,
                            branch,
                            &prepared_expression,
                            netlist.source_path.as_deref(),
                            self.config.resource_limits,
                        )
                        .map_err(SimulationError::Circuit)?;
                    bvs.set_temperature(crate::analysis::temperature::kelvin_to_celsius(
                        self.config.temperature,
                    ));
                    bvs.set_gmin(self.config.convergence_config.junction_gmin_target);
                    bvs.set_expression_dialect(netlist.params.expression_dialect());
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

                    let mut bcs =
                        crate::device::BehavioralCurrentSource::new_with_source_path_and_limits(
                            element.name.clone(),
                            np,
                            nn,
                            &prepared_expression,
                            netlist.source_path.as_deref(),
                            self.config.resource_limits,
                        )
                        .map_err(SimulationError::Circuit)?;
                    bcs.set_temperature(crate::analysis::temperature::kelvin_to_celsius(
                        self.config.temperature,
                    ));
                    bcs.set_gmin(self.config.convergence_config.junction_gmin_target);
                    bcs.set_expression_dialect(netlist.params.expression_dialect());
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
                                let matched =
                                    device.try_set_parameter(name, resolved).map_err(|error| {
                                        SimulationError::Circuit(format!(
                                            "Verilog-A device '{}' rejected parameter '{}': {}",
                                            element.name, name, error
                                        ))
                                    })?;
                                if !matched && name.eq_ignore_ascii_case("m") {
                                    if !resolved.is_finite() || resolved <= 0.0 {
                                        return Err(SimulationError::Circuit(format!(
                                            "Verilog-A device '{}' multiplicity must be a positive finite value, got {}",
                                            element.name, resolved
                                        )));
                                    }
                                    device.set_multiplicity(resolved);
                                } else if !matched {
                                    return Err(SimulationError::Circuit(format!(
                                        "Verilog-A device '{}' model '{}' has no parameter named '{}'",
                                        element.name, subckt_name, name
                                    )));
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
                            device
                                .try_set_temperature(self.config.temperature)
                                .map_err(|err| {
                                    SimulationError::Circuit(format!(
                                        "Verilog-A device '{}' temperature update failed: {}",
                                        element.name, err
                                    ))
                                })?;
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
                    if model_def.model_type.eq_ignore_ascii_case("ISWITCH")
                        || model_def.model_type.eq_ignore_ascii_case("ISW")
                        || model_def.model_type.eq_ignore_ascii_case("CSW")
                    {
                        let control_element =
                            parse_direct_branch_current_control(control_expression).ok_or_else(
                                || {
                                    SimulationError::Circuit(format!(
                                        "Generic switch '{}' does not yet support ISWITCH CONTROL expression '{}'; native ISWITCH mapping currently requires a direct branch-current control I(source)",
                                        element.name, control_expression
                                    ))
                                },
                            )?;
                        let params_map = resolve_supported_model_params_upper_map(
                            netlist,
                            model_def,
                            "Generic current-controlled switch",
                            &element.name,
                            model,
                            ISWITCH_MODEL_PARAMS,
                            self.config.temperature,
                        )?;

                        let mut sw = crate::device::CurrentSwitch::new(
                            element.name.clone(),
                            np,
                            nn,
                            control_element.clone(),
                        )
                        .with_params(&params_map);
                        if let Some(state) = initial_state {
                            sw = sw.with_initial_state(map_switch_state(*state));
                        }
                        let iswitch_idx = circuit.iswitches.len();
                        circuit.iswitches.push(sw);
                        circuit.add_iswitch_pending(iswitch_idx, control_element);
                        continue;
                    }
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
                    sw.set_expression_context(
                        crate::analysis::temperature::kelvin_to_celsius(self.config.temperature),
                        self.config.convergence_config.junction_gmin_target,
                        netlist.params.expression_dialect(),
                    );
                    if sw.has_solution_references() {
                        return Err(SimulationError::Circuit(format!(
                            "Generic switch '{}' CONTROL expression references circuit nodes or branch currents; native SWITCH CONTROL currently requires expressions independent of solution unknowns",
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
                    // Xyce's lossless TRA instance default is NL=0.25.  It
                    // participates only in the F/NL delay form; an explicit
                    // TD remains authoritative and does not acquire an
                    // electrical-length parameter.
                    let nl_eff = (*nl)
                        .or(model_params.and_then(|m| m.nl))
                        .or_else(|| (td.is_none() && freq_eff.is_some()).then_some(0.25));
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
                    pspice_u_timing: _,
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
                            self.config.spice_dialect,
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
                    instance.set_digital_delay_type(self.config.digital_delay_type);
                    instance.set_resource_limits(self.config.resource_limits);

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

        let default_auto_bridge_vcc = xspice_auto_bridge_vcc(netlist);
        let scoped_auto_bridge_metadata =
            xspice_auto_bridge_scoped_metadata(&circuit, &flattened.xspice_auto_bridge_node_hints);
        let auto_bridges = plan_xspice_auto_bridges(
            &circuit,
            &flat_elements,
            &scoped_auto_bridge_metadata,
            default_auto_bridge_vcc,
        );
        if !auto_bridges.is_empty() {
            if netlist.options.auto_bridge.unwrap_or(true) {
                add_planned_xspice_auto_bridges(
                    &mut circuit,
                    &auto_bridges,
                    &netlist.options.auto_bridge_templates,
                    netlist.source_path.as_deref(),
                    netlist.options.auto_bridge_family.unwrap_or(true),
                    self.config.temperature,
                    self.config.ramptime,
                    self.config.digital_delay_type,
                    self.config.spice_dialect,
                    netlist.options.auto_bridge_show_generated.unwrap_or(false),
                    self.config.resource_limits,
                    abort,
                )?;
            } else {
                reject_disabled_xspice_auto_bridge(&circuit, &auto_bridges)?;
            }
        }

        check_build_abort(abort)?;
        check_circuit_resource_limits(self, &circuit)?;

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
        for (coupling_index, coupling) in couplings.iter().enumerate() {
            if coupling_index.is_multiple_of(64) {
                check_build_abort(abort)?;
            }
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
                    let mut device = crate::device::CoupledInductorPair::new(
                        coupling.name.clone(),
                        circuit.inductors.node_pos[i],
                        circuit.inductors.node_neg[i],
                        circuit.inductors.inductances[i],
                        circuit.inductors.node_pos[j],
                        circuit.inductors.node_neg[j],
                        circuit.inductors.inductances[j],
                        coupling.coefficient,
                    );
                    device.set_initial_currents(
                        circuit.inductors.ic[i].unwrap_or(0.0),
                        circuit.inductors.ic[j].unwrap_or(0.0),
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

        check_build_abort(abort)?;
        Ok(circuit)
    }
}
