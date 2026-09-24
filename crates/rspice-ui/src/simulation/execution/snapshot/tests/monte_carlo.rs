//! Snapshot tests bind multipoint checkpoint resumes to selected trial populations.

use super::*;
use crate::simulation::dialog::McVariationSource;
use crate::simulation::runner::monte_carlo_checkpoint::{
    MonteCarloCheckpointInput, MonteCarloCheckpointRequest,
};
use crate::simulation::runner::study::{
    StudyRunConfig,
    monte_carlo::{
        MonteCarloContinuation, checkpoint::StudyMonteCarloCheckpoint,
        run_monte_carlo_with_continuation, source_with_statistics,
    },
};
use rspice_core::{NoAbort, ResourceLimits};
use std::sync::atomic::{AtomicUsize, Ordering};

fn experiment() -> SnapshotParts {
    let mut parts = parts();
    let analysis = AnalysisConfig::dc_op();
    let base = StudyRunConfig {
        postprocess: None,
        constraints: vec![],
        objective_terms: vec![],
        instance_id: instance_id("mc-base"),
        source_revision: ObjectRevision::INITIAL,
        analysis_line: analysis.to_spice(),
        analysis: analysis.into(),
        numeric_options: ".OPTIONS RELTOL=1e-5".into(),
        measurements: vec!["scalar:V(out)".into()],
        histogram_bins: 5,
    };
    parts.tasks = vec![prepared(
        "mc",
        "MC",
        QueuedAnalysis {
            spec: AnalysisSpec::MonteCarlo {
                variation_source: McVariationSource::ParameterTolerance,
                params: vec![],
            },
            config: None,
            analysis_line: ".mc 3 uniform 0.2 seed 37".into(),
            numeric_override: None,
            spec_options: SpecExecutionOptions {
                study_base: Some(base),
                mc_checkpoint: Some(MonteCarloCheckpointRequest {
                    publish_every: 1.try_into().unwrap(),
                    trial_range: None,
                    resume: None,
                }),
                ..Default::default()
            },
        },
    )];
    parts.executable_netlist = "Point resume\n.param r=1k\nV1 in 0 1\nR1 in out {r}\nR2 out 0 1k\n.mc 3 uniform 0.2 seed 37\n.end\n".into();
    parts.run_set = Some(global_parameter_run_set("r", &["1k", "2k", "3k"]));
    parts
}

fn execute(
    task: &PreparedTask,
    source: &str,
    range: Option<std::ops::Range<usize>>,
) -> (
    crate::services::simulation_runner::MonteCarloData,
    StudyMonteCarloCheckpoint,
    usize,
) {
    let options = &task.task.spec_options;
    let policy = options.mc_checkpoint.as_ref().unwrap();
    let mut journal = policy.resume.as_ref().map(|input| input.decode().unwrap());
    let source = source_with_statistics(
        task.executable_netlist_override
            .as_deref()
            .unwrap_or(source),
        McVariationSource::ParameterTolerance,
        options.mc_statistics.as_ref(),
    )
    .unwrap();
    let publications = AtomicUsize::new(0);
    let result = run_monte_carlo_with_continuation(
        options.study_base.as_ref().unwrap(),
        McVariationSource::ParameterTolerance,
        &source,
        None,
        task.execution_environment.clone(),
        &NoAbort,
        Some(MonteCarloContinuation {
            checkpoint: &mut journal,
            trial_range: range,
            publish_every: policy.publish_every,
            publish: &|_| {
                publications.fetch_add(1, Ordering::Relaxed);
                Ok(())
            },
        }),
    )
    .unwrap();
    (
        result,
        journal.unwrap(),
        publications.load(Ordering::Relaxed),
    )
}

fn selected(checkpoints: &[StudyMonteCarloCheckpoint]) -> Vec<PreparedMonteCarloResume> {
    checkpoints
        .iter()
        .map(|journal| {
            let bytes = journal
                .to_bytes_with_limits(ResourceLimits::default(), &NoAbort)
                .unwrap();
            PreparedMonteCarloResume::from_input(
                MonteCarloCheckpointInput::from_bytes(bytes).unwrap(),
            )
            .unwrap()
        })
        .collect()
}

#[test]
fn monte_carlo_multi_point_resume_routes_exact_trials_and_keeps_unselected_points() {
    let fresh = PreparedRunSnapshot::new(experiment()).unwrap();
    assert_eq!(fresh.tasks.len(), 3);
    let checkpoints = fresh.tasks[..2]
        .iter()
        .map(|task| execute(task, &fresh.executable_netlist, Some(0..1)).1)
        .collect::<Vec<_>>();
    assert_ne!(
        checkpoints[0].population_identity(),
        checkpoints[1].population_identity()
    );
    let mut parts = experiment();
    let task = parts
        .tasks
        .remove(0)
        .with_monte_carlo_resumes(selected(&checkpoints));
    assert_ne!(task.config_digest(), fresh.tasks[0].config_digest());
    parts.tasks.push(task);
    let resumed = PreparedRunSnapshot::new(parts).unwrap();
    assert_eq!(resumed.tasks.len(), 3);
    let mut values = Vec::new();
    for (index, (task, fresh_task)) in resumed.tasks.iter().zip(&fresh.tasks).enumerate() {
        assert!(
            task.monte_carlo_resumes().is_empty(),
            "only the routed input reaches dispatch"
        );
        assert_eq!(
            task.task
                .spec_options
                .mc_checkpoint
                .as_ref()
                .unwrap()
                .resume
                .is_some(),
            index < 2
        );
        let (actual, journal, publications) = execute(task, &resumed.executable_netlist, None);
        let (expected, _, _) = execute(fresh_task, &fresh.executable_netlist, None);
        assert_eq!(
            publications,
            if index < 2 { 2 } else { 3 },
            "missing trials only"
        );
        assert_eq!(journal.completed_trials(), 3);
        assert_eq!(actual.trial_measurements, expected.trial_measurements);
        assert_eq!(actual.variables[0].samples, expected.variables[0].samples);
        values.push(actual.variables[0].samples.clone());
    }
    assert_ne!(values[0], values[1]);
    assert_ne!(values[1], values[2]);

    let mut changed = experiment();
    changed.tasks[0]
        .task
        .spec_options
        .study_base
        .as_mut()
        .unwrap()
        .numeric_options = ".OPTIONS RELTOL=1e-3".into();
    let task = changed
        .tasks
        .remove(0)
        .with_monte_carlo_resumes(selected(&checkpoints));
    changed.tasks.push(task);
    assert!(
        PreparedRunSnapshot::new(changed)
            .unwrap_err()
            .to_string()
            .contains("matches no requested Run Set point")
    );
    let mut narrowed = experiment();
    narrowed.run_set = Some(global_parameter_run_set("r", &["1k", "3k"]));
    let task = narrowed
        .tasks
        .remove(0)
        .with_monte_carlo_resumes(selected(&checkpoints));
    narrowed.tasks.push(task);
    assert!(
        PreparedRunSnapshot::new(narrowed)
            .unwrap_err()
            .to_string()
            .contains("matches no requested Run Set point")
    );
}
