use rspice_core::Value;
use rspice_core::xspice::ifspec::{
    IfSpec, IfSpecBounds, IfSpecDefault, IfSpecParameter, IfSpecPort, parse_ifspec,
};
use rspice_core::xspice::{CodeModelRegistry, ParamSpec, PortSpec, PortType};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const DEFAULT_MAX_ISSUES: usize = 200;

const RSPICE_ALIAS_MODEL_NAMES: &[&str] = &[
    "capacitor",
    "differentiator",
    "divider",
    "file_source",
    "icm_spice2poly",
    "inductor",
    "integrator",
    "r_to_v",
    "s_h",
    "seegenerator",
];

fn main() {
    if let Err(err) = run() {
        eprintln!("xspice ifspec audit error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse(std::env::args().skip(1))?;
    let specs = load_ifspecs(&args.ngspice_source_root)?;
    if specs.is_empty() {
        return Err(format!(
            "no ifspec.ifs files found under '{}'",
            args.ngspice_source_root.display()
        ));
    }

    let registry = CodeModelRegistry::with_builtins();
    let issues = audit_registry_against_ifspecs(&registry, &specs);
    print_report(&args.ngspice_source_root, &specs, &issues, args.max_issues);

    if issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} XSPICE metadata issue(s) found; fix the Rust metadata or intentionally document an alias/extension",
            issues.len()
        ))
    }
}

struct Args {
    ngspice_source_root: PathBuf,
    max_issues: usize,
}

impl Args {
    fn parse<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut ngspice_source_root = None;
        let mut max_issues = DEFAULT_MAX_ISSUES;

        while let Some(flag) = args.next() {
            match flag.as_str() {
                "--ngspice-source-root" => {
                    ngspice_source_root = Some(next_path(&mut args, &flag)?);
                }
                "--max-issues" => {
                    max_issues = next_parse(&mut args, &flag)?;
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => return Err(format!("unknown argument '{flag}'")),
            }
        }

        let ngspice_source_root = ngspice_source_root
            .or_else(|| std::env::var_os("NGSPICE_SOURCE_ROOT").map(PathBuf::from))
            .ok_or_else(|| "missing --ngspice-source-root or NGSPICE_SOURCE_ROOT".to_string())?;

        Ok(Self {
            ngspice_source_root,
            max_issues,
        })
    }
}

fn print_help() {
    println!(
        "Usage: xspice_ifspec_audit --ngspice-source-root <path> [--max-issues <n>]\n\
         Compares RSpice builtin XSPICE metadata against ngspice src/xspice/**/ifspec.ifs."
    );
}

fn next_path<I>(args: &mut I, flag: &str) -> Result<PathBuf, String>
where
    I: Iterator<Item = String>,
{
    args.next()
        .map(PathBuf::from)
        .ok_or_else(|| format!("missing value for {flag}"))
}

fn next_parse<I, T>(args: &mut I, flag: &str) -> Result<T, String>
where
    I: Iterator<Item = String>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    args.next()
        .ok_or_else(|| format!("missing value for {flag}"))?
        .parse::<T>()
        .map_err(|err| format!("invalid value for {flag}: {err}"))
}

#[derive(Debug)]
struct LocatedIfSpec {
    path: PathBuf,
    spec: IfSpec,
}

#[derive(Debug)]
struct AuditIssue {
    model: String,
    field: String,
    expected: String,
    actual: String,
}

fn load_ifspecs(source_root: &Path) -> Result<BTreeMap<String, LocatedIfSpec>, String> {
    let xspice_root = source_root.join("src").join("xspice");
    if !xspice_root.is_dir() {
        return Err(format!(
            "ngspice source root must contain src/xspice: '{}'",
            source_root.display()
        ));
    }

    let mut files = Vec::new();
    collect_ifspec_files(&xspice_root, &mut files)?;
    files.sort();

    let mut specs = BTreeMap::<String, LocatedIfSpec>::new();
    for path in files {
        let content = std::fs::read_to_string(&path)
            .map_err(|err| format!("failed to read '{}': {err}", path.display()))?;
        let spec = parse_ifspec(&content)
            .map_err(|err| format!("failed to parse '{}': {err}", path.display()))?;
        let key = spec.spice_model_name.to_ascii_lowercase();
        let located = LocatedIfSpec {
            path: path.clone(),
            spec,
        };
        match specs.get(&key) {
            Some(existing)
                if ifspec_source_priority(&located.path)
                    < ifspec_source_priority(&existing.path) =>
            {
                specs.insert(key, located);
            }
            None => {
                specs.insert(key, located);
            }
            _ => {}
        }
    }

    Ok(specs)
}

fn collect_ifspec_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in std::fs::read_dir(dir)
        .map_err(|err| format!("failed to read directory '{}': {err}", dir.display()))?
    {
        let entry =
            entry.map_err(|err| format!("failed to read entry in '{}': {err}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_ifspec_files(&path, files)?;
        } else if path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case("ifspec.ifs"))
        {
            files.push(path);
        }
    }
    Ok(())
}

fn ifspec_source_priority(path: &Path) -> usize {
    if path.components().any(|component| {
        component
            .as_os_str()
            .to_string_lossy()
            .eq_ignore_ascii_case("icm")
    }) {
        0
    } else {
        1
    }
}

fn audit_registry_against_ifspecs(
    registry: &CodeModelRegistry,
    specs: &BTreeMap<String, LocatedIfSpec>,
) -> Vec<AuditIssue> {
    let mut issues = Vec::new();

    for (name, located) in specs {
        let Some(model) = registry.get(name) else {
            issues.push(AuditIssue {
                model: name.clone(),
                field: "registry".to_string(),
                expected: format!("registered builtin for {}", located.path.display()),
                actual: "missing".to_string(),
            });
            continue;
        };

        compare_ports(name, &located.spec.ports, model.ports(), &mut issues);
        compare_parameters(
            name,
            &located.spec.parameters,
            model.parameters(),
            &mut issues,
        );
    }

    let official_names = specs.keys().cloned().collect::<BTreeSet<_>>();
    for name in registry.model_names() {
        let key = name.to_ascii_lowercase();
        if official_names.contains(&key)
            || RSPICE_ALIAS_MODEL_NAMES
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(name))
        {
            continue;
        }
        issues.push(AuditIssue {
            model: key,
            field: "registry".to_string(),
            expected: "ngspice ifspec model or documented RSpice alias".to_string(),
            actual: "extra undocumented builtin".to_string(),
        });
    }

    issues.sort_by(|left, right| {
        left.model
            .cmp(&right.model)
            .then_with(|| left.field.cmp(&right.field))
    });
    issues
}

fn compare_ports(
    model: &str,
    expected: &[IfSpecPort],
    actual: &[PortSpec],
    issues: &mut Vec<AuditIssue>,
) {
    if expected.len() != actual.len() {
        issues.push(issue(
            model,
            "ports.count",
            expected.len().to_string(),
            actual.len().to_string(),
        ));
        return;
    }

    for (index, (expected, actual)) in expected.iter().zip(actual).enumerate() {
        let prefix = format!("ports[{index}].{}", expected.name);
        compare_string(
            model,
            &format!("{prefix}.name"),
            &expected.name,
            &actual.name,
            issues,
        );
        compare_value(
            model,
            &format!("{prefix}.direction"),
            expected.direction,
            actual.direction,
            issues,
        );
        compare_value(
            model,
            &format!("{prefix}.default_type"),
            expected.default_type,
            actual.default_type,
            issues,
        );
        compare_port_type_sets(
            model,
            &format!("{prefix}.allowed_types"),
            &expected.allowed_types,
            &actual.allowed_types,
            issues,
        );
        compare_value(
            model,
            &format!("{prefix}.vector"),
            expected.is_vector,
            actual.is_vector,
            issues,
        );
        compare_value(
            model,
            &format!("{prefix}.null_allowed"),
            expected.null_allowed,
            actual.null_allowed,
            issues,
        );
        compare_option(
            model,
            &format!("{prefix}.vector_min_len"),
            expected.vector_bounds.min_usize(),
            actual.vector_min_len,
            issues,
        );
        compare_option(
            model,
            &format!("{prefix}.vector_max_len"),
            expected.vector_bounds.max_usize(),
            actual.vector_max_len,
            issues,
        );
    }
}

fn compare_parameters(
    model: &str,
    expected: &[IfSpecParameter],
    actual: &[ParamSpec],
    issues: &mut Vec<AuditIssue>,
) {
    let actual_by_name = actual
        .iter()
        .map(|param| (param.name.to_ascii_lowercase(), param))
        .collect::<BTreeMap<_, _>>();
    let expected_names = expected
        .iter()
        .map(|param| param.name.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();

    for expected in expected {
        let key = expected.name.to_ascii_lowercase();
        let Some(actual) = actual_by_name.get(&key).copied() else {
            issues.push(issue(
                model,
                &format!("params.{}", expected.name),
                "present".to_string(),
                "missing".to_string(),
            ));
            continue;
        };

        let prefix = format!("params.{}", expected.name);
        if let Some(expected_type) = expected.rspice_param_type() {
            compare_value(
                model,
                &format!("{prefix}.type"),
                expected_type,
                actual.param_type,
                issues,
            );
        }
        compare_value(
            model,
            &format!("{prefix}.required"),
            expected.required(),
            actual.required,
            issues,
        );
        compare_option_real(
            model,
            &format!("{prefix}.min"),
            expected.limits.min_real(),
            actual.min,
            issues,
        );
        compare_option_real(
            model,
            &format!("{prefix}.max"),
            expected.limits.max_real(),
            actual.max,
            issues,
        );
        if !matches!(expected.vector_bounds, IfSpecBounds::SameAs(_)) {
            compare_option(
                model,
                &format!("{prefix}.vector_min_len"),
                expected.vector_bounds.min_usize(),
                actual.vector_min_len,
                issues,
            );
            compare_option(
                model,
                &format!("{prefix}.vector_max_len"),
                expected.vector_bounds.max_usize(),
                actual.vector_max_len,
                issues,
            );
        }
        compare_default(model, &prefix, expected, actual, issues);
    }

    for actual in actual {
        let key = actual.name.to_ascii_lowercase();
        if expected_names.contains(&key) {
            continue;
        }
        issues.push(issue(
            model,
            &format!("params.{}", actual.name),
            "ngspice ifspec parameter".to_string(),
            "extra RSpice-only parameter".to_string(),
        ));
    }
}

fn compare_default(
    model: &str,
    prefix: &str,
    expected: &IfSpecParameter,
    actual: &ParamSpec,
    issues: &mut Vec<AuditIssue>,
) {
    match &expected.default {
        IfSpecDefault::None => {
            if expected.null_allowed {
                return;
            }
        }
        IfSpecDefault::Real(value) => {
            compare_real(
                model,
                &format!("{prefix}.default"),
                *value,
                actual.default,
                issues,
            );
        }
        IfSpecDefault::Integer(value) => {
            compare_real(
                model,
                &format!("{prefix}.default"),
                *value as Value,
                actual.default,
                issues,
            );
        }
        IfSpecDefault::Boolean(value) => {
            compare_real(
                model,
                &format!("{prefix}.default"),
                if *value { 1.0 } else { 0.0 },
                actual.default,
                issues,
            );
        }
        IfSpecDefault::String(value) => {
            compare_option_string(
                model,
                &format!("{prefix}.default"),
                Some(value.as_str()),
                actual.string_default.as_deref(),
                issues,
            );
        }
        IfSpecDefault::RealVector(values) => {
            compare_real_vec(
                model,
                &format!("{prefix}.default"),
                values,
                actual.real_vector_default.as_deref().unwrap_or_default(),
                issues,
            );
        }
        IfSpecDefault::IntegerVector(values) => {
            let actual_values = actual
                .integer_vector_default
                .as_deref()
                .unwrap_or_default()
                .to_vec();
            if *values != actual_values {
                issues.push(issue(
                    model,
                    &format!("{prefix}.default"),
                    format!("{values:?}"),
                    format!("{actual_values:?}"),
                ));
            }
        }
        IfSpecDefault::Complex(value) => {
            if actual.complex_default != Some(*value) {
                issues.push(issue(
                    model,
                    &format!("{prefix}.default"),
                    format!("{value:?}"),
                    format!("{:?}", actual.complex_default),
                ));
            }
        }
        IfSpecDefault::ComplexVector(values) => {
            let actual_values = actual.complex_vector_default.as_deref().unwrap_or_default();
            if values.as_slice() != actual_values {
                issues.push(issue(
                    model,
                    &format!("{prefix}.default"),
                    format!("{values:?}"),
                    format!("{actual_values:?}"),
                ));
            }
        }
        IfSpecDefault::StringVector(values) => {
            let actual_values = actual.string_vector_default.as_deref().unwrap_or_default();
            if values.as_slice() != actual_values {
                issues.push(issue(
                    model,
                    &format!("{prefix}.default"),
                    format!("{values:?}"),
                    format!("{actual_values:?}"),
                ));
            }
        }
    }
}

fn compare_string(
    model: &str,
    field: &str,
    expected: &str,
    actual: &str,
    issues: &mut Vec<AuditIssue>,
) {
    if !expected.eq_ignore_ascii_case(actual) {
        issues.push(issue(
            model,
            field,
            expected.to_string(),
            actual.to_string(),
        ));
    }
}

fn compare_value<T>(model: &str, field: &str, expected: T, actual: T, issues: &mut Vec<AuditIssue>)
where
    T: PartialEq + std::fmt::Debug,
{
    if expected != actual {
        issues.push(issue(
            model,
            field,
            format!("{expected:?}"),
            format!("{actual:?}"),
        ));
    }
}

fn compare_option<T>(
    model: &str,
    field: &str,
    expected: Option<T>,
    actual: Option<T>,
    issues: &mut Vec<AuditIssue>,
) where
    T: PartialEq + std::fmt::Debug,
{
    if expected != actual {
        issues.push(issue(
            model,
            field,
            format!("{expected:?}"),
            format!("{actual:?}"),
        ));
    }
}

fn compare_option_real(
    model: &str,
    field: &str,
    expected: Option<Value>,
    actual: Option<Value>,
    issues: &mut Vec<AuditIssue>,
) {
    match (expected, actual) {
        (Some(expected), Some(actual)) => compare_real(model, field, expected, actual, issues),
        _ if expected == actual => {}
        _ => issues.push(issue(
            model,
            field,
            format!("{expected:?}"),
            format!("{actual:?}"),
        )),
    }
}

fn compare_option_string(
    model: &str,
    field: &str,
    expected: Option<&str>,
    actual: Option<&str>,
    issues: &mut Vec<AuditIssue>,
) {
    if expected != actual {
        issues.push(issue(
            model,
            field,
            format!("{expected:?}"),
            format!("{actual:?}"),
        ));
    }
}

fn compare_real(
    model: &str,
    field: &str,
    expected: Value,
    actual: Value,
    issues: &mut Vec<AuditIssue>,
) {
    let scale = expected.abs().max(actual.abs()).max(1.0);
    if (expected - actual).abs() > scale * 1.0e-12 {
        issues.push(issue(
            model,
            field,
            format!("{expected:.17e}"),
            format!("{actual:.17e}"),
        ));
    }
}

fn compare_real_vec(
    model: &str,
    field: &str,
    expected: &[Value],
    actual: &[Value],
    issues: &mut Vec<AuditIssue>,
) {
    if expected.len() != actual.len() {
        issues.push(issue(
            model,
            field,
            format!("{expected:?}"),
            format!("{actual:?}"),
        ));
        return;
    }

    for (index, (expected, actual)) in expected.iter().zip(actual).enumerate() {
        compare_real(
            model,
            &format!("{field}[{index}]"),
            *expected,
            *actual,
            issues,
        );
    }
}

fn compare_port_type_sets(
    model: &str,
    field: &str,
    expected: &[PortType],
    actual: &[PortType],
    issues: &mut Vec<AuditIssue>,
) {
    let mut expected = expected
        .iter()
        .map(|value| format!("{value:?}"))
        .collect::<Vec<_>>();
    let mut actual = actual
        .iter()
        .map(|value| format!("{value:?}"))
        .collect::<Vec<_>>();
    expected.sort();
    actual.sort();
    if expected != actual {
        issues.push(issue(
            model,
            field,
            format!("{expected:?}"),
            format!("{actual:?}"),
        ));
    }
}

fn issue(
    model: &str,
    field: &str,
    expected: impl Into<String>,
    actual: impl Into<String>,
) -> AuditIssue {
    AuditIssue {
        model: model.to_string(),
        field: field.to_string(),
        expected: expected.into(),
        actual: actual.into(),
    }
}

fn print_report(
    source_root: &Path,
    specs: &BTreeMap<String, LocatedIfSpec>,
    issues: &[AuditIssue],
    max_issues: usize,
) {
    println!("XSPICE ifspec audit");
    println!("ngspice source root: {}", source_root.display());
    println!("official models parsed: {}", specs.len());
    println!("metadata issues: {}", issues.len());

    for issue in issues.iter().take(max_issues) {
        println!(
            "- {} {}: expected {}, actual {}",
            issue.model, issue.field, issue.expected, issue.actual
        );
    }
    if issues.len() > max_issues {
        println!(
            "... {} additional issue(s) omitted; rerun with --max-issues {} or higher",
            issues.len() - max_issues,
            issues.len()
        );
    }
}
