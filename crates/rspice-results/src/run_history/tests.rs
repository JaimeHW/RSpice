//! Revision invalidation through the canonical retained-history owner.

use super::*;
use crate::run::ExecutionTarget;

fn history() -> RunHistory {
    vec![
        SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop),
        SimulationRun::new(2, 0.0, ExecutionTarget::LocalDesktop),
    ]
    .into()
}

#[test]
fn history_reads_and_clones_preserve_revision_but_forked_edits_do_not() {
    let mut original = history();
    let revision = original.revision();
    let mut fork = original.clone();
    assert_eq!(fork.revision(), revision);
    assert_eq!(original.len(), 2);
    assert_eq!(
        original.iter().map(|run| run.id).collect::<Vec<_>>(),
        [1, 2]
    );
    for run in &original {
        assert!(run.id > 0);
    }
    assert_eq!(original.revision(), revision);
    original.prune_runs(2);
    original.prune_plan_runs(SimulationPlanId::new(), 1, None);
    assert_eq!(original.newest_retained_result_run_index(), None);
    assert_eq!(
        original.retained_plan_dataset_count(SimulationPlanId::new()),
        0
    );
    assert_eq!(original.pinned_plan_run_count(SimulationPlanId::new()), 0);
    assert_eq!(original.revision(), revision);
    fork[0].label = "Edited".to_owned();
    assert_ne!(fork.revision(), revision);
    assert_eq!(original.revision(), revision);
    assert_ne!(original[0].label, "Edited");
}

#[test]
fn every_mutable_history_access_invalidates_a_retained_revision() {
    let edits: [fn(&mut RunHistory); 11] = [
        |runs| runs.push(SimulationRun::new(3, 0.0, ExecutionTarget::LocalDesktop)),
        |runs| {
            runs.pop();
        },
        |runs| runs.swap(0, 1),
        |runs| runs.get_mut(0).unwrap().success = false,
        |runs| runs.iter_mut().next().unwrap().label.clear(),
        |runs| {
            for run in runs {
                run.label.clear();
            }
        },
        |runs| runs.retain(|run| run.id == 1),
        |runs| runs.clear(),
        |runs| runs.prune_runs(1),
        |runs| {
            let id = runs[0].run_id;
            assert!(runs.set_run_retention(id, RunRetention::GoldenBaseline));
        },
        |runs| {
            assert!(!runs.set_run_retention(RunId::new(), RunRetention::GoldenBaseline));
        },
    ];
    for edit in edits {
        let mut runs = history();
        let revision = runs.revision();
        edit(&mut runs);
        assert_ne!(runs.revision(), revision);
    }
}

#[test]
fn replacing_a_history_cannot_reuse_a_retained_revision() {
    let mut runs = history();
    let revision = runs.revision();
    let retained = std::mem::take(&mut runs);
    assert_ne!(runs.revision(), revision);
    assert_eq!(retained.revision(), revision);
    runs = history();
    assert_ne!(runs.revision(), revision);
    runs = retained;
    assert_eq!(runs.revision(), revision);
}
