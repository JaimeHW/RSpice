//! Circuit builder - constructs CircuitData from Netlist
//!
//! This module handles the conversion from parsed netlist elements
//! to the runtime circuit representation.

use super::{Engine, SimulationError, extract_dc_value};
use crate::netlist::{ElementKind, flatten_netlist};
use crate::{CircuitData, Netlist};
#[cfg(feature = "veriloga")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "veriloga")]
use std::io::Read;
#[cfg(feature = "veriloga")]
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
#[cfg(feature = "veriloga")]
use std::sync::RwLock;

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

#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_RECORD_VERSION: u32 = 1;

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerilogADependencyFingerprint {
    canonical_path: PathBuf,
    modified_ns: Option<u128>,
    file_len: u64,
    content_hash: [u8; 32],
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerilogADiskCacheRecord {
    version: u32,
    source_path: PathBuf,
    dependencies: Vec<VerilogADependencyFingerprint>,
    model: rspice_veriloga::CompiledModel,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
struct CachedVerilogAModel {
    dependencies: Vec<VerilogADependencyFingerprint>,
    model: rspice_veriloga::CompiledModel,
}

#[cfg(feature = "veriloga")]
fn veriloga_model_cache() -> &'static RwLock<HashMap<PathBuf, CachedVerilogAModel>> {
    static CACHE: OnceLock<RwLock<HashMap<PathBuf, CachedVerilogAModel>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

#[cfg(feature = "veriloga")]
fn canonicalize_for_cache(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(feature = "veriloga")]
fn normalize_model_key(name: &str) -> String {
    name.to_ascii_uppercase()
}

#[cfg(feature = "veriloga")]
fn metadata_modified_ns(metadata: &std::fs::Metadata) -> Option<u128> {
    use std::time::UNIX_EPOCH;

    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_nanos())
}

#[cfg(feature = "veriloga")]
fn hash_file(path: &Path) -> std::io::Result<[u8; 32]> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(*hasher.finalize().as_bytes())
}

#[cfg(feature = "veriloga")]
fn dependency_fingerprint(path: &Path) -> Option<VerilogADependencyFingerprint> {
    let canonical_path = canonicalize_for_cache(path);
    let metadata = std::fs::metadata(&canonical_path).ok()?;
    let content_hash = hash_file(&canonical_path).ok()?;
    Some(VerilogADependencyFingerprint {
        canonical_path,
        modified_ns: metadata_modified_ns(&metadata),
        file_len: metadata.len(),
        content_hash,
    })
}

#[cfg(feature = "veriloga")]
fn fingerprint_paths(
    paths: &[PathBuf],
) -> Result<Vec<VerilogADependencyFingerprint>, SimulationError> {
    let mut canonical_paths: Vec<PathBuf> =
        paths.iter().map(|p| canonicalize_for_cache(p)).collect();
    canonical_paths.sort();
    canonical_paths.dedup();

    let mut fingerprints = Vec::with_capacity(canonical_paths.len());
    for canonical_path in canonical_paths {
        let fingerprint = dependency_fingerprint(&canonical_path).ok_or_else(|| {
            SimulationError::Netlist(format!(
                "Verilog-A dependency does not exist or is unreadable: {}",
                canonical_path.display()
            ))
        })?;
        fingerprints.push(fingerprint);
    }

    Ok(fingerprints)
}

#[cfg(feature = "veriloga")]
fn dependency_matches_cached_fingerprint(dep: &VerilogADependencyFingerprint) -> bool {
    let metadata = match std::fs::metadata(&dep.canonical_path) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };

    let current_modified_ns = metadata_modified_ns(&metadata);
    if metadata.len() == dep.file_len && current_modified_ns == dep.modified_ns {
        return true;
    }

    match hash_file(&dep.canonical_path) {
        Ok(hash) => hash == dep.content_hash,
        Err(_) => false,
    }
}

#[cfg(feature = "veriloga")]
fn dependencies_are_fresh(dependencies: &[VerilogADependencyFingerprint]) -> bool {
    dependencies
        .iter()
        .all(dependency_matches_cached_fingerprint)
}

#[cfg(feature = "veriloga")]
fn veriloga_cache_root() -> PathBuf {
    if let Some(override_dir) = std::env::var_os("RSPICE_VERILOGA_CACHE_DIR") {
        return PathBuf::from(override_dir);
    }

    if let Some(cache_dir) = dirs::cache_dir() {
        return cache_dir.join("rspice").join("veriloga");
    }

    std::env::temp_dir().join("rspice-veriloga-cache")
}

#[cfg(feature = "veriloga")]
fn cache_record_path(source_path: &Path) -> PathBuf {
    let canonical = canonicalize_for_cache(source_path);
    let mut hasher = blake3::Hasher::new();
    hasher.update(canonical.to_string_lossy().as_bytes());
    let key = hasher.finalize().to_hex().to_string();
    veriloga_cache_root().join(format!("{key}.bin"))
}

#[cfg(feature = "veriloga")]
fn persist_model_to_disk(source_path: &Path, entry: &CachedVerilogAModel) -> Result<(), String> {
    let cache_path = cache_record_path(source_path);
    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create cache directory: {}", e))?;
    }

    let record = VerilogADiskCacheRecord {
        version: VERILOGA_CACHE_RECORD_VERSION,
        source_path: canonicalize_for_cache(source_path),
        dependencies: entry.dependencies.clone(),
        model: entry.model.clone(),
    };
    let encoded = bincode::serialize(&record)
        .map_err(|e| format!("failed to serialize Verilog-A cache record: {}", e))?;

    let tmp_path = cache_path.with_extension("tmp");
    std::fs::write(&tmp_path, encoded)
        .map_err(|e| format!("failed to write Verilog-A cache record: {}", e))?;
    std::fs::rename(&tmp_path, &cache_path)
        .map_err(|e| format!("failed to finalize Verilog-A cache record: {}", e))?;

    Ok(())
}

#[cfg(feature = "veriloga")]
fn load_model_from_disk(source_path: &Path) -> Option<CachedVerilogAModel> {
    let cache_path = cache_record_path(source_path);
    let bytes = std::fs::read(cache_path).ok()?;
    let record: VerilogADiskCacheRecord = bincode::deserialize(&bytes).ok()?;
    if record.version != VERILOGA_CACHE_RECORD_VERSION {
        return None;
    }

    let requested_source = canonicalize_for_cache(source_path);
    let record_source = canonicalize_for_cache(&record.source_path);
    if requested_source != record_source {
        return None;
    }

    if !dependencies_are_fresh(&record.dependencies) {
        return None;
    }

    Some(CachedVerilogAModel {
        dependencies: record.dependencies,
        model: record.model,
    })
}

#[cfg(feature = "veriloga")]
fn resolve_cached_or_compile_veriloga(
    path: &Path,
) -> Result<rspice_veriloga::CompiledModel, SimulationError> {
    let canonical = canonicalize_for_cache(path);

    if let Ok(cache) = veriloga_model_cache().read() {
        if let Some(entry) = cache.get(&canonical) {
            if dependencies_are_fresh(&entry.dependencies) {
                return Ok(entry.model.clone());
            }
        }
    }

    if let Some(entry) = load_model_from_disk(&canonical) {
        let model = entry.model.clone();
        if let Ok(mut cache) = veriloga_model_cache().write() {
            cache.insert(canonical.clone(), entry);
        }
        return Ok(model);
    }

    let compiler = rspice_veriloga::VerilogACompiler::default();
    let compiled = compiler.compile_file_with_metadata(path).map_err(|e| {
        SimulationError::Netlist(format!(
            "Failed to compile Verilog-A '{}': {}",
            path.display(),
            e
        ))
    })?;

    let dependencies = fingerprint_paths(&compiled.dependencies)?;
    let entry = CachedVerilogAModel {
        dependencies,
        model: compiled.model.clone(),
    };

    if let Ok(mut cache) = veriloga_model_cache().write() {
        cache.insert(canonical.clone(), entry.clone());
    }

    if let Err(err) = persist_model_to_disk(&canonical, &entry) {
        log::warn!(
            "Failed to persist Verilog-A cache entry for '{}': {}",
            canonical.display(),
            err
        );
    }

    Ok(compiled.model)
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This allows UI workflows to compile once on import and reuse the compiled
/// artifact during simulation without recompilation.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    let canonical_source = canonicalize_for_cache(source_path.as_ref());
    let mut dependency_paths = dependencies.to_vec();
    if dependency_paths.is_empty() {
        dependency_paths.push(canonical_source.clone());
    }
    let dependency_fingerprints = fingerprint_paths(&dependency_paths)
        .map_err(|e| format!("dependency fingerprinting failed: {}", e))?;

    let entry = CachedVerilogAModel {
        dependencies: dependency_fingerprints,
        model,
    };

    let mut cache = veriloga_model_cache()
        .write()
        .map_err(|_| "failed to acquire Verilog-A cache lock".to_string())?;
    cache.insert(canonical_source.clone(), entry.clone());
    drop(cache);

    if let Err(err) = persist_model_to_disk(&canonical_source, &entry) {
        log::warn!(
            "Failed to persist precompiled Verilog-A cache for '{}': {}",
            canonical_source.display(),
            err
        );
    }

    Ok(())
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This compatibility wrapper fingerprints only the source file path.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model(
    source_path: impl AsRef<Path>,
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    let dependency = vec![canonicalize_for_cache(source_path.as_ref())];
    register_precompiled_veriloga_model_with_dependencies(source_path, &dependency, model)
}

#[derive(Debug, Clone, Copy, Default)]
struct TransmissionLineModelParams {
    z0: Option<f64>,
    td: Option<f64>,
    freq: Option<f64>,
    nl: Option<f64>,
    r: Option<f64>,
    g: Option<f64>,
    len: Option<f64>,
    alpha: Option<f64>,
    atten: Option<f64>,
}

fn model_param(params: &[(String, f64)], names: &[&str]) -> Option<f64> {
    params.iter().find_map(|(name, value)| {
        if names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
        {
            Some(*value)
        } else {
            None
        }
    })
}

fn resolve_tline_model_params(
    netlist: &Netlist,
    model_name: &str,
) -> Option<TransmissionLineModelParams> {
    let model = netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name))?;

    let mut params = TransmissionLineModelParams {
        z0: model_param(&model.params, &["Z0", "ZO"]),
        td: model_param(&model.params, &["TD", "TDELAY"]),
        freq: model_param(&model.params, &["F", "FREQ"]),
        nl: model_param(&model.params, &["NL"]),
        r: model_param(&model.params, &["R", "R0"]),
        g: model_param(&model.params, &["G", "G0"]),
        len: model_param(&model.params, &["LEN", "LENGTH"]),
        alpha: model_param(&model.params, &["ALPHA"]),
        atten: model_param(&model.params, &["ATTEN", "ATTENDB", "LOSSDB"]),
    };

    let l = model_param(&model.params, &["L", "L0"]);
    let c = model_param(&model.params, &["C", "C0"]);
    let len = params.len;

    if params.z0.is_none() {
        if let (Some(l), Some(c)) = (l, c) {
            if l > 0.0 && c > 0.0 {
                params.z0 = Some((l / c).sqrt());
            }
        }
    }

    if params.td.is_none() {
        if let (Some(f), Some(nl)) = (params.freq, params.nl) {
            if f > 0.0 {
                params.td = Some(nl / f);
            }
        }
    }

    if params.td.is_none() {
        if let (Some(l), Some(c), Some(len)) = (l, c, len) {
            if l > 0.0 && c > 0.0 && len > 0.0 {
                params.td = Some(len * (l * c).sqrt());
            }
        }
    }

    Some(params)
}

fn tline_model_attenuation(params: TransmissionLineModelParams, z0: f64) -> Option<f64> {
    let len = params.len.unwrap_or(1.0).max(0.0);

    // Explicit alpha (Np/unit length) takes precedence.
    if let Some(alpha) = params.alpha {
        if alpha.is_finite() && alpha >= 0.0 {
            return Some((-alpha * len).exp());
        }
    }

    // ATTEN/ATTENDB: interpret <=1 as linear ratio, otherwise as dB.
    if let Some(atten) = params.atten {
        if atten.is_finite() && atten >= 0.0 {
            if atten <= 1.0 {
                return Some(atten);
            }
            let db_total = if params.len.is_some() {
                atten * len
            } else {
                atten
            };
            return Some(10_f64.powf(-db_total / 20.0));
        }
    }

    // Derive from primary RLGC line loss when available.
    let r = params.r.unwrap_or(0.0).max(0.0);
    let g = params.g.unwrap_or(0.0).max(0.0);
    if (r > 0.0 || g > 0.0) && z0.is_finite() && z0 > 0.0 {
        let alpha = r / (2.0 * z0) + g * z0 / 2.0;
        if alpha.is_finite() && alpha >= 0.0 {
            return Some((-alpha * len).exp());
        }
    }

    None
}

fn resolve_bjt_type_from_model(model_type: &str) -> Option<crate::netlist::BjtType> {
    if model_type.eq_ignore_ascii_case("NPN") {
        Some(crate::netlist::BjtType::Npn)
    } else if model_type.eq_ignore_ascii_case("PNP") {
        Some(crate::netlist::BjtType::Pnp)
    } else {
        None
    }
}

fn resolve_mos_type_from_model(model_type: &str) -> Option<crate::netlist::MosType> {
    if model_type.eq_ignore_ascii_case("NMOS") {
        Some(crate::netlist::MosType::Nmos)
    } else if model_type.eq_ignore_ascii_case("PMOS") {
        Some(crate::netlist::MosType::Pmos)
    } else {
        None
    }
}

fn resolve_jfet_type_from_model(model_type: &str) -> Option<crate::netlist::JfetType> {
    if model_type.eq_ignore_ascii_case("NJF") {
        Some(crate::netlist::JfetType::Njf)
    } else if model_type.eq_ignore_ascii_case("PJF") {
        Some(crate::netlist::JfetType::Pjf)
    } else {
        None
    }
}

fn resolve_mesfet_type_from_model(model_type: &str) -> Option<crate::netlist::MesfetType> {
    if model_type.eq_ignore_ascii_case("NMF") {
        Some(crate::netlist::MesfetType::Nmf)
    } else if model_type.eq_ignore_ascii_case("PMF") {
        Some(crate::netlist::MesfetType::Pmf)
    } else {
        None
    }
}

fn find_model_def<'a>(
    netlist: &'a Netlist,
    model_name: &str,
) -> Option<&'a crate::netlist::ModelDef> {
    netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name))
}

fn expected_model_type_text(expected_types: &[&str]) -> String {
    match expected_types {
        [] => String::new(),
        [single] => (*single).to_string(),
        [left, right] => format!("{left} or {right}"),
        _ => expected_types.join(", "),
    }
}

fn ensure_model_type(
    element_kind: &str,
    element_name: &str,
    model_name: &str,
    model_def: &crate::netlist::ModelDef,
    expected_types: &[&str],
) -> Result<(), SimulationError> {
    if expected_types
        .iter()
        .any(|kind| model_def.model_type.eq_ignore_ascii_case(kind))
    {
        return Ok(());
    }

    let expected = expected_model_type_text(expected_types);
    Err(SimulationError::Circuit(format!(
        "{} '{}' references model '{}' with incompatible type '{}'; expected {}",
        element_kind, element_name, model_name, model_def.model_type, expected
    )))
}

fn map_switch_state(state: crate::netlist::SwitchState) -> crate::device::SwitchState {
    match state {
        crate::netlist::SwitchState::On => crate::device::SwitchState::On,
        crate::netlist::SwitchState::Off => crate::device::SwitchState::Off,
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
                ElementKind::Resistor { value } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    circuit.resistors.add(element.name.clone(), np, nn, *value);
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
                // Jiles-Atherton hysteresis inductor - currently treated as standard inductor
                // TODO: Look up JA model parameters and create JilesAthertonInductor device
                ElementKind::JilesAthertonInductor {
                    value,
                    model: _,
                    initial_current: _,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    // For now, create standard inductor - full JA integration requires:
                    // 1. Looking up .MODEL CORE JA parameters from netlist
                    // 2. Creating JilesAthertonInductor device with hysteresis state
                    log::info!(
                        "Jiles-Atherton inductor {} created as linear inductor (JA model not yet integrated)",
                        element.name
                    );
                    circuit
                        .inductors
                        .add(element.name.clone(), np, nn, branch, *value);
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
                    if element.nodes.len() > 4 {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' has {} nodes; coupled/multiconductor P-lines are not yet supported",
                            element.name,
                            element.nodes.len()
                        )));
                    }
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

                    let mut tline = crate::device::TransmissionLine::new(
                        element.name.clone(),
                        p1p,
                        p1n,
                        p2p,
                        p2n,
                        z0_eff,
                        delay,
                    );
                    tline.freq = freq_eff;
                    tline.nl = nl_eff;
                    if let Some(att) = model_params.and_then(|p| tline_model_attenuation(p, z0_eff))
                    {
                        tline.set_attenuation(att);
                    }
                    circuit.tlines.push(tline);
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
mod veriloga_cache_tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "rspice_core_va_cache_{}_{}_{}",
            label,
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&dir).expect("failed to create temp directory");
        dir
    }

    fn dummy_model() -> rspice_veriloga::CompiledModel {
        rspice_veriloga::CompiledModel {
            name: "dummy".into(),
            num_terminals: 2,
            terminal_names: vec!["p".into(), "n".into()],
            parameters: vec![rspice_veriloga::codegen::CompiledParameter {
                name: "gain".into(),
                default: 1.0,
                min: Some(0.0),
                max: None,
            }],
            num_variables: 0,
            assignment_programs: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: 0,
            branch_currents: 0,
            laplace_filters: Vec::new(),
        }
    }

    #[test]
    fn test_dependency_fingerprint_invalidates_after_file_change() {
        let dir = create_temp_dir("invalidates");
        let file = dir.join("model.va");
        fs::write(&file, "module m; endmodule\n").expect("failed to write model file");

        let fingerprint =
            dependency_fingerprint(&file).expect("initial dependency fingerprint expected");
        assert!(dependency_matches_cached_fingerprint(&fingerprint));

        fs::write(&file, "module m; parameter real x=1; endmodule\n")
            .expect("failed to update model file");
        assert!(!dependency_matches_cached_fingerprint(&fingerprint));

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_fingerprint_paths_deduplicates_same_file() {
        let dir = create_temp_dir("dedup");
        let file = dir.join("model.va");
        fs::write(&file, "module m; endmodule\n").expect("failed to write model file");

        let canonical = file.canonicalize().expect("canonical path expected");
        let fingerprints = fingerprint_paths(&[file.clone(), canonical.clone()])
            .expect("fingerprints should succeed");
        assert_eq!(fingerprints.len(), 1);
        assert_eq!(fingerprints[0].canonical_path, canonical);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_cache_record_serialization_roundtrip() {
        let dir = create_temp_dir("serde");
        let file = dir.join("model.va");
        fs::write(&file, "module m; endmodule\n").expect("failed to write model file");
        let dep = dependency_fingerprint(&file).expect("dependency fingerprint expected");

        let record = VerilogADiskCacheRecord {
            version: VERILOGA_CACHE_RECORD_VERSION,
            source_path: file.canonicalize().expect("canonical path expected"),
            dependencies: vec![dep],
            model: dummy_model(),
        };

        let encoded =
            bincode::serialize(&record).expect("cache record should serialize successfully");
        let decoded: VerilogADiskCacheRecord =
            bincode::deserialize(&encoded).expect("cache record should deserialize");

        assert_eq!(decoded.version, VERILOGA_CACHE_RECORD_VERSION);
        assert_eq!(decoded.model.name.as_str(), "dummy");
        assert_eq!(decoded.dependencies.len(), 1);

        let _ = fs::remove_dir_all(dir);
    }
}
