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
use crate::ui::widgets::{Button, IconButton, select};

use super::commands::Command;
use super::design_system::WorkbenchIcon;
use super::state::ProjectLauncherSort;

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
}

pub(super) fn show(ctx: &Context, app: &mut RSpiceApp) {
    if !app.state.workbench.project_launcher_open {
        return;
    }

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
                    egui::ScrollArea::vertical()
                        .id_salt("workbench.project_launcher.body")
                        .auto_shrink([false, false])
                        .max_height((size.y - 91.0).max(180.0))
                        .show(ui, |ui| {
                            Frame::new()
                                .inner_margin(Margin::symmetric(20, 18))
                                .show(ui, |ui| launcher_body(ui, app, &mut action));
                        });
                });
        });

    if let Some(action) = action {
        app.state.workbench.project_launcher_open = false;
        match action {
            LauncherAction::Close => {}
            LauncherAction::Browse => Command::OpenProject.execute(app),
            LauncherAction::NewProject => Command::NewProject.execute(app),
            LauncherAction::Open(recent) => app.open_recent_file(recent),
        }
    }
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
