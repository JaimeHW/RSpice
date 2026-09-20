//! All native noise options round-trip and drive authenticated QPSS execution.
use super::*;
use crate::abort_signal::CountingAbort;
use crate::netlist::AnalysisCommand;
fn parse(card: &str) -> Result<QpnoiseCard, String> {
    let net = Netlist::parse(&format!("QPNOISE card\n.param probe=31.25\n{card}\n.end\n"))
        .map_err(|e| e.to_string())?;
    let AnalysisCommand::Qpnoise(card) = &net.analyses[0] else {
        panic!("wrong native analysis")
    };
    Ok(card.as_ref().clone())
}
#[test]
fn qpnoise_card_roundtrip_preserves_all_measurement_and_solver_controls() {
    for sweep in [
        "LIN 5 -100 100",
        "DEC 3 1 1k",
        "OCT 2 2 128",
        "LIST=(-137,0,{probe},1k)",
    ] {
        let card=parse(&format!(".QPNOISE {sweep} OUTS=((V(out/ref,ref-2),(1,-2,0)),(I(L1),(-1,0,2))) SOURCE=Xmix:V-one INLATTICE=(0,0,0) MINLATTICE=(-1,-2,-3) MAXLATTICE=(2,2,3) EXCLUDESOURCES=(\"R quiet thermal\") AXIS=OFFSET INTEGRATED=YES BAND=(10,500) INTEGRATION=LOGLOG RANK=YES NOISEFIGURE=YES SOURCERESISTOR=Rs TREF=298.765 REFLATTICES=((0,0,0),(1,-1,0)) SOLVER=KRYLOV KRYLOVRESTART=24 KRYLOVCYCLES=7 RELTOL=2.345678901234567e-11 FROM=QPSS")).unwrap();
        assert_eq!(parse(&card.to_spice()).unwrap(), card);
        let req = QpnoiseRequest::from_qpnoise_card(&card).unwrap();
        let roundtrip =
            QpnoiseRequest::from_qpnoise_card(&parse(&req.to_spice().unwrap()).unwrap()).unwrap();
        assert_eq!(roundtrip, req);
        assert_eq!(req.outputs.len(), 2);
        assert_eq!(req.linear.restart, 24);
        assert_eq!(
            req.noise_figure.as_ref().unwrap().reference_temperature,
            298.765
        );
        assert_eq!(
            req.integration.as_ref().unwrap().method,
            QpnoiseIntegrationMethod::LogLog
        );
    }
    for selection in [
        "NOISELATTICES=ALL",
        "NOISELATTICES=((-1,0),(0,1))",
        "MAXORDERS=(1,2)",
        "MINLATTICE=(-1,-2) MAXLATTICE=(2,1)",
        "NOISESOURCES=(\"Rs thermal\",\"Q1 flicker\")",
    ] {
        let card = parse(&format!(
            ".QPNOISE LIST=(1) OUT=V(out) OUTLATTICE=(0,0) {selection} INTEGRATED=NO RANK=NO"
        ))
        .unwrap();
        let req = QpnoiseRequest::from_qpnoise_card(&card).unwrap();
        assert!(req.input.is_none());
        assert!(req.integration.is_none());
        assert!(!req.contributor_ranking);
        assert_eq!(card, parse(&card.to_spice()).unwrap());
    }
    let mut req = QpnoiseRequest::from_qpnoise_card(
        &parse(".QPNOISE LIN 5 -100 100 OUT=V(out) OUTLATTICE=(0,0)").unwrap(),
    )
    .unwrap();
    assert_eq!(req.frequencies_hz, [-100.0, -50.0, 0.0, 50.0, 100.0]);
    req.sources = QpnoiseSources::Only(vec!["X one:\"probe\\path thermal".into()]);
    req.outputs[0].observation = QpnoiseObservation::BranchCurrent {
        branch: "probe) AXIS=oops".into(),
    };
    assert_eq!(
        req,
        QpnoiseRequest::from_qpnoise_card(&parse(&req.to_spice().unwrap()).unwrap()).unwrap()
    );
}
#[test]
fn qpnoise_card_rejects_inert_ambiguous_and_unbounded_options() {
    for suffix in [
        "AXIS=bad",
        "AXIS=OUTPUT AXIS=OFFSET",
        "SOURCE=V1",
        "INLATTICE=(0,0)",
        "SOLVER=bad",
        "KRYLOVRESTART=1",
        "KRYLOVCYCLES=0",
        "LINEARTOL=.1 RELTOL=.2",
        "RELTOL=0",
        "NOISELATTICES=((0,0),(0,0))",
        "NOISELATTICES=ALL MAXORDERS=(1,1)",
        "MAXORDERS=(-1,1)",
        "MINLATTICE=(0,0)",
        "MINLATTICE=(1,0) MAXLATTICE=(0,1)",
        "MINLATTICE=(0,0) MAXLATTICE=(1,1) NOISELATTICES=ALL",
        "NOISESOURCES=(\"R1 thermal\",\"r1 thermal\")",
        "NOISESOURCES=ALL EXCLUDESOURCES=(\"R1 thermal\")",
        "INTEGRATED=NO BAND=(1,2)",
        "INTEGRATED=NO INTEGRATION=LINEAR",
        "BAND=(2,1)",
        "BAND=(1,2,3)",
        "RANK=bad",
        "NOISEFIGURE=YES",
        "SOURCERESISTOR=Rs",
        "TREF=290",
        "REFLATTICES=((0,0))",
        "SOURCE=V1 INLATTICE=(0,0) NOISEFIGURE=YES SOURCERESISTOR=Rs TREF=0",
        "FROM=HB",
        "MAG=2",
        "OUTS=((V(out),(0,0)))",
    ] {
        let result = parse(&format!(
            ".QPNOISE LIST=(1) OUT=V(out) OUTLATTICE=(0,0) {suffix}"
        ))
        .and_then(|c| QpnoiseRequest::from_qpnoise_card(&c).map_err(|e| e.to_string()));
        assert!(result.is_err(), "accepted {suffix}");
    }
    let card = parse(".QPNOISE LIN 1000000 1 1e6 OUT=V(out) OUTLATTICE=(0,0)").unwrap();
    let mut limits = ResourceLimits::default();
    limits.max_analysis_points = 20;
    assert!(matches!(
        QpnoiseRequest::validate_qpnoise_card(&card, &limits),
        Err(SimulationError::ResourceLimit(_))
    ));
    assert!(matches!(
        QpnoiseRequest::from_qpnoise_card_with_abort(
            &card,
            &ResourceLimits::default(),
            &CountingAbort::new(0)
        ),
        Err(SimulationError::Aborted)
    ));
}
#[test]
fn qpnoise_native_card_executes_on_exact_retained_qpss_producer() {
    let net=Netlist::parse("Native QPNOISE\nV1 in 0 SIN(1 .1 1k)\nRs in out 1k\nRl out 0 2k noisy=0\n.QPSS 1k 1.4142135623730951k HARMS=1\n.QPNOISE DEC 2 10 1k OUT=V(out) OUTLATTICE=(0,0) SOURCE=V1 INLATTICE=(0,0) NOISEFIGURE=YES SOURCERESISTOR=Rs\n.end\n").unwrap();
    let (AnalysisCommand::Qpss(qpss), AnalysisCommand::Qpnoise(noise)) =
        (&net.analyses[0], &net.analyses[1])
    else {
        panic!("missing producer or consumer")
    };
    let engine = Engine::default();
    let point = engine
        .run_qpss(&net, QpssConfig::from_qpss_card(qpss).unwrap())
        .unwrap();
    let result = engine
        .run_qpnoise_card_from_qpss_with_abort(&net, noise, &point, &NoAbort)
        .unwrap();
    assert_eq!(
        result.metadata.operating_point_identity,
        point.retained_identity()
    );
    assert_eq!(result.outputs[0].frequencies_hz.len(), 5);
    assert!(
        result.outputs[0]
            .noise_figure_db
            .as_ref()
            .unwrap()
            .iter()
            .all(|v| matches!(v,QpnoiseValue::Finite(db) if db.abs()<1e-10))
    );
    assert!(
        matches!(result.outputs[0].integrated.as_ref().unwrap().output_rms,QpnoiseValue::Finite(v) if v>0.0)
    );
}
