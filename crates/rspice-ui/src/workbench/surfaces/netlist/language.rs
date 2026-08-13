//! Purpose-built semantic source actions.

use crate::diagnostics::ConsoleMessage;
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::{MessageId, RSpiceApp};

pub(super) fn rename_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.ui.netlist.rename_dialog.open {
        return;
    }
    let mut dialog = app.state.ui.netlist.rename_dialog.clone();
    let messages = app.state.ui.messages();
    let replacement_valid = {
        let value = dialog.replacement.trim();
        !value.is_empty()
            && !value.eq_ignore_ascii_case(&dialog.original)
            && value.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '_' | '$')
            })
    };
    let choice = Dialog::new(
        messages.text(MessageId::NetlistRenameEyebrow),
        messages.text(MessageId::NetlistRenameTitle),
        messages.text(MessageId::NetlistRename),
    )
    .description(messages.text(MessageId::NetlistRenameDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(replacement_valid)
    .ghost(messages.text(MessageId::CommonCancel))
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        ui.horizontal(|ui| {
            ui.label("Current name");
            ui.monospace(&dialog.original);
        });
        ui.label(messages.text(MessageId::NetlistRenameNewName));
        let response = ui.add(
            egui::TextEdit::singleline(&mut dialog.replacement)
                .desired_width(f32::INFINITY)
                .char_limit(240),
        );
        if let Some(error) = dialog.error.as_deref() {
            ui.colored_label(t.color.err, error);
        }
        Some(response.id)
    });
    match choice {
        DialogChoice::Primary => {
            app.state.ui.netlist.rename_dialog = dialog;
            match crate::workbench::documents::netlist_document::language::commit_rename(
                &mut app.state,
            ) {
                Ok(count) => app
                    .state
                    .push_user_message(ConsoleMessage::info(messages.format(
                        MessageId::NetlistRenameSucceeded,
                        &[("count", &count.to_string())],
                    ))),
                Err(error) => app.state.ui.netlist.rename_dialog.error = Some(error),
            }
        }
        DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
            app.state.ui.netlist.rename_dialog = Default::default();
        }
        DialogChoice::None => app.state.ui.netlist.rename_dialog = dialog,
    }
}
