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
    ImportedResult, ResultSnapshot, detect_format, load_result_selected, supports_sections,
};

mod dc_match;
mod determinations;
mod evidence;
mod fft;
mod interpolation;
mod selection;
mod sensitivity;
use selection::{parse_variable_name, variable_name_matches};
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
    /// Total mismatching values, including those beyond the displayed examples.
    pub num_differences: usize,
    /// First ten differences, the largest report preview any output format uses.
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

impl CompareResult {
    /// Record exact totals and extrema while retaining only reportable examples.
    /// Waveforms and FFT coefficients use the same tolerance calculation.
    fn compare_number(
        &mut self,
        result: f64,
        golden: f64,
        variable: &str,
        index: usize,
        args: &CompareArgs,
    ) -> bool {
        let abs_diff = (result - golden).abs();
        let rel_diff = if result == golden {
            0.0
        } else if golden == 0.0 {
            f64::INFINITY
        } else if abs_diff.is_finite() {
            abs_diff / golden.abs()
        } else {
            // Finite operands can overflow subtraction while their ratio
            // still establishes a finite relative error.
            (result / golden - 1.0).abs()
        };
        if abs_diff > self.max_abs_diff {
            self.max_abs_diff = abs_diff;
            self.max_diff_variable = variable.to_string();
        }
        self.max_rel_diff = self.max_rel_diff.max(rel_diff);
        let differs = abs_diff > args.abstol && rel_diff > args.reltol;
        if differs {
            self.passed = false;
            self.num_differences += 1;
            if self.differences.len() < 10 {
                self.differences.push(Difference {
                    variable: variable.to_string(),
                    index,
                    result_value: result,
                    golden_value: golden,
                    abs_diff,
                    rel_diff,
                });
            }
        }
        differs
    }
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
    // Promotion and validation must use the same bytes even if a simulator
    // replaces or edits the result while comparison is running.
    let snapshot = args
        .bless
        .then(|| ResultSnapshot::read(&args.result, config.resources.limits()))
        .transpose()?;
    if !args.golden.exists() {
        if args.bless {
            // Missing-golden bootstrap is still a promotion of a result
            // artifact. Validate the result before copying so malformed CSV,
            // JSON, RAW, etc. cannot become the accepted baseline.
            let snapshot = snapshot.as_ref().expect("bless captured a snapshot");
            let data = comparison_data(&args.result, snapshot.load(args.section.as_deref())?)?;
            let mut comparison = validate_bless_candidate(&data, &args)?;
            bless_golden(snapshot, &data, &args.golden, quiet, "no golden file yet")?;
            if args.format == OutputFormat::Json {
                comparison.passed = false;
                comparison
                    .problems
                    .push("golden file did not exist".to_string());
                output_json(&comparison, true, args.section.as_deref())?;
            }
            return Ok(());
        }
        return Err(CliError::InputNotFound {
            path: args.golden.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "Golden file not found"),
        });
    }

    if !quiet {
        crate::console::line(format_args!(
            "Comparing: {} vs {}",
            args.result.display(),
            args.golden.display()
        ))?;
        crate::console::line(format_args!(
            "  Tolerances: abstol={:.2e}, reltol={:.2e}",
            args.abstol, args.reltol
        ))?;
    }

    // Load and parse files
    let result_data = if let Some(snapshot) = &snapshot {
        comparison_data(&args.result, snapshot.load(args.section.as_deref())?)?
    } else {
        load_comparison_data(
            &args.result,
            config.resources.limits(),
            args.section.as_deref(),
        )?
    };
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
            bless_golden(
                snapshot.as_ref().expect("bless captured a snapshot"),
                &result_data,
                &args.golden,
                quiet,
                "differences accepted",
            )?;
        }
        output_json(&cmp_result, blessed, args.section.as_deref())?;
    } else {
        output_text(&cmp_result, quiet)?;
        if blessed {
            bless_golden(
                snapshot.as_ref().expect("bless captured a snapshot"),
                &result_data,
                &args.golden,
                quiet,
                "differences accepted",
            )?;
        }
    }

    if cmp_result.passed || blessed {
        Ok(())
    } else {
        let mut parts = Vec::new();
        if cmp_result.num_differences > 0 {
            parts.push(format!(
                "{} value difference(s), max {:.2e} ({})",
                cmp_result.num_differences, cmp_result.max_abs_diff, cmp_result.max_diff_variable
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
    result: &ResultSnapshot,
    candidate: &ComparisonData,
    golden: &std::path::Path,
    quiet: bool,
    why: &str,
) -> Result<(), CliError> {
    if detect_format(result.path()) != detect_format(golden) {
        return Err(CliError::InvalidArgument {
            message: "--bless requires the result and golden to use the same file format"
                .to_string(),
            suggestion: Some(
                "convert the result to the golden's format before blessing it".to_string(),
            ),
        });
    }
    if detect_format(result.path()) == crate::cli::InputFormat::Touchstone {
        let invalid_destination = |detail: String| {
            CliError::InvalidArgument {
            message: format!(
                "--bless cannot preserve the Touchstone result at '{}': {detail}",
                golden.display()
            ),
            suggestion: Some("use a destination name that preserves the original port count and network interpretation".into()),
        }
        };
        let destination = result
            .touchstone_at(golden)
            .and_then(|data| comparison_data(golden, data))
            .map_err(|error| invalid_destination(error.to_string()))?;
        // This is an artifact-identity check, independent of probe selection,
        // comparison tolerances, interpolation, and truncation allowances.
        if !matches!((candidate, &destination),
            (ComparisonData::Waveform(original), ComparisonData::Waveform(promoted)) if original == promoted
        ) {
            return Err(invalid_destination(
                "the destination filename changes the decoded network".into(),
            ));
        }
    }
    publish::artifact(golden, |writer| {
        writer
            .write_all(result.bytes())
            .map_err(|source| CliError::output_error(golden, source))
    })
    .map_err(|error| map_atomic_output_error(golden, error))?;
    if !quiet {
        crate::console::line(format_args!(
            "✓ Golden updated ({}): {}",
            why,
            golden.display()
        ))?;
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
#[derive(PartialEq)]
struct WaveformData {
    variables: Vec<String>,
    variable_types: Vec<String>,
    units: Vec<Option<String>>,
    values: Vec<Vec<f64>>,
    /// Undefined samples have zero padding that must never enter arithmetic.
    validity: Vec<Option<Vec<bool>>>,
}

impl WaveformData {
    fn sample(&self, column: usize, index: usize) -> Option<f64> {
        self.validity[column]
            .as_ref()
            .is_none_or(|valid| valid[index])
            .then_some(self.values[column][index])
    }
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
    comparison_data(
        path,
        load_result_selected(path, format, resource_limits, section)?,
    )
}

fn comparison_data(
    path: &std::path::Path,
    result: ImportedResult,
) -> Result<ComparisonData, CliError> {
    let table = match result {
        ImportedResult::Table(table) => table,
        ImportedResult::Fft(fft) => return Ok(ComparisonData::Fft(Box::new(fft))),
    };
    let mut variables = vec![table.scale_name];
    let mut variable_types = vec![table.scale_type];
    let mut units = vec![table.scale_unit];
    let mut values = vec![table.scale];
    let mut validity = vec![None];
    for column in table.columns {
        match column.data {
            ColumnData::NullableReal(series) => {
                variables.push(column.name);
                variable_types.push(column.var_type);
                units.push(column.unit);
                validity.push(Some(series.iter().map(Option::is_some).collect()));
                values.push(
                    series
                        .into_iter()
                        .map(|value| value.unwrap_or(0.0))
                        .collect(),
                );
            }
            ColumnData::Real(series) => {
                variables.push(column.name);
                variable_types.push(column.var_type);
                units.push(column.unit);
                values.push(series);
                validity.push(None);
            }
            ColumnData::Complex { real, imag } => {
                variables.extend([
                    format!("Re({})", column.name),
                    format!("Im({})", column.name),
                ]);
                variable_types.extend([column.var_type.clone(), column.var_type]);
                units.extend([column.unit.clone(), column.unit]);
                values.extend([real, imag]);
                validity.extend([None, None]);
            }
            ColumnData::NullableComplex(series) => {
                variables.extend([
                    format!("Re({})", column.name),
                    format!("Im({})", column.name),
                ]);
                variable_types.extend([column.var_type.clone(), column.var_type]);
                units.extend([column.unit.clone(), column.unit]);
                let defined: Vec<bool> = series.iter().map(Option::is_some).collect();
                validity.extend([Some(defined.clone()), Some(defined)]);
                let (real, imag) = series
                    .into_iter()
                    .map(|value| value.map_or((0.0, 0.0), |value| (value.re, value.im)))
                    .unzip();
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
        validity,
    }))
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
        num_differences: 0,
        differences: Vec::new(),
        problems: Vec::new(),
    };

    let pairs = selection::pairs(result, golden, args, &mut cmp_result);
    let result_determinations = determinations::Determinations::new(result);
    let golden_determinations = determinations::Determinations::new(golden);
    cmp_result.problems.extend(evidence::problems(result));
    cmp_result.problems.extend(evidence::problems(golden));
    let result_sensitivity = sensitivity::Evidence::new(result);
    let golden_sensitivity = sensitivity::Evidence::new(golden);
    cmp_result
        .problems
        .extend(result_sensitivity.problems.iter().cloned());
    cmp_result
        .problems
        .extend(golden_sensitivity.problems.iter().cloned());
    let exact_args = CompareArgs {
        abstol: 0.0,
        reltol: 0.0,
        ..Default::default()
    };
    for (evidence, data) in [
        (&result_determinations, result),
        (&golden_determinations, golden),
    ] {
        for name in evidence.invalid_names(data) {
            cmp_result
                .problems
                .push(format!("'{name}': invalid scalar unavailability evidence"));
        }
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
        // Either side may be an untyped CSV/TSV export. Use the available
        // digital declaration after checking both quantity/unit contracts.
        let held = [(result, var_idx), (golden, golden_idx)]
            .into_iter()
            .any(|(data, index)| {
                quantity_type(&data.variable_types[index]).as_deref() == Some("logic")
                    || data.units[index].as_deref() == Some("logic")
            })
            || strip_outer_call(var_name, "D").is_some()
            || strip_outer_call(var_name, "E").is_some();

        let mut defined_points = 0usize;
        let mut undefined_mismatches = 0usize;
        let mut first_undefined_mismatch = None;
        let mut determined_points = 0usize;
        let mut first_reason_mismatch = None;
        let status =
            result_sensitivity.is_status(var_idx) || golden_sensitivity.is_status(golden_idx);
        for i in 0..num_points {
            let rv = if let Some(interpolation) = &interpolation {
                if var_idx == 0 {
                    Some(interpolation.target[i])
                } else if status {
                    interpolation.availability_status(
                        result_vals,
                        result.validity[var_idx].as_deref(),
                        i,
                    )
                } else {
                    interpolation.sample(
                        result_vals,
                        result.validity[var_idx].as_deref(),
                        i,
                        held,
                    )?
                }
            } else {
                result.sample(var_idx, i)
            };
            let gv = golden.sample(golden_idx, i);

            match (rv, gv) {
                (Some(rv), Some(gv)) => {
                    defined_points += 1;
                    if cmp_result.compare_number(
                        rv,
                        gv,
                        var_name,
                        i,
                        if status { &exact_args } else { args },
                    ) && args.fail_fast
                    {
                        return Ok(cmp_result);
                    }
                }
                (None, None) => {
                    let result_index = interpolation
                        .as_ref()
                        .map_or(Some(i), |plan| plan.observed_index(i));
                    let left = result_index
                        .and_then(|row| result_sensitivity.reason(result, var_idx, row));
                    let right = golden_sensitivity.reason(golden, golden_idx, i);
                    if left.is_some() || right.is_some() {
                        if left == right {
                            determined_points += 1;
                        } else {
                            first_reason_mismatch.get_or_insert(i);
                            if args.fail_fast {
                                break;
                            }
                        }
                    }
                }
                _ => {
                    undefined_mismatches += 1;
                    first_undefined_mismatch.get_or_insert(i);
                    if args.fail_fast {
                        break;
                    }
                }
            }
        }
        if let Some(first) = first_reason_mismatch {
            cmp_result.problems.push(format!(
                "'{var_name}': sensitivity unavailability reasons differ at index {first}"
            ));
        }
        if let Some(first) = first_undefined_mismatch {
            cmp_result.problems.push(format!(
                "'{var_name}': {undefined_mismatches} sample(s) are undefined on only one side, first at index {first}"
            ));
        } else if defined_points == 0
            && determined_points == 0
            && (num_points == 0
                || !result_determinations.agrees(var_idx, &golden_determinations, golden_idx))
        {
            cmp_result
                .problems
                .push(format!("'{var_name}': no defined samples were compared"));
        }
        if args.fail_fast && !cmp_result.problems.is_empty() {
            cmp_result.passed = false;
            return Ok(cmp_result);
        }
    }

    if !cmp_result.problems.is_empty() {
        cmp_result.passed = false;
    }

    Ok(cmp_result)
}

/// Output comparison result as JSON
fn output_json(
    result: &CompareResult,
    blessed: bool,
    section: Option<&str>,
) -> Result<(), CliError> {
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
        "num_differences": result.num_differences,
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
    crate::console::json(&json, true)
}

/// Output comparison result as text
fn output_text(result: &CompareResult, quiet: bool) -> Result<(), CliError> {
    if result.passed {
        if !quiet {
            crate::console::line(format_args!("✓ Comparison PASSED"))?;
            crate::console::line(format_args!(
                "  Compared {} variables, {} points",
                result.num_variables, result.num_points
            ))?;
            crate::console::line(format_args!(
                "  Max difference: {:.2e} ({})",
                result.max_abs_diff, result.max_diff_variable
            ))?;
        }
    } else {
        crate::console::line(format_args!("✗ Comparison FAILED"))?;
        for problem in &result.problems {
            crate::console::line(format_args!("  {}", problem))?;
        }
        crate::console::line(format_args!(
            "  {} differences found",
            result.num_differences
        ))?;

        // Show first few differences
        for (i, d) in result.differences.iter().take(5).enumerate() {
            crate::console::line(format_args!(
                "  [{}] {} @ {}: result={:.6e}, golden={:.6e}, diff={:.2e}",
                i + 1,
                d.variable,
                d.index,
                d.result_value,
                d.golden_value,
                d.abs_diff
            ))?;
        }
        if result.num_differences > 5 {
            crate::console::line(format_args!(
                "  ... and {} more",
                result.num_differences - 5
            ))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blessing_publishes_validated_bytes_after_the_source_changes_or_disappears() {
        struct TestDirectory(PathBuf);
        impl Drop for TestDirectory {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
        let dir = TestDirectory(
            std::env::temp_dir().join(format!("rspice_bless_snapshot_{}", std::process::id())),
        );
        std::fs::create_dir(&dir.0).unwrap();
        let source = dir.0.join("result.csv");
        let golden = dir.0.join("golden.csv");
        let original = b"time,V(out)\r\n0,1.000\r\n1,2.000\r\n";
        std::fs::write(&source, original).unwrap();
        let snapshot = ResultSnapshot::read(&source, Default::default()).unwrap();
        let data = comparison_data(&source, snapshot.load(None).unwrap()).unwrap();
        validate_bless_candidate(&data, &CompareArgs::default()).unwrap();

        std::fs::write(&source, b"invalid replacement").unwrap();
        bless_golden(&snapshot, &data, &golden, true, "source changed").unwrap();
        assert_eq!(std::fs::read(&golden).unwrap(), original);
        std::fs::remove_file(&source).unwrap();
        bless_golden(&snapshot, &data, &golden, true, "source removed").unwrap();
        assert_eq!(std::fs::read(&golden).unwrap(), original);
    }

    #[test]
    fn comparison_retains_only_the_report_preview_for_large_failures() {
        let waveform = |values: Vec<f64>| WaveformData {
            variables: vec!["time".into(), "V(out)".into()],
            variable_types: vec!["time".into(), "voltage".into()],
            units: vec![None, None],
            values: vec![(0..values.len()).map(|i| i as f64).collect(), values],
            validity: vec![None, None],
        };
        let result = waveform((2..=100_001).map(f64::from).collect());
        let golden = waveform(vec![1.0; 100_000]);
        let comparison = compare_waveforms(&result, &golden, &CompareArgs::default()).unwrap();
        assert!(!comparison.passed);
        assert_eq!(comparison.num_differences, 100_000);
        assert_eq!(comparison.differences.len(), 10);
        assert_eq!(comparison.max_abs_diff, 100_000.0);
        assert_eq!(comparison.max_rel_diff, 100_000.0);
        assert_eq!(comparison.differences.last().unwrap().index, 9);
    }
}
