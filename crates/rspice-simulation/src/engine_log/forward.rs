//! Lines the engine reports by callback rather than through the logger: a
//! digital testbench's own output and the Verilog-A compiler's findings.

use super::*;
use rspice_core::config::{HdlOutputRecord, HdlOutputSink, HdlSeverity, HdlStream};
use rspice_veriloga::{CompileDiagnosticSeverity, SourceCompileDiagnostic};

/// Delivers lines to the run that was executing on the thread that made it,
/// from whichever thread the engine later calls back on.
///
/// Neither producer is held to the warning limit: a testbench's `$display` is
/// the user's own output, and the engine hands each distinct compiler finding
/// over once.
#[derive(Clone)]
pub(crate) struct RunLineForwarder {
    scope: LogScope,
}

impl std::fmt::Debug for RunLineForwarder {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("RunLineForwarder")
    }
}

impl RunLineForwarder {
    /// The run executing on this thread, if one is.
    pub(crate) fn for_current_run() -> Option<Self> {
        let scope = log_scope::capture()?;
        scope.owner().downcast_ref::<ActiveRun>()?;
        Some(Self { scope })
    }

    fn forward(&self, mut line: EngineLogLine) {
        let Some(run) = self.scope.owner().downcast_ref::<ActiveRun>() else {
            return;
        };
        if !run.sink.admits(line.severity) {
            return;
        }
        // A Monte Carlo sample's testbench says which sample it is, as the
        // engine's own lines do, when the calling thread is inside this run.
        let point = log_scope::capture()
            .filter(|current| {
                std::ptr::addr_eq(current.owner() as *const _, self.scope.owner() as *const _)
            })
            .and_then(|current| current.point().map(str::to_owned));
        if let Some(point) = point {
            line.message = format!("{point}: {}", line.message);
        }
        run.deliver(|| line);
    }

    /// One nonfatal compiler finding, in the words the command line prints it.
    pub(crate) fn forward_compiler_diagnostic(&self, diagnostic: &SourceCompileDiagnostic) {
        self.forward(compiler_line(diagnostic));
    }

    /// An HDL output sink that writes this run's log.
    pub(crate) fn hdl_output(self) -> Arc<dyn HdlOutputSink> {
        Arc::new(HdlRunLog(self))
    }
}

#[derive(Debug)]
struct HdlRunLog(RunLineForwarder);

impl HdlOutputSink for HdlRunLog {
    fn emit(&self, record: &HdlOutputRecord) {
        if let Some(line) = hdl_line(record) {
            self.0.forward(line);
        }
    }
}

/// One HDL output record as a run log line, or `None` for one that is not
/// console output: a write to a file the design opened, or a `$finish(0)`,
/// which IEEE 1364-2005 section 17.4 has print nothing.
fn hdl_line(record: &HdlOutputRecord) -> Option<EngineLogLine> {
    let (severity, text) = match &record.stream {
        HdlStream::File { .. } => return None,
        HdlStream::Finish { level: 0, .. } => return None,
        HdlStream::Finish { stop, .. } => (
            LogSeverity::Info,
            format!(
                "{} at {}",
                if *stop { "$stop" } else { "$finish" },
                rspice_results::number_format::fmt_si_readout(record.seconds, "s", 6)
            ),
        ),
        HdlStream::Stdout => (LogSeverity::Info, record.text.clone()),
        HdlStream::Stderr => (LogSeverity::Warning, record.text.clone()),
        HdlStream::Diagnostic { severity } => (
            match severity {
                HdlSeverity::Info => LogSeverity::Info,
                HdlSeverity::Warning => LogSeverity::Warning,
                HdlSeverity::Error | HdlSeverity::Fatal => LogSeverity::Error,
            },
            record.text.clone(),
        ),
    };
    let text = text.trim_end_matches(['\r', '\n']);
    let origin = &record.origin;
    let location = (!origin.file.is_empty() && origin.line > 0)
        .then(|| format!("{}:{}", origin.file, origin.line));
    let message = match (origin.scope.as_str(), location) {
        ("", None) => text.to_owned(),
        ("", Some(location)) => format!("{text} \u{2014} {location}"),
        (scope, None) => format!("{text} \u{2014} {scope}"),
        (scope, Some(location)) => format!("{text} \u{2014} {scope} ({location})"),
    };
    Some(EngineLogLine {
        severity,
        message,
        origin: EngineLogOrigin::Hdl,
    })
}

/// The run log line for one compiler finding: the command line's own text,
/// which is the finding's `Display`, so the two never word it apart.
fn compiler_line(diagnostic: &SourceCompileDiagnostic) -> EngineLogLine {
    EngineLogLine {
        severity: match diagnostic.severity {
            CompileDiagnosticSeverity::Error => LogSeverity::Error,
            CompileDiagnosticSeverity::Warning => LogSeverity::Warning,
        },
        message: diagnostic.to_string(),
        origin: EngineLogOrigin::VerilogA,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::config::{HdlOrigin, HdlPoint};

    fn record(
        stream: HdlStream,
        text: &str,
        scope: &str,
        file: &str,
        line: u32,
    ) -> HdlOutputRecord {
        HdlOutputRecord {
            point: HdlPoint::Transient { time: 1.2e-6 },
            tick: 1200,
            seconds: 1.2e-6,
            stream,
            text: text.to_owned(),
            origin: HdlOrigin {
                scope: scope.to_owned(),
                file: file.to_owned(),
                line,
            },
        }
    }

    #[test]
    fn hdl_output_reads_as_the_testbench_wrote_it_and_says_where() {
        let display = hdl_line(&record(HdlStream::Stdout, "n=5\n", "tb", "tb.v", 12))
            .expect("a display is console output");
        assert_eq!(display.severity, LogSeverity::Info);
        assert_eq!(display.origin, EngineLogOrigin::Hdl);
        assert_eq!(display.message, "n=5 \u{2014} tb (tb.v:12)");

        let error = hdl_line(&record(
            HdlStream::Diagnostic {
                severity: HdlSeverity::Fatal,
            },
            "mismatch 7",
            "tb.check",
            "",
            0,
        ))
        .expect("a severity task is console output");
        assert_eq!(error.severity, LogSeverity::Error);
        assert_eq!(error.message, "mismatch 7 \u{2014} tb.check");

        let stderr = hdl_line(&record(HdlStream::Stderr, "to stderr", "", "tb.v", 3))
            .expect("a standard-error write is console output");
        assert_eq!(stderr.severity, LogSeverity::Warning);
        assert_eq!(stderr.message, "to stderr \u{2014} tb.v:3");

        let finish = hdl_line(&record(
            HdlStream::Finish {
                stop: false,
                level: 1,
            },
            "",
            "tb",
            "tb.v",
            20,
        ))
        .expect("$finish(1) is reported");
        assert_eq!(finish.severity, LogSeverity::Info);
        assert_eq!(
            finish.message,
            "$finish at 1.2 \u{b5}s \u{2014} tb (tb.v:20)"
        );
    }

    #[test]
    fn file_writes_and_a_silent_finish_are_not_console_output() {
        let file = HdlStream::File {
            handle: 3,
            path: "out.txt".into(),
        };
        assert!(hdl_line(&record(file, "row\n", "tb", "tb.v", 4)).is_none());
        let silent = HdlStream::Finish {
            stop: false,
            level: 0,
        };
        assert!(hdl_line(&record(silent, "", "tb", "tb.v", 9)).is_none());
    }

    /// A testbench prints every line it asks for: its output is not a warning
    /// flood for the limiter to hold back.
    #[test]
    fn forwarded_lines_reach_the_run_unlimited_from_any_thread() {
        let queue = Arc::new(Mutex::new(EngineLogQueue::default()));
        {
            let _guard = install(RunLogSink::queued(Arc::clone(&queue), false));
            let forwarder = RunLineForwarder::for_current_run().expect("a run is installed");
            let hdl = forwarder.clone().hdl_output();
            std::thread::scope(|threads| {
                threads.spawn(|| {
                    for index in 0..7 {
                        hdl.emit(&record(HdlStream::Stderr, &format!("w{index}"), "", "", 0));
                    }
                });
            });
            forwarder.forward_compiler_diagnostic(&SourceCompileDiagnostic {
                severity: CompileDiagnosticSeverity::Warning,
                phase: rspice_veriloga::CompileDiagnosticPhase::Semantic,
                code: "VA-SEM-NO-EFFECT-SYSTEM-TASK".into(),
                message: "$strobe has no effect in analog blocks".into(),
                path: Some("model.va".into()),
                byte_start: Some(0),
                byte_end: Some(1),
                line: Some(4),
                column: Some(5),
            });
        }
        let lines = lock_queue(&queue).drain();
        assert_eq!(lines.len(), 8, "{lines:?}");
        assert!(
            lines[..7]
                .iter()
                .all(|line| line.origin == EngineLogOrigin::Hdl)
        );
        assert_eq!(lines[7].origin, EngineLogOrigin::VerilogA);
        assert_eq!(lines[7].severity, LogSeverity::Warning);
        assert_eq!(
            lines[7].message,
            "Warning: model.va:4:5: [VA-SEM-NO-EFFECT-SYSTEM-TASK] $strobe has no effect in \
             analog blocks"
        );
        assert_eq!(lock_queue(&queue).suppressed_warnings(), 0);
    }

    #[test]
    fn there_is_no_forwarder_outside_a_run() {
        assert!(RunLineForwarder::for_current_run().is_none());
    }
}
