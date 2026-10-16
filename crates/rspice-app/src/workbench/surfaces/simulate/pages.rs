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
    // Resolving either list walks the design's nets, so only the page that
    // shows them pays for it.
    let (excitations, rf_ports) = if page == SimulationPage::Excitations {
        design_excitations(app)
    } else {
        (Vec::new(), Vec::new())
    };
    workspace_title_row(ui, |ui| page_heading(ui, app, page));
    setup_page(ui, |ui| match page {
        SimulationPage::Analyses => {}
        SimulationPage::Excitations => {
            super::page_excitations::show(ui, &mut app.state, &excitations, &rf_ports);
        }
        SimulationPage::Variables => super::page_variables::show(ui, app),
        SimulationPage::Outputs => super::page_outputs::show(ui, app),
        SimulationPage::Specifications => super::page_specs::show(ui, app),
        SimulationPage::RunSet => super::page_runset::show(ui, app),
        SimulationPage::Models => super::page_models::show(ui, app),
        SimulationPage::Solver => super::page_solver::show(ui, app),
        SimulationPage::Save => super::page_save::show(ui, app),
    });
}

/// What the run is driven by: every excitation the configured design places,
/// at every occurrence the run reaches one through.
///
/// The whole design rather than the open sheet, because that is the question
/// this page asks. A source drawn inside a child master is netlisted, biases
/// the circuit and is read by the analyses exactly as one drawn at the root is
/// — and while this page read the editor's own buffer, the root of every
/// hierarchical design was told it places no sources at all.
///
/// A design whose configuration does not resolve falls back to that buffer.
/// The alternative is a page that answers "nothing" to "what drives this
/// circuit" because a binding elsewhere is unresolved, which is the wrong
/// answer to a different question; the rows say which reading they came from
/// by carrying no occurrence.
///
/// The reading itself is
/// [`whole_design_excitations`](crate::simulation::placed_sources::whole_design_excitations),
/// beside the count the navigator rail states for this page, so the number on
/// the way in and the rows on the way out cannot come from different designs.
fn design_excitations(
    app: &RSpiceApp,
) -> (
    Vec<crate::simulation::placed_sources::PlacedSource>,
    Vec<crate::simulation::placed_sources::PlacedRfPort>,
) {
    crate::simulation::placed_sources::whole_design_excitations(
        &app.state.library_manager,
        &app.state.workspace,
        &app.state.schematic,
        app.state.sim_setup.analysis_plan.as_ref(),
    )
}

/// The page's title, its preflight standing, and its own primary action.
fn page_heading(ui: &mut Ui, app: &mut RSpiceApp, page: SimulationPage) {
    let chip = PreflightChip::resolve(app);
    ui.horizontal(|ui| {
        let available = ui.available_width();
        let action = primary_action(page);
        let action_width = if action.is_some() { 190.0 } else { 0.0 };
        let chip_reserve = chip.as_ref().map_or(0.0, |chip| chip.reserve(ui));
        ui.allocate_ui_with_layout(
            egui::vec2((available - action_width - chip_reserve).max(1.0), 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                // Title only. The sentence that sat under it told the reader
                // what the page is for in the words of whoever built it, which
                // is a thing the controls under it already show.
                heading(ui, page.title(), None);
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
            if let Some(chip) = &chip {
                ui.add_space(CHIP_GAP);
                if chip.show(ui) {
                    Command::PreflightChecks.execute(app);
                }
            }
        });
    });
}

/// The preflight standing one heading states, resolved once for that heading.
///
/// A heading reserves the chip's width before it lays the title out, and
/// paints the chip after — two moments over one fact. Carrying it as a value
/// keeps the reserve and the paint from disagreeing, which is exactly the
/// shape of the bug that clipped this surface's accent action once already.
///
/// It is a value rather than three loose functions because the Analyses route
/// paints its own heading and cannot go through [`page_heading`]: that route
/// owns an ordered plan and a five-action title row, and until it was handed
/// this it was the one setup route that never stated where preflight stood.
/// Handing it the finished unit is what keeps the two headings saying the same
/// thing in the same words — the text, the tone, the announcement and the
/// rerun all stay here.
pub(super) struct PreflightChip {
    currency: PreflightCurrency,
    text: String,
}

impl PreflightChip {
    /// The chip this session owes the reader, or `None` when it owes none.
    ///
    /// A report that authorizes dispatch owes nothing: there is no problem to
    /// state, and the route that needs the task count has it. The chip is for
    /// the two standings that leave the reader something to do.
    pub(super) fn resolve(app: &RSpiceApp) -> Option<Self> {
        let currency = preflight_currency(app);
        currency.wants_rerun().then(|| Self {
            currency,
            text: chip_text(currency),
        })
    }

    /// What painting it will take, including the gap it keeps from its
    /// neighbour.
    pub(super) fn reserve(&self, ui: &Ui) -> f32 {
        chip_width(ui, &self.text) + mark_advance(self.currency) + CHIP_GAP
    }

    /// Paint it. Returns whether the reader asked for a rerun.
    pub(super) fn show(&self, ui: &mut Ui) -> bool {
        preflight_chip(ui, self.currency, &self.text)
    }
}

/// The five rows of the frozen dispatch contract, under the ordered plan.
///
/// This is what the retained report *authorizes*: the exact design revision,
/// the stable analysis identities, the PVT points, the task count and the
/// target. It belongs with the plan because the plan is the thing it
/// authorizes — it used to be a panel inside the preflight dialog, which
/// meant the one statement of what a run will dispatch was readable only
/// while a modal was up, and never while anyone was editing the plan.
///
/// Nothing is drawn without an authorized current report. A blocked or
/// expired report has no contract to state, and fabricating one would be
/// stating a dispatch nobody is authorized to make; the chip beside the
/// heading already says which of the two it is.
pub(super) fn dispatch_contract(ui: &mut Ui, app: &RSpiceApp) {
    use crate::workbench::design_system::{property_card, property_row};

    if !matches!(
        preflight_currency(app),
        PreflightCurrency::Authorized { .. }
    ) {
        return;
    }
    let Some(prepared) = app
        .state
        .workbench
        .preflight
        .report
        .as_ref()
        .and_then(|report| report.prepared.as_ref().map(|prepared| (report, prepared)))
    else {
        return;
    };
    let (report, prepared) = prepared;
    ui.add_space(CONTRACT_GAP);
    property_card(ui, "Authorized for dispatch", |ui| {
        property_row(ui, "Revision", &report.project_revision.to_string());
        property_row(
            ui,
            "Analysis identities",
            &format!(
                "{} stable instance{}",
                prepared.analysis_ids.len(),
                if prepared.analysis_ids.len() == 1 {
                    ""
                } else {
                    "s"
                }
            ),
        );
        property_row(ui, "PVT points", &prepared.pvt_point_count.to_string());
        property_row(ui, "Tasks", &prepared.task_count.to_string());
        property_row(ui, "Target", prepared.target);
    });
}

/// The gap between the last plan row and the contract under it.
const CONTRACT_GAP: f32 = 12.0;

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
        app.state.workspace.content.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        app.state.active_plan_revision(),
    )
}

fn chip_text(currency: PreflightCurrency) -> String {
    match currency.detail() {
        Some(detail) => format!("Preflight {} · {detail}", currency.status()),
        None => format!("Preflight {}", currency.status()),
    }
}

/// Side of the chip's problem mark, and the advance from its left edge to the
/// words after it: the kit's own row metrics.
const MARK_SIDE: f32 = 11.0;
const MARK_ADVANCE: f32 = 17.0;

/// How much room a standing's mark takes before its words. Only a blocked
/// report has one: an expired report is not a finding, it is a report that
/// has to be run again.
const fn mark_advance(currency: PreflightCurrency) -> f32 {
    match currency {
        PreflightCurrency::Blocked { .. } => MARK_ADVANCE,
        _ => 0.0,
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
    if !currency.wants_rerun() {
        return false;
    }
    let tone = t.color.text_dim;
    let actionable = true;
    let advance = mark_advance(currency);
    let galley = ui.fonts_mut(|fonts| fonts.layout_no_wrap(text.to_owned(), chip_font(), tone));
    let size = egui::vec2(
        galley.size().x + advance + CHIP_PADDING_X * 2.0,
        CHIP_HEIGHT,
    );
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
            if response.hovered() {
                t.color.bg_hover
            } else {
                egui::Color32::TRANSPARENT
            },
            egui::Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        if advance > 0.0 {
            crate::ui::widgets::paint_problem_mark(
                ui.painter(),
                egui::Rect::from_center_size(
                    egui::pos2(
                        rect.left() + CHIP_PADDING_X + MARK_SIDE * 0.5,
                        rect.center().y,
                    ),
                    egui::Vec2::splat(MARK_SIDE),
                ),
                crate::ui::widgets::Problem::Failure,
            );
        }
        ui.painter().galley(
            egui::pos2(
                rect.left() + CHIP_PADDING_X + advance,
                rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            tone,
        );
    }
    // The chip is a painted pill: an actionable one is a tab stop, so it owes
    // the reader the same keyboard outline every other chip paints.
    theme::paint_focus_ring(ui, &response, rect);
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

type PageCommand = fn(&mut RSpiceApp);

fn primary_action(page: SimulationPage) -> Option<(&'static str, PageCommand)> {
    match page {
        SimulationPage::Variables => Some(("Add variable…", open_variable_dialog)),
        SimulationPage::Outputs => Some(("Add output…", open_output_dialog)),
        // Excitations has no primary action: a source is placed on the
        // schematic, and offering "Add source…" here would open a drawing tool
        // from a page that cannot show the drawing.
        SimulationPage::Analyses
        | SimulationPage::Excitations
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
        state: &crate::workbench::AppState,
        blockers: Vec<PreflightIssue>,
        prepared: Option<PreparedPreflightContract>,
    ) -> PreflightReport {
        let (topology_root, topology_revision, topology_closure) =
            state.configured_topology_revision();
        let plan = state.active_plan_revision();
        PreflightReport {
            project_revision: state.workspace.content.project.revision().get(),
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
        assert!(!PreflightCurrency::Absent.wants_rerun());

        app.state.workbench.preflight.report = Some(current_report(
            &app.state,
            Vec::new(),
            Some(prepared_contract(3)),
        ));
        assert_eq!(
            preflight_currency(&app),
            PreflightCurrency::Authorized { tasks: 3 }
        );
        assert!(!preflight_currency(&app).wants_rerun());

        app.state.workbench.preflight.report = Some(current_report(
            &app.state,
            vec![blocker()],
            Some(prepared_contract(3)),
        ));
        assert_eq!(
            preflight_currency(&app),
            PreflightCurrency::Blocked { blockers: 1 }
        );

        // Anything the report was frozen against moving expires it, and the
        // routes that are not Analyses are exactly where that editing happens.
        let mut stale = current_report(&app.state, Vec::new(), Some(prepared_contract(3)));
        stale.project_revision += 1;
        app.state.workbench.preflight.report = Some(stale);
        assert_eq!(preflight_currency(&app), PreflightCurrency::Expired);
        assert!(preflight_currency(&app).wants_rerun());
    }

    /// Render one route through the surface entry point and return its
    /// AccessKit node labels.
    ///
    /// Deliberately `simulate::show` rather than this module's own `show`:
    /// Analyses is routed past this module entirely and paints its heading in
    /// `super`, so a helper that called `pages::show` could only ever ask
    /// eight of the nine routes whether they state their preflight standing —
    /// and the ninth is the one that went without it.
    fn route_announcements(
        page: SimulationPage,
        seed: impl FnOnce(&mut crate::workbench::AppState),
    ) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.simulation_page = page;
        seed(&mut app.state);
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
                    .show(ctx, |ui| super::super::show(ui, &mut app));
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

    /// Every route, including Analyses.
    ///
    /// The loop used to name three routes under a title that claimed all of
    /// them, and the route it most needed to name was the one it could not
    /// reach: Analyses paints its own heading and had no chip at all. Walking
    /// `NAVIGATION` is what makes the title true and what stops the next route
    /// from being added without one.
    #[test]
    fn an_expired_report_offers_its_rerun_from_the_route_the_reader_is_on() {
        for page in SimulationPage::NAVIGATION {
            let labels = route_announcements(page, |state| {
                let mut stale = current_report(state, Vec::new(), Some(prepared_contract(2)));
                stale.project_revision += 1;
                state.workbench.preflight.report = Some(stale);
            });
            assert!(
                labels
                    .iter()
                    .any(|label| label.contains("Preflight expired")
                        && label.contains("Rerun preflight checks")),
                "{page:?} must state the expired report and offer its rerun: {labels:?}"
            );
        }
    }

    /// One route from each of the two headings that can paint the chip.
    #[test]
    fn a_route_states_nothing_when_no_preflight_has_been_run() {
        for page in [SimulationPage::Solver, SimulationPage::Analyses] {
            let labels = route_announcements(page, |_| {});
            assert!(
                !labels.iter().any(|label| label.contains("Preflight")),
                "an unrun preflight is the ordinary state and is not an announcement on \
                 {page:?}: {labels:?}"
            );
        }
    }

    /// Everything the Analyses route states, by name and by value: a label
    /// widget's text is its AccessKit value, not its name.
    fn analyses_text(seed: impl FnOnce(&mut crate::workbench::AppState)) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.simulation_page = SimulationPage::Analyses;
        seed(&mut app.state);
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
                    .show(ctx, |ui| super::super::show(ui, &mut app));
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes
            .iter()
            .flat_map(|(_, node)| [node.label(), node.value()])
            .flatten()
            .map(str::to_owned)
            .collect()
    }

    /// The dispatch contract states what an authorized, current report
    /// authorizes — under the plan it authorizes — and nothing for a report
    /// that authorizes nothing: one that is blocked, one that has expired,
    /// or none at all.
    #[test]
    fn the_dispatch_contract_is_stated_only_for_an_authorized_current_report() {
        const TARGET: &str = "Desktop background thread";

        let authorized = analyses_text(|state| {
            state.workbench.preflight.report = Some(current_report(
                state,
                Vec::new(),
                Some(prepared_contract(3)),
            ));
        });
        for expected in ["Authorized for dispatch", "1 stable instance", TARGET] {
            assert!(
                authorized.iter().any(|text| text.contains(expected)),
                "an authorized report states {expected:?}: {authorized:?}"
            );
        }

        let blocked = analyses_text(|state| {
            state.workbench.preflight.report = Some(current_report(
                state,
                vec![blocker()],
                Some(prepared_contract(3)),
            ));
        });
        let expired = analyses_text(|state| {
            let mut stale = current_report(state, Vec::new(), Some(prepared_contract(3)));
            stale.project_revision += 1;
            state.workbench.preflight.report = Some(stale);
        });
        let absent = analyses_text(|_| {});
        for (standing, text) in [
            ("blocked", blocked),
            ("expired", expired),
            ("absent", absent),
        ] {
            assert!(
                !text
                    .iter()
                    .any(|text| text.contains("Authorized for dispatch") || text.contains(TARGET)),
                "a {standing} report authorizes no dispatch and states none: {text:?}"
            );
        }
    }

    /// A report that authorizes dispatch is a report with nothing to say.
    ///
    /// It painted a green "PREFLIGHT CURRENT · 2 tasks ready" chip on every
    /// setup route, which is success reported as evidence. Success is the
    /// absence of a problem, so the chip is gone and only the two standings
    /// that owe the reader a rerun paint one.
    #[test]
    fn an_authorized_report_paints_no_chip() {
        for page in [SimulationPage::Solver, SimulationPage::Analyses] {
            let labels = route_announcements(page, |state| {
                state.workbench.preflight.report = Some(current_report(
                    state,
                    Vec::new(),
                    Some(prepared_contract(2)),
                ));
            });
            assert!(
                !labels.iter().any(|label| label.contains("Preflight")),
                "an authorized report states nothing on {page:?}: {labels:?}"
            );
        }
    }
}
