//! The shell's transactions over the whole project, carried out where a
//! feature's own work asks for one.
//!
//! A feature holds only its part of the application, so it cannot close or
//! replace the open project itself. It hands the shell what it needs between
//! the steps of its work, and the shell runs the transaction there, in the
//! same frame and the same order as if the feature had run it, and tells the
//! feature how it went.

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::netlist_document::adopt_live_owned_source;
use crate::workbench::framework::requests::{self, AdoptLiveOwnedSource};
use crate::workbench::lifecycle::project_lifecycle::{
    ProjectCloseDestination, has_unsaved_changes,
};
use crate::workbench::live_session::{CloudHandles, DiscardMirror, LiveHost, MirrorApply};
use crate::workbench::workflows::project_workflow::{
    LiveProjectApply, apply_live_project_snapshot, close_project_discard,
};

impl RSpiceApp {
    /// Run the live session for this frame, with the project transactions
    /// it asks for.
    pub(crate) fn pump_live_session(&mut self) {
        pump_live_session(&mut self.state, &mut self.cloud);
    }

    /// Close the open project to make room for the live session host's as
    /// this install's mirror: through the close review when it holds unsaved
    /// work, at once otherwise. The close lets the mirror in once it lands.
    pub(crate) fn close_for_live_mirror(&mut self) {
        let state = &mut self.state;
        let destination = ProjectCloseDestination::LiveMirror;
        if has_unsaved_changes(state) {
            state
                .workbench
                .project_review_dialog
                .show_close_project(destination);
        } else {
            close_project_discard(state, destination);
        }
    }
}

/// Drive the live session for one frame: settle who this install is in the
/// session, then its inbound half, then the host's snapshot replacing a
/// guest's mirror, then its outbound half. Between the phases the shell
/// closes a mirror whose session ended without a copy kept, at once or once
/// the local run that held it has stopped, has the netlist editor adopt the
/// project source a participant's update replaced, before a snapshot can
/// replace that project, and applies the host's project snapshot.
pub(crate) fn pump_live_session(state: &mut AppState, cloud: &mut CloudHandles) {
    let CloudHandles {
        account,
        live_session,
    } = cloud;
    if let Some(DiscardMirror) = live_session.reconcile_role(&mut live_host(state), account) {
        let closed = close_project_discard(state, ProjectCloseDestination::EmptyWorkbench);
        live_session.mirror_retired(closed);
    }
    if let Some(DiscardMirror) = live_session.mirror_discard_due(&live_host(state)) {
        let closed = close_project_discard(state, ProjectCloseDestination::EmptyWorkbench);
        live_session.mirror_discard_settled(closed);
    }
    if !live_session.pump_inbound(&mut live_host(state), account) {
        return;
    }
    requests::drain(
        state,
        |state| &mut state.workbench.outbox,
        |state, AdoptLiveOwnedSource(document)| {
            adopt_live_owned_source(&mut state.code.netlist, document);
        },
    );
    if let Some(snapshot) = live_session.take_mirror_snapshot() {
        let outcome =
            match apply_live_project_snapshot(state, snapshot.bytes(), snapshot.host_label()) {
                LiveProjectApply::Applied => MirrorApply::Applied,
                LiveProjectApply::RetryLater => MirrorApply::RetryLater,
                LiveProjectApply::Rejected => MirrorApply::Rejected,
            };
        live_session.mirror_snapshot_applied(&mut live_host(state), snapshot, outcome);
    }
    live_session.pump_outbound(&mut live_host(state));
}

/// What the live session's engine reads and writes of `state` in one phase
/// of the pump. A project snapshot reads the same four parts from it as from
/// `state` itself.
fn live_host(state: &mut AppState) -> LiveHost<'_> {
    LiveHost {
        model: &mut state.model,
        workbench: &mut state.workbench,
        platform: &state.platform,
        presentation: &state.results.results,
        design: &*state.design,
        code: &*state.code,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::cloud_account::CloudAccountService;
    use crate::workbench::framework::requests::LiveMirrorEntry;
    use crate::workbench::live_session::LiveSessionEngine;

    /// A close-to-mirror transaction that outlives its session leaves the
    /// reader where every project close does: on the no-project landing.
    #[test]
    fn a_mirror_entry_that_outlives_its_session_stays_on_the_no_project_landing() {
        let mut state = AppState::default();
        assert!(close_project_discard(
            &mut state,
            ProjectCloseDestination::LiveMirror,
        ));

        pump_live_session(
            &mut state,
            &mut CloudHandles {
                account: CloudAccountService::unconfigured(),
                live_session: LiveSessionEngine::default(),
            },
        );

        assert!(!state.project.project_launcher_open);
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Project
        );
        assert!(
            state.workbench.outbox.peek::<LiveMirrorEntry>().is_none(),
            "the pump consumes the stale entry"
        );
    }
}
