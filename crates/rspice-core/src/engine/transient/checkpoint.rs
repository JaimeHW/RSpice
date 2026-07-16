//! Transient checkpoint/restore.
//!
//! A checkpoint captures the integrator state at an accepted time point:
//! the full MNA solution plus the capacitor and inductor companion-model
//! histories. Restoring injects that state into a freshly built circuit and
//! continues integration from the checkpoint time with absolute-time source
//! evaluation — the same numerical regime as a breakpoint restart, which
//! the integrator already performs at every source discontinuity.
//!
//! Scope, stated precisely: accepted linear-reactive histories, generated
//! Verilog-A `ddt`/`idt` histories and limiter anchors, and XSPICE model-owned
//! checkpoint state are captured bit-exactly. Continuation deliberately takes
//! one order-one breakpoint-restart step before higher-order integration
//! resumes. Other nonlinear iteration memories and transmission-line delay
//! histories re-derive from the restored solution. Decks dominated by
//! transmission-line delays should prefer unsegmented runs (a warning is
//! logged at capture).
//!
//! The on-disk format is a versioned, line-oriented text format using
//! Rust's shortest-round-trip float formatting, so save/load reproduces
//! every `f64` bit-exactly with no serialization dependencies (core stays
//! lean for the wasm build).

use crate::Value;
use crate::analysis::LteEstimator;
use crate::circuit::Circuit;
use crate::device::veriloga_generated::{
    GENERATED_PERSISTENT_STATE_VERSION, GeneratedVerilogAInstanceCheckpoint,
    GeneratedVerilogAPersistentState,
};
use crate::engine::SimulationConfig;
use crate::expr::{Expr, Function, parse_expression_strict};
use crate::netlist::expr::prepare_behavioral_expression;
use crate::netlist::{
    Element, ElementKind, Netlist, ParamContext, SourceSpec, SubcircuitDef, TransientLteReference,
    flatten_netlist_with_models,
};
use crate::xspice::{CmContextCheckpoint, XspiceInstanceCheckpoint};

/// Format version written to and required from checkpoint files.
const FORMAT_VERSION: u32 = 9;

/// Snapshot of transient-integration state at an accepted time point.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientCheckpoint {
    /// Simulation time of the snapshot (s).
    pub time: Value,
    /// Full MNA solution (node voltages then branch currents).
    pub solution: Vec<Value>,
    /// Fingerprint of the netlist this state belongs to; restore refuses a
    /// mismatch rather than silently continuing a different circuit.
    pub netlist_fingerprint: u64,
    /// Collision-resistant identity of the fully elaborated semantic netlist.
    /// Legacy files have no safe identity and are refused for resume.
    netlist_identity: Option<String>,
    /// Identity of the resolved, state-affecting simulation configuration.
    /// Kept optional solely so legacy checkpoint files can be parsed and
    /// rejected with a precise diagnostic.
    simulation_identity: Option<String>,

    cap_v_prev: Vec<Value>,
    cap_v_prev_prev: Vec<Value>,
    cap_v_prev_prev_prev: Vec<Value>,
    cap_i_prev: Vec<Value>,
    cap_i_eq: Vec<Value>,
    ind_i_prev: Vec<Value>,
    ind_i_prev_prev: Vec<Value>,
    ind_v_prev: Vec<Value>,
    lte_signal_global_reference: Value,
    lte_signal_local_reference: Vec<Value>,
    lte_reference_history_available: bool,
    lte_reference_mode: Option<TransientLteReference>,
    xspice_instances: Vec<String>,
    xspice_resume_blockers: Vec<String>,
    xspice_instance_states: Vec<XspiceInstanceCheckpoint>,
    generated_veriloga_state_available: bool,
    generated_veriloga_instance_states: Vec<GeneratedVerilogAInstanceCheckpoint>,
}

/// Stable legacy fingerprint of the netlist identity (FNV-1a over source text
/// when available). Resume authorization additionally requires the complete
/// collision-resistant semantic identity below.
pub fn netlist_fingerprint(netlist: &Netlist) -> u64 {
    const OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01B3;
    let mut hash = OFFSET;
    let mut feed = |bytes: &[u8]| {
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
    };

    if let Some(text) = &netlist.source_text {
        feed(text.as_bytes());
    } else {
        for element in &netlist.elements {
            feed(element.name.as_bytes());
            for node in &element.nodes {
                feed(node.as_bytes());
            }
        }
    }
    hash
}

fn hash_field(hasher: &mut blake3::Hasher, name: &str, value: impl std::fmt::Debug) {
    let value = format!("{value:?}");
    hasher.update(&(name.len() as u64).to_le_bytes());
    hasher.update(name.as_bytes());
    hasher.update(&(value.len() as u64).to_le_bytes());
    hasher.update(value.as_bytes());
}

fn hash_subcircuits(hasher: &mut blake3::Hasher, subcircuits: &[SubcircuitDef]) {
    hasher.update(&(subcircuits.len() as u64).to_le_bytes());
    for subcircuit in subcircuits {
        hash_field(hasher, "name", &subcircuit.name);
        hash_field(hasher, "ports", &subcircuit.ports);
        hash_field(hasher, "elements", &subcircuit.elements);
        hash_field(hasher, "initial_conditions", &subcircuit.initial_conditions);
        hash_field(hasher, "node_sets", &subcircuit.node_sets);
        hash_field(hasher, "params", &subcircuit.params);
        hash_field(hasher, "expr_params", &subcircuit.expr_params);
        hash_field(hasher, "string_params", &subcircuit.string_params);
        hash_field(hasher, "body_params", &subcircuit.body_params);
        hash_field(hasher, "body_expr_params", &subcircuit.body_expr_params);
        hash_field(hasher, "body_string_params", &subcircuit.body_string_params);
        hash_field(hasher, "body_functions", &subcircuit.body_functions);
        let mut local_options = subcircuit
            .local_options
            .iter()
            .map(|(name, value)| (name, value.to_bits()))
            .collect::<Vec<_>>();
        local_options.sort_unstable_by(|left, right| left.0.cmp(right.0));
        hash_field(hasher, "local_options", local_options);
        hash_field(hasher, "library_ref", &subcircuit.library_ref);
        hash_subcircuits(hasher, &subcircuit.nested_subcircuits);
    }
}

fn resolved_dependency_path(
    path: &str,
    source_path: Option<&std::path::Path>,
) -> std::path::PathBuf {
    let candidate = std::path::Path::new(path);
    if candidate.is_absolute() {
        candidate.to_path_buf()
    } else if let Some(parent) = source_path.and_then(std::path::Path::parent) {
        parent.join(candidate)
    } else {
        candidate.to_path_buf()
    }
}

fn hash_dependency(
    hasher: &mut blake3::Hasher,
    path: &str,
    source_path: Option<&std::path::Path>,
    xspice_virtual_aware: bool,
) {
    let resolved = resolved_dependency_path(path, source_path);
    let resolved_text = resolved.to_string_lossy();
    hash_field(hasher, "dependency_path", &resolved_text);
    if xspice_virtual_aware
        && let Some(contents) = crate::xspice::checkpoint_virtual_data_file_contents(&resolved_text)
    {
        hash_field(hasher, "dependency_kind", "virtual");
        hasher.update(&(contents.len() as u64).to_le_bytes());
        hasher.update(contents.as_bytes());
    } else {
        match std::fs::read(&resolved) {
            Ok(contents) => {
                hash_field(hasher, "dependency_kind", "native");
                hasher.update(&(contents.len() as u64).to_le_bytes());
                hasher.update(&contents);
            }
            Err(error) => {
                hash_field(hasher, "dependency_kind", "unavailable");
                hash_field(hasher, "dependency_error_kind", error.kind());
            }
        }
    }
}

fn hash_source_dependencies(
    hasher: &mut blake3::Hasher,
    source: &SourceSpec,
    source_path: Option<&std::path::Path>,
) {
    match source {
        SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
            hash_source_dependencies(hasher, inner, source_path);
        }
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            hash_source_dependencies(hasher, transient, source_path);
        }
        SourceSpec::PwlFile { path, .. } => hash_dependency(hasher, path, source_path, false),
        _ => {}
    }
}

fn file_lookup_function(function: Function) -> bool {
    matches!(
        function,
        Function::Table
            | Function::TableFile
            | Function::FastTable
            | Function::FastTableFile
            | Function::Cubic
            | Function::CubicFile
            | Function::Akima
            | Function::AkimaFile
            | Function::Wodicka
            | Function::WodickaFile
            | Function::Barycentric
            | Function::BarycentricFile
    )
}

fn hash_expression_dependencies(
    hasher: &mut blake3::Hasher,
    expression: &str,
    params: &ParamContext,
    source_path: Option<&std::path::Path>,
) {
    let Ok(prepared) = prepare_behavioral_expression(expression, params) else {
        return;
    };
    let Ok(expression) = parse_expression_strict(&prepared) else {
        return;
    };
    fn visit(
        hasher: &mut blake3::Hasher,
        expression: &Expr,
        source_path: Option<&std::path::Path>,
    ) {
        match expression {
            Expr::Function { func, args } => {
                if file_lookup_function(*func)
                    && let Some(Expr::StringLiteral(path)) = args.first()
                {
                    hash_dependency(hasher, path, source_path, false);
                }
                for argument in args {
                    visit(hasher, argument, source_path);
                }
            }
            Expr::Unary { operand, .. } => visit(hasher, operand, source_path),
            Expr::Binary { left, right, .. } => {
                visit(hasher, left, source_path);
                visit(hasher, right, source_path);
            }
            _ => {}
        }
    }
    visit(hasher, &expression, source_path);
}

fn hash_element_dependencies(
    hasher: &mut blake3::Hasher,
    elements: &[Element],
    params: &ParamContext,
    source_path: Option<&std::path::Path>,
) {
    for element in elements {
        match &element.kind {
            ElementKind::VoltageSource(source) | ElementKind::CurrentSource(source) => {
                hash_source_dependencies(hasher, source, source_path);
            }
            ElementKind::BehavioralVoltage { expression, .. }
            | ElementKind::BehavioralCurrent { expression, .. } => {
                hash_expression_dependencies(hasher, expression, params, source_path);
            }
            _ => {}
        }
    }
}

fn model_string_is_dependency(name: &str) -> bool {
    let name = name.trim().to_ascii_lowercase();
    name == "simulation"
        || name.ends_with("file")
        || name.ends_with("_file")
        || name.ends_with("path")
}

fn hash_external_dependencies(hasher: &mut blake3::Hasher, netlist: &Netlist) {
    let source_path = netlist.source_path.as_deref();
    let mut isolated_netlist = netlist.clone();
    isolated_netlist.params = netlist.params.checkpoint_isolated_clone();
    let flattened = flatten_netlist_with_models(&isolated_netlist).ok();
    let elements = flattened
        .as_ref()
        .map_or(netlist.elements.as_slice(), |flat| flat.elements.as_slice());
    hash_element_dependencies(hasher, elements, &isolated_netlist.params, source_path);
    let models = netlist.models.iter().chain(
        flattened
            .as_ref()
            .into_iter()
            .flat_map(|flat| flat.scoped_models.iter()),
    );
    for model in models {
        for (name, path) in &model.string_params {
            if model_string_is_dependency(name) {
                let path = if name.eq_ignore_ascii_case("process_file") {
                    path.trim_end_matches('|')
                } else {
                    path
                };
                let virtual_aware = !name.eq_ignore_ascii_case("process_file")
                    && !name.eq_ignore_ascii_case("simulation");
                hash_dependency(hasher, path, source_path, virtual_aware);
            }
        }
    }
    for include in &netlist.veriloga_includes {
        hash_dependency(
            hasher,
            &include.file_path.to_string_lossy(),
            source_path,
            false,
        );
    }
}

fn effective_device_initial_condition_projection(
    netlist: &Netlist,
) -> Option<Vec<(String, &'static str, Vec<Option<u64>>)>> {
    let mut isolated = netlist.clone();
    isolated.params = netlist.params.checkpoint_isolated_clone();
    let flattened = flatten_netlist_with_models(&isolated).ok()?;
    Some(
        flattened
            .elements
            .iter()
            .filter_map(|element| {
                let canonical_name = element.name.trim().replace(':', ".").to_ascii_uppercase();
                match &element.kind {
                    ElementKind::Capacitor {
                        initial_voltage, ..
                    } => Some((
                        canonical_name,
                        "capacitor",
                        vec![initial_voltage.map(f64::to_bits)],
                    )),
                    ElementKind::Mosfet {
                        instance_params, ..
                    } => {
                        const LABELS: [&str; 5] =
                            ["IC_VDS", "IC_VGS", "IC_VBS", "IC_VES", "IC_VPS"];
                        Some((
                            canonical_name,
                            "mosfet",
                            LABELS
                                .iter()
                                .map(|label| {
                                    instance_params
                                        .iter()
                                        .rev()
                                        .find(|(name, _)| name.eq_ignore_ascii_case(label))
                                        .map(|(_, value)| value.to_bits())
                                })
                                .collect(),
                        ))
                    }
                    _ => None,
                }
            })
            .collect(),
    )
}

fn hash_effective_device_initial_condition_overlay(hasher: &mut blake3::Hasher, netlist: &Netlist) {
    if netlist.device_initial_conditions.is_none() {
        return;
    }
    let Some(applied) = effective_device_initial_condition_projection(netlist) else {
        return;
    };
    let mut baseline = netlist.clone();
    baseline.device_initial_conditions = None;
    let Some(without_directive) = effective_device_initial_condition_projection(&baseline) else {
        return;
    };
    if applied != without_directive {
        hash_field(
            hasher,
            "effective_device_initial_condition_overlay",
            applied,
        );
    }
}

/// Collision-resistant identity of the fully elaborated semantic netlist.
/// Source paths, diagnostics, and original source spelling are excluded;
/// expanded include/SPEF content and public post-parse AST edits are included.
pub(super) fn netlist_checkpoint_identity(netlist: &Netlist) -> Option<String> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"rspice-transient-elaborated-netlist-v3\0");
    hash_field(&mut hasher, "title", &netlist.title);
    hash_field(&mut hasher, "elements", &netlist.elements);
    hash_field(&mut hasher, "analyses", &netlist.analyses);
    hash_field(&mut hasher, "fft_analyses", &netlist.fft_analyses);
    hash_field(&mut hasher, "data_tables", &netlist.data_tables);
    hash_field(&mut hasher, "models", &netlist.models);
    hash_subcircuits(&mut hasher, &netlist.subcircuits);
    hash_field(
        &mut hasher,
        "params",
        netlist.params.checkpoint_semantic_snapshot(),
    );
    hash_field(
        &mut hasher,
        "initial_conditions",
        &netlist.initial_conditions,
    );
    hash_field(&mut hasher, "node_sets", &netlist.node_sets);
    let mut global_nodes = netlist.global_nodes.iter().collect::<Vec<_>>();
    global_nodes.sort_unstable();
    hash_field(&mut hasher, "global_nodes", global_nodes);
    hash_field(&mut hasher, "measurements", &netlist.measurements);
    hash_field(&mut hasher, "saves", &netlist.saves);
    hash_field(
        &mut hasher,
        "output_requests",
        netlist
            .output_requests
            .iter()
            .map(|request| {
                (
                    request.directive,
                    request.name.as_deref(),
                    request.dependencies.as_slice(),
                )
            })
            .collect::<Vec<_>>(),
    );
    hash_field(&mut hasher, "options", &netlist.options);
    hash_field(&mut hasher, "veriloga_includes", &netlist.veriloga_includes);
    hash_field(&mut hasher, "spef_includes", &netlist.spef_includes);
    hash_effective_device_initial_condition_overlay(&mut hasher, netlist);
    hash_external_dependencies(&mut hasher, netlist);
    Some(hasher.finalize().to_hex().to_string())
}

pub(super) fn simulation_checkpoint_identity(config: &SimulationConfig) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"rspice-transient-resolved-config-v1\0");
    hash_field(&mut hasher, "temperature", config.temperature.to_bits());
    hash_field(&mut hasher, "ramptime", config.ramptime.to_bits());
    hash_field(&mut hasher, "digital_delay_type", config.digital_delay_type);
    hash_field(&mut hasher, "integration_method", config.integration_method);
    hash_field(&mut hasher, "spice_dialect", config.spice_dialect);
    hash_field(
        &mut hasher,
        "jfet_level2_model",
        config.resolved_jfet_level2_model(),
    );
    hash_field(&mut hasher, "b3soi_gmin_scaling", config.b3soi_gmin_scaling);
    hash_field(
        &mut hasher,
        "transient_trtol",
        config.transient_trtol.to_bits(),
    );
    hash_field(
        &mut hasher,
        "transient_lte_reltol",
        config.transient_lte_reltol.map(f64::to_bits),
    );
    hash_field(
        &mut hasher,
        "transient_lte_abstol",
        config.transient_lte_abstol.map(f64::to_bits),
    );
    hash_field(
        &mut hasher,
        "transient_lte_reference",
        config.transient_lte_reference,
    );
    hash_field(
        &mut hasher,
        "transient_new_bp_stepping",
        config.transient_new_bp_stepping,
    );
    hash_field(
        &mut hasher,
        "transient_node_activity_bound",
        config.transient_node_activity_bound.to_bits(),
    );
    hash_field(
        &mut hasher,
        "gmin_target",
        config.convergence_config.gmin_target.to_bits(),
    );
    hash_field(
        &mut hasher,
        "junction_gmin_target",
        config.convergence_config.junction_gmin_target.to_bits(),
    );
    hasher.finalize().to_hex().to_string()
}

fn parse_count_header(line: &str, name: &str) -> Result<usize, String> {
    let mut fields = line.split_whitespace();
    let section = fields
        .next()
        .ok_or_else(|| format!("malformed '{name}' header: '{line}'"))?;
    if section != name {
        return Err(format!("malformed '{name}' header: '{line}'"));
    }
    let count = fields
        .next()
        .ok_or_else(|| format!("malformed '{name}' header: '{line}'"))?
        .parse::<usize>()
        .map_err(|_| format!("malformed '{name}' header: '{line}'"))?;
    if let Some(extra) = fields.next() {
        return Err(format!("malformed '{name}' header: extra field '{extra}'"));
    }
    Ok(count)
}

struct CheckpointLines<'a> {
    inner: std::str::Lines<'a>,
    remaining: usize,
}

impl<'a> CheckpointLines<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            inner: text.lines(),
            remaining: text.lines().count(),
        }
    }

    fn remaining(&self) -> usize {
        self.remaining
    }
}

impl<'a> Iterator for CheckpointLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.inner.next()?;
        self.remaining -= 1;
        Some(line)
    }
}

fn allocate_checkpoint_rows<T>(
    lines: &CheckpointLines<'_>,
    count: usize,
    name: &str,
) -> Result<Vec<T>, String> {
    let remaining_rows = lines.remaining();
    if count > remaining_rows {
        return Err(format!(
            "'{name}' declares {count} rows but only {remaining_rows} checkpoint rows remain"
        ));
    }

    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| format!("'{name}' count {count} exceeds checkpoint allocation limits"))?;
    Ok(values)
}

fn write_value_vector(out: &mut String, name: &str, values: &[Value]) {
    out.push_str(&format!("{name} {}\n", values.len()));
    for value in values {
        out.push_str(&value.to_string());
        out.push('\n');
    }
}

fn write_i64_vector(out: &mut String, name: &str, values: &[i64]) {
    out.push_str(&format!("{name} {}\n", values.len()));
    for value in values {
        out.push_str(&value.to_string());
        out.push('\n');
    }
}

fn read_value_vector(lines: &mut CheckpointLines<'_>, name: &str) -> Result<Vec<Value>, String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' vector"))?;
    let count = parse_count_header(header, name)?;
    let mut values = allocate_checkpoint_rows(lines, count, name)?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' vector truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        let field = fields
            .next()
            .ok_or_else(|| format!("'{name}' row {row} is empty"))?;
        let value = field
            .parse::<Value>()
            .map_err(|_| format!("'{name}' row {row}: bad value '{field}'"))?;
        if let Some(extra) = fields.next() {
            return Err(format!("'{name}' row {row}: extra field '{extra}'"));
        }
        values.push(value);
    }
    Ok(values)
}

fn read_value_section(
    lines: &mut CheckpointLines<'_>,
    name: &str,
    columns: usize,
) -> Result<Vec<Vec<Value>>, String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' section"))?;
    let count = parse_count_header(header, name)?;
    if columns == 0 {
        return Err(format!("'{name}' section must have at least one column"));
    }
    let mut cols = Vec::new();
    cols.try_reserve_exact(columns)
        .map_err(|_| format!("'{name}' column count {columns} exceeds allocation limits"))?;
    for _ in 0..columns {
        cols.push(allocate_checkpoint_rows(lines, count, name)?);
    }
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        for col in &mut cols {
            let field = fields
                .next()
                .ok_or_else(|| format!("'{name}' row {row} is short"))?;
            let value = field
                .parse::<Value>()
                .map_err(|_| format!("'{name}' row {row}: bad value '{field}'"))?;
            col.push(value);
        }
        if let Some(extra) = fields.next() {
            return Err(format!("'{name}' row {row}: extra field '{extra}'"));
        }
    }
    Ok(cols)
}

fn read_i64_vector(lines: &mut CheckpointLines<'_>, name: &str) -> Result<Vec<i64>, String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' vector"))?;
    let count = parse_count_header(header, name)?;
    let mut values = allocate_checkpoint_rows(lines, count, name)?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' vector truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        let field = fields
            .next()
            .ok_or_else(|| format!("'{name}' row {row} is empty"))?;
        let value = field
            .parse::<i64>()
            .map_err(|_| format!("'{name}' row {row}: bad value '{field}'"))?;
        if let Some(extra) = fields.next() {
            return Err(format!("'{name}' row {row}: extra field '{extra}'"));
        }
        values.push(value);
    }
    Ok(values)
}

fn read_nonempty_line_vector(
    lines: &mut CheckpointLines<'_>,
    name: &str,
) -> Result<Vec<String>, String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' section"))?;
    let count = parse_count_header(header, name)?;
    let mut values = allocate_checkpoint_rows(lines, count, name)?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' truncated at row {row}"))?;
        let value = line.trim();
        if value.is_empty() {
            return Err(format!("'{name}' row {row} is empty"));
        }
        values.push(value.to_string());
    }
    Ok(values)
}

fn read_xspice_instance_states(
    lines: &mut CheckpointLines<'_>,
    version: u32,
) -> Result<Vec<XspiceInstanceCheckpoint>, String> {
    let header = lines
        .next()
        .ok_or_else(|| "missing 'xspice_states' section".to_string())?;
    let count = parse_count_header(header, "xspice_states")?;
    let mut states = allocate_checkpoint_rows(lines, count, "xspice_states")?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'xspice_states' truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        if fields.next() != Some("xspice_state") {
            return Err(format!("malformed 'xspice_state' header: '{line}'"));
        }
        let name = fields
            .next()
            .ok_or_else(|| format!("'xspice_state' row {row} is missing instance name"))?;
        let model = fields
            .next()
            .ok_or_else(|| format!("'xspice_state' row {row} is missing model name"))?;
        if let Some(extra) = fields.next() {
            return Err(format!("'xspice_state' row {row}: extra field '{extra}'"));
        }
        let (time, time_prev) = if version >= 5 {
            let times = read_value_vector(lines, "context_time")?;
            if times.len() != 2 {
                return Err(format!(
                    "'context_time' for XSPICE state row {row} must contain 2 values, got {}",
                    times.len()
                ));
            }
            (times[0], times[1])
        } else {
            (0.0, 0.0)
        };
        states.push(XspiceInstanceCheckpoint {
            name: name.to_string(),
            model: model.to_string(),
            context: CmContextCheckpoint {
                time,
                time_prev,
                state: read_value_vector(lines, "state")?,
                state_prev: read_value_vector(lines, "state_prev")?,
                int_state: read_i64_vector(lines, "int_state")?,
            },
        });
    }
    Ok(states)
}

fn parse_checkpoint_bool(field: &str, context: &str) -> Result<bool, String> {
    match field {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err(format!(
            "{context}: checkpoint boolean must be 0 or 1, found '{field}'"
        )),
    }
}

fn read_generated_state_rows(
    lines: &mut CheckpointLines<'_>,
    name: &str,
    value_columns: usize,
) -> Result<(Vec<Vec<Value>>, Vec<bool>), String> {
    let header = lines
        .next()
        .ok_or_else(|| format!("missing '{name}' section"))?;
    let count = parse_count_header(header, name)?;
    let mut values = Vec::with_capacity(value_columns);
    for _ in 0..value_columns {
        values.push(allocate_checkpoint_rows(lines, count, name)?);
    }
    let mut initialized = allocate_checkpoint_rows(lines, count, name)?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'{name}' truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        for column in &mut values {
            let field = fields
                .next()
                .ok_or_else(|| format!("'{name}' row {row} is short"))?;
            let value = field
                .parse::<Value>()
                .map_err(|_| format!("'{name}' row {row}: bad value '{field}'"))?;
            if !value.is_finite() {
                return Err(format!("'{name}' row {row}: non-finite value '{field}'"));
            }
            column.push(value);
        }
        let boolean = fields
            .next()
            .ok_or_else(|| format!("'{name}' row {row} is missing initialized state"))?;
        initialized.push(parse_checkpoint_bool(
            boolean,
            &format!("'{name}' row {row}"),
        )?);
        if let Some(extra) = fields.next() {
            return Err(format!("'{name}' row {row}: extra field '{extra}'"));
        }
    }
    Ok((values, initialized))
}

fn read_generated_veriloga_states(
    lines: &mut CheckpointLines<'_>,
) -> Result<Vec<GeneratedVerilogAInstanceCheckpoint>, String> {
    let header = lines
        .next()
        .ok_or_else(|| "missing 'generated_veriloga_states' section".to_string())?;
    let count = parse_count_header(header, "generated_veriloga_states")?;
    let mut states = allocate_checkpoint_rows(lines, count, "generated_veriloga_states")?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'generated_veriloga_states' truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        if fields.next() != Some("generated_veriloga_state") {
            return Err(format!(
                "malformed 'generated_veriloga_state' header: '{line}'"
            ));
        }
        let instance_name = fields
            .next()
            .ok_or_else(|| format!("generated state row {row} is missing instance name"))?;
        let model_name = fields
            .next()
            .ok_or_else(|| format!("generated state row {row} is missing model name"))?;
        let model_identity = fields
            .next()
            .ok_or_else(|| format!("generated state row {row} is missing model identity"))?;
        let state_version = fields
            .next()
            .ok_or_else(|| format!("generated state row {row} is missing state version"))?
            .parse::<u32>()
            .map_err(|_| format!("generated state row {row} has invalid state version"))?;
        if state_version != GENERATED_PERSISTENT_STATE_VERSION {
            return Err(format!(
                "generated state row {row} uses unsupported persistent-state version {state_version}"
            ));
        }
        if let Some(extra) = fields.next() {
            return Err(format!("generated state row {row}: extra field '{extra}'"));
        }

        let (mut ddt, ddt_initialized) = read_generated_state_rows(lines, "ddt_state", 3)?;
        let (mut idt, idt_initialized) = read_generated_state_rows(lines, "idt_state", 1)?;
        let (mut limiter, limiter_initialized) =
            read_generated_state_rows(lines, "limiter_state", 1)?;
        states.push(GeneratedVerilogAInstanceCheckpoint {
            instance_name: instance_name.to_string(),
            model_name: model_name.to_string(),
            model_identity: model_identity.to_string(),
            state_version,
            state: GeneratedVerilogAPersistentState {
                ddt_previous: ddt.remove(0),
                ddt_older: ddt.remove(0),
                ddt_derivative_previous: ddt.remove(0),
                ddt_initialized,
                idt_previous: idt.remove(0),
                idt_initialized,
                limiter_anchor: limiter.remove(0),
                limiter_initialized,
            },
        });
    }
    Ok(states)
}

impl TransientCheckpoint {
    fn validate_numeric_state(&self) -> Result<(), String> {
        if !self.time.is_finite() || self.time < 0.0 {
            return Err("checkpoint time must be finite and non-negative".to_string());
        }
        if self.netlist_identity.as_ref().is_some_and(|identity| {
            identity.len() != 64
                || !identity
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        }) {
            return Err(
                "checkpoint netlist identity must be 64 lowercase hexadecimal digits".to_string(),
            );
        }
        if self.solution.iter().any(|value| !value.is_finite()) {
            return Err("checkpoint solution values must be finite".to_string());
        }

        let capacitor_len = self.cap_v_prev.len();
        if [
            self.cap_v_prev_prev.len(),
            self.cap_v_prev_prev_prev.len(),
            self.cap_i_prev.len(),
            self.cap_i_eq.len(),
        ]
        .into_iter()
        .any(|len| len != capacitor_len)
        {
            return Err(
                "checkpoint capacitor history vectors have inconsistent lengths".to_string(),
            );
        }
        let inductor_len = self.ind_i_prev.len();
        if [self.ind_i_prev_prev.len(), self.ind_v_prev.len()]
            .into_iter()
            .any(|len| len != inductor_len)
        {
            return Err(
                "checkpoint inductor history vectors have inconsistent lengths".to_string(),
            );
        }
        if self
            .cap_v_prev
            .iter()
            .chain(&self.cap_v_prev_prev)
            .chain(&self.cap_v_prev_prev_prev)
            .chain(&self.cap_i_prev)
            .chain(&self.cap_i_eq)
            .chain(&self.ind_i_prev)
            .chain(&self.ind_i_prev_prev)
            .chain(&self.ind_v_prev)
            .any(|value| !value.is_finite())
        {
            return Err("checkpoint reactive history values must be finite".to_string());
        }
        if !self.lte_signal_global_reference.is_finite()
            || self.lte_signal_global_reference < 0.0
            || self
                .lte_signal_local_reference
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(
                "checkpoint LTE reference values must be finite and non-negative".to_string(),
            );
        }
        if self.xspice_instance_states.iter().any(|instance| {
            !instance.context.time.is_finite()
                || instance.context.time < 0.0
                || !instance.context.time_prev.is_finite()
                || instance.context.time_prev < 0.0
                || instance
                    .context
                    .state
                    .iter()
                    .chain(&instance.context.state_prev)
                    .any(|value| !value.is_finite())
        }) {
            return Err("checkpoint XSPICE floating-point state must be finite".to_string());
        }
        if !self.generated_veriloga_state_available
            && !self.generated_veriloga_instance_states.is_empty()
        {
            return Err(
                "generated Verilog-A checkpoint state is present without availability provenance"
                    .to_string(),
            );
        }
        for (index, instance) in self.generated_veriloga_instance_states.iter().enumerate() {
            if instance.instance_name.is_empty()
                || instance.instance_name.chars().any(char::is_whitespace)
                || instance.model_name.is_empty()
                || instance.model_name.chars().any(char::is_whitespace)
                || instance.model_identity.len() != 64
                || !instance
                    .model_identity
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                return Err(format!(
                    "generated Verilog-A checkpoint instance {index} has invalid textual provenance"
                ));
            }
            if instance.state_version != GENERATED_PERSISTENT_STATE_VERSION {
                return Err(format!(
                    "generated Verilog-A checkpoint instance {index} uses unsupported persistent-state version {}",
                    instance.state_version
                ));
            }
            let state = &instance.state;
            if state.ddt_previous.len() != state.ddt_older.len()
                || state.ddt_previous.len() != state.ddt_derivative_previous.len()
                || state.ddt_previous.len() != state.ddt_initialized.len()
                || state.idt_previous.len() != state.idt_initialized.len()
                || state.limiter_anchor.len() != state.limiter_initialized.len()
            {
                return Err(format!(
                    "generated Verilog-A checkpoint instance {index} has inconsistent persistent-state lengths"
                ));
            }
            if state
                .ddt_previous
                .iter()
                .chain(&state.ddt_older)
                .chain(&state.ddt_derivative_previous)
                .chain(&state.idt_previous)
                .chain(&state.limiter_anchor)
                .any(|value| !value.is_finite())
            {
                return Err(format!(
                    "generated Verilog-A checkpoint instance {index} contains non-finite persistent state"
                ));
            }
        }
        Ok(())
    }

    /// Capture the integrator state from a circuit at time `time` with the
    /// current accepted `solution`.
    pub(crate) fn capture(
        fingerprint: u64,
        netlist_identity: Option<String>,
        simulation_identity: String,
        time: Value,
        solution: &[Value],
        circuit: &Circuit,
        lte_estimator: Option<&LteEstimator>,
    ) -> Self {
        if !circuit.tlines.is_empty() || !circuit.coupled_tlines.is_empty() {
            log::warn!(
                "transient checkpoint at t={time:.6e}: transmission-line delay \
                 histories are re-derived on resume (breakpoint-restart \
                 semantics); prefer unsegmented runs for delay-dominated decks"
            );
        }
        let xspice_instances: Vec<String> = circuit
            .xspice_instances
            .iter()
            .map(|instance| format!("{}({})", instance.name, instance.model_name()))
            .collect();
        let xspice_resume_blockers = circuit.xspice_checkpoint_resume_blockers();
        let xspice_instance_states = if xspice_resume_blockers.is_empty() {
            circuit.xspice_checkpoint_instance_states()
        } else {
            Vec::new()
        };
        if !xspice_resume_blockers.is_empty() {
            log::warn!(
                "transient checkpoint at t={time:.6e}: XSPICE code-model \
                 state is not fully serialized; this checkpoint will be refused \
                 for resume: {}",
                xspice_resume_blockers.join("; ")
            );
        }

        let (lte_signal_global_reference, lte_signal_local_reference) = lte_estimator
            .map(LteEstimator::signal_reference_snapshot)
            .map_or((0.0, Vec::new()), |(global, local)| {
                (global, local.to_vec())
            });

        Self {
            time,
            solution: solution.to_vec(),
            netlist_fingerprint: fingerprint,
            netlist_identity,
            simulation_identity: Some(simulation_identity),
            cap_v_prev: circuit.capacitors.v_prev.clone(),
            cap_v_prev_prev: circuit.capacitors.v_prev_prev.clone(),
            cap_v_prev_prev_prev: circuit.capacitors.v_prev_prev_prev.clone(),
            cap_i_prev: circuit.capacitors.i_prev.clone(),
            cap_i_eq: circuit.capacitors.i_eq.clone(),
            ind_i_prev: circuit.inductors.i_prev.clone(),
            ind_i_prev_prev: circuit.inductors.i_prev_prev.clone(),
            ind_v_prev: circuit.inductors.v_prev.clone(),
            lte_signal_global_reference,
            lte_signal_local_reference,
            lte_reference_history_available: lte_estimator.is_some(),
            lte_reference_mode: lte_estimator.map(LteEstimator::reference_mode),
            xspice_instances,
            xspice_resume_blockers,
            xspice_instance_states,
            generated_veriloga_state_available: true,
            generated_veriloga_instance_states: circuit.generated_veriloga_checkpoint_states(),
        }
    }

    /// Inject the captured reactive-state histories into a freshly built
    /// circuit. Lengths must match the capture exactly.
    pub(crate) fn inject(&self, circuit: &mut Circuit) -> Result<(), String> {
        self.validate_numeric_state()?;
        let target_lengths = [
            (
                "capacitor v_prev",
                self.cap_v_prev.len(),
                circuit.capacitors.v_prev.len(),
            ),
            (
                "capacitor v_prev_prev",
                self.cap_v_prev_prev.len(),
                circuit.capacitors.v_prev_prev.len(),
            ),
            (
                "capacitor v_prev_prev_prev",
                self.cap_v_prev_prev_prev.len(),
                circuit.capacitors.v_prev_prev_prev.len(),
            ),
            (
                "capacitor i_prev",
                self.cap_i_prev.len(),
                circuit.capacitors.i_prev.len(),
            ),
            (
                "capacitor i_eq",
                self.cap_i_eq.len(),
                circuit.capacitors.i_eq.len(),
            ),
            (
                "inductor i_prev",
                self.ind_i_prev.len(),
                circuit.inductors.i_prev.len(),
            ),
            (
                "inductor i_prev_prev",
                self.ind_i_prev_prev.len(),
                circuit.inductors.i_prev_prev.len(),
            ),
            (
                "inductor v_prev",
                self.ind_v_prev.len(),
                circuit.inductors.v_prev.len(),
            ),
        ];
        if let Some((name, captured, target)) = target_lengths
            .into_iter()
            .find(|(_, captured, target)| captured != target)
        {
            return Err(format!(
                "checkpoint {name} shape mismatch: captured {captured}, circuit has {target}"
            ));
        }
        circuit.validate_xspice_checkpoint_instance_states(&self.xspice_instance_states)?;
        circuit.validate_generated_veriloga_checkpoint_states(
            &self.generated_veriloga_instance_states,
            self.generated_veriloga_state_available,
        )?;

        // All state families are validated before the first mutation. The
        // restore calls below repeat their local validation defensively, but
        // cannot fail after this point without a violated internal invariant.
        circuit.capacitors.v_prev.copy_from_slice(&self.cap_v_prev);
        circuit
            .capacitors
            .v_prev_prev
            .copy_from_slice(&self.cap_v_prev_prev);
        circuit
            .capacitors
            .v_prev_prev_prev
            .copy_from_slice(&self.cap_v_prev_prev_prev);
        circuit.capacitors.i_prev.copy_from_slice(&self.cap_i_prev);
        circuit.capacitors.i_eq.copy_from_slice(&self.cap_i_eq);
        circuit.inductors.i_prev.copy_from_slice(&self.ind_i_prev);
        circuit
            .inductors
            .i_prev_prev
            .copy_from_slice(&self.ind_i_prev_prev);
        circuit.inductors.v_prev.copy_from_slice(&self.ind_v_prev);
        circuit.restore_xspice_checkpoint_instance_states(&self.xspice_instance_states)?;
        circuit.restore_generated_veriloga_checkpoint_states(
            &self.generated_veriloga_instance_states,
            self.generated_veriloga_state_available,
        )?;
        Ok(())
    }

    /// Restore accepted-solution LTE reference history for a resumed run.
    pub(crate) fn restore_lte_references(
        &self,
        estimator: &mut LteEstimator,
    ) -> Result<(), String> {
        if estimator.requires_signal_reference_history() && !self.lte_reference_history_available {
            return Err(
                "legacy transient checkpoint does not contain NEWLTE signal-history state"
                    .to_string(),
            );
        }
        if self.lte_reference_history_available
            && self.lte_reference_mode != Some(estimator.reference_mode())
        {
            return Err(format!(
                "transient checkpoint LTE reference mode {:?} does not match resumed mode {:?}",
                self.lte_reference_mode,
                estimator.reference_mode()
            ));
        }
        estimator.restore_signal_reference_snapshot(
            self.lte_signal_global_reference,
            &self.lte_signal_local_reference,
        )
    }

    /// Validate this checkpoint against a netlist before resuming.
    pub fn validate_for(&self, netlist: &Netlist) -> Result<(), String> {
        self.validate_numeric_state()?;
        let target_identity = netlist_checkpoint_identity(netlist)
            .expect("semantic netlist identity is available for every elaborated AST");
        let Some(captured_identity) = self.netlist_identity.as_deref() else {
            return Err(
                "legacy transient checkpoint does not contain a collision-resistant netlist identity"
                    .to_string(),
            );
        };
        if captured_identity != target_identity {
            return Err(format!(
                "checkpoint was captured from a different netlist (identity {captured_identity}, this deck is {target_identity}); refusing to resume mismatched state"
            ));
        }
        let fingerprint = netlist_fingerprint(netlist);
        if fingerprint != self.netlist_fingerprint {
            return Err(format!(
                "checkpoint was captured from a different netlist \
                 (fingerprint {:#018x}, this deck is {:#018x}); refusing to \
                 resume mismatched state",
                self.netlist_fingerprint, fingerprint
            ));
        }
        if !self.xspice_resume_blockers.is_empty() {
            return Err(format!(
                "transient checkpoint resume cannot restore unsupported XSPICE \
                 state: {}. Run XSPICE transient decks unsegmented.",
                self.xspice_resume_blockers.join("; ")
            ));
        }
        if self.xspice_instances.is_empty() && netlist_has_xspice(netlist) {
            return Err(
                "transient checkpoint resume cannot verify XSPICE state for this \
                 legacy checkpoint format; the target netlist contains XSPICE \
                 code-model instances. Run XSPICE transient decks unsegmented."
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(crate) fn validate_for_with_config(
        &self,
        netlist: &Netlist,
        config: &SimulationConfig,
    ) -> Result<(), String> {
        self.validate_for(netlist)?;
        let Some(captured_identity) = self.simulation_identity.as_deref() else {
            return Err(
                "legacy transient checkpoint does not contain a collision-resistant simulation configuration identity"
                    .to_string(),
            );
        };
        let target_identity = simulation_checkpoint_identity(config);
        if captured_identity != target_identity {
            return Err(format!(
                "checkpoint was captured with a different resolved simulation configuration (identity {captured_identity}, this run is {target_identity}); refusing to resume mismatched state"
            ));
        }
        Ok(())
    }

    //=========================================================================
    // Text serialization (versioned; exact f64 round-trip via shortest
    // round-trip Display formatting)
    //=========================================================================

    /// Serialize to the versioned text format.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("RSPICE-CHECKPOINT {FORMAT_VERSION}\n"));
        out.push_str(&format!("fingerprint {:#018x}\n", self.netlist_fingerprint));
        out.push_str(&format!(
            "netlist_identity {}\n",
            self.netlist_identity.as_deref().unwrap_or("none")
        ));
        out.push_str(&format!(
            "simulation_identity {}\n",
            self.simulation_identity.as_deref().unwrap_or("none")
        ));
        out.push_str(&format!("time {}\n", self.time));

        let section = |out: &mut String, name: &str, rows: &[&[Value]]| {
            let len = rows.first().map_or(0, |r| r.len());
            out.push_str(&format!("{name} {len}\n"));
            for i in 0..len {
                let line: Vec<String> = rows.iter().map(|r| r[i].to_string()).collect();
                out.push_str(&line.join(" "));
                out.push('\n');
            }
        };

        section(&mut out, "solution", &[&self.solution]);
        let lte_mode = match self.lte_reference_mode {
            None => "none".to_string(),
            Some(TransientLteReference::PredictorLocal) => "predictor-local".to_string(),
            Some(mode) => mode
                .xyce_selector()
                .expect("Xyce LTE mode has a selector")
                .to_string(),
        };
        out.push_str(&format!("lte_reference_mode {lte_mode}\n"));
        write_value_vector(
            &mut out,
            "lte_signal_global",
            &[self.lte_signal_global_reference],
        );
        write_value_vector(
            &mut out,
            "lte_signal_local",
            &self.lte_signal_local_reference,
        );
        section(
            &mut out,
            "capacitors",
            &[
                &self.cap_v_prev,
                &self.cap_v_prev_prev,
                &self.cap_v_prev_prev_prev,
                &self.cap_i_prev,
                &self.cap_i_eq,
            ],
        );
        section(
            &mut out,
            "inductors",
            &[&self.ind_i_prev, &self.ind_i_prev_prev, &self.ind_v_prev],
        );
        out.push_str(&format!("xspice {}\n", self.xspice_instances.len()));
        for instance in &self.xspice_instances {
            out.push_str(instance);
            out.push('\n');
        }
        out.push_str(&format!(
            "xspice_blockers {}\n",
            self.xspice_resume_blockers.len()
        ));
        for blocker in &self.xspice_resume_blockers {
            out.push_str(blocker);
            out.push('\n');
        }
        out.push_str(&format!(
            "xspice_states {}\n",
            self.xspice_instance_states.len()
        ));
        for instance in &self.xspice_instance_states {
            out.push_str(&format!(
                "xspice_state {} {}\n",
                instance.name, instance.model
            ));
            write_value_vector(
                &mut out,
                "context_time",
                &[instance.context.time, instance.context.time_prev],
            );
            write_value_vector(&mut out, "state", &instance.context.state);
            write_value_vector(&mut out, "state_prev", &instance.context.state_prev);
            write_i64_vector(&mut out, "int_state", &instance.context.int_state);
        }
        out.push_str(&format!(
            "generated_veriloga_state_available {}\n",
            u8::from(self.generated_veriloga_state_available)
        ));
        out.push_str(&format!(
            "generated_veriloga_states {}\n",
            self.generated_veriloga_instance_states.len()
        ));
        for instance in &self.generated_veriloga_instance_states {
            out.push_str(&format!(
                "generated_veriloga_state {} {} {} {}\n",
                instance.instance_name,
                instance.model_name,
                instance.model_identity,
                instance.state_version
            ));
            let state = &instance.state;
            out.push_str(&format!("ddt_state {}\n", state.ddt_previous.len()));
            for index in 0..state.ddt_previous.len() {
                out.push_str(&format!(
                    "{} {} {} {}\n",
                    state.ddt_previous[index],
                    state.ddt_older[index],
                    state.ddt_derivative_previous[index],
                    u8::from(state.ddt_initialized[index])
                ));
            }
            out.push_str(&format!("idt_state {}\n", state.idt_previous.len()));
            for index in 0..state.idt_previous.len() {
                out.push_str(&format!(
                    "{} {}\n",
                    state.idt_previous[index],
                    u8::from(state.idt_initialized[index])
                ));
            }
            out.push_str(&format!("limiter_state {}\n", state.limiter_anchor.len()));
            for index in 0..state.limiter_anchor.len() {
                out.push_str(&format!(
                    "{} {}\n",
                    state.limiter_anchor[index],
                    u8::from(state.limiter_initialized[index])
                ));
            }
        }
        out
    }

    /// Parse the versioned text format.
    pub fn from_text(text: &str) -> Result<Self, String> {
        let mut lines = CheckpointLines::new(text);

        let header = lines.next().ok_or("empty checkpoint file")?;
        let version: u32 = header
            .strip_prefix("RSPICE-CHECKPOINT ")
            .and_then(|v| v.trim().parse().ok())
            .ok_or_else(|| format!("not a checkpoint file (header: '{header}')"))?;
        if !(1..=FORMAT_VERSION).contains(&version) {
            return Err(format!(
                "unsupported checkpoint version {version} (this build reads {FORMAT_VERSION})"
            ));
        }

        let fingerprint_line = lines.next().ok_or("missing fingerprint line")?;
        let netlist_fingerprint = fingerprint_line
            .strip_prefix("fingerprint ")
            .map(str::trim)
            .and_then(|v| v.strip_prefix("0x"))
            .and_then(|v| u64::from_str_radix(v, 16).ok())
            .ok_or_else(|| format!("malformed fingerprint line: '{fingerprint_line}'"))?;

        let netlist_identity = if version >= 8 {
            let identity_line = lines.next().ok_or("missing netlist identity line")?;
            let identity = identity_line
                .strip_prefix("netlist_identity ")
                .map(str::trim)
                .ok_or_else(|| format!("malformed netlist identity line: '{identity_line}'"))?;
            if identity == "none" {
                None
            } else if identity.len() == 64
                && identity
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                Some(identity.to_string())
            } else {
                return Err(format!(
                    "malformed netlist identity line: '{identity_line}'"
                ));
            }
        } else {
            None
        };

        let simulation_identity = if version >= 9 {
            let identity_line = lines.next().ok_or("missing simulation identity line")?;
            let identity = identity_line
                .strip_prefix("simulation_identity ")
                .map(str::trim)
                .ok_or_else(|| format!("malformed simulation identity line: '{identity_line}'"))?;
            if identity == "none" {
                None
            } else if identity.len() == 64
                && identity
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            {
                Some(identity.to_string())
            } else {
                return Err(format!(
                    "malformed simulation identity line: '{identity_line}'"
                ));
            }
        } else {
            None
        };

        let time_line = lines.next().ok_or("missing time line")?;
        let time: Value = time_line
            .strip_prefix("time ")
            .and_then(|v| v.trim().parse().ok())
            .ok_or_else(|| format!("malformed time line: '{time_line}'"))?;

        let mut solution_cols = read_value_section(&mut lines, "solution", 1)?;
        if solution_cols[0].iter().any(|value| !value.is_finite()) {
            return Err("checkpoint solution values must be finite".to_string());
        }
        let (lte_reference_mode, lte_signal_global_reference, lte_signal_local_reference) =
            if version >= 6 {
                let mode_line = lines
                    .next()
                    .ok_or_else(|| "missing 'lte_reference_mode' line".to_string())?;
                let mode = match mode_line.strip_prefix("lte_reference_mode ").map(str::trim) {
                    Some("none") => None,
                    Some("predictor-local") => Some(TransientLteReference::PredictorLocal),
                    Some(selector) => {
                        let selector = selector.parse::<u8>().map_err(|_| {
                            format!("malformed LTE reference mode line: '{mode_line}'")
                        })?;
                        Some(
                            TransientLteReference::from_xyce_selector(selector).ok_or_else(
                                || format!("unsupported LTE reference mode in line: '{mode_line}'"),
                            )?,
                        )
                    }
                    None => {
                        return Err(format!("malformed LTE reference mode line: '{mode_line}'"));
                    }
                };
                let global = read_value_vector(&mut lines, "lte_signal_global")?;
                if global.len() != 1 || !global[0].is_finite() || global[0] < 0.0 {
                    return Err(
                        "'lte_signal_global' must contain one finite non-negative value"
                            .to_string(),
                    );
                }
                let local = read_value_vector(&mut lines, "lte_signal_local")?;
                if local.iter().any(|value| !value.is_finite() || *value < 0.0) {
                    return Err(
                        "'lte_signal_local' values must be finite and non-negative".to_string()
                    );
                }
                (mode, global[0], local)
            } else {
                (None, 0.0, Vec::new())
            };
        let cap_cols = read_value_section(&mut lines, "capacitors", 5)?;
        let ind_cols = read_value_section(&mut lines, "inductors", 3)?;
        let xspice_instances = if version >= 2 {
            read_nonempty_line_vector(&mut lines, "xspice")?
        } else {
            Vec::new()
        };
        let mut xspice_resume_blockers = if version >= 3 {
            read_nonempty_line_vector(&mut lines, "xspice_blockers")?
        } else {
            Vec::new()
        };
        if version == 2 && !xspice_instances.is_empty() {
            xspice_resume_blockers.extend(xspice_instances.iter().map(|instance| {
                format!("{instance}: legacy checkpoint did not record model checkpoint support")
            }));
        }
        let xspice_instance_states = if version >= 4 {
            read_xspice_instance_states(&mut lines, version)?
        } else {
            Vec::new()
        };
        let (generated_veriloga_state_available, generated_veriloga_instance_states) =
            if version >= 7 {
                let availability_line = lines.next().ok_or_else(|| {
                    "missing 'generated_veriloga_state_available' line".to_string()
                })?;
                let mut fields = availability_line.split_whitespace();
                if fields.next() != Some("generated_veriloga_state_available") {
                    return Err(format!(
                        "malformed generated Verilog-A availability line: '{availability_line}'"
                    ));
                }
                let available = fields
                    .next()
                    .ok_or_else(|| {
                        "generated Verilog-A availability line is missing its boolean".to_string()
                    })
                    .and_then(|field| {
                        parse_checkpoint_bool(field, "generated Verilog-A availability")
                    })?;
                if let Some(extra) = fields.next() {
                    return Err(format!(
                        "generated Verilog-A availability line has extra field '{extra}'"
                    ));
                }
                (available, read_generated_veriloga_states(&mut lines)?)
            } else {
                (false, Vec::new())
            };
        if let Some(extra) = lines.find(|line| !line.trim().is_empty()) {
            return Err(format!("checkpoint has trailing content: '{extra}'"));
        }

        let mut cap_iter = cap_cols.into_iter();
        let mut ind_iter = ind_cols.into_iter();
        let checkpoint = Self {
            time,
            solution: solution_cols.swap_remove(0),
            netlist_fingerprint,
            netlist_identity,
            simulation_identity,
            cap_v_prev: cap_iter.next().unwrap(),
            cap_v_prev_prev: cap_iter.next().unwrap(),
            cap_v_prev_prev_prev: cap_iter.next().unwrap(),
            cap_i_prev: cap_iter.next().unwrap(),
            cap_i_eq: cap_iter.next().unwrap(),
            ind_i_prev: ind_iter.next().unwrap(),
            ind_i_prev_prev: ind_iter.next().unwrap(),
            ind_v_prev: ind_iter.next().unwrap(),
            lte_signal_global_reference,
            lte_signal_local_reference,
            lte_reference_history_available: lte_reference_mode.is_some(),
            lte_reference_mode,
            xspice_instances,
            xspice_resume_blockers,
            xspice_instance_states,
            generated_veriloga_state_available,
            generated_veriloga_instance_states,
        };
        checkpoint.validate_numeric_state()?;
        Ok(checkpoint)
    }

    /// Write the checkpoint to a file.
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        self.validate_numeric_state()?;
        std::fs::write(path, self.to_text())
            .map_err(|e| format!("cannot write checkpoint '{}': {e}", path.display()))
    }

    /// Read a checkpoint from a file.
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read checkpoint '{}': {e}", path.display()))?;
        Self::from_text(&text)
    }
}

fn netlist_has_xspice(netlist: &Netlist) -> bool {
    netlist
        .elements
        .iter()
        .any(|element| matches!(element.kind, ElementKind::Xspice { .. }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TransientCheckpoint {
        TransientCheckpoint {
            time: 1.2345678901234567e-6,
            solution: vec![0.5, -3.25, 1.0e-15, f64::MIN_POSITIVE, -0.0],
            netlist_fingerprint: 0xDEAD_BEEF_0123_4567,
            netlist_identity: Some("fedcba9876543210".repeat(4)),
            simulation_identity: Some("abcdef0123456789".repeat(4)),
            cap_v_prev: vec![0.1, -0.2],
            cap_v_prev_prev: vec![0.09, -0.19],
            cap_v_prev_prev_prev: vec![0.08, -0.18],
            cap_i_prev: vec![1e-3, -2e-3],
            cap_i_eq: vec![5e-4, -6e-4],
            ind_i_prev: vec![7e-3],
            ind_i_prev_prev: vec![6.5e-3],
            ind_v_prev: vec![0.02],
            lte_signal_global_reference: 3.25,
            lte_signal_local_reference: Vec::new(),
            lte_reference_history_available: true,
            lte_reference_mode: Some(TransientLteReference::SignalGlobal),
            xspice_instances: Vec::new(),
            xspice_resume_blockers: Vec::new(),
            xspice_instance_states: Vec::new(),
            generated_veriloga_state_available: true,
            generated_veriloga_instance_states: vec![GeneratedVerilogAInstanceCheckpoint {
                instance_name: "xgen1".to_string(),
                model_name: "generated_model".to_string(),
                model_identity: "0123456789abcdef".repeat(4),
                state_version: GENERATED_PERSISTENT_STATE_VERSION,
                state: GeneratedVerilogAPersistentState {
                    ddt_previous: vec![-0.0, f64::MIN_POSITIVE],
                    ddt_older: vec![1.25, -2.5],
                    ddt_derivative_previous: vec![3.0, -4.0],
                    ddt_initialized: vec![true, false],
                    idt_previous: vec![5.5],
                    idt_initialized: vec![true],
                    limiter_anchor: vec![-0.75],
                    limiter_initialized: vec![true],
                },
            }],
        }
    }

    fn legacy_text(checkpoint: &TransientCheckpoint, version: u32) -> String {
        let text = checkpoint.to_text().replace(
            &format!("RSPICE-CHECKPOINT {FORMAT_VERSION}"),
            &format!("RSPICE-CHECKPOINT {version}"),
        );
        let mut output = String::new();
        let mut lines = text.lines();
        while let Some(line) = lines.next() {
            if version < 8 && line.starts_with("netlist_identity ") {
                continue;
            }
            if version < 9 && line.starts_with("simulation_identity ") {
                continue;
            }
            if version < 7 && line.starts_with("generated_veriloga_state_available ") {
                break;
            }
            if version < 6 && line.starts_with("lte_reference_mode ") {
                continue;
            }
            if version < 6
                && (line.starts_with("lte_signal_global ") || line.starts_with("lte_signal_local "))
            {
                let count = line
                    .split_whitespace()
                    .nth(1)
                    .expect("LTE checkpoint vector count")
                    .parse::<usize>()
                    .expect("numeric LTE checkpoint vector count");
                for _ in 0..count {
                    lines.next().expect("complete LTE checkpoint vector");
                }
                continue;
            }
            output.push_str(line);
            output.push('\n');
        }
        output
    }

    #[test]
    fn text_round_trip_is_bit_exact() {
        let original = sample();
        let restored = TransientCheckpoint::from_text(&original.to_text()).unwrap();
        assert_eq!(original, restored);
        // Bit-level check on the touchy values (subnormals, negative zero).
        for (a, b) in original.solution.iter().zip(&restored.solution) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
        for (a, b) in original.generated_veriloga_instance_states[0]
            .state
            .ddt_previous
            .iter()
            .zip(
                &restored.generated_veriloga_instance_states[0]
                    .state
                    .ddt_previous,
            )
        {
            assert_eq!(a.to_bits(), b.to_bits());
        }
    }

    #[test]
    fn signal_history_lte_references_round_trip_and_legacy_resume_fails_closed() {
        let checkpoint = sample();
        let restored = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("current checkpoint format parses");
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            crate::netlist::TransientLteReference::SignalGlobal,
        );
        estimator.seed_reference_prefix(&restored.solution, restored.solution.len());
        restored
            .restore_lte_references(&mut estimator)
            .expect("current checkpoint restores signal history");
        let (global, local) = estimator.signal_reference_snapshot();
        assert_eq!(global.to_bits(), 3.25f64.to_bits());
        assert!(local.is_empty());

        let legacy = TransientCheckpoint::from_text(&legacy_text(&checkpoint, 5))
            .expect("version-five checkpoint remains readable");
        let err = legacy
            .restore_lte_references(&mut estimator)
            .expect_err("legacy checkpoint cannot resume signal-history NEWLTE exactly");
        assert!(
            err.contains("NEWLTE signal-history"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn checkpoint_lte_reference_mode_mismatch_fails_closed() {
        let checkpoint = sample();
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            TransientLteReference::SignalLocal,
        );
        estimator.seed_reference_prefix(&checkpoint.solution, checkpoint.solution.len());

        let err = checkpoint
            .restore_lte_references(&mut estimator)
            .expect_err("checkpoint mode provenance must match the resumed solver");
        assert!(err.contains("does not match"), "unexpected error: {err}");
    }

    #[test]
    fn legacy_checkpoint_upgrade_does_not_invent_signal_history() {
        let legacy = TransientCheckpoint::from_text(&legacy_text(&sample(), 5))
            .expect("version-five checkpoint remains readable");
        assert!(!legacy.lte_reference_history_available);
        assert!(!legacy.generated_veriloga_state_available);
        assert!(legacy.generated_veriloga_instance_states.is_empty());

        let upgraded = TransientCheckpoint::from_text(&legacy.to_text())
            .expect("legacy checkpoint can be re-serialized in the current format");
        assert!(!upgraded.lte_reference_history_available);
        assert_eq!(upgraded.lte_reference_mode, None);
        assert!(!upgraded.generated_veriloga_state_available);
        assert!(upgraded.generated_veriloga_instance_states.is_empty());

        let mut estimator = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            TransientLteReference::SignalGlobal,
        );
        estimator.seed_reference_prefix(&upgraded.solution, upgraded.solution.len());
        let err = upgraded
            .restore_lte_references(&mut estimator)
            .expect_err("upgrading a legacy file cannot synthesize signal history");
        assert!(
            err.contains("NEWLTE signal-history"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn checkpoint_rejects_non_finite_solution_and_lte_reference_values() {
        for non_finite in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
            let mut checkpoint = sample();
            checkpoint.solution[0] = non_finite;
            let err = TransientCheckpoint::from_text(&checkpoint.to_text())
                .expect_err("non-finite accepted solutions must fail closed");
            assert!(err.contains("solution values"), "unexpected error: {err}");

            let mut checkpoint = sample();
            checkpoint.lte_signal_global_reference = non_finite;
            let err = TransientCheckpoint::from_text(&checkpoint.to_text())
                .expect_err("non-finite signal history must fail closed");
            assert!(err.contains("lte_signal_global"), "unexpected error: {err}");
        }

        let mut checkpoint = sample();
        checkpoint.time = -1.0;
        let err = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect_err("negative checkpoint time must fail closed");
        assert!(err.contains("checkpoint time"), "unexpected error: {err}");

        let mut checkpoint = sample();
        checkpoint.cap_v_prev[0] = Value::NAN;
        let err = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect_err("non-finite reactive history must fail closed");
        assert!(err.contains("reactive history"), "unexpected error: {err}");

        let mut checkpoint = sample();
        checkpoint.xspice_instance_states = vec![XspiceInstanceCheckpoint {
            name: "a1".to_string(),
            model: "stateful".to_string(),
            context: CmContextCheckpoint {
                time: 1.0,
                time_prev: 0.5,
                state: vec![Value::INFINITY],
                state_prev: vec![0.0],
                int_state: Vec::new(),
            },
        }];
        let err = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect_err("non-finite XSPICE state must fail closed");
        assert!(err.contains("XSPICE"), "unexpected error: {err}");
    }

    #[test]
    fn generated_state_parser_rejects_invalid_provenance_shape_and_values() {
        let text = sample().to_text();
        let err = TransientCheckpoint::from_text(&text.replace(
            "generated_veriloga_state_available 1",
            "generated_veriloga_state_available 2",
        ))
        .expect_err("availability provenance must be a strict checkpoint boolean");
        assert!(err.contains("must be 0 or 1"), "unexpected error: {err}");

        let identity = "0123456789abcdef".repeat(4);
        for invalid_identity in ["abc".to_string(), "A".repeat(64), "g".repeat(64)] {
            let malformed = text.replacen(&identity, &invalid_identity, 1);
            let err = TransientCheckpoint::from_text(&malformed)
                .expect_err("generated model identity must be lowercase BLAKE3 hex");
            assert!(
                err.contains("invalid textual provenance"),
                "unexpected error for identity {invalid_identity}: {err}"
            );
        }

        let err = TransientCheckpoint::from_text(&text.replace(
            &format!("generated_veriloga_state xgen1 generated_model {identity} 1"),
            &format!("generated_veriloga_state xgen1 generated_model {identity} 2"),
        ))
        .expect_err("unknown generated persistent-state versions must fail closed");
        assert!(
            err.contains("unsupported persistent-state version"),
            "unexpected error: {err}"
        );

        let err = TransientCheckpoint::from_text(&text.replacen("-0 1.25 3 1", "-0 1.25 3 2", 1))
            .expect_err("generated initialized state must be a strict boolean");
        assert!(err.contains("must be 0 or 1"), "unexpected error: {err}");

        let mut inconsistent = sample();
        inconsistent.generated_veriloga_instance_states[0]
            .state
            .ddt_previous
            .push(0.0);
        let err = inconsistent
            .validate_numeric_state()
            .expect_err("inconsistent generated state lengths must fail closed");
        assert!(
            err.contains("inconsistent persistent-state lengths"),
            "unexpected error: {err}"
        );

        let mut non_finite = sample();
        non_finite.generated_veriloga_instance_states[0]
            .state
            .limiter_anchor[0] = Value::NAN;
        let err = non_finite
            .validate_numeric_state()
            .expect_err("non-finite generated state must fail closed");
        assert!(
            err.contains("non-finite persistent state"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn generated_restore_rejection_leaves_all_reactive_histories_unchanged() {
        let checkpoint = sample();
        let mut circuit = Circuit::new();
        circuit.capacitors.v_prev = vec![91.0, 92.0];
        circuit.capacitors.v_prev_prev = vec![81.0, 82.0];
        circuit.capacitors.v_prev_prev_prev = vec![71.0, 72.0];
        circuit.capacitors.i_prev = vec![61.0, 62.0];
        circuit.capacitors.i_eq = vec![51.0, 52.0];
        circuit.inductors.i_prev = vec![41.0];
        circuit.inductors.i_prev_prev = vec![31.0];
        circuit.inductors.v_prev = vec![21.0];
        let before = (
            circuit.capacitors.v_prev.clone(),
            circuit.capacitors.v_prev_prev.clone(),
            circuit.capacitors.v_prev_prev_prev.clone(),
            circuit.capacitors.i_prev.clone(),
            circuit.capacitors.i_eq.clone(),
            circuit.inductors.i_prev.clone(),
            circuit.inductors.i_prev_prev.clone(),
            circuit.inductors.v_prev.clone(),
        );

        let err = checkpoint
            .inject(&mut circuit)
            .expect_err("generated instance mismatch must reject the checkpoint");
        assert!(err.contains("generated Verilog-A checkpoint"));
        assert_eq!(
            before,
            (
                circuit.capacitors.v_prev,
                circuit.capacitors.v_prev_prev,
                circuit.capacitors.v_prev_prev_prev,
                circuit.capacitors.i_prev,
                circuit.capacitors.i_eq,
                circuit.inductors.i_prev,
                circuit.inductors.i_prev_prev,
                circuit.inductors.v_prev,
            ),
            "checkpoint rejection must occur before any circuit state mutation"
        );
    }

    #[test]
    fn programmatic_netlists_have_complete_semantic_identities() {
        let netlist = Netlist::default();
        let identity = netlist_checkpoint_identity(&netlist).expect("semantic identity");
        let mut checkpoint = sample();
        checkpoint.netlist_fingerprint = netlist_fingerprint(&netlist);
        checkpoint.netlist_identity = Some(identity);
        checkpoint
            .validate_for(&netlist)
            .expect("programmatic AST identity authorizes its own checkpoint");
    }

    #[test]
    fn initcond_checkpoint_identity_uses_effective_semantic_overlay() {
        let first = Netlist::parse("initcond checkpoint\n.INITCOND C1 IC=1\nC1 1 0 1u\n.END\n")
            .expect("first INITCOND deck parses");
        let second = Netlist::parse("initcond checkpoint\n.INITCOND C1 IC=2\nC1 1 0 1u\n.END\n")
            .expect("second INITCOND deck parses");
        assert_ne!(
            netlist_checkpoint_identity(&first),
            netlist_checkpoint_identity(&second),
            "semantic INITCOND values must participate in checkpoint identity"
        );

        let relocated_source = "relocated initcond\n.INITCOND C1 IC=1\nC1 1 0 1u\n.END\n";
        let relocated_a = Netlist::parse_with_path(
            relocated_source,
            std::path::Path::new("location-a/deck.cir"),
        )
        .expect("first relocated deck parses");
        let relocated_b = Netlist::parse_with_path(
            relocated_source,
            std::path::Path::new("location-b/deck.cir"),
        )
        .expect("second relocated deck parses");
        assert_eq!(
            netlist_checkpoint_identity(&relocated_a),
            netlist_checkpoint_identity(&relocated_b),
            "directive source paths must not perturb semantic checkpoint identity"
        );

        let colon = Netlist::parse(
            "hierarchy spelling\n\
             .SUBCKT CELL d g s b\n\
             M1 d g s b MOD\n\
             .ENDS\n\
             .INITCOND X1:M1 IC=2,0\n\
             X1 1 2 0 0 CELL\n\
             .MODEL MOD NMOS\n\
             .END\n",
        )
        .expect("colon hierarchy deck parses");
        let dotted = Netlist::parse(
            "hierarchy spelling\n\
             .SUBCKT CELL d g s b\n\
             M1 d g s b MOD\n\
             .ENDS\n\
             .INITCOND x1.m1 IC=2,0\n\
             X1 1 2 0 0 CELL\n\
             .MODEL MOD NMOS\n\
             .END\n",
        )
        .expect("dotted hierarchy deck parses");
        assert_eq!(
            netlist_checkpoint_identity(&colon),
            netlist_checkpoint_identity(&dotted),
            "case and hierarchy-separator aliases must have one semantic identity"
        );

        let duplicate_history =
            Netlist::parse("last wins\n.INITCOND C1 IC=1 C1 IC=2\nC1 1 0 1u\n.END\n")
                .expect("duplicate target history parses");
        let effective_only = Netlist::parse("last wins\n.INITCOND C1 IC=2\nC1 1 0 1u\n.END\n")
            .expect("effective-only target parses");
        assert_eq!(
            netlist_checkpoint_identity(&duplicate_history),
            netlist_checkpoint_identity(&effective_only),
            "overwritten INITCOND history must not perturb checkpoint identity"
        );

        let no_overlay = Netlist::parse(
            "no-op targets\n\
             .SUBCKT CELL p n\n\
             R1 p n 1\n\
             .ENDS\n\
             L1 1 0 1m IC=10\n\
             L2 2 0 2m\n\
             K1 L1 L2 .5\n\
             X1 1 0 CELL\n\
             .END\n",
        )
        .expect("baseline no-op target deck parses");
        let ignored_overlay = Netlist::parse(
            "no-op targets\n\
             .SUBCKT CELL p n\n\
             R1 p n 1\n\
             .ENDS\n\
             .INITCOND L1 IC=99 K1 IC=.1 X1 IC=7 NO_SUCH_DEVICE IC=8\n\
             L1 1 0 1m IC=10\n\
             L2 2 0 2m\n\
             K1 L1 L2 .5\n\
             X1 1 0 CELL\n\
             .END\n",
        )
        .expect("ignored target deck parses");
        assert_eq!(
            netlist_checkpoint_identity(&no_overlay),
            netlist_checkpoint_identity(&ignored_overlay),
            "X/K/L and unmatched INITCOND targets are semantic no-ops"
        );

        let mut external = first.clone();
        external.device_initial_conditions =
            Some(crate::netlist::DeviceInitialConditionDirective {
                origin: crate::netlist::NetlistSourceLocation::in_memory(2),
                source: crate::netlist::DeviceInitialConditionSource::File {
                    requested_path: "initcond.dat".to_string(),
                    resolved_path: Some(std::path::PathBuf::from("initcond.dat")),
                    content_identity: Some("a".repeat(64)),
                },
                entries: first
                    .device_initial_conditions
                    .as_ref()
                    .expect("inline directive exists")
                    .entries
                    .clone(),
            });
        let mut changed_content = external.clone();
        let Some(crate::netlist::DeviceInitialConditionDirective {
            source:
                crate::netlist::DeviceInitialConditionSource::File {
                    content_identity, ..
                },
            ..
        }) = changed_content.device_initial_conditions.as_mut()
        else {
            panic!("external directive retained");
        };
        *content_identity = Some("b".repeat(64));
        assert_eq!(
            netlist_checkpoint_identity(&external),
            netlist_checkpoint_identity(&changed_content),
            "raw external content identity is provenance, not simulation semantics"
        );
        changed_content
            .device_initial_conditions
            .as_mut()
            .expect("external directive retained")
            .entries[0]
            .values[0] = 2.0;
        assert_ne!(
            netlist_checkpoint_identity(&external),
            netlist_checkpoint_identity(&changed_content),
            "external content that changes the effective IC value must change identity"
        );
    }

    #[test]
    fn replaceground_false_is_default_identity_while_true_is_semantic() {
        let base = "checkpoint replaceground\nR1 out 0 1k\n.print dc V(out)\n.end\n";
        let explicit_false = "checkpoint replaceground\nR1 out 0 1k\n.print dc V(out)\n.end\n.PREPROCESS REPLACEGROUND FALSE\n";
        let enabled = "checkpoint replaceground\nR1 out 0 1k\n.print dc V(out)\n.end\n.PREPROCESS REPLACEGROUND TRUE\n";
        let base = Netlist::parse(base).expect("base deck parses");
        let explicit_false = Netlist::parse(explicit_false).expect("FALSE deck parses");
        let enabled = Netlist::parse(enabled).expect("TRUE deck parses");

        assert_eq!(
            netlist_checkpoint_identity(&base),
            netlist_checkpoint_identity(&explicit_false),
            "explicit FALSE is the omitted semantic default"
        );
        assert_ne!(
            netlist_checkpoint_identity(&base),
            netlist_checkpoint_identity(&enabled),
            "enabled REPLACEGROUND changes checkpoint semantic identity"
        );
    }

    #[test]
    fn netlist_identity_is_collision_resistant_and_legacy_v7_resume_fails_closed() {
        let first = Netlist::parse("identity bench\nr1 1 0 1k\n.end\n").expect("first deck parses");
        let second =
            Netlist::parse("identity bench\nr1 1 0 2k\n.end\n").expect("second deck parses");
        let first_identity = netlist_checkpoint_identity(&first).expect("source identity");
        let second_identity = netlist_checkpoint_identity(&second).expect("source identity");
        assert_eq!(first_identity.len(), 64);
        assert_ne!(
            first_identity, second_identity,
            "same-shape semantic source changes must alter checkpoint identity"
        );

        let mut checkpoint = sample();
        checkpoint.netlist_fingerprint = netlist_fingerprint(&first);
        checkpoint.netlist_identity = Some(first_identity);
        checkpoint
            .validate_for(&first)
            .expect("matching cryptographic netlist identity validates");
        let err = checkpoint
            .validate_for(&second)
            .expect_err("changed canonical source must be refused");
        assert!(err.contains("different netlist"), "unexpected error: {err}");

        let legacy = TransientCheckpoint::from_text(&legacy_text(&checkpoint, 7))
            .expect("version-7 checkpoint still parses");
        let err = legacy
            .validate_for(&first)
            .expect_err("legacy FNV-only identity cannot authorize state restore");
        assert!(
            err.contains("collision-resistant netlist identity"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn netlist_identity_parser_requires_lowercase_blake3_hex() {
        let text = sample().to_text();
        let valid = "fedcba9876543210".repeat(4);
        for invalid in ["abc".to_string(), "A".repeat(64), "g".repeat(64)] {
            let err = TransientCheckpoint::from_text(&text.replacen(&valid, &invalid, 1))
                .expect_err("malformed netlist identity must fail during parsing");
            assert!(
                err.contains("malformed netlist identity"),
                "unexpected error for identity {invalid}: {err}"
            );
        }
    }

    #[test]
    fn simulation_identity_parser_requires_lowercase_blake3_hex() {
        let text = sample().to_text();
        let valid = "abcdef0123456789".repeat(4);
        for invalid in ["abc".to_string(), "A".repeat(64), "g".repeat(64)] {
            let err = TransientCheckpoint::from_text(&text.replacen(&valid, &invalid, 1))
                .expect_err("malformed simulation identity must fail during parsing");
            assert!(
                err.contains("malformed simulation identity"),
                "unexpected error for identity {invalid}: {err}"
            );
        }
    }

    #[test]
    fn elaborated_include_and_public_ast_mutations_change_identity() {
        let unique = format!(
            "rspice-checkpoint-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let directory = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&directory).expect("create checkpoint test directory");
        let root = directory.join("root.cir");
        let include = directory.join("part.inc");
        let source = "include identity\n.include \"part.inc\"\n.end\n";
        std::fs::write(&include, "r1 1 0 1k\n").expect("write first include");
        let first = Netlist::parse_with_path(source, &root).expect("first include parses");
        std::fs::write(&include, "r1 1 0 2k\n").expect("write changed include");
        let second = Netlist::parse_with_path(source, &root).expect("changed include parses");
        std::fs::remove_dir_all(&directory).expect("remove checkpoint test directory");

        assert_ne!(
            netlist_checkpoint_identity(&first),
            netlist_checkpoint_identity(&second),
            "same root text with changed elaborated include semantics must be rejected"
        );

        let mut mutated = first.clone();
        let ElementKind::Resistor { value, .. } = &mut mutated.elements[0].kind else {
            panic!("included element is a resistor");
        };
        *value = 3_000.0;
        assert_ne!(
            netlist_checkpoint_identity(&first),
            netlist_checkpoint_identity(&mutated),
            "public post-parse semantic mutations must alter checkpoint identity"
        );
    }

    #[test]
    fn same_path_external_waveform_content_changes_identity() {
        let unique = format!(
            "rspice-checkpoint-pwl-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let directory = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&directory).expect("create PWL checkpoint test directory");
        let root = directory.join("root.cir");
        let waveform = directory.join("wave.csv");
        let source = "pwl dependency identity\nv1 1 0 pwl file=\"wave.csv\"\nr1 1 0 1k\n.end\n";
        std::fs::write(&waveform, "0,0\n1e-9,1\n").expect("write first waveform");
        let netlist = Netlist::parse_with_path(source, &root).expect("PWL deck parses");
        let first = netlist_checkpoint_identity(&netlist);
        std::fs::write(&waveform, "0,0\n1e-9,2\n").expect("replace waveform in place");
        let second = netlist_checkpoint_identity(&netlist);
        std::fs::remove_dir_all(&directory).expect("remove PWL checkpoint test directory");

        assert_ne!(
            first, second,
            "same-path external waveform changes must invalidate checkpoint provenance"
        );
    }

    #[test]
    fn native_waveform_provenance_is_not_masked_by_xspice_virtual_files() {
        let unique = format!(
            "rspice-checkpoint-native-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let directory = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&directory).expect("create native dependency test directory");
        let root = directory.join("root.cir");
        let waveform = directory.join("wave.csv");
        let waveform_key = waveform.to_string_lossy().into_owned();
        std::fs::write(&waveform, "0,0\n1e-9,1\n").expect("write native waveform");
        crate::xspice::register_data_file(&waveform_key, "0,0\n1e-9,99\n")
            .expect("register colliding virtual data file");
        let netlist = Netlist::parse_with_path(
            "native dependency identity\nv1 1 0 pwl file=\"wave.csv\"\nr1 1 0 1k\n.end\n",
            &root,
        )
        .expect("PWL deck parses");
        let first = netlist_checkpoint_identity(&netlist);
        std::fs::write(&waveform, "0,0\n1e-9,2\n").expect("replace native waveform");
        let second = netlist_checkpoint_identity(&netlist);
        crate::xspice::unregister_data_file(&waveform_key)
            .expect("unregister colliding virtual data file");
        std::fs::remove_dir_all(&directory).expect("remove native dependency test directory");
        assert_ne!(
            first, second,
            "PWL provenance must follow its native loader"
        );
    }

    #[test]
    fn prepared_behavioral_file_lookup_content_changes_identity() {
        let unique = format!(
            "rspice-checkpoint-table-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let directory = std::env::temp_dir().join(unique);
        std::fs::create_dir_all(&directory).expect("create table dependency test directory");
        let root = directory.join("root.cir");
        let table = directory.join("wave.dat");
        std::fs::write(&table, "0 0\n1e-9 1\n").expect("write behavioral table");
        let mut netlist = Netlist::parse_with_path(
            "prepared dependency identity\nb1 1 0 v=lookup\nr1 1 0 1k\n.end\n",
            &root,
        )
        .expect("behavioral deck parses");
        netlist
            .params
            .define_global_expression("lookup", "tablefile(\"wave.dat\")", None);
        let first = netlist_checkpoint_identity(&netlist);
        std::fs::write(&table, "0 0\n1e-9 2\n").expect("replace behavioral table");
        let second = netlist_checkpoint_identity(&netlist);
        std::fs::remove_dir_all(&directory).expect("remove table dependency test directory");
        assert_ne!(
            first, second,
            "prepared parameter-expanded file lookups must bind file contents"
        );
    }

    #[test]
    fn semantic_identity_does_not_advance_live_statistical_stream() {
        let deck = "statistical identity purity\n\
                    .subckt sampled a b\n\
                    r1 a b {aunif(1000,100)}\n\
                    .ends sampled\n\
                    x1 1 0 sampled\n\
                    .end\n";
        let first = Netlist::parse(deck).expect("first statistical deck parses");
        let control = Netlist::parse(deck).expect("control statistical deck parses");
        let _ = netlist_checkpoint_identity(&first).expect("identity computes");
        assert_eq!(
            first.params.random().next_uniform().to_bits(),
            control.params.random().next_uniform().to_bits(),
            "checkpoint identity calculation must not consume the live deck RNG"
        );
    }

    #[test]
    fn resolved_temperature_dialect_and_model_routing_are_bound() {
        let netlist = Netlist::parse("config identity\nr1 1 0 1k\n.end\n").unwrap();
        let base = SimulationConfig::default();
        let mut checkpoint = sample();
        checkpoint.netlist_fingerprint = netlist_fingerprint(&netlist);
        checkpoint.netlist_identity = netlist_checkpoint_identity(&netlist);
        checkpoint.simulation_identity = Some(simulation_checkpoint_identity(&base));
        checkpoint
            .validate_for_with_config(&netlist, &base)
            .expect("matching resolved config validates");

        let mut changed_temperature = base.clone();
        changed_temperature.temperature += 1.0;
        let error = checkpoint
            .validate_for_with_config(&netlist, &changed_temperature)
            .expect_err("temperature mismatch must reject state");
        assert!(error.contains("simulation configuration"));

        let changed_dialect = base
            .clone()
            .with_spice_dialect(crate::engine::SpiceDialect::Xyce);
        checkpoint
            .validate_for_with_config(&netlist, &changed_dialect)
            .expect_err("dialect mismatch must reject state");

        let mut changed_model = base;
        changed_model.jfet_level2_model = crate::engine::JfetLevel2Model::XyceModifiedShockley;
        checkpoint
            .validate_for_with_config(&netlist, &changed_model)
            .expect_err("resolved model-routing mismatch must reject state");
    }

    #[test]
    fn legacy_v8_configuration_identity_fails_closed() {
        let netlist = Netlist::parse("legacy config\nr1 1 0 1k\n.end\n").unwrap();
        let mut current = sample();
        current.netlist_fingerprint = netlist_fingerprint(&netlist);
        current.netlist_identity = netlist_checkpoint_identity(&netlist);
        let legacy = TransientCheckpoint::from_text(&legacy_text(&current, 8))
            .expect("version-8 checkpoint parses");
        let error = legacy
            .validate_for_with_config(&netlist, &SimulationConfig::default())
            .expect_err("v8 has no resolved configuration identity");
        assert!(error.contains("simulation configuration identity"));
    }

    #[test]
    fn version_one_checkpoint_files_still_load_without_xspice_state() {
        let version_one =
            legacy_text(&sample(), 1).replace("xspice 0\nxspice_blockers 0\nxspice_states 0\n", "");
        let restored = TransientCheckpoint::from_text(&version_one)
            .expect("v1 checkpoint without XSPICE section still loads");
        assert!(restored.xspice_instances.is_empty());
        assert!(restored.xspice_resume_blockers.is_empty());
        assert!(restored.xspice_instance_states.is_empty());
        assert!(!restored.lte_reference_history_available);
    }

    #[test]
    fn xspice_blockers_round_trip_and_legacy_v2_refuses_resume() {
        let mut original = sample();
        original.xspice_instances = vec!["a1(gain)".to_string()];
        original.xspice_resume_blockers = vec!["a1(gain): model owns pending state".to_string()];
        let restored = TransientCheckpoint::from_text(&original.to_text()).unwrap();
        assert_eq!(original, restored);

        let version_two = legacy_text(&original, 2).replace(
            "xspice_blockers 1\na1(gain): model owns pending state\nxspice_states 0\n",
            "",
        );
        let restored = TransientCheckpoint::from_text(&version_two)
            .expect("v2 checkpoint with XSPICE instance list still loads");
        assert_eq!(restored.xspice_instances, vec!["a1(gain)"]);
        assert!(
            restored.xspice_resume_blockers[0].contains("legacy checkpoint"),
            "legacy v2 checkpoints must remain blocked, got {:?}",
            restored.xspice_resume_blockers
        );
    }

    #[test]
    fn xspice_instance_states_round_trip_and_v3_loads_without_state_section() {
        let mut original = sample();
        original.xspice_instances = vec!["a1(int)".to_string()];
        original.xspice_instance_states = vec![XspiceInstanceCheckpoint {
            name: "a1".to_string(),
            model: "int".to_string(),
            context: CmContextCheckpoint {
                time: 1.25,
                time_prev: 1.0,
                state: vec![1.0, -0.0],
                state_prev: vec![0.5, f64::MIN_POSITIVE],
                int_state: vec![42, -7],
            },
        }];
        let restored = TransientCheckpoint::from_text(&original.to_text()).unwrap();
        assert_eq!(original, restored);

        let version_three = legacy_text(&sample(), 3).replace("xspice_states 0\n", "");
        let restored = TransientCheckpoint::from_text(&version_three)
            .expect("v3 checkpoint without serialized XSPICE state still loads");
        assert!(restored.xspice_instance_states.is_empty());

        let version_four = legacy_text(&original, 4).replace("context_time 2\n1.25\n1\n", "");
        let restored = TransientCheckpoint::from_text(&version_four)
            .expect("v4 XSPICE state checkpoint without context times still loads");
        assert_eq!(restored.xspice_instance_states[0].context.time, 0.0);
        assert_eq!(restored.xspice_instance_states[0].context.time_prev, 0.0);
    }

    #[test]
    fn malformed_input_fails_with_a_clear_message() {
        assert!(TransientCheckpoint::from_text("").is_err());
        assert!(
            TransientCheckpoint::from_text("RSPICE-CHECKPOINT 999\nfingerprint 0x0\ntime 0\n")
                .unwrap_err()
                .contains("version")
        );
        // Cut mid-file so a whole section is missing rows — trimming only
        // trailing digits would still be a syntactically valid file.
        let text = sample().to_text();
        let truncated = &text[..text.len() / 2];
        assert!(TransientCheckpoint::from_text(truncated).is_err());
    }

    #[test]
    fn declared_section_counts_are_bounded_before_allocation() {
        let count = usize::MAX;

        let text = format!("state {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_value_vector(&mut lines, "state")
            .expect_err("oversized floating-point vectors must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");

        let text = format!("solution {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_value_section(&mut lines, "solution", 1)
            .expect_err("oversized table sections must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");

        let text = format!("int_state {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_i64_vector(&mut lines, "int_state")
            .expect_err("oversized integer vectors must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");

        let text = format!("xspice {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_nonempty_line_vector(&mut lines, "xspice")
            .expect_err("oversized string sections must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");

        let text = format!("xspice_states {count}\n");
        let mut lines = CheckpointLines::new(&text);
        let err = read_xspice_instance_states(&mut lines, FORMAT_VERSION)
            .expect_err("oversized nested XSPICE sections must fail closed");
        assert!(err.contains("rows remain"), "unexpected error: {err}");
    }

    #[test]
    fn malformed_input_rejects_extra_row_fields_and_trailing_data() {
        let text = sample().to_text();
        let extra_row_field = text.replacen("0.5\n", "0.5 99\n", 1);
        let err = TransientCheckpoint::from_text(&extra_row_field)
            .expect_err("extra checkpoint row fields must be rejected");
        assert!(
            err.contains("extra field"),
            "expected extra-field diagnostic, got {err}"
        );

        let trailing = format!("{text}unexpected trailer\n");
        let err = TransientCheckpoint::from_text(&trailing)
            .expect_err("trailing checkpoint content must be rejected");
        assert!(
            err.contains("trailing content"),
            "expected trailing-content diagnostic, got {err}"
        );
    }
}
