//! The toolbar's simulation-plan chip: the product's global entry to the plan
//! manager.
//!
//! The chip is measured, painted, and announced from one derivation of its
//! copy, because its width, its two clipped lines, and its accessibility label
//! all describe the same plan. Naming that plan from a literal was how the chip
//! kept announcing a fixture plan after a rename, while the live summary beside
//! it made the stale name read as authoritative — so the guard below scans the
//! shipped halves of this file and of the toolbar that reserves room for it.

use egui::Vec2;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::CommandAvailability;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::WorkbenchIcon;

/// Widest the plan chip may grow before its copy is clipped.
///
/// Plan names are user-authored and validated up to 96 characters, so some
/// name always has to be clipped; the question is only where. 190 pt left
/// 137 pt of copy — barely the default name — and the space the chip takes
/// comes out of the context-tool lane, which already scrolls. This is the same
/// ceiling `explicit_label_width` gives the toolbar's other text-bearing
/// controls, and `trailing_controls_width` must reserve exactly it: a
/// reservation smaller than the paint width lets the scrolling lane run under
/// the chip.
pub(super) const RUN_CONFIG_CHIP_MAX_WIDTH: f32 = 224.0;

/// Every string the plan chip measures, paints, and announces.
struct RunConfigChipCopy {
    title: String,
    summary: String,
    accessibility: String,
}

fn run_config_chip_copy(app: &RSpiceApp) -> RunConfigChipCopy {
    let title = app.state.sim_setup.active_plan_name().as_str().to_owned();
    let analysis_count = app.state.sim_setup.enabled_analysis_instance_count();
    let pvt_count = configured_pvt_count(app);
    RunConfigChipCopy {
        accessibility: format!("Simulation plan {title}; manage and switch simulation plans"),
        title,
        summary: format!("{pvt_count} PVT · {analysis_count} analyses"),
    }
}

/// Chip box for measured copy: the icon gutter, the wider of the two lines,
/// and the chevron gutter, capped at what the toolbar reserves for it.
fn run_config_chip_width(title_width: f32, summary_width: f32) -> f32 {
    (31.0 + title_width.max(summary_width) + 22.0).min(RUN_CONFIG_CHIP_MAX_WIDTH)
}

pub(super) fn run_config_selector(ui: &mut egui::Ui, app: &mut RSpiceApp, height: f32) {
    let t = Tokens::get(ui.ctx());
    let copy = run_config_chip_copy(app);
    let availability = Command::ManageSimulationPlans.availability(app);
    let enabled = availability == CommandAvailability::Available && ui.is_enabled();
    let title_width = ui
        .painter()
        .layout_no_wrap(
            copy.title.clone(),
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.text,
        )
        .size()
        .x;
    let summary_width = ui
        .painter()
        .layout_no_wrap(
            copy.summary.clone(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        )
        .size()
        .x;
    let width = run_config_chip_width(title_width, summary_width);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width, height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, &copy.accessibility)
    });

    let hovered = enabled && response.hovered();
    ui.painter().rect(
        rect,
        t.radius,
        if hovered {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
        egui::Stroke::new(
            1.0,
            if hovered {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Sliders.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 16.0, rect.center().y),
            Vec2::splat(16.0),
        ),
        if enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let copy_clip = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 31.0, rect.top()),
        egui::pos2(rect.right() - 22.0, rect.bottom()),
    );
    let copy_painter = ui.painter().with_clip_rect(copy_clip);
    copy_painter.text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y - 6.5),
        egui::Align2::LEFT_CENTER,
        &copy.title,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    copy_painter.text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y + 7.0),
        egui::Align2::LEFT_CENTER,
        &copy.summary,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.right() - 10.0, rect.center().y),
            Vec2::splat(11.0),
        ),
        t.color.text_faint,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    if response.clicked() && enabled {
        Command::ManageSimulationPlans.execute(app);
    }
    if enabled {
        response.on_hover_text(&copy.accessibility);
    } else if let CommandAvailability::Disabled(reason) = availability {
        response.on_hover_text(reason);
    }
}

fn configured_pvt_count(app: &RSpiceApp) -> usize {
    // The plan-wide Run Set expands every enabled analysis. Legacy Corner and
    // Temperature analysis drafts no longer own the workspace's PVT count and
    // must not be added a second time in the chrome summary.
    app.state.sim_setup.run_set.point_count().max(1)
}

#[cfg(test)]
mod tests {
    use super::super::{TOOLBAR_CONTEXT_GAP, trailing_controls_width};
    use super::*;
    use crate::workbench::layout::LayoutSpec;
    use crate::workbench::state::{WorkbenchState, Workspace};

    /// The chip is the product's global entry to the plan manager, so the plan
    /// it names has to be the plan that is active — not the name the fixture
    /// project happened to ship with.
    #[test]
    fn plan_chip_copy_follows_a_rename_of_the_active_plan() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .id();

        app.state
            .sim_setup
            .rename_plan(plan_id, "Automotive temperature sweep")
            .expect("rename the active plan");
        let copy = run_config_chip_copy(&app);

        assert_eq!(copy.title, "Automotive temperature sweep");
        assert_eq!(
            copy.accessibility,
            "Simulation plan Automotive temperature sweep; \
             manage and switch simulation plans"
        );
        assert!(!copy.title.contains("Lab characterization"));
        assert!(!copy.accessibility.contains("Lab characterization"));
        // The live half of the chip is unchanged by the rename.
        assert!(copy.summary.ends_with(" analyses"));
    }

    /// Three literals — the width measurement, the accessibility label and the
    /// painted line — named one fixture plan while the summary beside them was
    /// live, which is what made the stale name read as authoritative.
    ///
    /// The guard follows the paint. `include_str!` resolves against the
    /// including file's own directory, so it reads this module for the chip's
    /// copy and `../toolbar.rs` for the lane that reserves room for it: the
    /// literal must not come back in either place.
    #[test]
    fn the_toolbar_never_hardcodes_a_simulation_plan_name() {
        for source in [
            include_str!("run_config_chip.rs"),
            include_str!("../toolbar.rs"),
        ] {
            let production = crate::source_guard::production_half(source);

            assert!(!production.contains("Lab characterization"));
        }
    }

    /// A user-authored name is valid up to 96 characters, so the chip must clip
    /// rather than grow — and it must clip at exactly the width the toolbar
    /// reserved for it, or the scrolling context lane runs underneath it.
    #[test]
    fn a_long_plan_name_clips_inside_the_width_the_toolbar_reserves() {
        assert_eq!(run_config_chip_width(400.0, 120.0), RUN_CONFIG_CHIP_MAX_WIDTH);
        assert_eq!(run_config_chip_width(120.0, 400.0), RUN_CONFIG_CHIP_MAX_WIDTH);
        assert_eq!(run_config_chip_width(90.0, 100.0), 153.0);

        let wide = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchState::default());
        assert!(wide.show_run_config_selector);
        let mut without_chip = wide;
        without_chip.show_run_config_selector = false;
        assert_eq!(
            trailing_controls_width(wide, 5.0, true, 92.0, 112.0)
                - trailing_controls_width(without_chip, 5.0, true, 92.0, 112.0),
            RUN_CONFIG_CHIP_MAX_WIDTH + TOOLBAR_CONTEXT_GAP
        );
    }

    /// The chip's whole purpose is the plan manager. Navigating to Simulation
    /// Studio instead left the manager reachable only from a button inside it.
    #[test]
    fn the_plan_chip_owns_no_dialog_construction_of_its_own() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        app.state.workbench.workspace = Workspace::Design;

        Command::ManageSimulationPlans.execute(&mut app);

        assert!(matches!(
            app.state.workbench.simulation_workflow,
            Some(crate::workbench::state::SimulationWorkflowDialog::PlanManager(_))
        ));
        assert_ne!(app.state.workbench.workspace, Workspace::Simulate);
    }

    #[test]
    fn toolbar_pvt_summary_uses_the_global_run_set_exactly_once() {
        use crate::product::ProcessCorner;
        use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig};

        let mut app = RSpiceApp::test_instance();
        app.state.sim_setup.run_set =
            crate::simulation::run_set::RunSetState::from_corner_config(&CornerConfig {
                process_corners: vec![ProcessCorner::TT, ProcessCorner::SS],
                voltages: vec![1.0],
                supply_source_names: Vec::new(),
                temperatures: vec![-40.0, 27.0, 125.0],
                full_matrix: true,
                points: Vec::new(),
                base_analysis: CornerBaseAnalysis::Op,
            });

        assert_eq!(configured_pvt_count(&app), 6);
    }
}
