//! Event histories as rawfile plots, and back.
//!
//! A transient carries two event families that have times of their own:
//! XSPICE digital nodes and XSPICE real-valued nodes. The tabular projection
//! beside this module answers what a node held at each analysis time point,
//! which is what a row grid can say; this module answers when the node
//! changed and to what, which is what the history actually recorded.
//!
//! The rawfile carrier is a plot per node, written by
//! [`crate::io::write_event_plots`] and read back by the multi-plot reader.
//! The format modules stay free of result types — they take names, times and
//! values — so the mapping between an engine result and those plain columns
//! lives here, in both directions.

use thiserror::Error;

use crate::Value;
use crate::engine::{DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint};
use crate::io::raw_export::{RawEventKind, RawEventTimeline};
use crate::io::{RawFile, RawWaveformData};
use crate::xspice::DigitalValue;

/// Why one rawfile event plot could not be decoded.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum EventPlotError {
    /// The plot does not declare exactly a time column and one node.
    #[error(
        "'{plot}' declares {variables} variable(s); an event plot declares a time column and one node"
    )]
    VariableCount {
        /// Plot name that named the event family.
        plot: String,
        /// Variable count the plot declared.
        variables: usize,
    },

    /// The value column is not spelled as a node of the plot's family.
    #[error(
        "'{plot}' names its value column '{variable}', which does not spell a node of that family"
    )]
    VariableName {
        /// Plot name that named the event family.
        plot: String,
        /// Value-column name the plot declared.
        variable: String,
    },

    /// The value column is not typed as the plot's family types it.
    #[error("'{plot}' types its value column '{variable}' as '{var_type}', not '{expected}'")]
    VariableType {
        /// Plot name that named the event family.
        plot: String,
        /// Value-column name the plot declared.
        variable: String,
        /// Variable type the plot declared.
        var_type: String,
        /// Variable type the family declares.
        expected: &'static str,
    },

    /// A digital plot carries a value that is not an XSPICE event code.
    #[error(
        "digital event plot for '{node}' carries {value} at t={time}, which is not one of the thirteen XSPICE event codes"
    )]
    DigitalCode {
        /// Node the plot carried.
        node: String,
        /// Time the offending value sits at.
        time: Value,
        /// Value that is not an event code.
        value: Value,
    },
}

/// The event histories one rawfile's appended plots carry.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawEventTraces {
    /// Digital node histories, in the order their plots appear.
    pub digital_traces: Vec<DigitalTrace>,
    /// Real-valued node histories, in the order their plots appear.
    pub real_traces: Vec<RealTrace>,
}

/// Project a transient's captured event histories onto rawfile event plots.
///
/// Digital nodes come first in result order, then real nodes, which is the
/// order the plots are appended in. Times are the accepted event times exactly
/// as recorded; a digital value is written as the event code
/// [`DigitalValue::event_code`] produces, which names its resolved state and
/// drive strength and decodes back to both.
///
/// A trace with no events is dropped rather than written: it carries nothing,
/// and a plot declaring zero points has no boundary a reader can find.
pub fn transient_event_plots(
    digital_traces: &[DigitalTrace],
    real_traces: &[RealTrace],
) -> Vec<RawEventTimeline> {
    let digital = digital_traces.iter().map(|trace| RawEventTimeline {
        kind: RawEventKind::Digital,
        node_name: trace.node_name.clone(),
        times: trace.points.iter().map(|point| point.time).collect(),
        values: trace
            .points
            .iter()
            .map(|point| Value::from(point.value.event_code()))
            .collect(),
    });
    let real = real_traces.iter().map(|trace| RawEventTimeline {
        kind: RawEventKind::Real,
        node_name: trace.node_name.clone(),
        times: trace.points.iter().map(|point| point.time).collect(),
        values: trace.points.iter().map(|point| point.value).collect(),
    });
    digital
        .chain(real)
        .filter(|timeline| !timeline.times.is_empty())
        .collect()
}

/// Decode every event plot a rawfile carries back into event histories.
///
/// Plots whose name does not declare an event family — the analysis plot a
/// rawfile opens with, and anything another writer appended — are ordinary
/// plots and are passed over, never refused. A plot that does declare a
/// family and then contradicts it is a corrupt artifact and is refused.
pub fn decode_event_plots(file: &RawFile) -> Result<RawEventTraces, EventPlotError> {
    let mut traces = RawEventTraces::default();
    for plot in &file.plots {
        let Some(kind) = RawEventKind::from_plot_name(&plot.header.plotname) else {
            continue;
        };
        match kind {
            RawEventKind::Digital => traces.digital_traces.push(decode_digital_plot(plot)?),
            RawEventKind::Real => traces.real_traces.push(decode_real_plot(plot)?),
        }
    }
    Ok(traces)
}

/// The node name and the two columns one event plot declares.
///
/// The reader delivers every column of a plot at that plot's point count, so
/// the two columns are aligned by the time they arrive here.
fn event_plot_columns(
    plot: &RawWaveformData,
    kind: RawEventKind,
) -> Result<(String, &[Value], &[Value]), EventPlotError> {
    let plot_name = plot.header.plotname.clone();
    let (Some(value_variable), Some(times), Some(values), 2) = (
        plot.variables.get(1),
        plot.waveforms.first(),
        plot.waveforms.get(1),
        plot.variables.len(),
    ) else {
        return Err(EventPlotError::VariableCount {
            plot: plot_name,
            variables: plot.variables.len(),
        });
    };
    let Some(node) = kind.node_name(&value_variable.name) else {
        return Err(EventPlotError::VariableName {
            plot: plot_name,
            variable: value_variable.name.clone(),
        });
    };
    let expected = kind.variable_type().as_str();
    if !value_variable.var_type.eq_ignore_ascii_case(expected) {
        return Err(EventPlotError::VariableType {
            plot: plot_name,
            variable: value_variable.name.clone(),
            var_type: value_variable.var_type.clone(),
            expected,
        });
    }
    Ok((node.to_string(), &times.y, &values.y))
}

fn decode_digital_plot(plot: &RawWaveformData) -> Result<DigitalTrace, EventPlotError> {
    let (node_name, times, values) = event_plot_columns(plot, RawEventKind::Digital)?;
    let mut points = Vec::with_capacity(times.len());
    for (&time, &value) in times.iter().zip(values) {
        let code = event_code(value).ok_or_else(|| EventPlotError::DigitalCode {
            node: node_name.clone(),
            time,
            value,
        })?;
        points.push(DigitalTracePoint { time, value: code });
    }
    Ok(DigitalTrace { node_name, points })
}

fn decode_real_plot(plot: &RawWaveformData) -> Result<RealTrace, EventPlotError> {
    let (node_name, times, values) = event_plot_columns(plot, RawEventKind::Real)?;
    let points = times
        .iter()
        .zip(values)
        .map(|(&time, &value)| RealTracePoint { time, value })
        .collect();
    Ok(RealTrace { node_name, points })
}

/// The digital value one stored event code names, if it names one.
///
/// The rawfile stores codes as the doubles every other column stores, so the
/// value has to be a whole number in the code range before it means anything.
fn event_code(value: Value) -> Option<DigitalValue> {
    if !value.is_finite() || value.fract() != 0.0 || !(0.0..=255.0).contains(&value) {
        return None;
    }
    DigitalValue::from_event_code(value as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::{RawFormat, parse_raw_plots_reader_with_limits, write_event_plots};
    use crate::resource::ResourceLimits;
    use crate::xspice::{DigitalState, DigitalStrength};
    use std::io::Cursor;

    fn digital_trace() -> DigitalTrace {
        DigitalTrace {
            node_name: "clk".to_string(),
            points: vec![
                DigitalTracePoint {
                    time: 0.0,
                    value: DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
                },
                DigitalTracePoint {
                    time: 1.5e-9,
                    value: DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
                },
                DigitalTracePoint {
                    time: 3.25e-9,
                    value: DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive),
                },
                DigitalTracePoint {
                    time: 7.0e-9,
                    value: DigitalValue::high_z(),
                },
            ],
        }
    }

    fn real_trace() -> RealTrace {
        RealTrace {
            node_name: "vctrl".to_string(),
            points: vec![
                RealTracePoint {
                    time: 0.0,
                    value: -1.234_567_890_123_456_7e-3,
                },
                RealTracePoint {
                    time: 2.5e-9,
                    value: 9.876_543_210_987_654e6,
                },
            ],
        }
    }

    fn round_trip(format: RawFormat) -> RawEventTraces {
        let timelines = transient_event_plots(&[digital_trace()], &[real_trace()]);
        let mut bytes = Vec::new();
        write_event_plots(&mut bytes, &timelines, format).expect("write event plots");
        let file =
            parse_raw_plots_reader_with_limits(&mut Cursor::new(bytes), ResourceLimits::default())
                .expect("parse event plots");
        decode_event_plots(&file).expect("decode event plots")
    }

    #[test]
    fn event_plots_round_trip_every_state_and_value_in_both_encodings() {
        for format in [RawFormat::Binary, RawFormat::Ascii] {
            let decoded = round_trip(format);
            assert_eq!(decoded.digital_traces, vec![digital_trace()], "{format:?}");
            assert_eq!(decoded.real_traces, vec![real_trace()], "{format:?}");
        }
    }

    #[test]
    fn empty_traces_are_dropped_rather_than_written_as_boundaryless_plots() {
        let timelines = transient_event_plots(
            &[DigitalTrace {
                node_name: "unused".to_string(),
                points: Vec::new(),
            }],
            &[RealTrace {
                node_name: "quiet".to_string(),
                points: Vec::new(),
            }],
        );
        assert!(timelines.is_empty());
    }

    #[test]
    fn ordinary_plots_are_passed_over_rather_than_refused() {
        let mut bytes = b"Title: t\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n\t0\ttime\ttime\n\t1\tV(out)\tvoltage\nValues:\n0\t0.0\t1.0\n".to_vec();
        write_event_plots(
            &mut bytes,
            &transient_event_plots(&[digital_trace()], &[]),
            RawFormat::Ascii,
        )
        .expect("append event plot");

        let file =
            parse_raw_plots_reader_with_limits(&mut Cursor::new(bytes), ResourceLimits::default())
                .expect("parse mixed file");
        assert_eq!(file.plots.len(), 2);
        let decoded = decode_event_plots(&file).expect("decode event plots");
        assert_eq!(decoded.digital_traces, vec![digital_trace()]);
        assert!(decoded.real_traces.is_empty());
    }

    #[test]
    fn a_digital_plot_carrying_a_value_that_is_not_a_code_is_refused() {
        let source = "Plotname: Digital Events (rspice-digital-events/1)\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n\t0\ttime\ttime\n\t1\tD(clk)\tdigital\nValues:\n0\t0.0\t13.0\n";
        let file = parse_raw_plots_reader_with_limits(
            &mut Cursor::new(source.as_bytes()),
            ResourceLimits::default(),
        )
        .expect("parse digital event plot");

        assert_eq!(
            decode_event_plots(&file),
            Err(EventPlotError::DigitalCode {
                node: "clk".to_string(),
                time: 0.0,
                value: 13.0,
            })
        );
    }

    #[test]
    fn a_plot_that_declares_a_family_and_contradicts_it_is_refused() {
        let source = "Plotname: Real Events (rspice-real-events/1)\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n\t0\ttime\ttime\n\t1\tV(out)\tvoltage\nValues:\n0\t0.0\t1.0\n";
        let file = parse_raw_plots_reader_with_limits(
            &mut Cursor::new(source.as_bytes()),
            ResourceLimits::default(),
        )
        .expect("parse mis-declared event plot");

        assert_eq!(
            decode_event_plots(&file),
            Err(EventPlotError::VariableName {
                plot: "Real Events (rspice-real-events/1)".to_string(),
                variable: "V(out)".to_string(),
            })
        );
    }

    #[test]
    fn a_family_plot_typing_its_value_column_as_something_else_is_refused() {
        let source = "Plotname: Real Events (rspice-real-events/1)\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n\t0\ttime\ttime\n\t1\tE(vctrl)\tvoltage\nValues:\n0\t0.0\t1.0\n";
        let file = parse_raw_plots_reader_with_limits(
            &mut Cursor::new(source.as_bytes()),
            ResourceLimits::default(),
        )
        .expect("parse mistyped event plot");

        assert_eq!(
            decode_event_plots(&file),
            Err(EventPlotError::VariableType {
                plot: "Real Events (rspice-real-events/1)".to_string(),
                variable: "E(vctrl)".to_string(),
                var_type: "voltage".to_string(),
                expected: "real",
            })
        );
    }

    #[test]
    fn a_family_plot_carrying_more_than_one_node_is_refused() {
        let source = "Plotname: Digital Events (rspice-digital-events/1)\nFlags: real\nNo. Variables: 3\nNo. Points: 1\nVariables:\n\t0\ttime\ttime\n\t1\tD(a)\tdigital\n\t2\tD(b)\tdigital\nValues:\n0\t0.0\t1.0\t0.0\n";
        let file = parse_raw_plots_reader_with_limits(
            &mut Cursor::new(source.as_bytes()),
            ResourceLimits::default(),
        )
        .expect("parse wide event plot");

        assert_eq!(
            decode_event_plots(&file),
            Err(EventPlotError::VariableCount {
                plot: "Digital Events (rspice-digital-events/1)".to_string(),
                variables: 3,
            })
        );
    }
}
