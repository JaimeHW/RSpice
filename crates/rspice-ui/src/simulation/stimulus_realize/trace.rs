//! Whether a preview draws a curve or an envelope, and what it draws from.
//!
//! A uniform grid over the window is a false statement about any waveform whose
//! period is shorter than a few of that grid's columns. Four hundred and eighty
//! samples of a `PULSE(5 1 0 1n 1n 1u 2u)` over a 1 ms transient — five hundred
//! cycles — paint about twenty fat ones, at a period the card does not have,
//! with edges nobody authored and a minimum, maximum and midpoint read off
//! whichever twentieth of the cycles the grid happened to land on. The same
//! grid draws a 1 ns edge in a 100 µs window as a ramp across a column, or
//! misses the pulse between two samples entirely. A proof surface may not draw
//! either.
//!
//! So a realization is one of two things and says which:
//!
//! - a CURVE — the uniform grid with the waveform's own breakpoints merged into
//!   it, so every corner the engine schedules is a vertex and an edge is
//!   vertical;
//! - an ENVELOPE — one (minimum, maximum) pair per column, each *measured* over
//!   the time that column covers rather than sampled at a point in it.
//!
//! The envelope is not a decimation of the curve, and that is the whole reason
//! it exists. Each column is measured over its own interval, except that a
//! waveform repeating faster than the column is measured over one of its
//! periods instead — which for a strictly periodic shape is the same set of
//! values, and bounds the work whether the window holds five hundred cycles or
//! five million. `AM` and `SFFM` are measured over one carrier period, so the
//! band follows the modulation rather than collapsing into a beat nobody
//! authored.
//!
//! Both modes answer the same three questions — what does it cover, what does
//! it read at a time, what does it call itself — so no painter has to know
//! which one it was handed in order to state it correctly.

use rspice_core::netlist::SourceSpec;

use super::derived::{breakpoints_between, resolution_period};
use super::{PreviewTiming, PreviewWindow, WaveformReadouts, evaluate_at, transient_part};

/// How few columns of a window a period may cover before the window is drawn as
/// an envelope.
///
/// Four columns per cycle is already past the point where a polyline states the
/// shape: the corners land wherever the grid's own phase puts them, and the
/// picture reports a period that is the beat between the waveform and the
/// sampling rather than the waveform's own.
const ENVELOPE_COLUMNS_PER_CYCLE: f64 = 4.0;

/// How many breakpoints a curve will merge before the window is an envelope by
/// construction. A window carrying more corners than this has more corners than
/// the plot has pixels, and a polyline through a decimated subset of them is a
/// shape the source does not have.
const MAX_MERGED_BREAKPOINTS: usize = 2_000;

/// How many of a column's own breakpoints its extremes are measured at. The
/// families that repeat fast enough to reach envelope mode place at most a
/// handful of corners in one column; the ceiling is what keeps a pathological
/// table from turning one column into the whole window's work.
const COLUMN_BREAKPOINTS: usize = 64;

/// How finely a column is swept between its breakpoints. A sine measured this
/// way reports a peak within a tenth of a percent of the true one, and the
/// piecewise-linear families take their extremes at the corners regardless.
const COLUMN_SWEEP: usize = 24;

/// One column of an envelope: what the waveform covers over the time the column
/// spans.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EnvelopeColumn {
    /// The column's own time, at its centre, which is where it is drawn.
    pub time: f64,
    /// The lowest value the waveform takes over the column.
    pub minimum: f64,
    /// The highest value the waveform takes over the column.
    pub maximum: f64,
}

/// What a preview has to draw.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum WaveformTrace {
    /// One value per time, in increasing time order: the engine's samples on
    /// the uniform grid with every breakpoint in the window merged in.
    Curve(Vec<(f64, f64)>),
    /// One band per column, and how many cycles of the waveform's own period
    /// the window holds where it has one.
    Envelope {
        /// The bands, in increasing time order.
        columns: Vec<EnvelopeColumn>,
        /// Cycles in the window, or `None` for a window that is an envelope
        /// because of how many corners it carries rather than how fast it
        /// repeats.
        cycles: Option<f64>,
    },
}

/// What a trace reads at one time, which is what a hover readout states.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TraceReading {
    /// A value the engine produced at a time it produced it at.
    Sample {
        /// When.
        time: f64,
        /// What.
        value: f64,
    },
    /// The range a column covers. A single value here would be a number the
    /// run never produced, picked out of a range the reader cannot see.
    Band {
        /// The column's time.
        time: f64,
        /// Its lowest value.
        minimum: f64,
        /// Its highest value.
        maximum: f64,
    },
}

impl WaveformTrace {
    /// Whether there is nothing to draw.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Curve(samples) => samples.len() < 2,
            Self::Envelope { columns, .. } => columns.len() < 2,
        }
    }

    /// What the trace covers vertically.
    ///
    /// In envelope mode these are the extremes of the measured columns, not of
    /// a grid: the minimum and maximum a reader sees on the axis are the ones
    /// the waveform actually reaches inside the window, which is exactly what
    /// an undersampled grid cannot promise.
    #[must_use]
    pub fn readouts(&self) -> Option<WaveformReadouts> {
        match self {
            Self::Curve(samples) => WaveformReadouts::of(samples),
            Self::Envelope { columns, .. } => {
                if columns.len() < 2 {
                    return None;
                }
                let (minimum, maximum) = columns
                    .iter()
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), column| {
                        (low.min(column.minimum), high.max(column.maximum))
                    });
                WaveformReadouts::spanning(minimum, maximum)
            }
        }
    }

    /// What the trace calls itself, where that is not simply a curve.
    ///
    /// Stated here rather than in either painter because both of them state it,
    /// and a band captioned one way on the instrument and another way in the
    /// properties dialog would be two answers to what the reader is looking at.
    #[must_use]
    pub fn caption(&self) -> Option<String> {
        match self {
            Self::Curve(_) => None,
            Self::Envelope { cycles, .. } => Some(match cycles {
                Some(cycles) => format!("{cycles:.0} cycles \u{b7} envelope"),
                None => "envelope".to_owned(),
            }),
        }
    }

    /// What this trace reads at a time, or `None` when it has nothing to read.
    #[must_use]
    pub fn reading_at(&self, time: f64) -> Option<TraceReading> {
        match self {
            // Nearest rather than interpolated: the curve is the engine's own
            // samples, and a value between two of them is a number the run
            // never produced.
            Self::Curve(samples) => samples
                .iter()
                .min_by(|left, right| (left.0 - time).abs().total_cmp(&(right.0 - time).abs()))
                .map(|(time, value)| TraceReading::Sample {
                    time: *time,
                    value: *value,
                }),
            Self::Envelope { columns, .. } => columns
                .iter()
                .min_by(|left, right| {
                    (left.time - time)
                        .abs()
                        .total_cmp(&(right.time - time).abs())
                })
                .map(|column| TraceReading::Band {
                    time: column.time,
                    minimum: column.minimum,
                    maximum: column.maximum,
                }),
        }
    }
}

/// This spec's waveform over this window, as finely as the window can carry it.
///
/// `window.samples` is the number of columns the plot can resolve, and it is
/// what decides the mode: a window holding more cycles than a quarter of that,
/// or more breakpoints than a polyline can merge, is measured rather than
/// sampled.
pub(crate) fn sample_trace(
    spec: &SourceSpec,
    window: PreviewWindow,
    timing: PreviewTiming,
) -> WaveformTrace {
    let span = window.stop - window.start;
    if window.samples < 2 || !window.start.is_finite() || !span.is_finite() || span <= 0.0 {
        return WaveformTrace::Curve(Vec::new());
    }
    let shape = transient_part(spec);
    let pitch = span / (window.samples - 1) as f64;
    let period = resolution_period(shape, timing);
    if period.is_some_and(|period| period < ENVELOPE_COLUMNS_PER_CYCLE * pitch) {
        return envelope(spec, shape, window, timing, period, pitch);
    }
    let merged = breakpoints_between(
        shape,
        timing,
        window.start,
        window.stop,
        MAX_MERGED_BREAKPOINTS,
    );
    if merged.saturated {
        return envelope(spec, shape, window, timing, period, pitch);
    }
    curve(spec, window, timing, &merged.times)
}

/// The uniform grid with the window's own breakpoints merged into it.
fn curve(
    spec: &SourceSpec,
    window: PreviewWindow,
    timing: PreviewTiming,
    breakpoints: &[f64],
) -> WaveformTrace {
    let span = window.stop - window.start;
    let mut times = Vec::with_capacity(window.samples + breakpoints.len());
    for index in 0..window.samples {
        times.push(window.start + span * index as f64 / (window.samples - 1) as f64);
    }
    times.extend(
        breakpoints
            .iter()
            .copied()
            .filter(|time| *time > window.start && *time < window.stop),
    );
    times.sort_by(f64::total_cmp);
    times.dedup();
    WaveformTrace::Curve(
        times
            .into_iter()
            .map(|time| (time, evaluate_at(spec, time, timing)))
            .collect(),
    )
}

/// One measured band per column.
///
/// `period` shortens the interval a column is measured over, and it is the only
/// thing that keeps the work bounded: a 1 ns period in a 1 ms window would
/// otherwise be a million periods to sweep. Shortening is exact for a strictly
/// periodic waveform, because the values it takes over one period are the ones
/// it takes over any longer stretch; where it is not — the lead-in before a
/// delay, the last period of a bounded `PULSE` train, the modulation of an `AM`
/// carrier — the column reports the period starting at its own time, so a band
/// can lead a transition by at most one period, and never understates one.
fn envelope(
    spec: &SourceSpec,
    shape: &SourceSpec,
    window: PreviewWindow,
    timing: PreviewTiming,
    period: Option<f64>,
    pitch: f64,
) -> WaveformTrace {
    let half = pitch * 0.5;
    let columns = (0..window.samples)
        .map(|index| {
            let time = window.start + pitch * index as f64;
            let from = (time - half).max(window.start);
            let to = (time + half).min(window.stop);
            let to = match period {
                Some(period) if period < to - from => from + period,
                _ => to,
            };
            let (minimum, maximum) = extremes(spec, shape, timing, from, to);
            EnvelopeColumn {
                time,
                minimum,
                maximum,
            }
        })
        .collect();
    WaveformTrace::Envelope {
        columns,
        // A window measured this way because of how many corners it carries
        // may hold less than one cycle of anything, and "0 cycles" would be a
        // count of something that is not there.
        cycles: period
            .map(|period| (window.stop - window.start) / period)
            .filter(|cycles| *cycles >= 1.0),
    }
}

/// The lowest and highest value this waveform takes over one interval.
///
/// Swept uniformly and at its own corners both: the sweep catches the peak of a
/// smooth family, and the corners are where every piecewise-linear family takes
/// its extremes exactly.
fn extremes(
    spec: &SourceSpec,
    shape: &SourceSpec,
    timing: PreviewTiming,
    from: f64,
    to: f64,
) -> (f64, f64) {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut read = |time: f64| {
        let value = evaluate_at(spec, time, timing);
        minimum = minimum.min(value);
        maximum = maximum.max(value);
    };
    if to > from {
        for index in 0..=COLUMN_SWEEP {
            read(from + (to - from) * index as f64 / COLUMN_SWEEP as f64);
        }
        for time in breakpoints_between(shape, timing, from, to, COLUMN_BREAKPOINTS).times {
            read(time);
        }
    } else {
        read(from);
    }
    (minimum, maximum)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A window of `stop` seconds at the density the instrument asks for.
    fn window(stop: f64) -> PreviewWindow {
        PreviewWindow {
            start: 0.0,
            stop,
            samples: 480,
        }
    }

    fn timing(tstop: f64) -> PreviewTiming {
        PreviewTiming {
            tstep: tstop / 1000.0,
            tstop,
            from_analysis: true,
        }
    }

    fn spec(text: &str) -> SourceSpec {
        rspice_core::netlist::parse_source_spec_text(
            text,
            0,
            &rspice_core::netlist::ParamContext::new(),
        )
        .expect("a card the engine's own parser reads")
    }

    fn columns_of(trace: &WaveformTrace) -> &[EnvelopeColumn] {
        match trace {
            WaveformTrace::Envelope { columns, .. } => columns,
            WaveformTrace::Curve(_) => panic!("this window is a curve, not an envelope"),
        }
    }

    /// Five hundred cycles in four hundred and eighty columns is the defect
    /// this module exists for: the grid paints about twenty of them, at a
    /// period the card does not have. Measured instead, the band states the
    /// levels the source actually holds, and the readouts beside it are the
    /// ones a single period gives.
    #[test]
    fn a_train_with_more_cycles_than_columns_is_measured_rather_than_sampled() {
        let pulse = spec("PULSE(5 1 0 1n 1n 1u 2u)");
        let trace = sample_trace(&pulse, window(1e-3), timing(1e-3));

        let WaveformTrace::Envelope { columns, cycles } = &trace else {
            panic!("a five-hundred-cycle window is an envelope");
        };
        assert!(
            cycles.is_some_and(|cycles| (cycles - 500.0).abs() < 0.5),
            "the window holds five hundred cycles, not {cycles:?}"
        );
        assert_eq!(columns.len(), 480);
        assert_eq!(
            trace.caption().as_deref(),
            Some("500 cycles \u{b7} envelope")
        );

        let readouts = trace.readouts().expect("a band has readouts");
        assert_eq!(readouts.minimum, 1.0);
        assert_eq!(readouts.maximum, 5.0);
        assert_eq!(
            Some(readouts),
            sample_trace(&pulse, window(2e-6), timing(1e-3)).readouts(),
            "a measured window reads what one period of it reads"
        );
    }

    /// A window that can carry its waveform keeps carrying it: one point per
    /// column, no band, and nothing added to the grid a sine has no corners
    /// to add.
    #[test]
    fn a_sine_the_window_can_carry_stays_a_curve() {
        let trace = sample_trace(&spec("SIN(0 2m 1k)"), window(1e-3), timing(1e-3));

        let WaveformTrace::Curve(samples) = &trace else {
            panic!("one cycle in four hundred and eighty columns is a curve");
        };
        assert_eq!(samples.len(), 480);
        assert_eq!(trace.caption(), None);
    }

    /// A 1 ns edge in a 100 µs window falls between two columns of any grid the
    /// plot can afford: the grid draws it as a ramp across a column, or lands
    /// either side of the pulse and misses it. Merged, every edge in the window
    /// has a sample at each of its own endpoints, so it is drawn where it
    /// happens and as steeply as it happens.
    #[test]
    fn a_nanosecond_edge_is_drawn_where_it_happens() {
        let trace = sample_trace(
            &spec("PULSE(0 5 0 1n 1n 1u 2u)"),
            window(1e-4),
            timing(1e-3),
        );

        let WaveformTrace::Curve(samples) = &trace else {
            panic!("fifty cycles in four hundred and eighty columns is a curve");
        };
        // Sought by time to within a femtosecond of a nanosecond edge, and
        // read to within a nanovolt of a five volt swing: the corner times are
        // summed in the card's own order, and the evaluator reconstructs a
        // cycle's phase by subtracting rather than by adding, so the two agree
        // to the last few bits rather than to all of them.
        let at = |time: f64| {
            samples
                .iter()
                .find(|(sampled, _)| (sampled - time).abs() < 1e-15)
                .map(|(_, value)| *value)
                .unwrap_or_else(|| panic!("no sample at {time}"))
        };
        for cycle in 0..50 {
            let base = f64::from(cycle) * 2e-6;
            assert!(at(base) < 1e-9, "cycle {cycle} rises from nowhere");
            assert!(
                at(base + 1e-9) > 5.0 - 1e-9,
                "cycle {cycle} rises to nowhere"
            );
            assert!(
                at(base + 1e-9 + 1e-6) > 5.0 - 1e-9,
                "cycle {cycle} falls from nowhere"
            );
            assert!(
                at(base + 1e-9 + 1e-6 + 1e-9) < 1e-9,
                "cycle {cycle} falls to nowhere"
            );
        }
    }

    /// An `AM` carrier is measured over one carrier period rather than one
    /// modulating period, so the band it draws is the modulation. Measured the
    /// other way it would be one flat pair of levels, which is the beat the
    /// grid draws and not a waveform anyone authored.
    #[test]
    fn an_amplitude_modulated_carrier_bands_its_modulation() {
        let trace = sample_trace(&spec("AM(0 100m 80m 1k 150k)"), window(2e-3), timing(2e-3));

        let columns = columns_of(&trace);
        let quarter = columns
            .iter()
            .min_by(|left, right| {
                (left.time - 0.25e-3)
                    .abs()
                    .total_cmp(&(right.time - 0.25e-3).abs())
            })
            .expect("a column near the quarter period");
        let depth = quarter.maximum - columns[0].maximum;
        assert!(
            (depth - 80e-3).abs() < 8e-3,
            "the band grew by {depth} where the modulation is 80 mV"
        );
    }

    /// A table with more corners than a polyline can merge is measured, and
    /// the measuring costs what the columns cost rather than what the table
    /// does: each column sweeps its own interval and reads at most
    /// [`COLUMN_BREAKPOINTS`] of the knots inside it.
    #[test]
    fn a_table_denser_than_the_grid_is_measured_and_bounded() {
        let points = (0..5_000)
            .map(|index| (index as f64 * 2e-7, f64::from(index % 2)))
            .collect::<Vec<_>>();
        let trace = sample_trace(
            &SourceSpec::Pwl {
                points,
                delay: 0.0,
                repeat_from: None,
            },
            window(1e-3),
            timing(1e-3),
        );

        let columns = columns_of(&trace);
        assert_eq!(columns.len(), 480);
        assert_eq!(trace.caption().as_deref(), Some("envelope"));
        let readouts = trace.readouts().expect("a band has readouts");
        assert_eq!((readouts.minimum, readouts.maximum), (0.0, 1.0));
    }

    /// A hover over a band reads the range the column covers. One value out of
    /// that range would be a number the run never produced, picked out of a
    /// spread the reader cannot see.
    #[test]
    fn a_band_reads_its_range_and_a_curve_reads_its_sample() {
        let band = sample_trace(
            &spec("PULSE(5 1 0 1n 1n 1u 2u)"),
            window(1e-3),
            timing(1e-3),
        );
        assert!(matches!(
            band.reading_at(0.5e-3),
            Some(TraceReading::Band {
                minimum, maximum, ..
            }) if minimum == 1.0 && maximum == 5.0
        ));

        let curve = sample_trace(&spec("SIN(0 2m 1k)"), window(1e-3), timing(1e-3));
        assert!(matches!(
            curve.reading_at(0.25e-3),
            Some(TraceReading::Sample { value, .. }) if (value - 2e-3).abs() < 1e-5
        ));
    }
}
