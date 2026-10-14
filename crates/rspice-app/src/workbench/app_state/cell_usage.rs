//! Where a library cell is placed: the schematic instances that bind it.
//!
//! The library's Where used report, the inspector's consumer count and the
//! Verilog-A navigator's Used in section all ask this one question, so it is
//! answered once. Every schematic of the project is in the workspace's
//! buffers; the one open in the editor is read from the editor instead,
//! because its buffer is only brought up to date when the view is left.

use rspice_design::schematic::document::SchematicDocument;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::AppState;

/// One placed instance of a library cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CellInstanceUse {
    /// The schematic the instance is placed in.
    pub(crate) schematic: CellViewRef,
    /// The instance's name in that schematic, `I3`.
    pub(crate) instance: String,
    /// The instance's component identity in that schematic.
    pub(crate) component_id: u64,
}

impl AppState {
    /// Every instance in the project that binds `library`/`cell`, ordered by
    /// schematic and then by instance name.
    ///
    /// With `view` it counts only instances bound to that view. Without one it
    /// counts every view of the cell: an instance placed from a cell's symbol
    /// is still an instance of the cell its Verilog-A view describes.
    pub(crate) fn cell_instances(
        &self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) -> Vec<CellInstanceUse> {
        let content = &self.workspace.content;
        let live = matches!(
            content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
        .then(|| content.active_schematic_reference());
        let mut uses = Vec::new();
        if let Some(reference) = &live {
            collect(
                self.schematic.as_ref(),
                reference,
                (library, cell, view),
                &mut uses,
            );
        }
        for (key, schematic) in &content.schematic_buffers {
            let Some(reference) = reference_from_key(key) else {
                continue;
            };
            if live
                .as_ref()
                .is_some_and(|live| live.key().eq_ignore_ascii_case(key))
            {
                continue;
            }
            collect(
                schematic.as_ref(),
                &reference,
                (library, cell, view),
                &mut uses,
            );
        }
        uses.sort_by(|left, right| {
            (
                left.schematic.key().to_ascii_lowercase(),
                left.instance.to_ascii_lowercase(),
            )
                .cmp(&(
                    right.schematic.key().to_ascii_lowercase(),
                    right.instance.to_ascii_lowercase(),
                ))
        });
        uses
    }
}

fn collect(
    document: &SchematicDocument,
    schematic: &CellViewRef,
    (library, cell, view): (&str, &str, Option<&str>),
    uses: &mut Vec<CellInstanceUse>,
) {
    for component in &document.components {
        let Some(binding) = component.library_cell.as_ref() else {
            continue;
        };
        if binding.library.eq_ignore_ascii_case(library)
            && binding.cell.eq_ignore_ascii_case(cell)
            && view.is_none_or(|view| binding.view.eq_ignore_ascii_case(view))
        {
            uses.push(CellInstanceUse {
                schematic: schematic.clone(),
                instance: component.name.clone(),
                component_id: component.id,
            });
        }
    }
}

/// The reference a `library/cell/view` buffer key names. Names cannot hold a
/// slash, so the key splits exactly.
fn reference_from_key(key: &str) -> Option<CellViewRef> {
    let mut parts = key.split('/');
    let (Some(library), Some(cell), Some(view), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return None;
    };
    Some(CellViewRef::new(library, cell, view))
}

#[cfg(test)]
mod tests {
    use crate::state::{CellViewRef, LibraryCellInstance, Point, SchematicState};
    use crate::workbench::app_state::AppState;

    /// The open schematic is read from the editor, never twice, and every
    /// other schematic from its buffer; a view filter keeps only that view.
    #[test]
    fn instances_come_from_the_open_editor_and_every_other_buffer_once() {
        let mut state = AppState::default();
        let active = state.workspace.content.active_view.clone();
        let symbol = LibraryCellInstance::new("behavioral", "gain", "symbol");
        let veriloga = LibraryCellInstance::new("behavioral", "gain", "veriloga");
        state
            .schematic
            .add_library_cell_component(Point::origin(), symbol.clone());
        // A stale copy of the open schematic must not be counted beside it.
        state
            .workspace
            .insert_schematic_editor(active.key(), SchematicState::default());
        let mut bench = SchematicState::default();
        bench.add_library_cell_component(Point::origin(), veriloga);
        state
            .workspace
            .insert_schematic_editor("work/bench/schematic".to_owned(), bench);

        let every_view = state.cell_instances("behavioral", "gain", None);
        assert_eq!(
            every_view
                .iter()
                .map(|usage| usage.schematic.clone())
                .collect::<Vec<_>>(),
            [
                active.clone(),
                CellViewRef::new("work", "bench", "schematic")
            ]
        );
        let symbol_only = state.cell_instances("behavioral", "gain", Some("symbol"));
        assert_eq!(symbol_only.len(), 1);
        assert_eq!(symbol_only[0].schematic, active);
    }
}
