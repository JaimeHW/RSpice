//! Spectre correlation regression suite.
//!
//! This suite provides:
//! - deterministic built-in fixtures for CI and release gating,
//! - optional external fixture execution from `RSPICE_SPECTRE_DATA_DIR`.

use rspice_core::testing::{CorrelationTolerancePolicy, compare_scalar, compare_waveform};
use std::fs;
use std::path::{Path, PathBuf};

fn parse_waveform_csv(path: &Path) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {}", path.display(), e))?;
    let mut x = Vec::new();
    let mut reference = Vec::new();
    let mut candidate = Vec::new();
    for (line_idx, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split(',').map(str::trim).collect();
        if cols.len() != 3 {
            return Err(format!(
                "{}:{} expected 3 columns (x,reference,candidate)",
                path.display(),
                line_idx + 1
            ));
        }
        let parse = |value: &str, col_name: &str| -> Result<f64, String> {
            value.parse::<f64>().map_err(|e| {
                format!(
                    "{}:{} failed to parse {}='{}': {}",
                    path.display(),
                    line_idx + 1,
                    col_name,
                    value,
                    e
                )
            })
        };
        x.push(parse(cols[0], "x")?);
        reference.push(parse(cols[1], "reference")?);
        candidate.push(parse(cols[2], "candidate")?);
    }
    if x.is_empty() {
        return Err(format!("{} contains no waveform samples", path.display()));
    }
    Ok((x, reference, candidate))
}

fn collect_csv_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for entry in
        fs::read_dir(dir).map_err(|e| format!("failed to read {}: {}", dir.display(), e))?
    {
        let entry = entry.map_err(|e| format!("failed to inspect {}: {}", dir.display(), e))?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("csv"))
        {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

#[test]
fn test_spectre_correlation_policy_release_defaults_are_sane() {
    let policy = CorrelationTolerancePolicy::release_default();
    assert!(policy.validate().is_ok());
    assert!(policy.scalar_abs > 0.0);
    assert!(policy.waveform_rel > 0.0);
}

#[test]
fn test_spectre_correlation_builtin_scalar_fixture() {
    let policy = CorrelationTolerancePolicy::release_default();
    let cmp = compare_scalar(
        1.800_000_000,
        1.800_000_090,
        policy.scalar_abs,
        policy.scalar_rel,
    )
    .expect("scalar comparison should succeed");
    assert!(
        cmp.within_limits,
        "scalar fixture should pass release policy: abs={} rel={}",
        cmp.abs_error, cmp.rel_error
    );
}

#[test]
fn test_spectre_correlation_builtin_waveform_fixture() {
    let policy = CorrelationTolerancePolicy::release_default();
    // Synthetic RC-like low-pass magnitude fixture sampled on a log-ish grid.
    let x = vec![1e3, 3e3, 1e4, 3e4, 1e5, 3e5, 1e6];
    let reference = vec![
        0.99995, 0.99955, 0.99504, 0.95690, 0.70710, 0.31623, 0.09950,
    ];
    let candidate = vec![
        0.99994, 0.99952, 0.99500, 0.95660, 0.70690, 0.31610, 0.09940,
    ];
    let cmp = compare_waveform(
        &x,
        &reference,
        &x,
        &candidate,
        policy.waveform_abs,
        policy.waveform_rel,
        policy.waveform_rms_rel,
    )
    .expect("waveform comparison should succeed");
    assert!(
        cmp.within_limits,
        "builtin waveform fixture should pass: max_abs={} max_rel={} rms_rel={}",
        cmp.max_abs_error, cmp.max_rel_error, cmp.rms_rel_error
    );
}

#[test]
fn test_spectre_correlation_external_fixture_directory_if_configured() {
    let Ok(dir) = std::env::var("RSPICE_SPECTRE_DATA_DIR") else {
        eprintln!(
            "RSPICE_SPECTRE_DATA_DIR not set; skipping external Spectre correlation fixtures"
        );
        return;
    };
    let dir_path = Path::new(&dir);
    let files = collect_csv_files(dir_path).expect("failed to collect external fixture CSV files");
    assert!(
        !files.is_empty(),
        "RSPICE_SPECTRE_DATA_DIR was set to '{}' but no CSV files were found",
        dir
    );

    let policy = CorrelationTolerancePolicy::release_default();
    for file in files {
        let (x, reference, candidate) = parse_waveform_csv(&file)
            .unwrap_or_else(|e| panic!("invalid fixture {}: {}", file.display(), e));
        let cmp = compare_waveform(
            &x,
            &reference,
            &x,
            &candidate,
            policy.waveform_abs,
            policy.waveform_rel,
            policy.waveform_rms_rel,
        )
        .unwrap_or_else(|e| panic!("comparison failed for {}: {}", file.display(), e));
        assert!(
            cmp.within_limits,
            "fixture {} exceeded policy: max_abs={} max_rel={} rms_rel={}",
            file.display(),
            cmp.max_abs_error,
            cmp.max_rel_error,
            cmp.rms_rel_error
        );
    }
}
