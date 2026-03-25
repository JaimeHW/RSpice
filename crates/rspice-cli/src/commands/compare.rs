//! Compare Command - Golden file regression testing
//!
//! Compares simulation results against reference files for CI/CD testing.
//! Supports configurable tolerances (absolute and relative).

use crate::cli::{CliError, OutputFormat};
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
    } else {
        Err(CliError::ConversionError {
            message: format!(
                "Comparison failed: {} differences found (max diff: {:.2e})",
                cmp_result.differences.len(),
                cmp_result.max_abs_diff
            ),
        })
    }
}

/// Waveform data structure for comparison
struct WaveformData {
    variables: Vec<String>,
    values: Vec<Vec<f64>>,
}

/// Load waveform data from file
fn load_waveform_data(path: &std::path::Path) -> Result<WaveformData, CliError> {
    let content = std::fs::read_to_string(path).map_err(|e| CliError::InputReadError {
        path: path.to_path_buf(),
        source: e,
    })?;

    // Simple CSV parser
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err(CliError::ConversionError {
            message: "Empty file".to_string(),
        });
    }

    let variables: Vec<String> = lines[0].split(',').map(|s| s.trim().to_string()).collect();
    let mut values: Vec<Vec<f64>> = vec![Vec::new(); variables.len()];

    for line in lines.iter().skip(1) {
        let vals: Vec<&str> = line.split(',').collect();
        for (i, val) in vals.iter().enumerate() {
            if i < values.len()
                && let Ok(v) = val.trim().parse::<f64>() {
                    values[i].push(v);
                }
        }
    }

    Ok(WaveformData { variables, values })
}

/// Compare two waveform datasets
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
    };

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
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_csv(path: &std::path::Path, content: &str) {
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn test_compare_identical() {
        let dir = tempdir().unwrap();
        let result_path = dir.path().join("result.csv");
        let golden_path = dir.path().join("golden.csv");

        let content = "time,V1\n0.0,1.0\n1.0,2.0\n2.0,3.0\n";
        create_csv(&result_path, content);
        create_csv(&golden_path, content);

        let result_data = load_waveform_data(&result_path).unwrap();
        let golden_data = load_waveform_data(&golden_path).unwrap();

        let args = CompareArgs::default();
        let cmp = compare_waveforms(&result_data, &golden_data, &args).unwrap();

        assert!(cmp.passed);
        assert_eq!(cmp.differences.len(), 0);
    }

    #[test]
    fn test_compare_different() {
        let dir = tempdir().unwrap();
        let result_path = dir.path().join("result.csv");
        let golden_path = dir.path().join("golden.csv");

        create_csv(&result_path, "time,V1\n0.0,1.0\n1.0,2.5\n");
        create_csv(&golden_path, "time,V1\n0.0,1.0\n1.0,2.0\n");

        let result_data = load_waveform_data(&result_path).unwrap();
        let golden_data = load_waveform_data(&golden_path).unwrap();

        let args = CompareArgs {
            abstol: 0.1,
            reltol: 0.01,
            ..Default::default()
        };
        let cmp = compare_waveforms(&result_data, &golden_data, &args).unwrap();

        assert!(!cmp.passed);
        assert!(!cmp.differences.is_empty());
    }

    #[test]
    fn test_compare_within_tolerance() {
        let dir = tempdir().unwrap();
        let result_path = dir.path().join("result.csv");
        let golden_path = dir.path().join("golden.csv");

        create_csv(&result_path, "time,V1\n0.0,1.0\n1.0,2.001\n");
        create_csv(&golden_path, "time,V1\n0.0,1.0\n1.0,2.0\n");

        let result_data = load_waveform_data(&result_path).unwrap();
        let golden_data = load_waveform_data(&golden_path).unwrap();

        let args = CompareArgs {
            abstol: 0.01,
            reltol: 0.01,
            ..Default::default()
        };
        let cmp = compare_waveforms(&result_data, &golden_data, &args).unwrap();

        assert!(cmp.passed); // 0.001 is within 0.01 tolerance
    }
}
