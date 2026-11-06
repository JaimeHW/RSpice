//! ngspice `fft` (frontend/com_fft.c): the spectrum of a transient plot's
//! vectors, in a new plot.

use super::*;
use crate::engine::control::shell::{ShellValue, percent_g};

impl ControlCircuit {
    /// ngspice `fft vector ...` (frontend/com_fft.c:27 com_fft, the Green
    /// FFT its Windows build uses): each real vector of the current transient
    /// plot, weighted by the `specwindow` window (hanning unless set, of
    /// order `specwindoworder`, at least 2) and padded with zeros to a power
    /// of two N, becomes a complex vector of N/2 + 1 bins in a new spectrum
    /// plot, its scale `frequency` in steps of length/(span N), the span the
    /// time from the first point to the last plus the last step. A bin is the
    /// transform over N/2, the first and the last over N. Returns the report.
    pub(in crate::engine) fn fft(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<String, ControlExecutionError> {
        let line = command.line;
        let times = self.fft_times(line)?;
        let length = times.len();
        if length < 2 {
            return Err(command_error(
                line,
                "fft needs more than one time point, check the tran simulation",
            )
            .into());
        }
        let span = times[length - 1] - times[0] + times[length - 1] - times[length - 2];
        let size = length.next_power_of_two();
        let points = size / 2 + 1;
        let scale = size as Value / 2.0;
        let mut report = String::new();
        let window = match self.shell.get("specwindow") {
            Some(ShellValue::Word(text)) => text.to_ascii_lowercase(),
            _ => "hanning".to_owned(),
        };
        let order = match self.shell.get("specwindoworder") {
            Some(ShellValue::Word(text)) => text.parse::<Value>().map_or(2, |order| order as i64),
            _ => 2,
        }
        .max(2);
        let weights = fft_window(&window, &times, span, order).unwrap_or_else(|| {
            report.push_str(&format!(
                "Warning: unknown window type {window} for fft, set to \"none\"\n"
            ));
            vec![1.0; length]
        });

        let mut inputs = Vec::new();
        let mut input = command.arguments.trim();
        while !input.is_empty() {
            check_abort(abort, line)?;
            let (expression, label) = expression(&mut input, line, abort)?;
            let value = self.evaluate_vector(&expression, variables, line, abort)?;
            let name = label.to_ascii_lowercase();
            if value.samples.len() != length {
                report.push_str(&format!(
                    "Error: lengths of {name} vectors don't match: {}, {length}\n",
                    value.samples.len()
                ));
            } else if value.complex {
                report.push_str(&format!("Error: {name} isn't real!\n"));
            } else if value.unit != SignalUnit::Second {
                inputs.push((name, value));
            }
        }
        if inputs.is_empty() {
            return Ok(report);
        }
        if points.saturating_mul(inputs.len() + 1) > self.value_limit {
            return Err(command_error(
                line,
                format!("fft: {points} points exceed the result value limit"),
            )
            .into());
        }
        report.push_str(&format!(
            "FFT: Time span: {} s, input length: {length}, zero padding: {}\n\
             FFT: Frequency resolution: {} Hz, output length: {points}\n",
            percent_g(span),
            size - length,
            percent_g(1.0 / span)
        ));

        let name = self.new_plot_name("sp", line)?;
        self.vectors
            .new_analysis_plot(name, self.datasets.len(), "Spectrum");
        let frequencies = (0..points)
            .map(|bin| (bin as Value / span * length as Value / size as Value).into())
            .collect();
        self.push_vector(
            "frequency",
            VectorValue {
                samples: frequencies,
                complex: false,
                scale: None,
                unit: SignalUnit::Hertz,
            },
            false,
        );
        for (name, value) in inputs {
            check_abort(abort, line)?;
            let mut data: Vec<ComplexValue> = value
                .samples
                .iter()
                .zip(&weights)
                .map(|(sample, weight)| ComplexValue::new(sample.re * weight, 0.0))
                .collect();
            data.resize(size, ComplexValue::new(0.0, 0.0));
            transform(&mut data);
            let last = points - 1;
            let samples = (0..points)
                .map(|bin| {
                    if bin == 0 || bin == last {
                        ComplexValue::new(data[bin].re / scale / 2.0, 0.0)
                    } else {
                        ComplexValue::new(data[bin].re / scale, data[bin].im / scale)
                    }
                })
                .collect();
            self.push_vector(
                &name,
                VectorValue {
                    samples,
                    complex: true,
                    scale: None,
                    unit: SignalUnit::Unspecified,
                },
                false,
            );
        }
        Ok(report)
    }

    /// The current plot's time scale (com_fft.c:51).
    fn fft_times(&self, line: usize) -> Result<Vec<Value>, ControlError> {
        let not_time = || command_error(line, "fft needs real time scale");
        if let Some(plot) = self.vectors.current_plot() {
            let scale = self.vectors.plot_scale(plot).ok_or_else(not_time)?;
            let (_, value) = self
                .vectors
                .vectors_in(plot)
                .find(|(name, _)| name.eq_ignore_ascii_case(scale))
                .ok_or_else(not_time)?;
            if value.complex || value.unit != SignalUnit::Second {
                return Err(not_time());
            }
            return Ok(value.samples.iter().map(|sample| sample.re).collect());
        }
        let dataset = self
            .current_dataset()
            .ok_or_else(|| command_error(line, "fft: no vectors loaded"))?;
        if !matches!(dataset.result, ControlAnalysisResult::Transient(_)) {
            return Err(not_time());
        }
        (0..dataset.length())
            .map(|row| dataset.scale_value(row).filter(|time| time.is_finite()))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(not_time)
    }
}

/// ngspice's spectrum windows (maths/fft/fftext.c:97 fft_windows), each of
/// mean one, over the last `span` of the times: none, rectangular, triangle
/// (bartlet, bartlett), hann (hanning, cosine), hamming, blackman,
/// blackmanharris, flattop and gaussian of `order`. None for a name it
/// does not know.
fn fft_window(window: &str, times: &[Value], span: Value, order: i64) -> Option<Vec<Value>> {
    let maxt = times[times.len() - 1];
    let cosines = |terms: &[Value]| -> Vec<Value> {
        times
            .iter()
            .map(|&time| {
                if maxt - time > span {
                    return 0.0;
                }
                let phase = 2.0 * std::f64::consts::PI * (time - maxt) / span;
                terms
                    .iter()
                    .enumerate()
                    .map(|(harmonic, weight)| weight * (harmonic as Value * phase).cos())
                    .sum()
            })
            .collect()
    };
    Some(match window {
        "none" => vec![1.0; times.len()],
        "rectangular" => cosines(&[1.0]),
        "triangle" | "bartlet" | "bartlett" => times
            .iter()
            .map(|&time| {
                let back = maxt - time;
                if back > span {
                    0.0
                } else if back < span / 2.0 {
                    4.0 * back / span
                } else {
                    4.0 - 4.0 * back / span
                }
            })
            .collect(),
        "hann" | "hanning" | "cosine" => cosines(&[1.0, -1.0]),
        "hamming" => cosines(&[1.0, -0.46 / 0.54]),
        "blackman" => cosines(&[1.0, -0.50 / 0.42, 0.08 / 0.42]),
        "blackmanharris" => cosines(&[
            1.0,
            -0.48829 / 0.35875,
            0.14128 / 0.35875,
            -0.01168 / 0.35875,
        ]),
        "flattop" => cosines(&[1.0, -1.93, 1.29, -0.388, 0.032]),
        "gaussian" => {
            let sigma = 1.0 / order as Value;
            let scale = 0.83 / sigma;
            times
                .iter()
                .map(|&time| {
                    if maxt - time > span {
                        0.0
                    } else {
                        scale * (-0.5 * ((time - maxt / 2.0) / (sigma * maxt / 2.0)).powi(2)).exp()
                    }
                })
                .collect()
        }
        _ => return None,
    })
}

/// The forward transform, sum of x[n] e^(-2 pi i k n / N), in place, of a
/// power-of-two length.
fn transform(data: &mut [ComplexValue]) {
    let size = data.len();
    let mut swap = 0;
    for index in 1..size {
        let mut bit = size >> 1;
        while swap & bit != 0 {
            swap ^= bit;
            bit >>= 1;
        }
        swap |= bit;
        if index < swap {
            data.swap(index, swap);
        }
    }
    let mut width = 2;
    while width <= size {
        let angle = -2.0 * std::f64::consts::PI / width as Value;
        for start in (0..size).step_by(width) {
            for offset in 0..width / 2 {
                let (sin, cos) = (angle * offset as Value).sin_cos();
                let twiddle = ComplexValue::new(cos, sin);
                let even = data[start + offset];
                let odd = data[start + offset + width / 2] * twiddle;
                data[start + offset] = even + odd;
                data[start + offset + width / 2] = even - odd;
            }
        }
        width <<= 1;
    }
}
