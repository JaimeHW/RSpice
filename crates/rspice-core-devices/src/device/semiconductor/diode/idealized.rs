//! The idealized diode: three conduction regions separated by quadratic joins.
//!
//! A `.model` that names `Ron`, `Roff`, `Vfwd`, `Vrev` or `Rrev` selects it.
//! The device is off between its thresholds, conducts through `Ron` above
//! `Vfwd` and through `Rrev` below `-Vrev`. Both thresholds are signed, so a
//! negative `Vfwd` conducts below zero and a negative `Vrev` starts reverse
//! conduction above zero; forward conduction wins where the two overlap.
//! `Epsilon` and `Revepsilon` round each knee with a quadratic that matches
//! value and slope at both ends, and `Ilimit` and `Revilimit` bound a branch by
//! a hyperbolic tangent. `Rs`, `Is` and the other junction parameters do not
//! apply.

use crate::Value;

const MODEL_NAMES: [&str; 6] = ["RON", "ROFF", "VFWD", "VREV", "RREV", "RZEN"];

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct IdealizedDiode {
    /// Forward conductance, `1 / Ron`.
    gon: Value,
    /// Off conductance. `None` takes the circuit's GMIN.
    goff: Option<Value>,
    /// Reverse conductance, `1 / Rrev`.
    grev: Value,
    /// Anode voltage where forward conduction begins.
    forward_knee: Value,
    /// Anode voltage where reverse conduction begins, `-Vrev`.
    reverse_knee: Value,
    /// Width of the forward quadratic.
    epsilon: Value,
    /// Width of the reverse quadratic.
    reverse_epsilon: Value,
    /// Forward current ceiling, when the card gives one.
    forward_limit: Option<Value>,
    /// Reverse current ceiling, when the card gives one.
    reverse_limit: Option<Value>,
    /// Parallel devices: area and multiplicity scale the current.
    parallel: Value,
    /// Series devices: the junction sees `1 / series` of the bias.
    series: Value,
}

impl IdealizedDiode {
    /// The idealized law a model card selects, or `None` for a junction diode.
    pub(crate) fn from_model_params(
        params: &std::collections::HashMap<String, Value>,
    ) -> Option<Self> {
        if !MODEL_NAMES.iter().any(|name| params.contains_key(*name)) {
            return None;
        }
        let param = |name: &str| params.get(name).copied().filter(|v| v.is_finite());
        let positive = |name: &str| param(name).filter(|v| *v > 0.0);
        let gon = 1.0 / positive("RON").unwrap_or(1.0e-3);
        let grev = positive("RREV")
            .or_else(|| positive("RZEN"))
            .map_or(gon, |ohms| 1.0 / ohms);
        Some(Self {
            gon,
            goff: positive("ROFF").map(|ohms| 1.0 / ohms),
            grev,
            forward_knee: param("VFWD").unwrap_or(0.0),
            reverse_knee: param("VREV").map_or(Value::NEG_INFINITY, |v| -v),
            epsilon: param("EPSILON").map_or(0.0, |v| v.max(0.0)),
            reverse_epsilon: param("REVEPSILON").map_or(0.0, |v| v.max(0.0)),
            forward_limit: positive("ILIMIT"),
            reverse_limit: positive("REVILIMIT"),
            parallel: 1.0,
            series: 1.0,
        })
    }

    pub(crate) fn scale_parallel(&mut self, scale: Value) {
        self.parallel *= scale;
    }

    pub(crate) fn set_series(&mut self, series: Value) {
        self.series = series;
    }

    /// Terminal current from anode to cathode and its voltage derivative.
    pub(crate) fn current_and_conductance(self, bias: Value, gmin: Value) -> (Value, Value) {
        let goff = self.goff.unwrap_or(gmin);
        let junction = bias / self.series;
        let (current, conductance) = if junction >= self.forward_knee {
            conduction(
                junction,
                self.forward_knee,
                1.0,
                self.gon,
                self.epsilon,
                self.forward_limit,
                goff,
            )
        } else if junction > self.reverse_knee {
            (junction * goff, goff)
        } else {
            conduction(
                junction,
                self.reverse_knee,
                -1.0,
                self.grev,
                self.reverse_epsilon,
                self.reverse_limit,
                goff,
            )
        };
        (
            self.parallel * current,
            self.parallel * conductance / self.series,
        )
    }

    /// This law with the circuit's GMIN folded into its off conductance.
    pub(crate) fn resolved(self, gmin: Value) -> Self {
        Self {
            goff: Some(self.goff.unwrap_or(gmin)),
            ..self
        }
    }
}

/// Current and slope past one knee. `sign` is `+1` above the forward knee and
/// `-1` below the reverse knee.
fn conduction(
    bias: Value,
    knee: Value,
    sign: Value,
    g: Value,
    width: Value,
    limit: Option<Value>,
    goff: Value,
) -> (Value, Value) {
    let past = sign * (bias - knee);
    let Some(limit) = limit else {
        return if past < width {
            let rise = (g - goff) / width;
            (
                bias * goff + sign * 0.5 * past * past * rise,
                goff + past * rise,
            )
        } else {
            (
                knee * goff + sign * (0.5 * width * (g + goff) + (past - width) * g),
                g,
            )
        };
    };
    let tangent = entry_tangent(width, g, goff, limit);
    if past < width {
        let curve = (limit * tangent - width * goff) / (width * width);
        return (
            bias * goff + sign * curve * past * past,
            goff + 2.0 * curve * past,
        );
    }
    let tangent_past = (tangent.atanh() + (past - width) * g / limit).tanh();
    (
        knee * goff + sign * limit * tangent_past,
        g * (1.0 - tangent_past * tangent_past),
    )
}

/// `tanh` of the limited branch where the quadratic hands over to it: the root
/// that makes the join continuous in value and in slope.
fn entry_tangent(width: Value, g: Value, goff: Value, limit: Value) -> Value {
    if width <= 0.0 {
        return 0.0;
    }
    let drive = limit / width;
    ((-drive + (drive * drive + g * (g + goff)).sqrt()) / g).min(1.0 - Value::EPSILON)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn law(entries: &[(&str, Value)]) -> IdealizedDiode {
        let params: HashMap<String, Value> = entries
            .iter()
            .map(|(name, value)| (name.to_string(), *value))
            .collect();
        IdealizedDiode::from_model_params(&params).expect("idealized card")
    }

    /// LTspice 26.1.1 batch DC sweeps of a one-diode deck.
    #[test]
    fn current_matches_measured_law_in_every_region() {
        let on = [("RON", 10.0), ("ROFF", 1e3)];
        let cases: &[(Vec<(&str, Value)>, Value, Value)] = &[
            (
                [&on[..], &[("VFWD", 1.0), ("EPSILON", 0.5)]].concat(),
                0.5,
                5.0e-4,
            ),
            (
                [&on[..], &[("VFWD", 1.0), ("EPSILON", 0.5)]].concat(),
                1.25,
                1.25e-3 + 0.5 * 0.25 * 0.25 * (0.1 - 1e-3) / 0.5,
            ),
            // A negative Vfwd conducts below zero: measured 0.000425 at -.4 V.
            (
                [&on[..], &[("VFWD", -0.45), ("EPSILON", 0.15)]].concat(),
                -0.4,
                4.25e-4,
            ),
            // A negative Vrev starts reverse conduction above zero: measured -.398 at 0 V.
            (
                [&on[..], &[("VFWD", 1.0), ("VREV", -2.0), ("RREV", 5.0)]].concat(),
                0.0,
                -0.398,
            ),
            // Current limit through a quadratic join: measured .001495512 at .7 V.
            (
                vec![
                    ("RON", 1.0),
                    ("ROFF", 1e3),
                    ("VFWD", 0.5),
                    ("EPSILON", 0.2),
                    ("ILIMIT", 1e-3),
                ],
                0.7,
                1.495512e-3,
            ),
            // Reverse limit: measured -.003 at -5 V.
            (
                vec![
                    ("RON", 1.0),
                    ("ROFF", 1e3),
                    ("VFWD", 0.5),
                    ("VREV", 1.0),
                    ("REVEPSILON", 0.2),
                    ("REVILIMIT", 2e-3),
                ],
                -5.0,
                -3.0e-3,
            ),
        ];
        for (card, bias, expected) in cases {
            let (current, _) = law(card).current_and_conductance(*bias, 1e-12);
            let relative = (current - expected).abs() / expected.abs().max(1e-12);
            assert!(
                relative < 1e-6,
                "{card:?} at {bias}: {current} against {expected}"
            );
        }
    }

    #[test]
    fn conductance_is_the_slope_of_current_in_every_region() {
        let device = law(&[
            ("RON", 2.0),
            ("ROFF", 1e4),
            ("VFWD", 0.5),
            ("EPSILON", 0.2),
            ("ILIMIT", 5e-3),
            ("VREV", 1.5),
            ("RREV", 4.0),
            ("REVEPSILON", 0.1),
            ("REVILIMIT", 8e-3),
        ]);
        for bias in [-4.0, -1.55, -1.45, 0.1, 0.55, 0.7, 0.75, 3.0] {
            let step = 1e-7;
            let (_, slope) = device.current_and_conductance(bias, 1e-12);
            let (above, _) = device.current_and_conductance(bias + step, 1e-12);
            let (below, _) = device.current_and_conductance(bias - step, 1e-12);
            let numeric = (above - below) / (2.0 * step);
            assert!(
                (slope - numeric).abs() <= 1e-5 * (slope.abs() + 1e-6),
                "at {bias}: {slope} against {numeric}"
            );
        }
    }

    #[test]
    fn parallel_and_series_devices_scale_current_and_bias() {
        let mut device = law(&[("RON", 10.0), ("ROFF", 1e3), ("VFWD", 1.0)]);
        device.scale_parallel(3.0);
        device.set_series(2.0);
        let (current, conductance) = device.current_and_conductance(6.0, 1e-12);
        assert!((current - 3.0 * (1e-3 + 2.0 * 0.1)).abs() <= 1e-12);
        assert!((conductance - 3.0 * 0.1 / 2.0).abs() <= 1e-12);
    }

    #[test]
    fn a_junction_card_is_not_idealized() {
        let params: HashMap<String, Value> =
            HashMap::from([("IS".to_string(), 1e-14), ("EPSILON".to_string(), 0.1)]);
        assert!(IdealizedDiode::from_model_params(&params).is_none());
    }
}
