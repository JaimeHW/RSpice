//! The workspace state's wire spellings and the catalog facts its facets
//! promise the reader.

use super::{ModelsCatalogSortKey, ModelsTableSort, RSpicePartFacet};

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
