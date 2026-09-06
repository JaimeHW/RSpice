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
//!
//! The round trip is exact, which is what makes a rawfile a lossless carrier
//! of an event history rather than a picture of one. A digital value is
//! written as the integer [`DigitalValue::event_code`] produces, and that
//! integer names the resolved state *and* the drive strength, so
//! [`decode_event_plots`] hands back the same twelve-state value the run
//! committed, at the same time, with no interpolation and no rounding. That
//! is the difference between this carrier and the two lossy ones: a `D(node)`
//! grid column keeps three levels, and a VCD dump keeps four bit states and
//! no strength at all.
//!
//! What a node plot does *not* carry is everything outside the timeline
//! itself: no unit, no width, and no record of which `.SAVE` or `.OPTIONS`
//! decision selected the node in the first place. A plot is one node's times
//! and one node's values.
//!
//! # Buses
//!
//! The one exception is the grouping. A third family,
//! `Digital Bus (rspice-digital-bus/1)`, gives each declared bus a plot whose
//! `Title:` is `name[msb:lsb]` and whose variables are the members in
//! declaration order — so the declaration a rawfile could not previously carry
//! rides in a header key that is free text to every reader, and the member
//! order rides in the variable list.
//!
//! **The member plots are written anyway, and they are the authoritative
//! copy.** A reader that has never heard of the bus family passes over it and
//! keeps every node it read before, and [`decode_event_plots`] takes member
//! values from the member plots, falling back to a bus column only when the
//! member plot is absent — which happens in a file somebody edited or a
//! foreign one, not in one this build wrote.

use thiserror::Error;

use crate::Value;
use crate::engine::{
    DigitalBusDeclaration, DigitalBusError, DigitalBusSource, DigitalTrace, DigitalTracePoint,
    RealTrace, RealTracePoint, canonical_event_name, validate_digital_bus_table,
};
use crate::execution::event_bus::{BusMemberHistory, bus_events, split_bus_notation};
use crate::io::raw_export::{RawBusTimeline, RawEventKind, RawEventTimeline};
use crate::io::{RawFile, RawWaveformData};
use crate::xspice::{DigitalState, DigitalStrength, DigitalValue};

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

    /// A bus plot's title does not spell a declaration.
    #[error("bus plot titled '{title}' does not spell a declaration as 'name[msb:lsb]'")]
    BusTitle {
        /// Title the plot declared.
        title: String,
    },

    /// A bus plot declares a range whose width is not its column count.
    #[error(
        "bus plot '{title}' declares {declared} member(s) by its range but carries {columns} member column(s)"
    )]
    BusWidth {
        /// Title the plot declared.
        title: String,
        /// Width the range describes.
        declared: u64,
        /// Number of member columns the plot carries.
        columns: usize,
    },

    /// A bus plot declares fewer variables than a time column and one member.
    #[error(
        "bus plot '{title}' declares {variables} variable(s); a bus plot declares a time column and at least one member"
    )]
    BusVariableCount {
        /// Title the plot declared.
        title: String,
        /// Variable count the plot declared.
        variables: usize,
    },

    /// The decoded bus table is not well formed.
    #[error(transparent)]
    Bus(#[from] DigitalBusError),
}

/// The event histories one rawfile's appended plots carry.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawEventTraces {
    /// Digital node histories, in the order their plots appear.
    pub digital_traces: Vec<DigitalTrace>,
    /// Buses declared by the bus plots, in the order those plots appear.
    pub digital_buses: Vec<DigitalBusDeclaration>,
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

/// Project a transient's declared buses onto rawfile bus plots.
///
/// One plot per bus, in declaration order, with the rows being every time at
/// least one member changed and the columns being what each member held at
/// those times — the reassembly [`crate::execution::bus_events`] performs, so
/// a bus plot and a VCD vector say the same thing about the same run.
///
/// A member that has no recorded point yet is written as the code for an
/// unknown value at undetermined strength, because that is what a node the run
/// has not stated yet holds. The ambiguity that introduces — was it observed as
/// unknown, or not observed at all — costs nothing in practice: the member
/// plots are written beside these and are what [`decode_event_plots`] reads,
/// and where a member plot is missing "unknown, undetermined" is the truthful
/// answer anyway.
///
/// A bus with no events at all is dropped rather than written, for the same
/// reason an empty node timeline is: a plot declaring zero points has no
/// boundary a reader can find.
pub fn transient_bus_plots(
    digital_traces: &[DigitalTrace],
    digital_buses: &[DigitalBusDeclaration],
) -> Vec<RawBusTimeline> {
    let mut plots = Vec::new();
    for bus in digital_buses {
        let histories: Vec<Vec<(Value, u8)>> = bus
            .members
            .iter()
            .map(|member| {
                let key = canonical_event_name(member);
                digital_traces
                    .iter()
                    .find(|trace| canonical_event_name(&trace.node_name) == key)
                    .map(|trace| {
                        trace
                            .points
                            .iter()
                            .map(|point| (point.time, point.value.event_code()))
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .collect();
        let members: Vec<BusMemberHistory<'_>> = histories
            .iter()
            .map(|points| BusMemberHistory { points })
            .collect();
        let events = bus_events(&members);
        if events.is_empty() {
            continue;
        }
        let mut times = Vec::with_capacity(events.len());
        let mut columns = vec![Vec::with_capacity(events.len()); bus.members.len()];
        for (time, codes) in events {
            times.push(time);
            for (column, code) in columns.iter_mut().zip(codes) {
                column.push(Value::from(code.unwrap_or(unobserved_event_code())));
            }
        }
        plots.push(RawBusTimeline {
            name: bus.name.clone(),
            msb: bus.msb,
            lsb: bus.lsb,
            members: bus.members.clone(),
            times,
            values: columns,
        });
    }
    plots
}

/// The code a member with no recorded point yet is written as: unknown, at
/// undetermined strength, which is what such a node holds.
fn unobserved_event_code() -> u8 {
    DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined).event_code()
}

/// Decode every event plot a rawfile carries back into event histories.
///
/// Plots whose name does not declare an event family — the analysis plot a
/// rawfile opens with, and anything another writer appended — are ordinary
/// plots and are passed over, never refused. A plot that does declare a
/// family and then contradicts it is a corrupt artifact and is refused.
///
/// # Buses
///
/// A bus plot contributes its declaration, and its member *values* only where
/// the member has no node plot of its own. The node plots are the
/// authoritative copy — they are exactly what the run recorded, change for
/// change — while a bus plot's columns are a row grid shared with the other
/// members and carry a synthesized code for a member that had not been
/// observed at a row's time. In a file this build wrote both are present and
/// agree; in an edited or foreign one the column is what is left, and taking
/// it is better than dropping the member.
pub fn decode_event_plots(file: &RawFile) -> Result<RawEventTraces, EventPlotError> {
    let mut traces = RawEventTraces::default();
    let mut bus_plots = Vec::new();
    for plot in &file.plots {
        let Some(kind) = RawEventKind::from_plot_name(&plot.header.plotname) else {
            continue;
        };
        match kind {
            RawEventKind::Digital => traces.digital_traces.push(decode_digital_plot(plot)?),
            RawEventKind::Real => traces.real_traces.push(decode_real_plot(plot)?),
            // Held back: whether a member needs its column depends on the node
            // plots, and a bus plot may precede them in the file.
            RawEventKind::Bus => bus_plots.push(decode_bus_plot(plot)?),
        }
    }

    for bus in bus_plots {
        for (index, member) in bus.members.iter().enumerate() {
            let key = canonical_event_name(member);
            if traces
                .digital_traces
                .iter()
                .any(|trace| canonical_event_name(&trace.node_name) == key)
            {
                continue;
            }
            let Some(column) = bus.values.get(index) else {
                continue;
            };
            traces
                .digital_traces
                .push(decode_bus_column(member, &bus.times, column)?);
        }
        traces.digital_buses.push(DigitalBusDeclaration::new(
            bus.name,
            bus.msb,
            bus.lsb,
            bus.members,
            DigitalBusSource::Import,
        )?);
    }

    validate_digital_bus_table(
        &traces.digital_buses,
        traces
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str()),
    )?;
    Ok(traces)
}

/// Read one bus plot's declaration and columns.
fn decode_bus_plot(plot: &RawWaveformData) -> Result<RawBusTimeline, EventPlotError> {
    let title = plot.header.title.trim().to_owned();
    let (name, Some((msb, lsb))) = split_bus_notation(&title) else {
        return Err(EventPlotError::BusTitle { title });
    };
    let name = name.to_owned();
    if plot.variables.len() < 2 {
        return Err(EventPlotError::BusVariableCount {
            title,
            variables: plot.variables.len(),
        });
    }
    let expected = RawEventKind::Bus.variable_type().as_str();
    let mut members = Vec::with_capacity(plot.variables.len().saturating_sub(1));
    for variable in plot.variables.iter().skip(1) {
        let Some(member) = RawEventKind::Bus.node_name(&variable.name) else {
            return Err(EventPlotError::VariableName {
                plot: plot.header.plotname.clone(),
                variable: variable.name.clone(),
            });
        };
        if !variable.var_type.eq_ignore_ascii_case(expected) {
            return Err(EventPlotError::VariableType {
                plot: plot.header.plotname.clone(),
                variable: variable.name.clone(),
                var_type: variable.var_type.clone(),
                expected,
            });
        }
        members.push(member.to_owned());
    }
    let declared = msb.abs_diff(lsb).saturating_add(1);
    if declared != members.len() as u64 {
        return Err(EventPlotError::BusWidth {
            title,
            declared,
            columns: members.len(),
        });
    }
    let times = plot
        .waveforms
        .first()
        .map(|waveform| waveform.y.clone())
        .unwrap_or_default();
    let values = plot
        .waveforms
        .iter()
        .skip(1)
        .map(|waveform| waveform.y.clone())
        .collect();
    Ok(RawBusTimeline {
        name,
        msb,
        lsb,
        members,
        times,
        values,
    })
}

/// Rebuild one member's history from a bus column, keeping changes only.
///
/// A bus row grid holds every member's value at every bus event, so the column
/// repeats a value the member did not change at. A recorded history keeps
/// changes only, which is the invariant every other producer of one holds to,
/// so the repeats are dropped on the way in.
fn decode_bus_column(
    member: &str,
    times: &[Value],
    column: &[Value],
) -> Result<DigitalTrace, EventPlotError> {
    let mut points: Vec<DigitalTracePoint> = Vec::new();
    for (&time, &value) in times.iter().zip(column) {
        let code = event_code(value).ok_or_else(|| EventPlotError::DigitalCode {
            node: member.to_owned(),
            time,
            value,
        })?;
        if points.last().is_some_and(|last| last.value == code) {
            continue;
        }
        points.push(DigitalTracePoint { time, value: code });
    }
    Ok(DigitalTrace {
        node_name: member.to_owned(),
        points,
    })
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
        write_event_plots(&mut bytes, &timelines, &[], format).expect("write event plots");
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
            &[],
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

#[cfg(test)]
mod bus_tests {
    use super::*;
    use crate::io::{RawFormat, parse_raw_plots_reader_with_limits, write_event_plots};
    use crate::resource::ResourceLimits;
    use std::io::Cursor;

    fn trace(node: &str, points: &[(Value, DigitalValue)]) -> DigitalTrace {
        DigitalTrace {
            node_name: node.to_string(),
            points: points
                .iter()
                .map(|(time, value)| DigitalTracePoint {
                    time: *time,
                    value: *value,
                })
                .collect(),
        }
    }

    fn bus(name: &str, msb: i64, lsb: i64, members: &[&str]) -> DigitalBusDeclaration {
        DigitalBusDeclaration::new(
            name,
            msb,
            lsb,
            members.iter().map(|member| (*member).to_string()).collect(),
            DigitalBusSource::Schematic,
        )
        .expect("the fixture bus is well formed")
    }

    fn two_bit_fixture() -> (Vec<DigitalTrace>, Vec<DigitalBusDeclaration>) {
        let traces = vec![
            trace(
                "d1",
                &[(0.0, DigitalValue::zero()), (2.0e-9, DigitalValue::one())],
            ),
            trace(
                "d0",
                &[(0.0, DigitalValue::one()), (1.0e-9, DigitalValue::high_z())],
            ),
        ];
        (traces, vec![bus("d", 1, 0, &["d1", "d0"])])
    }

    fn written(format: RawFormat) -> Vec<u8> {
        let (traces, buses) = two_bit_fixture();
        let mut bytes = Vec::new();
        write_event_plots(
            &mut bytes,
            &transient_event_plots(&traces, &[]),
            &transient_bus_plots(&traces, &buses),
            format,
        )
        .expect("write event and bus plots");
        bytes
    }

    #[test]
    fn a_bus_plot_declares_itself_in_its_title_and_its_members_in_its_variables() {
        let text = String::from_utf8(written(RawFormat::Ascii)).expect("ASCII rawfile");
        assert!(
            text.contains("Title: d[1:0]"),
            "the declaration rides in a header key every reader treats as free text: {text}"
        );
        assert!(
            text.contains("Plotname: Digital Bus (rspice-digital-bus/1)"),
            "{text}"
        );
        assert!(text.contains("\t1\tD(d1)\tdigital"), "{text}");
        assert!(text.contains("\t2\tD(d0)\tdigital"), "{text}");
        assert!(
            text.contains("Plotname: Digital Events (rspice-digital-events/1)"),
            "the member plots are written too: {text}"
        );
    }

    #[test]
    fn a_bus_round_trips_in_both_encodings_and_the_member_plots_stay_authoritative() {
        let (traces, buses) = two_bit_fixture();
        for format in [RawFormat::Binary, RawFormat::Ascii] {
            let file = parse_raw_plots_reader_with_limits(
                &mut Cursor::new(written(format)),
                ResourceLimits::default(),
            )
            .expect("parse");
            let decoded = decode_event_plots(&file).expect("decode");
            let expected: Vec<DigitalBusDeclaration> = buses
                .iter()
                .map(|bus| DigitalBusDeclaration {
                    // A rawfile is a claim by whatever wrote it, so a
                    // declaration read out of one is an import however it was
                    // declared before it was written.
                    source: DigitalBusSource::Import,
                    ..bus.clone()
                })
                .collect();
            assert_eq!(decoded.digital_buses, expected, "{format:?}");
            assert_eq!(
                decoded.digital_traces, traces,
                "the member plots come back exactly, change for change: {format:?}"
            );
        }
    }

    #[test]
    fn a_reader_that_ignores_the_bus_family_still_sees_every_member() {
        // The compatibility claim the family rests on: adding the grouping
        // must cost a reader that has never heard of it nothing.
        let (traces, _) = two_bit_fixture();
        let file = parse_raw_plots_reader_with_limits(
            &mut Cursor::new(written(RawFormat::Binary)),
            ResourceLimits::default(),
        )
        .expect("parse");
        let member_plots = file
            .plots
            .iter()
            .filter(|plot| {
                RawEventKind::from_plot_name(&plot.header.plotname) == Some(RawEventKind::Digital)
            })
            .count();
        assert_eq!(member_plots, traces.len());
    }

    #[test]
    fn a_member_whose_plot_is_missing_falls_back_to_the_bus_column() {
        // A file somebody edited, or a foreign one: the bus plot is all that
        // is left of one member, so its column is what the member becomes.
        let (traces, buses) = two_bit_fixture();
        let mut bytes = Vec::new();
        write_event_plots(
            &mut bytes,
            &transient_event_plots(&traces[..1], &[]),
            &transient_bus_plots(&traces, &buses),
            RawFormat::Ascii,
        )
        .expect("write a bus plot with only one member plot beside it");
        let trimmed =
            parse_raw_plots_reader_with_limits(&mut Cursor::new(bytes), ResourceLimits::default())
                .expect("parse");
        let decoded = decode_event_plots(&trimmed).expect("decode");

        let recovered = decoded
            .digital_traces
            .iter()
            .find(|trace| trace.node_name == "d0")
            .expect("the missing member is recovered from the bus column");
        assert_eq!(
            recovered.points,
            vec![
                DigitalTracePoint {
                    time: 0.0,
                    value: DigitalValue::one(),
                },
                DigitalTracePoint {
                    time: 1.0e-9,
                    value: DigitalValue::high_z(),
                },
            ],
            "the bus row grid repeats a held value; a recovered history keeps changes only"
        );
        assert_eq!(decoded.digital_buses.len(), 1);
    }

    #[test]
    fn a_member_not_yet_observed_is_written_as_unknown_at_undetermined_strength() {
        let traces = vec![
            trace("d1", &[(0.0, DigitalValue::one())]),
            trace("d0", &[(3.0e-9, DigitalValue::zero())]),
        ];
        let plots = transient_bus_plots(&traces, &[bus("d", 1, 0, &["d1", "d0"])]);
        let plot = plots.first().expect("one bus plot");
        assert_eq!(plot.times, vec![0.0, 3.0e-9]);
        assert_eq!(
            plot.values[1].first().copied(),
            Some(Value::from(
                DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined)
                    .event_code()
            )),
            "a node the run has not stated yet holds unknown at undetermined strength"
        );
    }

    #[test]
    fn a_bus_with_no_events_is_dropped_rather_than_written_without_a_boundary() {
        let traces = vec![trace("d1", &[]), trace("d0", &[])];
        assert!(transient_bus_plots(&traces, &[bus("d", 1, 0, &["d1", "d0"])]).is_empty());
    }

    #[test]
    fn a_bus_plot_whose_title_does_not_declare_a_range_is_refused() {
        let source = "Title: not a declaration\nPlotname: Digital Bus (rspice-digital-bus/1)\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n\t0\ttime\ttime\n\t1\tD(d0)\tdigital\nValues:\n0\t0.0\t1.0\n";
        let file = parse_raw_plots_reader_with_limits(
            &mut Cursor::new(source.as_bytes()),
            ResourceLimits::default(),
        )
        .expect("parse");
        assert_eq!(
            decode_event_plots(&file),
            Err(EventPlotError::BusTitle {
                title: "not a declaration".to_string(),
            })
        );
    }

    #[test]
    fn a_bus_plot_whose_range_and_columns_disagree_is_refused() {
        let source = "Title: d[7:0]\nPlotname: Digital Bus (rspice-digital-bus/1)\nFlags: real\nNo. Variables: 2\nNo. Points: 1\nVariables:\n\t0\ttime\ttime\n\t1\tD(d0)\tdigital\nValues:\n0\t0.0\t1.0\n";
        let file = parse_raw_plots_reader_with_limits(
            &mut Cursor::new(source.as_bytes()),
            ResourceLimits::default(),
        )
        .expect("parse");
        assert_eq!(
            decode_event_plots(&file),
            Err(EventPlotError::BusWidth {
                title: "d[7:0]".to_string(),
                declared: 8,
                columns: 1,
            })
        );
    }

    #[test]
    fn a_bus_written_from_a_node_timeline_is_refused_by_the_writer() {
        let mut bytes = Vec::new();
        let error = write_event_plots(
            &mut bytes,
            &[RawEventTimeline {
                kind: RawEventKind::Bus,
                node_name: "d".to_string(),
                times: vec![0.0],
                values: vec![0.0],
            }],
            &[],
            RawFormat::Ascii,
        )
        .expect_err("a bus plot has N columns and a node timeline has one");
        assert!(error.to_string().contains("RawBusTimeline"), "{error}");
    }
}
