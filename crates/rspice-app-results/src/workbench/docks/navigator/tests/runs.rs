//! A run's row: renamed, noted, pinned, deleted and restored, as
//! Assembler's History tab renames, notes, locks and deletes a history item.

use super::super::results_runs::{
    delete_refusal, delete_run, rename_run, restore_deleted_runs, restore_refusal, set_run_pinned,
    write_run_note,
};
use super::browser::hierarchical_run;

#[test]
fn a_runs_row_names_notes_pins_deletes_and_restores_it() {
    let mut host = hierarchical_run();
    let run = &host.model.simulation.retained.runs[0];
    let (dataset, stamp) = (run.dataset_id, run.start_stamp());

    rename_run(&mut host.results_parts(), dataset, "  Bias sweep ").unwrap();
    assert_eq!(
        rename_run(&mut host.results_parts(), dataset, "  "),
        Err("A run needs a name.".to_owned())
    );
    write_run_note(&mut host.results_parts(), dataset, "ripple fixed ").unwrap();
    let run = &host.model.simulation.retained.runs[0];
    assert_eq!(run.name(), "Bias sweep");
    assert_eq!(
        Some(run.label.clone()),
        stamp.map(|stamp| format!("Bias sweep ({stamp})")),
        "the label goes on carrying the start"
    );
    assert_eq!(run.note, "ripple fixed");

    set_run_pinned(&mut host.results_parts(), dataset, true).unwrap();
    assert!(
        host.model.simulation.retained.runs[0]
            .retention()
            .is_pinned()
    );
    let refusal = Some("The run is pinned; unpin it to delete it.".to_owned());
    assert_eq!(delete_refusal(&host.results_parts(), dataset), refusal);
    assert_eq!(
        delete_run(&mut host.results_parts(), dataset),
        Err(refusal.unwrap())
    );
    assert_eq!(
        host.model.simulation.retained.runs.len(),
        1,
        "a pinned run stays"
    );

    set_run_pinned(&mut host.results_parts(), dataset, false).unwrap();
    assert_eq!(
        restore_refusal(&host.results_parts()),
        Some("No run has been deleted since the project opened.")
    );
    delete_run(&mut host.results_parts(), dataset).unwrap();
    assert!(host.model.simulation.retained.runs.is_empty());
    assert_eq!(
        restore_deleted_runs(&mut host.results_parts()),
        Ok("Restored 1 run.".to_owned())
    );
    let run = &host.model.simulation.retained.runs[0];
    assert_eq!(
        (run.dataset_id, run.name(), run.note.as_str()),
        (dataset, "Bias sweep", "ripple fixed"),
        "the run comes back as the reader left it"
    );
}

#[test]
fn a_run_cannot_be_deleted_or_restored_while_a_run_owns_the_history() {
    let mut host = hierarchical_run();
    let dataset = host.model.simulation.retained.runs[0].dataset_id;
    host.model.simulation.execution.is_running = true;
    let running = "A run is in progress; the history can change when it ends.";
    assert_eq!(
        delete_refusal(&host.results_parts(), dataset).as_deref(),
        Some(running)
    );
    assert!(delete_run(&mut host.results_parts(), dataset).is_err());
    assert_eq!(host.model.simulation.retained.runs.len(), 1);
}
