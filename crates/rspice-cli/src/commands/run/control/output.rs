//! Script presentation files: readable SVG plus lossless, typed sample data.

use super::*;
use rspice_core::engine::{
    ControlPresentation, ControlPresentationKind, ControlTrace, ControlVector,
};
use rspice_core::execution::BoundedWriteFailure;
use serde::Serialize;
use std::io::Write;
use std::path::Path;

pub(super) fn present(
    request: &ControlPresentation,
    circuit: &ControlCircuit,
    args: &RunArgs,
    config: &Config,
    run_label: Option<&str>,
    ordinal: usize,
    quiet: bool,
) -> Result<Vec<PathBuf>, CliError> {
    let (traces, options) = match &request.kind {
        ControlPresentationKind::Plot { traces, options } => (traces.as_slice(), Some(options)),
        ControlPresentationKind::Print(traces) => (traces.as_slice(), None),
        ControlPresentationKind::UnitsChanged { .. }
        | ControlPresentationKind::Text(_)
        | ControlPresentationKind::Listing
        | ControlPresentationKind::ResourceUsage
        | ControlPresentationKind::Ignored(_)
        | ControlPresentationKind::File { .. } => return Ok(Vec::new()),
    };
    let summary_stdout = args.summary.as_deref() == Some(Path::new("-"));
    if options.is_none() && !summary_stdout {
        // An authored print is output, including under --quiet. That flag
        // suppresses progress chatter, not the script's requested table.
        let mut stdout = std::io::stdout().lock();
        let mut scalar_index = 0;
        let mut trace_index = 0;
        for position in 0..traces.len().saturating_add(request.scalars.len()) {
            if rspice_core::AbortSignal::is_aborted(&crate::abort::ProcessAbort) {
                return Err(cancellation_cli_error(args.timeout));
            }
            if let Some(scalar) = request
                .scalars
                .get(scalar_index)
                .filter(|scalar| scalar.position == position)
            {
                use rspice_core::execution::result_document::{ScalarUnavailability, ScalarValue};
                let text = match scalar.scalar.value() {
                    ScalarValue::Unavailable {
                        reason: ScalarUnavailability::PositiveInfinity,
                    } => "inf".to_owned(),
                    ScalarValue::Unavailable {
                        reason: ScalarUnavailability::NegativeInfinity,
                    } => "-inf".to_owned(),
                    value => {
                        serde_json::to_string(value).map_err(|error| CliError::InternalError {
                            message: error.to_string(),
                        })?
                    }
                };
                let unit = scalar
                    .scalar
                    .unit()
                    .map_or_else(|| "unspecified".into(), |unit| unit.symbol());
                writeln!(
                    stdout,
                    "# {} [{}]\n{}",
                    scalar.scalar.display_name(),
                    unit,
                    text
                )
                .map_err(|error| CliError::output_error(Path::new("stdout"), error))?;
                scalar_index += 1;
                continue;
            }
            let trace = traces
                .get(trace_index)
                .ok_or_else(|| CliError::InternalError {
                    message: "control print scalar positions do not match its traces".into(),
                })?;
            trace_index += 1;
            writeln!(
                stdout,
                "# {} [{}] vs {} [{}]",
                trace.y.expression,
                trace.y.unit.symbol(),
                trace.x.expression,
                trace.x.unit.symbol()
            )
            .map_err(|error| CliError::output_error(Path::new("stdout"), error))?;
            for (x, y) in trace.x.samples.iter().zip(&trace.y.samples) {
                rspice_core::AbortSignal::is_aborted(&crate::abort::ProcessAbort)
                    .then(|| cancellation_cli_error(args.timeout))
                    .map_or(Ok(()), Err)?;
                writeln!(
                    stdout,
                    "{:.17e}\t{:.17e}\t{:.17e}\t{:.17e}",
                    x.re, x.im, y.re, y.im
                )
                .map_err(|error| CliError::output_error(Path::new("stdout"), error))?;
            }
        }
        stdout
            .flush()
            .map_err(|error| CliError::output_error(Path::new("stdout"), error))?;
    }
    let selected = resolve_output_path(args.output.clone(), config)?;
    if selected.is_none() && options.is_none() && !summary_stdout {
        return Ok(Vec::new());
    }
    let mut base = selected.unwrap_or_else(|| {
        let path = if args.input.as_os_str() == "-" {
            PathBuf::from("rspice-control")
        } else {
            args.input.with_extension("control")
        };
        args.selected_corner
            .as_deref()
            .map_or_else(|| path.clone(), |corner| tag_output_path(&path, corner))
    });
    if let Some(label) = run_label {
        base = tag_output_path(&base, &sanitize_run_tag(label));
    }
    let tagged = tag_output_path(&base, &format!("control-{ordinal:03}"));
    let data_path = tagged.with_extension("json");
    let svg_path = tagged.with_extension("svg");
    let byte_limit = presentation_byte_limit(config);
    let document = PresentationDocument {
        schema: "rspice.control-presentation",
        version: 3,
        command: &request.command.name,
        line: request.command.line,
        title: options.and_then(|options| options.title.as_deref()),
        x_limits: options.and_then(|options| options.x_limits),
        y_limits: options.and_then(|options| options.y_limits),
        x_logarithmic: options.is_some_and(|options| options.x_logarithmic),
        y_logarithmic: options.is_some_and(|options| options.y_logarithmic),
        traces: traces
            .iter()
            .map(|trace| TraceDocument {
                x: vector_document(&trace.x, circuit),
                y: vector_document(&trace.y, circuit),
            })
            .collect(),
        scalars: request
            .scalars
            .iter()
            .map(|scalar| ScalarDocument {
                position: scalar.position,
                dataset: &scalar.dataset,
                scalar: &scalar.scalar,
            })
            .collect(),
    };
    publish_presentation(&data_path, byte_limit, args, |writer| {
        serde_json::to_writer(writer, &document).map_err(|error| error.to_string())
    })?;
    let mut paths = vec![data_path];
    if let Some(options) = options {
        publish_presentation(&svg_path, byte_limit, args, |writer| {
            render_svg(writer, traces, options).map_err(|error| error.to_string())
        })?;
        paths.push(svg_path);
    }
    if !quiet {
        for path in &paths {
            crate::console::line(format_args!("Control output: {}", path.display()))?;
        }
    }
    Ok(paths)
}

/// Batch output of `echo`, `display`, `setplot`, `listing` and `rusage`, and
/// the warning for a command ngspice -b ignores. Script text is output even
/// under --quiet, as an authored print is; it yields stdout to a summary.
pub(super) fn batch_text(
    kind: &ControlPresentationKind,
    netlist: &Netlist,
    args: &RunArgs,
    quiet: bool,
    started: Instant,
) -> Result<(), CliError> {
    let text = match kind {
        ControlPresentationKind::Text(text) => text.clone(),
        ControlPresentationKind::Listing => listing(netlist, args),
        // ngspice prints its own CPU and memory figures here; the elapsed time
        // is the one this host measures for the whole script.
        ControlPresentationKind::ResourceUsage => format!(
            "\nTotal elapsed time (seconds) = {:.3} \n",
            started.elapsed().as_secs_f64()
        ),
        ControlPresentationKind::Ignored(reason) => {
            if !quiet {
                crate::console::diagnostic_line(format_args!("Warning: {reason}"));
            }
            return Ok(());
        }
        _ => return Ok(()),
    };
    if args.summary.as_deref() == Some(Path::new("-")) {
        return Ok(());
    }
    crate::console::bytes(text.as_bytes())
}

/// ngspice `listing` (frontend/inp.c `inp_list`, logical form): the title,
/// then each non-comment logical line under its line number, then `.end`.
fn listing(netlist: &Netlist, args: &RunArgs) -> String {
    let mut text = format!("\t{}\n\n", netlist.title);
    let source = (args.input.as_os_str() != "-")
        .then(|| std::fs::read_to_string(&args.input).ok())
        .flatten()
        .unwrap_or_default();
    let mut lines: Vec<(usize, String)> = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix('+')
            && let Some((_, logical)) = lines.last_mut()
        {
            logical.push(' ');
            logical.push_str(rest.trim());
            continue;
        }
        let lower = trimmed.to_ascii_lowercase();
        if trimmed.is_empty()
            || trimmed.starts_with('*')
            || (lower.starts_with(".end")
                && !lower[4..].starts_with(|c: char| c.is_ascii_alphabetic()))
        {
            continue;
        }
        lines.push((index + 1, trimmed.to_owned()));
    }
    let end = lines.last().map_or(1, |(number, _)| number + 1);
    for (number, line) in lines {
        text.push_str(&format!("{number:6} : {line}\n"));
    }
    text.push_str(&format!("{end:6} : .end\n"));
    text
}

#[derive(Serialize)]
struct PresentationDocument<'a> {
    schema: &'static str,
    version: u32,
    command: &'a str,
    line: usize,
    title: Option<&'a str>,
    x_limits: Option<[f64; 2]>,
    y_limits: Option<[f64; 2]>,
    x_logarithmic: bool,
    y_logarithmic: bool,
    traces: Vec<TraceDocument<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    scalars: Vec<ScalarDocument<'a>>,
}

#[derive(Serialize)]
struct ScalarDocument<'a> {
    position: usize,
    dataset: &'a str,
    scalar: &'a rspice_core::execution::result_document::ResultScalar,
}

#[derive(Serialize)]
struct TraceDocument<'a> {
    x: VectorDocument<'a>,
    y: VectorDocument<'a>,
}

#[derive(Serialize)]
struct VectorDocument<'a> {
    expression: &'a str,
    dataset: &'a str,
    unit: String,
    /// Complex samples serialize as [real, imaginary], without projection.
    samples: &'a [rspice_core::ComplexValue],
    current_sources: Vec<CurrentDocument<'a>>,
    voltage_sources: Vec<VoltageDocument<'a>>,
}

#[derive(Serialize)]
struct CurrentDocument<'a> {
    dataset: &'a str,
    owner: &'a rspice_core::CurrentImpulseOwner,
    /// Null means unrecorded; only a complete empty trace proves absence.
    impulses: Option<&'a rspice_core::CurrentImpulseTrace>,
}

#[derive(Serialize)]
struct VoltageDocument<'a> {
    dataset: &'a str,
    node_name: &'a str,
    impulses: Option<&'a rspice_core::VoltageImpulseTrace>,
}

fn vector_document<'a>(
    vector: &'a ControlVector,
    circuit: &'a ControlCircuit,
) -> VectorDocument<'a> {
    VectorDocument {
        expression: &vector.expression,
        dataset: &vector.dataset,
        unit: vector.unit.symbol(),
        samples: &vector.samples,
        voltage_sources: vector
            .voltage_sources
            .iter()
            .map(|source| {
                // A destroyed plot's name can return, so the newest dataset
                // of the name is the one the vector read.
                let impulses = circuit
                    .datasets()
                    .iter()
                    .rev()
                    .find(|dataset| dataset.name == source.dataset)
                    .and_then(|dataset| match &dataset.result {
                        ControlAnalysisResult::Transient(result) => {
                            result.voltage_impulses.as_deref()
                        }
                        _ => None,
                    })
                    .and_then(|traces| {
                        traces
                            .iter()
                            .find(|trace| trace.node_name.eq_ignore_ascii_case(&source.node_name))
                    });
                VoltageDocument {
                    dataset: &source.dataset,
                    node_name: &source.node_name,
                    impulses,
                }
            })
            .collect(),
        current_sources: vector
            .current_sources
            .iter()
            .map(|source| {
                let impulses = circuit
                    .datasets()
                    .iter()
                    .rev()
                    .find(|dataset| dataset.name == source.dataset)
                    .and_then(|dataset| match &dataset.result {
                        ControlAnalysisResult::Transient(result) => {
                            result.current_impulses.as_deref()
                        }
                        _ => None,
                    })
                    .and_then(|traces| {
                        traces
                            .iter()
                            .find(|trace| same_owner(&trace.owner, &source.owner))
                    });
                CurrentDocument {
                    dataset: &source.dataset,
                    owner: &source.owner,
                    impulses,
                }
            })
            .collect(),
    }
}

fn same_owner(a: &rspice_core::CurrentImpulseOwner, b: &rspice_core::CurrentImpulseOwner) -> bool {
    use rspice_core::CurrentImpulseOwner::*;
    match (a, b) {
        (Branch { branch_name: a }, Branch { branch_name: b }) => a.eq_ignore_ascii_case(b),
        (
            DeviceLead {
                device_name: a,
                parameter: ap,
            },
            DeviceLead {
                device_name: b,
                parameter: bp,
            },
        ) => a.eq_ignore_ascii_case(b) && ap.eq_ignore_ascii_case(bp),
        _ => false,
    }
}

/// The files a control script's redirections write, in the order it first
/// named them. Each is published once, when the script ends, since a run
/// claims a destination once.
#[derive(Default)]
pub(super) struct ControlFiles {
    files: Vec<(PathBuf, String)>,
}

impl ControlFiles {
    /// Record a redirection's text: `>` replaces the file, `>>` appends to
    /// it, to the bytes already on disk when this run has not written it.
    /// A relative path names a file in the deck's directory, where ngspice
    /// -b run from there writes it; a path that leaves that directory is
    /// refused with the reason.
    pub(super) fn record(
        &mut self,
        path: &str,
        text: &str,
        append: bool,
        args: &RunArgs,
    ) -> Result<Result<(), String>, CliError> {
        let destination = match control_file_destination(path, args)? {
            Ok(destination) => destination,
            Err(reason) => return Ok(Err(reason)),
        };
        match self
            .files
            .iter_mut()
            .find(|(known, _)| *known == destination)
        {
            Some((_, contents)) if append => contents.push_str(text),
            Some((_, contents)) => *contents = text.to_owned(),
            None => {
                let mut contents = if append {
                    match std::fs::read_to_string(&destination) {
                        Ok(existing) => existing,
                        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
                        Err(error) => return Err(CliError::output_error(&destination, error)),
                    }
                } else {
                    String::new()
                };
                contents.push_str(text);
                self.files.push((destination, contents));
            }
        }
        Ok(Ok(()))
    }

    /// Publish every recorded file; returns their paths.
    pub(super) fn publish(self) -> Result<Vec<PathBuf>, CliError> {
        let mut written = Vec::with_capacity(self.files.len());
        for (path, contents) in self.files {
            publish_bytes(&path, contents.as_bytes())?;
            written.push(path);
        }
        Ok(written)
    }
}

fn control_file_destination(
    path: &str,
    args: &RunArgs,
) -> Result<Result<PathBuf, String>, CliError> {
    let directory = if crate::commands::is_stdin(&args.input) {
        PathBuf::from(".")
    } else {
        match args.input.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
            _ => PathBuf::from("."),
        }
    };
    let directory = std::path::absolute(&directory)
        .map_err(|error| CliError::output_error(&directory, error))?;
    let directory = lexical(&directory);
    let destination = lexical(&directory.join(path));
    if destination.starts_with(&directory) && destination != directory {
        Ok(Ok(destination))
    } else {
        Ok(Err(format!(
            "`{path}` resolves to {}, outside the deck's directory {}; a control script \
             writes files only inside it",
            destination.display(),
            directory.display()
        )))
    }
}

/// Lexical normalization: `.` dropped and `..` folded into its parent, so a
/// path that climbs out of the deck's directory no longer starts with it.
fn lexical(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    out.push(component);
                }
            }
            other => out.push(other),
        }
    }
    out
}

fn publish_bytes(path: &Path, bytes: &[u8]) -> Result<(), CliError> {
    publish::artifact(path, |writer| {
        writer
            .write_all(bytes)
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

/// Upper bound on the bytes one sample takes in a presentation file: a JSON
/// `{re,im}` pair for each of the trace's two coordinates, or an SVG point.
const PRESENTATION_BYTES_PER_VALUE: u64 = 128;

/// What one presentation file may grow to, scaled from the result-value cap
/// rather than the budget for the data files a simulation reads: a batch run
/// that plots every vector it saved fits, and a plot that repeats its samples
/// past that scale fails before it fills the disk.
fn presentation_byte_limit(config: &Config) -> u64 {
    u64::try_from(config.resources.limits().max_result_values)
        .unwrap_or(u64::MAX)
        .saturating_mul(PRESENTATION_BYTES_PER_VALUE)
}

/// Streams one presentation file to its staged destination, refusing it at
/// its byte limit and polling for cancellation between writes, so a plot's
/// text is never held in memory beside the samples it came from.
struct PresentationWriter<'a> {
    destination: &'a mut dyn Write,
    written: u64,
    byte_limit: u64,
    failure: Option<BoundedWriteFailure>,
}

impl Write for PresentationWriter<'_> {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        if rspice_core::AbortSignal::is_aborted(&crate::abort::ProcessAbort) {
            self.failure = Some(BoundedWriteFailure::Aborted);
            return Err(std::io::Error::other("presentation cancelled"));
        }
        let written = self.written.saturating_add(buffer.len() as u64);
        if written > self.byte_limit {
            self.failure = Some(BoundedWriteFailure::ByteLimitExceeded {
                limit_bytes: self.byte_limit,
            });
            return Err(std::io::Error::other("presentation file too large"));
        }
        self.destination.write_all(buffer)?;
        self.written = written;
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.destination.flush()
    }
}

fn publish_presentation(
    path: &Path,
    byte_limit: u64,
    args: &RunArgs,
    write: impl FnOnce(&mut PresentationWriter<'_>) -> Result<(), String>,
) -> Result<(), CliError> {
    publish::artifact(path, |destination| {
        let mut writer = PresentationWriter {
            destination,
            written: 0,
            byte_limit,
            failure: None,
        };
        write(&mut writer).map_err(|message| writer_error(writer.failure, path, args, message))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

fn writer_error(
    failure: Option<BoundedWriteFailure>,
    path: &Path,
    args: &RunArgs,
    message: String,
) -> CliError {
    match failure {
        Some(BoundedWriteFailure::Aborted) => cancellation_cli_error(args.timeout),
        Some(BoundedWriteFailure::ByteLimitExceeded { limit_bytes }) => {
            let values =
                usize::try_from(limit_bytes / PRESENTATION_BYTES_PER_VALUE).unwrap_or(usize::MAX);
            CliError::ResourceLimit {
                path: path.to_path_buf(),
                source: rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::ResultValues,
                    requested: values.saturating_add(1),
                    limit: values,
                },
            }
        }
        _ => CliError::output_error(path, std::io::Error::other(message)),
    }
}

fn xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn transformed(value: f64, log: bool) -> Option<f64> {
    if log {
        (value > 0.0).then(|| value.log10())
    } else {
        Some(value)
    }
}

fn extent(
    traces: &[ControlTrace],
    x: bool,
    log: bool,
    limits: Option<[f64; 2]>,
) -> std::io::Result<[f64; 2]> {
    if let Some(limits) = limits {
        return Ok([
            transformed(limits[0], log)
                .ok_or_else(|| std::io::Error::other("invalid logarithmic limit"))?,
            transformed(limits[1], log)
                .ok_or_else(|| std::io::Error::other("invalid logarithmic limit"))?,
        ]);
    }
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for trace in traces {
        let vector = if x { &trace.x } else { &trace.y };
        // A complex sample plots its real part, as ngspice draws a
        // rectangular grid (frontend/plotting/graf.c:1048); the companion
        // JSON keeps both parts.
        for sample in &vector.samples {
            if let Some(value) = transformed(sample.re, log) {
                low = low.min(value);
                high = high.max(value);
            }
        }
    }
    if !low.is_finite() || !high.is_finite() {
        return Err(std::io::Error::other(
            "plot has no samples in the axis domain",
        ));
    }
    if low == high {
        let padding = (low.abs() * 0.05).max(1.0);
        let lower = low - padding;
        let upper = high + padding;
        if lower.is_finite() {
            low = lower;
        }
        if upper.is_finite() {
            high = upper;
        }
    }
    Ok([low, high])
}

fn fraction(value: f64, bounds: [f64; 2]) -> f64 {
    let scale = bounds[0].abs().max(bounds[1].abs()).max(f64::MIN_POSITIVE);
    (value / scale - bounds[0] / scale) / (bounds[1] / scale - bounds[0] / scale)
}

fn render_svg(
    writer: &mut PresentationWriter<'_>,
    traces: &[ControlTrace],
    options: &rspice_core::engine::ControlPlotOptions,
) -> std::io::Result<()> {
    let xb = extent(traces, true, options.x_logarithmic, options.x_limits)?;
    let yb = extent(traces, false, options.y_logarithmic, options.y_limits)?;
    let height = 590usize.max(100usize.saturating_add(traces.len().saturating_mul(24)));
    let title = options.title.as_deref().unwrap_or("RSpice control plot");
    writeln!(
        writer,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"{height}\" viewBox=\"0 0 1200 {height}\" role=\"img\"><title>{}</title><desc>Resolved control-script traces. Exact samples and current impulse coverage are in the companion JSON file.</desc><rect width=\"1200\" height=\"{height}\" fill=\"white\"/><defs><clipPath id=\"plot\"><rect x=\"96\" y=\"70\" width=\"660\" height=\"410\"/></clipPath></defs><g font-family=\"Arial, sans-serif\" fill=\"#172b4d\"><text x=\"96\" y=\"34\" font-size=\"20\">{}</text>",
        xml(title),
        xml(title)
    )?;
    for index in 0..=5 {
        let r = index as f64 / 5.0;
        let x = 96.0 + 660.0 * r;
        let y = 480.0 - 410.0 * r;
        let xv = xb[0] * (1.0 - r) + xb[1] * r;
        let yv = yb[0] * (1.0 - r) + yb[1] * r;
        let xv = if options.x_logarithmic {
            10f64.powf(xv)
        } else {
            xv
        };
        let yv = if options.y_logarithmic {
            10f64.powf(yv)
        } else {
            yv
        };
        writeln!(
            writer,
            "<path d=\"M {x} 70 V 480 M 96 {y} H 756\" stroke=\"#e2e8f0\"/><text x=\"{x}\" y=\"500\" font-size=\"11\" text-anchor=\"middle\">{xv:.3e}</text><text x=\"86\" y=\"{}\" font-size=\"11\" text-anchor=\"end\">{yv:.3e}</text>",
            y + 4.0
        )?;
    }
    writeln!(
        writer,
        "<rect x=\"96\" y=\"70\" width=\"660\" height=\"410\" fill=\"none\" stroke=\"#65758b\"/>"
    )?;
    let colors = [
        "#145da0", "#c44b16", "#19805c", "#8153a5", "#bd3369", "#347c86", "#887018", "#535b68",
    ];
    for (index, trace) in traces.iter().enumerate() {
        let color = colors[index % colors.len()];
        let ly = 88 + index * 24;
        let label = format!("{} [{}]", trace.y.expression, trace.y.unit.symbol());
        let short: String = label.chars().take(54).collect();
        let short = if short.len() < label.len() {
            format!("{short}…")
        } else {
            short
        };
        writeln!(
            writer,
            "<path d=\"M 785 {ly} h 22\" stroke=\"{color}\" stroke-width=\"2\"/><text x=\"816\" y=\"{}\" font-size=\"12\"><title>{}</title>{}</text><path clip-path=\"url(#plot)\" stroke=\"{color}\" stroke-width=\"1.6\" fill=\"none\" d=\"",
            ly + 4,
            xml(&label),
            xml(&short)
        )?;
        let mut connected = false;
        for (x, y) in trace.x.samples.iter().zip(&trace.y.samples) {
            let (Some(x), Some(y)) = (
                transformed(x.re, options.x_logarithmic),
                transformed(y.re, options.y_logarithmic),
            ) else {
                connected = false;
                continue;
            };
            let px = 96.0 + 660.0 * fraction(x, xb);
            let py = 480.0 - 410.0 * fraction(y, yb);
            if !px.is_finite() || !py.is_finite() {
                return Err(std::io::Error::other(
                    "plot coordinates exceed numeric range",
                ));
            }
            write!(
                writer,
                "{} {px:.6} {py:.6} ",
                if connected { "L" } else { "M" }
            )?;
            connected = true;
        }
        writeln!(writer, "\"/>")?;
    }
    if let Some(first) = traces.first() {
        writeln!(
            writer,
            "<text x=\"426\" y=\"530\" font-size=\"14\" text-anchor=\"middle\">{} [{}]</text>",
            xml(&first.x.expression),
            xml(&first.x.unit.symbol())
        )?;
    }
    writeln!(
        writer,
        "<text x=\"96\" y=\"558\" font-size=\"11\" fill=\"#52637b\">Exact per-trace axes, samples, units and current impulse coverage: companion JSON.</text>"
    )?;
    if options.x_logarithmic || options.y_logarithmic {
        writeln!(
            writer,
            "<text x=\"96\" y=\"576\" font-size=\"11\" fill=\"#52637b\">Nonpositive samples are outside the logarithmic axis domain and are omitted from this drawing.</text>"
        )?;
    }
    writeln!(writer, "</g></svg>")
}
