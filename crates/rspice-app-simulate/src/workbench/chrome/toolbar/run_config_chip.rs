//! The toolbar's run chip: the analysis Run runs, edited in place, and the
//! product's global entry to the plan manager.
//!
//! The chip names what a press of Run will do: the analysis and its main
//! number, over the plan it belongs to. A press opens a popover that changes
//! the analysis's kind and its main numbers where the reader is, as ADE's
//! analysis row and ADS's simulation controller do, without a trip to the
//! Simulate workspace; the plan manager is one row further down.
//!
//! The chip is measured, painted, and announced from one derivation of its
//! copy, because its width, its two clipped lines, and its accessibility label
//! all describe the same plan. Naming that plan from a literal was how the chip
//! kept announcing a fixture plan after a rename, while the live summary beside
//! it made the stale name read as authoritative.

use egui::Vec2;
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::analysis_kind::AnalysisKind;
use rspice_simulation_contract::plan_model::AnalysisInstance;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::Model;
use crate::workbench::design_system::{WorkbenchIcon, elide_middle, elide_text};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::layout::RUN_CONFIG_CHIP_MAX_WIDTH;

/// What the reader asked of the run chip this frame.
#[derive(Debug, Clone)]
pub(in crate::workbench) enum RunChipRequest {
    /// Open the plan manager.
    ManagePlans,
    /// Run a fresh analysis of this kind in place of the one the chip names.
    Kind(AnalysisKind),
    /// The analysis the chip names, with the numbers the reader typed.
    Draft(Box<AnalysisDraft>),
    /// A number the reader typed that is not one: what the status line says.
    Refused(String),
}

/// The kinds the chip switches between, with the words it offers them in.
const KINDS: [(AnalysisKind, &str); 3] = [
    (AnalysisKind::OperatingPoint, "Operating point"),
    (AnalysisKind::Transient, "Transient"),
    (AnalysisKind::Ac, "AC"),
];

/// How wide the popover stands: the three kinds side by side.
const POPOVER_WIDTH: f32 = 264.0;

/// Every string the run chip measures, paints, and announces.
struct RunConfigChipCopy {
    title: String,
    summary: String,
    accessibility: String,
}

fn run_config_chip_copy(model: &Model) -> RunConfigChipCopy {
    let plan = model.sim_setup.active_plan_name().as_str().to_owned();
    let title = analysis_title(model);
    let pvt_count = configured_pvt_count(model);
    RunConfigChipCopy {
        accessibility: format!(
            "Analysis {title}, plan {plan}; change the analysis or manage simulation plans"
        ),
        summary: format!("{plan} \u{00b7} {pvt_count} PVT"),
        title,
    }
}

/// The analyses Run is for: the plan's enabled analyses that no other
/// enabled analysis runs as its prerequisite, in plan order. An AC sweep's
/// operating point runs because the sweep needs it, so the sweep is what the
/// chip names.
fn leading_analyses(model: &Model) -> Vec<&AnalysisInstance> {
    let enabled: Vec<&AnalysisInstance> = model.sim_setup.enabled_analysis_instances().collect();
    let prerequisite = |candidate: &AnalysisInstance| {
        enabled.iter().any(|other| {
            other
                .dependencies()
                .iter()
                .any(|dependency| dependency.target() == candidate.id())
        })
    };
    let leading: Vec<&AnalysisInstance> = enabled
        .iter()
        .copied()
        .filter(|candidate| !prerequisite(candidate))
        .collect();
    if leading.is_empty() { enabled } else { leading }
}

/// The analysis the chip names and edits: the first of [`leading_analyses`].
pub(in crate::workbench) fn lead_analysis(model: &Model) -> Option<&AnalysisInstance> {
    leading_analyses(model).into_iter().next()
}

/// The analysis Run runs, as the chip's first line names it: the lead
/// analysis and its main number, and how many more run beside it.
fn analysis_title(model: &Model) -> String {
    let leading = leading_analyses(model);
    let Some(lead) = leading.first() else {
        return "No analysis".to_owned();
    };
    let name = analysis_name(lead.draft());
    match leading.len() - 1 {
        0 => name,
        more => format!("{name} +{more}"),
    }
}

/// `Transient · 1 ms`, `AC · 1 Hz to 1 GHz`, `Operating point`: a kind the
/// chip edits by its main numbers, any other kind by its name.
fn analysis_name(draft: &AnalysisDraft) -> String {
    match draft {
        AnalysisDraft::OperatingPoint(_) => "Operating point".to_owned(),
        AnalysisDraft::Transient(setup) => {
            format!("Transient \u{00b7} {}", spelled(&setup.stop, "s"))
        }
        AnalysisDraft::Ac(setup) => format!(
            "AC \u{00b7} {} to {}",
            spelled(&setup.fstart, "Hz"),
            spelled(&setup.fstop, "Hz")
        ),
        other => other.kind().label().to_owned(),
    }
}

/// `text` as a reader writes the quantity, `1 ms`, or as typed when it is not
/// a number, such as a design variable.
fn spelled(text: &str, unit: &str) -> String {
    rspice_app_state::quantity::parse_engineering_value(text).map_or_else(
        |_| text.trim().to_owned(),
        |value| rspice_results::verification::format_bound(value, unit),
    )
}

/// Chip box for measured copy: the icon gutter, the wider of the two lines,
/// and the chevron gutter, capped at what the toolbar reserves for it.
fn run_config_chip_width(title_width: f32, summary_width: f32) -> f32 {
    (31.0 + title_width.max(summary_width) + 22.0).min(RUN_CONFIG_CHIP_MAX_WIDTH)
}

/// Draw the run chip over `model`, enabled as `manage`, the dispatcher's
/// answer for Manage plans, says. A press opens the popover that edits the
/// analysis; returns what the reader asked of it.
pub(in crate::workbench) fn run_config_selector(
    ui: &mut egui::Ui,
    model: &Model,
    manage: CommandAvailability,
    height: f32,
) -> Option<RunChipRequest> {
    let t = Tokens::get(ui.ctx());
    let copy = run_config_chip_copy(model);
    let availability = manage;
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
    let popover = ui.make_persistent_id("workbench.run_chip.popover");
    let open = egui::Popup::is_id_open(ui.ctx(), popover);

    let hovered = enabled && (response.hovered() || open);
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
    // At its cap the chip shortens its lines with an ellipsis rather than
    // cutting a glyph at the chevron's gutter.
    let room = rect.width() - 31.0 - 22.0;
    let title_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let summary_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    ui.painter().text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y - 6.5),
        egui::Align2::LEFT_CENTER,
        elide_text(ui, &copy.title, &title_font, room),
        title_font,
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    ui.painter().text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y + 7.0),
        egui::Align2::LEFT_CENTER,
        elide_text(ui, &copy.summary, &summary_font, room),
        summary_font,
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
    if enabled && response.clicked() {
        egui::Popup::toggle_id(ui.ctx(), popover);
    }
    let mut request = None;
    egui::Popup::from_response(&response)
        .id(popover)
        .open_memory(None)
        .align(egui::RectAlign::BOTTOM_END)
        .width(POPOVER_WIDTH)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            ui.set_min_width(POPOVER_WIDTH);
            request = analysis_popover(ui, model);
            if matches!(request, Some(RunChipRequest::ManagePlans)) {
                ui.close();
            }
        });
    if enabled {
        response.on_hover_text(&copy.accessibility);
    } else if let CommandAvailability::Disabled(reason) = availability {
        response.on_hover_text(reason);
    }
    request
}

/// The popover's rows: the analysis's kind, its main numbers, and the plan
/// manager. Returns what the reader asked of them.
fn analysis_popover(ui: &mut egui::Ui, model: &Model) -> Option<RunChipRequest> {
    let t = Tokens::get(ui.ctx());
    let lead = lead_analysis(model);
    let mut request = None;
    ui.spacing_mut().item_spacing.y = 6.0;
    let mut kind = lead
        .and_then(|lead| KINDS.iter().position(|(kind, _)| *kind == lead.kind()))
        .unwrap_or(usize::MAX);
    let labels = KINDS.map(|(_, label)| label);
    if crate::ui::widgets::segmented(
        ui,
        "workbench.run_chip.kind",
        &labels,
        &mut kind,
        crate::ui::widgets::SegmentedWidth::Equal,
    ) && let Some((chosen, _)) = KINDS.get(kind)
    {
        request = Some(RunChipRequest::Kind(*chosen));
    }
    match lead.map(|lead| lead.draft()) {
        Some(AnalysisDraft::Transient(setup)) => {
            let mut edited = setup.clone();
            let stop = number_row(ui, "Stop time", "s", &setup.stop, Number::Positive);
            let step = number_row(ui, "Step", "s", &setup.step, Number::Positive);
            request = request.or_else(|| {
                commit(stop, &mut edited.stop)
                    .or_else(|| commit(step, &mut edited.step))
                    .map(|refused| {
                        refused.unwrap_or(RunChipRequest::Draft(Box::new(
                            AnalysisDraft::Transient(edited),
                        )))
                    })
            });
        }
        Some(AnalysisDraft::Ac(setup)) => {
            let mut edited = setup.clone();
            let start = number_row(ui, "Start", "Hz", &setup.fstart, Number::Positive);
            let stop = number_row(ui, "Stop", "Hz", &setup.fstop, Number::Positive);
            let per = match setup.sweep {
                0 => "Points/decade",
                1 => "Points/octave",
                _ => "Points",
            };
            let points = number_row(ui, per, "", &setup.points, Number::Count);
            request = request.or_else(|| {
                commit(start, &mut edited.fstart)
                    .or_else(|| commit(stop, &mut edited.fstop))
                    .or_else(|| commit(points, &mut edited.points))
                    .map(|refused| {
                        refused
                            .unwrap_or(RunChipRequest::Draft(Box::new(AnalysisDraft::Ac(edited))))
                    })
            });
        }
        Some(AnalysisDraft::OperatingPoint(_)) => {
            popover_note(ui, "The DC solution at the reference PVT.");
        }
        Some(other) => {
            popover_note(
                ui,
                &format!("{} is edited on the Analyses page.", other.kind().label()),
            );
        }
        None => popover_note(ui, "The plan runs no analysis."),
    }
    ui.add_space(2.0);
    let (rule, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().rect_filled(rule, 0.0, t.color.border);
    if crate::ui::widgets::Button::new("Manage plans\u{2026}")
        .show(ui)
        .clicked()
    {
        request = Some(RunChipRequest::ManagePlans);
    }
    request
}

/// A faint line under the kind, where a kind has no numbers to edit here.
fn popover_note(ui: &mut egui::Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

/// What a number row takes.
#[derive(Clone, Copy)]
enum Number {
    /// A quantity above zero, in engineering notation.
    Positive,
    /// A whole count of one or more.
    Count,
}

/// What a number row's field gave up this frame.
enum Typed {
    /// Nothing: the field still has focus, or what it holds is what it showed.
    Nothing,
    /// The reader left the field holding this, a number it takes.
    Number(String),
    /// The reader left the field holding something it does not take.
    Refused(String),
}

/// One labelled field over `current`, committed when the reader leaves it.
///
/// While the field has focus it keeps what is typed; once it loses focus it
/// shows the plan again, so a refused entry never stands in place of the
/// value the plan runs.
fn number_row(ui: &mut egui::Ui, label: &str, unit: &str, current: &str, takes: Number) -> Typed {
    let id = ui.make_persistent_id(("workbench.run_chip.field", label));
    let mut text = ui
        .data(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| current.to_owned());
    let response = crate::ui::widgets::input_row(ui, &with_unit(label, unit), &mut text);
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
        return Typed::Nothing;
    }
    ui.data_mut(|data| data.remove::<String>(id));
    if !response.lost_focus() || text.trim() == current.trim() {
        return Typed::Nothing;
    }
    let text = text.trim().to_owned();
    let (taken, wanted) = match takes {
        Number::Positive => (
            rspice_app_state::quantity::parse_engineering_value(&text)
                .is_ok_and(|value| value.is_finite() && value > 0.0),
            if unit == "Hz" {
                "a frequency above zero, such as 1k"
            } else {
                "a time above zero, such as 1m"
            },
        ),
        Number::Count => (
            text.parse::<u32>().is_ok_and(|count| count > 0),
            "a whole number of one or more",
        ),
    };
    if taken {
        Typed::Number(text)
    } else {
        Typed::Refused(format!("{label} takes {wanted}, not \"{text}\"."))
    }
}

/// `Stop time, s`, as the inspector's rows carry their unit.
fn with_unit(label: &str, unit: &str) -> String {
    if unit.is_empty() {
        label.to_owned()
    } else {
        format!("{label}, {unit}")
    }
}

/// Write what `typed` holds into `field`: `None` when it holds nothing,
/// `Some(None)` when the field took it, `Some(Some(refusal))` when it did not.
fn commit(typed: Typed, field: &mut String) -> Option<Option<RunChipRequest>> {
    match typed {
        Typed::Nothing => None,
        Typed::Number(text) => {
            *field = text;
            Some(None)
        }
        Typed::Refused(words) => Some(Some(RunChipRequest::Refused(words))),
    }
}

/// Draw the chip that stands in for the plan chip where Run runs a deck:
/// the deck's file name over "Deck", enabled as `show`, the dispatcher's
/// answer for Show deck, says. Returns whether the reader pressed it.
pub(in crate::workbench) fn deck_chip(
    ui: &mut egui::Ui,
    deck: &str,
    show: CommandAvailability,
    height: f32,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let enabled = show == CommandAvailability::Available && ui.is_enabled();
    let accessibility = format!("Deck {deck}; show the deck Run runs");
    let summary = "Deck";
    let measure = |text: &str, font: egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font, t.color.text)
            .size()
            .x
    };
    let width = run_config_chip_width(
        measure(deck, theme::sans(tokens::FS_0, FontWeight::Medium)),
        measure(summary, theme::mono(tokens::FS_0, FontWeight::Regular)),
    );
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width, height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, &accessibility)
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
    WorkbenchIcon::Netlist.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 16.0, rect.center().y),
            Vec2::splat(16.0),
        ),
        t.color.text_dim,
    );
    // A long file name keeps its start and its extension, shortened in the
    // middle, and leaves the right inset the chip's width allowed for; hover
    // names it whole.
    let name_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    ui.painter().text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y - 6.5),
        egui::Align2::LEFT_CENTER,
        elide_middle(ui, deck, &name_font, rect.width() - 31.0 - 22.0),
        name_font,
        t.color.text,
    );
    ui.painter().text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y + 7.0),
        egui::Align2::LEFT_CENTER,
        summary,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    let pressed = response.clicked() && enabled;
    response.on_hover_text(&accessibility);
    pressed
}

fn configured_pvt_count(model: &Model) -> usize {
    // The plan-wide Run Set expands every enabled analysis. Legacy Corner and
    // Temperature analysis drafts no longer own the workspace's PVT count and
    // must not be added a second time in the chrome summary.
    model.sim_setup.run_set.point_count().max(1)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::state::pdk_config::PdkConfig;

    use rspice_app_state::workbench::workspace::restore_model_library_manager;

    /// The chip is the product's global entry to the plan manager, so the plan
    /// it names has to be the plan that is active — not the name the fixture
    /// project happened to ship with.
    #[test]
    fn plan_chip_copy_follows_a_rename_of_the_active_plan() {
        let mut model =
            Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
        let plan_id = model
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .id();

        model
            .sim_setup
            .rename_plan(plan_id, "Automotive temperature sweep")
            .expect("rename the active plan");
        let copy = run_config_chip_copy(&model);

        assert_eq!(copy.summary, "Automotive temperature sweep \u{00b7} 1 PVT");
        assert_eq!(
            copy.accessibility,
            "Analysis Transient \u{00b7} 1 ms, plan Automotive temperature sweep; \
             change the analysis or manage simulation plans"
        );
        assert!(!copy.summary.contains("Default plan"));
        assert!(!copy.accessibility.contains("Default plan"));
    }

    /// The chip names what Run runs: the first enabled analysis by its kind
    /// and main numbers, and how many more run after it.
    #[test]
    fn the_chip_names_the_analysis_run_runs() {
        use rspice_simulation_contract::analysis_kind::AnalysisKind;
        let mut model =
            Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
        assert_eq!(
            run_config_chip_copy(&model).title,
            "Transient \u{00b7} 1 ms"
        );

        let plan = model.sim_setup.stable_analysis_plan_mut().unwrap();
        let transient = plan.instances()[0].id();
        plan.edit(transient, |draft| {
            if let AnalysisDraft::Transient(setup) = draft {
                "20u".clone_into(&mut setup.stop);
            }
        })
        .unwrap();
        assert_eq!(
            run_config_chip_copy(&model).title,
            "Transient \u{00b7} 20 \u{b5}s"
        );

        // The sweep's own operating point is part of the sweep; another
        // analysis beside it is counted.
        let plan = model.sim_setup.stable_analysis_plan_mut().unwrap();
        let (ac, _) = plan.insert_at(AnalysisKind::Ac, 0).unwrap();
        plan.repair_dependencies(ac).unwrap();
        assert_eq!(
            run_config_chip_copy(&model).title,
            "AC \u{00b7} 1 Hz to 1 GHz +1"
        );
    }

    #[test]
    fn toolbar_pvt_summary_uses_the_global_run_set_exactly_once() {
        use crate::product::ProcessCorner;
        use rspice_simulation_contract::corner_config::{CornerBaseAnalysis, CornerConfig};

        let mut model =
            Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
        model.sim_setup.run_set =
            crate::simulation::controller::test_execution::pvt::from_corner_config(&CornerConfig {
                process_corners: vec![ProcessCorner::TT, ProcessCorner::SS],
                voltages: vec![1.0],
                supply_source_names: Vec::new(),
                temperatures: vec![-40.0, 27.0, 125.0],
                full_matrix: true,
                points: Vec::new(),
                base_analysis: CornerBaseAnalysis::Op,
            });

        assert_eq!(configured_pvt_count(&model), 6);
    }
}
