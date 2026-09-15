//! Script presentation files: readable SVG plus lossless, typed sample data.

use super::*;
use rspice_core::engine::{
    ControlPresentation, ControlPresentationKind, ControlTrace, ControlVector,
};
use rspice_core::execution::{BoundedAbortWriter, BoundedWriteFailure};
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
        ControlPresentationKind::UnitsChanged { .. } => return Ok(Vec::new()),
    };
    if options.is_none() {
        // An authored print is output, including under --quiet. That flag
        // suppresses progress chatter, not the script's requested table.
        let mut stdout = std::io::stdout().lock();
        for trace in traces {
            writeln!(
                stdout,
                "# {} [{}] vs {} [{}]",
                trace.y.expression,
                trace.y.unit.symbol(),
                trace.x.expression,
                trace.x.unit.symbol()
            )
            .map_err(|error| CliError::output_error(Path::new("<stdout>"), error))?;
            for (x, y) in trace.x.samples.iter().zip(&trace.y.samples) {
                rspice_core::AbortSignal::is_aborted(&crate::abort::ProcessAbort)
                    .then(|| cancellation_cli_error(args.timeout))
                    .map_or(Ok(()), Err)?;
                writeln!(
                    stdout,
                    "{:.17e}\t{:.17e}\t{:.17e}\t{:.17e}",
                    x.re, x.im, y.re, y.im
                )
                .map_err(|error| CliError::output_error(Path::new("<stdout>"), error))?;
            }
        }
    }
    let selected = resolve_output_path(args.output.clone(), config)?;
    if selected.is_none() && options.is_none() {
        return Ok(Vec::new());
    }
    let mut base = selected.unwrap_or_else(|| {
        if args.input.as_os_str() == "-" {
            PathBuf::from("rspice-control")
        } else {
            args.input.with_extension("control")
        }
    });
    if let Some(label) = run_label {
        base = tag_output_path(&base, &sanitize_run_tag(label));
    }
    let tagged = tag_output_path(&base, &format!("control-{ordinal:03}"));
    let data_path = tagged.with_extension("json");
    let svg_path = tagged.with_extension("svg");
    let mut writer = BoundedAbortWriter::new(
        &crate::abort::ProcessAbort,
        config.resources.limits().max_external_data_bytes as u64,
    );
    let document = PresentationDocument {
        schema: "rspice.control-presentation",
        version: 1,
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
    };
    if let Err(error) = serde_json::to_writer(&mut writer, &document) {
        return Err(writer_error(&writer, &data_path, args, error.to_string()));
    }
    publish_bytes(&data_path, &writer.into_bytes())?;
    let mut paths = vec![data_path];
    if let Some(options) = options {
        let mut writer = BoundedAbortWriter::new(
            &crate::abort::ProcessAbort,
            config.resources.limits().max_external_data_bytes as u64,
        );
        render_svg(&mut writer, traces, options)
            .map_err(|error| writer_error(&writer, &svg_path, args, error.to_string()))?;
        publish_bytes(&svg_path, &writer.into_bytes())?;
        paths.push(svg_path);
    }
    if !quiet {
        for path in &paths {
            println!("Control output: {}", path.display());
        }
    }
    Ok(paths)
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
}

#[derive(Serialize)]
struct CurrentDocument<'a> {
    dataset: &'a str,
    owner: &'a rspice_core::CurrentImpulseOwner,
    /// Null means unrecorded; only a complete empty trace proves absence.
    impulses: Option<&'a rspice_core::CurrentImpulseTrace>,
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
        current_sources: vector
            .current_sources
            .iter()
            .map(|source| {
                let impulses = circuit
                    .datasets()
                    .iter()
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

fn publish_bytes(path: &Path, bytes: &[u8]) -> Result<(), CliError> {
    publish::artifact(path, |writer| {
        writer
            .write_all(bytes)
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

fn writer_error(
    writer: &BoundedAbortWriter<'_>,
    path: &Path,
    args: &RunArgs,
    message: String,
) -> CliError {
    match writer.failure() {
        Some(BoundedWriteFailure::Aborted) => cancellation_cli_error(args.timeout),
        Some(BoundedWriteFailure::ByteLimitExceeded { limit_bytes }) => CliError::ResourceLimit {
            path: path.to_path_buf(),
            source: rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::ExternalDataBytes,
                requested: usize::try_from(limit_bytes)
                    .unwrap_or(usize::MAX)
                    .saturating_add(1),
                limit: usize::try_from(limit_bytes).unwrap_or(usize::MAX),
            },
        },
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
        for sample in &vector.samples {
            if sample.im != 0.0 {
                return Err(std::io::Error::other(
                    "SVG plots require real expressions; use real(...) or abs(...) for complex data",
                ));
            }
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
    writer: &mut BoundedAbortWriter<'_>,
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
            if x.im != 0.0 || y.im != 0.0 {
                return Err(std::io::Error::other("SVG plots require real expressions"));
            }
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
