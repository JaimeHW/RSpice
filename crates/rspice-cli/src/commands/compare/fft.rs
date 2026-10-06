//! Compare validated FFT documents without erasing transform provenance.
use super::*;
use serde_json::Value;

pub(super) fn compare(
    result: &Value,
    golden: &Value,
    args: &CompareArgs,
) -> Result<CompareResult, CliError> {
    let mut comparison = CompareResult {
        passed: true,
        num_variables: 0,
        num_points: 0,
        max_abs_diff: 0.0,
        max_rel_diff: 0.0,
        max_diff_variable: String::new(),
        differences: Vec::new(),
        problems: Vec::new(),
    };
    for key in ["parent_analysis_id", "coordinate"] {
        if result[key] != golden[key] {
            comparison.problems.push(format!("FFT {key} differs"));
        }
    }
    let result_entries = result["results"].as_array().expect("validated FFT results");
    let golden_entries = golden["results"].as_array().expect("validated FFT results");
    let selects = |entry: &Value, name: &str| {
        entry["analysis_id"]
            .as_str()
            .is_some_and(|id| id.eq_ignore_ascii_case(name))
            || entry["signal"]["name"]
                .as_str()
                .is_some_and(|signal| variable_name_matches(signal, name))
    };
    for name in &args.variables {
        if !golden_entries.iter().any(|entry| selects(entry, name)) {
            comparison.problems.push(format!(
                "FFT selection '{name}' is absent from the golden file"
            ));
        }
    }
    for golden in golden_entries {
        if crate::abort::reason().is_some() {
            return Err(CliError::Interrupted);
        }
        if !args.variables.is_empty() && !args.variables.iter().any(|name| selects(golden, name)) {
            continue;
        }
        let id = golden["analysis_id"]
            .as_str()
            .expect("validated FFT identity");
        let Some(result) = result_entries
            .iter()
            .find(|entry| entry["analysis_id"] == golden["analysis_id"])
        else {
            if !args.ignore_missing {
                comparison
                    .problems
                    .push(format!("FFT request {id} is missing"));
            }
            continue;
        };
        for key in ["source", "signal", "sampling", "transform", "status"] {
            if result[key] != golden[key] {
                comparison
                    .problems
                    .push(format!("{id}: {key} contract differs"));
            }
        }
        // Matching unavailable requests provide no evidence of numerical agreement.
        if result["spectrum"]["bins"]
            .as_array()
            .is_none_or(Vec::is_empty)
            || golden["spectrum"]["bins"]
                .as_array()
                .is_none_or(Vec::is_empty)
        {
            comparison
                .problems
                .push(format!("{id}: spectrum is unavailable"));
            continue;
        }
        comparison.num_variables += 1;
        comparison.num_points += golden["spectrum"]["bins"].as_array().unwrap().len();
        compare_values(
            &result["spectrum"],
            &golden["spectrum"],
            &format!("{id}/spectrum"),
            args,
            &mut comparison,
        )?;
        compare_values(
            &result["metrics"],
            &golden["metrics"],
            &format!("{id}/metrics"),
            args,
            &mut comparison,
        )?;
        if args.fail_fast && (!comparison.passed || !comparison.problems.is_empty()) {
            break;
        }
    }
    if comparison.num_variables == 0 {
        comparison
            .problems
            .push("no complete FFT requests were compared".into());
    }
    comparison.passed &= comparison.problems.is_empty();
    Ok(comparison)
}

fn compare_values(
    result: &Value,
    golden: &Value,
    path: &str,
    args: &CompareArgs,
    comparison: &mut CompareResult,
) -> Result<(), CliError> {
    if args.fail_fast && (!comparison.passed || !comparison.problems.is_empty()) {
        return Ok(());
    }
    match (result, golden) {
        (Value::Object(result), Value::Object(golden)) => {
            for (key, value) in golden {
                let nested = format!("{path}/{key}");
                match result.get(key) {
                    Some(result) => compare_values(result, value, &nested, args, comparison)?,
                    None => comparison.problems.push(format!("{nested} is missing")),
                }
            }
        }
        (Value::Array(result), Value::Array(golden)) => {
            if result.len() != golden.len() {
                comparison.problems.push(format!("{path}: lengths differ"));
            }
            for (index, (result, golden)) in result.iter().zip(golden).enumerate() {
                if index.is_multiple_of(256) && crate::abort::reason().is_some() {
                    return Err(CliError::Interrupted);
                }
                compare_values(result, golden, &format!("{path}/{index}"), args, comparison)?;
            }
        }
        (Value::Number(result), Value::Number(golden)) if result.is_f64() && golden.is_f64() => {
            let rv = result.as_f64().unwrap();
            let gv = golden.as_f64().unwrap();
            let abs_diff = (rv - gv).abs();
            let rel_diff = if rv == gv {
                0.0
            } else if gv == 0.0 {
                f64::INFINITY
            } else if abs_diff.is_finite() {
                abs_diff / gv.abs()
            } else {
                (rv / gv - 1.0).abs()
            };
            if abs_diff > comparison.max_abs_diff {
                comparison.max_abs_diff = abs_diff;
                comparison.max_diff_variable = path.into();
            }
            comparison.max_rel_diff = comparison.max_rel_diff.max(rel_diff);
            if abs_diff > args.abstol && rel_diff > args.reltol {
                comparison.passed = false;
                comparison.differences.push(Difference {
                    variable: path.into(),
                    index: 0,
                    result_value: rv,
                    golden_value: gv,
                    abs_diff,
                    rel_diff,
                });
            }
        }
        _ if result != golden => comparison
            .problems
            .push(format!("{path}: values or types differ")),
        _ => {}
    }
    Ok(())
}
