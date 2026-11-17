//! How the engine reads the deck the Netlist page shows.

use std::path::PathBuf;

use rspice_core::netlist::SourceKind;

use super::{ActiveNetlistDocument, Model, NetlistDocumentState};
use crate::workbench::documents::text_editor_commands::EditorSyntax;

/// Where the shown document is read from, and whether it is read as the root
/// deck: what decides, with its text, how the engine reads it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ShownSource {
    /// The file, or a name that carries its extension; empty for text that is
    /// no file, such as the generated netlist.
    pub path: PathBuf,
    pub root: bool,
}

impl ShownSource {
    /// How the engine reads `text` from here.
    pub(crate) fn kind(&self, text: &str) -> SourceKind {
        SourceKind::of(&self.path, text, self.root)
    }

    /// The syntax the editor edits the source in: Spectre for a file that is
    /// Spectre from its first line, which its name alone decides.
    pub(crate) fn syntax(&self) -> EditorSyntax {
        if self.kind("") == SourceKind::Spectre {
            EditorSyntax::Spectre
        } else {
            EditorSyntax::Spice
        }
    }
}

/// The source the page shows, over the model and the netlist editor's state;
/// `None` for the comparison, which is a diff rather than a deck.
pub(crate) fn shown_source_in(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<ShownSource> {
    if let Some(dependency) = super::active_dependency_in(model, netlist) {
        let locator = dependency.locator();
        return Some(ShownSource {
            path: PathBuf::from(
                locator
                    .native_origin()
                    .unwrap_or_else(|| locator.logical_identity()),
            ),
            root: false,
        });
    }
    let path = match netlist.active_document {
        ActiveNetlistDocument::OwnedSource => {
            let content = &model.workspace.content;
            content.netlist_source_path.clone().or_else(|| {
                content
                    .netlist_descriptor
                    .as_ref()
                    .map(|descriptor| PathBuf::from(&descriptor.artifact_name))
            })
        }
        ActiveNetlistDocument::Generated | ActiveNetlistDocument::RunSnapshot => None,
        ActiveNetlistDocument::GeneratedDiff => return None,
    };
    Some(ShownSource {
        path: path.unwrap_or_default(),
        root: true,
    })
}
