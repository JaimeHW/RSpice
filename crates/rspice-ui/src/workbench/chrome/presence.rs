//! Live-session presence chrome.
//!
//! Presence is transient — who is connected right now and which document
//! they are looking at — so it lives in chrome rather than in any document,
//! and it disappears the moment the session ends. The join code, the policy,
//! and participant capabilities belong to the session dialog; this module
//! owns only the title-actions cluster and the popover hanging off it.
//!
//! The cluster reads the application and returns what the user asked for.
//! Executing those requests needs the whole application mutably, and the
//! title bar already holds it, so the caller runs them.

use egui::{Align, Align2, Layout, Sense, Ui, Vec2};

use super::title_bar::MENU_POPUP_GAP;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app::RSpiceApp;

/// One roster row resolved for the presence chrome.
struct PresencePeer {
    principal_id: String,
    name: String,
    role: &'static str,
    is_self: bool,
    connected: bool,
}

/// The presence cluster's facts, resolved from the session snapshot. None
/// hides the cluster entirely — presence exists only while a session runs.
struct PresenceFacts {
    hosting: bool,
    relay_connected: bool,
    started_at: String,
    peers: Vec<PresencePeer>,
    connected_count: usize,
}

fn presence_facts(app: &RSpiceApp) -> Option<PresenceFacts> {
    use crate::services::cloud_account::LiveSessionState;
    let (hosting, summary) = match app.cloud_account.snapshot().live_session.as_ref()? {
        LiveSessionState::Hosting(summary) => (true, summary),
        LiveSessionState::Participating(summary) => (false, summary),
        _ => return None,
    };
    let connected_principals: std::collections::HashSet<String> = app
        .live_session
        .peers()
        .map(|peer| peer.identity.principal_id.to_string())
        .collect();
    let peers: Vec<_> = summary
        .participants
        .iter()
        .map(|participant| PresencePeer {
            principal_id: participant.principal_id.clone(),
            name: participant.display_name.clone(),
            role: if participant.is_host {
                "host"
            } else if participant.pending {
                "pending"
            } else if participant.editor {
                "editor"
            } else {
                "viewer"
            },
            is_self: participant.is_self,
            connected: if participant.is_self {
                summary.relay_connected
            } else {
                connected_principals.contains(&participant.principal_id)
            },
        })
        .collect();
    let connected_count = peers.iter().filter(|peer| peer.connected).count();
    Some(PresenceFacts {
        hosting,
        relay_connected: summary.relay_connected,
        started_at: summary.started_at.clone(),
        peers,
        connected_count,
    })
}

fn peer_initials(name: &str) -> String {
    let words = name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<Vec<_>>();
    match words.as_slice() {
        [] => "?".to_owned(),
        [first] => first.to_uppercase().collect(),
        [first, .., last] => format!("{}{}", first.to_uppercase(), last.to_uppercase()),
    }
}

/// What the user asked the popover for. The cluster only reads the
/// application, so these travel back to the title bar — which already holds
/// it mutably — instead of another function taking the whole application.
pub(super) enum PresenceAction {
    OpenDialog,
    EndSession,
    LeaveSession,
    RequestLease(String),
    ReleaseLease(String),
}

/// Title-actions presence cluster: a LIVE mark plus up to three participant
/// chips, present exactly while a session runs. Clicking opens the
/// participants popover; policy and the join code stay owned by the dialog.
pub(super) fn session_presence_cluster(
    ui: &mut Ui,
    app: &RSpiceApp,
    large_target: bool,
) -> Vec<PresenceAction> {
    let mut actions: Vec<PresenceAction> = Vec::new();
    let Some(facts) = presence_facts(app) else {
        return actions;
    };
    let t = Tokens::get(ui.ctx());
    let mark_text = if facts.relay_connected {
        "LIVE"
    } else {
        "SYNC"
    };
    let mark_color = if facts.relay_connected {
        t.color.ok
    } else {
        t.color.warn
    };
    let mark_galley = ui.painter().layout_no_wrap(
        mark_text.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        mark_color,
    );
    let admitted: Vec<&PresencePeer> = facts
        .peers
        .iter()
        .filter(|peer| peer.connected && peer.role != "pending")
        .collect();
    let chip_count = admitted.len().min(3);
    let overflow = admitted.len().saturating_sub(chip_count);
    let chip = if large_target { 24.0 } else { 20.0 };
    let width = mark_galley.size().x
        + 8.0
        + chip_count as f32 * (chip + 2.0)
        + if overflow > 0 { chip + 2.0 } else { 0.0 }
        + 8.0;
    let height = if large_target { 44.0 } else { 27.0 };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    let accessible_label = format!(
        "Live session — {} in session, {}. Opens participants and session controls.",
        facts.connected_count,
        if facts.relay_connected {
            "streaming"
        } else {
            "reconnecting"
        }
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let mut x = rect.left() + 4.0;
    ui.painter().galley(
        egui::pos2(x, rect.center().y - mark_galley.size().y * 0.5),
        mark_galley.clone(),
        mark_color,
    );
    x += mark_galley.size().x + 6.0;
    for peer in admitted.iter().take(chip_count) {
        let center = egui::pos2(x + chip * 0.5, rect.center().y);
        ui.painter()
            .circle_filled(center, chip * 0.5, t.color.bg_panel_2);
        ui.painter().circle_stroke(
            center,
            chip * 0.5,
            egui::Stroke::new(1.0, t.color.border_strong),
        );
        ui.painter().text(
            center,
            Align2::CENTER_CENTER,
            peer_initials(&peer.name),
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_dim,
        );
        x += chip + 2.0;
    }
    if overflow > 0 {
        let center = egui::pos2(x + chip * 0.5, rect.center().y);
        ui.painter()
            .circle_filled(center, chip * 0.5, t.color.bg_panel_2);
        ui.painter().text(
            center,
            Align2::CENTER_CENTER,
            format!("+{overflow}"),
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_dim,
        );
    }
    theme::paint_focus_ring_outset(ui, &response, rect);
    let response = response.on_hover_text(accessible_label);

    egui::Popup::menu(&response)
        .gap(MENU_POPUP_GAP)
        .show(|ui| render_presence_popover(ui, app, &facts, &mut actions));
    actions
}

/// Popover body: who is here now and where they work, the transient facts,
/// and the session actions. Owns nothing the dialog owns.
fn render_presence_popover(
    ui: &mut Ui,
    app: &RSpiceApp,
    facts: &PresenceFacts,
    actions: &mut Vec<PresenceAction>,
) {
    let t = Tokens::get(ui.ctx());
    ui.set_min_width(280.0);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Live session")
                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} connected · {} in roster",
                    facts.connected_count,
                    facts.peers.len()
                ))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        });
    });
    ui.add_space(4.0);
    // Where each participant works right now, from their presence frames.
    let focused: std::collections::HashMap<String, String> = app
        .live_session
        .peers()
        .filter_map(|peer| {
            let location = match peer.cursor.as_ref() {
                Some(crate::services::live_protocol::CursorLocus::Netlist { doc, line }) => {
                    format!("{} · line {line}", presented_doc(doc))
                }
                Some(crate::services::live_protocol::CursorLocus::Canvas { doc, .. }) => {
                    presented_doc(doc)
                }
                None => presented_doc(peer.focused_doc.as_ref()?),
            };
            Some((peer.identity.principal_id.to_string(), location))
        })
        .collect();
    for peer in &facts.peers {
        ui.horizontal(|ui| {
            let name = if peer.is_self {
                format!("{} · you", peer.name)
            } else {
                peer.name.clone()
            };
            ui.label(
                egui::RichText::new(name)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let detail = match focused.get(&peer.principal_id) {
                    Some(doc) if !peer.is_self && peer.connected => {
                        format!("{} · {doc}", peer.role)
                    }
                    _ if !peer.connected => format!("{} · offline", peer.role),
                    _ => peer.role.to_owned(),
                };
                ui.label(
                    egui::RichText::new(detail)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            });
        });
    }
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(format!(
            "relay · streamed, not stored — live since {}",
            facts.started_at
        ))
        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
        .color(t.color.text_faint),
    );
    ui.add_space(6.0);

    // Write-lease affordance for the document in front of a mirroring guest.
    if !facts.hosting && app.state.workbench.live_write_locks.mirror {
        let (doc, held_by_other) =
            if app.state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
                (
                    "netlist".to_owned(),
                    app.state.workbench.live_write_locks.netlist.clone(),
                )
            } else {
                let key = app.state.workspace.active_key();
                (
                    format!("schematic/{key}"),
                    app.state
                        .workbench
                        .live_write_locks
                        .schematic_views
                        .get(&key)
                        .cloned(),
                )
            };
        match held_by_other {
            Some(holder) => {
                if ui
                    .button(format!("Request the write lease · {holder} holds it"))
                    .clicked()
                {
                    actions.push(PresenceAction::RequestLease(doc));
                    ui.close();
                }
            }
            None => {
                if ui.button("Release your write lease").clicked() {
                    actions.push(PresenceAction::ReleaseLease(doc));
                    ui.close();
                }
            }
        }
    }
    if ui.button("Invite and session policy…").clicked() {
        actions.push(PresenceAction::OpenDialog);
        ui.close();
    }
    if facts.hosting {
        if ui.button("End session").clicked() {
            actions.push(PresenceAction::EndSession);
            ui.close();
        }
    } else if ui.button("Leave session").clicked() {
        actions.push(PresenceAction::LeaveSession);
        ui.close();
    }
}

/// A wire document key as the roster shows it.
fn presented_doc(doc: &str) -> String {
    doc.strip_prefix("schematic/").unwrap_or(doc).to_owned()
}
