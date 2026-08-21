//! Persisting a run whose tasks the plan did not author.
//!
//! Three shipping families expand one authored analysis into several
//! dispatched tasks under identities of their own: the multi-point Run Set and
//! a declared corner or temperature space mint one per point, and a PSS asked
//! to retain harmonics earns a spectrum companion. None of those identities is
//! an authored plan instance, and the check that closes retained results over
//! the plan used to accept only authored ones — so a project holding any such
//! run was refused at save.
//!
//! One rule admits all three: a task the plan did not author states the
//! authored instance its identity was derived from and the roles that produced
//! it, and is accepted exactly when that root is in the plan and the roles
//! re-derive the identity it claims. The cases here pin both halves — what now
//! round-trips, and what is still refused.

use super::*;

use crate::product::DerivedAnalysisIdentity;

/// One task of a fixture run: its identity, how it was derived if it was, the
/// result family it produced, its canonical tag, and its graph edges.
struct TaskFixture {
    derivation: Option<DerivedAnalysisIdentity>,
    instance_id: AnalysisInstanceId,
    analysis_type: AnalysisType,
    analysis_kind_tag: u8,
    dependencies: Vec<AnalysisInstanceId>,
}

impl TaskFixture {
    fn authored(
        instance_id: AnalysisInstanceId,
        analysis_type: AnalysisType,
        analysis_kind_tag: u8,
    ) -> Self {
        Self {
            derivation: None,
            instance_id,
            analysis_type,
            analysis_kind_tag,
            dependencies: Vec::new(),
        }
    }

    fn derived(
        derivation: DerivedAnalysisIdentity,
        analysis_type: AnalysisType,
        analysis_kind_tag: u8,
    ) -> Self {
        Self {
            instance_id: derivation.instance_id(),
            derivation: Some(derivation),
            analysis_type,
            analysis_kind_tag,
            dependencies: Vec::new(),
        }
    }

    fn after(mut self, dependencies: Vec<AnalysisInstanceId>) -> Self {
        self.dependencies = dependencies;
        self
    }
}

const RUN_ID: u64 = 31;
const SNAPSHOT: [u8; 32] = [0x5c; 32];

/// The AC instance the execution-context fixture authors, with the revision
/// and plan identity a run against it has to quote.
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

/// Author one more analysis of the given kind and answer with its identity and
/// the plan revision that inserted it.
fn author(project: &mut ProjectFile, kind: AnalysisKind) -> (AnalysisInstanceId, ObjectRevision) {
    let plan = project
        .execution_context
        .as_mut()
        .expect("execution context")
        .simulation_plan
        .stable_analysis_plan_mut()
        .expect("stable plan");
    let (id, _) = plan.insert(kind).expect("analysis inserts");
    (id, plan.revision())
}

/// Attach one prepared run built from the supplied tasks, with a result for
/// each of the first `produced` of them.
fn attach_run(
    project: &mut ProjectFile,
    plan_id: SimulationPlanId,
    source_revision: ObjectRevision,
    tasks: &[TaskFixture],
    produced: usize,
) {
    let snapshot = ContentDigest::from_bytes(SNAPSHOT);
    let mut run = SimulationRun::new(RUN_ID);
    for (index, task) in tasks.iter().take(produced).enumerate() {
        run.add_analysis(
            AnalysisResult::new(index as u64 + 1, task.analysis_type, "result").with_provenance(
                AnalysisResultProvenance::new(
                    task.instance_id,
                    source_revision,
                    snapshot,
                    task.dependencies.clone(),
                )
                .expect("prepared provenance"),
            ),
        );
    }
    let receipts = tasks
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let config_digest = ContentDigest::from_bytes([0xc0_u8.wrapping_add(index as u8); 32]);
            match task.derivation.clone() {
                Some(derivation) => PreparedRunTaskReceipt::new_derived(
                    derivation,
                    source_revision,
                    task.dependencies.clone(),
                    task.analysis_kind_tag,
                    config_digest,
                ),
                None => PreparedRunTaskReceipt::new(
                    task.instance_id,
                    source_revision,
                    task.dependencies.clone(),
                    task.analysis_kind_tag,
                    config_digest,
                ),
            }
            .expect("prepared task receipt")
        })
        .collect::<Vec<_>>();
    let receipt = PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(plan_id),
        project.workspace.project.revision(),
        snapshot,
        ContentDigest::from_bytes([0x5b; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x5a; 32])),
        receipts,
    )
    .expect("prepared run receipt");
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("prepared fixture seals explicitly");

    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = RUN_ID + 1;
    project.simulation_results = ProjectSimulationResults::from_state(&simulation);
}

/// Save, reload, and answer with the restored task identities. Panics with the
/// reader's own warning if the run did not survive, because a dropped result
/// history is the failure this whole module exists to keep from happening.
fn round_trip_task_identities(project: &ProjectFile) -> Vec<AnalysisInstanceId> {
    let json = serialize_project_file(project).expect("the project serializes");
    let restored = load_project_text(&json, None).expect("the project loads");
    assert!(
        restored.simulation_results_warning.is_none(),
        "{:?}",
        restored.simulation_results_warning
    );
    let run = restored
        .simulation_results
        .runs
        .first()
        .expect("the retained run survives the round trip");
    run.prepared_receipt
        .as_ref()
        .expect("the restored run keeps its receipt")
        .tasks
        .iter()
        .map(|task| task.source_instance_id)
        .collect()
}

/// The Run Set's per-point role, spelled as preparation spells it.
fn run_set_point_role(index: usize) -> String {
    format!(
        "rspice-global-run-set/v2/{index}/TT/{:016x}/{:016x}/none",
        0_u64,
        27.0_f64.to_bits()
    )
}

#[test]
fn a_single_point_run_against_an_authored_analysis_saves() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[TaskFixture::authored(source_id, AnalysisType::Ac, 2)],
        1,
    );

    assert_eq!(round_trip_task_identities(&project), vec![source_id]);
}

#[test]
fn a_multi_point_run_set_round_trips_through_a_project_file() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    let points = (0..2)
        .map(|index| {
            DerivedAnalysisIdentity::new(source_id, run_set_point_role(index))
                .expect("a point role is named")
        })
        .collect::<Vec<_>>();
    let tasks = points
        .iter()
        .map(|point| TaskFixture::derived(point.clone(), AnalysisType::Ac, 2))
        .collect::<Vec<_>>();
    attach_run(&mut project, plan_id, source_revision, &tasks, tasks.len());

    assert_eq!(
        round_trip_task_identities(&project),
        points
            .iter()
            .map(DerivedAnalysisIdentity::instance_id)
            .collect::<Vec<_>>()
    );
}

/// A declared space's points run its *base* analysis, so their canonical tag
/// is not the declaration's. That divergence is legitimate and has to survive.
#[test]
fn a_declared_corner_space_round_trips_with_its_base_analysis_tag() {
    let mut project = project_with_execution_context();
    let plan_id = authored_ac(&project).2;
    let (declaration, source_revision) = author(&mut project, AnalysisKind::Corner);
    let points = (0..2)
        .map(|index| {
            DerivedAnalysisIdentity::new(
                declaration,
                format!("rspice-corner-run-point/v1/{index}/2/TT/1800000/27"),
            )
            .expect("a point role is named")
        })
        .collect::<Vec<_>>();
    let mut tasks = points
        .iter()
        .map(|point| TaskFixture::derived(point.clone(), AnalysisType::Transient, 5))
        .collect::<Vec<_>>();
    // The declaration keeps its own task, which assembles the family last.
    tasks.push(
        TaskFixture::authored(declaration, AnalysisType::Corner, 19).after(
            points
                .iter()
                .map(DerivedAnalysisIdentity::instance_id)
                .collect(),
        ),
    );
    attach_run(&mut project, plan_id, source_revision, &tasks, tasks.len());

    let restored = round_trip_task_identities(&project);
    assert_eq!(restored.len(), 3);
    assert_eq!(restored[2], declaration);
}

#[test]
fn a_pss_spectrum_companion_round_trips_through_a_project_file() {
    let mut project = project_with_execution_context();
    let plan_id = authored_ac(&project).2;
    let (producer, source_revision) = author(&mut project, AnalysisKind::Pss);
    let spectrum =
        DerivedAnalysisIdentity::new(producer, "pss-spectrum").expect("the spectrum role is named");
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[
            TaskFixture::authored(producer, AnalysisType::Pss, 7),
            TaskFixture::derived(spectrum.clone(), AnalysisType::HarmonicBalance, 35)
                .after(vec![producer]),
        ],
        2,
    );

    assert_eq!(
        round_trip_task_identities(&project),
        vec![producer, spectrum.instance_id()]
    );
}

/// A spectrum of a PSS solved at one point of a Run Set is two derivation
/// steps from the plan, and the path composes rather than restarting.
#[test]
fn a_two_step_derivation_still_roots_at_the_authored_analysis() {
    let mut project = project_with_execution_context();
    let plan_id = authored_ac(&project).2;
    let (producer, source_revision) = author(&mut project, AnalysisKind::Pss);
    let spectrum =
        DerivedAnalysisIdentity::new(producer, "pss-spectrum").expect("the spectrum role is named");
    let point = spectrum
        .clone()
        .extended(run_set_point_role(1))
        .expect("a point role is named");
    assert_eq!(point.authored(), producer);
    assert_ne!(point.instance_id(), spectrum.instance_id());
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[TaskFixture::derived(
            point.clone(),
            AnalysisType::HarmonicBalance,
            35,
        )],
        1,
    );

    assert_eq!(
        round_trip_task_identities(&project),
        vec![point.instance_id()]
    );
}

/// Aborting a multi-point run leaves an ordered prefix of its tasks. Every
/// truncation has to remain a valid record, including the empty one.
#[test]
fn every_truncation_of_a_multi_point_run_is_a_valid_prefix() {
    const POINTS: usize = 3;

    for produced in 0..=POINTS {
        // The fixture mints a fresh plan identity, so each pass reads the
        // authored analysis out of the project it is about to save.
        let mut project = project_with_execution_context();
        let (source_id, source_revision, plan_id) = authored_ac(&project);
        let tasks = (0..POINTS)
            .map(|index| {
                TaskFixture::derived(
                    DerivedAnalysisIdentity::new(source_id, run_set_point_role(index))
                        .expect("a point role is named"),
                    AnalysisType::Ac,
                    2,
                )
            })
            .collect::<Vec<_>>();
        attach_run(&mut project, plan_id, source_revision, &tasks, produced);
        let json = serialize_project_file(&project)
            .unwrap_or_else(|error| panic!("prefix of {produced} results saves: {error}"));
        let restored = load_project_text(&json, None).expect("the project loads");
        assert!(
            restored.simulation_results_warning.is_none(),
            "prefix of {produced} results: {:?}",
            restored.simulation_results_warning
        );
        assert_eq!(
            restored.simulation_results.runs[0].analyses.len(),
            produced,
            "the restored prefix keeps exactly the results the run produced"
        );
    }
}

#[test]
fn a_receipt_naming_a_never_authored_instance_is_still_refused() {
    let mut project = project_with_execution_context();
    let (_, source_revision, plan_id) = authored_ac(&project);
    let forged = AnalysisInstanceId::new();
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[TaskFixture::authored(forged, AnalysisType::Ac, 2)],
        1,
    );

    let error = serialize_project_file(&project)
        .expect_err("an instance the plan never authored is refused")
        .to_string();
    assert!(
        error.contains(&format!(
            "runs[0].prepared_receipt.tasks[0].source_instance_id {forged} is absent from the persisted plan and its tombstones"
        )),
        "{error}"
    );
}

#[test]
fn a_derived_task_whose_producer_was_never_authored_is_refused() {
    let mut project = project_with_execution_context();
    let (_, source_revision, plan_id) = authored_ac(&project);
    let stranger = AnalysisInstanceId::new();
    let derivation = DerivedAnalysisIdentity::new(stranger, run_set_point_role(0))
        .expect("a point role is named");
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[TaskFixture::derived(derivation, AnalysisType::Ac, 2)],
        1,
    );

    let error = serialize_project_file(&project)
        .expect_err("a derivation from an unauthored producer is refused")
        .to_string();
    assert!(
        error.contains(&format!(
            "runs[0].prepared_receipt.tasks[0].source_instance_id {stranger} is absent from the persisted plan and its tombstones"
        )),
        "{error}"
    );
}

/// The receipt layer refuses to build a task whose identity its own derivation
/// does not produce, so this forgery can only be written into the file. It has
/// to be caught on the way back in.
#[test]
fn a_derived_identity_that_does_not_recompute_is_refused_on_load() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    let derivation = DerivedAnalysisIdentity::new(source_id, run_set_point_role(0))
        .expect("a point role is named");
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[TaskFixture::derived(derivation, AnalysisType::Ac, 2)],
        1,
    );
    let json = serialize_project_file(&project).expect("the honest project serializes");

    let mut tampered: serde_json::Value = serde_json::from_str(&json).expect("project parses");
    let claimed = AnalysisInstanceId::new().to_string();
    tampered["simulation_results"]["runs"][0]["prepared_receipt"]["tasks"][0]["source_instance_id"] =
        serde_json::Value::String(claimed.clone());
    tampered["simulation_results"]["runs"][0]["analyses"][0]["provenance"]["source_instance_id"] =
        serde_json::Value::String(claimed);
    let tampered = serde_json::to_string(&tampered).expect("tampered project re-encodes");

    let restored = load_project_text(&tampered, None).expect("a tampered project still opens");
    let warning = restored
        .simulation_results_warning
        .expect("the tampered history is refused with a stated reason");
    assert!(
        warning.contains("does not re-derive from analysis"),
        "{warning}"
    );
    assert!(restored.simulation_results.runs.is_empty());
}

/// A manual deck authors no plan instance, so a manual-deck task has nothing
/// to derive from and may not claim to.
#[test]
fn a_manual_deck_task_cannot_claim_a_derivation() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    let derivation = DerivedAnalysisIdentity::new(source_id, run_set_point_role(0))
        .expect("a point role is named");
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[TaskFixture::derived(derivation, AnalysisType::Ac, 2)],
        1,
    );
    let json = serialize_project_file(&project).expect("the plan-sourced project serializes");

    let mut manual: serde_json::Value = serde_json::from_str(&json).expect("project parses");
    let receipt = &mut manual["simulation_results"]["runs"][0]["prepared_receipt"];
    receipt["source_domain"] = serde_json::json!("manual_deck");
    receipt
        .as_object_mut()
        .expect("receipt object")
        .remove("simulation_plan_id");
    receipt["source_check_receipt"]["kind"] = serde_json::json!("manual_source_check");
    manual["simulation_results"]["runs"][0]["analyses"][0]["provenance"]["source_domain"] =
        serde_json::json!("manual_deck");
    let manual = serde_json::to_string(&manual).expect("manual project re-encodes");

    let restored = load_project_text(&manual, None).expect("the project still opens");
    let warning = restored
        .simulation_results_warning
        .expect("a manual-deck derivation is refused with a stated reason");
    assert!(
        warning.contains("cannot derive from a plan instance"),
        "{warning}"
    );
}

/// Every project written before this rule existed holds authored task
/// identities only — a run with derived ones could not be saved at all — so
/// nothing pre-existing carries the field, and a file that claims an older
/// schema while carrying one is a rewrite rather than an old file.
#[test]
fn a_receipt_written_before_derived_identities_still_loads() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[TaskFixture::authored(source_id, AnalysisType::Ac, 2)],
        1,
    );
    let json = serialize_project_file(&project).expect("the project serializes");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("project parses");
    assert!(
        parsed["simulation_results"]["runs"][0]["prepared_receipt"]["tasks"][0]
            .get("derived_from")
            .is_none(),
        "an authored task writes no derivation record at all"
    );

    let restored = load_project_text(&json, None).expect("the project loads");
    assert!(restored.simulation_results_warning.is_none());
}

#[test]
fn an_older_schema_carrying_a_derivation_record_is_refused() {
    let mut project = project_with_execution_context();
    let (source_id, source_revision, plan_id) = authored_ac(&project);
    let derivation = DerivedAnalysisIdentity::new(source_id, run_set_point_role(0))
        .expect("a point role is named");
    attach_run(
        &mut project,
        plan_id,
        source_revision,
        &[TaskFixture::derived(derivation, AnalysisType::Ac, 2)],
        1,
    );
    let json = serialize_project_file(&project).expect("the current-schema project serializes");

    let mut relabelled: serde_json::Value = serde_json::from_str(&json).expect("project parses");
    relabelled["simulation_results"]["schema_version"] =
        serde_json::json!(GOVERNED_SPECIFICATION_RESULTS_SCHEMA_VERSION);
    let relabelled = serde_json::to_string(&relabelled).expect("relabelled project re-encodes");

    let restored = load_project_text(&relabelled, None).expect("the project still opens");
    let warning = restored
        .simulation_results_warning
        .expect("a relabelled file is refused with a stated reason");
    assert!(
        warning.contains("carries a derived task identity that schema could not persist"),
        "{warning}"
    );
}
