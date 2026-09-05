//! Raw Waveform Export
//!
//! Provides standard .raw file export for simulation results.
//! Supports both binary (fast, compact) and ASCII (human-readable) formats.
//!
//! # Format Specification
//! .raw files have a header followed by data:
//! ```text
//! Title: <simulation name>
//! Date: <date>
//! Plotname: Transient Analysis
//! Flags: real
//! No. Variables: <N>
//! No. Points: <M>
//! Variables:
//!     0   time    time
//!     1   V(1)    voltage
//!     ...
//! Binary: / Values:
//! <data>
//! ```

use crate::Value;
use rspice_output::{AtomicArtifactError, write_atomic};
use std::io::{self, Write};
use std::path::Path;

/// Export format for raw files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawFormat {
    /// Binary format (compact, fast to load)
    Binary,
    /// ASCII format (human-readable)
    Ascii,
}

/// Variable type in raw file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    Time,
    Frequency,
    Voltage,
    Current,
    /// Event-driven digital node, carrying one XSPICE event code per point.
    Digital,
    /// Event-driven real-valued node, carrying its value at each event.
    Real,
}

impl VariableType {
    /// The rawfile spelling of this type, as it appears in a variable line.
    pub fn as_str(&self) -> &'static str {
        match self {
            VariableType::Time => "time",
            VariableType::Frequency => "frequency",
            VariableType::Voltage => "voltage",
            VariableType::Current => "current",
            VariableType::Digital => "digital",
            VariableType::Real => "real",
        }
    }
}

/// Which event family an appended rawfile plot carries.
///
/// Event histories are irregular: a node's value changes when it changes, at
/// times of its own, so it cannot share the analysis plot's row grid without
/// being resampled onto it. Each node therefore gets its own plot, holding its
/// own time column and its own value column, appended after the analysis plot
/// a rawfile opens with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawEventKind {
    /// XSPICE digital events. Values are the event codes
    /// [`crate::xspice::DigitalValue::event_code`] produces, `0..=12`, which
    /// name the resolved state and drive strength exactly.
    Digital,
    /// XSPICE real-valued events. Values are the event values themselves.
    Real,
}

impl RawEventKind {
    /// The `Plotname:` this family writes.
    ///
    /// The plot name is the version carrier. A rawfile has no header key for
    /// a private extension: ngspice executes any `Command:` line it does not
    /// recognise as its own and aborts the load on any header line it cannot
    /// key, while a plot name is free text to every reader of the format.
    pub const fn plot_name(self) -> &'static str {
        match self {
            Self::Digital => "Digital Events (rspice-digital-events/1)",
            Self::Real => "Real Events (rspice-real-events/1)",
        }
    }

    /// The family a plot name declares, or `None` for an ordinary plot.
    pub fn from_plot_name(plot_name: &str) -> Option<Self> {
        let plot_name = plot_name.trim();
        [Self::Digital, Self::Real]
            .into_iter()
            .find(|kind| kind.plot_name() == plot_name)
    }

    /// The rawfile variable type of this family's value column.
    pub const fn variable_type(self) -> VariableType {
        match self {
            Self::Digital => VariableType::Digital,
            Self::Real => VariableType::Real,
        }
    }

    /// The variable name one node's timeline is declared under.
    ///
    /// `D(..)` for digital and `E(..)` for real is the spelling the workbench
    /// event sheet already reads, so a rawfile written here is one an existing
    /// reader recognises rather than a second convention.
    pub fn variable_name(self, node_name: &str) -> String {
        match self {
            Self::Digital => format!("D({node_name})"),
            Self::Real => format!("E({node_name})"),
        }
    }

    /// The node one variable name spells, when it is spelled for this family.
    pub fn node_name(self, variable_name: &str) -> Option<&str> {
        let prefix = match self {
            Self::Digital => "D(",
            Self::Real => "E(",
        };
        variable_name
            .strip_prefix(prefix)
            .and_then(|rest| rest.strip_suffix(')'))
    }
}

/// One node's event timeline, ready to be written as its own rawfile plot.
///
/// The times are the accepted event times the run recorded, kept exactly as
/// recorded; nothing here is resampled, decimated, or interpolated.
#[derive(Debug, Clone, PartialEq)]
pub struct RawEventTimeline {
    /// Event family this timeline belongs to.
    pub kind: RawEventKind,
    /// Netlist node name, wrapped in the family's spelling on the way out.
    pub node_name: String,
    /// Event times in seconds, one per event.
    pub times: Vec<Value>,
    /// Value at each event time: an event code for
    /// [`RawEventKind::Digital`], the value itself for [`RawEventKind::Real`].
    pub values: Vec<Value>,
}

/// A variable (signal) in the raw file
#[derive(Debug, Clone)]
pub struct RawVariable {
    /// Variable name (e.g., "V(1)", "I(R1)")
    pub name: String,
    /// Variable type
    pub var_type: VariableType,
}

impl RawVariable {
    /// Create a time variable
    pub fn time() -> Self {
        Self {
            name: "time".to_string(),
            var_type: VariableType::Time,
        }
    }

    /// Create a frequency variable
    pub fn frequency() -> Self {
        Self {
            name: "frequency".to_string(),
            var_type: VariableType::Frequency,
        }
    }

    /// Create a voltage variable
    pub fn voltage(node: &str) -> Self {
        Self {
            name: format!("V({})", node),
            var_type: VariableType::Voltage,
        }
    }

    /// Create a current variable
    pub fn current(element: &str) -> Self {
        Self {
            name: format!("I({})", element),
            var_type: VariableType::Current,
        }
    }

    /// Create the value variable of one node's event plot.
    pub fn event(kind: RawEventKind, node_name: &str) -> Self {
        Self {
            name: kind.variable_name(node_name),
            var_type: kind.variable_type(),
        }
    }
}

/// Raw file writer for exporting simulation results
pub struct RawExporter {
    /// Title of the simulation
    title: String,
    /// Plot name (e.g., "Transient Analysis")
    plot_name: String,
    /// Variables in the file
    variables: Vec<RawVariable>,
    /// Data points (time/freq values in column 0, then signals)
    data: Vec<Vec<Value>>,
}

impl RawExporter {
    /// Create a new raw exporter for transient analysis
    pub fn new_transient(title: &str) -> Self {
        Self {
            title: title.to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables: vec![RawVariable::time()],
            data: Vec::new(),
        }
    }

    /// Create an exporter for one node's event timeline.
    ///
    /// The plot carries exactly two variables — the node's own event times and
    /// its values — so an event plot is a complete, self-describing plot and
    /// not a private layout appended to somebody else's.
    pub fn new_event_plot(kind: RawEventKind, node_name: &str) -> Self {
        Self {
            title: kind.plot_name().to_string(),
            plot_name: kind.plot_name().to_string(),
            variables: vec![RawVariable::time(), RawVariable::event(kind, node_name)],
            data: Vec::new(),
        }
    }

    /// Add a voltage variable
    pub fn add_voltage(&mut self, node: &str) {
        self.variables.push(RawVariable::voltage(node));
    }

    /// Add a current variable
    pub fn add_current(&mut self, element: &str) {
        self.variables.push(RawVariable::current(element));
    }

    /// Add a data point (all values for one time/frequency point)
    /// First value should be time/frequency, rest are signal values
    pub fn add_point(&mut self, values: Vec<Value>) {
        if values.len() == self.variables.len() {
            self.data.push(values);
        }
    }

    /// Add data from transient result
    pub fn add_transient_data(
        &mut self,
        times: &[Value],
        waveforms: &[Vec<Value>],
    ) -> io::Result<()> {
        validate_series_lengths("time", times.len(), &self.variables[1..], waveforms)?;
        for (i, &t) in times.iter().enumerate() {
            let mut point = vec![t];
            for waveform in waveforms {
                point.push(waveform[i]);
            }
            self.data.push(point);
        }
        Ok(())
    }

    /// Number of data points
    pub fn num_points(&self) -> usize {
        self.data.len()
    }

    /// Number of variables
    pub fn num_variables(&self) -> usize {
        self.variables.len()
    }

    /// Write to file in specified format
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P, format: RawFormat) -> io::Result<()> {
        publish_raw_file(path.as_ref(), |writer| self.write(writer, format))
    }

    /// Write to any writer
    pub fn write<W: Write + ?Sized>(&self, writer: &mut W, format: RawFormat) -> io::Result<()> {
        self.validate_data_shape()?;
        self.write_header(writer)?;

        match format {
            RawFormat::Binary => {
                writeln!(writer, "Binary:")?;
                for point in &self.data {
                    for &value in point {
                        writer.write_all(&value.to_le_bytes())?;
                    }
                }
            }
            RawFormat::Ascii => {
                writeln!(writer, "Values:")?;
                for (i, point) in self.data.iter().enumerate() {
                    write!(writer, "{}", i)?;
                    for &value in point {
                        write!(writer, "\t{:.17e}", value)?;
                    }
                    writeln!(writer)?;
                }
            }
        }

        Ok(())
    }

    /// Write Xyce-compatible ASCII RAW data.
    ///
    /// Xyce writes one variable value per physical line: the first line of a
    /// point contains its index and axis value, followed by one line for each
    /// dependent variable. This is distinct from the row-oriented ASCII
    /// layout emitted by [`RawFormat::Ascii`], while sharing the same header
    /// and typed variable schema.
    pub fn write_xyce_ascii<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        self.validate_data_shape()?;
        self.write_header(writer)?;
        writeln!(writer, "Values:")?;
        for (index, point) in self.data.iter().enumerate() {
            let Some((axis, signals)) = point.split_first() else {
                return Err(invalid_data(format!(
                    "raw point {index} contains no axis value"
                )));
            };
            writeln!(writer, "{index}\t{axis:.8e}")?;
            for value in signals {
                writeln!(writer, "\t{value:.8e}")?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }

    fn write_header<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "Title: {}", self.title)?;
        writeln!(writer, "Date: {}", chrono_date())?;
        writeln!(writer, "Plotname: {}", self.plot_name)?;
        writeln!(writer, "Flags: real")?;
        writeln!(writer, "No. Variables: {}", self.variables.len())?;
        writeln!(writer, "No. Points: {}", self.data.len())?;
        writeln!(writer, "Variables:")?;
        for (index, variable) in self.variables.iter().enumerate() {
            writeln!(
                writer,
                "\t{}\t{}\t{}",
                index,
                variable.name,
                variable.var_type.as_str()
            )?;
        }
        Ok(())
    }

    fn validate_data_shape(&self) -> io::Result<()> {
        for (idx, point) in self.data.iter().enumerate() {
            if point.len() != self.variables.len() {
                return Err(invalid_data(format!(
                    "raw point {idx} has {} values for {} variables",
                    point.len(),
                    self.variables.len()
                )));
            }
        }
        Ok(())
    }
}

fn publish_raw_file(
    destination: &Path,
    write: impl FnOnce(&mut dyn Write) -> io::Result<()>,
) -> io::Result<()> {
    write_atomic(destination, write).map_err(AtomicArtifactError::into_io_error)
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn validate_series_lengths(
    axis_name: &str,
    axis_len: usize,
    variables: &[RawVariable],
    series: &[Vec<Value>],
) -> io::Result<()> {
    if series.len() != variables.len() {
        return Err(invalid_data(format!(
            "raw export has {} signal vectors for {} variables",
            series.len(),
            variables.len()
        )));
    }
    for (variable, values) in variables.iter().zip(series) {
        if values.len() != axis_len {
            return Err(invalid_data(format!(
                "raw export variable {} has {} samples but {axis_name} axis has {axis_len}",
                variable.name,
                values.len()
            )));
        }
    }
    Ok(())
}

/// Get current date/time string (std only, no chrono dependency)
fn chrono_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    asctime_utc(secs)
}

/// asctime-style timestamp (UTC) like ngspice writes in rawfile headers.
/// Civil-date conversion uses Hinnant's days-from-epoch algorithm;
/// 1970-01-01 was a Thursday.
fn asctime_utc(secs: u64) -> String {
    const WEEKDAYS: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let days = (secs / 86_400) as i64;
    let tod = secs % 86_400;
    let (hour, minute, second) = (tod / 3600, (tod % 3600) / 60, tod % 60);

    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);

    format!(
        "{} {} {} {:02}:{:02}:{:02} {}",
        WEEKDAYS[days.rem_euclid(7) as usize],
        MONTHS[(month - 1) as usize],
        day,
        hour,
        minute,
        second,
        year
    )
}

//=============================================================================
// Convenience functions for engine integration
//=============================================================================

/// Export transient results to raw file
pub fn export_transient<P: AsRef<Path>>(
    path: P,
    times: &[Value],
    node_names: &[String],
    waveforms: &[Vec<Value>],
    format: RawFormat,
) -> io::Result<()> {
    publish_raw_file(path.as_ref(), |writer| {
        write_transient(writer, times, node_names, waveforms, format)
    })
}

/// Write transient results to any writer.
pub fn write_transient<W: Write + ?Sized>(
    writer: &mut W,
    times: &[Value],
    node_names: &[String],
    waveforms: &[Vec<Value>],
    format: RawFormat,
) -> io::Result<()> {
    let mut exporter = RawExporter::new_transient("Transient Analysis");

    for name in node_names {
        exporter.add_voltage(name);
    }

    exporter.add_transient_data(times, waveforms)?;
    exporter.write(writer, format)
}

/// Append event timelines to an open rawfile, one plot per node.
///
/// The writer is called after an analysis plot's data block and leaves that
/// plot byte-identical: a rawfile is a sequence of plots, each bounded by its
/// own `No. Points`, so a reader that wants only the analysis reads the first
/// plot and stops exactly where it stopped before.
///
/// A timeline is refused rather than written when it cannot be read back:
/// mismatched time and value counts, no events at all — a plot declaring zero
/// points has no boundary a reader can find — or a node name carrying
/// whitespace, which rawfile variable declarations have no way to quote.
pub fn write_event_plots<W: Write + ?Sized>(
    writer: &mut W,
    timelines: &[RawEventTimeline],
    format: RawFormat,
) -> io::Result<()> {
    for timeline in timelines {
        if timeline.node_name.chars().any(char::is_whitespace) {
            return Err(invalid_data(format!(
                "event node name '{}' contains whitespace, which a rawfile variable declaration cannot carry",
                timeline.node_name
            )));
        }
        if timeline.times.is_empty() {
            return Err(invalid_data(format!(
                "event plot for '{}' carries no events; a plot declaring zero points has no boundary a reader can find",
                timeline.node_name
            )));
        }
        let mut exporter = RawExporter::new_event_plot(timeline.kind, &timeline.node_name);
        exporter.add_transient_data(&timeline.times, std::slice::from_ref(&timeline.values))?;
        exporter.write(writer, format)?;
    }
    Ok(())
}

/// Export DC sweep results to raw file
pub fn export_dc_sweep<P: AsRef<Path>>(
    path: P,
    sweep_values: &[Value],
    sweep_name: &str,
    node_names: &[String],
    results: &[Vec<Value>],
    format: RawFormat,
) -> io::Result<()> {
    publish_raw_file(path.as_ref(), |writer| {
        write_dc_sweep(
            writer,
            sweep_values,
            sweep_name,
            node_names,
            results,
            format,
        )
    })
}

/// Write DC sweep results to any writer.
pub fn write_dc_sweep<W: Write + ?Sized>(
    writer: &mut W,
    sweep_values: &[Value],
    sweep_name: &str,
    node_names: &[String],
    results: &[Vec<Value>],
    format: RawFormat,
) -> io::Result<()> {
    let mut exporter = RawExporter {
        title: "DC Sweep".to_string(),
        plot_name: "DC transfer characteristic".to_string(),
        variables: vec![RawVariable {
            name: sweep_name.to_string(),
            var_type: VariableType::Voltage,
        }],
        data: Vec::new(),
    };

    for name in node_names {
        exporter.add_voltage(name);
    }

    validate_series_lengths(
        "sweep",
        sweep_values.len(),
        &exporter.variables[1..],
        results,
    )?;

    for (i, &sweep_val) in sweep_values.iter().enumerate() {
        let mut point = vec![sweep_val];
        for result in results {
            point.push(result[i]);
        }
        exporter.data.push(point);
    }

    exporter.write(writer, format)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod date_tests {
    use super::asctime_utc;

    #[test]
    fn asctime_matches_reference_dates() {
        assert_eq!(asctime_utc(0), "Thu Jan 1 00:00:00 1970");
        assert_eq!(asctime_utc(1_750_000_000), "Sun Jun 15 15:06:40 2025");
    }
}

#[cfg(test)]
mod export_tests {
    use super::*;
    use crate::io::parse_raw_reader;
    use rspice_output::stale_artifacts;
    use std::fs;
    use std::io::Cursor;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let serial = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let directory = std::env::temp_dir().join(format!(
                "rspice-raw-export-{tag}-{}-{serial}",
                std::process::id()
            ));
            fs::create_dir(&directory).expect("create raw-export test directory");
            Self(directory)
        }

        fn destination(&self, file_name: &str) -> PathBuf {
            self.0.join(file_name)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn seed_existing(destination: &Path) {
        fs::write(destination, b"old complete raw artifact").expect("seed existing raw artifact");
    }

    fn assert_existing_preserved(destination: &Path) {
        assert_eq!(
            fs::read(destination).expect("read preserved raw artifact"),
            b"old complete raw artifact"
        );
        assert!(
            stale_artifacts(destination)
                .expect("inspect raw staging artifacts")
                .is_empty(),
            "failed export left a staging artifact"
        );
    }

    fn assert_committed_raw(
        destination: &Path,
        expected_plot_name: &str,
        expected_variables: usize,
        expected_points: usize,
    ) {
        let bytes = fs::read(destination).expect("read committed raw artifact");
        assert_ne!(bytes, b"old complete raw artifact");
        let parsed =
            parse_raw_reader(&mut Cursor::new(bytes)).expect("parse committed raw artifact");
        assert_eq!(parsed.header.plotname, expected_plot_name);
        assert_eq!(parsed.header.no_variables, expected_variables);
        assert_eq!(parsed.header.no_points, expected_points);
        assert!(
            stale_artifacts(destination)
                .expect("inspect raw staging artifacts")
                .is_empty(),
            "successful export left a staging artifact"
        );
    }

    fn analysis_plot(format: RawFormat) -> Vec<u8> {
        let mut exporter = RawExporter::new_transient("Transient Analysis");
        exporter.add_voltage("out");
        exporter
            .add_transient_data(&[0.0, 1.0e-9], &[vec![1.0, 2.5]])
            .expect("add shaped transient data");
        let mut bytes = Vec::new();
        exporter
            .write(&mut bytes, format)
            .expect("write the analysis plot");
        bytes
    }

    fn event_timelines() -> Vec<RawEventTimeline> {
        vec![
            RawEventTimeline {
                kind: RawEventKind::Digital,
                node_name: "clk".to_string(),
                times: vec![0.0, 2.5e-10, 7.0e-10],
                values: vec![9.0, 1.0, 12.0],
            },
            RawEventTimeline {
                kind: RawEventKind::Real,
                node_name: "vctrl".to_string(),
                times: vec![0.0, 5.0e-10],
                values: vec![-1.234_567_890_123_456_7e-3, 9.876_543_210_987_654e6],
            },
        ]
    }

    #[test]
    fn an_event_plot_declares_a_versioned_plot_name_and_nothing_a_reader_would_execute() {
        let mut bytes = Vec::new();
        write_event_plots(
            &mut bytes,
            std::slice::from_ref(&event_timelines()[0]),
            RawFormat::Ascii,
        )
        .expect("write one digital event plot");
        let text = std::str::from_utf8(&bytes).expect("ASCII event plot is UTF-8");
        let lines: Vec<&str> = text.lines().collect();

        assert_eq!(lines[0], "Title: Digital Events (rspice-digital-events/1)");
        assert!(lines[1].starts_with("Date: "), "{:?}", lines[1]);
        assert_eq!(
            lines[2],
            "Plotname: Digital Events (rspice-digital-events/1)"
        );
        assert_eq!(lines[3], "Flags: real");
        assert_eq!(lines[4], "No. Variables: 2");
        assert_eq!(lines[5], "No. Points: 3");
        assert_eq!(lines[6], "Variables:");
        assert_eq!(lines[7], "\t0\ttime\ttime");
        assert_eq!(lines[8], "\t1\tD(clk)\tdigital");
        assert_eq!(lines[9], "Values:");
        // ngspice executes any `Command:` line that is not its own, so an
        // extension must not carry its version there.
        assert!(
            !lines.iter().any(|line| line.starts_with("Command:")),
            "an event plot must not write a Command: line"
        );
    }

    #[test]
    fn appended_event_plots_leave_the_analysis_plot_exactly_as_written() {
        for format in [RawFormat::Binary, RawFormat::Ascii] {
            let alone = analysis_plot(format);
            let mut events = Vec::new();
            write_event_plots(&mut events, &event_timelines(), format)
                .expect("write the event plots on their own");

            let mut combined = analysis_plot(format);
            write_event_plots(&mut combined, &event_timelines(), format)
                .expect("append the event plots");

            // The event writer contributes its own plots and nothing else: no
            // separator, no trailing marker, nothing that reaches backwards.
            let mut expected = alone.clone();
            expected.extend_from_slice(&events);
            assert_eq!(combined, expected, "{format:?}");

            // And the reader every existing caller uses still sees the
            // analysis plot, unchanged.
            let single = parse_raw_reader(&mut Cursor::new(combined))
                .expect("the first plot still parses on its own");
            let reference =
                parse_raw_reader(&mut Cursor::new(alone)).expect("reference plot parses");
            assert_eq!(single.header.no_points, reference.header.no_points);
            assert_eq!(single.header.no_variables, reference.header.no_variables);
            assert_eq!(
                single.waveforms[0].y, reference.waveforms[0].y,
                "{format:?}"
            );
            assert_eq!(
                single.waveforms[1].y, reference.waveforms[1].y,
                "{format:?}"
            );
        }
    }

    #[test]
    fn event_plot_times_and_values_survive_both_encodings_exactly() {
        for format in [RawFormat::Binary, RawFormat::Ascii] {
            let mut bytes = analysis_plot(format);
            write_event_plots(&mut bytes, &event_timelines(), format)
                .expect("append the event plots");

            let file = crate::io::parse_raw_plots_reader_with_limits(
                &mut Cursor::new(bytes),
                crate::resource::ResourceLimits::default(),
            )
            .expect("parse every plot");
            assert_eq!(file.plots.len(), 3, "{format:?}");

            for (plot, timeline) in file.plots.iter().skip(1).zip(event_timelines()) {
                assert_eq!(
                    plot.header.plotname,
                    timeline.kind.plot_name(),
                    "{format:?}"
                );
                assert_eq!(
                    plot.variables[1].name,
                    timeline.kind.variable_name(&timeline.node_name),
                    "{format:?}"
                );
                assert_eq!(
                    plot.variables[1].var_type,
                    timeline.kind.variable_type().as_str(),
                    "{format:?}"
                );
                assert_eq!(plot.waveforms[0].y, timeline.times, "{format:?}");
                assert_eq!(plot.waveforms[1].y, timeline.values, "{format:?}");
            }
        }
    }

    #[test]
    fn a_timeline_a_reader_could_not_bound_is_refused_before_anything_is_written() {
        let unwritable = [
            RawEventTimeline {
                kind: RawEventKind::Digital,
                node_name: "quiet".to_string(),
                times: Vec::new(),
                values: Vec::new(),
            },
            RawEventTimeline {
                kind: RawEventKind::Digital,
                node_name: "two words".to_string(),
                times: vec![0.0],
                values: vec![1.0],
            },
            RawEventTimeline {
                kind: RawEventKind::Real,
                node_name: "vctrl".to_string(),
                times: vec![0.0, 1.0e-9],
                values: vec![1.0],
            },
        ];

        for timeline in unwritable {
            let mut bytes = Vec::new();
            let error = write_event_plots(&mut bytes, &[timeline], RawFormat::Ascii)
                .expect_err("an unreadable event plot must not be written");
            assert_eq!(error.kind(), io::ErrorKind::InvalidData);
            assert!(bytes.is_empty(), "refusal must not leave a partial plot");
        }
    }

    #[test]
    fn xyce_ascii_export_uses_vertical_layout_and_roundtrips() {
        let mut exporter = RawExporter::new_transient("Xyce ASCII regression");
        exporter.add_voltage("1");
        exporter.add_current("R1");
        exporter
            .add_transient_data(&[0.0, 1.0], &[vec![0.0, 1.0], vec![-0.0, 1.0]])
            .expect("add shaped transient data");

        let mut bytes = Vec::new();
        exporter
            .write_xyce_ascii(&mut bytes)
            .expect("write Xyce-layout ASCII RAW");
        let text = std::str::from_utf8(&bytes).expect("ASCII RAW is UTF-8");
        assert!(text.contains(
            "Values:\n0\t0.00000000e0\n\t0.00000000e0\n\t-0.00000000e0\n\n1\t1.00000000e0"
        ));

        let parsed = parse_raw_reader(&mut Cursor::new(bytes))
            .expect("parse exported Xyce-layout ASCII RAW");
        assert!(!parsed.header.is_binary);
        assert_eq!(parsed.header.no_variables, 3);
        assert_eq!(parsed.header.no_points, 2);
        assert_eq!(
            parsed
                .variables
                .iter()
                .map(|variable| variable.name.as_str())
                .collect::<Vec<_>>(),
            ["time", "V(1)", "I(R1)"]
        );
        assert_eq!(parsed.waveforms[0].y, [0.0, 1.0]);
        assert_eq!(parsed.waveforms[1].y, [0.0, 1.0]);
        assert_eq!(parsed.waveforms[2].y, [-0.0, 1.0]);
    }

    #[test]
    fn transient_export_rejects_waveform_length_mismatch() {
        let mut out = Vec::new();
        let err = write_transient(
            &mut out,
            &[0.0, 1.0, 2.0],
            &["out".to_string()],
            &[vec![0.0, 1.0]],
            RawFormat::Ascii,
        )
        .expect_err("truncated transient waveform must not be zero-padded");

        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(
            err.to_string().contains("V(out)"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn dc_sweep_export_rejects_result_length_mismatch() {
        let mut out = Vec::new();
        let err = write_dc_sweep(
            &mut out,
            &[0.0, 1.0, 2.0],
            "vin",
            &["out".to_string()],
            &[vec![0.0, 1.0]],
            RawFormat::Ascii,
        )
        .expect_err("truncated DC sweep result must not be zero-padded");

        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(
            err.to_string().contains("V(out)"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn direct_file_exports_preserve_existing_destination_on_serialization_failure() {
        let directory = TestDirectory::new("preserve");

        let destination = directory.destination("exporter.raw");
        seed_existing(&destination);
        let malformed_exporter = RawExporter {
            title: "Malformed".to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables: vec![RawVariable::time(), RawVariable::voltage("out")],
            data: vec![vec![0.0]],
        };
        let error = malformed_exporter
            .write_to_file(&destination, RawFormat::Ascii)
            .expect_err("malformed exporter must not replace the destination");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_existing_preserved(&destination);

        let destination = directory.destination("transient.raw");
        seed_existing(&destination);
        let error = export_transient(
            &destination,
            &[0.0, 1.0],
            &["out".to_string()],
            &[vec![0.0]],
            RawFormat::Binary,
        )
        .expect_err("malformed transient data must not replace the destination");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_existing_preserved(&destination);

        let destination = directory.destination("dc.raw");
        seed_existing(&destination);
        let error = export_dc_sweep(
            &destination,
            &[0.0, 1.0],
            "vin",
            &["out".to_string()],
            &[vec![0.0]],
            RawFormat::Ascii,
        )
        .expect_err("malformed DC data must not replace the destination");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_existing_preserved(&destination);
    }

    #[test]
    fn direct_file_exports_atomically_replace_existing_destination_on_success() {
        let directory = TestDirectory::new("success");

        let destination = directory.destination("exporter.raw");
        seed_existing(&destination);
        let mut exporter = RawExporter::new_transient("Exporter");
        exporter.add_voltage("out");
        exporter
            .add_transient_data(&[0.0, 1.0], &[vec![0.0, 2.0]])
            .expect("add shaped exporter data");
        exporter
            .write_to_file(&destination, RawFormat::Binary)
            .expect("publish exporter RAW");
        assert_committed_raw(&destination, "Transient Analysis", 2, 2);

        let destination = directory.destination("transient.raw");
        seed_existing(&destination);
        export_transient(
            &destination,
            &[0.0, 1.0],
            &["out".to_string()],
            &[vec![0.0, 2.0]],
            RawFormat::Ascii,
        )
        .expect("publish transient RAW");
        assert_committed_raw(&destination, "Transient Analysis", 2, 2);

        let destination = directory.destination("dc.raw");
        seed_existing(&destination);
        export_dc_sweep(
            &destination,
            &[0.0, 1.0],
            "vin",
            &["out".to_string()],
            &[vec![1.0, 2.0]],
            RawFormat::Binary,
        )
        .expect("publish DC RAW");
        assert_committed_raw(&destination, "DC transfer characteristic", 2, 2);
    }
}
