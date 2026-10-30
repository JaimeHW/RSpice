//! The Configuration page's selection, over a catalog built in place.

use super::*;
use crate::state::{
    CellViewRef, ConfigurationBlackBoxPolicy, ConfigurationModelProfile,
    ConfigurationSetDefinition, UnresolvedBindingPolicy,
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
    assert!(page.refusal.is_some(), "the same selection keeps its refusal");
    page.select(ConfigurationSelection::Configuration(
        ConfigurationSetId::new(),
    ));
    assert!(page.refusal.is_none());
}

#[test]
fn the_built_in_order_matches_what_the_resolver_searches() {
    assert_eq!(
        built_in::default_view_search_order("schematic", false),
        ["schematic", "extracted", "spice"]
    );
    assert_eq!(
        built_in::default_view_search_order("schematic", true),
        ["schematic", "spice"]
    );
    assert_eq!(built_in::default_view_search_order("spice", false), ["spice"]);
    assert_eq!(
        built_in::default_view_search_order("symbol", false),
        ["schematic", "extracted", "spice"]
    );
}
