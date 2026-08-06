//! Shared closed-form device physics for oracle computation.
//!
//! Constants are the ngspice-dialect definitions the adapter pins
//! (CODATA k and q, 27°C nominal = 300.15 K): SPICE model equations are
//! *defined* in terms of these values, so using them is part of the model
//! specification, not an imitation of the engine.

pub const K_BOLTZMANN: f64 = 1.380649e-23;
pub const Q_ELECTRON: f64 = 1.602176634e-19;
pub const KELVIN_AT_ZERO_CELSIUS: f64 = 273.15;
pub const NOMINAL_CELSIUS: f64 = 27.0;

pub fn thermal_voltage(temperature_celsius: f64) -> f64 {
    (temperature_celsius + KELVIN_AT_ZERO_CELSIUS) * K_BOLTZMANN / Q_ELECTRON
}

/// SPICE junction saturation-current temperature scaling with the level-1
/// diode defaults `eg = 1.11`, `xti = 3`:
/// `is(T) = is * (T/Tnom)^(xti/n) * exp(eg/(n*vt) * (T/Tnom - 1))`.
pub fn diode_saturation_current_at(is_nominal: f64, n: f64, temperature_celsius: f64) -> f64 {
    const ENERGY_GAP: f64 = 1.11;
    const XTI: f64 = 3.0;
    let t = temperature_celsius + KELVIN_AT_ZERO_CELSIUS;
    let t_nominal = NOMINAL_CELSIUS + KELVIN_AT_ZERO_CELSIUS;
    let ratio = t / t_nominal;
    let vt = thermal_voltage(temperature_celsius);
    is_nominal * ratio.powf(XTI / n) * (ENERGY_GAP / (n * vt) * (ratio - 1.0)).exp()
}

/// Node voltage of a source-fed diode with series resistance: the root of
/// `(vs - v) / r = is * (exp(v / (n*vt)) - 1)`, bracketed and bisected to
/// the last representable bit. Independent mathematics — the analytic node
/// equation solved directly, not the engine's companion-model iteration.
pub fn diode_node_voltage(vs: f64, r: f64, is: f64, n: f64, temperature_celsius: f64) -> f64 {
    let n_vt = n * thermal_voltage(temperature_celsius);
    let residual = |v: f64| (vs - v) / r - is * ((v / n_vt).exp() - 1.0);
    let (mut low, mut high) = (0.0_f64, vs);
    for _ in 0..200 {
        let middle = 0.5 * (low + high);
        if middle <= low || middle >= high {
            break;
        }
        if residual(middle) > 0.0 {
            low = middle;
        } else {
            high = middle;
        }
    }
    0.5 * (low + high)
}

/// First-order temperature-coefficient resistance:
/// `r(T) = r * (1 + tc1*dT + tc2*dT^2)` around the 27°C nominal.
pub fn resistance_at(r_nominal: f64, tc1: f64, tc2: f64, temperature_celsius: f64) -> f64 {
    let delta = temperature_celsius - NOMINAL_CELSIUS;
    r_nominal * (1.0 + tc1 * delta + tc2 * delta * delta)
}

/// The SPICE `PULSE(V1 V2 TD TR TF PW PER)` source: both the deck spelling
/// and the exact piecewise-linear waveform that spelling defines.
pub struct Pulse {
    pub initial: f64,
    pub pulsed: f64,
    pub delay: f64,
    pub rise: f64,
    pub fall: f64,
    pub width: f64,
    pub period: f64,
}

impl Pulse {
    pub fn spice(&self) -> String {
        format!(
            "pulse({} {} {} {} {} {} {})",
            self.initial, self.pulsed, self.delay, self.rise, self.fall, self.width, self.period
        )
    }

    /// The waveform value at any simulation time.
    pub fn value_at(&self, time: f64) -> f64 {
        if time < self.delay {
            return self.initial;
        }
        let cycle_time = (time - self.delay) % self.period;
        if cycle_time < self.rise {
            self.initial + (self.pulsed - self.initial) * cycle_time / self.rise
        } else if cycle_time < self.rise + self.width {
            self.pulsed
        } else if cycle_time < self.rise + self.width + self.fall {
            let into_fall = cycle_time - self.rise - self.width;
            self.pulsed + (self.initial - self.pulsed) * into_fall / self.fall
        } else {
            self.initial
        }
    }

    /// Piecewise-linear breakpoints covering `[0, stop]`. Points past `stop`
    /// are retained so a marching evaluator can finish mid-segment.
    pub fn breakpoints(&self, stop: f64) -> Vec<(f64, f64)> {
        let mut points = vec![(0.0, self.initial)];
        let mut cycle_start = self.delay;
        while cycle_start < stop {
            points.push((cycle_start, self.initial));
            points.push((cycle_start + self.rise, self.pulsed));
            points.push((cycle_start + self.rise + self.width, self.pulsed));
            points.push((
                cycle_start + self.rise + self.width + self.fall,
                self.initial,
            ));
            cycle_start += self.period;
        }
        points
    }
}

/// Exact final value at `stop` of the first-order lag
/// `tau * dv/dt = u(t) - v` driven by the piecewise-linear waveform through
/// `points`, starting from `initial`. Each segment advances the closed-form
/// solution in a cancellation-free arrangement (`exp_m1`, and the segment
/// rise in place of slope-times-tau), so ramps orders of magnitude steeper
/// than the lag lose nothing to floating-point cancellation.
pub fn first_order_final(points: &[(f64, f64)], initial: f64, tau: f64, stop: f64) -> f64 {
    let mut value = initial;
    let (mut t0, mut u0) = points[0];
    for &(t1, u1) in &points[1..] {
        if t0 >= stop {
            return value;
        }
        let segment_end = t1.min(stop);
        if segment_end > t0 {
            let dt = segment_end - t0;
            let delta = dt / tau;
            let rise = (u1 - u0) * (dt / (t1 - t0));
            let smoothing = (delta + (-delta).exp_m1()) / delta;
            value = u0 + (value - u0) * (-delta).exp() + rise * smoothing;
        }
        if segment_end >= stop {
            return value;
        }
        (t0, u0) = (t1, u1);
    }
    if stop > t0 {
        let delta = (stop - t0) / tau;
        value = u0 + (value - u0) * (-delta).exp();
    }
    value
}

/// Capacitor voltage of a series RLC circuit `t` seconds after an ideal
/// voltage step of `step` volts, in the underdamped regime.
pub fn series_rlc_step_capacitor_voltage(step: f64, r: f64, l: f64, c: f64, t: f64) -> f64 {
    let alpha = r / (2.0 * l);
    let natural_squared = 1.0 / (l * c);
    let damped = (natural_squared - alpha * alpha).sqrt();
    let envelope = (-alpha * t).exp();
    step * (1.0 - envelope * ((damped * t).cos() + alpha / damped * (damped * t).sin()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dialect_thermal_voltage_is_the_codata_value() {
        let vt = thermal_voltage(NOMINAL_CELSIUS);
        assert!((vt - 0.02586493).abs() < 1e-8, "vt(27C) = {vt}");
    }

    #[test]
    fn diode_bisection_solves_the_node_equation_to_machine_precision() {
        let v = diode_node_voltage(5.0, 1.0e3, 1.0e-14, 1.0, NOMINAL_CELSIUS);
        let n_vt = thermal_voltage(NOMINAL_CELSIUS);
        let residual = (5.0 - v) / 1.0e3 - 1.0e-14 * ((v / n_vt).exp() - 1.0);
        assert!(residual.abs() < 1e-12, "residual {residual}");
        assert!((0.5..0.8).contains(&v), "forward drop {v}");
    }

    #[test]
    fn saturation_current_scaling_is_monotone_and_nominal_at_nominal() {
        assert_eq!(
            diode_saturation_current_at(1e-14, 1.0, NOMINAL_CELSIUS),
            1e-14
        );
        assert!(diode_saturation_current_at(1e-14, 1.0, 85.0) > 1e-12);
        assert!(diode_saturation_current_at(1e-14, 1.0, -40.0) < 1e-16);
    }

    #[test]
    fn resistance_scaling_matches_the_polynomial() {
        assert_eq!(resistance_at(1000.0, 0.0, 0.0, 85.0), 1000.0);
        let r = resistance_at(1000.0, 2e-3, 1e-5, 85.0);
        assert_eq!(r, 1000.0 * (1.0 + 2e-3 * 58.0 + 1e-5 * 58.0 * 58.0));
    }

    #[test]
    fn the_first_order_march_reproduces_the_two_segment_closed_form() {
        // A unit ramp over [0, 2] with tau = 1, held flat afterwards: the
        // ramp response and the subsequent relaxation both have textbook
        // closed forms to compare against segment by segment.
        let points = [(0.0, 0.0), (2.0, 1.0), (10.0, 1.0)];
        let at_ramp_end = 1.0 - (1.0 - (-2.0_f64).exp()) / 2.0;
        let march = first_order_final(&points, 0.0, 1.0, 2.0);
        assert!((march - at_ramp_end).abs() < 1e-15, "ramp end {march}");
        let at_five = 1.0 + (at_ramp_end - 1.0) * (-3.0_f64).exp();
        let march = first_order_final(&points, 0.0, 1.0, 5.0);
        assert!((march - at_five).abs() < 1e-15, "mid-hold {march}");
        // Past the last breakpoint the drive holds its final level.
        let at_twenty = 1.0 + (at_ramp_end - 1.0) * (-18.0_f64).exp();
        let march = first_order_final(&points, 0.0, 1.0, 20.0);
        assert!((march - at_twenty).abs() < 1e-15, "held tail {march}");
    }

    #[test]
    fn the_first_order_march_survives_ramps_far_steeper_than_the_lag() {
        // A 1e6-volt drive rising in a nanosecond against a 1000-second lag:
        // the naive slope-times-tau arrangement loses ~13 digits here; the
        // stable arrangement must agree with the hand-built closed form.
        let points = [(0.0, 0.0), (1e-9, 1e6), (10.0, 1e6)];
        let tau = 1000.0_f64;
        let stop = 1e-3;
        let ramp_delta: f64 = 1e-9 / tau;
        let at_ramp_end = 1e6 * (ramp_delta + (-ramp_delta).exp_m1()) / ramp_delta;
        let tail = (stop - 1e-9) / tau;
        let reference = 1e6 + (at_ramp_end - 1e6) * (-tail).exp();
        let march = first_order_final(&points, 0.0, tau, stop);
        assert!(
            ((march - reference) / reference).abs() < 1e-12,
            "march {march} vs {reference}"
        );
    }

    #[test]
    fn pulse_breakpoints_agree_with_the_pointwise_waveform() {
        let pulse = Pulse {
            initial: 0.5,
            pulsed: 3.0,
            delay: 1e-4,
            rise: 1e-6,
            fall: 2e-6,
            width: 4e-4,
            period: 1e-3,
        };
        // Corner agreement is limited by the modulo arithmetic in
        // `value_at`: an ulp of the absolute time, divided by the edge
        // duration, scaled by the swing.
        for (time, level) in pulse.breakpoints(3.5e-3) {
            assert!(
                (pulse.value_at(time) - level).abs() < 1e-9,
                "breakpoint at {time}"
            );
        }
        assert_eq!(pulse.value_at(0.0), 0.5);
        assert_eq!(pulse.value_at(2e-4), 3.0);
    }

    #[test]
    fn the_rlc_step_response_starts_flat_and_settles_to_the_step() {
        let (step, r, l, c) = (5.0, 20.0, 1e-3, 1e-6);
        assert!(series_rlc_step_capacitor_voltage(step, r, l, c, 0.0).abs() < 1e-12);
        let settled = series_rlc_step_capacitor_voltage(step, r, l, c, 5e-3);
        assert!((settled - step).abs() < 1e-9, "settled {settled}");
        // The capacitor voltage rises quadratically from rest —
        // `step * omega0^2 * t^2 / 2`, about 2.5e-9 V here — because the
        // inductor current cannot change instantaneously.
        let early = series_rlc_step_capacitor_voltage(step, r, l, c, 1e-9);
        assert!(early > 0.0 && early < 5e-9, "early {early}");
    }
}
