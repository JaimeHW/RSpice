//! Project revision history under a finger, through the whole application:
//! on a tablet and on a phone, every comparison and restore the history
//! offers is reached by taps alone, and each revision states the date and
//! time it was made.

use egui::{Modifiers, Pos2, Rect, Vec2, pos2, vec2};

use super::recovery_frames::{offscreen, painted};
use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, Library, Point, SchematicState, ValidatedSchematicRevisionId,
    View, ViewType,
};
use rspice_app_state::time_compat::utc_stamp;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_design::schematic::validated_revision::{
    ValidatedRevisionRequest, ValidationFindingCounts,
};

const TABLET: Vec2 = vec2(820.0, 1180.0);
const PHONE: Vec2 = vec2(390.0, 844.0);

/// One whole application frame `size` points wide and tall.
fn frame(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    size: Vec2,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            events,
            ..Default::default()
        },
        |ui| {
            let ctx = ui.ctx().clone();
            app.prepare_frame(&ctx);
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(&ctx);
        },
    )
}

/// A few frames with nothing done, as the screen settles.
fn settle(ctx: &egui::Context, app: &mut RSpiceApp, size: Vec2) -> egui::FullOutput {
    let mut output = frame(ctx, app, size, Vec::new());
    for _ in 0..3 {
        output = frame(ctx, app, size, Vec::new());
    }
    output
}

/// Every label the frame announces.
fn labels(output: &egui::FullOutput) -> Vec<String> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect()
}

/// What the frame says: every string it painted and every label it
/// announces, whose words a narrow header may elide on the screen.
fn said(output: &egui::FullOutput) -> String {
    [painted(output), labels(output).join("\n")].join("\n")
}

/// A finger's tap on the part of the control `output` announces with a
/// label `named` picks that lies on the screen, then the frames it settles
/// over.
fn tap(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    size: Vec2,
    output: &egui::FullOutput,
    named: impl Fn(&str) -> bool,
) -> egui::FullOutput {
    let bounds = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| named(node.label()?).then_some(node.bounds()?))
        .unwrap_or_else(|| panic!("no control is announced so: {:#?}", labels(output)));
    let left = (bounds.x0 as f32).max(0.0);
    let right = (bounds.x1 as f32).min(size.x);
    let at = pos2((left + right) / 2.0, ((bounds.y0 + bounds.y1) / 2.0) as f32);
    assert!(
        left < right && (0.0..size.y).contains(&at.y),
        "the control lies off the screen at {bounds:?}"
    );
    let press = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::default(),
    };
    frame(
        ctx,
        app,
        size,
        vec![egui::Event::PointerMoved(at), press(true), press(false)],
    );
    settle(ctx, app, size)
}

/// A drawn revision as the history names it: its digest's first ten
/// characters, and when it was made.
struct Made {
    id: ValidatedSchematicRevisionId,
    digest: String,
    stamp: String,
}

/// Record the drawing `schematic` holds, of `view` in `project`, as a
/// validated revision noted `note`.
fn validate(schematic: &mut SchematicState, project: &str, view: &str, note: &str) -> Made {
    let digest = schematic
        .editor_ref()
        .design
        .validated_design_content_digest()
        .expect("the drawing has a digest");
    let id = schematic
        .append_validated_revision(ValidatedRevisionRequest {
            project_id: project.to_owned(),
            project_revision: 1,
            view_identity: view.to_owned(),
            revision_note: note.to_owned(),
            author: "Test engineer".to_owned(),
            validation_receipt_digest: digest,
            finding_counts: ValidationFindingCounts::default(),
            dependencies: Vec::new(),
            advisory_dispositions: Vec::new(),
        })
        .expect("the revision is recorded");
    let record = schematic
        .document()
        .validated_revisions
        .records()
        .iter()
        .find(|record| record.id() == id)
        .expect("the journal holds it");
    Made {
        id,
        digest: record
            .revision_digest()
            .to_string()
            .chars()
            .take(10)
            .collect(),
        stamp: utc_stamp(record.created_unix_ms()),
    }
}

/// The schematic view of a second cell, `bench/amp/schematic`.
fn amplifier() -> CellViewRef {
    CellViewRef::new("bench", "amp", "schematic")
}

/// A project under a finger whose open schematic has two revisions, a
/// baseline and the resistor added after it, and whose amplifier, not open,
/// has one it has moved on from; with the history open on it.
fn history_app() -> (RSpiceApp, [Made; 3]) {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.coarse_pointer = true;
    let mut library = Library::new("bench");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);

    let project = app.state.model.workspace.content.project.id().to_string();
    let open = app.state.model.workspace.content.active_view.key();
    let top = &mut app.state.model.schematic;
    let baseline = validate(top, &project, &open, "Baseline review");
    top.add_component(ComponentType::Resistor, Point::new(40, 0));
    let resistor = validate(top, &project, &open, "Resistor review");

    let mut amp = SchematicState::default();
    amp.add_component(ComponentType::Capacitor, Point::new(0, 0));
    let amplifier_review = validate(&mut amp, &project, &amplifier().key(), "Amplifier review");
    amp.add_component(ComponentType::Resistor, Point::new(40, 0));
    app.state
        .model
        .workspace
        .content
        .schematic_buffers
        .insert(amplifier().key(), amp.editor_ref().design.clone());

    app.state.workbench.activate(Workspace::Project);
    app.execute_command_key(ProjectCommand::RevisionHistory.key());
    assert!(app.state.project.project_revision_history.open);
    (app, [baseline, resistor, amplifier_review])
}

fn selected(app: &RSpiceApp) -> Vec<String> {
    app.state.project.project_revision_history.selected()
}

fn key(made: &Made) -> String {
    format!("schematic:{}", made.id.as_uuid())
}

/// On a tablet and on a phone, a finger compares a revision with the design
/// as it stands, adds a second revision by the mark before its row and
/// compares the two, and restores a revision of a schematic that is not
/// open, which opens it: no key is held and nothing is hovered. Every
/// revision states the date and time it was made.
#[test]
fn every_comparison_and_restore_is_reached_by_taps_alone() {
    for size in [TABLET, PHONE] {
        let ctx = offscreen();
        let (mut app, [baseline, resistor, amp]) = history_app();
        let output = settle(&ctx, &mut app, size);
        let text = painted(&output);
        for made in [&baseline, &resistor, &amp] {
            assert!(
                text.contains(&made.stamp),
                "{size:?}: {} is not stated:\n{text}",
                made.stamp
            );
        }

        // The baseline row, then Compare with current.
        let output = tap(&ctx, &mut app, size, &output, |label| {
            label.contains("Change: Baseline review")
        });
        assert_eq!(selected(&app), vec![key(&baseline)], "{size:?}");
        let output = tap(&ctx, &mut app, size, &output, |label| {
            label == "Compare with current"
        });
        let text = said(&output);
        assert!(
            text.contains(&format!(
                "{} ({}) \u{2192} current design",
                baseline.digest, baseline.stamp
            )),
            "{size:?}:\n{text}"
        );

        // The mark before the resistor's row adds it; Compare selected.
        let output = tap(&ctx, &mut app, size, &output, |label| {
            label == format!("Select {}", resistor.digest)
        });
        assert_eq!(
            selected(&app),
            vec![key(&resistor), key(&baseline)],
            "{size:?}"
        );
        let output = tap(&ctx, &mut app, size, &output, |label| {
            label == "Compare selected"
        });
        let text = said(&output);
        assert!(
            text.contains(&format!(
                "{} ({}) \u{2192} {} ({})",
                baseline.digest, baseline.stamp, resistor.digest, resistor.stamp
            )),
            "{size:?}:\n{text}"
        );

        // The amplifier's revision alone, then Restore and its confirmation.
        let output = tap(&ctx, &mut app, size, &output, |label| {
            label.contains("Change: Amplifier review")
        });
        assert_eq!(selected(&app), vec![key(&amp)], "{size:?}");
        let output = tap(&ctx, &mut app, size, &output, |label| {
            label == "Restore selected\u{2026}"
        });
        let text = said(&output);
        assert!(
            text.contains("Restore opens bench/amp/schematic and replaces its drawing"),
            "{size:?}:\n{text}"
        );
        let output = tap(&ctx, &mut app, size, &output, |label| {
            label == "Confirm restore"
        });
        let model = &app.state.model;
        assert_eq!(model.workspace.content.active_view, amplifier(), "{size:?}");
        assert_eq!(
            model.schematic.document().components.len(),
            1,
            "{size:?}: the amplifier is its revision's drawing again"
        );
        assert!(app.state.project.project_revision_history.open, "{size:?}");
        let text = said(&output);
        assert!(
            !text.contains("Revision history cannot continue"),
            "{size:?}:\n{text}"
        );
    }
}
