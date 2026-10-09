use super::*;
use crate::execution_options::SpecExecutionOptions;
use rspice_core::NoAbort;
use rspice_core::abort_signal::CountingAbort;
use rspice_simulation_contract::config::{AnalysisConfig, FftRequest, FrequencySweep};

fn request(spec: AnalysisSpec) -> HeadlessTaskRequest {
    let analysis_line = match &spec {
        AnalysisSpec::Fft { request } => request.to_card(),
        _ => "* typed test request".into(),
    };
    HeadlessTaskRequest {
        instance_id: AnalysisInstanceId::new(),
        label: spec.run_type().display_name().into(),
        dependencies: Vec::new(),
        analysis: QueuedAnalysis {
            spec,
            config: None,
            spec_options: SpecExecutionOptions::default(),
            analysis_line,
            numeric_override: None,
        },
    }
}

fn op() -> HeadlessTaskRequest {
    request(AnalysisSpec::dc_op())
}

fn transient() -> HeadlessTaskRequest {
    request(AnalysisSpec::Transient {
        stop_time: 1.0,
        step_time: 0.01,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    })
}

fn fft(producer: AnalysisInstanceId, stop: f64) -> HeadlessTaskRequest {
    let mut task = request(AnalysisSpec::Fft {
        request: FftRequest {
            stop: Some(stop),
            points: 16,
            ..Default::default()
        },
    });
    task.dependencies.push(producer);
    task
}

fn prepare(requests: Vec<HeadlessTaskRequest>) -> ServiceRunResult<Vec<PreparedTask>> {
    prepare_headless_tasks(
        requests,
        ObjectRevision::INITIAL,
        ResourceLimits::default(),
        &NoAbort,
    )
}

#[test]
fn ready_tasks_keep_authored_priority_even_when_a_dependency_finishes_later() {
    let producer = op();
    let mut consumer = op();
    consumer.dependencies.push(producer.instance_id);
    let free_first = op();
    let free_last = op();
    let expected = [
        free_first.instance_id,
        producer.instance_id,
        consumer.instance_id,
        free_last.instance_id,
    ];
    let tasks = prepare(vec![consumer, free_first, producer, free_last]).unwrap();
    assert_eq!(
        tasks
            .iter()
            .map(PreparedTask::instance_id)
            .collect::<Vec<_>>(),
        expected
    );
}

#[test]
fn rejects_empty_duplicate_missing_self_and_cyclic_graphs() {
    assert!(
        prepare(vec![])
            .unwrap_err()
            .to_string()
            .contains("at least one")
    );
    let first = op();
    assert!(
        prepare(vec![first.clone(), first.clone()])
            .unwrap_err()
            .to_string()
            .contains("Duplicate analysis")
    );

    let mut other = op();
    other.dependencies = vec![first.instance_id];
    assert!(
        prepare(vec![other.clone()])
            .unwrap_err()
            .to_string()
            .contains("missing dependency")
    );
    other.dependencies.push(first.instance_id);
    assert!(
        prepare(vec![first.clone(), other.clone()])
            .unwrap_err()
            .to_string()
            .contains("repeats dependency")
    );
    other.dependencies = vec![other.instance_id];
    assert!(
        prepare(vec![other.clone()])
            .unwrap_err()
            .to_string()
            .contains("itself")
    );
    other.dependencies = vec![first.instance_id];
    let mut first = first;
    first.dependencies = vec![other.instance_id];
    assert!(
        prepare(vec![first, other])
            .unwrap_err()
            .to_string()
            .contains("cycle")
    );
}

#[test]
fn preserves_cancellation_and_graph_resource_classification() {
    for threshold in [0, 5] {
        let abort = CountingAbort::new(threshold);
        assert!(matches!(
            prepare_headless_tasks(
                vec![op(), op(), op()],
                ObjectRevision::INITIAL,
                ResourceLimits::default(),
                &abort
            ),
            Err(ServiceRunError::Aborted)
        ));
    }
    let mut limits = ResourceLimits::default();
    limits.max_batch_runs = 1;
    assert!(matches!(
        prepare_headless_tasks(vec![op(), op()], ObjectRevision::INITIAL, limits, &NoAbort),
        Err(ServiceRunError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::BatchRuns,
            requested: 2,
            limit: 1
        }))
    ));
    let first = op();
    let mut second = op();
    second.dependencies.push(first.instance_id);
    let mut limits = ResourceLimits::default();
    limits.max_result_values = 0;
    assert!(matches!(
        prepare_headless_tasks(
            vec![first, second],
            ObjectRevision::INITIAL,
            limits,
            &NoAbort
        ),
        Err(ServiceRunError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::ResultValues,
            requested: 1,
            limit: 0
        }))
    ));
}

#[test]
fn rejects_invalid_specs_and_configurations_that_would_bypass_the_spec() {
    let invalid = request(AnalysisSpec::Ac {
        start_freq: -1.0,
        stop_freq: 1e3,
        points_per_unit: 10,
        sweep: FrequencySweep::Decade,
    });
    assert!(prepare(vec![invalid]).is_err());
    let mut task = transient();
    task.analysis.config = Some(AnalysisConfig::dc_op());
    assert!(
        prepare(vec![task])
            .unwrap_err()
            .to_string()
            .contains("configuration disagrees")
    );
    let producer = transient();
    let mut task = fft(producer.instance_id, 1.0);
    task.analysis.config = Some(AnalysisConfig::dc_op());
    assert!(
        prepare(vec![producer, task])
            .unwrap_err()
            .to_string()
            .contains("typed specification")
    );
}

#[test]
fn binds_fft_to_the_named_transient_after_all_sampling_cards_change_its_digest() {
    let first = transient();
    let second = transient();
    let one = fft(second.instance_id, 0.5);
    let two = fft(second.instance_id, 1.0);
    let expected_cards = vec![
        one.analysis.analysis_line.clone(),
        two.analysis.analysis_line.clone(),
    ];
    let revision = ObjectRevision::new(7).unwrap();
    let tasks = prepare_headless_tasks(
        vec![first, one, two, second],
        revision,
        ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap();
    assert!(tasks[0].bound_observation_cards().is_empty());
    let producer = &tasks[1];
    assert_eq!(producer.bound_observation_cards(), expected_cards);
    for consumer in &tasks[2..] {
        let binding = &consumer.dependency_bindings()[0];
        assert_eq!(binding.producer_instance_id(), producer.instance_id());
        assert_eq!(binding.producer_source_revision(), revision);
        assert_eq!(binding.producer_config_digest(), producer.config_digest());
    }
}

#[test]
fn rejects_absent_ambiguous_and_incompatible_carriers() {
    let first = transient();
    let second = transient();
    let mut task = fft(first.instance_id, 1.0);
    task.dependencies.clear();
    assert!(
        prepare(vec![first.clone(), task.clone()])
            .unwrap_err()
            .to_string()
            .contains("found 0")
    );
    task.dependencies = vec![first.instance_id, second.instance_id];
    assert!(
        prepare(vec![first.clone(), second, task])
            .unwrap_err()
            .to_string()
            .contains("found 2")
    );
    let task = fft(first.instance_id, 2.0);
    assert!(
        prepare(vec![first, task])
            .unwrap_err()
            .to_string()
            .contains("exceeds transient stop")
    );
}

#[test]
fn refuses_an_fft_card_that_does_not_describe_the_typed_transform() {
    let producer = transient();
    let mut task = fft(producer.instance_id, 1.0);
    task.analysis
        .analysis_line
        .push_str("\n.options reltol=0.1");
    assert!(
        prepare(vec![producer, task])
            .unwrap_err()
            .to_string()
            .contains("cards disagree")
    );
}

#[test]
fn explicit_graph_preserves_the_existing_manual_queue_identity_and_payload() {
    let source = "Graph parity\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.op\n.tran .01 1\n.end\n";
    let queue = crate::manual_deck::build_manual_deck_queue(27.0, source).unwrap();
    let manual = super::super::task_preparation::prepare_manual_tasks(
        crate::sealed_source::manual_executable_source_digest(source),
        ObjectRevision::INITIAL,
        queue,
    )
    .unwrap();
    let explicit = prepare(
        manual
            .iter()
            .map(|task| HeadlessTaskRequest {
                instance_id: task.instance_id(),
                label: task.queued_analysis().spec.run_type().display_name().into(),
                dependencies: task.dependencies().to_vec(),
                analysis: task.queued_analysis().clone(),
            })
            .collect(),
    )
    .unwrap();
    for (left, right) in manual.iter().zip(&explicit) {
        assert_eq!(left.instance_id(), right.instance_id());
        assert_eq!(left.config_digest(), right.config_digest());
        assert_eq!(left.source_revision(), right.source_revision());
        assert_eq!(left.dependencies(), right.dependencies());
    }
    assert_eq!(explicit.len(), 2);
}
