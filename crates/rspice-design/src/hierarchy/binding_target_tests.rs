//! A configuration binding that runs another cell in a placement's place:
//! which cell runs, what it is measured against, and what a cell that stops
//! fitting does to the hierarchy.

use super::*;
use crate::configuration_set::{
    ConfigurationBlackBoxPolicy, ConfigurationModelProfile, ConfigurationSetCatalog,
    ConfigurationSetDefinition, ConfigurationSetError,
};
use crate::schematic::component::Component;
use crate::schematic::port::PortDirection;
use rspice_design_model::Point;

struct Documents(HashMap<String, SchematicDocument>);

impl HierarchyDocuments for Documents {
    fn find_schematic(&self, reference: &CellViewRef) -> Option<HierarchySchematic<'_>> {
        self.0
            .get(&reference.key())
            .map(|document| HierarchySchematic {
                document,
                modified: false,
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

/// A master's port, named and directed as its schematic declares it.
fn port(id: u64, name: &str, direction: &str) -> Component {
    let mut port = Component::new(id, ComponentType::Port, Point::new(0, id as i32 * 40))
        .with_name_value("", name);
    port.params = format!("dir={direction}");
    port
}

fn master(ports: &[(&str, &str)]) -> SchematicDocument {
    let mut document = SchematicDocument::default();
    for (index, (name, direction)) in ports.iter().enumerate() {
        document
            .components
            .push(port(index as u64 + 1, name, direction));
    }
    document
}

/// A master cell: its library, its name, and its pins with their directions.
type Master<'a> = (&'a str, &'a str, &'a [(&'a str, &'a str)]);

/// `work/top` places `X1` and `X2` of `work/amp`, an input `IN` and an
/// output `OUT`. Beside it: `vendor/amp` with the same pins, `work/amp_b`
/// with them in the other order, `work/amp_trim` with a `TRIM` input of
/// its own, and `work/amp_half` with no `OUT`.
fn design() -> (Documents, LibraryCatalog) {
    let masters: [Master; 5] = [
        ("work", "amp", &[("IN", "in"), ("OUT", "out")]),
        ("vendor", "amp", &[("IN", "in"), ("OUT", "out")]),
        ("work", "amp_b", &[("OUT", "out"), ("IN", "in")]),
        (
            "work",
            "amp_trim",
            &[("IN", "in"), ("OUT", "out"), ("TRIM", "in")],
        ),
        ("work", "amp_half", &[("IN", "in")]),
    ];
    let mut documents = HashMap::new();
    let mut work = Library::new("work");
    let mut vendor = Library::new("vendor");
    for (library, cell, ports) in masters {
        let mut entry = Cell::new(cell);
        entry.add_view(View::new("schematic", ViewType::Schematic));
        if library == "work" {
            work.add_cell(entry);
        } else {
            vendor.add_cell(entry);
        }
        documents.insert(
            CellViewRef::new(library, cell, "schematic").key(),
            master(ports),
        );
    }
    let mut top_cell = Cell::new("top");
    top_cell.add_view(View::new("schematic", ViewType::Schematic));
    work.add_cell(top_cell);
    let mut top = SchematicDocument::default();
    for (id, name) in [(1, "X1"), (2, "X2")] {
        let mut placed = LibraryCellInstance::new("work", "amp", "schematic");
        placed.bind_interface(&[
            PortSpec {
                name: "IN".to_owned(),
                direction: PortDirection::In,
            },
            PortSpec {
                name: "OUT".to_owned(),
                direction: PortDirection::Out,
            },
        ]);
        top.components.push(
            Component::new(id, ComponentType::CellInstance, Point::new(0, 0))
                .with_library_cell(placed)
                .with_name_value(name, "amp"),
        );
    }
    documents.insert(CellViewRef::new("work", "top", "schematic").key(), top);
    let mut libraries = LibraryCatalog::default();
    libraries.add_library(work);
    libraries.add_library(vendor);
    (Documents(documents), libraries)
}

fn configuration() -> ConfigurationSetDefinition {
    ConfigurationSetDefinition {
        name: "Swap".to_owned(),
        root: CellViewRef::new("work", "top", "schematic"),
        dut_path: "/".to_owned(),
        executable_view_policy: vec!["schematic".to_owned()],
        stop_views: Vec::new(),
        unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
        black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
        overrides: Vec::new(),
        cell_bindings: Vec::new(),
        model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        owner: "Design".to_owned(),
    }
}

fn instance_target(path: &str, library: &str, cell: &str) -> ConfigurationSetOverride {
    ConfigurationSetOverride {
        instance_path: path.to_owned(),
        executable_views: Vec::new(),
        stop_view: None,
        model_section: None,
        eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
        target: Some(BindingTarget::new(library, cell)),
    }
}

fn cell_target(cell: &str, library: &str, target: &str) -> ConfigurationCellBinding {
    ConfigurationCellBinding {
        library: "work".to_owned(),
        cell: cell.to_owned(),
        executable_views: Vec::new(),
        stop_view: None,
        target: Some(BindingTarget::new(library, target)),
    }
}

fn resolve(
    definition: ConfigurationSetDefinition,
) -> (HierarchyResolution, ConfigurationExecutionPlan) {
    let (documents, libraries) = design();
    let mut catalog = ConfigurationSetCatalog::default();
    let id = catalog
        .create(definition)
        .expect("the configuration validates");
    let sources = ProjectSourceRegistry::default();
    resolve_hierarchy(
        HierarchyContext {
            documents: &documents,
            libraries: &libraries,
            project_id: ProjectId::new(),
            project_sources: &sources,
            source_files: &NoSourceFiles,
        },
        CellViewRef::new("work", "top", "schematic"),
        catalog.find(id),
    )
}

fn row<'r>(resolution: &'r HierarchyResolution, path: &str) -> &'r ResolvedHierarchyBinding {
    resolution
        .bindings
        .iter()
        .find(|row| row.instance_paths.iter().any(|candidate| candidate == path))
        .unwrap_or_else(|| panic!("a row for {path}: {:#?}", resolution.bindings))
}

fn bound_cell(plan: &ConfigurationExecutionPlan, path: &str) -> Option<(String, String)> {
    let path = InstancePath::parse(path).expect("a path");
    plan.binding(&path)
        .and_then(ConfigurationExecutionBinding::materialized_binding)
        .map(|binding| (binding.library.clone(), binding.cell.clone()))
}

/// A cell binding naming another library's cell runs that cell for every
/// instance of the placed one: the row names both, the plan netlists the
/// target, and the target's pins are what the instance binds.
#[test]
fn a_cell_binding_runs_another_librarys_cell_for_every_instance() {
    let mut definition = configuration();
    definition.cell_bindings = vec![cell_target("amp", "vendor", "amp")];
    let (resolution, plan) = resolve(definition);
    assert!(resolution.is_valid(), "{:#?}", resolution.bindings);

    let bound = row(&resolution, "/X1");
    assert_eq!(
        bound.reference,
        CellViewRef::new("vendor", "amp", "schematic")
    );
    assert_eq!(
        bound.placed,
        Some(CellViewRef::new("work", "amp", "schematic"))
    );
    assert_eq!(bound.rule, HierarchyBindingRule::CellBinding);
    assert_eq!(bound.instance_paths, ["/X1", "/X2"]);
    for path in ["/X1", "/X2"] {
        assert_eq!(
            bound_cell(&plan, path),
            Some(("vendor".to_owned(), "amp".to_owned())),
            "{path}"
        );
    }
}

/// An instance's own target beats its cell's, and a cell whose pins are the
/// placed ones in another order fits: pins meet by name.
#[test]
fn an_instance_target_beats_the_cell_target_and_pins_meet_by_name() {
    let mut definition = configuration();
    definition.cell_bindings = vec![cell_target("amp", "vendor", "amp")];
    definition.overrides = vec![instance_target("/X2", "work", "amp_b")];
    let (resolution, plan) = resolve(definition);
    assert!(resolution.is_valid(), "{:#?}", resolution.bindings);

    assert_eq!(
        row(&resolution, "/X2").reference,
        CellViewRef::new("work", "amp_b", "schematic")
    );
    assert_eq!(
        bound_cell(&plan, "/X2"),
        Some(("work".to_owned(), "amp_b".to_owned()))
    );
    assert_eq!(
        bound_cell(&plan, "/X1"),
        Some(("vendor".to_owned(), "amp".to_owned()))
    );
    let materialized = plan
        .binding(&InstancePath::parse("/X2").unwrap())
        .and_then(ConfigurationExecutionBinding::materialized_binding)
        .unwrap();
    assert_eq!(
        materialized.terminal_order,
        ["OUT", "IN"],
        "the instance binds the target's own pin order"
    );
}

/// A target that lost a pin the instance connects binds nothing: the row is
/// a problem naming the pin, the plan holds no binding to netlist, and the
/// hierarchy is not valid to run.
#[test]
fn a_target_that_no_longer_fits_binds_nothing_and_names_the_pin() {
    let mut definition = configuration();
    definition.overrides = vec![instance_target("/X1", "work", "amp_half")];
    let (resolution, plan) = resolve(definition);
    assert!(!resolution.is_valid());

    let refused = row(&resolution, "/X1");
    assert_eq!(refused.status, HierarchyBindingStatus::Incompatible);
    assert_eq!(
        refused.reference,
        CellViewRef::new("work", "amp_half", "schematic")
    );
    let diagnostic = refused.diagnostic.as_deref().unwrap_or_default();
    assert!(
        diagnostic.contains("work/amp_half no longer fits /X1: it has no pin OUT"),
        "{diagnostic}"
    );
    assert_eq!(bound_cell(&plan, "/X1"), None);
    assert_eq!(
        bound_cell(&plan, "/X2"),
        Some(("work".to_owned(), "amp".to_owned())),
        "an instance no binding names keeps its cell"
    );
}

/// A target with a pin of its own the instance does not connect fits, and
/// the row says that pin is left unconnected.
#[test]
fn a_target_pin_the_instance_does_not_place_is_left_unconnected_and_said() {
    let mut definition = configuration();
    definition.overrides = vec![instance_target("/X1", "work", "amp_trim")];
    let (resolution, plan) = resolve(definition);
    assert!(resolution.is_valid(), "{:#?}", resolution.bindings);
    let bound = row(&resolution, "/X1");
    assert_eq!(bound.status, HierarchyBindingStatus::Resolved);
    assert_eq!(
        bound.warnings,
        ["work/amp_trim pin TRIM is left unconnected: the instance does not place it"]
    );
    assert_eq!(
        bound_cell(&plan, "/X1"),
        Some(("work".to_owned(), "amp_trim".to_owned()))
    );
}

/// A pattern names instances of many cells, so it holds no target; a cell
/// binding that names a target needs no view list of its own.
#[test]
fn only_an_instance_or_a_cell_binding_holds_a_target() {
    let mut definition = configuration();
    definition.overrides = vec![instance_target("/*", "vendor", "amp")];
    assert!(matches!(
        ConfigurationSetCatalog::default().create(definition),
        Err(ConfigurationSetError::PatternBindingTarget(pattern)) if pattern == "/*"
    ));

    let mut definition = configuration();
    definition.cell_bindings = vec![cell_target("amp", "vendor", "amp")];
    assert!(
        ConfigurationSetCatalog::default()
            .create(definition)
            .is_ok()
    );
}

/// Renaming or moving the cell a binding runs takes the binding with it.
#[test]
fn a_target_follows_its_cell_when_it_is_renamed_or_moved() {
    let mut definition = configuration();
    definition.cell_bindings = vec![cell_target("amp", "vendor", "amp")];
    definition.overrides = vec![instance_target("/X2", "work", "amp_b")];
    let mut catalog = ConfigurationSetCatalog::default();
    let id = catalog.create(definition).unwrap();

    catalog
        .rename_cell_roots("vendor", "amp", "amp_v2")
        .unwrap();
    catalog
        .move_cell_roots(("work", "amp_b"), ("vendor", "amp_b"))
        .unwrap();
    let definition = catalog.find(id).unwrap().definition();
    assert_eq!(
        definition.cell_bindings[0].target,
        Some(BindingTarget::new("vendor", "amp_v2"))
    );
    assert_eq!(
        definition.overrides[0].target,
        Some(BindingTarget::new("vendor", "amp_b"))
    );
}
