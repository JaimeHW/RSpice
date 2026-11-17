//! Models & sections > Add library > From file…, worked from the window: a
//! model file the project has never held is imported and bound to the plan
//! in one gesture, its row names the file and the section it runs, and the
//! run reads the card of the section the row picks. A model file that cannot
//! be read is refused on the page by name.

use egui::accesskit::Role;
use rspice_app_state::state::SimulationRun;

use crate::feature_tests::engineer::{Engineer, Scratch};

/// One diode card in two process sections, its saturation current a hundred
/// times larger in `ff`.
const SECTIONED: &str = "* One diode in two process sections.\n\
                         .lib tt\n.model dx d(is=1e-14)\n.endl tt\n\
                         .lib ff\n.model dx d(is=1e-12)\n.endl ff\n";

/// V(`node`) held at `time`.
fn held_at(run: &SimulationRun, node: &str, time: f64) -> f64 {
    let wave = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .find(|wave| wave.name.eq_ignore_ascii_case(&format!("V({node})")))
        .unwrap_or_else(|| panic!("no V({node}) in the run"));
    let index = wave
        .x
        .iter()
        .rposition(|t| *t <= time)
        .expect("a sample at or before the time");
    wave.y[index]
}

/// The diode's anode, fed from 5 V through 1 kΩ, where the diode's current
/// `is (exp(v/vt) - 1)` meets the resistor's at 27 °C.
fn anode(is: f64) -> f64 {
    let vt = 1.380_649e-23 * 300.15 / 1.602_176_634e-19;
    let mut v: f64 = 0.6;
    for _ in 0..100 {
        let diode = is * ((v / vt).exp() - 1.0);
        let residual = (5.0 - v) / 1e3 - diode;
        let slope = -1e-3 - is * (v / vt).exp() / vt;
        v -= residual / slope;
    }
    v
}

/// A diode on its anode's net, its card in a file the project has never
/// held: From file… imports and binds the file in its `tt` section, the row
/// names the file and the section, and the run reads `tt`'s card; `ff` picked
/// on the row, the next run reads `ff`'s.
#[test]
fn a_model_file_from_the_page_is_bound_and_runs_the_section_its_row_picks() {
    let file = Scratch::new("diodes.lib", SECTIONED);
    let mut e = Engineer::new_project("diodes");
    assert_eq!(
        e.add_model_file(&file.0),
        Ok("diodes".to_owned()),
        "the file binds"
    );
    let shown = e.step(Vec::new()).to_vec();
    for cell in ["diodes", "diodes.lib \u{b7} 1 pinned"] {
        assert!(
            shown
                .iter()
                .any(|control| control.role == Role::Label && control.value == cell),
            "the row reads {cell}"
        );
    }
    assert!(
        shown.iter().any(|control| control.role == Role::ComboBox
            && control.label == "Corner section"
            && control.value == "tt"),
        "the row runs tt: {:?}",
        shown
            .iter()
            .filter(|control| control.role == Role::ComboBox)
            .collect::<Vec<_>>()
    );

    e.top_sheet();
    let source = e.place(
        "voltage source",
        "Voltage Source",
        (100, 100),
        &[("Instance", "v1"), ("Value", "5")],
    );
    e.connect(&source, &["in", "0"]);
    let resistor = e.place(
        "resistor",
        "Resistor",
        (250, 100),
        &[("Instance", "r1"), ("Value", "1k")],
    );
    e.connect(&resistor, &["in", "a"]);
    let diode = e.place(
        "diode",
        "Diode",
        (400, 100),
        &[("Instance", "d1"), ("Value", "dx")],
    );
    e.connect(&diode, &["a", "0"]);
    e.transient("10n", "100n");

    let typical = held_at(&e.run(), "a", 50e-9);
    assert!(
        (typical - anode(1e-14)).abs() <= 1e-3,
        "tt: V(a) {typical} against {}",
        anode(1e-14)
    );

    e.setup_page("Models & sections");
    e.model_section("ff");
    let binding = &e.replay.app.state.model.sim_setup.model_bindings[0];
    assert_eq!(binding.selected_corner.as_deref(), Some("ff"));
    let fast = held_at(&e.run(), "a", 50e-9);
    assert!(
        (fast - anode(1e-12)).abs() <= 1e-3,
        "ff: V(a) {fast} against {}",
        anode(1e-12)
    );
}

/// A model file whose include is not there is refused on the page, by its
/// name and why, and the project neither holds nor binds it.
#[test]
fn a_model_file_that_cannot_be_read_is_refused_on_the_page_by_name() {
    let file = Scratch::new(
        "notes.lib",
        "* Includes a file that is not there.\n.include absent.lib\n",
    );
    let mut e = Engineer::new_project("refused");
    let refusal = e.add_model_file(&file.0).expect_err("the file is refused");
    assert!(
        refusal.contains("notes.lib was not added: ") && refusal.contains("absent.lib"),
        "{refusal}"
    );
    assert!(
        e.replay.app.state.model.sim_setup.model_bindings.is_empty(),
        "nothing is bound"
    );
    assert!(
        e.replay
            .app
            .state
            .model
            .model_library_manager
            .get_library("notes")
            .is_none(),
        "nothing is imported"
    );
}
