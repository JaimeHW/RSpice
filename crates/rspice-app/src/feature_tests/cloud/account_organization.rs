//! The account console over a whole application: the route it opens on
//! and closes back from, the actions it offers, and what a signed-in
//! console paints.

use crate::services::cloud_account::{
    CloudAccountAvailability, CloudSessionPhase, CloudSessionSnapshot, EntitlementSummary,
    LeaseSummary, NativeLicenseSummary, PrincipalSummary,
};
use crate::services::license::LicenseInfo;
use crate::workbench::account_organization::*;
use crate::workbench::{
    AppState, BrowserHistoryEffect, RSpiceApp, RouteTransitionSource, SurfaceId, SurfaceRoute,
};

/// The console's parts in `app`.
fn console_parts(app: &mut RSpiceApp) -> AccountConsoleParts<'_> {
    let cloud = &mut *app.state.cloud;
    AccountConsoleParts {
        license_dialog: &mut cloud.license_dialog,
        license: cloud.license.as_ref(),
        account: &mut app.cloud.account,
        workbench: &mut app.state.workbench,
    }
}

/// Draw the console over `app`'s parts and run the command the reader
/// asked of it, as the cloud feature's dialog step does.
fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    if let Some(key) = crate::workbench::account_organization::show(ctx, &mut console_parts(app)) {
        app.execute_command_key(key);
    }
}

/// Carry out `action` over `app`'s parts and run the command it asks
/// for, as the cloud feature's dialog step does.
fn execute_action(app: &mut RSpiceApp, action: AccountAction) {
    if let Some(key) =
        crate::workbench::account_organization::execute_action(&mut console_parts(app), action)
    {
        app.execute_command_key(key);
    }
}

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
    close_to_source(&mut state.workbench);
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
    close_to_source(&mut state.workbench);
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
        assert!(license_app.state.cloud.license_dialog.open);
    }

    let mut help_app = RSpiceApp::test_instance();
    execute_action(&mut help_app, AccountAction::SupportBundle);
    assert!(help_app.state.shell.help_center.open);

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
    assert!(!cloud_app.cloud.account.snapshot().signed_in());
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
    app.cloud.account = crate::services::cloud_account::CloudAccountService::with_snapshot(
        CloudAccountAvailability::Native,
        snapshot,
    );
    app.state.cloud.license = Some(sample_license());
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
    use crate::workbench::cloud::commands::CloudCommand;
    use crate::workbench::commands::vocabulary::command_registry;

    let key = CloudCommand::AccountOrganization.key();
    assert!(command_registry().order().contains(&key));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.execute_command_key(key);
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
