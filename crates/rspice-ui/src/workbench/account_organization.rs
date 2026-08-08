//! Account and administration console.
//!
//! The shape is a datasheet, not a dashboard: one surface read top to bottom
//! — sign-in, license, what it unlocks, device sessions, build, data — with
//! every band stating only facts the application can prove. Provable sources
//! are exactly three: the verified local license file, the cloud account
//! session (server-verified principal, entitlements, and license leases from
//! `services::cloud_account`), and this build/process. Anything without a
//! backing authority is reported as an explicit boundary instead of being
//! rendered as an inert or simulated control, and every exposed action routes
//! to a real executor.

use egui::{Align, Color32, Frame, Layout, Margin, RichText, Sense, Stroke, Ui, Vec2, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::services::cloud_account::{
    CloudAccountAvailability, CloudSessionPhase, CloudSessionSnapshot,
};
use crate::services::license::LicenseInfo;
use crate::ui::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, Dialog, DialogChoice, DialogSize},
};
use crate::workbench::{AppState, RSpiceApp};

use super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

const ACCOUNT_DESCRIPTION: &str = "The account this installation is signed in with, the license it runs under, the build it runs, and exactly what leaves this machine.";
// The 920 pt dialog loses a small amount to its vertical scrollbar; retain the
// mockup's table layout on the full desktop surface and collapse only when
// the actual content track is materially narrower.
const ACCOUNT_TABLE_BREAKPOINT: f32 = 860.0;
const ACCOUNT_PROPERTY_TOP: i8 = 7;
const ACCOUNT_PROPERTY_BOTTOM: i8 = 10;
const ACCOUNT_TABLE_CELL_INSET: f32 = 8.0;
const ACCOUNT_SECTION_HEADER_HEIGHT: f32 = 29.0;
const ACCOUNT_SCROLL_END_PADDING: f32 = 12.0;
const ACCOUNT_NARROW_BREAKPOINT: f32 = 820.0;

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BadgeTone {
    Ok,
    Warn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StripModel {
    title: String,
    detail: String,
    meta: String,
    avatar: String,
    badge: (String, BadgeTone),
    error: Option<String>,
    actions: Vec<AccountAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnlockRow {
    feature: String,
    detail: String,
    state: &'static str,
    licensed: bool,
    term: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeviceRow {
    device: String,
    issued: String,
    expires: String,
    /// Lease ID to revoke; `None` marks this installation's own row.
    revoke: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AccountConsoleModel {
    strip: StripModel,
    license_rows: Vec<(String, String)>,
    license_chip: (String, BadgeTone),
    unlocks: Vec<UnlockRow>,
    devices: Vec<DeviceRow>,
    build_rows: Vec<(String, String)>,
    data_rows: Vec<(String, String)>,
    boundary: String,
}

impl AccountConsoleModel {
    fn project(
        license: Option<&LicenseInfo>,
        cloud: &CloudSessionSnapshot,
        availability: CloudAccountAvailability,
    ) -> Self {
        Self {
            strip: strip_model(cloud, availability),
            license_rows: license_rows(license, cloud),
            license_chip: if license.is_some() {
                ("activated".to_owned(), BadgeTone::Ok)
            } else {
                ("no license file".to_owned(), BadgeTone::Warn)
            },
            unlocks: unlock_rows(license, cloud),
            devices: device_rows(cloud),
            build_rows: vec![
                (
                    "Application".to_owned(),
                    concat!("RSpice ", env!("CARGO_PKG_VERSION")).to_owned(),
                ),
                ("Platform".to_owned(), current_platform_label().to_owned()),
            ],
            data_rows: data_rows(availability),
            boundary: boundary_statement(license, cloud),
        }
    }
}

fn strip_model(cloud: &CloudSessionSnapshot, availability: CloudAccountAvailability) -> StripModel {
    let signed_out_title = "Not signed in".to_owned();
    match availability {
        CloudAccountAvailability::UnconfiguredBuild => StripModel {
            title: signed_out_title,
            detail: "This build carries no cloud account endpoints.".to_owned(),
            meta: "Sign-in, cloud licensing, web publishing, and live collaboration are \
                   unavailable in this build."
                .to_owned(),
            avatar: "—".to_owned(),
            badge: ("LOCAL / OFFLINE".to_owned(), BadgeTone::Warn),
            error: None,
            actions: Vec::new(),
        },
        CloudAccountAvailability::BrowserPending => StripModel {
            title: signed_out_title,
            detail: "Browser sign-in arrives with the hosted deployment.".to_owned(),
            meta: "Use the desktop application to sign in and manage licensing today.".to_owned(),
            avatar: "—".to_owned(),
            badge: ("LOCAL / OFFLINE".to_owned(), BadgeTone::Warn),
            error: None,
            actions: Vec::new(),
        },
        CloudAccountAvailability::Native => match &cloud.phase {
            CloudSessionPhase::SignedOut { last_error } => StripModel {
                title: signed_out_title,
                detail: "Sign in to connect licensing, web publishing, and collaboration."
                    .to_owned(),
                meta: "Signing in opens your browser; RSpice never sees your password.".to_owned(),
                avatar: "—".to_owned(),
                badge: ("LOCAL / OFFLINE".to_owned(), BadgeTone::Warn),
                error: last_error.clone(),
                actions: vec![AccountAction::SignIn],
            },
            CloudSessionPhase::WaitingForBrowser => StripModel {
                title: "Waiting for the browser…".to_owned(),
                detail: "Complete sign-in in the browser window that just opened.".to_owned(),
                meta: "Nothing happens in RSpice until the browser hands the sign-in back."
                    .to_owned(),
                avatar: "…".to_owned(),
                badge: ("SIGNING IN".to_owned(), BadgeTone::Warn),
                error: None,
                actions: vec![AccountAction::ReopenSignInPage, AccountAction::CancelSignIn],
            },
            CloudSessionPhase::ExchangingTokens | CloudSessionPhase::Bootstrapping => StripModel {
                title: "Signing in…".to_owned(),
                detail: "Establishing the account session.".to_owned(),
                meta: String::new(),
                avatar: "…".to_owned(),
                badge: ("SIGNING IN".to_owned(), BadgeTone::Warn),
                error: None,
                actions: Vec::new(),
            },
            CloudSessionPhase::Active => {
                let (title, detail) = identity_lines(cloud);
                StripModel {
                    avatar: initials(&title),
                    meta: cloud
                        .verified_at
                        .as_deref()
                        .map(|stamp| format!("Account verified {}", humanize_stamp(stamp)))
                        .unwrap_or_default(),
                    title,
                    detail,
                    badge: ("SIGNED IN".to_owned(), BadgeTone::Ok),
                    error: None,
                    actions: vec![AccountAction::RefreshSession, AccountAction::SignOut],
                }
            }
            CloudSessionPhase::OfflineLicensed => {
                let (title, detail) = identity_lines(cloud);
                let meta = cloud
                    .native_license
                    .as_ref()
                    .map(|license| {
                        format!(
                            "Offline — this device stays licensed through {}",
                            unix_date(license.expires_at_unix_seconds)
                        )
                    })
                    .unwrap_or_default();
                StripModel {
                    avatar: initials(&title),
                    title,
                    detail,
                    meta,
                    badge: ("OFFLINE · LICENSED".to_owned(), BadgeTone::Warn),
                    error: None,
                    actions: vec![AccountAction::RefreshSession, AccountAction::SignOut],
                }
            }
        },
    }
}

fn identity_lines(cloud: &CloudSessionSnapshot) -> (String, String) {
    let principal = cloud.principal.as_ref();
    let display_name = principal.and_then(|principal| principal.display_name.clone());
    let email = principal.and_then(|principal| principal.email.clone());
    match (display_name, email) {
        (Some(name), Some(email)) => (name, email),
        (Some(name), None) => (name, String::new()),
        (None, Some(email)) => (email, String::new()),
        (None, None) => ("Signed in".to_owned(), String::new()),
    }
}

fn license_rows(
    license: Option<&LicenseInfo>,
    cloud: &CloudSessionSnapshot,
) -> Vec<(String, String)> {
    let storage = license_storage_description();
    let mut rows = match license {
        Some(info) => vec![
            (
                "State".to_owned(),
                if info.updates_expired {
                    "Verified · perpetual use retained · updates window ended".to_owned()
                } else {
                    "Verified locally · active for this build".to_owned()
                },
            ),
            ("Licensed to".to_owned(), info.licensed_to.clone()),
            ("Tier".to_owned(), info.tier.clone()),
            ("Updates until".to_owned(), info.updates_until.clone()),
            ("License ID".to_owned(), info.license_id.clone()),
            ("Storage".to_owned(), storage),
        ],
        None => vec![
            (
                "State".to_owned(),
                "No activated local license file".to_owned(),
            ),
            ("Storage".to_owned(), storage),
        ],
    };
    if let Some(native) = &cloud.native_license {
        rows.push((
            "Cloud license".to_owned(),
            format!("{} · {} plan", native.product, native.plan),
        ));
        rows.push((
            "Offline lease renews by".to_owned(),
            unix_date(native.expires_at_unix_seconds),
        ));
    }
    rows
}

/// Cloud entitlement feature keys mapped to the product language, so the
/// Unlocks table names shipped capabilities rather than wire identifiers.
fn cloud_feature_display(key: &str) -> (String, String) {
    match key {
        "cloud_publishing" => (
            "Web publishing".to_owned(),
            "published circuit pages on RSpice Cloud".to_owned(),
        ),
        "live_collaboration" => (
            "Live collaboration".to_owned(),
            "live sessions on RSpice Cloud".to_owned(),
        ),
        "cloud_simulation" => (
            "Cloud simulation".to_owned(),
            "runs queued to RSpice Cloud workers".to_owned(),
        ),
        "native_license" => (
            "Offline desktop license".to_owned(),
            "a verified lease keeps this device licensed offline".to_owned(),
        ),
        other => (other.to_owned(), "account entitlement".to_owned()),
    }
}

fn unlock_rows(license: Option<&LicenseInfo>, cloud: &CloudSessionSnapshot) -> Vec<UnlockRow> {
    let mut rows = vec![UnlockRow {
        feature: "Schematic capture & simulation".to_owned(),
        detail: "DC · AC · transient · noise · S-parameter".to_owned(),
        state: "included",
        licensed: true,
        term: "perpetual".to_owned(),
    }];

    // The local license's feature catalog: every label the signer can grant,
    // with this license's actual grants marked.
    for label in crate::services::license::FEATURE_LABELS
        .iter()
        .map(|(_, label)| *label)
    {
        let granted = license.is_some_and(|info| info.features.iter().any(|f| f == label));
        rows.push(UnlockRow {
            feature: label.to_owned(),
            detail: "local license grant".to_owned(),
            state: if granted { "licensed" } else { "not licensed" },
            licensed: granted,
            term: if granted {
                "perpetual".to_owned()
            } else {
                "—".to_owned()
            },
        });
    }

    // Cloud grants appear only when the signed-in account actually holds
    // them: they are server-verified facts, never a catalog of hopes.
    if cloud.signed_in() {
        let mut granted: Vec<(String, Option<String>)> = Vec::new();
        for entitlement in &cloud.entitlements {
            if entitlement.status == "active" || entitlement.status == "grace_period" {
                for key in &entitlement.granted_features {
                    if !granted.iter().any(|(existing, _)| existing == key) {
                        granted.push((key.clone(), entitlement.valid_until.clone()));
                    }
                }
            }
        }
        if let Some(native) = &cloud.native_license {
            for key in &native.granted_features {
                if !granted.iter().any(|(existing, _)| existing == key) {
                    granted.push((key.clone(), None));
                }
            }
        }
        granted.sort_by(|a, b| a.0.cmp(&b.0));
        for (key, valid_until) in granted {
            let (feature, detail) = cloud_feature_display(&key);
            rows.push(UnlockRow {
                feature,
                detail,
                state: "licensed",
                licensed: true,
                term: match valid_until {
                    Some(stamp) => format!("through {}", humanize_stamp(&stamp)),
                    None => "while the subscription is active".to_owned(),
                },
            });
        }
    }
    rows
}

fn device_rows(cloud: &CloudSessionSnapshot) -> Vec<DeviceRow> {
    cloud
        .device_leases
        .iter()
        .filter(|lease| lease.revoked_at.is_none())
        .map(|lease| DeviceRow {
            device: if lease.this_device {
                format!("This installation · {} plan", lease.plan)
            } else {
                format!(
                    "Signed-in device · lease {} · {} plan",
                    short_id(&lease.id),
                    lease.plan
                )
            },
            issued: humanize_stamp(&lease.issued_at),
            expires: humanize_stamp(&lease.expires_at),
            revoke: if lease.this_device {
                None
            } else {
                Some(lease.id.clone())
            },
        })
        .collect()
}

fn data_rows(availability: CloudAccountAvailability) -> Vec<(String, String)> {
    let network = match availability {
        CloudAccountAvailability::Native => {
            "sign-in, licensing, and RSpice Cloud services only".to_owned()
        }
        CloudAccountAvailability::UnconfiguredBuild => {
            "none — this build reaches no account service".to_owned()
        }
        CloudAccountAvailability::BrowserPending => {
            "the hosted deployment serves this session".to_owned()
        }
    };
    vec![
        ("Telemetry".to_owned(), "none — nothing is sent".to_owned()),
        ("Network access".to_owned(), network),
        (
            "Engineering data".to_owned(),
            "files on this device".to_owned(),
        ),
    ]
}

fn boundary_statement(license: Option<&LicenseInfo>, cloud: &CloudSessionSnapshot) -> String {
    if let Some(native) = &cloud.native_license {
        return format!(
            "Without a reachable licensing service this device stays licensed through {}, \
             and projects, results, and reports on this machine stay readable forever — a \
             license gates new work, never your existing files.",
            unix_date(native.expires_at_unix_seconds)
        );
    }
    if license.is_some() {
        return "License verification is local and signed; nothing about your projects \
                leaves this machine."
            .to_owned();
    }
    "Projects, results, and reports on this machine stay readable without any license or \
     account."
        .to_owned()
}

fn short_id(id: &str) -> String {
    id.chars().take(8).collect()
}

/// `2026-08-06T13:41:00Z` → `2026-08-06`; anything unparsable passes through.
fn humanize_stamp(stamp: &str) -> String {
    match stamp.split_once('T') {
        Some((date, _)) if date.len() == 10 => date.to_owned(),
        _ => stamp.to_owned(),
    }
}

/// Unix seconds → `YYYY-MM-DD` (proleptic Gregorian, civil-date algorithm).
/// Dependency-free so the model compiles identically on every target.
fn unix_date(unix_seconds: i64) -> String {
    let days = unix_seconds.div_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    format!("{year:04}-{month:02}-{day:02}")
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum AccountAction {
    PersonalPreferences,
    LicenseManager,
    SignIn,
    ReopenSignInPage,
    CancelSignIn,
    SignOut,
    RefreshSession,
    RevokeLease(String),
    LegalPrivacy,
    SupportBundle,
    HelpCenter,
}

impl AccountAction {
    fn label(&self) -> &'static str {
        match self {
            Self::PersonalPreferences => "Personal preferences…",
            Self::LicenseManager => "License & activation…",
            Self::SignIn => "Sign in…",
            Self::ReopenSignInPage => "Open the sign-in page again",
            Self::CancelSignIn => "Cancel sign-in",
            Self::SignOut => "Sign out",
            Self::RefreshSession => "Refresh",
            Self::RevokeLease(_) => "Revoke",
            Self::LegalPrivacy => "Legal and privacy…",
            Self::SupportBundle => "Create support bundle…",
            Self::HelpCenter => "Help center…",
        }
    }
}

fn execute_action(app: &mut RSpiceApp, action: AccountAction) {
    use crate::workbench::commands::vocabulary::Command;
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
        AccountAction::SignIn => app.cloud_account.sign_in(),
        AccountAction::ReopenSignInPage => app.cloud_account.reopen_sign_in_page(),
        AccountAction::CancelSignIn => app.cloud_account.cancel_sign_in(),
        AccountAction::SignOut => app.cloud_account.sign_out(),
        AccountAction::RefreshSession => app.cloud_account.refresh(),
        AccountAction::RevokeLease(lease_id) => app.cloud_account.revoke_lease(lease_id),
        AccountAction::LegalPrivacy => Command::LegalPrivacy.execute(app),
        AccountAction::SupportBundle => Command::SupportBundle.execute(app),
        AccountAction::HelpCenter => Command::HelpCenter.execute(app),
    }
}

// ---------------------------------------------------------------------------
// Routing
// ---------------------------------------------------------------------------

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

    let model = AccountConsoleModel::project(
        app.state.license.as_ref(),
        app.cloud_account.snapshot(),
        app.cloud_account.availability(),
    );
    let mut requested_action = None;
    let choice = Dialog::new(
        "Account · license · data",
        "Account and administration",
        "Close",
    )
    .description(ACCOUNT_DESCRIPTION)
    .size(DialogSize::AccountManager)
    .flush_body()
    .show(ctx, |ui| {
        render_strip(ui, &model.strip, &mut requested_action);
        render_owner_actions(ui, &mut requested_action);
        render_license_band(ui, &model, &mut requested_action);
        render_unlocks_band(ui, &model);
        render_devices_band(ui, &model, &mut requested_action);
        render_build_band(ui, &model);
        render_data_band(ui, &model, &mut requested_action);
        render_support_band(ui, &mut requested_action);
        render_boundary(ui, &model.boundary);
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

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

fn render_strip(ui: &mut Ui, strip: &StripModel, action: &mut Option<AccountAction>) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let narrow = ui.available_width() <= ACCOUNT_NARROW_BREAKPOINT;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                avatar(ui, &strip.avatar);
                ui.vertical(|ui| {
                    render_strip_text(ui, strip);
                    if narrow {
                        ui.add_space(3.0);
                        render_badge(ui, strip);
                    }
                });
                if !narrow {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        render_badge(ui, strip);
                    });
                }
            });
            if !strip.actions.is_empty() || strip.error.is_some() {
                ui.add_space(6.0);
                if let Some(error) = &strip.error {
                    let label = ui.add(
                        egui::Label::new(
                            RichText::new(error)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.err),
                        )
                        .wrap(),
                    );
                    accessible_text(ui, &label, error);
                }
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
                    for strip_action in &strip.actions {
                        if Button::new(strip_action.label()).show(ui).clicked() {
                            *action = Some(strip_action.clone());
                        }
                    }
                });
            }
        });
    horizontal_rule(ui, t.color.border_strong);
}

fn render_badge(ui: &mut Ui, strip: &StripModel) {
    let t = Tokens::get(ui.ctx());
    let color = match strip.badge.1 {
        BadgeTone::Ok => t.color.ok,
        BadgeTone::Warn => t.color.warn,
    };
    status_badge(ui, &strip.badge.0, color);
}

fn render_strip_text(ui: &mut Ui, strip: &StripModel) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 2.0;
    let title = ui.label(
        RichText::new(&strip.title)
            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
            .color(t.color.text),
    );
    accessible_text(ui, &title, &strip.title);
    if !strip.detail.is_empty() {
        let detail = ui.label(
            RichText::new(&strip.detail)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        accessible_text(ui, &detail, &strip.detail);
    }
    if !strip.meta.is_empty() {
        let meta = ui.add(
            egui::Label::new(
                RichText::new(&strip.meta)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            )
            .wrap(),
        );
        accessible_text(ui, &meta, &strip.meta);
    }
}

fn render_owner_actions(ui: &mut Ui, action: &mut Option<AccountAction>) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            let narrow = ui.available_width() <= ACCOUNT_NARROW_BREAKPOINT;
            let render_buttons = |ui: &mut Ui, action: &mut Option<AccountAction>| {
                for owner_action in [
                    AccountAction::PersonalPreferences,
                    AccountAction::LicenseManager,
                ] {
                    if Button::new(owner_action.label())
                        .min_width(if narrow { ui.available_width() } else { 0.0 })
                        .show(ui)
                        .clicked()
                    {
                        *action = Some(owner_action.clone());
                    }
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

fn render_license_band(
    ui: &mut Ui,
    model: &AccountConsoleModel,
    action: &mut Option<AccountAction>,
) {
    let t = Tokens::get(ui.ctx());
    section_title(ui, "License");
    property_list(ui, |ui| {
        for (label, value) in &model.license_rows {
            property_row(ui, label, value);
        }
    });
    band_actions(ui, &[AccountAction::LicenseManager], action);
    horizontal_rule(ui, t.color.border_strong);
}

fn render_unlocks_band(ui: &mut Ui, model: &AccountConsoleModel) {
    let t = Tokens::get(ui.ctx());
    let licensed = model.unlocks.iter().filter(|row| row.licensed).count();
    section_title(
        ui,
        &format!(
            "Unlocks · {licensed} of {} feature sets",
            model.unlocks.len()
        ),
    );
    if ui.available_width() < ACCOUNT_TABLE_BREAKPOINT {
        property_list(ui, |ui| {
            for row in &model.unlocks {
                property_row(ui, &row.feature, &format!("{} · {}", row.state, row.term));
            }
        });
    } else {
        let widths = [0.46_f32, 0.18, 0.36];
        let total = ui.available_width();
        egui::Grid::new("account-organization.unlocks")
            .num_columns(3)
            .striped(false)
            .min_col_width(0.0)
            .spacing(Vec2::ZERO)
            .show(ui, |ui| {
                for (heading, width) in ["FEATURE", "STATE", "TERM"].iter().zip(widths) {
                    table_header_cell(ui, heading, total * width);
                }
                ui.end_row();
                for row in &model.unlocks {
                    let feature = if row.detail.is_empty() {
                        row.feature.clone()
                    } else {
                        format!("{} — {}", row.feature, row.detail)
                    };
                    table_cell(ui, &feature, total * widths[0]);
                    table_cell(ui, row.state, total * widths[1]);
                    table_cell(ui, &row.term, total * widths[2]);
                    ui.end_row();
                }
            });
    }
    horizontal_rule(ui, t.color.border_strong);
}

fn render_devices_band(
    ui: &mut Ui,
    model: &AccountConsoleModel,
    action: &mut Option<AccountAction>,
) {
    let t = Tokens::get(ui.ctx());
    if model.devices.is_empty() {
        section_title(ui, "Device sessions");
        property_list(ui, |ui| {
            property_row(
                ui,
                "Current session",
                &format!(
                    "RSpice · this application\n{} · current process\nLocation not collected · local trust boundary",
                    current_platform_label()
                ),
            );
        });
        horizontal_rule(ui, t.color.border_strong);
        return;
    }

    section_title(
        ui,
        &format!("Device sessions · {} signed in", model.devices.len()),
    );
    if ui.available_width() < ACCOUNT_TABLE_BREAKPOINT {
        property_list(ui, |ui| {
            for row in &model.devices {
                property_row(
                    ui,
                    &row.device,
                    &format!("issued {} · renews by {}", row.issued, row.expires),
                );
            }
        });
    } else {
        let widths = [0.4_f32, 0.2, 0.2, 0.2];
        let total = ui.available_width();
        egui::Grid::new("account-organization.devices")
            .num_columns(4)
            .striped(false)
            .min_col_width(0.0)
            .spacing(Vec2::ZERO)
            .show(ui, |ui| {
                for (heading, width) in ["DEVICE", "ISSUED", "RENEWS BY", "ACTION"]
                    .iter()
                    .zip(widths)
                {
                    table_header_cell(ui, heading, total * width);
                }
                ui.end_row();
                for row in &model.devices {
                    table_cell(ui, &row.device, total * widths[0]);
                    table_cell(ui, &row.issued, total * widths[1]);
                    table_cell(ui, &row.expires, total * widths[2]);
                    match &row.revoke {
                        Some(lease_id) => {
                            if Button::new("Revoke").show(ui).clicked() {
                                *action = Some(AccountAction::RevokeLease(lease_id.clone()));
                            }
                        }
                        None => table_cell(ui, "this device", total * widths[3]),
                    }
                    ui.end_row();
                }
            });
    }
    horizontal_rule(ui, t.color.border_strong);
}

fn render_build_band(ui: &mut Ui, model: &AccountConsoleModel) {
    let t = Tokens::get(ui.ctx());
    section_title(ui, "Build");
    property_list(ui, |ui| {
        for (label, value) in &model.build_rows {
            property_row(ui, label, value);
        }
    });
    horizontal_rule(ui, t.color.border_strong);
}

fn render_data_band(ui: &mut Ui, model: &AccountConsoleModel, action: &mut Option<AccountAction>) {
    let t = Tokens::get(ui.ctx());
    section_title(ui, "Data");
    property_list(ui, |ui| {
        for (label, value) in &model.data_rows {
            property_row(ui, label, value);
        }
    });
    band_actions(ui, &[AccountAction::LegalPrivacy], action);
    horizontal_rule(ui, t.color.border_strong);
}

fn render_support_band(ui: &mut Ui, action: &mut Option<AccountAction>) {
    let t = Tokens::get(ui.ctx());
    section_title(ui, "Support");
    band_actions(
        ui,
        &[AccountAction::SupportBundle, AccountAction::HelpCenter],
        action,
    );
    horizontal_rule(ui, t.color.border_strong);
}

fn render_boundary(ui: &mut Ui, boundary: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            let label = ui.add(
                egui::Label::new(
                    RichText::new(boundary)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
            accessible_text(ui, &label, boundary);
        });
    ui.add_space(ACCOUNT_SCROLL_END_PADDING);
}

fn band_actions(ui: &mut Ui, actions: &[AccountAction], requested: &mut Option<AccountAction>) {
    Frame::NONE
        .inner_margin(Margin::symmetric(10, 4))
        .show(ui, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                for action in actions.iter().rev() {
                    if Button::new(action.label()).show(ui).clicked() {
                        *requested = Some(action.clone());
                    }
                }
            });
        });
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
    use crate::services::cloud_account::{
        EntitlementSummary, LeaseSummary, NativeLicenseSummary, PrincipalSummary,
    };
    use crate::workbench::BrowserHistoryEffect;

    fn account_route() -> SurfaceRoute {
        SurfaceRoute::surface(SurfaceId::AccountOrganization)
    }

    fn active_cloud_snapshot() -> CloudSessionSnapshot {
        CloudSessionSnapshot {
            phase: CloudSessionPhase::Active,
            principal: Some(PrincipalSummary {
                email: Some("engineer@example.com".to_owned()),
                display_name: Some("Example Engineer".to_owned()),
            }),
            entitlements: vec![EntitlementSummary {
                status: "active".to_owned(),
                granted_features: vec!["cloud_publishing".to_owned(), "native_license".to_owned()],
                valid_until: Some("2027-07-01T00:00:00Z".to_owned()),
            }],
            native_license: Some(NativeLicenseSummary {
                plan: "professional".to_owned(),
                product: "rspice".to_owned(),
                granted_features: vec!["native_license".to_owned()],
                expires_at_unix_seconds: 1_785_604_800,
                lease_id: "lease-a".to_owned(),
            }),
            device_leases: vec![
                LeaseSummary {
                    id: "lease-a".to_owned(),
                    plan: "professional".to_owned(),
                    issued_at: "2026-08-01T00:00:00Z".to_owned(),
                    expires_at: "2026-08-08T00:00:00Z".to_owned(),
                    revoked_at: None,
                    this_device: true,
                },
                LeaseSummary {
                    id: "lease-b".to_owned(),
                    plan: "professional".to_owned(),
                    issued_at: "2026-07-20T00:00:00Z".to_owned(),
                    expires_at: "2026-08-05T00:00:00Z".to_owned(),
                    revoked_at: None,
                    this_device: false,
                },
                LeaseSummary {
                    id: "lease-revoked".to_owned(),
                    plan: "professional".to_owned(),
                    issued_at: "2026-07-01T00:00:00Z".to_owned(),
                    expires_at: "2026-07-08T00:00:00Z".to_owned(),
                    revoked_at: Some("2026-07-02T00:00:00Z".to_owned()),
                    this_device: false,
                },
            ],
            publish: None,
            publications: Vec::new(),
            live_session: None,
            workspaces: Vec::new(),
            verified_at: Some("2026-08-06T13:41:00Z".to_owned()),
            authorization_url: None,
        }
    }

    #[test]
    fn unconfigured_model_never_invents_identity_or_authority() {
        let model = AccountConsoleModel::project(
            None,
            &CloudSessionSnapshot::default(),
            CloudAccountAvailability::UnconfiguredBuild,
        );
        assert_eq!(model.strip.title, "Not signed in");
        assert!(model.strip.actions.is_empty(), "no endpoints, no sign-in");
        assert!(model.devices.is_empty());
        assert!(
            model
                .unlocks
                .iter()
                .all(|row| row.detail != "account entitlement")
        );
        let disclosed = format!("{model:?}");
        for fixture in ["James Whitfield", "Acme Engineering", "Chicago", "SAML"] {
            assert!(!disclosed.contains(fixture));
        }
    }

    #[test]
    fn signed_out_native_build_offers_exactly_sign_in() {
        let model = AccountConsoleModel::project(
            None,
            &CloudSessionSnapshot::default(),
            CloudAccountAvailability::Native,
        );
        assert_eq!(model.strip.actions, vec![AccountAction::SignIn]);
        assert_eq!(model.strip.badge.0, "LOCAL / OFFLINE");
    }

    #[test]
    fn active_session_projects_server_facts_only() {
        let cloud = active_cloud_snapshot();
        let model = AccountConsoleModel::project(None, &cloud, CloudAccountAvailability::Native);
        assert_eq!(model.strip.title, "Example Engineer");
        assert_eq!(model.strip.detail, "engineer@example.com");
        assert_eq!(model.strip.badge.0, "SIGNED IN");
        assert!(
            model
                .unlocks
                .iter()
                .any(|row| row.feature == "Web publishing"
                    && row.licensed
                    && row.term == "through 2027-07-01")
        );
        // Revoked leases never render; the foreign lease is revocable.
        assert_eq!(model.devices.len(), 2);
        assert_eq!(model.devices[0].revoke, None);
        assert_eq!(model.devices[1].revoke, Some("lease-b".to_owned()));
        assert!(model.boundary.contains("stays licensed through"));
    }

    #[test]
    fn expired_entitlements_grant_no_unlock_rows() {
        let mut cloud = active_cloud_snapshot();
        cloud.entitlements[0].status = "expired".to_owned();
        cloud.native_license = None;
        let model = AccountConsoleModel::project(None, &cloud, CloudAccountAvailability::Native);
        assert!(
            !model
                .unlocks
                .iter()
                .any(|row| row.feature == "Web publishing")
        );
    }

    #[test]
    fn local_license_rows_project_exact_verified_data() {
        let info = crate::services::license::parse_and_verify(crate::services::license::SAMPLE_KEY)
            .expect("signed sample license");
        let model = AccountConsoleModel::project(
            Some(&info),
            &CloudSessionSnapshot::default(),
            CloudAccountAvailability::UnconfiguredBuild,
        );
        let row = |label: &str| {
            model
                .license_rows
                .iter()
                .find(|(name, _)| name == label)
                .map(|(_, value)| value.clone())
                .expect(label)
        };
        assert_eq!(row("Licensed to"), info.licensed_to);
        assert_eq!(row("Tier"), info.tier);
        assert_eq!(row("Updates until"), info.updates_until);
        assert_eq!(row("License ID"), info.license_id);
        for feature in &info.features {
            assert!(
                model
                    .unlocks
                    .iter()
                    .any(|unlock| &unlock.feature == feature && unlock.licensed),
                "granted local feature must appear licensed: {feature}"
            );
        }
    }

    #[test]
    fn unix_date_matches_known_calendar_points() {
        assert_eq!(unix_date(0), "1970-01-01");
        assert_eq!(unix_date(1_785_604_800), "2026-08-01");
        assert_eq!(unix_date(946_684_800), "2000-01-01");
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
    fn every_exposed_action_has_a_real_executor() {
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

        let mut help_app = RSpiceApp::test_instance();
        execute_action(&mut help_app, AccountAction::SupportBundle);
        assert!(help_app.state.dialogs.help_center.open);

        // Cloud session actions delegate to the account service; on an
        // unconfigured test instance they must be accepted no-ops.
        let mut cloud_app = RSpiceApp::test_instance();
        for action in [
            AccountAction::SignIn,
            AccountAction::ReopenSignInPage,
            AccountAction::CancelSignIn,
            AccountAction::RefreshSession,
            AccountAction::RevokeLease("lease".to_owned()),
            AccountAction::SignOut,
        ] {
            execute_action(&mut cloud_app, action);
        }
        assert!(!cloud_app.cloud_account.snapshot().signed_in());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn typed_command_and_rendered_surface_disclose_the_same_offline_boundary() {
        use crate::workbench::commands::vocabulary::{COMMAND_REGISTRY, Command};

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

        let output = ctx.run_ui(
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
            "Not signed in",
            "This build carries no cloud account endpoints.",
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
