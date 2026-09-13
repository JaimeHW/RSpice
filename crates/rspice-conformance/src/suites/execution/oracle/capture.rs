//! Capture each promoted analysis in an independent ngspice run.

use super::*;
use crate::suites::execution::oracle_bundle::{
    HEADER, OracleReferences, analysis_key, source_fingerprint,
};

pub(super) fn capture_analyses(
    executable: &Path,
    version: &str,
    path: &Path,
    expanded: &str,
    requests: &[OutputRequest],
    timeout_ms: u128,
) -> Result<Option<String>, String> {
    let netlist = Netlist::parse_with_path(expanded, path)
        .map_err(|error| format!("cannot identify capture analyses: {error}"))?;
    let normalized = reference_deck(expanded, requests);
    let mut captured = BTreeSet::new();
    let mut output = format!(
        "{HEADER}\n# ngspice: {version}\n# input-blake3: {}\n",
        source_fingerprint(expanded)
    );
    let started = Instant::now();
    for analysis in &netlist.analyses {
        let Some(key) = analysis_key(analysis)? else {
            if matches!(
                analysis,
                AnalysisCommand::Temp { .. } | AnalysisCommand::Four { .. }
            ) {
                continue;
            }
            return Err(format!(
                "analysis is not supported by the execution-oracle capture: {analysis:?}"
            ));
        };
        if !captured.insert(key.clone()) {
            continue;
        }
        let source = isolated_analysis_source(&normalized, &key)?;
        let remaining = timeout_ms
            .checked_sub(started.elapsed().as_millis())
            .filter(|remaining| *remaining > 0)
            .ok_or_else(|| {
                format!("oracle capture exhausted its {timeout_ms}ms deck budget before {key}")
            })?;
        let plots = run_ngspice_raw(executable, path, &source, remaining)?;
        let selected: Vec<_> = plots
            .iter()
            .filter(|plot| {
                !plot
                    .plotname
                    .to_ascii_lowercase()
                    .contains("integrated noise")
            })
            .collect();
        if selected.len() != 1 {
            return Err(format!(
                "{key}: expected exactly one analysis plot, got {}",
                selected.len()
            ));
        }
        let plot = selected[0];
        validate_plot(plot, requests)?;
        let mut tables = reference_tables(&plots, requests);
        if tables.len() != 1 {
            return Err(format!(
                "{key}: no single comparable requested output table"
            ));
        }
        let table = tables.remove(0);
        let expected_axis = match analysis {
            AnalysisCommand::Op => "op",
            AnalysisCommand::Dc { .. } => "v-sweep",
            AnalysisCommand::Tran { .. } => "time",
            _ => "frequency",
        };
        if table.x_name != expected_axis {
            return Err(format!("{key}: unexpected raw plot axis {}", table.x_name));
        }
        output.push_str(&format!("# analysis: {key}\n"));
        output.push_str(&serialize_tables(&[table]));
        output.push_str("# end-analysis\n");
    }
    if captured.is_empty() {
        return Ok(None);
    }
    // Validate the actual artifact through the consumer before making it available.
    OracleReferences::parse(&output, expanded, &netlist.analyses)?;
    Ok(Some(output))
}

fn isolated_analysis_source(normalized: &str, key: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut skip_continuation = false;
    let mut inserted = false;
    for line in normalized.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('+') && skip_continuation {
            continue;
        }
        let command = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        skip_continuation = matches!(
            command.as_str(),
            ".op" | ".dc" | ".ac" | ".tran" | ".noise" | ".sp"
        );
        if skip_continuation {
            continue;
        }
        if command == ".control" && !inserted {
            output.push_str(key);
            output.push('\n');
            inserted = true;
        }
        output.push_str(line);
        output.push('\n');
    }
    if !inserted {
        return Err("normalized capture deck has no execution control block".into());
    }
    Ok(output)
}

fn validate_plot(plot: &RawReferencePlot, requests: &[OutputRequest]) -> Result<(), String> {
    let count = plot.data.first().map_or(0, Vec::len);
    if count == 0
        || plot.data.len() != plot.variables.len()
        || plot.data.iter().any(|series| {
            series.len() != count
                || series
                    .iter()
                    .any(|value| !value.re.is_finite() || !value.im.is_finite())
        })
    {
        return Err("oracle capture contains empty, incomplete or nonfinite raw samples".into());
    }
    let axis = plot_axis(plot);
    for request in requests.iter().filter(|request| {
        request
            .axis
            .as_deref()
            .is_none_or(|requested| requested == axis)
    }) {
        if matches!(normalize(&request.expression).as_str(), "all" | "allv") {
            continue;
        }
        let (_, values) = resolve_raw_series(plot, &request.expression).ok_or_else(|| {
            format!(
                "oracle capture is missing requested output {}",
                request.expression
            )
        })?;
        if values.len() != count || values.iter().any(|value| !value.is_finite()) {
            return Err(format!(
                "oracle capture has incomplete or nonfinite output {}",
                request.expression
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolated_capture_keeps_the_title_models_and_selected_analysis_only() {
        let source = "* title must remain the first line\nV1 a 0 1\n.model m npn\n+ bf=100\n.dc v1 0 2 1\n+ v2 0 1 1\n.tran 1n 2n\n.control\nrun\n.endc\n.end\n";
        let isolated = isolated_analysis_source(source, ".dc v1 0 1 1").unwrap();
        assert!(isolated.starts_with("* title must remain the first line\nV1"));
        assert!(isolated.contains(".model m npn\n+ bf=100"));
        assert!(!isolated.contains("+ v2"));
        assert!(!isolated.contains(".tran"));
        assert_eq!(isolated.matches(".dc").count(), 1);
        assert!(isolated.contains(".dc v1 0 1 1\n.control"));
    }

    #[test]
    fn capture_rejects_missing_nonfinite_and_incomplete_samples() {
        let mut plot = RawReferencePlot {
            plotname: "DC transfer characteristic".into(),
            variables: vec!["v-sweep".into(), "v(out)".into()],
            data: vec![
                vec![Complex64::new(0.0, 0.0)],
                vec![Complex64::new(1.0, 0.0)],
            ],
            is_complex: false,
        };
        let requests = vec![OutputRequest {
            axis: None,
            expression: "v(out)".into(),
        }];
        validate_plot(&plot, &requests).unwrap();
        assert!(
            validate_plot(
                &plot,
                &[OutputRequest {
                    axis: None,
                    expression: "v(missing)".into()
                }]
            )
            .is_err()
        );
        plot.data[1][0].re = f64::NAN;
        assert!(validate_plot(&plot, &requests).is_err());
        plot.data[1].clear();
        assert!(validate_plot(&plot, &requests).is_err());
    }
}
