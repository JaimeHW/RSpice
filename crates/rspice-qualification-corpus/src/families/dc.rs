//! DC-sweep case families: linear sweeps, semiconductor model curves, and
//! temperature behavior, all with closed-form oracles evaluated at the
//! sweep's final point (a series measurement's scalar is its last sample).
//!
//! Endpoint float accumulation in the engine's sweep enumeration perturbs
//! the final abscissa by at most a few ulps; the relative tolerances here
//! sit far above that and far below any physics error.

use crate::capture::{CaseDraft, Expectation, Parameter, Probe};
use crate::families::physics::{diode_node_voltage, diode_saturation_current_at, resistance_at};

fn volts(name: &str, expected: f64, absolute: &'static str, relative: &'static str) -> Probe {
    Probe {
        name: format!("v({name})"),
        unit: "V",
        expected,
        absolute_tolerance: absolute,
        relative_tolerance: relative,
    }
}

pub fn drafts() -> Vec<CaseDraft> {
    let mut drafts = Vec::new();

    // Linear divider sweeps: the final output is the stop voltage through
    // the divider ratio, whatever the step raster.
    for (ordinal, (stop, step, r1, r2)) in [
        (5.0, 0.5, 1.0e3, 1.0e3),
        (10.0, 0.1, 2.2e3, 4.7e3),
        (-8.0, -0.25, 1.0e4, 1.0e4),
        (3.3, 0.05, 3.3e2, 1.2e3),
    ]
    .into_iter()
    .enumerate()
    {
        drafts.push(CaseDraft {
            id: format!("dc.divider.{:03}", ordinal + 1),
            primary_category: "dc_sweep",
            extra_categories: vec![],
            deck: format!(
                "* swept resistive divider\n\
                 v1 in 0 dc 0\n\
                 r1 in out {r1}\n\
                 r2 out 0 {r2}\n\
                 .dc v1 0 {stop} {step}\n\
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
                    name: "vstop",
                    unit: "V",
                    value: stop,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![volts(
                "out",
                stop * r2 / (r1 + r2),
                "1e-12",
                "1e-9",
            )]),
        });
    }

    // A swept controlled source: gain composes with the divider linearly.
    drafts.push(CaseDraft {
        id: "dc.vcvs.001".to_owned(),
        primary_category: "dc_sweep",
        extra_categories: vec![],
        deck: "* swept voltage-controlled voltage source\n\
               v1 in 0 dc 0\n\
               r1 in mid 1k\n\
               r2 mid 0 3k\n\
               e1 out 0 mid 0 -2.5\n\
               rload out 0 10k\n\
               .dc v1 0 4 0.1\n\
               .end\n"
            .to_owned(),
        parameters: vec![
            Parameter {
                name: "gain",
                unit: "V/V",
                value: -2.5,
            },
            Parameter {
                name: "vstop",
                unit: "V",
                value: 4.0,
            },
        ],
        temperature_celsius: 27.0,
        repetitions: 1,
        expectation: Expectation::Succeeds(vec![volts(
            "out",
            -2.5 * (4.0 * 0.75),
            "1e-12",
            "1e-9",
        )]),
    });

    // Current sweep into a resistor pair.
    drafts.push(CaseDraft {
        id: "dc.isweep.001".to_owned(),
        primary_category: "dc_sweep",
        extra_categories: vec![],
        deck: "* swept current source into a resistor chain\n\
               i1 0 out dc 0\n\
               r1 out mid 1.5k\n\
               r2 mid 0 500\n\
               .dc i1 0 2m 0.05m\n\
               .end\n"
            .to_owned(),
        parameters: vec![
            Parameter {
                name: "istop",
                unit: "A",
                value: 2.0e-3,
            },
            Parameter {
                name: "r1",
                unit: "Ohm",
                value: 1.5e3,
            },
            Parameter {
                name: "r2",
                unit: "Ohm",
                value: 5.0e2,
            },
        ],
        temperature_celsius: 27.0,
        repetitions: 1,
        expectation: Expectation::Succeeds(vec![
            volts("out", 2.0e-3 * 2.0e3, "1e-12", "1e-9"),
            volts("mid", 2.0e-3 * 5.0e2, "1e-12", "1e-9"),
        ]),
    });

    // Diode forward transfer curves: the final anode voltage is the
    // bisected Shockley node equation at the stop drive.
    for (ordinal, (stop, series, is, emission)) in [
        (5.0, 1.0e3, 1.0e-14, 1.0),
        (3.3, 4.7e2, 1.0e-12, 1.0),
        (12.0, 1.0e4, 1.0e-15, 1.5),
        (2.0, 1.0e5, 1.0e-9, 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        let anode = diode_node_voltage(stop, series, is, emission, 27.0);
        drafts.push(CaseDraft {
            id: format!("dc.semi.diode-forward.{:03}", ordinal + 1),
            primary_category: "semiconductor_models",
            extra_categories: vec!["dc_sweep", "nonlinear_devices"],
            deck: format!(
                "* diode forward transfer curve\n\
                 v1 in 0 dc 0\n\
                 r1 in anode {series}\n\
                 d1 anode 0 dmod\n\
                 .model dmod d(is={is} n={emission})\n\
                 .dc v1 0 {stop} {}\n\
                 .end\n",
                stop / 50.0
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
                    name: "vstop",
                    unit: "V",
                    value: stop,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![volts("anode", anode, "1e-9", "1e-6")]),
        });
    }

    // Level-1 MOSFET transfer curves ending in saturation: square law at
    // the final gate voltage.
    let beta = 2.0e-5 * 10.0;
    for (ordinal, (gate_stop, drain_resistor)) in [(3.0, 2.0e3), (2.5, 4.7e3), (2.0, 1.0e4)]
        .into_iter()
        .enumerate()
    {
        let supply = 5.0;
        let overdrive = gate_stop - 1.0;
        let drain_current = 0.5 * beta * overdrive * overdrive;
        let drain = supply - drain_current * drain_resistor;
        assert!(
            drain > overdrive,
            "transfer endpoint must stay in saturation"
        );
        drafts.push(CaseDraft {
            id: format!("dc.semi.mos1-transfer.{:03}", ordinal + 1),
            primary_category: "semiconductor_models",
            extra_categories: vec!["dc_sweep", "nonlinear_devices"],
            deck: format!(
                "* level-1 mosfet transfer curve\n\
                 vdd vdd 0 dc {supply}\n\
                 vg gate 0 dc 0\n\
                 rd vdd drain {drain_resistor}\n\
                 m1 drain gate 0 0 nmod w=10u l=1u\n\
                 .model nmod nmos(level=1 vto=1 kp=2e-5 lambda=0)\n\
                 .dc vg 0 {gate_stop} 0.05\n\
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
                    name: "vgstop",
                    unit: "V",
                    value: gate_stop,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![volts("drain", drain, "1e-9", "1e-6")]),
        });
    }

    // Level-1 MOSFET output curve ending in saturation with channel-length
    // modulation: id = beta/2 * vov^2 * (1 + lambda*vd), against the swept
    // drain supply directly (no load resistor, so vd is the source value).
    {
        let (gate, lambda, drain_stop) = (2.0, 0.02, 5.0);
        let overdrive: f64 = gate - 1.0;
        let drain_current = 0.5 * beta * overdrive * overdrive * (1.0 + lambda * drain_stop);
        assert!(drain_stop > overdrive);
        drafts.push(CaseDraft {
            id: "dc.semi.mos1-output.001".to_owned(),
            primary_category: "semiconductor_models",
            extra_categories: vec!["dc_sweep", "nonlinear_devices"],
            deck: format!(
                "* level-1 mosfet output curve with channel-length modulation\n\
                 vd drain 0 dc 0\n\
                 vg gate 0 dc {gate}\n\
                 vsense drain2 drain dc 0\n\
                 m1 drain2 gate 0 0 nmod w=10u l=1u\n\
                 .model nmod nmos(level=1 vto=1 kp=2e-5 lambda={lambda})\n\
                 .dc vd 0 {drain_stop} 0.1\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "lambda",
                    unit: "perV",
                    value: lambda,
                },
                Parameter {
                    name: "vdstop",
                    unit: "V",
                    value: drain_stop,
                },
                Parameter {
                    name: "vgs",
                    unit: "V",
                    value: gate,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![Probe {
                name: "i(vsense)".to_owned(),
                unit: "A",
                // SPICE branch convention: current into the sense source's
                // positive terminal, which here opposes the drain current.
                expected: -drain_current,
                absolute_tolerance: "1e-12",
                relative_tolerance: "1e-6",
            }]),
        });
    }

    // Temperature-coefficient resistor dividers at hot and cold corners:
    // r2 carries tc1/tc2, r1 is temperature-flat, so the ratio moves by
    // the closed-form polynomial.
    for (ordinal, (temperature, tc1, tc2)) in [
        (85.0, 2.0e-3, 0.0),
        (-40.0, 2.0e-3, 0.0),
        (125.0, 0.0, 1.5e-5),
        (85.0, 1.0e-3, 8.0e-6),
        (-40.0, 1.0e-3, 8.0e-6),
        (0.0, 3.9e-3, 0.0),
        (175.0, 2.0e-3, 1.0e-5),
        (60.0, -5.0e-4, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        let supply = 10.0;
        let r1 = 1.0e3;
        let r2 = resistance_at(1.0e3, tc1, tc2, temperature);
        drafts.push(CaseDraft {
            id: format!("dc.temp.resistor.{:03}", ordinal + 1),
            primary_category: "temperature_and_corners",
            extra_categories: vec!["dc_sweep"],
            deck: format!(
                "* temperature-coefficient divider at {temperature}c\n\
                 v1 in 0 dc 0\n\
                 r1 in out 1k\n\
                 r2 out 0 rmod 1k\n\
                 .model rmod r(tc1={tc1} tc2={tc2})\n\
                 .options temp={temperature}\n\
                 .dc v1 0 {supply} 0.5\n\
                 .end\n"
            ),
            parameters: vec![
                Parameter {
                    name: "tc1",
                    unit: "perC",
                    value: tc1,
                },
                Parameter {
                    name: "tc2",
                    unit: "perC2",
                    value: tc2,
                },
                Parameter {
                    name: "tcelsius",
                    unit: "degC",
                    value: temperature,
                },
                Parameter {
                    name: "vstop",
                    unit: "V",
                    value: supply,
                },
            ],
            temperature_celsius: temperature,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![volts(
                "out",
                supply * r2 / (r1 + r2),
                "1e-12",
                "1e-9",
            )]),
        });
    }

    // Diode forward drop across temperature: saturation-current scaling
    // with the standard level-1 defaults (eg=1.11, xti=3) plus the shifted
    // thermal voltage, still solved by bisection.
    for (ordinal, temperature) in [85.0, -40.0].into_iter().enumerate() {
        let (stop, series, is, emission) = (5.0, 1.0e3, 1.0e-14, 1.0);
        let is_at_t = diode_saturation_current_at(is, emission, temperature);
        let anode = diode_node_voltage(stop, series, is_at_t, emission, temperature);
        drafts.push(CaseDraft {
            id: format!("dc.temp.diode.{:03}", ordinal + 1),
            primary_category: "temperature_and_corners",
            extra_categories: vec!["dc_sweep", "semiconductor_models"],
            deck: format!(
                "* diode forward transfer at {temperature}c\n\
                 v1 in 0 dc 0\n\
                 r1 in anode {series}\n\
                 d1 anode 0 dmod\n\
                 .model dmod d(is={is} n={emission})\n\
                 .options temp={temperature}\n\
                 .dc v1 0 {stop} 0.1\n\
                 .end\n"
            ),
            parameters: vec![
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
                    name: "tcelsius",
                    unit: "degC",
                    value: temperature,
                },
                Parameter {
                    name: "vstop",
                    unit: "V",
                    value: stop,
                },
            ],
            temperature_celsius: temperature,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![volts("anode", anode, "1e-6", "1e-4")]),
        });
    }

    drafts
}
