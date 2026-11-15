//! C2 — charge-pump PLL.
//!
//! The VCO is ngspice's `d_osc` PWL table (control → frequency), the divider
//! is 40, the reference 10 MHz; the loop filter is `loopf` (R1 300, R2 200,
//! C1 10 nF, C2 5 nF, R3 50, C3 0.5 nF).

use num_complex::Complex64;

use super::{Context, Registry, complex, points, values};
use crate::suites::mixed::compare::Expected;

pub const REGISTRY: Registry = &[
    ("vlock", vlock),
    ("fdiv", fdiv),
    ("zlf", zlf),
    ("phase_noise", phase_noise),
];

const CONTROL: [f64; 5] = [0.5, 1.0, 1.5, 2.0, 2.5];
const FREQUENCY: [f64; 5] = [879.0820e6, 747.2197e6, 579.9500e6, 377.2727e6, 161.1650e6];
const DIVISOR: f64 = 40.0;
const REFERENCE: f64 = 10e6;
const R2: f64 = 200.0;
const R3: f64 = 50.0;
const C1: f64 = 10e-9;
const C2: f64 = 5e-9;
const C3: f64 = 0.5e-9;
/// The VCO's frequency-noise density, `sf` in `net/vco_pwl.va` (Hz²/Hz,
/// one-sided): V(fn) carries it through 1 Ω and adds to f at 1 Hz per volt.
const FREQUENCY_NOISE: f64 = 10.0;
/// The trnoise deck's `NOISEFMAX`. Every source is rendered as a held
/// sequence on the grid NT = 1/(2·NOISEFMAX), whose density is the source's
/// times sinc²(f·NT).
const NOISE_FMAX: f64 = 100e6;
/// The pump's charge per second of PFD edge offset at the locked offset,
/// dQ/dΔ, from the O1 deck `oracle/mx.pump.ngspice.cir`: the `loopf_mx`
/// switches and R1 into int1 held at the lock voltage, their gates driven
/// as the PFD drives them (1 ps edges behind the built-in D/A's 20 Ω, both
/// cleared 100 ps after the later edge). The charge per period is zero at
/// Δ₀ = −28.0 ps (the reference edge leads; the switches' 15 nA leakage
/// over a period moves it 0.4 ps), and its slope there is 3.603 mA over
/// Δ₀ ± 5 ps (3.605 over ± 2 ps, 3.596 over ± 10 ps).
const PUMP_GAIN: f64 = 3.603e-3;
/// The run's temperature (27 °C) times Boltzmann's constant, for the loop
/// filter resistors' thermal noise.
const THERMAL: f64 = 1.380649e-23 * 300.15;
/// Reference harmonics summed for the sampled loop gain and the aliased
/// VCO noise. Both terms fall at least as 1/k², so 200 leave < 1e-4 dB.
const HARMONICS: i32 = 200;

/// The control voltage at which the PWL table gives `frequency`.
fn control_for(frequency: f64) -> Option<f64> {
    CONTROL
        .windows(2)
        .zip(FREQUENCY.windows(2))
        .find(|(_, f)| (f[0] - frequency) * (f[1] - frequency) <= 0.0)
        .map(|(v, f)| v[0] + (v[1] - v[0]) * (f[0] - frequency) / (f[0] - f[1]))
}

/// `V_lock = 1.5 + 0.5·(579.95 − 400)/(579.95 − 377.2727) = 1.943932 V`
/// (the design writes 1.94394).
fn vlock(_: &Context<'_>) -> Result<Expected, String> {
    let lock = control_for(DIVISOR * REFERENCE).ok_or("400 MHz is outside the VCO table")?;
    Ok(values(vec![lock]))
}

/// Locked divider period: one reference period.
fn fdiv(_: &Context<'_>) -> Result<Expected, String> {
    Ok(values(vec![1.0 / REFERENCE]))
}

/// The loop-filter impedance seen by the pump, at the deck's injection.
///
/// `Z(jω) = Z1/(1 + jωR3C3)`, `Z1 = [jωC2 + 1/(R2 + 1/jωC1) + 1/(R3 + 1/jωC3)]⁻¹`,
/// valid for `mx`, whose switches are both off at the D-A state. The deck's
/// `iinj xlf.dra 0 ac 1` draws its unit current *out of* `dra` (SPICE: current
/// flows from n+ through the source to n−), so `v(cont) = −Z(jω)`.
fn zlf(context: &Context<'_>) -> Result<Expected, String> {
    let expected = points(context.quantity)?
        .into_iter()
        .map(|frequency| -impedance(frequency))
        .collect();
    Ok(complex(expected))
}

fn impedance(frequency: f64) -> Complex64 {
    let s = Complex64::new(0.0, 2.0 * std::f64::consts::PI * frequency);
    let one = Complex64::new(1.0, 0.0);
    let admittance = s * C2 + one / (one * R2 + one / (s * C1)) + one / (one * R3 + one / (s * C3));
    (one / admittance) / (one + s * R3 * C3)
}

/// The VCO tuning slope at the lock voltage, |df/dV| (Hz/V).
fn tuning_slope() -> Result<f64, String> {
    let lock = control_for(DIVISOR * REFERENCE).ok_or("400 MHz is outside the VCO table")?;
    let segment = CONTROL
        .windows(2)
        .position(|v| lock >= v[0] && lock <= v[1])
        .ok_or("the lock voltage is outside the VCO table")?;
    Ok((FREQUENCY[segment] - FREQUENCY[segment + 1]).abs()
        / (CONTROL[segment + 1] - CONTROL[segment]))
}

/// The single-sideband phase noise L(f) at the VCO output of the locked
/// loop, in dBc/Hz averaged over the band between the two `points`, as the
/// trnoise deck renders its sources, and the interval the measured band mean
/// falls in at 99.9%. The average is over the record's bins, `k/T` for the
/// window's length `T`, the frequencies the measure's band mean holds.
///
/// Every source is frequency noise at the VCO: the VCO's own `sf`, and the
/// loop filter's thermal noise at its output times |Kv|², each rendered on
/// the NOISEFMAX grid:
/// `S_f(f) = [sf + Kv²·Σ 4kT/R·|Z_out,R(f)|²]·sinc²(f·NT)`, with
/// `Z_out,R` the transfer from R2's or R3's Norton current to v(cont). Its
/// phase density at the VCO is `S_f/f²`.
///
/// The PFD samples the VCO phase once per reference period and the pump
/// delivers K_q·Δ of charge, Δ = θ/(2π·f_vco), so the loop is the sampled
/// (Hanumolu 2004) one: with `G_c(s) = K_q·Z(s)·|Kv|/(N·s)` and
/// `G_eff(s) = Σ_m G_c(s + j·m·ω_ref)`,
/// `L(f) = Σ_k |T_k(f)|²·S_f(f + k·f_ref)/(2·(f + k·f_ref)²)`, where
/// `T_0 = 1 − G_c/(1 + G_eff)` is the closed loop's high-pass on VCO noise
/// and `T_k = −G_c/(1 + G_eff)` carries the noise the sampling aliases to f.
/// K_q is [`PUMP_GAIN`]; Z, N, f_ref and the table are the deck's.
///
/// Each bin's power is exponential about L at its frequency, and adjacent
/// Hann bins correlate in power by 4/9 (1/36 two apart), so the band mean is
/// chi-square with Satterthwaite's `nu = 2·mean²/var` degrees of freedom; the
/// interval is its 99.9% two-sided quantiles (Wilson–Hilferty), widened by
/// the band mean's change under a 2% error in K_q and by [`MODEL_MARGIN_DB`].
fn phase_noise(context: &Context<'_>) -> Result<Expected, String> {
    let id = &context.quantity.id;
    let band = points(context.quantity)?;
    let [low, high] = band[..] else {
        return Err(format!("{id}: phase_noise needs one band, two points"));
    };
    if !(high > low && low > 0.0) {
        return Err(format!("{id}: phase_noise needs increasing band edges"));
    }
    // A reference line is not noise, and the sampled loop's transfer is
    // singular on it.
    if (low / REFERENCE).ceil() * REFERENCE <= high {
        return Err(format!(
            "{id}: the band {band:?} holds a reference harmonic"
        ));
    }
    let [start, stop] = context
        .quantity
        .measure
        .window
        .ok_or_else(|| format!("{id}: phase_noise needs the record's window"))?;
    let spacing = 1.0 / (stop - start);
    let first = (low / spacing).ceil() as u64;
    let last = (high / spacing).ceil() as u64;
    if first >= last {
        return Err(format!(
            "{id}: the band {band:?} holds no bin of the record"
        ));
    }
    let slope = tuning_slope()?;
    let bins = |pump_gain: f64| -> Vec<f64> {
        (first..last)
            .map(|bin| locked_phase_noise(bin as f64 * spacing, slope, pump_gain))
            .collect()
    };
    let expected = bins(PUMP_GAIN);
    let count = expected.len() as f64;
    let mean = expected.iter().sum::<f64>() / count;
    let level = 10.0 * mean.log10();
    let pump = [0.98, 1.02]
        .map(|scale| {
            let shifted = bins(PUMP_GAIN * scale).iter().sum::<f64>() / count;
            (10.0 * shifted.log10() - level).abs()
        })
        .into_iter()
        .fold(0.0, f64::max);
    let (below, above) = chi_square_band(&expected);
    let margin = pump + MODEL_MARGIN_DB;
    Ok(Expected::Interval([
        level + below - margin,
        level + above + margin,
    ]))
}

/// What the interval of [`phase_noise`] allows beyond the statistics and
/// K_q: Hann leakage across the band edges, the linearized pump, and the
/// acquisition residual the window's leading taper leaves (under 0.2 dB from
/// 20 us on).
const MODEL_MARGIN_DB: f64 = 0.3;

/// The 99.9% two-sided quantiles (dB about the expectation) of the mean of
/// Hann-windowed periodogram bins whose expected powers are `bins`.
fn chi_square_band(bins: &[f64]) -> (f64, f64) {
    const CORRELATION: [f64; 3] = [1.0, 4.0 / 9.0, 1.0 / 36.0];
    const Z: f64 = 3.2905;
    let count = bins.len();
    let mean = bins.iter().sum::<f64>() / count as f64;
    let variance = (0..count)
        .flat_map(|i| (i.saturating_sub(2)..(i + 3).min(count)).map(move |j| (i, j)))
        .map(|(i, j)| bins[i] * bins[j] * CORRELATION[i.abs_diff(j)])
        .sum::<f64>()
        / (count * count) as f64;
    let nu = 2.0 * mean * mean / variance;
    let quantile = |z: f64| {
        let spread = 2.0 / (9.0 * nu);
        nu * (1.0 - spread + z * spread.sqrt()).powi(3)
    };
    (
        10.0 * (quantile(-Z) / nu).log10(),
        10.0 * (quantile(Z) / nu).log10(),
    )
}

/// L(f) (1/Hz, linear) of [`phase_noise`], for the pump gain `pump_gain`.
fn locked_phase_noise(frequency: f64, slope: f64, pump_gain: f64) -> f64 {
    let loop_gain = |f: f64| {
        let s = Complex64::new(0.0, 2.0 * std::f64::consts::PI * f);
        impedance(f) * (pump_gain * slope / DIVISOR) / s
    };
    let shift = |k: i32| frequency + f64::from(k) * REFERENCE;
    let effective: Complex64 = (-HARMONICS..=HARMONICS).map(|k| loop_gain(shift(k))).sum();
    let gain = loop_gain(frequency);
    let sampled = gain / (Complex64::new(1.0, 0.0) + effective);
    let held = 0.5 / NOISE_FMAX;
    (-HARMONICS..=HARMONICS)
        .map(|k| {
            let offset = shift(k).abs();
            let transfer = if k == 0 {
                (Complex64::new(1.0, 0.0) - sampled).norm_sqr()
            } else {
                sampled.norm_sqr()
            };
            let x = std::f64::consts::PI * offset * held;
            let sinc = x.sin() / x;
            let frequency_noise =
                (FREQUENCY_NOISE + slope * slope * filter_noise(offset)) * sinc * sinc;
            transfer * frequency_noise / (2.0 * offset * offset)
        })
        .sum()
}

/// The loop filter's thermal noise at v(cont) (V²/Hz), from R2 and R3: the
/// network's nodal equations at int1, c1 (between R2 and C1) and vout give
/// each resistor's Norton current its transfer to vout. R1 carries no
/// current while the pump is off (the pump's own pulses are ~1% of the
/// period), and the VCO input loads nothing.
fn filter_noise(frequency: f64) -> f64 {
    let s = Complex64::new(0.0, 2.0 * std::f64::consts::PI * frequency);
    let (g2, g3) = (1.0 / R2, 1.0 / R3);
    let one = Complex64::new(1.0, 0.0);
    // Y·v = i over (int1, c1, vout).
    let y = [
        [s * C2 + g2 + g3, -g2 * one, -g3 * one],
        [-g2 * one, s * C1 + g2, 0.0 * one],
        [-g3 * one, 0.0 * one, s * C3 + g3],
    ];
    // The vout row of Y⁻¹ (Y is symmetric, so it is the solution for a unit
    // current injected at vout).
    let row = solve3(y, [0.0 * one, 0.0 * one, one]);
    let r2 = (row[1] - row[0]).norm_sqr() * 4.0 * THERMAL / R2;
    let r3 = (row[2] - row[0]).norm_sqr() * 4.0 * THERMAL / R3;
    r2 + r3
}

/// `a·x = b` for a 3×3 complex system, by Cramer's rule.
fn solve3(a: [[Complex64; 3]; 3], b: [Complex64; 3]) -> [Complex64; 3] {
    let det = |m: [[Complex64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let whole = det(a);
    let mut x = [Complex64::new(0.0, 0.0); 3];
    for (column, value) in x.iter_mut().enumerate() {
        let mut m = a;
        for row in 0..3 {
            m[row][column] = b[row];
        }
        *value = det(m) / whole;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_voltage_and_loop_filter_limits() {
        // 1.5 + 0.5·179.95/202.6773 = 1.943932 (the design writes 1.94394).
        let lock = control_for(400e6).unwrap();
        assert!((lock - 1.943932).abs() < 1e-6, "{lock}");
        // Low frequency: C1 + C2 + C3 dominate; high frequency: C2 ∥ C3 path.
        let low = impedance(1.0);
        let expected = 1.0 / (2.0 * std::f64::consts::PI * (C1 + C2 + C3));
        assert!((low.norm() - expected).abs() / expected < 1e-3, "{low}");
        assert!(impedance(1e9).norm() < 1.0);
    }

    #[test]
    fn locked_phase_noise_is_the_vco_noise_high_passed_by_the_loop() {
        let slope = tuning_slope().unwrap();
        // 405.3546 MHz/V on [1.5, 2] V.
        assert!((slope - 405.3546e6).abs() < 1.0, "{slope}");
        let free = |f: f64| FREQUENCY_NOISE / (2.0 * f * f);
        // Far above the loop bandwidth the VCO runs free: within the filter
        // resistors' share (< 1%) and the loop's last 0.1 dB.
        let high = locked_phase_noise(5e6, slope, PUMP_GAIN) / free(5e6);
        assert!((high - 1.0).abs() < 0.02, "{high}");
        // Deep inside it the loop suppresses the VCO noise by |G|².
        assert!(locked_phase_noise(30e3, slope, PUMP_GAIN) < free(30e3) * 1e-3);
        // The filter resistors: R3's 4kT·R3 reaches vout below C3's pole.
        let low = filter_noise(1e3);
        assert!(low > 0.0 && low < 4.0 * THERMAL * (R2 + R3), "{low}");
    }

    #[test]
    fn a_band_of_independent_equal_bins_is_chi_square_with_twice_their_count() {
        // Correlation-free bins would give nu = 2M; Hann's 4/9 and 1/36
        // lower it to 2M/(1 + 2·4/9 + 2·1/36) away from the band's ends.
        let (below, above) = chi_square_band(&[1.0; 400]);
        let nu: f64 = 800.0 / (1.0 + 8.0 / 9.0 + 1.0 / 18.0);
        let spread = 2.0 / (9.0 * nu);
        let quantile = |z: f64| (1.0 - spread + z * spread.sqrt()).powi(3);
        assert!(
            (below - 10.0 * quantile(-3.2905).log10()).abs() < 0.01,
            "{below}"
        );
        assert!(
            (above - 10.0 * quantile(3.2905).log10()).abs() < 0.01,
            "{above}"
        );
        assert!(below < 0.0 && above > 0.0 && -below > above);
    }
}
