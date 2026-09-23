use super::*;
use crate::abort_signal::CountingAbort;
use crate::config::SimulationConfig;
use crate::netlist::AnalysisCommand;

fn parse(line: &str) -> Result<QpacCard, String> {
    let net = Netlist::parse(&format!("QPAC card\n.param probe=31.25\n{line}\n.end\n"))
        .map_err(|e| e.to_string())?;
    let AnalysisCommand::Qpac(card) = &net.analyses[0] else {
        panic!("wrong card")
    };
    Ok(card.as_ref().clone())
}

const PATH: &str = "SOURCE=Xmix:V-one OUT=V(out/ref,ref-2) INLATTICE=(-1,0,2) OUTLATTICE=(1,-2,0)";

#[test]
fn qpac_card_round_trip_preserves_sweeps_arbitrary_tuples_and_all_controls() {
    for sweep in [
        "LIN 5 0 100",
        "DEC 3 1 1k",
        "OCT 2 2 128",
        "LIST=(-137,0,{probe},1k)",
    ] {
        let card = parse(&format!(".QPAC {sweep} {PATH} MAG=.002 PHASE=-137.12345678901234 SOLVER=KRYLOV KRYLOVRESTART=24 KRYLOVCYCLES=7 RELTOL=2.345678901234567e-11 IABSTOL=3e-14 VABSTOL=4e-10 FROM=QPSS")).unwrap();
        assert_eq!(card, parse(&card.to_spice()).unwrap());
        let request = QpacRequest::from_qpac_card(&card).unwrap();
        let restored =
            QpacRequest::from_qpac_card(&parse(&request.to_spice().unwrap()).unwrap()).unwrap();
        assert_eq!(request, restored);
        assert_eq!(request.input_source, "Xmix:V-one");
        assert_eq!(request.output_node, "out/ref");
        assert_eq!(request.output_ref, "ref-2");
        assert_eq!(request.solver.linear.restart, 24);
        assert_eq!(request.solver.linear.max_cycles, 7);
        assert_eq!(request.solver.current_absolute_tolerance, 3e-14);
        assert_eq!(request.solver.voltage_absolute_tolerance, 4e-10);
    }
    let default =
        QpacRequest::from_qpac_card(&parse(&format!(".QPAC LIST=(0) {PATH}")).unwrap()).unwrap();
    assert_eq!(default.solver, QuasiPeriodicAcConfig::default());
    assert_eq!((default.magnitude, default.phase_degrees), (1.0, 0.0));
    assert_eq!(default.offsets_hz, [0.0]);
    // Escaped punctuation never injects an extra card or keyword on writing.
    let mut names = default.clone();
    names.input_source = "X one:\"probe\\path".into();
    names.output_node = "out) MAG=99".into();
    assert_eq!(
        names,
        QpacRequest::from_qpac_card(&parse(&names.to_spice().unwrap()).unwrap()).unwrap()
    );
}

#[test]
fn qpac_card_rejects_dropped_options_invalid_grids_and_resource_overruns() {
    for suffix in [
        "MAG=0",
        "MAG=1 MAG=2",
        "PHASE=bad",
        "SOLVER=bad",
        "KRYLOVRESTART=65",
        "KRYLOVCYCLES=0",
        "LINEARTOL=1",
        "RELTOL=1e-9 LINEARTOL=1e-10",
        "IABSTOL=-1",
        "ABSTOL=1e-12 IABSTOL=1e-12",
        "VABSTOL=0",
        "FROM=HB",
        "SOURCE=V2",
        "DAMPING=1",
        "junk",
    ] {
        let result = parse(&format!(".QPAC LIST=(1,10) {PATH} {suffix}"))
            .and_then(|card| QpacRequest::from_qpac_card(&card).map_err(|e| e.to_string()));
        assert!(result.is_err(), "{suffix}");
    }
    for sweep in [
        "LIST=()",
        "LIST=(1,1)",
        "LIST=(2,1)",
        "LIST=(0,)",
        "LIST=(1 2)",
        "DEC 1 0 100",
        "LIN 0 0 1",
        "LIN 3 1 1",
        "LIN 3 10 1",
    ] {
        assert!(
            parse(&format!(".QPAC {sweep} {PATH}"))
                .and_then(|card| QpacRequest::from_qpac_card(&card).map_err(|e| e.to_string()))
                .is_err(),
            "{sweep}"
        );
    }
    for tuples in [
        "INLATTICE=(0) OUTLATTICE=(0)",
        "INLATTICE=(0,1) OUTLATTICE=(0,1,2)",
        "INLATTICE=(0,1.5) OUTLATTICE=(0,0)",
        "INLATTICE=(-2147483649,0) OUTLATTICE=(0,0)",
    ] {
        assert!(parse(&format!(".QPAC LIST=(1) SOURCE=V1 OUT=V(out) {tuples}")).is_err());
    }
    let card = parse(&format!(".QPAC DEC 100 1 1e100 {PATH}")).unwrap();
    let limits = ResourceLimits {
        max_analysis_points: 10,
        ..Default::default()
    };
    assert!(matches!(
        QpacRequest::from_qpac_card_with_abort(&card, &limits, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert!(matches!(
        QpacRequest::validate_qpac_card(&card, &limits),
        Err(SimulationError::ResourceLimit(_))
    ));
    let abort = CountingAbort::new(0);
    assert!(matches!(
        QpacRequest::from_qpac_card_with_abort(&card, &ResourceLimits::default(), &abort),
        Err(SimulationError::Aborted)
    ));
}

#[test]
fn qpac_card_executes_against_the_decks_retained_qpss_state() {
    let deck = Netlist::parse("native QPAC\nV1 in 0 DC 1\nR1 in out 1k\nC1 out 0 100n\n.QPSS 1k 1.4142135623730951k HARMS=1\n.QPAC LIN 3 0 100 SOURCE=V1 OUT=V(out) INLATTICE=(0,0) OUTLATTICE=(0,0) MAG=.3 PHASE=40\n.end\n").unwrap();
    let (AnalysisCommand::Qpss(qpss), AnalysisCommand::Qpac(qpac)) =
        (&deck.analyses[0], &deck.analyses[1])
    else {
        panic!("wrong cards")
    };
    let engine = Engine::new(SimulationConfig::default());
    let point = engine
        .run_qpss(&deck, QpssConfig::from_qpss_card(qpss).unwrap())
        .unwrap();
    for (sweep, expected_offsets) in [
        (qpac.as_ref().clone(), vec![0.0, 50.0, 100.0]),
        (parse(".QPAC DEC 2 10 800 SOURCE=V1 OUT=V(out) INLATTICE=(0,0) OUTLATTICE=(0,0) MAG=.3 PHASE=40").unwrap(), vec![10.0, 10.0 * 10_f64.sqrt(), 100.0, 100.0 * 10_f64.sqrt()]),
        (parse(".QPAC OCT 2 8 32 SOURCE=V1 OUT=V(out) INLATTICE=(0,0) OUTLATTICE=(0,0) MAG=.3 PHASE=40").unwrap(), vec![8.0, 8.0 * 2_f64.sqrt(), 16.0, 16.0 * 2_f64.sqrt(), 32.0]),
        (parse(".QPAC LIN 3 -100 100 SOURCE=V1 OUT=V(out) INLATTICE=(0,0) OUTLATTICE=(0,0) MAG=.3 PHASE=40").unwrap(), vec![-100.0, 0.0, 100.0]),
    ] {
        let limits = ResourceLimits::default();
        assert_eq!(QpacRequest::validate_qpac_card(&sweep, &limits).unwrap(), expected_offsets.len());
        let mut limited = limits;
        limited.max_analysis_points = expected_offsets.len() - 1;
        assert!(matches!(QpacRequest::validate_qpac_card(&sweep, &limited), Err(SimulationError::ResourceLimit(_))));
        let result = engine.run_qpac_card_from_qpss_with_abort(&deck, &sweep, &point, &NoAbort).unwrap();
        assert_eq!(result.metadata.request.offsets_hz.len(), expected_offsets.len());
        for (k, &f) in expected_offsets.iter().enumerate() {
            assert!((result.metadata.request.offsets_hz[k] - f).abs() <= 1e-12 * f.abs().max(1.0));
            let expected = Complex64::new(1.0, 0.0) / Complex64::new(1.0, std::f64::consts::TAU * f * 1e-4);
            assert!((result.output_transfer[k] - expected).norm() < 1e-10);
            assert!((result.output_response[k] - expected * Complex64::from_polar(0.3, 40.0_f64.to_radians())).norm() < 1e-10);
        }
    }
}
