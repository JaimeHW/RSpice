//! What the embedded foundation library projects into the catalog.
//!
//! The cards are compiled in, so nothing about them is discovered at run time
//! and everything about them can be asserted exactly: how many there are, what
//! device family each one lands under, and that the default every unbound
//! schematic placement resolves to is a card the Models workspace can show.

use super::*;

#[test]
fn builtin_catalog_is_one_complete_foundation_library() {
    let mut manager = ModelLibraryManager::new();
    manager.load_builtin_models();

    assert_eq!(manager.library_count(), 1);
    let foundation = manager
        .get_library("RSpice Foundation")
        .expect("foundation library");
    assert_eq!(foundation.models.len(), 16);
    assert_eq!(foundation.top_level_models.len(), 16);
    assert_eq!(foundation.pack_id.as_deref(), Some("rspice-foundation"));
    let opamp = foundation
        .subcircuits
        .get("RSPICE_OPAMP")
        .expect("foundation op-amp interface");
    assert_eq!(opamp.ports, ["INP", "INN", "OUT"]);
    assert_eq!(foundation.source_authority, ModelSourceAuthority::BuiltIn);
}

/// Every foundation card lands in the catalog under the device family it was
/// authored as: the type token it declares, the family that token classifies
/// to, and the level.
///
/// All three, because each answers a different question and each has been
/// wrong on its own. The token is what the binding gate reads. The family is
/// what the shelf, the class chips and the symbol builder read, and it is
/// asserted *against the token* rather than against a fourth column, because
/// the defect it guards is the catalog classifying by something else: reading
/// the core index's coarse type instead left six of these sixteen cards as
/// `Other`. And the two kinds of card that share a token with a plain one —
/// SOI with bulk MOSFET, thermal bipolar with bipolar — are told apart by the
/// level alone.
///
/// Which family each token names is [`ModelType::from_name`]'s own table, and
/// is pinned beside it.
#[test]
fn every_foundation_card_carries_its_declared_family() {
    let mut manager = ModelLibraryManager::new();
    manager.load_builtin_models();
    let foundation = manager
        .get_library("RSpice Foundation")
        .expect("foundation library");

    for (name, spice_type, level) in [
        ("RSPICE_DIODE", "D", ModelLevel::Unknown),
        ("RSPICE_ZENER", "D", ModelLevel::Unknown),
        ("RSPICE_NPN", "NPN", ModelLevel::Unknown),
        ("RSPICE_PNP", "PNP", ModelLevel::Unknown),
        ("RSPICE_NPN_THERMAL", "NPN", ModelLevel::Vbic),
        ("RSPICE_PNP_THERMAL", "PNP", ModelLevel::Vbic),
        ("RSPICE_NJFET", "NJF", ModelLevel::Unknown),
        ("RSPICE_PJFET", "PJF", ModelLevel::Unknown),
        ("RSPICE_NMESFET", "NMF", ModelLevel::Unknown),
        ("RSPICE_PMESFET", "PMF", ModelLevel::Unknown),
        ("RSPICE_NMOS", "NMOS", ModelLevel::SpiceLevel1),
        ("RSPICE_PMOS", "PMOS", ModelLevel::SpiceLevel1),
        ("RSPICE_NVDMOS", "NVDMOS", ModelLevel::Vdmos),
        ("RSPICE_PVDMOS", "PVDMOS", ModelLevel::Vdmos),
        ("RSPICE_NMOS_SOI", "NMOS", ModelLevel::BsimSoi),
        ("RSPICE_PMOS_SOI", "PMOS", ModelLevel::BsimSoi),
    ] {
        let model = foundation
            .models
            .get(name)
            .unwrap_or_else(|| panic!("{name} is missing from the catalog"));
        assert_eq!(model.spice_type.as_deref(), Some(spice_type), "{name}");
        assert_eq!(
            model.model_type,
            ModelType::from_name(spice_type),
            "{name} is not classified by the token it declares"
        );
        assert_eq!(model.level, level, "{name}");
    }
}

/// A bare schematic placement and the catalog must agree on what the
/// foundation default for its family is: the netlist generator writes a card
/// name into the deck, and the Models workspace has to be able to show it.
#[test]
fn every_unbound_device_family_default_is_in_the_catalog() {
    let mut manager = ModelLibraryManager::new();
    manager.load_builtin_models();
    let foundation = manager
        .get_library("RSpice Foundation")
        .expect("foundation library");

    for family in rspice_core::library::FoundationDeviceFamily::ALL {
        let name = family.default_model_name();
        assert!(
            foundation.models.contains_key(name),
            "{family:?} default '{name}' is not shown in the Models workspace"
        );
    }
}
