//! Frequency-domain families: AC small-signal transfer accuracy and
//! thermal-noise densities.
//!
//! The adapter reports magnitude (`vm`, volts) and phase (`vp`, degrees)
//! per node and the sweep's scalar is its final sample, so every oracle
//! here is the exact complex transfer function evaluated at the stop
//! frequency. The engine's sweep enumerator lands on the stop frequency to
//! within a few accumulated ulps, and the transfer functions' log-frequency
//! sensitivity is order one, so that abscissa error sits six decades below
//! the tolerances. Noise oracles are the 4kT closed forms at the dialect
//! temperature (300.15 K) with the CODATA Boltzmann constant, reported as
//! amplitude densities in V/Hz^0.5.

use std::f64::consts::{FRAC_PI_2, PI};

use crate::capture::{CaseDraft, Expectation, Parameter, Probe};
use crate::families::physics::{K_BOLTZMANN, KELVIN_AT_ZERO_CELSIUS, NOMINAL_CELSIUS};

/// First-order low-pass `1 / (1 + j*w*r*c)` as (magnitude, phase degrees).
fn rc_low_pass_response(frequency: f64, r: f64, c: f64) -> (f64, f64) {
    let x = 2.0 * PI * frequency * r * c;
    (1.0 / x.hypot(1.0), -x.atan().to_degrees())
}

/// First-order high-pass `j*w*r*c / (1 + j*w*r*c)`.
fn rc_high_pass_response(frequency: f64, r: f64, c: f64) -> (f64, f64) {
    let x = 2.0 * PI * frequency * r * c;
    (x / x.hypot(1.0), (FRAC_PI_2 - x.atan()).to_degrees())
}

/// Series RLC probed across the capacitor:
/// `1 / (1 - w^2*l*c + j*w*r*c)`.
fn rlc_capacitor_response(frequency: f64, r: f64, l: f64, c: f64) -> (f64, f64) {
    let omega = 2.0 * PI * frequency;
    let re = 1.0 - omega * omega * l * c;
    let im = omega * r * c;
    (1.0 / im.hypot(re), -im.atan2(re).to_degrees())
}

/// Series RLC probed across the resistor (the band-pass output):
/// `j*w*r*c / (1 - w^2*l*c + j*w*r*c)`.
fn rlc_resistor_response(frequency: f64, r: f64, l: f64, c: f64) -> (f64, f64) {
    let omega = 2.0 * PI * frequency;
    let re = 1.0 - omega * omega * l * c;
    let im = omega * r * c;
    (im / im.hypot(re), (FRAC_PI_2 - im.atan2(re)).to_degrees())
}

/// Thermal amplitude density `sqrt(4*k*T*r)` at the dialect temperature.
fn thermal_noise_density(resistance: f64) -> f64 {
    let temperature = NOMINAL_CELSIUS + KELVIN_AT_ZERO_CELSIUS;
    (4.0 * K_BOLTZMANN * temperature * resistance).sqrt()
}

fn transfer_probes(node: &str, magnitude: f64, phase_degrees: f64) -> Expectation {
    Expectation::Succeeds(vec![
        Probe {
            name: format!("vm({node})"),
            unit: "V",
            expected: magnitude,
            absolute_tolerance: "1e-12",
            relative_tolerance: "1e-9",
        },
        Probe {
            name: format!("vp({node})"),
            unit: "deg",
            expected: phase_degrees,
            absolute_tolerance: "1e-9",
            relative_tolerance: "1e-9",
        },
    ])
}

/// The sweep's numeric identity; the variation keyword lives in the deck,
/// which the fixture digest already covers.
fn sweep_parameters(points: f64, start: f64, stop: f64) -> Vec<Parameter> {
    vec![
        Parameter {
            name: "f.start",
            unit: "Hz",
            value: start,
        },
        Parameter {
            name: "f.stop",
            unit: "Hz",
            value: stop,
        },
        Parameter {
            name: "sweep.points",
            unit: "count",
            value: points,
        },
    ]
}

fn ohms(name: &'static str, value: f64) -> Parameter {
    Parameter {
        name,
        unit: "Ohm",
        value,
    }
}

fn farads(value: f64) -> Parameter {
    Parameter {
        name: "c1",
        unit: "F",
        value,
    }
}

pub fn drafts() -> Vec<CaseDraft> {
    let mut drafts = Vec::new();

    // RC low-pass magnitude and phase: at the pole, one decade above, and
    // two decades below, each with the exact transfer at the final sweep
    // frequency.
    for (id, r, c, variation, points, start, stop) in [
        (
            "ac.rc-low-pass.001",
            1.0e3,
            1.59e-7,
            "dec",
            10.0,
            10.0,
            1.0e3,
        ),
        (
            "ac.rc-low-pass.002",
            4.7e3,
            1.0e-7,
            "dec",
            20.0,
            10.0,
            1.0e4,
        ),
        (
            "ac.rc-low-pass.003",
            1.0e2,
            1.0e-8,
            "lin",
            101.0,
            10.0,
            1.0e3,
        ),
        ("ac.rc-low-pass.004", 2.2e3, 2.2e-7, "dec", 10.0, 3.0, 3.0e2),
        (
            "ac.rc-low-pass.005",
            5.6e4,
            1.0e-9,
            "dec",
            20.0,
            100.0,
            1.0e5,
        ),
        (
            "ac.rc-low-pass.006",
            3.3e2,
            1.0e-6,
            "lin",
            51.0,
            10.0,
            5.1e2,
        ),
        (
            "ac.rc-low-pass.007",
            7.5e3,
            3.3e-8,
            "dec",
            10.0,
            10.0,
            1.0e4,
        ),
    ] {
        let (magnitude, phase) = rc_low_pass_response(stop, r, c);
        let mut parameters = sweep_parameters(points, start, stop);
        parameters.push(farads(c));
        parameters.push(ohms("r1", r));
        drafts.push(CaseDraft {
            id: id.to_owned(),
            primary_category: "ac_small_signal",
            extra_categories: vec![],
            deck: format!(
                "* rc low-pass small-signal transfer\n\
                 v1 in 0 dc 0 ac 1\n\
                 r1 in out {r}\n\
                 c1 out 0 {c}\n\
                 .ac {variation} {points} {start} {stop}\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: transfer_probes("out", magnitude, phase),
        });
    }

    // RC high-pass: the series capacitor leads the phase.
    for (id, r, c, variation, points, start, stop) in [
        (
            "ac.rc-high-pass.001",
            1.0e4,
            1.59e-8,
            "dec",
            10.0,
            100.0,
            1.0e3,
        ),
        (
            "ac.rc-high-pass.002",
            2.2e4,
            1.0e-8,
            "dec",
            10.0,
            1.0,
            100.0,
        ),
        (
            "ac.rc-high-pass.003",
            4.7e3,
            4.7e-8,
            "dec",
            10.0,
            10.0,
            1.0e4,
        ),
        (
            "ac.rc-high-pass.004",
            1.0e5,
            1.0e-9,
            "lin",
            101.0,
            100.0,
            2.0e3,
        ),
    ] {
        let (magnitude, phase) = rc_high_pass_response(stop, r, c);
        let mut parameters = sweep_parameters(points, start, stop);
        parameters.push(farads(c));
        parameters.push(ohms("r1", r));
        drafts.push(CaseDraft {
            id: id.to_owned(),
            primary_category: "ac_small_signal",
            extra_categories: vec![],
            deck: format!(
                "* rc high-pass small-signal transfer\n\
                 v1 in 0 dc 0 ac 1\n\
                 c1 in out {c}\n\
                 r1 out 0 {r}\n\
                 .ac {variation} {points} {start} {stop}\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: transfer_probes("out", magnitude, phase),
        });
    }

    // A loaded divider low-pass: the Thevenin reduction scales the
    // magnitude by the divider ratio and filters with the parallel
    // resistance.
    {
        let (r1, r2, c, stop) = (2.0e3, 2.0e3, 1.0e-7, 2.0e3);
        let ratio = r2 / (r1 + r2);
        let parallel = r1 * r2 / (r1 + r2);
        let (magnitude, phase) = rc_low_pass_response(stop, parallel, c);
        let mut parameters = sweep_parameters(10.0, 100.0, stop);
        parameters.push(farads(c));
        parameters.push(ohms("r1", r1));
        parameters.push(ohms("r2", r2));
        drafts.push(CaseDraft {
            id: "ac.divider-low-pass.001".to_owned(),
            primary_category: "ac_small_signal",
            extra_categories: vec![],
            deck: format!(
                "* loaded divider low-pass small-signal transfer\n\
                 v1 in 0 dc 0 ac 1\n\
                 r1 in out {r1}\n\
                 r2 out 0 {r2}\n\
                 c1 out 0 {c}\n\
                 .ac dec 10 100 {stop}\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: transfer_probes("out", ratio * magnitude, phase),
        });
    }
    {
        let (r1, r2, c, stop) = (1.0e3, 9.0e3, 4.7e-8, 1.5e3);
        let ratio = r2 / (r1 + r2);
        let parallel = r1 * r2 / (r1 + r2);
        let (magnitude, phase) = rc_low_pass_response(stop, parallel, c);
        let mut parameters = sweep_parameters(10.0, 10.0, stop);
        parameters.push(farads(c));
        parameters.push(ohms("r1", r1));
        parameters.push(ohms("r2", r2));
        drafts.push(CaseDraft {
            id: "ac.divider-low-pass.002".to_owned(),
            primary_category: "ac_small_signal",
            extra_categories: vec![],
            deck: format!(
                "* loaded divider low-pass small-signal transfer\n\
                 v1 in 0 dc 0 ac 1\n\
                 r1 in out {r1}\n\
                 r2 out 0 {r2}\n\
                 c1 out 0 {c}\n\
                 .ac dec 10 10 {stop}\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: transfer_probes("out", ratio * magnitude, phase),
        });
    }

    // An inverting gain stage after the filter: the negative gain adds a
    // half-turn to the phase, exercising the wrapped-angle convention.
    {
        let (r, c, gain, stop) = (1.0e3, 1.0e-7, -4.0, 1.0e3);
        let (magnitude, phase) = rc_low_pass_response(stop, r, c);
        let mut parameters = sweep_parameters(10.0, 10.0, stop);
        parameters.push(farads(c));
        parameters.push(Parameter {
            name: "gain",
            unit: "V/V",
            value: gain,
        });
        parameters.push(ohms("r1", r));
        drafts.push(CaseDraft {
            id: "ac.vcvs-low-pass.001".to_owned(),
            primary_category: "ac_small_signal",
            extra_categories: vec![],
            deck: format!(
                "* inverting gain after an rc low-pass\n\
                 v1 in 0 dc 0 ac 1\n\
                 r1 in out {r}\n\
                 c1 out 0 {c}\n\
                 e1 amp 0 out 0 {gain}\n\
                 rload amp 0 10k\n\
                 .ac dec 10 10 {stop}\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: transfer_probes("amp", -gain * magnitude, phase + 180.0),
        });
    }

    // Series RLC probed across the capacitor, below and above resonance:
    // the second-order transfer peaks past unity below resonance and rolls
    // off at minus 40 dB per decade with the phase past a quarter turn
    // above it.
    for (id, r, l, c, points, start, stop) in [
        (
            "ac.rlc-low-pass.001",
            20.0,
            1.0e-3,
            1.0e-6,
            20.0,
            100.0,
            2.0e3,
        ),
        (
            "ac.rlc-low-pass.002",
            8.0,
            1.0e-4,
            2.5e-6,
            20.0,
            1.0e3,
            2.5e4,
        ),
        // Swept into the immediate vicinity of resonance, where the
        // magnitude peaks at the quality factor.
        (
            "ac.rlc-low-pass.003",
            10.0,
            1.0e-3,
            1.0e-6,
            20.0,
            100.0,
            5.033e3,
        ),
        (
            "ac.rlc-low-pass.004",
            5.0,
            1.0e-3,
            1.0e-6,
            20.0,
            100.0,
            5.033e3,
        ),
    ] {
        let (magnitude, phase) = rlc_capacitor_response(stop, r, l, c);
        let mut parameters = sweep_parameters(points, start, stop);
        parameters.push(farads(c));
        parameters.push(Parameter {
            name: "l1",
            unit: "H",
            value: l,
        });
        parameters.push(ohms("r1", r));
        drafts.push(CaseDraft {
            id: id.to_owned(),
            primary_category: "ac_small_signal",
            extra_categories: vec![],
            deck: format!(
                "* series rlc probed across the capacitor\n\
                 v1 in 0 dc 0 ac 1\n\
                 r1 in mid {r}\n\
                 l1 mid out {l}\n\
                 c1 out 0 {c}\n\
                 .ac dec {points} {start} {stop}\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: transfer_probes("out", magnitude, phase),
        });
    }

    // The band-pass output across the resistor, swept up to the immediate
    // vicinity of resonance: magnitude within a whisker of unity, phase
    // within a few millidegrees of zero.
    {
        let (r, l, c, stop) = (50.0, 1.0e-3, 1.0e-6, 5.033e3);
        let (magnitude, phase) = rlc_resistor_response(stop, r, l, c);
        let mut parameters = sweep_parameters(101.0, 1.0e3, stop);
        parameters.push(farads(c));
        parameters.push(Parameter {
            name: "l1",
            unit: "H",
            value: l,
        });
        parameters.push(ohms("r1", r));
        drafts.push(CaseDraft {
            id: "ac.rlc-band-pass.001".to_owned(),
            primary_category: "ac_small_signal",
            extra_categories: vec![],
            deck: format!(
                "* series rlc band-pass probed across the resistor\n\
                 v1 in 0 dc 0 ac 1\n\
                 l1 in mid {l}\n\
                 c1 mid out {c}\n\
                 r1 out 0 {r}\n\
                 .ac lin 101 1000 {stop}\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: transfer_probes("out", magnitude, phase),
        });
    }

    // Divider thermal noise: flat 4kT of the parallel resistance.
    for (id, r1, r2) in [
        ("noise.divider.001", 1.0e3, 1.0e3),
        ("noise.divider.002", 1.0e4, 2.2e3),
        ("noise.divider.003", 1.0e5, 1.0e5),
        ("noise.divider.004", 4.7e4, 1.0e3),
        ("noise.divider.005", 1.0e2, 1.0e2),
        ("noise.divider.006", 2.2e3, 3.3e3),
        ("noise.divider.007", 1.5e4, 1.5e4),
        ("noise.divider.008", 6.8e4, 1.2e4),
        ("noise.divider.009", 3.3e2, 4.7e2),
        ("noise.divider.010", 2.0e5, 2.0e5),
        ("noise.divider.011", 9.1e3, 1.1e3),
    ] {
        let parallel = r1 * r2 / (r1 + r2);
        let expected = thermal_noise_density(parallel);
        let mut parameters = sweep_parameters(3.0, 100.0, 1.0e4);
        parameters.push(ohms("r1", r1));
        parameters.push(ohms("r2", r2));
        drafts.push(CaseDraft {
            id: id.to_owned(),
            primary_category: "noise",
            extra_categories: vec![],
            deck: format!(
                "* resistor divider thermal noise\n\
                 v1 in 0 dc 0 ac 1\n\
                 r1 in out {r1}\n\
                 r2 out 0 {r2}\n\
                 .noise v(out) v1 lin 3 100 10000\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: noise_probe(expected),
        });
    }

    // The same dividers with a shunt capacitor: the density at the final
    // frequency is the flat floor shaped by the single real pole.
    for (id, r1, r2, c, variation, points, start, stop) in [
        (
            "noise.rc-filter.001",
            1.0e3,
            1.0e3,
            1.0e-7,
            "lin",
            5.0,
            100.0,
            1.0e4,
        ),
        (
            "noise.rc-filter.002",
            1.0e4,
            1.0e4,
            1.0e-6,
            "lin",
            5.0,
            10.0,
            1.0e3,
        ),
        (
            "noise.rc-filter.003",
            2.0e3,
            2.0e3,
            1.0e-8,
            "lin",
            5.0,
            10.0,
            1.0e3,
        ),
        (
            "noise.rc-filter.004",
            4.7e3,
            4.7e3,
            1.0e-7,
            "dec",
            10.0,
            10.0,
            1.0e5,
        ),
        (
            "noise.rc-filter.005",
            3.3e3,
            3.3e3,
            1.0e-7,
            "lin",
            5.0,
            100.0,
            964.0,
        ),
        (
            "noise.rc-filter.006",
            1.2e4,
            1.2e4,
            2.2e-8,
            "lin",
            5.0,
            100.0,
            2.0e3,
        ),
        (
            "noise.rc-filter.007",
            5.6e2,
            5.6e2,
            1.0e-6,
            "lin",
            5.0,
            10.0,
            5.0e3,
        ),
        (
            "noise.rc-filter.008",
            2.7e3,
            3.9e3,
            1.0e-7,
            "dec",
            10.0,
            10.0,
            1.0e4,
        ),
        (
            "noise.rc-filter.009",
            8.2e3,
            8.2e3,
            4.7e-8,
            "lin",
            9.0,
            50.0,
            1.2e3,
        ),
    ] {
        let parallel = r1 * r2 / (r1 + r2);
        let x = 2.0 * PI * stop * parallel * c;
        let expected = thermal_noise_density(parallel) / x.hypot(1.0);
        let mut parameters = sweep_parameters(points, start, stop);
        parameters.push(farads(c));
        parameters.push(ohms("r1", r1));
        parameters.push(ohms("r2", r2));
        drafts.push(CaseDraft {
            id: id.to_owned(),
            primary_category: "noise",
            extra_categories: vec![],
            deck: format!(
                "* divider thermal noise shaped by a shunt capacitor\n\
                 v1 in 0 dc 0 ac 1\n\
                 r1 in out {r1}\n\
                 r2 out 0 {r2}\n\
                 c1 out 0 {c}\n\
                 .noise v(out) v1 {variation} {points} {start} {stop}\n\
                 .end\n"
            ),
            parameters,
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: noise_probe(expected),
        });
    }

    drafts
}

fn noise_probe(expected: f64) -> Expectation {
    Expectation::Succeeds(vec![Probe {
        name: "onoise".to_owned(),
        unit: "V/Hz0.5",
        expected,
        absolute_tolerance: "1e-15",
        relative_tolerance: "1e-6",
    }])
}
