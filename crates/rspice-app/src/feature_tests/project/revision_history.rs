//! Schematic revisions on the History page, through the whole application:
//! each revision states the date and time it was made, Compare… names what
//! changed in its view's drawing since, and Restore… on a revision of a
//! schematic that is not open opens it and puts its drawing back.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use super::recovery_frames::{
    app_frame, click_lowest, click_on, offscreen, paint_until, painted, select_row,
};
use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, Library, Point, SchematicState, ValidatedSchematicRevisionId,
    View, ViewType,
};
use rspice_app_state::time_compat::local_stamp;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_design::schematic::validated_revision::{
    ValidatedRevisionRequest, ValidationFindingCounts,
};

/// A drawn revision: which it is, and when it was made as the history
/// states it.
struct Made {
    id: ValidatedSchematicRevisionId,
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
        stamp: local_stamp(record.created_unix_ms()),
    }
}

/// The schematic view of a second cell, `bench/amp/schematic`.
fn amplifier() -> CellViewRef {
    CellViewRef::new("bench", "amp", "schematic")
}

/// A project whose open schematic has two revisions, a baseline and the
/// resistor added after it, and whose amplifier, not open, has one it has
/// moved on from; with File > Revision history… taken, which shows History.
fn history_app() -> (RSpiceApp, [Made; 3]) {
    let mut app = RSpiceApp::test_instance();
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
    assert_eq!(app.state.project.project_page, ProjectPage::History);
    (app, [baseline, resistor, amplifier_review])
}

fn selected(app: &RSpiceApp) -> Option<String> {
    app.state.project.project_history_selection.clone()
}

fn key(made: &Made) -> String {
    format!("schematic:{}", made.id.as_uuid())
}

/// Revision history opens on the newest revision of the schematic in front.
/// Every revision states when it was made; Compare… on the baseline names
/// the resistor added since; and Restore… on the amplifier's revision asks
/// first, then opens the amplifier with its revision's drawing.
#[test]
fn a_schematic_revision_is_compared_and_restored_from_the_history_page() {
    let ctx = offscreen();
    let (mut app, [baseline, resistor, amp]) = history_app();
    assert_eq!(selected(&app), Some(key(&resistor)));
    let shown = paint_until(&ctx, &mut app, |text| text.contains("Amplifier review"));
    let text = painted(&shown);
    for made in [&baseline, &resistor, &amp] {
        assert!(
            text.contains(&made.stamp),
            "{} is not stated:\n{text}",
            made.stamp
        );
    }

    // The baseline: what changed since it, by name.
    let picked = select_row(&ctx, &mut app, &shown, "Baseline review");
    assert_eq!(selected(&app), Some(key(&baseline)));
    app_frame(&ctx, &mut app, click_on(&picked, "Compare\u{2026}"));
    let compared = paint_until(&ctx, &mut app, |text| {
        text.contains("Compare schematic revision") && text.contains("Added")
    });
    let text = painted(&compared);
    for stated in [baseline.stamp.as_str(), "Components"] {
        assert!(text.contains(stated), "{stated:?}\n{text}");
    }
    app_frame(&ctx, &mut app, click_lowest(&compared, "Close"));
    let shown = paint_until(&ctx, &mut app, |text| {
        !text.contains("Compare schematic revision")
    });

    // The amplifier's revision, then Restore and its question.
    let picked = select_row(&ctx, &mut app, &shown, "Amplifier review");
    assert_eq!(selected(&app), Some(key(&amp)));
    app_frame(&ctx, &mut app, click_on(&picked, "Restore\u{2026}"));
    let asked = paint_until(&ctx, &mut app, |text| {
        text.contains("Only the drawing of bench/amp/schematic goes back to this revision")
    });
    app_frame(&ctx, &mut app, click_on(&asked, "Restore"));
    let deadline = Instant::now() + Duration::from_secs(30);
    while app.state.model.workspace.content.active_view != amplifier()
        || app.state.model.schematic.document().components.len() != 1
    {
        assert!(
            Instant::now() < deadline,
            "the amplifier never opened on its revision's drawing"
        );
        app_frame(&ctx, &mut app, Vec::new());
    }
    let after = app_frame(&ctx, &mut app, Vec::new());
    assert!(
        !painted(&after).contains("Restore schematic revision"),
        "the question is answered"
    );
}
