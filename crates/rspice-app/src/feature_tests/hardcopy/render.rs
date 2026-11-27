//! The Print and Export dialog drawn in a whole application, and the widget
//! identities it keeps as its rows show and hide.

use crate::feature_tests::hardcopy::hardcopy_parts;
use crate::workbench::app::RSpiceApp;
use egui::Context;
use egui::Rect;
use egui::Vec2;
use egui::vec2;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::HardcopyWorkflow;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::publish;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::render::*;

#[cfg(not(target_arch = "wasm32"))]
fn studio_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state.workbench.documents.activate(
        rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId::CellView(
            reference,
        ),
    );
    publish::open_hardcopy_workflow(&mut hardcopy_parts(&mut app), HardcopyWorkflow::Export);
    app
}

#[cfg(not(target_arch = "wasm32"))]
fn render_studio(ctx: &Context, app: &mut RSpiceApp, viewport: Vec2) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, viewport)),
            ..Default::default()
        },
        |ui| render_hardcopy_dialog(&mut hardcopy_parts(app), ui),
    )
}

/// Every rectangle egui outlined because the widget occupying it answered
/// to a different id than it did on the previous pass. egui reports these
/// as a log warning and a red outline; the outline is the part a test can
/// read, and it carries the rectangle that names the offender.
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
fn id_change_marks(output: &egui::FullOutput) -> Vec<Rect> {
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Rect(shape)
                if clipped.clip_rect == Rect::EVERYTHING
                    && shape.stroke.color == egui::Color32::RED
                    && shape.stroke.width == 2.0
                    && shape.stroke_kind == egui::StrokeKind::Outside =>
            {
                Some(shape.rect)
            }
            _ => None,
        })
        .collect()
}

/// A widget's id is its continuity: focus, drag state, tooltips and the
/// scroll offsets of everything under it are keyed by it. An id derived
/// from how many siblings happened to be laid out first changes whenever
/// the surface does — a section appearing, a refusal opening the banner —
/// and every one of those is an ordinary move in this dialog.
#[test]
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
fn no_widget_changes_its_id_as_the_studio_changes() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = studio_app();
    let viewport = vec2(1_440.0, 900.0);

    let mut marks = Vec::new();
    let mut record = |output: &egui::FullOutput| {
        marks.extend(id_change_marks(output).into_iter().map(|rect| {
            format!(
                "[{:.0},{:.0}]-[{:.0},{:.0}]",
                rect.left(),
                rect.top(),
                rect.right(),
                rect.bottom()
            )
        }));
    };
    for _ in 0..200 {
        record(&render_studio(&ctx, &mut app, viewport));
        if app.state.hardcopy.hardcopy.preview.is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    // The page only arrives once the source has resolved. Without it the
    // rest of this probe would pass by never having been tested.
    assert!(
        app.state.hardcopy.hardcopy.preview.is_some(),
        "the studio never resolved a source and an exact page preview"
    );
    record(&render_studio(&ctx, &mut app, viewport));
    // The custom margins show their four fields in a row that keeps its
    // place while they are hidden.
    for custom in [true, false] {
        app.state.hardcopy.hardcopy.margins_custom = custom;
        record(&render_studio(&ctx, &mut app, viewport));
        record(&render_studio(&ctx, &mut app, viewport));
    }
    // A refusal opens the banner strip between the body and the command
    // row, and clearing it closes it again. Both are ordinary, and a
    // publication that fails late raises one over a plan that stayed valid.
    for refusal in [true, false] {
        app.state.hardcopy.hardcopy.error =
            refusal.then(|| "the publication was refused".to_owned());
        record(&render_studio(&ctx, &mut app, viewport));
        record(&render_studio(&ctx, &mut app, viewport));
    }
    app.state.hardcopy.hardcopy.margin_left = "100".to_owned();
    app.state.hardcopy.hardcopy.refresh_preview();
    record(&render_studio(&ctx, &mut app, viewport));
    record(&render_studio(&ctx, &mut app, viewport));
    app.state.hardcopy.hardcopy.margin_left = "0.5".to_owned();
    app.state.hardcopy.hardcopy.refresh_preview();
    record(&render_studio(&ctx, &mut app, viewport));
    record(&render_studio(&ctx, &mut app, viewport));

    assert!(
        marks.is_empty(),
        "widgets changed id between passes at {marks:?}"
    );
}
