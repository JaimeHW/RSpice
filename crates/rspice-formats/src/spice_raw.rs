//! Waveform interpretation over the existing bounded core SPICE RAW reader,
//! and the publication of a retained analysis through the core RAW writer.

use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal};
use rspice_core::io::RawParseError;

#[cfg(feature = "result-waveform")]
pub mod result;

#[derive(Debug)]
pub enum RawReadError {
    Parse(RawParseError),
    NoVariables,
    MultiplePlots(usize),
    Coordinate(String),
    Column(String),
    /// An event plot that declares its family and then contradicts it.
    Events(rspice_core::execution::EventPlotError),
    /// Event plots with no single transient analysis plot to belong to.
    UnattributedEvents {
        transient_plots: usize,
    },
}

impl std::fmt::Display for RawReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(source) => source.fmt(f),
            Self::NoVariables => f.write_str("rawfile contains no variables"),
            Self::MultiplePlots(count) => write!(
                f,
                "rawfile contains {count} plots; select one analysis with rspice convert --section before importing"
            ),
            Self::Coordinate(message) | Self::Column(message) => f.write_str(message),
            Self::Events(source) => source.fmt(f),
            Self::UnattributedEvents { transient_plots: 0 } => f.write_str(
                "rawfile carries event plots but no transient analysis plot for them to belong to",
            ),
            Self::UnattributedEvents { transient_plots } => write!(
                f,
                "rawfile carries event plots beside {transient_plots} transient analysis plots; \
                 event histories belong to exactly one"
            ),
        }
    }
}

impl std::error::Error for RawReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(source) => Some(source),
            Self::Events(source) => Some(source),
            Self::NoVariables
            | Self::MultiplePlots(_)
            | Self::Coordinate(_)
            | Self::Column(_)
            | Self::UnattributedEvents { .. } => None,
        }
    }
}

/// One analysis plot of a rawfile, read as a dataset.
#[derive(Debug)]
pub struct DecodedRawPlot {
    /// The plot's own `Plotname:` title, as the writer gave it.
    pub plot_name: String,
    /// A scalar result (an operating point, a transfer function, noise
    /// totals) indexed by point rather than drawn over a coordinate.
    pub scalar: bool,
    pub dataset: DecodedNumericDataset,
    /// The event histories the file's event plots carry. They belong to the
    /// file's one transient plot and ride on it; every other plot has none.
    pub events: Option<rspice_core::execution::RawEventTraces>,
    /// The `.step` run this dataset is, when it came from an LTspice
    /// `stepped` plot: such a plot holds every run, and reads as one dataset
    /// per run, in run order.
    pub step: Option<RawStep>,
}

/// One run of a `.step` sweep that an LTspice `stepped` plot holds.
#[derive(Debug, Clone, PartialEq)]
pub struct RawStep {
    /// The run's place in the sweep, from 0, as a deck's `.step` point is
    /// numbered.
    pub index: usize,
    /// The value each stepped parameter takes in this run, as the run's log
    /// states it (`.step r=2000` is `("r", 2000.0)`). Empty when no log
    /// beside the rawfile names the runs.
    pub coordinate: Vec<(String, f64)>,
}

/// Read the one dataset a single-plot rawfile holds.
///
/// A file of several plots is refused rather than read in part; read those
/// with [`decode_spice_raw_plots`].
pub fn decode_spice_raw(
    bytes: &[u8],
    limits: rspice_core::ResourceLimits,
) -> Result<DecodedNumericDataset, RawReadError> {
    // Parse the complete bounded file so further plots (including corrupt
    // ones) cannot disappear behind success.
    let mut file = rspice_core::io::ltspice_raw::parse_raw_plots_bytes_with_limits(bytes, limits)
        .map_err(RawReadError::Parse)?;
    drop_noise_totals(&mut file.plots);
    if file.plots.len() > 1 {
        return Err(RawReadError::MultiplePlots(file.plots.len()));
    }
    let parsed = file.plots.pop().ok_or(RawReadError::NoVariables)?;
    decode_plot(parsed)
}

/// Read every analysis plot of a rawfile, in file order.
///
/// ngspice writes one plot per analysis (an operating point and then a
/// transient, say), and RSpice follows a transient with one plot per event
/// node. Analysis plots come back one dataset each. Event plots are not
/// analyses: they are decoded whole and attached to the file's transient
/// plot, and a file whose event plots have no single transient to belong to
/// is refused. Any plot that fails to parse or decode refuses the file.
pub fn decode_spice_raw_plots(
    bytes: &[u8],
    limits: rspice_core::ResourceLimits,
) -> Result<Vec<DecodedRawPlot>, RawReadError> {
    decode_spice_raw_plots_with_log(bytes, None, limits)
}

/// [`decode_spice_raw_plots`], with the simulator's log beside the rawfile.
///
/// An LTspice `stepped` plot holds every run of a `.step` sweep, the scale
/// restarting at its first value for each, and reads as one dataset per run.
/// LTspice's log states each run's parameter values (`.step r=2000`); when
/// it names as many runs as the plot holds, each run carries its values,
/// and otherwise only its number.
pub fn decode_spice_raw_plots_with_log(
    bytes: &[u8],
    log: Option<&[u8]>,
    limits: rspice_core::ResourceLimits,
) -> Result<Vec<DecodedRawPlot>, RawReadError> {
    let mut file = rspice_core::io::ltspice_raw::parse_raw_plots_bytes_with_limits(bytes, limits)
        .map_err(RawReadError::Parse)?;
    let logged_steps = log
        .map(rspice_core::io::ltspice_raw::ltspice_log_step_values)
        .unwrap_or_default();
    let events = rspice_core::execution::decode_event_plots(&file).map_err(RawReadError::Events)?;
    drop_noise_totals(&mut file.plots);
    let mut event_plots = 0_usize;
    let mut decoded = Vec::with_capacity(file.plots.len());
    for parsed in file.plots {
        // The same test the event decoder applies: explicit table metadata
        // makes an ordinary plot of a matching name.
        if !rspice_core::io::ltspice_raw::raw_table_has_coordinate(&parsed.header)
            .map_err(RawReadError::Parse)?
            && rspice_core::io::raw_export::RawEventKind::from_plot_name(&parsed.header.plotname)
                .is_some()
        {
            event_plots += 1;
            continue;
        }
        let plot_name = parsed.header.plotname.trim().to_owned();
        let scalar = parsed.has_ordinal_axis().map_err(RawReadError::Parse)?;
        if scalar || !parsed.is_stepped() {
            decoded.push(DecodedRawPlot {
                plot_name,
                scalar,
                dataset: decode_plot(parsed)?,
                events: None,
                step: None,
            });
            continue;
        }
        let runs = parsed.into_stepped_runs();
        let named = logged_steps.len() == runs.len();
        for (index, run) in runs.into_iter().enumerate() {
            decoded.push(DecodedRawPlot {
                plot_name: plot_name.clone(),
                scalar,
                dataset: decode_plot(run)?,
                events: None,
                step: Some(RawStep {
                    index,
                    coordinate: if named {
                        logged_steps[index].clone()
                    } else {
                        Vec::new()
                    },
                }),
            });
        }
    }
    if event_plots > 0 {
        let mut transients = decoded
            .iter_mut()
            .filter(|plot| !plot.scalar && plot.dataset.domain == crate::WaveformDomain::Transient);
        match (transients.next(), transients.count()) {
            (Some(transient), 0) => transient.events = Some(events),
            (first, more) => {
                return Err(RawReadError::UnattributedEvents {
                    transient_plots: usize::from(first.is_some()) + more,
                });
            }
        }
    }
    if decoded.is_empty() {
        return Err(RawReadError::NoVariables);
    }
    Ok(decoded)
}

/// ngspice follows a noise spectrum with a one-point "Integrated Noise" plot
/// of its band totals: a summary of that dataset, not another one.
fn drop_noise_totals(plots: &mut Vec<rspice_core::io::ltspice_raw::RawWaveformData>) {
    let mut index = 1;
    while index < plots.len() {
        if plots[index - 1]
            .header
            .plotname
            .eq_ignore_ascii_case("Noise Spectral Density Curves")
            && plots[index]
                .header
                .plotname
                .eq_ignore_ascii_case("Integrated Noise")
        {
            plots.remove(index);
        }
        index += 1;
    }
}

fn decode_plot(
    parsed: rspice_core::io::ltspice_raw::RawWaveformData,
) -> Result<DecodedNumericDataset, RawReadError> {
    parsed
        .validate_real_coordinate()
        .map_err(RawReadError::Parse)?;
    let ordinal = parsed.has_ordinal_axis().map_err(RawReadError::Parse)?;
    let domain = coordinate_domain(&parsed, ordinal)?;
    let units = rspice_core::io::ltspice_raw::raw_table_units(&parsed.header)
        .map_err(RawReadError::Parse)?
        .unwrap_or_else(|| {
            let mut units = vec![None; parsed.variables.len()];
            if !ordinal && let Some(coordinate) = parsed.variables.first() {
                // Ordinary SPICE RAW declares SI quantities rather than a
                // separate unit. Explicit table metadata above may instead
                // declare an unknown unit and must never inherit this fallback.
                let unit = match coordinate.var_type.trim().to_ascii_lowercase().as_str() {
                    "time" => Some("s"),
                    "frequency" => Some("Hz"),
                    "voltage" => Some("V"),
                    "current" => Some("A"),
                    "index" => Some("1"),
                    _ => match domain {
                        crate::WaveformDomain::Transient => Some("s"),
                        crate::WaveformDomain::Ac => Some("Hz"),
                        crate::WaveformDomain::DcSweep => None,
                    },
                };
                units[0] = unit.map(str::to_owned);
            }
            units
        });
    let mut waveforms = parsed.waveforms.into_iter().zip(units).peekable();
    let (coordinate_name, coordinate, coordinate_unit) = if ordinal {
        let (first, _) = waveforms.peek().ok_or(RawReadError::NoVariables)?;
        (
            "point".to_owned(),
            (0..first.y.len()).map(|index| index as f64).collect(),
            Some("1".to_owned()),
        )
    } else {
        let (scale, unit) = waveforms.next().ok_or(RawReadError::NoVariables)?;
        (scale.name, scale.y, unit)
    };
    let mut signals = Vec::new();
    let mut columns = waveforms.zip(parsed.variables.iter().skip(usize::from(!ordinal)));
    while let Some(((mut waveform, unit), variable)) = columns.next() {
        use crate::numeric::nullable::{
            DenseNumericColumn, decode_dense_validity, nullable_value_type,
        };
        if nullable_value_type(&variable.var_type).is_some() {
            let ((mask, mask_unit), mask_variable) = columns.next().ok_or_else(|| {
                RawReadError::Column("nullable value column has no validity column".into())
            })?;
            let defined = decode_dense_validity(
                DenseNumericColumn {
                    kind: &variable.var_type,
                    unit: unit.as_deref(),
                    real: &waveform.y,
                    imag: waveform.y_imag.as_deref(),
                },
                DenseNumericColumn {
                    kind: &mask_variable.var_type,
                    unit: mask_unit.as_deref(),
                    real: &mask.y,
                    imag: mask.y_imag.as_deref(),
                },
            )
            .map_err(RawReadError::Column)?;
            for (index, defined) in defined.into_iter().enumerate() {
                if !defined {
                    waveform.y[index] = f64::NAN;
                    if let Some(imag) = &mut waveform.y_imag {
                        imag[index] = f64::NAN;
                    }
                }
            }
        } else if variable.var_type.starts_with("nullable_validity:") {
            return Err(RawReadError::Column(
                "nullable validity column has no preceding value column".into(),
            ));
        }
        signals.push(DecodedNumericSignal {
            name: waveform.name,
            real: waveform.y,
            imag: waveform.y_imag,
            unit,
        });
    }
    let mut dataset = DecodedNumericDataset {
        domain,
        coordinate_name,
        coordinate_unit,
        coordinate,
        signals,
    };
    dataset
        .normalize_coordinate_unit()
        .map_err(RawReadError::Coordinate)?;
    Ok(dataset)
}

fn coordinate_domain(
    parsed: &rspice_core::io::ltspice_raw::RawWaveformData,
    ordinal: bool,
) -> Result<crate::WaveformDomain, RawReadError> {
    use crate::WaveformDomain::{Ac, DcSweep, Transient};
    if ordinal {
        return Ok(DcSweep);
    }
    let coordinate = parsed.variables.first().ok_or(RawReadError::NoVariables)?;
    // The declared independent quantity outranks display titles and the
    // storage representation of dependent signals. Sweeps can be complex.
    let declared = match coordinate.var_type.trim().to_ascii_lowercase().as_str() {
        "time" => Some(Transient),
        "frequency" => Some(Ac),
        "index" | "voltage" | "current" | "temperature" | "resistance" | "capacitance"
        | "inductance" | "power" | "conductance" => Some(DcSweep),
        _ => None,
    };
    if let Some(domain) = declared {
        return Ok(domain);
    }
    match coordinate.name.trim().to_ascii_lowercase().as_str() {
        "time" | "t" | "timestamp" => return Ok(Transient),
        "frequency" | "freq" | "hz" => return Ok(Ac),
        "point" | "index" => return Ok(DcSweep),
        _ => {}
    }
    if rspice_core::io::ltspice_raw::raw_table_has_coordinate(&parsed.header)
        .map_err(RawReadError::Parse)?
    {
        return Ok(DcSweep);
    }
    // Legacy sources sometimes omit a meaningful coordinate declaration.
    // Match analysis titles as titles, not substrings such as "ac" in "package".
    Ok(
        match parsed.header.plotname.trim().to_ascii_lowercase().as_str() {
            "transient analysis" | "transient" | "tran" => Transient,
            "ac analysis" | "ac" | "noise spectral density" | "distortion analysis" => Ac,
            "dc transfer characteristic" | "dc sweep" | "dc analysis" => DcSweep,
            _ if parsed.header.is_complex => Ac,
            _ => DcSweep,
        },
    )
}

#[cfg(test)]
mod ltspice_step_tests;

#[cfg(test)]
mod tests {
    use super::*;

    fn binary_fixture(plot: &str, coordinate: &str, complex: bool) -> Vec<u8> {
        let flags = if complex {
            "complex double"
        } else {
            "real double"
        };
        let kind = match coordinate.to_ascii_lowercase().as_str() {
            "time" => "time",
            "frequency" => "frequency",
            _ => "value",
        };
        let mut bytes = format!("Title: fixture\nPlotname: {plot}\nFlags: {flags}\nNo. Variables: 2\nNo. Points: 1\nVariables:\n0 {coordinate} {kind}\n1 V(out) voltage\nBinary:\n").into_bytes();
        let adjacent = f64::from_bits(1.0_f64.to_bits() + 1);
        let values = if complex {
            vec![2.0, 0.0, adjacent, -0.0]
        } else {
            vec![2.0, adjacent]
        };
        for value in values {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    #[test]
    fn core_raw_projection_preserves_domain_rules_and_exact_samples() {
        use crate::WaveformDomain::{Ac, DcSweep, Transient};
        for (plot, coordinate, complex, domain) in [
            ("unknown", "x", false, DcSweep),
            ("Transient Analysis", "x", false, Transient),
            ("unknown", "TIME", false, Transient),
            ("AC Analysis", "frequency", false, Ac),
            ("unknown", "x", true, Ac),
            ("package transfer", "x", false, DcSweep),
            ("Transient Analysis", "x", true, Transient),
            ("DC transfer characteristic", "x", true, DcSweep),
        ] {
            let bytes = binary_fixture(plot, coordinate, complex);
            let decoded = decode_spice_raw(&bytes, Default::default()).unwrap();
            assert_eq!(decoded.domain, domain);
            assert_eq!(decoded.coordinate_name, coordinate);
            assert_eq!(decoded.coordinate, [2.0]);
            assert_eq!(decoded.signals.len(), 1);
            assert_eq!(decoded.signals[0].name, "V(out)");
            assert_eq!(decoded.signals[0].real[0].to_bits(), 1.0_f64.to_bits() + 1);
            assert!(decoded.signals[0].unit.is_none());
            if complex {
                assert_eq!(
                    decoded.signals[0].imag.as_ref().unwrap()[0].to_bits(),
                    (-0.0_f64).to_bits()
                );
            } else {
                assert!(decoded.signals[0].imag.is_none());
            }
        }
    }

    /// The first three transient rows of the op-then-tran rawfile ngspice-46
    /// writes under `-r` for an RC divider stepped from 0.5 V: its header
    /// lines verbatim, an operating-point plot, then the transient plot.
    const NGSPICE_OP_HEADER: &str = "Title: rc op then tran\nDate: Fri Nov 13 06:07:38  2026\nCommand: ngspice-46, Build \nPlotname: Operating Point\nFlags: real\nNo. Variables: 3\nNo. Points: 1       \nVariables:\n\t0\tv(in)\tvoltage\n\t1\tv(out)\tvoltage\n\t2\ti(v1)\tcurrent\n";
    const NGSPICE_TRAN_HEADER: &str = "Title: rc op then tran\nDate: Fri Nov 13 06:07:38  2026\nCommand: ngspice-46, Build \nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 4\nNo. Points: 3       \nVariables:\n\t0\ttime\ttime\n\t1\tv(in)\tvoltage\n\t2\tv(out)\tvoltage\n\t3\ti(v1)\tcurrent\n";
    const NGSPICE_OP: [f64; 3] = [0.5, 0.375, -0.000125];
    const NGSPICE_TRAN: [[f64; 4]; 3] = [
        [0.0, 0.5, 0.375, -0.000125],
        [
            4.000000000000001e-12,
            0.5002,
            0.37500007995735607,
            -0.00012519992004264395,
        ],
        [
            8.000000000000001e-12,
            0.5004,
            0.375000239829447,
            -0.000125399760170553,
        ],
    ];

    fn ngspice_op_then_tran_binary() -> Vec<u8> {
        let mut bytes = format!("{NGSPICE_OP_HEADER}Binary:\n").into_bytes();
        for value in NGSPICE_OP {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(format!("{NGSPICE_TRAN_HEADER}Binary:\n").as_bytes());
        for value in NGSPICE_TRAN.as_flattened() {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    fn ngspice_op_then_tran_ascii() -> Vec<u8> {
        format!(
            "{NGSPICE_OP_HEADER}Values:\n0\t\t5.000000000000000e-01\n\t3.750000000000000e-01\n\t-1.250000000000000e-04\n\
             {NGSPICE_TRAN_HEADER}Values:\n0\t\t0.000000000000000e+00\n\t5.000000000000000e-01\n\t3.750000000000000e-01\n\t-1.250000000000000e-04\n\
             1\t\t4.000000000000001e-12\n\t5.002000000000000e-01\n\t3.750000799573561e-01\n\t-1.251999200426440e-04\n\
             2\t\t8.000000000000001e-12\n\t5.004000000000000e-01\n\t3.750002398294470e-01\n\t-1.253997601705530e-04\n"
        )
        .into_bytes()
    }

    /// ngspice writes one plot per analysis. Every plot reads back as its own
    /// dataset, in file order: the operating point as a scalar result over
    /// points, the transient over time, each value as written.
    #[test]
    fn every_plot_of_an_ngspice_op_then_tran_rawfile_reads_back() {
        for (bytes, exact) in [
            (ngspice_op_then_tran_binary(), true),
            (ngspice_op_then_tran_ascii(), false),
        ] {
            let plots = decode_spice_raw_plots(&bytes, Default::default()).unwrap();
            let [op, tran] = plots.as_slice() else {
                panic!("two plots, got {}", plots.len());
            };
            assert_eq!(
                (op.plot_name.as_str(), op.scalar),
                ("Operating Point", true)
            );
            assert_eq!(op.dataset.coordinate_name, "point");
            assert_eq!(op.dataset.coordinate, [0.0]);
            let op_values = op
                .dataset
                .signals
                .iter()
                .map(|signal| (signal.name.as_str(), signal.real[0]))
                .collect::<Vec<_>>();
            assert_eq!(
                op_values,
                [("v(in)", 0.5), ("v(out)", 0.375), ("i(v1)", -0.000125)]
            );
            assert_eq!(
                (tran.plot_name.as_str(), tran.scalar),
                ("Transient Analysis", false)
            );
            assert_eq!(tran.dataset.domain, crate::WaveformDomain::Transient);
            assert_eq!(tran.dataset.coordinate_name, "time");
            assert_eq!(tran.dataset.coordinate_unit.as_deref(), Some("s"));
            assert!(op.events.is_none() && tran.events.is_none());
            for (row, expected) in NGSPICE_TRAN.iter().enumerate() {
                let read = std::iter::once(tran.dataset.coordinate[row])
                    .chain(tran.dataset.signals.iter().map(|signal| signal.real[row]));
                for (read, expected) in read.zip(expected) {
                    // ASCII carries sixteen significant digits, binary every bit.
                    let tolerance = if exact { 0.0 } else { expected.abs() * 1e-15 };
                    assert!((read - expected).abs() <= tolerance, "{read} vs {expected}");
                }
            }
            assert!(matches!(
                decode_spice_raw(&bytes, Default::default()),
                Err(RawReadError::MultiplePlots(2))
            ));
        }
    }

    /// A plot that does not parse refuses the file: no successful partial read.
    #[test]
    fn a_broken_further_plot_refuses_the_whole_file() {
        let mut bytes = ngspice_op_then_tran_ascii();
        bytes.extend_from_slice(b"Title: incomplete third plot\n");
        assert!(decode_spice_raw_plots(&bytes, Default::default()).is_err());
        let mut bytes = ngspice_op_then_tran_binary();
        bytes.truncate(bytes.len() - 4);
        assert!(decode_spice_raw_plots(&bytes, Default::default()).is_err());
    }

    #[test]
    fn core_raw_refusals_retain_the_parser_and_resource_cause() {
        use std::error::Error as _;
        let bytes = binary_fixture("unknown", "x", false);
        let mut limits = rspice_core::ResourceLimits::default();
        limits.max_external_data_bytes = bytes.len() - 1;
        let error = decode_spice_raw(&bytes, limits).unwrap_err();
        assert!(matches!(
            &error,
            RawReadError::Parse(RawParseError::ResourceLimit(_))
        ));
        let source = error.source().unwrap();
        assert!(source.is::<RawParseError>());
        assert_eq!(error.to_string(), source.to_string());
        let error = decode_spice_raw(b"invalid", Default::default()).unwrap_err();
        assert!(matches!(&error, RawReadError::Parse(_)));
        assert!(error.source().unwrap().is::<RawParseError>());
    }
}
