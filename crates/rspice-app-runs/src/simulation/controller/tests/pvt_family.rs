//! Project persistence for authenticated PVT point families.

use crate::state::{AnalysisResultFamilyMetadata, AnalysisType};

const DERATED_DIVIDER: &str = "temperature family\n\
     VDD vdd 0 DC 1.8 AC 1\n\
     R1 vdd out 1k TC1=0.01\n\
     R2 out 0 1k\n\
     C1 out 0 1p\n\
     .step temp list -40 27 125\n\
     .op\n\
     .end\n";

/// A run's retained results have to be an exact ordered prefix of the task
/// graph its receipt authenticated, and that is checked on project load as a
/// hard error rather than a warning. The family is a result, so it has to be a
/// task: a family assembled outside the task list would leave a saved run
/// carrying one more result than it had authenticated tasks, and the project
/// would refuse to reopen.
#[test]
fn a_temperature_run_is_an_authentic_prefix_of_its_receipt_and_survives_a_project_round_trip() {
    let run = super::super::test_execution::run_manual_batch(DERATED_DIVIDER);
    run.validate_provenance()
        .expect("every retained result answers for an authenticated task");
    assert_eq!(
        run.prepared_receipt()
            .expect("the run is sealed by its dispatch")
            .tasks()
            .len(),
        run.analyses.len(),
        "three points and the family they add up to, against four authenticated tasks"
    );

    let mut surplus = run.clone();
    let duplicate = surplus.analyses[0].clone();
    surplus.analyses.push(duplicate);
    surplus
        .validate_provenance()
        .expect_err("a result with no authenticated task cannot be retained");

    let mut simulation = crate::state::SimulationState::default();
    simulation.retained.next_run_id = run.id;
    simulation.retained.runs = vec![run].into();

    let persisted = crate::io::capture_simulation_results(&simulation);
    persisted
        .validate()
        .expect("a temperature run is persistable");

    let mut reloaded = crate::state::SimulationState::default();
    crate::io::restore_simulation_results(persisted, &mut reloaded)
        .expect("a saved temperature run reopens");

    let restored = reloaded
        .retained
        .runs
        .first()
        .expect("the run survives the reload");
    assert_eq!(
        restored.analyses.len(),
        simulation.retained.runs[0].analyses.len()
    );
    assert!(
        restored.analyses.iter().any(
            |analysis| analysis.analysis_type == AnalysisType::Parametric
                && matches!(
                    analysis.family_metadata,
                    Some(AnalysisResultFamilyMetadata::Parametric { .. })
                )
        ),
        "the parametric plot's axis survives the round trip"
    );
    restored
        .validate_provenance()
        .expect("the reloaded run is still an authentic prefix");
}

#[test]
fn a_changed_pvt_basis_keeps_successful_points_and_the_failed_family_after_reload() {
    const CONDITIONAL: &str = "Conditional family\n\
        VDD vdd 0 DC 1.8 AC 1\n\
        VBIAS bias 0 0\n\
        R1 vdd out 1k\n\
        R2 out 0 1k\n\
        .if (TEMPER < 50)\n\
        R3 vdd extra 1k\n\
        R4 extra 0 1k\n\
        .endif\n";
    for temperatures in ["27 85", "85 27"] {
        for analysis in [
            ".op",
            ".dc VDD 0 1.8 0.9",
            ".dc VDD 0 1.8 0.9 VBIAS 0 0.1 0.1",
            ".tran 1e-8 1e-6",
            ".tran 1e-8 1e-6 0.5e-6 1e-8",
            ".ac dec 2 1e3 1e4",
        ] {
            let source = format!("{CONDITIONAL}.step temp list {temperatures}\n{analysis}\n.end\n");
            let run = super::super::test_execution::run_manual_batch_with_lifecycle(
                &source,
                rspice_results::run::SimulationRunLifecycle::Failed,
            );
            let mut state = crate::state::SimulationState::default();
            state.retained.next_run_id = run.id;
            state.retained.runs = vec![run].into();
            let persisted = crate::io::capture_simulation_results(&state);
            persisted
                .validate()
                .expect("successful points and a failed family remain saveable");
            let decoded = serde_json::from_slice(&serde_json::to_vec(&persisted).unwrap()).unwrap();
            let mut restored = crate::state::SimulationState::default();
            crate::io::restore_simulation_results(decoded, &mut restored).unwrap();
            let restored = &restored.retained.runs[0];
            assert_eq!(restored.analyses.len(), 3);
            assert!(restored.analyses[..2].iter().all(|point| point.success));
            let family = &restored.analyses[2];
            assert!(!family.success);
            assert!(family.waveforms.is_empty());
            assert!(
                family
                    .error_message
                    .as_deref()
                    .unwrap()
                    .contains("point 2 changed the solved node basis")
            );
            restored.validate_provenance().unwrap();
        }
    }
}
