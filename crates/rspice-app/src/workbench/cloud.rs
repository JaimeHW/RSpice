//! The cloud feature: live sessions and web publication, the account console
//! and the license.
//!
//! Its commands are answered and carried out in a [`Cx`] over the feature's
//! own slot, the model, the session and the platform services' state, and
//! the account service and live-session engine the feature runs, which the
//! application holds beside its state and lends with the context. Publish to
//! web still opens its dialog in the shell, on the request the command
//! raises, straight after the command in the same dispatch.

pub(crate) mod commands;

use crate::services::cloud_account::{
    CLOUD_PUBLISHING_FEATURE, CloudAccountAvailability, LIVE_COLLABORATION_FEATURE,
};
use crate::workbench::app::dialogs::license::open_license_dialog;
use crate::workbench::app::dialogs::live_session::open_live_session_dialog;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureRegistry, Reads, Session,
};
use crate::workbench::framework::requests::OpenPublishWebDialog;
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
            Self::PublishToWeb => cx.wb.outbox().push(OpenPublishWebDialog),
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
