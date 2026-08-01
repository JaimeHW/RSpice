//! Canonical task-owner surfaces.

mod automation;
mod design;
mod library_cellview_manager;
mod model_correlation;
mod model_correlation_controller;
mod model_editor;
mod models;
mod netlist;
mod pdk_technology_admin;
mod project;
pub(crate) mod report_authoring;
mod results;
mod simulate;
mod verify;
mod veriloga;

use egui::{Align, Layout, Rect, Sense, Ui, UiBuilder, WidgetInfo, WidgetType};

use crate::ui::tokens::Tokens;
use crate::workbench::RSpiceApp;

use super::state::Workspace;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if split_results_is_visible(app) {
        show_split_with_results(ui, app);
    } else {
        show_primary(ui, app);
    }
}

fn show_primary(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.workbench.current_route().surface_id() == super::SurfaceId::VisualizationStudio {
        crate::workbench::documents::visualization_studio::show(ui, app);
        return;
    }
    if app.state.workbench.current_route().surface_id() == super::SurfaceId::ReportAuthoring {
        report_authoring::show(ui, app);
        return;
    }
    if app.state.workbench.current_route().surface_id() == super::SurfaceId::ModelEditor {
        model_editor::show(ui, app);
        return;
    }
    if app.state.workbench.current_route().surface_id() == super::SurfaceId::ModelCorrelation {
        model_correlation::show(ui, app);
        return;
    }
    if app.state.workbench.current_route().surface_id() == super::SurfaceId::PdkTechnologyAdmin {
        pdk_technology_admin::show(ui, app);
        return;
    }
    if app.state.workbench.current_route().surface_id() == super::SurfaceId::LibraryCellviewManager
    {
        library_cellview_manager::show(ui, app);
        return;
    }
    match app.state.workbench.workspace {
        Workspace::Project => project::show(ui, app),
        Workspace::Design => design::show(ui, app),
        Workspace::Simulate => simulate::show(ui, app),
        Workspace::Results => results::show(ui, app),
        Workspace::Verify => verify::show(ui, app),
        Workspace::Models => models::show(ui, app),
        Workspace::Netlist => {
            netlist::prepare_workspace(app);
            match app.state.ui.code_workspace.page {
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist => {
                    netlist::show_prepared(ui, app)
                }
                crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA => {
                    veriloga::show(ui, app)
                }
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation => {
                    automation::show(ui, app)
                }
            }
        }
    }
}

fn split_results_is_visible(app: &RSpiceApp) -> bool {
    app.state.workbench.results_split_visible(
        app.state.project_lifecycle.project_open,
        app.state.simulation.has_retained_result_dataset(),
    )
}

/// Exact upgraded-mockup geometry: two equal min-zero panes, no gap, and one
/// physical workbench divider between them.
fn split_stage_rects(stage: Rect) -> (Rect, Rect, Rect) {
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

fn show_split_with_results(ui: &mut Ui, app: &mut RSpiceApp) {
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
    crate::workbench::documents::result_document::show_compact_split(&mut results, app);
}

fn register_results_split_region(ui: &mut Ui, results_rect: Rect) {
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

#[cfg(test)]
mod tests {
    use super::{register_results_split_region, split_stage_rects};

    #[test]
    fn split_stage_is_equal_zero_gap_with_one_pixel_divider() {
        let stage = egui::Rect::from_min_size(egui::pos2(7.0, 11.0), egui::vec2(1001.0, 620.0));
        let (primary, divider, secondary) = split_stage_rects(stage);

        assert_eq!(primary.left(), stage.left());
        assert_eq!(primary.right(), divider.left());
        assert_eq!(divider.width(), 1.0);
        assert_eq!(divider.right(), secondary.left());
        assert_eq!(secondary.right(), stage.right());
        assert!((primary.width() - secondary.width()).abs() <= f32::EPSILON);
        assert_eq!(primary.height(), stage.height());
        assert_eq!(secondary.height(), stage.height());
    }

    #[test]
    fn split_stage_remains_bounded_at_narrow_widths() {
        for width in [0.0, 0.5, 1.0, 2.0, 44.0] {
            let stage = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(width, 80.0));
            let (primary, divider, secondary) = split_stage_rects(stage);
            for rect in [primary, divider, secondary] {
                assert!(rect.left() >= stage.left());
                assert!(rect.right() <= stage.right());
                assert!(rect.width() >= 0.0);
            }
        }
    }

    #[test]
    fn results_secondary_pane_is_an_accessible_named_region() {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let rect = ui.available_rect_before_wrap();
                    register_results_split_region(ui, rect);
                });
            },
        );

        let nodes = output
            .platform_output
            .accesskit_update
            .expect("split accessibility tree")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Region
                && node.label() == Some("Results split pane")
        }));
    }
}
