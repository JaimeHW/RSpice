//! Operating-point and nonlinear-device case families with closed-form
//! oracles.
//!
//! Every expected value here is derived from circuit algebra evaluated by
//! this module — never from the engine. Where a device equation has no
//! closed form (the diode exponential against a resistor), the oracle is
//! the root of the analytic node equation found by bisection to machine
//! precision, which is independent mathematics even though it iterates.
//!
//! Constants are the ngspice-dialect definitions the adapter pins
//! (CODATA k and q, 27°C reference = 300.15 K): the SPICE model equations
//! are *defined* in terms of these values, so using them is part of the
//! model specification, not an imitation of the engine.

use crate::capture::{CaseDraft, Expectation, Parameter, Probe};
use crate::families::physics::diode_node_voltage;

fn volts(name: &str, expected: f64, absolute: &'static str, relative: &'static str) -> Probe {
    Probe {
        name: format!("v({name})"),
        unit: "V",
        expected,
        absolute_tolerance: absolute,
        relative_tolerance: relative,
    }
}

/// Linear resistive circuits solve to machine precision; these tolerances
/// leave three orders of margin over double-precision residuals while
/// still rejecting any physics error.
fn linear_volts(name: &str, expected: f64) -> Probe {
    volts(name, expected, "1e-12", "1e-9")
}

pub fn drafts() -> Vec<CaseDraft> {
    let mut drafts = Vec::new();

    // Two-resistor dividers across value decades and polarities.
    for (ordinal, (supply, r1, r2)) in [
        (10.0, 1.0e3, 1.0e3),
        (5.0, 4.7e3, 3.3e2),
        (12.0, 1.0e4, 2.2e3),
        (2.5, 1.0e5, 1.0e5),
        (-5.0, 1.0e3, 9.0e3),
        (1.0, 1.0e6, 1.0e3),
    ]
    .into_iter()
    .enumerate()
    {
        let out = supply * r2 / (r1 + r2);
        let source_current = -(supply / (r1 + r2));
        drafts.push(CaseDraft {
            id: format!("op.divider.{:03}", ordinal + 1),
            primary_category: "operating_point",
            extra_categories: vec![],
            deck: format!(
                "* resistive divider operating point\n\
                 v1 in 0 dc {supply}\n\
                 r1 in out {r1}\n\
                 r2 out 0 {r2}\n\
                 .op\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "r1",
                    unit: "Ohm",
                    value: r1,
                },
                Parameter {
                    name: "r2",
                    unit: "Ohm",
                    value: r2,
                },
                Parameter {
                    name: "v1",
                    unit: "V",
                    value: supply,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![
                linear_volts("out", out),
                Probe {
                    name: "i(v1)".to_owned(),
                    unit: "A",
                    expected: source_current,
                    absolute_tolerance: "1e-15",
                    relative_tolerance: "1e-9",
                },
            ]),
        });
    }

    // Uniform ladders: tap k of n equal resistors sits at v * (n-k)/n.
    for (ordinal, stages) in [4_usize, 8].into_iter().enumerate() {
        let supply = 9.0;
        let mut deck = String::from("* uniform resistor ladder operating point\nv1 n0 0 dc 9\n");
        for stage in 0..stages {
            let top = if stage == 0 {
                "n0".to_owned()
            } else {
                format!("n{stage}")
            };
            let bottom = if stage + 1 == stages {
                "0".to_owned()
            } else {
                format!("n{}", stage + 1)
            };
            deck.push_str(&format!("r{} {top} {bottom} 2k\n", stage + 1));
        }
        deck.push_str(".op\n.end\n");
        let tap = |k: usize| supply * (stages - k) as f64 / stages as f64;
        drafts.push(CaseDraft {
            id: format!("op.ladder.{:03}", ordinal + 1),
            primary_category: "operating_point",
            extra_categories: vec![],
            deck,
            parameters: vec![
                Parameter {
                    name: "stages",
                    unit: "count",
                    value: stages as f64,
                },
                Parameter {
                    name: "rstage",
                    unit: "Ohm",
                    value: 2.0e3,
                },
                Parameter {
                    name: "v1",
                    unit: "V",
                    value: supply,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![
                linear_volts("n1", tap(1)),
                linear_volts(&format!("n{}", stages / 2), tap(stages / 2)),
            ]),
        });
    }

    // A Wheatstone bridge is two independent dividers; probe both midpoints.
    for (ordinal, (ra, rb, rc, rd)) in [(1.0e3, 1.0e3, 1.0e3, 1.0e3), (1.0e3, 2.0e3, 3.3e3, 1.2e3)]
        .into_iter()
        .enumerate()
    {
        let supply = 6.0;
        let left = supply * rb / (ra + rb);
        let right = supply * rd / (rc + rd);
        drafts.push(CaseDraft {
            id: format!("op.bridge.{:03}", ordinal + 1),
            primary_category: "operating_point",
            extra_categories: vec![],
            deck: format!(
                "* wheatstone bridge operating point\n\
                 v1 top 0 dc {supply}\n\
                 r1 top left {ra}\n\
                 r2 left 0 {rb}\n\
                 r3 top right {rc}\n\
                 r4 right 0 {rd}\n\
                 .op\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "r1",
                    unit: "Ohm",
                    value: ra,
                },
                Parameter {
                    name: "r2",
                    unit: "Ohm",
                    value: rb,
                },
                Parameter {
                    name: "r3",
                    unit: "Ohm",
                    value: rc,
                },
                Parameter {
                    name: "r4",
                    unit: "Ohm",
                    value: rd,
                },
                Parameter {
                    name: "v1",
                    unit: "V",
                    value: supply,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![
                linear_volts("left", left),
                linear_volts("right", right),
            ]),
        });
    }

    // Controlled sources: each of the four dependent-source elements with
    // a directly computable output.
    for (ordinal, gain) in [2.0, -4.0].into_iter().enumerate() {
        let input = 1.5; // divider halves 3 V
        drafts.push(CaseDraft {
            id: format!("op.vcvs.{:03}", ordinal + 1),
            primary_category: "operating_point",
            extra_categories: vec![],
            deck: format!(
                "* voltage-controlled voltage source operating point\n\
                 v1 in 0 dc 3\n\
                 r1 in mid 1k\n\
                 r2 mid 0 1k\n\
                 e1 out 0 mid 0 {gain}\n\
                 rload out 0 10k\n\
                 .op\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "gain",
                    unit: "V/V",
                    value: gain,
                },
                Parameter {
                    name: "v1",
                    unit: "V",
                    value: 3.0,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![linear_volts("out", gain * input)]),
        });
    }
    for (ordinal, transconductance) in [1.0e-3, 2.5e-4].into_iter().enumerate() {
        let control = 2.0; // full source voltage across the control divider top
        let load = 4.7e3;
        drafts.push(CaseDraft {
            id: format!("op.vccs.{:03}", ordinal + 1),
            primary_category: "operating_point",
            extra_categories: vec![],
            deck: format!(
                "* voltage-controlled current source operating point\n\
                 v1 in 0 dc {control}\n\
                 r1 in 0 1k\n\
                 g1 0 out in 0 {transconductance}\n\
                 rload out 0 {load}\n\
                 .op\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "gm",
                    unit: "A/V",
                    value: transconductance,
                },
                Parameter {
                    name: "rload",
                    unit: "Ohm",
                    value: load,
                },
                Parameter {
                    name: "v1",
                    unit: "V",
                    value: control,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![linear_volts(
                "out",
                transconductance * control * load,
            )]),
        });
    }
    {
        // CCVS and CCCS sense the source branch of a known divider current.
        let sense_current = 5.0 / 2.0e3;
        drafts.push(CaseDraft {
            id: "op.ccvs.001".to_owned(),
            primary_category: "operating_point",
            extra_categories: vec![],
            deck: "* current-controlled voltage source operating point\n\
                   v1 in 0 dc 5\n\
                   vsense in mid dc 0\n\
                   r1 mid 0 2k\n\
                   h1 out 0 vsense 1200\n\
                   rload out 0 8.2k\n\
                   .op\n\
                   .end\n"
                .to_owned(),
            parameters: vec![
                Parameter {
                    name: "r1",
                    unit: "Ohm",
                    value: 2.0e3,
                },
                Parameter {
                    name: "transresistance",
                    unit: "V/A",
                    value: 1.2e3,
                },
                Parameter {
                    name: "v1",
                    unit: "V",
                    value: 5.0,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![linear_volts("out", 1.2e3 * sense_current)]),
        });
        drafts.push(CaseDraft {
            id: "op.cccs.001".to_owned(),
            primary_category: "operating_point",
            extra_categories: vec![],
            deck: "* current-controlled current source operating point\n\
                   v1 in 0 dc 5\n\
                   vsense in mid dc 0\n\
                   r1 mid 0 2k\n\
                   f1 0 out vsense 40\n\
                   rload out 0 150\n\
                   .op\n\
                   .end\n"
                .to_owned(),
            parameters: vec![
                Parameter {
                    name: "current-gain",
                    unit: "A/A",
                    value: 40.0,
                },
                Parameter {
                    name: "r1",
                    unit: "Ohm",
                    value: 2.0e3,
                },
                Parameter {
                    name: "rload",
                    unit: "Ohm",
                    value: 150.0,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![linear_volts(
                "out",
                40.0 * sense_current * 150.0,
            )]),
        });
    }

    // Diode forward drops across saturation currents, emission
    // coefficients, and drive levels: the bisected Shockley node equation.
    for (ordinal, (supply, series, is, emission)) in [
        (5.0, 1.0e3, 1.0e-14, 1.0),
        (5.0, 1.0e4, 1.0e-14, 1.0),
        (3.3, 4.7e2, 1.0e-12, 1.0),
        (12.0, 1.0e3, 1.0e-15, 1.8),
        (1.2, 1.0e5, 1.0e-9, 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        let anode = diode_node_voltage(supply, series, is, emission, 27.0);
        drafts.push(CaseDraft {
            id: format!("op.nonlinear.diode.{:03}", ordinal + 1),
            primary_category: "nonlinear_devices",
            extra_categories: vec!["operating_point"],
            deck: format!(
                "* forward diode bias against a series resistor\n\
                 v1 in 0 dc {supply}\n\
                 r1 in anode {series}\n\
                 d1 anode 0 dmod\n\
                 .model dmod d(is={is} n={emission})\n\
                 .op\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "emission",
                    unit: "n",
                    value: emission,
                },
                Parameter {
                    name: "is",
                    unit: "A",
                    value: is,
                },
                Parameter {
                    name: "r1",
                    unit: "Ohm",
                    value: series,
                },
                Parameter {
                    name: "v1",
                    unit: "V",
                    value: supply,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![volts("anode", anode, "1e-9", "1e-6")]),
        });
    }

    // Level-1 MOSFETs where the square law is exact: saturation with
    // lambda = 0, plus one triode point from the closed-form quadratic.
    let beta = 2.0e-5 * 10.0; // kp * w / l
    for (ordinal, (gate, drain_resistor)) in
        [(2.0, 1.0e4), (2.5, 4.7e3), (3.0, 2.0e3), (1.5, 5.6e4)]
            .into_iter()
            .enumerate()
    {
        let supply = 5.0;
        let overdrive = gate - 1.0; // vto = 1
        let drain_current = 0.5 * beta * overdrive * overdrive;
        let drain = supply - drain_current * drain_resistor;
        assert!(drain > overdrive, "bias must stay in saturation");
        drafts.push(CaseDraft {
            id: format!("op.nonlinear.mos1-sat.{:03}", ordinal + 1),
            primary_category: "nonlinear_devices",
            extra_categories: vec!["operating_point"],
            deck: format!(
                "* level-1 mosfet saturation operating point\n\
                 vdd vdd 0 dc {supply}\n\
                 vg gate 0 dc {gate}\n\
                 rd vdd drain {drain_resistor}\n\
                 m1 drain gate 0 0 nmod w=10u l=1u\n\
                 .model nmod nmos(level=1 vto=1 kp=2e-5 lambda=0)\n\
                 .op\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "rd",
                    unit: "Ohm",
                    value: drain_resistor,
                },
                Parameter {
                    name: "vdd",
                    unit: "V",
                    value: supply,
                },
                Parameter {
                    name: "vgs",
                    unit: "V",
                    value: gate,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![volts("drain", drain, "1e-9", "1e-6")]),
        });
    }
    {
        // Triode: beta*((vov)*vd - vd^2/2) = (vdd - vd)/rd, the physical
        // root of a quadratic in vd. The load line is chosen so saturation
        // cannot carry it (the saturation drop would exceed the supply).
        let (supply, gate, drain_resistor) = (5.0, 4.5, 5.0e3);
        let overdrive = gate - 1.0;
        let a = 0.5 * beta;
        let b = -(beta * overdrive + 1.0 / drain_resistor);
        let c = supply / drain_resistor;
        let drain = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
        assert!(drain < overdrive, "bias must stay in triode");
        drafts.push(CaseDraft {
            id: "op.nonlinear.mos1-triode.001".to_owned(),
            primary_category: "nonlinear_devices",
            extra_categories: vec!["operating_point"],
            deck: format!(
                "* level-1 mosfet triode operating point\n\
                 vdd vdd 0 dc {supply}\n\
                 vg gate 0 dc {gate}\n\
                 rd vdd drain {drain_resistor}\n\
                 m1 drain gate 0 0 nmod w=10u l=1u\n\
                 .model nmod nmos(level=1 vto=1 kp=2e-5 lambda=0)\n\
                 .op\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "rd",
                    unit: "Ohm",
                    value: drain_resistor,
                },
                Parameter {
                    name: "vdd",
                    unit: "V",
                    value: supply,
                },
                Parameter {
                    name: "vgs",
                    unit: "V",
                    value: gate,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![volts("drain", drain, "1e-9", "1e-6")]),
        });
    }

    drafts
}
