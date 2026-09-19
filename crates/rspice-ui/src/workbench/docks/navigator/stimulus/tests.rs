//! What the library browser and the definition inspector are held to.
//!
//! Both docks are gated here because they are read together: a count in the
//! browser's footer and a count in the inspector's Adopters heading are the
//! same walk of the same design, and a test that only ever opened one of them
//! would not notice the two disagreeing.

use super::*;

use crate::state::stimulus_library::definition::StimulusDefinition;
use crate::state::stimulus_library::draft::DefinitionDraft;
use crate::state::stimulus_library::fixtures;
use crate::state::{Component, ComponentType, Point};
use crate::workbench::RSpiceApp;
use crate::workbench::app::actions::stimulus as stimulus_verbs;
use crate::workbench::state::PURPOSE_FIELD;

use super::super::super::inspector::stimulus as inspector_dock;

/// The dock's own minimum, at the fit the workspace is held to.
const NAVIGATOR: egui::Vec2 = egui::vec2(228.0, 640.0);
/// The inspector's own width at the same fit.
const INSPECTOR: egui::Vec2 = egui::vec2(300.0, 640.0);
/// The column the inspector's five sections claim when nothing is scrolled
/// away, which is what a review of them has to look at: the dock itself puts
/// them in a scroll area, so a render held to the dock's height would only ever
/// show the first two.
const INSPECTOR_COLUMN: egui::Vec2 = egui::vec2(300.0, 1180.0);

/// Lay one dock out and read back everything it announced.
fn announced(size: egui::Vec2, mut pass: impl FnMut(&mut egui::Ui)) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    // Two passes: the first builds the font set and the caches the rows read,
    // the second lays out against both.
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
        ..Default::default()
    };
    let _ = ctx.run_ui(input(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| pass(ui));
    });
    let output = ctx.run_ui(input(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| pass(ui));
    });
    output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree")
        .nodes
        .into_iter()
        .flat_map(|(_, node)| {
            [
                node.label().map(str::to_owned),
                node.value().map(str::to_owned),
            ]
        })
        .flatten()
        .collect()
}

/// Everything the browser announces for this state.
fn browser(state: &mut AppState) -> Vec<String> {
    announced(NAVIGATOR, |ui| show(ui, state))
}

/// Everything the inspector announces for this state.
fn inspector(state: &mut AppState) -> Vec<String> {
    announced(INSPECTOR, |ui| inspector_dock::show(ui, state))
}

/// Whether the console holds a user line containing `fragment`.
fn console_says(state: &AppState, fragment: &str) -> bool {
    state
        .log_buffer
        .entries()
        .any(|entry| entry.message.contains(fragment))
}

/// The mockup's fourteen definitions, with nothing adopted, in the workspace
/// whose navigator lists them.
fn library(state: &mut AppState) {
    state.workspace.stimulus_library = fixtures::library();
    state.workbench.activate(Workspace::Stimulus);
    state.workbench.selected_stimulus_definition = Some("sensor_diff_1k".to_owned());
}

/// One placed source carrying `definition`, wired across two nets.
fn adopt(state: &mut AppState, id: u64, reference: &str, definition: &str) {
    let held = state
        .workspace
        .stimulus_library
        .get(definition)
        .cloned()
        .expect("the fixture library holds it");
    let mut component = Component::new(id, held.component_type(), Point::new(4 + id as i32, 4));
    component.name = reference.to_owned();
    held.adopt_onto(&mut component).expect("adopt");
    state.schematic.components.push(component);
}

/// Every definition is under the heading of the family it realizes, and a
/// family the library does not use gets no heading at all.
#[test]
fn definitions_are_grouped_under_the_family_they_realize() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    let published = browser(&mut app.state);

    for family in ["SIN", "PULSE", "PWL", "EXP", "AM", "SFFM", "PAT", "TRNOISE"] {
        assert!(
            published.iter().any(|run| run.starts_with(family)),
            "{family} has no heading: {published:?}"
        );
    }
    // The fixtures author no AC definition, so nothing claims the heading.
    assert!(
        !published.iter().any(|run| run.starts_with("AC \u{b7}")),
        "an unused family was headed anyway: {published:?}"
    );
    for name in fixtures::names() {
        assert!(
            published.iter().any(|run| run.starts_with(name)),
            "{name} has no row: {published:?}"
        );
    }
}

/// A row announces its name, its revision and its meta column together: the
/// key figure and the adopter count are the whole of what tells two
/// definitions of one family apart, and a reader working by ear is owed them.
#[test]
fn a_row_announces_its_revision_and_its_meta_beside_its_name() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    adopt(&mut app.state, 1, "V1", "bridge_cal_step");
    let published = browser(&mut app.state);

    assert!(
        published
            .iter()
            .any(|run| run.starts_with("bridge_cal_step, r1, PER 1 ms")),
        "the meta column never reached AccessKit: {published:?}"
    );
    assert!(
        published
            .iter()
            .any(|run| run.contains("bridge_cal_step, r1, PER 1 ms \u{b7} \u{2192}1")),
        "the adopter count is missing: {published:?}"
    );
    // A current source says so in front of its figure; a voltage source does
    // not, because it is the ordinary case.
    assert!(
        published
            .iter()
            .any(|run| run.starts_with("load_step_50u, r1, I \u{b7} PER 5 ms")),
        "a current definition did not state its quantity: {published:?}"
    );
}

/// The dirty dot follows the editor, and reads error rather than warning when
/// the draft says something the deck would refuse.
#[test]
fn the_row_states_an_unapplied_draft_and_whether_it_is_refused() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    assert!(
        !browser(&mut app.state)
            .iter()
            .any(|run| run.contains("draft")),
        "a clean library announced a draft"
    );

    let saved = app
        .state
        .workspace
        .stimulus_library
        .get("sensor_diff_1k")
        .cloned()
        .expect("held");
    app.state
        .workbench
        .stimulus_editor
        .draft_for(&saved)
        .edit(|working| working.params = "va=4m freq=2k".to_owned());
    assert!(
        browser(&mut app.state)
            .iter()
            .any(|run| run.contains("sensor_diff_1k") && run.contains("draft not applied")),
        "an unapplied draft was not announced"
    );

    // A negative PWL delay is one of the few things the netlist parser refuses
    // outright, so it is a draft the deck would not take.
    let ramp = app
        .state
        .workspace
        .stimulus_library
        .get("vdd_ramp_1ms")
        .cloned()
        .expect("held");
    app.state
        .workbench
        .stimulus_editor
        .draft_for(&ramp)
        .edit(|working| working.params = "td=-1n".to_owned());
    assert!(
        browser(&mut app.state)
            .iter()
            .any(|run| run.contains("vdd_ramp_1ms") && run.contains("draft refused")),
        "a refused draft read the same as an ordinary one"
    );
}

/// The scope strip counts the library rather than the listing, and narrowing
/// to Unadopted leaves exactly the definitions nothing carries.
#[test]
fn the_scope_strip_counts_the_library_and_narrows_the_tree() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    adopt(&mut app.state, 1, "V1", "bridge_cal_step");
    adopt(&mut app.state, 2, "V2", "vdd_operate");

    let published = browser(&mut app.state);
    for option in ["All 14", "Adopted 2", "Unadopted 12"] {
        assert!(
            published.iter().any(|run| run == option),
            "{option} is not on the scope strip: {published:?}"
        );
    }

    app.state.workbench.stimulus_browser.scope = StimulusScope::Adopted;
    let published = browser(&mut app.state);
    assert!(
        published
            .iter()
            .any(|run| run.starts_with("bridge_cal_step"))
    );
    assert!(
        !published.iter().any(|run| run.starts_with("emi_am_150k")),
        "an unadopted definition survived the Adopted scope: {published:?}"
    );
    // The counts are the library's either way, so the strip still says how
    // much of it the reader is not looking at.
    assert!(published.iter().any(|run| run == "Unadopted 12"));
}

/// A filter opens every group it leaves standing: a reader who typed a name
/// and was shown a closed heading would read it as no match at all.
#[test]
fn a_filter_shows_its_matches_expanded_even_where_the_group_was_folded() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    app.state
        .workbench
        .navigator_trees
        .for_workspace(Workspace::Stimulus)
        .toggle(folded_node(StimulusFamily::Am));
    assert!(
        !browser(&mut app.state)
            .iter()
            .any(|run| run.starts_with("emi_am_150k")),
        "a folded group still listed its definitions"
    );

    app.state.workbench.set_navigator_filter("emi");
    let published = browser(&mut app.state);
    assert!(
        published.iter().any(|run| run.starts_with("emi_am_150k")),
        "the filter's match stayed folded: {published:?}"
    );
}

/// An empty library and a filter that matched nothing are different facts, and
/// each offers the press that resolves it.
#[test]
fn an_unauthored_library_and_a_filter_that_matched_nothing_read_differently() {
    let mut empty = RSpiceApp::test_instance();
    let published = browser(&mut empty.state);
    assert!(published.contains(&"This project has no stimulus definitions".to_owned()));
    assert!(published.contains(&"New definition".to_owned()));

    let mut filtered = RSpiceApp::test_instance();
    library(&mut filtered.state);
    filtered
        .state
        .workbench
        .set_navigator_filter("nothing_matches_this");
    let published = browser(&mut filtered.state);
    assert!(published.contains(&"No definition matches this filter".to_owned()));
    assert!(published.contains(&"Clear filters".to_owned()));
    assert!(!published.contains(&"This project has no stimulus definitions".to_owned()));
}

/// The scope strip counts definitions and the footer counts placed sources, so
/// the two say different numbers about the same library and each has to name
/// what it counted. Spelled the same way — `Adopted 1` over `2 adopted` — a
/// reader can only read one of them as a mistake.
#[test]
fn the_scope_strip_and_the_footer_name_the_different_things_they_count() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    adopt(&mut app.state, 1, "V1", "bridge_cal_step");
    adopt(&mut app.state, 2, "V2", "bridge_cal_step");

    let published = browser(&mut app.state);
    assert!(
        published.iter().any(|run| run == "Adopted 1"),
        "the scope strip counts definitions: {published:?}"
    );
    assert!(
        published
            .iter()
            .any(|run| run.starts_with("2 adopters \u{b7} ")),
        "the footer counts the placed sources: {published:?}"
    );
}

/// The footer states who is carrying the library and what the engine says
/// about it, and warns while anything is behind.
#[test]
fn the_footer_states_the_adoption_and_the_library_audit() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    assert!(
        browser(&mut app.state)
            .iter()
            .any(|run| run.starts_with("no adopters \u{b7} ")),
        "an unadopted library did not say so"
    );

    adopt(&mut app.state, 1, "V1", "bridge_cal_step");
    assert!(
        browser(&mut app.state)
            .iter()
            .any(|run| run.starts_with("1 adopter \u{b7} ")),
        "the footer did not count the adopter"
    );

    let mut draft = DefinitionDraft::new(
        app.state
            .workspace
            .stimulus_library
            .get("bridge_cal_step")
            .cloned()
            .expect("held"),
    );
    draft.edit(|working| working.params = "v2=20m per=1m".to_owned());
    app.state.workspace.stimulus_library.apply(&mut draft);
    assert!(
        browser(&mut app.state)
            .iter()
            .any(|run| run.starts_with("1 adopter \u{b7} 1 behind \u{b7} ")),
        "the footer did not count what the publish left behind"
    );
}

/// The engine is asked for a definition's waveform once, not once per frame.
///
/// A fourteen row library evaluated per frame is fourteen engine parses and
/// nearly nine hundred sample steps for every pointer move, and nothing about
/// the drawing would show it.
#[test]
fn the_minis_are_evaluated_once_across_two_frames() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    let _ = browser(&mut app.state);
    let after_first = app.state.workbench.stimulus_browser.evaluations();
    assert_eq!(
        after_first, 14,
        "every fixture definition should have been evaluated exactly once"
    );

    let _ = browser(&mut app.state);
    let _ = browser(&mut app.state);
    assert_eq!(
        app.state.workbench.stimulus_browser.evaluations(),
        after_first,
        "a frame that changed nothing asked the engine again"
    );

    // Publishing moves the revision, which is half the cache key, so the row
    // redraws rather than showing the picture it had.
    let mut draft = DefinitionDraft::new(
        app.state
            .workspace
            .stimulus_library
            .get("sensor_diff_1k")
            .cloned()
            .expect("held"),
    );
    draft.edit(|working| working.params = "va=9m freq=1k".to_owned());
    app.state.workspace.stimulus_library.apply(&mut draft);
    let _ = browser(&mut app.state);
    assert_eq!(
        app.state.workbench.stimulus_browser.evaluations(),
        after_first + 1,
        "a published revision did not re-evaluate its mini"
    );
}

/// A definition the engine can step through holds a curve; one it cannot holds
/// the refusal, which is what the row paints a mark for instead of a line.
#[test]
fn a_family_with_no_waveform_at_this_boundary_holds_its_refusal_rather_than_a_curve() {
    use crate::simulation::stimulus_realize::WaveformTrace;

    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    let _ = browser(&mut app.state);
    let browser_state = &app.state.workbench.stimulus_browser;

    let sine = browser_state
        .mini("sensor_diff_1k", 1)
        .expect("the sine was evaluated");
    assert!(
        matches!(sine, Ok(WaveformTrace::Curve(samples)) if samples.len() > 8),
        "the sine has no curve to draw"
    );
    let noise = browser_state
        .mini("supply_trnoise", 1)
        .expect("the noise train was evaluated");
    assert!(
        noise.is_err(),
        "a noise train claimed a waveform it has no way to produce"
    );
}

/// Every row's three tracks stay positive at the dock's own minimum, so a name
/// is elided rather than painted over the mini or under the revision.
#[test]
fn every_row_track_survives_the_navigator_minimum() {
    let name_track = NAVIGATOR.x
        - row::INDENT
        - crate::properties::source_preview::MINI_SIZE.x
        - row::TEXT_GAP
        - row::RIGHT_INSET
        - row::DOT_TRACK;
    assert!(
        name_track >= 100.0,
        "a name row has only {name_track} points at the dock's minimum"
    );
    assert!(row::ROW_HEIGHT >= crate::properties::source_preview::MINI_SIZE.y + 8.0);
    assert!(SCOPE_BAND + FOOTER_BAND + row::ROW_HEIGHT * 2.0 < NAVIGATOR.y);
}

/// The inspector lists every adopter in the design, states where the run
/// reaches it, and offers the verbs its occurrence can carry.
#[test]
fn the_inspector_cards_every_adopter_in_the_design() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    app.state.workbench.selected_stimulus_definition = Some("bridge_cal_step".to_owned());
    adopt(&mut app.state, 1, "V1", "bridge_cal_step");
    adopt(&mut app.state, 2, "V2", "bridge_cal_step");
    let published = inspector(&mut app.state);

    assert!(
        published.contains(&"ADOPTERS, 2".to_owned()),
        "{published:?}"
    );
    for reference in ["V1", "V2"] {
        assert!(
            published
                .iter()
                .any(|run| run.starts_with(&format!("{reference} \u{b7} / \u{b7} adopted"))),
            "{reference} has no card: {published:?}"
        );
    }
    assert!(published.contains(&"Show in schematic".to_owned()));
    assert!(published.contains(&"Properties\u{2026}".to_owned()));
    assert!(
        published.iter().any(|run| run.contains("2 placed sources")),
        "the edit impact did not count the adopters: {published:?}"
    );
}

/// A library nothing has adopted says how to adopt it rather than heading an
/// empty list.
#[test]
fn the_inspector_says_how_to_adopt_when_nothing_has() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    let published = inspector(&mut app.state);

    assert!(
        published.contains(&"ADOPTERS, 0".to_owned()),
        "{published:?}"
    );
    assert!(
        published
            .iter()
            .any(|run| run.starts_with("Nothing in this design has adopted")),
        "{published:?}"
    );
    // With no adopter there is no reader, so the consumers section is absent
    // rather than headed over nothing.
    assert!(
        !published
            .iter()
            .any(|run| run.starts_with("ANALYSIS CONSUMERS")),
        "{published:?}"
    );
}

/// The engine contract states the card this family carries when nothing is
/// authored, and whatever `source_contract` says about it — never a second
/// table of field names written out in the dock.
#[test]
fn the_engine_contract_is_the_card_the_netlister_writes_for_the_family() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    app.state.workbench.selected_stimulus_definition = Some("bridge_cal_step".to_owned());
    let published = inspector(&mut app.state);

    assert!(
        published.contains(&"ENGINE CONTRACT \u{b7} PULSE".to_owned()),
        "{published:?}"
    );
    let empty = StimulusDefinition::new("bridge_cal_step", ComponentType::VoltageSourcePulse)
        .expect("a pulse source is placeable");
    let card = empty
        .card_text(crate::simulation::stimulus_realize::DETACHED_NETS)
        .expect("the netlister writes a card for an unauthored pulse");
    assert!(
        published.iter().any(|run| run == &card),
        "the family's own card is missing: {published:?}"
    );
    // Every finding the module does produce is stated; the rules fire on
    // authored values, so an unauthored card provokes none and the section
    // says exactly that rather than leaving the reader with a heading.
    let findings = stimulus_verbs::contract_findings(&app.state, &empty);
    for finding in &findings {
        assert!(
            published
                .iter()
                .any(|run| run.contains(finding.message.as_str())),
            "the contract row for {} is missing: {published:?}",
            finding.field
        );
    }
    if findings.is_empty() {
        assert!(
            published.iter().any(|run| run
                == "An empty PULSE card carries no substitution the engine makes silently"),
            "{published:?}"
        );
    }
}

/// The purpose writes through the draft, so it is undoable and dirties the
/// definition exactly as a waveform field does.
#[test]
fn editing_the_purpose_goes_through_the_draft_and_is_undoable() {
    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    stimulus_verbs::edit_field(&mut app.state, PURPOSE_FIELD, "differential sensor drive");

    assert!(
        app.state
            .workbench
            .stimulus_editor
            .is_dirty("sensor_diff_1k"),
        "the purpose edit did not dirty the definition"
    );
    assert_eq!(
        app.state
            .workbench
            .stimulus_editor
            .draft("sensor_diff_1k")
            .map(|draft| draft.working().purpose.as_str()),
        Some("differential sensor drive")
    );
    assert!(stimulus_verbs::undo_draft(&mut app.state));
    assert_eq!(
        app.state
            .workbench
            .stimulus_editor
            .draft("sensor_diff_1k")
            .map(|draft| draft.working().purpose.as_str()),
        Some("")
    );
    // The library itself is untouched until Apply publishes, exactly as it is
    // for every other field.
    assert!(
        app.state
            .workspace
            .stimulus_library
            .get("sensor_diff_1k")
            .expect("held")
            .purpose
            .is_empty()
    );
}

/// Place arms the cursor with the library's saved revision, and says so when
/// the editor is holding a draft it is not placing.
#[test]
fn place_arms_the_saved_revision_and_warns_about_an_unapplied_draft() {
    use crate::state::Tool;

    let mut app = RSpiceApp::test_instance();
    library(&mut app.state);
    stimulus_verbs::place_selected_definition(&mut app.state);

    assert_eq!(
        app.state.schematic.tool,
        Tool::Place(ComponentType::VoltageSourceSin)
    );
    let armed = app
        .state
        .schematic
        .pending_stimulus
        .clone()
        .expect("a definition is armed");
    assert_eq!(armed.definition(), "sensor_diff_1k");
    assert_eq!(armed.revision(), 1);
    assert!(
        !console_says(&app.state, "apply the draft first"),
        "a clean definition was warned about"
    );

    let saved = app
        .state
        .workspace
        .stimulus_library
        .get("sensor_diff_1k")
        .cloned()
        .expect("held");
    app.state
        .workbench
        .stimulus_editor
        .draft_for(&saved)
        .edit(|working| working.params = "va=9m freq=1k".to_owned());
    stimulus_verbs::place_selected_definition(&mut app.state);
    assert!(
        console_says(
            &app.state,
            "Placement copies the saved r1 of 'sensor_diff_1k'; apply the draft first"
        ),
        "the reader was not told which revision the pointer is carrying"
    );
    assert_eq!(
        app.state
            .schematic
            .pending_stimulus
            .as_ref()
            .map(crate::state::PendingStimulusPlacement::revision),
        Some(1),
        "the draft was armed instead of the published revision"
    );
}

/// Write both docks to PNGs for a human to look at.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn print_stimulus_docks_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    for (stem, mode) in [
        ("dark", crate::ui::Mode::Dark),
        ("light", crate::ui::Mode::Light),
    ] {
        for (dock, size) in [("navigator", NAVIGATOR), ("inspector", INSPECTOR_COLUMN)] {
            let theme = crate::ui::Theme {
                mode,
                ..crate::ui::Theme::default()
            };
            let mut app = RSpiceApp::test_instance();
            library(&mut app.state);
            app.state.workbench.selected_stimulus_definition = Some("bridge_cal_step".to_owned());
            adopt(&mut app.state, 1, "V1", "bridge_cal_step");
            adopt(&mut app.state, 2, "V2", "bridge_cal_step");
            let held = app
                .state
                .workspace
                .stimulus_library
                .get("vdd_ramp_1ms")
                .cloned()
                .expect("held");
            app.state
                .workbench
                .stimulus_editor
                .draft_for(&held)
                .edit(|working| working.value = "0 0 2m 5 6m 5".to_owned());
            let canvas = crate::ui::raster::render_themed(theme, size, |ui, _background| {
                if dock == "navigator" {
                    show(ui, &mut app.state);
                } else {
                    inspector_dock::show(ui, &mut app.state);
                }
            });
            let height = canvas.content_height().max(1);
            let path = directory.join(format!("stimulus-{dock}-{stem}.png"));
            std::fs::write(&path, canvas.png(height)).expect("write dock render");
            writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                .expect("write raster qualification report");
        }
    }
}
