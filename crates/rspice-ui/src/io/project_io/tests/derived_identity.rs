//! What a run whose tasks carry derived identities does to a project save.
//!
//! Two shipping families expand one authored analysis into several dispatched
//! tasks and give each of them its own derived identity: the multi-point Run
//! Set and the declared corner/temperature spaces mint one identity per point,
//! and a PSS asked to retain harmonics earns a spectrum companion. None of
//! those identities is an authored plan instance, and the cross-document check
//! that closes retained results over the plan only accepts authored ones.
//!
//! These cases record the measured consequence.

use super::*;

/// Mint the per-point identity exactly as the global Run Set expansion does.
fn run_set_point_id(producer: AnalysisInstanceId, index: usize) -> AnalysisInstanceId {
    AnalysisInstanceId::from_namespace(
        producer.as_uuid(),
        format!(
            "rspice-global-run-set/v2/{index}/TT/{:016x}/{:016x}/none",
            0_u64,
            27.0_f64.to_bits()
        )
        .as_bytes(),
    )
}

/// Mint the per-point identity exactly as a declared corner space does.
fn corner_point_id(
    declaration: AnalysisInstanceId,
    index: usize,
    count: usize,
) -> AnalysisInstanceId {
    AnalysisInstanceId::from_namespace(
        declaration.as_uuid(),
        format!("rspice-corner-run-point/v1/{index}/{count}/TT/1800000/27").as_bytes(),
    )
}

/// The AC instance the execution-context fixture authors, and the revision and
/// plan identity a run against it has to quote.
fn authored_ac(project: &ProjectFile) -> (AnalysisInstanceId, ObjectRevision, SimulationPlanId) {
    let plan = project
        .execution_context
        .as_ref()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan()
        .expect("stable plan");
    let id = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Ac)
        .expect("AC fixture instance")
        .id();
    (id, plan.revision(), plan.id())
}

/// Attach one prepared run whose tasks carry the supplied identities.
fn attach_run(
    project: &mut ProjectFile,
    plan_id: SimulationPlanId,
    source_revision: ObjectRevision,
    tasks: &[(AnalysisInstanceId, AnalysisType, u8, Vec<AnalysisInstanceId>)],
) {
    let snapshot = ContentDigest::from_bytes([0x5c; 32]);
    let mut run = SimulationRun::new(31);
    for (index, (id, analysis_type, _, dependencies)) in tasks.iter().enumerate() {
        run.add_analysis(
            AnalysisResult::new(index as u64 + 1, *analysis_type, "result").with_provenance(
                AnalysisResultProvenance::new(
                    *id,
                    source_revision,
                    snapshot,
                    dependencies.clone(),
                )
                .expect("prepared provenance"),
            ),
        );
    }
    let tags = tasks.iter().map(|(_, _, tag, _)| *tag).collect::<Vec<_>>();
    seal_prepared_run(
        &mut run,
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        project.workspace.project.revision(),
        ContentDigest::from_bytes([0x5b; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x5a; 32])),
        &tags,
    );
    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 32;
    project.simulation_results = ProjectSimulationResults::from_state(&simulation);
}

/// A single-point run saves, so the defect is the expansion and not the run.
#[test]
fn a_single_point_run_against_an_authored_analysis_saves() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[(source_id, AnalysisType::Ac, 2, Vec::new())],
    );

    serialize_project_file(&project).expect("an authored single-point run saves");
}

#[test]
fn a_multi_point_run_set_cannot_be_saved_today() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[
            (
                run_set_point_id(source_id, 0),
                AnalysisType::Ac,
                2,
                Vec::new(),
            ),
            (
                run_set_point_id(source_id, 1),
                AnalysisType::Ac,
                2,
                Vec::new(),
            ),
        ],
    );

    let error = serialize_project_file(&project)
        .expect_err("a two-point Run Set cannot be written to a project file")
        .to_string();
    assert!(
        error.contains("runs[0].prepared_receipt.tasks[0].source_instance_id")
            && error.contains("is absent from the persisted plan and its tombstones"),
        "{error}"
    );
}

#[test]
fn a_declared_corner_space_cannot_be_saved_today() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[
            (
                corner_point_id(source_id, 0, 2),
                AnalysisType::Ac,
                2,
                Vec::new(),
            ),
            (
                corner_point_id(source_id, 1, 2),
                AnalysisType::Ac,
                2,
                Vec::new(),
            ),
        ],
    );

    let error = serialize_project_file(&project)
        .expect_err("a two-point declared corner space cannot be written to a project file")
        .to_string();
    assert!(
        error.contains("is absent from the persisted plan and its tombstones"),
        "{error}"
    );
}

#[test]
fn a_pss_spectrum_companion_cannot_be_saved_today() {
    let mut project = project_with_execution_context();
    let plan_id = authored_ac(&project).2;
    let (producer, source_revision) = {
        let plan = project
            .execution_context
            .as_mut()
            .expect("execution context")
            .simulation_plan
            .stable_analysis_plan_mut()
            .expect("stable plan");
        let (id, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
        (id, plan.revision())
    };
    let spectrum = crate::product::derived_analysis_instance_id(producer, "pss-spectrum");
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[
            (producer, AnalysisType::Pss, 7, Vec::new()),
            (
                spectrum,
                AnalysisType::HarmonicBalance,
                35,
                vec![producer],
            ),
        ],
    );

    let error = serialize_project_file(&project)
        .expect_err("a PSS spectrum companion cannot be written to a project file")
        .to_string();
    assert!(
        error.contains("runs[0].prepared_receipt.tasks[1].source_instance_id")
            && error.contains("is absent from the persisted plan and its tombstones"),
        "{error}"
    );
}
