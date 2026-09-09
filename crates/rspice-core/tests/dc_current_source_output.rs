use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::measure_signals::evaluate_dc_output_requests_with_abort;
use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;
use rspice_core::resource::ResourceLimits;

#[test]
fn tied_current_terminals_cannot_erase_other_sources_in_dc_or_transient() {
    use rspice_core::engine::{SimulationConfig, SpiceDialect};
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        for terminals in ["out out", "0 0"] {
            let netlist = Netlist::parse(&format!(
                "tied current terminals\nI1 0 out 1\nI2 {terminals} DC 1e100 PWL(0 1e100 1n -1e100 2n -1e100)\nR1 out 0 1\n.end\n"
            )).unwrap();
            assert!(
                (engine
                    .run_dc_op(&netlist)
                    .unwrap()
                    .try_voltage_named("out")
                    .unwrap()
                    - 1.0)
                    .abs()
                    < 1e-12
            );
            let result = engine.run_tran(&netlist, 2e-9, 1e-9).unwrap();
            assert_eq!(result.time.last().copied(), Some(2e-9));
            assert!(
                result
                    .try_voltage_waveform_named("out")
                    .unwrap()
                    .iter()
                    .all(|value| (value - 1.0).abs() < 1e-12)
            );
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

#[test]
fn waveform_ac_terms_preserve_dc_bias_and_explicit_overrides() {
    // ngspice 46 retains 0.65 V for all three waveforms with or without AC.
    // Test both polarities of the MNA source interface and both term orders.
    let engine = Engine::default();
    for (waveform, initial) in [
        ("SIN(.65 .05 1meg)", 0.65),
        ("SIN(.65 .05 1meg 0 0 90)", 0.70),
        ("PULSE(.65 .7 0 1n 1n 5n 10n)", 0.65),
        ("PWL(0 .65 20n .7)", 0.65),
    ] {
        for source in ["V1 out 0", "I1 0 out"] {
            for terms in [
                waveform.to_owned(),
                format!("{waveform} AC 2 90"),
                format!("AC 2 90 {waveform}"),
            ] {
                for (dc, expected) in [("", initial), (" DC 0", 0.0), (" DC=-.2", -0.2)] {
                    let deck = format!("waveform bias\n{source} {terms}{dc}\nR1 out 0 1\n.end\n");
                    let netlist = Netlist::parse(&deck).unwrap();
                    let dc = engine.run_dc_op(&netlist).unwrap();
                    let index = dc
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("out"))
                        .unwrap();
                    assert!(
                        (dc.node_voltages[index] - expected).abs() < 1e-12,
                        "{deck}: {dc:?}"
                    );
                    if terms.contains("AC") {
                        let ac = engine.run_ac(&netlist, &[1e3]).unwrap();
                        let index = ac[0]
                            .node_names
                            .iter()
                            .position(|name| name.eq_ignore_ascii_case("out"))
                            .unwrap();
                        assert!(
                            (ac[0].voltages[index] - rspice_core::Complex64::new(0.0, 2.0)).norm()
                                < 1e-12,
                            "{deck}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn adding_ac_programmatically_preserves_waveform_bias_and_transient_drive() {
    use rspice_core::engine::{extract_ac_value, extract_dc_value};
    use rspice_core::netlist::{ElementKind, SourceSpec};

    for source in ["V1 out 0", "I1 0 out"] {
        let mut netlist = Netlist::parse(&format!(
            "source edits\n{source} PWL(0 .65 20n .7)\nR1 out 0 1\n.end\n"
        ))
        .unwrap();
        let spec = match &mut netlist.elements[0].kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => spec,
            _ => unreachable!(),
        };
        *spec = spec.clone().with_ac(2.0, 0.25).with_ac(3.0, 0.5);
        assert_eq!(extract_dc_value(spec), 0.65);
        assert_eq!(extract_ac_value(spec), (3.0, 0.5));
        assert!(
            matches!(spec, SourceSpec::AcTransient { transient, .. } if matches!(transient.as_ref(), SourceSpec::Pwl { .. }))
        );
        let overridden = spec.clone().with_dc_value(-0.2);
        assert_eq!(extract_dc_value(&overridden), -0.2);
        assert_eq!(extract_ac_value(&overridden), (3.0, 0.5));

        let result = Engine::default().run_tran(&netlist, 20e-9, 1e-9).unwrap();
        let index = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        for (time, voltage) in result.time.iter().zip(&result.voltages[index]) {
            let expected = 0.65 + 0.05 * (time / 20e-9).min(1.0);
            assert!(
                (voltage - expected).abs() < 1e-12,
                "{source}: t={time}, v={voltage}, expected={expected}"
            );
        }
        assert!((result.time.last().unwrap() - 20e-9).abs() < 1e-20);
    }
}

#[test]
fn subcircuit_multiplicity_scales_waveform_bias_and_ac_excitation() {
    let netlist = Netlist::parse(
        "multiplied waveform\n.subckt cell out\nI1 0 out PWL(0 .65 20n .7) AC 2\n.ends\nX1 out cell M=3\nR1 out 0 1\n.end\n"
    ).unwrap();
    let engine = Engine::default();
    let dc = engine.run_dc_op(&netlist).unwrap();
    let index = dc
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!((dc.node_voltages[index] - 1.95).abs() < 1e-12);
    let ac = engine.run_ac(&netlist, &[1e3]).unwrap();
    let index = ac[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!((ac[0].voltages[index] - rspice_core::Complex64::new(6.0, 0.0)).norm() < 1e-12);
}

#[test]
fn signed_fractional_source_values_reach_the_dc_equations() {
    let netlist = Netlist::parse("signed sources\nV1 v 0 -.3\nI1 i 0 -.3m\nV2 p 0 +later\nRv v 0 1k\nRi i 0 1k\nRp p 0 1k\n.param later=.3\n.end\n").unwrap();
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    for (node, expected) in [("v", -0.3), ("i", 0.3), ("p", 0.3)] {
        let index = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap();
        assert!(
            (result.node_voltages[index] - expected).abs() < 1e-12,
            "{node}"
        );
    }
}

#[test]
fn swept_current_source_exposes_current_and_power_on_every_dc_row() {
    let netlist = Netlist::parse(
        "swept current-source output\n\
         I1 in 0 0\n\
         R1 in 0 1k\n\
         .dc I1 -1m 1m 1m\n\
         .print dc I(I1) P(I1) W(I1) V(in)\n\
         .end\n",
    )
    .expect("deck parses");
    let sweep = Engine::default()
        .run_dc_sweep(&netlist, "I1", -1.0e-3, 1.0e-3, 1.0e-3)
        .expect("current-source sweep solves");
    let columns = evaluate_dc_output_requests_with_abort(
        &netlist,
        &sweep,
        ResourceLimits::default(),
        &NoAbort,
    )
    .expect("all authored DC outputs project");

    assert_eq!(columns.len(), 4);
    assert_eq!(columns[0].0, "I(I1)");
    assert_eq!(columns[0].1, "current");
    assert_series(&columns[0].2, &[-1.0e-3, 0.0, 1.0e-3], 1.0e-15);
    assert_eq!(columns[1].0, "P(I1)");
    assert_series(&columns[1].2, &[-1.0e-3, 0.0, -1.0e-3], 1.0e-12);
    assert_eq!(columns[2].0, "W(I1)");
    assert_series(&columns[2].2, &columns[1].2, 1.0e-15);
    assert_eq!(columns[3].0, "V(in)");
    assert_series(&columns[3].2, &[1.0, 0.0, -1.0], 1.0e-9);
}

fn assert_series(actual: &[f64], expected: &[f64], tolerance: f64) {
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() <= tolerance,
            "row {index}: got {actual}, expected {expected} within {tolerance}"
        );
    }
}
