//! Signal sets: saved, renamed, deleted and applied, and kept with the
//! project rather than the session.

use rspice_results::result_presentation::{ResultPresentation, SourceWaveformPresentationKey};
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultsTestHost, restore_presentation,
};

use super::super::results_actions::set_result_quantity_ticked;
use super::super::results_sets::{
    apply_signal_set, delete_signal_set, rename_signal_set, save_signal_set,
};
use super::browser::hierarchical_run;

fn key(host: &ResultsTestHost, name: &str) -> ResultBrowserSelectionKey {
    let run = host.model.simulation.active_run().expect("a run");
    ResultBrowserSelectionKey::Waveform(SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        name,
    ))
}

fn plotted(host: &ResultsTestHost) -> Vec<String> {
    let names = host.model.simulation.active_run().expect("a run").analyses[0]
        .waveforms
        .iter()
        .map(|waveform| waveform.name.clone())
        .collect::<Vec<_>>();
    names
        .into_iter()
        .filter(|name| {
            let key = key(host, name);
            crate::workbench::commands::result_navigation::ticked_result_quantities(
                &host.model,
                &host.results.results,
            )
            .contains(&key)
        })
        .collect()
}

#[test]
fn a_signal_set_is_saved_with_the_project_and_reopens_with_it() {
    let mut host = hierarchical_run();
    let keys = [key(&host, "V(out)"), key(&host, "V(x1.a)")];
    let message = save_signal_set(&mut host.results_parts(), "  Probe points ", &keys).unwrap();
    assert_eq!(message, "Saved 2 signals as the set Probe points.");
    assert!(
        save_signal_set(&mut host.results_parts(), "probe POINTS", &keys[..1]).is_ok(),
        "saving under a name in use replaces that set"
    );
    assert_eq!(host.results.results.signal_sets.len(), 1);
    assert_eq!(host.results.results.signal_sets[0].members.len(), 1);
    save_signal_set(&mut host.results_parts(), "Probe points", &keys).unwrap();
    save_signal_set(&mut host.results_parts(), "Spare", &keys[1..]).unwrap();
    assert!(save_signal_set(&mut host.results_parts(), "Empty", &[]).is_err());
    assert_eq!(
        rename_signal_set(&mut host.results_parts(), 1, "probe points"),
        Err("A set named probe points already exists.".to_owned())
    );
    rename_signal_set(&mut host.results_parts(), 1, "Inner node").unwrap();

    // The project file carries the sets beside the run, under their own key.
    let presentation = host
        .results
        .results
        .project_presentation(&host.model.simulation);
    let json = serde_json::to_value(&presentation).unwrap();
    assert_eq!(json["result_signal_sets"][1]["name"], "Inner node");
    assert_eq!(
        json["result_signal_sets"][0]["members"][1]["name"],
        "V(x1.a)"
    );
    let presentation: ResultPresentation = serde_json::from_value(json).unwrap();

    // A project without sets writes no key at all.
    let mut bare = hierarchical_run();
    let bare_json = serde_json::to_value(
        bare.results
            .results
            .project_presentation(&bare.model.simulation),
    )
    .unwrap();
    assert!(bare_json.get("result_signal_sets").is_none());

    // Reopened, the sets are there and apply to the run.
    bare.model.simulation = host.model.simulation.clone();
    restore_presentation(&mut bare.results_parts(), presentation);
    let names = bare
        .results
        .results
        .signal_sets
        .iter()
        .map(|set| set.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["Probe points", "Inner node"]);
    let unplot = key(&bare, "V(in)");
    set_result_quantity_ticked(&mut bare.results_parts(), &unplot, true);
    let message = apply_signal_set(&mut bare.results_parts(), 0).unwrap();
    assert_eq!(message, "Plotted the set Probe points: 2 signals.");
    assert_eq!(plotted(&bare), ["V(out)", "V(x1.a)"]);

    delete_signal_set(&mut bare.results_parts(), 0);
    assert_eq!(bare.results.results.signal_sets.len(), 1);
    assert_eq!(bare.results.results.signal_sets[0].name, "Inner node");
}

#[test]
fn a_set_applied_to_a_run_without_some_members_names_them() {
    let mut host = hierarchical_run();
    let keys = [key(&host, "V(out)")];
    save_signal_set(&mut host.results_parts(), "Out", &keys).unwrap();
    host.results.results.signal_sets[0].members.push(
        rspice_results::result_presentation::SignalSetMember {
            analysis: crate::state::AnalysisType::Transient,
            name: "V(gone)".to_owned(),
            artifact: false,
        },
    );
    let message = apply_signal_set(&mut host.results_parts(), 0).unwrap();
    assert_eq!(
        message,
        "Plotted the set Out: 1 signal; this run did not retain V(gone)."
    );
    assert_eq!(plotted(&host), ["V(out)"]);
}

#[test]
fn a_project_file_s_sets_are_repaired_when_it_opens() {
    use rspice_results::result_presentation::{SignalSet, SignalSetMember};
    let member = |name: &str| SignalSetMember {
        analysis: crate::state::AnalysisType::Transient,
        name: name.to_owned(),
        artifact: false,
    };
    let mut host = hierarchical_run();
    restore_presentation(
        &mut host.results_parts(),
        ResultPresentation {
            signal_sets: vec![
                SignalSet {
                    name: " A ".to_owned(),
                    members: vec![member("V(out)"), member("V(out)")],
                },
                SignalSet {
                    name: "a".to_owned(),
                    members: vec![member("V(in)")],
                },
                SignalSet {
                    name: "   ".to_owned(),
                    members: vec![member("V(in)")],
                },
                SignalSet {
                    name: "Empty".to_owned(),
                    members: Vec::new(),
                },
            ],
            ..Default::default()
        },
    );
    let sets = &host.results.results.signal_sets;
    assert_eq!(sets.len(), 1);
    assert_eq!(sets[0].name, "A");
    assert_eq!(sets[0].members, [member("V(out)")]);
}
