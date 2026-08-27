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
use super::page_kit::{ReceiptRow, Tone, receipts_card, setup_page};

/// Horizontal padding inside the preflight chip, and the gap it keeps from
/// the page's own primary action.
const CHIP_PADDING_X: f32 = 9.0;
const CHIP_GAP: f32 = 8.0;
const CHIP_HEIGHT: f32 = 22.0;

/// Render the selected setup page. The analyses route is handled by the
/// surface itself and never reaches here.
pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp, page: SimulationPage) {
    // The Excitations page and its own heading state the same two lists, so
    // each is resolved once here and lent to both. Each resolved it separately,
    // and resolving one walks the design's nets — so a frame showing that page
    // paid for the walk twice to print one count above the table it counts.
    let (excitations, rf_ports) = if page == SimulationPage::Excitations {
        design_excitations(app)
    } else {
        (Vec::new(), Vec::new())
    };
    workspace_title_row(ui, |ui| {
        page_heading(ui, app, page, &excitations, &rf_ports)
    });
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
fn design_excitations(
    app: &RSpiceApp,
) -> (
    Vec<crate::simulation::placed_sources::PlacedSource>,
    Vec<crate::simulation::placed_sources::PlacedRfPort>,
) {
    let plan = app.state.sim_setup.analysis_plan.as_ref();
    match app.state.workspace.design_projection(
        &app.state.library_manager,
        &app.state.workspace.active_view,
        &app.state.schematic,
    ) {
        Ok(projection) => (
            crate::simulation::placed_sources::design_sources(
                &app.state.library_manager,
                &projection,
                plan,
            ),
            crate::simulation::placed_sources::design_rf_ports(
                &app.state.library_manager,
                &projection,
                plan,
            ),
        ),
        Err(_) => (
            crate::simulation::placed_sources::placed_sources(&app.state.schematic, plan),
            crate::simulation::placed_sources::placed_rf_ports(&app.state.schematic, plan),
        ),
    }
}

/// The plan's own configuration receipts, as the registry pages show them.
///
/// Every registry route commits into one log, because every one of them edits
/// the same plan: a variable, an output, a model binding and a save policy all
/// advance one revision and take one sequence number. Showing that whole log on
/// each of them is therefore not repetition — it is the page saying what state
/// the plan is in, including the edit somebody made on another route a moment
/// ago and the one this page just made.
///
/// Until this card existed, a successful edit was the one outcome with no
/// surface at all. A refusal toasted. A receipt advanced a counter that a
/// single-slot status line then overwrote, and the Analyses route was the only
/// place that even read it.
pub(super) fn plan_configuration_receipts(ui: &mut Ui, app: &RSpiceApp) {
    let rows = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| {
            plan.configuration_receipts()
                .iter()
                .map(|receipt| ReceiptRow {
                    sequence: receipt.sequence().to_string(),
                    action: receipt.detail().to_owned(),
                    tone: Tone::Ok,
                    revision: format!(
                        "{} → {}",
                        receipt.source_revision().get(),
                        receipt.committed_revision().get()
                    ),
                    // A receipt written before digests existed reports absence
                    // rather than a digest that would not reproduce.
                    digest: receipt.digest().unwrap_or("not retained").to_owned(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    receipts_card(
        ui,
        "Plan configuration receipts",
        "no committed change",
        (
            "This plan has adopted no configuration change yet. Adding a variable, saving an \
             output, attaching a model library or changing the save policy each commits one, and \
             each is recorded here with the digest of the plan it adopted.",
            "Every row is a change that committed. A refused edit never reaches this log — it \
             leaves the plan byte-for-byte as it was and is reported where it was attempted.",
        ),
        &rows,
    );
}

/// Eyebrow, title, description, and the page's own primary action.
///
/// The eyebrow states the page's authority in the plan, derived from the same
/// state the page edits — never a static caption.
fn page_heading(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    page: SimulationPage,
    excitations: &[crate::simulation::placed_sources::PlacedSource],
    rf_ports: &[crate::simulation::placed_sources::PlacedRfPort],
) {
    let eyebrow = eyebrow(app, page, excitations, rf_ports);
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
    pub(super) fn resolve(app: &RSpiceApp) -> Option<Self> {
        let currency = preflight_currency(app);
        currency.is_stated().then(|| Self {
            currency,
            text: chip_text(currency),
        })
    }

    /// What painting it will take, including the gap it keeps from its
    /// neighbour.
    pub(super) fn reserve(&self, ui: &Ui) -> f32 {
        chip_width(ui, &self.text) + CHIP_GAP
    }

    /// Paint it. Returns whether the reader asked for a rerun.
    pub(super) fn show(&self, ui: &mut Ui) -> bool {
        preflight_chip(ui, self.currency, &self.text)
    }
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

/// The plan payload the three registry pages edit, borrowed rather than cloned.
///
/// Each of those pages resolves this same payload and clones it, which is right
/// for a page body that then edits it and wrong for a heading that runs every
/// frame on every route.
fn plan_payload(app: &RSpiceApp) -> Option<&crate::state::SimulationPlanPayload> {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| app.state.workspace.plan_data(plan_id))
}

/// A count and the thing it counts, pluralized: `3 VARIABLES`, `1 VARIABLE`.
fn counted(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {}", singular.to_uppercase())
    } else {
        format!("{count} {}", plural.to_uppercase())
    }
}

/// What this page has authority over, and what it currently holds.
///
/// Every arm reads the state its page edits. Six of these were static captions
/// naming the *topic* of the page -- "SAVED SIGNALS · EXPRESSIONS" read the
/// same above an empty registry and a full one, which is the one moment a
/// heading could have been useful. A caption that cannot change is decoration
/// occupying the position a reader has learned carries state.
///
/// Cheap, with one exception that is named rather than glossed. Every arm but
/// the first is a `len`, a `count` over a small vector, or a `Copy` scalar. The
/// expensive answers each page can give -- the resolved run space, the
/// saved-output preflight, the model qualification gate -- are page-body work
/// and stay there.
///
/// The exception is Excitations, whose eyebrow counts the placed-source and
/// placed-port lists over the whole design. Resolving either walks the nets of
/// every master that places one, so each is resolved once by [`show`] and lent
/// to the heading and the page body together rather than resolved by each. The
/// Design navigator's rail asks for the same two lists on the same frame and
/// costs nothing for it: the walk is retained against the design projection it
/// was derived from, so the second caller of a frame pays a pointer comparison.
/// The walk itself skips every master that places none of what it lists, so a
/// sheet being drawn pays nothing at all.
fn eyebrow(
    app: &RSpiceApp,
    page: SimulationPage,
    excitations: &[crate::simulation::placed_sources::PlacedSource],
    rf_ports: &[crate::simulation::placed_sources::PlacedRfPort],
) -> String {
    match page {
        // `unread` counts sources, as it always has. A port's readership is not
        // the same fact — a port no `.sp` run indexes is still terminating the
        // design — and the card's own chip is where that is stated.
        SimulationPage::Excitations => {
            let unread = excitations
                .iter()
                .filter(|source| !source.is_read())
                .count();
            let sources = counted(excitations.len(), "source", "sources");
            if rf_ports.is_empty() {
                format!("DESIGN · {sources} · {unread} unread")
            } else {
                format!(
                    "DESIGN · {sources} · {} · {unread} unread",
                    counted(rf_ports.len(), "RF port", "RF ports")
                )
            }
        }
        SimulationPage::Analyses => {
            let plan = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .map(|plan| plan.instances().len())
                .unwrap_or_default();
            format!("PLAN · {}", counted(plan, "analysis", "analyses"))
        }
        SimulationPage::Variables => {
            let variables =
                plan_payload(app).map_or(&[][..], |payload| payload.design_variables.as_slice());
            let sweepable = variables
                .iter()
                .filter(|variable| {
                    !matches!(
                        variable.sweep_eligibility,
                        crate::state::DesignVariableSweepEligibility::FixedParameter
                    )
                })
                .count();
            format!(
                "PARAMETERIZATION · {} · {sweepable} SWEEPABLE",
                counted(variables.len(), "variable", "variables")
            )
        }
        SimulationPage::Outputs => {
            let outputs =
                plan_payload(app).map_or(&[][..], |payload| payload.saved_outputs.as_slice());
            let derived = outputs
                .iter()
                .filter(|output| {
                    matches!(
                        output.kind,
                        crate::state::SavedOutputKind::DerivedExpression
                    )
                })
                .count();
            format!(
                "SAVED SIGNALS · {} · {derived} DERIVED",
                counted(outputs.len(), "signal", "signals")
            )
        }
        SimulationPage::Specifications => {
            let payload = plan_payload(app);
            let limits = payload.map_or(0, |payload| payload.specs.len());
            // A legacy plan can hold projected entries with no governed records
            // behind them, because the migration that mints those needs a
            // mutable workspace this heading does not have. Reporting "0
            // governed" there would name a real zero; "ungoverned" names the
            // state the page will actually show.
            let governed = payload.map_or(0, |payload| payload.specification_definitions.len());
            format!(
                "REQUIREMENTS · {} · {}",
                counted(limits, "limit", "limits"),
                if limits > 0 && governed == 0 {
                    "UNGOVERNED".to_owned()
                } else {
                    format!("{governed} GOVERNED")
                }
            )
        }
        SimulationPage::RunSet => {
            let axes = app.state.sim_setup.run_set.enabled_dimensions().count();
            let variation = if app
                .state
                .sim_setup
                .has_enabled_analysis_kind(crate::simulation::plan::AnalysisKind::MonteCarlo)
            {
                "MONTE CARLO"
            } else {
                "DETERMINISTIC"
            };
            format!(
                "RUN SPACE · {} · {variation}",
                counted(axes, "axis", "axes")
            )
        }
        SimulationPage::Models => {
            let bindings = &app.state.sim_setup.model_bindings;
            let pinned = bindings
                .iter()
                .filter(|binding| binding.selected_corner.is_some())
                .count();
            format!(
                "MODEL CLOSURE · {} · {pinned} SECTION-PINNED",
                counted(bindings.len(), "library", "libraries")
            )
        }
        // One term, not two: "DESIGN CONTRACT" was a fixed caption naming the
        // topic, which is the shape the rule above rejects. The preset is the
        // state this page holds.
        SimulationPage::Solver => format!(
            "NUMERICS · {}",
            app.state.sim_setup.options.preset_label().to_uppercase()
        ),
        SimulationPage::Save => {
            let policy = app.state.sim_setup.save_policy;
            format!(
                // `retained_dataset_limit` is the ceiling this plan keeps to,
                // not a count of what it has kept: "20 RETAINED" read as twenty
                // datasets on disk above a project holding none.
                "RESULT STORAGE · {} · KEEP {} · {}",
                policy.output_selection_mode.label().to_uppercase(),
                policy.retained_dataset_limit,
                if policy.live_streaming_enabled {
                    "STREAMING"
                } else {
                    "ON COMPLETION"
                }
            )
        }
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
            project_revision: state.workspace.project.revision().get(),
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
                    .any(|label| label.contains("PREFLIGHT EXPIRED")
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
                !labels.iter().any(|label| label.contains("PREFLIGHT")),
                "an unrun preflight is the ordinary state and is not an announcement on \
                 {page:?}: {labels:?}"
            );
        }
    }
}
