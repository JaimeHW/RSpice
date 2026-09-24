//! Capture the original ordered script through ngspice's own interpreter.

use super::*;
use crate::suites::execution::control::{
    ControlContract, ControlOracle, ControlRunReference, declarative_run_kind,
};

const MARKER: &str = "RSPICE_CONTROL_CAPTURE";

pub(super) fn capture_control(
    executable: &Path,
    version: &str,
    path: &Path,
    expanded: &str,
    timeout_ms: u128,
) -> Result<String, String> {
    let contract = ControlContract::load(path, expanded)?;
    let source = instrument(expanded)?;
    let (plots, log) = run_ngspice_raw_with_log(executable, path, &source, timeout_ms)?;
    let occurrences = log
        .lines()
        .filter_map(|line| line.strip_prefix(MARKER))
        .map(|line| {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() != 2 {
                return Err("malformed ngspice control occurrence".to_string());
            }
            let line = fields[0]
                .parse::<usize>()
                .map_err(|_| "invalid control source line")?;
            Ok((line, fields[1].to_string()))
        })
        .collect::<Result<Vec<_>, String>>()?;
    if plots.len() != contract.runs.len() || occurrences.len() != plots.len() {
        return Err(format!(
            "ordered capture expected {} runs, got {} plots and {} command occurrences",
            contract.runs.len(),
            plots.len(),
            occurrences.len()
        ));
    }
    let mut ordinals = BTreeMap::<String, usize>::new();
    let mut runs = Vec::new();
    for ((plot, (line, kind)), expected) in plots.iter().zip(occurrences).zip(&contract.runs) {
        let ordinal = ordinals.entry(kind.clone()).or_default();
        *ordinal += 1;
        let dataset = format!("{kind}{ordinal}");
        let axis = plot_axis(plot);
        let expected_axis = match kind.as_str() {
            "op" => "op",
            "ac" => "frequency",
            "tran" => "time",
            _ => return Err("unsupported control analysis kind".into()),
        };
        if dataset != expected.dataset || line != expected.line || axis != expected_axis {
            return Err(format!(
                "actual ngspice occurrence {dataset} at line {line} on {axis} differs from its contract"
            ));
        }
        let requests: Vec<_> = expected
            .probes
            .iter()
            .map(|expression| OutputRequest {
                axis: Some(axis.clone()),
                expression: expression.clone(),
            })
            .collect();
        super::capture::validate_plot(plot, &requests)?;
        let values = expected
            .probes
            .iter()
            .map(|probe| {
                resolve_raw_series(plot, probe)
                    .map(|(_, values)| values)
                    .ok_or_else(|| format!("{dataset}: missing requested probe {probe}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        runs.push(ControlRunReference {
            line,
            dataset,
            axis,
            coordinates: if kind == "op" {
                vec![0.0]
            } else {
                plot.data[0].iter().map(|value| value.re).collect()
            },
            values,
        });
    }
    let binary =
        fs::read(executable).map_err(|error| format!("oracle executable identity: {error}"))?;
    let oracle = ControlOracle {
        version: 1,
        producer: format!(
            "{version}; csnumprec=17; executable-blake3={}",
            blake3::hash(&binary)
        ),
        source_fingerprint: super::super::oracle_bundle::source_fingerprint(expanded),
        contract_fingerprint: contract.fingerprint(),
        runs,
    };
    oracle.validate(&contract, expanded)?;
    serde_json::to_string(&oracle)
        .map(|content| content + "\n")
        .map_err(|error| error.to_string())
}

/// ngspice retains the original control flow, expressions, alterations and
/// quit. CSNUMPREC=17 prevents its default six-digit console substitution from
/// rounding scalar analysis arguments; this precision is explicit in the
/// contract and producer identity. Other additions only record raw results.
fn instrument(source: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut in_control = false;
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        let command = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if index == 0 {
            output.push_str(line);
            output.push('\n');
            continue;
        }
        if command == ".control" {
            if in_control {
                return Err("nested capture control block".into());
            }
            in_control = true;
            output.push_str(line);
            output.push_str("\nset appendwrite\nset filetype=binary\nset csnumprec=17\n");
            continue;
        }
        if command == ".endc" {
            in_control = false;
        }
        if in_control && !trimmed.is_empty() && !trimmed.starts_with('*') {
            if !matches!(
                command.as_str(),
                "foreach"
                    | "repeat"
                    | "while"
                    | "dowhile"
                    | "if"
                    | "else"
                    | "end"
                    | "let"
                    | "alter"
                    | "option"
                    | "options"
                    | "set"
                    | "settype"
                    | "plot"
                    | "print"
                    | "op"
                    | "ac"
                    | "tran"
                    | "run"
                    | "quit"
                    | "break"
                    | "continue"
            ) || trimmed.contains([';', '`'])
            {
                return Err(format!(
                    "control line {} is outside the bounded headless capture contract",
                    index + 1
                ));
            }
            if command == "set"
                && trimmed.split_whitespace().skip(1).any(|word| {
                    [
                        "appendwrite",
                        "rawfile",
                        "filetype",
                        "plainwrite",
                        "csnumprec",
                    ]
                    .iter()
                    .any(|name| {
                        word.split('=')
                            .next()
                            .unwrap_or_default()
                            .eq_ignore_ascii_case(name)
                    })
                })
            {
                return Err("control script changes a capture-owned output setting".into());
            }
        }
        output.push_str(line);
        output.push('\n');
        if in_control && matches!(command.as_str(), "op" | "ac" | "tran" | "run") {
            let kind = if command == "run" {
                declarative_run_kind(source)?
            } else {
                &command
            };
            writeln!(output, "echo {MARKER} {} {kind}\nwrite", index + 1).unwrap();
        }
    }
    if in_control {
        return Err("unterminated capture control block".into());
    }
    Ok(output)
}
