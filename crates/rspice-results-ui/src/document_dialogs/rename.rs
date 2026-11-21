//! The name a saved result document goes by: its tab's, its report's, and
//! the one no other document of the project may share.

use egui::{Context, Id};
use rspice_ui_kit::widgets::{CommandForm, DialogChoice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameDraft {
    /// The name the document has now.
    pub from: String,
    /// The name typed.
    pub title: String,
}

impl RenameDraft {
    /// A draft that starts from the document's name, ready to be typed over.
    #[must_use]
    pub fn new(title: &str) -> Self {
        Self {
            from: title.to_owned(),
            title: title.to_owned(),
        }
    }
}

pub fn show(ctx: &Context, draft: &mut RenameDraft) -> DialogChoice {
    let field = Id::new("rspice.rename-result-document.name");
    CommandForm::new("Rename document", "Rename")
        .describe("Give the saved result document a name no other document of the project has.")
        .first_field(field)
        .primary_on_enter(true)
        .show(ctx, |rows| {
            rows.value("Document", &draft.from);
            rows.text_sans("Name", field, &mut draft.title, "");
            None
        })
}
