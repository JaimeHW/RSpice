//! The Connect rules page on the running application: what an edit there
//! does to the deck and to the run, which choices it offers and which it
//! refuses, where it is reached from, and how its grid takes the keyboard.
//!
//! Driven through the page itself, a frame at a time with real input events,
//! on the application state a user's session holds: a Verilog-A device whose
//! logic input `mode` reads the net `drv`.

use super::engine::{
    TWO_RULE_BLOCKS, app_with_a_moded_device, moded_device_on_a_ramp, moded_device_with,
};
use super::setup_page::{Page, stored_options};
use super::*;
use rspice_app_state::state::AnalysisResultPayload;
use rspice_app_state::state::SimulationPage;

impl Page {
    pub(super) fn open(app: RSpiceApp) -> Self {
        Self::open_on(app, SimulationPage::ConnectRules)
    }

    /// Name the run's connect rules in the combo box, by typing.
    fn name_rules(&mut self, name: &str) {
        self.click(CONNECT_RULES);
        self.type_over(name);
        self.key(egui::Key::Enter);
    }

    /// Pick `option` from the combo box's list, as a hand does.
    fn pick_rules(&mut self, option: &str) {
        self.click(&format!("{CONNECT_RULES} choices"));
        let at = self
            .controls
            .iter()
            .find(|control| {
                control.role == egui::accesskit::Role::ListBoxOption && control.label == option
            })
            .unwrap_or_else(|| panic!("the list offers {option}"))
            .rect
            .center();
        self.click_at(at);
    }
}

const CONNECT_RULES: &str = "Connect rules \u{b7} CONNECTRULES";

/// The deck the run would execute, as Inspect netlist shows it.
fn deck(app: &RSpiceApp) -> String {
    rspice_app_runs::simulation::controller::SimulationController::prepare_design_netlist_for_inspection(
        &app.state.model,
    )
    .expect("the design netlists")
}

/// The `.CONNECTRULES` card the deck writes, joined onto one line.
fn rules_card(deck: &str) -> Option<String> {
    let mut lines = deck
        .lines()
        .skip_while(|line| !line.starts_with(".CONNECTRULES"));
    let first = lines.next()?;
    let mut card = first.to_owned();
    for line in lines.take_while(|line| line.starts_with('+')) {
        card.push(' ');
        card.push_str(line.trim_start_matches('+').trim());
    }
    Some(card)
}

/// Check the run, and the rows the report lists: (net, rule, supply, set).
fn checked_rows(app: &mut RSpiceApp) -> Vec<[String; 4]> {
    app.invalidate_simulation_preflight();
    start(&mut app.state, &mut app.simulation_controller, false);
    app.settle_preflight();
    app.state
        .runs
        .preflight
        .converters
        .iter()
        .flat_map(|set| &set.converters)
        .map(|converter| {
            let [net, _, _, _, rule, supply, set] = converter.cells.clone();
            [net, rule, supply, set]
        })
        .collect()
}

// ------------------------------------------------------------- edit to run

/// Run the ramp and say when it switched: the converter's own edge, the
/// event time its digital output rises at, and the first row whose drive
/// current over the ramp's 3.3 V/µs reads the 1 mS the device takes with
/// `mode` high, rather than the 2 mS it takes low.
fn edge_times(app: &mut RSpiceApp) -> (f64, f64) {
    app.invalidate_simulation_preflight();
    start(&mut app.state, &mut app.simulation_controller, false);
    app.settle_preflight();
    assert!(
        preflight_blockers(app).is_empty(),
        "{:#?}",
        preflight_blockers(app)
    );
    let _ = app.queued_by_its_check();
    let _ = app.dispatch_frame();
    app.finish_the_run();
    let run = app
        .state
        .model
        .simulation
        .retained
        .runs
        .first()
        .expect("the run is retained");
    switch_times(run, "drive current")
}

/// When `run` switched: the first time the converter's output rises, and the
/// first row whose `current` over the ramp's voltage reads the 1 mS the
/// device takes with `mode` high.
fn switch_times(run: &rspice_app_state::state::SimulationRun, current: &str) -> (f64, f64) {
    assert!(run.success, "{:?}", run.analyses);
    let waveforms = &run.analyses[0].waveforms;
    let amps = waveforms
        .iter()
        .find(|waveform| waveform.name.eq_ignore_ascii_case(current))
        .unwrap_or_else(|| {
            panic!(
                "the run saves {current}: {:?} {:#?}",
                waveforms
                    .iter()
                    .map(|w| w.name.as_str())
                    .collect::<Vec<_>>(),
                run.analyses[0]
                    .saved_output_receipts
                    .iter()
                    .map(|receipt| (&receipt.name, &receipt.status))
                    .collect::<Vec<_>>()
            )
        });
    let row = amps
        .x
        .iter()
        .zip(amps.y.iter())
        .find(|(t, i)| {
            let volts = 3.3 * **t / 1e-6;
            volts > 0.05 && i.abs() / volts < 1.5e-3
        })
        .map(|(t, _)| *t)
        .expect("the device switches during the ramp");
    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
        &run.analyses[0].result_payload
    else {
        panic!("the run keeps its event histories");
    };
    // The converter's output is the run's one digital node; a code below 12
    // whose level is one is a 1 at any strength.
    let rose = digital_traces
        .iter()
        .flat_map(|trace| &trace.points)
        .filter(|point| point.value_code < 12 && point.value_code % 3 == 1)
        .map(|point| point.time_s)
        .min_by(f64::total_cmp)
        .unwrap_or_else(|| {
            panic!(
                "the converter's output rises: {:?}",
                digital_traces
                    .iter()
                    .map(|trace| trace.node_name.as_str())
                    .collect::<Vec<_>>()
            )
        });
    (rose, row)
}

/// The deck File \u{25b8} Export \u{25b8} Netlist writes runs outside the
/// application as the design runs in it. Exported with a 2 V threshold typed
/// into the run's block, the deck carries the transient, the options and the
/// block, and copies the device's Verilog-A and the connect library beside
/// itself under relative names; moved to another folder whole and run there
/// on the deck route, it converts at the same femtosecond and switches the
/// device on the same row as the application's run.
#[test]
fn the_exported_deck_runs_with_the_converter_edge_the_application_ran() {
    let mut page = Page::open(moded_device_on_a_ramp());
    page.name_rules("run_rules");
    page.retype("a2d Vth high", "2");
    let (rose, row) = edge_times(&mut page.app);

    let folder = std::env::temp_dir().join(format!("rspice_ramp_export_{}", std::process::id()));
    let moved = folder.with_extension("moved");
    for stale in [&folder, &moved] {
        let _ = std::fs::remove_dir_all(stale);
    }
    crate::feature_tests::code::netlist_export::export_spice(
        &mut page.app,
        &folder.join("ramp.spice"),
    );
    std::fs::rename(&folder, &moved).expect("the export moves whole");
    let path = moved.join("ramp.spice");
    let deck = std::fs::read_to_string(&path).expect("the deck is written");
    assert!(!deck.contains("__rspice_"), "{deck}");
    assert!(
        deck.lines()
            .any(|line| line.trim() == ".tran 0.000000001 0.000001 0 0.000000001"),
        "the deck carries the run's 1 ns, 1 us transient:\n{deck}"
    );
    assert!(
        rules_card(&deck).is_some_and(|card| card.contains("connect a2d #(.vthi(2.0))")),
        "{deck}"
    );

    let mut run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch_at(
        &deck,
        &path,
        rspice_results::run::SimulationRunLifecycle::Completed,
    );
    let _ = std::fs::remove_dir_all(&moved);
    // The deck route keeps every solver point; the application's run samples
    // on the transient's 1 ns output grid. The same solve sampled on that
    // grid is the same trace.
    let current = run.analyses[0]
        .waveforms
        .iter_mut()
        .find(|wave| wave.name.eq_ignore_ascii_case("I(V1)"))
        .expect("the deck route keeps the drive current");
    *current = rspice_simulation::output_contract::materialization::resample_selected_and_final(
        &*current,
        crate::feature_tests::code::netlist_export::output_grid(&deck),
    )
    .expect("the exported trace samples on the output grid");
    let sampled = current.y.to_vec();
    let ran = page
        .app
        .state
        .model
        .simulation
        .retained
        .runs
        .first()
        .expect("the run")
        .analyses[0]
        .waveforms
        .iter()
        .find(|wave| wave.name == "drive current")
        .expect("the application's run saves the drive current")
        .y
        .to_vec();
    assert_eq!(
        sampled
            .iter()
            .map(|amps| amps.to_bits())
            .collect::<Vec<_>>(),
        ran.iter().map(|amps| amps.to_bits()).collect::<Vec<_>>(),
        "the drive current on the output grid"
    );
    let (exported_rose, exported_row) = switch_times(&run, "I(V1)");
    assert!(
        (exported_rose - rose).abs() <= 1e-15,
        "the exported deck converts at {exported_rose:e} s, the application at {rose:e} s"
    );
    assert_eq!(
        exported_row, row,
        "the exported deck switches on the row at {exported_row:e} s, the application on {row:e} s"
    );
}

/// A threshold typed into the run's own block is the one the deck writes and
/// the run converts at. A new block starts from the built-in rules, a
/// comparator at half its 3.3 V supply with no X timeout, so on a 3.3 V/µs
/// ramp the converter's edge is 500 ns in. A 2 V Vth high typed into the grid
/// holds the output low through the band to 2 V, 606.06 ns in. Clearing the cell
/// takes the half-supply edge back, in the deck and in the run.
#[test]
fn a_threshold_typed_in_the_run_block_moves_the_converter_edge() {
    let mut page = Page::open(moded_device_on_a_ramp());
    page.name_rules("run_rules");
    let deck_half = rules_card(&deck(&page.app)).expect("the deck writes the run's block");
    assert!(
        deck_half.starts_with(".CONNECTRULES run_rules") && !deck_half.contains("vthi"),
        "{deck_half}"
    );
    let at_half = edge_times(&mut page.app);

    page.retype("a2d Vth high", "2");
    let deck_2 = rules_card(&deck(&page.app)).expect("the card");
    assert!(deck_2.contains("connect a2d #(.vthi(2.0))"), "{deck_2}");
    let at_2 = edge_times(&mut page.app);

    page.retype("a2d Vth high", "");
    let cleared = rules_card(&deck(&page.app)).expect("the card");
    assert_eq!(
        cleared, deck_half,
        "clearing the cell takes the set's value back"
    );
    let again = edge_times(&mut page.app);

    // The converter dates its edge at the located crossing, to the femtosecond.
    // The device's conductance changes at the first analog solution after
    // that event, so on the run's 1 ns rows it reads on the first row after
    // the crossing: never before it, and never a row late.
    let ramp = 3.3 / 1e-6;
    let step = 1e-9;
    for ((rose, row), threshold) in [(at_half, 1.65), (at_2, 2.0), (again, 1.65)] {
        let crossing = threshold / ramp;
        assert!(
            (rose - crossing).abs() <= 1e-15,
            "the converter rose at {rose:e} s, the ramp crosses at {crossing:e} s"
        );
        assert!(
            row > crossing && row - crossing <= step * (1.0 + 1e-6),
            "the device switched on the row at {row:e} s, the ramp crosses at {crossing:e} s"
        );
    }
}

// ------------------------------------------------------------------ picker

/// What the combo box names is what the deck writes and what converts: a
/// shipped set, the run's own block, or a block a bound library declares.
/// A name nothing declares is refused at preflight, and the refusal opens
/// this page.
#[test]
fn each_rule_set_picked_is_the_one_the_deck_writes_and_the_report_lists() {
    let mut page = Page::open(app_with_a_moded_device(TWO_RULE_BLOCKS).0);

    // A shipped set is chosen by typing its name, as a reader who knows it
    // does; the design's own blocks lead the list and are picked from it.
    page.name_rules("rspice_3v3_fast");
    assert!(
        deck(&page.app).contains("CONNECTRULES=\"rspice_3v3_fast\""),
        "{}",
        deck(&page.app)
    );
    let rows = checked_rows(&mut page.app);
    assert!(
        matches!(rows.as_slice(), [[_, rule, supply, _]] if rule == "rspice_3v3_fast" && supply == "3.3 V"),
        "{rows:?}"
    );

    page.pick_rules("run_rules");
    let written = deck(&page.app);
    assert!(
        written.contains("CONNECTRULES=\"run_rules\"") && rules_card(&written).is_some(),
        "{written}"
    );
    let rows = checked_rows(&mut page.app);
    assert!(
        matches!(rows.as_slice(), [[_, rule, _, _]] if rule == "run_rules"),
        "{rows:?}"
    );

    page.pick_rules("High");
    assert!(deck(&page.app).contains("CONNECTRULES=\"High\""));
    let rows = checked_rows(&mut page.app);
    assert!(
        matches!(rows.as_slice(), [[_, rule, supply, _]] if rule == "High" && supply == "5 V"),
        "{rows:?}"
    );

    page.name_rules("Nowhere");
    assert!(deck(&page.app).contains("CONNECTRULES=\"Nowhere\""));
    let _ = checked_rows(&mut page.app);
    let refusal = preflight_blockers(&page.app)
        .iter()
        .find(|issue| issue.check == "Connect rules")
        .unwrap_or_else(|| {
            panic!(
                "an undeclared block is refused: {:#?}",
                preflight_blockers(&page.app)
            )
        })
        .clone();
    assert!(refusal.observed.contains("Nowhere"), "{refusal:?}");
    assert_eq!(refusal.remediation, PreflightRemediation::ConnectRules);
    page.app.state.simulate.simulation_page = SimulationPage::Solver;
    apply_remediation(&mut page.app, refusal.remediation);
    assert_eq!(
        page.app.state.simulate.simulation_page,
        SimulationPage::ConnectRules
    );
}

// --------------------------------------------------------------- keyboard

/// Tab walks the run block's grid row by row, each editable cell in column
/// order, and a cell takes typed text from the keyboard alone. Text a cell
/// cannot hold is refused where it was typed: the cell keeps it, says why,
/// naming the unit, and the block is unchanged.
#[test]
fn the_grid_is_walked_and_edited_from_the_keyboard() {
    let mut page = Page::open(app_with_a_moded_device("").0);
    page.name_rules("run_rules");

    let grid = [
        "a2d Supply",
        "a2d Vth high",
        "a2d Vth low",
        "d2a Supply",
        "d2a R high",
        "d2a R low",
        "d2a Rise",
        "d2a Fall",
        "bidir Supply",
        "bidir Vth high",
        "bidir Vth low",
        "bidir R high",
        "bidir R low",
        "bidir Rise",
        "bidir Fall",
        "Select r2e",
        "Select e2r",
        "Select rbidir",
    ];
    page.click(grid[0]);
    let mut walked = vec![page.focused.clone().unwrap_or_default()];
    for _ in 1..grid.len() {
        page.key(egui::Key::Tab);
        walked.push(page.focused.clone().unwrap_or_default());
    }
    assert_eq!(walked, grid, "Tab follows the grid");

    // From the keyboard alone: focus the cell by Tab, type, Tab off.
    page.click("d2a Supply");
    page.key(egui::Key::Tab);
    assert_eq!(page.focused.as_deref(), Some("d2a R high"));
    page.type_over("75");
    page.key(egui::Key::Tab);
    let block = page
        .app
        .state
        .model
        .sim_setup
        .options
        .connect_rule_block
        .clone()
        .expect("the run's block");
    assert!(block.card().contains(".rhi(75.0)"), "{}", block.card());

    page.retype("d2a Rise", "fast");
    assert_eq!(
        page.app.state.model.sim_setup.options.connect_rule_block,
        Some(block),
        "refused text changes nothing"
    );
    let cell = page.control("d2a Rise").clone();
    assert_eq!(
        cell.value.as_deref(),
        Some("fast"),
        "the cell keeps what was typed"
    );
    let reason = cell.description.unwrap_or_default();
    assert!(
        reason.contains("seconds") && reason.contains("'fast'"),
        "{reason}"
    );
    let prompt = page.app.state.prompt_text().unwrap_or_default().to_owned();
    assert!(prompt.contains("seconds"), "{prompt}");
}

// ------------------------------------------------- persistence and history

/// What the plan names and the run's own block, as the plan holds them.
fn choice(
    app: &RSpiceApp,
) -> (
    Option<String>,
    Option<rspice_simulation_contract::options::RunConnectRules>,
) {
    let options = &app.state.model.sim_setup.options;
    (
        options.connect_rules.clone(),
        options.connect_rule_block.clone(),
    )
}

/// The run's block and the name picked on the page are part of the project:
/// both come back from the saved file, and the deck the reopened project
/// writes is the one it wrote before. Every edit on the page is one step of
/// Undo, a value typed a keystroke at a time among them, each puts back
/// exactly the options it replaced, and Redo takes each back in turn.
#[test]
fn the_rules_survive_save_and_reopen_and_every_edit_undoes() {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        DestinationAuthority, SaveScope, save_native,
    };
    use rspice_app_workbench::workbench::framework::command_registry::ids;

    let mut page = Page::open(app_with_a_moded_device("").0);
    let untouched = choice(&page.app);
    let mut exact = vec![stored_options(&page.app)];
    page.name_rules("run_rules");
    let named = choice(&page.app);
    exact.push(stored_options(&page.app));
    page.type_slowly("a2d Vth high", "1.2");
    let typed = choice(&page.app);
    exact.push(stored_options(&page.app));
    page.click("Start from");
    page.click_at(
        page.controls
            .iter()
            .filter(|control| control.label == "rspice_1v8_slow")
            .map(|control| control.rect.center())
            .next_back()
            .expect("the list offers rspice_1v8_slow"),
    );
    let rebased = choice(&page.app);
    exact.push(stored_options(&page.app));
    assert!(
        untouched != named && named != typed && typed != rebased,
        "each edit changes the plan: {named:?} {typed:?} {rebased:?}"
    );

    let folder = std::env::temp_dir().join(format!(
        "rspice-connect-rules-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&folder).expect("the test folder");
    let path = folder.join("rules.rspiceproj");
    let card = rules_card(&deck(&page.app));
    save_native(
        &mut page.app.state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("the project saves");
    let mut reopened = RSpiceApp::test_instance();
    assert!(
        reopened.state.load_project_from_path(&path),
        "the saved project opens"
    );
    assert_eq!(choice(&reopened), rebased, "the plan comes back as saved");
    assert_eq!(
        rules_card(&deck(&reopened)),
        card,
        "and writes the same card"
    );
    let _ = std::fs::remove_dir_all(&folder);

    for (step, (expected, whole)) in [&typed, &named, &untouched]
        .into_iter()
        .zip(exact[..3].iter().rev())
        .enumerate()
    {
        page.app.execute_command_key(ids::UNDO);
        assert_eq!(&choice(&page.app), expected, "undo {}", step + 1);
        assert_eq!(
            &stored_options(&page.app),
            whole,
            "undo {} restores exactly",
            step + 1
        );
    }
    for (step, (expected, whole)) in [&named, &typed, &rebased]
        .into_iter()
        .zip(&exact[1..])
        .enumerate()
    {
        page.app.execute_command_key(ids::REDO);
        assert_eq!(&choice(&page.app), expected, "redo {}", step + 1);
        assert_eq!(
            &stored_options(&page.app),
            whole,
            "redo {} restores exactly",
            step + 1
        );
    }
    page.pass(Vec::new());
    assert_eq!(
        page.control(CONNECT_RULES).value.as_deref(),
        Some("run_rules"),
        "the page shows what the step restored"
    );
}

// -------------------------------------------------------------- navigation

/// The page is a Setup command, found under Simulate in the palette, and a
/// connect-rule finding's repair opens it as a Problems row's press does. A
/// report row selects the net it converts on the schematic, and a row of a
/// block the design declares opens the source at its statement.
#[test]
fn the_page_is_reached_from_every_route_and_its_rows_lead_out() {
    use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticRoute;
    use rspice_app_workbench::workbench::app::command_palette::{PaletteEntry, palette_commands};

    let key = SimulateCommand::Page(SimulationPage::ConnectRules).key();
    let entry = palette_commands(&RSpiceApp::test_instance())
        .find(|listed| listed.key() == key)
        .map(PaletteEntry::Command)
        .expect("the palette lists the page");
    assert!(
        entry.detail().contains("Setup"),
        "found under the Setup menu that holds it: {}",
        entry.detail()
    );
    let mut app = RSpiceApp::test_instance();
    app.execute_command_key(key);
    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(
        app.state.simulate.simulation_page,
        SimulationPage::ConnectRules
    );

    let mut app = app_with_a_moded_device(TWO_RULE_BLOCKS).0;
    let _ = checked_rows(&mut app);
    let finding = preflight_blockers(&app)
        .iter()
        .find(|issue| issue.check == "Connect rules")
        .expect("two blocks and no choice is a finding")
        .clone();
    app.state.workbench.activate(Workspace::Design);
    app.state
        .model
        .diagnostics
        .request_route(DiagnosticRoute::Repair(finding.remediation));
    crate::workbench::diagnostic_routes::apply_pending(&egui::Context::default(), &mut app);
    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(
        app.state.simulate.simulation_page,
        SimulationPage::ConnectRules
    );

    let mut page = Page::open(app);
    page.pick_rules("High");
    let _ = checked_rows(&mut page.app);
    page.pass(Vec::new());
    page.click("DRV");
    assert!(
        matches!(
            page.routed,
            Some(DiagnosticRoute::Repair(
                PreflightRemediation::Drawing { .. }
            ))
        ),
        "{:?}",
        page.routed
    );
    assert_eq!(page.app.state.workbench.workspace, Workspace::Design);
    assert!(
        !page
            .app
            .state
            .model
            .schematic
            .session
            .editor
            .selection
            .is_empty(),
        "the row selects the net it converts"
    );

    let tail = "connectrules Mine;\n  connect a2d #(.vsup(2.0));\nendconnectrules\n";
    let mut page = Page::open(moded_device_with("", "gnd", tail).0);
    page.click("a2d");
    match &page.routed {
        Some(DiagnosticRoute::CodeDocument {
            logical_path, line, ..
        }) => assert_eq!((logical_path.as_ref(), *line), ("moded.va", Some(10))),
        routed => panic!("the row opens its source: {routed:?}"),
    }
    assert_eq!(page.app.state.workbench.workspace, Workspace::Design);
}

// ----------------------------------------------------------------- freshness

/// The table lists what the newest check found and nothing older: a check
/// after a rule change lists the new rule's converter, and an edit of the
/// plan or of the design since the check leaves no row standing until the
/// run is checked again.
#[test]
fn the_report_lists_only_what_the_current_check_found() {
    use rspice_app_state::state::{ComponentType, Point};

    let listed = |page: &Page| page.controls.iter().any(|control| control.label == "DRV");
    let mut page = Page::open(app_with_a_moded_device(TWO_RULE_BLOCKS).0);
    page.pick_rules("High");
    assert!(!listed(&page), "nothing before a check");
    let rows = checked_rows(&mut page.app);
    assert!(
        matches!(rows.as_slice(), [[_, rule, _, _]] if rule == "High"),
        "{rows:?}"
    );
    page.pass(Vec::new());
    assert!(listed(&page));

    page.pick_rules("Low");
    assert!(!listed(&page), "a plan edit leaves the old rows out");
    let rows = checked_rows(&mut page.app);
    assert!(
        matches!(rows.as_slice(), [[_, rule, supply, _]] if rule == "Low" && supply == "1 V"),
        "{rows:?}"
    );
    page.pass(Vec::new());
    assert!(listed(&page), "the check lists the new rule's converter");

    page.app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(600, 600));
    page.app.state.sync_active_schematic_to_workspace();
    page.pass(Vec::new());
    assert!(!listed(&page), "a design edit leaves the old rows out");
}
