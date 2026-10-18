//! What a run says.
//!
//! A run writes two lines in the session's log, both anchored to it: one when
//! it starts, naming its analyses, and one when it ends, saying how. Every
//! other line it writes goes in its own log, which the run owns: a header,
//! each analysis's figures, its warnings and failures, and a summary that is
//! the same text as the session's end line.

use super::*;
use crate::diagnostics::{LogAnchor, LogSeverity, LogSource};
use rspice_app_types::run_log::RunLogLine;

fn now_ms() -> u64 {
    crate::time_compat::checked_unix_time_ms().unwrap_or_default()
}

/// Write one of the run's own lines into its log, and say where it is there.
///
/// A line whose run is gone goes to the session's log instead, so what it
/// says is not lost with the run; it then has no place in the run's log.
pub(super) fn write_run_line(
    state: &mut AppState,
    run_sequence: Option<u64>,
    severity: LogSeverity,
    message: impl Into<String>,
) -> Option<usize> {
    let message = message.into();
    match run_sequence
        .and_then(|run_sequence| state.simulation.retained.run_by_sequence_mut(run_sequence))
    {
        Some(run) => {
            // The run's own lines are never withheld.
            let position = run.log.lines().len();
            run.log.push(RunLogLine {
                unix_ms: now_ms(),
                severity,
                source: LogSource::Run,
                message,
            });
            Some(position)
        }
        None => {
            state.record(severity, LogSource::Run, message);
            None
        }
    }
}

impl SimulationController {
    /// Write one of the active run's own lines into its log.
    pub(super) fn run_line(
        &self,
        state: &mut AppState,
        severity: LogSeverity,
        message: impl Into<String>,
    ) {
        write_run_line(state, self.current_run_id, severity, message);
    }

    /// Write one of the active run's own lines into its log, and say where
    /// it is there, for a finding the line states.
    pub(super) fn placed_run_line(
        &self,
        state: &mut AppState,
        severity: LogSeverity,
        message: impl Into<String>,
    ) -> Option<usize> {
        write_run_line(state, self.current_run_id, severity, message)
    }

    /// The failure a failed run's end line quotes: the first error the run
    /// wrote about itself. Never that end line, which is written last and
    /// says only that the run failed.
    pub(crate) fn run_failure(
        run_sequence: u64,
        log: &rspice_app_types::run_log::RunLog,
    ) -> Option<&str> {
        let end_line = format!("Run {run_sequence} failed after ");
        log.lines()
            .iter()
            .find(|line| {
                line.source == LogSource::Run
                    && line.severity == LogSeverity::Error
                    && !line.message.starts_with(&end_line)
            })
            .map(|line| line.message.as_str())
    }
}

/// Open a run's log with its header and say in the session's log that it
/// started.
///
/// The header is `Run 3 · RSpice 0.1.0 (1a2b3c4d5) · inv_tb · 2026-10-16
/// 14:03:12 · desktop`. A field the run does not know is left out rather than
/// written as a placeholder.
pub(super) fn announce_run_start(
    state: &mut AppState,
    run_sequence: u64,
    design: Option<&str>,
    analyses: &[&str],
) {
    let Some(run) = state.simulation.retained.run_by_sequence(run_sequence) else {
        return;
    };
    // Watched from here, so a run that ends before a frame has seen it
    // running is still announced when it ends.
    let run_id = run.run_id;
    let mut header = vec![format!("Run {}", run.id), product_version()];
    header.extend(
        design
            .filter(|design| !design.is_empty())
            .map(str::to_owned),
    );
    header.extend(local_start_time(run.timestamp));
    header.extend(
        run.execution_target
            .map(|target| platform(target).to_owned()),
    );
    write_run_line(
        state,
        Some(run_sequence),
        LogSeverity::Info,
        header.join(" \u{b7} "),
    );
    state.record_anchored(
        LogSeverity::Info,
        LogSource::Run,
        format!("Run {run_sequence} started: {}", analyses.join(", ")),
        LogAnchor::ResultRun { run_sequence },
    );
    state.ui.run_watch.run_started(run_id);
}

/// Close a run's log with its summary and write the same line in the
/// session's log.
///
/// Called once the run's lifecycle is terminal, which is when its elapsed
/// time is sealed. The counts are the log's own Error and Warning lines,
/// taken before the summary is added.
pub(super) fn announce_run_end(state: &mut AppState, run_sequence: u64) {
    let Some(run) = state.simulation.retained.run_by_sequence_mut(run_sequence) else {
        return;
    };
    let unix_ms = now_ms();
    run.log.seal(unix_ms);
    let id = run.id;
    let elapsed = crate::quantity::format_elapsed(run.elapsed_time);
    let counts = || {
        let errors = run.log.count(LogSeverity::Error);
        let warnings = run.log.count(LogSeverity::Warning);
        let mut counts = format!(
            "{} {}, {} {}",
            grouped(errors as u64),
            if errors == 1 { "error" } else { "errors" },
            grouped(warnings as u64),
            if warnings == 1 { "warning" } else { "warnings" },
        );
        let suppressed = run.log.suppressed_warnings();
        if suppressed > 0 {
            counts.push_str(&format!(" ({} suppressed)", grouped(suppressed as u64)));
        }
        counts
    };
    let (severity, summary) = match run.lifecycle {
        SimulationRunLifecycle::Aborted => (
            LogSeverity::Warning,
            format!("Run {id} was cancelled after {elapsed}"),
        ),
        SimulationRunLifecycle::Completed => (
            LogSeverity::Info,
            format!("Run {id} completed in {elapsed}: {}", counts()),
        ),
        // An interrupted run did not complete, whatever it finished first.
        SimulationRunLifecycle::Failed if run.analyses.iter().any(|analysis| analysis.success) => (
            LogSeverity::Error,
            format!("Run {id} completed with errors in {elapsed}: {}", counts()),
        ),
        _ => {
            let failure = SimulationController::run_failure(id, &run.log).map(str::to_owned);
            (
                LogSeverity::Error,
                match failure {
                    Some(failure) => format!("Run {id} failed after {elapsed}: {failure}"),
                    None => format!("Run {id} failed after {elapsed}"),
                },
            )
        }
    };
    run.log.push(RunLogLine {
        unix_ms,
        severity,
        source: LogSource::Run,
        message: summary.clone(),
    });
    state.record_anchored(
        severity,
        LogSource::Run,
        summary,
        LogAnchor::ResultRun { run_sequence },
    );
}

/// One line for an analysis that completed: its name, its wall time and,
/// for a transient solve, what the solver did.
///
/// `Transient: 1.32 s, 1,204 steps accepted, 37 rejected, 5,512 Newton
/// iterations`. Only what the result's convergence report holds for that
/// kind of analysis is stated; an analysis without one states its time.
pub(super) fn analysis_figures(
    label: &str,
    elapsed: Option<std::time::Duration>,
    result: &crate::simulation::SimulationResult,
) -> String {
    use crate::simulation::SimulationResult;

    let mut figures = Vec::new();
    figures.extend(elapsed.map(|elapsed| crate::quantity::format_elapsed(elapsed.as_secs_f64())));
    // A transient-derived spectrum or FFT carries its source transient's
    // report; those steps were not this analysis's, so only a transient
    // solve states them.
    if let SimulationResult::Transient {
        convergence: Some(convergence),
        ..
    }
    | SimulationResult::Soa {
        convergence: Some(convergence),
        ..
    } = result
    {
        let report = &convergence.transient;
        match report.time_basis.as_ref() {
            Some(basis) => {
                figures.push(format!("{} steps accepted", grouped(basis.sample_count)));
                figures.push(format!("{} rejected", grouped(report.timestep_reductions)));
            }
            None => figures.push(format!(
                "{} steps rejected",
                grouped(report.timestep_reductions)
            )),
        }
        figures.push(format!(
            "{} Newton iterations",
            grouped(report.total_iterations)
        ));
    }
    if figures.is_empty() {
        format!("{label}: completed")
    } else {
        format!("{label}: {}", figures.join(", "))
    }
}

/// The product and version the About dialog shows, with its build when the
/// build knows it.
fn product_version() -> String {
    let build = env!("RSPICE_BUILD_HASH");
    if build == "unknown" {
        concat!("RSpice ", env!("CARGO_PKG_VERSION")).to_owned()
    } else {
        format!("RSpice {} ({build})", env!("CARGO_PKG_VERSION"))
    }
}

/// When the run started, on the reader's clock: `2026-10-16 14:03:12`.
/// `None` when the run's clock did not know.
fn local_start_time(unix_seconds: f64) -> Option<String> {
    if !(unix_seconds.is_finite() && unix_seconds > 0.0) {
        return None;
    }
    let local = crate::time_compat::local_date_time((unix_seconds * 1_000.0) as u64)?;
    Some(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        local.year(),
        u8::from(local.month()),
        local.day(),
        local.hour(),
        local.minute(),
        local.second()
    ))
}

fn platform(target: crate::state::ExecutionTarget) -> &'static str {
    match target {
        crate::state::ExecutionTarget::LocalDesktop => "desktop",
        crate::state::ExecutionTarget::BrowserWorker => "browser",
        crate::state::ExecutionTarget::NativeMobile => "tablet",
    }
}

/// A count with its thousands grouped: `5,512`.
fn grouped(count: u64) -> String {
    let digits = count.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index != 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_group_their_thousands() {
        assert_eq!(grouped(0), "0");
        assert_eq!(grouped(999), "999");
        assert_eq!(grouped(1_204), "1,204");
        assert_eq!(grouped(1_000_000), "1,000,000");
    }

    #[test]
    fn an_interrupted_run_failed_though_an_analysis_completed_first() {
        let mut state = AppState::default();
        let run_sequence = state.simulation.start_run().id;
        let run = state
            .simulation
            .retained
            .run_by_sequence_mut(run_sequence)
            .expect("the run is retained");
        run.mark_running().expect("the run starts");
        run.analyses.push(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::DcOp,
            "DC Operating Point",
        ));
        run.finish_lifecycle(SimulationRunLifecycle::Interrupted)
            .expect("the run is interrupted");
        let interruption = "Interrupted: the design it was running was closed or replaced";
        write_run_line(
            &mut state,
            Some(run_sequence),
            LogSeverity::Error,
            interruption,
        );

        announce_run_end(&mut state, run_sequence);

        let summary = state
            .log_buffer
            .entries()
            .last()
            .expect("the session's end line")
            .message
            .clone();
        assert!(
            summary.starts_with(&format!("Run {run_sequence} failed after "))
                && summary.ends_with(&format!(": {interruption}")),
            "{summary}"
        );
    }

    #[test]
    fn an_analysis_without_a_convergence_report_states_its_time() {
        let result = crate::simulation::SimulationResult::Ac {
            convergence: None,
            frequencies: Vec::new(),
            waveforms: Default::default(),
            measurements: Vec::new(),
            reference_impedances_ohm: None,
            noise_reference_temperature_kelvin: None,
        };
        assert_eq!(
            analysis_figures("AC", Some(std::time::Duration::from_millis(210)), &result),
            "AC: 0.210 s"
        );
    }
}
