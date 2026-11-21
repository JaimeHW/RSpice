//! Two-port amplifier figures and Smith-chart circles from retained
//! S-parameters: the Rollett and Edwards-Sinsky stability factors, the source
//! and load stability circles, available- and power-gain circles, the
//! maximum gain, and noise-figure circles from retained two-port noise
//! parameters.
//!
//! The formulas are the standard ones (Gonzalez, *Microwave Transistor
//! Amplifiers*, 2nd ed., sections 3.3, 3.6 and 4.3; Edwards and Sinsky, IEEE
//! MTT-40, 1992). Each port's coefficients are referred to that port's own
//! retained reference impedance, so a source-plane circle is read against
//! port 1's Z0 and a load-plane circle against port 2's.
//!
//! Nothing here invents a figure the evidence does not support: a circle
//! whose defining denominator vanishes, a gain no termination reaches, or a
//! noise factor below the minimum is `None`, never a plausible-looking ring.

use num_complex::Complex64;

use crate::analysis_result::AnalysisResult;
use crate::analysis_type::AnalysisType;
use crate::waveform::{Fetched, HeldColumn, RetainedWaveform, WaveformValues};

/// A circle on the reflection-coefficient plane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SmithCircle {
    pub center: Complex64,
    pub radius: f64,
}

impl SmithCircle {
    /// Whether a reflection coefficient lies strictly inside the circle.
    pub fn contains(&self, gamma: Complex64) -> bool {
        (gamma - self.center).norm() < self.radius
    }
}

/// A stability circle and the side of it on which the other port's
/// reflection coefficient stays below unity magnitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StabilityCircle {
    pub circle: SmithCircle,
    /// The stable terminations are the circle's interior; otherwise its
    /// exterior.
    pub stable_inside: bool,
}

impl StabilityCircle {
    /// Whether a termination on this plane keeps the opposite port's
    /// reflection below unity magnitude.
    pub fn is_stable(&self, gamma: Complex64) -> bool {
        self.circle.contains(gamma) == self.stable_inside
    }
}

/// The largest transducer gain the two-port offers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaximumGain {
    /// Maximum available gain of an unconditionally stable two-port, reached
    /// with a simultaneous conjugate match. Linear power ratio.
    Available(f64),
    /// Maximum stable gain |S21|/|S12| of a potentially unstable two-port.
    /// Linear power ratio.
    Stable(f64),
}

impl MaximumGain {
    /// The gain as a linear power ratio, whichever kind it is.
    pub fn linear(self) -> f64 {
        match self {
            Self::Available(gain) | Self::Stable(gain) => gain,
        }
    }
}

/// One frequency of a two-port's scattering matrix.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoPortScattering {
    pub s11: Complex64,
    pub s12: Complex64,
    pub s21: Complex64,
    pub s22: Complex64,
}

impl TwoPortScattering {
    /// The determinant Δ = S11·S22 − S12·S21.
    pub fn determinant(&self) -> Complex64 {
        self.s11 * self.s22 - self.s12 * self.s21
    }

    /// 1 − |S11|² − |S22|² + |Δ|², the numerator of the Rollett factor.
    fn rollett_numerator(&self) -> f64 {
        1.0 - self.s11.norm_sqr() - self.s22.norm_sqr() + self.determinant().norm_sqr()
    }

    /// The Rollett stability factor K. Infinite for a unilateral two-port.
    pub fn rollett_k(&self) -> f64 {
        self.rollett_numerator() / (2.0 * (self.s12 * self.s21).norm())
    }

    /// The Edwards-Sinsky load-plane factor μ: the distance from the chart
    /// centre to the nearest load that makes the input unstable. The two-port
    /// is unconditionally stable exactly when μ > 1.
    pub fn mu_load(&self) -> f64 {
        let delta = self.determinant();
        (1.0 - self.s11.norm_sqr())
            / ((self.s22 - delta * self.s11.conj()).norm() + (self.s12 * self.s21).norm())
    }

    /// The Edwards-Sinsky source-plane factor μ′, the same distance on the
    /// source plane.
    pub fn mu_source(&self) -> f64 {
        let delta = self.determinant();
        (1.0 - self.s22.norm_sqr())
            / ((self.s11 - delta * self.s22.conj()).norm() + (self.s12 * self.s21).norm())
    }

    /// Whether every passive source and load keeps both ports stable.
    pub fn is_unconditionally_stable(&self) -> bool {
        self.mu_load() > 1.0
    }

    /// The reflection looking into port 1 with port 2 terminated in `load`.
    pub fn input_reflection(&self, load: Complex64) -> Complex64 {
        self.s11 + self.s12 * self.s21 * load / (1.0 - self.s22 * load)
    }

    /// The reflection looking into port 2 with port 1 driven from `source`.
    pub fn output_reflection(&self, source: Complex64) -> Complex64 {
        self.s22 + self.s12 * self.s21 * source / (1.0 - self.s11 * source)
    }

    /// Input (source-plane) stability circle: the sources for which
    /// |Γout| = 1. `None` when the boundary is a straight line.
    pub fn source_stability_circle(&self) -> Option<StabilityCircle> {
        let delta = self.determinant();
        stability_circle(self.s11, self.s22, delta, (self.s12 * self.s21).norm())
    }

    /// Output (load-plane) stability circle: the loads for which |Γin| = 1.
    /// `None` when the boundary is a straight line.
    pub fn load_stability_circle(&self) -> Option<StabilityCircle> {
        let delta = self.determinant();
        stability_circle(self.s22, self.s11, delta, (self.s12 * self.s21).norm())
    }

    /// MAG for an unconditionally stable two-port, otherwise MSG. `None` for
    /// a potentially unstable two-port with no reverse transmission.
    pub fn maximum_gain(&self) -> Option<MaximumGain> {
        let forward = self.s21.norm_sqr();
        let reverse = (self.s12 * self.s21).norm();
        if self.is_unconditionally_stable() {
            // MAG = |S21|/|S12|·(K − √(K² − 1)), written without the division
            // by |S12| so a unilateral two-port reads |S21|²/((1−|S11|²)(1−|S22|²)).
            let numerator = self.rollett_numerator();
            let discriminant = numerator * numerator - 4.0 * reverse * reverse;
            if numerator <= 0.0 || discriminant < 0.0 {
                return None;
            }
            let gain = 2.0 * forward / (numerator + discriminant.sqrt());
            return gain.is_finite().then_some(MaximumGain::Available(gain));
        }
        let ratio = self.s21.norm() / self.s12.norm();
        (ratio.is_finite() && ratio > 0.0).then_some(MaximumGain::Stable(ratio))
    }

    /// Available power gain with port 1 driven from `source` and port 2
    /// conjugately matched.
    pub fn available_gain(&self, source: Complex64) -> f64 {
        let output = self.output_reflection(source);
        self.s21.norm_sqr() * (1.0 - source.norm_sqr())
            / ((1.0 - self.s11 * source).norm_sqr() * (1.0 - output.norm_sqr()))
    }

    /// Operating power gain with port 2 terminated in `load`.
    pub fn power_gain(&self, load: Complex64) -> f64 {
        let input = self.input_reflection(load);
        self.s21.norm_sqr() * (1.0 - load.norm_sqr())
            / ((1.0 - input.norm_sqr()) * (1.0 - self.s22 * load).norm_sqr())
    }

    /// The source-plane circle of constant available gain `gain` (a linear
    /// power ratio). `None` when no source reaches that gain.
    pub fn available_gain_circle(&self, gain: f64) -> Option<SmithCircle> {
        let delta = self.determinant();
        self.gain_circle(gain, self.s11, self.s22, delta)
    }

    /// The load-plane circle of constant operating power gain `gain` (a
    /// linear power ratio). `None` when no load reaches that gain.
    pub fn power_gain_circle(&self, gain: f64) -> Option<SmithCircle> {
        let delta = self.determinant();
        self.gain_circle(gain, self.s22, self.s11, delta)
    }

    /// Gonzalez (3.6.10)-(3.6.11) and their source-plane twins: with
    /// `own` the plane's own reflection term and `other` the opposite one,
    /// C = g·(own − Δ·other*)* / (1 + g(|own|² − |Δ|²)) and
    /// r = √(1 − 2K|S12S21|g + |S12S21|²g²) / |1 + g(|own|² − |Δ|²)|.
    fn gain_circle(
        &self,
        gain: f64,
        own: Complex64,
        other: Complex64,
        delta: Complex64,
    ) -> Option<SmithCircle> {
        let forward = self.s21.norm_sqr();
        if !gain.is_finite() || gain <= 0.0 || forward == 0.0 {
            return None;
        }
        let normalized = gain / forward;
        let reverse = (self.s12 * self.s21).norm();
        let radicand = 1.0 - normalized * self.rollett_numerator()
            + reverse * reverse * normalized * normalized;
        let denominator = 1.0 + normalized * (own.norm_sqr() - delta.norm_sqr());
        if radicand < 0.0 || denominator == 0.0 {
            return None;
        }
        let center = normalized * (own - delta * other.conj()).conj() / denominator;
        let radius = radicand.sqrt() / denominator.abs();
        (center.re.is_finite() && center.im.is_finite() && radius.is_finite())
            .then_some(SmithCircle { center, radius })
    }
}

/// Gonzalez (3.3.7)-(3.3.10): with `own` the plane's own reflection term
/// and `other` the opposite one, C = (own − Δ·other*)* / (|own|² − |Δ|²) and
/// r = |S12S21| / ||own|² − |Δ|²|. Expanding |Γ_other| < 1 gives
/// (|own|² − |Δ|²)(|Γ − C|² − r²) > 0, so the interior is stable exactly when
/// the denominator is negative.
fn stability_circle(
    own: Complex64,
    other: Complex64,
    delta: Complex64,
    reverse: f64,
) -> Option<StabilityCircle> {
    let denominator = own.norm_sqr() - delta.norm_sqr();
    if denominator == 0.0 {
        return None;
    }
    let center = (own - delta * other.conj()).conj() / denominator;
    let radius = reverse / denominator.abs();
    (center.re.is_finite() && center.im.is_finite() && radius.is_finite()).then_some(
        StabilityCircle {
            circle: SmithCircle { center, radius },
            stable_inside: denominator < 0.0,
        },
    )
}

/// Two-port noise parameters at one frequency.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoPortNoiseParameters {
    /// Minimum noise factor, linear.
    pub minimum_noise_factor: f64,
    /// Equivalent noise resistance, ohms.
    pub noise_resistance_ohm: f64,
    /// Source reflection coefficient achieving the minimum, referred to
    /// `reference_impedance_ohm`.
    pub optimum_source_reflection: Complex64,
    /// Port 1's reference impedance, ohms.
    pub reference_impedance_ohm: f64,
}

impl TwoPortNoiseParameters {
    /// 4·Rn/Z0 · 1/|1 + Γopt|², the factor the source-mismatch term carries.
    fn mismatch_weight(&self) -> f64 {
        4.0 * self.noise_resistance_ohm
            / self.reference_impedance_ohm
            / (1.0 + self.optimum_source_reflection).norm_sqr()
    }

    /// The noise factor with port 1 driven from `source`.
    pub fn noise_factor(&self, source: Complex64) -> f64 {
        self.minimum_noise_factor
            + self.mismatch_weight() * (source - self.optimum_source_reflection).norm_sqr()
                / (1.0 - source.norm_sqr())
    }

    /// The source-plane circle of constant noise factor `noise_factor`
    /// (linear). Gonzalez (4.3.6)-(4.3.8): N = (F − Fmin)/(4rn)·|1 + Γopt|²,
    /// C = Γopt/(1 + N), r = √(N(N + 1 − |Γopt|²))/(1 + N). `None` below the
    /// minimum noise factor.
    pub fn noise_circle(&self, noise_factor: f64) -> Option<SmithCircle> {
        let weight = self.mismatch_weight();
        if !noise_factor.is_finite()
            || noise_factor < self.minimum_noise_factor
            || !weight.is_finite()
            || weight <= 0.0
        {
            return None;
        }
        let n = (noise_factor - self.minimum_noise_factor) / weight;
        let radicand = n * (n + 1.0 - self.optimum_source_reflection.norm_sqr());
        if radicand < 0.0 {
            return None;
        }
        Some(SmithCircle {
            center: self.optimum_source_reflection / (1.0 + n),
            radius: radicand.sqrt() / (1.0 + n),
        })
    }
}

/// A two-port S-parameter sweep resolved from one retained analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct TwoPortSweep {
    pub frequencies: Vec<f64>,
    pub scattering: Vec<TwoPortScattering>,
    /// Port 1 and port 2 reference impedances, ohms.
    pub reference_impedances_ohm: [f64; 2],
    /// Retained two-port noise parameters on the same grid, when the run
    /// requested port noise.
    pub noise: Option<Vec<TwoPortNoiseParameters>>,
}

impl TwoPortSweep {
    /// Resolve the physical two-port of a successful S-parameter analysis.
    ///
    /// `Ready(None)` unless the run declared exactly two ports, retained all four
    /// single-ended coefficients on one finite, strictly increasing grid, and
    /// carries no sideband terms. Noise parameters are attached only when
    /// `Rn`, `Fmin` and the complex `Sopt` were retained on that same grid.
    ///
    /// The network is solved at every frequency of the sweep at once, so a stored sweep is asked
    /// of the reader service whole: on the thread that draws the window `Pending` while it is
    /// read; `Refused` when it cannot be (a sweep larger than the loaded-results budget).
    pub fn resolve<W: AsRef<RetainedWaveform>>(
        analysis: &AnalysisResult<W>,
    ) -> Fetched<Option<Self>> {
        match Self::read(analysis) {
            Ok(sweep) => Fetched::Ready(sweep),
            Err(Unread::Pending) => Fetched::Pending,
            Err(Unread::Refused(reason)) => Fetched::Refused(reason),
        }
    }

    fn read<W: AsRef<RetainedWaveform>>(
        analysis: &AnalysisResult<W>,
    ) -> Result<Option<Self>, Unread> {
        if analysis.analysis_type != AnalysisType::SParameter {
            return Ok(None);
        }
        let Some(layout) = super::resolve(analysis) else {
            return Ok(None);
        };
        let &[port1, port2] = layout.references() else {
            return Ok(None);
        };
        let Some(block) = layout
            .blocks()
            .iter()
            .find(|block| !block.is_mixed() && block.key.sidebands.is_none())
        else {
            return Ok(None);
        };
        let Some(cells) = block
            .cells()
            .iter()
            .map(|index| analysis.waveforms.get(*index).map(AsRef::as_ref))
            .collect::<Option<Vec<_>>>()
        else {
            return Ok(None);
        };
        let Some(axis) = cells.first().map(|cell| &cell.x) else {
            return Ok(None);
        };
        let grid = held(axis)?;
        if grid.is_empty()
            || grid.iter().any(|frequency| !frequency.is_finite())
            || !grid.windows(2).all(|pair| pair[0] < pair[1])
        {
            return Ok(None);
        }
        let mut columns = Vec::with_capacity(4);
        for waveform in &cells {
            let Some(complex) = waveform.complex.as_ref() else {
                return Ok(None);
            };
            if complex.real.len() != grid.len()
                || complex.imag.len() != grid.len()
                || !on_grid(&waveform.x, axis, &grid)?
            {
                return Ok(None);
            }
            columns.push((held(&complex.real)?, held(&complex.imag)?));
        }
        let scattering = (0..grid.len())
            .map(|index| {
                let at =
                    |cell: usize| Complex64::new(columns[cell].0[index], columns[cell].1[index]);
                TwoPortScattering {
                    s11: at(0),
                    s12: at(1),
                    s21: at(2),
                    s22: at(3),
                }
            })
            .collect::<Vec<_>>();
        if scattering.iter().any(|point| {
            [point.s11, point.s12, point.s21, point.s22]
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
        }) {
            return Ok(None);
        }
        let noise = retained_noise(analysis, axis, &grid, port1)?;
        Ok(Some(Self {
            frequencies: grid.to_vec(),
            scattering,
            reference_impedances_ohm: [port1, port2],
            noise,
        }))
    }

    /// The retained sample nearest `frequency`.
    pub fn nearest_index(&self, frequency: f64) -> Option<usize> {
        (0..self.frequencies.len()).min_by(|left, right| {
            (self.frequencies[*left] - frequency)
                .abs()
                .total_cmp(&(self.frequencies[*right] - frequency).abs())
        })
    }
}

/// Why a column the two-port needs is not to hand yet.
enum Unread {
    /// The reader service is reading it.
    Pending,
    /// It cannot be read.
    Refused(String),
}

/// Every sample of `values`, as the reader service holds them.
fn held(values: &WaveformValues) -> Result<HeldColumn, Unread> {
    match values.fetch_whole_or_wait() {
        Fetched::Ready(samples) => Ok(samples),
        Fetched::Pending => Err(Unread::Pending),
        Fetched::Refused(reason) => Err(Unread::Refused(reason)),
    }
}

/// Whether `x` is the scattering grid, `axis` held as `grid`: the same column without a read,
/// else the same samples.
fn on_grid(x: &WaveformValues, axis: &WaveformValues, grid: &[f64]) -> Result<bool, Unread> {
    if WaveformValues::ptr_eq(x, axis) {
        return Ok(true);
    }
    if x.len() != grid.len() {
        return Ok(false);
    }
    Ok(held(x)?[..] == *grid)
}

/// `Rn`, `Fmin` and `Sopt` as the S-parameter runner retains them, on the
/// scattering grid.
fn retained_noise<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    axis: &WaveformValues,
    grid: &[f64],
    reference_impedance_ohm: f64,
) -> Result<Option<Vec<TwoPortNoiseParameters>>, Unread> {
    let named = |name: &str| -> Result<Option<&RetainedWaveform>, Unread> {
        for waveform in analysis.waveforms.iter().map(AsRef::as_ref) {
            if waveform.name == name && on_grid(&waveform.x, axis, grid)? {
                return Ok(Some(waveform));
            }
        }
        Ok(None)
    };
    let (Some(resistance), Some(minimum), Some(optimum)) =
        (named("Rn")?, named("Fmin")?, named("Sopt")?)
    else {
        return Ok(None);
    };
    let Some(optimum) = optimum.complex.as_ref() else {
        return Ok(None);
    };
    if resistance.y.len() != grid.len()
        || minimum.y.len() != grid.len()
        || optimum.real.len() != grid.len()
        || optimum.imag.len() != grid.len()
    {
        return Ok(None);
    }
    let (resistance, minimum) = (held(&resistance.y)?, held(&minimum.y)?);
    let (optimum_real, optimum_imag) = (held(&optimum.real)?, held(&optimum.imag)?);
    let parameters = (0..grid.len())
        .map(|index| TwoPortNoiseParameters {
            minimum_noise_factor: minimum[index],
            noise_resistance_ohm: resistance[index],
            optimum_source_reflection: Complex64::new(optimum_real[index], optimum_imag[index]),
            reference_impedance_ohm,
        })
        .collect::<Vec<_>>();
    Ok(parameters
        .iter()
        .all(|point| {
            point.minimum_noise_factor.is_finite()
                && point.minimum_noise_factor >= 1.0
                && point.noise_resistance_ohm.is_finite()
                && point.noise_resistance_ohm >= 0.0
                && point.optimum_source_reflection.norm() < 1.0
        })
        .then_some(parameters))
}

#[cfg(test)]
mod tests;
