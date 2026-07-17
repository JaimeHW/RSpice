//! Account, organization, and licensing manager.
//!
//! The mockup defines this manager's composition, but its bundled identity,
//! organization, and device rows are design fixtures. RSpice does not yet
//! have an account authority backend, so this executor reports that boundary
//! explicitly and projects only the verified local license and current
//! application process. Backend-owned administration controls are omitted
//! instead of being rendered as inert or simulated actions.

use egui::{Align, Color32, Frame, Layout, Margin, RichText, Sense, Stroke, Ui, Vec2, vec2};

use crate::common::{AppState, RSpiceApp, app::ConsoleMessage};
use crate::services::license::LicenseInfo;
use crate::ui::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, Dialog, DialogChoice, DialogSize},
};

use super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

const ACCOUNT_DESCRIPTION: &str = "Review the local application session, account and organization authority boundary, and verified on-device license.";
const ACCOUNT_SPLIT_BREAKPOINT: f32 = 720.0;
// The 920 pt dialog loses a small amount to its vertical scrollbar; retain the
// mockup's six-column table on the full desktop surface and collapse only when
// the actual content track is materially narrower.
const ACCOUNT_TABLE_BREAKPOINT: f32 = 860.0;
const ACCOUNT_PROPERTY_TOP: i8 = 7;
const ACCOUNT_PROPERTY_BOTTOM: i8 = 10;
const ACCOUNT_TABLE_CELL_INSET: f32 = 8.0;
const ACCOUNT_TABLE_COLUMNS: f32 = 6.0;
const ACCOUNT_LICENSE_ACTION_BREAKPOINT: f32 = 560.0;
const ACCOUNT_SECTION_HEADER_HEIGHT: f32 = 29.0;

#[derive(Debug, Clone, PartialEq, Eq)]
struct AccountOrganizationSnapshot {
    summary_title: String,
    summary_detail: String,
    summary_meta: String,
    avatar_text: String,
    license_status: String,
    licensee: String,
    tier: String,
    updates_until: String,
    license_id: String,
    features: Vec<String>,
    storage: String,
    platform: &'static str,
}

impl AccountOrganizationSnapshot {
    fn from_license(license: Option<&LicenseInfo>) -> Self {
        let storage = license_storage_description();
        match license {
            Some(info) => Self {
                summary_title: "Local license active".to_owned(),
                summary_detail: format!("Licensed to {}", info.licensed_to),
                summary_meta:
                    "Local-only application session · no account or organization service connected"
                        .to_owned(),
                avatar_text: initials(&info.licensed_to),
                license_status: if info.updates_expired {
                    "Verified · perpetual use retained · updates window ended".to_owned()
                } else {
                    "Verified locally · active for this build".to_owned()
                },
                licensee: info.licensed_to.clone(),
                tier: info.tier.clone(),
                updates_until: info.updates_until.clone(),
                license_id: info.license_id.clone(),
                features: info.features.clone(),
                storage,
                platform: current_platform_label(),
            },
            None => Self {
                summary_title: "Local-only session".to_owned(),
                summary_detail: "No account identity is connected".to_owned(),
                summary_meta:
                    "Organization membership, roles, policy, and remote sessions are unavailable"
                        .to_owned(),
                avatar_text: "—".to_owned(),
                license_status: "No activated local license grant".to_owned(),
                licensee: "Not licensed on this device".to_owned(),
                tier: "Not available".to_owned(),
                updates_until: "Not available".to_owned(),
                license_id: "Not available".to_owned(),
                features: Vec::new(),
                storage,
                platform: current_platform_label(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccountAction {
    PersonalPreferences,
    LicenseManager,
}

/// Open the canonical manager route from title chrome or another manager.
pub(crate) fn open(app: &mut RSpiceApp) {
    let route = SurfaceRoute::surface(SurfaceId::AccountOrganization);
    if let Err(error) = app
        .state
        .workbench
        .navigate(route, RouteTransitionSource::User)
    {
        app.state
            .push_user_message(ConsoleMessage::warning(error.to_string()));
    }
}

/// Render the route owner. The local license dialog is rendered earlier in
/// the application pass; temporarily yielding this parent keeps that child
/// transaction visually and interactively on top.
pub(crate) fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.workbench.current_route().surface_id() != SurfaceId::AccountOrganization
        || app.state.dialogs.license_dialog.open
    {
        return;
    }

    let snapshot = AccountOrganizationSnapshot::from_license(app.state.license.as_ref());
    let mut requested_action = None;
    let choice = Dialog::new(
        "Identity · organization · licensing",
        "Account and administration",
        "Close",
    )
    .description(ACCOUNT_DESCRIPTION)
    .size(DialogSize::AccountManager)
    .flush_body()
    .show(ctx, |ui| {
        render_summary(ui, &snapshot);
        render_owner_actions(ui, &mut requested_action);
        render_authority_sections(ui);
        render_session_section(ui, &snapshot);
        render_license_section(ui, &snapshot, &mut requested_action);
    });

    if matches!(
        choice,
        DialogChoice::Primary | DialogChoice::Cancelled | DialogChoice::Ghost
    ) {
        close_to_source(&mut app.state);
        return;
    }
    if let Some(action) = requested_action {
        execute_action(app, action);
    }
}

fn execute_action(app: &mut RSpiceApp, action: AccountAction) {
    match action {
        AccountAction::PersonalPreferences => {
            if app
                .state
                .workbench
                .previous_route()
                .is_some_and(|route| route.surface_id() == SurfaceId::Preferences)
            {
                app.state
                    .workbench
                    .navigate_back(RouteTransitionSource::User);
                return;
            }
            let route = SurfaceRoute::surface(SurfaceId::Preferences);
            if let Err(error) = app
                .state
                .workbench
                .navigate(route, RouteTransitionSource::User)
            {
                app.state
                    .push_user_message(ConsoleMessage::warning(error.to_string()));
            }
        }
        AccountAction::LicenseManager => app.open_license_dialog(),
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
            "Could not close Account and administration: {error}"
        )));
    }
}

fn render_summary(ui: &mut Ui, snapshot: &AccountOrganizationSnapshot) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let narrow = ui.available_width() <= 820.0;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                avatar(ui, &snapshot.avatar_text);
                ui.vertical(|ui| {
                    render_summary_text(ui, snapshot);
                    if narrow {
                        ui.add_space(3.0);
                        status_badge(ui, "LOCAL / OFFLINE", t.color.warn);
                    }
                });
                if !narrow {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        status_badge(ui, "LOCAL / OFFLINE", t.color.warn);
                    });
                }
            });
        });
    horizontal_rule(ui, t.color.border_strong);
}

fn render_summary_text(ui: &mut Ui, snapshot: &AccountOrganizationSnapshot) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 2.0;
    let title = ui.label(
        RichText::new(&snapshot.summary_title)
            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
            .color(t.color.text),
    );
    accessible_text(ui, &title, &snapshot.summary_title);
    let detail = ui.label(
        RichText::new(&snapshot.summary_detail)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    accessible_text(ui, &detail, &snapshot.summary_detail);
    let meta = ui.add(
        egui::Label::new(
            RichText::new(&snapshot.summary_meta)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
        )
        .wrap(),
    );
    accessible_text(ui, &meta, &snapshot.summary_meta);
}

fn accessible_text(ui: &Ui, response: &egui::Response, text: &str) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(text);
    });
}

fn avatar(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
    ui.painter()
        .circle_filled(rect.center(), 19.0, t.color.bg_elevated);
    ui.painter()
        .circle_stroke(rect.center(), 19.0, Stroke::new(1.0, t.color.border_strong));
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
}

fn status_badge(ui: &mut Ui, label: &str, color: Color32) {
    Frame::NONE
        .fill(color.gamma_multiply(0.10))
        .stroke(Stroke::new(1.0, color.gamma_multiply(0.65)))
        .inner_margin(Margin::symmetric(7, 4))
        .show(ui, |ui| {
            ui.label(
                RichText::new(label)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(color),
            );
        });
}

fn render_owner_actions(ui: &mut Ui, action: &mut Option<AccountAction>) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            let narrow = ui.available_width() <= 820.0;
            let render_buttons = |ui: &mut Ui, action: &mut Option<AccountAction>| {
                if Button::new("Personal preferences…")
                    .min_width(if narrow { ui.available_width() } else { 0.0 })
                    .show(ui)
                    .clicked()
                {
                    *action = Some(AccountAction::PersonalPreferences);
                }
                if Button::new("License & activation…")
                    .min_width(if narrow { ui.available_width() } else { 0.0 })
                    .show(ui)
                    .clicked()
                {
                    *action = Some(AccountAction::LicenseManager);
                }
            };
            if narrow {
                ui.vertical(|ui| render_buttons(ui, action));
            } else {
                ui.horizontal(|ui| render_buttons(ui, action));
            }
        });
    horizontal_rule(ui, t.color.border_strong);
}

fn render_authority_sections(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let wide = ui.available_width() >= ACCOUNT_SPLIT_BREAKPOINT;
    if wide {
        let top = ui.cursor().top();
        let left = ui.cursor().left();
        let width = ui.available_width();
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.columns(2, |columns| {
                account_section(&mut columns[0], render_organization_authority);
                account_section(&mut columns[1], render_authentication_authority);
            });
        });
        let bottom = ui.cursor().top();
        ui.painter().vline(
            left + width * 0.5,
            top..=bottom,
            Stroke::new(1.0, t.color.border_strong),
        );
        horizontal_rule(ui, t.color.border_strong);
    } else {
        account_section(ui, render_organization_authority);
        horizontal_rule(ui, t.color.border_strong);
        account_section(ui, render_authentication_authority);
        horizontal_rule(ui, t.color.border_strong);
    }
}

fn account_section(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
    body(ui);
}

fn property_list(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
    Frame::NONE
        .inner_margin(Margin {
            left: 0,
            right: 0,
            top: ACCOUNT_PROPERTY_TOP,
            bottom: ACCOUNT_PROPERTY_BOTTOM,
        })
        .show(ui, body);
}

fn render_organization_authority(ui: &mut Ui) {
    section_title(ui, "Organization and role");
    property_list(ui, |ui| {
        property_row(ui, "Organization", "Not connected");
        property_row(ui, "Workspace role", "Local project editor");
        property_row(
            ui,
            "Approval authority",
            "Unavailable without organization service",
        );
        property_row(
            ui,
            "Managed policy",
            "No organization policy provider configured",
        );
    });
}

fn render_authentication_authority(ui: &mut Ui) {
    section_title(ui, "Authentication");
    property_list(ui, |ui| {
        property_row(ui, "Sign-in provider", "Not configured");
        property_row(ui, "Account session", "No authenticated account session");
        property_row(ui, "Multi-factor", "Not applicable to this local session");
        property_row(
            ui,
            "Recovery methods",
            "No account recovery authority available",
        );
    });
}

fn render_session_section(ui: &mut Ui, snapshot: &AccountOrganizationSnapshot) {
    section_title(ui, "Active sessions and devices");
    let detail = format!(
        "RSpice · this application\n{} · current process\nLocation not collected · local trust boundary",
        snapshot.platform
    );
    if ui.available_width() < ACCOUNT_TABLE_BREAKPOINT {
        property_row(ui, "Current session", &detail);
        return;
    }

    let column_width = ui.available_width() / ACCOUNT_TABLE_COLUMNS;
    egui::Grid::new("account-organization.sessions")
        .num_columns(6)
        .striped(false)
        .min_col_width(0.0)
        .spacing(Vec2::ZERO)
        .show(ui, |ui| {
            for heading in [
                "DEVICE",
                "PLATFORM",
                "LOCATION",
                "LAST ACTIVE",
                "TRUST",
                "ACTION",
            ] {
                table_header_cell(ui, heading, column_width);
            }
            ui.end_row();
            table_cell(ui, "RSpice · this application", column_width);
            table_cell(ui, snapshot.platform, column_width);
            table_cell(ui, "Not collected", column_width);
            table_cell(ui, "Now", column_width);
            table_cell(ui, "Local process", column_width);
            table_cell(ui, "Current", column_width);
            ui.end_row();
        });
}

fn render_license_section(
    ui: &mut Ui,
    snapshot: &AccountOrganizationSnapshot,
    action: &mut Option<AccountAction>,
) {
    section_title(ui, "Licensing and entitlements");
    let entitled_features = if snapshot.features.is_empty() {
        "No feature grants in an activated local license".to_owned()
    } else {
        snapshot.features.join(" · ")
    };
    property_list(ui, |ui| {
        property_row(ui, "State", &snapshot.license_status);
        property_row(ui, "Licensed to", &snapshot.licensee);
        property_row(ui, "Tier", &snapshot.tier);
        property_row(ui, "Updates until", &snapshot.updates_until);
        property_row(ui, "License ID", &snapshot.license_id);
        property_row(ui, "Local storage", &snapshot.storage);
        property_row(ui, "Entitled features", &entitled_features);
    });
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let copy = "License verification is local and signed. Server pools, borrowing, usage history, and organization entitlements are not configured.";
            let action_label = "Manage local license…";
            if ui.available_width() >= ACCOUNT_LICENSE_ACTION_BREAKPOINT {
                let gap = 8.0;
                let action_width = button_width(ui, action_label);
                let copy_width = (ui.available_width() - action_width - gap).max(1.0);
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.allocate_ui_with_layout(
                        Vec2::new(copy_width, 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new(copy)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_dim),
                                )
                                .wrap(),
                            );
                        },
                    );
                    if Button::new(action_label).show(ui).clicked() {
                        *action = Some(AccountAction::LicenseManager);
                    }
                });
            } else {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;
                    ui.add(
                        egui::Label::new(
                            RichText::new(copy)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                    if Button::new(action_label)
                        .min_width(ui.available_width())
                        .show(ui)
                        .clicked()
                    {
                        *action = Some(AccountAction::LicenseManager);
                    }
                });
            }
        });
}

fn button_width(ui: &Ui, label: &str) -> f32 {
    ui.painter()
        .layout_no_wrap(
            label.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            Color32::WHITE,
        )
        .size()
        .x
        + 20.0
}

fn section_title(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), ACCOUNT_SECTION_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect_filled(
        rect,
        0.0,
        Color32::from_rgba_unmultiplied(
            t.color.bg_panel_2.r(),
            t.color.bg_panel_2.g(),
            t.color.bg_panel_2.b(),
            204,
        ),
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &title.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: title_font,
            color: t.color.text_dim,
            extra_letter_spacing: 0.055 * tokens::FS_0,
            ..Default::default()
        },
    );
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let text_rect = rect.shrink2(vec2(10.0, 0.0));
    ui.painter().with_clip_rect(text_rect).galley(
        text_rect.left_center() - vec2(0.0, galley.size().y * 0.5),
        galley,
        t.color.text_dim,
    );
}

fn property_row(ui: &mut Ui, label: &str, value: &str) {
    let accessible_label = format!("{label}: {value}");
    let response = super::design_system::property_row(ui, label, value);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &accessible_label)
    });
    accessible_text(ui, &response, &accessible_label);
}

fn table_header_cell(ui: &mut Ui, value: &str, width: f32) {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let (rect, response) = ui.allocate_exact_size(vec2(width, 27.0), Sense::hover());
    let text_rect = rect.shrink2(vec2(ACCOUNT_TABLE_CELL_INSET, 0.0));
    let text = super::design_system::elide_text(ui, value, &font, text_rect.width());
    ui.painter().text(
        text_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        text,
        font,
        t.color.text_dim,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), value));
}

fn table_cell(ui: &mut Ui, value: &str, width: f32) {
    let t = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.row_h), Sense::hover());
    let text_rect = rect.shrink2(vec2(ACCOUNT_TABLE_CELL_INSET, 0.0));
    let text = super::design_system::elide_text(ui, value, &font, text_rect.width());
    ui.painter().text(
        text_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        text,
        font,
        t.color.text,
    );
    response
        .on_hover_text(value)
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, value));
}

fn horizontal_rule(ui: &mut Ui, color: Color32) {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, 1.0), Sense::hover());
    ui.painter()
        .hline(rect.x_range(), rect.center().y, Stroke::new(1.0, color));
}

fn initials(value: &str) -> String {
    let mut letters = value
        .split_whitespace()
        .filter_map(|part| part.chars().find(char::is_ascii_alphanumeric))
        .take(2)
        .collect::<String>()
        .to_uppercase();
    if letters.is_empty() {
        letters.push('—');
    }
    letters
}

#[cfg(target_arch = "wasm32")]
fn license_storage_description() -> String {
    "Browser application session storage".to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
fn license_storage_description() -> String {
    crate::services::license::license_file_path().map_or_else(
        || "Configuration directory unavailable".to_owned(),
        |path| path.display().to_string(),
    )
}

#[cfg(target_arch = "wasm32")]
const fn current_platform_label() -> &'static str {
    "Web browser"
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
const fn current_platform_label() -> &'static str {
    "Windows desktop"
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
const fn current_platform_label() -> &'static str {
    "macOS desktop"
}

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
const fn current_platform_label() -> &'static str {
    "Linux desktop"
}

#[cfg(all(
    not(target_arch = "wasm32"),
    not(any(target_os = "windows", target_os = "macos", target_os = "linux"))
))]
const fn current_platform_label() -> &'static str {
    "Native application"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::BrowserHistoryEffect;

    fn account_route() -> SurfaceRoute {
        SurfaceRoute::surface(SurfaceId::AccountOrganization)
    }

    #[test]
    fn unconfigured_snapshot_never_invents_identity_or_authority() {
        let snapshot = AccountOrganizationSnapshot::from_license(None);
        assert_eq!(snapshot.summary_title, "Local-only session");
        assert_eq!(snapshot.summary_detail, "No account identity is connected");
        assert_eq!(snapshot.licensee, "Not licensed on this device");
        assert!(snapshot.features.is_empty());
        let disclosed = format!("{snapshot:?}");
        for fixture in ["James Whitfield", "Acme Engineering", "Chicago", "SAML"] {
            assert!(!disclosed.contains(fixture));
        }
    }

    #[test]
    fn active_snapshot_projects_exact_verified_license_data() {
        let info = crate::services::license::parse_and_verify(crate::services::license::SAMPLE_KEY)
            .expect("signed sample license");
        let snapshot = AccountOrganizationSnapshot::from_license(Some(&info));
        assert_eq!(snapshot.licensee, info.licensed_to);
        assert_eq!(snapshot.tier, info.tier);
        assert_eq!(snapshot.updates_until, info.updates_until);
        assert_eq!(snapshot.license_id, info.license_id);
        assert_eq!(snapshot.features, info.features);
        assert!(snapshot.license_status.contains("Verified"));
    }

    #[test]
    fn account_route_returns_to_exact_source_route() {
        let mut state = AppState::default();
        let source = state.workbench.current_route();
        state
            .workbench
            .navigate(account_route(), RouteTransitionSource::User)
            .expect("account manager route is executable");
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Push(account_route()))
        );
        close_to_source(&mut state);
        assert_eq!(state.workbench.current_route(), source);
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Traverse {
                delta: -1,
                destination: source,
            })
        );
    }

    #[test]
    fn direct_deep_link_close_replaces_with_retained_workspace() {
        let mut state = AppState::default();
        let fallback = SurfaceRoute::surface(SurfaceId::Design);
        state
            .workbench
            .navigate(account_route(), RouteTransitionSource::BrowserPop)
            .expect("account manager route is executable");
        close_to_source(&mut state);
        assert_eq!(state.workbench.current_route(), fallback);
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Replace(fallback))
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn both_exposed_actions_have_real_executors() {
        let mut preferences_app = RSpiceApp::test_instance();
        preferences_app
            .state
            .workbench
            .navigate(account_route(), RouteTransitionSource::BrowserPop)
            .unwrap();
        execute_action(&mut preferences_app, AccountAction::PersonalPreferences);
        assert_eq!(
            preferences_app.state.workbench.current_route().surface_id(),
            SurfaceId::Preferences
        );

        let mut license_app = RSpiceApp::test_instance();
        execute_action(&mut license_app, AccountAction::LicenseManager);
        assert!(license_app.state.dialogs.license_dialog.open);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn typed_command_and_rendered_surface_disclose_the_same_offline_boundary() {
        use crate::workbench::commands::{COMMAND_REGISTRY, Command};

        assert!(COMMAND_REGISTRY.contains(&Command::AccountOrganization));
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        Command::AccountOrganization.execute(&mut app);
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            SurfaceId::AccountOrganization
        );

        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_100.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| show(ctx, &mut app),
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("account manager accessibility tree")
            .nodes;
        let labels = nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect::<Vec<_>>();
        for expected in [
            "Account and administration",
            "Local-only session",
            "No account identity is connected",
            "Personal preferences…",
            "License & activation…",
        ] {
            assert!(
                labels.contains(&expected),
                "missing rendered label: {expected}; labels: {labels:?}"
            );
        }
        for fixture in ["James Whitfield", "Acme Engineering", "Chicago", "SAML"] {
            assert!(!labels.contains(&fixture), "fixture leaked into account UI");
        }
    }

    #[test]
    fn initials_are_derived_without_fixture_fallbacks() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("RSpice Labs LLC"), "RL");
        assert_eq!(initials("—"), "—");
    }
}
