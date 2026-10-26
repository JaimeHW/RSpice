//! Canonical task-owner surfaces.

use egui::{Align, Layout, Rect, Sense, Ui, UiBuilder, WidgetInfo, WidgetType};

use crate::ui::tokens::Tokens;
use crate::workbench::framework::host::Point;
use crate::workbench::framework::requests::Drain;
use crate::workbench::framework::workbench_host::WorkbenchHost;

pub fn show(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    // The page that draws a source editor publishes it again below; whatever
    // else is on screen has none for the Edit menu to act on.
    app.workbench_mut().active_source_editor = None;
    if split_results_is_visible(app) {
        show_split_with_results(ui, app);
    } else {
        show_primary(ui, app);
    }
    if app.workbench().outbox.waits_at(Drain::SourceEditor) {
        if app.workbench().active_source_editor.is_some() {
            // Asked after the editor was drawn: it takes it next frame.
            ui.ctx().request_repaint();
        } else {
            // No editor is on screen to take it.
            app.workbench_mut().outbox.decline_at(Drain::SourceEditor);
        }
    }
}

/// The stage of the workspace that is showing, drawn by whoever owns it.
fn show_primary(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    let workspace = app.workbench().workspace;
    app.contribute(Point::Surface(workspace), ui);
}

fn split_results_is_visible(app: &impl WorkbenchHost) -> bool {
    app.workbench().results_split_visible(
        app.platform().project_lifecycle.is_open(),
        app.model()
            .simulation
            .retained
            .has_retained_result_dataset(),
    )
}

/// Exact upgraded-mockup geometry: two equal min-zero panes, no gap, and one
/// physical workbench divider between them.
pub fn split_stage_rects(stage: Rect) -> (Rect, Rect, Rect) {
    let divider_width = 1.0_f32.min(stage.width().max(0.0));
    let pane_width = (stage.width() - divider_width).max(0.0) * 0.5;
    let primary = Rect::from_min_max(
        stage.min,
        egui::pos2(stage.left() + pane_width, stage.bottom()),
    );
    let divider = Rect::from_min_max(
        primary.right_top(),
        egui::pos2(primary.right() + divider_width, stage.bottom()),
    );
    let secondary = Rect::from_min_max(
        egui::pos2(divider.right(), stage.top()),
        stage.right_bottom(),
    );
    (primary, divider, secondary)
}

fn show_split_with_results(ui: &mut Ui, app: &mut impl WorkbenchHost) {
    let stage = ui.available_rect_before_wrap();
    let (_, stage_response) = ui.allocate_exact_size(stage.size(), Sense::hover());
    let (primary_rect, divider_rect, results_rect) = split_stage_rects(stage_response.rect);
    let divider_color = Tokens::get(ui.ctx()).color.border_strong;
    ui.painter().rect_filled(divider_rect, 0.0, divider_color);

    let mut primary = ui.new_child(
        UiBuilder::new()
            .max_rect(primary_rect)
            .layout(Layout::top_down(Align::LEFT)),
    );
    primary.set_clip_rect(primary_rect);
    primary.set_min_size(primary_rect.size());
    primary.set_max_size(primary_rect.size());
    show_primary(&mut primary, app);

    register_results_split_region(ui, results_rect);

    let mut results = ui.new_child(
        UiBuilder::new()
            .max_rect(results_rect)
            .layout(Layout::top_down(Align::LEFT)),
    );
    results.set_clip_rect(results_rect);
    results.set_min_size(results_rect.size());
    results.set_max_size(results_rect.size());
    app.contribute(Point::SplitResults, &mut results);
}

pub fn register_results_split_region(ui: &mut Ui, results_rect: Rect) {
    let region = ui.interact(
        results_rect,
        ui.make_persistent_id("results-split-pane"),
        Sense::hover(),
    );
    region.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Other, ui.is_enabled(), "Results split pane")
    });
    ui.ctx().accesskit_node_builder(region.id, |node| {
        node.set_role(egui::accesskit::Role::Region);
        node.set_label("Results split pane");
    });
}
