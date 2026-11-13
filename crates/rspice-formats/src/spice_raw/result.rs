//! SPICE rawfile publication of one retained analysis.
//!
//! The bytes are the core's. The analysis is one plot written by
//! [`write_raw_plot`] with the table metadata the CLI's `--format raw` writes
//! ([`write_raw_table_layout_metadata`]), and a transient's event histories
//! follow it as the per-node plots [`write_event_plots`] appends after an
//! engine run. A file written here therefore reads the way `rspice run -f raw`
//! output does: in ngspice and LTspice as an ordinary rawfile, and in RSpice
//! with its units and exact labels as well.
//!
//! # What a rawfile carries
//!
//! One sampled analysis on one scale: a transient on `time`, an AC sweep on
//! `frequency` or a DC sweep on its swept quantity. A rawfile plot names its
//! analysis in `Plotname:`, and the three names written here are the ones
//! ngspice writes and every rawfile reader classifies, this product's
//! included. Another analysis is refused by name rather than published under
//! one of those headings.
//!
//! An AC sweep that retains its rectangular samples is a complex plot of
//! them, as ngspice writes one; the magnitude a sheet draws is derived from
//! those and is not written beside them. A real trace in a complex plot is
//! listed as real in the table metadata, so RSpice reads it back as the real
//! trace it was rather than as a phase of zero.
//!
//! # Event histories
//!
//! A transient's digital and real event nodes have times of their own, so
//! each is its own plot after the analysis plot, and a declared bus is one
//! more plot over its members. Those plots are written only for the nodes the
//! caller names: the analysis plot is unchanged by them, and a reader that
//! wants only the analysis reads the first plot and stops.

use rspice_core::io::ltspice_raw::write_raw_table_layout_metadata;
use rspice_core::io::raw_export::{RawPlotLayout, write_raw_exponent, write_raw_plot};
use rspice_core::io::{RawFormat, write_event_plots};
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::result_import::waveforms::WaveformImportLimits;
use rspice_results::waveform::RetainedWaveform;

use crate::SignalType;
use crate::waveform_io::result::{
    WaveformExportError, axis_signal_for_analysis, shared_coordinate,
    signal_type_from_waveform_name,
};

#[derive(Debug)]
pub enum RawProjectionError {
    UnsupportedAnalysis {
        analysis: AnalysisType,
        label: String,
    },
    Waveforms(WaveformExportError),
    /// The retained event history could not be stated as rawfile plots.
    Events(String),
    Write(std::io::Error),
}

impl std::fmt::Display for RawProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedAnalysis { label, .. } => write!(
                f,
                "A SPICE rawfile carries one sampled analysis: a transient, an AC sweep or a DC \
                 sweep. '{label}' is none of those, and a rawfile reader would classify it as one \
                 of them by its plot name. Export CSV, or an RSpice bundle, which carries this \
                 analysis whole."
            ),
            Self::Waveforms(source) => source.fmt(f),
            Self::Events(message) => write!(f, "the event history cannot be written: {message}"),
            Self::Write(source) => write!(f, "the rawfile could not be written: {source}"),
        }
    }
}

impl std::error::Error for RawProjectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Waveforms(source) => Some(source),
            Self::Write(source) => Some(source),
            Self::UnsupportedAnalysis { .. } | Self::Events(_) => None,
        }
    }
}

/// One encoded rawfile, and what the reader is owed about it.
#[derive(Debug)]
pub struct RawExport {
    pub bytes: Vec<u8>,
    /// The analysis plot's `Plotname:`.
    pub plot_name: &'static str,
    /// The analysis plot's scale variable.
    pub scale_name: String,
    pub points: usize,
    /// Variables of the analysis plot, the scale included.
    pub variables: usize,
    pub complex: bool,
    /// Event-node plots appended after the analysis plot.
    pub event_plots: usize,
    /// Declared-bus plots appended after the node plots.
    pub bus_plots: usize,
}

/// The `Plotname:` ngspice writes for an analysis, which is also what every
/// rawfile reader classifies the plot by.
const fn plot_name(analysis: AnalysisType) -> Option<&'static str> {
    match analysis {
        AnalysisType::Transient => Some("Transient Analysis"),
        AnalysisType::Ac => Some("AC Analysis"),
        AnalysisType::DcSweep => Some("DC transfer characteristic"),
        _ => None,
    }
}

/// One variable of the analysis plot after the scale.
struct Column<'a> {
    name: &'a str,
    var_type: &'static str,
    unit: Option<&'a str>,
    real: &'a [f64],
    imag: Option<&'a [f64]>,
}

/// The rawfile type a signal is declared with: its unit's quantity when it
/// states one, else what its name says, else `parameter`, the type the CLI
/// gives a scalar. The exact unit travels in the table metadata either way.
fn variable_type(name: &str, unit: Option<&str>) -> &'static str {
    match unit {
        Some("V") => "voltage",
        Some("A") => "current",
        Some("logic") => "digital",
        _ => match signal_type_from_waveform_name(name) {
            SignalType::Voltage => "voltage",
            SignalType::Current => "current",
            _ => "parameter",
        },
    }
}

/// The scale's name, rawfile type and unit.
fn scale<W>(analysis: &AnalysisResult<W>) -> (String, &'static str, Option<String>) {
    let (name, kind) = axis_signal_for_analysis(analysis);
    let imported_unit = analysis
        .imported_coordinate()
        .and_then(|coordinate| coordinate.unit.clone());
    let (var_type, unit) = match kind {
        SignalType::Time => ("time", Some("s".to_owned())),
        SignalType::Frequency => ("frequency", Some("Hz".to_owned())),
        SignalType::Voltage => ("voltage", Some("V".to_owned())),
        SignalType::Current => ("current", Some("A".to_owned())),
        _ if name.eq_ignore_ascii_case("temp") => ("temperature", imported_unit),
        _ => ("notype", imported_unit),
    };
    (name.to_owned(), var_type, unit)
}

/// Encode `analysis` as a rawfile: its `waveforms` as the analysis plot, then
/// a plot for every event node of a transient that `carries_event_node`
/// accepts, and one for every declared bus whose members all ride along.
pub fn encode_result_raw<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    waveforms: &[&W],
    carries_event_node: &dyn Fn(&str) -> bool,
    format: RawFormat,
    limits: WaveformImportLimits,
) -> Result<RawExport, RawProjectionError> {
    let Some(plot_name) = plot_name(analysis.analysis_type) else {
        return Err(RawProjectionError::UnsupportedAnalysis {
            analysis: analysis.analysis_type,
            label: analysis.label.clone(),
        });
    };
    let coordinate = shared_coordinate(waveforms, limits).map_err(RawProjectionError::Waveforms)?;
    let (scale_name, scale_type, scale_unit) = scale(analysis);

    let mut columns: Vec<Column<'_>> = Vec::with_capacity(waveforms.len());
    for waveform in waveforms {
        let waveform = (*waveform).as_ref();
        let column = match &waveform.complex {
            Some(complex) => Column {
                name: &complex.source_name,
                var_type: variable_type(&complex.source_name, waveform.unit.as_deref()),
                unit: waveform.unit.as_deref(),
                real: complex.real.as_ref(),
                imag: Some(complex.imag.as_ref()),
            },
            None => Column {
                name: &waveform.name,
                var_type: variable_type(&waveform.name, waveform.unit.as_deref()),
                unit: waveform.unit.as_deref(),
                real: waveform.y.as_ref(),
                imag: None,
            },
        };
        // Two drawn traces of one retained complex signal are one variable.
        if column.imag.is_some()
            && columns
                .iter()
                .any(|existing| existing.imag.is_some() && existing.name == column.name)
        {
            continue;
        }
        columns.push(column);
    }
    let complex = columns.iter().any(|column| column.imag.is_some());

    let declared = std::iter::once((scale_name.as_str(), scale_type))
        .chain(columns.iter().map(|column| (column.name, column.var_type)))
        .collect::<Vec<_>>();
    let escaped = declared
        .iter()
        .map(|(name, var_type)| (raw_token(name), *var_type))
        .collect::<Vec<_>>();
    let needs_labels = escaped
        .iter()
        .any(|(token, _)| matches!(token, std::borrow::Cow::Owned(_)));
    let units = std::iter::once(scale_unit)
        .chain(columns.iter().map(|column| column.unit.map(str::to_owned)))
        .collect::<Vec<_>>();
    let real_variables = if complex {
        std::iter::once(0)
            .chain(
                columns
                    .iter()
                    .enumerate()
                    .filter_map(|(index, column)| column.imag.is_none().then_some(index + 1)),
            )
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let mut options = Vec::new();
    write_raw_table_layout_metadata(
        &mut options,
        &real_variables,
        &units,
        needs_labels.then_some((plot_name, declared.as_slice())),
    )
    .map_err(RawProjectionError::Write)?;
    let variables = escaped
        .iter()
        .map(|(token, var_type)| (token.as_ref(), *var_type))
        .collect::<Vec<_>>();
    let title = title(&analysis.label, plot_name);
    let date = date(analysis.timestamp);
    let layout = RawPlotLayout {
        title: &title,
        date: &date,
        plot_name,
        options: &options,
        complex,
        variables: &variables,
        points: coordinate.len(),
    };

    let (events, buses) = event_plots(analysis, carries_event_node)?;
    let mut bytes = Vec::new();
    write_raw_plot(
        &mut bytes,
        &layout,
        format,
        write_raw_exponent,
        |row, variable| match variable.checked_sub(1) {
            None => (coordinate[row], 0.0),
            Some(column) => {
                let column = &columns[column];
                (column.real[row], column.imag.map_or(0.0, |imag| imag[row]))
            }
        },
    )
    .and_then(|()| write_event_plots(&mut bytes, &events, &buses, format))
    .map_err(RawProjectionError::Write)?;
    let variables = variables.len();
    Ok(RawExport {
        bytes,
        plot_name,
        scale_name,
        points: coordinate.len(),
        variables,
        complex,
        event_plots: events.len(),
        bus_plots: buses.len(),
    })
}

/// The event plots and bus plots of the nodes the export carries.
///
/// A bus rides along only when all of its members do, because its plot is a
/// grouping of their histories and is decoded against them.
fn event_plots<W>(
    analysis: &AnalysisResult<W>,
    carries_event_node: &dyn Fn(&str) -> bool,
) -> Result<
    (
        Vec<rspice_core::io::RawEventTimeline>,
        Vec<rspice_core::io::RawBusTimeline>,
    ),
    RawProjectionError,
> {
    let Some(AnalysisResultPayload::TransientEvents {
        digital_traces,
        real_traces,
        digital_buses,
        ..
    }) = analysis.result_payload.as_ref()
    else {
        return Ok((Vec::new(), Vec::new()));
    };
    let digital = digital_traces
        .iter()
        .filter(|trace| carries_event_node(&trace.node_name))
        .cloned()
        .collect::<Vec<_>>();
    let real = real_traces
        .iter()
        .filter(|trace| carries_event_node(&trace.node_name))
        .cloned()
        .collect::<Vec<_>>();
    let buses = digital_buses
        .iter()
        .filter(|bus| {
            bus.members
                .iter()
                .all(|member| digital.iter().any(|trace| &trace.node_name == member))
        })
        .map(rspice_core::engine::DigitalBusDeclaration::from)
        .collect::<Vec<_>>();
    let (digital, real) = rspice_results::events::core_event_traces(&digital, &real)
        .map_err(RawProjectionError::Events)?;
    let events = rspice_core::execution::transient_event_plots(&digital, &real);
    let buses = rspice_core::execution::transient_bus_plots(&digital, &buses)
        .map_err(|error| RawProjectionError::Events(error.to_string()))?;
    Ok((events, buses))
}

/// `Title:` is free text to every reader but ends at the line: the analysis
/// label, with anything that would break the header line made a space.
fn title(label: &str, plot_name: &str) -> String {
    let label = label
        .chars()
        .map(|character| {
            if character.is_control() {
                ' '
            } else {
                character
            }
        })
        .collect::<String>();
    let label = label.trim();
    if label.is_empty() {
        plot_name.to_owned()
    } else {
        label.to_owned()
    }
}

/// `Date:` in the `asctime` form ngspice writes, from when the analysis ran.
fn date(timestamp: f64) -> String {
    if timestamp.is_finite() && timestamp >= 0.0 {
        rspice_core::time_compat::asctime_utc(timestamp as u64)
    } else {
        "unstated".to_owned()
    }
}

/// A rawfile declaration has no quoting convention, so whitespace, a control
/// character or the escape introducer `%` is written as `%XX` per UTF-8 byte,
/// and a lone `%` stands for the empty label. The exact label travels in the
/// table metadata beside it. This is the spelling the CLI's table export
/// gives the same names, so the two publish the same declarations.
fn raw_token(value: &str) -> std::borrow::Cow<'_, str> {
    let unsafe_character =
        |character: char| character.is_whitespace() || character.is_control() || character == '%';
    if !value.is_empty() && !value.chars().any(unsafe_character) {
        return std::borrow::Cow::Borrowed(value);
    }
    use std::fmt::Write as _;
    let mut token = String::new();
    for character in value.chars() {
        if unsafe_character(character) {
            for byte in character.encode_utf8(&mut [0; 4]).bytes() {
                write!(token, "%{byte:02X}").expect("writing to a String cannot fail");
            }
        } else {
            token.push(character);
        }
    }
    if token.is_empty() {
        token.push('%');
    }
    std::borrow::Cow::Owned(token)
}

#[cfg(test)]
mod tests;
