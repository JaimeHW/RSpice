//! Independent source classification, and what the hierarchy walk checks.

use super::*;
use crate::configuration_set::{
    ConfigurationBlackBoxPolicy, ConfigurationModelProfile, ConfigurationSetCatalog,
    ConfigurationSetDefinition, UnresolvedBindingPolicy,
};
use crate::connectivity::terminal_positions_with_hierarchy;
use crate::hierarchy::{
    ConfigurationExecutionPlan, HierarchyContext, HierarchyDocuments, HierarchySchematic,
    HierarchySourceFiles, resolve_hierarchy,
};
use crate::library::{Cell, Library, LibraryCatalog, View, ViewType};
use crate::project_sources::{
    ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry,
};
use crate::schematic::component::LibraryCellInstance;
use crate::schematic::net_label::NetLabel;
use crate::schematic::wire::Wire;
use rspice_app_types::product::ProjectId;
use rspice_design_model::Point;
use std::collections::HashMap;
use std::path::Path;

/// A cell instance bound to one project cellview that declares no ports, so
/// the instance itself has no terminal to leave open and every finding below
/// it is the master's.
fn instance(id: u64, name: &str, cell: &str) -> Component {
    let mut binding = LibraryCellInstance::new("work", cell, "schematic");
    binding.bind_interface(&[]);
    Component::new(id, ComponentType::CellInstance, Point::new(0, 0))
        .with_library_cell(binding)
        .with_name_value(name, cell)
}

/// A primitive whose two legs the drawing leaves open.
fn open_resistor(id: u64, name: &str) -> Component {
    Component::new(id, ComponentType::Resistor, Point::new(0, 0)).with_name_value(name, "1k")
}

/// A resistor wired to a ground symbol: node 0 bound by something other than
/// the symbol's own terminal, which is what the ground rule asks for.
fn grounded_resistor() -> SchematicDocument {
    let resistor = open_resistor(1, "R1");
    let ground = Component::new(2, ComponentType::Ground, Point::new(0, 200));
    let wire = Wire::segment(
        10,
        terminal_positions_with_hierarchy(&resistor, None)[1].1,
        terminal_positions_with_hierarchy(&ground, None)[0].1,
    );
    SchematicDocument {
        components: vec![resistor, ground],
        wires: vec![wire],
        ..SchematicDocument::default()
    }
}

fn buffers(
    views: impl IntoIterator<Item = (&'static str, SchematicDocument)>,
) -> HashMap<String, SchematicDocument> {
    views
        .into_iter()
        .map(|(cell, document)| (format!("work/{cell}/schematic"), document))
        .collect()
}

fn connectivity_only() -> DrcConfig {
    DrcConfig {
        check_missing_ground: false,
        check_floating_nodes: false,
        ..DrcConfig::default()
    }
}

fn of_type(result: &DrcResult, violation_type: DrcViolationType) -> Vec<&DrcViolation> {
    result
        .violations()
        .iter()
        .filter(|violation| violation.violation_type == violation_type)
        .collect()
}

/// Where each finding sits: the occurrence path and cellview for one below the
/// subject, the root path and no cellview for the subject's own.
fn placed(result: &DrcResult, violation_type: DrcViolationType) -> Vec<(String, Option<String>)> {
    let mut placed = of_type(result, violation_type)
        .into_iter()
        .map(|finding| match &finding.location {
            DrcLocation::InsideInstance { path, view, .. } => (path.to_string(), Some(view.key())),
            _ => ("/".to_owned(), None),
        })
        .collect::<Vec<_>>();
    placed.sort();
    placed
}

/// The walk checks the cellviews a design instantiates, and says which
/// occurrence each finding belongs to. Before it did, a hierarchical design
/// passed with every defect below the subject invisible.
#[test]
fn a_sub_cellviews_findings_are_stated_at_every_occurrence() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "X1", "amp"));
    top.components.push(instance(2, "X2", "amp"));
    let mut amp = SchematicDocument::default();
    amp.components.push(open_resistor(1, "R1"));
    let views = buffers([("amp", amp)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let open = of_type(&result, DrcViolationType::UnconnectedPin);
    assert_eq!(open.len(), 4, "two legs at each of two occurrences");
    let mut paths = open
        .iter()
        .map(|finding| match &finding.location {
            DrcLocation::InsideInstance { path, view, inner } => {
                assert_eq!(view.key(), "work/amp/schematic");
                assert!(
                    matches!(inner.as_ref(), DrcLocation::Terminal { .. }),
                    "{inner:?}"
                );
                path.to_string()
            }
            other => panic!("a sub-cell finding carries its occurrence path: {other:?}"),
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    assert_eq!(paths, vec!["/X1".to_owned(), "/X2".to_owned()]);
    // The report names the terminal inside the occurrence, which is the whole
    // point of carrying the path.
    assert!(
        open[0]
            .location
            .display()
            .starts_with("/X1 \u{00b7} Pin R1."),
        "{}",
        open[0].location.display()
    );
}

/// A cellview placed more times than the report lists is stated once, with the
/// exact count. Repeating it would bury every other finding, and dropping the
/// occurrences silently would understate the design.
#[test]
fn a_flood_of_occurrences_is_counted_rather_than_repeated() {
    let mut top = SchematicDocument::default();
    for id in 1..=12u64 {
        top.components.push(instance(id, &format!("X{id}"), "amp"));
    }
    let mut amp = SchematicDocument::default();
    amp.components.push(open_resistor(1, "R1"));
    let views = buffers([("amp", amp)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let open = of_type(&result, DrcViolationType::UnconnectedPin);
    assert_eq!(
        open.len(),
        2,
        "one finding per open leg, not per occurrence"
    );
    assert!(
        open.iter().all(|finding| finding
            .message
            .contains("(12 occurrences; reported once, at /X1)")),
        "{:?}",
        open.iter()
            .map(|finding| &finding.message)
            .collect::<Vec<_>>()
    );
}

/// The count multiplies down the levels: three `mid` placing four `leaf` each
/// is twelve leaves. Counting one per edge instead would say four, and would
/// list eight places where twelve exist.
#[test]
fn an_occurrence_count_multiplies_down_the_levels() {
    let mut top = SchematicDocument::default();
    for id in 1..=3u64 {
        top.components.push(instance(id, &format!("X{id}"), "mid"));
    }
    let mut mid = SchematicDocument::default();
    for id in 1..=4u64 {
        mid.components.push(instance(id, &format!("Y{id}"), "leaf"));
    }
    let mut leaf = SchematicDocument::default();
    leaf.components.push(open_resistor(1, "R1"));
    let views = buffers([("mid", mid), ("leaf", leaf)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let open = of_type(&result, DrcViolationType::UnconnectedPin);
    assert_eq!(open.len(), 2, "{:?}", result.violations());
    for finding in open {
        assert!(
            finding
                .message
                .contains("(12 occurrences; reported once, at /X1/Y1)"),
            "{}",
            finding.message
        );
    }
}

/// Sixty-four levels that each place the next twice reach the last level 2^64
/// times, one more than a `u64` holds. The count stops at its ceiling and the
/// report says the true count is at least that, rather than wrapping or
/// quietly stating a ceiling as an exact number.
#[test]
fn an_occurrence_count_past_its_ceiling_is_stated_as_a_floor() {
    let mut views = HashMap::new();
    for level in 0..64u32 {
        let next = format!("c{}", level + 1);
        let mut cell = SchematicDocument::default();
        cell.components.push(instance(1, "XA", &next));
        cell.components.push(instance(2, "XB", &next));
        views.insert(format!("work/c{level}/schematic"), cell);
    }
    let mut last = SchematicDocument::default();
    last.components.push(open_resistor(1, "R1"));
    views.insert("work/c64/schematic".to_owned(), last);
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "X0", "c0"));
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let open = of_type(&result, DrcViolationType::UnconnectedPin);
    assert_eq!(open.len(), 2, "{:?}", result.violations());
    for finding in open {
        assert!(
            finding.message.contains(&format!(
                "(at least {} occurrences; reported once, at /X0/XA",
                u64::MAX
            )),
            "{}",
            finding.message
        );
    }
}

/// An occurrence whose path runs past the hierarchy's length limit has no
/// path to be stated at. It is still counted, and the report says how many
/// occurrences it could not list.
#[test]
fn an_occurrence_without_a_path_is_counted_rather_than_dropped() {
    let long = "M".repeat(1022);
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "M", "mid"));
    top.components.push(instance(2, &long, "mid"));
    let mut mid = SchematicDocument::default();
    mid.components.push(instance(1, "L", "leaf"));
    let mut leaf = SchematicDocument::default();
    leaf.components.push(open_resistor(1, "R1"));
    let views = buffers([("mid", mid), ("leaf", leaf)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let open = of_type(&result, DrcViolationType::UnconnectedPin);
    assert_eq!(open.len(), 2, "{:?}", result.violations());
    for finding in open {
        assert!(
            matches!(&finding.location, DrcLocation::InsideInstance { path, .. } if path.to_string() == "/M/L"),
            "{:?}",
            finding.location
        );
        assert!(
            finding
                .message
                .ends_with("(2 occurrences; 1 has no instance path within the hierarchy limits)"),
            "{}",
            finding.message
        );
    }
}

/// A cellview reached through two parents is checked once and stated at both
/// occurrences.
#[test]
fn a_diamond_is_stated_at_both_of_its_paths() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "XA", "a"));
    top.components.push(instance(2, "XB", "b"));
    let mut a = SchematicDocument::default();
    a.components.push(instance(1, "XL", "leaf"));
    let mut b = SchematicDocument::default();
    b.components.push(instance(1, "XL", "leaf"));
    let mut leaf = SchematicDocument::default();
    leaf.components.push(open_resistor(1, "R1"));
    let views = buffers([("a", a), ("b", b), ("leaf", leaf)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let leaf_key = Some("work/leaf/schematic".to_owned());
    assert_eq!(
        placed(&result, DrcViolationType::UnconnectedPin),
        vec![
            ("/XA/XL".to_owned(), leaf_key.clone()),
            ("/XA/XL".to_owned(), leaf_key.clone()),
            ("/XB/XL".to_owned(), leaf_key.clone()),
            ("/XB/XL".to_owned(), leaf_key),
        ]
    );
}

/// A cellview that reaches itself has no finite expansion. The walk states it
/// and stops rather than following the cycle.
#[test]
fn a_cycle_is_reported_rather_than_followed() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "XA", "a"));
    let mut a = SchematicDocument::default();
    a.components.push(instance(1, "XB", "b"));
    let mut b = SchematicDocument::default();
    b.components.push(instance(1, "XA", "a"));
    let views = buffers([("a", a), ("b", b)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let recursive = of_type(&result, DrcViolationType::RecursiveInstantiation);
    assert_eq!(recursive.len(), 1, "{:?}", result.violations());
    assert_eq!(recursive[0].severity, DrcSeverity::Critical);
    assert!(
        recursive[0]
            .message
            .contains("work/a/schematic -> work/b/schematic -> work/a/schematic"),
        "{}",
        recursive[0].message
    );
    assert!(matches!(
        &recursive[0].location,
        DrcLocation::InsideInstance { path, .. } if path.to_string() == "/XA/XB"
    ));
    // The description holds for a cycle through another cellview, which is
    // not a cellview instantiating itself.
    assert_eq!(
        DrcViolationType::RecursiveInstantiation.description(),
        "Cellview reaches itself through its instances"
    );
}

/// A cellview that instantiates itself directly terminates too.
#[test]
fn a_self_referential_cellview_terminates() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "XA", "a"));
    let mut a = SchematicDocument::default();
    a.components.push(instance(1, "XA", "a"));
    let views = buffers([("a", a)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    assert_eq!(
        of_type(&result, DrcViolationType::RecursiveInstantiation).len(),
        1,
        "{:?}",
        result.violations()
    );
}

/// A subject that places its own cellview closes the cycle at itself: the
/// instance is the finding, on the subject, and nothing of the subject is
/// stated a second time under it.
#[test]
fn a_subject_that_instantiates_itself_closes_the_cycle_at_the_subject() {
    let mut top = SchematicDocument::default();
    top.components.push(open_resistor(1, "R1"));
    top.components.push(instance(2, "X1", "top"));
    let views = buffers([("top", top)]);
    let hierarchy = HierarchySource::from_buffers(&views);
    let subject = &views["work/top/schematic"];

    let result = run_check_with_hierarchy(subject, &hierarchy, connectivity_only());

    let recursive = of_type(&result, DrcViolationType::RecursiveInstantiation);
    assert_eq!(recursive.len(), 1, "{:?}", result.violations());
    assert_eq!(
        recursive[0].location,
        DrcLocation::Component {
            id: 2,
            name: "X1".to_owned()
        }
    );
    assert_eq!(
        placed(&result, DrcViolationType::UnconnectedPin),
        vec![("/".to_owned(), None), ("/".to_owned(), None)],
        "the subject's two open legs, once"
    );
}

/// A cycle through the subject, A -> B -> A with A checked: the cycle is
/// stated once, where it closes, and A's own findings are not repeated under
/// the occurrence that closes it. A check of either cellview alone states the
/// cycle on its own instance in it, since the cycle closes in the other one.
#[test]
fn a_cycle_through_the_subject_is_stated_once() {
    let mut a = SchematicDocument::default();
    a.components.push(open_resistor(1, "R1"));
    a.components.push(instance(2, "XB", "b"));
    let mut b = SchematicDocument::default();
    b.components.push(instance(1, "XA", "a"));
    let views = buffers([("a", a), ("b", b)]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result =
        run_check_with_hierarchy(&views["work/a/schematic"], &hierarchy, connectivity_only());

    assert_eq!(
        placed(&result, DrcViolationType::RecursiveInstantiation),
        vec![("/XB".to_owned(), Some("work/b/schematic".to_owned()))]
    );
    assert_eq!(
        placed(&result, DrcViolationType::UnconnectedPin),
        vec![("/".to_owned(), None), ("/".to_owned(), None)]
    );

    for (cell, own_instance) in [("a", "XB"), ("b", "XA")] {
        let reference = CellViewRef::new("work", cell, "schematic");
        let alone = run_check_of_cellview(
            &views[&reference.key()],
            &reference,
            &hierarchy,
            connectivity_only(),
            CheckReach::Cellview,
        );
        let recursive = of_type(&alone, DrcViolationType::RecursiveInstantiation);
        assert_eq!(recursive.len(), 1, "{cell}: {:?}", alone.violations());
        assert!(
            matches!(&recursive[0].location, DrcLocation::Component { name, .. } if name == own_instance),
            "{cell}: {:?}",
            recursive[0].location
        );
        assert!(
            recursive[0].message.contains("again through its instances"),
            "{}",
            recursive[0].message
        );
        assert!(
            alone
                .violations()
                .iter()
                .all(|finding| !matches!(finding.location, DrcLocation::InsideInstance { .. })),
            "{cell}: a cellview check restates nothing from below: {:?}",
            alone.violations()
        );
    }
}

/// The reference node is global, so a ground symbol inside a sub-cell grounds
/// the design above it. Asking the rule per cellview instead would fail the
/// top-level design of every correct hierarchy whose ground lives one level
/// down, and would fail every reusable cell that takes its reference from the
/// parent.
///
/// That holds for a port-less sub-cell too: its ground symbol is node 0 of the
/// flattened deck, the same node the top would name. A top circuit with no DC
/// path to that node is the no-DC-path rule's to refuse, not this one's.
#[test]
fn the_ground_rule_is_asked_once_over_the_whole_hierarchy() {
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "X1", "bias"));
    let views = buffers([("bias", grounded_resistor())]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let grounded_below = run_check_with_hierarchy(&top, &hierarchy, DrcConfig::default());
    assert!(
        of_type(&grounded_below, DrcViolationType::MissingGround).is_empty(),
        "{:?}",
        grounded_below.violations()
    );

    let ungrounded = buffers([(
        "bias",
        SchematicDocument {
            components: vec![open_resistor(1, "R1")],
            ..SchematicDocument::default()
        },
    )]);
    let hierarchy = HierarchySource::from_buffers(&ungrounded);
    let result = run_check_with_hierarchy(&top, &hierarchy, DrcConfig::default());
    let missing = of_type(&result, DrcViolationType::MissingGround);
    assert_eq!(
        missing.len(),
        1,
        "one design, one verdict: {:?}",
        result.violations()
    );
    assert_eq!(missing[0].location, DrcLocation::Global);
}

/// Every independent source is claimed by exactly one side of the pair, and
/// nothing else is claimed at all.
///
/// `VoltageSourcePwlFile` was missing from [`is_voltage_source`] while
/// `CurrentSourcePwlFile` was present in [`is_current_source`], so a voltage
/// source driven from a PWL file was not a driver as far as any rule that asks
/// whether a net has one — an asymmetry, not a decision. The oracle is the
/// deck: an independent source is a placed source whose SPICE card starts with
/// `V` or `I`, which is exactly what the emitter writes, so a family added
/// tomorrow is classified by this test before anyone has to remember the pair.
#[test]
fn every_independent_source_is_claimed_by_exactly_one_side() {
    for kind in ComponentType::ALL {
        let voltage = is_voltage_source(kind);
        let current = is_current_source(kind);
        // `LoopProbe` writes a `V` card and is not a source: it is a 0 V
        // probe, and the rules must not read it as a driver.
        let independent = kind.is_source().then(|| kind.spice_prefix());
        match independent {
            Some("V") => assert!(
                voltage && !current,
                "{kind:?} writes a V card and is a source; only the voltage \
                 side may claim it (voltage {voltage}, current {current})"
            ),
            Some("I") => assert!(
                current && !voltage,
                "{kind:?} writes an I card and is a source; only the current \
                 side may claim it (voltage {voltage}, current {current})"
            ),
            // A dependent or behavioural source drives through an expression
            // rather than by itself, and a non-source drives nothing.
            _ => assert!(
                !voltage && !current,
                "{kind:?} is not an independent source and must be claimed by \
                 neither side (voltage {voltage}, current {current})"
            ),
        }
    }
}

/// Cellviews looked up from the test's own buffers.
struct Buffers<'b>(&'b HashMap<String, SchematicDocument>);

impl HierarchyDocuments for Buffers<'_> {
    fn find_schematic(&self, reference: &CellViewRef) -> Option<HierarchySchematic<'_>> {
        self.0
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(&reference.key()))
            .map(|(_, document)| HierarchySchematic {
                document,
                modified: false,
            })
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

/// A top that places `amp` once, where `amp` has a schematic with an open
/// `R1`, a second schematic view with an open `R2`, and a Verilog-A view.
struct Rebindable {
    views: HashMap<String, SchematicDocument>,
    libraries: LibraryCatalog,
    sources: ProjectSourceRegistry,
    project: ProjectId,
}

impl Rebindable {
    fn new() -> Self {
        let mut top = SchematicDocument::default();
        top.components.push(instance(1, "X1", "amp"));
        let mut amp = SchematicDocument::default();
        amp.components.push(open_resistor(1, "R1"));
        let mut amp_v2 = SchematicDocument::default();
        amp_v2.components.push(open_resistor(1, "R2"));
        let views = HashMap::from([
            ("work/top/schematic".to_owned(), top),
            ("work/amp/schematic".to_owned(), amp),
            ("work/amp/schematic_v2".to_owned(), amp_v2),
        ]);

        let mut library = Library::new("work");
        for name in ["top", "wrap"] {
            let mut cell = Cell::new(name);
            cell.add_view(View::new("schematic", ViewType::Schematic));
            library.add_cell(cell);
        }
        let mut amp_cell = Cell::new("amp");
        amp_cell.add_view(View::new("schematic", ViewType::Schematic));
        amp_cell.add_view(View::new("schematic_v2", ViewType::Schematic));
        let mut veriloga = View::new("veriloga", ViewType::VerilogA);
        veriloga
            .metadata
            .insert("veriloga.module".to_owned(), "amp".to_owned());
        veriloga
            .metadata
            .insert("veriloga.ports".to_owned(), r#"["p"]"#.to_owned());
        amp_cell.add_view(veriloga);
        library.add_cell(amp_cell);
        let mut libraries = LibraryCatalog::default();
        libraries.add_library(library);

        let mut sources = ProjectSourceRegistry::default();
        sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    ProjectSourceOwner::cell_view(CellViewRef::new("work", "amp", "veriloga")),
                    ProjectSourceLanguage::VerilogA,
                    "amp.va",
                    "module amp(p);\n  inout p;\n  electrical p;\n  analog I(p) <+ V(p) / 1k;\nendmodule\n",
                    [],
                    [],
                )
                .expect("a cell-view bundle"),
            )
            .expect("one bundle per cell view");

        Self {
            views,
            libraries,
            sources,
            project: ProjectId::new(),
        }
    }

    /// The plan a configuration with this view policy resolves to.
    fn plan(&self, executable_views: &[&str], stop_views: &[&str]) -> ConfigurationExecutionPlan {
        let root = CellViewRef::new("work", "top", "schematic");
        let mut catalog = ConfigurationSetCatalog::default();
        let id = catalog
            .create(ConfigurationSetDefinition {
                name: "walk".to_owned(),
                root: root.clone(),
                dut_path: "/X1".to_owned(),
                executable_view_policy: executable_views
                    .iter()
                    .map(|view| (*view).to_owned())
                    .collect(),
                stop_views: stop_views.iter().map(|view| (*view).to_owned()).collect(),
                unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: ConfigurationModelProfile::ProjectRunSetSections,
                owner: "drc walk test".to_owned(),
            })
            .expect("the configuration is well formed");
        let (resolution, plan) = resolve_hierarchy(
            HierarchyContext {
                documents: &Buffers(&self.views),
                libraries: &self.libraries,
                project_id: self.project,
                project_sources: &self.sources,
                source_files: &ProjectKeysOnly,
            },
            root,
            catalog.find(id),
        );
        assert!(
            resolution.is_valid(),
            "{:?}",
            resolution
                .bindings
                .iter()
                .map(|binding| (&binding.reference, &binding.diagnostic))
                .collect::<Vec<_>>()
        );
        plan
    }

    fn check(&self, hierarchy: &HierarchySource<'_>) -> DrcResult {
        run_check_with_hierarchy(
            &self.views["work/top/schematic"],
            hierarchy,
            connectivity_only(),
        )
    }
}

/// Under a configuration the run gate checks what the deck runs: the view the
/// configuration materializes at each occurrence, and nothing below a view
/// the deck writes from its own source.
#[test]
fn a_configured_walk_descends_where_the_deck_does() {
    let design = Rebindable::new();
    let at = |result: &DrcResult| {
        of_type(result, DrcViolationType::UnconnectedPin)
            .into_iter()
            .map(|finding| match &finding.location {
                DrcLocation::InsideInstance { path, view, inner } => {
                    let DrcLocation::Terminal { component_name, .. } = inner.as_ref() else {
                        panic!("an open leg is located on its terminal: {inner:?}");
                    };
                    format!("{path} {} {component_name}", view.key())
                }
                other => panic!("the open leg is inside the occurrence: {other:?}"),
            })
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    };

    // Without a configuration, the drawing's placed binding is the one read.
    let drawn = HierarchySource::from_buffers(&design.views);
    assert_eq!(at(&design.check(&drawn)), vec!["/X1 work/amp/schematic R1"]);

    // A configuration that rebinds `amp` to its second schematic checks that
    // one, and not the drawn binding the deck no longer runs.
    let rebound = HierarchySource::from_buffers(&design.views)
        .with_execution_plan(design.plan(&["schematic_v2", "schematic"], &[]));
    assert_eq!(
        at(&design.check(&rebound)),
        vec!["/X1 work/amp/schematic_v2 R2"]
    );

    // A configuration that stops at the Verilog-A view runs no schematic of
    // `amp` at all, so none of its drawing's findings may refuse the run.
    let stopped = HierarchySource::from_buffers(&design.views)
        .with_execution_plan(design.plan(&["veriloga", "schematic"], &["veriloga"]));
    let result = design.check(&stopped);
    assert!(
        result.violations().is_empty(),
        "nothing below the stop view is checked: {:?}",
        result.violations()
    );
}

/// The configured walk keys a cellview by what its subtree resolves to. A
/// configuration that rebinds one occurrence of a cell's instance and not the
/// other makes the two occurrences different masters, and each is checked
/// with what it actually instantiates.
#[test]
fn a_rebound_occurrence_is_checked_with_what_it_instantiates() {
    let mut design = Rebindable::new();
    let mut top = SchematicDocument::default();
    top.components.push(instance(1, "X1", "wrap"));
    top.components.push(instance(2, "X2", "wrap"));
    let mut wrap = SchematicDocument::default();
    wrap.components.push(instance(1, "XA", "amp"));
    design.views.insert("work/top/schematic".to_owned(), top);
    design.views.insert("work/wrap/schematic".to_owned(), wrap);

    let root = CellViewRef::new("work", "top", "schematic");
    let mut catalog = ConfigurationSetCatalog::default();
    let id = catalog
        .create(ConfigurationSetDefinition {
            name: "one occurrence rebound".to_owned(),
            root: root.clone(),
            dut_path: "/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![crate::configuration_set::ConfigurationSetOverride {
                instance_path: "/X2/XA".to_owned(),
                executable_views: vec!["schematic_v2".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: crate::configuration_set::ConfigurationPlatform::ALL.to_vec(),
            }],
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
            owner: "drc walk test".to_owned(),
        })
        .expect("the configuration is well formed");
    let (resolution, plan) = resolve_hierarchy(
        HierarchyContext {
            documents: &Buffers(&design.views),
            libraries: &design.libraries,
            project_id: design.project,
            project_sources: &design.sources,
            source_files: &ProjectKeysOnly,
        },
        root,
        catalog.find(id),
    );
    assert!(resolution.is_valid());
    let hierarchy = HierarchySource::from_buffers(&design.views).with_execution_plan(plan);

    let result = design.check(&hierarchy);

    let mut legs = of_type(&result, DrcViolationType::UnconnectedPin)
        .into_iter()
        .map(|finding| match &finding.location {
            DrcLocation::InsideInstance { path, view, .. } => format!("{path} {}", view.key()),
            other => panic!("{other:?}"),
        })
        .collect::<Vec<_>>();
    legs.sort();
    legs.dedup();
    assert_eq!(
        legs,
        vec![
            "/X1/XA work/amp/schematic".to_owned(),
            "/X2/XA work/amp/schematic_v2".to_owned(),
        ]
    );
}

/// The reference rules decide by what the deck does with the name, driven
/// through the extraction that computes it.
#[test]
fn a_refused_reference_is_judged_by_what_the_deck_writes() {
    let mut top = SchematicDocument::default();
    // Imported: a resistor carrying a capacitor's letter, and one with none.
    top.components.push(
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("C4", "1k"),
    );
    top.components.push(
        Component::new(5, ComponentType::Resistor, Point::new(0, 100)).with_name_value("", "1k"),
    );
    // Cell instances are addressed by path; these two have no path segment.
    top.components.push(instance(7, "X1-a", "amp"));
    top.components.push(instance(8, "", "amp"));
    let views = buffers([("amp", SchematicDocument::default())]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(
        &top,
        &hierarchy,
        DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        },
    );

    let finding = |id: u64| {
        result
            .violations()
            .iter()
            .find(|finding| {
                matches!(&finding.location, DrcLocation::Component { id: found, .. } if *found == id)
            })
            .unwrap_or_else(|| panic!("component #{id} is reported: {:?}", result.violations()))
    };
    let misnamed = finding(1);
    assert_eq!(misnamed.violation_type, DrcViolationType::InvalidName);
    assert_eq!(misnamed.severity, DrcSeverity::Warning);
    assert!(
        misnamed.message.ends_with("the netlist writes it as RC4"),
        "{}",
        misnamed.message
    );
    let unnamed = finding(5);
    assert_eq!(unnamed.violation_type, DrcViolationType::EmptyName);
    assert_eq!(unnamed.severity, DrcSeverity::Warning);
    assert!(
        unnamed.message.ends_with("the netlist writes it as R5"),
        "{}",
        unnamed.message
    );
    for id in [7, 8] {
        assert_eq!(
            finding(id).severity,
            DrcSeverity::Error,
            "{:?}",
            finding(id)
        );
    }
    assert_eq!(result.errors().len(), 2, "{:?}", result.violations());
}

/// A name the netlister writes but the engine's element-head parser refuses
/// never runs, so it is an error rather than a renamed-and-run warning. Each
/// character the parser keeps for the card's own grammar is tried on its own;
/// punctuation the parser does read as part of a name stays a warning.
#[test]
fn a_reference_the_engine_cannot_parse_is_an_error() {
    let refused = [
        "R(1", "R1)", "R1,2", "Rx=1", "R{a", "Ra}", "R'a", "R\"a", "R;a",
    ];
    let mut top = SchematicDocument::default();
    for (index, name) in (1_u64..).zip(refused) {
        top.components.push(
            Component::new(
                index,
                ComponentType::Resistor,
                Point::new(0, 200 * index as i32),
            )
            .with_name_value(name, "1k"),
        );
    }
    top.components.push(
        Component::new(100, ComponentType::Resistor, Point::new(1000, 0))
            .with_name_value("R+1", "1k"),
    );
    let views = buffers([]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(
        &top,
        &hierarchy,
        DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        },
    );

    let finding = |id: u64| {
        result
            .violations()
            .iter()
            .find(|finding| {
                matches!(&finding.location, DrcLocation::Component { id: found, .. } if *found == id)
            })
            .unwrap_or_else(|| panic!("component #{id} is reported: {:?}", result.violations()))
    };
    for (id, name) in (1_u64..).zip(refused) {
        let refused = finding(id);
        assert_eq!(
            refused.violation_type,
            DrcViolationType::InvalidName,
            "{name}"
        );
        assert_eq!(refused.severity, DrcSeverity::Error, "{name}");
        assert!(
            refused
                .message
                .ends_with("the netlist cannot write this instance"),
            "{name}: {}",
            refused.message
        );
    }
    let readable = finding(100);
    assert_eq!(readable.severity, DrcSeverity::Warning);
    assert!(
        readable.message.ends_with("the netlist writes it as R+1"),
        "{}",
        readable.message
    );
}

/// The engine refuses two element names that fold together in one scope, and
/// what it reads are the written names. Each pair here is drawn under two
/// different names that the deck writes as one.
#[test]
fn instances_written_under_one_name_are_duplicates() {
    let resistor = |id: u64, name: &str| {
        Component::new(id, ComponentType::Resistor, Point::new(200 * id as i32, 0))
            .with_name_value(name, "1k")
    };
    let cases = [
        (
            vec![resistor(1, "C4"), resistor(2, "RC4")],
            "C4 and RC4 are both written RC4",
        ),
        (
            vec![resistor(4, ""), resistor(5, "R4")],
            "component #4 and R4 are both written R4",
        ),
        (
            vec![instance(1, "amp", "amp"), instance(2, "Xamp", "amp")],
            "amp and Xamp are both written Xamp",
        ),
        (
            vec![resistor(1, "R1"), resistor(2, "r1")],
            "R1 and r1 are written R1 and r1, which the simulator reads as one name",
        ),
    ];
    let views = buffers([("amp", SchematicDocument::default())]);
    let hierarchy = HierarchySource::from_buffers(&views);
    for (components, message) in cases {
        let top = SchematicDocument {
            components,
            ..SchematicDocument::default()
        };
        let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());
        let duplicates = of_type(&result, DrcViolationType::DuplicateName);
        assert_eq!(duplicates.len(), 1, "{message}: {:?}", result.violations());
        assert_eq!(duplicates[0].message, message);
        assert!(duplicates[0].severity >= DrcSeverity::Error);
    }
}

/// Every leg of a passive is undeclared, so a drawing made of passives raises
/// no undriven-input error however many terminals it leaves open — and each
/// open leg is one finding, not an open pin and a floating node besides.
#[test]
fn open_passive_legs_raise_no_input_error() {
    let mut top = SchematicDocument::default();
    top.components.push(open_resistor(1, "R1"));
    top.components.push(
        Component::new(2, ComponentType::Capacitor, Point::new(200, 0)).with_name_value("C1", "1p"),
    );
    let views = buffers([]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(
        &top,
        &hierarchy,
        DrcConfig {
            check_missing_ground: false,
            ..DrcConfig::default()
        },
    );

    assert_eq!(of_type(&result, DrcViolationType::UnconnectedPin).len(), 4);
    assert!(of_type(&result, DrcViolationType::FloatingNode).is_empty());
    assert!(of_type(&result, DrcViolationType::UndrivenInput).is_empty());
    assert!(of_type(&result, DrcViolationType::UnloadedOutput).is_empty());
    assert_eq!(result.total_count(), 4, "{:?}", result.violations());
    assert!(result.passed(), "{:?}", result.violations());
}

/// A structural object writes no instance, so it is never an unknown
/// component; a cell instance whose master resolves to nothing is, and it
/// cannot be written under any name.
#[test]
fn only_an_emitted_instance_can_be_an_unknown_component() {
    let mut top = SchematicDocument::default();
    top.components
        .push(Component::new(1, ComponentType::Port, Point::new(0, 0)).with_name_value("", "IN"));
    top.components.push(instance(2, "X2", "missing"));
    let views = buffers([]);
    let hierarchy = HierarchySource::from_buffers(&views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let unknown = of_type(&result, DrcViolationType::UnknownComponent);
    assert_eq!(unknown.len(), 1, "{:?}", result.violations());
    assert_eq!(
        unknown[0].location,
        DrcLocation::Component {
            id: 2,
            name: "X2".to_owned()
        }
    );
    assert_eq!(unknown[0].severity, DrcSeverity::Error);
}

/// The set and reset inputs of a built-in flip-flop may stay open: the
/// netlister writes `null` for them. The clocked inputs may not.
#[test]
fn an_optional_xspice_input_left_open_is_not_undriven() {
    let descriptor = crate::schematic::device_catalog::engine_only_xspice_devices()
        .iter()
        .find(|descriptor| descriptor.model_type == "d_srff")
        .expect("the catalog carries the SR flip-flop");
    let binding = crate::schematic::device_catalog::builtin_xspice_library_binding(descriptor)
        .expect("the catalog binding materializes");
    let mut top = SchematicDocument::default();
    top.components.push(
        Component::new(1, ComponentType::CellInstance, Point::new(0, 0))
            .with_library_cell(binding)
            .with_name_value("A1", "d_srff"),
    );
    // A library-bound source resolves the instance's symbol, which is what
    // names its terminals after the ports.
    let views = buffers([]);
    let libraries = LibraryCatalog::default();
    let hierarchy = HierarchySource::from_workspace(&libraries, &views);

    let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());

    let mut undriven = of_type(&result, DrcViolationType::UndrivenInput)
        .into_iter()
        .map(|finding| match &finding.location {
            DrcLocation::Terminal { pin_name, .. } => pin_name.to_ascii_lowercase(),
            other => panic!("{other:?}"),
        })
        .collect::<Vec<_>>();
    undriven.sort();
    assert_eq!(undriven, vec!["clk", "r", "s"], "{:?}", result.violations());
    assert!(
        result.violations().iter().all(|finding| !matches!(
            &finding.location,
            DrcLocation::Terminal { pin_name, .. }
                if pin_name.eq_ignore_ascii_case("set") || pin_name.eq_ignore_ascii_case("reset")
        )),
        "{:?}",
        result.violations()
    );
}

/// A master port named `dir=` and spelled however the master spells it.
fn port(id: u64, name: &str, direction: &str) -> Component {
    let mut port = Component::new(id, ComponentType::Port, Point::new(0, id as i32 * 40))
        .with_name_value("", name);
    port.params = format!("dir={direction}");
    port
}

/// An instance placed when `amp`'s only port was declared `pin`.
fn placed_amp(pin: &str, direction: PortDirection) -> Component {
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[PortSpec {
        name: pin.to_owned(),
        direction,
    }]);
    Component::new(1, ComponentType::CellInstance, Point::new(0, 0))
        .with_library_cell(binding)
        .with_name_value("X1", "amp")
}

/// Direction is read from the master as it stands, not from the copy the
/// instance took when it was placed, and a pin matches its port whatever the
/// case.
#[test]
fn an_instance_pin_is_classified_by_its_masters_live_interface() {
    let check = |placed: Component, master_port: Component| {
        let mut top = SchematicDocument::default();
        top.components.push(placed);
        let views = buffers([(
            "amp",
            SchematicDocument {
                components: vec![master_port],
                ..SchematicDocument::default()
            },
        )]);
        let libraries = LibraryCatalog::default();
        let hierarchy = HierarchySource::from_workspace(&libraries, &views);
        let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());
        result
            .violations()
            .iter()
            .filter(|finding| matches!(finding.location, DrcLocation::Terminal { .. }))
            .map(|finding| finding.violation_type)
            .collect::<Vec<_>>()
    };

    // Placed as an input; the master has since made it an output.
    assert_eq!(
        check(placed_amp("A", PortDirection::In), port(1, "A", "out")),
        vec![DrcViolationType::UnloadedOutput]
    );
    // The symbol spells it `A`, the master `a`: one pin.
    assert_eq!(
        check(placed_amp("A", PortDirection::In), port(1, "a", "in")),
        vec![DrcViolationType::UndrivenInput]
    );
}

/// A global is one node in the deck, so two placements of one cell that each
/// put a source across the same pair of globals are two ideal sources in
/// parallel — a loop no single cellview's check can see.
#[test]
fn sources_across_one_pair_of_globals_in_two_occurrences_are_a_loop() {
    let source = Component::new(1, ComponentType::VoltageSource, Point::new(0, 0))
        .with_name_value("V1", "1");
    let terminals = terminal_positions_with_hierarchy(&source, None);
    let at = |pin: &str| {
        terminals
            .iter()
            .find_map(|(name, point)| (name == pin).then_some(*point))
            .expect("a voltage source has both terminals")
    };
    let supply = SchematicDocument {
        net_labels: vec![
            NetLabel::new(10, at("+"), "vdd!"),
            NetLabel::new(11, at("-"), "0"),
        ],
        components: vec![source],
        ..SchematicDocument::default()
    };
    let views = buffers([("supply", supply)]);
    let libraries = LibraryCatalog::default();
    let contract = crate::connectivity_contract::ConnectivityContract::default();
    let hierarchy =
        HierarchySource::from_workspace(&libraries, &views).with_connectivity(&contract);

    let placed_twice = SchematicDocument {
        components: vec![instance(1, "X1", "supply"), instance(2, "X2", "supply")],
        ..SchematicDocument::default()
    };
    let result = run_check_with_hierarchy(&placed_twice, &hierarchy, DrcConfig::default());
    let loops = of_type(&result, DrcViolationType::SourceToSource);
    assert_eq!(loops.len(), 1, "{:?}", result.violations());
    assert_eq!(loops[0].severity, DrcSeverity::Error);
    assert!(
        loops[0].message.contains("/X1/V1") && loops[0].message.contains("/X2/V1"),
        "{}",
        loops[0].message
    );
    // Ground is the reference both sources hold the global against, so the
    // loop is stated at the global. Nothing in the top draws either source,
    // so it is stated inside the first occurrence, where it can be changed.
    assert!(
        loops[0]
            .message
            .starts_with("2 voltage sources form a closed loop through global node 'vdd!'"),
        "{}",
        loops[0].message
    );
    let DrcLocation::InsideInstance { path, view, inner } = &loops[0].location else {
        panic!("stated inside an occurrence: {:?}", loops[0].location);
    };
    assert_eq!(
        (path.to_string(), view.key()),
        ("/X1".to_owned(), "work/supply/schematic".to_owned())
    );
    assert_eq!(
        **inner,
        DrcLocation::Node {
            net_name: "vdd!".to_owned()
        }
    );
    // Within the cell the global is touched once, and that is not floating.
    assert!(of_type(&result, DrcViolationType::FloatingNode).is_empty());

    let placed_once = SchematicDocument {
        components: vec![instance(1, "X1", "supply")],
        ..SchematicDocument::default()
    };
    let result = run_check_with_hierarchy(&placed_once, &hierarchy, DrcConfig::default());
    assert!(
        of_type(&result, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        result.violations()
    );

    // With one of the two sources drawn in the top, the loop is the top's
    // own to break, and it is stated there.
    let mut beside = labelled(vec![(
        part(2, ComponentType::VoltageSource, "V2"),
        vec![("+", "vdd!"), ("-", "0")],
    )]);
    beside.components.push(instance(1, "X1", "supply"));
    let result = run_check_with_hierarchy(&beside, &hierarchy, DrcConfig::default());
    assert_eq!(
        placed(&result, DrcViolationType::SourceToSource),
        vec![("/".to_owned(), None)],
        "{:?}",
        result.violations()
    );
}

/// One cellview drawn as `parts`, each listed terminal labelled with the node
/// it stands on. Every part sits at its own spot, so only labels join them.
fn labelled(parts: Vec<(Component, Vec<(&str, &str)>)>) -> SchematicDocument {
    let mut document = SchematicDocument::default();
    for (component, nodes) in parts {
        let terminals = terminal_positions_with_hierarchy(&component, None);
        for (index, (pin, node)) in (0_u64..).zip(nodes) {
            let point = terminals
                .iter()
                .find_map(|(name, point)| (name == pin).then_some(*point))
                .unwrap_or_else(|| panic!("{} has a terminal {pin}", component.name));
            document
                .net_labels
                .push(NetLabel::new(component.id * 100 + index, point, node));
        }
        document.components.push(component);
    }
    document
}

/// A primitive at a spot of its own.
fn part(id: u64, kind: ComponentType, name: &str) -> Component {
    Component::new(id, kind, Point::new(400 * id as i32, 0)).with_name_value(name, "1")
}

/// A global continues into every cellview that names it, so a terminal on one
/// is connected only if something else in the design names it too. Touched
/// once in the whole design, it is floating; touched once in each of two
/// cells, it is two connections.
#[test]
fn a_global_touched_once_in_the_design_is_floating() {
    let resistor = |nodes: [&str; 2]| {
        labelled(vec![(
            part(1, ComponentType::Resistor, "R1"),
            vec![("+", nodes[0]), ("-", nodes[1])],
        )])
    };
    let libraries = LibraryCatalog::default();
    let contract = crate::connectivity_contract::ConnectivityContract::default();
    let vref = DrcLocation::Node {
        net_name: "vref!".to_owned(),
    };

    // R1 from `in` to `vref!`, and nothing else names `vref!`.
    let views = buffers([("top", resistor(["in", "vref!"]))]);
    let hierarchy =
        HierarchySource::from_workspace(&libraries, &views).with_connectivity(&contract);
    let result = run_check_with_hierarchy(
        &views["work/top/schematic"],
        &hierarchy,
        DrcConfig::default(),
    );
    let floating = of_type(&result, DrcViolationType::FloatingNode);
    let global = floating
        .iter()
        .find(|finding| finding.location == vref)
        .unwrap_or_else(|| panic!("vref! is floating: {:?}", result.violations()));
    assert_eq!(global.severity, DrcSeverity::Warning);

    let views = buffers([
        ("a", resistor(["vref!", "0"])),
        ("b", resistor(["vref!", "0"])),
    ]);
    let hierarchy =
        HierarchySource::from_workspace(&libraries, &views).with_connectivity(&contract);
    let both = SchematicDocument {
        components: vec![instance(1, "XA", "a"), instance(2, "XB", "b")],
        ..SchematicDocument::default()
    };
    let result = run_check_with_hierarchy(&both, &hierarchy, DrcConfig::default());
    assert!(
        of_type(&result, DrcViolationType::FloatingNode).is_empty(),
        "{:?}",
        result.violations()
    );

    // Either cell alone touches it once, and the finding is stated inside it.
    let one = SchematicDocument {
        components: vec![instance(1, "XA", "a")],
        ..SchematicDocument::default()
    };
    let result = run_check_with_hierarchy(&one, &hierarchy, DrcConfig::default());
    assert_eq!(
        placed(&result, DrcViolationType::FloatingNode),
        vec![("/XA".to_owned(), Some("work/a/schematic".to_owned()))],
        "{:?}",
        result.violations()
    );
}

/// What the cross-cellview driver rules leave alone: sources on different
/// globals, a source on a port or on a local net of a cell placed twice —
/// each occurrence's node is its own there — and a conflict confined to one
/// occurrence, which that cellview's own rule states once.
#[test]
fn only_a_conflict_on_one_global_across_occurrences_is_a_hierarchy_finding() {
    let source = |id: u64, plus: &'static str, minus: &'static str| {
        (
            part(id, ComponentType::VoltageSource, &format!("V{id}")),
            vec![("+", plus), ("-", minus)],
        )
    };
    let libraries = LibraryCatalog::default();
    let contract = crate::connectivity_contract::ConnectivityContract::default();
    let check = |views: &HashMap<String, SchematicDocument>, top: Vec<Component>| {
        let hierarchy =
            HierarchySource::from_workspace(&libraries, views).with_connectivity(&contract);
        let top = SchematicDocument {
            components: top,
            ..SchematicDocument::default()
        };
        run_check_with_hierarchy(&top, &hierarchy, DrcConfig::default())
    };

    let views = buffers([
        ("vdd", labelled(vec![source(1, "vdd!", "0")])),
        ("vcc", labelled(vec![source(1, "vcc!", "0")])),
    ]);
    let result = check(
        &views,
        vec![instance(1, "XD", "vdd"), instance(2, "XC", "vcc")],
    );
    assert!(
        of_type(&result, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        result.violations()
    );

    let views = buffers([
        (
            "port",
            labelled(vec![
                source(1, "p", "0"),
                (port(9, "p", "inout"), vec![("P", "p")]),
            ]),
        ),
        ("local", labelled(vec![source(1, "n", "0")])),
    ]);
    let result = check(
        &views,
        vec![
            instance(1, "XP1", "port"),
            instance(2, "XP2", "port"),
            instance(3, "XL1", "local"),
            instance(4, "XL2", "local"),
        ],
    );
    assert!(
        of_type(&result, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        result.violations()
    );

    let views = buffers([(
        "pair",
        labelled(vec![source(1, "vdd!", "0"), source(2, "vdd!", "0")]),
    )]);
    let result = check(&views, vec![instance(1, "X1", "pair")]);
    assert_eq!(
        placed(&result, DrcViolationType::SourceToSource),
        vec![("/X1".to_owned(), Some("work/pair/schematic".to_owned()))],
        "the cell's own rule states it, and the hierarchy rule does not repeat it"
    );
}

/// Across the hierarchy, as within one cellview, sources conflict only where
/// they close a loop: sharing a global is an ordinary circuit, a ring through
/// three cellviews is a loop, and so is a path through a cell's own node once
/// the cell is placed twice.
#[test]
fn cross_cellview_sources_conflict_only_where_they_close_a_loop() {
    let source = |id: u64, plus: &'static str, minus: &'static str| {
        (
            part(id, ComponentType::VoltageSource, &format!("V{id}")),
            vec![("+", plus), ("-", minus)],
        )
    };
    let libraries = LibraryCatalog::default();
    let contract = crate::connectivity_contract::ConnectivityContract::default();
    let check = |views: &HashMap<String, SchematicDocument>, top: SchematicDocument| {
        let hierarchy =
            HierarchySource::from_workspace(&libraries, views).with_connectivity(&contract);
        run_check_with_hierarchy(&top, &hierarchy, DrcConfig::default())
    };
    let cells = |instances: Vec<Component>| SchematicDocument {
        components: instances,
        ..SchematicDocument::default()
    };

    // V1 holds vdd! against ground; the other cell's V1 hangs a local node
    // off vdd!. The engine solves that.
    let views = buffers([
        ("supply", labelled(vec![source(1, "vdd!", "0")])),
        ("offset", labelled(vec![source(1, "vb", "vdd!")])),
    ]);
    let result = check(
        &views,
        cells(vec![
            instance(1, "XS", "supply"),
            instance(2, "XO", "offset"),
        ]),
    );
    assert!(
        of_type(&result, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        result.violations()
    );

    // vdd! to vss! in one cell, vss! to ground in another, and ground back to
    // vdd! in the top: one loop, stated in the top, which draws part of it.
    let views = buffers([
        ("upper", labelled(vec![source(1, "vdd!", "vss!")])),
        ("lower", labelled(vec![source(1, "vss!", "0")])),
    ]);
    let mut top = labelled(vec![source(3, "0", "vdd!")]);
    top.components.push(instance(1, "XU", "upper"));
    top.components.push(instance(2, "XL", "lower"));
    let result = check(&views, top);
    let loops = of_type(&result, DrcViolationType::SourceToSource);
    assert_eq!(loops.len(), 1, "{:?}", result.violations());
    assert!(
        loops[0]
            .message
            .starts_with("3 voltage sources form a closed loop through global node 'vdd!'"),
        "{}",
        loops[0].message
    );
    assert_eq!(
        placed(&result, DrcViolationType::SourceToSource),
        vec![("/".to_owned(), None)]
    );

    // A cell that holds vdd! against ground through a node of its own is
    // one path; placed twice, the two copies are a loop.
    let views = buffers([(
        "chain",
        labelled(vec![source(1, "vdd!", "mid"), source(2, "mid", "0")]),
    )]);
    let once = check(&views, cells(vec![instance(1, "X1", "chain")]));
    assert!(
        of_type(&once, DrcViolationType::SourceToSource).is_empty(),
        "{:?}",
        once.violations()
    );
    let twice = check(
        &views,
        cells(vec![instance(1, "X1", "chain"), instance(2, "X2", "chain")]),
    );
    let loops = of_type(&twice, DrcViolationType::SourceToSource);
    assert_eq!(loops.len(), 1, "{:?}", twice.violations());
    assert_eq!(
        loops[0].related_items,
        ["/X1/V1", "/X2/V1", "/X1/V2", "/X2/V2"].map(str::to_owned)
    );
}

/// A cell whose instance of `amp` drives `node` from each of `outputs`. The
/// instance's pins are where `amp`'s symbol puts them.
fn driving(
    amp: &SchematicDocument,
    node: &'static str,
    outputs: &[&'static str],
) -> SchematicDocument {
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(
        &outputs
            .iter()
            .map(|name| PortSpec {
                name: (*name).to_owned(),
                direction: PortDirection::Out,
            })
            .collect::<Vec<_>>(),
    );
    let instance = Component::new(1, ComponentType::CellInstance, Point::new(0, 0))
        .with_library_cell(binding)
        .with_name_value("X1", "amp");
    let views = buffers([("amp", amp.clone())]);
    let libraries = LibraryCatalog::default();
    let hierarchy = HierarchySource::from_workspace(&libraries, &views);
    let terminals = terminal_positions_with_hierarchy(&instance, Some(&hierarchy));
    let mut document = SchematicDocument::default();
    for (id, output) in (100_u64..).zip(outputs) {
        let point = terminals
            .iter()
            .find_map(|(name, point)| (name == output).then_some(*point))
            .unwrap_or_else(|| panic!("amp's symbol has a pin {output}: {terminals:?}"));
        document.net_labels.push(NetLabel::new(id, point, node));
    }
    document.components.push(instance);
    document
}

/// Ground is the reference, not a contested net: one declared output tied to
/// ground in a cell placed twice is not two outputs fighting. On a global it
/// is, and the drivers are counted per instance, as the cell's own rule
/// counts them, not per pin.
#[test]
fn cross_cellview_shorted_outputs_count_instances_and_leave_ground_alone() {
    let amp = SchematicDocument {
        components: vec![port(1, "o1", "out"), port(2, "o2", "out")],
        ..SchematicDocument::default()
    };
    let libraries = LibraryCatalog::default();
    let contract = crate::connectivity_contract::ConnectivityContract::default();
    let twice = SchematicDocument {
        components: vec![instance(1, "XA", "drv"), instance(2, "XB", "drv")],
        ..SchematicDocument::default()
    };

    let views = buffers([("amp", amp.clone()), ("drv", driving(&amp, "0", &["o1"]))]);
    let hierarchy =
        HierarchySource::from_workspace(&libraries, &views).with_connectivity(&contract);
    let result = run_check_with_hierarchy(&twice, &hierarchy, DrcConfig::default());
    assert!(
        of_type(&result, DrcViolationType::ShortedOutputs).is_empty(),
        "{:?}",
        result.violations()
    );

    let drv = driving(&amp, "bus!", &["o1", "o2"]);
    let views = buffers([("amp", amp), ("drv", drv)]);
    let hierarchy =
        HierarchySource::from_workspace(&libraries, &views).with_connectivity(&contract);
    let result = run_check_with_hierarchy(&twice, &hierarchy, DrcConfig::default());
    let shorted = of_type(&result, DrcViolationType::ShortedOutputs);
    assert_eq!(shorted.len(), 1, "{:?}", result.violations());
    assert!(
        shorted[0]
            .message
            .starts_with("Global node 'bus!' has 2 declared output drivers"),
        "{}",
        shorted[0].message
    );
}

/// A current-controlled source names its controlling element. The deck
/// writes the element a drawn name means under its emitted name, so a drawn
/// name is a reference that resolves; one that names nothing, or names two
/// instances, leaves the deck an element the engine does not have.
#[test]
fn a_controlling_reference_must_name_one_instance() {
    let check = |vref: &str, extra: Option<Component>| {
        let mut sensed =
            Component::new(1, ComponentType::Cccs, Point::new(0, 0)).with_name_value("F1", "2");
        sensed.params = format!("vref={vref}");
        let mut top = SchematicDocument::default();
        top.components.push(sensed);
        top.components
            .push(part(2, ComponentType::VoltageSource, "sense"));
        top.components.extend(extra);
        let views = buffers([]);
        let hierarchy = HierarchySource::from_buffers(&views);
        let result = run_check_with_hierarchy(&top, &hierarchy, connectivity_only());
        of_type(&result, DrcViolationType::UnresolvedReference)
            .into_iter()
            .map(|finding| (finding.severity, finding.message.clone()))
            .collect::<Vec<_>>()
    };

    // Drawn `sense`, written `Vsense`: either spelling names it.
    assert_eq!(check("sense", None), Vec::new());
    assert_eq!(check("VSENSE", None), Vec::new());
    assert_eq!(
        check("vbias", None),
        vec![(
            DrcSeverity::Error,
            "F1 vref='vbias' names no instance of this cellview".to_owned()
        )]
    );
    // A resistor drawn `sense` is written `Rsense`; the drawn name now names
    // two instances and neither written name.
    assert_eq!(
        check("sense", Some(part(3, ComponentType::Resistor, "sense"))),
        vec![(
            DrcSeverity::Error,
            "F1 vref='sense' names more than one instance of this cellview".to_owned()
        )]
    );
}
