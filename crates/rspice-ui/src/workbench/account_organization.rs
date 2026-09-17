//! Account and administration.
//!
//! One narrow column read top to bottom — who is signed in, the license this
//! installation runs under, what it unlocks, the devices holding a lease,
//! what leaves this machine, and the build — with every section stating only
//! facts the application can prove. Provable sources are exactly three: the
//! verified local license file, the cloud account session (server-verified
//! principal, entitlements, and license leases from `services::cloud_account`),
//! and this build/process. Anything without a backing authority is left out
//! rather than rendered as an inert or simulated control, and every exposed
//! action routes to a real executor.

use egui::{Align2, Color32, Rect, Sense, Stroke, Ui, Vec2, pos2, vec2};

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

use super::design_system::elide_text;
use super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

const ACCOUNT_TITLE: &str = "Account and administration";
const ACCOUNT_DESCRIPTION: &str = "The account this installation is signed in with, the license it runs under, what leaves this machine, and the build it runs.";
/// Side inset of every section.
const SECTION_INSET: f32 = 20.0;
const SECTION_TOP: f32 = 16.0;
const SECTION_BOTTOM: f32 = 16.0;
const AVATAR_SIZE: f32 = 40.0;
/// Below this body width a list row's action moves under its name, and the
/// identity block's state moves under the account name.
const LIST_STACK_WIDTH: f32 = 420.0;

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tone {
    Neutral,
    Info,
    Ok,
    Warn,
}

impl Tone {
    fn color(self, tokens: &Tokens) -> Color32 {
        match self {
            Self::Neutral => tokens.color.text_dim,
            Self::Info => tokens.color.info,
            Self::Ok => tokens.color.ok,
            Self::Warn => tokens.color.warn,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IdentityModel {
    title: String,
    detail: String,
    note: String,
    initials: Option<String>,
    status: (&'static str, Tone),
    error: Option<String>,
    actions: Vec<AccountAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FeatureRow {
    name: String,
    detail: String,
    licensed: bool,
    term: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeviceRow {
    name: String,
    detail: String,
    /// Lease ID to revoke; `None` marks this installation's own row.
    revoke: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AccountConsoleModel {
    identity: IdentityModel,
    license_rows: Vec<(String, String)>,
    license_status: (String, Tone),
    license_action: AccountAction,
    features: Vec<FeatureRow>,
    devices: Vec<DeviceRow>,
    privacy_rows: Vec<(String, String)>,
    about_rows: Vec<(String, String)>,
    boundary: String,
}

impl AccountConsoleModel {
    fn project(
        license: Option<&LicenseInfo>,
        cloud: &CloudSessionSnapshot,
        availability: CloudAccountAvailability,
    ) -> Self {
        Self {
            identity: identity_model(cloud, availability),
            license_rows: license_rows(license, cloud),
            license_status: license_status(license),
            license_action: if license.is_some() {
                AccountAction::ManageLicense
            } else {
                AccountAction::ActivateLicense
            },
            features: feature_rows(license, cloud),
            devices: device_rows(cloud),
            privacy_rows: privacy_rows(availability),
            about_rows: vec![
                (
                    "Version".to_owned(),
                    concat!("RSpice ", env!("CARGO_PKG_VERSION")).to_owned(),
                ),
                ("Platform".to_owned(), current_platform_label().to_owned()),
            ],
            boundary: boundary_statement(license, cloud),
        }
    }
}

fn identity_model(
    cloud: &CloudSessionSnapshot,
    availability: CloudAccountAvailability,
) -> IdentityModel {
    let signed_out = |detail: &str, note: &str, error, actions| IdentityModel {
        title: "Not signed in".to_owned(),
        detail: detail.to_owned(),
        note: note.to_owned(),
        initials: None,
        status: ("Local only", Tone::Neutral),
        error,
        actions,
    };
    match availability {
        CloudAccountAvailability::UnconfiguredBuild => signed_out(
            "This build can't connect to RSpice Cloud.",
            "Cloud licensing, web publishing, and live collaboration are unavailable.",
            None,
            Vec::new(),
        ),
        CloudAccountAvailability::Native | CloudAccountAvailability::Browser => {
            match &cloud.phase {
                CloudSessionPhase::SignedOut { last_error } => signed_out(
                    "Sign in to use cloud licensing, web publishing, and live collaboration.",
                    "Signing in opens your browser; RSpice never sees your password.",
                    last_error.clone(),
                    vec![AccountAction::SignIn],
                ),
                CloudSessionPhase::WaitingForBrowser => IdentityModel {
                    title: "Finish signing in".to_owned(),
                    detail: "Complete sign-in in the browser window RSpice opened.".to_owned(),
                    note: "Nothing changes here until the browser hands the sign-in back."
                        .to_owned(),
                    initials: None,
                    status: ("Signing in", Tone::Info),
                    error: None,
                    actions: vec![AccountAction::ReopenSignInPage, AccountAction::CancelSignIn],
                },
                CloudSessionPhase::ExchangingTokens | CloudSessionPhase::Bootstrapping => {
                    IdentityModel {
                        title: "Signing in\u{2026}".to_owned(),
                        detail: "Setting up your account session.".to_owned(),
                        note: String::new(),
                        initials: None,
                        status: ("Signing in", Tone::Info),
                        error: None,
                        actions: Vec::new(),
                    }
                }
                CloudSessionPhase::Active => {
                    let (title, detail) = identity_lines(cloud);
                    IdentityModel {
                        initials: Some(initials(&title)),
                        note: cloud
                            .verified_at
                            .as_deref()
                            .map(|stamp| format!("Account verified {}", humanize_stamp(stamp)))
                            .unwrap_or_default(),
                        title,
                        detail,
                        status: ("Signed in", Tone::Ok),
                        error: None,
                        actions: vec![AccountAction::RefreshSession, AccountAction::SignOut],
                    }
                }
                CloudSessionPhase::OfflineLicensed => {
                    let (title, detail) = identity_lines(cloud);
                    let note = cloud
                        .native_license
                        .as_ref()
                        .map(|license| {
                            format!(
                                "Offline \u{2014} this device stays licensed through {}",
                                unix_date(license.expires_at_unix_seconds)
                            )
                        })
                        .unwrap_or_default();
                    IdentityModel {
                        initials: Some(initials(&title)),
                        title,
                        detail,
                        note,
                        status: ("Offline", Tone::Warn),
                        error: None,
                        actions: vec![AccountAction::RefreshSession, AccountAction::SignOut],
                    }
                }
            }
        }
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

fn license_status(license: Option<&LicenseInfo>) -> (String, Tone) {
    match license {
        Some(info) if info.updates_expired => {
            ("Active \u{b7} updates ended".to_owned(), Tone::Warn)
        }
        Some(_) => ("Active".to_owned(), Tone::Ok),
        None => ("Not activated".to_owned(), Tone::Neutral),
    }
}

fn license_rows(
    license: Option<&LicenseInfo>,
    cloud: &CloudSessionSnapshot,
) -> Vec<(String, String)> {
    let mut rows = match license {
        Some(info) => vec![
            (
                "Status".to_owned(),
                if info.updates_expired {
                    "Verified \u{b7} perpetual use kept \u{b7} updates window ended".to_owned()
                } else {
                    "Verified on this device".to_owned()
                },
            ),
            ("Licensed to".to_owned(), info.licensed_to.clone()),
            ("Edition".to_owned(), info.tier.clone()),
            ("Updates until".to_owned(), info.updates_until.clone()),
            ("License ID".to_owned(), info.license_id.clone()),
        ],
        None => vec![("Status".to_owned(), "No license on this device".to_owned())],
    };
    if let Some(native) = &cloud.native_license {
        rows.push((
            "Cloud plan".to_owned(),
            format!("{} \u{b7} {}", native.product, native.plan),
        ));
        rows.push((
            "Offline until".to_owned(),
            unix_date(native.expires_at_unix_seconds),
        ));
    }
    rows.push(("License file".to_owned(), license_storage_description()));
    rows
}

/// Cloud entitlement feature keys mapped to the product language, so the
/// feature list names shipped capabilities rather than wire identifiers.
fn cloud_feature_display(key: &str) -> (String, String) {
    match key {
        "cloud_publishing" => (
            "Web publishing".to_owned(),
            "Published circuit pages on RSpice Cloud".to_owned(),
        ),
        "live_collaboration" => (
            "Live collaboration".to_owned(),
            "Live sessions on RSpice Cloud".to_owned(),
        ),
        "cloud_simulation" => (
            "Cloud simulation".to_owned(),
            "Runs queued to RSpice Cloud workers".to_owned(),
        ),
        "native_license" => (
            "Offline desktop license".to_owned(),
            "A verified lease keeps this device licensed offline".to_owned(),
        ),
        other => (other.to_owned(), "Account entitlement".to_owned()),
    }
}

fn feature_rows(license: Option<&LicenseInfo>, cloud: &CloudSessionSnapshot) -> Vec<FeatureRow> {
    let mut rows = vec![FeatureRow {
        name: "Schematic capture & simulation".to_owned(),
        detail: "DC \u{b7} AC \u{b7} transient \u{b7} noise \u{b7} S-parameter".to_owned(),
        licensed: true,
        term: "Included".to_owned(),
    }];

    // The local license's feature catalog: every label the signer can grant,
    // with this license's actual grants marked.
    for label in crate::services::license::FEATURE_LABELS
        .iter()
        .map(|(_, label)| *label)
    {
        let granted = license.is_some_and(|info| info.features.iter().any(|f| f == label));
        rows.push(FeatureRow {
            name: label.to_owned(),
            detail: String::new(),
            licensed: granted,
            term: if granted { "Perpetual" } else { "Not licensed" }.to_owned(),
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
            let (name, detail) = cloud_feature_display(&key);
            rows.push(FeatureRow {
                name,
                detail,
                licensed: true,
                term: match valid_until {
                    Some(stamp) => format!("Through {}", humanize_stamp(&stamp)),
                    None => "With subscription".to_owned(),
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
            name: if lease.this_device {
                "This device".to_owned()
            } else {
                format!("Device {}", short_id(&lease.id))
            },
            detail: format!(
                "{} plan \u{b7} issued {} \u{b7} renews by {}",
                capitalized(&lease.plan),
                humanize_stamp(&lease.issued_at),
                humanize_stamp(&lease.expires_at)
            ),
            revoke: if lease.this_device {
                None
            } else {
                Some(lease.id.clone())
            },
        })
        .collect()
}

fn privacy_rows(availability: CloudAccountAvailability) -> Vec<(String, String)> {
    let network = match availability {
        CloudAccountAvailability::Native => "Sign-in, licensing, and RSpice Cloud only",
        CloudAccountAvailability::Browser => "Sign-in and live collaboration only",
        CloudAccountAvailability::UnconfiguredBuild => "None",
    };
    vec![
        (
            "Usage data".to_owned(),
            "None \u{2014} nothing is sent".to_owned(),
        ),
        ("Network access".to_owned(), network.to_owned()),
        ("Project files".to_owned(), "Stay on this device".to_owned()),
    ]
}

fn boundary_statement(license: Option<&LicenseInfo>, cloud: &CloudSessionSnapshot) -> String {
    if let Some(native) = &cloud.native_license {
        return format!(
            "Without the licensing service this device stays licensed through {}. Projects, \
             results, and reports stay readable after that; a license gates new work, never \
             your existing files.",
            unix_date(native.expires_at_unix_seconds)
        );
    }
    if license.is_some() {
        return "The license is verified on this device; nothing about your projects is sent \
                to check it."
            .to_owned();
    }
    "Projects, results, and reports on this device stay readable without a license or account."
        .to_owned()
}

fn short_id(id: &str) -> String {
    id.chars().take(8).collect()
}

fn capitalized(text: &str) -> String {
    let mut characters = text.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect(),
        None => String::new(),
    }
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
    ActivateLicense,
    ManageLicense,
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
            Self::PersonalPreferences => "Preferences\u{2026}",
            Self::ActivateLicense => "Activate license\u{2026}",
            Self::ManageLicense => "Manage license\u{2026}",
            Self::SignIn => "Sign in\u{2026}",
            Self::ReopenSignInPage => "Open sign-in page again",
            Self::CancelSignIn => "Cancel",
            Self::SignOut => "Sign out",
            Self::RefreshSession => "Refresh",
            Self::RevokeLease(_) => "Revoke",
            Self::LegalPrivacy => "Legal and privacy\u{2026}",
            Self::SupportBundle => "Create support bundle\u{2026}",
            Self::HelpCenter => "Help center\u{2026}",
        }
    }

    /// The one action a section leads with.
    const fn is_primary(&self) -> bool {
        matches!(self, Self::SignIn | Self::ActivateLicense)
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
        AccountAction::ActivateLicense | AccountAction::ManageLicense => {
            app.open_license_dialog();
        }
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
///
/// Routing is a state transition, so this takes the state rather than the
/// application: the navigation and the message it may push are both
/// [`AppState`]'s, and nothing here needs the cloud session or the dialogs
/// the whole application would also hand over.
pub(crate) fn open(state: &mut AppState) {
    let route = SurfaceRoute::surface(SurfaceId::AccountOrganization);
    if let Err(error) = state
        .workbench
        .navigate(route, RouteTransitionSource::User)
    {
        state.push_user_message(ConsoleMessage::warning(error.to_string()));
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
    let choice = Dialog::prompt(ACCOUNT_TITLE, "Close")
        .description(ACCOUNT_DESCRIPTION)
        .size(DialogSize::AccountManager)
        .secondary(AccountAction::PersonalPreferences.label())
        .secondary_leading()
        .flush_body()
        .show(ctx, |ui| {
            identity_section(ui, &model.identity, &mut requested_action);
            section_rule(ui);
            license_section(ui, &model, &mut requested_action);
            section_rule(ui);
            features_section(ui, &model.features);
            if !model.devices.is_empty() {
                section_rule(ui);
                devices_section(ui, &model.devices, &mut requested_action);
            }
            section_rule(ui);
            privacy_section(ui, &model.privacy_rows, &mut requested_action);
            section_rule(ui);
            about_section(ui, &model.about_rows, &mut requested_action);
        });

    match choice {
        DialogChoice::Primary | DialogChoice::Cancelled | DialogChoice::Ghost => {
            close_to_source(&mut app.state);
            return;
        }
        DialogChoice::Secondary => requested_action = Some(AccountAction::PersonalPreferences),
        _ => {}
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

/// The hairline between two sections.
fn section_rule(ui: &mut Ui) {
    let tokens = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(
        (rect.left() + SECTION_INSET)..=(rect.right() - SECTION_INSET),
        rect.center().y,
        Stroke::new(1.0, tokens.color.border),
    );
}

/// Run `body` inside one section's inset, with its vertical breathing room.
fn section<R>(ui: &mut Ui, body: impl FnOnce(&mut Ui, f32) -> R) -> R {
    ui.add_space(SECTION_TOP);
    let width = (ui.available_width() - 2.0 * SECTION_INSET).max(1.0);
    let output = ui
        .horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(SECTION_INSET);
            ui.allocate_ui_with_layout(
                vec2(width, 1.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(width);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    body(ui, width)
                },
            )
            .inner
        })
        .inner;
    ui.add_space(SECTION_BOTTOM);
    output
}

/// A section's heading: its name, and on the right either a short state or
/// the one action the section offers.
fn section_heading(
    ui: &mut Ui,
    title: &str,
    trailing: Option<(&str, Color32)>,
    action: Option<&AccountAction>,
    requested: &mut Option<AccountAction>,
) {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let button = action.map(|action| {
        let button = Button::new(action.label());
        if action.is_primary() {
            button.accent()
        } else {
            button
        }
    });
    let button_width = button
        .as_ref()
        .map_or(0.0, |button| button.measured_width(ui));
    let height = if button.is_some() && tokens.metrics.is_touch() {
        tokens.metrics.ctl_h.max(tokens::TOUCH_TARGET)
    } else {
        tokens.metrics.ctl_h
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let title_font = theme::sans(tokens::FS_2, FontWeight::SemiBold);
    let title_rect = ui.painter().text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        title,
        title_font,
        tokens.color.text,
    );
    if let Some((text, color)) = trailing {
        let font = theme::sans(tokens::FS_1, FontWeight::Regular);
        let right = rect.right() - button_width - if button.is_some() { 12.0 } else { 0.0 };
        let text = elide_text(
            ui,
            text,
            &font,
            (right - title_rect.right() - 16.0).max(1.0),
        );
        ui.painter().text(
            pos2(right, rect.center().y),
            Align2::RIGHT_CENTER,
            text,
            font,
            color,
        );
    }
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    if let (Some(button), Some(action)) = (button, action) {
        let slot = Rect::from_min_size(
            pos2(rect.right() - button_width, rect.top()),
            vec2(button_width, rect.height()),
        );
        if button_in(ui, slot, button) {
            *requested = Some(action.clone());
        }
    }
}

/// Show `button` centred in `slot`, a part of a row the caller has already
/// allocated. A child, not a scope: a scope moves the column's cursor to the
/// bottom of the button, and when that is above the bottom of the row the
/// next row starts inside this one.
fn button_in(ui: &mut Ui, slot: Rect, button: Button<'_>) -> bool {
    let mut slot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(slot)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    button.show(&mut slot_ui).clicked()
}

fn identity_section(ui: &mut Ui, identity: &IdentityModel, requested: &mut Option<AccountAction>) {
    let tokens = Tokens::get(ui.ctx());
    section(ui, |ui, width| {
        let status_font = theme::sans(tokens::FS_1, FontWeight::Medium);
        let status_color = identity.status.1.color(&tokens);
        let status_width = ui
            .painter()
            .layout_no_wrap(
                identity.status.0.to_owned(),
                status_font.clone(),
                status_color,
            )
            .size()
            .x
            + 14.0;
        let text_left = AVATAR_SIZE + 14.0;
        // Narrow, the state takes its own line under the name rather than
        // squeezing the name and its sentences into a column beside it.
        let narrow = width < LIST_STACK_WIDTH;
        let text_width = if narrow {
            width - text_left
        } else {
            width - text_left - status_width - 12.0
        }
        .max(1.0);
        let status_line = if narrow { 20.0 } else { 0.0 };

        let title_font = theme::sans(tokens::FS_4, FontWeight::SemiBold);
        let detail_font = theme::sans(tokens::FS_1, FontWeight::Regular);
        let detail = ui.painter().layout(
            identity.detail.clone(),
            detail_font.clone(),
            tokens.color.text_dim,
            text_width,
        );
        let note = (!identity.note.is_empty()).then(|| {
            ui.painter().layout(
                identity.note.clone(),
                detail_font,
                tokens.color.text_faint,
                text_width,
            )
        });
        let text_height = 22.0
            + status_line
            + if identity.detail.is_empty() {
                0.0
            } else {
                detail.size().y + 2.0
            }
            + note.as_ref().map_or(0.0, |note| note.size().y + 2.0);
        let (rect, response) =
            ui.allocate_exact_size(vec2(width, text_height.max(AVATAR_SIZE)), Sense::hover());
        avatar(
            ui,
            Rect::from_min_size(rect.left_top(), Vec2::splat(AVATAR_SIZE)),
            identity.initials.as_deref(),
        );
        let text_x = rect.left() + text_left;
        let title = elide_text(ui, &identity.title, &title_font, text_width);
        ui.painter().text(
            pos2(text_x, rect.top() + 1.0),
            Align2::LEFT_TOP,
            title,
            title_font,
            tokens.color.text,
        );
        let mut y = rect.top() + 24.0 + status_line;
        if !identity.detail.is_empty() {
            let height = detail.size().y;
            ui.painter()
                .galley(pos2(text_x, y), detail, tokens.color.text_dim);
            y += height + 2.0;
        }
        if let Some(note) = note {
            ui.painter()
                .galley(pos2(text_x, y), note, tokens.color.text_faint);
        }
        // The session state, as a dot and a word: top-right, or under the
        // name when narrow.
        let (dot, label_anchor, align) = if narrow {
            let center_y = rect.top() + 24.0 + 8.0;
            (
                pos2(text_x + 3.5, center_y),
                pos2(text_x + 12.0, center_y),
                Align2::LEFT_CENTER,
            )
        } else {
            let center_y = rect.top() + 11.0;
            (
                pos2(rect.right() - status_width + 3.5, center_y),
                pos2(rect.right(), center_y),
                Align2::RIGHT_CENTER,
            )
        };
        ui.painter().text(
            label_anchor,
            align,
            identity.status.0,
            status_font,
            status_color,
        );
        ui.painter().circle_filled(dot, 3.5, status_color);
        let announced = [
            identity.title.as_str(),
            identity.detail.as_str(),
            identity.note.as_str(),
            identity.status.0,
        ]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(", ");
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announced)
        });

        if let Some(error) = &identity.error {
            ui.add_space(8.0);
            ui.horizontal_top(|ui| {
                ui.add_space(text_left);
                wrapped_text(
                    ui,
                    error,
                    theme::sans(tokens::FS_1, FontWeight::Regular),
                    tokens.color.err,
                );
            });
        }
        if !identity.actions.is_empty() {
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
                ui.add_space(text_left);
                for action in &identity.actions {
                    let button = Button::new(action.label());
                    let button = if action.is_primary() {
                        button.accent()
                    } else {
                        button
                    };
                    if button.show(ui).clicked() {
                        *requested = Some(action.clone());
                    }
                }
            });
        }
    });
}

fn license_section(
    ui: &mut Ui,
    model: &AccountConsoleModel,
    requested: &mut Option<AccountAction>,
) {
    let tokens = Tokens::get(ui.ctx());
    section(ui, |ui, _| {
        section_heading(
            ui,
            "License",
            Some((
                model.license_status.0.as_str(),
                model.license_status.1.color(&tokens),
            )),
            Some(&model.license_action),
            requested,
        );
        ui.add_space(6.0);
        for (label, value) in &model.license_rows {
            fact_row(ui, label, value);
        }
        ui.add_space(8.0);
        wrapped_text(
            ui,
            &model.boundary,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            tokens.color.text_faint,
        );
    });
}

fn features_section(ui: &mut Ui, features: &[FeatureRow]) {
    let tokens = Tokens::get(ui.ctx());
    let licensed = features.iter().filter(|row| row.licensed).count();
    section(ui, |ui, width| {
        section_heading(
            ui,
            "Features",
            Some((
                &format!("{licensed} of {} licensed", features.len()),
                tokens.color.text_faint,
            )),
            None,
            &mut None,
        );
        ui.add_space(4.0);
        for feature in features {
            let (term_color, mark_color) = if feature.licensed {
                (tokens.color.text_dim, tokens.color.ok)
            } else {
                (tokens.color.text_faint, tokens.color.text_faint)
            };
            list_row(
                ui,
                width,
                ListRow {
                    mark: Some((mark_color, feature.licensed)),
                    name: &feature.name,
                    name_color: if feature.licensed {
                        tokens.color.text
                    } else {
                        tokens.color.text_dim
                    },
                    detail: &feature.detail,
                    trailing: Trailing::Text(&feature.term, term_color),
                },
            );
        }
    });
}

fn devices_section(ui: &mut Ui, devices: &[DeviceRow], requested: &mut Option<AccountAction>) {
    let tokens = Tokens::get(ui.ctx());
    section(ui, |ui, width| {
        section_heading(
            ui,
            "Devices",
            Some((
                &format!("{} signed in", devices.len()),
                tokens.color.text_faint,
            )),
            None,
            &mut None,
        );
        ui.add_space(4.0);
        for device in devices {
            let trailing = match &device.revoke {
                Some(_) => Trailing::Action("Revoke"),
                None => Trailing::Text("This device", tokens.color.text_faint),
            };
            let clicked = list_row(
                ui,
                width,
                ListRow {
                    mark: None,
                    name: &device.name,
                    name_color: tokens.color.text,
                    detail: &device.detail,
                    trailing,
                },
            );
            if clicked && let Some(lease_id) = &device.revoke {
                *requested = Some(AccountAction::RevokeLease(lease_id.clone()));
            }
        }
    });
}

fn privacy_section(ui: &mut Ui, rows: &[(String, String)], requested: &mut Option<AccountAction>) {
    section(ui, |ui, _| {
        section_heading(
            ui,
            "Privacy",
            None,
            Some(&AccountAction::LegalPrivacy),
            requested,
        );
        ui.add_space(6.0);
        for (label, value) in rows {
            fact_row(ui, label, value);
        }
    });
}

fn about_section(ui: &mut Ui, rows: &[(String, String)], requested: &mut Option<AccountAction>) {
    section(ui, |ui, _| {
        section_heading(ui, "About", None, None, &mut None);
        ui.add_space(6.0);
        for (label, value) in rows {
            fact_row(ui, label, value);
        }
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
            for action in [AccountAction::HelpCenter, AccountAction::SupportBundle] {
                if Button::new(action.label()).show(ui).clicked() {
                    *requested = Some(action);
                }
            }
        });
    });
}

/// A label and its value on one line, in the section's own two columns.
fn fact_row(ui: &mut Ui, label: &str, value: &str) {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = 24.0;
    let label_width = (width * 0.34).clamp(96.0, 132.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let label_font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let value_font = theme::sans(tokens::FS_1, FontWeight::Regular);
    ui.painter().text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        elide_text(ui, label, &label_font, label_width - 12.0),
        label_font,
        tokens.color.text_faint,
    );
    let value_width = (width - label_width).max(1.0);
    let shown = elide_text(ui, value, &value_font, value_width);
    let elided = shown != value;
    ui.painter().text(
        pos2(rect.left() + label_width, rect.center().y),
        Align2::LEFT_CENTER,
        shown,
        value_font,
        tokens.color.text,
    );
    let announced = format!("{label}: {value}");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announced)
    });
    if elided {
        response.on_hover_text(value);
    }
}

enum Trailing<'a> {
    Text(&'a str, Color32),
    Action(&'a str),
}

struct ListRow<'a> {
    /// A dot before the name: its colour, and whether it is filled.
    mark: Option<(Color32, bool)>,
    name: &'a str,
    name_color: Color32,
    detail: &'a str,
    trailing: Trailing<'a>,
}

/// One entry of a section's list: a name with an optional detail under it,
/// and a state or an action on the right. Returns whether the action was
/// clicked.
fn list_row(ui: &mut Ui, width: f32, row: ListRow<'_>) -> bool {
    let tokens = Tokens::get(ui.ctx());
    let name_font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let detail_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let trailing_font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let button = match row.trailing {
        Trailing::Action(label) => Some(Button::new(label).destructive(true)),
        Trailing::Text(..) => None,
    };
    let trailing_width = match (&row.trailing, &button) {
        (_, Some(button)) => button.measured_width(ui),
        (Trailing::Text(text, color), None) => {
            ui.painter()
                .layout_no_wrap((*text).to_owned(), trailing_font.clone(), *color)
                .size()
                .x
        }
        (Trailing::Action(_), None) => 0.0,
    };
    // A short state keeps its place on the right at any width; a button
    // drops under the name once the two no longer fit side by side.
    let stacked = button.is_some() && width < LIST_STACK_WIDTH;
    let control_height = if button.is_some() {
        tokens.metrics.ctl_h.max(if tokens.metrics.is_touch() {
            tokens::TOUCH_TARGET
        } else {
            0.0
        })
    } else {
        0.0
    };
    let mark_width = if row.mark.is_some() { 16.0 } else { 0.0 };
    let text_left = mark_width;
    let text_right = if stacked {
        width
    } else {
        width - trailing_width - 16.0
    };
    let text_width = (text_right - text_left).max(1.0);
    let has_detail = !row.detail.is_empty();
    let text_height = if has_detail { 34.0 } else { 18.0 };
    let height = if stacked {
        8.0 + text_height + 4.0 + control_height.max(18.0) + 8.0
    } else {
        (text_height + 12.0).max(control_height + 8.0)
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let text_top = if stacked {
        rect.top() + 8.0
    } else {
        rect.center().y - text_height * 0.5
    };
    if let Some((color, filled)) = row.mark {
        let center = pos2(rect.left() + 4.0, text_top + 9.0);
        if filled {
            ui.painter().circle_filled(center, 3.5, color);
        } else {
            ui.painter()
                .circle_stroke(center, 3.0, Stroke::new(1.0, color));
        }
    }
    ui.painter().text(
        pos2(rect.left() + text_left, text_top),
        Align2::LEFT_TOP,
        elide_text(ui, row.name, &name_font, text_width),
        name_font,
        row.name_color,
    );
    if has_detail {
        ui.painter().text(
            pos2(rect.left() + text_left, text_top + 18.0),
            Align2::LEFT_TOP,
            elide_text(ui, row.detail, &detail_font, text_width),
            detail_font,
            tokens.color.text_faint,
        );
    }
    let trailing_rect = if stacked {
        Rect::from_min_size(
            pos2(
                rect.left() + text_left,
                rect.bottom() - 8.0 - control_height.max(18.0),
            ),
            vec2(trailing_width, control_height.max(18.0)),
        )
    } else {
        Rect::from_min_max(
            pos2(rect.right() - trailing_width, rect.top()),
            rect.right_bottom(),
        )
    };
    let announced = match &row.trailing {
        Trailing::Text(text, _) => format!("{}, {}, {text}", row.name, row.detail),
        Trailing::Action(_) => format!("{}, {}", row.name, row.detail),
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announced)
    });
    match (row.trailing, button) {
        (_, Some(button)) => button_in(ui, trailing_rect, button),
        (Trailing::Text(text, color), None) => {
            ui.painter().text(
                trailing_rect.left_center(),
                Align2::LEFT_CENTER,
                text,
                trailing_font,
                color,
            );
            false
        }
        (Trailing::Action(_), None) => false,
    }
}

/// Wrapped copy, painted as text rather than laid out as a selectable label.
fn wrapped_text(ui: &mut Ui, text: &str, font: egui::FontId, color: Color32) {
    let galley = ui
        .painter()
        .layout(text.to_owned(), font, color, ui.available_width().max(1.0));
    let (rect, response) = ui.allocate_exact_size(galley.size(), Sense::hover());
    ui.painter().galley(rect.min, galley, color);
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
}

/// The account's initials in a circle, or a person mark when no one is
/// signed in.
fn avatar(ui: &Ui, rect: Rect, initials: Option<&str>) {
    let tokens = Tokens::get(ui.ctx());
    let radius = rect.width() * 0.5;
    match initials {
        Some(initials) => {
            ui.painter().circle_filled(
                rect.center(),
                radius,
                tokens.color.accent.gamma_multiply(0.18),
            );
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                initials,
                theme::sans(tokens::FS_2, FontWeight::SemiBold),
                tokens.color.accent,
            );
        }
        None => {
            ui.painter()
                .circle_filled(rect.center(), radius, tokens.color.bg_elevated);
            ui.painter().circle_stroke(
                rect.center(),
                radius - 0.5,
                Stroke::new(1.0, tokens.color.border),
            );
            // A head over a pair of shoulders, drawn to the circle's scale.
            let stroke = Stroke::new(1.5, tokens.color.text_faint);
            let unit = radius / 20.0;
            let center = rect.center();
            ui.painter()
                .circle_stroke(center + vec2(0.0, -4.0 * unit), 4.5 * unit, stroke);
            let shoulders = (0..=16)
                .map(|step| {
                    let angle = std::f32::consts::PI * (1.0 + step as f32 / 16.0);
                    center + vec2(0.0, 11.0 * unit) + 8.5 * unit * vec2(angle.cos(), angle.sin())
                })
                .collect::<Vec<_>>();
            ui.painter().add(egui::Shape::line(shoulders, stroke));
        }
    }
}

fn initials(value: &str) -> String {
    let mut letters = value
        .split_whitespace()
        .filter_map(|part| part.chars().find(char::is_ascii_alphanumeric))
        .take(2)
        .collect::<String>()
        .to_uppercase();
    if letters.is_empty() {
        letters.push('\u{2014}');
    }
    letters
}

#[cfg(target_arch = "wasm32")]
fn license_storage_description() -> String {
    "Browser session storage".to_owned()
}

#[cfg(not(target_arch = "wasm32"))]
fn license_storage_description() -> String {
    crate::services::license::license_file_path().map_or_else(
        || "Configuration folder unavailable".to_owned(),
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

    fn sample_license() -> LicenseInfo {
        crate::services::license::parse_and_verify(crate::services::license::SAMPLE_KEY)
            .expect("signed sample license")
    }

    #[test]
    fn unconfigured_model_never_invents_identity_or_authority() {
        let model = AccountConsoleModel::project(
            None,
            &CloudSessionSnapshot::default(),
            CloudAccountAvailability::UnconfiguredBuild,
        );
        assert_eq!(model.identity.title, "Not signed in");
        assert!(
            model.identity.actions.is_empty(),
            "no endpoints, no sign-in"
        );
        assert!(model.identity.initials.is_none());
        assert!(model.devices.is_empty());
        assert!(
            model
                .features
                .iter()
                .all(|row| row.detail != "Account entitlement")
        );
        assert_eq!(model.license_action, AccountAction::ActivateLicense);
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
        assert_eq!(model.identity.actions, vec![AccountAction::SignIn]);
        assert_eq!(model.identity.status, ("Local only", Tone::Neutral));
    }

    #[test]
    fn active_session_projects_server_facts_only() {
        let cloud = active_cloud_snapshot();
        let model = AccountConsoleModel::project(None, &cloud, CloudAccountAvailability::Native);
        assert_eq!(model.identity.title, "Example Engineer");
        assert_eq!(model.identity.detail, "engineer@example.com");
        assert_eq!(model.identity.initials.as_deref(), Some("EE"));
        assert_eq!(model.identity.status, ("Signed in", Tone::Ok));
        assert!(model.features.iter().any(|row| row.name == "Web publishing"
            && row.licensed
            && row.term == "Through 2027-07-01"));
        // Revoked leases never render; the foreign lease is revocable.
        assert_eq!(model.devices.len(), 2);
        assert_eq!(model.devices[0].name, "This device");
        assert_eq!(model.devices[0].revoke, None);
        assert_eq!(model.devices[1].revoke, Some("lease-b".to_owned()));
        assert!(model.boundary.contains("stays licensed through"));
    }

    #[test]
    fn expired_entitlements_grant_no_feature_rows() {
        let mut cloud = active_cloud_snapshot();
        cloud.entitlements[0].status = "expired".to_owned();
        cloud.native_license = None;
        let model = AccountConsoleModel::project(None, &cloud, CloudAccountAvailability::Native);
        assert!(
            !model
                .features
                .iter()
                .any(|row| row.name == "Web publishing")
        );
    }

    #[test]
    fn local_license_rows_project_exact_verified_data() {
        let info = sample_license();
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
        assert_eq!(row("Edition"), info.tier);
        assert_eq!(row("Updates until"), info.updates_until);
        assert_eq!(row("License ID"), info.license_id);
        assert_eq!(model.license_action, AccountAction::ManageLicense);
        for feature in &info.features {
            assert!(
                model
                    .features
                    .iter()
                    .any(|row| &row.name == feature && row.licensed),
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

        for action in [AccountAction::ActivateLicense, AccountAction::ManageLicense] {
            let mut license_app = RSpiceApp::test_instance();
            execute_action(&mut license_app, action);
            assert!(license_app.state.dialogs.license_dialog.open);
        }

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

    /// Draw the console for `app` at `size` and collect every painted string,
    /// plus every accessible name and value, which also covers the rows the
    /// body has scrolled out of view.
    #[cfg(not(target_arch = "wasm32"))]
    fn painted_console(app: &mut RSpiceApp, size: egui::Vec2) -> String {
        fn collect(shape: &egui::epaint::Shape, rendered: &mut String) {
            match shape {
                egui::epaint::Shape::Text(text) => {
                    rendered.push_str(&text.galley.job.text);
                    rendered.push('\n');
                }
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect(shape, rendered);
                    }
                }
                _ => {}
            }
        }
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut rendered = String::new();
        // A content-height dialog settles its size over its first passes.
        for _ in 0..4 {
            rendered.clear();
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| show(ctx, app),
            );
            for shape in &output.shapes {
                collect(&shape.shape, &mut rendered);
            }
            for (_, node) in output
                .platform_output
                .accesskit_update
                .iter()
                .flat_map(|update| update.nodes.iter())
            {
                for text in [node.label(), node.value()].into_iter().flatten() {
                    rendered.push_str(text);
                    rendered.push('\n');
                }
            }
        }
        rendered
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn signed_in_app() -> RSpiceApp {
        signed_in_app_with(active_cloud_snapshot())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn signed_in_app_with(snapshot: CloudSessionSnapshot) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.cloud_account = crate::services::cloud_account::CloudAccountService::with_snapshot(
            CloudAccountAvailability::Native,
            snapshot,
        );
        app.state.license = Some(sample_license());
        app.state
            .workbench
            .navigate(account_route(), RouteTransitionSource::User)
            .expect("account manager route is executable");
        app
    }

    /// Every section reaches the reader. The right-aligned action row that
    /// used to follow the license facts took all the height left below it,
    /// which pushed every later section out of the dialog.
    ///
    /// A phone sheet as tall as the whole console shows every control; on the
    /// desktop the body scrolls, and a button scrolled out of view is not
    /// drawn, so there only the sections themselves are required.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_section_of_a_signed_in_console_is_painted() {
        const BELOW_THE_DESKTOP_FOLD: [&str; 3] = [
            "Revoke",
            "Help center\u{2026}",
            "Create support bundle\u{2026}",
        ];
        for size in [egui::vec2(1_440.0, 1_400.0), egui::vec2(390.0, 2_400.0)] {
            let mut app = signed_in_app();
            let text = painted_console(&mut app, size);
            let scrolls = size.x > 560.0;
            for expected in [
                "Account and administration",
                "Example Engineer",
                "engineer@example.com",
                "Signed in",
                "Refresh",
                "Sign out",
                "License",
                "Manage license\u{2026}",
                "Licensed to",
                "Features",
                "Web publishing",
                "Through 2027-07-01",
                "Devices",
                "This device",
                "Device lease-b",
                "Revoke",
                "Privacy",
                "Legal and privacy\u{2026}",
                "About",
                "Help center\u{2026}",
                "Create support bundle\u{2026}",
                "Preferences\u{2026}",
                "Close",
            ]
            .into_iter()
            .filter(|expected| !(scrolls && BELOW_THE_DESKTOP_FOLD.contains(expected)))
            {
                assert!(
                    text.contains(expected),
                    "{expected:?} missing at {}x{}:\n{text}",
                    size.x,
                    size.y
                );
            }
            for retired in [
                "ACCOUNT \u{b7} LICENSE \u{b7} DATA",
                "LOCAL / OFFLINE",
                "SIGNED IN",
                "Personal preferences\u{2026}",
                "License & activation\u{2026}",
                "Location not collected",
                "FEATURE",
            ] {
                assert!(
                    !text.contains(retired),
                    "{retired:?} is still painted at {}x{}:\n{text}",
                    size.x,
                    size.y
                );
            }
        }
    }

    /// Device rows follow one another without overlapping, including on a
    /// phone, where Revoke sits under the device name. That button was laid
    /// out in a scope, which pulled the list's cursor back up to its bottom
    /// edge, eight points above the bottom of the row, so the row after a
    /// revocable device started inside it.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn device_rows_follow_one_another_without_overlapping() {
        let mut snapshot = active_cloud_snapshot();
        snapshot.device_leases.push(LeaseSummary {
            id: "lease-c".to_owned(),
            plan: "professional".to_owned(),
            issued_at: "2026-07-28T00:00:00Z".to_owned(),
            expires_at: "2026-08-04T00:00:00Z".to_owned(),
            revoked_at: None,
            this_device: false,
        });
        for size in [egui::vec2(1_440.0, 1_400.0), egui::vec2(390.0, 2_400.0)] {
            let mut app = signed_in_app_with(snapshot.clone());
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let mut rows = Vec::new();
            for _ in 0..4 {
                let output = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                        ..Default::default()
                    },
                    |ctx| show(ctx, &mut app),
                );
                rows = output
                    .platform_output
                    .accesskit_update
                    .iter()
                    .flat_map(|update| update.nodes.iter())
                    .filter(|(_, node)| {
                        [node.label(), node.value()]
                            .into_iter()
                            .flatten()
                            .any(|text| text.contains(" plan \u{b7} issued "))
                    })
                    .filter_map(|(_, node)| node.bounds())
                    .map(|bounds| (bounds.y0, bounds.y1))
                    .collect::<Vec<_>>();
            }
            rows.sort_by(|a, b| a.0.total_cmp(&b.0));
            assert_eq!(rows.len(), 3, "{rows:?} at {}x{}", size.x, size.y);
            for pair in rows.windows(2) {
                assert!(
                    (pair[1].0 - pair[0].1).abs() < 0.5,
                    "rows {pair:?} overlap or part at {}x{}",
                    size.x,
                    size.y
                );
            }
        }
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
        // Controls carry their text as a name; static text carries it as a
        // value.
        let labels = nodes
            .iter()
            .flat_map(|(_, node)| [node.label(), node.value()])
            .flatten()
            .collect::<Vec<_>>();
        for expected in [
            "Account and administration",
            "Preferences\u{2026}",
            "Activate license\u{2026}",
        ] {
            assert!(
                labels.contains(&expected),
                "missing rendered label: {expected}; labels: {labels:?}"
            );
        }
        assert!(
            labels.iter().any(|label| label.contains("Not signed in")
                && label.contains("This build can't connect to RSpice Cloud.")),
            "the identity block must state the offline boundary; labels: {labels:?}"
        );
        for fixture in ["James Whitfield", "Acme Engineering", "Chicago", "SAML"] {
            assert!(
                !labels.iter().any(|label| label.contains(fixture)),
                "fixture leaked into account UI"
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_the_account_console_for_review() {
        use crate::services::cloud_account::CloudAccountService;
        use std::io::Write as _;

        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        let states: [(&str, CloudAccountAvailability, CloudSessionSnapshot, bool); 4] = [
            (
                "unconfigured",
                CloudAccountAvailability::UnconfiguredBuild,
                CloudSessionSnapshot::default(),
                false,
            ),
            (
                "signed-out",
                CloudAccountAvailability::Native,
                CloudSessionSnapshot {
                    phase: CloudSessionPhase::SignedOut {
                        last_error: Some(
                            "The sign-in page did not answer. Check your connection and try again."
                                .to_owned(),
                        ),
                    },
                    ..CloudSessionSnapshot::default()
                },
                false,
            ),
            (
                "signed-in",
                CloudAccountAvailability::Native,
                active_cloud_snapshot(),
                true,
            ),
            (
                "waiting",
                CloudAccountAvailability::Native,
                CloudSessionSnapshot {
                    phase: CloudSessionPhase::WaitingForBrowser,
                    ..CloudSessionSnapshot::default()
                },
                false,
            ),
        ];
        for (name, availability, snapshot, licensed) in states {
            for size in [
                egui::vec2(1280.0, 800.0),
                egui::vec2(820.0, 1180.0),
                egui::vec2(390.0, 844.0),
            ] {
                let mut app = RSpiceApp::test_instance();
                app.cloud_account =
                    CloudAccountService::with_snapshot(availability, snapshot.clone());
                app.state.license = licensed.then(sample_license);
                app.state
                    .workbench
                    .navigate(account_route(), RouteTransitionSource::User)
                    .expect("account manager route is executable");
                let canvas = crate::ui::raster::render(size, |ui, _| show(ui.ctx(), &mut app));
                let path = directory.join(format!(
                    "account-{name}-{}x{}.png",
                    size.x as usize, size.y as usize
                ));
                std::fs::write(&path, canvas.png(size.y as usize)).expect("write the render");
                writeln!(std::io::stderr(), "{}", path.display()).ok();
            }
        }
    }

    #[test]
    fn initials_are_derived_without_fixture_fallbacks() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("RSpice Labs LLC"), "RL");
        assert_eq!(initials("\u{2014}"), "\u{2014}");
    }
}
