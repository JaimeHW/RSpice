//! One run's own log.
//!
//! What a run wrote about itself, in order, each line with the wall-clock time
//! it was written: its header, each analysis's figures, its summary, and
//! everything the engines it drove said along the way. The log belongs to the
//! run rather than to the session, so a verbose run cannot push the session's
//! history out of the Console, and a past run's log can be read again after
//! the project is reopened.

use serde::{Deserialize, Serialize};

use crate::diagnostics::{LogSeverity, LogSource};

/// Engine lines one run keeps before it counts them instead.
///
/// A solver trace is unbounded, and a run's log is read by a person. The run's
/// own lines do not count against the bound: there are a handful per analysis,
/// and they are the ones that say how the run went.
pub const RUN_LOG_ENGINE_LINES: usize = 2_000;

/// One line of a run's log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunLogLine {
    /// When the line was written, in milliseconds since the Unix epoch.
    pub unix_ms: u64,
    pub severity: LogSeverity,
    /// [`LogSource::Run`] for the run's own lines; the engine, the testbench
    /// or the Verilog-A compiler for theirs.
    pub source: LogSource,
    pub message: String,
}

/// A run's log, bounded at [`RUN_LOG_ENGINE_LINES`] engine lines.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunLog {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    lines: Vec<RunLogLine>,
    /// Engine lines past the bound, counted and not kept.
    #[serde(default, skip_serializing_if = "is_zero")]
    withheld: usize,
    /// Warnings the engine did not repeat because their call site had already
    /// said it would suppress the rest.
    #[serde(default, skip_serializing_if = "is_zero")]
    suppressed_warnings: usize,
}

fn is_zero(count: &usize) -> bool {
    *count == 0
}

impl RunLog {
    /// Append a line, or count it when the log already holds its bound of
    /// engine lines.
    pub fn push(&mut self, line: RunLogLine) {
        if line.source != LogSource::Run && self.is_full() {
            self.withheld = self.withheld.saturating_add(1);
            return;
        }
        self.lines.push(line);
    }

    /// Whether another engine line would be past the bound.
    ///
    /// Counted from the lines rather than kept alongside them, so a log read
    /// back from a project needs no second number to agree with. The count is
    /// taken only once the log is long enough to be full, and only until it
    /// is: a withheld line means it is.
    fn is_full(&self) -> bool {
        self.withheld > 0
            || (self.lines.len() >= RUN_LOG_ENGINE_LINES
                && self
                    .lines
                    .iter()
                    .filter(|line| line.source != LogSource::Run)
                    .count()
                    >= RUN_LOG_ENGINE_LINES)
    }

    /// Account for an engine queue that bounded itself and closed with one
    /// row standing for the lines it withheld.
    ///
    /// The queue keeps at most [`RUN_LOG_ENGINE_LINES`] lines of one analysis,
    /// and every line it keeps arrives here, so by the time it withholds
    /// anything this log is already full and counts the queue's closing row
    /// as one withheld line. That row stood for `withheld` lines; the rest are
    /// counted here. The queue's suppressed warnings join the run's.
    pub fn absorb_engine_seal(&mut self, withheld: usize, suppressed_warnings: usize) {
        self.withheld = self.withheld.saturating_add(withheld.saturating_sub(1));
        self.suppressed_warnings = self.suppressed_warnings.saturating_add(suppressed_warnings);
    }

    /// Close the log with one row for the engine lines it withheld, if any.
    pub fn seal(&mut self, unix_ms: u64) {
        if self.withheld == 0 {
            return;
        }
        self.lines.push(RunLogLine {
            unix_ms,
            severity: LogSeverity::Info,
            source: LogSource::Run,
            message: format!(
                "\u{2026} {} further engine lines were not shown (the Console keeps the first \
                 {RUN_LOG_ENGINE_LINES} of a run)",
                self.withheld
            ),
        });
    }

    pub fn lines(&self) -> &[RunLogLine] {
        &self.lines
    }

    /// Lines of one severity the log holds.
    pub fn count(&self, severity: LogSeverity) -> usize {
        self.lines
            .iter()
            .filter(|line| line.severity == severity)
            .count()
    }

    /// Engine lines past the bound, counted and not kept.
    pub fn withheld(&self) -> usize {
        self.withheld
    }

    /// Warnings the engine suppressed after a call site's fifth.
    pub fn suppressed_warnings(&self) -> usize {
        self.suppressed_warnings
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty() && self.withheld == 0 && self.suppressed_warnings == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(source: LogSource, message: &str) -> RunLogLine {
        RunLogLine {
            unix_ms: 1_760_000_000_000,
            severity: LogSeverity::Info,
            source,
            message: message.to_owned(),
        }
    }

    fn full_of_engine_lines() -> RunLog {
        let mut log = RunLog::default();
        for index in 0..RUN_LOG_ENGINE_LINES {
            log.push(line(LogSource::Engine, &format!("step {index}")));
        }
        log
    }

    #[test]
    fn engine_lines_past_the_bound_are_counted_and_the_runs_own_lines_are_kept() {
        let mut log = full_of_engine_lines();
        log.push(line(LogSource::Hdl, "one more"));
        log.push(line(LogSource::Engine, "and another"));
        log.push(line(LogSource::Run, "Transient: 1.32 s"));

        assert_eq!(log.lines().len(), RUN_LOG_ENGINE_LINES + 1);
        assert_eq!(log.withheld(), 2);
        assert_eq!(
            log.lines().last().map(|line| line.message.as_str()),
            Some("Transient: 1.32 s")
        );
    }

    #[test]
    fn the_runs_own_lines_do_not_count_against_the_bound() {
        let mut log = RunLog::default();
        log.push(line(LogSource::Run, "Run 1"));
        for index in 0..RUN_LOG_ENGINE_LINES {
            log.push(line(LogSource::Engine, &format!("step {index}")));
        }
        assert_eq!(log.withheld(), 0);
        log.push(line(LogSource::VerilogA, "past the bound"));
        assert_eq!(log.withheld(), 1);
    }

    /// The queue's own closing row is one withheld line here and stood for
    /// all of them, so the run states the queue's number, not one.
    #[test]
    fn an_engine_queues_seal_is_counted_once_at_its_own_number() {
        let mut log = full_of_engine_lines();
        log.push(line(
            LogSource::Engine,
            "\u{2026} 41 further engine lines were not shown",
        ));
        log.absorb_engine_seal(41, 3);
        log.seal(1_760_000_000_500);

        assert_eq!(log.withheld(), 41);
        assert_eq!(log.suppressed_warnings(), 3);
        assert_eq!(
            log.lines().last().map(|line| line.message.as_str()),
            Some(
                "\u{2026} 41 further engine lines were not shown (the Console keeps the first \
                 2000 of a run)"
            )
        );
    }

    #[test]
    fn a_log_that_withheld_nothing_seals_without_a_row() {
        let mut log = RunLog::default();
        log.push(line(
            LogSource::Engine,
            "Transient noise: 2 device noise source(s)",
        ));
        log.absorb_engine_seal(0, 2);
        log.seal(0);
        assert_eq!(log.lines().len(), 1);
        assert_eq!(log.suppressed_warnings(), 2);
    }

    #[test]
    fn an_empty_log_costs_nothing_on_disk_and_reads_back_whole() {
        assert_eq!(
            serde_json::to_string(&RunLog::default()).expect("serializes"),
            "{}"
        );
        let mut log = RunLog::default();
        log.push(RunLogLine {
            severity: LogSeverity::Warning,
            ..line(LogSource::Hdl, "edge 1 \u{2014} tb (tb.v:9)")
        });
        log.absorb_engine_seal(0, 4);
        let text = serde_json::to_string(&log).expect("serializes");
        let read: RunLog = serde_json::from_str(&text).expect("deserializes");
        assert_eq!(read, log);
        assert_eq!(read.count(LogSeverity::Warning), 1);
    }
}
