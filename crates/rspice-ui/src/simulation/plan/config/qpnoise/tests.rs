//! Studio authoring, native interchange and exact producer service integration.
use super::*;
use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;
use rspice_core::engine::{QpnoiseRequest, QpnoiseValue};
fn authored() -> QuasiPeriodicNoiseDraft {
    QuasiPeriodicNoiseDraft {
        explicit_frequencies: "100,300,700".into(),
        output_lattice: "0,0,0".into(),
        input_lattice: "0,0,0".into(),
        additional_outputs: vec![QpnoiseOutputDraft {
            node: "in".into(),
            reference: "out".into(),
            lattice: "1,-1,0".into(),
            ..Default::default()
        }],
        lattice_selection: QpnoiseLatticeSelection::Range,
        lattice_products: "-1:1,-1:1,-1:1".into(),
        integrated_noise: true,
        integration_method: QpnoiseIntegrationMethod::LogLog,
        band_start: "200".into(),
        band_stop: "500".into(),
        source_selection: QpnoiseSourceSelection::Only,
        source_names: "RS thermal\nRL thermal".into(),
        noise_figure: true,
        source_resistor: "Rs".into(),
        reference_temperature: "290".into(),
        reference_lattices: "0,0,0;1,-1,0".into(),
        linear_method: QuasiPeriodicLinearMethod::Krylov,
        krylov_restart: "18".into(),
        krylov_cycles: "9".into(),
        linear_tolerance: "2e-11".into(),
        ..Default::default()
    }
}
#[test]
fn qpnoise_studio_controls_survive_draft_worker_native_and_service_boundaries() {
    let draft: QuasiPeriodicNoiseDraft =
        ron::from_str(&ron::to_string(&authored()).unwrap()).unwrap();
    let spec = draft.to_spec().unwrap();
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let restored: AnalysisSpec =
        serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
            .unwrap()
            .into();
    assert_eq!(restored, spec);
    let card = spec.qpnoise_card().unwrap();
    assert_eq!(AnalysisSpec::from_qpnoise_card(&card).unwrap(), spec);
    let net =
        rspice_core::netlist::Netlist::parse(&format!("Studio card\n{}\n.end\n", card.to_spice()))
            .unwrap();
    let rspice_core::netlist::AnalysisCommand::Qpnoise(native) = &net.analyses[0] else {
        panic!("wrong card")
    };
    assert_eq!(
        QpnoiseRequest::from_qpnoise_card(native).unwrap(),
        QpnoiseRequest::from_qpnoise_card(&card).unwrap()
    );
    let mut producer = QpssDraft::default();
    producer.tones = "1k,1414.2135623730951,1732.0508075688772".into();
    producer.harmonics = "1,1,1".into();
    producer.relative_tolerance = "3e-8".into();
    let deck = "Studio noise\nV1 in 0 SIN(1 .1 1k)\nRs in out 1k\nRl out 0 2k\nC1 out 0 1n\n.temp 42\n.end\n";
    let service = &crate::services::simulation_runner::run_qpss_analysis_with_source_path_and_abort;
    let point = service(
        deck,
        producer.to_spec().unwrap().driven_qpss_config().unwrap(),
        None,
        &rspice_core::NoAbort,
    )
    .unwrap()
    .operating_point;
    let result=crate::services::simulation_runner::run_qpnoise_analysis_from_qpss_with_source_path_and_abort(deck,&card,&point,None,&rspice_core::NoAbort).unwrap();
    assert_eq!(
        result.metadata.operating_point_identity,
        point.retained_identity()
    );
    assert_eq!(result.metadata.request.linear.restart, 18);
    assert_eq!(result.metadata.request.linear.max_cycles, 9);
    assert_eq!(
        result.metadata.request.frequencies_hz,
        [100.0, 300.0, 700.0]
    );
    assert_eq!(result.outputs.len(), 2);
    assert!(
        result.outputs[0]
            .input_noise
            .as_ref()
            .unwrap()
            .iter()
            .all(|v| matches!(v,QpnoiseValue::Finite(v) if *v>0.0))
    );
    assert!(
        result.outputs[0]
            .noise_figure_db
            .as_ref()
            .unwrap()
            .iter()
            .all(|v| matches!(v,QpnoiseValue::Finite(v) if *v>0.0))
    );
    let changed = deck.replace("Rs in out 1k", "Rs in out 2k");
    assert!(crate::services::simulation_runner::run_qpnoise_analysis_from_qpss_with_source_path_and_abort(&changed,&card,&point,None,&rspice_core::NoAbort).is_err());
}
#[test]
fn qpnoise_studio_legacy_bounds_restore_and_inactive_text_is_ignored() {
    let old = serde_json::json!({"sweep":{"start":"1k","stop":"2k","points":"2","sweep":2},"output_node":"out","output_ref":"0","input_source":"V1","lattice_products":"-2:3,-1:1","integrated_noise":true,"contributor_ranking":true});
    let draft: QuasiPeriodicNoiseDraft = serde_json::from_value(old).unwrap();
    assert_eq!(draft.lattice_selection, QpnoiseLatticeSelection::Range);
    let spec = draft.to_spec().unwrap();
    let mut old_spec = serde_json::to_value(&spec).unwrap();
    old_spec["Qpnoise"]
        .as_object_mut()
        .unwrap()
        .remove("controls");
    assert_eq!(
        serde_json::from_value::<AnalysisSpec>(old_spec).unwrap(),
        spec
    );
    let mut draft = authored();
    draft.noise_figure = false;
    draft.input_referral = false;
    draft.current_output = true;
    draft.output_branch = "Lprobe".into();
    draft.additional_outputs.clear();
    draft.integrated_noise = false;
    draft.lattice_selection = QpnoiseLatticeSelection::AllRetained;
    draft.source_selection = QpnoiseSourceSelection::All;
    draft.linear_method = QuasiPeriodicLinearMethod::Direct;
    draft.sweep.start = "bad".into();
    draft.sweep.points = "bad".into();
    draft.input_lattice = "bad".into();
    draft.output_node = "".into();
    draft.output_ref = "".into();
    draft.reference_temperature = "bad".into();
    draft.reference_lattices = "bad".into();
    draft.band_start = "bad".into();
    draft.band_stop = "bad".into();
    draft.lattice_products = "bad".into();
    draft.source_names = "\n".into();
    draft.krylov_restart = "bad".into();
    draft.krylov_cycles = "bad".into();
    let spec = draft.to_spec().unwrap();
    let request = QpnoiseRequest::from_qpnoise_card(&spec.qpnoise_card().unwrap()).unwrap();
    assert!(request.input.is_none());
    assert!(request.integration.is_none());
    assert!(request.noise_figure.is_none());
    assert_eq!(request.input_lattices, QpnoiseLattices::AllRetained);
    draft.input_referral = true;
    assert!(draft.to_spec().is_err());
}
