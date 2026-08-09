//! No-project landing workspace.
//!
//! This is not a synthetic project overview. It projects only durable
//! device-local recents and verified interrupted-session candidates, then
//! dispatches the same file and lifecycle workflows as the application menu.

use egui::{
    Align, Align2, Color32, Frame, Layout, Margin, Rect, Response, ScrollArea, Sense, Stroke, Ui,
    UiBuilder, Vec2, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::{RecentFile, RecentKind};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::lifecycle::recovery::{RecoveryCandidate, RecoveryIntegrity};
use crate::workbench::state::{ProjectLauncherFilter, ProjectLauncherPage, Workspace};

const HEADER_HEIGHT: f32 = 116.0;
const CONTENT_MAX_WIDTH: f32 = 1240.0;
const DESKTOP_GUTTER: f32 = 30.0;
const BODY_TOP: f32 = 14.0;
const BODY_BOTTOM: f32 = 26.0;
const RAIL_WIDTH: f32 = 322.0;
const COLUMN_GAP: f32 = 18.0;
const STACK_BREAKPOINT: f32 = 900.0;
const PHONE_RECENTS_BREAKPOINT: f32 = 620.0;
const TABLE_MIN_WIDTH: f32 = 720.0;
const TABLE_HEADER_HEIGHT: f32 = 27.0;
const TABLE_ROW_HEIGHT: f32 = 32.0;
const TABLE_TOOLBAR_HEIGHT: f32 = 34.0;
const SECTION_HEADER_HEIGHT: f32 = 24.0;
const ACTION_ROW_HEIGHT: f32 = 51.0;
const EXAMPLE_ROW_HEIGHT: f32 = 43.0;

const EXAMPLES: [ExampleEntry; 6] = [
    ExampleEntry {
        name: "RC Lowpass Filter",
        category: "Analog",
        detail: "1st-order RC · 1 kHz corner",
    },
    ExampleEntry {
        name: "Voltage Divider",
        category: "Basics",
        detail: "Resistive ratio and loading",
    },
    ExampleEntry {
        name: "Common Emitter Amplifier",
        category: "Analog",
        detail: "BJT bias point and small-signal gain",
    },
    ExampleEntry {
        name: "CMOS Inverter",
        category: "Digital",
        detail: "Complementary NOT gate",
    },
    ExampleEntry {
        name: "Differential Pair",
        category: "Analog",
        detail: "BJT input stage",
    },
    ExampleEntry {
        name: "Opamp Inverting Amplifier",
        category: "Analog",
        detail: "Closed-loop gain and bandwidth",
    },
];

#[derive(Debug, Clone, Copy)]
struct ExampleEntry {
    name: &'static str,
    category: &'static str,
    detail: &'static str,
}

#[derive(Debug, Clone)]
struct RecentProjectRow {
    recent: RecentFile,
    name: String,
    location: String,
    owner: String,
    last_opened: String,
    availability: &'static str,
    availability_color: Color32,
    pinned: bool,
}

#[derive(Debug, Clone)]
enum LandingAction {
    NewProject,
    OpenProject,
    OpenNetlist,
    OpenSchematic,
    OpenRecent(RecentFile),
    ReviewRecovery,
    LoadExample(&'static str),
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    crate::workbench::lifecycle::recovery::refresh_catalog_if_requested(app);

    let mut action = None;
    egui::Frame::new()
        .fill(Tokens::get(ui.ctx()).color.bg_app)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            landing_header(ui);
            landing_body(ui, app, &mut action);
        });

    if let Some(action) = action {
        execute_action(app, action);
    }
}

fn landing_header(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, HEADER_HEIGHT), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );

    let content = centered_content_rect(rect, DESKTOP_GUTTER);
    let mut header = ui.new_child(
        UiBuilder::new()
            .max_rect(content)
            .layout(Layout::top_down(Align::Min)),
    );
    header.add_space(26.0);
    header.label(
        egui::RichText::new("NO PROJECT OPEN")
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint)
            .extra_letter_spacing(0.09 * tokens::FS_0),
    );
    header.add_space(5.0);
    header.label(
        egui::RichText::new("RSpice Workbench")
            .font(theme::sans(20.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    header.add_space(3.0);
    header.label(
        egui::RichText::new(
            "Nothing is loaded. Open recent work, create a project, or import engineering data.",
        )
        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
        .color(t.color.text_dim),
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            "No project open. RSpice Workbench",
        )
    });
}

fn landing_body(ui: &mut Ui, app: &mut RSpiceApp, action: &mut Option<LandingAction>) {
    let available = ui.available_rect_before_wrap();
    ui.allocate_rect(available, Sense::hover());
    let mut content = centered_content_rect(available, DESKTOP_GUTTER);
    content.min.y = (content.top() + BODY_TOP).min(content.bottom());
    content.max.y = (content.bottom() - BODY_BOTTOM).max(content.top());

    if content.width() >= STACK_BREAKPOINT {
        let rail_width = RAIL_WIDTH.min((content.width() * 0.40).max(1.0));
        let continue_width = (content.width() - rail_width - COLUMN_GAP).max(1.0);
        let left_rect = Rect::from_min_size(content.min, vec2(continue_width, content.height()));
        let right_rect = Rect::from_min_max(
            pos2(left_rect.right() + COLUMN_GAP, content.top()),
            content.right_bottom(),
        );

        let mut left = ui.new_child(
            UiBuilder::new()
                .max_rect(left_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        left.set_clip_rect(left_rect);
        continue_column(&mut left, app, action);

        let mut right = ui.new_child(
            UiBuilder::new()
                .max_rect(right_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        right.set_clip_rect(right_rect);
        ScrollArea::vertical()
            .id_salt("workbench.project.landing.rail")
            .auto_shrink([false, false])
            .show(&mut right, |ui| start_rail(ui, action));
    } else {
        let mut stacked = ui.new_child(
            UiBuilder::new()
                .max_rect(content)
                .layout(Layout::top_down(Align::Min)),
        );
        stacked.set_clip_rect(content);
        ScrollArea::vertical()
            .id_salt("workbench.project.landing.stacked")
            .auto_shrink([false, false])
            .show(&mut stacked, |ui| {
                ui.set_min_width(content.width());
                continue_column(ui, app, action);
                ui.add_space(16.0);
                start_rail(ui, action);
            });
    }
}

fn centered_content_rect(rect: Rect, minimum_gutter: f32) -> Rect {
    let gutter = if rect.width() <= 560.0 {
        16.0
    } else {
        minimum_gutter
    };
    let max_width = (rect.width() - gutter * 2.0).clamp(1.0, CONTENT_MAX_WIDTH);
    Rect::from_center_size(rect.center(), vec2(max_width, rect.height()))
}

fn continue_column(ui: &mut Ui, app: &mut RSpiceApp, action: &mut Option<LandingAction>) {
    let candidates = app
        .state
        .workbench
        .project_launcher_recovery
        .candidates
        .clone();
    if !candidates.is_empty() {
        recovery_card(ui, &candidates, action);
        ui.add_space(8.0);
    }
    recent_projects(ui, app, action);
}

fn recovery_card(
    ui: &mut Ui,
    candidates: &[RecoveryCandidate],
    action: &mut Option<LandingAction>,
) {
    let t = Tokens::get(ui.ctx());
    let candidate = &candidates[0];
    let shown = Frame::new()
        .fill(t.color.warn.gamma_multiply(0.055))
        .stroke(Stroke::new(1.0, t.color.warn.gamma_multiply(0.58)))
        .corner_radius(t.radius)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            compact_section_header(
                ui,
                "Interrupted session",
                &format!("{} to review", candidates.len()),
                t.color.warn,
            );
            Frame::new()
                .inner_margin(Margin::symmetric(10, 7))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let (icon_rect, _) =
                            ui.allocate_exact_size(vec2(17.0, 32.0), Sense::hover());
                        WorkbenchIcon::History.paint(
                            ui.painter(),
                            Rect::from_center_size(icon_rect.center(), Vec2::splat(14.0)),
                            t.color.warn,
                        );
                        ui.allocate_ui_with_layout(
                            vec2((ui.available_width() - 86.0).max(1.0), 34.0),
                            Layout::top_down(Align::Min),
                            |ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&candidate.display_name)
                                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                            .color(t.color.text),
                                    )
                                    .truncate(),
                                );
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(recovery_detail(candidate))
                                            .font(theme::sans(
                                                tokens::FS_MICRO,
                                                FontWeight::Regular,
                                            ))
                                            .color(t.color.text_dim),
                                    )
                                    .truncate(),
                                );
                            },
                        );
                        if Button::new("Review…").show(ui).clicked() {
                            *action = Some(LandingAction::ReviewRecovery);
                        }
                    });
                });
        });
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_label("Interrupted session recovery");
        node.set_description(recovery_detail(candidate));
    });
}

fn recovery_detail(candidate: &RecoveryCandidate) -> String {
    let contents = match &candidate.integrity {
        RecoveryIntegrity::Verified {
            components,
            wires,
            changed_objects,
            ..
        } => changed_objects.map_or_else(
            || format!("{components} components · {wires} wires"),
            |changed| format!("{changed} changed objects"),
        ),
        RecoveryIntegrity::Invalid(_) => "checkpoint needs diagnostics".to_owned(),
    };
    format!("{contents} · {}", candidate.age)
}

fn recent_projects(ui: &mut Ui, app: &mut RSpiceApp, action: &mut Option<LandingAction>) {
    let t = Tokens::get(ui.ctx());
    let rows = recent_project_rows(app, &t);
    let compact = ui.available_width() < PHONE_RECENTS_BREAKPOINT;
    // A stacked landing page lives inside an outer scroll area, where the
    // available height can be unbounded. Keep the table useful without ever
    // allowing it to claim an infinite or phone-hostile viewport.
    let height = if compact {
        if rows.is_empty() {
            222.0
        } else {
            (142.0 + rows.len() as f32 * 58.0).clamp(250.0, 390.0)
        }
    } else {
        ui.available_height().clamp(180.0, 420.0)
    };
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect(
        rect,
        t.radius,
        t.color.bg_panel,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );

    let mut panel = ui.new_child(
        UiBuilder::new()
            .max_rect(rect.shrink(1.0))
            .layout(Layout::top_down(Align::Min)),
    );
    recent_toolbar(&mut panel, app, rows.len());
    let rows_rect = panel.available_rect_before_wrap();
    panel.allocate_rect(rows_rect, Sense::hover());
    let mut rows_ui = panel.new_child(
        UiBuilder::new()
            .max_rect(rows_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    rows_ui.set_clip_rect(rows_rect);
    let no_recents = app
        .state
        .recent_files
        .iter()
        .all(|recent| recent.kind != RecentKind::Project);
    if compact {
        ScrollArea::vertical()
            .id_salt("workbench.project.landing.recents.compact")
            .auto_shrink([false, false])
            .show(&mut rows_ui, |ui| {
                ui.set_min_width(rows_rect.width());
                compact_recent_header(ui);
                if rows.is_empty() {
                    recent_empty_state(ui, no_recents);
                } else {
                    for row in &rows {
                        let response = compact_recent_row(ui, row);
                        if response_activated(ui, &response) {
                            *action = Some(LandingAction::OpenRecent(row.recent.clone()));
                        }
                    }
                }
            });
    } else {
        ScrollArea::both()
            .id_salt("workbench.project.landing.recents")
            .auto_shrink([false, false])
            .show(&mut rows_ui, |ui| {
                ui.set_min_width(TABLE_MIN_WIDTH.max(rows_rect.width()));
                recent_table_header(ui);
                if rows.is_empty() {
                    recent_empty_state(ui, no_recents);
                } else {
                    for row in &rows {
                        let response = recent_table_row(ui, row);
                        if response_activated(ui, &response) {
                            *action = Some(LandingAction::OpenRecent(row.recent.clone()));
                        }
                    }
                }
            });
    }
}

fn recent_toolbar(ui: &mut Ui, app: &mut RSpiceApp, shown: usize) {
    let t = Tokens::get(ui.ctx());
    let frame = Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(9, 4))
        .show(ui, |ui| {
            ui.set_min_height(TABLE_TOOLBAR_HEIGHT - 8.0);
            if ui.available_width() < PHONE_RECENTS_BREAKPOINT {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;
                    recent_toolbar_title(ui, shown);
                    ui.add_space((ui.available_width() - 154.0).max(4.0));
                    recent_toolbar_filters(ui, app);
                });
                ui.add_space(4.0);
                let search_width = ui.available_width();
                recent_toolbar_search(ui, app, search_width);
            } else {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 7.0;
                    recent_toolbar_title(ui, shown);
                    let search_width = (ui.available_width() - 174.0).clamp(150.0, 250.0);
                    recent_toolbar_search(ui, app, search_width);
                    recent_toolbar_filters(ui, app);
                });
            }
        });
    ui.painter().hline(
        frame.response.rect.x_range(),
        frame.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn recent_toolbar_title(ui: &mut Ui, shown: usize) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new("Recent projects")
            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(shown.to_string())
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn recent_toolbar_search(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let search = ui.add_sized(
        [width.max(1.0), 24.0],
        egui::TextEdit::singleline(&mut app.state.workbench.project_launcher_query)
            .hint_text("Project, path, owner…"),
    );
    ui.ctx().accesskit_node_builder(search.id, |node| {
        node.set_label("Search recent projects");
    });
}

fn recent_toolbar_filters(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for (filter, label) in [
            (ProjectLauncherFilter::All, "All"),
            (ProjectLauncherFilter::Pinned, "Pinned"),
            (ProjectLauncherFilter::Shared, "Shared"),
        ] {
            if segment_button(
                ui,
                label,
                app.state.workbench.project_launcher_filter == filter,
            )
            .clicked()
            {
                app.state.workbench.project_launcher_filter = filter;
            }
        }
    });
}

fn segment_button(ui: &mut Ui, label: &str, selected: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let width = match label {
        "Pinned" | "Shared" => 56.0,
        _ => 42.0,
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, 24.0), Sense::click());
    ui.painter().rect_filled(
        rect,
        0.0,
        if selected {
            t.color.bg_active
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
    );
    ui.painter().rect_stroke(
        rect,
        0.0,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    if selected {
        ui.painter().rect_filled(
            Rect::from_min_max(pos2(rect.left(), rect.bottom() - 2.0), rect.right_bottom()),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn recent_table_header(ui: &mut Ui) {
    table_cells(
        ui,
        [
            "Project",
            "Location",
            "Owner",
            "Last opened",
            "Availability",
        ],
        true,
        None,
    );
}

fn compact_recent_header(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), TABLE_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    ui.painter().text(
        pos2(rect.left() + 8.0, rect.center().y),
        Align2::LEFT_CENTER,
        "Project",
        font.clone(),
        t.color.text_faint,
    );
    ui.painter().text(
        pos2(rect.right() - 8.0, rect.center().y),
        Align2::RIGHT_CENTER,
        "Last opened",
        font,
        t.color.text_faint,
    );
}

fn compact_recent_row(ui: &mut Ui, row: &RecentProjectRow) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 58.0), Sense::click());
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let right_width = 96.0;
    let left_width = (rect.width() - right_width - 24.0).max(24.0);
    let name_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let location_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let name = super::elide_text(ui, &row.name, &name_font, left_width);
    let location = super::elide_text(ui, &row.location, &location_font, left_width);
    ui.painter().text(
        pos2(rect.left() + 9.0, rect.top() + 15.0),
        Align2::LEFT_CENTER,
        name,
        name_font,
        t.color.text,
    );
    ui.painter().text(
        pos2(rect.left() + 9.0, rect.bottom() - 15.0),
        Align2::LEFT_CENTER,
        location,
        location_font,
        t.color.text_faint,
    );
    ui.painter().text(
        pos2(rect.right() - 9.0, rect.top() + 15.0),
        Align2::RIGHT_CENTER,
        &row.last_opened,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        pos2(rect.right() - 9.0, rect.bottom() - 15.0),
        Align2::RIGHT_CENTER,
        row.availability,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        row.availability_color,
    );
    if row.pinned {
        WorkbenchIcon::Pin.paint(
            ui.painter(),
            Rect::from_center_size(
                pos2(rect.right() - right_width - 7.0, rect.top() + 15.0),
                Vec2::splat(10.0),
            ),
            t.color.text_faint,
        );
    }
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Button,
            ui.is_enabled(),
            format!("Open project {}", row.name),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(format!(
            "{}; {}; {}; {}",
            row.location, row.owner, row.last_opened, row.availability
        ));
    });
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn recent_table_row(ui: &mut Ui, row: &RecentProjectRow) -> Response {
    let t = Tokens::get(ui.ctx());
    let fields = [
        row.name.as_str(),
        row.location.as_str(),
        row.owner.as_str(),
        row.last_opened.as_str(),
        row.availability,
    ];
    let (response, cells) = table_cells(ui, fields, false, Some(row.availability_color));
    if response.hovered() {
        ui.painter()
            .rect_filled(response.rect, 0.0, t.color.bg_hover);
        // Repaint the cells above the hover fill.
        paint_table_text(
            ui,
            response.rect,
            &cells,
            fields,
            false,
            Some(row.availability_color),
        );
    }
    if row.pinned {
        WorkbenchIcon::Pin.paint(
            ui.painter(),
            Rect::from_center_size(
                pos2(cells[0].right() - 12.0, cells[0].center().y),
                Vec2::splat(10.0),
            ),
            t.color.text_faint,
        );
    }
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Button,
            ui.is_enabled(),
            format!("Open project {}", row.name),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(format!(
            "{}; {}; {}; {}",
            row.location, row.owner, row.last_opened, row.availability
        ));
    });
    theme::paint_focus_ring(ui, &response, response.rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn table_cells<const N: usize>(
    ui: &mut Ui,
    fields: [&str; N],
    header: bool,
    trailing_color: Option<Color32>,
) -> (Response, Vec<Rect>) {
    debug_assert_eq!(N, 5);
    let t = Tokens::get(ui.ctx());
    let height = if header {
        TABLE_HEADER_HEIGHT
    } else {
        TABLE_ROW_HEIGHT
    };
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width().max(TABLE_MIN_WIDTH), height),
        if header {
            Sense::hover()
        } else {
            Sense::click()
        },
    );
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let cells = table_cell_rects(rect);
    paint_table_text(ui, rect, &cells, fields, header, trailing_color);
    (response, cells)
}

fn table_cell_rects(rect: Rect) -> Vec<Rect> {
    let fractions = [0.25_f32, 0.33, 0.17, 0.14, 0.11];
    let mut cells = Vec::with_capacity(fractions.len());
    let mut left = rect.left();
    for (index, fraction) in fractions.into_iter().enumerate() {
        let right = if index == fractions.len() - 1 {
            rect.right()
        } else {
            left + rect.width() * fraction
        };
        cells.push(Rect::from_min_max(
            pos2(left, rect.top()),
            pos2(right, rect.bottom()),
        ));
        left = right;
    }
    cells
}

fn paint_table_text<const N: usize>(
    ui: &Ui,
    row: Rect,
    cells: &[Rect],
    fields: [&str; N],
    header: bool,
    trailing_color: Option<Color32>,
) {
    let t = Tokens::get(ui.ctx());
    for (index, (cell, text)) in cells.iter().zip(fields).enumerate() {
        let font = if header {
            theme::sans(tokens::FS_0, FontWeight::Medium)
        } else if matches!(index, 1 | 3 | 4) {
            theme::mono(tokens::FS_0, FontWeight::Regular)
        } else {
            theme::sans(tokens::FS_0, FontWeight::Regular)
        };
        let color = if header {
            t.color.text_faint
        } else if index == N - 1 {
            trailing_color.unwrap_or(t.color.text_dim)
        } else if index == 1 {
            t.color.text_faint
        } else {
            t.color.text_dim
        };
        let inset = cell.shrink2(vec2(8.0, 0.0));
        let max_width = if index == 0 && !header {
            (inset.width() - 18.0).max(1.0)
        } else {
            inset.width()
        };
        let text = super::elide_text(ui, text, &font, max_width);
        ui.painter().with_clip_rect(inset.intersect(row)).text(
            inset.left_center(),
            Align2::LEFT_CENTER,
            text,
            font,
            color,
        );
    }
}

fn recent_empty_state(ui: &mut Ui, no_recents: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 82.0), Sense::hover());
    let mut empty = ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::top_down(Align::Center)),
    );
    empty.add_space(21.0);
    empty.label(
        egui::RichText::new(if no_recents {
            "No projects yet"
        } else {
            "No projects match this view"
        })
        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
        .color(t.color.text),
    );
    empty.label(
        egui::RichText::new(if no_recents {
            "Projects you create or open are listed here, most recent first."
        } else {
            "Clear the search or filter to see every recent project."
        })
        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
        .color(t.color.text_dim),
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            if no_recents {
                "No recent projects"
            } else {
                "No projects match this view"
            },
        )
    });
}

fn recent_project_rows(app: &RSpiceApp, t: &Tokens) -> Vec<RecentProjectRow> {
    let query = app
        .state
        .workbench
        .project_launcher_query
        .trim()
        .to_ascii_lowercase();
    let filter = app.state.workbench.project_launcher_filter;
    app.state
        .recent_files
        .iter()
        .filter(|recent| recent.kind == RecentKind::Project)
        .filter_map(|recent| {
            let shared = path_is_shared(&recent.path);
            let matches_filter = match filter {
                ProjectLauncherFilter::All | ProjectLauncherFilter::Recent => true,
                ProjectLauncherFilter::Pinned => recent.pinned,
                ProjectLauncherFilter::Shared => shared,
            };
            let name = recent_project_name(recent);
            let location = recent.path.display().to_string();
            let owner = recent.owner.clone().unwrap_or_else(|| "—".to_owned());
            let searchable = format!("{} {} {} {}", name, location, owner, recent.tags.join(" "))
                .to_ascii_lowercase();
            if !matches_filter || (!query.is_empty() && !searchable.contains(&query)) {
                return None;
            }
            let (availability, availability_color) = if cfg!(target_arch = "wasm32") {
                ("browse", t.color.warn)
            } else if shared {
                ("shared", t.color.ok)
            } else {
                ("local", t.color.text_dim)
            };
            Some(RecentProjectRow {
                recent: recent.clone(),
                name,
                location,
                owner,
                last_opened: recent_age(recent.opened_at_unix_ms),
                availability,
                availability_color,
                pinned: recent.pinned,
            })
        })
        .collect()
}

fn recent_project_name(recent: &RecentFile) -> String {
    recent
        .path
        .file_stem()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Unnamed project")
        .to_owned()
}

fn path_is_shared(path: &std::path::Path) -> bool {
    let path = path.as_os_str().to_string_lossy();
    path.starts_with(r"\\") || path.starts_with("//")
}

fn recent_age(opened_at_unix_ms: u64) -> String {
    if opened_at_unix_ms == 0 {
        return "unknown".to_owned();
    }
    let now = crate::time_compat::unix_epoch()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX);
    recent_age_at(opened_at_unix_ms, now)
}

fn recent_age_at(opened_at_unix_ms: u64, now_unix_ms: u64) -> String {
    let elapsed = now_unix_ms.saturating_sub(opened_at_unix_ms) / 1_000;
    match elapsed {
        0..=59 => "just now".to_owned(),
        60..=3_599 => format!("{} min", elapsed / 60),
        3_600..=86_399 => format!("{} h", elapsed / 3_600),
        86_400..=172_799 => "Yesterday".to_owned(),
        _ => format!("{} d", elapsed / 86_400),
    }
}

fn start_rail(ui: &mut Ui, action: &mut Option<LandingAction>) {
    for (title, detail, icon, primary, next) in [
        (
            "New project",
            "Analog, RF or mixed-signal · blank top cell",
            WorkbenchIcon::Add,
            true,
            LandingAction::NewProject,
        ),
        (
            "Open project…",
            if cfg!(target_arch = "wasm32") {
                "Choose a .rspiceproj file to import"
            } else {
                "Local or shared .rspiceproj"
            },
            WorkbenchIcon::Folder,
            false,
            LandingAction::OpenProject,
        ),
        (
            "Open SPICE deck…",
            "Spectre · HSPICE · ngspice · Xyce · CDL",
            WorkbenchIcon::Code,
            false,
            LandingAction::OpenNetlist,
        ),
        (
            "Open schematic…",
            "Single .rsch cell view",
            WorkbenchIcon::Design,
            false,
            LandingAction::OpenSchematic,
        ),
    ] {
        if landing_action_row(ui, title, detail, icon, primary).clicked() {
            *action = Some(next);
        }
        ui.add_space(5.0);
    }

    ui.add_space(4.0);
    compact_section_header(
        ui,
        "Example circuits",
        &EXAMPLES.len().to_string(),
        Tokens::get(ui.ctx()).color.text_dim,
    );
    for example in EXAMPLES {
        if example_row(ui, example).clicked() {
            *action = Some(LandingAction::LoadExample(example.name));
        }
        ui.add_space(3.0);
    }
}

fn landing_action_row(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    icon: WorkbenchIcon,
    primary: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let height = ACTION_ROW_HEIGHT.max(if t.metrics.ctl_h >= 44.0 { 56.0 } else { 0.0 });
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    let border = if primary {
        t.color.accent.gamma_multiply(0.58)
    } else if response.hovered() {
        t.color.accent
    } else {
        t.color.border
    };
    let fill = if primary {
        t.color.accent_dim
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        t.color.bg_panel
    };
    ui.painter().rect(
        rect,
        t.radius,
        fill,
        Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );
    let icon_rect =
        Rect::from_center_size(pos2(rect.left() + 18.0, rect.center().y), Vec2::splat(16.0));
    icon.paint(ui.painter(), icon_rect, t.color.text_dim);
    let text_rect = Rect::from_min_max(
        pos2(rect.left() + 38.0, rect.top() + 7.0),
        pos2(rect.right() - 10.0, rect.bottom() - 6.0),
    );
    let mut copy = ui.new_child(
        UiBuilder::new()
            .max_rect(text_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    copy.spacing_mut().item_spacing.y = 1.0;
    copy.add(
        egui::Label::new(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text),
        )
        .truncate(),
    );
    copy.add(
        egui::Label::new(
            egui::RichText::new(detail)
                .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                .color(t.color.text_faint),
        )
        .truncate(),
    );
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), title.to_owned()));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(detail);
    });
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn example_row(ui: &mut Ui, example: ExampleEntry) -> Response {
    let t = Tokens::get(ui.ctx());
    let height = EXAMPLE_ROW_HEIGHT.max(if t.metrics.ctl_h >= 44.0 { 50.0 } else { 0.0 });
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    ui.painter().rect(
        rect,
        t.radius,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
        Stroke::new(
            1.0,
            if response.hovered() {
                t.color.border_strong
            } else {
                Color32::TRANSPARENT
            },
        ),
        egui::StrokeKind::Inside,
    );
    let inset = rect.shrink2(vec2(11.0, 5.0));
    let category_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let category_width = ui
        .painter()
        .layout_no_wrap(
            example.category.to_owned(),
            category_font.clone(),
            t.color.text_faint,
        )
        .size()
        .x;
    ui.painter().text(
        inset.right_top(),
        Align2::RIGHT_TOP,
        example.category,
        category_font,
        t.color.text_faint,
    );
    let name_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let name = super::elide_text(
        ui,
        example.name,
        &name_font,
        (inset.width() - category_width - 10.0).max(1.0),
    );
    ui.painter().text(
        inset.left_top(),
        Align2::LEFT_TOP,
        name,
        name_font,
        t.color.text,
    );
    let detail_font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let detail = super::elide_text(ui, example.detail, &detail_font, inset.width());
    ui.painter().text(
        pos2(inset.left(), inset.bottom()),
        Align2::LEFT_BOTTOM,
        detail,
        detail_font,
        t.color.text_dim,
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Button,
            ui.is_enabled(),
            format!("Load example {}", example.name),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(format!("{}; {}", example.category, example.detail));
    });
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn compact_section_header(ui: &mut Ui, title: &str, meta: &str, meta_color: Color32) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), SECTION_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        pos2(rect.left() + 10.0, rect.center().y),
        Align2::LEFT_CENTER,
        title.to_ascii_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    ui.painter().text(
        pos2(rect.right() - 10.0, rect.center().y),
        Align2::RIGHT_CENTER,
        meta.to_ascii_uppercase(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        meta_color,
    );
}

fn response_activated(ui: &mut Ui, response: &Response) -> bool {
    response.clicked()
        || (response.has_focus()
            && ui.input_mut(|input| {
                input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    || input.consume_key(egui::Modifiers::NONE, egui::Key::Space)
            }))
}

fn execute_action(app: &mut RSpiceApp, action: LandingAction) {
    match action {
        LandingAction::NewProject => Command::NewProject.execute(app),
        LandingAction::OpenProject => Command::OpenProject.execute(app),
        LandingAction::OpenNetlist => Command::OpenNetlist.execute(app),
        LandingAction::OpenSchematic => Command::OpenDocument.execute(app),
        LandingAction::OpenRecent(recent) => {
            #[cfg(not(target_arch = "wasm32"))]
            app.open_recent_file(recent);
            #[cfg(target_arch = "wasm32")]
            {
                let _ = recent;
                Command::OpenProject.execute(app);
            }
        }
        LandingAction::ReviewRecovery => {
            app.state.workbench.open_project_launcher();
            app.state.workbench.project_launcher_page = ProjectLauncherPage::Recovery;
            app.state
                .workbench
                .project_launcher_recovery
                .request_refresh();
        }
        LandingAction::LoadExample(name) => load_example_project(app, name),
    }
}

fn load_example_project(app: &mut RSpiceApp, name: &str) {
    crate::workbench::workflows::project_workflow::create_new_project(&mut app.state);
    if !app.state.project_lifecycle.project_open {
        return;
    }
    crate::workbench::examples::load_example(name, &mut app.state.schematic);
    app.state.sync_active_schematic_to_workspace();
    app.state
        .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
            "Loaded example circuit: {name}"
        )));
    Command::OpenWorkspace(Workspace::Design).execute(app);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project_recent(path: &str, pinned: bool, owner: Option<&str>) -> RecentFile {
        RecentFile {
            kind: RecentKind::Project,
            path: path.into(),
            opened_at_unix_ms: 1_000,
            pinned,
            owner: owner.map(str::to_owned),
            tags: Vec::new(),
        }
    }

    #[test]
    fn landing_recents_share_search_pin_and_owner_authority_with_the_launcher() {
        let mut app = RSpiceApp::test_instance();
        app.state.recent_files = vec![
            project_recent("C:/Engineering/afe.rspiceproj", true, Some("Analog Design")),
            project_recent("//lab/rf/rf_pa.rspiceproj", false, Some("RF Systems")),
        ];
        app.state.workbench.project_launcher_filter = ProjectLauncherFilter::Pinned;
        app.state.workbench.project_launcher_query = "analog".to_owned();
        let tokens = Tokens::default();

        let rows = recent_project_rows(&app, &tokens);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "afe");
        assert_eq!(rows[0].owner, "Analog Design");
        assert!(rows[0].pinned);
    }

    #[test]
    fn landing_shared_filter_uses_portable_unc_identity() {
        let mut app = RSpiceApp::test_instance();
        app.state.recent_files = vec![
            project_recent("C:/Engineering/afe.rspiceproj", false, None),
            project_recent("//lab/rf/rf_pa.rspiceproj", false, None),
        ];
        app.state.workbench.project_launcher_filter = ProjectLauncherFilter::Shared;

        let rows = recent_project_rows(&app, &Tokens::default());

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "rf_pa");
    }

    #[test]
    fn every_landing_example_creates_authored_project_content() {
        for example in EXAMPLES {
            let mut app = RSpiceApp::test_instance();
            app.state.project_lifecycle.project_open = false;

            load_example_project(&mut app, example.name);

            assert!(app.state.project_lifecycle.project_open, "{}", example.name);
            assert!(
                !app.state.schematic.components.is_empty(),
                "{} did not create a circuit",
                example.name
            );
            assert_eq!(app.state.workbench.workspace, Workspace::Design);
        }
    }

    #[test]
    fn landing_renders_without_project_state_at_all_supported_widths() {
        for size in [vec2(1440.0, 900.0), vec2(820.0, 1180.0), vec2(390.0, 844.0)] {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut app = RSpiceApp::test_instance();
            app.state.project_lifecycle.project_open = false;
            app.state.workbench.project_launcher_recovery = Default::default();

            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
                },
            );

            assert!(!output.shapes.is_empty(), "{}x{}", size.x, size.y);
            assert!(!app.state.project_lifecycle.project_open);
        }
    }

    #[test]
    fn relative_time_boundaries_are_stable() {
        let now = 10 * 86_400_000;
        assert_eq!(recent_age_at(now - 59_000, now), "just now");
        assert_eq!(recent_age_at(now - 60_000, now), "1 min");
        assert_eq!(recent_age_at(now - 3_600_000, now), "1 h");
        assert_eq!(recent_age_at(now - 86_400_000, now), "Yesterday");
        assert_eq!(recent_age_at(now - 3 * 86_400_000, now), "3 d");
    }
}
