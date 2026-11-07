//! The parts a design builds, walked through its hierarchy for a bill of
//! materials.

use super::HierarchySource;
use crate::schematic::component::{Component, LibraryCellInstance};
use crate::schematic::component_type::ComponentType;
use crate::schematic::document::SchematicDocument;
use rspice_design_model::cell_view::CellViewRef;
use rspice_vocabulary::hierarchy_path::InstancePath;

impl<'a> HierarchySource<'a> {
    /// Every component the design builds from `top`, with the reference a
    /// bill of materials states it under: its own name at the top, and
    /// `X1/R3` inside the occurrence `X1` of a cell, one segment per level.
    ///
    /// A cell instance that descends into a schematic is not bought itself:
    /// the parts of that schematic are, once per occurrence. One that writes
    /// its own netlist text (a macromodel, a code model) or whose master is
    /// missing is listed as itself. Where `top` is the root of the plan this
    /// source carries, each occurrence descends into the master the plan
    /// resolved for it and into nothing where the plan stopped or rejected
    /// it, though a compiled built-in the plan stops at is listed as itself;
    /// otherwise into the master its placement names. An occurrence that
    /// reaches a cellview it is already inside is not followed.
    pub fn bill_of_materials_parts(
        &self,
        top: &'a SchematicDocument,
        reference: &CellViewRef,
    ) -> Vec<(String, &'a Component)> {
        let follows_plan = self
            .execution_plan()
            .is_some_and(|plan| plan.root().key().eq_ignore_ascii_case(&reference.key()));
        let mut parts = Vec::new();
        self.collect_parts(
            top,
            follows_plan.then(InstancePath::root),
            "",
            &mut vec![top],
            &mut parts,
        );
        parts
    }

    fn collect_parts(
        &self,
        document: &'a SchematicDocument,
        path: Option<InstancePath>,
        prefix: &str,
        enclosing: &mut Vec<&'a SchematicDocument>,
        parts: &mut Vec<(String, &'a Component)>,
    ) {
        for component in &document.components {
            let reference = format!("{prefix}{}", component.name);
            if component.kind != ComponentType::CellInstance {
                parts.push((reference, component));
                continue;
            }
            let child_path = match &path {
                Some(path) => match path.child(&component.name) {
                    Ok(child) => Some(child),
                    Err(_) => continue,
                },
                None => None,
            };
            match self.bill_of_materials_master(component, child_path.as_ref()) {
                Descent::Itself => parts.push((reference, component)),
                Descent::Nothing => {}
                Descent::Into(master) => {
                    if enclosing.iter().any(|open| std::ptr::eq(*open, master)) {
                        continue;
                    }
                    enclosing.push(master);
                    self.collect_parts(
                        master,
                        child_path,
                        &format!("{reference}/"),
                        enclosing,
                        parts,
                    );
                    enclosing.pop();
                }
            }
        }
    }

    fn bill_of_materials_master(
        &self,
        instance: &Component,
        path: Option<&InstancePath>,
    ) -> Descent<'a> {
        let writes_its_own_text = |binding: &LibraryCellInstance| {
            binding.source_path.is_some()
                || binding.netlist_template.is_some()
                || binding.is_executable_builtin()
        };
        if let Some(path) = path {
            let Some(execution) = self.execution_binding(path) else {
                return Descent::Nothing;
            };
            let Some(materialized) = execution.materialized_binding() else {
                return Descent::Nothing;
            };
            // The plan stops at a compiled built-in because it is a leaf the
            // engine owns, not because a configuration stopped the walk.
            if materialized.is_executable_builtin() {
                return Descent::Itself;
            }
            if execution.stop_boundary() {
                return Descent::Nothing;
            }
            if writes_its_own_text(materialized) {
                return Descent::Itself;
            }
            let resolved = execution.resolved_reference();
            return self
                .master_view(&resolved.library, &resolved.cell, &resolved.view)
                .map_or(Descent::Itself, Descent::Into);
        }
        let Some(binding) = instance.library_cell.as_ref() else {
            return Descent::Itself;
        };
        if writes_its_own_text(binding) {
            return Descent::Itself;
        }
        self.schematic_master_for_binding(binding)
            .map_or(Descent::Itself, Descent::Into)
    }
}

/// What one cell instance contributes to the bill of materials.
enum Descent<'a> {
    /// The instance is a part in its own right.
    Itself,
    /// The plan builds nothing here.
    Nothing,
    /// The parts of this schematic are.
    Into(&'a SchematicDocument),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::LibraryCatalog;
    use crate::schematic::bom::{bom_csv, bom_lines};
    use rspice_design_model::Point;
    use std::collections::HashMap;

    fn part(id: u64, kind: ComponentType, name: &str, value: &str) -> Component {
        Component::new(id, kind, Point::new(0, 0)).with_name_value(name, value)
    }

    fn instance(id: u64, name: &str, cell: &str) -> Component {
        let mut binding = LibraryCellInstance::new("work", cell, "schematic");
        binding.bind_interface(&[]);
        Component::new(id, ComponentType::CellInstance, Point::new(0, 0))
            .with_library_cell(binding)
            .with_name_value(name, cell)
    }

    fn document(components: Vec<Component>) -> SchematicDocument {
        SchematicDocument {
            components,
            ..SchematicDocument::default()
        }
    }

    /// A filter cell used twice and nested once more: every occurrence's
    /// parts are counted under its hierarchical reference, the instances
    /// themselves are not, and a cell that reaches itself stops there.
    #[test]
    fn parts_in_child_cells_count_once_per_occurrence() {
        let mut views = HashMap::new();
        views.insert(
            "work/filter/schematic".to_owned(),
            document(vec![
                part(1, ComponentType::Resistor, "R1", "10k"),
                part(2, ComponentType::Capacitor, "C1", "1n"),
            ]),
        );
        views.insert(
            "work/stage/schematic".to_owned(),
            document(vec![
                instance(1, "X1", "filter"),
                part(2, ComponentType::Resistor, "R9", "10k"),
                instance(3, "X9", "stage"),
            ]),
        );
        let top = document(vec![
            part(1, ComponentType::Resistor, "R1", "10k"),
            part(2, ComponentType::VoltageSource, "V1", "5"),
            instance(3, "X1", "filter"),
            instance(4, "X2", "stage"),
        ]);
        let libraries = LibraryCatalog::default();
        let hierarchy = HierarchySource::from_workspace(&libraries, &views);

        let parts = hierarchy.bill_of_materials_parts(&top, &CellViewRef::default_top());
        let references = parts
            .iter()
            .map(|(reference, _)| reference.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            references,
            [
                "R1", "V1", "X1/R1", "X1/C1", "X2/X1/R1", "X2/X1/C1", "X2/R9"
            ]
        );

        let csv = bom_csv(&bom_lines(
            parts
                .iter()
                .map(|(reference, component)| (reference.as_str(), *component)),
        ));
        assert_eq!(
            csv.split("\r\n").skip(1).collect::<Vec<_>>(),
            [
                "\"R1, X1/R1, X2/R9, X2/X1/R1\",4,10k,,,,,,,",
                "\"X1/C1, X2/X1/C1\",2,1n,,,,,,,",
                "",
            ]
        );
    }
}
