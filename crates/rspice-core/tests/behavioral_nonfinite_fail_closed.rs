//! Behavioral B sources must never turn non-finite equations into zero-valued
//! sources. Every public analysis boundary reports the authored source and the
//! analysis coordinates that produced the invalid value.

use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("behavioral non-finite regression deck parses")
}

fn assert_behavioral_error(error: impl std::fmt::Display, kind: &str, name: &str) {
    let message = error.to_string().to_ascii_lowercase();
    assert!(
        message.contains(&format!("behavioral {kind} source '{name}'")),
        "error must identify the behavioral source: {message}"
    );
    assert!(
        message.contains("non-finite expression value"),
        "error must identify the invalid expression value: {message}"
    );
    assert!(
        message.contains("time") && message.contains("frequency"),
        "error must carry analysis coordinates: {message}"
    );
}

fn source_cases(expression: &str) -> [(String, &'static str, &'static str); 2] {
    [
        (
            format!("BVERR out 0 V={{{expression}}}"),
            "voltage",
            "bverr",
        ),
        (
            format!("BIERR out 0 I={{{expression}}}"),
            "current",
            "bierr",
        ),
    ]
}

#[test]
fn dc_op_rejects_nonfinite_voltage_and_current_source_values() {
    for (source, kind, name) in source_cases("1e308*1e308") {
        let deck =
            format!("behavioral {kind} DC non-finite value\n{source}\nRLOAD out 0 1k\n.OP\n.END\n");
        let error = Engine::new(SimulationConfig::default())
            .run_dc_op(&parse(&deck))
            .expect_err("a non-finite B-source DC equation must fail closed");
        assert_behavioral_error(error, kind, name);
    }
}

#[test]
fn ac_and_noise_reject_frequency_activated_nonfinite_voltage_and_current_sources() {
    for (source, kind, name) in source_cases("FREQ*1e308*1e308") {
        let deck = format!(
            "behavioral {kind} small-signal non-finite value\n{source}\nRLOAD out 0 1k\n.AC LIN 1 1k 1k\n.END\n"
        );
        let netlist = parse(&deck);
        let engine = Engine::new(SimulationConfig::default());

        let ac_error = engine
            .run_ac(&netlist, &[1.0e3])
            .expect_err("AC must reject a frequency-activated non-finite B source");
        assert_behavioral_error(ac_error, kind, name);

        let noise_error = engine
            .run_noise(&netlist, 1, &[1.0e3], 300.15)
            .expect_err("noise must reject a frequency-activated non-finite B source");
        assert_behavioral_error(noise_error, kind, name);
    }
}

#[test]
fn transient_rejects_time_activated_nonfinite_voltage_and_current_sources() {
    for (source, kind, name) in source_cases("TIME*1e308*1e308") {
        let deck = format!(
            "behavioral {kind} transient non-finite value\n{source}\nRLOAD out 0 1k\n.TRAN 1n 2n\n.END\n"
        );
        let error = Engine::new(SimulationConfig::default())
            .run_tran(&parse(&deck), 2.0e-9, 1.0e-9)
            .expect_err("transient must reject a time-activated non-finite B source");
        assert_behavioral_error(error, kind, name);
    }
}

#[test]
fn pss_rejects_time_activated_nonfinite_voltage_and_current_sources() {
    for (source, kind, name) in source_cases("TIME*1e308*1e308") {
        let deck = format!(
            "behavioral {kind} PSS non-finite value\n{source}\nRLOAD out 0 1k\nCLOAD out 0 1p\n.END\n"
        );
        let error = Engine::new(SimulationConfig::default())
            .run_pss(
                &parse(&deck),
                PssConfig::new(1.0e6)
                    .with_harmonics(2)
                    .with_points_per_period(8)
                    .with_tstab_periods(0),
            )
            .expect_err("PSS must reject a time-activated non-finite B source");
        assert_behavioral_error(error, kind, name);
    }
}
