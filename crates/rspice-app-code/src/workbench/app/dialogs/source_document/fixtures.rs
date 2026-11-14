//! What the source document dialog's cases read: the facts its properties
//! state for the document in front, worded in English. The dialog's own
//! cases and the code feature's whole-application tests share them.

use super::{DocumentFacts, document_facts};
use crate::workbench::MessageCatalog;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState;
use crate::workbench::state::Workspace;

pub(crate) fn english() -> MessageCatalog {
    MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
}

pub fn facts_of(
    workspace: Workspace,
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
) -> Option<DocumentFacts> {
    document_facts(workspace, model, code, platform, english())
}
