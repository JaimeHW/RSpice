//! Typed net-label placement, entered at the click that anchored it.
//!
//! The Label tool captures one snapped canvas anchor and raises a small entry
//! beside it. The entry owns the isolated name draft and publishes a durable
//! [`NetLabel`] only after the complete document authority and naming policy
//! have been revalidated.
//!
//! The off-sheet connector tool reaches the same transaction. Its extra
//! declaration is a direction, so the entry offers exactly one more control
//! and publishes name, anchor and kind together — a connector never exists as
//! a plain label in any recorded state.

use egui::Context;
use rspice_schematic_editor::net_label_placement;
use rspice_schematic_editor::session::net_label_sequence::{NetLabelSequence, entry_names};

use crate::state::{CrossSheetPortDirection, NetLabel, NetLabelKind, Point, Tool};
use crate::ui::widgets::{CanvasEntry, CanvasEntryChoice};

use crate::workbench::app::{RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const TITLE: &str = "Net label";
const CONNECTOR_TITLE: &str = "Off-sheet connector";

/// The most names one entry may ask for, so a mistyped range cannot arm a
/// sequence no one will click through.
const MAX_NAMES: usize = 1024;

#[derive(Debug, Clone)]
struct NetLabelPlacementCommit {
    authority: SchematicEditAuthority,
    anchor: Point,
    name: String,
    kind: NetLabelKind,
    /// What the tool places on the next clicks.
    following: Option<NetLabelSequence>,
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

    /// Why the name is refused, if it is.
    ///
    /// An entry nothing has been typed into has refused nothing: it opens
    /// under the pointer and says so by being there.
    fn refusal(&self) -> Option<&str> {
        match self {
            Self::Incomplete(_) | Self::Valid(_) => None,
            Self::Invalid(message) => Some(message.as_str()),
        }
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

    let kind = if state.schematic.session.editor.tool == Tool::OffSheetConnector {
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
        let refusal = self
            .state
            .dialogs
            .net_label_placement
            .validation_error
            .clone()
            .or_else(|| validation.refusal().map(str::to_owned));
        let off_sheet = self
            .state
            .dialogs
            .net_label_placement
            .kind
            .off_sheet_direction()
            .is_some();
        // The pointer is still where the click that opened this was, so the
        // entry can take its place from it without the canvas having to hand
        // one over.
        let anchor = self.state.dialogs.net_label_placement.screen_anchor_or(
            ctx.pointer_latest_pos()
                .unwrap_or_else(|| ctx.viewport_rect().center()),
        );

        let typed_before = self.state.dialogs.net_label_placement.name.clone();
        let draft = &mut self.state.dialogs.net_label_placement;
        let choice = CanvasEntry::new(
            "place-net-label",
            if off_sheet { CONNECTOR_TITLE } else { TITLE },
            anchor,
        )
        .commit_enabled(validation.can_commit())
        .status(match refusal.as_deref() {
            Some(message) => Err(message),
            None => Ok(None),
        })
        .show(ctx, |rows| {
            net_label_placement::show(rows, &mut draft.name, &mut draft.kind)
        });
        // Typing answers the refusal the last attempt recorded.
        if self.state.dialogs.net_label_placement.name != typed_before {
            self.state.dialogs.net_label_placement.validation_error = None;
        }

        match choice {
            CanvasEntryChoice::Commit => {
                // Revalidate after this frame's text edit so Enter can never
                // publish a candidate that became invalid in the same pass.
                match validate_draft(&self.state) {
                    DraftValidation::Valid(commit) => {
                        match apply_commit(&mut self.state, *commit) {
                            // The label is on the sheet: the drawing is the
                            // report, and a second one would only cover it.
                            Ok(_) => self.state.dialogs.net_label_placement.close(),
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
            CanvasEntryChoice::Cancelled => self.state.dialogs.net_label_placement.close(),
            CanvasEntryChoice::None => {}
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
    // A list or a bus range names one label per click; on a bus, a range
    // names the bus whole.
    let names = entry_names(&draft.name, anchor_on_bus(state, anchor));
    let Some(first) = names.first() else {
        return DraftValidation::Incomplete("Enter the electrical net name.");
    };
    if names.len() > MAX_NAMES {
        return DraftValidation::Invalid(format!("Name at most {MAX_NAMES} nets at once."));
    }
    let policy = state.schematic.document().document_policy.net_naming;
    for name in &names {
        if let Err(reason) = NetLabel::validate_name(name, policy) {
            return DraftValidation::Invalid(if names.len() > 1 {
                format!("{name}: net name: {reason}.")
            } else {
                format!("Net name: {reason}.")
            });
        }
    }
    DraftValidation::Valid(Box::new(NetLabelPlacementCommit {
        authority: authority.clone(),
        anchor,
        name: first.clone(),
        kind: draft.kind,
        following: NetLabelSequence::following(&names, draft.kind),
    }))
}

/// Whether a bus on the active sheet runs through `anchor`.
fn anchor_on_bus(state: &AppState, anchor: Point) -> bool {
    state.schematic.document().buses.iter().any(|bus| {
        bus.contains_point(anchor)
            && crate::schematic::view::sheet_visibility::object_is_on_active_sheet(state, bus.id)
    })
}

/// Place the name an armed sequence holds next at `anchor`, as one undo
/// step, and arm the one after it. Returns whether a sequence answered the
/// click; with none armed, the click opens the entry.
pub(crate) fn place_sequenced_net_label(state: &mut AppState, anchor: Point) -> bool {
    let Some(mut sequence) = state
        .schematic
        .session
        .editor
        .pending_net_label_sequence
        .clone()
    else {
        return false;
    };
    let Some(name) = sequence.next_name() else {
        state.schematic.session.editor.pending_net_label_sequence = None;
        return false;
    };
    if state.dialogs.application_modal_open() {
        return true;
    }
    if state.schematic_edit_read_only() {
        state.deny_read_only_edit();
        return true;
    }
    let policy = state.schematic.document().document_policy.net_naming;
    if let Err(reason) = NetLabel::validate_name(&name, policy) {
        state.schematic.session.editor.pending_net_label_sequence = None;
        state.prompt(format!("{name}: net name: {reason}."));
        return true;
    }
    let kind = sequence.kind;
    let undo_label = if kind.off_sheet_direction().is_some() {
        "place off-sheet connector"
    } else {
        "place net label"
    };
    let placed = state.schematic.with_undo(undo_label, |schematic| {
        schematic.add_net_label_with_kind(anchor, name.clone(), kind);
    });
    if !placed {
        state.prompt(format!("{name} was not placed."));
        return true;
    }
    state.schematic.session.editor.pending_net_label_sequence =
        sequence.advance().then_some(sequence);
    state.sync_active_schematic_to_workspace();
    true
}

fn apply_commit(state: &mut AppState, commit: NetLabelPlacementCommit) -> Result<u64, String> {
    let kind = commit.kind;
    let off_sheet = kind.off_sheet_direction().is_some();
    // History and authority errors name what was actually placed, so a
    // connector never appears in either as an ordinary label.
    commit
        .authority
        .validate(state, if off_sheet { CONNECTOR_TITLE } else { TITLE })?;
    NetLabel::validate_name(
        &commit.name,
        state.schematic.document().document_policy.net_naming,
    )
    .map_err(|reason| format!("Net name: {reason}."))?;

    let mut placed_id = None;
    let undo_label = if off_sheet {
        "place off-sheet connector"
    } else {
        "place net label"
    };
    let changed = state.schematic.with_undo(undo_label, |schematic| {
        let id = schematic.add_net_label_with_kind(commit.anchor, commit.name, kind);
        placed_id = Some(id);
    });
    match (changed, placed_id) {
        (true, Some(id)) => {
            // The tool stays armed with the next name, which the ghost shows.
            state.schematic.session.editor.pending_net_label_sequence = commit.following;
            state.sync_active_schematic_to_workspace();
            Ok(id)
        }
        _ => Err(
            "The active schematic rejected the placement; no label or undo record was created."
                .to_owned(),
        ),
    }
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
        app.state.schematic.session.editor.tool = Tool::Label;
        assert!(open_net_label_placement(&mut app.state, anchor));
    }

    fn open_connector_at(app: &mut RSpiceApp, anchor: Point) {
        app.state.schematic.session.editor.tool = Tool::OffSheetConnector;
        assert!(open_net_label_placement(&mut app.state, anchor));
    }

    #[test]
    fn opening_input_reaches_the_name_field_for_both_placement_kinds() {
        for connector in [false, true] {
            let ctx = Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut app = RSpiceApp::test_instance();
            if connector {
                open_connector_at(&mut app, Point::origin());
            } else {
                open_at(&mut app, Point::origin());
            }
            // The entry asks for the caret once its field exists, so the
            // frame that builds it is the frame before the one that can be
            // typed into.
            let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
                app.render_net_label_dialog(ctx);
            });
            let _ = ctx.run_ui(
                dialog_input(vec![egui::Event::Paste("BUSY".into())]),
                |ctx| {
                    app.render_net_label_dialog(ctx);
                },
            );
            assert_eq!(app.state.dialogs.net_label_placement.name, "BUSY");
            assert!(app.state.schematic.document().net_labels.is_empty());
        }
    }

    #[test]
    fn anchor_capture_is_non_mutating_and_valid_commit_is_one_undo_unit() {
        let mut app = RSpiceApp::test_instance();
        let topology_before = app.state.schematic.topology_version();
        open_at(&mut app, Point::new(40, -20));
        assert!(app.state.schematic.document().net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
        assert_eq!(app.state.schematic.topology_version(), topology_before);

        app.state.dialogs.net_label_placement.name = "DATA[7]".to_owned();
        let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
            panic!("valid typed label draft");
        };
        let id = apply_commit(&mut app.state, *commit).expect("placement");
        assert_eq!(
            app.state.schematic.document().net_labels,
            vec![NetLabel::new(id, Point::new(40, -20), "DATA[7]")]
        );
        assert!(app.state.schematic.can_undo());
        assert_eq!(app.state.schematic.session.editor.tool, Tool::Label);
        assert!(app.state.schematic.undo());
        assert!(app.state.schematic.document().net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn invalid_name_cannot_produce_a_commit_or_history() {
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::origin());
        app.state.dialogs.net_label_placement.name = "bell\u{7}node".to_owned();
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        // One bad name in a list refuses the whole entry, naming it.
        app.state.dialogs.net_label_placement.name = "a DATA[7 c".to_owned();
        match validate_draft(&app.state) {
            DraftValidation::Invalid(message) => assert!(message.starts_with("DATA[7:")),
            other => panic!("a list with a bad name was accepted: {other:?}"),
        }
        assert!(app.state.schematic.document().net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());

        app.state.dialogs.net_label_placement.name = "DATA[7".to_owned();
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        assert!(app.state.schematic.document().net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
    }

    /// Place what the entry names at `anchor`.
    fn commit_entry(state: &mut AppState, anchor: Point, entry: &str) {
        state.schematic.session.editor.tool = Tool::Label;
        assert!(open_net_label_placement(state, anchor));
        state.dialogs.net_label_placement.name = entry.to_owned();
        let DraftValidation::Valid(commit) = validate_draft(state) else {
            panic!("{entry} is a valid entry");
        };
        apply_commit(state, *commit).expect("placement");
        state.dialogs.net_label_placement.close();
    }

    fn label_names(app: &RSpiceApp) -> Vec<String> {
        app.state
            .schematic
            .document()
            .net_labels
            .iter()
            .map(|label| label.name.clone())
            .collect()
    }

    fn next_name(app: &RSpiceApp) -> Option<String> {
        app.state
            .schematic
            .session
            .editor
            .pending_net_label_sequence
            .as_ref()
            .and_then(|sequence| sequence.next_name())
    }

    /// After `D0` the tool stays armed with `D1`: each click places the
    /// next number without the entry, as its own undo step (audit G-6).
    #[test]
    fn a_numbered_label_keeps_the_tool_armed_with_the_next_name() {
        let mut app = RSpiceApp::test_instance();
        commit_entry(&mut app.state, Point::new(0, 0), "D0");
        assert_eq!(next_name(&app).as_deref(), Some("D1"));

        assert!(place_sequenced_net_label(&mut app.state, Point::new(0, 20)));
        assert!(place_sequenced_net_label(&mut app.state, Point::new(0, 40)));
        assert!(!app.state.dialogs.net_label_placement.open);
        assert_eq!(label_names(&app), ["D0", "D1", "D2"]);
        assert_eq!(next_name(&app).as_deref(), Some("D3"));
        assert_eq!(app.state.schematic.session.editor.tool, Tool::Label);

        assert!(app.state.schematic.undo());
        assert_eq!(label_names(&app), ["D0", "D1"]);
        // Esc ends the tool and the sequence with it.
        app.state.schematic.cancel_interaction_step();
        assert_eq!(next_name(&app), None);
    }

    /// A list or a bus range places one name per click, then the next click
    /// asks for a name again; a range typed onto a bus names the bus.
    #[test]
    fn a_list_or_a_bus_range_places_one_name_per_click() {
        for (entry, expected) in [
            ("a b c", vec!["a", "b", "c"]),
            ("D<0:2>", vec!["D<0>", "D<1>", "D<2>"]),
        ] {
            let mut app = RSpiceApp::test_instance();
            commit_entry(&mut app.state, Point::new(0, 0), entry);
            let mut y = 0;
            while next_name(&app).is_some() {
                y += 20;
                assert!(place_sequenced_net_label(&mut app.state, Point::new(0, y)));
            }
            assert_eq!(label_names(&app), expected);
            assert!(!place_sequenced_net_label(
                &mut app.state,
                Point::new(0, 200)
            ));
        }

        let mut app = RSpiceApp::test_instance();
        app.state.schematic.document_mut_for_test().buses.push(
            crate::state::Bus::segment(90, Point::new(-50, 0), Point::new(50, 0), None)
                .expect("bus"),
        );
        commit_entry(&mut app.state, Point::new(0, 0), "D<0:2>");
        assert_eq!(label_names(&app), ["D<0:2>"]);
        assert_eq!(next_name(&app), None);
    }

    /// Leaving the entry leaves the document alone and keeps the tool armed,
    /// so the next click names the next net.
    #[test]
    fn closing_the_entry_leaves_schematic_and_history_untouched() {
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::new(10, 20));
        app.state.dialogs.net_label_placement.name = "sense".to_owned();
        app.state.dialogs.net_label_placement.close();
        assert!(!app.state.dialogs.net_label_placement.open);
        assert!(app.state.schematic.document().net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
        assert_eq!(app.state.schematic.session.editor.tool, Tool::Label);
    }

    #[test]
    fn read_only_and_stale_authority_fail_closed() {
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::origin());
        app.state.dialogs.net_label_placement.name = "sense".to_owned();
        app.state.schematic.session.read_only = true;
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        app.state.schematic.session.read_only = false;
        let DraftValidation::Valid(commit) = validate_draft(&app.state) else {
            panic!("authority should recover after read-only mode is cleared");
        };
        app.state.schematic.bump_topology_version();
        assert!(matches!(
            validate_draft(&app.state),
            DraftValidation::Invalid(_)
        ));
        assert!(apply_commit(&mut app.state, *commit).is_err());
        assert!(app.state.schematic.document().net_labels.is_empty());
        assert!(!app.state.schematic.can_undo());
    }

    /// Enter places the label; Escape leaves at once, because the entry owns
    /// nothing there is anything to confirm the discard of.
    #[test]
    fn rendered_enter_commits_and_escape_closes_at_once() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::new(30, 50));
        app.state.dialogs.net_label_placement.name = "sense_out".to_owned();

        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Enter)]), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        assert!(!app.state.dialogs.net_label_placement.open);
        assert_eq!(
            app.state.schematic.document().net_labels[0].name,
            "sense_out"
        );
        assert!(app.state.schematic.can_undo());

        open_at(&mut app, Point::new(60, 50));
        app.state.dialogs.net_label_placement.name = "discard_me".to_owned();
        let _ = ctx.run_ui(dialog_input(Vec::new()), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        let _ = ctx.run_ui(dialog_input(vec![key_event(egui::Key::Escape)]), |ctx| {
            app.render_net_label_dialog(ctx)
        });
        assert!(!app.state.dialogs.net_label_placement.open);
        assert_eq!(app.state.schematic.document().net_labels.len(), 1);
    }

    /// The entry sits where the click did, not in the middle of the screen,
    /// and stays there while the name is typed.
    #[test]
    fn the_entry_is_anchored_at_the_click_and_does_not_follow_the_pointer() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        open_at(&mut app, Point::new(30, 50));
        let click = egui::pos2(420.0, 260.0);
        let mut input = dialog_input(vec![egui::Event::PointerMoved(click)]);
        input.events.push(egui::Event::PointerMoved(click));
        let _ = ctx.run_ui(input, |ctx| app.render_net_label_dialog(ctx));
        let anchored = app
            .state
            .dialogs
            .net_label_placement
            .screen_anchor
            .expect("the entry took its place from the click");
        assert_eq!(anchored, click);

        let _ = ctx.run_ui(
            dialog_input(vec![egui::Event::PointerMoved(egui::pos2(900.0, 700.0))]),
            |ctx| app.render_net_label_dialog(ctx),
        );
        assert_eq!(
            app.state.dialogs.net_label_placement.screen_anchor,
            Some(anchored),
            "the card does not chase the pointer"
        );
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

        let key = app
            .state
            .workspace
            .content
            .active_schematic_reference()
            .key();
        let retained = app
            .state
            .workspace
            .content
            .schematic_buffers
            .get(&key)
            .expect("active schematic is retained by the project workspace");
        assert_eq!(retained.document().net_labels.len(), 1);
        assert_eq!(retained.document().net_labels[0].pos, Point::new(80, 30));
        assert_eq!(retained.document().net_labels[0].name, "project_net");
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
            app.state.schematic.document().net_labels,
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
        assert!(app.state.schematic.document().net_labels.is_empty());

        // The plain label tool still publishes a local label through the same
        // transaction. The primary action closes the dialog after a commit,
        // and no placement opens over a modal that is still up.
        app.state.dialogs.net_label_placement.close();
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
        schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::off_sheet(
                near_label,
                Point::origin(),
                "BIAS",
                CrossSheetPortDirection::Output,
            ));
        let far_label = schematic.next_id();
        schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::off_sheet(
                far_label,
                Point::new(1_000_000, 0),
                "BIAS",
                CrossSheetPortDirection::Input,
            ));

        let joined: Vec<_> = rspice_design::connectivity::summary::design_nets(&schematic)
            .into_iter()
            .filter(|net| net.name == "BIAS")
            .collect();
        assert_eq!(joined.len(), 1, "the shared name is one node");
        assert!(joined[0].authored_name);
        assert_eq!(joined[0].wire_ids.len(), 2, "both conductors joined");
    }
}
