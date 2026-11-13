//! PSpice Type-I CHEBYSHEV controlled sources exercised through the real
//! parser, circuit builder, nonlinear operating point, complex AC engine, and
//! adaptive transient integrator.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn deck(filter: &str) -> String {
    format!(
        "* PSpice CHEBYSHEV source\n\
         vin in 0 dc 1 ac 1\n\
         efilter out 0 CHEBYSHEV {{V(in)}} = {filter}\n\
         rload out 0 1k\n\
         .op\n\
         .ac lin 1 1 1\n\
         .end\n"
    )
}

fn operating_point_voltage(source: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(source).expect("CHEBYSHEV deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point solves");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from operating point"));
    result.node_voltages[index]
}

fn ac_magnitude(source: &str, node: &str, frequency_hz: f64) -> f64 {
    let netlist = Netlist::parse(source).expect("CHEBYSHEV deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[frequency_hz])
        .expect("AC analysis solves");
    let index = result[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from AC result"));
    result[0].voltages[index].norm()
}

fn assert_pass_and_stop(source: &str, pass_edges: &[f64], stop_edges: &[f64]) {
    let pass_floor = 10.0f64.powf(-0.1 / 20.0);
    let stop_ceiling = 10.0f64.powf(-50.0 / 20.0);
    for edge in pass_edges {
        let response = ac_magnitude(source, "out", *edge);
        assert!(
            response >= pass_floor * (1.0 - 2.0e-4) && response <= 1.0 + 2.0e-4,
            "pass edge {edge} Hz response {response} is outside [{pass_floor}, 1]"
        );
    }
    for edge in stop_edges {
        let response = ac_magnitude(source, "out", *edge);
        assert!(
            response <= stop_ceiling * (1.0 + 2.0e-3),
            "stop edge {edge} Hz response {response} exceeds {stop_ceiling}"
        );
    }
}

#[test]
fn official_low_pass_source_meets_ripple_stop_and_dc_requirements() {
    let source = deck("LP (800Hz 1.2kHz) .1dB 50dB");
    assert_eq!(
        Netlist::parse(&source)
            .expect("CHEBYSHEV source parses")
            .pspice_chebyshev_source_count(),
        1
    );
    assert_pass_and_stop(&source, &[800.0], &[1_200.0]);
    let dc = operating_point_voltage(&source, "out");
    assert!((dc - 1.0).abs() < 1.0e-8, "odd-order LP DC gain: {dc}");
}

#[test]
fn official_high_pass_source_meets_ripple_stop_and_dc_requirements() {
    let source = deck("HP (1.2kHz 800Hz) .1dB 50dB");
    assert_pass_and_stop(&source, &[1_200.0], &[800.0]);
    let dc = operating_point_voltage(&source, "out");
    assert!(dc.abs() < 1.0e-8, "HP must reject DC: {dc}");
}

#[test]
fn official_band_pass_source_meets_all_four_edges() {
    let source = deck("BP (800Hz 1.2kHz 2kHz 3kHz) .1dB 50dB");
    assert_pass_and_stop(&source, &[1_200.0, 2_000.0], &[800.0, 3_000.0]);
}

#[test]
fn official_band_reject_source_uses_pspice_cutoff_order_and_meets_all_edges() {
    let source = deck("BR (800Hz 1.2kHz 3kHz 2kHz) .1dB 50dB");
    assert_pass_and_stop(&source, &[800.0, 3_000.0], &[1_200.0, 2_000.0]);
}

#[test]
fn current_output_form_uses_the_same_exact_filter_transfer() {
    let voltage_source = deck("LP (800Hz 1.2kHz) .1dB 50dB");
    let current_source = "* G CHEBYSHEV\n\
        vin in 0 ac 1\n\
        gfilter 0 out CHEBYSHEV {V(in)} = LP (800Hz 1.2kHz) .1dB 50dB\n\
        rload out 0 1\n\
        .end\n";
    for frequency in [100.0, 800.0, 1_200.0, 10_000.0] {
        let voltage_gain = ac_magnitude(&voltage_source, "out", frequency);
        let transconductance = ac_magnitude(current_source, "out", frequency);
        assert!(
            (voltage_gain - transconductance).abs() <= voltage_gain.max(1.0) * 1.0e-8,
            "E/G transfer mismatch at {frequency} Hz: {voltage_gain} vs {transconductance}"
        );
    }
}

#[test]
fn low_pass_step_response_runs_through_the_adaptive_transient_integrator() {
    let source = "* third-order CHEBYSHEV transient\n\
        vin in 0 pulse(0 1 0 1n 1n 1 2)\n\
        efilter out 0 CHEBYSHEV {V(in)} = LP (1kHz 5kHz) 1dB 40dB\n\
        rload out 0 1k\n\
        .tran 5u 5m\n\
        .end\n";
    let netlist = Netlist::parse(source).expect("transient CHEBYSHEV deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 5.0e-3, 5.0e-6)
        .expect("adaptive transient solves");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node exists");
    let final_value = result.voltages[index]
        .last()
        .copied()
        .expect("transient has samples");
    assert!(
        (final_value - 1.0).abs() < 5.0e-3,
        "odd-order LP step must settle to unity: {final_value}"
    );
}

#[test]
fn parameterized_filter_requirements_resolve_before_exact_design() {
    let source = "* parameterized CHEBYSHEV\n\
        .param FP=800 FS=1.2k RP=.1 RS=50\n\
        vin in 0 ac 1\n\
        efilter out 0 CHEBYSHEV {V(in)} = LP (FP FS) RP RS\n\
        rload out 0 1k\n\
        .end\n";
    assert_pass_and_stop(source, &[800.0], &[1_200.0]);
}

#[test]
fn malformed_filter_contracts_fail_closed_with_source_locations() {
    for filter in [
        "LP (1.2kHz 800Hz) .1dB 50dB",
        "HP (800Hz 1.2kHz) .1dB 50dB",
        "BP (1.2kHz 800Hz 2kHz 3kHz) .1dB 50dB",
        "BR (800Hz 1.2kHz 2kHz 3kHz) .1dB 50dB",
        "LP (800Hz 1.2kHz) 0dB 50dB",
        "LP (800Hz 1.2kHz) 1dB .5dB",
        "LP (800Hz) .1dB 50dB",
        "XX (800Hz 1.2kHz) .1dB 50dB",
        "LP (800frogs 1.2kHz) .1dB 50dB",
        "LP (800Hz 1.2kHz) .1watts 50dB",
        "LP (800Hz 1.2kHz) .1dB 50garbage",
    ] {
        let error = Netlist::parse(&deck(filter)).expect_err("invalid CHEBYSHEV source must fail");
        let message = error.to_string();
        assert!(
            message.contains("line 3") || message.contains("Line 3"),
            "error must retain source line for `{filter}`: {message}"
        );
    }
}
