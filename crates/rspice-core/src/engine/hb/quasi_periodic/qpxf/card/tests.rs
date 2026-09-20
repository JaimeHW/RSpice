//! Native configuration round trips, bounded authoring and a real QPSS-dependent run.
use super::*;
use crate::abort_signal::CountingAbort;
use crate::netlist::AnalysisCommand;
fn parse(line: &str) -> Result<QpxfCard, String> {
    let net = Netlist::parse(&format!("QPXF card\n.param probe=31.25\n{line}\n.end\n"))
        .map_err(|e| e.to_string())?;
    let AnalysisCommand::Qpxf(card) = &net.analyses[0] else {
        panic!("wrong card")
    };
    Ok(card.as_ref().clone())
}
const PATH: &str = "SOURCES=(Xmix:V-one,Iprobe) OUT=V(out/ref,ref-2) INLATTICES=((-1,0,2),(1,-2,0)) OUTLATTICE=(1,-2,0)";
#[test]
fn qpxf_card_round_trip_preserves_every_control_and_selection() {
    for sweep in [
        "LIN 5 -100 100",
        "LIN 3 0 100",
        "DEC 3 1 1k",
        "OCT 2 2 128",
        "LIST=(-137,0,{probe},1k)",
    ] {
        let card = parse(&format!(".QPXF {sweep} {PATH} AXIS=OFFSET SOLVER=KRYLOV KRYLOVRESTART=24 KRYLOVCYCLES=7 RELTOL=2.345678901234567e-11 GROUPDELAY=YES GDFLOOR=3.21e-14 FROM=QPSS")).unwrap();
        assert_eq!(card, parse(&card.to_spice()).unwrap());
        let request = QpxfRequest::from_qpxf_card(&card).unwrap();
        assert_eq!(
            request,
            QpxfRequest::from_qpxf_card(&parse(&request.to_spice().unwrap()).unwrap()).unwrap()
        );
        assert_eq!(request.frequency_axis, QpxfFrequencyAxis::Offset);
        assert_eq!(request.linear.restart, 24);
        assert_eq!(request.linear.max_cycles, 7);
        assert!(request.group_delay);
        assert_eq!(request.group_delay_magnitude_floor, 3.21e-14);
        assert_eq!(
            request.input_sources,
            QpxfSources::Named(vec!["Xmix:V-one".into(), "Iprobe".into()])
        );
    }
    let default =
        QpxfRequest::from_qpxf_card(&parse(".QPXF LIST=(0) OUT=I(L1) OUTLATTICE=(0,0)").unwrap())
            .unwrap();
    assert_eq!(default.frequency_axis, QpxfFrequencyAxis::Output);
    assert_eq!(default.input_sources, QpxfSources::AllIndependent);
    assert_eq!(default.input_lattices, QpxfInputLattices::AllRetained);
    assert_eq!(default.linear, QuasiPeriodicLinearConfig::default());
    assert!(!default.group_delay);
    for selection in [
        "SOURCE=Iprobe MAXORDERS=(2,0)",
        "SOURCES=ALL INLATTICES=ALL",
        "SOURCE=Iprobe INLATTICE=(-1,0)",
    ] {
        let card = parse(&format!(
            ".QPXF LIST=(0) OUT=I(L1) OUTLATTICE=(0,0) {selection}"
        ))
        .unwrap();
        assert_eq!(card, parse(&card.to_spice()).unwrap());
        let request = QpxfRequest::from_qpxf_card(&card).unwrap();
        assert_eq!(
            request,
            QpxfRequest::from_qpxf_card(&parse(&request.to_spice().unwrap()).unwrap()).unwrap()
        );
    }
    let mut escaped = default;
    escaped.input_sources = QpxfSources::Named(vec!["X one:\"probe\\path".into()]);
    escaped.output = QpxfOutput::BranchCurrent {
        branch: "probe) AXIS=oops".into(),
    };
    assert_eq!(
        escaped,
        QpxfRequest::from_qpxf_card(&parse(&escaped.to_spice().unwrap()).unwrap()).unwrap()
    );
    let signed = QpxfRequest::from_qpxf_card(
        &parse(".QPXF LIN 5 -100 100 OUT=V(out) OUTLATTICE=(0,0)").unwrap(),
    )
    .unwrap();
    assert_eq!(signed.frequencies_hz, [-100.0, -50.0, 0.0, 50.0, 100.0]);
}
#[test]
fn qpxf_card_rejects_dropped_options_and_preflights_large_sweeps() {
    for suffix in [
        "AXIS=bad",
        "AXIS=OUTPUT AXIS=OFFSET",
        "SOLVER=bad",
        "KRYLOVRESTART=1",
        "KRYLOVCYCLES=0",
        "RELTOL=0",
        "LINEARTOL=.1 RELTOL=.2",
        "GROUPDELAY=bad",
        "GDFLOOR=-1",
        "FROM=HB",
        "MAG=2",
        "IABSTOL=1e-10",
        "SOURCES=(V1,v1)",
        "SOURCE=V1 SOURCES=ALL",
        "INLATTICES=((0,0),(0,0))",
        "INLATTICES=ALL MAXORDERS=(1,1)",
        "MAXORDERS=(-1,1)",
        "INLATTICE=(0,0,0)",
        "OUT=I(R1)",
        "MAXORDERS=(1)",
        "INLATTICES=()",
    ] {
        let result = parse(&format!(
            ".QPXF LIST=(0) OUT=V(out) OUTLATTICE=(0,0) {suffix}"
        ))
        .and_then(|c| QpxfRequest::from_qpxf_card(&c).map_err(|e| e.to_string()));
        assert!(result.is_err(), "accepted {suffix}");
    }
    for sweep in [
        "LIST=()",
        "LIST=(1,1)",
        "LIST=(1,0)",
        "LIN 0 0 1",
        "LIN 3 1 1",
        "LIN 2 2 1",
        "DEC 1 0 1",
        "OCT 2 -1 1",
    ] {
        assert!(
            parse(&format!(".QPXF {sweep} OUT=V(out) OUTLATTICE=(0,0)"))
                .and_then(|c| QpxfRequest::from_qpxf_card(&c).map_err(|e| e.to_string()))
                .is_err(),
            "accepted {sweep}"
        );
    }
    let card = parse(".QPXF DEC 1000000000 1 1e100 OUT=V(out) OUTLATTICE=(0,0)").unwrap();
    let limits = ResourceLimits {
        max_analysis_points: 100,
        ..Default::default()
    };
    assert!(matches!(
        QpxfRequest::validate_qpxf_card(&card, &limits),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert!(matches!(
        QpxfRequest::from_qpxf_card_with_abort(&card, &limits, &NoAbort),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert!(matches!(
        QpxfRequest::from_qpxf_card_with_abort(&card, &limits, &CountingAbort::new(0)),
        Err(SimulationError::Aborted)
    ));
}
#[test]
fn qpxf_card_executes_selected_paths_from_retained_native_qpss() {
    let net = Netlist::parse("native QPXF\nV1 in 0 DC 1\nI1 0 out DC 0\nR1 in out 1k\nC1 out 0 100n\n.QPSS 1k 1.4142135623730951k HARMS=1\n.QPXF LIN 3 -100 100 SOURCES=(V1,I1) OUT=V(out) INLATTICES=((1,-1),(0,0)) OUTLATTICE=(1,-1) GROUPDELAY=YES GDFLOOR=1e-8\n.end\n").unwrap();
    let (AnalysisCommand::Qpss(qpss), AnalysisCommand::Qpxf(qpxf)) =
        (&net.analyses[0], &net.analyses[1])
    else {
        panic!("wrong cards")
    };
    let engine = Engine::new(Default::default());
    let point = engine
        .run_qpss(&net, QpssConfig::from_qpss_card(qpss).unwrap())
        .unwrap();
    let result = engine
        .run_qpxf_card_from_qpss_with_abort(&net, qpxf, &point, &NoAbort)
        .unwrap();
    assert_eq!(result.metadata.output_frequencies_hz, [-100.0, 0.0, 100.0]);
    assert_eq!(result.transfers.len(), 4);
    for transfer in &result.transfers {
        for (i, value) in transfer.values.iter().enumerate() {
            let expected = if transfer.input_lattice == [1, -1] {
                Complex64::new(
                    if transfer.input_source == 0 {
                        1.0
                    } else {
                        1000.0
                    },
                    0.0,
                ) / Complex64::new(
                    1.0,
                    std::f64::consts::TAU * result.metadata.output_frequencies_hz[i] * 1e-4,
                )
            } else {
                Complex64::ZERO
            };
            assert!((*value - expected).norm() < 1e-8);
        }
    }
}
