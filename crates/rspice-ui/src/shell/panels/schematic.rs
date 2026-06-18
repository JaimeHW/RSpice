//! Schematic-context side panels: the left rail (navigator + library +
//! place strip) and the instance inspector (right).
//!
//! The rail implements design/app/volta-schematic-rail.html: one rail, two
//! contexts. NAVIGATOR answers "where am I, what is here" — nameplate,
//! occurrence path, instances/nets/ports. LIBRARY answers "what can I
//! place" — palette categories, project and vendor libraries, a stable
//! preview. The PLACE strip at the bottom serves both, so recall
//! placement never requires leaving the navigator.

use std::collections::{HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use egui::Ui;

use crate::common::AppState;
use crate::shell::{
    WorkspaceView,
    state::{NavMode, RailTab},
};
use crate::state::{CellViewRef, ComponentType, NavColumn, PortSpec, Tool};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, TreeRow, chip, input_row, input_row_readonly, kv_row, mono_input, section_header,
    select,
};

// ---------------------------------------------------------------------------
// Left rail
// ---------------------------------------------------------------------------

/// Render the schematic context's left rail.
pub fn left(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    // The place strip owns the bottom edge; everything else fills above it.
    egui::TopBottomPanel::bottom("volta.rail.place")
        .frame(
            egui::Frame::none()
                .fill(c.bg_panel)
                .inner_margin(egui::Margin {
                    left: 12.0,
                    right: 12.0,
                    top: 8.0,
                    bottom: 10.0,
                }),
        )
        .show_separator_line(false)
        .show_inside(ui, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.top() - 8.0,
                egui::Stroke::new(1.0, c.border),
            );
            place_strip(ui, state, symbol_library);
        });

    rail_tabs(ui, state);
    match state.shell.rail_tab {
        RailTab::Navigator => navigator(ui, state),
        RailTab::Library => library(ui, state, symbol_library),
    }
}

/// Two equal tabs: NAVIGATOR | LIBRARY. The active one carries the accent
/// underline; there is deliberately no third tab.
fn rail_tabs(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 35.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, theme::mix(c.bg_panel, c.bg_inset, 0.18));

    let tabs_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 6.0, rect.top() + 6.0),
        egui::pos2(rect.right() - 6.0, rect.bottom()),
    );
    painter.rect_filled(tabs_rect, t.radius, c.bg_inset);
    let half = tabs_rect.width() / 2.0;

    for (index, (tab, label)) in [
        (RailTab::Navigator, "NAVIGATOR"),
        (RailTab::Library, "LIBRARY"),
    ]
    .into_iter()
    .enumerate()
    {
        let tab_rect = egui::Rect::from_min_size(
            egui::pos2(tabs_rect.left() + half * index as f32, tabs_rect.top()),
            egui::vec2(half, tabs_rect.height()),
        );
        let response = ui.interact(
            tab_rect,
            ui.id().with(("rail.tab", index)),
            egui::Sense::click(),
        );
        let active = state.shell.rail_tab == tab;
        let hover =
            ui.ctx()
                .animate_bool_with_time(response.id, response.hovered() && !active, 0.16);
        let fill = if active {
            c.bg_panel
        } else {
            theme::mix(c.bg_inset, c.bg_panel, 0.38)
        };
        painter.rect_filled(tab_rect, 0.0, fill);
        if hover > 0.0 {
            painter.rect_filled(
                tab_rect,
                0.0,
                theme::mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
            );
        }
        painter.text(
            tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            if active { c.text } else { c.text_faint },
        );
        if active {
            painter.hline(
                tab_rect.x_range().shrink(1.0),
                tab_rect.top() + 1.0,
                egui::Stroke::new(2.0, c.accent),
            );
        }
        if response.clicked() {
            state.shell.rail_tab = tab;
        }
    }
    painter.vline(
        tabs_rect.center().x,
        tabs_rect.y_range(),
        egui::Stroke::new(1.0, c.border),
    );
    painter.rect_stroke(tabs_rect, t.radius, egui::Stroke::new(1.0, c.border));
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, c.border),
    );
}

// ---------------------------------------------------------------------------
// Navigator
// ---------------------------------------------------------------------------

fn navigator(ui: &mut Ui, state: &mut AppState) {
    premium_nameplate(ui, state);
    pathbar(ui, state);

    // Find in design.
    ui.add_space(8.0);
    let response = rail_search_input(ui, &mut state.shell.nav_search, "find object", "/");
    if state.shell.focus_nav_search {
        response.request_focus();
        state.shell.focus_nav_search = false;
    }
    response.on_hover_text("Find instances, nets and ports (/)");

    nav_segments(ui, state);

    let c = Tokens::get(ui.ctx()).color;
    egui::Frame::none()
        .inner_margin(egui::Margin {
            left: 10.0,
            right: 10.0,
            top: 0.0,
            bottom: 0.0,
        })
        .show(ui, |ui| {
            egui::Frame::none()
                .fill(c.bg_inset)
                .stroke(egui::Stroke::new(1.0, c.border))
                .inner_margin(egui::Margin {
                    left: 4.0,
                    right: 4.0,
                    top: 5.0,
                    bottom: 5.0,
                })
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("volta.rail.tree")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            let query = state.shell.nav_search.trim().to_ascii_lowercase();
                            if query.is_empty() {
                                match state.shell.nav_mode {
                                    NavMode::Instances => instance_rows(ui, state, None),
                                    NavMode::Nets => net_rows(ui, state, None),
                                    NavMode::Ports => port_rows(ui, state, None),
                                }
                            } else {
                                // One query, all three kinds, grouped — click stays live.
                                let counts = navigator_search_counts(state, &query);
                                if counts.is_empty() {
                                    nav_no_match(ui, state, &query);
                                } else {
                                    if counts.instances > 0 {
                                        instance_rows(ui, state, Some(&query));
                                    }
                                    if counts.nets > 0 {
                                        net_rows(ui, state, Some(&query));
                                    }
                                    if counts.ports > 0 {
                                        port_rows(ui, state, Some(&query));
                                    }
                                }
                            }
                            ui.add_space(8.0);
                        });
                });
        });
}

fn rail_search_input(
    ui: &mut Ui,
    value: &mut String,
    hint: &'static str,
    shortcut: &str,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let (outer_rect, _) = ui.allocate_exact_size(egui::vec2(width, 27.0), egui::Sense::hover());
    let rect = egui::Rect::from_min_max(
        egui::pos2(outer_rect.left() + 10.0, outer_rect.top()),
        egui::pos2(outer_rect.right() - 10.0, outer_rect.bottom()),
    );
    let field_response = ui.interact(rect, ui.id().with("premium.search"), egui::Sense::click());
    let painter = ui.painter();
    let hovered = field_response.hovered();
    painter.rect(
        rect,
        t.radius,
        c.bg_inset,
        egui::Stroke::new(1.0, if hovered { c.border_strong } else { c.border }),
    );

    let icon_center = egui::pos2(rect.left() + 14.0, rect.center().y - 1.0);
    let icon_stroke = egui::Stroke::new(1.5, c.text_faint);
    painter.circle_stroke(icon_center, 4.2, icon_stroke);
    painter.line_segment(
        [
            egui::pos2(icon_center.x + 3.2, icon_center.y + 3.2),
            egui::pos2(icon_center.x + 6.2, icon_center.y + 6.2),
        ],
        icon_stroke,
    );

    let key_font = theme::mono(10.0, FontWeight::Medium);
    let key_width = text_width(ui, shortcut, &key_font) + 9.0;
    let key_rect = egui::Rect::from_min_size(
        egui::pos2(rect.right() - key_width - 6.0, rect.center().y - 8.0),
        egui::vec2(key_width, 16.0),
    );
    painter.rect(
        key_rect,
        2.0,
        theme::mix(c.bg_panel, c.bg_inset, 0.55),
        egui::Stroke::new(1.0, c.border),
    );
    painter.text(
        key_rect.center(),
        egui::Align2::CENTER_CENTER,
        shortcut,
        key_font,
        c.text_faint,
    );

    let input_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 26.0, rect.top() + 1.0),
        egui::pos2(
            (key_rect.left() - 6.0).max(rect.left() + 54.0),
            rect.bottom() - 1.0,
        ),
    );
    let mut input_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(input_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let response = input_ui.add_sized(
        input_rect.size(),
        egui::TextEdit::singleline(value)
            .font(egui::TextStyle::Monospace)
            .hint_text(hint)
            .frame(false)
            .margin(egui::Margin::symmetric(0.0, 4.0)),
    );
    if field_response.clicked() {
        response.request_focus();
    }
    response
}

// Premium project capsule: library/cell identity, view affordance, and
// canonical counts for the active schematic.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PremiumProjectMetric {
    value: String,
    label: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PremiumProjectCapsule {
    library: String,
    cell: String,
    view: String,
    metrics: [PremiumProjectMetric; 4],
}

#[cfg(test)]
impl PremiumProjectCapsule {
    fn metric_labels(&self) -> [&'static str; 4] {
        [
            self.metrics[0].label,
            self.metrics[1].label,
            self.metrics[2].label,
            self.metrics[3].label,
        ]
    }
}

fn premium_project_capsule(state: &AppState) -> PremiumProjectCapsule {
    let reference = state.workspace.active_view.clone();
    let instances = state
        .schematic
        .components
        .iter()
        .filter(|component| component.kind != ComponentType::Port)
        .count();
    let nets = design_nets_cached(state).len();
    let ports = state.schematic.interface_ports().len();

    PremiumProjectCapsule {
        library: reference.library,
        cell: reference.cell,
        view: reference.view,
        metrics: [
            PremiumProjectMetric {
                value: instances.to_string(),
                label: "inst",
            },
            PremiumProjectMetric {
                value: nets.to_string(),
                label: "nets",
            },
            PremiumProjectMetric {
                value: ports.to_string(),
                label: "ports",
            },
            PremiumProjectMetric {
                value: "1".to_string(),
                label: "sheet",
            },
        ],
    }
}

fn premium_nav_filter_labels() -> [&'static str; 3] {
    ["Inst", "Nets", "Ports"]
}

fn premium_nameplate(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let capsule = premium_project_capsule(state);
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 88.0), egui::Sense::hover());
    let painter = ui.painter();

    let card_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 10.0, rect.top() + 8.0),
        egui::pos2(rect.right() - 10.0, rect.bottom() - 8.0),
    );
    let header_rect = egui::Rect::from_min_max(
        card_rect.min,
        egui::pos2(card_rect.right(), card_rect.top() + 40.0),
    );
    let metrics_rect = egui::Rect::from_min_max(
        egui::pos2(card_rect.left(), header_rect.bottom()),
        card_rect.max,
    );
    let header_fill = theme::mix(c.bg_panel, c.bg_hover, 0.38);

    painter.rect_filled(rect, 0.0, c.bg_panel);
    painter.rect_filled(card_rect, t.radius_lg, c.bg_inset);
    painter.rect_filled(header_rect, t.radius_lg, header_fill);
    painter.rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(header_rect.left(), header_rect.bottom() - t.radius_lg),
            header_rect.right_bottom(),
        ),
        0.0,
        header_fill,
    );
    painter.hline(
        header_rect.x_range(),
        header_rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    painter.rect_stroke(
        card_rect,
        t.radius_lg,
        egui::Stroke::new(1.0, c.border_strong),
    );

    let lib_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let cell_font = theme::mono(tokens::FS_2, FontWeight::SemiBold);
    let chip_font = theme::mono(10.0, FontWeight::Medium);
    let chip_width = text_width(ui, &capsule.view, &chip_font) + 18.0;
    let chip_rect = egui::Rect::from_min_size(
        egui::pos2(
            header_rect.right() - chip_width - 7.0,
            header_rect.top() + 11.0,
        ),
        egui::vec2(chip_width, 18.0),
    );
    let text_right = (chip_rect.left() - 8.0).max(header_rect.left() + 36.0);
    let text_budget = (text_right - header_rect.left() - 8.0).max(24.0);
    let library = fit_text(ui, &capsule.library, &lib_font, text_budget);
    let cell = fit_text(ui, &capsule.cell, &cell_font, text_budget);

    painter.text(
        egui::pos2(header_rect.left() + 8.0, header_rect.top() + 7.0),
        egui::Align2::LEFT_TOP,
        library,
        lib_font,
        c.text_faint,
    );
    painter.text(
        egui::pos2(header_rect.left() + 8.0, header_rect.top() + 22.0),
        egui::Align2::LEFT_TOP,
        cell,
        cell_font,
        c.text,
    );

    let chip_response = ui
        .interact(
            chip_rect,
            ui.id().with(("premium.view.chip", &capsule.view)),
            egui::Sense::click(),
        )
        .on_hover_text("Open this view in Library");
    paint_premium_view_chip(
        ui,
        chip_rect,
        &capsule.view,
        chip_response.hovered(),
        chip_response.is_pointer_button_down_on(),
    );
    if chip_response.clicked() {
        focus_active_project_view_in_library(state, &capsule);
    }

    let metric_width = metrics_rect.width() / capsule.metrics.len() as f32;
    let value_font = theme::mono(tokens::FS_1, FontWeight::SemiBold);
    let label_font = theme::mono(10.0, FontWeight::Regular);
    for (index, metric) in capsule.metrics.iter().enumerate() {
        let left = metrics_rect.left() + metric_width * index as f32;
        let metric_rect = egui::Rect::from_min_size(
            egui::pos2(left, metrics_rect.top()),
            egui::vec2(metric_width, metrics_rect.height()),
        );
        if index > 0 {
            painter.vline(
                metric_rect.left(),
                metric_rect.y_range().shrink(5.0),
                egui::Stroke::new(1.0, c.border),
            );
        }
        let value = fit_text(ui, &metric.value, &value_font, metric_width - 10.0);
        painter.text(
            egui::pos2(metric_rect.left() + 6.0, metric_rect.top() + 5.0),
            egui::Align2::LEFT_TOP,
            value,
            value_font.clone(),
            c.text,
        );
        painter.text(
            egui::pos2(metric_rect.left() + 6.0, metric_rect.top() + 18.0),
            egui::Align2::LEFT_TOP,
            metric.label,
            label_font.clone(),
            c.text_faint,
        );
    }

    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
}

fn focus_active_project_view_in_library(state: &mut AppState, capsule: &PremiumProjectCapsule) {
    if state
        .library_manager
        .get_library(&capsule.library)
        .is_some()
    {
        state.library_manager.select_library(&capsule.library);
        state
            .library_manager
            .select_cell(&capsule.library, &capsule.cell);
        state
            .library_manager
            .select_view(&capsule.library, &capsule.cell, &capsule.view);
        state.library_manager.nav_column = NavColumn::View;
    }
    state.shell.view = WorkspaceView::Library;
}

fn paint_premium_view_chip(ui: &Ui, rect: egui::Rect, text: &str, hovered: bool, pressed: bool) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let fill = if pressed {
        theme::mix(c.bg_inset, c.accent_dim, 0.75)
    } else if hovered {
        theme::mix(c.bg_inset, c.bg_hover, 0.75)
    } else {
        c.bg_inset
    };
    let stroke = if hovered {
        egui::Stroke::new(1.0, c.accent)
    } else {
        egui::Stroke::new(1.0, c.border_strong)
    };
    ui.painter().rect(rect, t.radius, fill, stroke);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        theme::mono(10.0, FontWeight::Medium),
        if hovered { c.text } else { c.text_faint },
    );
}

/// Occurrence path: the one stateful, accent-washed surface. Appears only
/// when descended; crumbs jump, the trailing action ascends one level (U).
fn pathbar(ui: &mut Ui, state: &mut AppState) {
    if state.workspace.hierarchy_stack.len() <= 1 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let labels = state.workspace.occurrence_labels();
    let last = labels.len() - 1;

    let mut focus: Option<usize> = None;
    let mut ascend = false;
    egui::Frame::none()
        .fill(c.accent_dim)
        .inner_margin(egui::Margin::symmetric(12.0, 5.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                let font = theme::mono(tokens::FS_0, FontWeight::Regular);
                let font_medium = theme::mono(tokens::FS_0, FontWeight::Medium);

                // The path mid-collapses, never the endpoints: deep
                // occurrences hide crumbs after TOP behind one '…' (the
                // tooltip carries them) so the row always fits — an
                // overflowing row would ratchet the panel wider.
                let sep_w = text_width(ui, "▸", &font) + 8.0;
                let ascend_w = text_width(ui, "ascend U", &font) + 12.0;
                let avail = (ui.available_width() - ascend_w).max(40.0);

                let mut visible: Vec<(String, usize)> = labels
                    .iter()
                    .enumerate()
                    .map(|(index, label)| {
                        let display = if index == 0 {
                            "TOP".to_owned()
                        } else {
                            label.clone()
                        };
                        (display, index)
                    })
                    .collect();
                let mut hidden: Vec<String> = Vec::new();
                let row_width = |ui: &Ui, visible: &[(String, usize)], hidden: &[String]| {
                    let mut width = 0.0;
                    for (position, (display, index)) in visible.iter().enumerate() {
                        if position > 0 {
                            width += sep_w;
                        }
                        let crumb_font = if *index == last { &font_medium } else { &font };
                        width += text_width(ui, display, crumb_font);
                    }
                    if !hidden.is_empty() {
                        width += text_width(ui, "…", &font) + sep_w;
                    }
                    width
                };
                while row_width(ui, &visible, &hidden) > avail && visible.len() > 2 {
                    hidden.push(visible.remove(1).0);
                }
                // Last resort: a single oversized crumb gets elided too.
                if let Some((display, index)) = visible.last().cloned() {
                    let others = row_width(ui, &visible[..visible.len() - 1], &hidden)
                        + if visible.len() > 1 { sep_w } else { 0.0 };
                    let budget = (avail - others).max(24.0);
                    let crumb_font = if index == last { &font_medium } else { &font };
                    let fitted = fit_text(ui, &display, crumb_font, budget);
                    visible.last_mut().expect("non-empty").0 = fitted;
                }

                for (position, (display, index)) in visible.iter().enumerate() {
                    if position > 0 {
                        ui.label(
                            egui::RichText::new("▸")
                                .font(font.clone())
                                .color(c.accent.gamma_multiply(0.5)),
                        );
                    }
                    if *index == last {
                        ui.label(
                            egui::RichText::new(display)
                                .font(font_medium.clone())
                                .color(c.accent),
                        );
                    } else if ui
                        .link(
                            egui::RichText::new(display)
                                .font(font.clone())
                                .color(c.accent),
                        )
                        .clicked()
                    {
                        focus = Some(*index);
                    }
                    if position == 0 && !hidden.is_empty() {
                        ui.label(
                            egui::RichText::new("▸")
                                .font(font.clone())
                                .color(c.accent.gamma_multiply(0.5)),
                        );
                        ui.label(egui::RichText::new("…").font(font.clone()).color(c.accent))
                            .on_hover_text(hidden.join(" ▸ "));
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .link(
                            egui::RichText::new("ascend U")
                                .font(font.clone())
                                .color(c.accent),
                        )
                        .on_hover_text("Up one level (U)")
                        .clicked()
                    {
                        ascend = true;
                    }
                });
            });
        });
    let rect = ui.max_rect();
    ui.painter().hline(
        rect.x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, c.border),
    );

    if let Some(index) = focus {
        state.focus_workspace_breadcrumb(index);
    }
    if ascend {
        state.ascend_workspace_level();
    }
}

/// Instances | Nets | Ports — equal thirds with live counts.
fn nav_segments(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let labels = premium_nav_filter_labels();

    ui.add_space(6.0);
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 24.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            // The three segments plus the layout's inter-item gaps must
            // total exactly the row — overflow ratchets the panel wider.
            ui.spacing_mut().item_spacing.x = 4.0;
            ui.add_space(10.0);
            let third = (ui.available_width() - 10.0 - 4.0 * 2.0) / 3.0;
            for (mode, label) in [
                (NavMode::Instances, labels[0]),
                (NavMode::Nets, labels[1]),
                (NavMode::Ports, labels[2]),
            ] {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(third, 24.0), egui::Sense::click());
                let active = state.shell.nav_mode == mode;
                let hover = ui.ctx().animate_bool_with_time(
                    response.id,
                    response.hovered() && !active,
                    0.16,
                );
                let painter = ui.painter();
                if active {
                    painter.rect(rect, t.radius, c.bg_inset, egui::Stroke::new(1.0, c.border));
                } else if hover > 0.0 {
                    painter.rect_filled(
                        rect,
                        t.radius,
                        theme::mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
                    );
                }
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    label.to_ascii_uppercase(),
                    theme::mono(10.0, FontWeight::SemiBold),
                    if active { c.text } else { c.text_faint },
                );
                if response.clicked() {
                    state.shell.nav_mode = mode;
                }
            }
        },
    );
    ui.add_space(5.0);
}

/// Instance rows — selection-synced; cell instances peek on the twist and
/// descend on double-click (the canvas gesture) or Shift+E.
fn instance_rows(ui: &mut Ui, state: &mut AppState, query: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    if let Some(query) = query {
        let count = state
            .schematic
            .components
            .iter()
            .filter(|component| component.kind != ComponentType::Port)
            .filter(|component| row_matches(component, query))
            .count();
        nav_group_header(ui, "Instances", count);
        if count == 0 {
            return;
        }
    } else {
        let cell = state.workspace.active_view.cell.clone();
        TreeRow::new(&cell).twist(true).meta("sheet 1/1").show(ui);
    }

    let mut clicked: Option<u64> = None;
    let mut peek_toggle: Option<u64> = None;
    let mut descend: Option<(String, CellViewRef)> = None;

    for index in 0..state.schematic.components.len() {
        let Some(component) = state.schematic.components.get(index) else {
            continue;
        };
        if component.kind == ComponentType::Port {
            continue; // ports live in their own segment
        }
        if let Some(query) = query
            && !row_matches(component, query)
        {
            continue;
        }

        let meta = if component.value.is_empty() {
            component.kind.display_name().to_owned()
        } else {
            component.value.clone()
        };
        let hierarchical =
            component.kind == ComponentType::CellInstance && component.library_cell.is_some();
        let status = component
            .library_cell
            .as_ref()
            .map(|binding| master_status(state, binding));
        let peeked = state.shell.nav_peek.contains(&component.id);
        let selected = state.schematic.selection.has_component(component.id);

        let mut row = TreeRow::new(&component.name)
            .meta(&meta)
            .indent(1)
            .mono()
            .selected(selected)
            .highlight_query(query);
        if hierarchical {
            row = row.twist(peeked);
        }
        if status.is_some_and(MasterStatus::is_missing) {
            row = row.chip_dot(c.err);
        }
        let result = row.show(ui);

        if result.response.double_clicked() && hierarchical {
            let binding = component.library_cell.as_ref().expect("hierarchical");
            descend = Some((
                component.name.clone(),
                CellViewRef::new(
                    binding.library.clone(),
                    binding.cell.clone(),
                    binding.view.clone(),
                ),
            ));
        } else if result.response.clicked() {
            // Clicking the twist zone peeks; anywhere else selects.
            let twist_zone = result.response.rect.left() + 16.0 + 12.0;
            let on_twist = hierarchical
                && result
                    .response
                    .interact_pointer_pos()
                    .is_some_and(|pos| pos.x < twist_zone);
            if on_twist {
                peek_toggle = Some(component.id);
            } else {
                clicked = Some(component.id);
            }
        }

        // Ghost peek: the master's contents, read-only — cheap to look,
        // impossible to edit by accident. Descend to make it real.
        if peeked && hierarchical {
            let binding = component.library_cell.as_ref().expect("hierarchical");
            match master_status(state, binding) {
                MasterStatus::Open => {
                    let key = format!("{}/{}/{}", binding.library, binding.cell, binding.view);
                    let master = state
                        .workspace
                        .schematic_buffers
                        .get(&key)
                        .expect("open status requires a schematic buffer");
                    for child in master.components.iter().take(8) {
                        if child.kind == ComponentType::Port {
                            continue;
                        }
                        let child_meta = if child.value.is_empty() {
                            child.kind.display_name().to_owned()
                        } else {
                            child.value.clone()
                        };
                        TreeRow::new(&child.name)
                            .meta(&child_meta)
                            .indent(2)
                            .mono()
                            .dim()
                            .show(ui);
                    }
                    let extra = master
                        .components
                        .iter()
                        .filter(|child| child.kind != ComponentType::Port)
                        .count()
                        .saturating_sub(8);
                    if extra > 0 {
                        TreeRow::new(&format!("… {extra} more"))
                            .indent(2)
                            .mono()
                            .dim()
                            .show(ui);
                    }
                }
                status => {
                    let message = status.peek_message(binding);
                    let mut row = TreeRow::new(&message).indent(2).dim();
                    if status.is_missing() {
                        row = row.chip_dot(c.err);
                    }
                    row.show(ui);
                }
            }
            ui.horizontal(|ui| {
                ui.add_space(12.0 + 32.0);
                if ui
                    .link(
                        egui::RichText::new("descend to edit ⌄")
                            .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                            .color(c.accent),
                    )
                    .on_hover_text("Open the master in its own context (Shift+E)")
                    .clicked()
                {
                    let binding = state
                        .schematic
                        .components
                        .get(index)
                        .and_then(|component| component.library_cell.clone());
                    if let (Some(binding), Some(component)) =
                        (binding, state.schematic.components.get(index))
                    {
                        descend = Some((
                            component.name.clone(),
                            CellViewRef::new(binding.library, binding.cell, binding.view),
                        ));
                    }
                }
            });
        }
    }

    if let Some(id) = clicked {
        state.schematic.selection.select_only_component(id);
    }
    if let Some(id) = peek_toggle
        && !state.shell.nav_peek.remove(&id)
    {
        state.shell.nav_peek.insert(id);
    }
    if let Some((instance, reference)) = descend {
        state.descend_into_instance(Some(instance), reference);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MasterStatus {
    Open,
    NotOpen,
    MissingLibrary,
    MissingCell,
    MissingView,
}

impl MasterStatus {
    fn is_missing(self) -> bool {
        matches!(
            self,
            Self::MissingLibrary | Self::MissingCell | Self::MissingView
        )
    }

    fn peek_message(self, binding: &crate::state::LibraryCellInstance) -> String {
        match self {
            Self::Open => String::new(),
            Self::NotOpen => format!(
                "{} / {} / {} is available; descend to inspect",
                binding.library, binding.cell, binding.view
            ),
            Self::MissingLibrary => format!("missing library {}", binding.library),
            Self::MissingCell => format!("missing cell {} / {}", binding.library, binding.cell),
            Self::MissingView => format!(
                "missing view {} / {} / {}",
                binding.library, binding.cell, binding.view
            ),
        }
    }
}

fn master_status(state: &AppState, binding: &crate::state::LibraryCellInstance) -> MasterStatus {
    let Some(library) = state.library_manager.get_library(&binding.library) else {
        return MasterStatus::MissingLibrary;
    };
    let Some(cell) = library.get_cell(&binding.cell) else {
        return MasterStatus::MissingCell;
    };
    if cell.get_view(&binding.view).is_none() {
        return MasterStatus::MissingView;
    }

    let reference = CellViewRef::new(&binding.library, &binding.cell, &binding.view);
    if state
        .workspace
        .schematic_buffers
        .contains_key(&reference.key())
    {
        MasterStatus::Open
    } else {
        MasterStatus::NotOpen
    }
}

/// Net rows — click cross-probes the canvas (net highlight).
fn net_rows(ui: &mut Ui, state: &mut AppState, query: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let nets = design_nets_cached(state);
    let visible: Vec<usize> = nets
        .iter()
        .enumerate()
        .filter(|(_, net)| query.is_none_or(|q| net.name.to_ascii_lowercase().contains(q)))
        .map(|(index, _)| index)
        .collect();

    if query.is_some() {
        nav_group_header(ui, "Nets", visible.len());
        if visible.is_empty() {
            return;
        }
    } else if visible.is_empty() {
        empty_note(ui, "No nets yet — draw a wire.");
        return;
    }

    let highlighted = &state.schematic.net_highlight.highlighted_wires;
    let mut highlight: Option<HashSet<u64>> = None;
    for index in visible {
        let net = &nets[index];
        let meta = if net.is_port {
            format!("port · {} pins", net.pin_count)
        } else {
            format!("{} pins", net.pin_count)
        };
        let selected = net_row_is_probed(
            &net.wire_ids,
            highlighted,
            state.schematic.net_highlight.active,
        );
        let mut row = TreeRow::new(&net.name)
            .meta(&meta)
            .indent(1)
            .mono()
            .selected(selected)
            .highlight_query(query);
        if net_row_is_probed(
            &net.wire_ids,
            highlighted,
            state.schematic.net_highlight.active,
        ) {
            row = row.chip_dot(c.warn);
        }
        let row = row.show(ui);
        if row.response.clicked() {
            highlight = Some(if selected {
                HashSet::new() // toggle off
            } else {
                net.wire_ids.iter().copied().collect()
            });
        }
    }
    if let Some(wires) = highlight {
        state.schematic.selection.clear();
        state.schematic.net_highlight.highlight_wires(wires);
    }
}

/// Port rows — the cell's interface, in document (netlist) order.
fn net_row_is_probed(wire_ids: &[u64], highlighted: &HashSet<u64>, active: bool) -> bool {
    active && wire_ids.iter().any(|id| highlighted.contains(id))
}

fn port_rows(ui: &mut Ui, state: &mut AppState, query: Option<&str>) {
    let ports = state.schematic.interface_ports();
    let visible: Vec<&PortSpec> = ports
        .iter()
        .filter(|port| query.is_none_or(|q| port.name.to_ascii_lowercase().contains(q)))
        .collect();

    if query.is_some() {
        nav_group_header(ui, "Ports", visible.len());
        if visible.is_empty() {
            return;
        }
    } else if visible.is_empty() {
        empty_note(
            ui,
            "No ports — this is a testbench. Place Port components to make this cell placeable.",
        );
        return;
    }

    for port in visible {
        TreeRow::new(&port.name)
            .meta(port.direction.keyword())
            .indent(1)
            .mono()
            .highlight_query(query)
            .show(ui);
    }
}

fn nav_group_header(ui: &mut Ui, title: &str, count: usize) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(title.to_uppercase())
                .font(theme::mono(10.0, FontWeight::Regular))
                .color(t.color.text_faint),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(count.to_string())
                    .font(theme::mono(10.0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
    });
    ui.add_space(2.0);
}

fn empty_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(14.0);
    ui.vertical_centered(|ui| {
        ui.set_max_width(ui.available_width() - 32.0);
        ui.label(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_faint),
        );
    });
    ui.add_space(14.0);
}

fn nav_no_match(ui: &mut Ui, state: &mut AppState, query: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(14.0);
    egui::Frame::none()
        .inner_margin(egui::Margin {
            left: 18.0,
            right: 18.0,
            top: 10.0,
            bottom: 10.0,
        })
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!("No match for \"{query}\" in this cell"))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
            ui.add_space(4.0);
            if ui
                .link(
                    egui::RichText::new("Clear search")
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.accent),
                )
                .clicked()
            {
                state.shell.nav_search.clear();
            }
        });
    ui.add_space(14.0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NavigatorSearchCounts {
    instances: usize,
    nets: usize,
    ports: usize,
}

impl NavigatorSearchCounts {
    fn is_empty(self) -> bool {
        self.instances == 0 && self.nets == 0 && self.ports == 0
    }
}

fn navigator_search_counts(state: &AppState, query: &str) -> NavigatorSearchCounts {
    let query = query.trim().to_ascii_lowercase();
    if query.is_empty() {
        return NavigatorSearchCounts {
            instances: 0,
            nets: 0,
            ports: 0,
        };
    }

    NavigatorSearchCounts {
        instances: state
            .schematic
            .components
            .iter()
            .filter(|component| component.kind != ComponentType::Port)
            .filter(|component| row_matches(component, &query))
            .count(),
        nets: design_nets_cached(state)
            .iter()
            .filter(|net| net.name.to_ascii_lowercase().contains(&query))
            .count(),
        ports: state
            .schematic
            .interface_ports()
            .iter()
            .filter(|port| port.name.to_ascii_lowercase().contains(&query))
            .count(),
    }
}

fn row_matches(component: &crate::state::Component, query: &str) -> bool {
    component.name.to_ascii_lowercase().contains(query)
        || component.value.to_ascii_lowercase().contains(query)
        || component
            .kind
            .display_name()
            .to_ascii_lowercase()
            .contains(query)
}

/// Live design nets, cached per (cell, topology) — recomputed only when
/// the schematic actually changes.
fn design_nets_cached(state: &AppState) -> Rc<Vec<crate::simulation::netlist_gen::DesignNet>> {
    thread_local! {
        #[allow(clippy::type_complexity)]
        static NET_CACHE: std::cell::RefCell<
            Option<((String, u64, u64, u64), Rc<Vec<crate::simulation::netlist_gen::DesignNet>>)>,
        > = const { std::cell::RefCell::new(None) };
    }
    let key = (
        state.workspace.active_view.key(),
        state.schematic.topology_version(),
        state.library_manager.revision(),
        schematic_buffers_signature(state),
    );
    NET_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((cached_key, nets)) = cache.as_ref()
            && *cached_key == key
        {
            return Rc::clone(nets);
        }
        let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
            &state.library_manager,
            &state.workspace.schematic_buffers,
        );
        let nets = Rc::new(crate::simulation::netlist_gen::design_nets_with_hierarchy(
            &state.schematic,
            &hierarchy,
        ));
        *cache = Some((key, Rc::clone(&nets)));
        nets
    })
}

fn schematic_buffers_signature(state: &AppState) -> u64 {
    let mut entries: Vec<_> = state
        .workspace
        .schematic_buffers
        .iter()
        .map(|(key, schematic)| (key, schematic.topology_version()))
        .collect();
    entries.sort_by(|(left, _), (right, _)| left.cmp(right));

    let mut hasher = DefaultHasher::new();
    for (key, version) in entries {
        key.hash(&mut hasher);
        version.hash(&mut hasher);
    }
    hasher.finish()
}

// ---------------------------------------------------------------------------
// Library
// ---------------------------------------------------------------------------

/// Library filter options for the component browser.
fn cell_sources(state: &AppState) -> Vec<String> {
    let mut libs: Vec<String> = vec!["All libs".to_owned(), "primitives".to_owned()];
    libs.extend(
        state
            .library_manager
            .libraries_sorted()
            .iter()
            .map(|lib| lib.name.clone()),
    );
    libs.dedup();
    libs
}

/// One placeable cell in the browser list.
pub(crate) enum CellEntry {
    /// A built-in primitive (palette entry).
    Primitive(ComponentType, &'static str),
    /// A library cell (library, cell name).
    LibraryCell(String, String),
}

impl CellEntry {
    /// Stable ref string ("prim:<label>" / "cell:<lib>/<cell>") — selection,
    /// pins and recents survive list reordering and filtering.
    fn entry_ref(&self) -> String {
        match self {
            CellEntry::Primitive(_, label) => format!("prim:{label}"),
            CellEntry::LibraryCell(lib, cell) => format!("cell:{lib}/{cell}"),
        }
    }

    fn label(&self) -> &str {
        match self {
            CellEntry::Primitive(_, label) => label,
            CellEntry::LibraryCell(_, cell) => cell,
        }
    }
}

/// Resolve a ref string back to a placeable entry, validating it still
/// exists (libraries change; pins and recents must not dangle).
fn entry_from_ref(state: &AppState, entry_ref: &str) -> Option<CellEntry> {
    if let Some(label) = entry_ref.strip_prefix("prim:") {
        for section in crate::schematic::component_palette() {
            for entry in section.entries {
                if entry.label == label {
                    return Some(CellEntry::Primitive(entry.kind, entry.label));
                }
            }
        }
        return None;
    }
    if let Some(path) = entry_ref.strip_prefix("cell:") {
        let (lib, cell) = path.split_once('/')?;
        let library = state.library_manager.get_library(lib)?;
        library.get_cell(cell)?;
        return Some(CellEntry::LibraryCell(lib.to_owned(), cell.to_owned()));
    }
    None
}

/// The palette ref for a primitive kind (recents bookkeeping at drop time).
pub(crate) fn palette_ref(kind: ComponentType) -> Option<String> {
    for section in crate::schematic::component_palette() {
        for entry in section.entries {
            if entry.kind == kind {
                return Some(format!("prim:{}", entry.label));
            }
        }
    }
    None
}

/// One collapsible browser group.
struct BrowserGroup {
    title: String,
    read_only: bool,
    entries: Vec<CellEntry>,
}

/// Browser groups for the current query + filter: pinned favorites first,
/// then palette categories, then libraries.
fn browser_groups(state: &AppState) -> Vec<BrowserGroup> {
    let query = state.shell.cell_search.trim().to_ascii_lowercase();
    let filter = &state.shell.cell_lib_filter;
    let mut groups = Vec::new();

    if query.is_empty() && !state.shell.lib_pins.is_empty() {
        let entries: Vec<CellEntry> = state
            .shell
            .lib_pins
            .iter()
            .filter_map(|pin| entry_from_ref(state, pin))
            .collect();
        if !entries.is_empty() {
            groups.push(BrowserGroup {
                title: "★ Pinned".to_owned(),
                read_only: false,
                entries,
            });
        }
    }

    if filter == "All libs" || filter == "primitives" {
        for section in crate::schematic::component_palette() {
            let entries: Vec<CellEntry> = section
                .entries
                .iter()
                .filter(|entry| {
                    query.is_empty() || entry.label.to_ascii_lowercase().contains(&query)
                })
                .map(|entry| CellEntry::Primitive(entry.kind, entry.label))
                .collect();
            if !entries.is_empty() {
                groups.push(BrowserGroup {
                    title: section.title.to_owned(),
                    read_only: false,
                    entries,
                });
            }
        }
    }
    for library in state.library_manager.libraries_sorted() {
        if filter != "All libs" && *filter != library.name {
            continue;
        }
        let entries: Vec<CellEntry> = library
            .cells_sorted()
            .iter()
            .filter(|cell| {
                query.is_empty()
                    || cell.name.to_ascii_lowercase().contains(&query)
                    || library.name.to_ascii_lowercase().contains(&query)
            })
            .map(|cell| CellEntry::LibraryCell(library.name.clone(), cell.name.clone()))
            .collect();
        if !entries.is_empty() || (query.is_empty() && !library.read_only) {
            groups.push(BrowserGroup {
                title: library.name.clone(),
                read_only: library.read_only,
                entries,
            });
        }
    }
    groups
}

fn library(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    // Stable preview card pinned above the place strip — selection changes
    // never shift the list layout.
    egui::TopBottomPanel::bottom("volta.rail.preview")
        .frame(
            egui::Frame::none()
                .fill(c.bg_panel)
                .inner_margin(egui::Margin {
                    left: 12.0,
                    right: 12.0,
                    top: 8.0,
                    bottom: 4.0,
                }),
        )
        .show_separator_line(false)
        .show_inside(ui, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.top() - 8.0,
                egui::Stroke::new(1.0, c.border),
            );
            preview_card(ui, state, symbol_library);
        });

    // Search + library filter.
    ui.add_space(8.0);
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), t.metrics.ctl_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(12.0);
            ui.spacing_mut().item_spacing.x = 6.0;
            let search_width = ui.available_width() - 96.0 - 12.0 - 6.0;
            mono_input(ui, &mut state.shell.cell_search, search_width.max(60.0));
            let libs = cell_sources(state);
            let current = state.shell.cell_lib_filter.clone();
            if let Some(index) = select(ui, "volta.cell.lib", &current, &libs, 90.0) {
                state.shell.cell_lib_filter = libs[index].clone();
            }
        },
    );
    ui.add_space(4.0);

    let groups = browser_groups(state);
    let searching = !state.shell.cell_search.trim().is_empty();
    let highlight_query = if searching {
        Some(state.shell.cell_search.trim().to_owned())
    } else {
        None
    };

    egui::ScrollArea::vertical()
        .id_salt("volta.rail.lib")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            if groups.is_empty() {
                empty_note(ui, "No cells match — clear the search or the filter.");
                return;
            }

            let mut select_ref: Option<String> = None;
            let mut place: Option<String> = None;
            let mut toggle_group: Option<String> = None;
            let mut toggle_pin: Option<String> = None;

            for group in &groups {
                let open = searching || !state.shell.lib_groups_closed.contains(&group.title);
                let meta = if group.read_only {
                    format!("ro · {}", group.entries.len())
                } else {
                    group.entries.len().to_string()
                };
                let header = TreeRow::new(&group.title)
                    .twist(open)
                    .meta(&meta)
                    .height(24.0)
                    .show(ui);
                if header.response.clicked() {
                    toggle_group = Some(group.title.clone());
                }
                if group.read_only {
                    header
                        .response
                        .on_hover_text("Read-only library — placeable, never editable");
                }
                if !open {
                    continue;
                }
                if group.entries.is_empty() {
                    TreeRow::new("No cells yet")
                        .meta("Library Manager")
                        .indent(1)
                        .dim()
                        .height(30.0)
                        .show(ui)
                        .response
                        .on_hover_text("Create a cell or import a design from the Library view");
                    continue;
                }

                for entry in &group.entries {
                    let entry_ref = entry.entry_ref();
                    let selected = state.shell.cell_selected.as_deref() == Some(&entry_ref);
                    let pinned = state.shell.lib_pins.contains(&entry_ref);

                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        ui.add_space(12.0);
                        paint_entry_thumb(ui, entry, symbol_library);
                        let meta = match entry {
                            CellEntry::Primitive(..) => if pinned { "★" } else { "" }.to_owned(),
                            CellEntry::LibraryCell(lib, _) => {
                                if pinned {
                                    format!("★ {lib}")
                                } else {
                                    lib.clone()
                                }
                            }
                        };
                        let row_width = (ui.available_width() - 26.0).max(80.0);
                        let row = ui
                            .allocate_ui_with_layout(
                                egui::vec2(row_width, 30.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    TreeRow::new(entry.label())
                                        .meta(&meta)
                                        .mono()
                                        .selected(selected)
                                        .highlight_query(highlight_query.as_deref())
                                        .height(30.0)
                                        .show(ui)
                                },
                            )
                            .inner;
                        if pin_star(ui, pinned).clicked() {
                            toggle_pin = Some(entry_ref.clone());
                        }
                        if row.response.double_clicked() {
                            place = Some(entry_ref.clone());
                        } else if row.response.clicked() {
                            select_ref = Some(entry_ref.clone());
                        }
                        row.response.context_menu(|ui| {
                            if ui
                                .button(if pinned {
                                    "Unpin from favorites"
                                } else {
                                    "Pin to favorites"
                                })
                                .clicked()
                            {
                                toggle_pin = Some(entry_ref.clone());
                                ui.close_menu();
                            }
                            if ui.button("Place").clicked() {
                                place = Some(entry_ref.clone());
                                ui.close_menu();
                            }
                        });
                    });
                }
            }

            if let Some(title) = toggle_group
                && !state.shell.lib_groups_closed.remove(&title)
            {
                state.shell.lib_groups_closed.insert(title);
            }
            if let Some(entry_ref) = toggle_pin {
                if let Some(index) = state.shell.lib_pins.iter().position(|p| *p == entry_ref) {
                    state.shell.lib_pins.remove(index);
                } else {
                    state.shell.lib_pins.push(entry_ref);
                }
            }
            if let Some(entry_ref) = select_ref {
                state.shell.cell_selected = Some(entry_ref);
            }
            if let Some(entry_ref) = place {
                state.shell.cell_selected = Some(entry_ref.clone());
                arm_ref(state, &entry_ref);
            }
            ui.add_space(8.0);
        });
}

fn pin_star(ui: &mut Ui, pinned: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let label = if pinned { "★" } else { "☆" };
    ui.add_sized(
        egui::vec2(18.0, 24.0),
        egui::Button::new(
            egui::RichText::new(label)
                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                .color(if pinned { c.accent } else { c.text_faint }),
        )
        .frame(false),
    )
    .on_hover_text(if pinned {
        "Unpin from favorites"
    } else {
        "Pin to favorites"
    })
}

/// 34x22 leading thumbnail: the real symbol for primitives, a block glyph
/// for cells.
fn paint_entry_thumb(
    ui: &mut Ui,
    entry: &CellEntry,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(34.0, 22.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect(rect, t.radius, c.bg_inset, egui::Stroke::new(1.0, c.border));
    let inner = rect.shrink(2.0);
    match entry {
        CellEntry::Primitive(kind, _) => {
            crate::schematic::view::draw_symbol_preview(
                painter,
                inner,
                *kind,
                c.symbol,
                symbol_library,
            );
        }
        CellEntry::LibraryCell(..) => {
            let block = egui::Rect::from_center_size(inner.center(), egui::vec2(14.0, 12.0));
            painter.rect_stroke(block, 0.0, egui::Stroke::new(1.0, c.symbol));
            for dy in [-3.0, 3.0] {
                painter.hline(
                    egui::Rangef::new(block.left() - 5.0, block.left()),
                    block.center().y + dy,
                    egui::Stroke::new(1.0, c.symbol),
                );
                painter.hline(
                    egui::Rangef::new(block.right(), block.right() + 5.0),
                    block.center().y + dy,
                    egui::Stroke::new(1.0, c.symbol),
                );
            }
        }
    }
}

/// Stable-height preview: symbol, identity, pins, one accent action.
fn preview_place_label() -> &'static str {
    "Place ⏎"
}

fn preview_card(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let entry = state
        .shell
        .cell_selected
        .clone()
        .and_then(|entry_ref| entry_from_ref(state, &entry_ref));
    let Some(entry) = entry else {
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 64.0), egui::Sense::hover());
        ui.painter()
            .rect(rect, t.radius, c.bg_inset, egui::Stroke::new(1.0, c.border));
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "select a component to preview",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
        ui.add_space(44.0);
        return;
    };

    // Symbol stage.
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 64.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect(rect, t.radius, c.bg_inset, egui::Stroke::new(1.0, c.border));
    let stage = rect.shrink(6.0);
    let (name, meta) = match &entry {
        CellEntry::Primitive(kind, label) => {
            crate::schematic::view::draw_symbol_preview(
                painter,
                stage,
                *kind,
                c.symbol,
                symbol_library,
            );
            (
                format!("primitives / {label}"),
                format!("symbol · {}", kind.display_name()),
            )
        }
        CellEntry::LibraryCell(lib, cell) => {
            let key = format!("{lib}/{cell}/schematic");
            let ports = state
                .workspace
                .schematic_buffers
                .get(&key)
                .map(|master| master.interface_ports())
                .unwrap_or_default();
            paint_generated_preview(painter, stage, &ports, c.symbol);
            let views = state
                .library_manager
                .get_library(lib)
                .and_then(|library| library.get_cell(cell))
                .map(|cell| {
                    cell.views_sorted()
                        .iter()
                        .map(|view| view.name.clone())
                        .collect::<Vec<_>>()
                        .join(" · ")
                })
                .unwrap_or_default();
            let meta = if ports.is_empty() {
                "no ports yet — open the cell and place Port components".to_owned()
            } else {
                let pins: Vec<&str> = ports.iter().map(|p| p.name.as_str()).collect();
                format!("pins {} · {views}", pins.join(" "))
            };
            (format!("{lib} / {cell}"), meta)
        }
    };

    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(name)
            .font(theme::mono(tokens::FS_1, FontWeight::Medium))
            .color(c.text),
    );
    ui.label(
        egui::RichText::new(meta)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(c.text_faint),
    );
    ui.add_space(6.0);

    let entry_ref = entry.entry_ref();
    let pinned = state.shell.lib_pins.contains(&entry_ref);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        // Measured, not guessed: Place + the pin chip must fill the row
        // exactly — one extra pixel here ratchets the panel wider.
        let pin_label = if pinned { "★ pinned" } else { "☆ pin" };
        let chip_width = text_width(
            ui,
            pin_label,
            &theme::mono(tokens::FS_0, FontWeight::Regular),
        ) + 18.0;
        let place_width = ui.available_width() - chip_width - 6.0;
        if Button::new(preview_place_label())
            .accent()
            .min_width(place_width.max(60.0))
            .show(ui)
            .clicked()
        {
            arm_ref(state, &entry_ref);
        }
        if chip(ui, pin_label, pinned).clicked() {
            if let Some(index) = state.shell.lib_pins.iter().position(|p| *p == entry_ref) {
                state.shell.lib_pins.remove(index);
            } else {
                state.shell.lib_pins.push(entry_ref);
            }
        }
    });
    ui.add_space(2.0);
}

/// Miniature of the generated block symbol: body, side stubs, rails.
fn paint_generated_preview(
    painter: &egui::Painter,
    rect: egui::Rect,
    ports: &[PortSpec],
    color: egui::Color32,
) {
    let stroke = egui::Stroke::new(1.2, color);
    let symbol = crate::state::generate_symbol(ports);
    let (w, h) = (symbol.width as f32, symbol.height as f32);
    let scale = (rect.width() / (w + 10.0))
        .min(rect.height() / (h + 10.0))
        .min(1.2);
    let center = rect.center();
    let to_screen = |x: f32, y: f32| egui::pos2(center.x + x * scale, center.y + y * scale);

    let hh_body = (h / 2.0 - 5.0).max(15.0);
    painter.rect_stroke(
        egui::Rect::from_min_max(to_screen(-20.0, -hh_body), to_screen(20.0, hh_body)),
        0.0,
        stroke,
    );
    for pin in &symbol.pins {
        let (px, py) = (pin.offset.x as f32, pin.offset.y as f32);
        let inner = if py.abs() > hh_body {
            (px, py.signum() * hh_body)
        } else {
            (px.signum() * 20.0, py)
        };
        painter.line_segment([to_screen(px, py), to_screen(inner.0, inner.1)], stroke);
        painter.circle_filled(to_screen(px, py), 1.4, color);
    }
}

// ---------------------------------------------------------------------------
// Placement
// ---------------------------------------------------------------------------

/// Begin placement for a browser entry.
fn place_entry(state: &mut AppState, entry: &CellEntry) {
    match entry {
        CellEntry::Primitive(kind, _) => {
            state.schematic.tool = Tool::Place(*kind);
        }
        CellEntry::LibraryCell(lib, cell) => {
            let mut binding =
                crate::state::LibraryCellInstance::new(lib.clone(), cell.clone(), "schematic");
            // Bind the master's interface at placement time: the instance
            // gets its real pin count, names and directions, and the
            // generated symbol replaces the anonymous two-pin block.
            let key = format!("{lib}/{cell}/schematic");
            if let Some(master) = state.workspace.schematic_buffers.get(&key) {
                binding.bind_interface(&master.interface_ports());
            }
            state.schematic.pending_library_cell = Some(binding);
            state.schematic.tool = Tool::Place(ComponentType::CellInstance);
        }
    }
    state.shell.view = crate::shell::WorkspaceView::Schematic;
}

/// Arm placement from a ref string (chips, typeahead, preview).
fn arm_ref(state: &mut AppState, entry_ref: &str) {
    if let Some(entry) = entry_from_ref(state, entry_ref) {
        place_entry(state, &entry);
    }
}

/// Pixel width of `text` at `font`.
fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.fonts(|f| {
        f.layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
            .size()
            .x
    })
}

/// Elide `text` with '…' to fit `budget` px at `font`.
///
/// For one-line labels inside horizontal layouts, where egui labels extend
/// instead of wrapping — an overflowing row is not just clipped: egui
/// persists the content rect as the panel's next-frame width, so a single
/// too-wide row ratchets the rail toward its maximum and fights the user's
/// resize. Every horizontal row in this panel must fit its budget.
fn fit_text(ui: &Ui, text: &str, font: &egui::FontId, budget: f32) -> String {
    if text_width(ui, text, font) <= budget {
        return text.to_owned();
    }
    let mut out = text.to_owned();
    while !out.is_empty() {
        out.pop();
        let candidate = format!("{out}…");
        if text_width(ui, &candidate, font) <= budget {
            return candidate;
        }
    }
    "…".to_owned()
}

/// Short chip label for a ref ("Resistor" → "Resistor", cells → cell name).
fn ref_chip_label(entry_ref: &str) -> &str {
    entry_ref
        .strip_prefix("prim:")
        .or_else(|| {
            entry_ref
                .strip_prefix("cell:")
                .and_then(|p| p.split('/').nth(1))
        })
        .unwrap_or(entry_ref)
}

// ---------------------------------------------------------------------------
// Place strip
// ---------------------------------------------------------------------------

/// The persistent placement surface: a command slot with typeahead, and
/// the recently placed entries as one-click chips.
fn place_strip(
    ui: &mut Ui,
    state: &mut AppState,
    _symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("PLACE")
                .font(theme::mono(10.0, FontWeight::SemiBold))
                .color(c.text_faint),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new("Shift+I")
                    .font(theme::mono(10.0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        });
    });
    ui.add_space(5.0);

    let input = mono_input(ui, &mut state.shell.place_cmd, ui.available_width());
    if state.shell.focus_cell_search {
        input.request_focus();
        state.shell.focus_cell_search = false;
    }

    // Typeahead: the rail's one elevated surface. Enter arms the active
    // match; arrows move; Escape returns to the canvas.
    let query = state.shell.place_cmd.trim().to_ascii_lowercase();
    if !query.is_empty() && input.has_focus() {
        let matches = command_matches(state, &query);
        if !matches.is_empty() {
            let count = matches.len();
            ui.input(|i| {
                if i.key_pressed(egui::Key::ArrowDown) {
                    state.shell.place_pop_index = (state.shell.place_pop_index + 1) % count;
                }
                if i.key_pressed(egui::Key::ArrowUp) {
                    state.shell.place_pop_index = (state.shell.place_pop_index + count - 1) % count;
                }
            });
            state.shell.place_pop_index = state.shell.place_pop_index.min(count - 1);

            let active = state.shell.place_pop_index;
            let mut armed: Option<String> = None;
            egui::Area::new(ui.id().with("volta.place.pop"))
                .order(egui::Order::Foreground)
                .pivot(egui::Align2::LEFT_BOTTOM)
                .fixed_pos(input.rect.left_top() - egui::vec2(0.0, 4.0))
                .show(ui.ctx(), |ui| {
                    egui::Frame::none()
                        .fill(c.bg_elevated)
                        .stroke(egui::Stroke::new(1.0, c.border_strong))
                        .rounding(t.radius_lg)
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(0.0, 4.0),
                            blur: 16.0,
                            spread: 0.0,
                            color: egui::Color32::from_black_alpha(96),
                        })
                        .inner_margin(egui::Margin::same(3.0))
                        .show(ui, |ui| {
                            ui.set_width(input.rect.width() - 6.0);
                            ui.spacing_mut().item_spacing.y = 0.0;
                            for (index, (entry_ref, label, group)) in matches.iter().enumerate() {
                                let row = TreeRow::new(label)
                                    .meta(group)
                                    .mono()
                                    .selected(index == active)
                                    .highlight_query(Some(query.as_str()))
                                    .height(24.0)
                                    .show(ui);
                                if row.response.clicked() {
                                    armed = Some(entry_ref.clone());
                                }
                            }
                        });
                });

            let commit = input.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if commit {
                armed = matches
                    .get(state.shell.place_pop_index)
                    .map(|(entry_ref, ..)| entry_ref.clone());
            }
            if let Some(entry_ref) = armed {
                arm_ref(state, &entry_ref);
                state.shell.place_cmd.clear();
                state.shell.place_pop_index = 0;
            }
        }
    } else if query.is_empty() {
        state.shell.place_pop_index = 0;
    }

    // Recents — the five-things-you-actually-place row.
    if !state.shell.lib_recents.is_empty() {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            // Chips render until the row is full, never past it — a chip
            // that doesn't fit is dropped, not clipped (an overflowing row
            // ratchets the panel wider).
            let chip_font = theme::mono(tokens::FS_0, FontWeight::Regular);
            let mut budget = ui.available_width();
            let recents = state.shell.lib_recents.clone();
            for entry_ref in recents.iter().take(6) {
                let chip_width = text_width(ui, ref_chip_label(entry_ref), &chip_font) + 18.0;
                if chip_width > budget {
                    break;
                }
                budget -= chip_width + 4.0;
                let armed = match (&state.schematic.tool, entry_from_ref(state, entry_ref)) {
                    (Tool::Place(active), Some(CellEntry::Primitive(kind, _))) => {
                        *active == kind && kind != ComponentType::CellInstance
                    }
                    (
                        Tool::Place(ComponentType::CellInstance),
                        Some(CellEntry::LibraryCell(lib, cell)),
                    ) => state
                        .schematic
                        .pending_library_cell
                        .as_ref()
                        .is_some_and(|binding| binding.library == lib && binding.cell == cell),
                    _ => false,
                };
                if chip(ui, ref_chip_label(entry_ref), armed)
                    .on_hover_text(format!("Place {}", ref_chip_label(entry_ref)))
                    .clicked()
                {
                    if armed {
                        state.schematic.tool = Tool::Select;
                    } else {
                        arm_ref(state, entry_ref);
                    }
                }
            }
        });
    }
}

/// Typeahead matches: palette + project/vendor cells, prefix matches
/// first, capped at six. The open cell is excluded — a cell cannot be
/// placed inside itself.
fn command_matches(state: &AppState, query: &str) -> Vec<(String, String, String)> {
    let active_cell = state.workspace.active_view.cell.to_ascii_lowercase();
    let mut matches: Vec<(String, String, String, bool)> = Vec::new();

    for section in crate::schematic::component_palette() {
        for entry in section.entries {
            let label_lower = entry.label.to_ascii_lowercase();
            if label_lower.contains(query) {
                matches.push((
                    format!("prim:{}", entry.label),
                    entry.label.to_owned(),
                    section.title.to_owned(),
                    label_lower.starts_with(query),
                ));
            }
        }
    }
    for library in state.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            let cell_lower = cell.name.to_ascii_lowercase();
            if cell_lower == active_cell {
                continue;
            }
            if cell_lower.contains(query) || library.name.to_ascii_lowercase().contains(query) {
                matches.push((
                    format!("cell:{}/{}", library.name, cell.name),
                    cell.name.clone(),
                    library.name.clone(),
                    cell_lower.starts_with(query),
                ));
            }
        }
    }

    matches.sort_by(|a, b| (!a.3, a.1.to_ascii_lowercase()).cmp(&(!b.3, b.1.to_ascii_lowercase())));
    matches
        .into_iter()
        .take(6)
        .map(|(entry_ref, label, group, _)| (entry_ref, label, group))
        .collect()
}

// ---------------------------------------------------------------------------
// Right panel — inspector
// ---------------------------------------------------------------------------

/// Commit the in-flight inspector edit session as one undo entry.
fn commit_inspector_edit(state: &mut AppState) {
    if let Some(edit) = state.shell.inspector_edit.take() {
        state
            .schematic
            .commit_undo_from(edit.before, "edit properties");
    }
}

/// Render the schematic context's right panel (instance inspector).
pub fn right(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Inspector", None);

    let single = state.schematic.selection.single_component();

    // The edit session ends when the inspected component changes.
    if state
        .shell
        .inspector_edit
        .as_ref()
        .is_some_and(|edit| Some(edit.component_id) != single)
    {
        commit_inspector_edit(state);
    }

    let Some(component_id) = single else {
        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new(if state.schematic.selection.is_empty() {
                    "Select an instance to inspect"
                } else {
                    "Multiple instances selected"
                })
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_faint),
            );
        });
        return;
    };

    let Some(component_idx) = state
        .schematic
        .components
        .iter()
        .position(|component| component.id == component_id)
    else {
        return;
    };

    // Editable snapshot — written back through an undo-tracked operation when
    // any field changes.
    let snapshot = state.schematic.components[component_idx].clone();
    let mut name = snapshot.name.clone();
    let mut value = snapshot.value.clone();
    let mut params = snapshot.params.clone();

    ui.add_space(2.0);
    let mut fields_focused = false;
    egui::Frame::none()
        .inner_margin(egui::Margin {
            left: 12.0,
            right: 12.0,
            top: 0.0,
            bottom: 12.0,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;

            fields_focused |= input_row(ui, "Instance", &mut name).has_focus();
            input_row_readonly(ui, "Type", snapshot.kind.display_name());
            let value_label =
                crate::properties::property_bridge::get_value_display_name(snapshot.kind);
            fields_focused |= input_row(ui, value_label, &mut value).has_focus();
            fields_focused |= input_row(ui, "Params", &mut params).has_focus();

            // Terminal connectivity.
            let terminals = snapshot.terminal_positions();
            if !terminals.is_empty() {
                section_header(ui, "Nets", None);
                for (terminal, position) in &terminals {
                    let net = state
                        .schematic
                        .net_mapping
                        .get(position)
                        .cloned()
                        .unwrap_or_else(|| "—".to_owned());
                    kv_row(ui, terminal, &net);
                }
            }
        });

    // Apply edits live so typing reflects immediately, but capture the
    // pre-edit snapshot once per session — every keystroke used to deep-
    // clone the whole design twice and mint its own undo entry.
    let changed = name != snapshot.name || value != snapshot.value || params != snapshot.params;
    if changed {
        if state.shell.inspector_edit.is_none() {
            state.shell.inspector_edit = Some(crate::shell::state::InspectorEdit {
                component_id,
                before: crate::state::SchematicSnapshot::capture(&state.schematic),
            });
        }
        let component = &mut state.schematic.components[component_idx];
        component.name = name;
        component.value = value;
        component.params = params;
        state.schematic.is_dirty = true;
    }

    // The session also ends when focus leaves the fields (click away,
    // Enter, Tab out of the panel).
    if !fields_focused && state.shell.inspector_edit.is_some() {
        commit_inspector_edit(state);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_place_label_matches_design() {
        assert_eq!(preview_place_label(), "Place \u{23ce}");
    }

    #[test]
    fn net_probe_dot_tracks_active_highlighted_net() {
        let mut highlighted = HashSet::new();
        highlighted.insert(42);

        assert!(net_row_is_probed(&[42], &highlighted, true));
        assert!(net_row_is_probed(&[7, 42], &highlighted, true));
        assert!(!net_row_is_probed(&[42], &highlighted, false));
        assert!(!net_row_is_probed(&[7], &highlighted, true));
        assert!(!net_row_is_probed(&[], &highlighted, true));
    }

    #[test]
    fn navigator_search_counts_all_visible_groups() {
        let mut state = AppState::default();
        state.schematic.components.clear();
        state.schematic.components.push(
            crate::state::Component::new(
                1,
                ComponentType::Resistor,
                crate::state::Point::new(0, 0),
            )
            .with_name_value("RLOAD", "10k"),
        );
        state.schematic.components.push(
            crate::state::Component::new(2, ComponentType::Port, crate::state::Point::new(10, 0))
                .with_name_value("P1", "LOAD_OUT"),
        );

        let counts = navigator_search_counts(&state, "load");
        assert_eq!(counts.instances, 1);
        assert_eq!(counts.nets, 1);
        assert_eq!(counts.ports, 1);
        assert!(!counts.is_empty());

        let empty = navigator_search_counts(&state, "definitely-missing");
        assert!(empty.is_empty());
    }

    #[test]
    fn browser_groups_include_empty_editable_libraries_when_not_searching() {
        let mut state = AppState::default();
        let mut library = crate::state::Library::new("empty_project");
        library
            .metadata
            .insert("role".to_string(), "project".to_string());
        state.library_manager.add_library(library);

        let groups = browser_groups(&state);
        assert!(
            groups
                .iter()
                .any(|group| group.title == "empty_project" && group.entries.is_empty())
        );

        state.shell.cell_search = "res".to_string();
        let groups = browser_groups(&state);
        assert!(!groups.iter().any(|group| group.title == "empty_project"));
    }

    #[test]
    fn master_status_distinguishes_missing_and_not_open_states() {
        let mut state = AppState::default();
        let mut binding = crate::state::LibraryCellInstance::new("status_lib", "amp", "schematic");

        assert_eq!(
            master_status(&state, &binding),
            MasterStatus::MissingLibrary
        );

        state
            .library_manager
            .add_library(crate::state::Library::new("status_lib"));
        assert_eq!(master_status(&state, &binding), MasterStatus::MissingCell);

        let mut cell = crate::state::Cell::new("amp");
        cell.add_view(crate::state::View::new(
            "symbol",
            crate::state::ViewType::Symbol,
        ));
        state
            .library_manager
            .get_library_mut("status_lib")
            .expect("library exists")
            .add_cell(cell);
        assert_eq!(master_status(&state, &binding), MasterStatus::MissingView);

        state
            .library_manager
            .get_library_mut("status_lib")
            .expect("library exists")
            .get_cell_mut("amp")
            .expect("cell exists")
            .add_view(crate::state::View::new(
                "schematic",
                crate::state::ViewType::Schematic,
            ));
        assert_eq!(master_status(&state, &binding), MasterStatus::NotOpen);

        state.workspace.schematic_buffers.insert(
            "status_lib/amp/schematic".to_string(),
            crate::state::SchematicState::default(),
        );
        assert_eq!(master_status(&state, &binding), MasterStatus::Open);

        binding.view = "layout".to_string();
        assert_eq!(master_status(&state, &binding), MasterStatus::MissingView);
    }

    /// egui persists the CONTENT rect as a panel's next-frame width, so any
    /// row wider than the rail ratchets the panel toward its maximum and
    /// fights the user's resize. Lay the rail out at the minimum panel
    /// width with worst-case state and assert nothing claims extra width.
    #[test]
    fn rail_content_never_overflows_panel_width() {
        const WIDTH: f32 = 232.0; // panels::PANEL_MIN

        let ctx = egui::Context::default();
        crate::ui::fonts::install(&ctx);

        let mut state = AppState::default();
        // Worst case: six recents, a pinned selection in the preview, a
        // deep occurrence path, and long library/cell names everywhere.
        state.shell.lib_recents = vec![
            "prim:Transmission Line".into(),
            "prim:Saturable Inductor".into(),
            "prim:Capacitor".into(),
            "prim:Resistor".into(),
            "prim:Ground".into(),
            "prim:NMOS".into(),
        ];
        state.shell.lib_pins = state.shell.lib_recents.clone();
        state.shell.cell_selected = Some("prim:Transmission Line".into());
        state.workspace.active_view.library = "a_long_library_name".into();
        state.workspace.active_view.cell = "an_extremely_long_cell_name_that_cannot_fit".into();
        for index in 0..6 {
            state.workspace.descend_into(
                format!("XLONGINSTANCENAME{index}"),
                CellViewRef::new("work", format!("deep_subcell_{index}"), "schematic"),
                crate::state::ViewType::Schematic,
            );
        }

        for tab in [RailTab::Navigator, RailTab::Library] {
            state.shell.rail_tab = tab;
            let mut content_width: f32 = 0.0;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let rect =
                        egui::Rect::from_min_size(ui.max_rect().min, egui::vec2(WIDTH, 700.0));
                    let mut rail = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(rect)
                            .layout(egui::Layout::top_down(egui::Align::Min)),
                    );
                    left(&mut rail, &mut state, None);
                    content_width = rail.min_rect().width();
                });
            });
            assert!(
                content_width <= WIDTH + 0.5,
                "{tab:?} rail content claims {content_width}px in a {WIDTH}px panel — \
                 this overflow ratchets the panel to its maximum width"
            );
        }
    }
}
