//! What adopting and extracting do to a placed source.
//!
//! The claims that matter are the ones a reader cannot check by eye: a
//! cross-family adoption really is a re-type that keeps both terminals on
//! their nets, one undo puts the old type *and* the old card back, and a
//! cross-kind adoption is refused rather than converted.

use super::*;
use crate::state::stimulus_library::provenance::ProvenanceState;
use crate::state::{ComponentType, Point};
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
    app.state
        .workspace
        .stimulus_library
        .insert(definition(
            "sensor_drive",
            ComponentType::VoltageSourceSin,
            "0",
            "va=3m freq=1k",
        ))
        .expect("insert");
    app.state
        .workspace
        .stimulus_library
        .insert(definition(
            "bias_rail",
            ComponentType::VoltageSource,
            "1.8",
            "",
        ))
        .expect("insert");
    app.state
        .workspace
        .stimulus_library
        .insert(definition("leak", ComponentType::CurrentSource, "1u", ""))
        .expect("insert");
    (app, id)
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

    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the instance survives the re-type");
    assert_eq!(component.kind, ComponentType::VoltageSourceSin);
    assert_eq!(component.value, "0");
    assert_eq!(component.params, "va=3m freq=1k");
    assert_eq!(
        app.state
            .workspace
            .stimulus_library
            .provenance_state(component),
        ProvenanceState::Adopted { revision: 1 }
    );
    assert_eq!(nets_of(&app, id), before, "both terminals keep their nets");

    assert!(app.state.schematic.undo(), "the adoption is undoable");
    let restored = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("undo restores the instance");
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
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("held");
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

    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("held");
    assert_eq!(component.kind, ComponentType::VoltageSourcePulse);
    assert!(component.stimulus_provenance.is_none());
}

/// The list is ordered so a reader meets the plain copies first, then the ones
/// that re-place the instance, then the ones that cannot be adopted at all.
#[test]
fn the_definition_list_leads_with_the_adoptions_that_are_a_copy() {
    let (mut app, id) = fixture();
    app.state
        .workspace
        .stimulus_library
        .insert(definition(
            "clock",
            ComponentType::VoltageSourcePulse,
            "0",
            "pw=1u",
        ))
        .expect("insert");
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
        .expect("held");

    let rows = adopt_rows(&app.state, &component);
    let order: Vec<&str> = rows.iter().map(|row| row.name.as_str()).collect();
    assert_eq!(order, ["clock", "bias_rail", "sensor_drive", "leak"]);
    assert_eq!(rows[0].fit, AdoptionFit::Same);
    assert!(matches!(rows[1].fit, AdoptionFit::Replace { .. }));
    assert!(
        rows[1].detail.contains("re-places the instance"),
        "{:?}",
        rows[1].detail
    );
    assert_eq!(rows[3].fit, AdoptionFit::Kind);
    assert!(rows[3].refusal.is_some(), "a refused row states why");
}

/// Extraction publishes the card the instance already carries and leaves the
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

    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("held");
    assert_eq!(
        app.state
            .workspace
            .stimulus_library
            .provenance_state(component),
        ProvenanceState::Adopted { revision: 1 }
    );
    assert_eq!(
        app.state.workbench.selected_stimulus_definition.as_deref(),
        Some("v1_pulse")
    );
}

/// A name the netlist reader would not accept, and one the library already
/// holds, are both refused in the model's own words.
#[test]
fn the_extract_name_is_refused_in_the_models_own_words() {
    let (app, id) = fixture();
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
        .expect("held");

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
}

/// Both modes state everything they are for at the smallest desktop viewport
/// the shell gates.
///
/// A height assertion proves nothing here — the rasterizer clips at the
/// viewport, so a surface that overflowed would report exactly the viewport's
/// height. What a reader loses when a dialog does not fit is a fact, so the
/// facts are what this checks: every definition the library holds is listed,
/// the instance is named, and the verb the reader came for is on screen.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn both_modes_state_their_whole_subject_at_the_gated_desktop_viewport() {
    // Nothing picked: the fixture instance adopted nothing and carries no
    // definition's card, so the dialog opens on the whole list and says what
    // the empty preview column is waiting for.
    let unpicked = painted(StimulusLinkMode::Adopt, None, egui::vec2(1024.0, 640.0));
    for expected in [
        "Adopt a stimulus definition \u{00b7} V1",
        "sensor_drive",
        "bias_rail",
        "leak",
        "re-places the instance",
        "Pick a definition",
        "Adopt definition",
        "Cancel",
    ] {
        assert!(
            unpicked.contains(expected),
            "adopt lost {expected:?}: {unpicked}"
        );
    }

    let picked = painted(
        StimulusLinkMode::Adopt,
        Some("sensor_drive"),
        egui::vec2(1024.0, 640.0),
    );
    for expected in [
        "The card after adoption",
        "SIN(",
        "Adopting a SIN definition onto a PULSE source",
        "Replace and adopt",
    ] {
        assert!(
            picked.contains(expected),
            "adopt lost {expected:?}: {picked}"
        );
    }

    let extract = painted(StimulusLinkMode::Extract, None, egui::vec2(1024.0, 640.0));
    for expected in [
        "Save as library definition",
        "Definition name",
        "Purpose",
        "The card this definition publishes",
        "Save definition",
    ] {
        assert!(
            extract.contains(expected),
            "extract lost {expected:?}: {extract}"
        );
    }
}

/// Everything one mode of the dialog paints, as text.
#[cfg(not(target_arch = "wasm32"))]
fn painted(mode: StimulusLinkMode, pick: Option<&str>, screen: egui::Vec2) -> String {
    fn walk(shape: &egui::epaint::Shape, into: &mut String) {
        match shape {
            egui::epaint::Shape::Text(painted) => {
                into.push_str(&painted.galley.job.text);
                into.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, into);
                }
            }
            _ => {}
        }
    }

    let (mut app, id) = fixture();
    open_stimulus_link(&mut app.state, id, mode).expect("opens");
    if let Some(pick) = pick {
        app.state.dialogs.stimulus_link.pick = Some(pick.to_owned());
    }
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, screen)),
        ..egui::RawInput::default()
    };
    // Two passes: the first builds the font set, the second lays out against
    // it, which is when the runs are worth reading.
    let _ = ctx.run_ui(input(), |ctx| {
        render_stimulus_link_dialog(ctx, &mut app.state);
    });
    let output = ctx.run_ui(input(), |ctx| {
        render_stimulus_link_dialog(ctx, &mut app.state);
    });
    let mut text = String::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut text);
    }
    text
}

/// Render one mode of the dialog over a viewport, for the review PNGs below.
#[cfg(not(target_arch = "wasm32"))]
fn render_mode(
    mode: StimulusLinkMode,
    pick: Option<&str>,
    screen: egui::Vec2,
) -> crate::ui::raster::Canvas {
    let (mut app, id) = fixture();
    open_stimulus_link(&mut app.state, id, mode).expect("opens");
    if let Some(pick) = pick {
        app.state.dialogs.stimulus_link.pick = Some(pick.to_owned());
    }
    crate::ui::raster::render(screen, |ui, background| {
        let _ = background;
        render_stimulus_link_dialog(ui.ctx(), &mut app.state);
    })
}

/// Write both modes to PNGs so their design can be reviewed.
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
    for (name, mode, pick) in [
        ("adopt", StimulusLinkMode::Adopt, Some("sensor_drive")),
        ("extract", StimulusLinkMode::Extract, None),
    ] {
        let canvas = render_mode(mode, pick, egui::vec2(1024.0, 640.0));
        let height = canvas.content_height().max(1);
        let path = directory.join(format!("stimulus-link-{name}.png"));
        std::fs::write(&path, canvas.png(height)).expect("write dialog render");
        writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
            .expect("write raster qualification report");
    }
}
