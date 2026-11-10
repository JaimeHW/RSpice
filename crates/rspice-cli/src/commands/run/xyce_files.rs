//! The files a run in the Xyce dialect writes beside its deck.
//!
//! Xyce writes each `.PRINT` table to a file named after the deck:
//! `deck.cir.prn` for a transient or DC table, `deck.cir.FD.prn` for AC,
//! `deck.cir.NOISE.prn` for noise, and `deck.cir.HB.FD.prn` with
//! `deck.cir.HB.TD.prn` for a harmonic-balance spectrum and its waveform; or
//! the file `.PRINT FILE=` names, in the layout `FORMAT=` selects. It writes
//! the measures to `deck.cir.mt0` (`.ma0` in an AC or noise run, `.ms0` in a
//! DC sweep), one file per `.STEP` coordinate, and the value each coordinate
//! stepped to `deck.cir.res`. A run in that dialect without `-o` writes the
//! same files in the same layouts, so a flow built on them runs unchanged.
//!
//! They are outputs beside the run's result, not the result: a file the
//! deck's directory refuses, or that cannot be written, is a warning, and the
//! run's exit status stands. Every destination resolves inside the deck's
//! directory, as a control script's files do.
//!
//! One CLI process runs one command, so every context of the run, one per
//! `.STEP` coordinate, records into one log, and the files are written once
//! the run has finished, each coordinate's rows after the one before.

use super::RunContext;
use crate::cli::{CliError, Config, RunArgs};
use crate::commands::export_table::{ColumnData, ExportTable};
use rspice_core::config::SpiceDialect;
use rspice_core::execution::RunAxisValue;
use rspice_core::io::xyce_prn::{
    XycePrnDelimiter, XycePrnFooter, XycePrnLimits, XycePrnOutputOptions, XycePrnRequest,
    XycePrnScientificStyle, XycePrnTable, format_xyce_prn_scientific, serialize_xyce_prn_sequence,
};
use rspice_core::netlist::{
    AnalysisCommand, OutputAnalysisKind, OutputDirectiveKind, OutputRequest, PrintDelimiter,
    SimulationOptions,
};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

/// Bytes one table value may take: Xyce's widest default field is 18.
const BYTES_PER_VALUE: usize = 128;

/// The coordinate column Xyce writes before a table's own columns.
#[derive(Debug, Clone, Copy)]
pub(super) enum Scale {
    /// `TIME`, before a transient table or a harmonic-balance waveform.
    Time,
    /// `FREQ`, before an AC, noise or harmonic-balance spectrum.
    Frequency,
    /// None: a DC table holds only what its card names.
    None,
}

/// How one `.PRINT` card lays its table out.
#[derive(Debug, Clone, PartialEq)]
struct Layout {
    /// `None` is Xyce's padded table; otherwise the unpadded separator.
    delimiter: Option<String>,
    precision: Option<i32>,
    width: Option<i32>,
    index: bool,
    header: Option<bool>,
    /// Whether the table ends with Xyce's closing line: a CSV table does not.
    footer: bool,
    csv: bool,
}

impl XycePrnRequest for Layout {
    fn xyce_prn_is_print_request(&self) -> bool {
        true
    }

    fn xyce_prn_delimiter(&self) -> Option<XycePrnDelimiter<'_>> {
        Some(
            self.delimiter
                .as_deref()
                .map_or(XycePrnDelimiter::Whitespace, XycePrnDelimiter::Separated),
        )
    }

    fn xyce_prn_precision(&self) -> Option<i32> {
        self.precision
    }

    fn xyce_prn_width(&self) -> Option<i32> {
        self.width
    }
}

impl XycePrnOutputOptions for Layout {
    fn xyce_prn_print_header(&self) -> Option<bool> {
        self.header
    }

    fn xyce_prn_print_footer(&self) -> Option<bool> {
        Some(self.footer)
    }
}

/// One coordinate's rows of one file.
struct Table {
    file: String,
    layout: Layout,
    step: usize,
    swept: bool,
    table: XycePrnTable,
}

/// One coordinate's measure lines.
struct Measures {
    step: usize,
    suffix: &'static str,
    lines: Vec<String>,
}

/// The values one `.STEP` coordinate assigned, by name.
struct Coordinate {
    step: usize,
    values: Vec<(String, f64)>,
}

#[derive(Default)]
struct Log {
    tables: Vec<Table>,
    measures: Vec<Measures>,
    coordinates: Vec<Coordinate>,
    /// Said once, however many coordinates met the same card.
    warnings: Vec<String>,
}

impl Log {
    fn warn(&mut self, message: String) {
        if !self.warnings.contains(&message) {
            self.warnings.push(message);
        }
    }
}

fn log() -> MutexGuard<'static, Log> {
    static LOG: OnceLock<Mutex<Log>> = OnceLock::new();
    LOG.get_or_init(Default::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// Whether this run writes Xyce's files: the Xyce dialect, a deck read from
/// a file, and no `-o`, which names the run's one result instead.
pub(super) fn writes_files(ctx: &RunContext<'_>) -> bool {
    ctx.engine.config().spice_dialect == SpiceDialect::Xyce
        && ctx.args.output.is_none()
        && ctx.args.corners.is_none()
        && !crate::commands::is_stdin(&ctx.args.input)
}

/// The `.STEP` coordinate a context runs, counted from 0, as Xyce numbers its
/// measure files; a run without `.STEP` is coordinate 0.
fn step(ctx: &RunContext<'_>) -> usize {
    ctx.run_coordinate()
        .map_or(0, rspice_core::execution::RunCoordinate::ordinal)
}

fn deck_name(args: &RunArgs) -> String {
    args.input
        .file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
}

/// The layout a card's `FORMAT=` and `DELIMITER=` select, or the format's
/// name when it is one this writer does not produce.
fn layout(request: &OutputRequest, options: &SimulationOptions) -> Result<Layout, String> {
    let separator = match &request.print_delimiter {
        None | Some(PrintDelimiter::Whitespace) => None,
        Some(delimiter) => Some(delimiter.separator().to_string()),
    };
    let header = options.output_print_header;
    let footer = options.output_print_footer.unwrap_or(true);
    let format = request.print_format.as_deref().unwrap_or("STD");
    let (delimiter, index, footer, csv) = match format {
        // GNUPLOT and SPLOT are the standard table; Xyce adds only blank
        // lines between `.STEP` coordinates.
        "STD" | "GNUPLOT" | "SPLOT" => (separator, true, footer, false),
        "NOINDEX" => (separator, false, footer, false),
        "CSV" => (
            Some(separator.unwrap_or_else(|| ",".into())),
            false,
            false,
            true,
        ),
        other => return Err(other.to_string()),
    };
    Ok(Layout {
        delimiter,
        precision: request.print_precision,
        width: request.print_width,
        index,
        header,
        footer,
        csv,
    })
}

/// The extension Xyce gives an analysis's table in the deck's name; a
/// harmonic-balance card writes its spectrum and its waveform.
fn extension(kind: OutputAnalysisKind, scale: Scale, csv: bool) -> Option<&'static str> {
    Some(match (kind, csv) {
        (OutputAnalysisKind::Hb, false) if matches!(scale, Scale::Time) => ".HB.TD.prn",
        (OutputAnalysisKind::Hb, true) if matches!(scale, Scale::Time) => ".HB.TD.csv",
        (OutputAnalysisKind::Hb, false) => ".HB.FD.prn",
        (OutputAnalysisKind::Hb, true) => ".HB.FD.csv",
        (OutputAnalysisKind::Tran | OutputAnalysisKind::Dc, false) => ".prn",
        (OutputAnalysisKind::Tran | OutputAnalysisKind::Dc, true) => ".csv",
        (OutputAnalysisKind::Ac, false) => ".FD.prn",
        (OutputAnalysisKind::Ac, true) => ".FD.csv",
        (OutputAnalysisKind::Noise, false) => ".NOISE.prn",
        (OutputAnalysisKind::Noise, true) => ".NOISE.csv",
        _ => return None,
    })
}

/// The file a card writes: the deck's name with the analysis's extension, or
/// the name `FILE=` gives, which takes the extension too when it names a
/// `.cir` file (N_IO_OutputterLocal.C:131-165).
fn file_name(deck: &str, authored: Option<&str>, extension: &str) -> String {
    match authored {
        None => format!("{deck}{extension}"),
        Some(name) if name == deck || name.ends_with(".cir") => format!("{name}{extension}"),
        Some(name) => name.to_string(),
    }
}

/// Record one coordinate's `.PRINT` table of `kind`, built only when the deck
/// prints that analysis.
///
/// Each operand of each card is one column of `table`, in card order, so a
/// card that names its own file takes its own columns there; cards that share
/// a file share its table, as Xyce merges them.
pub(super) fn record_print(
    ctx: &RunContext<'_>,
    kind: OutputAnalysisKind,
    scale: Scale,
    table: impl FnOnce() -> Result<ExportTable, CliError>,
) -> Result<(), CliError> {
    if !writes_files(ctx) {
        return Ok(());
    }
    let requests: Vec<&OutputRequest> = ctx
        .netlist
        .output_requests
        .iter()
        .filter(|request| {
            request.directive == OutputDirectiveKind::Print && request.analysis == Some(kind)
        })
        .collect();
    if requests.is_empty() {
        return Ok(());
    }
    let table = table()?;
    let deck = deck_name(ctx.args);
    let operands: usize = requests.iter().map(|request| request.operands.len()).sum();
    let by_card = operands == table.columns.len();
    let mut files: Vec<(String, Layout, Vec<usize>)> = Vec::new();
    let mut first = 0;
    let mut log = log();
    for request in requests {
        let columns: Vec<usize> = if by_card {
            (first..first + request.operands.len()).collect()
        } else {
            (0..table.columns.len()).collect()
        };
        first += request.operands.len();
        let layout = match layout(request, &ctx.netlist.options) {
            Ok(layout) => layout,
            Err(format) => {
                log.warn(format!(
                    "warning: line {}: .PRINT FORMAT={format} is not written; \
                     STD, NOINDEX and CSV are",
                    request.origin.line
                ));
                continue;
            }
        };
        let Some(extension) = extension(kind, scale, layout.csv) else {
            return Ok(());
        };
        let file = file_name(&deck, request.print_file.as_deref(), extension);
        // A `.PRINT HB FILE=` names one file, which Xyce gives the spectrum.
        if matches!(scale, Scale::Time)
            && kind == OutputAnalysisKind::Hb
            && request.print_file.as_deref() == Some(file.as_str())
        {
            continue;
        }
        match files.iter_mut().find(|(known, ..)| *known == file) {
            Some((_, _, known)) if by_card => known.extend(columns),
            Some(_) => {}
            None => files.push((file, layout, columns)),
        }
    }
    let swept = ctx.run_coordinate().is_some();
    for (file, layout, columns) in files {
        match prn_table(&table, scale, &columns, layout.index) {
            Ok(table) => log.tables.push(Table {
                file,
                layout,
                step: step(ctx),
                swept,
                table,
            }),
            Err(reason) => log.warn(format!("warning: {file} is not written: {reason}")),
        }
    }
    Ok(())
}

/// A column's name as Xyce prints it: upper case, except the `Re(..)` or
/// `Im(..)` around one part of a complex value.
fn xyce_name(name: &str) -> String {
    for part in ["Re(", "Im("] {
        if let Some(inner) = name
            .strip_prefix(part)
            .and_then(|inner| inner.strip_suffix(')'))
        {
            return format!("{part}{})", inner.to_ascii_uppercase());
        }
    }
    name.to_ascii_uppercase()
}

/// One coordinate's rows: Xyce's `Index`, the scale, then each column under
/// its name in upper case, a complex one as its `Re(..)` and `Im(..)`.
fn prn_table(
    table: &ExportTable,
    scale: Scale,
    columns: &[usize],
    index: bool,
) -> Result<XycePrnTable, String> {
    let rows = table.scale.len();
    let mut names = Vec::new();
    let mut values: Vec<Vec<f64>> = Vec::new();
    if index {
        names.push("Index".to_string());
        values.push((0..rows).map(|row| row as f64).collect());
    }
    match scale {
        Scale::Time => names.push("TIME".to_string()),
        Scale::Frequency => names.push("FREQ".to_string()),
        Scale::None => {}
    }
    if !matches!(scale, Scale::None) {
        values.push(table.scale.clone());
    }
    let complete = |name: &str, values: &[Option<f64>]| {
        values
            .iter()
            .copied()
            .collect::<Option<Vec<f64>>>()
            .ok_or_else(|| format!("{name} has samples this run did not produce"))
    };
    for &column in columns {
        let column = &table.columns[column];
        let name = xyce_name(&column.name);
        match &column.data {
            ColumnData::Real(real) => {
                names.push(name);
                values.push(real.clone());
            }
            ColumnData::NullableReal(real) => {
                values.push(complete(&name, real)?);
                names.push(name);
            }
            ColumnData::Complex { real, imag } => {
                names.extend([format!("Re({name})"), format!("Im({name})")]);
                values.extend([real.clone(), imag.clone()]);
            }
            ColumnData::NullableComplex(samples) => {
                let samples = samples
                    .iter()
                    .copied()
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(|| format!("{name} has samples this run did not produce"))?;
                names.extend([format!("Re({name})"), format!("Im({name})")]);
                values.push(samples.iter().map(|value| value.re).collect());
                values.push(samples.iter().map(|value| value.im).collect());
            }
        }
    }
    Ok(XycePrnTable {
        columns: names,
        rows: (0..rows)
            .map(|row| values.iter().map(|column| column[row]).collect())
            .collect(),
    })
}

/// Record one coordinate's measures as Xyce's measure file lists them:
/// `NAME = value`, or `NAME = FAILED` (N_IO_MeasureBase.h:213-227).
pub(super) fn record_measures(ctx: &RunContext<'_>, results: &[rspice_core::MeasureResult]) {
    if !writes_files(ctx) || results.is_empty() {
        return;
    }
    let options = &ctx.netlist.options;
    let lines = results
        .iter()
        .map(|result| {
            let value = match result.value {
                Some(value) => Some(value),
                None if options.measure_fail_output.unwrap_or(true) => None,
                None => Some(options.measure_default_value.unwrap_or(0.0)),
            };
            let text = value.map_or_else(|| "FAILED".to_string(), measure_value);
            format!("{} = {text}", result.name.to_ascii_uppercase())
        })
        .collect();
    log().measures.push(Measures {
        step: step(ctx),
        suffix: measure_suffix(&ctx.netlist.analyses),
        lines,
    });
}

/// `%e` with Xyce's default six digits, as C++ spells it.
fn measure_value(value: f64) -> String {
    format_xyce_prn_scientific(value, 6, 64, XycePrnScientificStyle::Canonical).unwrap_or_else(
        |_| match value {
            value if value.is_nan() => "nan".to_string(),
            value if value > 0.0 => "inf".to_string(),
            _ => "-inf".to_string(),
        },
    )
}

/// The measure file's extension follows the run's analysis: `.mt` for a
/// transient, `.ma` for AC or noise, `.ms` for a DC sweep
/// (N_IO_MeasureManager.C:544-559).
fn measure_suffix(analyses: &[AnalysisCommand]) -> &'static str {
    analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Tran { .. } => Some(".mt"),
            AnalysisCommand::Ac { .. } | AnalysisCommand::Noise { .. } => Some(".ma"),
            AnalysisCommand::Dc { .. } => Some(".ms"),
            _ => None,
        })
        .unwrap_or(".mt")
}

/// Record the values a `.STEP` coordinate assigned, for `deck.cir.res`.
pub(super) fn record_coordinate(ctx: &RunContext<'_>) {
    if !writes_files(ctx) {
        return;
    }
    let Some(coordinate) = ctx.run_coordinate() else {
        return;
    };
    let values = coordinate
        .assignments()
        .iter()
        .flat_map(|assignment| match assignment.value() {
            RunAxisValue::Numeric(value) => vec![(
                super::axis::implicit_axis_scale_name(assignment).to_ascii_uppercase(),
                *value,
            )],
            RunAxisValue::DataRow(bindings) => bindings
                .iter()
                .map(|binding| (binding.name().to_ascii_uppercase(), binding.value()))
                .collect(),
            RunAxisValue::AlterVariant { .. } => Vec::new(),
        })
        .collect();
    log().coordinates.push(Coordinate {
        step: coordinate.ordinal(),
        values,
    });
}

/// Write every file the run recorded, one line naming each; returns the
/// files written.
pub(super) fn publish(
    args: &RunArgs,
    config: &Config,
    quiet: bool,
) -> Result<Vec<PathBuf>, CliError> {
    let log = std::mem::take(&mut *log());
    for warning in &log.warnings {
        crate::observability::diagnostic("xyce-output-not-written", None, warning);
    }
    let deck = deck_name(args);
    let mut files: Vec<(String, Result<String, String>)> = Vec::new();

    let mut tables = log.tables;
    tables.sort_by_key(|table| table.step);
    let mut names: Vec<&str> = Vec::new();
    for table in &tables {
        if !names.contains(&table.file.as_str()) {
            names.push(&table.file);
        }
    }
    let limit = config.resources.limits().max_result_values;
    for name in names {
        let sequence: Vec<&Table> = tables.iter().filter(|table| table.file == name).collect();
        let layout = &sequence[0].layout;
        let footer = match (layout.footer, sequence[0].swept) {
            (false, _) => XycePrnFooter::None,
            (true, false) => XycePrnFooter::Simulation,
            (true, true) => XycePrnFooter::ParameterSweep,
        };
        let rows: Vec<XycePrnTable> = sequence.iter().map(|table| table.table.clone()).collect();
        let text = serialize_xyce_prn_sequence(
            &rows,
            layout,
            layout,
            footer,
            XycePrnLimits::new(limit, limit.saturating_mul(BYTES_PER_VALUE)),
        )
        .map_err(|error| error.to_string());
        files.push((name.to_string(), text));
    }

    let mut measures: Vec<(String, Vec<String>)> = Vec::new();
    for set in log.measures {
        let name = format!("{deck}{}{}", set.suffix, set.step);
        match measures.iter_mut().find(|(known, _)| *known == name) {
            Some((_, lines)) => lines.extend(set.lines),
            None => measures.push((name, set.lines)),
        }
    }
    for (name, lines) in measures {
        files.push((
            name,
            Ok(lines.iter().map(|line| format!("{line}\n")).collect()),
        ));
    }

    if !log.coordinates.is_empty() {
        let mut coordinates = log.coordinates;
        coordinates.sort_by_key(|coordinate| coordinate.step);
        files.push((format!("{deck}.res"), Ok(step_table(&coordinates))));
    }

    let mut written = Vec::with_capacity(files.len());
    for (name, text) in files {
        match text {
            Ok(text) => written.extend(write(args, &name, text.as_bytes(), quiet)?),
            Err(reason) => crate::observability::diagnostic(
                "xyce-output-not-written",
                None,
                format_args!("warning: {name} is not written: {reason}"),
            ),
        }
    }
    Ok(written)
}

/// `deck.cir.res`: `STEP`, then each stepped name right-aligned in 20
/// columns; each row the coordinate's number in 10 and its values in 20, left
/// aligned (N_IO_OutputResults.C:213-265).
fn step_table(coordinates: &[Coordinate]) -> String {
    let mut text = "STEP".to_string();
    for (name, _) in &coordinates[0].values {
        text.push_str(&format!(" {name:>20}"));
    }
    text.push('\n');
    for coordinate in coordinates {
        text.push_str(&format!("{:<10}", coordinate.step));
        for (_, value) in &coordinate.values {
            let value =
                format_xyce_prn_scientific(*value, 8, 64, XycePrnScientificStyle::Canonical)
                    .unwrap_or_else(|_| value.to_string());
            text.push_str(&format!(" {value:<20}"));
        }
        text.push('\n');
    }
    text.push_str("End of Xyce(TM) Parameter Sweep\n");
    text
}

/// Write one file inside the deck's directory, or say why it was not.
fn write(
    args: &RunArgs,
    name: &str,
    bytes: &[u8],
    quiet: bool,
) -> Result<Option<PathBuf>, CliError> {
    let not_written = |reason: &dyn std::fmt::Display| {
        crate::observability::diagnostic(
            "xyce-output-not-written",
            None,
            format_args!("warning: {name} is not written: {reason}"),
        );
    };
    let destination = match super::control::deck_directory_destination(name, args)? {
        Ok(destination) => destination,
        Err((destination, directory)) => {
            not_written(&format_args!(
                "it resolves to {}, outside the deck's directory {}",
                destination.display(),
                directory.display()
            ));
            return Ok(None);
        }
    };
    let published = crate::commands::publish::destinations::refuse_declared(&destination)
        .and_then(|()| super::control::publish_bytes(&destination, bytes));
    if let Err(error) = published {
        not_written(&error);
        return Ok(None);
    }
    if !quiet {
        let shown = match args.input.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => parent.join(name),
            _ => Path::new(name).to_path_buf(),
        };
        crate::console::line(format_args!("wrote {}", shown.display()))?;
    }
    Ok(Some(destination))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_card_file_takes_the_extension_only_when_it_names_a_deck() {
        assert_eq!(file_name("rc.cir", None, ".FD.prn"), "rc.cir.FD.prn");
        assert_eq!(file_name("rc.cir", Some("one.txt"), ".prn"), "one.txt");
        assert_eq!(file_name("rc.cir", Some("two"), ".prn"), "two");
        assert_eq!(
            file_name("rc.cir", Some("three.cir"), ".prn"),
            "three.cir.prn"
        );
    }

    #[test]
    fn a_complex_part_keeps_its_re_or_im() {
        assert_eq!(xyce_name("v(out)"), "V(OUT)");
        assert_eq!(xyce_name("Re(id(M1))"), "Re(ID(M1))");
        assert_eq!(xyce_name("Im({v(1)*2})"), "Im({V(1)*2})");
    }

    #[test]
    fn the_step_table_lays_out_as_xyce_writes_it() {
        let table = step_table(&[
            Coordinate {
                step: 0,
                values: vec![("R".into(), 1e3)],
            },
            Coordinate {
                step: 1,
                values: vec![("R".into(), 2e3)],
            },
        ]);
        assert_eq!(
            table,
            "STEP                    R\n\
             0          1.00000000e+03      \n\
             1          2.00000000e+03      \n\
             End of Xyce(TM) Parameter Sweep\n"
        );
    }
}
