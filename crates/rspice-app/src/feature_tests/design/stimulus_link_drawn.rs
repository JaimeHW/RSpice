//! Linking a stimulus, drawn through a whole application.

use crate::properties::source_preview::MiniCache;
use crate::properties::source_preview::ensure_minis;
use crate::simulation::stimulus_realize::PreviewTiming;
use crate::state::stimulus_library::provenance::ProvenanceState;
use crate::state::{ComponentType, Point};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::stimulus_link::adopt::{adopt_groups, stepped};
use crate::workbench::app::dialogs::stimulus_link::tests::{
    definition, fixture, instance_of, wide_fixture,
};
use crate::workbench::app::dialogs::stimulus_link::*;
use crate::workbench::app_state::AppState;

/// Save the instance's card as the dialog's primary does, over the whole
/// state.
fn commit_extraction(state: &mut AppState, component_id: u64) -> Result<String, String> {
    crate::workbench::app::dialogs::stimulus_link::commit_extraction(
        &state.design.stimulus_link,
        &mut state.design.selected_stimulus_definition,
        &mut state.model,
        component_id,
    )
}

/// A library holding nothing this instance could ever adopt.
fn kind_only_fixture() -> (RSpiceApp, u64) {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::VoltageSourcePulse, Point::new(40, 40));
    for held in [
        definition("leak", ComponentType::CurrentSource, "1u", ""),
        definition(
            "probe_tone",
            ComponentType::CurrentSourceSin,
            "0",
            "ia=1u freq=10k",
        ),
    ] {
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .insert(held)
            .expect("insert");
    }
    (app, id)
}

/// The groups the list would show, with every mini already evaluated.
fn groups_of(app: &RSpiceApp, id: u64, filter: &str, cache: &mut MiniCache) -> Vec<String> {
    ensure_minis(
        cache,
        &app.state.model.workspace.content.stimulus_library,
        PreviewTiming::default(),
    );
    let component = instance_of(app, id);
    adopt_groups(&app.state.model, &component, filter, cache)
        .into_iter()
        .flat_map(|group| {
            std::iter::once(group.label).chain(group.rows.into_iter().map(|row| row.name))
        })
        .collect()
}

fn nets_of(app: &RSpiceApp, id: u64) -> Vec<String> {
    crate::simulation::placed_sources::placed_sources(
        &app.state.model.schematic,
        &app.state.model.workspace.content.stimulus_library,
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
        .model
        .workspace
        .content
        .stimulus_library
        .get("sensor_drive")
        .cloned()
        .expect("held");
    let line = commit_adoption(&mut app.state.model, id, &held).expect("the adoption commits");
    assert!(line.contains("re-placed as"), "{line}");
    assert!(line.contains("kept their nets"), "{line}");

    let component = instance_of(&app, id);
    assert_eq!(component.kind, ComponentType::VoltageSourceSin);
    assert_eq!(component.value, "0");
    assert_eq!(component.params, "va=3m freq=1k");
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .provenance_state(&component),
        ProvenanceState::Adopted { revision: 1 }
    );
    assert_eq!(nets_of(&app, id), before, "both terminals keep their nets");

    assert!(app.state.model.schematic.undo(), "the adoption is undoable");
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
        .model
        .workspace
        .content
        .stimulus_library
        .insert(held.clone())
        .expect("insert");

    let line = commit_adoption(&mut app.state.model, id, &held).expect("the adoption commits");
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
        .model
        .workspace
        .content
        .stimulus_library
        .get("leak")
        .cloned()
        .expect("held");
    let refusal =
        commit_adoption(&mut app.state.model, id, &held).expect_err("kinds do not convert");
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
        &app.state.model.workspace.content.stimulus_library,
        PreviewTiming::default(),
    );
    let component = instance_of(&app, id);
    let groups = adopt_groups(&app.state.model, &component, "", &cache);

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
    let groups = adopt_groups(&app.state.model, &component, "", &cache);
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
        ensure_minis(
            &mut cache,
            &app.state.model.workspace.content.stimulus_library,
            timing
        ),
        5,
        "the first frame evaluates every definition the library holds"
    );
    for _ in 0..3 {
        assert_eq!(
            ensure_minis(
                &mut cache,
                &app.state.model.workspace.content.stimulus_library,
                timing
            ),
            0,
            "later frames evaluate nothing"
        );
    }

    app.state
        .model
        .workspace
        .content
        .stimulus_library
        .insert(definition(
            "ramp",
            ComponentType::VoltageSourcePwl,
            "0 0 1u 5",
            "",
        ))
        .expect("insert");
    assert_eq!(
        ensure_minis(
            &mut cache,
            &app.state.model.workspace.content.stimulus_library,
            timing
        ),
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
        ensure_minis(
            &mut stale,
            &app.state.model.workspace.content.stimulus_library,
            timing
        ),
        6,
        "every definition is still evaluated at the revision the library holds"
    );
}

/// Saving publishes the card the instance already carries and leaves the
/// instance reading `adopted · r1` rather than `modified`.
#[test]
fn extraction_publishes_the_instance_card_and_leaves_it_adopted() {
    let (mut app, id) = fixture();
    open_stimulus_link(
        &mut app.state.design.stimulus_link,
        &app.state.model,
        id,
        StimulusLinkMode::Extract,
    )
    .expect("opens");
    assert_eq!(app.state.design.stimulus_link.name, "v1_pulse");
    app.state.design.stimulus_link.purpose = "bench clock".to_owned();

    let line = commit_extraction(&mut app.state, id).expect("the extraction commits");
    assert!(line.contains("v1_pulse"), "{line}");
    let held = app
        .state
        .model
        .workspace
        .content
        .stimulus_library
        .get("v1_pulse")
        .expect("the library holds the new definition");
    assert_eq!(held.revision(), 1);
    assert_eq!(held.purpose, "bench clock");
    assert_eq!(held.component_type(), ComponentType::VoltageSourcePulse);

    let component = instance_of(&app, id);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .provenance_state(&component),
        ProvenanceState::Adopted { revision: 1 }
    );
    assert_eq!(
        app.state.design.selected_stimulus_definition.as_deref(),
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
    open_stimulus_link(
        &mut app.state.design.stimulus_link,
        &app.state.model,
        id,
        StimulusLinkMode::Extract,
    )
    .expect("opens");
    let held = app.state.model.workspace.content.stimulus_library.clone();

    // A name the library already holds is refused before anything moves.
    app.state.design.stimulus_link.name = "sensor_drive".to_owned();
    let refusal = commit_extraction(&mut app.state, id).expect_err("the name is taken");
    assert!(refusal.contains("already defines"), "{refusal}");
    assert_eq!(app.state.model.workspace.content.stimulus_library, held);

    // An instance transaction the schematic refuses takes the definition back
    // out, so the two halves cannot land apart.
    app.state.design.stimulus_link.name = "v1_pulse".to_owned();
    app.state
        .model
        .schematic
        .begin_operation("a gesture already in flight");
    let refusal = commit_extraction(&mut app.state, id)
        .expect_err("a pending gesture refuses a component edit");
    assert!(
        app.state
            .model
            .workspace
            .content
            .stimulus_library
            .get("v1_pulse")
            .is_none(),
        "the definition is withdrawn when the instance edit refuses: {refusal}"
    );
    assert_eq!(app.state.model.workspace.content.stimulus_library, held);
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
    use crate::workbench::app_state::services_access::accept_loaded_project;
    use crate::workbench::lifecycle::project_lifecycle::{
        ProjectDocumentId, dirty_documents, has_unsaved_changes, snapshot,
    };

    let (mut app, id) = fixture();
    let baseline = snapshot(&app.state).expect("the fixture project snapshots");
    accept_loaded_project(&mut app.state, baseline, None);
    assert!(
        !has_unsaved_changes(&app.state),
        "the fixture starts from an accepted baseline"
    );

    open_stimulus_link(
        &mut app.state.design.stimulus_link,
        &app.state.model,
        id,
        StimulusLinkMode::Extract,
    )
    .expect("opens");
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

    let invalid = extract_refusal(&app.state.model, &component, "two words").expect("refused");
    assert!(invalid.contains("is not a SPICE identifier"), "{invalid}");
    let duplicate = extract_refusal(&app.state.model, &component, "sensor_drive").expect("refused");
    assert!(
        duplicate.contains("this project already defines 'sensor_drive'"),
        "{duplicate}"
    );
    assert!(extract_refusal(&app.state.model, &component, "v1_pulse").is_none());
}

/// Adopt refuses to open over a project with no definitions, because a list
/// with nothing in it is a dialog that can only be cancelled.
#[test]
fn adopt_refuses_to_open_over_an_empty_library() {
    let mut app = RSpiceApp::test_instance();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::VoltageSourcePulse, Point::new(40, 40));
    let refusal = open_stimulus_link(
        &mut app.state.design.stimulus_link,
        &app.state.model,
        id,
        StimulusLinkMode::Adopt,
    )
    .expect_err("there is nothing to adopt");
    assert!(
        refusal.contains("holds no stimulus definitions"),
        "{refusal}"
    );
    assert!(!app.state.design.stimulus_link.open);
}

/// Opening adopt lands on the definition the instance already adopted.
#[test]
fn adopt_opens_on_the_definition_the_instance_already_carries() {
    let (mut app, id) = fixture();
    let held = app
        .state
        .model
        .workspace
        .content
        .stimulus_library
        .get("sensor_drive")
        .cloned()
        .expect("held");
    commit_adoption(&mut app.state.model, id, &held).expect("adopted");

    open_stimulus_link(
        &mut app.state.design.stimulus_link,
        &app.state.model,
        id,
        StimulusLinkMode::Adopt,
    )
    .expect("opens");
    assert_eq!(
        app.state.design.stimulus_link.pick.as_deref(),
        Some("sensor_drive")
    );
    assert_eq!(app.state.design.stimulus_link.chip, "adopted \u{00b7} r1");
    assert_eq!(
        app.state.design.stimulus_link.provenance,
        ProvenanceState::Adopted { revision: 1 }
    );
}
