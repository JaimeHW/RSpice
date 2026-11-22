//! A SPICE directive note's line that stops a run is repaired on the sheet:
//! following the finding opens the drawing with the note selected and says
//! which of its lines it is.

use super::*;
use rspice_app_state::state::{DesignNote, DesignNoteKind, DrawnObject, Point};

#[test]
fn following_a_directive_lines_refusal_selects_the_note_and_names_the_line() {
    let mut app = super::startup::rc();
    let root = app
        .state
        .model
        .workspace
        .content
        .simulation_root_reference();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(
            DesignNote::new(
                7_001,
                Point::new(40, 300),
                DesignNoteKind::SpiceDirective,
                ".param rload=1k\n.model = d",
            )
            .expect("a directive note"),
        );
    app.state.sync_active_schematic_to_workspace();

    start(&mut app.state, &mut app.simulation_controller, false);
    app.settle_preflight();
    let blockers = preflight_blockers(&app);
    let refusal = blockers
        .iter()
        .find(|issue| {
            matches!(
                issue.remediation,
                PreflightRemediation::Drawing {
                    object: DrawnObject::Note { .. },
                    ..
                }
            )
        })
        .unwrap_or_else(|| panic!("the run is refused on the note: {blockers:?}"))
        .clone();
    assert!(
        refusal.observed.starts_with(&format!(
            "SPICE directive on {}, line 2: ",
            root.display_path()
        )),
        "{refusal:?}"
    );

    app.state.workbench.activate(Workspace::Verify);
    apply_remediation(&mut app, refusal.remediation);

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .active_schematic_reference(),
        root
    );
    let editor = &app.state.model.schematic.session.editor;
    assert_eq!(
        editor.selection.design_notes.iter().collect::<Vec<_>>(),
        [&7_001],
        "the note, and nothing else, is selected"
    );
    assert!(editor.selection.components.is_empty());
    assert_eq!(editor.center_request, Some(Point::new(40, 300)));
    assert_eq!(
        app.state.prompt_text(),
        Some("Line 2 of the selected SPICE directive stopped the run.")
    );
}
