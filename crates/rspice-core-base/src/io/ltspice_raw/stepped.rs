//! LTspice's `stepped` layout: every run of a `.step` sweep in one plot.
//!
//! LTspice writes the runs one after another, the scale restarting at its
//! first value for each, under one point count that covers them all. The log
//! it writes beside the rawfile, `deck.log`, states each run's parameter
//! values, one `.step` line a run, in the rawfile header's encoding.

use super::{RawWaveform, RawWaveformData, header_text};

impl RawWaveformData {
    /// Whether the plot holds every run of a `.step` sweep: its flags say
    /// `stepped`.
    #[must_use]
    pub fn is_stepped(&self) -> bool {
        self.header
            .flags
            .iter()
            .any(|flag| flag.eq_ignore_ascii_case("stepped"))
    }

    /// The runs a `stepped` plot holds, in run order, each a plot of its own
    /// points and no longer flagged `stepped`. A run starts wherever the scale
    /// returns to its first value. A plot that is not stepped, or has no
    /// points, is one run: itself.
    #[must_use]
    pub fn into_stepped_runs(self) -> Vec<Self> {
        let starts = match self.waveforms.first().and_then(|scale| scale.y.first()) {
            Some(&first) if self.is_stepped() => self.waveforms[0]
                .y
                .iter()
                .enumerate()
                .filter_map(|(point, &value)| (point == 0 || value == first).then_some(point))
                .collect::<Vec<_>>(),
            _ => return vec![self],
        };
        let points = self.waveforms[0].y.len();
        let ends = starts
            .iter()
            .skip(1)
            .copied()
            .chain(std::iter::once(points));
        let mut header = self.header.clone();
        header
            .flags
            .retain(|flag| !flag.eq_ignore_ascii_case("stepped"));
        starts
            .iter()
            .zip(ends)
            .map(|(&start, end)| Self {
                header: super::RawFileHeader {
                    no_points: end - start,
                    ..header.clone()
                },
                variables: self.variables.clone(),
                waveforms: self
                    .waveforms
                    .iter()
                    .map(|waveform| RawWaveform {
                        name: waveform.name.clone(),
                        x: waveform.x[start..end].to_vec(),
                        y: waveform.y[start..end].to_vec(),
                        y_imag: waveform
                            .y_imag
                            .as_ref()
                            .map(|imag| imag[start..end].to_vec()),
                    })
                    .collect(),
            })
            .collect()
    }
}

/// The parameter values each `.step` run of an LTspice log states, in run
/// order: `.step r=1000 c=1e-09` is `[("r", 1000.0), ("c", 1e-9)]`.
///
/// The log is decoded as a rawfile header is, UTF-16LE or UTF-8 as its first
/// bytes declare. A log any of whose run lines does not read names no run,
/// and neither does one in neither encoding.
#[must_use]
pub fn ltspice_log_step_values(log: &[u8]) -> Vec<Vec<(String, f64)>> {
    let Ok(text) = header_text::decode_text_lossy(log) else {
        return Vec::new();
    };
    let runs: Option<Vec<Vec<(String, f64)>>> = text
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let (keyword, rest) = line.split_at_checked(5)?;
            // A run line assigns its values; any other `.step` text is not one.
            (keyword.eq_ignore_ascii_case(".step")
                && rest.starts_with([' ', '\t'])
                && rest.contains('='))
            .then_some(rest)
        })
        .map(|values| {
            values
                .split_whitespace()
                .map(|assignment| {
                    let (name, value) = assignment.split_once('=')?;
                    let value: f64 = value.parse().ok().filter(|value: &f64| value.is_finite())?;
                    (!name.is_empty()).then(|| (name.to_owned(), value))
                })
                .collect::<Option<Vec<_>>>()
                .filter(|run| !run.is_empty())
        })
        .collect();
    runs.unwrap_or_default()
}
