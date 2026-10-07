//! Corner runs use the ordinary deck executor and retain one typed report per
//! corner. Serial and parallel scheduling share the same job and merge order.
use super::*;
use crate::commands::truncate;

pub(super) fn names(
    args: &RunArgs,
    limits: rspice_core::ResourceLimits,
) -> Result<Vec<String>, CliError> {
    let Some(specification) = args.corners.as_deref() else {
        return Ok(Vec::new());
    };
    let mut names = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for name in specification.split(',').map(str::trim) {
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(CliError::InvalidArgument {
                message: format!("invalid corner name '{name}'; use letters, digits, '_' or '-'"),
                suggestion: Some("e.g. --corners tt,ss,ff".into()),
            });
        }
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(CliError::InvalidArgument {
                message: format!("duplicate corner name '{name}' would reuse an artifact path"),
                suggestion: Some("specify each corner once".into()),
            });
        }
        if names.len() >= limits.max_batch_runs {
            return Err(rspice_core::SimulationError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::BatchRuns,
                    requested: names.len().saturating_add(1),
                    limit: limits.max_batch_runs,
                },
            )
            .into());
        }
        names.push(name.to_string());
    }
    Ok(names)
}

pub(super) struct PreparedCorner {
    pub name: String,
    pub netlist: Netlist,
    pub args: RunArgs,
}

/// Inject the selected library before parsing any expressions or output requests.
pub(super) fn prepare(
    source: &str,
    args: &RunArgs,
    config: &Config,
) -> Result<Vec<PreparedCorner>, CliError> {
    let names = names(args, config.resources.limits())?;
    let library = args
        .corner_lib
        .as_ref()
        .map(|path| {
            std::path::absolute(path).map_err(|source| CliError::InputReadError {
                path: path.clone(),
                source,
            })
        })
        .transpose()?;
    if let Some(path) = &library
        && !path.is_file()
    {
        return Err(CliError::InputNotFound {
            path: path.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "corner library not found"),
        });
    }
    names
        .into_iter()
        .map(|name| {
            let mut child = args.clone();
            child.corners = None;
            child.selected_corner = Some(name.clone());
            child.corner_lib = None;
            child.jobs = 1;
            child.output = resolve_output_path(args.output.clone(), config)?
                .map(|path| tag_output_path(&path, &name));
            let source = if let Some(library) = &library {
                let (title, body) = source.split_once('\n').unwrap_or((source, ""));
                format!("{title}\n.lib \"{}\" {name}\n{body}", library.display())
            } else {
                source.to_string()
            };
            let netlist = load_netlist_from_source(&source, &child, config, false)?;
            Ok(PreparedCorner {
                name,
                netlist,
                args: child,
            })
        })
        .collect()
}

pub(super) fn run(
    prepared: &[PreparedCorner],
    args: &RunArgs,
    config: &Config,
    quiet: bool,
    run_label: Option<&str>,
) -> Result<DeckOutcome, CliError> {
    let jobs = effective_jobs(
        args.jobs,
        prepared.len(),
        config.resources.max_parallel_workers,
    )?;
    if !quiet {
        crate::console::line(format_args!(
            "Running process corner sweep: {} corners on {jobs} workers",
            prepared.len()
        ))?;
        if args.corner_lib.is_none() {
            crate::console::line(format_args!(
                "  No --corner-lib supplied; each corner uses the nominal models."
            ))?;
        }
    }
    let transaction = publish::current();
    let destinations = publish::destinations::current();
    let report_budget = ReportValueBudget::new(config.resources.limits().max_result_values);
    let job = |prepared: &PreparedCorner| -> Result<DeckOutcome, CliError> {
        let corner = &prepared.name;
        let started = Instant::now();
        let _joined = transaction.clone().map(publish::enter);
        let _destinations = destinations.clone().map(publish::destinations::enter);
        let execute = || -> Result<DeckOutcome, CliError> {
            if crate::abort::reason().is_some() {
                return Err(cancellation_cli_error(args.timeout));
            }
            run_deck(
                &prepared.netlist,
                &prepared.args,
                config,
                false,
                true,
                run_label,
            )
        };
        let mut outcome = match execute() {
            Ok(outcome) => outcome,
            Err(error) => DeckOutcome {
                reports: vec![SimulationReport {
                    name: run_label.unwrap_or("base").to_string(),
                    netlist: args.input.display().to_string(),
                    passed: false,
                    duration_secs: started.elapsed().as_secs_f64(),
                    error: Some(simulation_error_message(&error)),
                    error_details: Some(error.details()),
                    measurements: Vec::new(),
                }],
                outputs: Vec::new(),
            },
        };
        for report in &mut outcome.reports {
            report.name = format!("{} [corner {corner}]", report.name);
            for measurement in &mut report.measurements {
                measurement.name = format!("{corner}:{}", measurement.name);
            }
        }
        report_budget.admit_reports(&outcome.reports)?;
        Ok(outcome)
    };
    let outcomes: Vec<_> = if jobs > 1 {
        use rayon::prelude::*;
        rayon::ThreadPoolBuilder::new()
            .num_threads(jobs)
            .build()
            .map_err(|error| CliError::InternalError {
                message: format!("failed to create corner workers: {error}"),
            })?
            .install(|| {
                prepared
                    .par_iter()
                    .map(job)
                    .collect::<Result<Vec<_>, CliError>>()
            })?
    } else {
        prepared
            .iter()
            .map(job)
            .collect::<Result<Vec<_>, CliError>>()?
    };
    let results: Vec<_> = prepared
        .iter()
        .zip(&outcomes)
        .map(|(name, outcome)| {
            (
                name.name.clone(),
                outcome.reports.iter().all(|report| report.error.is_none()),
                outcome.reports.iter().all(|report| {
                    report
                        .measurements
                        .iter()
                        .all(|measurement| measurement.passed)
                }),
            )
        })
        .collect();
    if !quiet {
        for line in corner_summary_lines(&results) {
            crate::console::line(format_args!("{line}"))?;
        }
    }
    let mut combined = DeckOutcome {
        reports: Vec::new(),
        outputs: Vec::new(),
    };
    for outcome in outcomes {
        combined.reports.extend(outcome.reports);
        combined.outputs.extend(outcome.outputs);
    }
    Ok(combined)
}

/// Columns inside the corner-sweep summary frame, between its two borders.
const CORNER_SUMMARY_INTERIOR: usize = 37;

/// Columns the corner name is printed in.
///
/// Everything between the name and the status was padding: the name held six
/// columns and the status field nineteen more than the six a status occupies.
/// The name field is the whole span, which moves nothing on screen and shows
/// the corner names decks actually carry. Names are authored, so they are not
/// bounded by anything the frame knows about; one wider than this is cut to
/// the column rather than allowed to carry its own row's border past the
/// frame.
const CORNER_SUMMARY_NAME_WIDTH: usize = 25;

/// Columns the pass/fail status is printed in, right against the border.
const CORNER_SUMMARY_STATUS_WIDTH: usize = 6;

/// The whole corner-sweep summary frame, one line per element.
///
/// Rendered rather than printed so the widths are testable: every line is the
/// frame's two borders plus its interior, and each rune here occupies one
/// terminal column, so a row that outgrows the frame is a row whose character
/// count no longer matches the border's.
fn corner_summary_lines(results: &[(String, bool, bool)]) -> Vec<String> {
    let rule = "─".repeat(CORNER_SUMMARY_INTERIOR);
    let mut lines = vec![
        format!("┌{rule}┐"),
        format!(
            "│{:^width$}│",
            "Corner Sweep Summary",
            width = CORNER_SUMMARY_INTERIOR
        ),
        format!("├{rule}┤"),
    ];
    for (name, simulation_passed, measurements_passed) in results {
        lines.push(corner_summary_row(
            name,
            *simulation_passed && *measurements_passed,
        ));
    }
    lines.push(format!("└{rule}┘"));
    lines
}

/// One corner's row of the summary frame.
///
/// Two padding columns, the name, two more, the status, two more: the frame's
/// interior exactly. A format width pads but never truncates, so the authored
/// name goes through [`truncate`] first.
fn corner_summary_row(name: &str, passed: bool) -> String {
    let status = if passed { "✓ PASS" } else { "✗ FAIL" };
    format!(
        "│  {:name_width$}  {:>status_width$}  │",
        truncate(name, CORNER_SUMMARY_NAME_WIDTH),
        status,
        name_width = CORNER_SUMMARY_NAME_WIDTH,
        status_width = CORNER_SUMMARY_STATUS_WIDTH
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_corner_summary_line_is_the_frame_wide_for_any_corner_name() {
        let results = vec![
            ("tt".to_string(), true, true),
            ("ss_hot_slow_max_vdd_low_rc_worst".to_string(), false, true),
            ("ff".to_string(), true, false),
        ];

        let lines = corner_summary_lines(&results);
        assert_eq!(lines.len(), results.len() + 4, "three borders and a title");
        for line in &lines {
            assert_eq!(
                line.chars().count(),
                CORNER_SUMMARY_INTERIOR + 2,
                "this row leaves the frame: {line}"
            );
        }
        assert!(
            lines[4].contains("ss_hot_slow_max_vdd_lo..."),
            "a corner name wider than its column is cut to it: {}",
            lines[4]
        );
        // Byte for byte what this frame printed before the name field took the
        // padding that sat between it and the status: a short name renders in
        // the same columns it always did.
        assert_eq!(lines[1], "│        Corner Sweep Summary         │");
        assert_eq!(lines[3], "│  tt                         ✓ PASS  │");
        assert!(
            lines[3].contains("✓ PASS") && lines[4].contains("✗ FAIL"),
            "a corner passes only when its measurements do"
        );
    }
}
