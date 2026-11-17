//! The Overview's identity band: the project's name, its status summary, the
//! working-state facts and the quick actions, above a full-width rule.

use egui::{Align, Align2, Color32, Layout, Sense, Ui, pos2, vec2};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, PROBLEM_MARK_ADVANCE as MARK_ADVANCE};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::state::Workspace;

use super::super::page::{self, HEADER_TOP, elide_text, paint_elided};
use super::snapshot::{OverviewSnapshot, StatusSnapshot};
use super::{OverviewIntent, Tone, command_gate, paint_tone_mark, singular_or_plural};

/// Summarise the status register beside the project name.
///
/// The count is of *status areas* that need attention, not of findings. Naming
/// it "advisories" put a finding word on an area count directly above a register
/// whose first row reports genuine advisory counts, so two unrelated numbers
/// read as the same one. The denominator keeps the area reading unambiguous.
/// Nothing at all when every area is current: the register below states each
/// area, and "current · 6 areas" beside the project's name was a third copy of
/// a fact the Project status card and its six rows already carry.
pub(super) fn project_status_summary(statuses: &[StatusSnapshot], tone: Tone) -> String {
    let total = statuses.len();
    let flagged = statuses
        .iter()
        .filter(|status| matches!(status.tone, Tone::Warn | Tone::Error))
        .count();
    match tone {
        Tone::Error => format!("blocked · {flagged} of {total} areas"),
        Tone::Warn => format!("review · {flagged} of {total} areas"),
        Tone::Neutral => String::new(),
    }
}

/// The identity band: project name, status summary, working-state facts, and
/// the quick actions. It sits on the page ground above a full-width rule, the
/// same header rhythm as the workbench landing surfaces.
pub(super) fn overview_header(
    ui: &mut Ui,
    cx: &Cx<'_, ProjectFeature>,
    snapshot: &OverviewSnapshot,
    inset: f32,
    content_width: f32,
) -> Option<OverviewIntent> {
    let tokens = Tokens::get(ui.ctx());
    let mut intent = None;
    ui.add_space(HEADER_TOP);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(inset);
        ui.allocate_ui_with_layout(
            vec2(content_width, 1.0),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_width(content_width);
                ui.spacing_mut().item_spacing.y = 0.0;
                let project_tone = if snapshot
                    .statuses
                    .iter()
                    .any(|status| status.tone == Tone::Error)
                {
                    Tone::Error
                } else if snapshot
                    .statuses
                    .iter()
                    .any(|status| status.tone == Tone::Warn)
                {
                    Tone::Warn
                } else {
                    Tone::Neutral
                };
                let project_status = project_status_summary(&snapshot.statuses, project_tone);
                let status_font = theme::mono(tokens::FS_0, FontWeight::Medium);
                let status_width = (ui
                    .painter()
                    .layout_no_wrap(project_status.clone(), status_font.clone(), Color32::WHITE)
                    .size()
                    .x
                    + if project_tone.problem().is_some() {
                        MARK_ADVANCE
                    } else {
                        0.0
                    })
                .min(content_width * 0.42);
                let title_font = theme::sans(20.0, FontWeight::SemiBold);
                let (name_rect, name_response) =
                    ui.allocate_exact_size(vec2(content_width, 27.0), Sense::hover());
                let title = elide_text(
                    ui,
                    &snapshot.project_name,
                    &title_font,
                    (content_width - status_width - 12.0).max(1.0),
                );
                let title_width = ui
                    .painter()
                    .layout_no_wrap(title.clone(), title_font.clone(), Color32::WHITE)
                    .size()
                    .x;
                ui.painter().text(
                    pos2(name_rect.left(), name_rect.center().y),
                    Align2::LEFT_CENTER,
                    &title,
                    title_font,
                    tokens.color.text,
                );
                if !project_status.is_empty() {
                    let status_left = (name_rect.left() + title_width + 12.0)
                        .min((name_rect.right() - status_width).max(name_rect.left()));
                    let advance =
                        paint_tone_mark(ui, project_tone, status_left, name_rect.center().y + 2.0);
                    ui.painter().text(
                        pos2(status_left + advance, name_rect.center().y + 2.0),
                        Align2::LEFT_CENTER,
                        &project_status,
                        status_font,
                        tokens.color.text_dim,
                    );
                }
                let announced_status = if project_status.is_empty() {
                    snapshot.project_name.clone()
                } else {
                    format!("{} · {project_status}", snapshot.project_name)
                };
                name_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        format!("Project {announced_status}"),
                    )
                });
                name_response.on_hover_text(format!(
                    "Project {} · logical revision {} · schema {}",
                    snapshot.project_id, snapshot.project_revision, snapshot.schema_version
                ));

                let state_label = if !snapshot.persistence_bound {
                    "no saved baseline".to_owned()
                } else if snapshot.dirty_documents == 0 {
                    "working tree clean".to_owned()
                } else {
                    format!(
                        "{} working {}",
                        snapshot.dirty_documents,
                        singular_or_plural(snapshot.dirty_documents, "change", "changes")
                    )
                };
                let (facts_rect, _) =
                    ui.allocate_exact_size(vec2(content_width, 19.0), Sense::hover());
                paint_elided(
                    ui,
                    pos2(facts_rect.left(), facts_rect.top() + 4.0),
                    &format!(
                        "main @ r{} · {} · {}",
                        snapshot.project_revision, state_label, snapshot.path
                    ),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    tokens.color.text_faint,
                    content_width,
                );

                ui.add_space(10.0);
                // Every action is the Button widget's own disabled state, not
                // an `add_enabled_ui` scope: a scope is placed before its size
                // is known, so it can never wrap onto the next action row.
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
                    let primary = if snapshot.netlist_first {
                        Button::new("Open SPICE deck").icon(Icon::File)
                    } else {
                        Button::new("Open top schematic").icon(Icon::Schematic)
                    };
                    if primary.accent().show(ui).clicked() {
                        intent = Some(if snapshot.netlist_first {
                            OverviewIntent::Command(ids::open_workspace(Workspace::Netlist))
                        } else {
                            OverviewIntent::OpenProjectRoot
                        });
                    }
                    if !snapshot.netlist_first {
                        let run_label = if content_width <= 440.0 {
                            "Run plan\u{2026}".to_owned()
                        } else {
                            format!("Run {}…", snapshot.run_plan_name)
                        };
                        let command = ids::open_workspace(Workspace::Simulate);
                        let (enabled, disabled_reason) = command_gate(cx, command);
                        let run_plan = Button::new(&run_label).enabled(enabled).show(ui);
                        let run_plan = disabled_reason.map_or(run_plan.clone(), |reason| {
                            run_plan.on_disabled_hover_text(reason)
                        });
                        if run_plan.clicked() {
                            intent = Some(run_plan_intent());
                        }
                    }
                    let (new_cell_command_enabled, new_cell_command_reason) =
                        command_gate(cx, ProjectCommand::NewCell.key());
                    // Whether any library is writable needs no order: sorting
                    // the catalogue to ask it cost a sort on every frame.
                    let has_writable_library = cx
                        .model
                        .library_manager
                        .libraries_by_key()
                        .any(|(_, library)| !library.read_only);
                    let new_cell_allowed = new_cell_command_enabled
                        && !cx.model.safe_mode.project_read_only()
                        && has_writable_library;
                    let new_cell = Button::new("New cell…")
                        .enabled(new_cell_allowed)
                        .show(ui)
                        .on_disabled_hover_text(if let Some(reason) = new_cell_command_reason {
                            reason
                        } else if cx.model.safe_mode.project_read_only() {
                            "Safe mode opened this project read-only."
                        } else if !has_writable_library {
                            "Attach or create a writable design library first."
                        } else {
                            "New cell creation is unavailable."
                        });
                    if new_cell.clicked() {
                        intent = Some(OverviewIntent::Command(ProjectCommand::NewCell.key()));
                    }
                    let (open_deck_enabled, open_deck_disabled_reason) =
                        command_gate(cx, ids::OPEN_NETLIST);
                    let import_deck = Button::new("Import deck…")
                        .enabled(open_deck_enabled)
                        .show(ui);
                    let import_deck = open_deck_disabled_reason
                        .map_or(import_deck.clone(), |reason| {
                            import_deck.on_disabled_hover_text(reason)
                        });
                    if import_deck.clicked() {
                        intent = Some(import_deck_intent());
                    }
                    let (history_enabled, history_disabled_reason) =
                        command_gate(cx, ProjectCommand::RevisionHistory.key());
                    let history = Button::new("History…").enabled(history_enabled).show(ui);
                    let history = history_disabled_reason.map_or(history.clone(), |reason| {
                        history.on_disabled_hover_text(reason)
                    });
                    if history.clicked() {
                        intent = Some(OverviewIntent::Command(
                            ProjectCommand::RevisionHistory.key(),
                        ));
                    }
                });
            },
        );
    });
    page::header_rule(ui);
    intent
}

pub fn run_plan_intent() -> OverviewIntent {
    OverviewIntent::Command(ids::open_workspace(Workspace::Simulate))
}

pub(super) fn import_deck_intent() -> OverviewIntent {
    OverviewIntent::Command(ids::OPEN_NETLIST)
}
