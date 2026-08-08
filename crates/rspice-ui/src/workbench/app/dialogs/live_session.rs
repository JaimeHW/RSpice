//! Live session — the explicit switch onto the hosted relay.
//!
//! One dialog owns the whole lifecycle: the start review (what going live
//! does, who can join, the host-controlled policy), the host's live
//! controls (join code, roster, capability and policy changes), and the
//! guest's session view (mirror adoption, run status, leaving). Every
//! rendered fact comes from the cloud service's owned snapshot; the dialog
//! only issues typed commands and never invents session state.

use egui::Context;

use crate::services::cloud_account::{
    CloudAccountAvailability, LIVE_COLLABORATION_FEATURE, LiveParticipantSummary,
    LiveSessionPolicySummary, LiveSessionState, LiveSessionSummary,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::app::RSpiceApp;

/// Live-session dialog state. The session itself lives in the cloud
/// service's snapshot; this holds only the user's in-dialog inputs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LiveSessionDialogState {
    pub open: bool,
    /// Join-by-code input on the start surface.
    pub join_code_input: String,
    /// The host's policy as edited here; applied as one command. Seeded
    /// from the live summary, or from the conservative defaults.
    pub policy_draft: Option<LiveSessionPolicySummary>,
}

/// Fail-closed starting policy: read-only guests, every join reviewed,
/// no forked copies. The host relaxes each knob deliberately.
fn default_policy() -> LiveSessionPolicySummary {
    LiveSessionPolicySummary {
        guests_edit: false,
        approve_joins: true,
        allow_save_copy: false,
    }
}

/// One deferred user action collected while the body renders; executed
/// after the borrow of the dialog state ends.
enum LiveDialogAction {
    Join(String),
    CopyCode(String),
    RotateCode,
    Approve(String),
    SetEditor(String, bool),
    Remove(String),
    RequestHostRun,
}

impl RSpiceApp {
    /// Open the dialog against the current session state. No-op unless
    /// this build reaches the cloud and the seat carries the entitlement.
    pub(crate) fn open_live_session_dialog(&mut self) {
        if !matches!(
            self.cloud_account.availability(),
            CloudAccountAvailability::Native
        ) || !self
            .cloud_account
            .snapshot()
            .cloud_feature_enabled(LIVE_COLLABORATION_FEATURE)
        {
            return;
        }
        let policy_draft = match self.cloud_account.snapshot().live_session.as_ref() {
            Some(LiveSessionState::Hosting(summary)) => Some(summary.policy),
            _ => None,
        };
        self.state.dialogs.live_session = LiveSessionDialogState {
            open: true,
            policy_draft,
            ..Default::default()
        };
        // Freshen the roster and clear any stale notice before rendering.
        if self.cloud_account.snapshot().live_session.is_some() {
            self.cloud_account.refresh_live_session();
        }
    }

    pub(in crate::workbench) fn render_live_session_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.live_session.open {
            return;
        }

        let session = self.cloud_account.snapshot().live_session.clone();
        let mirroring = self.state.workbench.live_write_locks.mirror;
        let circuit_id = self
            .state
            .workspace
            .project
            .cloud_publication()
            .map(|binding| binding.circuit_id().to_owned());
        let project_open = self.state.project_lifecycle.project_open;
        #[cfg(not(target_arch = "wasm32"))]
        let incompatible_peer = self.live_session.incompatible_peer_seen();
        #[cfg(target_arch = "wasm32")]
        let incompatible_peer = false;
        #[cfg(not(target_arch = "wasm32"))]
        let host_run: Option<String> = self.live_session.guest_run_status().map(describe_run);
        #[cfg(target_arch = "wasm32")]
        let host_run: Option<String> = None;

        // Resolve the dialog chrome for the current lifecycle state.
        let (eyebrow, title, primary, primary_enabled, secondary): (
            &str,
            String,
            String,
            bool,
            Option<&str>,
        ) = match &session {
            None | Some(LiveSessionState::Failed { .. }) => (
                "LIVE SESSION · HOSTED RELAY · EXPLICIT SWITCH",
                "Start a live session".to_owned(),
                "Go live".to_owned(),
                project_open,
                None,
            ),
            Some(LiveSessionState::Starting) => (
                "LIVE SESSION · HOSTED RELAY · EXPLICIT SWITCH",
                "Start a live session".to_owned(),
                "Creating…".to_owned(),
                false,
                None,
            ),
            Some(LiveSessionState::Joining) => (
                "LIVE SESSION · HOSTED RELAY",
                "Join a live session".to_owned(),
                "Joining…".to_owned(),
                false,
                None,
            ),
            Some(LiveSessionState::AwaitingApproval(_)) => (
                "LIVE SESSION · HOSTED RELAY",
                "Join a live session".to_owned(),
                "Waiting for the host…".to_owned(),
                false,
                Some("Leave"),
            ),
            Some(LiveSessionState::Hosting(summary)) => {
                let draft = self
                    .state
                    .dialogs
                    .live_session
                    .policy_draft
                    .unwrap_or(summary.policy);
                (
                    "LIVE SESSION · HOST CONTROLS · STREAMED, NOT STORED",
                    match &summary.join_code {
                        Some(code) => format!("Live session {code}"),
                        None => "Live session".to_owned(),
                    },
                    "Apply session policy".to_owned(),
                    draft != summary.policy,
                    Some("End session"),
                )
            }
            Some(LiveSessionState::Participating(_)) => (
                "LIVE SESSION · MIRRORED FROM THE HOST",
                "Live session".to_owned(),
                if mirroring {
                    "Close".to_owned()
                } else {
                    "Open the host's project".to_owned()
                },
                true,
                Some("Leave session"),
            ),
        };

        let description = match &session {
            None | Some(LiveSessionState::Failed { .. }) => {
                "Share this machine's working copy over the hosted relay, or join \
                 another host's session by code."
            }
            Some(LiveSessionState::Hosting(_)) => {
                "Host controls: the join code, each participant's capability, and \
                 the session policy."
            }
            Some(LiveSessionState::Participating(_)) => {
                "Your seat in the host's session: the mirror, your capability, and \
                 the roster."
            }
            _ => "The session is being established.",
        };
        let mut dialog = Dialog::new(eyebrow, &title, &primary)
            .description(description)
            .size(DialogSize::Transaction)
            .ghost("Close")
            .primary_enabled(primary_enabled)
            .primary_on_enter(false);
        if let Some(secondary) = secondary {
            dialog = dialog.secondary(secondary);
        }

        let mut actions: Vec<LiveDialogAction> = Vec::new();
        let dialog_state = &mut self.state.dialogs.live_session;
        let choice = dialog.show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            let c = t.color;
            if incompatible_peer {
                note(
                    ui,
                    "A participant runs an incompatible RSpice build; their changes cannot \
                     be shared.",
                    c.warn,
                );
                ui.add_space(tokens::SP_4);
            }
            match &session {
                None => {
                    render_start(ui, dialog_state, project_open, &mut actions);
                }
                Some(LiveSessionState::Failed { message }) => {
                    note(ui, message, c.err);
                    ui.add_space(tokens::SP_4);
                    render_start(ui, dialog_state, project_open, &mut actions);
                }
                Some(LiveSessionState::Starting) => {
                    busy_row(ui, "Creating the session and attaching the relay…");
                }
                Some(LiveSessionState::Joining) => {
                    busy_row(ui, "Joining the session…");
                }
                Some(LiveSessionState::AwaitingApproval(summary)) => {
                    busy_row(ui, "Waiting for the host to admit you.");
                    ui.add_space(tokens::SP_4);
                    note(
                        ui,
                        "The host reviews each join under this session's policy. Leaving \
                         withdraws nothing on the host's side; the request simply expires.",
                        c.text_dim,
                    );
                    if let Some(notice) = &summary.notice {
                        ui.add_space(tokens::SP_4);
                        note(ui, notice, c.warn);
                    }
                }
                Some(LiveSessionState::Hosting(summary)) => {
                    render_hosting(ui, dialog_state, summary, &mut actions);
                }
                Some(LiveSessionState::Participating(summary)) => {
                    render_participating(ui, summary, mirroring, host_run.clone(), &mut actions);
                }
            }
        });

        for action in actions {
            match action {
                LiveDialogAction::Join(code) => self.cloud_account.join_live_session(code),
                LiveDialogAction::CopyCode(code) => ctx.copy_text(code),
                LiveDialogAction::RotateCode => self.cloud_account.regenerate_live_session_code(),
                LiveDialogAction::Approve(principal_id) => self
                    .cloud_account
                    .approve_live_session_participant(principal_id),
                LiveDialogAction::SetEditor(principal_id, editor) => self
                    .cloud_account
                    .set_live_session_participant_editor(principal_id, editor),
                LiveDialogAction::Remove(principal_id) => self
                    .cloud_account
                    .remove_live_session_participant(principal_id),
                LiveDialogAction::RequestHostRun =>
                {
                    #[cfg(not(target_arch = "wasm32"))]
                    self.live_session.request_run()
                }
            }
        }

        match choice {
            DialogChoice::None => {}
            DialogChoice::Cancelled | DialogChoice::Ghost => {
                // The session (hosted or joined) keeps running; the dialog
                // is a control surface, not the session's lifetime.
                self.state.dialogs.live_session.open = false;
            }
            DialogChoice::Secondary => match &session {
                Some(LiveSessionState::Hosting(_)) => self.cloud_account.end_live_session(),
                Some(LiveSessionState::AwaitingApproval(_))
                | Some(LiveSessionState::Participating(_)) => {
                    self.cloud_account.leave_live_session();
                }
                _ => {}
            },
            DialogChoice::Primary => match &session {
                None | Some(LiveSessionState::Failed { .. }) => {
                    let policy = self
                        .state
                        .dialogs
                        .live_session
                        .policy_draft
                        .unwrap_or_else(default_policy);
                    self.cloud_account.start_live_session(policy, circuit_id);
                }
                Some(LiveSessionState::Hosting(_)) => {
                    if let Some(policy) = self.state.dialogs.live_session.policy_draft {
                        self.cloud_account.apply_live_session_policy(policy);
                    }
                }
                Some(LiveSessionState::Participating(_)) => {
                    if mirroring {
                        self.state.dialogs.live_session.open = false;
                    } else {
                        #[cfg(not(target_arch = "wasm32"))]
                        self.live_session.request_mirror_entry(&mut self.state);
                    }
                }
                _ => {}
            },
        }
    }
}

/// The start surface: what going live does, who can join, the policy, and
/// the join-by-code lane.
fn render_start(
    ui: &mut egui::Ui,
    dialog: &mut LiveSessionDialogState,
    project_open: bool,
    actions: &mut Vec<LiveDialogAction>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    ui.columns(2, |columns| {
        {
            let ui = &mut columns[0];
            section_head(ui, "What going live does");
            for line in [
                "Everyone works on this machine's working copy: guests mirror it and \
                 edits stream through the hosted relay — relayed, never stored.",
                "You arbitrate per-document write leases; guests edit only what you \
                 grant and hand back.",
                "Runs on this machine stream their progress to every participant; \
                 every seat can also run the mirrored design locally.",
                "Ending the session disconnects everyone instantly and the join code \
                 dies; whether participants keep a copy follows the session policy.",
            ] {
                bullet(ui, line);
            }
            if !project_open {
                ui.add_space(tokens::SP_4);
                note(ui, "Open a project to host a session.", c.warn);
            }
        }
        {
            let ui = &mut columns[1];
            section_head(ui, "Session policy · host controlled");
            let draft = dialog.policy_draft.get_or_insert_with(default_policy);
            render_policy_fields(ui, draft, "live-start");
            ui.add_space(tokens::SP_4);
            note(
                ui,
                "Policy applies to guests joining by code. While the session runs you \
                 can make any participant a viewer, or remove them, from the roster.",
                c.text_faint,
            );
        }
    });

    ui.add_space(tokens::SP_4);
    section_head(ui, "Join a session instead");
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut dialog.join_code_input)
                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                .desired_width(180.0)
                .hint_text("join code from the host"),
        );
        let code = dialog.join_code_input.trim().to_owned();
        if ui
            .add_enabled(!code.is_empty(), egui::Button::new("Join"))
            .clicked()
        {
            actions.push(LiveDialogAction::Join(code));
        }
    });
    note(
        ui,
        "Joining mirrors the host's project on this install; your open project \
         closes first through the usual save review.",
        c.text_faint,
    );

    ui.add_space(tokens::SP_4);
    license_banner(ui);
}

/// The host's live controls: code, roster, and policy.
fn render_hosting(
    ui: &mut egui::Ui,
    dialog: &mut LiveSessionDialogState,
    summary: &LiveSessionSummary,
    actions: &mut Vec<LiveDialogAction>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    if let Some(notice) = &summary.notice {
        note(ui, notice, c.warn);
        ui.add_space(tokens::SP_4);
    }

    ui.columns(2, |columns| {
        {
            let ui = &mut columns[0];
            section_head(ui, "Join code");
            ui.horizontal(|ui| {
                match &summary.join_code {
                    Some(code) => {
                        ui.label(
                            egui::RichText::new(code)
                                .font(theme::mono(tokens::FS_2, FontWeight::Medium))
                                .color(c.text),
                        );
                        if ui.button("Copy").clicked() {
                            actions.push(LiveDialogAction::CopyCode(code.clone()));
                        }
                    }
                    None => {
                        // The server returns the code exactly once; after a
                        // restart only a rotation can mint a presentable one.
                        note(ui, "held by participants only", c.text_faint);
                    }
                }
                if ui.button("New code").clicked() {
                    actions.push(LiveDialogAction::RotateCode);
                }
            });
            note(
                ui,
                "Rotating kills the old code for anyone who has not joined; current \
                 participants stay connected.",
                c.text_faint,
            );
            ui.add_space(tokens::SP_4);
            section_head(
                ui,
                &format!("Participants · {}", summary.participants.len()),
            );
            for participant in &summary.participants {
                render_participant_row(ui, participant, actions);
            }
        }
        {
            let ui = &mut columns[1];
            section_head(ui, "Session policy · host controlled");
            let draft = dialog.policy_draft.get_or_insert(summary.policy);
            render_policy_fields(ui, draft, "live-host");
            ui.add_space(tokens::SP_4);
            kv_row(
                ui,
                "Relay",
                if summary.relay_connected {
                    "connected · streamed, not stored"
                } else {
                    "reconnecting…"
                },
                if summary.relay_connected { c.ok } else { c.warn },
            );
            kv_row(ui, "Live since", &summary.started_at, c.text);
            kv_row(ui, "Stored on the service", "nothing · relayed only", c.ok);
        }
    });

    ui.add_space(tokens::SP_4);
    license_banner(ui);
}

/// The guest's session view: mirror adoption, facts, roster, run status.
fn render_participating(
    ui: &mut egui::Ui,
    summary: &LiveSessionSummary,
    mirroring: bool,
    host_run: Option<String>,
    actions: &mut Vec<LiveDialogAction>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    if let Some(notice) = &summary.notice {
        note(ui, notice, c.warn);
        ui.add_space(tokens::SP_4);
    }
    let host = summary
        .participants
        .iter()
        .find(|participant| participant.is_host)
        .map(|participant| participant.display_name.as_str())
        .unwrap_or("the host");
    if mirroring {
        note(
            ui,
            &format!("This workbench mirrors {host}'s working copy."),
            c.text,
        );
    } else {
        note(
            ui,
            &format!(
                "You are in the session, but {host}'s project has not been opened \
                 here yet. Opening it closes your current project first through the \
                 usual save review."
            ),
            c.text,
        );
    }
    ui.add_space(tokens::SP_4);
    kv_row(
        ui,
        "Your capability",
        if summary.editor {
            "editor · request write leases, propose runs"
        } else {
            "viewer · read-only with live updates"
        },
        c.text,
    );
    kv_row(
        ui,
        "Save a copy",
        if summary.policy.allow_save_copy {
            "allowed · the mirror stays yours to save when the session ends"
        } else {
            "not allowed · work stays in the session"
        },
        if summary.policy.allow_save_copy {
            c.text
        } else {
            c.warn
        },
    );
    kv_row(
        ui,
        "Relay",
        if summary.relay_connected {
            "connected · streamed, not stored"
        } else {
            "reconnecting…"
        },
        if summary.relay_connected { c.ok } else { c.warn },
    );
    if let Some(line) = host_run {
        kv_row(ui, "Host run", &line, c.text);
    }
    if mirroring && summary.editor {
        ui.add_space(tokens::SP_4);
        if ui.button("Request a run on the host").clicked() {
            actions.push(LiveDialogAction::RequestHostRun);
        }
    }

    ui.add_space(tokens::SP_4);
    section_head(
        ui,
        &format!("Participants · {}", summary.participants.len()),
    );
    for participant in &summary.participants {
        let role = if participant.is_host {
            "host"
        } else if participant.pending {
            "pending approval"
        } else if participant.editor {
            "editor"
        } else {
            "viewer"
        };
        let name = if participant.is_self {
            format!("{} · you", participant.display_name)
        } else {
            participant.display_name.clone()
        };
        kv_row(ui, &name, role, c.text_dim);
    }

    ui.add_space(tokens::SP_4);
    license_banner(ui);
}

/// One host-side roster row: identity, admission, capability controls.
fn render_participant_row(
    ui: &mut egui::Ui,
    participant: &LiveParticipantSummary,
    actions: &mut Vec<LiveDialogAction>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.horizontal(|ui| {
        let name = if participant.is_self {
            format!("{} · you", participant.display_name)
        } else {
            participant.display_name.clone()
        };
        ui.label(
            egui::RichText::new(name)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(c.text),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if participant.is_host {
                ui.label(
                    egui::RichText::new("host")
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
                return;
            }
            if participant.pending {
                if ui.button("Admit").clicked() {
                    actions.push(LiveDialogAction::Approve(participant.principal_id.clone()));
                }
                if ui.button("Refuse").clicked() {
                    actions.push(LiveDialogAction::Remove(participant.principal_id.clone()));
                }
                ui.label(
                    egui::RichText::new("pending")
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(c.warn),
                );
                return;
            }
            let current = if participant.editor { "Editor" } else { "Viewer" };
            egui::ComboBox::from_id_salt(("live-session-capability", &participant.principal_id))
                .selected_text(current)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(participant.editor, "Editor").clicked() {
                        actions.push(LiveDialogAction::SetEditor(
                            participant.principal_id.clone(),
                            true,
                        ));
                    }
                    if ui.selectable_label(!participant.editor, "Viewer").clicked() {
                        actions.push(LiveDialogAction::SetEditor(
                            participant.principal_id.clone(),
                            false,
                        ));
                    }
                    if ui.selectable_label(false, "Remove from session").clicked() {
                        actions.push(LiveDialogAction::Remove(participant.principal_id.clone()));
                    }
                });
            ui.label(
                egui::RichText::new(&participant.joined_at)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        });
    });
}

/// The three host-controlled policy selects, mockup labels verbatim.
fn render_policy_fields(
    ui: &mut egui::Ui,
    draft: &mut LiveSessionPolicySummary,
    id_salt: &str,
) {
    field_label(ui, "Guests join as");
    egui::ComboBox::from_id_salt((id_salt, "guests"))
        .width(ui.available_width())
        .selected_text(if draft.guests_edit {
            "Editors · edit and trigger runs"
        } else {
            "Viewers · read-only with live updates"
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut draft.guests_edit,
                false,
                "Viewers · read-only with live updates",
            );
            ui.selectable_value(
                &mut draft.guests_edit,
                true,
                "Editors · edit and trigger runs",
            );
        });
    field_label(ui, "Joining");
    egui::ComboBox::from_id_salt((id_salt, "joining"))
        .width(ui.available_width())
        .selected_text(if draft.approve_joins {
            "Approve each join"
        } else {
            "Anyone with the code joins instantly"
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut draft.approve_joins, true, "Approve each join");
            ui.selectable_value(
                &mut draft.approve_joins,
                false,
                "Anyone with the code joins instantly",
            );
        });
    field_label(ui, "Save a copy");
    egui::ComboBox::from_id_salt((id_salt, "save-copy"))
        .width(ui.available_width())
        .selected_text(if draft.allow_save_copy {
            "Allowed · participants may fork"
        } else {
            "Not allowed · work stays in the session"
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut draft.allow_save_copy,
                false,
                "Not allowed · work stays in the session",
            );
            ui.selectable_value(
                &mut draft.allow_save_copy,
                true,
                "Allowed · participants may fork",
            );
        });
}

fn license_banner(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    note(
        ui,
        "Every participant signs in with a named-user seat licensed for live \
         collaboration — host and guests alike. The join code never admits a free \
         account; free users read published pages instead.",
        t.color.text_faint,
    );
}

/// One presentation line for the host's freshest run report.
#[cfg(not(target_arch = "wasm32"))]
fn describe_run(status: &crate::services::live_protocol::RunStatusPayload) -> String {
    let phase = match &status.phase {
        crate::services::live_protocol::RunPhase::Started => "started",
        crate::services::live_protocol::RunPhase::Progress => "running",
        crate::services::live_protocol::RunPhase::Finished => "finished",
        crate::services::live_protocol::RunPhase::Failed { message } => {
            return format!("failed — {message}");
        }
    };
    match status
        .progress
        .as_ref()
        .and_then(|progress| progress.get("fraction"))
        .and_then(serde_json::Value::as_f64)
    {
        Some(fraction) => format!("{phase} · {:.0}%", fraction * 100.0),
        None => phase.to_owned(),
    }
}

fn busy_row(ui: &mut egui::Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.add(egui::Spinner::new().size(14.0));
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text),
        );
    });
}

fn note(ui: &mut egui::Ui, text: &str, color: egui::Color32) {
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn bullet(ui: &mut egui::Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new("·")
                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text_dim),
        );
        ui.label(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text),
        );
    });
}

fn section_head(ui: &mut egui::Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
    ui.add_space(4.0);
}

fn field_label(ui: &mut egui::Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn kv_row(ui: &mut egui::Ui, key: &str, value: &str, value_color: egui::Color32) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(key)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(value_color),
            );
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_starting_policy_fails_closed_on_every_knob() {
        let policy = default_policy();
        assert!(!policy.guests_edit);
        assert!(policy.approve_joins);
        assert!(!policy.allow_save_copy);
    }
}
