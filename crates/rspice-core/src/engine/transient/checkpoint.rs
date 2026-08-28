//! Transient checkpoint/restore.
//!
//! A checkpoint captures the integrator state at an accepted time point:
//! the full MNA solution plus the capacitor and inductor companion-model
//! histories. Restoring injects that state into a freshly built circuit and
//! continues integration from the checkpoint time with absolute-time source
//! evaluation — the same numerical regime as a breakpoint restart, which
//! the integrator already performs at every source discontinuity.
//!
//! Scope, stated precisely: accepted linear-reactive histories, ordinary
//! lossless scalar transmission-line delay histories, generated Verilog-A
//! `ddt`/`idt` histories and limiter anchors, and XSPICE model-owned checkpoint
//! state are captured bit-exactly. Continuation deliberately takes one
//! order-one breakpoint-restart step before higher-order integration resumes.
//! Distributed LTRA/TXL and coupled-line convolution runtimes fail closed until
//! their complete native state has a versioned checkpoint contract.
//!
//! The on-disk format is a versioned, line-oriented text format using
//! Rust's shortest-round-trip float formatting, so save/load reproduces
//! every `f64` bit-exactly with no serialization dependencies (core stays
//! lean for the wasm build).

use crate::Value;
use crate::circuit::CircuitData;
use crate::device::veriloga_builtins::{
    GENERATED_PERSISTENT_STATE_VERSION, GeneratedVerilogAInstanceCheckpoint,
    GeneratedVerilogAPersistentState,
};
use crate::device::{TransmissionLine, TransmissionLineCheckpoint};
use crate::engine::SimulationConfig;
use crate::expr::{Expr, Function, parse_expression_strict};
use crate::netlist::expr::prepare_behavioral_expression;
use crate::netlist::{
    Element, ElementKind, Netlist, ParamContext, SourceSpec, SubcircuitDef,
    flatten_netlist_with_models,
};
use crate::numerics::integration::LteEstimator;
use crate::numerics::integration::TransientLteReference;
use crate::xspice::{CmContextCheckpoint, XspiceInstanceCheckpoint};
use std::io::Read;

use super::TransientStartupMode;

/// Format version written to and required from checkpoint files.
///
/// Version 14 adds validated scalar lossless transmission-line history,
/// pending dynamic line arrivals, the per-run maximum-step contract, and a
/// collision-resistant authored-restart compatibility identity. Earlier files
/// remain readable, but fail closed for these capabilities because inventing
/// delayed waves or authorizing a changed trajectory would be unsafe.
const FORMAT_VERSION: u32 = 14;

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
    /// Collision-resistant trajectory identity for authored restart decks.
    /// Unlike `netlist_identity`, this deliberately excludes only the
    /// transient stop horizon and `.OPTIONS RESTART` control-plane metadata.
    /// Legacy files have no safe compatible identity and fail closed.
    restart_identity: Option<String>,
    /// Identity of the resolved, state-affecting simulation configuration.
    /// Kept optional solely so legacy checkpoint files can be parsed and
    /// rejected with a precise diagnostic.
    simulation_identity: Option<String>,
    /// Startup contract of the selected `.TRAN` analysis. Optional only so
    /// older files can be parsed and rejected with a precise resume error.
    startup_mode: Option<TransientStartupMode>,
    /// Per-call transient maximum-step bound. This is separate from the
    /// resolved configuration identity because transient APIs accept an
    /// explicit cap that materially controls the integration trajectory.
    integration_max_step: Option<Value>,
    /// Dynamically discovered transmission-line arrivals that had not yet
    /// occurred at `time`. These are distinct from authored/source
    /// breakpoints: they arise from accepted wave changes and cannot always be
    /// reconstructed from the compacted delay history after a restart.
    pending_tline_arrivals: Vec<Value>,
    /// Total unique dynamic line arrivals admitted during the trajectory.
    /// Preserving this counter keeps the resource cap identical across a
    /// restart seam rather than granting each segment a fresh allowance.
    dynamic_tline_breakpoints_added: usize,

    cap_v_prev: Vec<Value>,
    cap_v_prev_prev: Vec<Value>,
    cap_v_prev_prev_prev: Vec<Value>,
    cap_i_prev: Vec<Value>,
    cap_i_eq: Vec<Value>,
    ind_i_prev: Vec<Value>,
    ind_i_prev_prev: Vec<Value>,
    /// Empty when `inductor_flux_history_available` is false: files older
    /// than format 13 never recorded the third accepted inductor current, and
    /// resume must not invent it.
    ind_i_prev_prev_prev: Vec<Value>,
    ind_v_prev: Vec<Value>,
    inductor_flux_history_available: bool,
    xyce_memristor_resistance_stores: Vec<Value>,
    generic_switch_stores: Vec<[Value; 4]>,
    tline_state_available: bool,
    tline_resume_blockers: Vec<String>,
    tline_states: Vec<TransmissionLineCheckpoint>,
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
    if path.contains("://") {
        return std::path::PathBuf::from(path);
    }
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
    max_bytes: Option<usize>,
) {
    let resolved = resolved_dependency_path(path, source_path);
    let resolved_text = resolved.to_string_lossy();
    hash_field(hasher, "dependency_path", &resolved_text);
    if xspice_virtual_aware
        && let Some(contents) = crate::xspice::checkpoint_virtual_data_file_contents(&resolved_text)
    {
        if max_bytes.is_some_and(|limit| contents.len() > limit) {
            hash_field(hasher, "dependency_kind", "oversized_virtual");
            hash_field(hasher, "dependency_length", contents.len());
            hash_field(hasher, "dependency_limit", max_bytes);
            return;
        }
        hash_field(hasher, "dependency_kind", "virtual");
        hasher.update(&(contents.len() as u64).to_le_bytes());
        hasher.update(blake3::hash(contents.as_bytes()).as_bytes());
    } else {
        match std::fs::File::open(&resolved) {
            Ok(mut file) => {
                let metadata_length = file.metadata().ok().map(|metadata| metadata.len());
                if let (Some(limit), Some(length)) = (max_bytes, metadata_length)
                    && length > limit as u64
                {
                    hash_field(hasher, "dependency_kind", "oversized_native");
                    hash_field(hasher, "dependency_length", length);
                    hash_field(hasher, "dependency_limit", limit);
                    return;
                }

                let mut content_hasher = blake3::Hasher::new();
                let mut length = 0usize;
                let mut buffer = [0u8; 64 * 1024];
                loop {
                    match file.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(read) => {
                            length = length.saturating_add(read);
                            if max_bytes.is_some_and(|limit| length > limit) {
                                hash_field(hasher, "dependency_kind", "oversized_native");
                                hash_field(hasher, "dependency_length_at_least", length);
                                hash_field(hasher, "dependency_limit", max_bytes);
                                return;
                            }
                            content_hasher.update(&buffer[..read]);
                        }
                        Err(error) => {
                            hash_field(hasher, "dependency_kind", "unavailable");
                            hash_field(hasher, "dependency_error_kind", error.kind());
                            return;
                        }
                    }
                }
                hash_field(hasher, "dependency_kind", "native");
                hasher.update(&(length as u64).to_le_bytes());
                hasher.update(content_hasher.finalize().as_bytes());
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
        SourceSpec::PwlFile { path, .. } => hash_dependency(hasher, path, source_path, false, None),
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
                    hash_dependency(hasher, path, source_path, false, None);
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
        || matches!(name.as_str(), "fxpdata" | "fxmdata")
        || name.ends_with("file")
        || name.ends_with("_file")
        || name.ends_with("path")
}

fn model_string_is_pem_table_dependency(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "fxpdata" | "fxmdata"
    )
}

fn model_uses_xyce_pem_tables(netlist: &Netlist, model: &crate::netlist::ModelDef) -> bool {
    let temperature_kelvin =
        crate::constants::celsius_to_kelvin(netlist.options.temp.unwrap_or(27.0));
    matches!(
        crate::engine::builder::resolve_native_xyce_memristor_family(
            netlist,
            model,
            "<checkpoint identity>",
            &model.name,
            temperature_kelvin,
        ),
        Ok(crate::engine::builder::NativeXyceMemristorFamily::Pem)
    )
}

fn xyce_pem_default_table_path(netlist: &Netlist, default: &str) -> std::path::PathBuf {
    netlist
        .source_path
        .as_deref()
        .and_then(std::path::Path::parent)
        .map_or_else(
            || std::path::PathBuf::from(default),
            |base| base.join(default),
        )
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
                let dependency_source_path = if model_string_is_pem_table_dependency(name) {
                    // PEM paths have already been normalized by the parser and
                    // are consumed verbatim by the device builder.
                    None
                } else {
                    source_path
                };
                let max_bytes = model_string_is_pem_table_dependency(name)
                    .then_some(crate::device::XYCE_PEM_MAX_TABLE_BYTES);
                hash_dependency(
                    hasher,
                    path,
                    dependency_source_path,
                    virtual_aware,
                    max_bytes,
                );
            }
        }

        if model_uses_xyce_pem_tables(&isolated_netlist, model) {
            for (parameter, default) in [
                (
                    "FXPDATA",
                    crate::device::XYCE_PEM_DEFAULT_POSITIVE_TABLE_FILE,
                ),
                (
                    "FXMDATA",
                    crate::device::XYCE_PEM_DEFAULT_NEGATIVE_TABLE_FILE,
                ),
            ] {
                if model
                    .string_params
                    .iter()
                    .any(|(name, _)| name.eq_ignore_ascii_case(parameter))
                {
                    continue;
                }
                let path = xyce_pem_default_table_path(netlist, default);
                hash_dependency(
                    hasher,
                    &path.to_string_lossy(),
                    None,
                    true,
                    Some(crate::device::XYCE_PEM_MAX_TABLE_BYTES),
                );
            }
        }
    }
    for include in &netlist.veriloga_includes {
        hash_dependency(
            hasher,
            &include.file_path.to_string_lossy(),
            source_path,
            false,
            None,
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

fn semantic_netlist_identity(netlist: &Netlist, domain: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain);
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
                    request.analysis,
                    request.name.as_deref(),
                    request.print_delimiter.as_ref(),
                    request.operands.as_slice(),
                    request.operand_kinds.as_slice(),
                    request.expressions.as_slice(),
                    request.dependencies.as_slice(),
                )
            })
            .collect::<Vec<_>>(),
    );
    let mut checkpoint_options = netlist.options.clone();
    for schedule in [
        &mut checkpoint_options.output_time_points,
        &mut checkpoint_options.timeint_breakpoints,
    ] {
        schedule.retain(|time| time.is_finite() && *time >= 0.0);
        schedule.iter_mut().for_each(|time| {
            if *time == 0.0 {
                *time = 0.0;
            }
        });
        schedule.sort_by(Value::total_cmp);
        schedule.dedup_by(|left, right| {
            (*left - *right).abs() <= crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE
        });
    }
    hash_field(&mut hasher, "options", &checkpoint_options);
    hash_field(&mut hasher, "veriloga_includes", &netlist.veriloga_includes);
    hash_field(&mut hasher, "spef_includes", &netlist.spef_includes);
    hash_effective_device_initial_condition_overlay(&mut hasher, netlist);
    hash_external_dependencies(&mut hasher, netlist);
    hasher.finalize().to_hex().to_string()
}

/// Collision-resistant identity of the fully elaborated semantic netlist.
/// Source paths, diagnostics, and original source spelling are excluded;
/// expanded include/SPEF content and public post-parse AST edits are included.
pub(crate) fn netlist_checkpoint_identity(netlist: &Netlist) -> Option<String> {
    Some(semantic_netlist_identity(
        netlist,
        b"rspice-transient-elaborated-netlist-v6\0",
    ))
}

/// Collision-resistant trajectory identity used by authored checkpoint/restart
/// decks whose run horizon and restart I/O metadata necessarily differ.
///
/// Only the stop horizon of `.TRAN` commands and the complete
/// `.OPTIONS RESTART` control plane are normalized. The transient print step,
/// start time, maximum step, UIC contract, circuit/model/source semantics,
/// output requests, external dependency content, and every other typed option
/// remain part of the identity through [`semantic_netlist_identity`].
pub(crate) fn restart_checkpoint_identity(netlist: &Netlist) -> Option<String> {
    let mut normalized = netlist.clone();
    for analysis in &mut normalized.analyses {
        if let crate::netlist::AnalysisCommand::Tran { stop, .. } = analysis {
            *stop = 0.0;
        }
    }
    normalized.options.restart = None;
    Some(semantic_netlist_identity(
        &normalized,
        b"rspice-transient-restart-compatible-netlist-v1\0",
    ))
}

pub(crate) fn simulation_checkpoint_identity(config: &SimulationConfig) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"rspice-transient-resolved-config-v4\0");
    hash_field(&mut hasher, "temperature", config.temperature.to_bits());
    hash_field(&mut hasher, "ramptime", config.ramptime.to_bits());
    hash_field(&mut hasher, "digital_delay_type", config.digital_delay_type);
    hash_field(&mut hasher, "integration_method", config.integration_method);
    hash_field(&mut hasher, "spice_dialect", config.spice_dialect);
    hash_field(
        &mut hasher,
        "xyce_tra_interpolation",
        config.xyce_tra_interpolation,
    );
    hash_field(
        &mut hasher,
        "jfet_level2_model",
        config.resolved_jfet_level2_model(),
    );
    hash_field(&mut hasher, "b3soi_gmin_scaling", config.b3soi_gmin_scaling);
    hash_field(
        &mut hasher,
        "device_voltage_limiting",
        config.device_voltage_limiting,
    );
    hash_field(&mut hasher, "rshunt", config.rshunt.map(f64::to_bits));
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
        "transient_timeint_max_timestep",
        config.transient_timeint_max_timestep.map(f64::to_bits),
    );
    hash_field(
        &mut hasher,
        "transient_use_device_max_timestep",
        config.transient_use_device_max_timestep,
    );
    hash_field(
        &mut hasher,
        "transient_error_control",
        config.transient_error_control,
    );
    hash_field(
        &mut hasher,
        "transient_min_steps_between_breakpoints",
        config.transient_min_steps_between_breakpoints,
    );
    hash_field(
        &mut hasher,
        "transient_timeint_nlmin",
        config.transient_timeint_nlmin,
    );
    hash_field(
        &mut hasher,
        "transient_timeint_nlmax",
        config.transient_timeint_nlmax,
    );
    hash_field(
        &mut hasher,
        "transient_timeint_min_order",
        config.transient_timeint_min_order,
    );
    hash_field(
        &mut hasher,
        "transient_timeint_max_order",
        config.transient_timeint_max_order,
    );
    hash_field(
        &mut hasher,
        "transient_timesteps_reversal",
        config.transient_timesteps_reversal,
    );
    hash_field(
        &mut hasher,
        "transient_nonlinear_reltol",
        config.transient_nonlinear_reltol.map(f64::to_bits),
    );
    hash_field(
        &mut hasher,
        "transient_nonlinear_abstol",
        config.transient_nonlinear_abstol.map(f64::to_bits),
    );
    hash_field(
        &mut hasher,
        "transient_nonlinear_deltaxtol",
        config.transient_nonlinear_deltaxtol.map(f64::to_bits),
    );
    hash_field(
        &mut hasher,
        "transient_nonlinear_rhstol",
        config.transient_nonlinear_rhstol.map(f64::to_bits),
    );
    hash_field(
        &mut hasher,
        "transient_nonlinear_max_iterations",
        config.transient_nonlinear_max_iterations,
    );
    hash_field(
        &mut hasher,
        "transient_nonlinear_nox",
        config.transient_nonlinear_nox,
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

fn read_tline_states(
    lines: &mut CheckpointLines<'_>,
) -> Result<Vec<TransmissionLineCheckpoint>, String> {
    let header = lines
        .next()
        .ok_or_else(|| "missing 'tline_states' section".to_string())?;
    let count = parse_count_header(header, "tline_states")?;
    let mut states = allocate_checkpoint_rows(lines, count, "tline_states")?;
    for row in 0..count {
        let line = lines
            .next()
            .ok_or_else(|| format!("'tline_states' truncated at row {row}"))?;
        let mut fields = line.split_whitespace();
        if fields.next() != Some("tline_state") {
            return Err(format!("malformed 'tline_state' header: '{line}'"));
        }
        let name = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its instance name"))?;
        let impedance = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its impedance"))?
            .parse::<Value>()
            .map_err(|_| format!("tline state row {row} has invalid impedance"))?;
        let initialized = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its initialized flag"))
            .and_then(|field| parse_checkpoint_bool(field, &format!("tline state row {row}")))?;
        let current_time = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing current time"))?
            .parse::<Value>()
            .map_err(|_| format!("tline state row {row} has invalid current time"))?;
        let launched_forward = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its forward wave"))?
            .parse::<Value>()
            .map_err(|_| format!("tline state row {row} has invalid forward wave"))?;
        let launched_backward = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its backward wave"))?
            .parse::<Value>()
            .map_err(|_| format!("tline state row {row} has invalid backward wave"))?;
        let initial_present = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its initial-state flag"))
            .and_then(|field| {
                parse_checkpoint_bool(field, &format!("tline state row {row} initial state"))
            })?;
        let sample_count = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its sample count"))?
            .parse::<usize>()
            .map_err(|_| format!("tline state row {row} has an invalid sample count"))?;
        let forward_count = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its forward sample count"))?
            .parse::<usize>()
            .map_err(|_| format!("tline state row {row} has an invalid forward sample count"))?;
        let backward_count = fields
            .next()
            .ok_or_else(|| format!("tline state row {row} is missing its backward sample count"))?
            .parse::<usize>()
            .map_err(|_| format!("tline state row {row} has an invalid backward sample count"))?;
        if let Some(extra) = fields.next() {
            return Err(format!("tline state row {row}: extra field '{extra}'"));
        }

        let read_sample = |lines: &mut CheckpointLines<'_>, label: &str| {
            let sample_line = lines
                .next()
                .ok_or_else(|| format!("'{label}' is missing"))?;
            let mut sample_fields = sample_line.split_whitespace();
            if sample_fields.next() != Some(label) {
                return Err(format!("malformed '{label}' row: '{sample_line}'"));
            }
            let mut sample = [0.0; 5];
            for value in &mut sample {
                let field = sample_fields
                    .next()
                    .ok_or_else(|| format!("'{label}' row is short"))?;
                *value = field
                    .parse::<Value>()
                    .map_err(|_| format!("'{label}' row has invalid value '{field}'"))?;
            }
            if let Some(extra) = sample_fields.next() {
                return Err(format!("'{label}' row has extra field '{extra}'"));
            }
            Ok(sample)
        };

        let initial_state = initial_present
            .then(|| read_sample(lines, "tline_initial"))
            .transpose()?;
        let mut state_history = allocate_checkpoint_rows(lines, sample_count, "tline_samples")?;
        for _ in 0..sample_count {
            state_history.push(read_sample(lines, "tline_sample")?);
        }
        let read_delay_sample = |lines: &mut CheckpointLines<'_>, label: &str| {
            let sample_line = lines
                .next()
                .ok_or_else(|| format!("'{label}' is missing"))?;
            let mut sample_fields = sample_line.split_whitespace();
            if sample_fields.next() != Some(label) {
                return Err(format!("malformed '{label}' row: '{sample_line}'"));
            }
            let mut sample = [0.0; 3];
            for value in &mut sample {
                let field = sample_fields
                    .next()
                    .ok_or_else(|| format!("'{label}' row is short"))?;
                *value = field
                    .parse::<Value>()
                    .map_err(|_| format!("'{label}' row has invalid value '{field}'"))?;
            }
            if let Some(extra) = sample_fields.next() {
                return Err(format!("'{label}' row has extra field '{extra}'"));
            }
            Ok(sample)
        };
        let mut forward_history =
            allocate_checkpoint_rows(lines, forward_count, "tline_forward_samples")?;
        for _ in 0..forward_count {
            forward_history.push(read_delay_sample(lines, "tline_forward")?);
        }
        let mut backward_history =
            allocate_checkpoint_rows(lines, backward_count, "tline_backward_samples")?;
        for _ in 0..backward_count {
            backward_history.push(read_delay_sample(lines, "tline_backward")?);
        }
        states.push(TransmissionLineCheckpoint {
            name: name.to_string(),
            impedance,
            initial_state,
            state_history,
            forward_history,
            backward_history,
            launched_forward,
            launched_backward,
            history_initialized: initialized,
            current_time,
        });
    }
    Ok(states)
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
        if self
            .integration_max_step
            .is_some_and(|max_step| !max_step.is_finite() || max_step <= 0.0)
        {
            return Err(
                "checkpoint integration maximum step must be finite and positive".to_string(),
            );
        }
        let mut previous_arrival = None;
        for &arrival in &self.pending_tline_arrivals {
            if !arrival.is_finite() || arrival <= self.time {
                return Err(format!(
                    "checkpoint pending transmission-line arrival must be finite and later than {:.17e}s, found {arrival:.17e}s",
                    self.time
                ));
            }
            if previous_arrival.is_some_and(|previous| arrival <= previous) {
                return Err(format!(
                    "checkpoint pending transmission-line arrivals must be strictly increasing, found {arrival:.17e}s after {:.17e}s",
                    previous_arrival.expect("checked above")
                ));
            }
            previous_arrival = Some(arrival);
        }
        if self.dynamic_tline_breakpoints_added < self.pending_tline_arrivals.len()
            || self.dynamic_tline_breakpoints_added > super::MAX_DYNAMIC_TLINE_BREAKPOINTS
        {
            return Err(format!(
                "checkpoint dynamic transmission-line breakpoint count {} is inconsistent with {} pending arrivals or exceeds the trajectory cap {}",
                self.dynamic_tline_breakpoints_added,
                self.pending_tline_arrivals.len(),
                super::MAX_DYNAMIC_TLINE_BREAKPOINTS
            ));
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
        if self.restart_identity.as_ref().is_some_and(|identity| {
            identity.len() != 64
                || !identity
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        }) {
            return Err(
                "checkpoint restart identity must be 64 lowercase hexadecimal digits".to_string(),
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
        let expected_flux_history_len = if self.inductor_flux_history_available {
            inductor_len
        } else {
            0
        };
        if self.ind_i_prev_prev_prev.len() != expected_flux_history_len {
            return Err(
                "checkpoint inductor flux history length disagrees with its availability"
                    .to_string(),
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
            .chain(&self.ind_i_prev_prev_prev)
            .chain(&self.ind_v_prev)
            .chain(&self.xyce_memristor_resistance_stores)
            .chain(self.generic_switch_stores.iter().flatten())
            .any(|value| !value.is_finite())
        {
            return Err(
                "checkpoint reactive history and device store values must be finite".to_string(),
            );
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
        if !self.tline_state_available && !self.tline_states.is_empty() {
            return Err(
                "transmission-line checkpoint state is present without availability provenance"
                    .to_string(),
            );
        }
        for (index, state) in self.tline_states.iter().enumerate() {
            TransmissionLine::validate_checkpoint_state(state).map_err(|error| {
                format!("transmission-line checkpoint state {index} is malformed: {error}")
            })?;
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
        circuit: &CircuitData,
        startup_mode: TransientStartupMode,
        lte_estimator: Option<&LteEstimator>,
    ) -> Self {
        Self::capture_with_restart_identity(
            fingerprint,
            netlist_identity,
            None,
            simulation_identity,
            time,
            solution,
            circuit,
            startup_mode,
            None,
            &[],
            0,
            lte_estimator,
        )
    }

    /// Capture transient state together with the authored-restart identity.
    pub(crate) fn capture_with_restart_identity(
        fingerprint: u64,
        netlist_identity: Option<String>,
        restart_identity: Option<String>,
        simulation_identity: String,
        time: Value,
        solution: &[Value],
        circuit: &CircuitData,
        startup_mode: TransientStartupMode,
        integration_max_step: Option<Value>,
        pending_tline_arrivals: &[Value],
        dynamic_tline_breakpoints_added: usize,
        lte_estimator: Option<&LteEstimator>,
    ) -> Self {
        let mut tline_states = Vec::with_capacity(circuit.tlines.len());
        let mut tline_resume_blockers = Vec::new();
        for line in &circuit.tlines {
            match line.checkpoint_state() {
                Ok(state) => tline_states.push(state),
                Err(blocker) => tline_resume_blockers.push(blocker),
            }
        }
        tline_resume_blockers.extend(circuit.coupled_tlines.iter().map(|line| {
            format!(
                "coupled transmission line '{}': convolution history is not checkpointable",
                line.name
            )
        }));
        if !tline_resume_blockers.is_empty() {
            log::warn!(
                "transient checkpoint at t={time:.6e}: resume will be refused because \
                 transmission-line state is incomplete: {}",
                tline_resume_blockers.join("; ")
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
        let mut pending_tline_arrivals = pending_tline_arrivals
            .iter()
            .copied()
            .filter(|arrival| arrival.is_finite() && *arrival > time)
            .collect::<Vec<_>>();
        pending_tline_arrivals.sort_by(Value::total_cmp);
        pending_tline_arrivals.dedup_by(|left, right| left.to_bits() == right.to_bits());

        Self {
            time,
            solution: solution.to_vec(),
            netlist_fingerprint: fingerprint,
            netlist_identity,
            restart_identity,
            simulation_identity: Some(simulation_identity),
            startup_mode: Some(startup_mode),
            integration_max_step,
            pending_tline_arrivals,
            dynamic_tline_breakpoints_added,
            cap_v_prev: circuit.capacitors.v_prev.clone(),
            cap_v_prev_prev: circuit.capacitors.v_prev_prev.clone(),
            cap_v_prev_prev_prev: circuit.capacitors.v_prev_prev_prev.clone(),
            cap_i_prev: circuit.capacitors.i_prev.clone(),
            cap_i_eq: circuit.capacitors.i_eq.clone(),
            ind_i_prev: circuit.inductors.i_prev.clone(),
            ind_i_prev_prev: circuit.inductors.i_prev_prev.clone(),
            ind_i_prev_prev_prev: circuit.inductors.i_prev_prev_prev.clone(),
            ind_v_prev: circuit.inductors.v_prev.clone(),
            inductor_flux_history_available: true,
            xyce_memristor_resistance_stores: circuit
                .xyce_memristors
                .iter()
                .map(|binding| binding.resistance_store)
                .collect(),
            generic_switch_stores: circuit.generic_switch_transient_store_snapshots(),
            tline_state_available: true,
            tline_resume_blockers,
            tline_states,
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
    pub(crate) fn inject(&self, circuit: &mut CircuitData) -> Result<(), String> {
        self.validate_numeric_state()?;

        // Validate identity-bearing device state before ordinal storage
        // shapes. Named instance mismatches are the most specific evidence
        // that a checkpoint belongs to a different elaboration, whereas a
        // later vector-length mismatch may only be a consequence of it.
        circuit.validate_xspice_checkpoint_instance_states(&self.xspice_instance_states)?;
        circuit.validate_generated_veriloga_checkpoint_states(
            &self.generated_veriloga_instance_states,
            self.generated_veriloga_state_available,
        )?;
        if !self.tline_resume_blockers.is_empty() {
            return Err(format!(
                "transient checkpoint resume cannot restore unsupported transmission-line state: {}",
                self.tline_resume_blockers.join("; ")
            ));
        }
        if !self.tline_state_available
            && (!circuit.tlines.is_empty() || !circuit.coupled_tlines.is_empty())
        {
            return Err(
                "legacy transient checkpoint does not contain transmission-line history; re-run the transient from t=0"
                    .to_string(),
            );
        }
        if !circuit.coupled_tlines.is_empty() {
            return Err(
                "transient checkpoint does not contain coupled transmission-line convolution history"
                    .to_string(),
            );
        }
        if self.tline_states.len() != circuit.tlines.len() {
            return Err(format!(
                "checkpoint transmission-line shape mismatch: captured {}, circuit has {}",
                self.tline_states.len(),
                circuit.tlines.len()
            ));
        }
        let mut restored_tlines = circuit.tlines.clone();
        for (line, state) in restored_tlines.iter_mut().zip(&self.tline_states) {
            if line.is_zero_length_pass_through() {
                if state.history_initialized
                    || state.initial_state.is_some()
                    || !state.state_history.is_empty()
                    || !state.forward_history.is_empty()
                    || !state.backward_history.is_empty()
                    || state.current_time.to_bits() != 0.0f64.to_bits()
                {
                    return Err(format!(
                        "checkpoint zero-length transmission line '{}' contains noncanonical dynamic history",
                        line.name
                    ));
                }
            } else if !state.history_initialized
                || state.initial_state.is_none()
                || state.state_history.is_empty()
                || state.forward_history.is_empty()
                || state.backward_history.is_empty()
                || state.current_time.to_bits() != self.time.to_bits()
                || state.state_history.last().map(|sample| sample[0].to_bits())
                    != Some(self.time.to_bits())
            {
                return Err(format!(
                    "checkpoint transmission line '{}' does not contain complete accepted history at {:.17e}s",
                    line.name, self.time
                ));
            }
            line.restore_checkpoint_state(state)?;
        }
        if !self.inductor_flux_history_available && !circuit.inductors.is_empty() {
            return Err(format!(
                "checkpoint predates the inductor flux history (format {FORMAT_VERSION} \
                 records i_prev_prev_prev); it cannot resume a circuit with {} inductor(s) — \
                 re-run the transient from t=0",
                circuit.inductors.len()
            ));
        }

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
                "inductor i_prev_prev_prev",
                self.ind_i_prev_prev_prev.len(),
                circuit.inductors.i_prev_prev_prev.len(),
            ),
            (
                "inductor v_prev",
                self.ind_v_prev.len(),
                circuit.inductors.v_prev.len(),
            ),
            (
                "Xyce memristor resistance store",
                self.xyce_memristor_resistance_stores.len(),
                circuit.xyce_memristors.len(),
            ),
            (
                "generic switch store",
                self.generic_switch_stores.len(),
                circuit.generic_switch_count(),
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
        circuit
            .inductors
            .i_prev_prev_prev
            .copy_from_slice(&self.ind_i_prev_prev_prev);
        circuit.inductors.v_prev.copy_from_slice(&self.ind_v_prev);
        for (binding, &resistance) in circuit
            .xyce_memristors
            .iter_mut()
            .zip(&self.xyce_memristor_resistance_stores)
        {
            binding.resistance_store = resistance;
        }
        circuit.restore_generic_switch_transient_store_snapshots(&self.generic_switch_stores);
        circuit.restore_xspice_checkpoint_instance_states(&self.xspice_instance_states)?;
        circuit.restore_generated_veriloga_checkpoint_states(
            &self.generated_veriloga_instance_states,
            self.generated_veriloga_state_available,
        )?;
        circuit.tlines = restored_tlines;
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
        self.validate_resume_capabilities(netlist)
    }

    /// Validate this checkpoint for an authored `.OPTIONS RESTART FILE` deck.
    ///
    /// Restart decks intentionally change their transient stop horizon and
    /// restart I/O metadata. This validation path therefore uses the dedicated
    /// collision-resistant restart identity and intentionally does not consult
    /// the legacy source-text fingerprint. All physical netlist semantics,
    /// output contracts, external dependencies, and trajectory-affecting
    /// transient controls remain identity-bound.
    pub fn validate_for_restart(&self, netlist: &Netlist) -> Result<(), String> {
        self.validate_numeric_state()?;
        let target_identity = restart_checkpoint_identity(netlist)
            .expect("restart identity is available for every elaborated AST");
        let Some(captured_identity) = self.restart_identity.as_deref() else {
            return Err(
                "legacy transient checkpoint does not contain a collision-resistant restart identity"
                    .to_string(),
            );
        };
        if captured_identity != target_identity {
            return Err(format!(
                "checkpoint was captured from a restart-incompatible netlist (identity {captured_identity}, this deck is {target_identity}); refusing to resume mismatched state"
            ));
        }
        self.validate_resume_capabilities(netlist)
    }

    fn validate_resume_capabilities(&self, netlist: &Netlist) -> Result<(), String> {
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
        self.validate_resolved_config(config)
    }

    pub(crate) fn validate_for_restart_with_config(
        &self,
        netlist: &Netlist,
        config: &SimulationConfig,
    ) -> Result<(), String> {
        self.validate_for_restart(netlist)?;
        self.validate_resolved_config(config)
    }

    fn validate_resolved_config(&self, config: &SimulationConfig) -> Result<(), String> {
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
        if self.startup_mode.is_none() {
            return Err(
                "legacy transient checkpoint does not record whether its selected .TRAN analysis used UIC; refusing to resume an ambiguous trajectory"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(crate) fn validate_integration_max_step(
        &self,
        requested_max_step: Value,
    ) -> Result<(), String> {
        let Some(captured_max_step) = self.integration_max_step else {
            return Err(
                "legacy transient checkpoint does not record its per-run maximum step".to_string(),
            );
        };
        if captured_max_step.to_bits() != requested_max_step.to_bits() {
            return Err(format!(
                "checkpoint maximum step {captured_max_step:.17e}s does not match requested maximum step {requested_max_step:.17e}s"
            ));
        }
        Ok(())
    }

    /// Clone an already-authenticated synthetic time-zero state and bind it to
    /// the maximum-step contract of its first real transient segment.
    ///
    /// HB and PSS construct phase-equivalent transient state before that
    /// future segment has a step bound, so their checkpoints intentionally
    /// carry no `integration_max_step`. Callers must first authenticate the
    /// higher-level continuation artifact and this checkpoint's exact
    /// netlist/configuration identity. Ordinary resume validation is still
    /// required after binding; this helper only makes that exact validation
    /// possible and cannot upgrade an accepted transient checkpoint.
    pub(in crate::engine) fn bind_authenticated_synthetic_origin_max_step(
        &self,
        requested_max_step: Value,
    ) -> Result<Self, String> {
        if self.time.to_bits() != 0.0_f64.to_bits() {
            return Err(format!(
                "synthetic transient-origin checkpoint must be captured at exact t=0, found {:.17e}s",
                self.time
            ));
        }
        if self.integration_max_step.is_some() {
            return Err(
                "synthetic transient-origin checkpoint already records an integration maximum step"
                    .to_string(),
            );
        }
        if !requested_max_step.is_finite() || requested_max_step <= 0.0 {
            return Err(format!(
                "synthetic transient-origin maximum step must be finite and positive, got {requested_max_step:.17e}s"
            ));
        }

        let mut bound = self.clone();
        bound.integration_max_step = Some(requested_max_step);
        bound.validate_integration_max_step(requested_max_step)?;
        Ok(bound)
    }

    /// Future dynamically discovered transmission-line arrivals that must be
    /// reinstated in the breakpoint manager before resumed integration.
    pub(crate) fn pending_tline_arrivals(&self) -> &[Value] {
        &self.pending_tline_arrivals
    }

    pub(crate) fn dynamic_tline_breakpoints_added(&self) -> usize {
        self.dynamic_tline_breakpoints_added
    }

    /// Number of scalar trajectory values retained by this snapshot. This is
    /// used with the transient result budget so a checkpoint schedule cannot
    /// multiply memory without bound even when every individual snapshot is
    /// valid.
    pub(crate) fn retained_value_count(&self) -> usize {
        let mut count = 7_usize
            .saturating_add(self.solution.len())
            .saturating_add(self.cap_v_prev.len())
            .saturating_add(self.cap_v_prev_prev.len())
            .saturating_add(self.cap_v_prev_prev_prev.len())
            .saturating_add(self.cap_i_prev.len())
            .saturating_add(self.cap_i_eq.len())
            .saturating_add(self.ind_i_prev.len())
            .saturating_add(self.ind_i_prev_prev.len())
            .saturating_add(self.ind_i_prev_prev_prev.len())
            .saturating_add(self.ind_v_prev.len())
            .saturating_add(self.xyce_memristor_resistance_stores.len())
            .saturating_add(self.generic_switch_stores.len().saturating_mul(4))
            .saturating_add(self.pending_tline_arrivals.len())
            .saturating_add(self.lte_signal_local_reference.len());

        for state in &self.tline_states {
            count = count
                .saturating_add(5)
                .saturating_add(usize::from(state.initial_state.is_some()).saturating_mul(5))
                .saturating_add(state.state_history.len().saturating_mul(5))
                .saturating_add(state.forward_history.len().saturating_mul(3))
                .saturating_add(state.backward_history.len().saturating_mul(3));
        }
        for instance in &self.xspice_instance_states {
            count = count
                .saturating_add(2)
                .saturating_add(instance.context.state.len())
                .saturating_add(instance.context.state_prev.len())
                .saturating_add(instance.context.int_state.len());
        }
        for instance in &self.generated_veriloga_instance_states {
            let state = &instance.state;
            count = count
                .saturating_add(1)
                .saturating_add(state.ddt_previous.len())
                .saturating_add(state.ddt_older.len())
                .saturating_add(state.ddt_derivative_previous.len())
                .saturating_add(state.ddt_initialized.len())
                .saturating_add(state.idt_previous.len())
                .saturating_add(state.idt_initialized.len())
                .saturating_add(state.limiter_anchor.len())
                .saturating_add(state.limiter_initialized.len());
        }
        count
    }

    /// Startup contract captured for the selected transient analysis.
    ///
    /// Legacy checkpoint formats return `None` and are refused for resume.
    pub fn startup_mode(&self) -> Option<TransientStartupMode> {
        self.startup_mode
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
            "restart_identity {}\n",
            self.restart_identity.as_deref().unwrap_or("none")
        ));
        out.push_str(&format!(
            "simulation_identity {}\n",
            self.simulation_identity.as_deref().unwrap_or("none")
        ));
        let startup_mode = match self.startup_mode {
            Some(TransientStartupMode::OperatingPoint) => "operating-point",
            Some(TransientStartupMode::Uic) => "uic",
            None => "unknown",
        };
        out.push_str(&format!("startup_mode {startup_mode}\n"));
        out.push_str(&format!("time {}\n", self.time));
        out.push_str(&format!(
            "integration_max_step {}\n",
            self.integration_max_step
                .map_or_else(|| "none".to_string(), |value| value.to_string())
        ));
        out.push_str(&format!(
            "pending_tline_arrivals {}",
            self.pending_tline_arrivals.len()
        ));
        for arrival in &self.pending_tline_arrivals {
            out.push(' ');
            out.push_str(&arrival.to_string());
        }
        out.push('\n');
        out.push_str(&format!(
            "dynamic_tline_breakpoints_added {}\n",
            self.dynamic_tline_breakpoints_added
        ));

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
        out.push_str(&format!(
            "inductor_flux_history_available {}\n",
            u8::from(self.inductor_flux_history_available)
        ));
        if self.inductor_flux_history_available {
            section(
                &mut out,
                "inductors",
                &[
                    &self.ind_i_prev,
                    &self.ind_i_prev_prev,
                    &self.ind_i_prev_prev_prev,
                    &self.ind_v_prev,
                ],
            );
        } else {
            section(
                &mut out,
                "inductors",
                &[&self.ind_i_prev, &self.ind_i_prev_prev, &self.ind_v_prev],
            );
        }
        write_value_vector(
            &mut out,
            "xyce_memristor_resistance_stores",
            &self.xyce_memristor_resistance_stores,
        );
        out.push_str(&format!(
            "generic_switch_stores {}\n",
            self.generic_switch_stores.len()
        ));
        for store in &self.generic_switch_stores {
            out.push_str(&format!(
                "{} {} {} {}\n",
                store[0], store[1], store[2], store[3]
            ));
        }
        out.push_str(&format!(
            "tline_state_available {}\n",
            u8::from(self.tline_state_available)
        ));
        out.push_str(&format!(
            "tline_blockers {}\n",
            self.tline_resume_blockers.len()
        ));
        for blocker in &self.tline_resume_blockers {
            out.push_str(blocker);
            out.push('\n');
        }
        out.push_str(&format!("tline_states {}\n", self.tline_states.len()));
        for state in &self.tline_states {
            out.push_str(&format!(
                "tline_state {} {} {} {} {} {} {} {} {} {}\n",
                state.name,
                state.impedance,
                u8::from(state.history_initialized),
                state.current_time,
                state.launched_forward,
                state.launched_backward,
                u8::from(state.initial_state.is_some()),
                state.state_history.len(),
                state.forward_history.len(),
                state.backward_history.len()
            ));
            if let Some(sample) = state.initial_state {
                out.push_str(&format!(
                    "tline_initial {} {} {} {} {}\n",
                    sample[0], sample[1], sample[2], sample[3], sample[4]
                ));
            }
            for sample in &state.state_history {
                out.push_str(&format!(
                    "tline_sample {} {} {} {} {}\n",
                    sample[0], sample[1], sample[2], sample[3], sample[4]
                ));
            }
            for sample in &state.forward_history {
                out.push_str(&format!(
                    "tline_forward {} {} {}\n",
                    sample[0], sample[1], sample[2]
                ));
            }
            for sample in &state.backward_history {
                out.push_str(&format!(
                    "tline_backward {} {} {}\n",
                    sample[0], sample[1], sample[2]
                ));
            }
        }
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

        let restart_identity = if version >= 14 {
            let identity_line = lines.next().ok_or("missing restart identity line")?;
            let identity = identity_line
                .strip_prefix("restart_identity ")
                .map(str::trim)
                .ok_or_else(|| format!("malformed restart identity line: '{identity_line}'"))?;
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
                    "malformed restart identity line: '{identity_line}'"
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

        let startup_mode = if version >= 12 {
            let mode_line = lines.next().ok_or("missing startup mode line")?;
            match mode_line.strip_prefix("startup_mode ").map(str::trim) {
                Some("operating-point") => Some(TransientStartupMode::OperatingPoint),
                Some("uic") => Some(TransientStartupMode::Uic),
                Some("unknown") => None,
                _ => return Err(format!("malformed startup mode line: '{mode_line}'")),
            }
        } else {
            None
        };

        let time_line = lines.next().ok_or("missing time line")?;
        let time: Value = time_line
            .strip_prefix("time ")
            .and_then(|v| v.trim().parse().ok())
            .ok_or_else(|| format!("malformed time line: '{time_line}'"))?;

        let integration_max_step =
            if version >= 14 {
                let line = lines
                    .next()
                    .ok_or_else(|| "missing integration maximum-step line".to_string())?;
                let field = line
                    .strip_prefix("integration_max_step ")
                    .map(str::trim)
                    .ok_or_else(|| format!("malformed integration maximum-step line: '{line}'"))?;
                if field == "none" {
                    None
                } else {
                    Some(field.parse::<Value>().map_err(|_| {
                        format!("malformed integration maximum-step line: '{line}'")
                    })?)
                }
            } else {
                None
            };

        let pending_tline_arrivals = if version >= 14 {
            let line = lines
                .next()
                .ok_or_else(|| "missing pending transmission-line arrivals line".to_string())?;
            let mut fields = line.split_whitespace();
            if fields.next() != Some("pending_tline_arrivals") {
                return Err(format!(
                    "malformed pending transmission-line arrivals line: '{line}'"
                ));
            }
            let count = fields
                .next()
                .ok_or_else(|| {
                    "pending transmission-line arrivals line is missing its count".to_string()
                })?
                .parse::<usize>()
                .map_err(|_| {
                    format!("malformed pending transmission-line arrivals line: '{line}'")
                })?;
            if count > super::MAX_DYNAMIC_TLINE_BREAKPOINTS {
                return Err(format!(
                    "pending transmission-line arrivals count {count} exceeds checkpoint limit {}",
                    super::MAX_DYNAMIC_TLINE_BREAKPOINTS
                ));
            }
            let arrivals = fields
                .map(|field| {
                    field.parse::<Value>().map_err(|_| {
                        format!("malformed pending transmission-line arrival '{field}'")
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if arrivals.len() != count {
                return Err(format!(
                    "pending transmission-line arrivals declared {count} values but contained {}",
                    arrivals.len()
                ));
            }
            arrivals
        } else {
            Vec::new()
        };
        let dynamic_tline_breakpoints_added = if version >= 14 {
            let line = lines.next().ok_or_else(|| {
                "missing dynamic transmission-line breakpoint count line".to_string()
            })?;
            let field = line
                .strip_prefix("dynamic_tline_breakpoints_added ")
                .map(str::trim)
                .ok_or_else(|| {
                    format!("malformed dynamic transmission-line breakpoint count line: '{line}'")
                })?;
            field.parse::<usize>().map_err(|_| {
                format!("malformed dynamic transmission-line breakpoint count line: '{line}'")
            })?
        } else {
            0
        };

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
        let inductor_flux_history_available = if version >= 13 {
            let availability_line = lines
                .next()
                .ok_or_else(|| "missing 'inductor_flux_history_available' line".to_string())?;
            let mut fields = availability_line.split_whitespace();
            if fields.next() != Some("inductor_flux_history_available") {
                return Err(format!(
                    "malformed inductor flux history availability line: '{availability_line}'"
                ));
            }
            let available = fields
                .next()
                .ok_or_else(|| {
                    "inductor flux history availability line is missing its boolean".to_string()
                })
                .and_then(|field| {
                    parse_checkpoint_bool(field, "inductor flux history availability")
                })?;
            if let Some(extra) = fields.next() {
                return Err(format!(
                    "inductor flux history availability line has extra field '{extra}'"
                ));
            }
            available
        } else {
            false
        };
        let ind_columns = if inductor_flux_history_available {
            4
        } else {
            3
        };
        let mut ind_cols = read_value_section(&mut lines, "inductors", ind_columns)?;
        if !inductor_flux_history_available {
            // Files that predate the flux history carry three columns; keep
            // the missing history empty so resume fails closed instead of
            // reading a neighbouring column as the third accepted current.
            ind_cols.insert(2, Vec::new());
        }
        let xyce_memristor_resistance_stores = if version >= 10 {
            read_value_vector(&mut lines, "xyce_memristor_resistance_stores")?
        } else {
            Vec::new()
        };
        let generic_switch_stores = if version >= 11 {
            let columns = read_value_section(&mut lines, "generic_switch_stores", 4)?;
            let count = columns.first().map_or(0, Vec::len);
            (0..count)
                .map(|index| {
                    [
                        columns[0][index],
                        columns[1][index],
                        columns[2][index],
                        columns[3][index],
                    ]
                })
                .collect()
        } else {
            Vec::new()
        };
        let (tline_state_available, tline_resume_blockers, tline_states) = if version >= 14 {
            let availability_line = lines
                .next()
                .ok_or_else(|| "missing 'tline_state_available' line".to_string())?;
            let mut fields = availability_line.split_whitespace();
            if fields.next() != Some("tline_state_available") {
                return Err(format!(
                    "malformed transmission-line availability line: '{availability_line}'"
                ));
            }
            let available = fields
                .next()
                .ok_or_else(|| {
                    "transmission-line availability line is missing its boolean".to_string()
                })
                .and_then(|field| {
                    parse_checkpoint_bool(field, "transmission-line state availability")
                })?;
            if let Some(extra) = fields.next() {
                return Err(format!(
                    "transmission-line availability line has extra field '{extra}'"
                ));
            }
            (
                available,
                read_nonempty_line_vector(&mut lines, "tline_blockers")?,
                read_tline_states(&mut lines)?,
            )
        } else {
            (false, Vec::new(), Vec::new())
        };
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
            restart_identity,
            simulation_identity,
            startup_mode,
            integration_max_step,
            pending_tline_arrivals,
            dynamic_tline_breakpoints_added,
            cap_v_prev: cap_iter.next().unwrap(),
            cap_v_prev_prev: cap_iter.next().unwrap(),
            cap_v_prev_prev_prev: cap_iter.next().unwrap(),
            cap_i_prev: cap_iter.next().unwrap(),
            cap_i_eq: cap_iter.next().unwrap(),
            ind_i_prev: ind_iter.next().unwrap(),
            ind_i_prev_prev: ind_iter.next().unwrap(),
            ind_i_prev_prev_prev: ind_iter.next().unwrap(),
            ind_v_prev: ind_iter.next().unwrap(),
            inductor_flux_history_available,
            xyce_memristor_resistance_stores,
            generic_switch_stores,
            tline_state_available,
            tline_resume_blockers,
            tline_states,
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
        atomic_write_checkpoint(path, self.to_text().as_bytes())
            .map_err(|e| format!("cannot write checkpoint '{}': {e}", path.display()))
    }

    /// Read a checkpoint from a file.
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read checkpoint '{}': {e}", path.display()))?;
        Self::from_text(&text)
    }
}

struct TemporaryCheckpoint {
    path: std::path::PathBuf,
    armed: bool,
}

impl Drop for TemporaryCheckpoint {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

fn write_and_close(mut file: std::fs::File, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;

    file.write_all(bytes)?;
    file.flush()?;
    file.sync_all()
}

fn atomic_write_checkpoint(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMPORARY: AtomicU64 = AtomicU64::new(0);

    reject_checkpoint_destination(path)?;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let file_name = path.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "checkpoint path has no file name",
        )
    })?;

    let mut opened = None;
    for _ in 0..128 {
        let sequence = NEXT_TEMPORARY.fetch_add(1, Ordering::Relaxed);
        let mut temporary_name = file_name.to_os_string();
        temporary_name.push(format!(
            ".rspice-checkpoint.tmp.{}.{sequence}",
            std::process::id()
        ));
        let temporary_path = parent.join(temporary_name);
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
        {
            Ok(file) => {
                opened = Some((temporary_path, file));
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    let (temporary_path, file) = opened.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "could not allocate a unique checkpoint temporary file",
        )
    })?;
    let mut guard = TemporaryCheckpoint {
        path: temporary_path.clone(),
        armed: true,
    };

    // Consumed so the handle is closed before the rename: Windows refuses to
    // replace a file that is still open.
    write_and_close(file, bytes)?;

    // Recheck immediately before replacing the namespace entry. Rename and
    // MoveFileEx replace a racing symlink itself; checkpoint bytes are never
    // opened through the destination.
    reject_checkpoint_destination(path)?;
    replace_checkpoint_atomically(&temporary_path, path)?;
    guard.armed = false;
    Ok(())
}

fn reject_checkpoint_destination(path: &std::path::Path) -> std::io::Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "refusing to replace checkpoint symlink '{}'",
                path.display()
            ),
        )),
        Ok(metadata) if metadata.is_dir() => Err(std::io::Error::new(
            std::io::ErrorKind::IsADirectory,
            format!("checkpoint destination '{}' is a directory", path.display()),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(not(windows))]
fn replace_checkpoint_atomically(
    from: &std::path::Path,
    to: &std::path::Path,
) -> std::io::Result<()> {
    std::fs::rename(from, to)?;
    std::fs::File::open(to.parent().unwrap_or_else(|| std::path::Path::new(".")))?.sync_all()
}

#[cfg(windows)]
fn replace_checkpoint_atomically(
    from: &std::path::Path,
    to: &std::path::Path,
) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt as _;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    let from_wide = from
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let to_wide = to
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    // SAFETY: both paths are NUL-terminated for the duration of the call.
    let result = unsafe {
        MoveFileExW(
            from_wide.as_ptr(),
            to_wide.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
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
    use crate::engine::Engine;

    fn sample() -> TransientCheckpoint {
        TransientCheckpoint {
            time: 1.2345678901234567e-6,
            solution: vec![0.5, -3.25, 1.0e-15, f64::MIN_POSITIVE, -0.0],
            netlist_fingerprint: 0xDEAD_BEEF_0123_4567,
            netlist_identity: Some("fedcba9876543210".repeat(4)),
            restart_identity: Some("1234567890abcdef".repeat(4)),
            simulation_identity: Some("abcdef0123456789".repeat(4)),
            startup_mode: Some(TransientStartupMode::Uic),
            integration_max_step: Some(2.5e-9),
            pending_tline_arrivals: vec![1.5e-6, 2.0e-6],
            dynamic_tline_breakpoints_added: 3,
            cap_v_prev: vec![0.1, -0.2],
            cap_v_prev_prev: vec![0.09, -0.19],
            cap_v_prev_prev_prev: vec![0.08, -0.18],
            cap_i_prev: vec![1e-3, -2e-3],
            cap_i_eq: vec![5e-4, -6e-4],
            ind_i_prev: vec![7e-3],
            ind_i_prev_prev: vec![6.5e-3],
            ind_i_prev_prev_prev: vec![6.25e-3],
            ind_v_prev: vec![0.02],
            inductor_flux_history_available: true,
            xyce_memristor_resistance_stores: Vec::new(),
            generic_switch_stores: vec![[-0.25, 0.125, 0.375, f64::MIN_POSITIVE]],
            tline_state_available: true,
            tline_resume_blockers: Vec::new(),
            tline_states: Vec::new(),
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
            if version < 13 && line.starts_with("inductor_flux_history_available ") {
                continue;
            }
            if version < 13 && line.starts_with("inductors ") {
                // Pre-13 files carried three inductor columns
                // (i_prev, i_prev_prev, v_prev): drop the flux-history column.
                let count = line
                    .split_whitespace()
                    .nth(1)
                    .expect("inductor checkpoint row count")
                    .parse::<usize>()
                    .expect("numeric inductor checkpoint row count");
                output.push_str(line);
                output.push('\n');
                for _ in 0..count {
                    let row = lines.next().expect("complete inductor checkpoint rows");
                    let fields = row.split_whitespace().collect::<Vec<_>>();
                    assert_eq!(
                        fields.len(),
                        4,
                        "current format writes four inductor columns"
                    );
                    output.push_str(&format!("{} {} {}\n", fields[0], fields[1], fields[3]));
                }
                continue;
            }
            if version < 8 && line.starts_with("netlist_identity ") {
                continue;
            }
            if version < 14 && line.starts_with("restart_identity ") {
                continue;
            }
            if version < 14 && line.starts_with("integration_max_step ") {
                continue;
            }
            if version < 14 && line.starts_with("pending_tline_arrivals ") {
                continue;
            }
            if version < 14 && line.starts_with("dynamic_tline_breakpoints_added ") {
                continue;
            }
            if version < 9 && line.starts_with("simulation_identity ") {
                continue;
            }
            if version < 12 && line.starts_with("startup_mode ") {
                continue;
            }
            if version < 10 && line.starts_with("xyce_memristor_resistance_stores ") {
                let count = line
                    .split_whitespace()
                    .nth(1)
                    .expect("memristor store checkpoint vector count")
                    .parse::<usize>()
                    .expect("numeric memristor store checkpoint vector count");
                for _ in 0..count {
                    lines
                        .next()
                        .expect("complete memristor store checkpoint vector");
                }
                continue;
            }
            if version < 11 && line.starts_with("generic_switch_stores ") {
                let count = line
                    .split_whitespace()
                    .nth(1)
                    .expect("generic switch checkpoint row count")
                    .parse::<usize>()
                    .expect("numeric generic switch checkpoint row count");
                for _ in 0..count {
                    lines
                        .next()
                        .expect("complete generic switch checkpoint rows");
                }
                continue;
            }
            if version < 14 && line.starts_with("tline_state_available ") {
                continue;
            }
            if version < 14
                && (line.starts_with("tline_blockers ") || line.starts_with("tline_states "))
            {
                let count = line
                    .split_whitespace()
                    .nth(1)
                    .expect("transmission-line checkpoint row count")
                    .parse::<usize>()
                    .expect("numeric transmission-line checkpoint row count");
                assert_eq!(
                    count, 0,
                    "the generic legacy fixture only supports empty transmission-line state"
                );
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
    fn authenticated_synthetic_origin_max_step_binding_is_exact_and_fails_closed() {
        let mut synthetic = sample();
        synthetic.time = 0.0;
        synthetic.integration_max_step = None;
        let requested = 7.5e-10;

        let bound = synthetic
            .bind_authenticated_synthetic_origin_max_step(requested)
            .expect("an authenticated unbound synthetic t=0 state can select its first cap");
        assert_eq!(synthetic.integration_max_step, None, "binding clones state");
        assert_eq!(
            bound.integration_max_step.map(Value::to_bits),
            Some(requested.to_bits())
        );
        bound
            .validate_integration_max_step(requested)
            .expect("ordinary exact maximum-step validation accepts the bound clone");
        let adjacent = Value::from_bits(requested.to_bits() + 1);
        assert!(
            bound
                .validate_integration_max_step(adjacent)
                .expect_err("ordinary resume remains bit-exact")
                .contains("does not match")
        );

        for invalid in [0.0, -1.0e-9, Value::NAN, Value::INFINITY] {
            let error = synthetic
                .bind_authenticated_synthetic_origin_max_step(invalid)
                .expect_err("invalid first-segment cap must fail closed");
            assert!(
                error.contains("finite and positive"),
                "unexpected invalid-cap error for {invalid:?}: {error}"
            );
        }

        let mut accepted_transient = synthetic.clone();
        accepted_transient.time = 1.0e-12;
        assert!(
            accepted_transient
                .bind_authenticated_synthetic_origin_max_step(requested)
                .expect_err("an accepted transient point is not a synthetic origin")
                .contains("exact t=0")
        );

        let mut negative_zero = synthetic.clone();
        negative_zero.time = -0.0;
        assert!(
            negative_zero
                .bind_authenticated_synthetic_origin_max_step(requested)
                .expect_err("only the canonical synthetic origin is accepted")
                .contains("exact t=0")
        );

        let mut already_bound = synthetic;
        already_bound.integration_max_step = Some(requested);
        assert!(
            already_bound
                .bind_authenticated_synthetic_origin_max_step(requested)
                .expect_err("an existing trajectory cap cannot be rebound")
                .contains("already records")
        );
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
    fn checkpoint_save_durably_replaces_an_existing_file() {
        let directory = std::env::temp_dir().join(format!(
            "rspice-checkpoint-save-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time after epoch")
                .as_nanos()
        ));
        std::fs::create_dir(&directory).expect("create checkpoint test directory");
        let path = directory.join("state.chk");
        std::fs::write(&path, b"obsolete partial checkpoint").expect("seed old checkpoint");

        let expected = sample();
        expected.save(&path).expect("atomically save checkpoint");
        let actual = TransientCheckpoint::load(&path).expect("load replaced checkpoint");

        assert_eq!(actual, expected);
        assert_eq!(
            std::fs::read_dir(&directory)
                .expect("read checkpoint test directory")
                .count(),
            1,
            "temporary checkpoint must not remain after commit"
        );
        std::fs::remove_dir_all(directory).expect("remove checkpoint test directory");
    }

    #[test]
    fn invalid_checkpoint_never_truncates_the_last_good_file() {
        let directory = std::env::temp_dir().join(format!(
            "rspice-checkpoint-invalid-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time after epoch")
                .as_nanos()
        ));
        std::fs::create_dir(&directory).expect("create checkpoint test directory");
        let path = directory.join("state.chk");
        std::fs::write(&path, b"last known good checkpoint").expect("seed old checkpoint");
        let mut invalid = sample();
        invalid.solution[0] = Value::NAN;

        invalid
            .save(&path)
            .expect_err("invalid state must fail before I/O");

        assert_eq!(
            std::fs::read(&path).expect("read preserved checkpoint"),
            b"last known good checkpoint"
        );
        std::fs::remove_dir_all(directory).expect("remove checkpoint test directory");
    }

    #[test]
    fn startup_mode_round_trips_and_legacy_format_does_not_invent_it() {
        let checkpoint = sample();
        let current = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("current checkpoint parses");
        assert_eq!(current.startup_mode(), Some(TransientStartupMode::Uic));

        let legacy = TransientCheckpoint::from_text(&legacy_text(&checkpoint, 11))
            .expect("version-eleven checkpoint remains parseable for diagnostics");
        assert_eq!(legacy.startup_mode(), None);
    }

    #[test]
    fn rshunt_participates_in_resolved_simulation_checkpoint_identity() {
        let base = SimulationConfig::default();
        let with_rshunt = SimulationConfig {
            rshunt: Some(1.0e9),
            ..base.clone()
        };
        assert_ne!(
            simulation_checkpoint_identity(&base),
            simulation_checkpoint_identity(&with_rshunt),
            "changing the physical global shunt must invalidate transient and HB continuation state"
        );
    }

    #[test]
    fn voltage_limiting_policy_participates_in_checkpoint_identity() {
        let enabled = SimulationConfig::default();
        let disabled = SimulationConfig {
            device_voltage_limiting: false,
            ..enabled.clone()
        };
        assert_ne!(
            simulation_checkpoint_identity(&enabled),
            simulation_checkpoint_identity(&disabled),
            "limiter-owned and raw Newton state must never share a checkpoint identity"
        );
    }

    #[test]
    fn pem_resistance_store_round_trips_and_legacy_resume_fails_closed() {
        let unique = format!(
            "checkpoint-pem-store-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let positive = format!("virtual://pem/{unique}/positive");
        let negative = format!("virtual://pem/{unique}/negative");
        crate::xspice::register_data_file(&positive, "0,1\n1,0\n")
            .expect("register positive PEM table");
        crate::xspice::register_data_file(&negative, "0,0\n1,1\n")
            .expect("register negative PEM table");
        let deck = format!(
            "PEM store checkpoint\n\
             .model pem memristor level=4 fxpdata={positive} fxmdata={negative}\n\
             YMEMRISTOR one 1 0 pem\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("PEM checkpoint deck parses");
        let engine = Engine::default();
        let mut captured_circuit = engine
            .build_circuit(&netlist)
            .expect("captured PEM circuit builds");
        captured_circuit.xyce_memristors[0].resistance_store = 4321.25;
        let solution = vec![0.0; captured_circuit.num_nodes() + captured_circuit.num_branches()];
        let checkpoint = TransientCheckpoint::capture_with_restart_identity(
            netlist_fingerprint(&netlist),
            netlist_checkpoint_identity(&netlist),
            restart_checkpoint_identity(&netlist),
            simulation_checkpoint_identity(engine.config()),
            0.0,
            &solution,
            &captured_circuit,
            TransientStartupMode::OperatingPoint,
            None,
            &[],
            0,
            None,
        );
        let restored = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("PEM store checkpoint parses");
        let mut resumed_circuit = engine
            .build_circuit(&netlist)
            .expect("resumed PEM circuit builds");
        restored
            .inject(&mut resumed_circuit)
            .expect("PEM store state injects");
        assert_eq!(
            resumed_circuit.xyce_memristors[0]
                .resistance_store
                .to_bits(),
            4321.25f64.to_bits()
        );

        let legacy = TransientCheckpoint::from_text(&legacy_text(&checkpoint, 9))
            .expect("version-9 checkpoint parses");
        let error = legacy
            .inject(&mut resumed_circuit)
            .expect_err("legacy checkpoint lacks PEM store state");
        assert!(error.contains("memristor resistance store"));
        crate::xspice::unregister_data_file(&positive).expect("unregister positive PEM table");
        crate::xspice::unregister_data_file(&negative).expect("unregister negative PEM table");
    }

    #[test]
    fn signal_history_lte_references_round_trip_and_legacy_resume_fails_closed() {
        let checkpoint = sample();
        let restored = TransientCheckpoint::from_text(&checkpoint.to_text())
            .expect("current checkpoint format parses");
        let mut estimator = LteEstimator::with_tolerances_and_reference(
            1.0e-3,
            1.0e-6,
            crate::numerics::integration::TransientLteReference::SignalGlobal,
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
        let mut circuit = CircuitData::new();
        circuit.capacitors.v_prev = vec![91.0, 92.0];
        circuit.capacitors.v_prev_prev = vec![81.0, 82.0];
        circuit.capacitors.v_prev_prev_prev = vec![71.0, 72.0];
        circuit.capacitors.i_prev = vec![61.0, 62.0];
        circuit.capacitors.i_eq = vec![51.0, 52.0];
        circuit.inductors.i_prev = vec![41.0];
        circuit.inductors.i_prev_prev = vec![31.0];
        circuit.inductors.i_prev_prev_prev = vec![26.0];
        circuit.inductors.v_prev = vec![21.0];
        let before = (
            circuit.capacitors.v_prev.clone(),
            circuit.capacitors.v_prev_prev.clone(),
            circuit.capacitors.v_prev_prev_prev.clone(),
            circuit.capacitors.i_prev.clone(),
            circuit.capacitors.i_eq.clone(),
            circuit.inductors.i_prev.clone(),
            circuit.inductors.i_prev_prev.clone(),
            circuit.inductors.i_prev_prev_prev.clone(),
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
                circuit.inductors.i_prev_prev_prev,
                circuit.inductors.v_prev,
            ),
            "checkpoint rejection must occur before any circuit state mutation"
        );
    }

    #[test]
    fn legacy_checkpoint_without_inductor_flux_history_fails_closed_for_inductors() {
        let mut checkpoint = sample();
        // Keep the generated Verilog-A gate out of the way: this test is
        // about the inductor refusal, which inject checks after it.
        checkpoint.generated_veriloga_instance_states.clear();
        let legacy = TransientCheckpoint::from_text(&legacy_text(&checkpoint, 12))
            .expect("version-12 checkpoint remains readable");
        assert!(!legacy.inductor_flux_history_available);
        assert!(legacy.ind_i_prev_prev_prev.is_empty());
        assert_eq!(legacy.ind_i_prev, checkpoint.ind_i_prev);
        assert_eq!(legacy.ind_i_prev_prev, checkpoint.ind_i_prev_prev);
        assert_eq!(legacy.ind_v_prev, checkpoint.ind_v_prev);

        // Re-serializing keeps the absence explicit rather than inventing a
        // third accepted current.
        let upgraded = TransientCheckpoint::from_text(&legacy.to_text())
            .expect("legacy checkpoint re-serializes in the current format");
        assert!(!upgraded.inductor_flux_history_available);
        assert!(upgraded.ind_i_prev_prev_prev.is_empty());
        assert_eq!(upgraded.ind_i_prev_prev, checkpoint.ind_i_prev_prev);

        let mut circuit = CircuitData::new();
        circuit.inductors.i_prev = vec![41.0];
        circuit.inductors.i_prev_prev = vec![31.0];
        circuit.inductors.i_prev_prev_prev = vec![26.0];
        circuit.inductors.v_prev = vec![21.0];
        circuit.inductors.names = vec!["L1".to_string()];
        let err = legacy
            .inject(&mut circuit)
            .expect_err("legacy checkpoint cannot resume a circuit with inductors");
        assert!(
            err.contains("inductor flux history"),
            "unexpected error: {err}"
        );
        assert_eq!(circuit.inductors.i_prev_prev_prev, vec![26.0]);
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
    fn transient_schedule_checkpoint_identity_is_canonical_and_semantic() {
        let first = Netlist::parse(
            "scheduled checkpoint\n\
             V1 1 0 1\n\
             .TRAN 1m 5m\n\
             .OPTIONS OUTPUT OUTPUTTIMEPOINTS=4m,1m,2m,1m\n\
             .OPTIONS TIMEINT BREAKPOINTS=3m,2m\n\
             .END\n",
        )
        .expect("first scheduled deck parses");
        let reordered = Netlist::parse(
            "scheduled checkpoint\n\
             V1 1 0 1\n\
             .TRAN 1m 5m\n\
             .OPTIONS OUTPUT OUTPUTTIMEPOINTS=1m,2m,4m\n\
             .OPTIONS TIMEINT BREAKPOINTS=2m,3m\n\
             .END\n",
        )
        .expect("reordered scheduled deck parses");
        let changed = Netlist::parse(
            "scheduled checkpoint\n\
             V1 1 0 1\n\
             .TRAN 1m 5m\n\
             .OPTIONS OUTPUT OUTPUTTIMEPOINTS=1m,2m,4m\n\
             .OPTIONS TIMEINT BREAKPOINTS=2m,3.5m\n\
             .END\n",
        )
        .expect("changed scheduled deck parses");

        assert_eq!(
            netlist_checkpoint_identity(&first),
            netlist_checkpoint_identity(&reordered),
            "sorted/deduplicated schedules must have one semantic identity"
        );
        assert_ne!(
            netlist_checkpoint_identity(&first),
            netlist_checkpoint_identity(&changed),
            "effective schedule changes must invalidate checkpoint identity"
        );

        let mut programmatic = reordered.clone();
        programmatic.options.output_time_points = vec![4.0e-3, 1.0e-3, 1.0e-3, 2.0e-3];
        programmatic.options.timeint_breakpoints = vec![3.0e-3, 2.0e-3, 2.0e-3];
        assert_eq!(
            netlist_checkpoint_identity(&first),
            netlist_checkpoint_identity(&programmatic),
            "programmatic AST schedule ordering and duplicates are not semantic"
        );
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
    fn output_request_analysis_and_delimiter_change_checkpoint_identity() {
        let base = Netlist::parse(
            "checkpoint output request\n\
             V1 out 0 1\n\
             R1 out 0 1k\n\
             .TRAN 1n 2n\n\
             .PRINT TRAN V(out)\n\
             .END\n",
        )
        .expect("output-request deck parses");
        let mut changed_analysis = base.clone();
        changed_analysis.output_requests[0].analysis = Some(crate::netlist::OutputAnalysisKind::Dc);
        assert_ne!(
            netlist_checkpoint_identity(&base),
            netlist_checkpoint_identity(&changed_analysis),
            "analysis ownership changes retained transient output state"
        );

        let mut changed_delimiter = base.clone();
        changed_delimiter.output_requests[0].print_delimiter =
            Some(crate::netlist::PrintDelimiter::Comma);
        assert_ne!(
            netlist_checkpoint_identity(&base),
            netlist_checkpoint_identity(&changed_delimiter),
            "output serialization semantics belong to checkpoint provenance"
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
    fn bug_1284_restart_identity_normalizes_only_horizon_and_restart_control_plane() {
        let deck = |tran: &str, restart: &str| {
            format!(
                "Transmission Line Circuit\n\
                 VIN 1 0 PULSE(0 5 0 0.1N 0.1N 5N 25N)\n\
                 RIN 1 2 50\n\
                 TLINE 2 0 3 0 Z0=50 TD=10N\n\
                 RL 3 0 50\n\
                 {tran}\n\
                 .PRINT TRAN V(2) V(3)\n\
                 {restart}\n\
                 .END\n"
            )
        };
        let baseline = Netlist::parse(&deck(".TRAN 0.25N 50N", ""))
            .expect("BUG_1284 baseline semantics parse");
        let first = Netlist::parse(&deck(
            ".TRAN 0.25N 20N",
            ".OPTIONS RESTART JOB=trans_test INITIAL_INTERVAL=5n",
        ))
        .expect("BUG_1284 first-run semantics parse");
        let restarted = Netlist::parse(&deck(
            ".TRAN 0.25N 50N",
            ".OPTIONS RESTART FILE=trans_test2e-08",
        ))
        .expect("BUG_1284 restarted-run semantics parse");

        let compatible = restart_checkpoint_identity(&first).expect("restart identity");
        assert_eq!(
            restart_checkpoint_identity(&baseline).as_deref(),
            Some(compatible.as_str()),
            "a no-RESTART baseline has the same physical trajectory contract"
        );
        assert_eq!(
            restart_checkpoint_identity(&restarted).as_deref(),
            Some(compatible.as_str()),
            "restart horizon and file metadata are control-plane differences"
        );
        assert_ne!(
            netlist_checkpoint_identity(&first),
            netlist_checkpoint_identity(&restarted),
            "the ordinary resume identity must remain exact"
        );

        let mut checkpoint = sample();
        checkpoint.netlist_fingerprint = netlist_fingerprint(&first);
        checkpoint.netlist_identity = netlist_checkpoint_identity(&first);
        checkpoint.restart_identity = Some(compatible);
        checkpoint
            .validate_for_restart(&restarted)
            .expect("canonical restarted deck is restart-compatible");
        let exact_error = checkpoint
            .validate_for(&restarted)
            .expect_err("ordinary resume must remain same-deck strict");
        assert!(
            exact_error.contains("different netlist"),
            "unexpected exact-resume diagnostic: {exact_error}"
        );

        let drifted_decks = [
            deck(".TRAN 0.25N 50N", "").replace("RIN 1 2 50", "RIN 1 2 51"),
            deck(".TRAN 0.25N 50N", "")
                .replace(".TRAN 0.25N 50N", "CEXTRA 2 0 1p\n.TRAN 0.25N 50N"),
            deck(".TRAN 0.5N 50N", ""),
            deck(".TRAN 0.25N 50N 1N", ""),
            deck(".TRAN 0.25N 50N 0 0.1N", ""),
            deck(".TRAN 0.25N 50N UIC", ""),
            deck(".TRAN 0.25N 50N", "").replace(".PRINT TRAN V(2) V(3)", ".PRINT TRAN V(2) V(1)"),
        ];
        for drifted in drifted_decks {
            let drifted = Netlist::parse(&drifted).expect("drifted restart deck parses");
            let error = checkpoint
                .validate_for_restart(&drifted)
                .expect_err("physical or transient-control drift must fail closed");
            assert!(
                error.contains("restart-incompatible netlist"),
                "unexpected restart incompatibility diagnostic: {error}"
            );
        }

        let legacy = TransientCheckpoint::from_text(&legacy_text(&checkpoint, 13))
            .expect("pre-restart-identity checkpoint remains parseable");
        let error = legacy
            .validate_for_restart(&first)
            .expect_err("legacy checkpoint must not authorize a changed restart deck");
        assert!(
            error.contains("collision-resistant restart identity"),
            "unexpected legacy restart diagnostic: {error}"
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
    fn restart_identity_parser_requires_lowercase_blake3_hex() {
        let text = sample().to_text();
        let valid = "1234567890abcdef".repeat(4);
        for invalid in ["abc".to_string(), "A".repeat(64), "g".repeat(64)] {
            let err = TransientCheckpoint::from_text(&text.replacen(&valid, &invalid, 1))
                .expect_err("malformed restart identity must fail during parsing");
            assert!(
                err.contains("malformed restart identity"),
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
    fn pem_virtual_table_content_changes_checkpoint_identity() {
        let unique = format!(
            "rspice-checkpoint-pem-authored-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let positive = format!("virtual://pem/{unique}/positive");
        let negative = format!("virtual://pem/{unique}/negative");
        crate::xspice::register_data_file(&positive, "0,0\n1,1\n")
            .expect("register positive PEM table");
        crate::xspice::register_data_file(&negative, "0,0\n1,-1\n")
            .expect("register negative PEM table");
        let root = std::env::temp_dir().join(&unique).join("root.cir");
        let source = format!(
            "PEM dependency identity\n\
             .model pem memristor level=4 fxpdata={positive} fxmdata={negative}\n\
             YMEMRISTOR mr1 1 0 pem\n\
             .end\n"
        );
        let netlist = Netlist::parse_with_path(&source, &root).expect("PEM deck parses");

        let first = netlist_checkpoint_identity(&netlist);
        crate::xspice::register_data_file(&positive, "0,0\n1,2\n")
            .expect("replace positive PEM table");
        let changed_positive = netlist_checkpoint_identity(&netlist);
        crate::xspice::register_data_file(&negative, "0,0\n1,-2\n")
            .expect("replace negative PEM table");
        let changed_negative = netlist_checkpoint_identity(&netlist);
        crate::xspice::unregister_data_file(&positive).expect("unregister positive PEM table");
        crate::xspice::unregister_data_file(&negative).expect("unregister negative PEM table");

        assert_ne!(
            first, changed_positive,
            "replacing FXPDATA contents must invalidate checkpoint provenance"
        );
        assert_ne!(
            changed_positive, changed_negative,
            "replacing FXMDATA contents must invalidate checkpoint provenance"
        );
    }

    #[test]
    fn pem_default_table_content_changes_checkpoint_identity() {
        let unique = format!(
            "rspice-checkpoint-pem-default-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        );
        let directory = std::env::temp_dir().join(unique);
        let root = directory.join("root.cir");
        let positive = directory
            .join(crate::device::XYCE_PEM_DEFAULT_POSITIVE_TABLE_FILE)
            .to_string_lossy()
            .into_owned();
        let negative = directory
            .join(crate::device::XYCE_PEM_DEFAULT_NEGATIVE_TABLE_FILE)
            .to_string_lossy()
            .into_owned();
        crate::xspice::register_data_file(&positive, "0,0\n1,1\n")
            .expect("register default positive PEM table");
        crate::xspice::register_data_file(&negative, "0,0\n1,-1\n")
            .expect("register default negative PEM table");
        let netlist = Netlist::parse_with_path(
            "PEM default dependency identity\n\
             .model pem memristor level=4\n\
             YMEMRISTOR mr1 1 0 pem\n\
             .end\n",
            &root,
        )
        .expect("PEM default-table deck parses");

        let first = netlist_checkpoint_identity(&netlist);
        crate::xspice::register_data_file(&positive, "0,0\n1,2\n")
            .expect("replace default positive PEM table");
        let changed_positive = netlist_checkpoint_identity(&netlist);
        crate::xspice::register_data_file(&negative, "0,0\n1,-2\n")
            .expect("replace default negative PEM table");
        let changed_negative = netlist_checkpoint_identity(&netlist);
        crate::xspice::unregister_data_file(&positive)
            .expect("unregister default positive PEM table");
        crate::xspice::unregister_data_file(&negative)
            .expect("unregister default negative PEM table");

        assert_ne!(
            first, changed_positive,
            "replacing default filep.dat contents must invalidate checkpoint provenance"
        );
        assert_ne!(
            changed_positive, changed_negative,
            "replacing default filem.dat contents must invalidate checkpoint provenance"
        );
    }

    #[test]
    fn oversized_pem_dependency_identity_is_bounded_without_hashing_contents() {
        let path = "virtual://checkpoint/pem-oversized-identity";
        crate::xspice::register_data_file(path, "first")
            .expect("register oversized identity fixture");
        let mut first = blake3::Hasher::new();
        hash_dependency(&mut first, path, None, true, Some(4));
        crate::xspice::register_data_file(path, "other")
            .expect("replace oversized identity fixture");
        let mut second = blake3::Hasher::new();
        hash_dependency(&mut second, path, None, true, Some(4));
        crate::xspice::unregister_data_file(path).expect("unregister oversized fixture");

        assert_eq!(
            first.finalize(),
            second.finalize(),
            "invalid over-budget dependencies must be identified by kind/length/limit without reading their payload"
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

        let mut changed_tra_interpolation = base.clone();
        changed_tra_interpolation.xyce_tra_interpolation =
            crate::engine::XyceTraInterpolation::LegacyQuadratic;
        checkpoint
            .validate_for_with_config(&netlist, &changed_tra_interpolation)
            .expect_err("TRA interpolation mismatch must reject state");

        let mut changed_nonlin_budget = base.clone();
        changed_nonlin_budget.transient_nonlinear_max_iterations = Some(21);
        checkpoint
            .validate_for_with_config(&netlist, &changed_nonlin_budget)
            .expect_err("transient nonlinear budget mismatch must reject state");

        let mut changed_delmax = base.clone();
        changed_delmax.transient_timeint_max_timestep = Some(1.0e-9);
        checkpoint
            .validate_for_with_config(&netlist, &changed_delmax)
            .expect_err("TIMEINT DELMAX mismatch must reject state");

        let mut changed_use_device_max = base.clone();
        changed_use_device_max.transient_use_device_max_timestep = Some(false);
        checkpoint
            .validate_for_with_config(&netlist, &changed_use_device_max)
            .expect_err("TIMEINT USEDEVICEMAX mismatch must reject state");

        let mut changed_model = base;
        changed_model.jfet_level2_model = crate::engine::JfetLevel2Model::XyceModifiedShockley;
        checkpoint
            .validate_for_with_config(&netlist, &changed_model)
            .expect_err("resolved model-routing mismatch must reject state");
    }

    #[test]
    fn timeint_iteration_control_and_orders_are_identity_bound_and_span_resume_is_allowed() {
        let netlist = Netlist::parse("TIMEINT checkpoint\nr1 1 0 1k\n.end\n").unwrap();
        let base =
            SimulationConfig::default().with_spice_dialect(crate::engine::SpiceDialect::Xyce);
        let mut iteration_control = base.clone();
        iteration_control.transient_error_control =
            crate::numerics::integration::TransientErrorControl::NonlinearIterations;

        assert_ne!(
            simulation_checkpoint_identity(&base),
            simulation_checkpoint_identity(&iteration_control),
            "ERROPTION must participate in checkpoint configuration identity"
        );
        let mut fixed_order = iteration_control.clone();
        fixed_order.transient_timeint_max_order = 1;
        assert_ne!(
            simulation_checkpoint_identity(&iteration_control),
            simulation_checkpoint_identity(&fixed_order),
            "MINORD/MAXORD must participate in checkpoint identity"
        );

        let mut checkpoint = sample();
        checkpoint.netlist_fingerprint = netlist_fingerprint(&netlist);
        checkpoint.netlist_identity = netlist_checkpoint_identity(&netlist);
        checkpoint.simulation_identity = Some(simulation_checkpoint_identity(&iteration_control));
        checkpoint
            .validate_for_with_config(&netlist, &iteration_control)
            .expect("resume reanchors the active span from checkpoint time");
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

    #[test]
    fn startup_warning_sidecars_do_not_split_semantic_checkpoint_identity() {
        let omitted = Netlist::parse("startup identity\nV1 1 0 1\n.OP\n.END\n")
            .expect("omitted startup deck parses");
        let empty = Netlist::parse("startup identity\nV1 1 0 1\n.IC\n.OP\n.END\n")
            .expect("empty startup deck parses");
        let missing = Netlist::parse("startup identity\nV1 1 0 1\n.IC V(missing)=2\n.OP\n.END\n")
            .expect("missing startup node is warning-only");
        assert_eq!(
            netlist_checkpoint_identity(&omitted),
            netlist_checkpoint_identity(&empty),
            "empty diagnostic provenance is not simulation state"
        );
        assert_eq!(
            netlist_checkpoint_identity(&omitted),
            netlist_checkpoint_identity(&missing),
            "ignored missing-node provenance is not simulation state"
        );

        let xyce_options = crate::netlist::NetlistParseOptions {
            statistical_mode: crate::netlist::StatisticalParamMode::Sample,
            expression_dialect: crate::config::ExpressionDialect::Xyce,
            parameter_redefinition_policy: crate::netlist::ParameterRedefinitionPolicy::UseLast,
            ..crate::netlist::NetlistParseOptions::default()
        };
        let scoped_omitted = crate::netlist::parse_netlist_with_options(
            "scoped startup identity\n\
             .GLOBAL VCC\n\
             V1 1 0 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL a b\n\
             R1 a b 1\n\
             .ENDS\n\
             .OP\n\
             .END\n",
            xyce_options,
        )
        .expect("scoped baseline parses");
        let scoped_global = crate::netlist::parse_netlist_with_options(
            "scoped startup identity\n\
             .GLOBAL VCC\n\
             V1 1 0 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL a b\n\
             R1 a b 1\n\
             .NODESET V(VCC)=3\n\
             .ENDS\n\
             .OP\n\
             .END\n",
            xyce_options,
        )
        .expect("scoped global startup is warning-only");
        assert_eq!(
            netlist_checkpoint_identity(&scoped_omitted),
            netlist_checkpoint_identity(&scoped_global),
            "whole-card scoped-global discard is simulation-identical to omission"
        );
    }

    #[test]
    fn effective_startup_value_changes_checkpoint_identity() {
        let first = Netlist::parse("startup value identity\nV1 1 0 1\n.IC V(1)=0.25\n.OP\n.END\n")
            .expect("first startup value parses");
        let second = Netlist::parse("startup value identity\nV1 1 0 1\n.IC V(1)=0.5\n.OP\n.END\n")
            .expect("second startup value parses");
        assert_ne!(
            netlist_checkpoint_identity(&first),
            netlist_checkpoint_identity(&second),
            "effective numeric startup values remain semantic checkpoint state"
        );
    }
}
