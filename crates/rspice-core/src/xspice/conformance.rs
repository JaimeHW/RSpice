//! XSPICE conformance helpers.
//!
//! These utilities compare RSpice's live code-model registry with the
//! authoritative metadata and examples in an ngspice source checkout. They are
//! intentionally reusable by tests, nightly jobs, and developer tooling.

use super::ifspec::{IfSpec, IfSpecDefault, IfSpecParamType, parse_ifspec};
use super::{CmContext, CmError, CodeModel, CodeModelRegistry, ParamSpec, ParamType, PortSpec};
use crate::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceIssue {
    pub severity: ConformanceSeverity,
    pub model: String,
    pub path: Option<PathBuf>,
    pub message: String,
}

impl ConformanceIssue {
    fn error(model: impl Into<String>, path: Option<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: ConformanceSeverity::Error,
            model: model.into(),
            path,
            message: message.into(),
        }
    }

    fn warning(
        model: impl Into<String>,
        path: Option<PathBuf>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: ConformanceSeverity::Warning,
            model: model.into(),
            path,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IfSpecConformanceReport {
    pub checked_models: usize,
    pub skipped_models: Vec<String>,
    pub issues: Vec<ConformanceIssue>,
}

impl IfSpecConformanceReport {
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ConformanceSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ConformanceSeverity::Warning)
            .count()
    }

    pub fn has_errors(&self) -> bool {
        self.error_count() != 0
    }
}

#[derive(Debug, Clone)]
pub struct IfSpecConformancePolicy {
    model_aliases: BTreeMap<String, String>,
    skipped_models: BTreeSet<String>,
}

impl IfSpecConformancePolicy {
    pub fn ngspice46() -> Self {
        let mut model_aliases = BTreeMap::new();
        for (ngspice, rspice) in [
            ("capacitor", "capacitoric"),
            ("file_source", "filesource"),
            ("icm_spice2poly", "spice2poly"),
            ("inductor", "inductoric"),
            ("seegenerator", "seegen"),
            ("table2d", "table2d"),
            ("table3d", "table3d"),
        ] {
            model_aliases.insert(ngspice.to_string(), rspice.to_string());
        }

        Self {
            model_aliases,
            skipped_models: BTreeSet::new(),
        }
    }

    pub fn rspice_model_name<'a>(&'a self, ngspice_model_name: &'a str) -> &'a str {
        let key = canonical_key(ngspice_model_name);
        self.model_aliases
            .get(&key)
            .map(String::as_str)
            .unwrap_or(ngspice_model_name)
    }

    pub fn should_skip_model(&self, name: &str) -> bool {
        self.skipped_models.contains(&canonical_key(name))
    }
}

impl Default for IfSpecConformancePolicy {
    fn default() -> Self {
        Self::ngspice46()
    }
}

#[derive(Debug)]
pub enum ConformanceError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        message: String,
    },
}

impl fmt::Display for ConformanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Parse { path, message } => write!(f, "{}: {message}", path.display()),
        }
    }
}

impl std::error::Error for ConformanceError {}

pub fn audit_ngspice_ifspec_tree(
    icm_root: &Path,
    registry: &CodeModelRegistry,
    policy: &IfSpecConformancePolicy,
) -> Result<IfSpecConformanceReport, ConformanceError> {
    let mut report = IfSpecConformanceReport::default();
    let mut paths = Vec::new();
    collect_ifspec_paths(icm_root, &mut paths)?;
    paths.sort();

    for path in paths {
        let source = read_to_string(&path)?;
        let spec = parse_ifspec(&source).map_err(|err| ConformanceError::Parse {
            path: path.clone(),
            message: err.to_string(),
        })?;
        if policy.should_skip_model(&spec.spice_model_name) {
            report.skipped_models.push(spec.spice_model_name);
            continue;
        }

        report.checked_models += 1;
        audit_model_ifspec(&spec, Some(path), registry, policy, &mut report);
    }

    Ok(report)
}

pub fn audit_model_ifspec(
    spec: &IfSpec,
    path: Option<PathBuf>,
    registry: &CodeModelRegistry,
    policy: &IfSpecConformancePolicy,
    report: &mut IfSpecConformanceReport,
) {
    let rspice_name = policy.rspice_model_name(&spec.spice_model_name);
    let Some(model) = registry.get(rspice_name) else {
        report.issues.push(ConformanceIssue::error(
            &spec.spice_model_name,
            path,
            format!("registered RSpice model '{rspice_name}' is missing"),
        ));
        return;
    };

    compare_ports(spec, model.ports(), path.as_ref(), report);
    compare_parameters(spec, model.parameters(), path.as_ref(), report);
}

fn compare_ports(
    spec: &IfSpec,
    rspice_ports: &[PortSpec],
    path: Option<&PathBuf>,
    report: &mut IfSpecConformanceReport,
) {
    let rspice_by_name = rspice_ports
        .iter()
        .map(|port| (canonical_key(&port.name), port))
        .collect::<BTreeMap<_, _>>();

    for ng_port in &spec.ports {
        let path = path.cloned();
        let Some(rspice_port) = rspice_by_name.get(&canonical_key(&ng_port.name)) else {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path,
                format!("port '{}' is missing from RSpice metadata", ng_port.name),
            ));
            continue;
        };

        if rspice_port.direction != ng_port.direction {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path.clone(),
                format!(
                    "port '{}' direction mismatch: ngspice {:?}, RSpice {:?}",
                    ng_port.name, ng_port.direction, rspice_port.direction
                ),
            ));
        }
        if rspice_port.default_type != ng_port.default_type {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path.clone(),
                format!(
                    "port '{}' default type mismatch: ngspice {:?}, RSpice {:?}",
                    ng_port.name, ng_port.default_type, rspice_port.default_type
                ),
            ));
        }

        let ng_allowed = port_type_set(&ng_port.allowed_types);
        let rspice_allowed = port_type_set(&rspice_port.allowed_types);
        if rspice_allowed != ng_allowed {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path.clone(),
                format!(
                    "port '{}' allowed types mismatch: ngspice {:?}, RSpice {:?}",
                    ng_port.name, ng_allowed, rspice_allowed
                ),
            ));
        }

        if rspice_port.is_vector != ng_port.is_vector {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path.clone(),
                format!(
                    "port '{}' vector flag mismatch: ngspice {}, RSpice {}",
                    ng_port.name, ng_port.is_vector, rspice_port.is_vector
                ),
            ));
        }
        if rspice_port.null_allowed != ng_port.null_allowed {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path.clone(),
                format!(
                    "port '{}' nullability mismatch: ngspice {}, RSpice {}",
                    ng_port.name, ng_port.null_allowed, rspice_port.null_allowed
                ),
            ));
        }
        compare_usize_bound(
            &spec.spice_model_name,
            &format!("port '{}' vector minimum", ng_port.name),
            ng_port.vector_bounds.min_usize(),
            rspice_port.vector_min_len,
            path.clone(),
            report,
        );
        compare_usize_bound(
            &spec.spice_model_name,
            &format!("port '{}' vector maximum", ng_port.name),
            ng_port.vector_bounds.max_usize(),
            rspice_port.vector_max_len,
            path,
            report,
        );
    }
}

fn compare_parameters(
    spec: &IfSpec,
    rspice_params: &[ParamSpec],
    path: Option<&PathBuf>,
    report: &mut IfSpecConformanceReport,
) {
    let rspice_by_name = rspice_params
        .iter()
        .map(|param| (canonical_key(&param.name), param))
        .collect::<BTreeMap<_, _>>();

    for ng_param in &spec.parameters {
        if ng_param.param_type == IfSpecParamType::Pointer {
            continue;
        }

        let path = path.cloned();
        let Some(rspice_param) = rspice_by_name.get(&canonical_key(&ng_param.name)) else {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path,
                format!(
                    "parameter '{}' is missing from RSpice metadata",
                    ng_param.name
                ),
            ));
            continue;
        };

        if let Some(ng_type) = ng_param.rspice_param_type()
            && rspice_param.param_type != ng_type
        {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path.clone(),
                format!(
                    "parameter '{}' type mismatch: ngspice {:?}, RSpice {:?}",
                    ng_param.name, ng_type, rspice_param.param_type
                ),
            ));
        }

        if rspice_param.required != ng_param.required() {
            report.issues.push(ConformanceIssue::error(
                &spec.spice_model_name,
                path.clone(),
                format!(
                    "parameter '{}' required flag mismatch: ngspice {}, RSpice {}",
                    ng_param.name,
                    ng_param.required(),
                    rspice_param.required
                ),
            ));
        }

        compare_usize_bound(
            &spec.spice_model_name,
            &format!("parameter '{}' vector minimum", ng_param.name),
            ng_param.vector_bounds.min_usize(),
            rspice_param.vector_min_len,
            path.clone(),
            report,
        );
        compare_usize_bound(
            &spec.spice_model_name,
            &format!("parameter '{}' vector maximum", ng_param.name),
            ng_param.vector_bounds.max_usize(),
            rspice_param.vector_max_len,
            path.clone(),
            report,
        );

        compare_parameter_default(spec, ng_param, rspice_param, path.clone(), report);
        compare_numeric_limit(
            &spec.spice_model_name,
            &format!("parameter '{}' minimum", ng_param.name),
            ng_param.limits.min_real(),
            rspice_param.min,
            path.clone(),
            report,
        );
        compare_numeric_limit(
            &spec.spice_model_name,
            &format!("parameter '{}' maximum", ng_param.name),
            ng_param.limits.max_real(),
            rspice_param.max,
            path,
            report,
        );
    }
}

fn compare_usize_bound(
    model: &str,
    label: &str,
    expected: Option<usize>,
    actual: Option<usize>,
    path: Option<PathBuf>,
    report: &mut IfSpecConformanceReport,
) {
    if expected != actual {
        report.issues.push(ConformanceIssue::error(
            model,
            path,
            format!("{label} mismatch: ngspice {expected:?}, RSpice {actual:?}"),
        ));
    }
}

fn compare_numeric_limit(
    model: &str,
    label: &str,
    expected: Option<Value>,
    actual: Option<Value>,
    path: Option<PathBuf>,
    report: &mut IfSpecConformanceReport,
) {
    if !same_optional_value(expected, actual) {
        report.issues.push(ConformanceIssue::warning(
            model,
            path,
            format!("{label} differs: ngspice {expected:?}, RSpice {actual:?}"),
        ));
    }
}

fn compare_parameter_default(
    spec: &IfSpec,
    ng_param: &super::ifspec::IfSpecParameter,
    rspice_param: &ParamSpec,
    path: Option<PathBuf>,
    report: &mut IfSpecConformanceReport,
) {
    let differs = match &ng_param.default {
        IfSpecDefault::None => false,
        IfSpecDefault::Real(value) => !same_value(*value, rspice_param.default),
        IfSpecDefault::Integer(value) => !same_value(*value as Value, rspice_param.default),
        IfSpecDefault::Boolean(value) => {
            !same_value(if *value { 1.0 } else { 0.0 }, rspice_param.default)
        }
        IfSpecDefault::Complex(value) => rspice_param.complex_default != Some(*value),
        IfSpecDefault::String(value) => rspice_param.string_default.as_deref() != Some(value),
        IfSpecDefault::RealVector(values) => {
            rspice_param.real_vector_default.as_deref() != Some(values.as_slice())
        }
        IfSpecDefault::IntegerVector(values) => {
            rspice_param.integer_vector_default.as_deref() != Some(values.as_slice())
        }
        IfSpecDefault::ComplexVector(values) => {
            rspice_param.complex_vector_default.as_deref() != Some(values.as_slice())
        }
        IfSpecDefault::StringVector(values) => {
            rspice_param.string_vector_default.as_deref() != Some(values.as_slice())
        }
    };

    if differs {
        report.issues.push(ConformanceIssue::warning(
            &spec.spice_model_name,
            path,
            format!(
                "parameter '{}' default differs from ngspice ifspec",
                ng_param.name
            ),
        ));
    }
}

fn port_type_set(types: &[super::PortType]) -> BTreeSet<String> {
    types.iter().map(|ty| format!("{ty:?}")).collect()
}

fn same_optional_value(left: Option<Value>, right: Option<Value>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => same_value(left, right),
        (None, None) => true,
        _ => false,
    }
}

fn same_value(left: Value, right: Value) -> bool {
    if left == right {
        return true;
    }
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= f64::EPSILON * 16.0 * scale
}

fn collect_ifspec_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), ConformanceError> {
    let entries = fs::read_dir(dir).map_err(|source| ConformanceError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| ConformanceError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_ifspec_paths(&path, paths)?;
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case("ifspec.ifs"))
        {
            paths.push(path);
        }
    }
    Ok(())
}

fn read_to_string(path: &Path) -> Result<String, ConformanceError> {
    fs::read_to_string(path).map_err(|source| ConformanceError::Io {
        path: path.to_path_buf(),
        source,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XspiceExampleDisposition {
    Runnable,
    ScriptedControl,
    ReusableInclude,
    ReusableSubcircuit,
    ExcludedThirdParty { reason: String },
    NeedsAdjudication { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XspiceExampleDeck {
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub disposition: XspiceExampleDisposition,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct XspiceExampleCorpusReport {
    pub decks: Vec<XspiceExampleDeck>,
}

impl XspiceExampleCorpusReport {
    pub fn runnable_count(&self) -> usize {
        self.decks
            .iter()
            .filter(|deck| deck.disposition == XspiceExampleDisposition::Runnable)
            .count()
    }

    pub fn scripted_control_count(&self) -> usize {
        self.decks
            .iter()
            .filter(|deck| deck.disposition == XspiceExampleDisposition::ScriptedControl)
            .count()
    }

    pub fn excluded_count(&self) -> usize {
        self.decks
            .iter()
            .filter(|deck| {
                matches!(
                    deck.disposition,
                    XspiceExampleDisposition::ExcludedThirdParty { .. }
                )
            })
            .count()
    }

    pub fn reusable_fragment_count(&self) -> usize {
        self.decks
            .iter()
            .filter(|deck| {
                matches!(
                    deck.disposition,
                    XspiceExampleDisposition::ReusableInclude
                        | XspiceExampleDisposition::ReusableSubcircuit
                )
            })
            .count()
    }

    pub fn needs_adjudication(&self) -> Vec<&XspiceExampleDeck> {
        self.decks
            .iter()
            .filter(|deck| {
                matches!(
                    deck.disposition,
                    XspiceExampleDisposition::NeedsAdjudication { .. }
                )
            })
            .collect()
    }
}

pub fn audit_ngspice_xspice_examples(
    ngspice_source_root: &Path,
) -> Result<XspiceExampleCorpusReport, ConformanceError> {
    let examples_root = ngspice_source_root.join("examples").join("xspice");
    let mut paths = Vec::new();
    collect_example_decks(&examples_root, &mut paths)?;
    paths.sort();

    let mut report = XspiceExampleCorpusReport::default();
    for path in paths {
        let source = read_to_string(&path)?;
        let relative_path = path
            .strip_prefix(&examples_root)
            .unwrap_or(&path)
            .to_path_buf();
        let disposition = classify_xspice_example(&relative_path, &source);
        report.decks.push(XspiceExampleDeck {
            path,
            relative_path,
            disposition,
        });
    }
    Ok(report)
}

fn collect_example_decks(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), ConformanceError> {
    let entries = fs::read_dir(dir).map_err(|source| ConformanceError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| ConformanceError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_example_decks(&path, paths)?;
        } else if is_example_deck(&path) {
            paths.push(path);
        }
    }
    Ok(())
}

fn is_example_deck(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .is_some_and(|ext| matches!(ext.as_str(), "cir" | "deck" | "net" | "spi"))
}

fn classify_xspice_example(relative_path: &Path, source: &str) -> XspiceExampleDisposition {
    if let Some(reason) = third_party_example_reason(relative_path, source) {
        return XspiceExampleDisposition::ExcludedThirdParty { reason };
    }
    if has_control_block(source) {
        return XspiceExampleDisposition::ScriptedControl;
    }
    if has_supported_analysis_directive(source) {
        return XspiceExampleDisposition::Runnable;
    }
    if has_subcircuit_definition(source) {
        return XspiceExampleDisposition::ReusableSubcircuit;
    }
    if has_include_directive(source) {
        return XspiceExampleDisposition::ReusableInclude;
    }
    XspiceExampleDisposition::NeedsAdjudication {
        reason: "no supported analysis directive or .control block".to_string(),
    }
}

fn third_party_example_reason(relative_path: &Path, source: &str) -> Option<String> {
    let normalized_path = relative_path
        .iter()
        .map(|part| part.to_string_lossy().to_ascii_lowercase())
        .collect::<Vec<_>>();
    for excluded in ["ghdl", "icarus_verilog", "verilator", "d_process"] {
        if normalized_path.iter().any(|part| part == excluded) {
            return Some(format!("requires third-party runtime '{excluded}'"));
        }
    }

    let normalized_source = source.to_ascii_lowercase();
    for token in [
        "d_cosim",
        "d_process",
        "verilog",
        "vhdl",
        "ghdl",
        "iverilog",
        "verilator",
    ] {
        if normalized_source.contains(token) {
            return Some(format!("references third-party runtime token '{token}'"));
        }
    }
    None
}

fn has_control_block(source: &str) -> bool {
    source.lines().any(|line| {
        let line = strip_inline_comment(line).trim();
        line.eq_ignore_ascii_case(".control")
    })
}

fn has_supported_analysis_directive(source: &str) -> bool {
    source.lines().any(|line| {
        let line = strip_inline_comment(line).trim().to_ascii_lowercase();
        matches!(
            line.split_whitespace().next(),
            Some(".ac" | ".dc" | ".op" | ".sp" | ".tran" | ".tf" | ".noise" | ".pz" | ".sens")
        )
    })
}

fn has_subcircuit_definition(source: &str) -> bool {
    source.lines().any(|line| {
        strip_inline_comment(line)
            .trim()
            .to_ascii_lowercase()
            .starts_with(".subckt")
    })
}

fn has_include_directive(source: &str) -> bool {
    source.lines().any(|line| {
        strip_inline_comment(line)
            .trim()
            .to_ascii_lowercase()
            .starts_with(".include")
    })
}

fn strip_inline_comment(line: &str) -> &str {
    line.split_once(';').map(|(head, _)| head).unwrap_or(line)
}

#[derive(Debug, Clone, Copy)]
pub struct PartialVerificationOptions {
    pub step: Value,
    pub absolute_tolerance: Value,
    pub relative_tolerance: Value,
}

impl Default for PartialVerificationOptions {
    fn default() -> Self {
        Self {
            step: 1.0e-6,
            absolute_tolerance: 1.0e-6,
            relative_tolerance: 1.0e-4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartialMismatch {
    pub model: String,
    pub output_port: String,
    pub input_port: String,
    pub input_index: Option<usize>,
    pub analytic: Value,
    pub numeric: Value,
    pub abs_error: Value,
    pub rel_error: Value,
}

pub fn verify_model_partials(
    model: &dyn CodeModel,
    ctx: &CmContext,
    options: PartialVerificationOptions,
) -> Result<Vec<PartialMismatch>, CmError> {
    let mut base = ctx.clone();
    model.init(&mut base)?;
    model.evaluate(&mut base)?;

    let mut mismatches = Vec::new();
    for output in model.ports().iter().filter(|port| {
        port.direction == super::PortDirection::Out
            && port.default_type.is_analog()
            && !port.is_vector
    }) {
        let analytic_scalars = model
            .output_input_partials(&base, &output.name)
            .into_iter()
            .map(|(name, value)| ((canonical_key(&name), None::<usize>), value))
            .collect::<BTreeMap<_, _>>();
        let analytic_vectors = model
            .output_input_vector_partials(&base, &output.name)
            .into_iter()
            .map(|(name, index, value)| ((canonical_key(&name), Some(index)), value))
            .collect::<BTreeMap<_, _>>();

        for input in model.ports().iter().filter(|port| {
            matches!(
                port.direction,
                super::PortDirection::In | super::PortDirection::InOut
            ) && port.default_type.is_analog()
        }) {
            if input.is_vector {
                let width = ctx.port_width(&input.name).max(1);
                for index in 0..width {
                    let numeric = finite_difference_vector_input(
                        model,
                        ctx,
                        &output.name,
                        &input.name,
                        index,
                        options.step,
                    )?;
                    let analytic = analytic_vectors
                        .get(&(canonical_key(&input.name), Some(index)))
                        .copied()
                        .unwrap_or(0.0);
                    push_partial_mismatch_if_needed(
                        model.name(),
                        &output.name,
                        &input.name,
                        Some(index),
                        analytic,
                        numeric,
                        options,
                        &mut mismatches,
                    );
                }
            } else {
                let numeric = finite_difference_scalar_input(
                    model,
                    ctx,
                    &output.name,
                    &input.name,
                    options.step,
                )?;
                let analytic = analytic_scalars
                    .get(&(canonical_key(&input.name), None))
                    .copied()
                    .unwrap_or(0.0);
                push_partial_mismatch_if_needed(
                    model.name(),
                    &output.name,
                    &input.name,
                    None,
                    analytic,
                    numeric,
                    options,
                    &mut mismatches,
                );
            }
        }
    }

    Ok(mismatches)
}

pub fn context_with_model_defaults(model: &dyn CodeModel) -> CmContext {
    let mut ctx = CmContext::new();
    for port in model.ports() {
        if port.is_vector {
            if let Some(min_len) = port.vector_min_len {
                ctx.set_port_width(&port.name, min_len.max(1));
            }
        }
    }
    for param in model.parameters() {
        apply_param_default(&mut ctx, param);
    }
    ctx
}

fn finite_difference_scalar_input(
    model: &dyn CodeModel,
    ctx: &CmContext,
    output: &str,
    input: &str,
    step: Value,
) -> Result<Value, CmError> {
    let current = ctx.input(input);
    let mut plus = ctx.clone();
    plus.set_input_analog(input, current + step);
    model.init(&mut plus)?;
    model.evaluate(&mut plus)?;

    let mut minus = ctx.clone();
    minus.set_input_analog(input, current - step);
    model.init(&mut minus)?;
    model.evaluate(&mut minus)?;

    Ok((plus.output(output) - minus.output(output)) / (2.0 * step))
}

fn finite_difference_vector_input(
    model: &dyn CodeModel,
    ctx: &CmContext,
    output: &str,
    input: &str,
    index: usize,
    step: Value,
) -> Result<Value, CmError> {
    let mut values = ctx.input_vector(input);
    let width = ctx.port_width(input).max(index + 1);
    values.resize(width, 0.0);

    let mut plus = ctx.clone();
    let mut plus_values = values.clone();
    plus_values[index] += step;
    plus.set_input_analog_vector_from_fn(input, width, |idx| {
        super::AnalogValue::new(plus_values[idx])
    })?;
    model.init(&mut plus)?;
    model.evaluate(&mut plus)?;

    let mut minus = ctx.clone();
    let mut minus_values = values;
    minus_values[index] -= step;
    minus.set_input_analog_vector_from_fn(input, width, |idx| {
        super::AnalogValue::new(minus_values[idx])
    })?;
    model.init(&mut minus)?;
    model.evaluate(&mut minus)?;

    Ok((plus.output(output) - minus.output(output)) / (2.0 * step))
}

fn push_partial_mismatch_if_needed(
    model: &str,
    output_port: &str,
    input_port: &str,
    input_index: Option<usize>,
    analytic: Value,
    numeric: Value,
    options: PartialVerificationOptions,
    mismatches: &mut Vec<PartialMismatch>,
) {
    let abs_error = (analytic - numeric).abs();
    let scale = analytic.abs().max(numeric.abs()).max(1.0);
    let rel_error = abs_error / scale;
    if abs_error > options.absolute_tolerance && rel_error > options.relative_tolerance {
        mismatches.push(PartialMismatch {
            model: model.to_string(),
            output_port: output_port.to_string(),
            input_port: input_port.to_string(),
            input_index,
            analytic,
            numeric,
            abs_error,
            rel_error,
        });
    }
}

fn apply_param_default(ctx: &mut CmContext, param: &ParamSpec) {
    match param.param_type {
        ParamType::Real | ParamType::Integer | ParamType::Boolean => {
            ctx.set_param(&param.name, param.default);
        }
        ParamType::Complex => {
            if let Some(value) = param.complex_default {
                ctx.set_complex_param(&param.name, value);
            }
        }
        ParamType::String => {
            if let Some(value) = &param.string_default {
                ctx.set_string_param(&param.name, value);
            }
        }
        ParamType::StringVector => {
            if let Some(value) = &param.string_vector_default {
                ctx.set_string_vector_param(&param.name, value.clone());
            }
        }
        ParamType::RealVector => {
            if let Some(value) = &param.real_vector_default {
                ctx.set_real_vector_param(&param.name, value.clone());
            }
        }
        ParamType::IntegerVector => {
            if let Some(value) = &param.integer_vector_default {
                ctx.set_integer_vector_param(&param.name, value.clone());
            }
        }
        ParamType::ComplexVector => {
            if let Some(value) = &param.complex_vector_default {
                ctx.set_complex_vector_param(&param.name, value.clone());
            }
        }
    }
}

fn canonical_key(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}
