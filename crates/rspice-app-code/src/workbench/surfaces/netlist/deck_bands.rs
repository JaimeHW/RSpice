//! The bands over a deck's file: a file changed on disk while its tab held
//! edits, an include's edits the check does not read yet, and in the
//! browser a deck that is a file on the desktop, which it cannot read or
//! run. A deck whose file is not where the project says has no band: the
//! page's body in front of it says so, the one place, and offers the two
//! ways out.
//!
//! The disk is looked at when the page is shown and when the window takes
//! the focus back with the page in front, never on every frame; a run and a
//! save look again first.

use egui::Ui;

use crate::ui::widgets::Button;
use crate::workbench::code::CodeFeature;
use crate::workbench::documents::deck_files;
use crate::workbench::framework::feature::Cx;
use crate::workbench::{MessageCatalog, MessageId};

/// The bands over the deck the page shows, after looking at the disk when
/// this frame is one of the moments to.
pub(super) fn show(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>) {
    let messages = cx.wb.messages();
    if disk_moment(ui) {
        for notice in deck_files::check_disk(cx.model, &mut cx.state.netlist) {
            cx.wb.prompt(notice);
        }
    }
    // A deck whose file is missing says so in the page's body alone.
    if deck_files::missing_deck_in_front(cx.model, &cx.state.netlist).is_some() {
        return;
    }
    if let Some(deck) = deck_files::desktop_deck(cx.model, &cx.state.netlist) {
        super::advisory_band(ui, None, |ui| {
            band_row::<MissingAnswer>(
                ui,
                &messages.format(
                    MessageId::NetlistDeckOnTheDesktop,
                    &[
                        ("name", &deck.name),
                        ("path", &deck.path.display().to_string()),
                    ],
                ),
                &[],
            );
        });
        return;
    }
    let Some(file) =
        crate::workbench::documents::netlist_document::active_deck_file_in(&cx.state.netlist)
    else {
        return;
    };
    let name = file.name();
    if file.disk_change.is_some() {
        let mut pressed = None;
        super::advisory_band(ui, Some(super::Problem::Warning), |ui| {
            pressed = band_row(
                ui,
                &messages.format(MessageId::NetlistDeckChangedOnDisk, &[("name", &name)]),
                &[
                    (
                        DiskAnswer::Reload,
                        messages.text(MessageId::NetlistDeckReload),
                    ),
                    (
                        DiskAnswer::KeepMine,
                        messages.text(MessageId::NetlistDeckKeepMine),
                    ),
                    (
                        DiskAnswer::Compare,
                        messages.text(MessageId::NetlistDeckCompare),
                    ),
                ],
            );
        });
        let include = cx.state.netlist.front.include().map(str::to_owned);
        let answered = match pressed {
            Some(DiskAnswer::Reload) => {
                deck_files::reload_from_disk(cx.model, &mut cx.state.netlist, include.as_deref())
                    .map(|()| messages.format(MessageId::NetlistDeckReloaded, &[("name", &name)]))
            }
            Some(DiskAnswer::KeepMine) => {
                deck_files::keep_mine(&mut cx.state.netlist, include.as_deref());
                Ok(messages.format(MessageId::NetlistDeckKeptMine, &[("name", &name)]))
            }
            Some(DiskAnswer::Compare) => {
                deck_files::compare_with_disk(cx.model, &mut cx.state.netlist, include.as_deref())
                    .map(|()| {
                        messages.format(MessageId::NetlistDeckComparedWithDisk, &[("name", &name)])
                    })
            }
            None => return,
        };
        match answered {
            Ok(said) | Err(said) => cx.wb.prompt(said),
        }
        return;
    }
    if file.dirty && cx.state.netlist.front.include().is_some() {
        super::advisory_band(ui, None, |ui| {
            ui.label(messages.text(MessageId::NetlistIncludeNotSaved));
        });
    }
}

/// What the change band's buttons answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiskAnswer {
    Reload,
    KeepMine,
    Compare,
}

/// The page's body for a deck whose file is missing: the file is missing,
/// where it was last, and the two ways out, find it again or take the deck
/// out of the project. Remove from project asks first: its row turns into
/// the question, with Remove and Cancel.
pub(super) fn missing_body(
    ui: &mut Ui,
    cx: &mut Cx<'_, CodeFeature>,
    missing: &deck_files::MissingDeck,
    messages: MessageCatalog,
) {
    let confirming = ui
        .ctx()
        .data(|data| data.get_temp::<bool>(remove_confirm_id(missing)))
        .unwrap_or_default();
    let mut pressed = None;
    crate::workbench::design_system::empty_state_with_actions(
        ui,
        crate::workbench::design_system::WorkbenchIcon::File,
        &messages.format(
            MessageId::NetlistDeckMissingTitle,
            &[("name", &missing.name)],
        ),
        &messages.format(
            MessageId::NetlistDeckMissingDescription,
            &[("path", &missing.path.display().to_string())],
        ),
        |ui| {
            let answers = if confirming {
                ui.add(
                    egui::Label::new(messages.format(
                        MessageId::NetlistDeckRemoveConfirm,
                        &[("name", &missing.name)],
                    ))
                    .truncate(),
                );
                vec![
                    (
                        MissingAnswer::Remove,
                        messages.text(MessageId::NetlistDeckRemove),
                    ),
                    (
                        MissingAnswer::Cancel,
                        messages.text(MessageId::CommonCancel),
                    ),
                ]
            } else {
                let mut answers = Vec::new();
                if cfg!(not(target_arch = "wasm32")) {
                    answers.push((
                        MissingAnswer::Locate,
                        messages.text(MessageId::NetlistDeckLocate),
                    ));
                }
                answers.push((
                    MissingAnswer::Ask,
                    messages.text(MessageId::NetlistDeckRemoveFromProject),
                ));
                answers
            };
            for (answer, label) in answers {
                if ui.button(label).clicked() {
                    pressed = Some(answer);
                }
            }
        },
    );
    if let Some(answer) = pressed {
        answer_missing(ui, cx, missing, answer);
    }
}

/// Carry out what the missing deck's body answered.
fn answer_missing(
    ui: &Ui,
    cx: &mut Cx<'_, CodeFeature>,
    missing: &deck_files::MissingDeck,
    answer: MissingAnswer,
) {
    let messages = cx.wb.messages();
    let confirm_id = remove_confirm_id(missing);
    let locate = answer == MissingAnswer::Locate;
    let remove = answer == MissingAnswer::Remove;
    let ask = answer == MissingAnswer::Ask;
    let cancel = answer == MissingAnswer::Cancel;
    if ask || cancel {
        ui.ctx().data_mut(|data| data.insert_temp(confirm_id, ask));
    }
    if remove {
        ui.ctx().data_mut(|data| data.remove::<bool>(confirm_id));
        match deck_files::remove_deck(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            missing.id,
        ) {
            Ok(name) => cx
                .wb
                .prompt(messages.format(MessageId::NetlistDeckRemoved, &[("name", &name)])),
            Err(error) => cx.wb.prompt(error),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    if locate
        && let Some(path) = crate::workbench::workflows::netlist_workflow::show_open_deck_dialog()
    {
        match deck_files::locate_deck(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            cx.runs,
            missing.id,
            &path,
        ) {
            Ok(()) => cx.wb.prompt(messages.format(
                MessageId::NetlistDeckLocated,
                &[
                    ("name", &missing.name),
                    ("path", &path.display().to_string()),
                ],
            )),
            Err(error) => cx.wb.prompt(error),
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = locate;
}

/// Where the body keeps that it is asking whether to remove `missing`.
fn remove_confirm_id(missing: &deck_files::MissingDeck) -> egui::Id {
    egui::Id::new("netlist.deck_bands.remove").with(missing.id)
}

/// What the missing deck's body answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MissingAnswer {
    Locate,
    Ask,
    Remove,
    Cancel,
}

/// One band's row after its mark: the words on the left, cut short with the
/// whole on hover when the row is too narrow for them, and the buttons kept
/// whole on the right, each as wide as the widest of them. The button
/// pressed, if one was.
fn band_row<T: Copy>(ui: &mut Ui, words: &str, buttons: &[(T, String)]) -> Option<T> {
    let mut pressed = None;
    ui.add_space(MARK_GAP);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = BUTTON_GAP;
        let column = buttons
            .iter()
            .map(|(_, label)| Button::new(label).compact().measured_width(ui))
            .fold(0.0, f32::max);
        for (answer, label) in buttons.iter().rev() {
            if Button::new(label)
                .compact()
                .min_width(column)
                .show(ui)
                .clicked()
            {
                pressed = Some(*answer);
            }
        }
        ui.add_space(BUTTON_GAP);
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.add(egui::Label::new(words).truncate());
        });
    });
    pressed
}

/// The air between a band's buttons, and between its words and the first.
const BUTTON_GAP: f32 = 8.0;

/// The air between a band's mark and its words.
const MARK_GAP: f32 = 6.0;

/// When the page last drew, and whether the window had the focus then.
#[derive(Debug, Clone, Copy)]
struct Seen {
    pass: u64,
    focused: bool,
}

/// Whether this frame is one to look at the disk on: the page is shown
/// after a frame that did not show it, or the window has just taken the
/// focus back.
fn disk_moment(ui: &Ui) -> bool {
    let id = egui::Id::new("netlist.deck_bands.seen");
    let ctx = ui.ctx();
    let now = Seen {
        pass: ctx.cumulative_pass_nr(),
        focused: ctx.input(|input| input.focused),
    };
    let last = ctx.data_mut(|data| {
        let last = data.get_temp::<Seen>(id);
        data.insert_temp(id, now);
        last
    });
    last.is_none_or(|last| now.pass > last.pass.saturating_add(1) || (now.focused && !last.focused))
}

#[cfg(test)]
mod tests {
    use super::{band_row, disk_moment};

    /// A band's buttons stand in equal columns, however long their labels.
    #[test]
    fn a_bands_buttons_are_as_wide_as_each_other() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            ui.horizontal(|ui| {
                band_row(
                    ui,
                    "amp_tb.cir changed on disk.",
                    &[
                        (0, "Reload".to_owned()),
                        (1, "Keep mine".to_owned()),
                        (2, "Compare".to_owned()),
                    ],
                );
            });
        });
        let widths = output
            .platform_output
            .accesskit_update
            .iter()
            .flat_map(|update| update.nodes.iter())
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
            .filter_map(|(_, node)| node.bounds())
            .map(|bounds| bounds.x1 - bounds.x0)
            .collect::<Vec<_>>();
        assert_eq!(widths.len(), 3, "{widths:?}");
        assert!(
            widths.iter().all(|width| (width - widths[0]).abs() < 0.5),
            "{widths:?}"
        );
    }

    /// For each pass that draws the page, whether it looked at the disk;
    /// each pass is `(the window has the focus, the page is drawn)`.
    fn looks(passes: &[(bool, bool)]) -> Vec<bool> {
        let ctx = egui::Context::default();
        let mut looked = Vec::new();
        for &(focused, drawn) in passes {
            let input = egui::RawInput {
                focused,
                ..Default::default()
            };
            let _ = ctx.run_ui(input, |ui| {
                if drawn {
                    looked.push(disk_moment(ui));
                }
            });
        }
        looked
    }

    #[test]
    fn the_disk_is_looked_at_when_the_page_shows_and_when_the_window_takes_the_focus_back() {
        let passes = [
            (true, true),
            (true, true),
            (true, false),
            (true, true),
            (false, true),
            (true, true),
            (true, true),
        ];
        assert_eq!(looks(&passes), [true, false, true, false, true, false]);
    }
}
