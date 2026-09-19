//! What adopting and saving do to a placed source, and what the dialog that
//! offers them states.
//!
//! The claims that matter are the ones a reader cannot check by eye: a
//! cross-family adoption really is a re-type that keeps both terminals on
//! their nets, one undo puts the old type *and* the old card back, a
//! cross-kind adoption is refused rather than converted, and the list a reader
//! walks never arrives on a row the primary would refuse.

use super::adopt::{adopt_groups, stepped};
use super::shell::MiniCache;
use super::*;
use crate::properties::source_preview::ensure_minis;
use crate::state::stimulus_library::provenance::ProvenanceState;
use crate::state::{ComponentType, Point};
#[cfg(not(target_arch = "wasm32"))]
use crate::ui::tokens::Mode;
use crate::workbench::app::RSpiceApp;

fn definition(name: &str, kind: ComponentType, value: &str, params: &str) -> StimulusDefinition {
    let mut definition = StimulusDefinition::new(name, kind).expect("definition");
    definition.value = value.to_owned();
    definition.params = params.to_owned();
    definition
}

/// One sheet with a wired `PULSE` source and a library holding a `SIN`
/// definition of the same quantity and a current definition of the other.
fn fixture() -> (RSpiceApp, u64) {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .schematic
        .add_component(ComponentType::VoltageSourcePulse, Point::new(40, 40));
    for held in [
        definition(
            "sensor_drive",
            ComponentType::VoltageSourceSin,
            "0",
            "va=3m freq=1k",
        ),
        definition("bias_rail", ComponentType::VoltageSource, "1.8", ""),
        definition("leak", ComponentType::CurrentSource, "1u", ""),
    ] {
        app.state
            .workspace
            .stimulus_library
            .insert(held)
            .expect("insert");
    }
    (app, id)
}

/// The fixture the list is judged on: five definitions covering all three
/// fits, so every group is non-empty and every group is ordered.
fn wide_fixture() -> (RSpiceApp, u64) {
    let (mut app, id) = fixture();
    for held in [
        definition(
            "clock",
            ComponentType::VoltageSourcePulse,
            "0",
            "v2=1.8 pw=1u per=2u",
        ),
        definition(
            "probe_tone",
            ComponentType::CurrentSourceSin,
            "0",
            "va=1u freq=10k",
        ),
    ] {
        app.state
            .workspace
            .stimulus_library
            .insert(held)
            .expect("insert");
    }
    (app, id)
}

/// A library holding nothing this instance could ever adopt.
fn kind_only_fixture() -> (RSpiceApp, u64) {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .schematic
        .add_component(ComponentType::VoltageSourcePulse, Point::new(40, 40));
    for held in [
        definition("leak", ComponentType::CurrentSource, "1u", ""),
        definition(
            "probe_tone",
            ComponentType::CurrentSourceSin,
            "0",
            "va=1u freq=10k",
        ),
    ] {
        app.state
            .workspace
            .stimulus_library
            .insert(held)
            .expect("insert");
    }
    (app, id)
}

fn instance_of(app: &RSpiceApp, id: u64) -> Component {
    app.state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
        .expect("the fixture instance is on the sheet")
}

/// The groups the list would show, with every mini already evaluated.
fn groups_of(app: &RSpiceApp, id: u64, filter: &str, cache: &mut MiniCache) -> Vec<String> {
    ensure_minis(
        cache,
        &app.state.workspace.stimulus_library,
        PreviewTiming::default(),
    );
    let component = instance_of(app, id);
    adopt_groups(&app.state, &component, filter, cache)
        .into_iter()
        .flat_map(|group| {
            std::iter::once(group.label).chain(group.rows.into_iter().map(|row| row.name))
        })
        .collect()
}

fn nets_of(app: &RSpiceApp, id: u64) -> Vec<String> {
    crate::simulation::placed_sources::placed_sources(
        &app.state.schematic,
        &app.state.workspace.stimulus_library,
        None,
    )
    .into_iter()
    .find(|source| source.component_id == id)
    .map(|source| source.nets)
    .unwrap_or_default()
}

/// Adopting a definition of another family on the same quantity re-types the
/// instance, copies the card, and leaves both terminals on the nets they were
/// on. Undo restores the type and the card together, because the whole thing
/// is one component transaction.
#[test]
fn a_cross_family_adoption_re_places_the_instance_and_keeps_both_nets() {
    let (mut app, id) = fixture();
    let before = nets_of(&app, id);
    assert_eq!(before.len(), 2, "the fixture source has two terminals");

    let held = app
        .state
        .workspace
        .stimulus_library
        .get("sensor_drive")
        .cloned()
        .expect("held");
    let line = commit_adoption(&mut app.state, id, &held).expect("the adoption commits");
    assert!(line.contains("re-placed as"), "{line}");
    assert!(line.contains("kept their nets"), "{line}");

    let component = instance_of(&app, id);
    assert_eq!(component.kind, ComponentType::VoltageSourceSin);
    assert_eq!(component.value, "0");
    assert_eq!(component.params, "va=3m freq=1k");
    assert_eq!(
        app.state
            .workspace
            .stimulus_library
            .provenance_state(&component),
        ProvenanceState::Adopted { revision: 1 }
    );
    assert_eq!(nets_of(&app, id), before, "both terminals keep their nets");

    assert!(app.state.schematic.undo(), "the adoption is undoable");
    let restored = instance_of(&app, id);
    assert_eq!(restored.kind, ComponentType::VoltageSourcePulse);
    assert!(
        restored.stimulus_provenance.is_none(),
        "one undo takes back the type and the card together"
    );
}

/// Same family, same quantity: a plain copy, and the console line says so.
#[test]
fn a_same_family_adoption_is_a_copy_with_a_receipt() {
    let (mut app, id) = fixture();
    let held = definition(
        "clock",
        ComponentType::VoltageSourcePulse,
        "0",
        "v2=1.8 pw=1u per=2u",
    );
    app.state
        .workspace
        .stimulus_library
        .insert(held.clone())
        .expect("insert");

    let line = commit_adoption(&mut app.state, id, &held).expect("the adoption commits");
    assert!(!line.contains("re-placed"), "{line}");
    let component = instance_of(&app, id);
    assert_eq!(component.kind, ComponentType::VoltageSourcePulse);
    assert_eq!(component.params, "v2=1.8 pw=1u per=2u");
}

/// Adopting across kinds is refused with the model's own sentence, and the
/// sheet is left exactly as it was.
#[test]
fn a_cross_kind_adoption_is_refused_rather_than_converted() {
    let (mut app, id) = fixture();
    let held = app
        .state
        .workspace
        .stimulus_library
        .get("leak")
        .cloned()
        .expect("held");
    let refusal = commit_adoption(&mut app.state, id, &held).expect_err("kinds do not convert");
    assert!(refusal.contains("current definition"), "{refusal}");
    assert!(refusal.contains("Duplicate the definition"), "{refusal}");

    let component = instance_of(&app, id);
    assert_eq!(component.kind, ComponentType::VoltageSourcePulse);
    assert!(component.stimulus_provenance.is_none());
}

/// The list is grouped by what adopting would do to this instance: the plain
/// copies first under the instance's own family, then the ones that re-place
/// it, then the ones of the other quantity that cannot drive it at all.
#[test]
fn the_list_groups_definitions_by_what_adopting_them_would_do() {
    let (app, id) = wide_fixture();
    let mut cache = MiniCache::new();
    assert_eq!(
        groups_of(&app, id, "", &mut cache),
        [
            "Same family \u{00b7} PULSE \u{00b7} copies parameters",
            "clock",
            "Other families \u{00b7} re-places V1",
            "bias_rail",
            "sensor_drive",
            "Current sources \u{00b7} cannot drive V1",
            "leak",
            "probe_tone",
        ],
        "family order inside a group, then name"
    );
}

/// A group with nothing in it is absent rather than headed by nothing.
#[test]
fn a_library_of_the_other_quantity_shows_one_group() {
    let (app, id) = kind_only_fixture();
    let mut cache = MiniCache::new();
    assert_eq!(
        groups_of(&app, id, "", &mut cache),
        [
            "Current sources \u{00b7} cannot drive V1",
            "leak",
            "probe_tone",
        ]
    );
}

/// The filter narrows the rows and keeps the labels of the groups that still
/// have one, so a reader can always see which of the three a match belongs to.
#[test]
fn the_filter_narrows_the_rows_and_keeps_the_group_labels() {
    let (app, id) = wide_fixture();
    let mut cache = MiniCache::new();
    assert_eq!(
        groups_of(&app, id, "sin", &mut cache),
        [
            "Other families \u{00b7} re-places V1",
            "sensor_drive",
            "Current sources \u{00b7} cannot drive V1",
            "probe_tone",
        ],
        "the family keyword matches, and both surviving groups keep their label"
    );
    assert_eq!(
        groups_of(&app, id, "CLO", &mut cache),
        [
            "Same family \u{00b7} PULSE \u{00b7} copies parameters",
            "clock"
        ],
        "matching is case-insensitive and drops the groups that empty"
    );
    assert!(
        groups_of(&app, id, "nothing matches this", &mut cache).is_empty(),
        "a filter that matches nothing leaves no labels behind"
    );
}

/// A walk of the list never lands on a row the primary would refuse.
#[test]
fn arrow_keys_step_over_the_rows_that_cannot_be_adopted() {
    let (app, id) = wide_fixture();
    let mut cache = MiniCache::new();
    ensure_minis(
        &mut cache,
        &app.state.workspace.stimulus_library,
        PreviewTiming::default(),
    );
    let component = instance_of(&app, id);
    let groups = adopt_groups(&app.state, &component, "", &cache);

    let mut pick = stepped(&groups, None, 1).expect("the first reachable row");
    let mut walk = vec![pick.clone()];
    for _ in 0..6 {
        pick = stepped(&groups, Some(&pick), 1).expect("a next row");
        walk.push(pick.clone());
    }
    assert_eq!(
        walk,
        [
            "clock",
            "bias_rail",
            "sensor_drive",
            "sensor_drive",
            "sensor_drive",
            "sensor_drive",
            "sensor_drive"
        ],
        "the walk crosses the group boundary and stops before the refused rows"
    );
    assert_eq!(
        stepped(&groups, Some("sensor_drive"), -1).as_deref(),
        Some("bias_rail"),
        "and walks back the same way"
    );

    let (app, id) = kind_only_fixture();
    let component = instance_of(&app, id);
    let groups = adopt_groups(&app.state, &component, "", &cache);
    assert!(
        stepped(&groups, None, 1).is_none(),
        "a library of nothing adoptable has nowhere to step to"
    );
}

/// Every definition's mini is evaluated once for as long as the dialog holds
/// it, and a revision the library has moved past is evaluated again.
#[test]
fn the_list_minis_are_evaluated_once_per_definition_revision() {
    let (mut app, _) = wide_fixture();
    let mut cache = MiniCache::new();
    let timing = PreviewTiming::default();
    assert_eq!(
        ensure_minis(&mut cache, &app.state.workspace.stimulus_library, timing),
        5,
        "the first frame evaluates every definition the library holds"
    );
    for _ in 0..3 {
        assert_eq!(
            ensure_minis(&mut cache, &app.state.workspace.stimulus_library, timing),
            0,
            "later frames evaluate nothing"
        );
    }

    app.state
        .workspace
        .stimulus_library
        .insert(definition(
            "ramp",
            ComponentType::VoltageSourcePwl,
            "0 0 1u 5",
            "",
        ))
        .expect("insert");
    assert_eq!(
        ensure_minis(&mut cache, &app.state.workspace.stimulus_library, timing),
        1,
        "a definition the library gained is evaluated, and only it"
    );

    // The revision is half the key: a curve held for another revision of the
    // same name does not answer for this one, which is what keeps a definition
    // edited in the Stimulus Library workspace from redrawing as its old shape.
    let mut stale = MiniCache::new();
    stale.insert(
        ("clock".to_owned(), 2),
        Ok(crate::simulation::stimulus_realize::WaveformTrace::Curve(
            Vec::new(),
        )),
    );
    assert_eq!(
        ensure_minis(&mut stale, &app.state.workspace.stimulus_library, timing),
        6,
        "every definition is still evaluated at the revision the library holds"
    );
}

/// Saving publishes the card the instance already carries and leaves the
/// instance reading `adopted · r1` rather than `modified`.
#[test]
fn extraction_publishes_the_instance_card_and_leaves_it_adopted() {
    let (mut app, id) = fixture();
    open_stimulus_link(&mut app.state, id, StimulusLinkMode::Extract).expect("opens");
    assert_eq!(app.state.dialogs.stimulus_link.name, "v1_pulse");
    app.state.dialogs.stimulus_link.purpose = "bench clock".to_owned();

    let line = commit_extraction(&mut app.state, id).expect("the extraction commits");
    assert!(line.contains("v1_pulse"), "{line}");
    let held = app
        .state
        .workspace
        .stimulus_library
        .get("v1_pulse")
        .expect("the library holds the new definition");
    assert_eq!(held.revision(), 1);
    assert_eq!(held.purpose, "bench clock");
    assert_eq!(held.component_type(), ComponentType::VoltageSourcePulse);

    let component = instance_of(&app, id);
    assert_eq!(
        app.state
            .workspace
            .stimulus_library
            .provenance_state(&component),
        ProvenanceState::Adopted { revision: 1 }
    );
    assert_eq!(
        app.state.workbench.selected_stimulus_definition.as_deref(),
        Some("v1_pulse")
    );
}

/// Neither half of an extraction lands without the other.
///
/// The library goes first and comes back out if the instance refuses, because
/// the other order leaves a receipt naming a definition the project does not
/// hold: every surface reads that as `definition removed`, over an extraction
/// the reader was told had failed.
#[test]
fn a_refused_extraction_leaves_both_the_library_and_the_instance_as_they_were() {
    let (mut app, id) = fixture();
    open_stimulus_link(&mut app.state, id, StimulusLinkMode::Extract).expect("opens");
    let held = app.state.workspace.stimulus_library.clone();

    // A name the library already holds is refused before anything moves.
    app.state.dialogs.stimulus_link.name = "sensor_drive".to_owned();
    let refusal = commit_extraction(&mut app.state, id).expect_err("the name is taken");
    assert!(refusal.contains("already defines"), "{refusal}");
    assert_eq!(app.state.workspace.stimulus_library, held);

    // An instance transaction the schematic refuses takes the definition back
    // out, so the two halves cannot land apart.
    app.state.dialogs.stimulus_link.name = "v1_pulse".to_owned();
    app.state
        .schematic
        .begin_operation("a gesture already in flight");
    let refusal = commit_extraction(&mut app.state, id)
        .expect_err("a pending gesture refuses a component edit");
    assert!(
        app.state
            .workspace
            .stimulus_library
            .get("v1_pulse")
            .is_none(),
        "the definition is withdrawn when the instance edit refuses: {refusal}"
    );
    assert_eq!(app.state.workspace.stimulus_library, held);
    let component = instance_of(&app, id);
    assert!(component.stimulus_provenance.is_none());
}

/// Saving a definition is a project edit, and the project says so.
///
/// The definitions ride the project document rather than a sidecar, so the
/// lifecycle registry digests the library as its own document; without that an
/// edited library would move the saved file while every document read clean.
#[test]
fn an_extraction_leaves_the_project_with_unsaved_changes() {
    use crate::workbench::lifecycle::project_lifecycle::{
        ProjectDocumentId, accept_loaded_project, dirty_documents, has_unsaved_changes, snapshot,
    };

    let (mut app, id) = fixture();
    let baseline = snapshot(&app.state).expect("the fixture project snapshots");
    accept_loaded_project(&mut app.state, baseline, None);
    assert!(
        !has_unsaved_changes(&app.state),
        "the fixture starts from an accepted baseline"
    );

    open_stimulus_link(&mut app.state, id, StimulusLinkMode::Extract).expect("opens");
    commit_extraction(&mut app.state, id).expect("the extraction commits");

    assert!(has_unsaved_changes(&app.state));
    assert!(
        dirty_documents(&app.state).contains(&ProjectDocumentId::StimulusLibrary),
        "the library is its own document: {:?}",
        dirty_documents(&app.state)
    );
}

/// A name the netlist reader would not accept, and one the library already
/// holds, are both refused in the model's own words.
#[test]
fn the_extract_name_is_refused_in_the_models_own_words() {
    let (app, id) = fixture();
    let component = instance_of(&app, id);

    let invalid = extract_refusal(&app.state, &component, "two words").expect("refused");
    assert!(invalid.contains("is not a SPICE identifier"), "{invalid}");
    let duplicate = extract_refusal(&app.state, &component, "sensor_drive").expect("refused");
    assert!(
        duplicate.contains("this project already defines 'sensor_drive'"),
        "{duplicate}"
    );
    assert!(extract_refusal(&app.state, &component, "v1_pulse").is_none());
}

/// Adopt refuses to open over a project with no definitions, because a list
/// with nothing in it is a dialog that can only be cancelled.
#[test]
fn adopt_refuses_to_open_over_an_empty_library() {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .schematic
        .add_component(ComponentType::VoltageSourcePulse, Point::new(40, 40));
    let refusal = open_stimulus_link(&mut app.state, id, StimulusLinkMode::Adopt)
        .expect_err("there is nothing to adopt");
    assert!(
        refusal.contains("holds no stimulus definitions"),
        "{refusal}"
    );
    assert!(!app.state.dialogs.stimulus_link.open);
}

/// Opening adopt lands on the definition the instance already adopted.
#[test]
fn adopt_opens_on_the_definition_the_instance_already_carries() {
    let (mut app, id) = fixture();
    let held = app
        .state
        .workspace
        .stimulus_library
        .get("sensor_drive")
        .cloned()
        .expect("held");
    commit_adoption(&mut app.state, id, &held).expect("adopted");

    open_stimulus_link(&mut app.state, id, StimulusLinkMode::Adopt).expect("opens");
    assert_eq!(
        app.state.dialogs.stimulus_link.pick.as_deref(),
        Some("sensor_drive")
    );
    assert_eq!(app.state.dialogs.stimulus_link.chip, "adopted \u{00b7} r1");
    assert_eq!(
        app.state.dialogs.stimulus_link.provenance,
        ProvenanceState::Adopted { revision: 1 }
    );
}

/// Both modes state their whole subject, in both themes, at the two desktop
/// viewports the shell gates — and nothing they paint is cut off.
///
/// A height assertion proves nothing here: the rasterizer clips at the
/// viewport, so a surface that overflowed would report exactly the viewport's
/// height. What a reader loses when a dialog does not fit is a run of text, so
/// the runs are what this checks — that the ones that carry the transaction are
/// there, and that every run painted lies inside the region it was clipped to.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn both_modes_fit_and_state_their_subject_at_every_gated_viewport() {
    for screen in [egui::vec2(1024.0, 640.0), egui::vec2(1440.0, 900.0)] {
        for mode in [Mode::Dark, Mode::Light] {
            let unpicked = painted(Scene::Adopt(None), screen, mode);
            for expected in [
                "Adopt a definition onto V1",
                "Same family \u{00b7} PULSE \u{00b7} copies parameters",
                "Other families \u{00b7} re-places V1",
                "Current sources \u{00b7} cannot drive V1",
                "sensor_drive",
                "leak",
                "Pick a definition to see the card V1 would hold",
                "Adopt",
                "Cancel",
            ] {
                assert!(
                    unpicked.text.contains(expected),
                    "{screen:?} {mode:?}: adopt lost {expected:?}: {}",
                    unpicked.text
                );
            }
            unpicked.assert_nothing_runs_past_its_pane(screen, mode);

            let picked = painted(Scene::Adopt(Some("sensor_drive")), screen, mode);
            for expected in [
                "After adoption \u{00b7} sensor_drive r1 \u{00b7} SIN",
                "SIN(",
                "Re-place and adopt",
                "Terminals",
                "Provenance",
            ] {
                assert!(
                    picked.text.contains(expected),
                    "{screen:?} {mode:?}: adopt lost {expected:?}: {}",
                    picked.text
                );
            }
            picked.assert_nothing_runs_past_its_pane(screen, mode);

            let save = painted(Scene::Save(None), screen, mode);
            for expected in [
                "Save V1 as a definition",
                "Name",
                "Purpose",
                "Published card",
                "Unique SPICE identifier",
                "Stored in",
                "Save definition",
            ] {
                assert!(
                    save.text.contains(expected),
                    "{screen:?} {mode:?}: save lost {expected:?}: {}",
                    save.text
                );
            }
            save.assert_nothing_runs_past_its_pane(screen, mode);
        }
    }
}

/// The two waveforms are one instrument: the same plot width, the same left
/// and right edges and the same height, so a time on one row is directly above
/// the same time on the other.
#[test]
fn the_two_waveform_rows_are_one_instrument() {
    let frame = egui::Rect::from_min_size(
        egui::pos2(120.0, 40.0),
        egui::vec2(
            400.0,
            crate::properties::source_preview::instrument_height(2, 64.0) - 16.0,
        ),
    );
    let rows = crate::properties::source_preview::strip_rows(frame, 64.0, 2);
    assert_eq!(rows.len(), 2);
    assert!(
        (rows[0].plot.left() - rows[1].plot.left()).abs() <= 0.5
            && (rows[0].plot.right() - rows[1].plot.right()).abs() <= 0.5,
        "the plots do not share their x range: {:?} {:?}",
        rows[0].plot,
        rows[1].plot
    );
    assert!(
        (rows[0].plot.height() - rows[1].plot.height()).abs() <= 0.5,
        "the plots are different heights: {} {}",
        rows[0].plot.height(),
        rows[1].plot.height()
    );
    for row in &rows {
        let gap = row.plot.top() - row.caption.bottom();
        assert!(
            (0.0..=4.0).contains(&gap),
            "a caption sits {gap} points above the plot it names"
        );
        assert_eq!(
            row.caption.left(),
            row.plot.left(),
            "both captions start at the plot's own left edge"
        );
        for other in &rows {
            // Touching edges are not an overlap: a caption's floor is above
            // the plot's ceiling, which is the whole point of giving it a row.
            assert!(
                row.caption.bottom() <= other.plot.top()
                    || row.caption.top() >= other.plot.bottom(),
                "a caption is painted over a plot: {:?} over {:?}",
                row.caption,
                other.plot
            );
        }
    }
    let air = rows[1].plot.top() - rows[0].plot.bottom();
    assert!(
        air >= 30.0,
        "only {air} points separate the two plots: the divider, the caption and both traces \
         are stacked with no air between them"
    );
}

/// One left edge serves the whole preview pane: the caption, the widest value
/// label and the card block all start on it, so there is no dead band between
/// the pane's inset and the numbers.
///
/// The footer's consequence note starts at the dialog's own left inset too,
/// directly after its mark, rather than floating in the middle of the footer.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_preview_pane_and_the_footer_note_share_one_left_edge() {
    let painted = painted(
        Scene::Adopt(Some("sensor_drive")),
        egui::vec2(1024.0, 640.0),
        Mode::Dark,
    );
    let facts = painted
        .run("Component")
        .expect("the pane's first fact label");
    let tick = painted
        .runs
        .iter()
        .filter(|(text, rect, _)| {
            rect.left() > facts.left() - 4.0 && (text.ends_with(" V") || text.ends_with(" mV"))
        })
        .map(|(_, rect, _)| rect.left())
        .fold(f32::INFINITY, f32::min);
    assert!(
        tick.is_finite() && (tick - facts.left()).abs() <= 2.0,
        "the widest value label starts at {tick} and the facts list at {}: there is a dead \
         band between the pane's inset and its numbers",
        facts.left()
    );
    // The fixture's pulse repeats five hundred times over the project's
    // transient, so the row is a measured band and its caption says so.
    let caption = painted
        .run("Now \u{00b7} PULSE \u{00b7} 500 cycles \u{00b7} envelope")
        .expect("the first row's caption");
    assert!(
        caption.left() > tick + 10.0,
        "the first row's caption is inside the frame, right of the shared value gutter"
    );

    // Every run that lies in the value gutter is a tick label. No two of them
    // crowd each other, which is the same claim as the two plots having air
    // between them, read off what was actually painted.
    let mut ticks: Vec<egui::Rect> = painted
        .runs
        .iter()
        .filter(|(_, rect, _)| rect.left() >= facts.left() - 1.0 && rect.right() <= caption.left())
        .map(|(_, rect, _)| *rect)
        .collect();
    ticks.sort_by(|left, right| left.top().total_cmp(&right.top()));
    assert!(
        ticks.len() >= 5,
        "the gutter labels were not found: {ticks:?}"
    );
    for pair in ticks.windows(2) {
        let gap = pair[1].top() - pair[0].bottom();
        assert!(
            gap >= 8.0,
            "two value labels are {gap} points apart: {:?} and {:?}",
            pair[0],
            pair[1]
        );
    }

    let note = painted
        .run("Re-places V1 as a SIN source. One undo restores the PULSE instance.")
        .expect("the consequence note");
    assert!(
        note.left() - painted.surface.left() <= 60.0,
        "the footer note starts {} points into a {} point dialog, which is centred rather \
         than left-aligned behind its mark",
        note.left() - painted.surface.left(),
        painted.surface.width()
    );
}

/// No axis label, readout or fact this dialog publishes is in exponent
/// notation. `3.00e-3` on a surface whose whole job is to say what a source
/// does was the defect that closed this class.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn nothing_the_dialog_paints_is_exponent_notation() {
    for scene in [
        Scene::Adopt(None),
        Scene::Adopt(Some("sensor_drive")),
        Scene::Adopt(Some("bias_rail")),
        Scene::Save(None),
    ] {
        let painted = painted(scene, egui::vec2(1024.0, 640.0), Mode::Dark);
        for run in painted.text.lines() {
            assert!(
                !is_exponent_notation(run),
                "{scene:?} painted {run:?} in exponent notation"
            );
        }
        assert!(
            painted.text.contains("mV") || painted.text.contains(" V"),
            "{scene:?} states no engineering unit at all: {}",
            painted.text
        );
    }
}

/// Whether a painted run holds a number in exponent notation. Spelled out
/// rather than matched on `e-`, because `re-places` is a word this dialog says.
#[cfg(not(target_arch = "wasm32"))]
fn is_exponent_notation(run: &str) -> bool {
    let bytes: Vec<char> = run.chars().collect();
    bytes.windows(3).any(|window| {
        window[0].is_ascii_digit()
            && (window[1] == 'e' || window[1] == 'E')
            && (window[2].is_ascii_digit() || window[2] == '-' || window[2] == '+')
    })
}

/// Which dialog, over which fixture, with what picked.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy)]
enum Scene {
    /// Adopt over the five-definition library, with an optional pick.
    Adopt(Option<&'static str>),
    /// Adopt over a library of nothing this instance can take.
    AdoptKindOnly,
    /// Save, optionally with a name the library already holds.
    Save(Option<&'static str>),
}

#[cfg(not(target_arch = "wasm32"))]
impl Scene {
    /// One app with this scene's dialog already open on it.
    fn open(self) -> RSpiceApp {
        let (mut app, id) = match self {
            Self::AdoptKindOnly => kind_only_fixture(),
            _ => wide_fixture(),
        };
        let mode = match self {
            Self::Save(_) => StimulusLinkMode::Extract,
            _ => StimulusLinkMode::Adopt,
        };
        open_stimulus_link(&mut app.state, id, mode).expect("opens");
        match self {
            Self::Adopt(pick) => app.state.dialogs.stimulus_link.pick = pick.map(str::to_owned),
            Self::Save(Some(name)) => {
                app.state.dialogs.stimulus_link.name = name.to_owned();
            }
            _ => {}
        }
        app
    }

    /// The file stem the review render is written under.
    fn stem(self) -> String {
        match self {
            Self::Adopt(None) => "adopt-nothing-picked".to_owned(),
            Self::Adopt(Some(pick)) => format!("adopt-{}", pick.replace('_', "-")),
            Self::AdoptKindOnly => "adopt-kind-only".to_owned(),
            Self::Save(None) => "save-valid-name".to_owned(),
            Self::Save(Some(_)) => "save-refused-name".to_owned(),
        }
    }
}

/// Everything one scene paints, and where.
#[cfg(not(target_arch = "wasm32"))]
struct Painted {
    text: String,
    /// Every painted run with the rectangle it occupies and the rectangle it
    /// was clipped to.
    runs: Vec<(String, egui::Rect, egui::Rect)>,
    /// The widest filled rectangle narrower than the viewport, which is the
    /// dialog's own surface. Read back rather than recomputed, so this stays a
    /// test of what was painted and not a second copy of the kit's geometry.
    surface: egui::Rect,
}

#[cfg(not(target_arch = "wasm32"))]
impl Painted {
    /// Where one exact run was painted.
    fn run(&self, text: &str) -> Option<egui::Rect> {
        self.runs
            .iter()
            .find(|(painted, _, _)| painted == text)
            .map(|(_, rect, _)| *rect)
    }

    /// Nothing this surface painted lands outside the dialog, and nothing the
    /// list painted runs out of the pane it was clipped to. Both are facts a
    /// reader silently does not get.
    ///
    /// Only the horizontal extent is checked against the clip: a long library
    /// is meant to scroll, so a row below the fold is a row the reader can
    /// reach, while a name past the pane's right edge is one they cannot.
    fn assert_nothing_runs_past_its_pane(&self, screen: egui::Vec2, theme_mode: Mode) {
        for (text, rect, clip) in &self.runs {
            let past_surface = (self.surface.left() - rect.left())
                .max(rect.right() - self.surface.right())
                .max(self.surface.top() - rect.top())
                .max(rect.bottom() - self.surface.bottom());
            assert!(
                past_surface <= 1.0,
                "{screen:?} {theme_mode:?}: {text:?} lands {past_surface} points outside the \
                 dialog (run {rect:?}, surface {:?})",
                self.surface
            );
            let past_clip = (clip.left() - rect.left()).max(rect.right() - clip.right());
            assert!(
                past_clip <= 1.0,
                "{screen:?} {theme_mode:?}: {text:?} runs {past_clip} points past its pane \
                 (run {rect:?}, pane {clip:?})"
            );
        }
    }
}

/// A theme in one mode, everything else as the product ships it.
#[cfg(not(target_arch = "wasm32"))]
fn themed(mode: Mode) -> crate::ui::Theme {
    crate::ui::Theme {
        mode,
        ..crate::ui::Theme::default()
    }
}

/// Lay one scene out and read back every run it painted.
#[cfg(not(target_arch = "wasm32"))]
fn painted(scene: Scene, screen: egui::Vec2, mode: Mode) -> Painted {
    fn walk(shape: &egui::epaint::Shape, clip: egui::Rect, into: &mut Painted, screen: egui::Vec2) {
        match shape {
            egui::epaint::Shape::Text(painted) => {
                let text = painted.galley.job.text.clone();
                into.text.push_str(&text);
                into.text.push('\n');
                into.runs.push((
                    text,
                    egui::Rect::from_min_size(painted.pos, painted.galley.size()),
                    clip,
                ));
            }
            egui::epaint::Shape::Rect(painted) => {
                let rect = painted.rect;
                if rect.width() < screen.x - 1.0 && rect.width() > into.surface.width() {
                    into.surface = rect;
                }
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, clip, into, screen);
                }
            }
            _ => {}
        }
    }

    let mut app = scene.open();
    let ctx = egui::Context::default();
    themed(mode).apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, screen)),
        ..egui::RawInput::default()
    };
    // Two passes: the first builds the font set and measures the surface, the
    // second lays out against both, which is when the runs are worth reading.
    let _ = ctx.run_ui(input(), |ctx| {
        render_stimulus_link_dialog(ctx, &mut app.state);
    });
    let output = ctx.run_ui(input(), |ctx| {
        render_stimulus_link_dialog(ctx, &mut app.state);
    });
    let mut painted = Painted {
        text: String::new(),
        runs: Vec::new(),
        surface: egui::Rect::NOTHING,
    };
    for clipped in &output.shapes {
        walk(&clipped.shape, clipped.clip_rect, &mut painted, screen);
    }
    assert!(
        painted.surface.width() > 400.0,
        "the dialog surface was not painted: {:?}",
        painted.surface
    );
    painted
}

/// Write every reviewable state of both modes, in both themes, to PNGs.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_stimulus_link_modes() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    for scene in [
        Scene::Adopt(None),
        Scene::Adopt(Some("clock")),
        Scene::Adopt(Some("sensor_drive")),
        Scene::AdoptKindOnly,
        Scene::Save(None),
        Scene::Save(Some("sensor_drive")),
    ] {
        for (suffix, mode) in [("dark", Mode::Dark), ("light", Mode::Light)] {
            let mut app = scene.open();
            let canvas = crate::ui::raster::render_themed(
                themed(mode),
                egui::vec2(1024.0, 640.0),
                |ui, background| {
                    let _ = background;
                    render_stimulus_link_dialog(ui.ctx(), &mut app.state);
                },
            );
            let height = canvas.content_height().max(1);
            let path = directory.join(format!("stimulus-link-{}-{suffix}.png", scene.stem()));
            std::fs::write(&path, canvas.png(height)).expect("write dialog render");
            writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                .expect("write raster qualification report");
        }
    }
}
