//! The Verilog-A device flow, driven end to end the way a user drives it.
//!
//! One project: File → New project, a right-click on the empty canvas and
//! New Verilog-A device…, the module written and compiled,
//! Place in the top schematic and a placement click, the wiring, `gain` set in
//! the inspector, Run plan and the operating point; then the project saved,
//! reopened from disk and run again. Each step goes through the entry point
//! the user's gesture reaches, every frame is a whole application frame, and
//! no configuration is ever made by hand.

use crate::state::CellViewRef;
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_flow_tests::{
    CELL, MODULE, PORTS, User, assert_five_volts, compile, new_device, new_project, run_plan,
    run_plan_of_instance, wired_device,
};
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::state::Workspace;

/// A device bound to a card of the project's model library runs with the
/// card's parameters: the output follows the card's gain of 2, not the
/// module's default of 10. The value the instance had set is the card's to
/// set now, so it is flagged and removed first.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_device_bound_to_a_library_card_runs_with_the_cards_parameters() {
    let (mut user, device) = wired_device();
    let library = user
        .app
        .state
        .model
        .model_library_manager
        .load_library_bytes(
            "cards.lib",
            format!(".model doubler {CELL} gain=2\n").into_bytes(),
            None,
        )
        .expect("the card's library loads");
    let binding = user
        .app
        .state
        .model
        .model_library_manager
        .simulation_plan_binding(&library)
        .expect("the library binds to the plan");
    user.app.state.model.sim_setup.model_bindings = vec![binding];
    let instance = user
        .app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| {
            component
                .library_cell
                .as_ref()
                .is_some_and(|binding| binding.cell == CELL)
        })
        .expect("the placed device")
        .id;
    assert!(
        user.app
            .state
            .model
            .schematic
            .select_instance_model_card(instance, Some("doubler".to_owned()))
    );
    let bound = user.component(instance);
    assert!(
        user.app
            .state
            .model
            .model_card_owns_parameter(&bound, "gain")
    );
    assert_eq!(
        user.app
            .state
            .model
            .remove_undeclared_instance_parameter(instance, "gain"),
        Ok(true)
    );
    user.settle();

    let outcome = run_plan_of_instance(&mut user, &device, Some("doubler"));
    assert!(
        outcome
            .out_volts
            .iter()
            .all(|volts| ((volts - 2.0) / 2.0).abs() <= 1e-9),
        "V(out) = {:?} V from:\n{}",
        &outcome.out_volts[..outcome.out_volts.len().min(8)],
        outcome.deck
    );
}

/// The whole flow, twice: once as built, and once from the project file.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_new_veriloga_device_runs_from_the_canvas_to_its_operating_point_and_back_from_disk() {
    use crate::workbench::lifecycle::project_lifecycle::{
        DestinationAuthority, SaveScope, save_native,
    };

    let (mut user, device) = wired_device();
    // 8 and 9.
    assert_five_volts(&run_plan(&mut user, &device));

    // 10. Saved, reopened from disk by a fresh application, and run again.
    let path = std::env::temp_dir().join(format!(
        "rspice-va-flow-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    save_native(
        &mut user.app.state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("the project saves");
    let top = user
        .app
        .state
        .model
        .workspace
        .content
        .simulation_root_reference();
    drop(user);

    let mut reopened = User::new(RSpiceApp::test_instance());
    assert!(
        reopened.app.state.load_project_from_path(&path),
        "the project reopens"
    );
    reopened.app.state.open_workspace_view(top);
    reopened.app.state.workbench.activate(Workspace::Design);
    reopened.settle();
    assert_five_volts(&run_plan(&mut reopened, &device));

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
}

/// A behavioural op-amp as a reviewer writes it: both divisions are by
/// quantities the parameter ranges keep positive, and the clip reassigns a
/// variable assigned on every path just before it.
const OPAMP: &str = r#"`include "constants.vams"
`include "disciplines.vams"

/* Behavioural op-amp macromodel:
   single pole, slew limited, clipped to the rails. */
module opamp_bhv(inp, inn, out, vdd, vss);
  input inp, inn;
  output out;
  inout vdd, vss;
  electrical inp, inn, out, vdd, vss, mid;

  (* desc="Open-loop gain", units="V/V" *) parameter real a0 = 1e5 from (0:inf);
  (* desc="Gain-bandwidth product", units="Hz" *) parameter real gbw = 10M from (0:inf);
  (* desc="Slew rate", units="V/s" *) parameter real sr = 5M from (0:inf);
  (* desc="Output resistance", units="Ohm" *) parameter real rout = 50 from (0:inf);
  parameter integer clip = 1 from [0:1];

  real vin, vslew, vtarget, tau;

  analog begin
    tau = a0 / (`M_TWO_PI * gbw);
    vin = V(inp, inn);
    (* integrator node *)
    I(mid) <+ -a0 * vin;
    I(mid) <+ V(mid) + tau * ddt(V(mid));
    vslew = slew(V(mid), sr, -sr);
    vtarget = vslew;
    if (clip)
      vtarget = min(max(vslew, V(vss)), V(vdd));
    I(out) <+ (V(out) - vtarget) / rout;
    I(inp, inn) <+ white_noise(4 * `P_K * $temperature * 1k, "input");
  end
endmodule
"#;

impl User {
    /// Press and release `key`, as a keyboard does.
    fn key(&mut self, key: egui::Key, modifiers: egui::Modifiers) {
        for pressed in [true, false] {
            self.frame(vec![egui::Event::Key {
                key,
                physical_key: Some(key),
                pressed,
                repeat: false,
                modifiers,
            }]);
        }
    }

    /// The one-based line and column of the caret the Verilog-A editor last
    /// reported.
    fn caret(&self) -> (usize, usize) {
        let selected =
            crate::workbench::documents::code_workspace::selected_veriloga_source(&self.app)
                .expect("a Verilog-A view is open");
        let path = selected.document().logical_path().to_owned();
        let source = selected
            .bundle()
            .file_content(&path)
            .expect("the open document")
            .to_owned();
        let index = *self
            .app
            .state
            .code
            .code_workspace
            .source_carets
            .get(&(selected.bundle().id(), path))
            .expect("the editor reported its caret");
        source
            .chars()
            .take(index)
            .fold((1, 1), |(line, column), character| {
                if character == '\n' {
                    (line + 1, 1)
                } else {
                    (line, column + 1)
                }
            })
    }

    fn prompt(&self) -> String {
        self.app.state.prompt_text().unwrap_or_default().to_owned()
    }
}

/// The reviewed op-amp compiles with nothing to say about its bounded
/// divisions or its clip, and says so where a screen reader hears it. With two
/// typos it fails on two errors, each listed on the line it is on, and each
/// reached from Problems, from the inspector and with F8 and Shift+F8.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_reviewed_op_amp_compiles_clean_and_each_of_its_errors_is_reached_where_it_was_written() {
    use crate::workbench::app_state::diagnostic_store::listing::listing;
    use crate::workbench::documents::code_workspace::CodeEditorSeverity;

    let (mut user, _, _) = new_device("opamp_bhv", "inp:in inn:in out:out vdd vss");
    super::veriloga_interface_tests::edit_source(&mut user.app, OPAMP);
    user.settle();
    compile(&mut user);

    let veriloga = &user.app.state.code.code_workspace.veriloga;
    assert!(
        veriloga.last_failure.is_empty(),
        "a valid module compiles: {:#?}",
        veriloga.last_failure
    );
    let receipt = veriloga.receipt.as_ref().expect("the module compiled");
    let codes = receipt
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code().to_owned())
        .collect::<Vec<_>>();
    for lint in [
        "VA-CONVERGENCE-DIVISION",
        "VA-HIDDEN-STATE-PARTIAL-ASSIGNMENT",
    ] {
        assert!(
            !codes.iter().any(|code| code == lint),
            "{lint} in {codes:?}"
        );
    }
    let compiled = user.prompt();
    assert!(
        compiled.starts_with("Compiled opamp_bhv: 5 ports, 5 parameters"),
        "{compiled:?}"
    );
    assert!(
        user.live.contains(&compiled),
        "a live status region says {compiled:?}: {:?}",
        user.live
    );

    // A division by a node voltage nothing keeps from zero is one piece of
    // advice, placed on the denominator: an information row the inspector
    // offers as a button, and no warning mark anywhere.
    let advised = OPAMP.replace(
        "I(out) <+ (V(out) - vtarget) / rout;",
        "I(out) <+ (V(out) - vtarget) / V(vdd, vss);",
    );
    super::veriloga_interface_tests::edit_source(&mut user.app, &advised);
    user.settle();
    compile(&mut user);
    assert_eq!(
        user.prompt(),
        "Compiled opamp_bhv: 5 ports, 5 parameters, 1 advisory"
    );
    user.expect("Information at line 30, column 36: Denominator can reach zero");
    let selected = crate::workbench::documents::code_workspace::selected_veriloga_source(&user.app)
        .expect("the module is open");
    assert_eq!(
        crate::workbench::documents::code_workspace::veriloga_compile_status(
            &user.app.state.code.code_workspace.veriloga,
            user.app.state.model.workspace.content.project.id(),
            &selected,
            user.app.state.workbench.messages(),
        )
        .map(|(status, problem)| (
            status
                .split(" · ")
                .next()
                .expect("a status has text")
                .to_owned(),
            problem
        )),
        Some(("1 advisory".to_owned(), None)),
        "advice is not a warning"
    );

    // `gbw` misspelt on line 21 and `vslew` on line 26.
    let broken = OPAMP
        .replace("vslew = slew(", "vslw = slew(")
        .replace("* gbw)", "* gbww)");
    super::veriloga_interface_tests::edit_source(&mut user.app, &broken);
    user.settle();
    compile(&mut user);
    let failure = std::sync::Arc::clone(&user.app.state.code.code_workspace.veriloga.last_failure);
    let mut errors = failure
        .iter()
        .filter(|diagnostic| diagnostic.severity == CodeEditorSeverity::Error)
        .map(|diagnostic| {
            (
                diagnostic.line.expect("an error names its line"),
                diagnostic.column.expect("an error names its column"),
                diagnostic.message.to_string(),
            )
        })
        .collect::<Vec<_>>();
    errors.sort();
    let places = errors
        .iter()
        .map(|(line, column, _)| (*line, *column))
        .collect::<Vec<_>>();
    assert_eq!(places, [(21, 29), (26, 5)], "{errors:#?}");
    assert_eq!(user.prompt(), "Compile failed: 2 errors");
    assert!(
        user.live
            .iter()
            .any(|said| said == "Compile failed: 2 errors"),
        "{:?}",
        user.live
    );
    let (gbww, vslw) = (&errors[0].2, &errors[1].2);

    // Problems lists the error on its own line, and opening it puts the
    // caret on its line and column.
    let row_route = {
        let listed = listing(&user.app.state);
        let row = (0..listed.len())
            .filter_map(|index| listed.row(index))
            .find(|row| row.message == gbww.as_str())
            .expect("Problems lists the error");
        assert_eq!(row.location, "line 21 · column 29");
        row.route.clone().expect("the row opens where the error is")
    };
    user.app.state.model.diagnostics.request_route(row_route);
    user.settle();
    assert_eq!(user.caret(), (21, 29));

    // F8 steps on through the document's findings and wraps; Shift+F8 steps
    // back.
    let path = crate::workbench::documents::code_workspace::selected_veriloga_source(&user.app)
        .expect("a Verilog-A view is open")
        .document()
        .logical_path()
        .to_owned();
    let located = failure.view(Some(&path)).located().len();
    user.key(egui::Key::F8, egui::Modifiers::NONE);
    assert_eq!(user.caret(), (26, 5));
    assert_eq!(
        user.prompt(),
        format!("Error 2 of {located}, line 26: {vslw}")
    );
    user.key(egui::Key::F8, egui::Modifiers::SHIFT);
    assert_eq!(user.caret(), (21, 29));
    assert_eq!(
        user.prompt(),
        format!("Error 1 of {located}, line 21: {gbww}")
    );
    user.key(egui::Key::F8, egui::Modifiers::SHIFT);
    assert_eq!(user.caret(), (26, 5), "Shift+F8 wraps to the last");

    // The inspector's row for an error is a button named for where it is,
    // and pressing it puts the caret there.
    user.click(&format!("Error at line 21, column 29: {gbww}"));
    assert_eq!(user.caret(), (21, 29));
}

/// A failed compile's errors stay on the text they are about while it is
/// edited: outdated, moved down with it when a line is added above, and gone
/// with the statement they named when it is deleted. "Show problem at caret"
/// says what the caret is on, and that it was found before the latest edits.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_failed_compiles_errors_follow_their_text_through_edits_until_it_is_deleted() {
    let (mut user, _, _) = new_device("opamp_bhv", "inp:in inn:in out:out vdd vss");
    let broken = OPAMP
        .replace("vslew = slew(", "vslw = slew(")
        .replace("* gbw)", "* gbww)");
    super::veriloga_interface_tests::edit_source(&mut user.app, &broken);
    user.settle();
    compile(&mut user);
    let path = crate::workbench::documents::code_workspace::selected_veriloga_source(&user.app)
        .expect("a Verilog-A view is open")
        .document()
        .logical_path()
        .to_owned();
    let failure = std::sync::Arc::clone(&user.app.state.code.code_workspace.veriloga.last_failure);
    let located = failure.view(Some(&path)).located();
    let count = located.len();
    let (code, gbww) = located
        .iter()
        .find(|diagnostic| diagnostic.line == Some(21))
        .map(|diagnostic| (diagnostic.code().to_owned(), diagnostic.message.to_string()))
        .expect("the misspelt gain-bandwidth product is an error");
    let vslw = located
        .iter()
        .find(|diagnostic| diagnostic.line == Some(26))
        .map(|diagnostic| diagnostic.message.to_string())
        .expect("the misspelt slew variable is an error");
    let step = |user: &mut User, command: CodeCommand| {
        user.app.execute_command_key(command.key());
        user.settle();
        user.prompt()
    };
    assert_eq!(
        step(&mut user, CodeCommand::NextProblem),
        format!("Error 1 of {count}, line 21: {gbww}")
    );
    assert_eq!(user.caret(), (21, 29));
    assert_eq!(
        step(&mut user, CodeCommand::ShowProblemAtCaret),
        format!("Error {code} at line 21, column 29: {gbww}")
    );

    // A line added above: the errors are where their text went.
    let commented = format!("// Reviewed.\n{broken}");
    super::veriloga_interface_tests::edit_source(&mut user.app, &commented);
    user.settle();
    assert_eq!(
        step(&mut user, CodeCommand::NextProblem),
        format!("Error 1 of {count}, line 22: {gbww}")
    );
    assert_eq!(user.caret(), (22, 29));
    assert_eq!(
        step(&mut user, CodeCommand::ShowProblemAtCaret),
        format!("Error {code} at line 22, column 29, found before your latest edits: {gbww}")
    );

    // The statement with the misspelt variable deleted: its error goes too.
    let statement = commented
        .lines()
        .find(|line| line.contains("vslw = slew("))
        .expect("the statement is in the source");
    let deleted = commented.replace(&format!("{statement}\n"), "");
    super::veriloga_interface_tests::edit_source(&mut user.app, &deleted);
    user.settle();
    for _ in 0..count {
        let said = step(&mut user, CodeCommand::NextProblem);
        assert!(said.contains(&format!(" of {}, ", count - 1)), "{said:?}");
        assert!(!said.contains(&vslw), "{said:?}");
    }
}

/// A module about two thousand lines long: a chain of variables, each one
/// more than the one before, driving one branch.
fn long_module(links: usize) -> String {
    let mut source = String::from(
        "`include \"disciplines.vams\"\n\nmodule chain(p, n);\n  inout p, n;\n  electrical p, n;\n",
    );
    for index in 0..links {
        source.push_str(&format!("  real x{index};\n"));
    }
    source.push_str("  analog begin\n    x0 = 1;\n");
    for index in 1..links {
        source.push_str(&format!("    x{index} = x{} + 1;\n", index - 1));
    }
    source.push_str(&format!(
        "    I(p, n) <+ x{} * V(p, n) * 1e-9;\n  end\nendmodule\n",
        links - 1
    ));
    source
}

/// A pause in writing checks the module, with no compile: an error typed
/// into a two-thousand-line module is found half a second after the typing
/// stops, on the line it was typed on, and correcting it clears it the same
/// way.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_pause_in_writing_checks_a_long_module_and_finds_the_error_typed_into_it() {
    let (mut user, _, _) = new_device("chain", "p n");
    let module = long_module(1000);
    assert!(module.lines().count() >= 2000);
    super::veriloga_interface_tests::edit_source(&mut user.app, &module);
    user.settle();
    compile(&mut user);
    assert!(
        user.app
            .state
            .code
            .code_workspace
            .veriloga
            .last_failure
            .is_empty()
    );
    let checking = |app: &RSpiceApp| {
        app.state
            .code
            .code_workspace
            .veriloga
            .live_check
            .running
            .is_some()
    };
    // The pause is timed the way the frame loop sees it.
    let pause_then_check = |user: &mut User, source: &str| {
        user.wait_for("no check on its way", |app| !checking(app));
        super::veriloga_interface_tests::edit_source(&mut user.app, source);
        let edited = user.ctx.input(|input| input.time);
        while !checking(&user.app) {
            user.frame(Vec::new());
            assert!(
                user.ctx.input(|input| input.time) - edited < 1.0,
                "no check started"
            );
        }
        let paused = user.ctx.input(|input| input.time) - edited;
        assert!((0.5..0.55).contains(&paused), "{paused}");
        user.wait_for("the check", |app| !checking(app));
        assert!(
            user.app
                .state
                .code
                .code_workspace
                .veriloga
                .pending
                .is_none(),
            "nothing was compiled"
        );
        user.app.execute_command_key(CodeCommand::NextProblem.key());
        user.settle();
        user.prompt()
    };

    let broken = module.replace("    x700 = x699 + 1;\n", "    x700 = x699 + y;\n");
    let line = broken
        .lines()
        .position(|text| text.contains("+ y;"))
        .expect("the typed error")
        + 1;
    let said = pause_then_check(&mut user, &broken);
    assert!(
        said.starts_with(&format!("Error 1 of 1, line {line}: ")) && said.contains("'y'"),
        "{said:?}"
    );
    assert_eq!(
        pause_then_check(&mut user, &module),
        "No problems in this document."
    );
}

/// Four compact models as their authors ship them, each a root with its
/// includes beside it: the folder under `models/veriloga/cmc`, the root, and
/// the module it declares.
const CMC_ROOTS: [(&str, &str, &str); 4] = [
    ("PSP104.1.0_vacode/vacode", "psp104.va", "PSP104VA"),
    ("BSIM-CMG_112.1.0_04282026/code", "bsimcmg.va", "bsimcmg_va"),
    ("505p5p0_va", "bjt505.va", "bjt505_va"),
    ("diode_cmc_3.0_20250714/vacode", "diode_cmc.va", "DIODE_CMC"),
];

/// A compact model imports from its source folder by picking its root: the
/// import reads every include from beside it, compiles, and is placed from
/// the library shelf on the top schematic, each model one cell of its own.
#[test]
fn compact_models_import_from_their_source_folders_compile_and_place() {
    use crate::state::workspace::ImportedInclude;
    use crate::workbench::documents::code_workspace;

    let (mut user, top) = new_project();
    let models = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/veriloga/cmc");
    for (index, (folder, root, module)) in CMC_ROOTS.into_iter().enumerate() {
        let folder = models.join(folder);
        let text = std::fs::read_to_string(folder.join(root)).expect("the root is readable");
        user.app
            .execute_command_key(CodeCommand::ImportVerilogA.key());
        code_workspace::complete_veriloga_files_pick(
            &mut user.app.code_workspace_parts(),
            &[(root, &text)],
            Some(&folder),
        );
        user.settle();
        let form = user
            .app
            .state
            .code
            .code_workspace
            .veriloga
            .import_form
            .clone()
            .expect("the import form opens");
        assert_eq!(
            form.closure.missing().count(),
            0,
            "{root}: {:?}",
            form.closure.files
        );
        let found = form
            .closure
            .files
            .iter()
            .filter(|file| file.include == ImportedInclude::Found)
            .count();
        assert!(found >= 4, "{root} brings its includes: {found}");
        user.click("Create cells");
        let view = user.app.state.model.workspace.content.active_view.clone();
        assert_eq!(
            (view.cell.as_str(), view.view.as_str()),
            (module, "veriloga")
        );

        compile(&mut user);
        let veriloga = &user.app.state.code.code_workspace.veriloga;
        assert!(
            veriloga.receipt.is_some(),
            "{root} compiles: {:?}",
            veriloga.last_failure
        );

        // The library shelf arms the cell as its row does, then a click on
        // the top schematic places it.
        let candidate = crate::state::library_cell_placement_candidates(
            &user.app.state.model.library_manager,
            &user.app.state.model.workspace,
        )
        .into_iter()
        .find(|candidate| candidate.library == view.library && candidate.cell == view.cell)
        .expect("the cell is on the shelf");
        assert!(candidate.ready, "{root}: {}", candidate.unavailable_reason);
        user.app.state.open_workspace_view(top.clone());
        user.settle();
        crate::workbench::app::schematic::placement::arm_library_cell_placement(
            &mut user.app.state,
            candidate.binding,
        );
        assert_eq!(user.app.state.model.workspace.content.active_view, top);
        let canvas = user.expect("Schematic canvas");
        let at = canvas.center() + egui::vec2(-240.0 + 160.0 * index as f32, 0.0);
        user.press(at, egui::PointerButton::Primary);
        user.settle();
        user.app.state.model.schematic.cancel_tool();
        let placed = user
            .app
            .state
            .model
            .schematic
            .document()
            .components
            .iter()
            .filter(|component| {
                component
                    .library_cell
                    .as_ref()
                    .is_some_and(|binding| binding.cell == module)
            })
            .count();
        assert_eq!(placed, 1, "{module} is placed once");
    }
}

/// The built-in library, as the Library page shows it, has a name, a class
/// heading and a version for each model; a copy of one is a cell the user can
/// edit, which compiles from its own source and places on the sheet, and it
/// carries the branch its generated code was built from.
#[test]
fn a_builtin_model_copied_from_the_library_page_compiles_and_places() {
    use crate::workbench::project::workspace_state::ProjectPage;

    let (mut user, top) = new_project();
    let library = top.library.clone();
    user.app.state.project.project_page = ProjectPage::Library;
    user.app.state.workbench.activate(Workspace::Project);
    user.settle();

    user.click("Verilog-A models");
    assert!(user.find("BSIM-BULK").is_some(), "the models are listed");
    let band = user.expect("Open read-only");
    let dock = user.expect("Console");
    assert!(
        band.bottom() <= dock.top() + 0.5,
        "the band's buttons stand above the dock: {band:?} {dock:?}"
    );
    user.app.state.project.project_library_filter = "vbic".to_owned();
    user.settle();
    user.click("VBIC 3-terminal");
    user.click("Copy cell…");
    let project = &user.app.state.project;
    assert!(project.copy_cell_dialog, "Copy cell opens");
    assert_eq!(project.copy_cell_target_library, library);
    assert_eq!(project.copy_cell_name, "vbic13");
    assert_eq!(
        user.expect("Source").left(),
        user.expect("New name").left(),
        "the source reads in the fields' column"
    );
    user.click("Copy");

    let view = CellViewRef::new(&library, "vbic13", "veriloga");
    assert!(
        !user.app.state.project.copy_cell_dialog,
        "{:?}",
        user.app.state.project.copy_cell_error
    );
    assert!(
        user.app
            .state
            .model
            .library_manager
            .get_library(&library)
            .and_then(|library| library.get_cell("vbic13"))
            .and_then(|cell| cell.get_view("veriloga"))
            .is_some(),
        "the copy is a Verilog-A view in the design's library"
    );
    assert!(
        !user.app.state.project.builtin_models.open,
        "the page moves to the copy"
    );

    user.app.state.open_workspace_view(view.clone());
    user.app.state.workbench.activate(Workspace::Design);
    user.settle();
    compile(&mut user);
    let veriloga = &user.app.state.code.code_workspace.veriloga;
    assert!(
        veriloga.receipt.is_some(),
        "the copy compiles: {:?}",
        veriloga.last_failure
    );
    let selected = crate::workbench::documents::code_workspace::selected_veriloga_source(&user.app)
        .expect("the copy is selected");
    let profile =
        rspice_simulation::project_veriloga::build_profile::resolve_veriloga_build_profile(
            selected.bundle(),
        )
        .expect("the copy has a build profile");
    assert!(
        profile
            .profile
            .preprocessor
            .defines
            .contains_key("__NGSPICE__"),
        "the copy builds the branch the shipped model was built from"
    );

    let candidate = crate::state::library_cell_placement_candidates(
        &user.app.state.model.library_manager,
        &user.app.state.model.workspace,
    )
    .into_iter()
    .find(|candidate| candidate.library == library && candidate.cell == "vbic13")
    .expect("the copy is on the shelf");
    assert!(candidate.ready, "{}", candidate.unavailable_reason);
    user.app.state.open_workspace_view(top.clone());
    user.settle();
    crate::workbench::app::schematic::placement::arm_library_cell_placement(
        &mut user.app.state,
        candidate.binding,
    );
    let canvas = user.expect("Schematic canvas");
    user.press(canvas.center(), egui::PointerButton::Primary);
    user.settle();
    user.app.state.model.schematic.cancel_tool();
    let placed = user
        .app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| {
            component
                .library_cell
                .as_ref()
                .is_some_and(|binding| binding.cell == "vbic13")
        })
        .count();
    assert_eq!(placed, 1, "the copy places once");
}

mod compile_controls;
mod contrast;
mod editing;
mod keyboard_reach;
mod language_tools;
mod size_and_zoom;
