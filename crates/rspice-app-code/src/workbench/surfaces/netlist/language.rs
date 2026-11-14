//! Purpose-built semantic source actions.

use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::MessageId;
use crate::workbench::code::CodeFeature;
use crate::workbench::code::commands::CodeRequest;
use crate::workbench::framework::feature::Cx;

pub(super) fn rename_dialog_window(ctx: &egui::Context, cx: &mut Cx<'_, CodeFeature>) {
    if !cx.state.netlist.rename_dialog.open {
        return;
    }
    let mut dialog = cx.state.netlist.rename_dialog.clone();
    let messages = cx.wb.messages();
    let replacement_valid = {
        let value = dialog.replacement.trim();
        !value.is_empty()
            && !value.eq_ignore_ascii_case(&dialog.original)
            && value.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '_' | '$')
            })
    };
    let choice = Dialog::new(
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
            cx.state.netlist.rename_dialog = dialog;
            cx.wb.outbox.push(CodeRequest::CommitNetlistRename);
        }
        DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
            cx.state.netlist.rename_dialog = Default::default();
        }
        DialogChoice::None => cx.state.netlist.rename_dialog = dialog,
    }
}
