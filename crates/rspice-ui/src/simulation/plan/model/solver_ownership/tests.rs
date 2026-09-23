use super::*;
use NumericOverrideOption as O;

fn study(
    plan: &mut SimulationPlan,
    kind: AnalysisKind,
    base: AnalysisInstanceId,
) -> AnalysisInstanceId {
    let id = plan.insert(kind).unwrap().0;
    select(plan, id, base);
    id
}

fn select(plan: &mut SimulationPlan, id: AnalysisInstanceId, base: AnalysisInstanceId) {
    plan.edit(id, |draft| match draft {
        AnalysisDraft::MonteCarlo(state) => state.base_analysis = Some(base),
        AnalysisDraft::Optimization(state) => state.base_analysis = Some(base),
        AnalysisDraft::Temperature(state) => state.base_analysis = Some(base),
        AnalysisDraft::Corner(state) => state.base_analysis = Some(base),
        _ => unreachable!(),
    })
    .unwrap();
}

fn accepts(plan: &SimulationPlan, id: AnalysisInstanceId, option: O) -> bool {
    option
        .refusal_for_instance(plan.instance(id).unwrap().kind(), plan.solver_ownership(id))
        .is_none()
}

fn set(
    plan: &mut SimulationPlan,
    id: AnalysisInstanceId,
    option: O,
    value: &str,
) -> Result<(), String> {
    let instance = plan.instance(id).unwrap();
    let mut record = instance.numeric_override().cloned().unwrap_or_default();
    record.set_for_instance(instance.kind(), plan.solver_ownership(id), option, value)?;
    plan.set_numeric_override(id, Some(record))
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[test]
fn configured_study_defaults_follow_base_ownership_and_restore_without_history_edits() {
    assert!(O::all().count() <= 64);
    for kind in [
        AnalysisKind::MonteCarlo,
        AnalysisKind::Optimization,
        AnalysisKind::Temperature,
        AnalysisKind::Corner,
    ] {
        let mut plan = SimulationPlan::empty();
        let ac = plan.insert(AnalysisKind::Ac).unwrap().0;
        let tran = plan.insert(AnalysisKind::Transient).unwrap().0;
        let id = study(&mut plan, kind, ac);
        assert!(accepts(&plan, id, O::Reltol));
        assert!(!accepts(&plan, id, O::Itl4));
        assert!(set(&mut plan, id, O::Itl4, "99").is_err());
        set(&mut plan, id, O::Reltol, "1e-5").unwrap();
        select(&mut plan, id, tran);
        set(&mut plan, id, O::MaximumTimestep, "2n").unwrap();
        set(&mut plan, id, O::OutputTimePoints, "0 1n").unwrap();
        set(&mut plan, tran, O::OutputTimePoints, "0 2n").unwrap();
        let defaults = plan
            .instance(id)
            .unwrap()
            .numeric_override()
            .unwrap()
            .to_spice_options();
        let overrides = plan
            .instance(tran)
            .unwrap()
            .numeric_override()
            .unwrap()
            .to_spice_options();
        let circuit = rspice_core::Netlist::parse(&format!(
            "study timing\nV1 n 0 1\nR1 n 0 1k\n.tran 1n 3n\n{defaults}\n{overrides}\n.end\n"
        ))
        .unwrap();
        assert_eq!(circuit.options.output_time_points, [0.0, 1e-9, 2e-9]);
        let merged = plan
            .instance(id)
            .unwrap()
            .numeric_override()
            .unwrap()
            .clone()
            .with_base_options(plan.instance(tran).unwrap().numeric_override().unwrap());
        let merged_circuit = rspice_core::Netlist::parse(&format!(
            "study timing\nV1 n 0 1\nR1 n 0 1k\n.tran 1n 3n\n{}\n.end\n",
            merged.to_spice_options(),
        ))
        .unwrap();
        assert_eq!(
            merged_circuit.options.output_time_points,
            circuit.options.output_time_points
        );
        assert_eq!(
            rspice_core::Engine::default()
                .resolved_for_netlist(&circuit)
                .config()
                .transient_timeint_max_timestep,
            Some(2e-9)
        );
        let mut restored = plan.clone();
        restored.prepare_after_restore();
        assert_eq!(
            restored.instance(id).unwrap().numeric_override(),
            plan.instance(id).unwrap().numeric_override()
        );
        plan.set_numeric_override(tran, None).unwrap();
        set(&mut plan, id, O::Itl4, "99").unwrap();
        let before = serde_json::to_value(&plan).unwrap();
        assert!(set(&mut plan, tran, O::Itl4, "88").is_err());
        assert_eq!(serde_json::to_value(&plan).unwrap(), before);
        plan.set_numeric_override(id, None).unwrap();
        set(&mut plan, tran, O::Itl4, "88").unwrap();
        assert!(!accepts(&plan, id, O::Itl4));
        set(&mut plan, tran, O::StrobeInterval, "1n").unwrap();
        assert!(!accepts(&plan, id, O::OutputTimePoints));
        assert!(set(&mut plan, id, O::OutputTimePoints, "0 1n").is_err());
        select(&mut plan, id, ac);
        // A restored record bypassed the current authoring gate.
        let index = plan.index_of(id).unwrap();
        plan.instances[index].numeric_override =
            Some(serde_json::from_str(r#"{"reltol":0.00001,"itl4":99}"#).unwrap());
        let receipts = serde_json::to_value(&plan.receipts).unwrap();
        let carrier = plan.instance(tran).unwrap().numeric_override().cloned();
        let mut restored: SimulationPlan =
            serde_json::from_value(serde_json::to_value(&plan).unwrap()).unwrap();
        restored.prepare_after_restore();
        let record = restored.instance(id).unwrap().numeric_override().unwrap();
        assert!(record.stated(O::Reltol).is_some());
        assert!(record.stated(O::Itl4).is_none());
        assert_eq!(
            restored.instance(tran).unwrap().numeric_override(),
            carrier.as_ref()
        );
        assert_eq!(serde_json::to_value(&restored.receipts).unwrap(), receipts);
        let once = serde_json::to_value(&restored).unwrap();
        restored.prepare_after_restore();
        assert_eq!(serde_json::to_value(&restored).unwrap(), once);
    }
}

#[test]
fn configured_study_defaults_follow_spectral_carriers_and_only_executed_op_seeds() {
    let mut plan = SimulationPlan::empty();
    let op = plan.insert(AnalysisKind::OperatingPoint).unwrap().0;
    let hb = plan.insert(AnalysisKind::HarmonicBalance).unwrap().0;
    plan.bind_dependency(hb, AnalysisKind::OperatingPoint, op)
        .unwrap();
    let consumer = plan.insert(AnalysisKind::Hbsp).unwrap().0;
    plan.bind_dependency(consumer, AnalysisKind::HarmonicBalance, hb)
        .unwrap();
    let id = study(&mut plan, AnalysisKind::MonteCarlo, consumer);
    assert!(accepts(&plan, id, O::HbInitialState));
    set(&mut plan, hb, O::Reltol, "1e-6").unwrap();
    assert!(
        accepts(&plan, id, O::Reltol),
        "the OP seed still inherits it"
    );
    set(&mut plan, id, O::HbInitialState, "0").unwrap();
    assert!(!accepts(&plan, id, O::Reltol), "zero start runs no OP seed");
    set(&mut plan, id, O::HbInitialState, "2").unwrap();
    assert!(accepts(&plan, id, O::Reltol));
    assert!(!accepts(&plan, id, O::Itl4));
    set(&mut plan, id, O::HbInitialState, "1").unwrap();
    assert!(accepts(&plan, id, O::Itl4));
    assert!(accepts(&plan, id, O::MaximumTimestep));
    assert!(!accepts(&plan, id, O::OutputTimePoints));
    assert!(!accepts(&plan, id, O::RetainEverySignal));
    set(&mut plan, id, O::Itl4, "90").unwrap();
    assert!(set(&mut plan, id, O::HbInitialState, "0").is_err());
    let mut restored = plan.clone();
    restored.prepare_after_restore();
    assert_eq!(
        restored.instance(id).unwrap().numeric_override(),
        plan.instance(id).unwrap().numeric_override()
    );
    let assisted = plan.instance(id).unwrap().numeric_override().cloned();
    plan.set_numeric_override(id, None).unwrap();
    // Importing startup and its dependent controls is one atomic edit.
    plan.set_numeric_override(id, assisted).unwrap();
    plan.set_numeric_override(id, None).unwrap();

    let qp = plan.insert(AnalysisKind::Qpss).unwrap().0;
    plan.bind_dependency(qp, AnalysisKind::OperatingPoint, op)
        .unwrap();
    set(&mut plan, qp, O::Reltol, "1e-6").unwrap();
    select(&mut plan, id, qp);
    assert!(!accepts(&plan, id, O::Reltol));
    plan.edit(qp, |draft| {
        let AnalysisDraft::Qpss(state) = draft else {
            unreachable!()
        };
        state.dc_initialization = true;
    })
    .unwrap();
    assert!(accepts(&plan, id, O::Reltol));

    let tran = plan.insert(AnalysisKind::Transient).unwrap().0;
    let fft = plan.insert(AnalysisKind::Fft).unwrap().0;
    plan.bind_dependency(fft, AnalysisKind::Transient, tran)
        .unwrap();
    select(&mut plan, id, fft);
    assert!(accepts(&plan, id, O::Itl4));
    assert!(!accepts(&plan, id, O::HbInitialState));
    set(&mut plan, tran, O::Itl4, "80").unwrap();
    assert!(!accepts(&plan, id, O::Itl4));
}
