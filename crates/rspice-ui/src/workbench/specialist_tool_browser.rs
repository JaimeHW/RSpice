//! Canonical specialist-workspace discovery manager.
//!
//! The mockup makes every specialist owner searchable without duplicating
//! documents. This implementation projects the governed 41-workspace catalog,
//! personal pins/favorites, device-local recents, and the fail-closed runtime
//! availability registry. A card can only navigate when its exact surface has
//! a registered executor; catalog metadata never turns a design into a feature.

use std::cmp::Ordering;

use egui::{
    Align, Align2, Color32, Frame, Id, Margin, Rect, Response, RichText, ScrollArea, Sense, Stroke,
    Ui, Vec2, pos2, vec2,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::common::{AppState, RSpiceApp, app::ConsoleMessage};
use crate::ui::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize},
};

use super::{
    RouteTransitionSource, SurfaceExecutionAvailability, SurfaceId, SurfaceRoute,
    design_system::WorkbenchIcon,
    feature_availability_data::{SPECIALIST_WORKSPACE_ROWS, SpecialistWorkspaceRow},
    state::{EngineeringProfile, SpecialistToolBrowserState, SpecialistToolFilter, Workspace},
};

const DESCRIPTION: &str = "Find every canonical specialist engineering owner without duplicating its document or overstating runtime availability.";
const PURPOSE_COPY: &str = "Every specialist workspace remains discoverable on tablet and phone. Opening a tool preserves its canonical owner and composes dense work into focused steps, drawers, exact-entry panels, and structured alternatives without disabling supported mutations.";
const OWNERSHIP_COPY: &str = "Favorites and pins are personal discovery preferences; recent tools are device-local history. They never duplicate documents or change canonical ownership. Profile-hidden tools remain inspectable because projects and deep links are portable. Optional target continuation preserves context but never replaces an otherwise supported phone task. Engine, connector, preview, and sign-off eligibility remain separate from workspace visibility.";
const SEARCH_PLACEHOLDER: &str = "Find AMS, RF, layout, extraction, SI/PI, model, automation…";
const TOOLBAR_PADDING_X: i8 = 10;
const TOOLBAR_PADDING_Y: i8 = 9;
const FILTER_PADDING_X: i8 = 10;
const FILTER_PADDING_Y: i8 = 7;
const CARD_HEIGHT: f32 = 58.0;
const TOUCH_CARD_HEIGHT: f32 = 64.0;
const CARD_ICON_SIDE: f32 = 28.0;
const CARD_ACTION_SIDE: f32 = 28.0;
const TOUCH_ACTION_SIDE: f32 = 44.0;
const GRID_MAX_HEIGHT: f32 = 560.0;
const GRID_VIEWPORT_RATIO: f32 = 0.58;
const COMPACT_BREAKPOINT: f32 = 820.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrowserAction {
    Unavailable(SurfaceId),
    ToggleFavorite(SurfaceId),
    TogglePin(SurfaceId),
    Open(SurfaceId),
}

pub(crate) fn open(app: &mut RSpiceApp) {
    let route = SurfaceRoute::surface(SurfaceId::SpecialistToolBrowser);
    if let Err(error) = app
        .state
        .workbench
        .navigate(route, RouteTransitionSource::User)
    {
        app.state
            .push_user_message(ConsoleMessage::warning(error.to_string()));
    }
}

pub(crate) fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.workbench.current_route().surface_id() != SurfaceId::SpecialistToolBrowser {
        return;
    }

    let profile = app.state.workbench.engineering_profile;
    let mut requested_action = None;
    let choice = Dialog::new(
        "WORKSPACES · DISCIPLINE TOOLS · DEVICE TASK MODE",
        "Specialist tool browser",
        "Close",
    )
    .description(DESCRIPTION)
    .size(DialogSize::SpecialistToolBrowser)
    .flush_body()
    .primary_on_enter(false)
    .initial_focus(DialogInitialFocus::BodyControl)
    .show_with_initial_body_focus(ctx, |ui| {
        let browser = &mut app.state.workbench.specialist_tool_browser;
        purpose_banner(ui);
        let search_id = toolbar(ui, browser, profile);
        filter_bar(ui, browser);
        let rows = visible_rows(browser, profile);
        tool_grid(ui, browser, profile, &rows, &mut requested_action);
        ownership_banner(ui);
        Some(search_id)
    });

    if let Some(action) = requested_action {
        execute_action(app, action);
    }
    if matches!(
        choice,
        DialogChoice::Primary | DialogChoice::Cancelled | DialogChoice::Ghost
    ) {
        close_to_source(&mut app.state);
    }
}

fn execute_action(app: &mut RSpiceApp, action: BrowserAction) {
    match action {
        BrowserAction::Unavailable(surface) => {
            let reason = match super::surface_availability(surface) {
                SurfaceExecutionAvailability::Unavailable { reason } => reason,
                SurfaceExecutionAvailability::Available { .. } => {
                    "This specialist workspace is not currently available."
                }
            };
            app.state.push_user_message(ConsoleMessage::warning(format!(
                "{} is unavailable: {reason}",
                surface.metadata().label
            )));
        }
        BrowserAction::ToggleFavorite(surface) => app
            .state
            .workbench
            .specialist_tool_browser
            .toggle_favorite(surface),
        BrowserAction::TogglePin(surface) => app
            .state
            .workbench
            .specialist_tool_browser
            .toggle_pin(surface),
        BrowserAction::Open(surface) => {
            let route = SurfaceRoute::surface(surface);
            match app
                .state
                .workbench
                .navigate(route, RouteTransitionSource::User)
            {
                Ok(_) => app
                    .state
                    .workbench
                    .specialist_tool_browser
                    .record_recent(surface),
                Err(error) => {
                    app.state
                        .push_user_message(ConsoleMessage::warning(error.to_string()));
                }
            }
        }
    }
}

fn close_to_source(state: &mut AppState) {
    if state
        .workbench
        .navigate_back(RouteTransitionSource::User)
        .is_some()
    {
        return;
    }
    let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(state.workbench.workspace));
    if let Err(error) = state
        .workbench
        .replace_route(fallback, RouteTransitionSource::User)
    {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Could not close Specialist tool browser: {error}"
        )));
    }
}

fn purpose_banner(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let response = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.set_min_height(19.0);
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let (icon_rect, _) = ui.allocate_exact_size(vec2(15.0, 15.0), Sense::hover());
                WorkbenchIcon::Info.paint(
                    ui.painter(),
                    Rect::from_center_size(icon_rect.center(), Vec2::splat(13.0)),
                    t.color.info,
                );
                ui.add(
                    egui::Label::new(
                        RichText::new(PURPOSE_COPY)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            });
        });
    dashed_hline(
        ui,
        response.response.rect.bottom(),
        response.response.rect.x_range(),
        t.color.border_strong,
    );
}

fn toolbar(
    ui: &mut Ui,
    browser: &mut SpecialistToolBrowserState,
    profile: EngineeringProfile,
) -> Id {
    let t = Tokens::get(ui.ctx());
    let frame = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(TOOLBAR_PADDING_X, TOOLBAR_PADDING_Y))
        .show(ui, |ui| {
            let count = visible_rows(browser, profile).len();
            let mut search_id = Id::NULL;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let count_width = 104.0_f32.min(ui.available_width() * 0.34);
                let field_width = (ui.available_width() - count_width - 8.0).max(1.0);
                let response = search_field(
                    ui,
                    &mut browser.query,
                    field_width,
                    std::mem::take(&mut browser.focus_search),
                );
                search_id = response.id;
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    count_label(ui, count);
                });
            });
            search_id
        });
    horizontal_rule(ui, frame.response.rect.bottom(), t.color.border);
    frame.inner
}

fn search_field(ui: &mut Ui, query: &mut String, width: f32, request_focus: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.ctl_h;
    let (rect, outer) = ui.allocate_exact_size(vec2(width.max(1.0), height), Sense::click());
    ui.painter().rect_filled(rect, t.radius, t.color.bg_inset);
    let content = Rect::from_min_max(
        pos2(rect.left() + 30.0, rect.top()),
        pos2((rect.right() - 8.0).max(rect.left() + 30.0), rect.bottom()),
    );
    let mut field_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(egui::Layout::left_to_right(Align::Center)),
    );
    let edit = field_ui.add_sized(
        content.size(),
        egui::TextEdit::singleline(query)
            .id_salt("workbench.specialist-tool-query")
            .hint_text(SEARCH_PLACEHOLDER)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .margin(Margin::ZERO)
            .vertical_align(Align::Center)
            .frame(Frame::NONE),
    );
    if request_focus || outer.clicked() {
        edit.request_focus();
    }
    let focused = request_focus || edit.has_focus();
    ui.painter().rect_stroke(
        rect,
        t.radius,
        Stroke::new(
            1.0,
            if focused {
                t.color.accent
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Search.paint(
        ui.painter(),
        Rect::from_center_size(pos2(rect.left() + 15.0, rect.center().y), Vec2::splat(15.0)),
        t.color.text_faint,
    );
    let response = outer.union(edit);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            "Search specialist tools",
        )
    });
    response
}

fn count_label(ui: &mut Ui, visible: usize) {
    let t = Tokens::get(ui.ctx());
    let response = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        let (dot, _) = ui.allocate_exact_size(Vec2::splat(6.0), Sense::hover());
        ui.painter()
            .circle_filled(dot.center(), 2.5, t.color.text_faint);
        ui.label(
            RichText::new(format!(
                "{visible} of {} tools",
                SPECIALIST_WORKSPACE_ROWS.len()
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
        );
    });
    response.response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            true,
            format!("{visible} of {} tools", SPECIALIST_WORKSPACE_ROWS.len()),
        )
    });
}

fn filter_bar(ui: &mut Ui, browser: &mut SpecialistToolBrowserState) {
    let t = Tokens::get(ui.ctx());
    let frame = Frame::NONE
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(FILTER_PADDING_X, FILTER_PADDING_Y))
        .show(ui, |ui| {
            let group = ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                for (index, filter) in SpecialistToolFilter::ALL.into_iter().enumerate() {
                    let selected = browser.filter == filter;
                    let response = filter_button(
                        ui,
                        filter.label(),
                        selected,
                        index + 1 == SpecialistToolFilter::ALL.len(),
                    );
                    if response.clicked() {
                        browser.filter = filter;
                        response.request_focus();
                    }
                }
            });
            ui.painter().rect_stroke(
                group.response.rect,
                t.radius,
                Stroke::new(1.0, t.color.border),
                egui::StrokeKind::Inside,
            );
            group.response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Other, true, "Specialist tool filter")
            });
        });
    horizontal_rule(ui, frame.response.rect.bottom(), t.color.border);
}

fn filter_button(ui: &mut Ui, label: &str, selected: bool, last: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let galley =
        ui.fonts_mut(|fonts| fonts.layout_no_wrap(label.to_owned(), font.clone(), t.color.text));
    let width = (galley.size().x + 16.0).max(54.0);
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, t.metrics.ctl_h.max(28.0)), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::RadioButton,
            ui.is_enabled(),
            selected,
            label,
        )
    });
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
    if !last {
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    ui.painter().galley(
        pos2(
            rect.center().x - galley.size().x * 0.5,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        t.color.text,
    );
    if selected {
        ui.painter().rect_filled(
            Rect::from_min_max(
                pos2(rect.left() + 2.0, rect.bottom() - 2.0),
                pos2(rect.right() - 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

fn visible_rows(
    browser: &SpecialistToolBrowserState,
    profile: EngineeringProfile,
) -> Vec<&'static SpecialistWorkspaceRow> {
    let query = browser.query.trim().to_lowercase();
    let mut rows = SPECIALIST_WORKSPACE_ROWS
        .iter()
        .filter(|row| filter_includes(browser, profile, row))
        .filter(|row| query_matches(row, &query))
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| compare_rows(browser, left, right));
    rows
}

fn filter_includes(
    browser: &SpecialistToolBrowserState,
    profile: EngineeringProfile,
    row: &SpecialistWorkspaceRow,
) -> bool {
    match browser.filter {
        SpecialistToolFilter::All => true,
        SpecialistToolFilter::ActiveProfile => row.shown_in_profile(profile),
        SpecialistToolFilter::Pinned => browser.is_pinned(row.surface_id),
        SpecialistToolFilter::Favorites => browser.is_favorite(row.surface_id),
        SpecialistToolFilter::Recent => browser.is_recent(row.surface_id),
    }
}

fn query_matches(row: &SpecialistWorkspaceRow, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let metadata = row.surface_id.metadata();
    let haystack = format!(
        "{} {} {} {} {} {} {} {}",
        metadata.stable_id,
        metadata.label,
        row.owner.owner_label(),
        row.purpose,
        row.primary_module_id,
        row.module_availability.as_str(),
        row.tier.as_str(),
        row.engine_service_boundary(),
    )
    .to_lowercase();
    query.split_whitespace().all(|term| haystack.contains(term))
}

fn compare_rows(
    browser: &SpecialistToolBrowserState,
    left: &SpecialistWorkspaceRow,
    right: &SpecialistWorkspaceRow,
) -> Ordering {
    let left_rank = discovery_rank(browser, left.surface_id);
    let right_rank = discovery_rank(browser, right.surface_id);
    left_rank
        .cmp(&right_rank)
        .then_with(|| {
            recent_rank(browser, left.surface_id).cmp(&recent_rank(browser, right.surface_id))
        })
        .then_with(|| left.owner.owner_label().cmp(right.owner.owner_label()))
        .then_with(|| left.label().cmp(right.label()))
}

fn discovery_rank(browser: &SpecialistToolBrowserState, surface: SurfaceId) -> u8 {
    if browser.is_pinned(surface) {
        0
    } else if browser.is_favorite(surface) {
        1
    } else if browser.is_recent(surface) {
        2
    } else {
        3
    }
}

fn recent_rank(browser: &SpecialistToolBrowserState, surface: SurfaceId) -> usize {
    browser
        .recents
        .iter()
        .position(|candidate| *candidate == surface)
        .unwrap_or(usize::MAX)
}

fn tool_grid(
    ui: &mut Ui,
    browser: &SpecialistToolBrowserState,
    profile: EngineeringProfile,
    rows: &[&SpecialistWorkspaceRow],
    requested_action: &mut Option<BrowserAction>,
) {
    let t = Tokens::get(ui.ctx());
    if rows.is_empty() {
        empty_state(ui);
        return;
    }
    let compact = compact_layout(ui);
    let columns = if compact { 1 } else { 2 };
    let row_height = if compact || t.metrics.ctl_h >= 44.0 {
        TOUCH_CARD_HEIGHT
    } else {
        CARD_HEIGHT
    };
    let render = |ui: &mut Ui, requested_action: &mut Option<BrowserAction>| {
        ui.set_width(ui.available_width());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        let card_width = ui.available_width() / columns as f32;
        for chunk in rows.chunks(columns) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                for (column, row) in chunk.iter().enumerate() {
                    if let Some(action) = tool_card(
                        ui,
                        browser,
                        profile,
                        row,
                        card_width,
                        row_height,
                        columns == 2 && column == 0,
                    ) {
                        *requested_action = Some(action);
                    }
                }
                if chunk.len() < columns {
                    ui.allocate_space(vec2(card_width, row_height));
                }
            });
        }
    };
    if compact {
        render(ui, requested_action);
    } else {
        let visible_row_count = rows.len().div_ceil(columns);
        let viewport_cap = ui.ctx().content_rect().height() * GRID_VIEWPORT_RATIO;
        let desired_height = (visible_row_count as f32 * row_height).clamp(
            row_height,
            GRID_MAX_HEIGHT.min(viewport_cap.max(row_height)),
        );
        ScrollArea::vertical()
            .id_salt("specialist-browser-grid")
            .max_height(desired_height)
            .min_scrolled_height(desired_height)
            .auto_shrink([false, false])
            .show(ui, |ui| render(ui, requested_action));
    }
    horizontal_rule(ui, ui.cursor().top(), t.color.border);
}

fn compact_layout(ui: &Ui) -> bool {
    ui.ctx().content_rect().width() <= COMPACT_BREAKPOINT
        || ui.ctx().input(|input| input.has_touch_screen())
}

fn tool_card(
    ui: &mut Ui,
    browser: &SpecialistToolBrowserState,
    profile: EngineeringProfile,
    row: &SpecialistWorkspaceRow,
    width: f32,
    height: f32,
    draw_right_divider: bool,
) -> Option<BrowserAction> {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let action_side = if height >= TOUCH_CARD_HEIGHT {
        TOUCH_ACTION_SIDE
    } else {
        CARD_ACTION_SIDE
    };
    let actions_width = if action_side >= TOUCH_ACTION_SIDE {
        108.0
    } else {
        70.0
    };
    let main_rect = rect;
    let main = ui.interact(
        main_rect,
        ui.id().with(("specialist-tool", row.surface_id)),
        Sense::click(),
    );
    let availability = row.runtime_availability();
    let available = availability.can_open();
    let fill = if main.hovered() {
        t.color.bg_hover
    } else {
        t.color.bg_inset
    };
    ui.painter().rect_filled(rect, 0.0, fill);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    if draw_right_divider {
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }

    let icon_rect = Rect::from_center_size(
        pos2(rect.left() + 24.0, rect.center().y),
        Vec2::splat(CARD_ICON_SIDE),
    );
    owner_icon(row.owner).paint(
        ui.painter(),
        Rect::from_center_size(icon_rect.center(), Vec2::splat(18.0)),
        t.color.accent,
    );
    let content_right = rect.right() - actions_width;
    let status_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let profile_copy = if row.shown_in_profile(profile) {
        "active profile"
    } else {
        "available by deep link"
    };
    let status_text_width = ui
        .painter()
        .layout_no_wrap(
            profile_copy.to_owned(),
            status_font.clone(),
            t.color.text_dim,
        )
        .size()
        .x;
    let status_width =
        (status_text_width + 11.0).min((content_right - icon_rect.right() - 48.0).max(62.0));
    let status_rect = Rect::from_min_max(
        pos2(content_right - status_width, rect.top() + 8.0),
        pos2(content_right, rect.bottom() - 8.0),
    );
    let text_rect = Rect::from_min_max(
        pos2(icon_rect.right() + 8.0, rect.top() + 8.0),
        pos2(status_rect.left() - 8.0, rect.bottom() - 8.0),
    );
    let painter = ui.painter().with_clip_rect(text_rect);
    let title_font = theme::sans(tokens::FS_1, FontWeight::SemiBold);
    let detail_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let title = elide_to_width(&painter, row.label(), &title_font, text_rect.width());
    let detail = elide_to_width(
        &painter,
        &format!(
            "{} · {}",
            row.owner.owner_label(),
            phone_mode(row.surface_id)
        ),
        &detail_font,
        text_rect.width(),
    );
    painter.text(
        text_rect.left_top(),
        Align2::LEFT_TOP,
        title,
        title_font,
        t.color.text,
    );
    painter.text(
        pos2(text_rect.left(), text_rect.top() + 20.0),
        Align2::LEFT_TOP,
        detail,
        detail_font,
        t.color.text_dim,
    );
    let dot_center = pos2(status_rect.left() + 2.5, status_rect.center().y);
    ui.painter().circle_filled(
        dot_center,
        2.5,
        if row.shown_in_profile(profile) {
            t.color.ok
        } else {
            t.color.text_faint
        },
    );
    let status = elide_to_width(
        ui.painter(),
        profile_copy,
        &status_font,
        (status_rect.width() - 11.0).max(1.0),
    );
    ui.painter().text(
        pos2(status_rect.left() + 11.0, status_rect.center().y),
        Align2::LEFT_CENTER,
        status,
        status_font,
        t.color.text_dim,
    );

    main.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            if available {
                format!("Open {}", row.label())
            } else {
                format!("Inspect availability for {}", row.label())
            },
        )
    });
    ui.ctx().accesskit_node_builder(main.id, |node| {
        node.set_description(format!(
            "{}; {}; {}; {}",
            row.purpose,
            row.owner.owner_label(),
            row.engine_service_boundary(),
            availability_copy(availability)
        ));
    });
    theme::paint_focus_ring(ui, &main, main_rect.shrink(1.0));
    let main = main.on_hover_text(availability_copy(availability));

    let favorite_rect = Rect::from_center_size(
        pos2(
            rect.right() - 7.0 - action_side * 1.5 - 2.0,
            rect.center().y,
        ),
        Vec2::splat(action_side),
    );
    let pin_rect = Rect::from_center_size(
        pos2(rect.right() - 7.0 - action_side * 0.5, rect.center().y),
        Vec2::splat(action_side),
    );
    let favorite = card_action(
        ui,
        favorite_rect,
        ("specialist-favorite", row.surface_id),
        if browser.is_favorite(row.surface_id) {
            "★"
        } else {
            "☆"
        },
        if browser.is_favorite(row.surface_id) {
            format!("Remove from favorites: {}", row.label())
        } else {
            format!("Add to favorites: {}", row.label())
        },
        browser.is_favorite(row.surface_id),
    );
    let pin = card_action(
        ui,
        pin_rect,
        ("specialist-pin", row.surface_id),
        if browser.is_pinned(row.surface_id) {
            "●"
        } else {
            "○"
        },
        if browser.is_pinned(row.surface_id) {
            format!("Unpin: {}", row.label())
        } else {
            format!("Pin: {}", row.label())
        },
        browser.is_pinned(row.surface_id),
    );

    if favorite.clicked() {
        Some(BrowserAction::ToggleFavorite(row.surface_id))
    } else if pin.clicked() {
        Some(BrowserAction::TogglePin(row.surface_id))
    } else if main.clicked() && available {
        Some(BrowserAction::Open(row.surface_id))
    } else if main.clicked() {
        Some(BrowserAction::Unavailable(row.surface_id))
    } else {
        None
    }
}

fn card_action(
    ui: &mut Ui,
    rect: Rect,
    salt: (&'static str, SurfaceId),
    glyph: &str,
    label: String,
    pressed: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let response = ui.interact(rect, ui.id().with(salt), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            pressed,
            label.clone(),
        )
    });
    if response.hovered() || response.has_focus() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        glyph,
        theme::sans(tokens::FS_2, FontWeight::Regular),
        if pressed {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(label)
}

fn availability_copy(availability: SurfaceExecutionAvailability) -> &'static str {
    match availability {
        SurfaceExecutionAvailability::Available {
            evidence_boundary, ..
        } => evidence_boundary,
        SurfaceExecutionAvailability::Unavailable { reason } => reason,
    }
}

fn ownership_banner(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let response = Frame::NONE
        .fill(t.color.bg_inset)
        .outer_margin(Margin::same(8))
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    RichText::new(OWNERSHIP_COPY)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    dashed_rect(ui, response.response.rect, t.radius, t.color.border_strong);
}

fn empty_state(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let frame = Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(Margin::same(24))
        .show(ui, |ui| {
            ui.set_min_height(92.0);
            ui.vertical_centered(|ui| {
                ui.add_space(18.0);
                ui.label(
                    RichText::new("No matching tools")
                        .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    RichText::new(
                        "Clear the search or choose a broader filter. Profile-hidden tools remain available under All.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
        });
    horizontal_rule(ui, frame.response.rect.bottom(), t.color.border);
}

fn owner_icon(owner: Workspace) -> WorkbenchIcon {
    match owner {
        Workspace::Project => WorkbenchIcon::Project,
        Workspace::Design => WorkbenchIcon::Design,
        Workspace::Simulate => WorkbenchIcon::Simulate,
        Workspace::Results => WorkbenchIcon::Results,
        Workspace::Verify => WorkbenchIcon::Verify,
        Workspace::Models => WorkbenchIcon::Models,
        Workspace::Netlist => WorkbenchIcon::Netlist,
    }
}

fn phone_mode(surface: SurfaceId) -> &'static str {
    if matches!(
        surface,
        SurfaceId::LayoutWorkbench
            | SurfaceId::LvsPexWorkbench
            | SurfaceId::EmWorkbench
            | SurfaceId::SiPiWorkbench
            | SurfaceId::PcbWorkbench
            | SurfaceId::PcellDesigner
            | SurfaceId::ModelExtraction
            | SurfaceId::ModelCorrelation
            | SurfaceId::PdkTechnologyAdmin
            | SurfaceId::LibraryCharacterization
            | SurfaceId::SolverQualificationCenter
            | SurfaceId::DeckCompatibilityCenter
            | SurfaceId::ProtectedIpCenter
            | SurfaceId::LibraryCellviewManager
            | SurfaceId::QuantumWorkbench
    ) {
        "adaptive authoring · focused steps"
    } else {
        "full compact task · author · review · monitor"
    }
}

fn horizontal_rule(ui: &Ui, y: f32, color: Color32) {
    ui.painter()
        .hline(ui.max_rect().x_range(), y, Stroke::new(1.0, color));
}

fn dashed_hline(ui: &Ui, y: f32, range: egui::Rangef, color: Color32) {
    let mut x = range.min;
    while x < range.max {
        ui.painter().line_segment(
            [pos2(x, y), pos2((x + 4.0).min(range.max), y)],
            Stroke::new(1.0, color),
        );
        x += 7.0;
    }
}

fn dashed_rect(ui: &Ui, rect: Rect, _radius: f32, color: Color32) {
    dashed_hline(ui, rect.top(), rect.x_range(), color);
    dashed_hline(ui, rect.bottom(), rect.x_range(), color);
    let mut y = rect.top();
    while y < rect.bottom() {
        let end = (y + 4.0).min(rect.bottom());
        ui.painter().line_segment(
            [pos2(rect.left(), y), pos2(rect.left(), end)],
            Stroke::new(1.0, color),
        );
        ui.painter().line_segment(
            [pos2(rect.right(), y), pos2(rect.right(), end)],
            Stroke::new(1.0, color),
        );
        y += 7.0;
    }
}

fn elide_to_width(
    painter: &egui::Painter,
    text: &str,
    font: &egui::FontId,
    max_width: f32,
) -> String {
    if painter
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let graphemes = text.graphemes(true).collect::<Vec<_>>();
    let mut low = 0;
    let mut high = graphemes.len();
    while low < high {
        let middle = (low + high).div_ceil(2);
        let candidate = format!("{}…", graphemes[..middle].concat());
        let width = painter
            .layout_no_wrap(candidate, font.clone(), Color32::WHITE)
            .size()
            .x;
        if width <= max_width {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    if low == 0 {
        "…".to_owned()
    } else {
        format!("{}…", graphemes[..low].concat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_projects_all_41_governed_specialist_owners() {
        let browser = SpecialistToolBrowserState::default();
        let rows = visible_rows(&browser, EngineeringProfile::AnalogIc);
        assert_eq!(rows.len(), 41);
        assert_eq!(
            rows[0].surface_id.archetype().as_str(),
            "specialist-workspace"
        );
    }

    #[test]
    fn search_matches_owner_module_purpose_and_stable_identity() {
        let mut browser = SpecialistToolBrowserState {
            query: "models-pdk correlation".to_owned(),
            ..SpecialistToolBrowserState::default()
        };
        let rows = visible_rows(&browser, EngineeringProfile::AnalogIc);
        assert!(
            rows.iter()
                .any(|row| row.surface_id == SurfaceId::ModelCorrelation)
        );
        assert!(rows.iter().all(|row| query_matches(row, &browser.query)));

        browser.query = "library-cellview-manager".to_owned();
        assert_eq!(
            visible_rows(&browser, EngineeringProfile::AnalogIc)
                .into_iter()
                .map(|row| row.surface_id)
                .collect::<Vec<_>>(),
            [SurfaceId::LibraryCellviewManager]
        );
    }

    #[test]
    fn filters_are_exact_non_destructive_projections() {
        let mut browser = SpecialistToolBrowserState::default();
        browser.pinned = vec![SurfaceId::RfWorkbench];
        browser.favorites = vec![SurfaceId::ModelEditor];
        browser.recents = vec![SurfaceId::PhotonicsWorkbench];

        browser.filter = SpecialistToolFilter::Pinned;
        assert_eq!(
            visible_rows(&browser, EngineeringProfile::AnalogIc)[0].surface_id,
            SurfaceId::RfWorkbench
        );
        browser.filter = SpecialistToolFilter::Favorites;
        assert_eq!(
            visible_rows(&browser, EngineeringProfile::AnalogIc)[0].surface_id,
            SurfaceId::ModelEditor
        );
        browser.filter = SpecialistToolFilter::Recent;
        assert_eq!(
            visible_rows(&browser, EngineeringProfile::AnalogIc)[0].surface_id,
            SurfaceId::PhotonicsWorkbench
        );
        assert_eq!(browser.pinned, [SurfaceId::RfWorkbench]);
        assert_eq!(browser.favorites, [SurfaceId::ModelEditor]);
        assert_eq!(browser.recents, [SurfaceId::PhotonicsWorkbench]);
    }

    #[test]
    fn pins_favorites_then_recents_define_stable_discovery_order() {
        let browser = SpecialistToolBrowserState {
            pinned: vec![SurfaceId::QuantumWorkbench],
            favorites: vec![SurfaceId::PcbWorkbench],
            recents: vec![SurfaceId::RfWorkbench],
            ..SpecialistToolBrowserState::default()
        };
        let rows = visible_rows(&browser, EngineeringProfile::All);
        assert_eq!(rows[0].surface_id, SurfaceId::QuantumWorkbench);
        assert_eq!(rows[1].surface_id, SurfaceId::PcbWorkbench);
        assert_eq!(rows[2].surface_id, SurfaceId::RfWorkbench);
    }

    #[test]
    fn responsive_geometry_and_phone_modes_match_the_mockup_contract() {
        assert_eq!(COMPACT_BREAKPOINT, 820.0);
        assert_eq!(CARD_HEIGHT, 58.0);
        assert_eq!(TOUCH_CARD_HEIGHT, 64.0);
        assert_eq!(CARD_ACTION_SIDE, 28.0);
        assert_eq!(TOUCH_ACTION_SIDE, 44.0);
        assert_eq!(
            phone_mode(SurfaceId::LayoutWorkbench),
            "adaptive authoring · focused steps"
        );
        assert_eq!(
            phone_mode(SurfaceId::VisualizationStudio),
            "full compact task · author · review · monitor"
        );
    }
}
