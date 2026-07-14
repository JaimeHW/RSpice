//! Project Launcher specified by the workbench mockup.
//!
//! This surface is deliberately data-backed. It shows only project entries
//! that exist in the application's persisted recent-file store; categories
//! requiring accounts, recovery manifests, or governed template packages are
//! withheld until those product services can supply real records.

use std::path::Path;

use egui::{Align, Context, Frame, Id, Margin, Order, Rect, Sense, Stroke, Ui, Vec2};

use crate::common::RSpiceApp;
use crate::common::app::{RecentFile, RecentKind};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, IconButton, select};

use super::commands::Command;
use super::design_system::WorkbenchIcon;
use super::recovery::{
    RecoveryCandidate, RecoveryIntegrity, RecoveryNoticeTone, discard_checkpoint, open_comparison,
    recovery_replacement_block_reason, refresh_catalog_if_requested, start_local_safe_mode,
};
use super::state::{LocalSafeModeOptions, ProjectLauncherPage, ProjectLauncherSort};

#[derive(Debug, Clone)]
struct ProjectEntry {
    recent: RecentFile,
    name: String,
    path_text: String,
    available: bool,
    current: bool,
}

enum LauncherAction {
    Close,
    Browse,
    NewProject,
    Open(RecentFile),
    Page(ProjectLauncherPage),
    Recover(RecoveryCandidate),
    RequestDiscard(RecoveryCandidate),
    StartSafeMode(LocalSafeModeOptions),
}

pub(super) fn show(ctx: &Context, app: &mut RSpiceApp) {
    if !app.state.workbench.project_launcher_open {
        show_discard_confirmation(ctx, app);
        return;
    }

    refresh_catalog_if_requested(app);

    let mut action = ctx
        .input(|input| input.key_pressed(egui::Key::Escape))
        .then_some(LauncherAction::Close);
    let t = Tokens::get(ctx);
    let screen = ctx.screen_rect();
    let size = Vec2::new(
        (screen.width() - 24.0).min(1040.0).max(280.0),
        (screen.height() - 24.0).min(760.0).max(280.0),
    );
    let surface_rect = Rect::from_center_size(screen.center(), size);

    egui::Area::new(Id::new("workbench.project_launcher"))
        .order(Order::Foreground)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            // The non-dismissable scrim prevents a project transition from
            // racing an interaction with the workbench below it.
            // accessibility-pointer-shim: it consumes pointer gestures while
            // remaining deliberately absent from keyboard/AT order.
            ui.allocate_rect(
                screen,
                Sense::click_and_drag().difference(Sense::focusable_noninteractive()),
            );
            ui.painter()
                .rect_filled(screen, 0.0, t.color.canvas_bg.gamma_multiply(0.62));

            let mut surface = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(surface_rect)
                    .layout(egui::Layout::top_down(Align::Min)),
            );
            Frame::new()
                .fill(t.color.bg_panel)
                .stroke(Stroke::new(1.0, t.color.border_strong))
                .corner_radius(t.radius_lg)
                .shadow(t.shadow())
                .show(&mut surface, |ui| {
                    ui.set_min_size(size);
                    if launcher_header(ui) {
                        action = Some(LauncherAction::Close);
                    }
                    launcher_status(ui, app);
                    ui.painter().hline(
                        ui.max_rect().x_range(),
                        ui.cursor().top(),
                        Stroke::new(1.0, t.color.border),
                    );
                    launcher_layout(ui, app, &mut action, size);
                });
        });

    if let Some(action) = action {
        match action {
            LauncherAction::Close => app.state.workbench.project_launcher_open = false,
            LauncherAction::Browse => {
                app.state.workbench.project_launcher_open = false;
                Command::OpenProject.execute(app);
            }
            LauncherAction::NewProject => {
                app.state.workbench.project_launcher_open = false;
                Command::NewProject.execute(app);
            }
            LauncherAction::Open(recent) => {
                app.state.workbench.project_launcher_open = false;
                app.open_recent_file(recent);
            }
            LauncherAction::Page(page) => {
                app.state.workbench.project_launcher_page = page;
                if page == ProjectLauncherPage::Recovery {
                    app.state
                        .workbench
                        .project_launcher_recovery
                        .request_refresh();
                }
            }
            LauncherAction::Recover(candidate) => match open_comparison(app, &candidate) {
                Ok(()) => app.state.workbench.project_launcher_open = false,
                Err(error) => app.state.workbench.project_launcher_recovery.warning(error),
            },
            LauncherAction::RequestDiscard(candidate) => {
                app.state
                    .workbench
                    .project_launcher_recovery
                    .pending_discard = Some(candidate);
            }
            LauncherAction::StartSafeMode(options) => match start_local_safe_mode(app, options) {
                Ok(()) => app.state.workbench.project_launcher_open = false,
                Err(error) => app.state.workbench.project_launcher_recovery.warning(error),
            },
        }
    }
    show_discard_confirmation(ctx, app);
}

fn launcher_layout(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    action: &mut Option<LauncherAction>,
    size: Vec2,
) {
    let compact = size.x < 680.0;
    if compact {
        Frame::new()
            .fill(Tokens::get(ui.ctx()).color.bg_inset)
            .inner_margin(Margin::symmetric(12, 8))
            .show(ui, |ui| launcher_nav(ui, app, action, true));
        ui.painter().hline(
            ui.max_rect().x_range(),
            ui.cursor().top(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
        );
        launcher_page_scroll(ui, app, action);
        return;
    }

    ui.horizontal(|ui| {
        Frame::new()
            .fill(Tokens::get(ui.ctx()).color.bg_inset)
            .inner_margin(Margin::symmetric(10, 14))
            .show(ui, |ui| {
                ui.set_width(174.0);
                ui.set_min_height(ui.available_height());
                launcher_nav(ui, app, action, false);
            });
        ui.painter().vline(
            ui.cursor().left(),
            ui.max_rect().y_range(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
        );
        launcher_page_scroll(ui, app, action);
    });
}

fn launcher_nav(ui: &mut Ui, app: &RSpiceApp, action: &mut Option<LauncherAction>, compact: bool) {
    let recovery_count = app
        .state
        .workbench
        .project_launcher_recovery
        .candidates
        .len();
    let render = |ui: &mut Ui, page: ProjectLauncherPage, action: &mut Option<LauncherAction>| {
        let icon = match page {
            ProjectLauncherPage::Projects => WorkbenchIcon::Folder,
            ProjectLauncherPage::Recovery => WorkbenchIcon::History,
            ProjectLauncherPage::SafeMode => WorkbenchIcon::Warning,
        };
        let label = if page == ProjectLauncherPage::Recovery && recovery_count > 0 {
            format!("{}  {recovery_count}", page.label())
        } else {
            page.label().to_owned()
        };
        let selected = app.state.workbench.project_launcher_page == page;
        let response = ui.selectable_label(selected, label);
        let icon_rect = Rect::from_min_size(
            egui::pos2(response.rect.left() + 5.0, response.rect.center().y - 8.0),
            Vec2::splat(16.0),
        );
        icon.paint(
            ui.painter(),
            icon_rect,
            if selected {
                Tokens::get(ui.ctx()).color.accent
            } else {
                Tokens::get(ui.ctx()).color.text_dim
            },
        );
        if response.clicked() {
            *action = Some(LauncherAction::Page(page));
        }
    };

    if compact {
        ui.horizontal_wrapped(|ui| {
            for page in ProjectLauncherPage::ALL {
                render(ui, page, action);
            }
        });
    } else {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("STARTUP")
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(Tokens::get(ui.ctx()).color.text_faint),
            );
            ui.add_space(5.0);
            for page in ProjectLauncherPage::ALL {
                render(ui, page, action);
            }
        });
    }
}

fn launcher_page_scroll(ui: &mut Ui, app: &mut RSpiceApp, action: &mut Option<LauncherAction>) {
    egui::ScrollArea::vertical()
        .id_salt("workbench.project_launcher.body")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            Frame::new()
                .inner_margin(Margin::symmetric(20, 18))
                .show(ui, |ui| match app.state.workbench.project_launcher_page {
                    ProjectLauncherPage::Projects => launcher_body(ui, app, action),
                    ProjectLauncherPage::Recovery => recovery_page(ui, app, action),
                    ProjectLauncherPage::SafeMode => safe_mode_page(ui, app, action),
                });
        });
}

fn launcher_header(ui: &mut Ui) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut close = false;
    Frame::new()
        .inner_margin(Margin::symmetric(18, 12))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(34.0), Sense::hover());
                WorkbenchIcon::Brand.paint(ui.painter(), icon_rect.shrink(4.0), t.color.accent);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("RSPICE COMMERCIAL WORKBENCH")
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.accent),
                    );
                    ui.label(
                        egui::RichText::new("Start RSpice")
                            .font(theme::sans(tokens::FS_4, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    close = IconButton::new(Icon::Close)
                        .side(28.0)
                        .tooltip("Close project launcher (Esc)")
                        .show(ui)
                        .clicked();
                });
            });
        });
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.0, t.color.border),
    );
    close
}

fn launcher_status(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(18, 7))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let (dot, _) = ui.allocate_exact_size(Vec2::splat(9.0), Sense::hover());
                let (license_text, color) = app.state.license.as_ref().map_or_else(
                    || ("No commercial entitlement loaded".to_owned(), t.color.warn),
                    |license| (format!("{} entitlement active", license.tier), t.color.ok),
                );
                ui.painter().circle_filled(dot.center(), 3.0, color);
                ui.label(
                    egui::RichText::new(license_text)
                        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.separator();
                ui.label(
                    egui::RichText::new(concat!(
                        "RSpice Desktop ",
                        env!("CARGO_PKG_VERSION"),
                        " · engine ",
                        env!("RSPICE_BUILD_HASH")
                    ))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
        });
}

fn launcher_body(ui: &mut Ui, app: &mut RSpiceApp, action: &mut Option<LauncherAction>) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("PROJECT LAUNCHER · LOCAL AND SHARED")
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.accent),
            );
            ui.label(
                egui::RichText::new("Open engineering work")
                    .font(theme::sans(22.0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                egui::RichText::new(
                    "Open a recent project, browse to another project, or create a new one.",
                )
                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        });
        ui.with_layout(egui::Layout::right_to_left(Align::TOP), |ui| {
            if Button::new("New project")
                .icon(Icon::Add)
                .accent()
                .show(ui)
                .clicked()
            {
                *action = Some(LauncherAction::NewProject);
            }
            if Button::new("Browse…").icon(Icon::Folder).show(ui).clicked() {
                *action = Some(LauncherAction::Browse);
            }
        });
    });
    ui.add_space(18.0);

    launcher_toolbar(ui, app);
    ui.add_space(12.0);

    let entries = project_entries(app);
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            Frame::new()
                .fill(t.color.bg_panel)
                .inner_margin(Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("RECENT")
                                .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                                .color(t.color.text_dim),
                        );
                        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(entries.len().to_string())
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_faint),
                            );
                        });
                    });
                });
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.cursor().top(),
                Stroke::new(1.0, t.color.border),
            );
            if entries.is_empty() {
                empty_project_list(ui, app.state.workbench.project_launcher_query.is_empty());
            } else {
                for entry in entries {
                    if project_row(ui, &entry).clicked() {
                        *action = Some(LauncherAction::Open(entry.recent));
                    }
                }
            }
        });
}

fn recovery_page(ui: &mut Ui, app: &mut RSpiceApp, action: &mut Option<LauncherAction>) {
    let t = Tokens::get(ui.ctx());
    launcher_page_heading(
        ui,
        "STARTUP RECOVERY · NON-DESTRUCTIVE",
        "Recover interrupted work",
        "Recovery opens an unsaved comparison project. It never overwrites the saved source or checkpoint.",
    );
    ui.add_space(16.0);

    if let Some(notice) = app.state.workbench.project_launcher_recovery.notice.clone() {
        let color = match notice.tone {
            RecoveryNoticeTone::Info => t.color.ok,
            RecoveryNoticeTone::Warning => t.color.warn,
        };
        Frame::new()
            .fill(color.gamma_multiply(0.10))
            .stroke(Stroke::new(1.0, color.gamma_multiply(0.65)))
            .corner_radius(t.radius)
            .inner_margin(Margin::symmetric(12, 9))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(notice.message)
                        .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                        .color(t.color.text),
                );
            });
        ui.add_space(10.0);
    }

    let candidates = app
        .state
        .workbench
        .project_launcher_recovery
        .candidates
        .clone();
    let selected = app
        .state
        .workbench
        .project_launcher_recovery
        .selected_checkpoint
        .clone();
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            if candidates.is_empty() {
                Frame::new()
                    .inner_margin(Margin::symmetric(18, 28))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("No interrupted work found")
                                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                                    .color(t.color.text),
                            );
                            ui.label(
                                egui::RichText::new(
                                    "No eligible interrupted-session checkpoint is associated with a recent local schematic.",
                                )
                                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                .color(t.color.text_dim),
                            );
                        });
                    });
                return;
            }

            for candidate in candidates {
                let is_selected = selected.as_ref() == Some(&candidate.checkpoint);
                if recovery_row(ui, &candidate, is_selected).clicked() {
                    app.state
                        .workbench
                        .project_launcher_recovery
                        .select(candidate.checkpoint.clone());
                }
                ui.painter().hline(
                    ui.max_rect().x_range(),
                    ui.cursor().top(),
                    Stroke::new(1.0, t.color.border),
                );
            }
        });

    let selected = app
        .state
        .workbench
        .project_launcher_recovery
        .selected()
        .cloned();
    if let Some(candidate) = &selected {
        ui.add_space(12.0);
        recovery_contract(ui, candidate);
    }

    ui.add_space(14.0);
    ui.horizontal_wrapped(|ui| {
        let can_discard = selected
            .as_ref()
            .is_some_and(RecoveryCandidate::can_discard);
        let discard = Button::new("Discard selected checkpoint…")
            .enabled(can_discard)
            .show(ui);
        let discard = if selected
            .as_ref()
            .is_some_and(RecoveryCandidate::is_legacy_checkpoint)
        {
            discard.on_hover_text(
                "Legacy checkpoint ownership cannot be proven; open it non-destructively or use explicit recovery maintenance or migration",
            )
        } else {
            discard
        };
        if discard.clicked()
            && let Some(candidate) = selected.clone()
        {
            *action = Some(LauncherAction::RequestDiscard(candidate));
        }
        if Button::new("Recovery options").show(ui).clicked() {
            *action = Some(LauncherAction::Page(ProjectLauncherPage::SafeMode));
        }
        let recoverable = selected
            .as_ref()
            .is_some_and(|candidate| candidate.integrity.is_recoverable());
        let replacement_block_reason = recoverable
            .then(|| recovery_replacement_block_reason(&app.state))
            .flatten();
        let response = Button::new("Open recovery comparison")
            .accent()
            .enabled(recoverable && replacement_block_reason.is_none())
            .show(ui);
        let response = if let Some(reason) = replacement_block_reason {
            response.on_hover_text(reason)
        } else if !recoverable && selected.is_some() {
            response.on_hover_text("The selected checkpoint failed integrity validation")
        } else {
            response
        };
        if response.clicked()
            && let Some(candidate) = selected
        {
            *action = Some(LauncherAction::Recover(candidate));
        }
    });
}

fn recovery_row(ui: &mut Ui, candidate: &RecoveryCandidate, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let response = Frame::new()
        .fill(if selected {
            t.color.accent_dim
        } else {
            egui::Color32::TRANSPARENT
        })
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.add(egui::RadioButton::new(selected, ""));
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
                WorkbenchIcon::History.paint(
                    ui.painter(),
                    icon_rect.shrink(5.0),
                    if candidate.integrity.is_recoverable() {
                        t.color.warn
                    } else {
                        t.color.err
                    },
                );
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "{} · interrupted session",
                            candidate.display_name
                        ))
                        .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(recovery_summary(candidate))
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(if candidate.integrity.is_recoverable() {
                            "review"
                        } else {
                            "blocked"
                        })
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(if candidate.integrity.is_recoverable() {
                            t.color.warn
                        } else {
                            t.color.err
                        }),
                    );
                    ui.label(
                        egui::RichText::new(&candidate.age)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                });
            });
        })
        .response
        .interact(Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::RadioButton,
            ui.is_enabled(),
            format!("Select recovery checkpoint for {}", candidate.display_name),
        )
    });
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn recovery_summary(candidate: &RecoveryCandidate) -> String {
    match &candidate.integrity {
        RecoveryIntegrity::Verified {
            baseline_available,
            baseline_note,
            components,
            wires,
            changed_objects,
        } => {
            let delta = changed_objects.map_or_else(
                || "change count unavailable".to_owned(),
                |count| format!("{count} structural changes"),
            );
            format!(
                "{components} components · {wires} wires · {delta} · {}",
                if *baseline_available && baseline_note.is_none() {
                    "checkpoint and baseline verified"
                } else if *baseline_available {
                    "checkpoint verified · saved baseline changed or unrecorded"
                } else {
                    "checkpoint verified · baseline unavailable"
                }
            )
        }
        RecoveryIntegrity::Invalid(error) => error.clone(),
    }
}

fn recovery_contract(ui: &mut Ui, candidate: &RecoveryCandidate) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            recovery_contract_row(ui, "Saved source", &candidate.original.display().to_string());
            recovery_contract_row(
                ui,
                "Recovery point",
                &candidate.checkpoint.display().to_string(),
            );
            match &candidate.integrity {
                RecoveryIntegrity::Verified {
                    baseline_available,
                    baseline_note,
                    ..
                } => {
                    recovery_contract_row(
                        ui,
                        "Protected data",
                        "Saved source and checkpoint remain unchanged; recovery opens as an unsaved project.",
                    );
                    recovery_contract_row(
                        ui,
                        "Recommended action",
                        if *baseline_available {
                            "Compare the editable recovery candidate with the read-only saved baseline."
                        } else {
                            baseline_note.as_deref().unwrap_or(
                                "Review the verified checkpoint without a saved baseline.",
                            )
                        },
                    );
                    if let Some(note) = baseline_note {
                        recovery_contract_row(ui, "Baseline status", note);
                    }
                }
                RecoveryIntegrity::Invalid(error) => {
                    recovery_contract_row(ui, "Integrity", error);
                    recovery_contract_row(
                        ui,
                        "Required action",
                        "Retain or discard the checkpoint; invalid content is never opened.",
                    );
                }
            }
        });
}

fn recovery_contract_row(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.add_sized(
            [130.0, 18.0],
            egui::Label::new(
                egui::RichText::new(label)
                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                    .color(t.color.text_dim),
            ),
        );
        ui.label(
            egui::RichText::new(value)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text),
        );
    });
}

fn safe_mode_page(ui: &mut Ui, app: &mut RSpiceApp, action: &mut Option<LauncherAction>) {
    let t = Tokens::get(ui.ctx());
    launcher_page_heading(
        ui,
        "SAFE MODE · STARTUP ISOLATION",
        "Start with recoverable session state isolated",
        "Safe mode changes only the current launch. The prior session remains the source of truth for the next normal launch.",
    );
    ui.add_space(16.0);

    let active = app.state.workbench.safe_mode.active;
    if active {
        Frame::new()
            .fill(t.color.ok.gamma_multiply(0.10))
            .stroke(Stroke::new(1.0, t.color.ok.gamma_multiply(0.65)))
            .corner_radius(t.radius)
            .inner_margin(Margin::symmetric(12, 9))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Safe mode is active for this launch")
                        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                        .color(t.color.text),
                );
            });
        ui.add_space(10.0);
    }

    ui.add_enabled_ui(!active, |ui| {
        safe_mode_option(
            ui,
            &mut app
                .state
                .workbench
                .safe_mode
                .draft
                .isolate_prior_documents,
            "Do not reopen prior documents",
            "Start with a new unsaved project. The complete prior session is restored on the next normal launch.",
        );
        safe_mode_option(
            ui,
            &mut app.state.workbench.safe_mode.draft.reset_layout,
            "Reset dock and monitor geometry",
            "Restore navigator, inspector, console, and dock dimensions to the primary workbench layout.",
        );
    });

    ui.add_space(14.0);
    let options = app.state.workbench.safe_mode.draft;
    let response = Button::new("Start RSpice in safe mode")
        .accent()
        .enabled(!active && options.has_effect())
        .show(ui);
    let response = if active {
        response.on_hover_text("Safe mode is already active for this launch")
    } else if !options.has_effect() {
        response.on_hover_text("Select at least one isolation option")
    } else {
        response
    };
    if response.clicked() {
        *action = Some(LauncherAction::StartSafeMode(options));
    }

    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(
            "Renderer and extension-host isolation are not offered here because this build has no runtime adapter that can enforce those transitions without a restart.",
        )
        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
        .color(t.color.text_faint),
    );
}

fn safe_mode_option(ui: &mut Ui, checked: &mut bool, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(checked, "");
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(title)
                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(detail)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
            });
        });
    ui.add_space(8.0);
}

fn launcher_page_heading(ui: &mut Ui, eyebrow: &str, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(eyebrow)
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.accent),
    );
    ui.label(
        egui::RichText::new(title)
            .font(theme::sans(22.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(detail)
            .font(theme::sans(tokens::FS_2, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn show_discard_confirmation(ctx: &Context, app: &mut RSpiceApp) {
    let Some(candidate) = app
        .state
        .workbench
        .project_launcher_recovery
        .pending_discard
        .clone()
    else {
        return;
    };
    let choice = Dialog::new(
        "Recovery",
        "Discard recovery checkpoint?",
        "Discard checkpoint",
    )
    .size(DialogSize::Sm)
    .destructive()
    .ghost("Keep checkpoint")
    .hint("The saved source is not modified")
    .show(ctx, |ui| {
        ui.label(format!(
            "Discard the autosave checkpoint for '{}'? This removes only '{}'.",
            candidate.display_name,
            candidate.checkpoint.display()
        ));
    });

    match choice {
        DialogChoice::None => {}
        DialogChoice::Primary => {
            let outcome = discard_checkpoint(&candidate);
            let catalog = &mut app.state.workbench.project_launcher_recovery;
            catalog.pending_discard = None;
            match outcome {
                Ok(message) => catalog.info(message),
                Err(error) => catalog.warning(error),
            }
            catalog.request_refresh();
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state
                .workbench
                .project_launcher_recovery
                .pending_discard = None;
        }
        DialogChoice::Secondary => {}
    }
}

fn launcher_toolbar(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        let search = ui.add_sized(
            [ui.available_width().min(560.0).max(220.0), t.metrics.ctl_h],
            egui::TextEdit::singleline(&mut app.state.workbench.project_launcher_query)
                .hint_text("Project, path, owner, tag…")
                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                .margin(Vec2::new(9.0, 5.0)),
        );
        search.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::TextEdit,
                ui.is_enabled(),
                "Project, path, owner, tag",
            )
        });
        if std::mem::take(&mut app.state.workbench.focus_project_launcher_search) {
            search.request_focus();
        }

        let selected = app.state.workbench.project_launcher_sort.label();
        let options = vec!["Last opened".to_owned(), "Name".to_owned()];
        if let Some(index) = select(
            ui,
            "project_launcher_sort",
            "Project sort",
            selected,
            &options,
            132.0,
        ) {
            app.state.workbench.project_launcher_sort = if index == 0 {
                ProjectLauncherSort::LastOpened
            } else {
                ProjectLauncherSort::Name
            };
        }
    });
}

fn project_entries(app: &RSpiceApp) -> Vec<ProjectEntry> {
    let query = app
        .state
        .workbench
        .project_launcher_query
        .trim()
        .to_lowercase();
    let current_path = app.state.workspace.project.path.as_deref();
    let current_name = app.state.workspace.project.display_name();
    let mut entries = app
        .state
        .recent_files
        .iter()
        .filter(|recent| recent.kind == RecentKind::Project)
        .filter_map(|recent| {
            let current = current_path.is_some_and(|path| path == recent.path);
            let name = if current {
                current_name.to_owned()
            } else {
                project_name_from_path(&recent.path)
            };
            let path_text = recent.path.display().to_string();
            let matches = query.is_empty()
                || name.to_lowercase().contains(&query)
                || path_text.to_lowercase().contains(&query);
            matches.then(|| ProjectEntry {
                recent: recent.clone(),
                name,
                path_text,
                available: project_path_available(&recent.path),
                current,
            })
        })
        .collect::<Vec<_>>();

    if app.state.workbench.project_launcher_sort == ProjectLauncherSort::Name {
        entries.sort_by(|left, right| {
            left.name
                .to_lowercase()
                .cmp(&right.name.to_lowercase())
                .then_with(|| left.path_text.cmp(&right.path_text))
        });
    }
    entries
}

fn project_row(ui: &mut Ui, entry: &ProjectEntry) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let response = Frame::new()
        .fill(if entry.current {
            t.color.accent_dim
        } else {
            egui::Color32::TRANSPARENT
        })
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
                WorkbenchIcon::Project.paint(
                    ui.painter(),
                    icon_rect.shrink(5.0),
                    if entry.available {
                        t.color.accent
                    } else {
                        t.color.err
                    },
                );
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(&entry.name)
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(&entry.path_text)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    let (label, color) = if !entry.available {
                        ("missing", t.color.err)
                    } else if entry.current {
                        ("ready", t.color.ok)
                    } else {
                        ("local", t.color.text_dim)
                    };
                    ui.label(
                        egui::RichText::new(label)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(color),
                    );
                });
            });
        })
        .response
        .interact(Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("Open project {}", entry.name),
        )
    });
    if response.hovered() {
        ui.painter().rect_stroke(
            response.rect,
            0.0,
            Stroke::new(1.0, t.color.border_strong),
            egui::StrokeKind::Inside,
        );
    }
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn empty_project_list(ui: &mut Ui, query_empty: bool) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .inner_margin(Margin::symmetric(18, 28))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(if query_empty {
                        "No recent projects"
                    } else {
                        "No projects match this view"
                    })
                    .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                    .color(t.color.text),
                );
                ui.label(
                    egui::RichText::new(if query_empty {
                        "Browse to an existing project or create a new one."
                    } else {
                        "Change the search term. Project data has not been modified."
                    })
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
        });
}

fn project_name_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Unnamed project")
        .to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
fn project_path_available(path: &Path) -> bool {
    path.is_file()
}

#[cfg(target_arch = "wasm32")]
fn project_path_available(_path: &Path) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_names_come_from_the_file_name_without_extension() {
        assert_eq!(
            project_name_from_path(Path::new("C:/Engineering/Precision AFE.rspiceproj")),
            "Precision AFE"
        );
    }

    #[test]
    fn project_names_have_a_nonempty_fallback() {
        assert_eq!(project_name_from_path(Path::new("/")), "Unnamed project");
    }
}
