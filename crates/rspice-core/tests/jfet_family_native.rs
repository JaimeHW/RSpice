//! Native classic JFET-family integration tests.
//!
//! JFET2 has its own focused coverage in `jfet2_native.rs`; this file pins the
//! classic level-1 JFET path and related `J`/`Z` family behavior to external
//! simulator oracles.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

#[test]
fn jfet_terminal_reports_include_gate_leakage_at_operating_point() {
    use rspice_core::engine::SpiceDialect;
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for level in [1, 2] {
            for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
                for series in ["", "RD=20 RS=10"] {
                    let netlist = Netlist::parse(&format!(
                        "JFET terminal report\nVD d 0 0\nVS s 0 0\nVG g 0 {}\nJ1 d g s jm\n.model jm {kind}(LEVEL={level} IS=1u {series})\n.end\n",
                        0.2 * polarity,
                    )).unwrap();
                    let (result, report) = Engine::new(SimulationConfig {
                        spice_dialect: dialect,
                        ..Default::default()
                    })
                    .run_dc_op_with_report(&netlist)
                    .unwrap();
                    let entry = report
                        .entries
                        .iter()
                        .find(|entry| entry.name == "J1")
                        .unwrap();
                    for (parameter, source) in [("id", "VD"), ("ig", "VG"), ("is", "VS")] {
                        let reported = entry
                            .params
                            .iter()
                            .find(|(name, _)| *name == parameter)
                            .unwrap_or_else(|| panic!("missing {parameter} in {entry:?}"))
                            .1;
                        let index = result
                            .branch_names
                            .iter()
                            .position(|name| name.eq_ignore_ascii_case(source))
                            .unwrap();
                        let expected = -result.branch_currents[index];
                        assert!(
                            (reported - expected).abs() < 1e-12 + expected.abs() * 1e-9,
                            "{dialect:?} {kind} L{level} {series}, {parameter}: {reported} vs {expected}"
                        );
                        assert_eq!(
                            result.try_dc_observable_named(&format!("J1:{parameter}")),
                            Some(reported)
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn jfet_terminal_reports_preserve_displacement_and_checkpoint_seams() {
    use rspice_core::engine::{
        SpiceDialect, TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode,
    };
    use rspice_core::numerics::integration::IntegrationMethod;
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for level in [1, 2] {
            for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
                for series in ["", "RD=20 RS=10"] {
                    let netlist = Netlist::parse(&format!(
                        "JFET current output\nVD d 0 PWL(0 0 1u {} 2u 0)\nVS s 0 0\nVG g 0 DC {} PWL(0 {} 1u {} 2u {})\nJ1 d g s jm\n.model jm {kind}(LEVEL={level} IS=1p CGS=1n CGD=2n CAPDS=3n {series})\n.print tran ID(J1) IG(J1) IS(J1) @J1[IGS] @J1[IGD] I(VD) I(VG) I(VS)\n.end\n",
                        0.2 * polarity, -polarity, -polarity, -0.5 * polarity, -polarity,
                    )).unwrap();
                    let engine = Engine::new(SimulationConfig {
                        spice_dialect: dialect,
                        integration_method: IntegrationMethod::BackwardEuler,
                        locked_time_grid: Some(std::sync::Arc::new(vec![
                            0.0, 0.5e-6, 1e-6, 1.5e-6, 2e-6,
                        ])),
                        ..Default::default()
                    });
                    let (full, checkpoints) = engine
                        .run_tran_checkpoint_schedule_with_startup_mode(
                            &netlist,
                            2e-6,
                            0.5e-6,
                            TransientStartupMode::OperatingPoint,
                            &[0.5e-6, 1e-6],
                        )
                        .unwrap();
                    for (parameter, source) in [("ID", "VD"), ("IG", "VG"), ("IS", "VS")] {
                        let reported = full
                            .try_device_op_waveform_named("J1", parameter)
                            .unwrap_or_else(|| panic!("missing {parameter}"));
                        let expected = full.try_branch_current_waveform_named(source).unwrap();
                        for (index, (actual, source_current)) in
                            reported.iter().zip(expected).enumerate()
                        {
                            assert!(
                                (actual + source_current).abs()
                                    < 1e-10 + source_current.abs() * 1e-6,
                                "{dialect:?} {kind} L{level} {series}, {parameter} at {}: {actual} vs {}",
                                full.time[index],
                                -source_current
                            );
                        }
                    }
                    for saved in checkpoints {
                        let checkpoint = TransientCheckpoint::from_bytes(
                            &saved
                                .checkpoint
                                .to_bytes(TransientCheckpointEncoding::Packed)
                                .unwrap(),
                        )
                        .unwrap();
                        let (resumed, _) = engine
                            .run_tran_resume(&netlist, &checkpoint, 2e-6, 0.5e-6)
                            .unwrap();
                        let offset = full
                            .time
                            .iter()
                            .position(|time| time.to_bits() == checkpoint.time.to_bits())
                            .unwrap();
                        assert_eq!(resumed.time, full.time[offset..]);
                        for parameter in ["ID", "IG", "IS", "IGS", "IGD"] {
                            let expected =
                                full.try_device_op_waveform_named("J1", parameter).unwrap();
                            let actual = resumed
                                .try_device_op_waveform_named("J1", parameter)
                                .unwrap();
                            assert_eq!(actual.len(), expected[offset..].len());
                            for (a, b) in actual.iter().zip(&expected[offset..]) {
                                assert_eq!(
                                    a.to_bits(),
                                    b.to_bits(),
                                    "{dialect:?} {kind} L{level} {series}, {parameter} seam at {}",
                                    checkpoint.time
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn tied_jfet_reports_keep_each_pin_charge_current() {
    use rspice_core::numerics::integration::IntegrationMethod;
    for (nodes, terminal_waveforms) in [
        ("d g d", [0, 1, 0]),
        ("d d s", [0, 0, 1]),
        ("d s s", [0, 1, 1]),
        ("d d d", [0, 0, 0]),
    ] {
        let nodes = nodes.replace('g', "s");
        let netlist = Netlist::parse(&format!(
            "tied JFET pin currents\nVD d 0 PWL(0 0 1u 0.1 2u 0)\nVS s 0 DC -1 PWL(0 -1 1u -0.5 2u -1)\nJ1 {nodes} jm\n.model jm NJF(BETA=1e-30 IS=0 CGS=1n CGD=2n M=0)\n.print tran ID(J1) IG(J1) IS(J1)\n.end\n",
        )).unwrap();
        let result = Engine::new(SimulationConfig {
            integration_method: IntegrationMethod::BackwardEuler,
            locked_time_grid: Some(std::sync::Arc::new(vec![0.0, 1e-6, 2e-6])),
            ..Default::default()
        })
        .run_tran(&netlist, 2e-6, 1e-6)
        .unwrap();
        let slopes = terminal_waveforms.map(|index| [0.1 / 1e-6, 0.5 / 1e-6][index]);
        let cqgs = 1e-9 * (slopes[1] - slopes[2]);
        let cqgd = 2e-9 * (slopes[1] - slopes[0]);
        for (parameter, expected) in [("ID", -cqgd), ("IG", cqgs + cqgd), ("IS", -cqgs)] {
            let current = result
                .try_device_op_waveform_named("J1", parameter)
                .unwrap();
            for (index, polarity) in [(1, 1.0), (2, -1.0)] {
                assert!(
                    (current[index] - polarity * expected).abs() < 2e-12,
                    "{nodes} {parameter} at {index}: {} vs {}",
                    current[index],
                    polarity * expected
                );
            }
        }
    }
}

#[test]
fn classic_jfet_temperature_mapped_ac_and_charge_match_ngspice46() {
    use rspice_core::engine::SpiceDialect;
    use rspice_core::numerics::integration::IntegrationMethod;
    // ngspice-46 AC current, divided by 2*pi*1 MHz: CGS+CGD at VGS=VGD=-1.
    // TNOM=50 exercises both sides of the nominal-to-reference mapping.
    for (temperature, nominal, capacitance) in [
        (-40.0, 27.0, 2.060_179_028_154_676e-9),
        (27.0, 27.0, 2.121_320_343_559_643e-9),
        (100.0, 27.0, 2.185_818_911_094_34e-9),
        (-40.0, 50.0, 2.044_034_522_607_255e-9),
        (100.0, 50.0, 2.163_254_513_623_347e-9),
    ] {
        for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
            for (ambient, instance) in [
                (temperature, String::new()),
                (27.0, format!("DTEMP={}", temperature - 27.0)),
                (27.0, format!("TEMP={temperature} DTEMP=10")),
            ] {
                let netlist = Netlist::parse(&format!(
                    "JFET temperature AC and charge\nVg gate 0 DC {} AC 1 PWL(0 {} 1u {} 2u {})\nJ1 0 gate 0 jm {instance}\n.model jm {kind}(BETA=1m VTO=-2 IS=1e-30 CGS=1n CGD=2n PB=1 FC=0.5 TNOM={nominal})\n.options TEMP={ambient}\n.end\n",
                    -polarity, -polarity, 0.75 * polarity, -polarity,
                )).unwrap();
                let engine = Engine::new(SimulationConfig {
                    spice_dialect: SpiceDialect::Ngspice,
                    integration_method: IntegrationMethod::BackwardEuler,
                    locked_time_grid: Some(std::sync::Arc::new(vec![0.0, 1e-6, 2e-6])),
                    ..Default::default()
                });
                let ac = engine.run_ac(&netlist, &[1e6]).unwrap();
                let branch = ac[0]
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("vg"))
                    .unwrap();
                let measured = -ac[0].currents[branch].im / (std::f64::consts::TAU * 1e6);
                assert!(
                    (measured - capacitance).abs() < 1e-12 * capacitance,
                    "{kind}, TEMP={ambient}, {instance}, TNOM={nominal}: {measured} vs {capacitance}",
                );
                if temperature == 100.0 && nominal == 50.0 {
                    // Independently recorded ngspice qgs+qgd at these two
                    // biases: -2.5467560155408902 nC and +3.095418536243149 nC.
                    let tran = engine.run_tran(&netlist, 2e-6, 1e-6).unwrap();
                    let current = tran.try_branch_current_waveform_named("vg").unwrap();
                    let mut charge = 0.0;
                    let mut reached_peak = false;
                    for (time, current) in tran.time.windows(2).zip(&current[1..]) {
                        charge -= current * (time[1] - time[0]);
                        if time[1] == 1e-6 {
                            assert!((charge - polarity * 5.642_174_551_784_039e-9).abs() < 2e-16);
                            reached_peak = true;
                        }
                    }
                    assert!(reached_peak);
                    assert!(charge.abs() < 1e-15);
                    assert_eq!(engine.convergence_quality().force_accepted_points, 0);
                }
            }
        }
    }
}

#[test]
fn jfet_capacitance_temperature_offsets_are_applied_once() {
    use rspice_core::device::Jfet;
    for mut device in [
        Jfet::njf("j1", 1, 2, 3),
        Jfet::njf("j1", 1, 2, 3).enable_xyce_jfet1_model(),
        Jfet::njf("j1", 1, 2, 3).enable_jfet2_model(),
        Jfet::njf("j1", 1, 2, 3).enable_xyce_jfet2_model(),
    ] {
        device.params.cgs = 1e-9;
        device.params.cgd = 2e-9;
        let reference = device.transient_capacitances(-0.1, -0.2, 323.15);
        for instance in [
            vec![("DTEMP".to_owned(), 23.0)],
            vec![("TEMP".to_owned(), 50.0), ("DTEMP".to_owned(), -70.0)],
        ] {
            let mapped = device.clone().with_instance_params(&instance);
            let actual = mapped.transient_capacitances(-0.1, -0.2, 300.15);
            assert_eq!(
                actual, reference,
                "{:?}: {instance:?}",
                device.params.channel_model
            );
        }
    }
}

#[test]
fn classic_jfet_capacitance_is_continuous_and_has_no_reverse_bias_floor() {
    let mut device = rspice_core::device::Jfet::njf("j1", 1, 2, 3);
    device.params.cgs = 1e-9;
    device.params.cgd = 2e-9;
    for grading in [0.0, 0.2, 0.5, 1.0, 1.5] {
        device.params.m = grading;
        for fc in [0.0, 0.3, 0.5, 0.9] {
            device.params.fc = fc;
            let knee = fc * device.params.pb;
            let below = device.capacitances(knee - 1e-9, knee - 1e-9);
            let above = device.capacitances(knee + 1e-9, knee + 1e-9);
            assert!((above.0 - below.0).abs() < 1e-7 * below.0);
            assert!((above.1 - below.1).abs() < 1e-7 * below.1);
        }
        let (actual, _) = device.capacitances(-1e8, -1e8);
        let expected = device.params.cgs * (1.0 + 1e8_f64).powf(-grading);
        assert!((actual - expected).abs() < 1e-13 * expected);
    }
}

#[test]
fn classic_jfet_transient_delivers_the_analytic_charge_on_every_step() {
    use rspice_core::engine::SpiceDialect;
    use rspice_core::numerics::integration::IntegrationMethod;

    // ngspice-46 jfetload.c: integrate the depletion Q(V), rather than
    // accumulating C(V_new) * delta_V, including crossing the forward knee.
    let charge = |voltage: f64| {
        if voltage < 0.5 {
            6e-9 * (1.0 - (1.0 - voltage).sqrt())
        } else {
            6e-9 * (1.0 - 0.5_f64.sqrt())
                + 3e-9 / 0.5_f64.powf(1.5)
                    * (0.25 * (voltage - 0.5) + (voltage * voltage - 0.25) / 4.0)
        }
    };
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::BestAvailable] {
        for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
            for (area, multiplicity) in [(1.0, 1.0), (2.0, 3.0)] {
                let waveform = [-2.0, -0.1, 0.4, 0.7, -0.2, -2.0]
                    .iter()
                    .enumerate()
                    .map(|(i, v)| format!("{} {}", i as f64 * 1e-6, polarity * v))
                    .collect::<Vec<_>>()
                    .join(" ");
                let netlist = Netlist::parse(&format!(
                    "JFET charge cycle\nVg gate 0 DC {} PWL({waveform})\nJ1 0 gate 0 jm {area} M={multiplicity}\n.model jm {kind}(BETA=1m VTO=-2 IS=0 CGS=1n CGD=2n PB=1 FC=0.5)\n.end\n",
                    -2.0 * polarity,
                )).unwrap();
                let engine = Engine::new(SimulationConfig {
                    spice_dialect: dialect,
                    integration_method: IntegrationMethod::BackwardEuler,
                    locked_time_grid: Some(std::sync::Arc::new(
                        (0..=5).map(|i| f64::from(i) * 1e-6).collect(),
                    )),
                    ..Default::default()
                });
                let result = engine.run_tran(&netlist, 5e-6, 1e-6).unwrap();
                let voltage = result.try_voltage_waveform_named("gate").unwrap();
                let current = result.try_branch_current_waveform_named("Vg").unwrap();
                let mut delivered = 0.0;
                for i in 1..result.time.len() {
                    let step_charge = -current[i] * (result.time[i] - result.time[i - 1]);
                    let expected = polarity
                        * area
                        * multiplicity
                        * (charge(polarity * voltage[i]) - charge(polarity * voltage[i - 1]));
                    assert!(
                        (step_charge - expected).abs() < 2e-16 + 2e-7 * expected.abs(),
                        "{dialect:?}, {kind}, area={area}, M={multiplicity}, t={}: got {step_charge}, expected {expected}",
                        result.time[i],
                    );
                    delivered += step_charge;
                }
                assert!(
                    delivered.abs() < 1e-15,
                    "a closed bias cycle must return its charge: {delivered}"
                );
                assert_eq!(engine.convergence_quality().force_accepted_points, 0);
            }
        }
    }
}

#[test]
fn tied_jfet_terminals_preserve_dc_ac_and_transient_at_large_scale() {
    use rspice_core::engine::SpiceDialect;
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        for kind in ["NJF", "PJF"] {
            for terminals in ["out out out", "0 0 0"] {
                let netlist = Netlist::parse(&format!(
                    "tied JFET\nI1 0 out DC 1 AC 1\nR1 out 0 1\nJ1 {terminals} jm\n.model jm {kind}(BETA=1e20 VTO=-1 IS=1e20 CGS=1e20 CGD=1e20)\n.end\n"
                )).unwrap();
                let dc = engine
                    .run_dc_op(&netlist)
                    .unwrap_or_else(|error| panic!("{dialect:?}, {kind}, {terminals}: {error}"));
                assert!((dc.try_voltage_named("out").unwrap() - 1.0).abs() < 1e-12);
                let ac = engine.run_ac(&netlist, &[1e6]).unwrap();
                assert!((ac[0].voltages[0] - rspice_core::Complex64::new(1.0, 0.0)).norm() < 1e-12);
                let tran = engine.run_tran(&netlist, 2e-9, 1e-9).unwrap();
                assert_eq!(tran.time.last().copied(), Some(2e-9));
                assert!(
                    tran.try_voltage_waveform_named("out")
                        .unwrap()
                        .iter()
                        .all(|v| (v - 1.0).abs() < 1e-12)
                );
                assert_eq!(engine.convergence_quality().force_accepted_points, 0);
            }
        }
    }
}

#[test]
fn tied_drain_source_preserves_gate_loading_independently_of_channel_scale() {
    use rspice_core::engine::SpiceDialect;
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let engine = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            locked_time_grid: Some(std::sync::Arc::new(
                (0..=8).map(|i| f64::from(i) * 0.25e-9).collect(),
            )),
            ..Default::default()
        });
        let solve = |beta| {
            let netlist = Netlist::parse(&format!(
                "tied JFET channel\nI1 0 out DC 1 PWL(0 1 2n 2) AC 1\nR1 out 0 1\nJ1 out 0 out jm\n.model jm NJF(BETA={beta} VTO=-2 IS=1e-14 CGS=1n CGD=2n)\n.end\n"
            )).unwrap();
            (
                engine.run_ac(&netlist, &[1e6]).unwrap()[0].voltages[0],
                engine.run_tran(&netlist, 2e-9, 0.25e-9).unwrap(),
            )
        };
        let (expected_ac, expected_tran) = solve(1.0);
        assert!(
            expected_ac.im.abs() > 1e-3,
            "gate charge must load the circuit"
        );
        let (actual_ac, actual_tran) = solve(1e20);
        assert!((actual_ac - expected_ac).norm() < 1e-12, "{dialect:?}");
        assert_eq!(actual_tran.time, expected_tran.time);
        for (actual, expected) in actual_tran
            .try_voltage_waveform_named("out")
            .unwrap()
            .iter()
            .zip(expected_tran.try_voltage_waveform_named("out").unwrap())
        {
            assert!(
                (actual - expected).abs() < 1e-12,
                "{dialect:?}: {actual} vs {expected}"
            );
        }
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }
}

fn xyce_pjfet_switch_deck() -> &'static str {
    "\
2N5144 PJFET Switching Speed Characteristic
Vin 3 0 pulse(12 0 10n 5n 5n 1u 1m)
Vds 4 0 -15
Rout 3 2 50
Rterm 2 0 50
Rload 4 1 500
J1 1 2 0 2N5114
.MODEL 2N5114 PJF
+        VTO = -5.288
+       BETA = 2.1897M
+     LAMBDA = 9.946M
+         RD = 22.042
+         RS = 22.042
+        CGS = 14.6595P
+        CGD = 14.6595P
+         PB = 1.40863
+         IS = 39.24F
+         KF = 0
+         AF = 1
+         FC = 0.5
.TRAN 0.5n 1.07u 1u .1n
.PRINT TRAN V(1) V(2) V(3)
.END
"
}

fn node_series<'a>(names: &[String], voltages: &'a [Vec<f64>], want: &str) -> &'a [f64] {
    let idx = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("missing {want} node in {:?}", names));
    &voltages[idx]
}

fn interpolate(time: &[f64], values: &[f64], target: f64) -> f64 {
    assert_eq!(time.len(), values.len(), "time and value vectors align");
    if target <= time[0] {
        return values[0];
    }
    for index in 1..time.len() {
        if time[index] >= target {
            let t0 = time[index - 1];
            let t1 = time[index];
            let y0 = values[index - 1];
            let y1 = values[index];
            let frac = if t1 == t0 {
                0.0
            } else {
                (target - t0) / (t1 - t0)
            };
            return y0 + frac * (y1 - y0);
        }
    }
    *values.last().expect("non-empty value vector")
}

#[test]
fn xyce_pjfet_switch_transient_matches_xyce710() {
    let netlist = Netlist::parse(xyce_pjfet_switch_deck()).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 1.07e-6, 0.1e-9)
        .expect("classic PJFET switch transient runs");

    let v1 = node_series(&result.node_names, &result.voltages, "1");
    let v2 = node_series(&result.node_names, &result.voltages, "2");
    let v3 = node_series(&result.node_names, &result.voltages, "3");

    // Xyce 7.10 regression oracle:
    // `PJFET_SWITCH/pjfet_tran.cir.prn`, selected dynamic and settled rows.
    let reference = [
        (1.00000000e-6, -2.49656419, -6.43755897e-11, 0.0, 3.0e-2),
        (1.01622408e-6, -1.99684488, 1.05032316, 2.93778066, 5.0e-2),
        (1.02000000e-6, -6.22583251, 5.35491369, 12.0, 5.0e-2),
        (1.04006746e-6, -14.9992918, 5.99996130, 12.0, 3.0e-2),
        (1.07000000e-6, -15.0, 6.0, 12.0, 3.0e-2),
    ];

    for (time, v1_ref, v2_ref, v3_ref, tol) in reference {
        let got_v1 = interpolate(&result.time, v1, time);
        let got_v2 = interpolate(&result.time, v2, time);
        let got_v3 = interpolate(&result.time, v3, time);
        assert!(
            (got_v1 - v1_ref).abs() < tol,
            "V(1) at {time:.8e}s: rspice={got_v1:.9e} xyce={v1_ref:.9e}"
        );
        assert!(
            (got_v2 - v2_ref).abs() < tol,
            "V(2) at {time:.8e}s: rspice={got_v2:.9e} xyce={v2_ref:.9e}"
        );
        assert!(
            (got_v3 - v3_ref).abs() < tol,
            "V(3) at {time:.8e}s: rspice={got_v3:.9e} xyce={v3_ref:.9e}"
        );
    }
}

#[test]
fn classic_jfet_transient_preserves_charge_when_c_phi_overflows() {
    use rspice_core::engine::SpiceDialect;
    use rspice_core::numerics::integration::IntegrationMethod;
    for (kind, p) in [("NJF", 1.0), ("PJF", -1.0)] {
        // C*Phi overflows, but Q=C*V and the companion conductance C/dt
        // are representable. A round trip also checks accepted charge history.
        let deck = Netlist::parse(&format!(
            "Junction range\nVx x 0 DC 0 PWL(0 0 1e12 {} 2e12 0)\nJ1 0 x 0 jm\n.model jm {kind}(IS=0 CGS=1e12 CGD=2e12 PB=1e300)\n.print tran V(x) I(Vx) IG(J1)\n.end\n", -0.5 * p
        )).unwrap();
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            integration_method: IntegrationMethod::BackwardEuler,
            locked_time_grid: Some(std::sync::Arc::new(vec![0.0, 1e12, 2e12])),
            ..Default::default()
        });
        let result = engine.run_tran(&deck, 2e12, 1e12).unwrap();
        assert_eq!(result.time, [0.0, 1e12, 2e12]);
        let current = result.try_branch_current_waveform_named("Vx").unwrap();
        let report = result.try_device_op_waveform_named("J1", "IG").unwrap();
        assert_eq!(current.len(), result.time.len());
        assert_eq!(report.len(), result.time.len());
        for (i, expected) in [(1, 1.5 * p), (2, -1.5 * p)] {
            assert!(
                (current[i] - expected).abs() < 1e-9,
                "{kind}: {} vs {expected}",
                current[i]
            );
            assert!((report[i] + current[i]).abs() < 1e-9);
        }
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }
}
