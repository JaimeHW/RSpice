//! Compare Command - Golden file regression testing
//!
//! Compares simulation results against reference files for CI/CD testing.
//! Both files may be in any supported result format (rawfile, CSV, TSV,
//! JSON, HDF5 — auto-detected by extension), with configurable absolute
//! and relative tolerances.

use crate::cli::{CliError, Config, OutputFormat, map_atomic_output_error};
use crate::commands::export_table::ColumnData;
use crate::commands::publish;
use crate::commands::waveform_io::{
    ImportedResult, detect_format, load_result_selected, supports_sections,
};

mod fft;
mod interpolation;
use std::collections::HashSet;
use std::path::PathBuf;

/// Arguments for the compare command
#[derive(Debug, Clone)]
pub struct CompareArgs {
    pub section: Option<String>,
    /// Result file to compare
    pub result: PathBuf,
    /// Golden (reference) file
    pub golden: PathBuf,
    /// Absolute tolerance
    pub abstol: f64,
    /// Relative tolerance
    pub reltol: f64,
    /// Output format for differences
    pub format: OutputFormat,
    /// Variables to compare (empty = all)
    pub variables: Vec<String>,
    /// Fail on first difference (vs. report all)
    pub fail_fast: bool,
    /// Tolerate point-count mismatches (compare the overlap only)
    pub allow_truncated: bool,
    /// Tolerate golden variables that are missing from the result
    pub ignore_missing: bool,
    /// On mismatch (or missing golden), copy the result over the golden file
    pub bless: bool,
    /// Resample the result onto the golden file's scale before comparing
    pub interpolate: bool,
}

impl Default for CompareArgs {
    fn default() -> Self {
        Self {
            section: None,
            result: PathBuf::new(),
            golden: PathBuf::new(),
            abstol: 1e-9,
            reltol: 1e-6,
            format: OutputFormat::Raw,
            variables: vec![],
            fail_fast: false,
            allow_truncated: false,
            ignore_missing: false,
            bless: false,
            interpolate: false,
        }
    }
}

/// Comparison result
#[derive(Debug)]
pub struct CompareResult {
    /// Whether comparison passed
    pub passed: bool,
    /// Number of variables compared
    pub num_variables: usize,
    /// Number of points compared
    pub num_points: usize,
    /// Maximum absolute difference found
    pub max_abs_diff: f64,
    /// Maximum relative difference found
    pub max_rel_diff: f64,
    /// Variable with maximum difference
    pub max_diff_variable: String,
    /// Differences found
    pub differences: Vec<Difference>,
    /// Structural failures: missing variables, point-count mismatches.
    /// These fail the comparison even when every overlapping value matches,
    /// so a truncated result cannot pass against a longer golden file.
    pub problems: Vec<String>,
}

/// A single difference between result and golden
#[derive(Debug)]
pub struct Difference {
    /// Variable name
    pub variable: String,
    /// Point index
    pub index: usize,
    /// Result value
    pub result_value: f64,
    /// Golden value
    pub golden_value: f64,
    /// Absolute difference
    pub abs_diff: f64,
    /// Relative difference
    pub rel_diff: f64,
}

/// Execute the compare command
pub fn execute(
    args: CompareArgs,
    config: &Config,
    _verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    validate_compare_tolerance("--abstol", args.abstol)?;
    validate_compare_tolerance("--reltol", args.reltol)?;
    if args.bless {
        publish::destinations::protect_sources(&args.golden, config.source_paths())?;
    }
    if args.section.is_some()
        && !supports_sections(detect_format(&args.result))
        && !supports_sections(detect_format(&args.golden))
    {
        return Err(CliError::InvalidArgument {
            message: "--section requires at least one RAW, HDF5 or Touchstone input".into(),
            suggestion: Some("omit --section when comparing single-document formats".into()),
        });
    }
    let quiet = quiet || args.format == OutputFormat::Json;

    // Validate files exist
    if !args.result.exists() {
        return Err(CliError::InputNotFound {
            path: args.result.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "Result file not found"),
        });
    }
    if !args.golden.exists() {
        if args.bless {
            // Missing-golden bootstrap is still a promotion of a result
            // artifact. Validate the result before copying so malformed CSV,
            // JSON, RAW, etc. cannot become the accepted baseline.
            let data = load_comparison_data(
                &args.result,
                config.resources.limits(),
                args.section.as_deref(),
            )?;
            let mut comparison = validate_bless_candidate(&data, &args)?;
            bless_golden(&args.result, &args.golden, quiet, "no golden file yet")?;
            if args.format == OutputFormat::Json {
                comparison.passed = false;
                comparison
                    .problems
                    .push("golden file did not exist".to_string());
                output_json(&comparison, true, args.section.as_deref());
            }
            return Ok(());
        }
        return Err(CliError::InputNotFound {
            path: args.golden.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "Golden file not found"),
        });
    }

    if !quiet {
        println!(
            "Comparing: {} vs {}",
            args.result.display(),
            args.golden.display()
        );
        println!(
            "  Tolerances: abstol={:.2e}, reltol={:.2e}",
            args.abstol, args.reltol
        );
    }

    // Load and parse files
    let result_data = load_comparison_data(
        &args.result,
        config.resources.limits(),
        args.section.as_deref(),
    )?;
    let golden_data = load_comparison_data(
        &args.golden,
        config.resources.limits(),
        args.section.as_deref(),
    )?;

    // Perform comparison
    let cmp_result = compare_data(&result_data, &golden_data, &args)?;

    let blessed = !cmp_result.passed && args.bless;
    if blessed {
        validate_bless_candidate(&result_data, &args)?;
    }

    // Output results. JSON reports the final command outcome, so bless first
    // and only then emit a machine-readable accepted/blessed status.
    if args.format == OutputFormat::Json {
        if blessed {
            bless_golden(&args.result, &args.golden, quiet, "differences accepted")?;
        }
        output_json(&cmp_result, blessed, args.section.as_deref());
    } else {
        output_text(&cmp_result, quiet);
        if blessed {
            bless_golden(&args.result, &args.golden, quiet, "differences accepted")?;
        }
    }

    if cmp_result.passed || blessed {
        Ok(())
    } else {
        let mut parts = Vec::new();
        if !cmp_result.differences.is_empty() {
            parts.push(format!(
                "{} value difference(s), max {:.2e} ({})",
                cmp_result.differences.len(),
                cmp_result.max_abs_diff,
                cmp_result.max_diff_variable
            ));
        }
        if !cmp_result.problems.is_empty() {
            parts.push(cmp_result.problems.join("; "));
        }
        Err(CliError::VerificationFailed {
            message: format!("comparison failed: {}", parts.join("; ")),
        })
    }
}

fn validate_compare_tolerance(name: &str, value: f64) -> Result<(), CliError> {
    if !value.is_finite() || value < 0.0 {
        return Err(CliError::InvalidArgument {
            message: format!("{name} must be a finite non-negative tolerance, got {value}"),
            suggestion: Some(
                "Use 0 for an exact comparison, or a positive SPICE value".to_string(),
            ),
        });
    }
    Ok(())
}

/// Promote the result file to the new golden reference.
fn bless_golden(
    result: &std::path::Path,
    golden: &std::path::Path,
    quiet: bool,
    why: &str,
) -> Result<(), CliError> {
    if detect_format(result) != detect_format(golden) {
        return Err(CliError::InvalidArgument {
            message: "--bless requires the result and golden to use the same file format"
                .to_string(),
            suggestion: Some(
                "convert the result to the golden's format before blessing it".to_string(),
            ),
        });
    }
    let mut source = std::fs::File::open(result).map_err(|source| CliError::InputReadError {
        path: result.to_path_buf(),
        source,
    })?;
    publish::artifact(golden, |writer| {
        std::io::copy(&mut source, writer)
            .map(|_| ())
            .map_err(|source| CliError::output_error(golden, source))
    })
    .map_err(|error| map_atomic_output_error(golden, error))?;
    if !quiet {
        println!("✓ Golden updated ({}): {}", why, golden.display());
    }
    Ok(())
}

enum ComparisonData {
    Waveform(WaveformData),
    Fft(Box<crate::commands::run::FftBundle>),
}

fn compare_data(
    result: &ComparisonData,
    golden: &ComparisonData,
    args: &CompareArgs,
) -> Result<CompareResult, CliError> {
    if args.interpolate
        && !matches!(
            (result, golden),
            (ComparisonData::Waveform(_), ComparisonData::Waveform(_))
        )
    {
        return Err(CliError::InvalidArgument {
            message: "FFT comparison requires matching discrete transform grids; --interpolate is only available for waveforms".into(),
            suggestion: None,
        });
    }
    match (result, golden) {
        (ComparisonData::Waveform(result), ComparisonData::Waveform(golden)) => {
            compare_waveforms(result, golden, args)
        }
        (ComparisonData::Fft(result), ComparisonData::Fft(golden)) => {
            fft::compare(result, golden, args)
        }
        _ => Err(CliError::VerificationFailed {
            message: "result kinds differ: a typed FFT cannot be compared to a waveform table"
                .into(),
        }),
    }
}

/// A blessed baseline must pass the same selection when compared to itself.
/// Missing or ambiguous requested probes cannot become an accepted reference.
fn validate_bless_candidate(
    candidate: &ComparisonData,
    args: &CompareArgs,
) -> Result<CompareResult, CliError> {
    let comparison = compare_data(candidate, candidate, args)?;
    if !comparison.passed {
        return Err(CliError::VerificationFailed {
            message: format!(
                "cannot bless a result that does not satisfy the requested comparison: {}",
                comparison.problems.join("; ")
            ),
        });
    }
    Ok(comparison)
}

/// Waveform data structure for comparison
struct WaveformData {
    variables: Vec<String>,
    variable_types: Vec<String>,
    units: Vec<Option<String>>,
    values: Vec<Vec<f64>>,
}

fn quantity_type(value: &str) -> Option<String> {
    if matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "" | "value" | "unknown" | "parameter"
    ) {
        return None;
    }
    // Known quantity aliases share one vocabulary with legacy unit inference.
    // Unknown types may themselves be SI symbols, so preserve their case.
    Some(
        crate::commands::export_table::type_unit(value)
            .unwrap_or(value.trim())
            .to_owned(),
    )
}

fn types_compatible(left: &str, right: &str) -> bool {
    match (quantity_type(left), quantity_type(right)) {
        (Some(left), Some(right)) => left == right,
        // Legacy table formats cannot declare every quantity. Unknown is
        // allowed, but two explicitly incompatible quantities never match.
        _ => true,
    }
}

fn units_compatible(left: &WaveformData, i: usize, right: &WaveformData, j: usize) -> bool {
    use crate::commands::export_table::type_unit;
    let unit = |data: &WaveformData, index: usize| {
        data.units[index]
            .clone()
            .or_else(|| type_unit(&data.variable_types[index]).map(str::to_owned))
    };
    match (unit(left, i), unit(right, j)) {
        // Unit symbols and SI prefixes are case-sensitive. Comparison does not
        // silently scale values or coordinates expressed in different units.
        (Some(left), Some(right)) => left == right,
        _ => true,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ComplexPart {
    Real,
    Imag,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct VariableKey {
    part: Option<ComplexPart>,
    base: String,
}

#[derive(Clone, Debug)]
struct ParsedVariableName {
    key: VariableKey,
    aliases: Vec<String>,
}

fn parsed_variable_names_match(left: &ParsedVariableName, right: &ParsedVariableName) -> bool {
    left.key == right.key
}

fn variable_name_matches(left: &str, right: &str) -> bool {
    parsed_variable_names_match(&parse_variable_name(left), &parse_variable_name(right))
}

fn contains_variable(variables: &[String], requested: &str) -> bool {
    variables
        .iter()
        .any(|candidate| variable_name_matches(candidate, requested))
}

fn find_variable_index(variables: &[String], requested: &str) -> Option<usize> {
    variables
        .iter()
        .position(|candidate| variable_name_matches(candidate, requested))
}

/// Load waveform data from a result file in any supported format.
///
/// The scale becomes the first compared series; complex signals expand to
/// `Re(name)` / `Im(name)` so AC results compare value-for-value.
fn load_comparison_data(
    path: &std::path::Path,
    resource_limits: rspice_core::ResourceLimits,
    section: Option<&str>,
) -> Result<ComparisonData, CliError> {
    let format = detect_format(path);
    // A selected container may be compared against an already extracted table.
    let section = section.filter(|_| supports_sections(format));
    let table = match load_result_selected(path, format, resource_limits, section)? {
        ImportedResult::Table(table) => table,
        ImportedResult::Fft(fft) => return Ok(ComparisonData::Fft(Box::new(fft))),
    };
    let mut variables = vec![table.scale_name];
    let mut variable_types = vec![table.scale_type];
    let mut units = vec![table.scale_unit];
    let mut values = vec![table.scale];
    for column in table.columns {
        match column.data {
            ColumnData::Real(series) => {
                variables.push(column.name);
                variable_types.push(column.var_type);
                units.push(column.unit);
                values.push(series);
            }
            ColumnData::Complex { real, imag } => {
                variables.extend([
                    format!("Re({})", column.name),
                    format!("Im({})", column.name),
                ]);
                variable_types.extend([column.var_type.clone(), column.var_type]);
                units.extend([column.unit.clone(), column.unit]);
                values.extend([real, imag]);
            }
        }
    }
    let mut seen = HashSet::new();
    for variable in &variables {
        if !seen.insert(parse_variable_name(variable).key) {
            return Err(CliError::VerificationFailed {
                message: format!(
                    "{} contains duplicate variable '{variable}'",
                    path.display()
                ),
            });
        }
    }
    Ok(ComparisonData::Waveform(WaveformData {
        variables,
        variable_types,
        units,
        values,
    }))
}

fn parse_variable_name(name: &str) -> ParsedVariableName {
    let trimmed = name.trim();
    let (part, base) = if let Some(inner) = strip_outer_call(trimmed, "Re") {
        (Some(ComplexPart::Real), inner)
    } else if let Some(inner) = strip_outer_call(trimmed, "Im") {
        (Some(ComplexPart::Imag), inner)
    } else {
        (None, trimmed)
    };
    let base = normalize_variable_name(base);
    let mut aliases = vec![base.clone()];
    if let Some(inner) = signal_inner_name(base.as_str()) {
        push_alias(&mut aliases, normalize_variable_name(inner));
    }
    ParsedVariableName {
        key: VariableKey { part, base },
        aliases,
    }
}

fn strip_outer_call<'a>(name: &'a str, function: &str) -> Option<&'a str> {
    let rest = name.get(function.len()..)?;
    if !name
        .get(..function.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(function))
        || !rest.starts_with('(')
        || !rest.ends_with(')')
    {
        return None;
    }
    rest.get(1..rest.len() - 1).map(str::trim)
}

fn normalize_variable_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn signal_inner_name(name: &str) -> Option<&str> {
    let (prefix, rest) = name.split_once('(')?;
    if prefix.eq_ignore_ascii_case("v") || prefix.eq_ignore_ascii_case("i") {
        return rest.strip_suffix(')').map(str::trim);
    }
    None
}

fn push_alias(aliases: &mut Vec<String>, alias: String) {
    if !aliases.iter().any(|existing| existing == &alias) {
        aliases.push(alias);
    }
}

fn requested_variable_matches(
    request: &ParsedVariableName,
    candidate: &ParsedVariableName,
) -> bool {
    if request.key.part.is_some() && request.key.part != candidate.key.part {
        return false;
    }
    // Quantity-qualified requests retain their meaning. A bare selector may
    // use a signal's alias, subject to the ambiguity check below.
    if signal_inner_name(&request.key.base).is_some() {
        request.key.base == candidate.key.base
    } else {
        candidate.aliases.contains(&request.key.base)
    }
}

fn explicit_variable_pairs(
    result: &WaveformData,
    golden: &WaveformData,
    args: &CompareArgs,
    cmp_result: &mut CompareResult,
) -> Vec<(usize, usize)> {
    let result_names: Vec<_> = result
        .variables
        .iter()
        .map(|name| parse_variable_name(name))
        .collect();
    let golden_names: Vec<_> = golden
        .variables
        .iter()
        .map(|name| parse_variable_name(name))
        .collect();

    let mut pairs = Vec::new();
    let mut seen = HashSet::new();

    for requested in &args.variables {
        let request = parse_variable_name(requested);
        let result_indices: Vec<_> = result_names
            .iter()
            .enumerate()
            .filter_map(|(index, parsed)| {
                requested_variable_matches(&request, parsed).then_some(index)
            })
            .collect();
        let golden_indices: Vec<_> = golden_names
            .iter()
            .enumerate()
            .filter_map(|(index, parsed)| {
                requested_variable_matches(&request, parsed).then_some(index)
            })
            .collect();

        if signal_inner_name(&request.key.base).is_none() {
            let meanings: HashSet<_> = result_indices
                .iter()
                .map(|&index| &result_names[index].key.base)
                .chain(
                    golden_indices
                        .iter()
                        .map(|&index| &golden_names[index].key.base),
                )
                .collect();
            if meanings.len() > 1 {
                cmp_result.problems.push(format!(
                    "variable selector '{requested}' is ambiguous; use a quantity-qualified name such as V({requested}) or I({requested})"
                ));
                continue;
            }
        }

        if result_indices.is_empty() || golden_indices.is_empty() {
            if !args.ignore_missing {
                if result_indices.is_empty() {
                    cmp_result
                        .problems
                        .push(format!("variable '{requested}' is missing from the result"));
                }
                if golden_indices.is_empty() {
                    cmp_result.problems.push(format!(
                        "variable '{requested}' is missing from the golden file"
                    ));
                }
            }
            continue;
        }

        let mut matched_golden = HashSet::new();

        for result_index in result_indices {
            let mut matched = false;
            for &golden_index in &golden_indices {
                if parsed_variable_names_match(
                    &result_names[result_index],
                    &golden_names[golden_index],
                ) {
                    matched = true;
                    matched_golden.insert(golden_index);
                    if seen.insert((result_index, golden_index)) {
                        pairs.push((result_index, golden_index));
                    }
                }
            }
            if !matched && !args.ignore_missing {
                cmp_result.problems.push(format!(
                    "variable '{}' is missing from the golden file",
                    result.variables[result_index]
                ));
            }
        }

        if !args.ignore_missing {
            for golden_index in golden_indices {
                if !matched_golden.contains(&golden_index) {
                    cmp_result.problems.push(format!(
                        "variable '{}' is missing from the result",
                        golden.variables[golden_index]
                    ));
                }
            }
        }
    }

    pairs
}

/// Compare two waveform datasets.
///
/// The golden file defines the contract: every golden variable must exist
/// in the result (unless `--ignore-missing`), and matched series must have
/// the same length (unless `--allow-truncated`). Extra variables in the
/// result are tolerated — new probes do not invalidate old references.
fn compare_waveforms(
    result: &WaveformData,
    golden: &WaveformData,
    args: &CompareArgs,
) -> Result<CompareResult, CliError> {
    // The imported files are already bounded and shape-checked. Interpolate
    // only matched samples, borrowing both grids instead of allocating the
    // result-column by golden-row cross product (including unused probes).
    let interpolation = args
        .interpolate
        .then(|| interpolation::Interpolation::new(result, golden))
        .transpose()?;
    let mut cmp_result = CompareResult {
        passed: true,
        num_variables: 0,
        num_points: 0,
        max_abs_diff: 0.0,
        max_rel_diff: 0.0,
        max_diff_variable: String::new(),
        differences: Vec::new(),
        problems: Vec::new(),
    };

    let explicit_pairs = if args.variables.is_empty() {
        if !args.ignore_missing {
            for var in &golden.variables {
                if !contains_variable(&result.variables, var) {
                    cmp_result
                        .problems
                        .push(format!("variable '{var}' is missing from the result"));
                }
            }
        }
        None
    } else {
        Some(explicit_variable_pairs(
            result,
            golden,
            args,
            &mut cmp_result,
        ))
    };

    // Find matching variables
    let mut pairs: Vec<_> = if let Some(pairs) = explicit_pairs {
        pairs
    } else {
        result
            .variables
            .iter()
            .enumerate()
            .filter_map(|(var_idx, var_name)| {
                find_variable_index(&golden.variables, var_name)
                    .map(|golden_idx| (var_idx, golden_idx))
            })
            .collect()
    };

    // Signal selection never discards the independent coordinate contract.
    if !variable_name_matches(&result.variables[0], &golden.variables[0]) {
        cmp_result.problems.push(format!(
            "independent coordinates differ: '{}' versus '{}'",
            result.variables[0], golden.variables[0]
        ));
    } else if !pairs.contains(&(0, 0)) {
        pairs.insert(0, (0, 0));
    }

    for (var_idx, golden_idx) in pairs {
        let var_name = &result.variables[var_idx];

        if !units_compatible(result, var_idx, golden, golden_idx) {
            cmp_result
                .problems
                .push(format!("'{var_name}': units differ"));
            continue;
        }

        if !types_compatible(
            &result.variable_types[var_idx],
            &golden.variable_types[golden_idx],
        ) {
            cmp_result.problems.push(format!(
                "'{var_name}': quantity types differ: '{}' versus '{}'",
                result.variable_types[var_idx], golden.variable_types[golden_idx]
            ));
            continue;
        }

        cmp_result.num_variables += 1;

        let result_vals = &result.values[var_idx];
        let golden_vals = &golden.values[golden_idx];

        if interpolation.is_none()
            && result_vals.len() != golden_vals.len()
            && !args.allow_truncated
        {
            cmp_result.problems.push(format!(
                "'{var_name}': result has {} points, golden has {} \
                 (--allow-truncated compares the overlap)",
                result_vals.len(),
                golden_vals.len()
            ));
        }

        let num_points = if interpolation.is_some() {
            golden_vals.len()
        } else {
            result_vals.len().min(golden_vals.len())
        };
        cmp_result.num_points = cmp_result.num_points.max(num_points);
        let held = quantity_type(&result.variable_types[var_idx]).as_deref() == Some("logic")
            || strip_outer_call(var_name, "D").is_some()
            || strip_outer_call(var_name, "E").is_some();

        for i in 0..num_points {
            let rv = if let Some(interpolation) = &interpolation {
                if var_idx == 0 {
                    interpolation.target[i]
                } else {
                    interpolation.sample(result_vals, i, held)?
                }
            } else {
                result_vals[i]
            };
            let gv = golden_vals[i];

            let abs_diff = (rv - gv).abs();
            let rel_diff = if rv == gv {
                0.0
            } else if gv == 0.0 {
                f64::INFINITY
            } else if abs_diff.is_finite() {
                abs_diff / gv.abs()
            } else {
                // Finite operands can overflow the subtraction. Their ratio
                // can still establish a finite relative error.
                (rv / gv - 1.0).abs()
            };

            // Track the two maxima independently, including the sample that
            // causes a --fail-fast return.
            if abs_diff > cmp_result.max_abs_diff {
                cmp_result.max_abs_diff = abs_diff;
                cmp_result.max_diff_variable = var_name.clone();
            }
            cmp_result.max_rel_diff = cmp_result.max_rel_diff.max(rel_diff);

            // Check if within tolerance
            let within_abstol = abs_diff <= args.abstol;
            let within_reltol = rel_diff <= args.reltol;

            if !within_abstol && !within_reltol {
                cmp_result.passed = false;
                cmp_result.differences.push(Difference {
                    variable: var_name.clone(),
                    index: i,
                    result_value: rv,
                    golden_value: gv,
                    abs_diff,
                    rel_diff,
                });

                if args.fail_fast {
                    return Ok(cmp_result);
                }
            }
        }
    }

    if !cmp_result.problems.is_empty() {
        cmp_result.passed = false;
    }

    Ok(cmp_result)
}

/// Output comparison result as JSON
fn output_json(result: &CompareResult, blessed: bool, section: Option<&str>) {
    let accepted = result.passed || blessed;
    let json = serde_json::json!({
        "passed": accepted,
        "section": section,
        "comparison_passed": result.passed,
        "accepted": accepted,
        "blessed": blessed,
        "num_variables": result.num_variables,
        "num_points": result.num_points,
        "max_abs_diff": result.max_abs_diff,
        "max_rel_diff": result.max_rel_diff,
        "max_diff_variable": result.max_diff_variable,
        "num_differences": result.differences.len(),
        "problems": result.problems,
        "differences": result.differences.iter().take(10).map(|d| {
            serde_json::json!({
                "variable": d.variable,
                "index": d.index,
                "result": d.result_value,
                "golden": d.golden_value,
                "abs_diff": d.abs_diff,
                "rel_diff": d.rel_diff,
            })
        }).collect::<Vec<_>>(),
    });
    let json = crate::observability::envelope("rspice.comparison", json);
    match serde_json::to_string_pretty(&json) {
        Ok(text) => println!("{text}"),
        Err(e) => eprintln!("Error: failed to serialize comparison report: {e}"),
    }
}

/// Output comparison result as text
fn output_text(result: &CompareResult, quiet: bool) {
    if result.passed {
        if !quiet {
            println!("✓ Comparison PASSED");
            println!(
                "  Compared {} variables, {} points",
                result.num_variables, result.num_points
            );
            println!(
                "  Max difference: {:.2e} ({})",
                result.max_abs_diff, result.max_diff_variable
            );
        }
    } else {
        println!("✗ Comparison FAILED");
        for problem in &result.problems {
            println!("  {}", problem);
        }
        println!("  {} differences found", result.differences.len());

        // Show first few differences
        for (i, d) in result.differences.iter().take(5).enumerate() {
            println!(
                "  [{}] {} @ {}: result={:.6e}, golden={:.6e}, diff={:.2e}",
                i + 1,
                d.variable,
                d.index,
                d.result_value,
                d.golden_value,
                d.abs_diff
            );
        }
        if result.differences.len() > 5 {
            println!("  ... and {} more", result.differences.len() - 5);
        }
    }
}
