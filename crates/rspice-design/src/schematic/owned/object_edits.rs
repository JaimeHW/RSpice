//! Prepared complete-object edits applied through the live document owner.
use super::super::{
    array::{ArrayObjectIds, SchematicArrayError, SchematicArrayImpact, SchematicArrayPlan},
    array_edit::{ArrayEdit, ArraySelection},
    clipboard::ClipboardData,
    clipboard_edit::{ClipboardPaste, PastedObjects},
    component::{Component, LibraryCellInstance},
    deletion::{DeletionSelection, ObjectDeletion},
    find_replace::{self, FindField, FindMatch},
    interface_repair::{InstanceInterfaceRepair, InterfaceRepairs, RepairedInterface},
    replacement::{
        SchematicReplacementAuthority, SchematicReplacementError, SchematicReplacementImpact,
        SchematicReplacementTargetSpec,
    },
    replacement_edit::{InstanceReplacement, ReplacementContext},
};
use super::{DocumentEdit, Schematic};
use crate::resolved_symbol::ResolvedCellSymbol;
use rspice_design_model::{Point, port::PortSpec};
use std::collections::HashSet;

impl Schematic {
    /// Paste `clipboard` at `pos` as one undo step. `hidden` names the cell
    /// view's other sheets' objects: a pasted dot goes only where the open
    /// sheet's wires meet, while fresh reference names avoid every sheet's.
    pub fn paste_at(
        &mut self,
        clipboard: &ClipboardData,
        pos: Point,
        hidden: impl Fn(u64) -> bool,
    ) -> Result<Option<DocumentEdit<PastedObjects<'_>>>, String> {
        let state = self.name_current();
        let Some(paste) = ClipboardPaste::prepare(
            &mut self.document,
            &mut self.identity,
            clipboard,
            pos,
            hidden,
        )?
        else {
            return Ok(None);
        };
        self.history.begin(paste.document(), Some(state), "paste");
        let pasted = paste.commit();
        if pasted.has_content() {
            self.topology_version = self.topology_version.wrapping_add(pasted.topology_changes);
        }
        let closing = self.history.closing_state();
        let committed = self.history.end(pasted.document());
        if committed {
            self.content_version = self.content_version.wrapping_add(1);
        }
        if closing.is_some() {
            // The pasted objects hold the document, so the stamp they leave
            // cannot be read: the content stands unnamed.
            self.position = super::Position::UNNAMED;
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
        let state = self.name_current();
        self.history.begin(&self.document, Some(state), "replace");
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
        let state = self.name_current();
        let deletion = ObjectDeletion::prepare(&mut self.document, selection)?;
        self.history
            .begin(deletion.document(), Some(state), "delete selection");
        let removes_electrical_object = deletion.commit();
        if removes_electrical_object {
            self.invalidate_topology();
        }
        Some(DocumentEdit {
            value: (),
            committed: self.end_operation(),
        })
    }

    /// This schematic without the complete objects `ids` names, and the
    /// markers that hang on them, as a candidate another step publishes: a
    /// sheet deleted with what it holds. Nothing goes on this schematic's own
    /// undo history. `None` when none of them is in the document.
    pub fn without_objects_candidate(&self, ids: &HashSet<u64>) -> Option<Self> {
        let junctions = self
            .document
            .junctions
            .iter()
            .filter(|junction| ids.contains(&junction.id))
            .map(|junction| junction.pos)
            .collect::<Vec<_>>();
        let mut candidate = self.clone();
        let deletion = ObjectDeletion::prepare(
            &mut candidate.document,
            DeletionSelection {
                components: ids,
                wires: ids,
                junctions: junctions.iter().copied(),
                net_labels: ids,
                buses: ids,
                bus_taps: ids,
                design_notes: ids,
                documentation_shapes: ids,
                probes: ids,
            },
        )?;
        deletion.commit();
        candidate.invalidate_topology();
        Some(candidate)
    }

    pub fn array_selection_resolved(
        &mut self,
        selection: ArraySelection<'_, impl Iterator<Item = Point> + Clone>,
        plan: &SchematicArrayPlan,
        terminal_points_for: impl FnMut(&Component) -> Vec<(String, Point)>,
        component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<DocumentEdit<(SchematicArrayImpact, ArrayObjectIds)>, SchematicArrayError> {
        let state = self.name_current();
        let array = ArrayEdit::prepare(
            &mut self.document,
            &mut self.identity,
            selection,
            plan,
            terminal_points_for,
            component_bounds_for,
        )?;
        let impact = array.impact();
        self.history
            .begin(array.document(), Some(state), "create array");
        let objects = array.commit();
        if impact.electrical {
            self.invalidate_topology();
        }
        Ok(DocumentEdit {
            value: (impact, objects),
            committed: self.end_operation(),
        })
    }

    /// Re-draws the parts `in_scope` accepts with what `swap` answers for
    /// them, as one undo step, and counts them.
    pub fn swap_part_symbols(
        &mut self,
        in_scope: impl Fn(u64) -> bool,
        swap: impl Fn(&Component) -> Option<Component>,
    ) -> DocumentEdit<usize> {
        let swapped = self
            .document
            .components
            .iter()
            .enumerate()
            .filter(|(_, component)| in_scope(component.id))
            .filter_map(|(index, component)| Some((index, swap(component)?)))
            .collect::<Vec<_>>();
        if swapped.is_empty() {
            return DocumentEdit {
                value: 0,
                committed: false,
            };
        }
        let count = swapped.len();
        let state = self.name_current();
        self.history
            .begin(&self.document, Some(state), "swap to RSpice symbols");
        for (index, component) in swapped {
            self.document.components[index] = component;
        }
        DocumentEdit {
            value: count,
            committed: self.end_operation(),
        }
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
        let state = self.name_current();
        let replacement =
            InstanceReplacement::prepare(&mut self.document, context, authority, target)?;
        let impact = replacement.impact();
        let reference_counter = replacement.reference_counter();
        self.history
            .begin(replacement.document(), Some(state), "replace instance");
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
        let state = self.name_current();
        let repair = InstanceInterfaceRepair::prepare(
            &mut self.document,
            component_id,
            master_ports,
            resolve_binding,
        )?;
        self.history
            .begin(repair.document(), Some(state), "update instance interface");
        let repaired = repair.commit();
        self.invalidate_topology();
        Ok(DocumentEdit {
            value: repaired,
            committed: self.end_operation(),
        })
    }

    /// Re-bind every instance `repairs` names to the interface its master
    /// presents now, as one undo step. Each repair is validated against what
    /// the ones before it left, exactly as one repair is. An instance whose
    /// repair is refused is reported by name, with why, and left as it was;
    /// the rest still land. Nothing repaired leaves the history untouched.
    pub fn update_instance_interfaces<'ports>(
        &mut self,
        repairs: &'ports [(u64, Vec<PortSpec>)],
        mut resolve_binding: impl FnMut(&LibraryCellInstance) -> Option<ResolvedCellSymbol>,
    ) -> DocumentEdit<InterfaceRepairs<'ports>> {
        let mut candidate = self.document.clone();
        let mut outcome = InterfaceRepairs::default();
        for (component_id, master_ports) in repairs {
            let name = candidate
                .components
                .iter()
                .find(|component| component.id == *component_id)
                .map(|component| component.name.clone())
                .unwrap_or_default();
            match InstanceInterfaceRepair::prepare(
                &mut candidate,
                *component_id,
                master_ports,
                &mut resolve_binding,
            ) {
                Ok(repair) => outcome.repaired.push(repair.commit()),
                Err(error) => outcome.refused.push((name, error)),
            }
        }
        if outcome.repaired.is_empty() {
            return DocumentEdit {
                value: outcome,
                committed: false,
            };
        }
        let state = self.name_current();
        self.history.begin(
            &self.document,
            Some(state),
            "update all instance interfaces",
        );
        self.document = candidate;
        self.invalidate_topology();
        DocumentEdit {
            value: outcome,
            committed: self.end_operation(),
        }
    }
}
