//! The workspace state's wire spellings and the catalog facts its facets
//! and operational states promise the reader.

use super::{
    ModelHubFacet, ModelsCatalogSortKey, ModelsOperationalState, ModelsTableSort,
    ModelsWorkbenchViewState, RSpicePartFacet,
};

/// Every class the catalog assigns is reachable from a class chip, or is
/// named here as one nothing offers.
///
/// A class no facet filters on hides its parts behind "All classes", which
/// is how the two foundation power MOSFETs and every p-channel MESFET
/// became unreachable — each by a different one-line omission, and neither
/// visible from the file that caused it. The five below have no chip
/// because no chip has been designed for them; they are a decision, not an
/// oversight, and designing one is what removes a name from this list.
#[test]
fn every_class_the_catalog_assigns_is_reachable_from_a_chip() {
    const NO_CHIP_OFFERS: &[&str] = &[
        "switch",
        "switch-current",
        "coupling",
        "distributed-rc",
        "piezo",
    ];

    for (token, device) in rspice_model_library::device_class::DEVICE_CLASS {
        if NO_CHIP_OFFERS.contains(device) {
            continue;
        }
        assert!(
            RSpicePartFacet::ALL
                .iter()
                .any(|facet| facet.device_filters().contains(device)),
            "'{token}' classifies to '{device}', which no class chip filters on"
        );
    }
}

/// A session saved under any spelling this facet has ever had still opens.
///
/// The facet is durable view state, so a rename is a compatibility event:
/// without the aliases, a session naming `updatable` deserializes as an
/// error and takes the whole `ModelsWorkbenchViewState` down with it — the
/// reader loses every selection in the workspace to a facet rename.
#[test]
fn every_spelling_this_facet_has_shipped_still_deserializes() {
    for (stored, expected) in [
        ("\"all\"", ModelHubFacet::All),
        ("\"needs-attention\"", ModelHubFacet::NeedsAttention),
        ("\"installed\"", ModelHubFacet::Installed),
        ("\"pinned\"", ModelHubFacet::Pinned),
        ("\"available\"", ModelHubFacet::Available),
        // Retired spellings. Both named an exception the ledger now
        // reports in its attention column, so both restore onto the facet
        // that collects exceptions rather than onto "all", which would
        // silently widen what the reader had narrowed.
        ("\"updatable\"", ModelHubFacet::NeedsAttention),
        ("\"incompatible\"", ModelHubFacet::NeedsAttention),
    ] {
        assert_eq!(
            serde_json::from_str::<ModelHubFacet>(stored).expect(stored),
            expected,
            "{stored} no longer opens"
        );
    }
    assert_eq!(
        serde_json::to_string(&ModelHubFacet::NeedsAttention).expect("serializes"),
        "\"needs-attention\"",
        "the spelling written today is the kebab-case one"
    );
}

/// A session written before the catalog could be sorted still opens.
///
/// The order is durable view state added after this struct shipped, so
/// every session on disk lacks the field. Without the `serde(default)` on
/// it the whole `ModelsWorkbenchViewState` fails to decode and the reader
/// loses every selection in the workspace to a table header.
#[test]
fn a_session_saved_before_the_catalog_could_be_sorted_still_opens() {
    let restored = serde_json::from_str::<ModelsWorkbenchViewState>(
        "{\"catalog_scope\":\"project\",\"catalog_query\":\"nch\"}",
    )
    .expect("a session with no recorded order still opens");
    assert_eq!(restored.catalog_query, "nch");
    assert_eq!(restored.catalog_sort, ModelsTableSort::default());
    assert_eq!(restored.catalog_sort.key, ModelsCatalogSortKey::Model);
    assert!(!restored.catalog_sort.descending);

    // Half a record is enough too: the direction alone restores onto the
    // default column rather than refusing the view state.
    let partial =
        serde_json::from_str::<ModelsTableSort>("{\"descending\":true}").expect("partial sort");
    assert_eq!(partial.key, ModelsCatalogSortKey::Model);
    assert!(partial.descending);
    assert_eq!(
        serde_json::to_string(&ModelsCatalogSortKey::UsedBy).expect("serializes"),
        "\"used-by\"",
        "the spelling written today is the kebab-case one"
    );
}

/// Clicking the ordered column reverses it; clicking another takes it over
/// ascending, which is the only reading of a first click on a new column.
#[test]
fn a_second_click_on_the_ordered_column_reverses_it() {
    let mut sort = ModelsTableSort::default();
    sort.toggle(ModelsCatalogSortKey::Status);
    assert_eq!(sort.key, ModelsCatalogSortKey::Status);
    assert!(!sort.descending);
    sort.toggle(ModelsCatalogSortKey::Status);
    assert!(sort.descending);
    sort.toggle(ModelsCatalogSortKey::Model);
    assert_eq!(sort.key, ModelsCatalogSortKey::Model);
    assert!(
        !sort.descending,
        "a column taken over inherits no direction from the one it replaced"
    );
}

#[test]
fn models_operational_state_registry_matches_the_mockup_contract() {
    assert_eq!(
        ModelsOperationalState::ALL.map(ModelsOperationalState::label),
        [
            "Ready",
            "Invalid input",
            "Execution error",
            "Read-only",
            "Offline",
            "Conflict",
            "Stale",
            "Permission",
            "Entitlement",
            "Cancelled",
            "Corrupted",
            "Recalled",
        ]
    );
}

#[test]
fn models_failures_map_to_actionable_operational_states() {
    use ModelsOperationalState as State;

    for (message, expected) in [
        ("Section name is required", State::InvalidInput),
        ("Project is read-only", State::ReadOnly),
        ("Catalog is offline", State::Offline),
        ("Duplicate model conflict", State::Conflict),
        ("Source drift detected", State::Stale),
        ("Access denied by permission policy", State::Permission),
        ("Pack license is restricted", State::Entitlement),
        ("Operation cancelled", State::Cancelled),
        ("Metadata is corrupted", State::Corrupted),
        ("malformed archive: truncated", State::Corrupted),
        ("Parser failed", State::ExecutionError),
        // The three trust refusals the Model Hub raises, in the exact
        // sentences it writes. A classifier that reads message text is
        // only as good as the texts it was checked against.
        (
            "the model hub offered catalog serial 40, which is stale beside serial 41 this \
             machine has already accepted; the held catalog was kept",
            State::Stale,
        ),
        (
            "the held catalog is stale: it expired at 2026-07-01T00:00:00Z, so the hub \
             offers nothing until it is refreshed",
            State::Stale,
        ),
        (
            "rspice-opamps 2.0.0 was recalled by its publisher: an invalid bias network was \
             published under a restricted licence",
            State::Recalled,
        ),
        // The three states this replaces — rollback, partial, recovered —
        // had classifiers and no producer: nothing that writes a models
        // receipt could say "rolled back", "partial" or "recovered", so
        // they were vocabulary the workspace could never reach. They now
        // land where every other unclassified failure lands.
        ("Transaction rolled back", State::ExecutionError),
        (
            "Import completed with partial results",
            State::ExecutionError,
        ),
    ] {
        assert_eq!(State::from_failure(message), expected, "{message}");
    }
}
