//! Typed net-label placement transaction.
//!
//! The Label tool first captures one snapped canvas anchor. This dialog owns
//! the isolated name draft and publishes a durable [`NetLabel`] only after the
//! complete document authority and naming policy have been revalidated.

use egui::{Context, Frame, Response, Stroke, TextEdit, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{NetLabel, NetNamingPolicy, Point};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};

use crate::workbench::app::dialogs::schematic_command::{field_label, read_only_value, snap_label};
use crate::workbench::app::{NetLabelPlacementDialogState, RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC \u{00b7} CONNECTIVITY";
const TITLE: &str = "Place net label";
const PRIMARY: &str = "Place label";
const DESCRIPTION: &str =
    "Assign a validated electrical name at the selected snapped schematic anchor.";
const FIELD_ID: &str = "place-net-label-name";
const DISCARD_TITLE: &str = "Unsaved label name";
const DISCARD_DETAIL: &str =
    "Choose Discard changes again to close. The schematic and undo history are unchanged.";

#[derive(Debug, Clone)]
struct NetLabelPlacementCommit {
    authority: SchematicEditAuthority,
    anchor: Point,
    name: String,
}

#[derive(Debug, Clone)]
enum DraftValidation {
    Incomplete(&'static str),
    Invalid(String),
    /// Boxed: the commit dwarfs the two message-only variants.
    Valid(Box<NetLabelPlacementCommit>),
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Incomplete(message) => Some(message),
            Self::Invalid(message) => Some(message.as_str()),
            Self::Valid(_) => None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }
}

/// Capture a snapped label anchor without changing the schematic.
///
/// Returning `false` means the active document does not currently own edit
/// authority or another application modal already owns the interaction.
pub(crate) fn open_net_label_placement(state: &mut AppState, anchor: Point) -> bool {
    if state.dialogs.application_modal_open() {
        return false;
    }
    if state.schematic_edit_read_only() {
        state.deny_read_only_edit();
        return false;
    }

    let authority = SchematicEditAuthority::capture(state);
    state.dialogs.net_label_placement.open(anchor, authority);
    true
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_net_label_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.net_label_placement.open {
            return;
        }

        let validation = validate_draft(&self.state);
        let message = self
            .state
            .dialogs
            .net_label_placement
            .validation_error
            .as_deref()
            .or_else(|| validation.message())
            .map(str::to_owned);
        let message_is_error = self
            .state
            .dialogs
            .net_label_placement
            .validation_error
            .is_some()
            || validation.is_error();
        let discard_confirm = self.state.dialogs.net_label_placement.discard_confirm;
        let mut edited = false;
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation.can_commit())
            .initial_focus(DialogInitialFocus::BodyControl);
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        }

        let anchor = self.state.dialogs.net_label_placement.anchor;
        let grid_pitch = self.state.schematic.document_policy.grid_pitch;
        let naming_policy = self.state.schematic.document_policy.net_naming;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let (focus, changed) = dialog_body(
                ui,
                anchor,
                grid_pitch,
                naming_policy,
                &mut self.state.dialogs.net_label_placement,
                message.as_deref(),
                message_is_error,
            );
            edited = changed;
            focus
        });
        if edited {
            self.state.dialogs.net_label_placement.mark_edited();
        }

        match choice {
            DialogChoice::Primary => {
                // Revalidate after this frame's text edit so Enter can never
                // publish a candidate that became invalid in the same pass.
                match validate_draft(&self.state) {
                    DraftValidation::Valid(commit) => {
                        match apply_commit(&mut self.state, *commit) {
                            Ok(id) => {
                                self.state.dialogs.net_label_placement.close();
                                self.state.push_user_message(ConsoleMessage::info(format!(
                                "Placed net label as one undoable transaction (stable ID NET-{id:03})."
                            )));
                                self.state.ui.toasts.success(
                                    ctx,
                                    "Net label placed",
                                    "The typed name and snapped anchor were committed atomically.",
                                );
                            }
                            Err(error) => {
                                self.state.dialogs.net_label_placement.validation_error =
                                    Some(error);
                            }
                        }
                    }
                    DraftValidation::Incomplete(message) => {
                        self.state.dialogs.net_label_placement.validation_error =
                            Some(message.to_owned());
                    }
                    DraftValidation::Invalid(message) => {
                        self.state.dialogs.net_label_placement.validation_error = Some(message);
                    }
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.net_label_placement.attempt_close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    let draft = &state.dialogs.net_label_placement;
    let Some(authority) = draft.authority.as_ref() else {
        return DraftValidation::Invalid(
            "The placement authority is unavailable. Cancel and select the anchor again."
                .to_owned(),
        );
    };
    if let Err(error) = authority.validate(state, "Place net label") {
        return DraftValidation::Invalid(error);
    }
    let Some(anchor) = draft.anchor else {
        return DraftValidation::Invalid(
            "The snapped placement anchor is unavailable. Cancel and select it again.".to_owned(),
        );
    };
    let name = draft.name.trim();
    if name.is_empty() {
        return DraftValidation::Incomplete("Enter the electrical net name.");
    }
    if let Err(reason) = NetLabel::validate_name(name, state.schematic.document_policy.net_naming) {
        return DraftValidation::Invalid(format!("Net name: {reason}."));
    }
    DraftValidation::Valid(Box::new(NetLabelPlacementCommit {
        authority: authority.clone(),
        anchor,
        name: name.to_owned(),
    }))
}

fn apply_commit(state: &mut AppState, commit: NetLabelPlacementCommit) -> Result<u64, String> {
    commit.authority.validate(state, "Place net label")?;
    NetLabel::validate_name(&commit.name, state.schematic.document_policy.net_naming)
        .map_err(|reason| format!("Net name: {reason}."))?;

    let mut placed_id = None;
    let changed = state.schematic.with_undo("place net label", |schematic| {
        placed_id = Some(schematic.add_net_label(commit.anchor, commit.name));
    });
    match (changed, placed_id) {
        (true, Some(id)) => {
            state.sync_active_schematic_to_workspace();
            Ok(id)
        }
        _ => Err(
            "The active schematic rejected the placement; no label or undo record was created."
                .to_owned(),
        ),
    }
}

fn dialog_body(
    ui: &mut Ui,
    anchor: Option<Point>,
    grid_pitch: crate::state::SchematicGridPitch,
    naming_policy: NetNamingPolicy,
    draft: &mut NetLabelPlacementDialogState,
    validation_message: Option<&str>,
    validation_is_error: bool,
) -> (Option<egui::Id>, bool) {
    let anchor_text = anchor.map_or_else(
        || "anchor unavailable".to_owned(),
        |anchor| {
            format!(
                "({}, {}) \u{00b7} snapped {}",
                anchor.x,
                anchor.y,
                snap_label(grid_pitch)
            )
        },
    );
    read_only_value(ui, "Anchor", &anchor_text);
    ui.add_space(10.0);

    let t = Tokens::get(ui.ctx());
    let response = field_label(ui, "Net name", |ui| {
        ui.add_sized(
            Vec2::new(ui.available_width(), t.metrics.ctl_h),
            TextEdit::singleline(&mut draft.name)
                .id_source(FIELD_ID)
                .font(egui::TextStyle::Monospace)
                .hint_text("for example: afe_out or DATA[7]")
                .margin(egui::Margin::symmetric(8, 4)),
        )
    });
    configure_name_accessibility(ui, &response, validation_message, validation_is_error);
    let changed = response.changed();

    // Reserve a stable validation row so typing never moves the remaining
    // controls or footer.
    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), 32.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            if let Some(message) = validation_message {
                ui.add_space(4.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(message)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(if validation_is_error {
                                t.color.err
                            } else {
                                t.color.text_dim
                            }),
                    )
                    .wrap(),
                );
            }
        },
    );

    let policy = match naming_policy {
        NetNamingPolicy::StrictCaseSensitive => "Strict case-sensitive SPICE syntax",
        NetNamingPolicy::SpiceCompatibleRelaxed => "SPICE-compatible relaxed syntax",
    };
    read_only_value(ui, "Document naming policy", policy);
    ui.add_space(10.0);
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(5.0)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_min_width((ui.available_width() - 16.0).max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(
                        "Labels with the same accepted name form one electrical net. Cancel or Escape leaves the document unchanged.",
                    )
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                )
                .wrap(),
            );
        });

    (Some(response.id), changed)
}

fn configure_name_accessibility(
    ui: &Ui,
    response: &Response,
    message: Option<&str>,
    invalid: bool,
) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label("Net name");
        node.set_description(match message {
            Some(message) => format!("Required electrical net name. {message}"),
            None => "Required electrical net name".to_owned(),
        });
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
        } else {
            node.clear_invalid();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Tool;

    fn dialog_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 850.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn key_event(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn open_at(app: &mut RSpiceApp, anchor: Point) {
        app.state.schematic.tool = Tool::Label;
        assert!(open_net_label_placement(&mut app.state, anchor));
    }

    #[test]
    fn anchor_capture_is_non_mutating_and_valid_commit_is_one_undo_unit() {
        let mut app = RSpiceApp::test_instance();
        let topology_before = app.state.schematic.topology_version();
        open_at(&mut app, Point::new(40, -20));
        assert!(app.state.schematic.net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
        assert_eq!(app.state.schematic.topology_version(), topology_before);

        app.state.dialogs.net_label_placement.name = "DATA[7]".to_owned();
        let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
            panic!("valid typed label draft");
        };
        let id = apply_commit(&mut app.state, *commit).expect("placement");
        assert_eq!(
            app.state.schematic.net_labels,
            vec![NetLabel::new(id, Point::new(40, -20), "DATA[7]")]
        );
        assert!(app.state.schematic.can_undo());
        assert_eq!(app.state.schematic.tool, Tool::Label);
        assert!(app.state.schematic.undo());
        assert!(app.state.schematic.net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn invalid_name_cannot_produce_a_commit_or_history() {
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::origin());
        app.state.dialogs.net_label_placement.name = "two nodes".to_owned();
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        assert!(app.state.schematic.net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());

        app.state.dialogs.net_label_placement.name = "DATA[7".to_owned();
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        assert!(app.state.schematic.net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn cancel_and_confirmed_discard_leave_schematic_and_history_untouched() {
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::new(10, 20));
        app.state.dialogs.net_label_placement.name = "sense".to_owned();
        app.state.dialogs.net_label_placement.mark_edited();
        assert!(!app.state.dialogs.net_label_placement.attempt_close());
        assert!(app.state.dialogs.net_label_placement.open);
        assert!(app.state.dialogs.net_label_placement.attempt_close());
        assert!(!app.state.dialogs.net_label_placement.open);
        assert!(app.state.schematic.net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
        assert_eq!(app.state.schematic.tool, Tool::Label);
    }

    #[test]
    fn read_only_and_stale_authority_fail_closed() {
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::origin());
        app.state.dialogs.net_label_placement.name = "sense".to_owned();
        app.state.schematic.read_only = true;
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        app.state.schematic.read_only = false;
        let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
            panic!("authority should recover after read-only mode is cleared");
        };
        app.state.schematic.bump_topology_version();
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        assert!(apply_commit(&mut app.state, *commit).is_err());
        assert!(app.state.schematic.net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn rendered_enter_commits_and_escape_requires_confirmed_discard() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::new(30, 50));
        app.state.dialogs.net_label_placement.name = "afe_out".to_owned();

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        assert!(!app.state.dialogs.net_label_placement.open);
        assert_eq!(app.state.schematic.net_labels[0].name, "afe_out");
        assert!(app.state.schematic.can_undo());

        open_at(&mut app, Point::new(60, 50));
        app.state.dialogs.net_label_placement.name = "discard_me".to_owned();
        app.state.dialogs.net_label_placement.mark_edited();
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        assert!(app.state.dialogs.net_label_placement.open);
        assert!(app.state.dialogs.net_label_placement.discard_confirm);
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        assert!(!app.state.dialogs.net_label_placement.open);
        assert_eq!(app.state.schematic.net_labels.len(), 1);
    }

    #[test]
    fn committed_label_is_synchronized_to_the_project_owned_cell_view() {
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::new(80, 30));
        app.state.dialogs.net_label_placement.name = "project_net".to_owned();
        let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
            panic!("valid label draft");
        };

        apply_commit(&mut app.state, *commit).expect("placement");

        let key = app.state.workspace.active_schematic_reference().key();
        let retained = app
            .state
            .workspace
            .schematic_buffers
            .get(&key)
            .expect("active schematic is retained by the project workspace");
        assert_eq!(retained.net_labels.len(), 1);
        assert_eq!(retained.net_labels[0].pos, Point::new(80, 30));
        assert_eq!(retained.net_labels[0].name, "project_net");
    }

    #[test]
    fn mockup_action_contract_and_field_identity_are_stable() {
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} CONNECTIVITY");
        assert_eq!(TITLE, "Place net label");
        assert_eq!(PRIMARY, "Place label");
        assert_eq!(FIELD_ID, "place-net-label-name");
        assert_eq!(
            DESCRIPTION,
            "Assign a validated electrical name at the selected snapped schematic anchor."
        );
        for value in [
            EYEBROW,
            TITLE,
            PRIMARY,
            DESCRIPTION,
            DISCARD_TITLE,
            DISCARD_DETAIL,
        ] {
            for forbidden in ['\u{00c2}', '\u{00e2}', '\u{fffd}'] {
                assert!(
                    !value.contains(forbidden),
                    "mojibake in rendered contract string: {value:?}"
                );
            }
        }
    }
}
