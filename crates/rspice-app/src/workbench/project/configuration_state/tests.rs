//! The Configuration page's selection, over a catalog built in place.

use super::*;
use crate::state::{
    ConfigurationBlackBoxPolicy, ConfigurationModelProfile, UnresolvedBindingPolicy,
};

fn definition(name: &str) -> ConfigurationSetDefinition {
    ConfigurationSetDefinition {
        name: name.to_owned(),
        root: CellViewRef::new("user", "top", "schematic"),
        dut_path: "/X1".to_owned(),
        executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
        stop_views: vec!["spice".to_owned()],
        unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
        black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
        overrides: Vec::new(),
        cell_bindings: Vec::new(),
        model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        owner: "Local project".to_owned(),
    }
}

/// A catalog whose first configuration is the one used for runs.
fn catalog_with_active() -> (ConfigurationSetCatalog, ConfigurationSetId) {
    let mut catalog = ConfigurationSetCatalog::default();
    let active = catalog.create(definition("schematic")).unwrap();
    catalog.create(definition("extracted")).unwrap();
    (catalog, active)
}

#[test]
fn the_page_opens_on_what_runs_use_and_keeps_what_the_reader_picks() {
    let (catalog, active) = catalog_with_active();
    let mut page = ConfigurationPageState::default();
    assert_eq!(
        page.selection(&catalog),
        ConfigurationSelection::Configuration(active)
    );
    page.select(ConfigurationSelection::Default);
    assert_eq!(page.selection(&catalog), ConfigurationSelection::Default);
    page.select(ConfigurationSelection::Configuration(
        ConfigurationSetId::new(),
    ));
    assert_eq!(
        page.selection(&catalog),
        ConfigurationSelection::Configuration(active),
        "a configuration that no longer exists falls back to the one used for runs"
    );
    assert_eq!(
        ConfigurationPageState::default().selection(&ConfigurationSetCatalog::default()),
        ConfigurationSelection::Default
    );
    assert!(used_for_runs(
        &ConfigurationSetCatalog::default(),
        ConfigurationSelection::Default
    ));
    assert!(!used_for_runs(&catalog, ConfigurationSelection::Default));
}

#[test]
fn a_new_selection_clears_the_refusal_it_did_not_cause() {
    let mut page = ConfigurationPageState::default();
    page.select(ConfigurationSelection::Default);
    page.refusal = Some("refused".to_owned());
    page.select(ConfigurationSelection::Default);
    assert!(
        page.refusal.is_some(),
        "the same selection keeps its refusal"
    );
    page.select(ConfigurationSelection::Configuration(
        ConfigurationSetId::new(),
    ));
    assert!(page.refusal.is_none());
}

#[test]
fn an_edit_makes_the_page_dirty_and_putting_it_back_makes_it_clean() {
    let (catalog, active) = catalog_with_active();
    let mut page = ConfigurationPageState::default();
    assert!(!page.is_dirty(&catalog));
    page.edit(&catalog, active, |definition| {
        definition.stop_views.clear();
    });
    assert!(page.is_dirty(&catalog));
    assert_eq!(page.draft().unwrap().base_revision, 1);
    assert!(
        page.definition(&catalog, active)
            .unwrap()
            .stop_views
            .is_empty(),
        "the page shows the edits, not what the project holds"
    );
    page.edit(&catalog, active, |definition| {
        definition.stop_views = vec!["spice".to_owned()];
    });
    assert!(!page.is_dirty(&catalog));
    assert!(page.draft().is_none(), "an edit put back ends the draft");

    page.edit(&catalog, active, |definition| definition.stop_views.clear());
    page.revert();
    assert!(!page.is_dirty(&catalog));
    assert_eq!(
        page.definition(&catalog, active),
        Some(catalog.find(active).unwrap().definition())
    );
}

#[test]
fn another_selection_waits_while_edits_are_not_applied() {
    let (catalog, active) = catalog_with_active();
    let mut page = ConfigurationPageState::default();
    page.edit(&catalog, active, |definition| definition.stop_views.clear());
    page.request_selection(&catalog, ConfigurationSelection::Default);
    assert_eq!(
        page.pending_selection,
        Some(ConfigurationSelection::Default)
    );
    assert_eq!(
        page.selection(&catalog),
        ConfigurationSelection::Configuration(active)
    );
    assert!(page.is_dirty(&catalog), "asking keeps the edits");

    // Discarding is selecting: the edits belong to the configuration left.
    page.select(ConfigurationSelection::Default);
    assert_eq!(page.pending_selection, None);
    assert!(page.draft().is_none());

    page.request_selection(&catalog, ConfigurationSelection::Configuration(active));
    assert_eq!(
        page.selection(&catalog),
        ConfigurationSelection::Configuration(active),
        "nothing unapplied, nothing to ask"
    );
}

#[test]
fn view_to_use_writes_an_exact_binding_and_inherited_removes_it() {
    let mut definition = definition("schematic");
    definition.overrides.push(ConfigurationSetOverride {
        instance_path: "/*".to_owned(),
        executable_views: vec!["spice".to_owned()],
        stop_view: Some("spice".to_owned()),
        model_section: None,
        eligible_platforms: vec![crate::state::ConfigurationPlatform::Desktop],
    });

    set_instance_view(&mut definition, "/X1", Some("extracted"));
    let exact = instance_binding(&definition, "/x1").expect("bound by its exact path");
    assert_eq!(exact.instance_path, "/X1");
    assert_eq!(exact.executable_views, ["extracted"]);
    assert_eq!(exact.stop_view, None);
    assert_eq!(
        exact.eligible_platforms,
        crate::state::ConfigurationPlatform::ALL.to_vec(),
        "a new binding runs on every platform, not only the one drawing it"
    );
    assert_eq!(
        governing_binding(&definition, "/X1").unwrap().instance_path,
        "/X1",
        "the exact binding is more specific than the pattern"
    );
    assert_eq!(
        governing_binding(&definition, "/X2").unwrap().instance_path,
        "/*"
    );
    assert!(is_pattern(&definition.overrides[0]));
    assert!(!is_pattern(exact));

    definition.overrides[1].model_section = Some("tt".to_owned());
    set_instance_view(&mut definition, "/X1", Some("schematic"));
    let exact = instance_binding(&definition, "/X1").unwrap();
    assert_eq!(exact.executable_views, ["schematic"]);
    assert_eq!(
        exact.model_section.as_deref(),
        Some("tt"),
        "a change of view keeps the binding's other properties"
    );

    set_instance_view(&mut definition, "/X1", None);
    assert!(instance_binding(&definition, "/X1").is_none());
    assert_eq!(definition.overrides.len(), 1, "the pattern binding stays");
    set_instance_view(&mut definition, "/X1", None);
    assert_eq!(definition.overrides.len(), 1);
}

#[test]
fn the_tree_puts_children_under_their_parents_and_hides_collapsed_ones() {
    use rspice_design::hierarchy::{
        HierarchyBindingRule, HierarchyBindingStatus, ResolvedHierarchyBinding,
    };

    let binding = |cell: &str, paths: &[&str]| ResolvedHierarchyBinding {
        reference: CellViewRef::new("user", cell, "schematic"),
        purpose: String::new(),
        view_search_order: vec!["schematic".to_owned()],
        stop_view: None,
        model_section: String::new(),
        status: HierarchyBindingStatus::Resolved,
        rule: HierarchyBindingRule::ViewList,
        instance_count: paths.len(),
        instance_paths: paths.iter().map(|path| (*path).to_owned()).collect(),
        used_review_fallback: false,
        diagnostic: None,
        warnings: Vec::new(),
    };
    let hierarchy = HierarchyResolution {
        bindings: vec![
            binding("amp", &["/X2", "/X1"]),
            binding("top", &["/"]),
            binding("filter", &["/X1/XF1", "/X2/XF1"]),
        ],
        total_instances: 5,
        resolved_instances: 5,
        configuration_id: None,
        configuration_revision: None,
        configuration_digest: None,
    };
    let rows = instance_rows(&hierarchy);
    let shape = rows
        .iter()
        .map(|row| (row.path.as_str(), row.depth, row.children))
        .collect::<Vec<_>>();
    assert_eq!(
        shape,
        [
            ("/", 0, 2),
            ("/X1", 1, 1),
            ("/X1/XF1", 2, 0),
            ("/X2", 1, 1),
            ("/X2/XF1", 2, 0),
        ]
    );

    let mut page = ConfigurationPageState::default();
    page.resolution.value = Some(Arc::new(ConfigurationResolution {
        hierarchy: Ok(hierarchy),
        instances: rows,
        top_cells: Vec::new(),
        view_names: Vec::new(),
    }));
    assert_eq!(&*page.visible_instances(), [0, 1, 2, 3, 4]);
    page.toggle_collapsed("/X1");
    assert!(page.is_collapsed("/x1"));
    assert_eq!(&*page.visible_instances(), [0, 1, 3, 4]);
    page.toggle_collapsed("/");
    assert_eq!(&*page.visible_instances(), [0]);
}

#[test]
fn refusals_say_what_is_wrong_in_the_page_words() {
    let (mut catalog, active) = catalog_with_active();
    let mut broken = definition("schematic");
    broken.stop_views = vec!["extracted".to_owned()];
    let refusal = plain_refusal(&catalog.update(active, 1, broken).unwrap_err());
    assert_eq!(
        refusal,
        "Stop list: extracted is not in its view list. Add it to the view list or remove the stop."
    );
    let stale = plain_refusal(&catalog.update(active, 7, definition("other")).unwrap_err());
    assert!(
        stale.starts_with("It changed while you were editing."),
        "{stale}"
    );
    for banned in ["configuration-set", "revision", "catalog", "executable"] {
        assert!(
            !refusal.contains(banned) && !stale.contains(banned),
            "{banned}"
        );
    }
}

#[test]
fn default_resolves_with_no_configuration_and_revises_none() {
    let (catalog, active) = catalog_with_active();
    let revision = catalog.find(active).unwrap().revision();
    let candidate = candidate_catalog(&catalog, ConfigurationSelection::Default, None).unwrap();
    assert_eq!(candidate.active_configuration_id(), None);
    assert_eq!(candidate.find(active).unwrap().revision(), revision);
    assert!(used_for_runs(&candidate, ConfigurationSelection::Default));

    let mut edited = definition("schematic");
    edited.stop_views.clear();
    let candidate = candidate_catalog(
        &candidate,
        ConfigurationSelection::Configuration(active),
        Some(&edited),
    )
    .unwrap();
    assert_eq!(candidate.active_configuration_id(), Some(active));
    assert!(candidate.find(active).unwrap().stop_views().is_empty());
}
