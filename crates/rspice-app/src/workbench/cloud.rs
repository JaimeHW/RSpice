//! The cloud feature: live sessions and web publication, the account console
//! and the license.
//!
//! Its commands are answered and carried out in a [`Cx`] over the feature's
//! own slot, the model, the session and the platform services' state, and
//! the account service and live-session engine the feature runs, which the
//! application holds beside its state and lends with the context. Publish to
//! web reviews what it would publish through the hardcopy feature's port.

pub(crate) mod commands;

use crate::services::cloud_account::{
    CLOUD_PUBLISHING_FEATURE, CloudAccountAvailability, LIVE_COLLABORATION_FEATURE,
};
use crate::workbench::account_organization::{self, AccountConsoleParts};
use crate::workbench::app::dialogs::license::{
    LicenseParts, open_license_dialog, render_license_dialog,
};
use crate::workbench::app::dialogs::live_session::{
    LiveSessionParts, open_live_session_dialog, render_live_session_dialog,
};
use crate::workbench::app::dialogs::publish_web::{
    PublishWebParts, SnapshotSource, open_publish_web_dialog, render_publish_web_dialog,
};
use crate::workbench::app::dialogs::unpublish_web::{
    UnpublishWebParts, render_unpublish_web_dialog,
};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureRegistry, Reads,
};
use crate::workbench::live_session::CloudHandles;
use crate::workbench::slots::CloudState;
use crate::workbench::state::WorkbenchSession;
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

use commands::CloudCommand;

/// The cloud feature.
pub(crate) struct CloudFeature;

impl Feature for CloudFeature {
    type State = CloudState;
    type Session = WorkbenchSession;
    type Handles = CloudHandles;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<CloudCommand>();
        registry.dialogs(DIALOGS);
    }
}

/// The dialog steps the feature draws, under the keys the shell's dialog
/// order lists them by: the license, publish, live-session and unpublish
/// dialogs, and the account console.
const DIALOGS: &[FeatureDialog<CloudFeature>] = &[
    FeatureDialog {
        key: DialogKey("cloud.license"),
        is_open: |cloud| cloud.license_dialog.open,
        render: |cx, ctx| render_license_dialog(&mut license_parts(cx), ctx),
    },
    FeatureDialog {
        key: DialogKey("cloud.publish_web"),
        is_open: |cloud| cloud.publish_web.open,
        render: |cx, ctx| render_publish_web_dialog(&mut publish_web_parts(cx), ctx),
    },
    FeatureDialog {
        key: DialogKey("cloud.live_session"),
        is_open: |cloud| cloud.live_session.open,
        render: |cx, ctx| render_live_session_dialog(&mut live_session_parts(cx), ctx),
    },
    FeatureDialog {
        key: DialogKey("cloud.unpublish_web"),
        is_open: |cloud| cloud.unpublish_web.open,
        render: |cx, ctx| render_unpublish_web_dialog(&mut unpublish_web_parts(cx), ctx),
    },
    // The console is open while its route is current. The route is the
    // shell's, and the dialog order's step asks it there; nothing in the
    // cloud's own state opens the console.
    FeatureDialog {
        key: DialogKey("cloud.account_organization"),
        is_open: |_| false,
        render: render_account_console,
    },
];

/// What the license dialog reads and writes, borrowed from the feature's
/// context.
fn license_parts<'a>(cx: &'a mut Cx<'_, CloudFeature>) -> LicenseParts<'a> {
    let cloud = &mut *cx.state;
    LicenseParts {
        dialog: &mut cloud.license_dialog,
        license_key: &mut cloud.license_key,
        license: &mut cloud.license,
        model: cx.model,
    }
}

/// What the publish dialog reads and writes, borrowed from the feature's
/// context.
fn publish_web_parts<'a>(cx: &'a mut Cx<'_, CloudFeature>) -> PublishWebParts<'a> {
    let cloud = &mut *cx.state;
    PublishWebParts {
        dialog: &mut cloud.publish_web,
        unpublish: &mut cloud.unpublish_web,
        account: &mut cx.own.account,
        hardcopy: cx.peers.hardcopy(),
        model: cx.model,
        workbench: cx.wb,
        results: cx.peers.results(),
        runs: cx.runs,
    }
}

/// What the live-session dialog reads and writes, borrowed from the
/// feature's context.
fn live_session_parts<'a>(cx: &'a mut Cx<'_, CloudFeature>) -> LiveSessionParts<'a> {
    LiveSessionParts {
        dialog: &mut cx.state.live_session,
        account: &mut cx.own.account,
        engine: &mut cx.own.live_session,
        model: cx.model,
        platform: cx.platform,
        preflight: &mut cx.runs.preflight,
        outbox: &mut cx.wb.outbox,
    }
}

/// What the unpublish dialog reads and writes, borrowed from the feature's
/// context.
fn unpublish_web_parts<'a>(cx: &'a mut Cx<'_, CloudFeature>) -> UnpublishWebParts<'a> {
    UnpublishWebParts {
        dialog: &mut cx.state.unpublish_web,
        account: &mut cx.own.account,
        workbench: cx.wb,
    }
}

/// Draw the account console while its route is current, then run the
/// command the reader asked of it once this step returns.
fn render_account_console(cx: &mut Cx<'_, CloudFeature>, ctx: &egui::Context) {
    let cloud = &mut *cx.state;
    let command = account_organization::show(
        ctx,
        &mut AccountConsoleParts {
            license_dialog: &mut cloud.license_dialog,
            license: cloud.license.as_ref(),
            account: &mut cx.own.account,
            workbench: cx.wb,
        },
    );
    if let Some(key) = command {
        cx.execute(key);
    }
}

impl CommandSet for CloudCommand {
    type Feature = CloudFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    /// A build with no cloud endpoints can never host a session or publish,
    /// whatever the reader does, so it offers neither rather than showing
    /// two rows that are grey for good.
    fn offered(self, reads: &Reads<'_, CloudFeature>) -> bool {
        match self {
            Self::LiveSession | Self::PublishToWeb => {
                reads.own.account.availability() != CloudAccountAvailability::UnconfiguredBuild
            }
            Self::AccountOrganization | Self::License => true,
        }
    }

    /// Publishing needs the desktop application and a plan that includes it;
    /// a live session needs a seat licensed for live collaboration, from the
    /// desktop or the browser. The account console and the license open from
    /// anywhere.
    fn availability(self, reads: &Reads<'_, CloudFeature>) -> CommandAvailability {
        let account = &reads.own.account;
        match self {
            Self::PublishToWeb => match account.availability() {
                CloudAccountAvailability::UnconfiguredBuild => {
                    CommandAvailability::Disabled("this build has no cloud endpoints pinned")
                }
                CloudAccountAvailability::Browser => CommandAvailability::Disabled(
                    "web publishing is currently available in the desktop application",
                ),
                CloudAccountAvailability::Native
                    if account
                        .snapshot()
                        .cloud_feature_enabled(CLOUD_PUBLISHING_FEATURE) =>
                {
                    CommandAvailability::Available
                }
                CloudAccountAvailability::Native if account.snapshot().signed_in() => {
                    CommandAvailability::Disabled(
                        "the signed-in plan does not include cloud publishing",
                    )
                }
                CloudAccountAvailability::Native => CommandAvailability::Disabled(
                    "sign in to RSpice Cloud from the Account console",
                ),
            },
            Self::LiveSession => match account.availability() {
                CloudAccountAvailability::UnconfiguredBuild => {
                    CommandAvailability::Disabled("this build has no cloud endpoints pinned")
                }
                CloudAccountAvailability::Native | CloudAccountAvailability::Browser
                    if account
                        .snapshot()
                        .cloud_feature_enabled(LIVE_COLLABORATION_FEATURE) =>
                {
                    CommandAvailability::Available
                }
                CloudAccountAvailability::Native | CloudAccountAvailability::Browser
                    if account.snapshot().signed_in() =>
                {
                    CommandAvailability::Disabled(
                        "the signed-in seat is not licensed for live collaboration",
                    )
                }
                CloudAccountAvailability::Native | CloudAccountAvailability::Browser => {
                    CommandAvailability::Disabled(
                        "sign in to RSpice Cloud from the Account console",
                    )
                }
            },
            Self::AccountOrganization | Self::License => CommandAvailability::Available,
        }
    }

    fn execute(self, cx: &mut Cx<'_, CloudFeature>) {
        match self {
            Self::LiveSession => {
                open_live_session_dialog(&mut cx.state.live_session, &mut cx.own.account);
            }
            Self::PublishToWeb => {
                let source = SnapshotSource {
                    hardcopy: cx.peers.hardcopy(),
                    model: cx.model,
                    workbench: cx.wb,
                    results: cx.peers.results(),
                    runs: cx.runs,
                };
                open_publish_web_dialog(&mut cx.state.publish_web, &mut cx.own.account, source);
            }
            Self::AccountOrganization => {
                let route = SurfaceRoute::surface(SurfaceId::AccountOrganization);
                if let Err(error) = cx.wb.navigate(route, RouteTransitionSource::User) {
                    cx.wb.prompt(error.to_string());
                }
            }
            Self::License => {
                let cloud = &mut *cx.state;
                open_license_dialog(&mut cloud.license_dialog, cloud.license.as_ref());
            }
        }
    }
}
