use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::measure_signals::evaluate_dc_output_requests_with_abort;
use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;
use rspice_core::resource::ResourceLimits;

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
