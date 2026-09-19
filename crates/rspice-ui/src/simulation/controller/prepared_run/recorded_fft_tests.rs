//! One solve, one deck: what a bound FFT analysis does to preparation.
//!
//! Every case here is about the seam a recorded FFT introduces — the card
//! rides the deck of the transient it is bound to and no other, which is both
//! why the transient's identity moves and why there is never a second solve.

use super::tests::runnable_state;
use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest};
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::SimulationRunIntent;
use crate::workbench::app_state::AppState;

fn edit_transient(
    plan: &mut crate::simulation::plan::SimulationPlan,
    id: AnalysisInstanceId,
    stop: &str,
) {
    plan.edit(id, |draft| {
        let AnalysisDraft::Transient(draft) = draft else {
            panic!("a transient instance owns a transient draft");
        };
        draft.stop = stop.to_owned();
        draft.step = "10u".to_owned();
    })
    .expect("transient edits");
}

/// A runnable project whose plan holds one transient, and optionally one FFT
/// bound to it. `stop` is what the FFT's `To` states; `None` leaves it
/// unauthored, which is the engine's own default of the transient stop time.
fn plan_with_bound_fft(with_fft: Option<Option<&str>>) -> AppState {
    let mut state = runnable_state();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("a runnable project owns a stable plan");
    // The project's own transient, edited rather than joined by a second one:
    // the cases below count transients, and a spare would make the count say
    // nothing.
    let transient = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Transient)
        .expect("a runnable project plans a transient")
        .id();
    edit_transient(plan, transient, "10m");
    if let Some(stop) = with_fft {
        let (fft, _) = plan.insert(AnalysisKind::Fft).expect("FFT inserts");
        plan.edit(fft, |draft| {
            let AnalysisDraft::Fft(draft) = draft else {
                panic!("an FFT instance owns an FFT draft");
            };
            draft.output = "V(out)".to_owned();
            draft.points = 256;
            if let Some(stop) = stop {
                draft.stop = stop.to_owned();
            }
        })
        .expect("FFT edits");
        plan.bind_dependency(fft, AnalysisKind::Transient, transient)
            .expect("FFT binds its transient");
    }
    state
}

fn compile(state: &AppState) -> Vec<crate::simulation::execution::PreparedTask> {
    let controller = SimulationController::new();
    let frozen = controller.build_analysis_plan(state).expect("plan freezes");
    let sealed = state
        .model_library_manager
        .seal_execution_sources()
        .expect("model sources seal");
    controller
        .build_queue_from_plan(state, &frozen, &sealed)
        .expect("plan compiles")
}

fn transient_digest(tasks: &[crate::simulation::execution::PreparedTask]) -> ContentDigest {
    tasks
        .iter()
        .find(|task| matches!(task.queued_analysis().spec, AnalysisSpec::Transient { .. }))
        .expect("the transient task")
        .config_digest()
}

type DispatchedTask = (
    AnalysisSpec,
    Vec<AnalysisInstanceId>,
    AnalysisInstanceId,
    String,
);

fn dispatch_tasks(state: &mut AppState, intent: SimulationRunIntent) -> Vec<DispatchedTask> {
    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(state, intent)
        .expect("the plan prepares");
    controller
        .authorize_snapshot(snapshot)
        .expect("the prepared snapshot authorizes");
    let dispatch = controller
        .consume_snapshot_for_dispatch(state)
        .expect("the authorized snapshot dispatches");
    dispatch
        .tasks()
        .map(|task| {
            (
                task.spec().clone(),
                task.dependencies().to_vec(),
                task.instance_id(),
                task.executable_netlist().to_string(),
            )
        })
        .collect()
}

#[test]
fn a_transient_without_a_bound_fft_keeps_its_config_digest() {
    // The reference is the same transient in a plan that has no FFT at all,
    // so this fails the moment the bound-card wrapper stops being conditional.
    let unbound = compile(&plan_with_bound_fft(None));
    let unbound_transient = unbound
        .iter()
        .find(|task| matches!(task.queued_analysis().spec, AnalysisSpec::Transient { .. }))
        .expect("the transient task");
    assert!(
        unbound_transient.bound_observation_cards().is_empty(),
        "a transient with no bound FFT carries no card"
    );
    let reference = transient_digest(&unbound);

    let bound = compile(&plan_with_bound_fft(Some(Some("8m"))));
    // And with one bound, the transient is a different request: the card adds
    // a solver stop at every requested sample time.
    assert_ne!(transient_digest(&bound), reference);
    assert_eq!(
        bound
            .iter()
            .find(|task| matches!(task.queued_analysis().spec, AnalysisSpec::Transient { .. }))
            .expect("the transient task")
            .bound_observation_cards()
            .len(),
        1
    );
}

#[test]
fn a_bound_fft_card_changes_the_transient_it_rides_on() {
    let tasks = compile(&plan_with_bound_fft(Some(Some("8m"))));
    let card = tasks
        .iter()
        .find(|task| matches!(task.queued_analysis().spec, AnalysisSpec::Fft { .. }))
        .expect("the FFT task")
        .queued_analysis()
        .analysis_line
        .clone();
    assert_eq!(card, ".fft V(out) NP=256 WINDOW=RECT STOP=0.008");
    let transient = tasks
        .iter()
        .find(|task| matches!(task.queued_analysis().spec, AnalysisSpec::Transient { .. }))
        .expect("the transient task");
    assert_eq!(transient.bound_observation_cards(), &[card]);
}

#[test]
fn an_fft_card_reaches_only_the_transient_it_is_bound_to() {
    let mut state = plan_with_bound_fft(Some(Some("8m")));
    // A second, shorter transient the card must never reach: it stops before
    // the card's STOP, so a shared deck would fail it outright.
    let plan = state.sim_setup.analysis_plan.as_mut().expect("stable plan");
    let (short, _) = plan
        .insert(AnalysisKind::Transient)
        .expect("second transient inserts");
    edit_transient(plan, short, "1m");

    let tasks = dispatch_tasks(&mut state, SimulationRunIntent::SimulateRunSet);
    let card = tasks
        .iter()
        .find_map(|task: &DispatchedTask| match &task.0 {
            AnalysisSpec::Fft { request } => Some(request.to_card()),
            _ => None,
        })
        .expect("the FFT task states its card");
    let mut carrying_transients = 0usize;
    let mut carrying_ffts = 0usize;
    for (spec, _, _, deck) in &tasks {
        if !deck.contains(&card) {
            continue;
        }
        match spec {
            AnalysisSpec::Transient { .. } => carrying_transients += 1,
            AnalysisSpec::Fft { .. } => carrying_ffts += 1,
            other => panic!("the card reached {}", other.run_type().display_name()),
        }
    }
    // Exactly one transient carries it, plus the FFT task's inherited copy of
    // that same producer deck. The 1 ms transient prepared and was not
    // refused, which is the whole point of not sharing one deck.
    assert_eq!(carrying_transients, 1);
    assert_eq!(carrying_ffts, 1);
    assert_eq!(
        tasks
            .iter()
            .filter(|(spec, _, _, _)| matches!(spec, AnalysisSpec::Transient { .. }))
            .count(),
        2,
        "both transients prepared; the shorter one was never refused"
    );
}

#[test]
fn an_fft_instance_adds_no_second_transient_solve() {
    let mut state = plan_with_bound_fft(Some(Some("8m")));
    let tasks = dispatch_tasks(&mut state, SimulationRunIntent::SimulateRunSet);
    let transients = tasks
        .iter()
        .filter(|(spec, _, _, _)| matches!(spec, AnalysisSpec::Transient { .. }))
        .map(|(_, _, id, _)| *id)
        .collect::<Vec<_>>();
    let ffts = tasks
        .iter()
        .filter(|(spec, _, _, _)| matches!(spec, AnalysisSpec::Fft { .. }))
        .collect::<Vec<_>>();
    assert_eq!(transients.len(), 1);
    assert_eq!(ffts.len(), 1);
    // The FFT task binds the trajectory the one transient produced. The
    // consumer that publishes its spectrum takes no netlist argument at all
    // (`runner::spec::recorded_fft::run`), so there is nothing for it to
    // solve even if it wanted to.
    assert_eq!(ffts[0].1, transients);
}

#[test]
fn an_fft_window_past_the_bound_transient_stop_is_refused_before_the_run() {
    let mut state = plan_with_bound_fft(None);
    let plan = state.sim_setup.analysis_plan.as_mut().expect("stable plan");
    let transient = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Transient)
        .expect("the transient instance")
        .id();
    let (fft, _) = plan.insert(AnalysisKind::Fft).expect("FFT inserts");
    plan.edit(fft, |draft| {
        let AnalysisDraft::Fft(draft) = draft else {
            panic!("an FFT draft");
        };
        draft.points = 256;
        draft.stop = "20m".to_owned();
    })
    .expect("FFT edits");
    // Refused where the pair is made, which is before any run exists: the
    // engine would fail that transient outright for this card.
    let error = plan
        .bind_dependency(fft, AnalysisKind::Transient, transient)
        .expect_err("a card past the transient stop time cannot bind");
    let message = error.to_string();
    assert!(
        message.contains("STOP 0.02 exceeds transient stop time 0.01"),
        "{message}"
    );
}

#[test]
fn an_fft_analysis_without_an_earlier_transient_is_refused_before_the_run() {
    let mut state = runnable_state();
    let plan = state.sim_setup.analysis_plan.as_mut().expect("stable plan");
    plan.insert(AnalysisKind::Fft).expect("FFT inserts");
    // An unbound FFT is a plan the studio will not freeze, so the refusal
    // arrives before a queue is ever compiled.
    let controller = SimulationController::new();
    let errors = controller
        .build_analysis_plan(&state)
        .expect_err("an unbound FFT cannot freeze");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("has no bound tran prerequisite")),
        "{errors:?}"
    );
}

#[test]
fn a_transient_with_a_fourier_and_an_fft_dependent_hands_both_one_artifact() {
    let mut state = plan_with_bound_fft(Some(Some("8m")));
    let plan = state.sim_setup.analysis_plan.as_mut().expect("stable plan");
    let transient = plan
        .instances()
        .iter()
        .find(|instance| instance.kind() == AnalysisKind::Transient)
        .expect("the transient instance")
        .id();
    let (fourier, _) = plan.insert(AnalysisKind::Fourier).expect("Fourier inserts");
    plan.edit(fourier, |draft| {
        let AnalysisDraft::Fourier(draft) = draft else {
            panic!("Fourier draft");
        };
        draft.fundamental = "1k".to_owned();
        draft.harmonics = "5".to_owned();
        draft.output_node = "out".to_owned();
        draft.start_time = "0".to_owned();
        draft.stop_time = "8m".to_owned();
    })
    .expect("Fourier edits");
    plan.bind_dependency(fourier, AnalysisKind::Transient, transient)
        .expect("Fourier binds the same transient");

    let tasks = dispatch_tasks(&mut state, SimulationRunIntent::SimulateRunSet);
    let consumers = tasks
        .iter()
        .filter(|(spec, _, _, _)| {
            matches!(
                spec,
                AnalysisSpec::Fourier { .. } | AnalysisSpec::Fft { .. }
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(consumers.len(), 2);
    // Both bind the same producer, which is what makes one artifact enough:
    // the controller holds one artifact per producer instance.
    for (_, dependencies, _, _) in consumers {
        assert_eq!(dependencies, &vec![transient]);
    }
}

#[test]
fn an_fft_instance_carries_no_solver_override() {
    use crate::simulation::plan::NumericOverrideOption;

    let mut refusals = 0usize;
    for option in NumericOverrideOption::all() {
        assert_eq!(
            option.refusal_for(AnalysisKind::Fft),
            Some(
                "an FFT analysis runs no solve of its own; state the option on the transient it \
                 is bound to"
            ),
            "{option:?}"
        );
        refusals += 1;
    }
    assert!(refusals > 0);
}

#[test]
fn a_hand_written_fft_deck_is_read_as_the_fft_analysis() {
    const DECK: &str = "recorded fft deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 10u 8m\n\
         .fft V(out) NP=256 WINDOW=HANN\n.fft V(out) NP=512 WINDOW=RECT FORMAT=UNORM\n.end\n";
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.netlist_source = Some(DECK.to_owned());
    let tasks = dispatch_tasks(&mut state, SimulationRunIntent::ManualDeck);
    let transient = tasks
        .iter()
        .find(|(spec, _, _, _)| matches!(spec, AnalysisSpec::Transient { .. }))
        .map(|(_, _, id, _)| *id)
        .expect("the deck's transient");
    let ffts = tasks
        .iter()
        .filter_map(|task: &DispatchedTask| match &task.0 {
            AnalysisSpec::Fft { request } => Some((request.clone(), task.1.clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(ffts.len(), 2);

    let parsed = rspice_core::Netlist::parse(DECK).expect("the deck parses");
    for (index, (request, dependencies)) in ffts.iter().enumerate() {
        assert_eq!(
            *request,
            crate::simulation::config::FftRequest::from_core(&parsed.fft_analyses[index])
        );
        // The engine's rule: every card binds the deck's first transient.
        assert_eq!(dependencies, &vec![transient], "card {index}");
    }
    assert_eq!(ffts[0].0.points, 256);
    assert_eq!(ffts[1].0.points, 512);
}

#[test]
fn a_hand_written_fft_binds_the_first_transient_as_the_engine_does() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    // Two transients, which the Studio refuses for `.FOUR` and the engine
    // accepts for `.FFT`: the card belongs to the first one.
    state.workspace.netlist_source = Some(
        "two transients\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 10u 8m\n.tran 5u 4m\n\
         .fft V(out) NP=256 WINDOW=RECT\n.end\n"
            .to_owned(),
    );
    let tasks = dispatch_tasks(&mut state, SimulationRunIntent::ManualDeck);
    let first_transient = tasks
        .iter()
        .find(|(spec, _, _, _)| matches!(spec, AnalysisSpec::Transient { .. }))
        .map(|(_, _, id, _)| *id)
        .expect("the first transient");
    let fft = tasks
        .iter()
        .find(|(spec, _, _, _)| matches!(spec, AnalysisSpec::Fft { .. }))
        .expect("the FFT task");
    assert_eq!(fft.1, vec![first_transient]);
}

#[test]
fn a_hand_written_fft_without_a_transient_is_refused_in_the_engine_s_words() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.workspace.netlist_source = Some(
        "fft with no transient\nV1 out 0 dc 0 ac 1\nR1 out 0 1k\n.ac dec 10 1 1k\n\
         .fft V(out) NP=256 WINDOW=RECT\n.end\n"
            .to_owned(),
    );
    let controller = SimulationController::new();
    let error = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::ManualDeck)
        .expect_err("a card with no transient to post-process is refused");
    let message = error.to_string();
    assert!(
        message
            .contains(".FFT requires a completed authored .TRAN to post-process in the same deck"),
        "{message}"
    );
}
