//! Typed net-label placement transaction.
//!
//! The Label tool first captures one snapped canvas anchor. This dialog owns
//! the isolated name draft and publishes a durable [`NetLabel`] only after the
//! complete document authority and naming policy have been revalidated.
//!
//! The off-sheet connector tool reaches the same transaction. Its extra
//! declaration is a direction, so the dialog offers exactly one more control
//! and publishes name, anchor and kind together — a connector never exists as
//! a plain label in any recorded state.

use egui::{Context, Frame, Response, Stroke, TextEdit, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{CrossSheetPortDirection, NetLabel, NetLabelKind, NetNamingPolicy, Point, Tool};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, select,
};

use crate::workbench::app::dialogs::schematic_command::{field_label, read_only_value, snap_label};
use crate::workbench::app::{NetLabelPlacementDialogState, RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC \u{00b7} CONNECTIVITY";
const TITLE: &str = "Place net label";
const PRIMARY: &str = "Place label";
const DESCRIPTION: &str =
    "Assign a validated electrical name at the selected snapped schematic anchor.";
const CONNECTOR_TITLE: &str = "Place off-sheet connector";
const CONNECTOR_PRIMARY: &str = "Place connector";
const CONNECTOR_DESCRIPTION: &str =
    "Declare a validated electrical name that continues on another sheet of this cellview.";
const FIELD_ID: &str = "place-net-label-name";
const DIRECTION_FIELD_ID: &str = "place-net-label-direction";
const DIRECTION_LABEL: &str = "Direction";
const DISCARD_TITLE: &str = "Unsaved label name";
const DISCARD_DETAIL: &str =
    "Choose Discard changes again to close. The schematic and undo history are unchanged.";

#[derive(Debug, Clone)]
struct NetLabelPlacementCommit {
    authority: SchematicEditAuthority,
    anchor: Point,
    name: String,
    kind: NetLabelKind,
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
/// The armed tool decides which label is being placed, so the canvas never has
/// to carry a second entry point for the connector.
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

    let kind = if state.schematic.tool == Tool::OffSheetConnector {
        NetLabelKind::OffSheet {
            direction: CrossSheetPortDirection::default(),
        }
    } else {
        NetLabelKind::Local
    };
    let authority = SchematicEditAuthority::capture(state);
    state
        .dialogs
        .net_label_placement
        .open(anchor, authority, kind);
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
        let off_sheet = self
            .state
            .dialogs
            .net_label_placement
            .kind
            .off_sheet_direction()
            .is_some();
        let mut edited = false;
        let mut dialog = Dialog::new(
            EYEBROW,
            if off_sheet { CONNECTOR_TITLE } else { TITLE },
            if off_sheet {
                CONNECTOR_PRIMARY
            } else {
                PRIMARY
            },
        )
        .description(if off_sheet {
            CONNECTOR_DESCRIPTION
        } else {
            DESCRIPTION
        })
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
                                let noun = if off_sheet {
                                    "off-sheet connector"
                                } else {
                                    "net label"
                                };
                                self.state.push_user_message(ConsoleMessage::info(format!(
                                    "Placed {noun} as one undoable transaction (stable ID NET-{id:03})."
                                )));
                                self.state.ui.toasts.success(
                                    ctx,
                                    if off_sheet {
                                        "Off-sheet connector placed"
                                    } else {
                                        "Net label placed"
                                    },
                                    if off_sheet {
                                        "The typed name, its direction and the snapped anchor were committed atomically."
                                    } else {
                                        "The typed name and snapped anchor were committed atomically."
                                    },
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
        kind: draft.kind,
    }))
}

fn apply_commit(state: &mut AppState, commit: NetLabelPlacementCommit) -> Result<u64, String> {
    let kind = commit.kind;
    let off_sheet = kind.off_sheet_direction().is_some();
    // History and authority errors name what was actually placed, so a
    // connector never appears in either as an ordinary label.
    commit
        .authority
        .validate(state, if off_sheet { CONNECTOR_TITLE } else { TITLE })?;
    NetLabel::validate_name(&commit.name, state.schematic.document_policy.net_naming)
        .map_err(|reason| format!("Net name: {reason}."))?;

    let mut placed_id = None;
    let undo_label = if off_sheet {
        "place off-sheet connector"
    } else {
        "place net label"
    };
    let changed = state.schematic.with_undo(undo_label, |schematic| {
        let id = schematic.add_net_label(commit.anchor, commit.name);
        // Inside the same transaction, so no recorded state ever holds the
        // connector as an ordinary local label.
        if let Some(label) = schematic.net_labels.iter_mut().find(|label| label.id == id) {
            label.kind = kind;
        }
        placed_id = Some(id);
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
    let mut changed = response.changed();
    if let Some(direction) = draft.kind.off_sheet_direction() {
        ui.add_space(10.0);
        changed |= direction_field(ui, direction, &mut draft.kind);
    }

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
    let note = if draft.kind.off_sheet_direction().is_some() {
        "Connectors with the same accepted name form one electrical net across every sheet of this cellview. Cancel or Escape leaves the document unchanged."
    } else {
        "Labels with the same accepted name form one electrical net. Cancel or Escape leaves the document unchanged."
    };
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(5.0)
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.set_min_width((ui.available_width() - 16.0).max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(note)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });

    (Some(response.id), changed)
}

/// The one control the connector adds. It is offered only when the armed tool
/// declared an off-sheet kind, so the plain label transaction is unchanged.
fn direction_field(
    ui: &mut Ui,
    selected: CrossSheetPortDirection,
    kind: &mut NetLabelKind,
) -> bool {
    let options = NetLabelKind::DIRECTIONS
        .map(|direction| NetLabelKind::direction_label(direction).to_owned());
    field_label(ui, DIRECTION_LABEL, |ui| {
        select(
            ui,
            DIRECTION_FIELD_ID,
            DIRECTION_LABEL,
            NetLabelKind::direction_label(selected),
            &options,
            ui.available_width(),
        )
    })
    .is_some_and(|index| {
        *kind = NetLabelKind::OffSheet {
            direction: NetLabelKind::DIRECTIONS[index],
        };
        true
    })
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

    fn open_connector_at(app: &mut RSpiceApp, anchor: Point) {
        app.state.schematic.tool = Tool::OffSheetConnector;
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
    fn the_armed_tool_decides_the_kind_and_publishes_it_with_the_name() {
        let mut app = RSpiceApp::test_instance();
        open_connector_at(&mut app, Point::new(60, -40));
        assert_eq!(
            app.state.dialogs.net_label_placement.kind,
            NetLabelKind::OffSheet {
                direction: CrossSheetPortDirection::default()
            }
        );

        app.state.dialogs.net_label_placement.name = "BIAS".to_owned();
        app.state.dialogs.net_label_placement.kind = NetLabelKind::OffSheet {
            direction: CrossSheetPortDirection::Output,
        };
        let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
            panic!("valid connector draft");
        };
        let id = apply_commit(&mut app.state, *commit).expect("placement");

        assert_eq!(
            app.state.schematic.net_labels,
            vec![NetLabel::off_sheet(
                id,
                Point::new(60, -40),
                "BIAS",
                CrossSheetPortDirection::Output
            )]
        );
        assert_eq!(
            app.state.schematic.undo_description(),
            Some("place off-sheet connector"),
            "history must name what was placed"
        );
        assert!(app.state.schematic.undo());
        assert!(app.state.schematic.net_labels.is_empty());

        // The plain label tool still publishes a local label through the same
        // transaction.
        open_at(&mut app, Point::new(10, 10));
        assert_eq!(
            app.state.dialogs.net_label_placement.kind,
            NetLabelKind::Local
        );
    }

    /// The kind is a drawing and review declaration, never an electrical one.
    /// Sheets are projected into separate coordinate namespaces, so two
    /// connectors that share a name have no geometry in common — the name is
    /// the only thing that can join them, exactly as for a plain label.
    #[test]
    fn connectors_sharing_a_name_join_into_one_node_across_sheet_namespaces() {
        let mut schematic = crate::state::SchematicState::default();
        let near = schematic
            .add_wire(vec![Point::origin(), Point::new(40, 0)])
            .expect("first sheet conductor");
        let far = schematic
            .add_wire(vec![Point::new(1_000_000, 0), Point::new(1_000_040, 0)])
            .expect("second sheet conductor");
        assert_ne!(near, far);
        let near_label = schematic.next_id();
        schematic.net_labels.push(NetLabel::off_sheet(
            near_label,
            Point::origin(),
            "BIAS",
            CrossSheetPortDirection::Output,
        ));
        let far_label = schematic.next_id();
        schematic.net_labels.push(NetLabel::off_sheet(
            far_label,
            Point::new(1_000_000, 0),
            "BIAS",
            CrossSheetPortDirection::Input,
        ));

        let joined: Vec<_> = crate::simulation::netlist_gen::design_nets(&schematic)
            .into_iter()
            .filter(|net| net.name == "BIAS")
            .collect();
        assert_eq!(joined.len(), 1, "the shared name is one node");
        assert!(joined[0].authored_name);
        assert_eq!(joined[0].wire_ids.len(), 2, "both conductors joined");
    }

    #[test]
    fn mockup_action_contract_and_field_identity_are_stable() {
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} CONNECTIVITY");
        assert_eq!(TITLE, "Place net label");
        assert_eq!(PRIMARY, "Place label");
        assert_eq!(FIELD_ID, "place-net-label-name");
        assert_eq!(CONNECTOR_TITLE, "Place off-sheet connector");
        assert_eq!(CONNECTOR_PRIMARY, "Place connector");
        assert_eq!(DIRECTION_FIELD_ID, "place-net-label-direction");
        assert_ne!(FIELD_ID, DIRECTION_FIELD_ID);
        assert_eq!(
            DESCRIPTION,
            "Assign a validated electrical name at the selected snapped schematic anchor."
        );
        for value in [
            EYEBROW,
            TITLE,
            PRIMARY,
            DESCRIPTION,
            CONNECTOR_TITLE,
            CONNECTOR_PRIMARY,
            CONNECTOR_DESCRIPTION,
            DIRECTION_LABEL,
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
