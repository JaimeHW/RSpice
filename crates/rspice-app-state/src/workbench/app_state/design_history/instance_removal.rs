//! Taking every placement of one deleted library object out of the project,
//! as part of the deletion's own step of Undo however many drawings it
//! reached.

use super::*;

impl Model {
    /// Take every placement of the deleted master `library`/`cell`, or of its
    /// `view` alone, off the live sheet and every loaded drawing, and state in
    /// the Console how many drawings it reached. `display_path` names the
    /// deleted object to the reader.
    ///
    /// Run before the deletion's revision is published: the reader answered
    /// one question about the deletion, so its step of Undo takes the
    /// placements with it, and one Undo puts back the master and every
    /// placement of it.
    pub fn remove_deleted_master_placements(
        &mut self,
        library: &str,
        cell: &str,
        view: Option<&str>,
        display_path: &str,
    ) {
        let places_target = |component: &crate::state::Component| {
            crate::workbench::app_state::cell_usage::places(component, library, cell, view)
        };

        let active = self.workspace.content.active_schematic_reference();
        let active_key = active.key();
        let mut edited = self
            .workspace
            .content
            .schematic_buffers
            .iter()
            .filter(|(key, schematic)| {
                key.as_str() != active_key
                    && schematic.document().components.iter().any(&places_target)
            })
            .filter_map(|(key, _)| {
                buffer_reference(key).map(|reference| {
                    (
                        reference,
                        self.workspace
                            .clone_schematic_editor(key)
                            .expect("retained drawing"),
                    )
                })
            })
            .collect::<Vec<_>>();
        if self
            .schematic
            .document()
            .components
            .iter()
            .any(&places_target)
        {
            edited.push((active, self.schematic.clone()));
        }
        if edited.is_empty() {
            return;
        }

        let drawings = edited.len();
        for (reference, mut drawing) in edited {
            drawing.remove_master_placements(library, cell, view);
            let key = reference.key();
            if key == active_key {
                self.schematic = drawing.clone();
            }
            self.workspace.insert_schematic_editor(key, drawing);
        }
        self.record(
            crate::diagnostics::LogSeverity::Info,
            crate::diagnostics::LogSource::Studio,
            format!(
                "Removed every placement of '{display_path}' from {drawings} {}.",
                if drawings == 1 { "drawing" } else { "drawings" }
            ),
        );
    }
}

/// The exact cell view a loaded schematic buffer belongs to.
///
/// A buffer key is `library/cell/view` and no name segment may contain a
/// slash, so this split is the exact inverse of the key it reads.
fn buffer_reference(key: &str) -> Option<CellViewRef> {
    let mut segments = key.split('/');
    let reference = CellViewRef::new(segments.next()?, segments.next()?, segments.next()?);
    segments.next().is_none().then_some(reference)
}
