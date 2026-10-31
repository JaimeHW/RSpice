//! The Configuration inspector: what it states with and without a row, and
//! that typing into it never re-resolves the design.

use super::*;

use crate::workbench::RSpiceApp;
use crate::workbench::examples::hierarchy_reference;

fn reference_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = hierarchy_reference::build().state;
    app
}

/// Resolve what the stage would show, as the page does every frame.
fn resolve(app: &mut RSpiceApp) {
    let state = &mut app.state;
    state.project.configuration.resolution(
        &state.model.workspace,
        &state.model.library_manager,
        &state.model.workspace.content.active_view,
        &state.model.schematic,
    );
}

/// One frame of the inspector with `events`, and every string it painted.
fn frame(ctx: &egui::Context, app: &mut RSpiceApp, events: Vec<egui::Event>) -> Vec<String> {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 1200.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                project_configuration(ui, &mut app.state.project.configuration, &app.state.model)
            });
        },
    );
    fn collect(shape: &egui::epaint::Shape, out: &mut Vec<String>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push(text.galley.job.text.clone()),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }
    let mut painted = Vec::new();
    for shape in &output.shapes {
        collect(&shape.shape, &mut painted);
    }
    painted
}

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

#[test]
fn with_nothing_selected_it_states_the_configuration() {
    let mut app = reference_app();
    resolve(&mut app);
    let painted = frame(&context(), &mut app, Vec::new());
    for fact in [
        "Reference hierarchy",
        "user/top/schematic",
        "/X1",
        "Used for runs",
        "Yes",
        "Instance bindings",
        "Cell bindings",
    ] {
        assert!(
            painted.iter().any(|text| text == fact),
            "{fact:?} not in {painted:#?}"
        );
    }
    for retired in [
        "Manage configuration sets\u{2026}",
        "Validate configuration",
        "Active binding",
        "Overrides",
    ] {
        assert!(!painted.iter().any(|text| text == retired), "{retired:?}");
    }
}

#[test]
fn on_default_it_states_the_built_in_facts() {
    let mut app = RSpiceApp::test_instance();
    resolve(&mut app);
    let painted = frame(&context(), &mut app, Vec::new());
    for fact in [
        "Default",
        "the instance's own view, then extracted, then spice",
        "spice, verilog, veriloga, extracted",
        "Stop the netlist",
        "Yes",
    ] {
        assert!(
            painted.iter().any(|text| text == fact),
            "{fact:?} not in {painted:#?}"
        );
    }
}

/// A model section is typed into a buffer; only Enter commits it, so the
/// design is resolved again once, not once per keystroke.
#[test]
fn typing_a_model_section_resolves_the_design_only_when_it_commits() {
    let mut app = reference_app();
    app.state.project.configuration.row = Some(ConfigurationRow::Instance("/X1".to_owned()));
    resolve(&mut app);
    let builds = app.state.project.configuration.resolution_builds();
    let ctx = context();
    frame(&ctx, &mut app, Vec::new());
    let field = egui::Id::new(("configuration-field", "instance-/X1-section"));
    ctx.memory_mut(|memory| memory.request_focus(field));
    for keystroke in ["t", "t"] {
        frame(
            &ctx,
            &mut app,
            vec![egui::Event::Text(keystroke.to_owned())],
        );
        resolve(&mut app);
    }
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    assert_eq!(
        app.state
            .project
            .configuration
            .field_edit
            .as_ref()
            .map(|edit| edit.text.as_str()),
        Some("tt")
    );
    assert!(!app.state.project.configuration.is_dirty(&catalog));
    assert_eq!(app.state.project.configuration.resolution_builds(), builds);

    frame(
        &ctx,
        &mut app,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
    resolve(&mut app);
    let id = catalog.active_configuration_id().unwrap();
    let definition = app
        .state
        .project
        .configuration
        .definition(&catalog, id)
        .unwrap();
    assert_eq!(
        instance_binding(definition, "/X1").and_then(|binding| binding.model_section.as_deref()),
        Some("tt")
    );
    assert_eq!(
        app.state.project.configuration.resolution_builds(),
        builds + 1
    );
}
