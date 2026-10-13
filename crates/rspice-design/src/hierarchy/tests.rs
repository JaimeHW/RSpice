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

/// Sources that are all project keys: an identity is the key itself, and a
/// host file is never read.
struct ProjectKeysOnly;

impl HierarchySourceFiles for ProjectKeysOnly {
    fn source_paths_match(&self, _left: &Path, _right: &Path) -> bool {
        panic!("a project-owned source must not be compared as a host file")
    }
    fn configured_source_identity(&self, path: &Path) -> String {
        path.display().to_string()
    }
    fn validate_source_file(
        &self,
        _path: &Path,
        _view: ViewType,
        _binding: &LibraryCellInstance,
    ) -> Result<(), String> {
        panic!("a project-owned source must not be read as a host file")
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

/// A schematic master that declares a parameter contract binds it as the
/// instance's allowlist, as a source view's does. Materialization used to
/// give the instance a fresh binding with none, so every override passed
/// through unchecked with a warning that the master declared no contract.
#[test]
fn a_schematic_masters_parameter_contract_is_its_instances_allowlist() {
    let root = CellViewRef::new("work", "top", "schematic");
    let child = CellViewRef::new("work", "amp", "schematic");
    let mut library = Library::new("work");
    for name in ["top", "amp"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        if name == "amp" {
            cell.metadata.insert(
                "cdf.parameter_contract".to_owned(),
                r#"[{"name":"gain","default":"2"},{"name":"bw"}]"#.to_owned(),
            );
        }
        library.add_cell(cell);
    }
    let mut libraries = LibraryCatalog::default();
    libraries.add_library(library);
    let mut top = SchematicDocument::default();
    top.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(0, 0))
            .with_library_cell(LibraryCellInstance::new("work", "amp", "schematic"))
            .with_name_value("X1", "amp"),
    );
    let documents = Documents(HashMap::from([
        (root.key(), (top, false)),
        (child.key(), (SchematicDocument::default(), false)),
    ]));
    let sources = ProjectSourceRegistry::default();
    let context = HierarchyContext {
        documents: &documents,
        libraries: &libraries,
        project_id: ProjectId::new(),
        project_sources: &sources,
        source_files: &NoSourceFiles,
    };

    let (_, plan) = resolve_hierarchy(context, root, None);

    let materialized = plan
        .binding(&InstancePath::root().child("X1").unwrap())
        .and_then(ConfigurationExecutionBinding::materialized_binding)
        .expect("the instance binds its master");
    assert!(materialized.parameters_declared());
    assert_eq!(materialized.parameter_order, ["gain", "bw"]);
}

/// With no configuration, an instance of a project-owned Verilog-A cell runs
/// the bundle its view owns, exactly as a configuration would bind it: Spectre
/// reaches `veriloga` with no configuration at all. The unconfigured path used
/// to judge the placed binding as a host file, refused its project key as
/// "not an absolute source identity", and no instance of a module the reader
/// had just written could run without a hand-made configuration.
#[test]
fn an_unconfigured_veriloga_instance_runs_the_bundle_its_view_owns() {
    use crate::project_sources::{ProjectSourceBundle, ProjectSourceLanguage};
    use rspice_design_model::port::{PortDirection, PortSpec};

    let root = CellViewRef::new("work", "top", "schematic");
    let module = CellViewRef::new("work", "gain", "veriloga");
    let mut library = Library::new("work");
    let mut top = Cell::new("top");
    top.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(top);
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut gain = Cell::new("gain");
    gain.add_view(view);
    library.add_cell(gain);
    let mut libraries = LibraryCatalog::default();
    libraries.add_library(library);
    let mut sources = ProjectSourceRegistry::default();
    sources
        .insert_bundle(
            ProjectSourceBundle::try_new(
                ProjectSourceOwner::cell_view(module.clone()),
                ProjectSourceLanguage::VerilogA,
                "gain.va",
                "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ V(p, n) / 1k;\nendmodule\n",
                [],
                [],
            )
            .expect("a cell-view bundle"),
        )
        .expect("one bundle per cell view");
    let project_id = ProjectId::new();
    let owned = project_veriloga_binding_for_view(project_id, &sources, &libraries, &module)
        .expect("the view owns its bundle");

    // The instance as the library shelf places it: the project key, the
    // module's own name and the interface the view states.
    let mut placed = LibraryCellInstance::new("work", "gain", "veriloga");
    placed.source_path = Some(PathBuf::from(owned.source_key()));
    placed.module_name = Some("gain".to_owned());
    placed.bind_interface(&[
        PortSpec {
            name: "p".to_owned(),
            direction: PortDirection::InOut,
        },
        PortSpec {
            name: "n".to_owned(),
            direction: PortDirection::InOut,
        },
    ]);
    let mut document = SchematicDocument::default();
    document.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(0, 0))
            .with_library_cell(placed)
            .with_name_value("X1", "gain"),
    );
    let documents = Documents(HashMap::from([(root.key(), (document, false))]));
    let context = HierarchyContext {
        documents: &documents,
        libraries: &libraries,
        project_id,
        project_sources: &sources,
        source_files: &ProjectKeysOnly,
    };

    let (resolution, plan) = resolve_hierarchy(context, root, None);

    let row = resolution
        .bindings
        .iter()
        .find(|row| row.reference == module)
        .expect("the instance's row");
    assert_eq!(
        row.status,
        HierarchyBindingStatus::Resolved,
        "{:?}",
        row.diagnostic
    );
    let execution = plan
        .binding(&InstancePath::root().child("X1").unwrap())
        .expect("the instance is planned");
    assert_eq!(execution.project_veriloga(), Some(&owned));
    let materialized = execution
        .materialized_binding()
        .expect("bound from the view");
    assert_eq!(
        materialized.source_path.as_deref(),
        Some(Path::new(owned.source_key()))
    );
    assert_eq!(
        materialized.module_name.as_deref(),
        Some(owned.netlist_alias())
    );
    assert_eq!(materialized.terminal_order, ["p", "n"]);
}
