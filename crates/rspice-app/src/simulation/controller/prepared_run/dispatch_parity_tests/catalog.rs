//! Every configured analysis must survive the actual saved-plan run path.
use super::*;
use crate::simulation::plan::AnalysisDraft;

fn configure_fixture_inputs(draft: &mut AnalysisDraft) {
    match draft {
        AnalysisDraft::Noise(draft) => {
            draft.input = FIXTURE_TONE_SOURCE.into();
            draft.output = "out".into();
        }
        AnalysisDraft::TransferFunction(draft) => {
            draft.input_source = FIXTURE_TONE_SOURCE.into();
            draft.output_expression = "V(out)".into();
        }
        AnalysisDraft::Pss(draft) => draft.tone_sources = FIXTURE_TONE_SOURCE.into(),
        AnalysisDraft::Envelope(draft) => {
            draft.modulation_sources = FIXTURE_TONE_SOURCE.into();
        }
        AnalysisDraft::Hbnoise(draft) => draft.source_resistor = "R1".into(),
        AnalysisDraft::Fft(draft) => draft.output = "V(out)".into(),
        _ => {}
    }
}

#[test]
fn every_configured_analysis_survives_save_prepare_and_dispatch() {
    let mut failures = Vec::new();
    for kind in AnalysisKind::ALL {
        let result = (|| -> Result<(), String> {
            let mut state = preflight_ready_state();
            let ids = only(&mut state, &with_prerequisites(kind));
            let own_id = *ids.last().unwrap();
            drive_pss_from_the_fixture_supply(&mut state);
            for id in ids {
                plan_mut(&mut state)
                    .edit(id, configure_fixture_inputs)
                    .map_err(|error| error.to_string())?;
            }
            let saved = serde_json::to_vec(plan_mut(&mut state)).unwrap();
            state.sim_setup.analysis_plan = Some(serde_json::from_slice(&saved).unwrap());
            plan_mut(&mut state).prepare_after_restore();

            let mut controller = SimulationController::new();
            let frozen = controller
                .build_analysis_plan(&state)
                .map_err(|e| e.join("; "))?;
            let queue = compiled_queue(&state).map_err(|e| e.join("; "))?;
            let own_task = queue
                .iter()
                .find(|task| task.instance_id() == own_id)
                .ok_or("configured analysis has no queued task")?;
            assert_eq!(
                crate::simulation::execution::analysis_kind_tag(&own_task.queued_analysis().spec),
                kind.canonical_kind().tag(),
                "{kind:?} task carries the wrong tag"
            );
            for instance in frozen.instances() {
                let task = queue
                    .iter()
                    .find(|task| task.instance_id() == instance.id())
                    .ok_or("an enabled analysis disappeared from the queue")?;
                let mut projected = state.clone();
                projected
                    .sim_setup
                    .apply_analysis_draft_projection(instance.draft());
                let displayed =
                    controller.analysis_draft_directive(&projected, instance.draft())?;
                assert_eq!(task.queued_analysis().analysis_line, displayed, "{kind:?}");
            }

            let snapshot = controller
                .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
                .map_err(|error| error.to_string())?;
            let receipt = snapshot
                .prepared_run_receipt()
                .map_err(|error| error.to_string())?;
            controller
                .authorize_snapshot(snapshot)
                .map_err(|error| error.to_string())?;
            let dispatch = controller
                .consume_snapshot_for_dispatch(&mut state)
                .map_err(|error| error.to_string())?;
            assert_eq!(
                receipt,
                dispatch
                    .prepared_run_receipt(crate::state::AnalysisResultSourceDomain::SimulationPlan)
                    .map_err(|error| error.to_string())?
            );
            assert!(
                dispatch
                    .into_tasks()
                    .iter()
                    .any(|task| task.instance_id() == own_id)
            );
            Ok(())
        })();
        if let Err(error) = result {
            failures.push(format!("{kind:?}: {error}"));
        }
    }
    assert!(
        failures.is_empty(),
        "configured analyses failed:\n{}",
        failures.join("\n")
    );
}
