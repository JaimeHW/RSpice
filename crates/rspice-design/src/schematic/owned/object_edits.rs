//! Prepared complete-object edits applied through the live document owner.
use super::super::{
    array::{ArrayObjectIds, SchematicArrayError, SchematicArrayImpact, SchematicArrayPlan},
    array_edit::{ArrayEdit, ArraySelection},
    clipboard::ClipboardData,
    clipboard_edit::{ClipboardPaste, PastedObjects},
    component::{Component, LibraryCellInstance},
    deletion::{DeletionSelection, ObjectDeletion},
    find_replace::{self, FindField, FindMatch},
    interface_repair::{InstanceInterfaceRepair, RepairedInterface},
    replacement::{
        SchematicReplacementAuthority, SchematicReplacementError, SchematicReplacementImpact,
        SchematicReplacementTargetSpec,
    },
    replacement_edit::{InstanceReplacement, ReplacementContext},
};
use super::{DocumentEdit, Schematic};
use crate::resolved_symbol::ResolvedCellSymbol;
use rspice_design_model::{Point, port::PortSpec};

impl Schematic {
    pub fn paste_at(
        &mut self,
        clipboard: &ClipboardData,
        pos: Point,
    ) -> Result<Option<DocumentEdit<PastedObjects<'_>>>, String> {
        let Some(paste) =
            ClipboardPaste::prepare(&mut self.document, &mut self.identity, clipboard, pos)?
        else {
            return Ok(None);
        };
        self.history.begin(paste.document(), "paste");
        let pasted = paste.commit();
        if pasted.has_content() {
            self.topology_version = self.topology_version.wrapping_add(pasted.topology_changes);
        }
        let committed = self.history.end(pasted.document());
        if committed {
            self.content_version = self.content_version.wrapping_add(1);
        }
        Ok(Some(DocumentEdit {
            value: pasted,
            committed,
        }))
    }

    /// Make the text changes a find and replace listed, as one undo step,
    /// after proving every new name legal and every instance name unique.
    /// Returns how many texts changed.
    pub fn replace_text(&mut self, matches: &[FindMatch]) -> Result<DocumentEdit<usize>, String> {
        if let Some(refusal) = find_replace::refusal(&self.document, matches) {
            return Err(refusal);
        }
        self.history.begin(&self.document, "replace");
        let changed = find_replace::replace_in_document(&mut self.document, matches);
        if matches
            .iter()
            .any(|found| found.field == FindField::NetLabel)
        {
            self.invalidate_topology();
        }
        Ok(DocumentEdit {
            value: changed,
            committed: self.end_operation(),
        })
    }

    pub fn delete_objects(
        &mut self,
        selection: DeletionSelection<'_, impl Iterator<Item = Point> + Clone>,
    ) -> Option<DocumentEdit<()>> {
        let deletion = ObjectDeletion::prepare(&mut self.document, selection)?;
        self.history.begin(deletion.document(), "delete selection");
        let removes_electrical_object = deletion.commit();
        if removes_electrical_object {
            self.invalidate_topology();
        }
        Some(DocumentEdit {
            value: (),
            committed: self.end_operation(),
        })
    }

    pub fn array_selection_resolved(
        &mut self,
        selection: ArraySelection<'_, impl Iterator<Item = Point> + Clone>,
        plan: &SchematicArrayPlan,
        terminal_points_for: impl FnMut(&Component) -> Vec<(String, Point)>,
        component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<DocumentEdit<(SchematicArrayImpact, ArrayObjectIds)>, SchematicArrayError> {
        let array = ArrayEdit::prepare(
            &mut self.document,
            &mut self.identity,
            selection,
            plan,
            terminal_points_for,
            component_bounds_for,
        )?;
        let impact = array.impact();
        self.history.begin(array.document(), "create array");
        let objects = array.commit();
        if impact.electrical {
            self.invalidate_topology();
        }
        Ok(DocumentEdit {
            value: (impact, objects),
            committed: self.end_operation(),
        })
    }

    pub fn replace_instance(
        &mut self,
        selected_component: Option<u64>,
        authority: &SchematicReplacementAuthority,
        target: &SchematicReplacementTargetSpec,
    ) -> Result<DocumentEdit<SchematicReplacementImpact>, SchematicReplacementError> {
        let context = ReplacementContext {
            selected_component,
            topology_version: self.topology_version,
        };
        let replacement =
            InstanceReplacement::prepare(&mut self.document, context, authority, target)?;
        let impact = replacement.impact();
        let reference_counter = replacement.reference_counter();
        self.history
            .begin(replacement.document(), "replace instance");
        replacement.commit();
        if let Some((prefix, number)) = reference_counter {
            self.identity.record_component_number(prefix, number);
        }
        self.invalidate_topology();
        Ok(DocumentEdit {
            value: impact,
            committed: self.end_operation(),
        })
    }

    pub fn update_instance_interface<'ports>(
        &mut self,
        component_id: u64,
        master_ports: &'ports [PortSpec],
        resolve_binding: impl FnMut(&LibraryCellInstance) -> Option<ResolvedCellSymbol>,
    ) -> Result<DocumentEdit<RepairedInterface<'ports>>, SchematicReplacementError> {
        let repair = InstanceInterfaceRepair::prepare(
            &mut self.document,
            component_id,
            master_ports,
            resolve_binding,
        )?;
        self.history
            .begin(repair.document(), "update instance interface");
        let repaired = repair.commit();
        self.invalidate_topology();
        Ok(DocumentEdit {
            value: repaired,
            committed: self.end_operation(),
        })
    }
}
