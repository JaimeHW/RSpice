//! Setup-page routing and the shared page heading.
//!
//! The analyses page owns the ordered plan; every other route owns exactly one
//! plan-scoped concern. Routing lives here so the surface entry point stays a
//! scroll host and each page states only what it owns.

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::{PreflightCurrency, SimulationPage};

use super::super::super::design_system::{heading, workspace_title_row};
use super::page_kit::setup_page;

/// Horizontal padding inside the preflight chip, and the gap it keeps from
/// the page's own primary action.
const CHIP_PADDING_X: f32 = 9.0;
const CHIP_GAP: f32 = 8.0;
const CHIP_HEIGHT: f32 = 22.0;

/// Render the selected setup page. The analyses route is handled by the
/// surface itself and never reaches here.
pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp, page: SimulationPage) {
    workspace_title_row(ui, |ui| page_heading(ui, app, page));
    setup_page(ui, |ui| match page {
        SimulationPage::Analyses => {}
        SimulationPage::Variables => super::page_variables::show(ui, app),
        SimulationPage::Outputs => super::page_outputs::show(ui, app),
        SimulationPage::Specifications => super::page_specs::show(ui, app),
        SimulationPage::RunSet => super::page_runset::show(ui, app),
        SimulationPage::Models => super::page_models::show(ui, app),
        SimulationPage::Solver => super::page_solver::show(ui, app),
        SimulationPage::Save => super::page_save::show(ui, app),
    });
}

/// Eyebrow, title, description, and the page's own primary action.
///
/// The eyebrow states the page's authority in the plan, derived from the same
/// state the page edits — never a static caption.
fn page_heading(ui: &mut Ui, app: &mut RSpiceApp, page: SimulationPage) {
    let eyebrow = eyebrow(app, page);
    let currency = preflight_currency(app);
    let chip = currency.is_stated().then(|| chip_text(currency));
    ui.horizontal(|ui| {
        let available = ui.available_width();
        let action = primary_action(page);
        let action_width = if action.is_some() { 190.0 } else { 0.0 };
        let chip_reserve = chip
            .as_ref()
            .map_or(0.0, |text| chip_width(ui, text) + CHIP_GAP);
        ui.allocate_ui_with_layout(
            egui::vec2((available - action_width - chip_reserve).max(1.0), 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                heading(ui, &eyebrow, page.title(), page.description());
            },
        );
        if action.is_none() && chip.is_none() {
            return;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if let Some((label, command)) = action
                && Button::new(label).accent().show(ui).clicked()
            {
                command(app);
            }
            if let Some(text) = chip {
                ui.add_space(CHIP_GAP);
                if preflight_chip(ui, currency, &text) {
                    Command::PreflightChecks.execute(app);
                }
            }
        });
    });
}

/// Where this session's preflight report stands, for the chip above.
///
/// A session with no report short-circuits before the topology closure is
/// walked: that walk traverses the configured hierarchy, and paying it on
/// every frame of every setup route to learn "no preflight yet" would be the
/// header charging the whole design for a fact the state already holds.
fn preflight_currency(app: &RSpiceApp) -> PreflightCurrency {
    if app.state.workbench.preflight.report.is_none() {
        return PreflightCurrency::Absent;
    }
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    app.state.workbench.preflight.currency(
        app.state.workspace.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        app.state.active_plan_revision(),
    )
}

fn chip_text(currency: PreflightCurrency) -> String {
    match currency.detail() {
        Some(detail) => format!("PREFLIGHT {} · {detail}", currency.status().to_uppercase()),
        None => format!("PREFLIGHT {}", currency.status().to_uppercase()),
    }
}

fn chip_font() -> egui::FontId {
    theme::mono(tokens::FS_0, FontWeight::Regular)
}

fn chip_width(ui: &Ui, text: &str) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(text.to_owned(), chip_font(), egui::Color32::WHITE)
            .size()
            .x
    }) + CHIP_PADDING_X * 2.0
}

/// Compact preflight-currency chip. Returns whether the reader asked to rerun.
///
/// Every setup route edits an input the preflight report was frozen against,
/// so every setup route owes the reader the report's standing. It is a button
/// exactly when rerunning is the next move — an authorized report has nothing
/// to ask for and stays a label rather than a control that does nothing.
fn preflight_chip(ui: &mut Ui, currency: PreflightCurrency, text: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let tone = match currency {
        PreflightCurrency::Absent => return false,
        PreflightCurrency::Expired => t.color.warn,
        PreflightCurrency::Blocked { .. } => t.color.err,
        PreflightCurrency::Authorized { .. } => t.color.ok,
    };
    let actionable = currency.wants_rerun();
    let galley = ui.fonts_mut(|fonts| fonts.layout_no_wrap(text.to_owned(), chip_font(), tone));
    let size = egui::vec2(galley.size().x + CHIP_PADDING_X * 2.0, CHIP_HEIGHT);
    let sense = if actionable {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(size, sense);
    let announcement = if actionable {
        format!("{text}. Rerun preflight checks")
    } else {
        text.to_owned()
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            if actionable {
                egui::WidgetType::Button
            } else {
                egui::WidgetType::Label
            },
            ui.is_enabled(),
            announcement.clone(),
        )
    });
    if ui.is_rect_visible(rect) {
        ui.painter().rect(
            rect,
            t.radius,
            if actionable && response.hovered() {
                t.color.bg_hover
            } else {
                egui::Color32::TRANSPARENT
            },
            egui::Stroke::new(1.0, tone),
            egui::StrokeKind::Inside,
        );
        ui.painter().galley(
            egui::pos2(
                rect.left() + CHIP_PADDING_X,
                rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            tone,
        );
    }
    if actionable {
        response
            .clone()
            .on_hover_text("Rerun simulation preflight against the current design.")
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        response.clicked()
    } else {
        response.on_hover_text("This preflight report is current for the design in front of you.");
        false
    }
}

fn eyebrow(app: &RSpiceApp, page: SimulationPage) -> String {
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.instances().len())
        .unwrap_or_default();
    match page {
        SimulationPage::Analyses => format!("PLAN · {plan} ANALYSES"),
        SimulationPage::Variables => "PARAMETERIZATION · PLAN SCOPE".to_owned(),
        SimulationPage::Outputs => "SAVED SIGNALS · EXPRESSIONS".to_owned(),
        SimulationPage::Specifications => "REQUIREMENTS · ACCEPTANCE LIMITS".to_owned(),
        SimulationPage::RunSet => "RUN SPACE · PROCESS · TEMPERATURE · VARIATION".to_owned(),
        SimulationPage::Models => "MODEL CLOSURE · SECTION BINDING · QUALIFICATION".to_owned(),
        SimulationPage::Solver => format!(
            "NUMERICS · {} · DESIGN CONTRACT",
            app.state.sim_setup.options.preset_label().to_uppercase()
        ),
        SimulationPage::Save => "RESULT STORAGE · STREAMING · RETENTION".to_owned(),
    }
}

type PageCommand = fn(&mut RSpiceApp);

fn primary_action(page: SimulationPage) -> Option<(&'static str, PageCommand)> {
    match page {
        SimulationPage::Variables => Some(("Add variable…", open_variable_dialog)),
        SimulationPage::Outputs => Some(("Add output…", open_output_dialog)),
        SimulationPage::Analyses
        | SimulationPage::Specifications
        | SimulationPage::RunSet
        | SimulationPage::Models
        | SimulationPage::Solver
        | SimulationPage::Save => None,
    }
}

fn open_variable_dialog(app: &mut RSpiceApp) {
    app.state.workbench.simulation_workflow = Some(
        crate::workbench::state::SimulationWorkflowDialog::DesignVariable(
            crate::workbench::state::DesignVariableDraft::default(),
        ),
    );
}

fn open_output_dialog(app: &mut RSpiceApp) {
    app.state.workbench.simulation_workflow = Some(
        crate::workbench::state::SimulationWorkflowDialog::SavedOutput(
            crate::workbench::state::SavedOutputDraft::default(),
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::state::{
        PreflightIssue, PreflightRemediation, PreflightReport, PreparedPreflightContract,
    };

    fn current_report(
        app: &RSpiceApp,
        blockers: Vec<PreflightIssue>,
        prepared: Option<PreparedPreflightContract>,
    ) -> PreflightReport {
        let (topology_root, topology_revision, topology_closure) =
            app.state.configured_topology_revision();
        let plan = app.state.active_plan_revision();
        PreflightReport {
            project_revision: app.state.workspace.project.revision().get(),
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan_id: plan.map(|(id, _)| id),
            simulation_plan_revision: plan.map(|(_, revision)| revision),
            blockers,
            advisories: Vec::new(),
            prepared,
        }
    }

    fn prepared_contract(task_count: usize) -> PreparedPreflightContract {
        PreparedPreflightContract {
            snapshot_digest: crate::product::ContentDigest::from_bytes([1; 32]),
            source_digest: crate::product::ContentDigest::from_bytes([2; 32]),
            receipt_digest: crate::product::ContentDigest::from_bytes([3; 32]),
            receipt_label: "receipt",
            analysis_ids: vec![crate::product::ContentDigest::from_bytes([4; 32])],
            task_count,
            saved_output_contract_count: 0,
            pvt_point_count: 1,
            target: "Desktop background thread",
            save_policy: "Retain engine-produced results",
            model_identity_count: 1,
        }
    }

    fn blocker() -> PreflightIssue {
        PreflightIssue {
            check: "Run set".to_owned(),
            observed: "No analysis is enabled".to_owned(),
            required: "One enabled analysis".to_owned(),
            remediation: PreflightRemediation::SimulationPlan,
        }
    }

    #[test]
    fn every_setup_route_states_the_preflight_report_it_was_frozen_against() {
        let mut app = RSpiceApp::test_instance();
        assert_eq!(preflight_currency(&app), PreflightCurrency::Absent);
        assert!(!PreflightCurrency::Absent.is_stated());

        app.state.workbench.preflight.report =
            Some(current_report(&app, Vec::new(), Some(prepared_contract(3))));
        assert_eq!(
            preflight_currency(&app),
            PreflightCurrency::Authorized { tasks: 3 }
        );
        assert!(!preflight_currency(&app).wants_rerun());

        app.state.workbench.preflight.report = Some(current_report(
            &app,
            vec![blocker()],
            Some(prepared_contract(3)),
        ));
        assert_eq!(
            preflight_currency(&app),
            PreflightCurrency::Blocked { blockers: 1 }
        );

        // Anything the report was frozen against moving expires it, and the
        // routes that are not Analyses are exactly where that editing happens.
        let mut stale = current_report(&app, Vec::new(), Some(prepared_contract(3)));
        stale.project_revision += 1;
        app.state.workbench.preflight.report = Some(stale);
        assert_eq!(preflight_currency(&app), PreflightCurrency::Expired);
        assert!(preflight_currency(&app).wants_rerun());
    }

    /// Render one non-Analyses route and return its AccessKit node labels.
    fn route_announcements(page: SimulationPage, seed: impl FnOnce(&mut RSpiceApp)) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.simulation_page = page;
        seed(&mut app);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 1600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| show(ui, &mut app, page));
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn an_expired_report_offers_its_rerun_from_the_route_the_reader_is_on() {
        for page in [
            SimulationPage::Outputs,
            SimulationPage::Models,
            SimulationPage::Solver,
        ] {
            let labels = route_announcements(page, |app| {
                let mut stale = current_report(app, Vec::new(), Some(prepared_contract(2)));
                stale.project_revision += 1;
                app.state.workbench.preflight.report = Some(stale);
            });
            assert!(
                labels
                    .iter()
                    .any(|label| label.contains("PREFLIGHT EXPIRED")
                        && label.contains("Rerun preflight checks")),
                "{page:?} must state the expired report and offer its rerun: {labels:?}"
            );
        }
    }

    #[test]
    fn a_route_states_nothing_when_no_preflight_has_been_run() {
        let labels = route_announcements(SimulationPage::Solver, |_| {});
        assert!(
            !labels.iter().any(|label| label.contains("PREFLIGHT")),
            "an unrun preflight is the ordinary state and is not an announcement: {labels:?}"
        );
    }
}
