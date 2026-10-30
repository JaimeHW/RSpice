//! Headless hierarchy resolution using borrowed design documents.

use super::*;
use crate::configuration_set::{
    ConfigurationBlackBoxPolicy, ConfigurationModelProfile, ConfigurationSetCatalog,
    ConfigurationSetDefinition,
};
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

/// `work/top` places `X1`, `X2` and `X4` of `work/amp` and `X3` of `work/buf`.
/// Every view is a schematic with a document, so the view each row selects
/// is visible without a source file and every listed name can resolve.
fn bound_design() -> (Documents, LibraryCatalog) {
    let mut documents = HashMap::new();
    let mut library = Library::new("work");
    let cells: [(&str, &[&str]); 3] = [
        ("top", &["schematic"]),
        ("amp", &["schematic", "fast", "slow"]),
        ("buf", &["schematic", "fast"]),
    ];
    for (name, views) in cells {
        let mut cell = Cell::new(name);
        for &view in views {
            cell.add_view(View::new(view, ViewType::Schematic));
            documents.insert(
                CellViewRef::new("work", name, view).key(),
                (SchematicDocument::default(), false),
            );
        }
        library.add_cell(cell);
    }
    let mut top = SchematicDocument::default();
    for (id, instance, cell) in [
        (1, "X1", "amp"),
        (2, "X2", "amp"),
        (3, "X3", "buf"),
        (4, "X4", "amp"),
    ] {
        top.components.push(
            Component::new(id, ComponentType::CellInstance, Point::new(0, 0))
                .with_library_cell(LibraryCellInstance::new("work", cell, "schematic"))
                .with_name_value(instance, cell),
        );
    }
    documents.insert(
        CellViewRef::new("work", "top", "schematic").key(),
        (top, false),
    );
    let mut libraries = LibraryCatalog::default();
    libraries.add_library(library);
    (Documents(documents), libraries)
}

fn views(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_owned()).collect()
}

fn bound_configuration(global: &[&str], stops: &[&str]) -> ConfigurationSetDefinition {
    ConfigurationSetDefinition {
        name: "Bindings".to_owned(),
        root: CellViewRef::new("work", "top", "schematic"),
        dut_path: "/".to_owned(),
        executable_view_policy: views(global),
        stop_views: views(stops),
        unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
        black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
        overrides: Vec::new(),
        cell_bindings: Vec::new(),
        model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        owner: "Design".to_owned(),
    }
}

fn instance_binding(path: &str, list: &[&str], stop: Option<&str>) -> ConfigurationSetOverride {
    ConfigurationSetOverride {
        instance_path: path.to_owned(),
        executable_views: views(list),
        stop_view: stop.map(str::to_owned),
        model_section: None,
        eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
    }
}

fn cell_binding(cell: &str, list: &[&str], stop: Option<&str>) -> ConfigurationCellBinding {
    ConfigurationCellBinding {
        library: "work".to_owned(),
        cell: cell.to_owned(),
        executable_views: views(list),
        stop_view: stop.map(str::to_owned),
    }
}

fn resolve_bound(definition: Option<ConfigurationSetDefinition>) -> HierarchyResolution {
    let (documents, libraries) = bound_design();
    let mut catalog = ConfigurationSetCatalog::default();
    let configuration = definition.map(|definition| {
        catalog
            .create(definition)
            .expect("the configuration validates")
    });
    let sources = ProjectSourceRegistry::default();
    let context = HierarchyContext {
        documents: &documents,
        libraries: &libraries,
        project_id: ProjectId::new(),
        project_sources: &sources,
        source_files: &NoSourceFiles,
    };
    let configuration = configuration.and_then(|id| catalog.find(id));
    resolve_hierarchy(
        context,
        CellViewRef::new("work", "top", "schematic"),
        configuration,
    )
    .0
}

fn row<'r>(resolution: &'r HierarchyResolution, path: &str) -> &'r ResolvedHierarchyBinding {
    resolution
        .bindings
        .iter()
        .find(|row| row.instance_paths.iter().any(|candidate| candidate == path))
        .unwrap_or_else(|| panic!("a row for {path}: {:#?}", resolution.bindings))
}

/// The configuration the precedence tests share: `amp` is bound to `fast`,
/// `X2` and `X4` are bound by instance, and `buf` has only the global list.
fn layered_configuration() -> ConfigurationSetDefinition {
    let mut definition = bound_configuration(&["schematic"], &["schematic"]);
    definition.cell_bindings = vec![cell_binding("AMP", &["fast", "schematic"], Some("fast"))];
    definition.overrides = vec![
        instance_binding("/X2", &["slow"], Some("slow")),
        instance_binding("/X4", &["slow", "fast"], None),
    ];
    definition
}

#[test]
fn an_instance_binding_beats_a_cell_binding_which_beats_the_view_list() {
    let resolution = resolve_bound(Some(layered_configuration()));
    assert!(resolution.is_valid(), "{:#?}", resolution.bindings);

    // The cell binding governs the instance no override names, view and stop.
    let bound = row(&resolution, "/X1");
    assert_eq!(bound.reference, CellViewRef::new("work", "amp", "fast"));
    assert_eq!(bound.view_search_order, ["fast", "schematic"]);
    assert_eq!(bound.stop_view.as_deref(), Some("fast"));
    assert!(
        bound
            .warnings
            .iter()
            .any(|warning| warning.contains("'fast'")),
        "the cell binding's stop is the one matched against the selected view: {:?}",
        bound.warnings
    );

    // An instance binding beats it for both.
    let named = row(&resolution, "/X2");
    assert_eq!(named.reference, CellViewRef::new("work", "amp", "slow"));
    assert_eq!(named.view_search_order, ["slow"]);
    assert_eq!(named.stop_view.as_deref(), Some("slow"));

    // An instance binding without a stop view takes the cell binding's stop,
    // not the global one.
    let unstopped = row(&resolution, "/X4");
    assert_eq!(unstopped.reference, CellViewRef::new("work", "amp", "slow"));
    assert_eq!(unstopped.view_search_order, ["slow", "fast"]);
    assert_eq!(unstopped.stop_view.as_deref(), Some("fast"));

    // A cell without a binding keeps the global view list and stop views.
    let global = row(&resolution, "/X3");
    assert_eq!(
        global.reference,
        CellViewRef::new("work", "buf", "schematic")
    );
    assert_eq!(global.view_search_order, ["schematic"]);
    assert_eq!(global.stop_view.as_deref(), Some("schematic"));
}

#[test]
fn reviewed_fallback_follows_the_cell_binding_that_won() {
    let mut definition = bound_configuration(&["schematic"], &[]);
    definition.cell_bindings = vec![cell_binding("amp", &["absent"], None)];

    let blocked = resolve_bound(Some(definition.clone()));
    let unresolved = row(&blocked, "/X1");
    assert_eq!(unresolved.status, HierarchyBindingStatus::Unresolved);
    assert_eq!(unresolved.view_search_order, ["absent"]);
    assert_eq!(unresolved.instance_paths, ["/X1", "/X2", "/X4"]);
    assert_eq!(
        row(&blocked, "/X3").status,
        HierarchyBindingStatus::Resolved
    );

    definition.unresolved_policy = UnresolvedBindingPolicy::ExplicitFallbackWithReview;
    let reviewed = resolve_bound(Some(definition));
    let fallback = row(&reviewed, "/X1");
    assert_eq!(
        fallback.reference,
        CellViewRef::new("work", "amp", "schematic")
    );
    assert_eq!(
        fallback.view_search_order,
        ["absent", "schematic", "extracted", "spice"],
        "the default search order follows the cell binding's list"
    );
    assert!(fallback.used_review_fallback);
    assert_eq!(fallback.status, HierarchyBindingStatus::Modified);
    let listed = row(&reviewed, "/X3");
    assert!(
        !listed.used_review_fallback,
        "a view the global list names is not a fallback"
    );
    assert_eq!(listed.status, HierarchyBindingStatus::Resolved);
}

#[test]
fn each_row_names_the_rule_that_chose_its_view() {
    let unconfigured = resolve_bound(None);
    assert_eq!(unconfigured.bindings.len(), 3);
    for row in &unconfigured.bindings {
        assert_eq!(
            row.rule,
            HierarchyBindingRule::Default,
            "{:?}",
            row.instance_paths
        );
    }

    let layered = resolve_bound(Some(layered_configuration()));
    for (path, rule) in [
        ("/", HierarchyBindingRule::ViewList),
        ("/X1", HierarchyBindingRule::CellBinding),
        ("/X2", HierarchyBindingRule::InstanceBinding),
        ("/X3", HierarchyBindingRule::ViewList),
        ("/X4", HierarchyBindingRule::InstanceBinding),
    ] {
        assert_eq!(row(&layered, path).rule, rule, "{path}");
    }
}

#[test]
fn rows_a_different_rule_chose_are_never_merged() {
    // Every `amp` instance searches the one view `schematic` with no stop, so
    // only the rule that chose that list tells their rows apart.
    let mut definition = bound_configuration(&["schematic"], &[]);
    definition.cell_bindings = vec![cell_binding("amp", &["schematic"], None)];
    definition.overrides = vec![instance_binding("/X2", &["schematic"], None)];
    let resolution = resolve_bound(Some(definition));

    let cell_bound = row(&resolution, "/X1");
    let instance_bound = row(&resolution, "/X2");
    assert_eq!(cell_bound.reference, instance_bound.reference);
    assert_eq!(
        cell_bound.view_search_order,
        instance_bound.view_search_order
    );
    assert_eq!(cell_bound.stop_view, instance_bound.stop_view);
    assert_eq!(cell_bound.rule, HierarchyBindingRule::CellBinding);
    assert_eq!(cell_bound.instance_paths, ["/X1", "/X4"]);
    assert_eq!(instance_bound.rule, HierarchyBindingRule::InstanceBinding);
    assert_eq!(instance_bound.instance_paths, ["/X2"]);
}
