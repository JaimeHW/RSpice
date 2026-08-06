//! Time-domain case families: transient accuracy, deterministic
//! reproducibility (the harness re-runs those decks three times and the
//! pinned series hashes force bit-identical waveforms), and mixed-signal
//! pulse-train filtering.
//!
//! Every driven deck uses a PULSE source, so the t=0 operating point is the
//! source's initial level and the oracle can march the exact piecewise-
//! linear response — finite rise and fall ramps included, no ideal-edge
//! approximation error. Second-order decks keep the edge four decades below
//! the oscillation period and take the ideal-step closed form at the ramp
//! midpoint. Every `.tran` pins an explicit step ceiling (the fourth
//! positional argument), so the local truncation error the tolerances must
//! absorb is bounded by the deck, not by a print-interval default.

use crate::capture::{CaseDraft, Expectation, Parameter, Probe};
use crate::families::physics::{Pulse, first_order_final, series_rlc_step_capacitor_voltage};

fn output_probe(expected: f64, absolute: &'static str, relative: &'static str) -> Expectation {
    Expectation::Succeeds(vec![Probe {
        name: "v(out)".to_owned(),
        unit: "V",
        expected,
        absolute_tolerance: absolute,
        relative_tolerance: relative,
    }])
}

fn pulse_parameters(pulse: &Pulse, unit: &'static str, stop: f64) -> Vec<Parameter> {
    vec![
        Parameter {
            name: "drive.initial",
            unit,
            value: pulse.initial,
        },
        Parameter {
            name: "drive.pulsed",
            unit,
            value: pulse.pulsed,
        },
        Parameter {
            name: "t.delay",
            unit: "s",
            value: pulse.delay,
        },
        Parameter {
            name: "t.fall",
            unit: "s",
            value: pulse.fall,
        },
        Parameter {
            name: "t.period",
            unit: "s",
            value: pulse.period,
        },
        Parameter {
            name: "t.rise",
            unit: "s",
            value: pulse.rise,
        },
        Parameter {
            name: "t.stop",
            unit: "s",
            value: stop,
        },
        Parameter {
            name: "t.width",
            unit: "s",
            value: pulse.width,
        },
    ]
}

/// A single-shot pulse: high for longer than any deck here runs.
fn step_pulse(initial: f64, pulsed: f64, delay: f64, edge: f64) -> Pulse {
    Pulse {
        initial,
        pulsed,
        delay,
        rise: edge,
        fall: edge,
        width: 1.0,
        period: 10.0,
    }
}

struct RcCase {
    id: &'static str,
    primary: &'static str,
    extra: Vec<&'static str>,
    repetitions: u64,
    pulse: Pulse,
    r: f64,
    c: f64,
    stop: f64,
    tmax: f64,
    absolute: &'static str,
    relative: &'static str,
}

fn rc_low_pass(case: RcCase) -> CaseDraft {
    let RcCase {
        id,
        primary,
        extra,
        repetitions,
        pulse,
        r,
        c,
        stop,
        tmax,
        absolute,
        relative,
    } = case;
    let expected = first_order_final(&pulse.breakpoints(stop), pulse.initial, r * c, stop);
    let mut parameters = pulse_parameters(&pulse, "V", stop);
    parameters.push(Parameter {
        name: "c1",
        unit: "F",
        value: c,
    });
    parameters.push(Parameter {
        name: "r1",
        unit: "Ohm",
        value: r,
    });
    CaseDraft {
        id: id.to_owned(),
        primary_category: primary,
        extra_categories: extra,
        deck: format!(
            "* pulse-driven rc low-pass\n\
             v1 in 0 {source}\n\
             r1 in out {r}\n\
             c1 out 0 {c}\n\
             .tran {tmax} {stop} 0 {tmax}\n\
             .end\n",
            source = pulse.spice(),
        ),
        parameters,
        temperature_celsius: 27.0,
        repetitions,
        expectation: output_probe(expected, absolute, relative),
    }
}

struct RlCase {
    id: &'static str,
    primary: &'static str,
    extra: Vec<&'static str>,
    repetitions: u64,
    pulse: Pulse,
    r: f64,
    l: f64,
    stop: f64,
    tmax: f64,
    relative: &'static str,
}

/// Series R into a grounded inductor; the probed node rides the inductor,
/// so a step decays as `V * exp(-t/tau)` with `tau = L/R`. The oracle
/// marches the inductor current exactly and converts at the final drive.
fn rl_decay(case: RlCase) -> CaseDraft {
    let RlCase {
        id,
        primary,
        extra,
        repetitions,
        pulse,
        r,
        l,
        stop,
        tmax,
        relative,
    } = case;
    let current_drive: Vec<(f64, f64)> = pulse
        .breakpoints(stop)
        .iter()
        .map(|&(time, level)| (time, level / r))
        .collect();
    let final_current = first_order_final(&current_drive, pulse.initial / r, l / r, stop);
    let expected = pulse.value_at(stop) - r * final_current;
    let mut parameters = pulse_parameters(&pulse, "V", stop);
    parameters.push(Parameter {
        name: "l1",
        unit: "H",
        value: l,
    });
    parameters.push(Parameter {
        name: "r1",
        unit: "Ohm",
        value: r,
    });
    CaseDraft {
        id: id.to_owned(),
        primary_category: primary,
        extra_categories: extra,
        deck: format!(
            "* pulse-driven rl decay\n\
             v1 in 0 {source}\n\
             r1 in out {r}\n\
             l1 out 0 {l}\n\
             .tran {tmax} {stop} 0 {tmax}\n\
             .end\n",
            source = pulse.spice(),
        ),
        parameters,
        temperature_celsius: 27.0,
        repetitions,
        expectation: output_probe(expected, "1e-9", relative),
    }
}

struct RlcCase {
    id: &'static str,
    primary: &'static str,
    extra: Vec<&'static str>,
    repetitions: u64,
    step: f64,
    delay: f64,
    r: f64,
    l: f64,
    c: f64,
    stop: f64,
    tmax: f64,
}

fn rlc_step(case: RlcCase) -> CaseDraft {
    let RlcCase {
        id,
        primary,
        extra,
        repetitions,
        step,
        delay,
        r,
        l,
        c,
        stop,
        tmax,
    } = case;
    let edge = 1e-8;
    let pulse = step_pulse(0.0, step, delay, edge);
    let expected = series_rlc_step_capacitor_voltage(step, r, l, c, stop - delay - edge / 2.0);
    let mut parameters = pulse_parameters(&pulse, "V", stop);
    parameters.push(Parameter {
        name: "c1",
        unit: "F",
        value: c,
    });
    parameters.push(Parameter {
        name: "l1",
        unit: "H",
        value: l,
    });
    parameters.push(Parameter {
        name: "r1",
        unit: "Ohm",
        value: r,
    });
    CaseDraft {
        id: id.to_owned(),
        primary_category: primary,
        extra_categories: extra,
        deck: format!(
            "* series rlc step response\n\
             v1 in 0 {source}\n\
             r1 in mid {r}\n\
             l1 mid out {l}\n\
             c1 out 0 {c}\n\
             .tran {tmax} {stop} 0 {tmax}\n\
             .end\n",
            source = pulse.spice(),
        ),
        parameters,
        temperature_celsius: 27.0,
        repetitions,
        expectation: output_probe(expected, "1e-6", "1e-3"),
    }
}

struct IntegratorCase {
    id: &'static str,
    primary: &'static str,
    extra: Vec<&'static str>,
    repetitions: u64,
    drive: f64,
    delay: f64,
    c: f64,
    stop: f64,
    tmax: f64,
}

/// A pulsed current source charging a capacitor with a 1 GOhm bleed: the
/// response is still exactly first-order (tau = RC ~ minutes), so the
/// near-perfect voltage ramp has a closed form with no approximation.
fn integrator(case: IntegratorCase) -> CaseDraft {
    let IntegratorCase {
        id,
        primary,
        extra,
        repetitions,
        drive,
        delay,
        c,
        stop,
        tmax,
    } = case;
    let bleed = 1e9;
    let pulse = step_pulse(0.0, drive, delay, 1e-9);
    let voltage_drive: Vec<(f64, f64)> = pulse
        .breakpoints(stop)
        .iter()
        .map(|&(time, level)| (time, level * bleed))
        .collect();
    let expected = first_order_final(&voltage_drive, 0.0, bleed * c, stop);
    let mut parameters = pulse_parameters(&pulse, "A", stop);
    parameters.push(Parameter {
        name: "c1",
        unit: "F",
        value: c,
    });
    parameters.push(Parameter {
        name: "r.bleed",
        unit: "Ohm",
        value: bleed,
    });
    CaseDraft {
        id: id.to_owned(),
        primary_category: primary,
        extra_categories: extra,
        deck: format!(
            "* current-source integrator with bleed resistance\n\
             i1 0 out {source}\n\
             r1 out 0 {bleed}\n\
             c1 out 0 {c}\n\
             .tran {tmax} {stop} 0 {tmax}\n\
             .end\n",
            source = pulse.spice(),
        ),
        parameters,
        temperature_celsius: 27.0,
        repetitions,
        expectation: output_probe(expected, "1e-9", "1e-6"),
    }
}

struct DividerCase {
    id: &'static str,
    pulse: Pulse,
    r1: f64,
    r2: f64,
    c: f64,
    stop: f64,
    tmax: f64,
    absolute: &'static str,
    relative: &'static str,
}

/// A pulse through a resistive divider into a capacitor: Thevenin reduction
/// makes it first-order with the drive scaled by the divider ratio.
fn divider_low_pass(case: DividerCase) -> CaseDraft {
    let DividerCase {
        id,
        pulse,
        r1,
        r2,
        c,
        stop,
        tmax,
        absolute,
        relative,
    } = case;
    let ratio = r2 / (r1 + r2);
    let tau = r1 * r2 / (r1 + r2) * c;
    let scaled: Vec<(f64, f64)> = pulse
        .breakpoints(stop)
        .iter()
        .map(|&(time, level)| (time, level * ratio))
        .collect();
    let expected = first_order_final(&scaled, pulse.initial * ratio, tau, stop);
    let mut parameters = pulse_parameters(&pulse, "V", stop);
    parameters.push(Parameter {
        name: "c1",
        unit: "F",
        value: c,
    });
    parameters.push(Parameter {
        name: "r1",
        unit: "Ohm",
        value: r1,
    });
    parameters.push(Parameter {
        name: "r2",
        unit: "Ohm",
        value: r2,
    });
    CaseDraft {
        id: id.to_owned(),
        primary_category: "mixed_signal",
        extra_categories: vec![],
        deck: format!(
            "* pulse through a resistive divider into a capacitor\n\
             v1 in 0 {source}\n\
             r1 in out {r1}\n\
             r2 out 0 {r2}\n\
             c1 out 0 {c}\n\
             .tran {tmax} {stop} 0 {tmax}\n\
             .end\n",
            source = pulse.spice(),
        ),
        parameters,
        temperature_celsius: 27.0,
        repetitions: 1,
        expectation: output_probe(expected, absolute, relative),
    }
}

pub fn drafts() -> Vec<CaseDraft> {
    let mut drafts = Vec::new();

    // Transient accuracy: RC charge curves probed at their final sample.
    for (ordinal, (level, r, c, delay, edge, stop, tmax, relative)) in [
        (5.0, 1.0e3, 1.0e-6, 1.0e-4, 1e-6, 5.2e-3, 5e-6, "1e-4"),
        (3.3, 4.7e3, 1.0e-7, 5.0e-5, 1e-6, 2.5e-3, 2e-6, "1e-4"),
        (12.0, 2.2e4, 1.0e-8, 2.0e-5, 5e-7, 1.2e-3, 1e-6, "1e-4"),
        // A mid-curve probe at ~3.1 tau, where the derivative is largest.
        (1.8, 1.0e2, 1.0e-5, 1.0e-4, 1e-6, 3.2e-3, 5e-6, "1e-3"),
    ]
    .into_iter()
    .enumerate()
    {
        drafts.push(rc_low_pass(RcCase {
            id: [
                "tran.rc-charge.001",
                "tran.rc-charge.002",
                "tran.rc-charge.003",
                "tran.rc-charge.004",
            ][ordinal],
            primary: "transient",
            extra: vec![],
            repetitions: 1,
            pulse: step_pulse(0.0, level, delay, edge),
            r,
            c,
            stop,
            tmax,
            absolute: "1e-9",
            relative,
        }));
    }

    // RC discharge: the deck starts charged (the pulse's initial level is
    // the operating point) and relaxes toward zero.
    for (id, level, r, c, delay, edge, stop, tmax) in [
        (
            "tran.rc-discharge.001",
            5.0,
            2.0e3,
            5.0e-7,
            2.0e-4,
            1e-6,
            1.2e-3,
            5e-6,
        ),
        (
            "tran.rc-discharge.002",
            9.0,
            1.0e4,
            5.0e-8,
            5.0e-5,
            5e-7,
            1.05e-3,
            2.5e-6,
        ),
    ] {
        drafts.push(rc_low_pass(RcCase {
            id,
            primary: "transient",
            extra: vec![],
            repetitions: 1,
            pulse: step_pulse(level, 0.0, delay, edge),
            r,
            c,
            stop,
            tmax,
            absolute: "1e-9",
            relative: "1e-3",
        }));
    }

    // RL decay probed across the inductor.
    for (id, level, r, l, delay, stop, tmax) in [
        ("tran.rl-step.001", 5.0, 1.0e2, 1.0e-2, 1.0e-5, 1.1e-4, 5e-7),
        (
            "tran.rl-step.002",
            2.5,
            5.0e1,
            2.5e-3,
            5.0e-6,
            1.05e-4,
            2.5e-7,
        ),
    ] {
        drafts.push(rl_decay(RlCase {
            id,
            primary: "transient",
            extra: vec![],
            repetitions: 1,
            pulse: step_pulse(0.0, level, delay, 1e-7),
            r,
            l,
            stop,
            tmax,
            relative: "1e-3",
        }));
    }

    // Underdamped series RLC ringing into its step target.
    drafts.push(rlc_step(RlcCase {
        id: "tran.rlc-underdamped.001",
        primary: "transient",
        extra: vec![],
        repetitions: 1,
        step: 1.0,
        delay: 1e-5,
        r: 20.0,
        l: 1e-3,
        c: 1e-6,
        stop: 3.1e-4,
        tmax: 5e-7,
    }));
    drafts.push(rlc_step(RlcCase {
        id: "tran.rlc-underdamped.002",
        primary: "transient",
        extra: vec![],
        repetitions: 1,
        step: 5.0,
        delay: 5e-6,
        r: 8.0,
        l: 1e-4,
        c: 2.5e-6,
        stop: 1.05e-4,
        tmax: 2e-7,
    }));

    // The near-perfect voltage ramp: trapezoidal integration is exact on
    // polynomials of this order, so the tolerance is a decade above
    // accumulation roundoff and four decades below every other family.
    drafts.push(integrator(IntegratorCase {
        id: "tran.integrator.001",
        primary: "transient",
        extra: vec![],
        repetitions: 1,
        drive: 1e-3,
        delay: 1e-5,
        c: 1e-6,
        stop: 1.01e-3,
        tmax: 5e-6,
    }));

    // Five full pulse periods, finishing on a decayed tail.
    drafts.push(rc_low_pass(RcCase {
        id: "tran.rc-pulse-train.001",
        primary: "transient",
        extra: vec![],
        repetitions: 1,
        pulse: Pulse {
            initial: 0.0,
            pulsed: 3.3,
            delay: 1e-4,
            rise: 1e-6,
            fall: 1e-6,
            width: 4e-4,
            period: 1e-3,
        },
        r: 2.5e3,
        c: 1e-7,
        stop: 5.05e-3,
        tmax: 5e-6,
        absolute: "1e-6",
        relative: "1e-3",
    }));

    // Deterministic reproducibility: the same transient physics at fresh
    // operating points, executed three times per evidence run. The pinned
    // series hashes make any run-to-run drift an admission failure.
    for (id, level, r, c, delay, edge, stop, tmax, relative) in [
        (
            "repro.rc-charge.001",
            2.0,
            1.5e3,
            1.0e-6,
            1.5e-4,
            1e-6,
            8.1e-3,
            5e-6,
            "1e-4",
        ),
        (
            "repro.rc-charge.002",
            7.5,
            3.3e3,
            2.2e-7,
            3.0e-5,
            1e-6,
            3.9e-3,
            3e-6,
            "1e-4",
        ),
        (
            "repro.rc-charge.003",
            0.9,
            4.7e4,
            1.0e-8,
            2.5e-5,
            5e-7,
            2.5e-3,
            2e-6,
            "1e-4",
        ),
    ] {
        drafts.push(rc_low_pass(RcCase {
            id,
            primary: "deterministic_reproducibility",
            extra: vec!["transient"],
            repetitions: 3,
            pulse: step_pulse(0.0, level, delay, edge),
            r,
            c,
            stop,
            tmax,
            absolute: "1e-9",
            relative,
        }));
    }
    drafts.push(rc_low_pass(RcCase {
        id: "repro.rc-discharge.001",
        primary: "deterministic_reproducibility",
        extra: vec!["transient"],
        repetitions: 3,
        pulse: step_pulse(4.0, 0.0, 8e-5, 1e-6),
        r: 6.8e3,
        c: 1e-7,
        stop: 1.1e-3,
        tmax: 2e-6,
        absolute: "1e-9",
        relative: "1e-3",
    }));
    for (id, level, r, l, delay, stop, tmax) in [
        (
            "repro.rl-step.001",
            3.0,
            7.5e1,
            7.5e-3,
            2.0e-5,
            1.7e-4,
            5e-7,
        ),
        (
            "repro.rl-step.002",
            10.0,
            2.0e2,
            5.0e-3,
            1.0e-5,
            6.0e-5,
            2e-7,
        ),
    ] {
        drafts.push(rl_decay(RlCase {
            id,
            primary: "deterministic_reproducibility",
            extra: vec!["transient"],
            repetitions: 3,
            pulse: step_pulse(0.0, level, delay, 1e-7),
            r,
            l,
            stop,
            tmax,
            relative: "1e-3",
        }));
    }
    drafts.push(rlc_step(RlcCase {
        id: "repro.rlc-underdamped.001",
        primary: "deterministic_reproducibility",
        extra: vec!["transient"],
        repetitions: 3,
        step: 2.0,
        delay: 1e-5,
        r: 10.0,
        l: 5e-4,
        c: 8e-7,
        stop: 2.6e-4,
        tmax: 2.5e-7,
    }));
    drafts.push(integrator(IntegratorCase {
        id: "repro.integrator.001",
        primary: "deterministic_reproducibility",
        extra: vec!["transient"],
        repetitions: 3,
        drive: 5e-4,
        delay: 2e-5,
        c: 2e-6,
        stop: 2.02e-3,
        tmax: 1e-5,
    }));
    drafts.push(rc_low_pass(RcCase {
        id: "repro.rc-pulse-train.001",
        primary: "deterministic_reproducibility",
        extra: vec!["transient"],
        repetitions: 3,
        pulse: Pulse {
            initial: 0.0,
            pulsed: 1.8,
            delay: 5e-5,
            rise: 2e-6,
            fall: 2e-6,
            width: 2e-4,
            period: 5e-4,
        },
        r: 1.8e3,
        c: 2.5e-7,
        stop: 3.05e-3,
        tmax: 5e-6,
        absolute: "1e-6",
        relative: "1e-3",
    }));
    // A settled tail: twelve time constants after the edge, both the
    // physics and the integrator's own memory of it have decayed, so the
    // claim tightens by an order of magnitude.
    drafts.push(rc_low_pass(RcCase {
        id: "repro.rc-settle.001",
        primary: "deterministic_reproducibility",
        extra: vec!["transient"],
        repetitions: 3,
        pulse: step_pulse(0.0, 6.0, 4e-5, 1e-6),
        r: 1.2e3,
        c: 1e-7,
        stop: 1.48e-3,
        tmax: 4e-6,
        absolute: "1e-9",
        relative: "1e-5",
    }));

    // Mixed signal: logic-rate square waves filtered by analog low-passes,
    // the settling of single events, and resistive summing of two sources.
    for (id, pulse, r, c, stop, tmax, absolute, relative) in [
        (
            "mixed.square-lp.001",
            Pulse {
                initial: 0.0,
                pulsed: 5.0,
                delay: 5e-5,
                rise: 1e-6,
                fall: 1e-6,
                width: 2.49e-4,
                period: 5e-4,
            },
            1.0e3,
            1e-7,
            2.05e-3,
            5e-6,
            "1e-6",
            "1e-3",
        ),
        (
            // A clock faster than the filter: the output rides the ripple
            // around the duty-cycle average.
            "mixed.square-lp.002",
            Pulse {
                initial: 0.0,
                pulsed: 3.3,
                delay: 2e-5,
                rise: 1e-6,
                fall: 1e-6,
                width: 4.9e-5,
                period: 1e-4,
            },
            2.0e3,
            1e-7,
            2.02e-3,
            2e-6,
            "1e-6",
            "1e-3",
        ),
        (
            // A clock much slower than the filter: the output tracks the
            // rails and the probe lands ten time constants into the high
            // phase.
            "mixed.square-lp.003",
            Pulse {
                initial: 0.0,
                pulsed: 2.5,
                delay: 1e-4,
                rise: 2e-6,
                fall: 2e-6,
                width: 9.96e-4,
                period: 2e-3,
            },
            5.0e2,
            1e-7,
            2.6e-3,
            2e-6,
            "1e-9",
            "1e-4",
        ),
        (
            "mixed.square-lp.004",
            Pulse {
                initial: 0.0,
                pulsed: 4.0,
                delay: 4e-5,
                rise: 1e-6,
                fall: 1e-6,
                width: 9.9e-5,
                period: 4e-4,
            },
            1.5e3,
            1e-7,
            3.24e-3,
            4e-6,
            "1e-6",
            "1e-3",
        ),
        (
            // A single event decayed ten time constants back toward zero:
            // the expected value is a quarter millivolt and the microvolt
            // absolute term carries the comparison.
            "mixed.settle-low.001",
            Pulse {
                initial: 0.0,
                pulsed: 5.0,
                delay: 5e-5,
                rise: 1e-6,
                fall: 1e-6,
                width: 3e-4,
                period: 1e-2,
            },
            8.0e2,
            1e-7,
            1.152e-3,
            4e-6,
            "1e-6",
            "1e-3",
        ),
    ] {
        drafts.push(rc_low_pass(RcCase {
            id,
            primary: "mixed_signal",
            extra: vec![],
            repetitions: 1,
            pulse,
            r,
            c,
            stop,
            tmax,
            absolute,
            relative,
        }));
    }
    drafts.push(rc_low_pass(RcCase {
        id: "mixed.settle-high.001",
        primary: "mixed_signal",
        extra: vec![],
        repetitions: 1,
        pulse: step_pulse(0.0, 1.2, 3e-5, 1e-6),
        r: 6.0e2,
        c: 1e-7,
        stop: 7.5e-4,
        tmax: 2.5e-6,
        absolute: "1e-9",
        relative: "1e-5",
    }));

    drafts.push(divider_low_pass(DividerCase {
        id: "mixed.divider-lp.001",
        pulse: step_pulse(0.0, 5.0, 5e-5, 1e-6),
        r1: 2.0e3,
        r2: 2.0e3,
        c: 2e-7,
        stop: 1.25e-3,
        tmax: 5e-6,
        absolute: "1e-9",
        relative: "1e-4",
    }));
    drafts.push(divider_low_pass(DividerCase {
        id: "mixed.divider-lp.002",
        pulse: Pulse {
            initial: 0.0,
            pulsed: 8.0,
            delay: 2e-5,
            rise: 1e-6,
            fall: 1e-6,
            width: 2e-4,
            period: 1e-2,
        },
        r1: 1.0e3,
        r2: 3.0e3,
        c: 1e-7,
        stop: 5.2e-4,
        tmax: 2e-6,
        absolute: "1e-6",
        relative: "1e-3",
    }));

    drafts.push(summer_case(SummerCase {
        id: "mixed.summer.001",
        first: Pulse {
            initial: 0.0,
            pulsed: 4.0,
            delay: 4e-5,
            rise: 1e-6,
            fall: 1e-6,
            width: 1.99e-4,
            period: 4e-4,
        },
        second: SecondSource::Pulse(Pulse {
            initial: 0.0,
            pulsed: 2.0,
            delay: 2.4e-4,
            rise: 1e-6,
            fall: 1e-6,
            width: 1.99e-4,
            period: 4e-4,
        }),
        r1: 1.0e3,
        r2: 1.0e3,
        c: 2.5e-7,
        stop: 2.44e-3,
        tmax: 4e-6,
    }));
    drafts.push(summer_case(SummerCase {
        id: "mixed.summer.002",
        first: Pulse {
            initial: 0.0,
            pulsed: 3.0,
            delay: 3e-5,
            rise: 1e-6,
            fall: 1e-6,
            width: 1.49e-4,
            period: 3e-4,
        },
        second: SecondSource::Dc(1.5),
        r1: 2.0e3,
        r2: 1.0e3,
        c: 1.5e-7,
        stop: 1.83e-3,
        tmax: 3e-6,
    }));

    drafts
}

/// The second input of a resistive summer: another pulse train, or a DC
/// bias that shifts the filtered square wave.
enum SecondSource {
    Pulse(Pulse),
    Dc(f64),
}

struct SummerCase {
    id: &'static str,
    first: Pulse,
    second: SecondSource,
    r1: f64,
    r2: f64,
    c: f64,
    stop: f64,
    tmax: f64,
}

/// Two sources through separate resistors into one capacitor node: the
/// node sees a first-order lag with `tau = C / (G1 + G2)` driven by the
/// conductance-weighted average of the inputs, which stays piecewise
/// linear on the union of both sources' breakpoints.
fn summer_case(case: SummerCase) -> CaseDraft {
    let SummerCase {
        id,
        first,
        second,
        r1,
        r2,
        c,
        stop,
        tmax,
    } = case;
    let (g1, g2) = (1.0 / r1, 1.0 / r2);
    let tau = c / (g1 + g2);
    let second_at = |time: f64| match &second {
        SecondSource::Pulse(pulse) => pulse.value_at(time),
        SecondSource::Dc(level) => *level,
    };
    let effective = |time: f64| (g1 * first.value_at(time) + g2 * second_at(time)) / (g1 + g2);

    let mut times: Vec<f64> = first
        .breakpoints(stop)
        .iter()
        .map(|&(time, _)| time)
        .collect();
    if let SecondSource::Pulse(pulse) = &second {
        times.extend(pulse.breakpoints(stop).iter().map(|&(time, _)| time));
    }
    times.sort_by(f64::total_cmp);
    times.dedup();
    let points: Vec<(f64, f64)> = times
        .into_iter()
        .map(|time| (time, effective(time)))
        .collect();
    let expected = first_order_final(&points, effective(0.0), tau, stop);

    let second_element = match &second {
        SecondSource::Pulse(pulse) => format!("v2 b 0 {}", pulse.spice()),
        SecondSource::Dc(level) => format!("v2 b 0 dc {level}"),
    };
    let mut parameters = pulse_parameters(&first, "V", stop);
    parameters.push(Parameter {
        name: "c1",
        unit: "F",
        value: c,
    });
    parameters.push(Parameter {
        name: "r1",
        unit: "Ohm",
        value: r1,
    });
    parameters.push(Parameter {
        name: "r2",
        unit: "Ohm",
        value: r2,
    });
    parameters.push(Parameter {
        name: "second.initial",
        unit: "V",
        value: match &second {
            SecondSource::Pulse(pulse) => pulse.initial,
            SecondSource::Dc(level) => *level,
        },
    });
    parameters.push(Parameter {
        name: "second.pulsed",
        unit: "V",
        value: match &second {
            SecondSource::Pulse(pulse) => pulse.pulsed,
            SecondSource::Dc(level) => *level,
        },
    });
    CaseDraft {
        id: id.to_owned(),
        primary_category: "mixed_signal",
        extra_categories: vec![],
        deck: format!(
            "* two sources resistively summed into one capacitor node\n\
             v1 a 0 {source}\n\
             {second_element}\n\
             r1 a out {r1}\n\
             r2 b out {r2}\n\
             c1 out 0 {c}\n\
             .tran {tmax} {stop} 0 {tmax}\n\
             .end\n",
            source = first.spice(),
        ),
        parameters,
        temperature_celsius: 27.0,
        repetitions: 1,
        expectation: output_probe(expected, "1e-6", "1e-3"),
    }
}
