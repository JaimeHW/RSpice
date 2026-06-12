//! Compare Command - Golden file regression testing
//!
//! Compares simulation results against reference files for CI/CD testing.
//! Both files may be in any supported result format (rawfile, CSV, TSV,
//! JSON, HDF5 — auto-detected by extension), with configurable absolute
//! and relative tolerances.

use crate::cli::{CliError, OutputFormat};
use crate::commands::waveform_io::{detect_format, load_table};
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
    // Validate files exist
    if !args.result.exists() {
        return Err(CliError::InputNotFound {
            path: args.result.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "Result file not found"),
        });
    }
    if !args.golden.exists() {
        if args.bless {
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

    // Perform comparison
    let cmp_result = compare_waveforms(&result_data, &golden_data, &args)?;

    // Output results
    if args.format == OutputFormat::Json {
        output_json(&cmp_result);
    } else {
        output_text(&cmp_result, quiet);
    }

    if cmp_result.passed {
        Ok(())
    } else if args.bless {
        bless_golden(&args.result, &args.golden, quiet, "differences accepted")?;
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

/// Load waveform data from a result file in any supported format.
///
/// The scale becomes the first compared series; complex signals expand to
/// `Re(name)` / `Im(name)` so AC results compare value-for-value.
fn load_waveform_data(path: &std::path::Path) -> Result<WaveformData, CliError> {
    let table = load_table(path, detect_format(path))?;
    let (variables, values) = table.to_real_series().into_iter().unzip();
    Ok(WaveformData { variables, values })
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

    // Structural checks: requested/golden variables must exist on both sides.
    if args.variables.is_empty() {
        if !args.ignore_missing {
            for var in &golden.variables {
                if !result.variables.contains(var) {
                    cmp_result
                        .problems
                        .push(format!("variable '{var}' is missing from the result"));
                }
            }
        }
    } else {
        for var in &args.variables {
            if !result.variables.contains(var) {
                cmp_result
                    .problems
                    .push(format!("variable '{var}' is missing from the result"));
            }
            if !golden.variables.contains(var) {
                cmp_result
                    .problems
                    .push(format!("variable '{var}' is missing from the golden file"));
            }
        }
    }

    // Find matching variables
    for (var_idx, var_name) in result.variables.iter().enumerate() {
        // Skip if not in filter list (if filter is specified)
        if !args.variables.is_empty() && !args.variables.contains(var_name) {
            continue;
        }

        // Find matching variable in golden
        let golden_idx = match golden.variables.iter().position(|v| v == var_name) {
            Some(idx) => idx,
            None => continue, // Variable not in golden
        };

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
fn output_json(result: &CompareResult) {
    let json = serde_json::json!({
        "passed": result.passed,
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
