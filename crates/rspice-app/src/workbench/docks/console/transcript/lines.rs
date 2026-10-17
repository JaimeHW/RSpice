//! A channel's lines, and the layout the body draws them from.
//!
//! The session's log and a run's own log are kept in different shapes; this
//! reads either as the same rows. The layout is the part of a frame that does
//! not depend on where the body has been scrolled to: which lines the filters
//! show, how tall each one is, and where each one starts. It is kept from frame
//! to frame and only extended as lines arrive, so a ten-thousand-line session
//! costs what is on the screen.

use std::collections::HashMap;
use std::ops::Range;

use rspice_app_types::run_log::RunLog;

use crate::diagnostics::{LogAnchor, LogBuffer, LogSeverity};
use crate::state::SimulationRun;
use crate::workbench::state::{ConsoleChannel, ConsoleSeverity, ConsoleView};

/// One line of a channel.
#[derive(Debug, Clone, Copy)]
pub(super) struct Line<'a> {
    /// The line's identity in its channel: a session entry's id, or a run
    /// line's position in the run's log. Keys only grow down a log.
    pub(super) key: u64,
    pub(super) unix_ms: u64,
    pub(super) severity: LogSeverity,
    pub(super) message: &'a str,
    pub(super) anchor: Option<&'a LogAnchor>,
}

/// The log a channel reads.
#[derive(Debug, Clone, Copy)]
pub(super) enum Source<'a> {
    Session(&'a LogBuffer),
    Run(&'a RunLog),
}

impl<'a> Source<'a> {
    /// The log `channel` names, or `None` for a run the session no longer
    /// holds.
    pub(super) fn of(
        session: &'a LogBuffer,
        runs: &'a [SimulationRun],
        channel: ConsoleChannel,
    ) -> Option<Self> {
        match channel {
            ConsoleChannel::Session => Some(Self::Session(session)),
            ConsoleChannel::Run(sequence) => runs
                .iter()
                .find(|run| run.id == sequence)
                .map(|run| Self::Run(&run.log)),
        }
    }

    pub(super) fn len(self) -> usize {
        match self {
            Self::Session(log) => log.len(),
            Self::Run(log) => log.lines().len(),
        }
    }

    fn first_key(self) -> Option<u64> {
        match self {
            Self::Session(log) => log.entries().next().map(|entry| entry.id),
            Self::Run(log) => (!log.lines().is_empty()).then_some(0),
        }
    }

    /// The line `key`, if the log still holds it.
    pub(super) fn line(self, key: u64) -> Option<Line<'a>> {
        match self {
            Self::Session(log) => {
                // Entry ids are consecutive within the buffer, so the entry's
                // position is its distance from the first one.
                let first = log.entries().next()?.id;
                let position = usize::try_from(key.checked_sub(first)?).ok()?;
                log.entries()
                    .nth(position)
                    .filter(|entry| entry.id == key)
                    .or_else(|| log.entries().find(|entry| entry.id == key))
                    .map(session_line)
            }
            Self::Run(log) => {
                let index = usize::try_from(key).ok()?;
                log.lines().get(index).map(|line| Line {
                    key,
                    unix_ms: line.unix_ms,
                    severity: line.severity,
                    message: &line.message,
                    anchor: None,
                })
            }
        }
    }

    /// Every line from key `from` on, in order.
    fn lines_from(self, from: u64) -> Box<dyn Iterator<Item = Line<'a>> + 'a> {
        match self {
            Self::Session(log) => {
                let skip = log
                    .entries()
                    .next()
                    .map_or(0, |first| from.saturating_sub(first.id));
                Box::new(
                    log.entries()
                        .skip(usize::try_from(skip).unwrap_or(usize::MAX))
                        .filter(move |entry| entry.id >= from)
                        .map(session_line),
                )
            }
            Self::Run(log) => Box::new(
                log.lines()
                    .iter()
                    .enumerate()
                    .skip(usize::try_from(from).unwrap_or(usize::MAX))
                    .map(|(index, line)| Line {
                        key: index as u64,
                        unix_ms: line.unix_ms,
                        severity: line.severity,
                        message: &line.message,
                        anchor: None,
                    }),
            ),
        }
    }
}

fn session_line(entry: &crate::diagnostics::LogEntry) -> Line<'_> {
    Line {
        key: entry.id,
        unix_ms: u64::try_from(entry.timestamp.as_millis()).unwrap_or(u64::MAX),
        severity: entry.severity,
        message: &entry.message,
        anchor: entry.anchor.as_ref(),
    }
}

/// What a set of rows was built for: the channel and the filters on it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Filter {
    channel: ConsoleChannel,
    hidden: [bool; 3],
    /// The find text, lowercased.
    find: String,
}

impl Filter {
    fn of(view: &ConsoleView) -> Self {
        Self {
            channel: view.channel,
            hidden: view.hidden(),
            find: view.find.to_lowercase(),
        }
    }
}

/// The type a row is set in, measured from the font rather than assumed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Metrics {
    /// Width a message wraps at.
    pub(super) wrap: f32,
    /// Advance of one character of the message face.
    pub(super) advance: f32,
    /// Height of one line of the message face.
    pub(super) line_height: f32,
    /// Height of a one-line row.
    pub(super) row_height: f32,
}

impl Metrics {
    /// Space above a message's first line, which centres that line in a
    /// one-line row so a wrapped row's first line sits where a short row's
    /// only line does.
    pub(super) fn inset(self) -> f32 {
        ((self.row_height - self.line_height) * 0.5).max(0.0)
    }

    /// The height of a row whose message laid out `message_height` tall.
    pub(super) fn row_height_for(self, message_height: f32) -> f32 {
        (message_height + self.inset() * 2.0).max(self.row_height)
    }

    /// A row's height before it has been laid out, counted from its length:
    /// the message face is monospaced, so the count is right but for the odd
    /// line a word wrap breaks early, and the row corrects itself the first
    /// time it is drawn.
    fn estimate(self, message: &str) -> f32 {
        let per_line = ((self.wrap / self.advance.max(1.0)).floor() as usize).max(1);
        let lines: usize = message
            .split('\n')
            .map(|segment| segment.chars().count().div_ceil(per_line).max(1))
            .sum();
        self.row_height_for(lines as f32 * self.line_height)
    }
}

/// The shown rows of one channel, and where each one starts.
#[derive(Debug, Clone, Default)]
pub(super) struct Rows {
    filter: Option<Filter>,
    /// The source's first key and length when its lines were last read.
    front: Option<u64>,
    len: usize,
    /// The key after the last line read.
    next: u64,
    /// The keys of the shown lines, in log order.
    keys: Vec<u64>,
    /// Lines of each severity in the channel, shown or not.
    counts: [usize; 3],
    /// Lines of any severity that contain the find text.
    found: usize,
    metrics: Option<Metrics>,
    heights: Vec<f32>,
    /// Where each shown row starts, and one past the last.
    tops: Vec<f32>,
    tops_stale: bool,
    /// Heights rows laid out at the current metrics, by key.
    measured: HashMap<u64, f32>,
}

impl Rows {
    fn id() -> egui::Id {
        egui::Id::new("workbench.console.rows")
    }

    /// The rows kept from the last frame. Taken rather than cloned, so ten
    /// thousand keys are not copied every frame; [`Self::store`] puts them
    /// back.
    pub(super) fn take(ctx: &egui::Context) -> Self {
        ctx.data_mut(|data| std::mem::take(data.get_temp_mut_or_default::<Self>(Self::id())))
    }

    pub(super) fn store(self, ctx: &egui::Context) {
        ctx.data_mut(|data| data.insert_temp(Self::id(), self));
    }

    /// Bring the rows up to date with `source` under `view`'s filters. Lines
    /// that only arrived since the last frame are read on their own; anything
    /// else reads the channel again.
    pub(super) fn sync(&mut self, source: Source<'_>, view: &ConsoleView) {
        let filter = Filter::of(view);
        let front = source.first_key();
        let len = source.len();
        let appended =
            self.filter.as_ref() == Some(&filter) && self.front == front && len >= self.len;
        if appended && len == self.len {
            return;
        }
        if !appended {
            self.keys.clear();
            self.heights.clear();
            self.counts = [0; 3];
            self.found = 0;
            self.next = front.unwrap_or(0);
            self.tops_stale = true;
        }
        for line in source.lines_from(self.next) {
            self.next = line.key + 1;
            let severity = ConsoleSeverity::of(line.severity);
            self.counts[severity.index()] += 1;
            let found =
                filter.find.is_empty() || line.message.to_lowercase().contains(&filter.find);
            if found && !filter.find.is_empty() {
                self.found += 1;
            }
            if found && !filter.hidden[severity.index()] {
                self.keys.push(line.key);
            }
        }
        self.filter = Some(filter);
        self.front = front;
        self.len = len;
    }

    /// Give every shown row a height at `metrics`: the one it laid out at, or
    /// an estimate until it has been drawn. New metrics forget every measured
    /// height, which belonged to the old wrap width.
    pub(super) fn sync_heights(&mut self, source: Source<'_>, metrics: Metrics) {
        if self.metrics != Some(metrics) {
            self.metrics = Some(metrics);
            self.measured.clear();
            self.heights.clear();
        }
        if self.heights.len() > self.keys.len() {
            self.heights.clear();
        }
        if self.heights.len() < self.keys.len() {
            let start = self.heights.len();
            for key in &self.keys[start..] {
                let height = self.measured.get(key).copied().unwrap_or_else(|| {
                    source
                        .line(*key)
                        .map_or(metrics.row_height, |line| metrics.estimate(line.message))
                });
                self.heights.push(height);
            }
            self.tops_stale = true;
        }
        if self.tops_stale || self.tops.len() != self.keys.len() + 1 {
            self.tops.clear();
            self.tops.reserve(self.heights.len() + 1);
            let mut top = 0.0;
            self.tops.push(top);
            for height in &self.heights {
                top += height;
                self.tops.push(top);
            }
            self.tops_stale = false;
        }
    }

    /// Record the height row `index` laid out at. Returns whether it differed
    /// from the one the layout was using, which moves every row below it.
    pub(super) fn measure(&mut self, index: usize, height: f32) -> bool {
        let Some(key) = self.keys.get(index).copied() else {
            return false;
        };
        self.measured.insert(key, height);
        let Some(stored) = self.heights.get_mut(index) else {
            return false;
        };
        if (*stored - height).abs() <= 0.5 {
            return false;
        }
        *stored = height;
        self.tops_stale = true;
        true
    }

    pub(super) fn keys(&self) -> &[u64] {
        &self.keys
    }

    pub(super) fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Lines of `severity` in the channel, whether or not they are shown.
    pub(super) fn count(&self, severity: ConsoleSeverity) -> usize {
        self.counts[severity.index()]
    }

    /// Lines of any severity that contain the find text.
    pub(super) fn found(&self) -> usize {
        self.found
    }

    /// The height of every shown row together.
    pub(super) fn height(&self) -> f32 {
        self.tops.last().copied().unwrap_or(0.0)
    }

    /// Where shown row `index` starts.
    pub(super) fn top(&self, index: usize) -> f32 {
        self.tops
            .get(index)
            .copied()
            .unwrap_or_else(|| self.height())
    }

    /// Where shown row `index` ends.
    pub(super) fn bottom(&self, index: usize) -> f32 {
        self.top(index + 1)
    }

    /// The shown rows any part of which lies between `top` and `bottom`.
    pub(super) fn between(&self, top: f32, bottom: f32) -> Range<usize> {
        let count = self.keys.len();
        if self.tops.len() != count + 1 {
            return 0..0;
        }
        let start = self.tops[1..].partition_point(|end| *end <= top);
        let end = self.tops[..count].partition_point(|start| *start < bottom);
        start..end.max(start)
    }

    /// The shown row of `key`, or where it would be.
    pub(super) fn position(&self, key: u64) -> Result<usize, usize> {
        self.keys.binary_search(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::LogSource;

    fn metrics() -> Metrics {
        Metrics {
            wrap: 100.0,
            advance: 10.0,
            line_height: 14.0,
            row_height: 16.0,
        }
    }

    #[test]
    fn a_row_is_estimated_one_line_per_wrap_width_of_characters() {
        let metrics = metrics();
        assert_eq!(metrics.estimate("short"), 16.0);
        // 25 characters at 10 per line: three lines, plus the one-line inset
        // above and below.
        assert_eq!(metrics.estimate(&"x".repeat(25)), 3.0 * 14.0 + 2.0);
        assert_eq!(metrics.estimate("one\ntwo"), 2.0 * 14.0 + 2.0);
    }

    #[test]
    fn rows_follow_appended_lines_and_rebuild_when_the_front_moves() {
        let mut log = LogBuffer::new(3);
        log.log(LogSeverity::Info, LogSource::Studio, "a");
        log.log(LogSeverity::Error, LogSource::Studio, "b");
        let view = ConsoleView::default();
        let mut rows = Rows::default();
        rows.sync(Source::Session(&log), &view);
        assert_eq!(rows.keys(), [0, 1]);
        assert_eq!(rows.count(ConsoleSeverity::Errors), 1);

        log.log(LogSeverity::Warning, LogSource::Studio, "c");
        rows.sync(Source::Session(&log), &view);
        assert_eq!(rows.keys(), [0, 1, 2]);

        // The ring drops its first line; the rows drop it too.
        log.log(LogSeverity::Info, LogSource::Studio, "d");
        rows.sync(Source::Session(&log), &view);
        assert_eq!(rows.keys(), [1, 2, 3]);
        assert_eq!(rows.count(ConsoleSeverity::Info), 1);
        assert_eq!(log.evicted(), 1);

        log.clear();
        assert_eq!(log.evicted(), 0, "a clear is not an eviction");
        rows.sync(Source::Session(&log), &view);
        assert!(rows.is_empty());
    }

    #[test]
    fn find_and_severity_filters_narrow_the_rows_and_counts_stay_whole() {
        let mut log = LogBuffer::new(10);
        log.log(LogSeverity::Error, LogSource::Studio, "Netlist FAILED");
        log.log(LogSeverity::Info, LogSource::Studio, "netlist written");
        log.log(LogSeverity::Debug, LogSource::Studio, "unrelated");
        let mut view = ConsoleView::default();
        view.find = "NETLIST".to_owned();
        let mut rows = Rows::default();
        rows.sync(Source::Session(&log), &view);
        assert_eq!(rows.keys(), [0, 1], "find is case-insensitive");
        assert_eq!(rows.found(), 2);

        view.toggle(ConsoleSeverity::Info);
        rows.sync(Source::Session(&log), &view);
        assert_eq!(rows.keys(), [0]);
        assert_eq!(
            rows.count(ConsoleSeverity::Info),
            2,
            "a debug line counts as Info, and a count is the channel's"
        );
    }

    #[test]
    fn layout_places_rows_end_to_end_and_finds_the_ones_in_view() {
        let mut log = LogBuffer::new(10);
        for message in ["a", &"x".repeat(25), "c"] {
            log.log(LogSeverity::Info, LogSource::Studio, message);
        }
        let mut rows = Rows::default();
        rows.sync(Source::Session(&log), &ConsoleView::default());
        rows.sync_heights(Source::Session(&log), metrics());
        assert_eq!(rows.top(1), 16.0);
        assert_eq!(rows.bottom(1), 16.0 + 44.0);
        assert_eq!(rows.height(), 16.0 + 44.0 + 16.0);
        assert_eq!(rows.between(0.0, 10.0), 0..1);
        assert_eq!(rows.between(20.0, 70.0), 1..3);

        assert!(
            rows.measure(1, 30.0),
            "a measured height replaces the estimate"
        );
        rows.sync_heights(Source::Session(&log), metrics());
        assert_eq!(rows.height(), 16.0 + 30.0 + 16.0);
    }
}
