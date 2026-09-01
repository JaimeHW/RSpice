//! Result serialization: Touchstone, SPICE raw, and CSV.
//!
//! Touchstone and CSV output is a pure function of the result, so those
//! artifacts can be diffed in a regression suite directly. A raw file carries
//! an ngspice-style `Date:` header, so callers who need byte-reproducible raw
//! output pin it through the `timestamp` argument.
//! Each format is offered both as an in-memory string (or byte vector) and as
//! a file write, because CI jobs frequently need the text without touching a
//! filesystem.
//!
//! # Touchstone
//!
//! Re-exported from `rspice_core`, which owns the single writer every
//! front-end shares. It stays visible through this module so the result types
//! reach all three formats through one path.
//!
//! # SPICE raw
//!
//! ngspice-compatible ASCII and binary raw files, including `Flags: complex`
//! for frequency-domain data, which the shared core exporter does not model.
//!
//! Nothing here touches the Python interpreter: every entry point returns a
//! plain `Result<_, String>` that the binding layer maps to an exception. That
//! keeps the format logic unit-testable without an embedded CPython.

use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::Complex64;

// The Touchstone writer lives in core so the CLI and the desktop runner emit
// byte-identical files; the result types keep reaching it through this module.
pub(crate) use rspice_core::analysis::s_param::{
    TouchstoneFormat, TouchstoneFrequencyUnit, TouchstoneInput, touchstone, touchstone_extension,
};

/// Render a float the way every serializer here needs it: shortest round-trip
/// form, with a `.0` kept on integral values so column types stay obvious.
fn format_float(value: f64) -> String {
    if value == value.trunc() && value.is_finite() {
        format!("{value:.1}")
    } else {
        format!("{value}")
    }
}

/// Uniform message for a rejected string-valued option.
fn unknown_option(kind: &str, value: &str, accepted: &[&str]) -> String {
    format!(
        "unknown {kind} '{value}'; expected one of {}",
        accepted.join(", ")
    )
}

//=============================================================================
// Shared helpers
//=============================================================================

/// asctime-style UTC timestamp, matching the header ngspice writes.
///
/// Civil-date conversion uses Hinnant's days-from-epoch algorithm;
/// 1970-01-01 was a Thursday.
fn asctime_utc_now() -> String {
    const WEEKDAYS: [&str; 7] = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    let days = (seconds / 86_400) as i64;
    let time_of_day = seconds % 86_400;
    let (hour, minute, second) = (
        time_of_day / 3600,
        (time_of_day % 3600) / 60,
        time_of_day % 60,
    );

    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_position = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_position + 2) / 5 + 1;
    let month = if month_position < 10 {
        month_position + 3
    } else {
        month_position - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);

    format!(
        "{} {} {day} {hour:02}:{minute:02}:{second:02} {year}",
        WEEKDAYS[days.rem_euclid(7) as usize],
        MONTHS[(month - 1) as usize],
    )
}

//=============================================================================
// SPICE raw
//=============================================================================

/// Physical quantity of one raw-file column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVariableKind {
    Time,
    Frequency,
    Voltage,
    Current,
}

impl RawVariableKind {
    fn keyword(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Frequency => "frequency",
            Self::Voltage => "voltage",
            Self::Current => "current",
        }
    }
}

/// One raw-file column.
pub(crate) struct RawVariable {
    pub name: String,
    pub kind: RawVariableKind,
}

/// A complete raw plot: header metadata plus column-major data.
pub(crate) struct RawPlot {
    pub title: String,
    pub plot_name: String,
    pub variables: Vec<RawVariable>,
    /// One series per variable, all the same length.
    pub series: Vec<Vec<Complex64>>,
    /// False writes `Flags: real` and one number per value.
    pub complex: bool,
    /// `Date:` header text. `None` stamps the current UTC time; a fixed value
    /// makes the whole artifact a pure function of the result.
    pub timestamp: Option<String>,
}

impl RawPlot {
    fn validate(&self) -> Result<usize, String> {
        if self.variables.is_empty() {
            return Err("raw export requires at least one variable".to_string());
        }
        if self.series.len() != self.variables.len() {
            return Err(format!(
                "raw export has {} series for {} variables",
                self.series.len(),
                self.variables.len()
            ));
        }
        let points = self.series[0].len();
        for (variable, series) in self.variables.iter().zip(&self.series) {
            if series.len() != points {
                return Err(format!(
                    "raw export variable '{}' has {} points but the sweep axis has {points}",
                    variable.name,
                    series.len()
                ));
            }
        }
        Ok(points)
    }

    fn header(&self, points: usize) -> String {
        let mut header = String::new();
        let _ = writeln!(header, "Title: {}", self.title);
        let _ = writeln!(
            header,
            "Date: {}",
            self.timestamp.clone().unwrap_or_else(asctime_utc_now)
        );
        let _ = writeln!(header, "Plotname: {}", self.plot_name);
        let _ = writeln!(
            header,
            "Flags: {}",
            if self.complex { "complex" } else { "real" }
        );
        let _ = writeln!(header, "No. Variables: {}", self.variables.len());
        let _ = writeln!(header, "No. Points: {points}");
        let _ = writeln!(header, "Variables:");
        for (index, variable) in self.variables.iter().enumerate() {
            let _ = writeln!(
                header,
                "\t{index}\t{}\t{}",
                variable.name,
                variable.kind.keyword()
            );
        }
        header
    }

    /// ASCII raw text. Complex values are written `re,im`, as ngspice does.
    fn to_ascii(&self) -> Result<String, String> {
        let points = self.validate()?;
        let mut output = self.header(points);
        let _ = writeln!(output, "Values:");
        for point in 0..points {
            for (column, series) in self.series.iter().enumerate() {
                let value = series[point];
                let rendered = if self.complex {
                    format!("{},{}", format_float(value.re), format_float(value.im))
                } else {
                    format_float(value.re)
                };
                if column == 0 {
                    let _ = write!(output, "{point}\t{rendered}");
                } else {
                    let _ = write!(output, "\t{rendered}");
                }
            }
            output.push('\n');
        }
        Ok(output)
    }

    /// Binary raw bytes: the same header, then little-endian f64 values.
    fn to_binary(&self) -> Result<Vec<u8>, String> {
        let points = self.validate()?;
        let mut output = self.header(points).into_bytes();
        output.extend_from_slice(b"Binary:\n");
        for point in 0..points {
            for series in &self.series {
                let value = series[point];
                output.extend_from_slice(&value.re.to_le_bytes());
                if self.complex {
                    output.extend_from_slice(&value.im.to_le_bytes());
                }
            }
        }
        Ok(output)
    }
}

/// Raw-file encoding selected by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawFormat {
    Ascii,
    Binary,
}

impl RawFormat {
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        match value.to_ascii_lowercase().as_str() {
            "ascii" | "text" => Ok(Self::Ascii),
            "binary" | "bin" => Ok(Self::Binary),
            other => Err(unknown_option("raw format", other, &["ascii", "binary"])),
        }
    }
}

/// Serialize a plot in the requested encoding.
pub(crate) fn raw_bytes(plot: &RawPlot, format: RawFormat) -> Result<Vec<u8>, String> {
    match format {
        RawFormat::Ascii => plot.to_ascii().map(String::into_bytes),
        RawFormat::Binary => plot.to_binary(),
    }
}

//=============================================================================
// CSV
//=============================================================================

/// Quote a CSV field only when the content requires it (RFC 4180).
fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// Render a table as RFC 4180 CSV with CRLF-free `\n` line endings.
///
/// Rows shorter or longer than the header are a programming error in the
/// caller, so the mismatch is reported rather than padded.
pub(crate) fn csv(headers: &[String], rows: &[Vec<f64>]) -> Result<String, String> {
    let mut output = String::new();
    let _ = writeln!(
        output,
        "{}",
        headers
            .iter()
            .map(|header| csv_field(header))
            .collect::<Vec<_>>()
            .join(",")
    );
    for (index, row) in rows.iter().enumerate() {
        if row.len() != headers.len() {
            return Err(format!(
                "CSV row {index} has {} values for {} columns",
                row.len(),
                headers.len()
            ));
        }
        let _ = writeln!(
            output,
            "{}",
            row.iter()
                .map(|value| format_float(*value))
                .collect::<Vec<_>>()
                .join(",")
        );
    }
    Ok(output)
}

//=============================================================================
// File writing
//=============================================================================

const TEMP_MARKER: &str = ".rspice-python-tmp-";
const MAX_TEMP_ATTEMPTS: u64 = 128;
static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

/// Publish bytes transactionally beside the destination.
///
/// Serialization is complete before this function is called. The complete
/// byte sequence is written to a uniquely created sibling, flushed, and
/// synchronized before an atomic same-volume rename makes it visible. A
/// failure therefore leaves an existing complete artifact unchanged or an
/// absent destination absent; callers never observe a truncated result file.
pub(crate) fn write_bytes(path: &Path, bytes: &[u8]) -> io::Result<()> {
    write_atomic_impl(
        path,
        |file| file.write_all(bytes),
        #[cfg(test)]
        None,
    )
}

fn write_atomic_impl<F>(
    destination: &Path,
    write: F,
    #[cfg(test)] fault: Option<TestFault>,
) -> io::Result<()>
where
    F: FnOnce(&mut File) -> io::Result<()>,
{
    reject_symlink_destination(destination)
        .map_err(|error| atomic_phase_error("preparation", error))?;
    let (temporary_path, mut temporary_file) = create_staging_file(destination)
        .map_err(|error| atomic_phase_error("preparation", error))?;
    let mut cleanup = StagingCleanup::new(temporary_path.clone());

    if let Err(error) = write(&mut temporary_file) {
        drop(temporary_file);
        cleanup.remove_now();
        return Err(atomic_phase_error("write", error));
    }

    #[cfg(test)]
    if let Err(error) = inject_fault(fault, TestFault::Flush) {
        drop(temporary_file);
        cleanup.remove_now();
        return Err(atomic_phase_error("flush", error));
    }

    if let Err(error) = temporary_file
        .flush()
        .and_then(|()| temporary_file.sync_all())
    {
        drop(temporary_file);
        cleanup.remove_now();
        return Err(atomic_phase_error("flush", error));
    }
    drop(temporary_file);

    #[cfg(test)]
    if let Err(error) = inject_fault(fault, TestFault::BeforeCommit) {
        cleanup.remove_now();
        return Err(atomic_phase_error("commit", error));
    }

    reject_symlink_destination(destination).map_err(|error| atomic_phase_error("commit", error))?;
    if let Err(error) = commit_staging_file(&temporary_path, destination) {
        cleanup.remove_now();
        return Err(atomic_phase_error("commit", error));
    }
    cleanup.disarm();

    #[cfg(unix)]
    sync_parent_directory(destination).map_err(|error| atomic_phase_error("commit", error))?;

    Ok(())
}

fn atomic_phase_error(phase: &str, error: io::Error) -> io::Error {
    io::Error::new(
        error.kind(),
        format!("atomic result output {phase} failed: {error}"),
    )
}

fn create_staging_file(destination: &Path) -> io::Result<(PathBuf, File)> {
    let file_name = destination.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "result destination must name a file",
        )
    })?;
    let parent = destination_parent(destination);
    let process_id = std::process::id();

    for _ in 0..MAX_TEMP_ATTEMPTS {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let mut staging_name = OsString::from(".");
        staging_name.push(file_name);
        staging_name.push(format!("{TEMP_MARKER}{process_id}-{id}"));
        let staging_path = parent.join(staging_name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&staging_path)
        {
            Ok(file) => return Ok((staging_path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        format!(
            "could not allocate a unique staging file beside {}",
            destination.display()
        ),
    ))
}

fn destination_parent(destination: &Path) -> &Path {
    match destination.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    }
}

fn reject_symlink_destination(destination: &Path) -> io::Result<()> {
    match std::fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "refusing to replace symlink result destination {}",
                destination.display()
            ),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(not(windows))]
fn commit_staging_file(staging: &Path, destination: &Path) -> io::Result<()> {
    std::fs::rename(staging, destination)
}

#[cfg(windows)]
fn commit_staging_file(staging: &Path, destination: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    fn wide_path(path: &Path) -> io::Result<Vec<u16>> {
        let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
        if wide.contains(&0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "result path contains an embedded NUL",
            ));
        }
        wide.push(0);
        Ok(wide)
    }

    let staging_wide = wide_path(staging)?;
    let destination_wide = wide_path(destination)?;

    // SAFETY: Both buffers are NUL-terminated and remain alive through this
    // call. The staging file is a sibling of the destination, so this is a
    // same-volume replace and never degrades into a copy operation.
    let succeeded = unsafe {
        MoveFileExW(
            staging_wide.as_ptr(),
            destination_wide.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if succeeded == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
fn sync_parent_directory(destination: &Path) -> io::Result<()> {
    File::open(destination_parent(destination))?.sync_all()
}

struct StagingCleanup {
    path: Option<PathBuf>,
}

impl StagingCleanup {
    fn new(path: PathBuf) -> Self {
        Self { path: Some(path) }
    }

    fn remove_now(&mut self) {
        if let Some(path) = self.path.as_ref() {
            match std::fs::remove_file(path) {
                Ok(()) => self.path = None,
                Err(error) if error.kind() == io::ErrorKind::NotFound => self.path = None,
                Err(_) => {}
            }
        }
    }

    fn disarm(&mut self) {
        self.path = None;
    }
}

impl Drop for StagingCleanup {
    fn drop(&mut self) {
        self.remove_now();
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TestFault {
    Flush,
    BeforeCommit,
}

#[cfg(test)]
fn inject_fault(selected: Option<TestFault>, current: TestFault) -> io::Result<()> {
    if selected == Some(current) {
        Err(io::Error::other(format!(
            "injected {} failure",
            match current {
                TestFault::Flush => "flush",
                TestFault::BeforeCommit => "pre-commit",
            }
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let id = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "rspice-python-atomic-export-{}-{id}-{tag}",
                std::process::id()
            ));
            std::fs::create_dir(&path).expect("create unique Python export test directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn assert_no_staging_files(directory: &Path) {
        let staging_files: Vec<PathBuf> = std::fs::read_dir(directory)
            .expect("read Python export test directory")
            .map(|entry| entry.expect("read directory entry").path())
            .filter(|path| path.to_string_lossy().contains(TEMP_MARKER))
            .collect();
        assert!(
            staging_files.is_empty(),
            "staging files were not cleaned: {staging_files:?}"
        );
    }

    fn assert_old_or_absent(destination: &Path, preexisting: bool) {
        if preexisting {
            assert_eq!(
                std::fs::read(destination).expect("read preserved result"),
                b"old complete result"
            );
        } else {
            assert!(
                !destination.exists(),
                "failed publication exposed a result artifact"
            );
        }
    }

    fn series(values: &[(f64, f64)]) -> Vec<Complex64> {
        values
            .iter()
            .map(|(re, im)| Complex64::new(*re, *im))
            .collect()
    }

    #[test]
    fn touchstone_two_port_uses_the_s11_s21_s12_s22_ordering() {
        let parameters = vec![
            vec![series(&[(0.1, 0.0)]), series(&[(0.2, 0.0)])],
            vec![series(&[(0.3, 0.0)]), series(&[(0.4, 0.0)])],
        ];
        let text = touchstone(
            &TouchstoneInput {
                frequencies: &[1e9],
                parameters: &parameters,
                reference_impedances: &[50.0, 50.0],
                comments: &["two port".to_string()],
            },
            TouchstoneFormat::RealImaginary,
            TouchstoneFrequencyUnit::GHz,
        )
        .expect("two-port export");

        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "! two port");
        assert_eq!(lines[1], "# GHZ S RI R 50");
        let fields: Vec<f64> = lines[2]
            .split('\t')
            .map(|field| field.parse().expect("numeric field"))
            .collect();
        // frequency in GHz, then S11, S21, S12, S22 as real/imaginary pairs.
        assert_eq!(fields[0], 1.0);
        assert_eq!(fields[1], 0.1);
        assert_eq!(fields[3], 0.3);
        assert_eq!(fields[5], 0.2);
        assert_eq!(fields[7], 0.4);
    }

    #[test]
    fn touchstone_rejects_mixed_reference_impedances() {
        let parameters = vec![
            vec![series(&[(0.0, 0.0)]), series(&[(0.0, 0.0)])],
            vec![series(&[(0.0, 0.0)]), series(&[(0.0, 0.0)])],
        ];
        let error = touchstone(
            &TouchstoneInput {
                frequencies: &[1e9],
                parameters: &parameters,
                reference_impedances: &[50.0, 75.0],
                comments: &[],
            },
            TouchstoneFormat::RealImaginary,
            TouchstoneFrequencyUnit::GHz,
        )
        .expect_err("mixed impedances must be rejected");
        assert!(error.to_string().contains("one reference impedance"));
    }

    #[test]
    fn touchstone_three_port_writes_one_line_per_matrix_row() {
        let entry = |value: f64| series(&[(value, 0.0)]);
        let parameters = vec![
            vec![entry(11.0), entry(12.0), entry(13.0)],
            vec![entry(21.0), entry(22.0), entry(23.0)],
            vec![entry(31.0), entry(32.0), entry(33.0)],
        ];
        let text = touchstone(
            &TouchstoneInput {
                frequencies: &[2e9],
                parameters: &parameters,
                reference_impedances: &[50.0; 3],
                comments: &[],
            },
            TouchstoneFormat::RealImaginary,
            TouchstoneFrequencyUnit::GHz,
        )
        .expect("three-port export");

        let data: Vec<&str> = text.lines().skip(1).collect();
        assert_eq!(data.len(), 3, "one line per matrix row");
        let first: Vec<f64> = data[0]
            .split('\t')
            .map(|field| field.parse().expect("numeric field"))
            .collect();
        assert_eq!(first[0], 2.0);
        assert_eq!(first[1], 11.0);
        assert_eq!(first[3], 12.0);
        assert_eq!(first[5], 13.0);
        let second: Vec<f64> = data[1]
            .split('\t')
            .map(|field| field.parse().expect("numeric field"))
            .collect();
        assert_eq!(second[0], 21.0);
    }

    #[test]
    fn raw_ascii_and_binary_agree_on_header_and_values() {
        let plot = RawPlot {
            title: "probe".to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables: vec![
                RawVariable {
                    name: "time".to_string(),
                    kind: RawVariableKind::Time,
                },
                RawVariable {
                    name: "V(out)".to_string(),
                    kind: RawVariableKind::Voltage,
                },
            ],
            series: vec![
                series(&[(0.0, 0.0), (1e-6, 0.0)]),
                series(&[(0.0, 0.0), (2.5, 0.0)]),
            ],
            complex: false,
            timestamp: None,
        };

        let ascii = plot.to_ascii().expect("ascii raw");
        assert!(ascii.contains("Flags: real"));
        assert!(ascii.contains("No. Variables: 2"));
        assert!(ascii.contains("No. Points: 2"));

        let binary = plot.to_binary().expect("binary raw");
        let marker = b"Binary:\n";
        let start = binary
            .windows(marker.len())
            .position(|window| window == marker)
            .expect("binary marker")
            + marker.len();
        // Two points x two real variables x 8 bytes.
        assert_eq!(binary.len() - start, 2 * 2 * 8);
        let last = f64::from_le_bytes(binary[binary.len() - 8..].try_into().expect("f64"));
        assert_eq!(last, 2.5);
    }

    #[test]
    fn raw_complex_writes_two_numbers_per_value() {
        let plot = RawPlot {
            title: "ac".to_string(),
            plot_name: "AC Analysis".to_string(),
            variables: vec![RawVariable {
                name: "frequency".to_string(),
                kind: RawVariableKind::Frequency,
            }],
            series: vec![series(&[(1e3, 0.0)])],
            complex: true,
            timestamp: Some("Thu Jan 1 00:00:00 1970".to_string()),
        };
        let ascii = plot.to_ascii().expect("ascii raw");
        assert!(ascii.contains("Flags: complex"));
        assert!(ascii.lines().last().expect("value line").contains(','));

        let binary = plot.to_binary().expect("binary raw");
        let marker = b"Binary:
";
        let start = binary
            .windows(marker.len())
            .position(|window| window == marker)
            .expect("binary marker")
            + marker.len();
        // One point x one complex variable x two f64.
        assert_eq!(binary.len() - start, 16);
    }

    #[test]
    fn raw_rejects_ragged_series() {
        let plot = RawPlot {
            title: "bad".to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables: vec![
                RawVariable {
                    name: "time".to_string(),
                    kind: RawVariableKind::Time,
                },
                RawVariable {
                    name: "V(out)".to_string(),
                    kind: RawVariableKind::Voltage,
                },
            ],
            series: vec![series(&[(0.0, 0.0), (1.0, 0.0)]), series(&[(0.0, 0.0)])],
            complex: false,
            timestamp: None,
        };
        assert!(plot.to_ascii().is_err());
    }

    #[test]
    fn csv_quotes_only_fields_that_need_it() {
        let text = csv(
            &["time".to_string(), "V(a,b)".to_string()],
            &[vec![0.0, 1.0]],
        )
        .expect("csv");
        let header = text.lines().next().expect("header");
        assert_eq!(header, "time,\"V(a,b)\"");
    }

    #[test]
    fn csv_rejects_rows_that_do_not_match_the_header() {
        assert!(csv(&["a".to_string(), "b".to_string()], &[vec![1.0]]).is_err());
    }

    #[test]
    fn atomic_write_failures_preserve_existing_or_absent_destination() {
        for preexisting in [false, true] {
            for (tag, partial) in [
                ("header", b"time,V(out)\n".as_slice()),
                ("values", b"time,V(out)\n0,1\n1,".as_slice()),
            ] {
                let directory = TestDirectory::new(tag);
                let destination = directory.path().join("result.csv");
                if preexisting {
                    std::fs::write(&destination, b"old complete result")
                        .expect("seed existing result");
                }

                let error = write_atomic_impl(
                    &destination,
                    |file| {
                        file.write_all(partial)?;
                        Err(io::Error::other("injected serializer failure"))
                    },
                    None,
                )
                .expect_err("injected writer failure must propagate");

                assert!(error.to_string().contains("output write failed"));
                assert_old_or_absent(&destination, preexisting);
                assert_no_staging_files(directory.path());
            }
        }
    }

    #[test]
    fn atomic_flush_and_commit_failures_preserve_destination() {
        for preexisting in [false, true] {
            for fault in [TestFault::Flush, TestFault::BeforeCommit] {
                let directory = TestDirectory::new(match fault {
                    TestFault::Flush => "flush",
                    TestFault::BeforeCommit => "commit",
                });
                let destination = directory.path().join("result.raw");
                if preexisting {
                    std::fs::write(&destination, b"old complete result")
                        .expect("seed existing result");
                }

                let error = write_atomic_impl(
                    &destination,
                    |file| file.write_all(b"new complete result"),
                    Some(fault),
                )
                .expect_err("injected publication failure must propagate");

                let expected_phase = match fault {
                    TestFault::Flush => "output flush failed",
                    TestFault::BeforeCommit => "output commit failed",
                };
                assert!(error.to_string().contains(expected_phase));
                assert_old_or_absent(&destination, preexisting);
                assert_no_staging_files(directory.path());
            }
        }
    }

    #[test]
    fn atomic_result_write_replaces_with_complete_bytes() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("success");
            let destination = directory.path().join("result.raw");
            if preexisting {
                std::fs::write(&destination, b"old complete result").expect("seed existing result");
            }

            write_bytes(&destination, b"new complete result")
                .expect("atomic result write succeeds");

            assert_eq!(
                std::fs::read(&destination).expect("read committed result"),
                b"new complete result"
            );
            assert_no_staging_files(directory.path());
        }
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn atomic_result_write_rejects_symlink_destination() {
        let directory = TestDirectory::new("symlink");
        let target = directory.path().join("target.csv");
        let destination = directory.path().join("result.csv");
        std::fs::write(&target, b"symlink target").expect("write symlink target");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &destination).expect("create result symlink");
        #[cfg(windows)]
        if let Err(error) = std::os::windows::fs::symlink_file(&target, &destination) {
            if error.kind() == io::ErrorKind::PermissionDenied || error.raw_os_error() == Some(1314)
            {
                return;
            }
            panic!("create result symlink: {error}");
        }

        let error = write_bytes(&destination, b"replacement")
            .expect_err("symlink result destination must be rejected");

        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains("refusing to replace symlink"));
        assert_eq!(
            std::fs::read(&target).expect("read symlink target"),
            b"symlink target"
        );
        assert!(destination.is_symlink());
        assert_no_staging_files(directory.path());
    }
}
