//! Compare Command - Golden file regression testing
//!
//! Compares simulation results against reference files for CI/CD testing.
//! Both files may be in any supported result format (rawfile, CSV, TSV,
//! JSON, HDF5 — auto-detected by extension), with configurable absolute
//! and relative tolerances.

use crate::cli::{CliError, OutputFormat};
use crate::commands::waveform_io::{detect_format, load_table};
use std::collections::HashSet;
use std::path::PathBuf;

/// Arguments for the compare command
#[derive(Debug, Clone)]
pub struct CompareArgs {
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
pub fn execute(args: CompareArgs, _verbose: bool, quiet: bool) -> Result<(), CliError> {
    validate_compare_tolerance("--abstol", args.abstol)?;
    validate_compare_tolerance("--reltol", args.reltol)?;

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
            let _ = load_waveform_data(&args.result)?;
            bless_golden(&args.result, &args.golden, quiet, "no golden file yet")?;
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
    let result_data = load_waveform_data(&args.result)?;
    let golden_data = load_waveform_data(&args.golden)?;

    let result_data = if args.interpolate {
        resample_onto_golden(result_data, &golden_data)?
    } else {
        result_data
    };

    // Perform comparison
    let cmp_result = compare_waveforms(&result_data, &golden_data, &args)?;

    let blessed = !cmp_result.passed && args.bless;

    // Output results. JSON reports the final command outcome, so bless first
    // and only then emit a machine-readable accepted/blessed status.
    if args.format == OutputFormat::Json {
        if blessed {
            bless_golden(&args.result, &args.golden, quiet, "differences accepted")?;
        }
        output_json(&cmp_result, blessed);
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
    std::fs::copy(result, golden).map_err(|e| CliError::OutputError {
        path: golden.to_path_buf(),
        source: e,
    })?;
    if !quiet {
        println!("✓ Golden updated ({}): {}", why, golden.display());
    }
    Ok(())
}

/// Waveform data structure for comparison
struct WaveformData {
    variables: Vec<String>,
    values: Vec<Vec<f64>>,
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
    left.key.part == right.key.part
        && left
            .aliases
            .iter()
            .any(|alias| right.aliases.iter().any(|candidate| candidate == alias))
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
fn load_waveform_data(path: &std::path::Path) -> Result<WaveformData, CliError> {
    let table = load_table(path, detect_format(path))?;
    let (variables, values) = table.to_real_series().into_iter().unzip();
    Ok(WaveformData { variables, values })
}

/// Linearly resample the result's series onto the golden file's scale so
/// runs with different time grids compare point-for-point. The scale is
/// each file's first series; the result scale must be strictly increasing
/// and must cover the golden range — interpolation never extrapolates.
fn resample_onto_golden(
    result: WaveformData,
    golden: &WaveformData,
) -> Result<WaveformData, CliError> {
    let invalid = |message: String| CliError::VerificationFailed { message };

    let result_scale = result
        .values
        .first()
        .ok_or_else(|| invalid("result file has no data to interpolate".to_string()))?
        .clone();
    let golden_scale = golden
        .values
        .first()
        .ok_or_else(|| invalid("golden file has no data to interpolate against".to_string()))?;

    if result_scale.len() < 2 {
        return Err(invalid(
            "result needs at least two points to interpolate".to_string(),
        ));
    }
    if result_scale.windows(2).any(|pair| pair[1] <= pair[0]) {
        return Err(invalid(
            "result scale is not strictly increasing; cannot interpolate".to_string(),
        ));
    }

    let (low, high) = (result_scale[0], *result_scale.last().expect("non-empty"));
    let slack = (high - low).abs().max(1.0) * 1e-9;
    for &point in golden_scale {
        if point < low - slack || point > high + slack {
            return Err(invalid(format!(
                "golden scale point {point:e} lies outside the result range                  [{low:e}, {high:e}]; interpolation would extrapolate"
            )));
        }
    }

    let interp_at = |series: &[f64], x: f64| -> f64 {
        // Index of the first scale point >= x (the scale is sorted).
        let upper = result_scale.partition_point(|&s| s < x);
        if upper == 0 {
            return series[0];
        }
        if upper >= result_scale.len() {
            return *series.last().expect("non-empty");
        }
        let (x0, x1) = (result_scale[upper - 1], result_scale[upper]);
        let (y0, y1) = (series[upper - 1], series[upper]);
        if x1 == x0 {
            return y0;
        }
        y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    };

    let mut values = Vec::with_capacity(result.values.len());
    values.push(golden_scale.clone());
    for series in result.values.iter().skip(1) {
        if series.len() != result_scale.len() {
            return Err(invalid(
                "result series lengths disagree with its scale; cannot interpolate".to_string(),
            ));
        }
        values.push(golden_scale.iter().map(|&x| interp_at(series, x)).collect());
    }

    Ok(WaveformData {
        variables: result.variables,
        values,
    })
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
    request
        .aliases
        .iter()
        .any(|alias| candidate.aliases.iter().any(|candidate| candidate == alias))
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
    let pairs: Vec<_> = if let Some(pairs) = explicit_pairs {
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

    for (var_idx, golden_idx) in pairs {
        let var_name = &result.variables[var_idx];

        cmp_result.num_variables += 1;

        let result_vals = &result.values[var_idx];
        let golden_vals = &golden.values[golden_idx];

        if result_vals.len() != golden_vals.len() && !args.allow_truncated {
            cmp_result.problems.push(format!(
                "'{var_name}': result has {} points, golden has {} \
                 (--allow-truncated compares the overlap)",
                result_vals.len(),
                golden_vals.len()
            ));
        }

        let num_points = result_vals.len().min(golden_vals.len());
        cmp_result.num_points = cmp_result.num_points.max(num_points);

        for i in 0..num_points {
            let rv = result_vals[i];
            let gv = golden_vals[i];

            let abs_diff = (rv - gv).abs();
            let rel_diff = if gv.abs() > 1e-20 {
                abs_diff / gv.abs()
            } else {
                abs_diff
            };

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

            // Track maximum differences
            if abs_diff > cmp_result.max_abs_diff {
                cmp_result.max_abs_diff = abs_diff;
                cmp_result.max_rel_diff = rel_diff;
                cmp_result.max_diff_variable = var_name.clone();
            }
        }
    }

    if !cmp_result.problems.is_empty() {
        cmp_result.passed = false;
    }

    Ok(cmp_result)
}

/// Output comparison result as JSON
fn output_json(result: &CompareResult, blessed: bool) {
    let accepted = result.passed || blessed;
    let json = serde_json::json!({
        "passed": accepted,
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
