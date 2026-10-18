//! A binned family bound by its name, from the schematic to the bin the
//! engine runs.
//!
//! The Properties picker binds `nch`, never one of its bins; these cases hold
//! that the prepared deck names `nch` with the library's every bin beside it,
//! and that the engine — not the deck — chooses the bin from the instance's L
//! and W, or refuses an instance no bin holds.

use super::*;
use crate::state::ComponentType;
use rspice_design::drc::DrcResult;
use std::fs;

/// Two bins of one NMOS family, far enough apart in threshold that the
/// inverter's output tells them apart.
const FAMILY: &str = "\
.model nch.1 nmos level=1 vto=0.3 kp=200u lmin=0.1u lmax=1u wmin=0.1u wmax=100u
.model nch.2 nmos level=1 vto=1.2 kp=60u lmin=1u lmax=10u wmin=0.1u wmax=100u
";

/// The CMOS inverter example with its pull-down bound to the family `nch`
/// from a library file holding [`FAMILY`], at `length`.
fn inverter_bound_to_the_family(label: &str, length: &str) -> (AppState, std::path::PathBuf) {
    let directory = std::env::temp_dir().join(format!(
        "rspice-binned-family-{label}-{}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).expect("fixture directory");
    let path = directory.join("kit.lib");
    fs::write(&path, FAMILY).expect("write the family");

    let mut state = AppState::default();
    for dimension in &mut state.sim_setup.run_set.dimensions {
        if dimension.kind
            == rspice_simulation_contract::run_set::RunSetDimensionKind::ProcessSection
        {
            dimension.enabled = false;
        }
    }
    crate::workbench::examples::load_example("CMOS Inverter", &mut state.schematic);
    let library = state
        .model_library_manager
        .load_library_file(&path, None)
        .expect("load the kit");
    state.sim_setup.model_bindings.push(
        state
            .model_library_manager
            .simulation_plan_binding(&library)
            .expect("bind the kit to the plan"),
    );
    let pull_down = state
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.kind == ComponentType::Nmos)
        .expect("the inverter's NMOS");
    pull_down.value = "nch".to_owned();
    pull_down.params = format!("w=1u l={length} model_library={library}");
    let mut drc = DrcResult::new();
    drc.completed = true;
    state.dialogs.drc_results = Some(drc);
    state.dialogs.drc_checked_version = state.schematic.topology_version();
    (state, directory)
}

/// The executable deck the run set dispatches.
fn prepared_deck(state: &AppState) -> String {
    SimulationController::build_prepared_snapshot(state, SimulationRunIntent::SimulateRunSet)
        .expect("the bound family prepares")
        .executable_netlist()
        .to_owned()
}

/// Whether a deck line is the pull-down's card: the one MOSFET naming `nch`.
fn is_pull_down(line: &str) -> bool {
    let mut tokens = line.split_whitespace();
    tokens
        .next()
        .is_some_and(|name| name.to_ascii_uppercase().starts_with('M'))
        && tokens.any(|token| token == "nch")
}

/// The deck with the pull-down's model replaced by `model`.
fn naming(deck: &str, model: &str) -> String {
    deck.lines()
        .map(|line| {
            if is_pull_down(line) {
                line.replace(" nch ", &format!(" {model} "))
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn output(deck: &str) -> Vec<f64> {
    let netlist = rspice_core::Netlist::parse(deck).expect("the deck parses");
    rspice_core::engine::Engine::new(rspice_core::SimulationConfig::default())
        .run_tran(&netlist, 20e-9, 0.1e-9)
        .expect("the transient runs")
        .try_voltage_waveform_named("out")
        .expect("the output node")
        .to_vec()
}

/// The deck names the family, carries every bin, and the transient runs the
/// bin whose window holds the instance's length: bit for bit what naming that
/// bin would run, and not what the other bin runs.
#[test]
fn a_family_bound_by_name_runs_the_bin_its_length_selects() {
    let (state, directory) = inverter_bound_to_the_family("selects", "2u");
    let deck = prepared_deck(&state);
    assert_eq!(
        deck.lines().filter(|line| is_pull_down(line)).count(),
        1,
        "{deck}"
    );
    assert!(deck.contains(".model nch.1"), "{deck}");
    assert!(deck.contains(".model nch.2"), "{deck}");

    let netlist = rspice_core::Netlist::parse(&deck).expect("the deck parses");
    let inspection = rspice_core::engine::Engine::new(rspice_core::SimulationConfig::default())
        .inspect_model_bins(&netlist)
        .expect("every instance resolves");
    let decision = inspection
        .instances
        .iter()
        .find(|instance| instance.requested_model.eq_ignore_ascii_case("nch"))
        .expect("the pull-down's bin decision");
    assert!(
        decision.selected_model.eq_ignore_ascii_case("nch.2"),
        "{decision:?}"
    );

    let run = output(&deck);
    assert_eq!(run, output(&naming(&deck, "nch.2")));
    assert_ne!(run, output(&naming(&deck, "nch.1")));
    fs::remove_dir_all(directory).expect("remove the fixture");
}

/// A length no bin holds is refused when the run is prepared, by the engine's
/// own bin resolution, which names the instance and the family: the binding
/// is sound, and the instance's size is what has to change.
#[test]
fn a_length_no_bin_holds_is_refused_when_the_run_is_prepared() {
    let (state, directory) = inverter_bound_to_the_family("uncovered", "25u");
    let Err(refusal) =
        SimulationController::build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
    else {
        panic!("no bin holds L = 25um, and the run prepared");
    };
    assert_eq!(refusal.stage(), PreparationStage::ModelBindings);
    let message = refusal.message().to_ascii_lowercase();
    assert!(
        message.contains("mosfet 'mn' geometry is uncovered by model family 'nch'"),
        "{refusal}"
    );
    fs::remove_dir_all(directory).expect("remove the fixture");
}
