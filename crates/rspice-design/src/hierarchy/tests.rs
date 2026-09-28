//! Headless hierarchy resolution using borrowed design documents.

use super::*;
use crate::schematic::component::Component;
use rspice_design_model::Point;

struct Documents(HashMap<String, (SchematicDocument, bool)>);

impl HierarchyDocuments for Documents {
    fn find_schematic(&self, reference: &CellViewRef) -> Option<HierarchySchematic<'_>> {
        self.0
            .get(&reference.key())
            .map(|(document, modified)| HierarchySchematic {
                document,
                modified: *modified,
            })
    }
}

struct NoSourceFiles;

impl HierarchySourceFiles for NoSourceFiles {
    fn source_paths_match(&self, _left: &Path, _right: &Path) -> bool {
        panic!("schematic-only hierarchy must not query host files")
    }
    fn configured_source_identity(&self, _path: &Path) -> String {
        panic!("schematic-only hierarchy must not query host files")
    }
    fn validate_source_file(
        &self,
        _path: &Path,
        _view: ViewType,
        _binding: &LibraryCellInstance,
    ) -> Result<(), String> {
        panic!("schematic-only hierarchy must not query host files")
    }
}

#[test]
fn borrowed_documents_resolve_shared_masters_without_editor_or_host_files() {
    let root = CellViewRef::new("work", "top", "schematic");
    let child = CellViewRef::new("work", "amp", "schematic");
    let mut library = Library::new("work");
    for name in ["top", "amp"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
    }
    let mut libraries = LibraryCatalog::default();
    libraries.add_library(library);
    let mut top = SchematicDocument::default();
    for (id, name) in [(1, "Xleft"), (2, "Xright")] {
        top.components.push(
            Component::new(id, ComponentType::CellInstance, Point::new(0, 0))
                .with_library_cell(LibraryCellInstance::new("work", "amp", "schematic"))
                .with_name_value(name, "amp"),
        );
    }
    let documents = Documents(HashMap::from([
        (root.key(), (top, false)),
        (child.key(), (SchematicDocument::default(), true)),
    ]));
    let sources = ProjectSourceRegistry::default();
    let context = HierarchyContext {
        documents: &documents,
        libraries: &libraries,
        project_id: ProjectId::new(),
        project_sources: &sources,
        source_files: &NoSourceFiles,
    };
    let (resolution, plan) = resolve_hierarchy(context, root.clone(), None);
    assert!(resolution.is_valid());
    assert_eq!(resolution.total_instances, 3);
    let child_row = resolution
        .bindings
        .iter()
        .find(|row| row.reference == child)
        .unwrap();
    assert_eq!(child_row.instance_count, 2);
    assert_eq!(child_row.status, HierarchyBindingStatus::Modified);
    assert_eq!(plan.bindings().len(), 3);
    assert_eq!(plan.masters().len(), 1);
    let left = InstancePath::root().child("Xleft").unwrap();
    let right = InstancePath::root().child("Xright").unwrap();
    assert!(plan.occurrence_master(&left).is_some());
    assert_eq!(
        plan.occurrence_master(&left),
        plan.occurrence_master(&right)
    );
    assert_eq!(documents.0[&root.key()].0.components[0].name, "Xleft");
    assert_eq!(documents.0[&child.key()].0.components.len(), 0);
}
