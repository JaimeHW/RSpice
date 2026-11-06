use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::measure_signals::{
    evaluate_dc_output_requests_with_abort, evaluate_tran_output_requests_with_abort,
};
use rspice_core::config::{ExpressionDialect, SpiceDialect};
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{Netlist, NetlistParseOptions};

const DEVICES: &str = "\
V1 1 0 DC 1 PWL 0 0 1u 1
R1 1 2 2
C1 2 0 1u
L1 2 3 1m
R2 3 0 1
M1 3 1 0 0 NM L=2u W=10u
.MODEL NM NMOS LEVEL=1
";

fn xyce_deck(analysis: &str) -> Netlist {
    Netlist::parse_with_options(
        &format!("instance parameters\n{DEVICES}{analysis}.END\n"),
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .expect("Xyce deck parses")
}

fn xyce_engine() -> Engine {
    Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
}

fn assert_columns(columns: &[(String, &'static str, Vec<f64>)], expected: &[(&str, &[f64])]) {
    for (name, values) in expected {
        let (_, _, column) = columns
            .iter()
            .find(|(column, _, _)| column == name)
            .unwrap_or_else(|| panic!("no column {name}"));
        assert_eq!(column.len(), values.len(), "{name} rows");
        for (actual, expected) in column.iter().zip(values.iter()) {
            assert!(
                (actual - expected).abs() <= 1e-12 * expected.abs(),
                "{name}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn a_transient_prints_each_static_instance_parameter_at_every_point() {
    // Xyce 7.10 prints R1:R 2, C1:C 1u, L1:L 1m, V1:DCV0 1, M1:L 2u,
    // M1:W 10u and {R1:R*2} 4 at each of its 33 time points.
    let netlist = xyce_deck(".TRAN 0 1u\n.PRINT TRAN R1:R C1:C L1:L V1:DCV0 M1:L M1:W {R1:R*2}\n");
    let result = xyce_engine()
        .run_tran(&netlist, 1e-6, 1e-7)
        .expect("transient runs");
    let columns =
        evaluate_tran_output_requests_with_abort(&netlist, &result, Default::default(), &NoAbort)
            .expect("every instance parameter resolves");
    let points = result.time.len();
    assert!(points > 2);
    let constant = |value: f64| vec![value; points];
    assert_columns(
        &columns,
        &[
            ("R1:R", &constant(2.0)),
            ("C1:C", &constant(1e-6)),
            ("L1:L", &constant(1e-3)),
            ("V1:DCV0", &constant(1.0)),
            ("M1:L", &constant(2e-6)),
            ("M1:W", &constant(1e-5)),
            ("{R1:R*2}", &constant(4.0)),
        ],
    );
}

#[test]
fn a_dc_sweep_prints_each_static_instance_parameter_at_its_point() {
    // Xyce 7.10: V1:DCV0 follows the sweep (1, 2); the others are the
    // installed values, and {C1:C*2} is 2u.
    let netlist = xyce_deck(".DC V1 1 2 1\n.PRINT DC V1:DCV0 C1:C L1:L M1:L M1:W {C1:C*2}\n");
    let sweep = xyce_engine()
        .run_dc_sweep(&netlist, "V1", 1.0, 2.0, 1.0)
        .expect("sweep runs");
    let columns =
        evaluate_dc_output_requests_with_abort(&netlist, &sweep, Default::default(), &NoAbort)
            .expect("every instance parameter resolves");
    assert_columns(
        &columns,
        &[
            ("V1:DCV0", &[1.0, 2.0]),
            ("C1:C", &[1e-6, 1e-6]),
            ("L1:L", &[1e-3, 1e-3]),
            ("M1:L", &[2e-6, 2e-6]),
            ("M1:W", &[1e-5, 1e-5]),
            ("{C1:C*2}", &[2e-6, 2e-6]),
        ],
    );
}

#[test]
fn ac_and_noise_report_the_instance_parameters_of_their_own_bias_point() {
    // Xyce 7.10 prints R1:R 2, C1:C 1u, M1:L 2u and M1:W 10u on every row of
    // a .PRINT AC or .PRINT NOISE card, the values .PRINT DC prints.
    let netlist = xyce_deck(".AC DEC 1 1 10\n.PRINT AC VR(2) R1:R C1:C M1:L M1:W\n");
    let engine = xyce_engine();
    let (results, ac) = engine
        .run_ac_reporting_operating_point_with_abort(&netlist, &[1.0, 10.0], &NoAbort)
        .expect("AC runs");
    assert_eq!(results.len(), 2);
    let (results, noise) = engine
        .run_noise_named_reporting_operating_point_with_abort(
            &netlist,
            "2",
            None,
            "V1",
            &[1.0, 10.0],
            300.15,
            &NoAbort,
        )
        .expect("noise runs");
    assert_eq!(results.len(), 2);
    for report in [&ac, &noise] {
        let value = |name: &str| {
            report
                .iter()
                .find(|(reported, _)| reported.eq_ignore_ascii_case(name))
                .map(|(_, value)| *value)
                .unwrap_or_else(|| panic!("no {name} in {report:?}"))
        };
        for (name, expected) in [
            ("R1:R", 2.0),
            ("C1:C", 1e-6),
            ("M1:L", 2e-6),
            ("M1:W", 1e-5),
        ] {
            assert!(
                (value(name) - expected).abs() <= 1e-12 * expected,
                "{name}: {}",
                value(name)
            );
        }
        assert!(
            report
                .iter()
                .all(|(name, _)| name.contains(':') && !name.contains('(')),
            "{report:?}"
        );
    }
}
