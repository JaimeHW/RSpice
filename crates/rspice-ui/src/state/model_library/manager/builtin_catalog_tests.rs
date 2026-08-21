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
/// authored as: the declared type token, the model type it classifies to, and
/// the level.
///
/// All three, because each answers a different question and each has been
/// wrong on its own. The token is what the binding gate reads; the model type
/// is what the shelf, the facets and the symbol builder read, and every JFET,
/// MESFET and VDMOS card here arrived as `Other` until the catalog classified
/// from the token; and the two families that share a token with a plain
/// MOSFET — SOI, and the thermal bipolars with their bipolar twins — are told
/// apart by the level alone.
#[test]
fn every_foundation_card_carries_its_declared_family() {
    let mut manager = ModelLibraryManager::new();
    manager.load_builtin_models();
    let foundation = manager
        .get_library("RSpice Foundation")
        .expect("foundation library");

    for (name, spice_type, model_type, level) in [
        ("RSPICE_DIODE", "D", ModelType::Diode, ModelLevel::Unknown),
        ("RSPICE_ZENER", "D", ModelType::Diode, ModelLevel::Unknown),
        ("RSPICE_NPN", "NPN", ModelType::Npn, ModelLevel::Unknown),
        ("RSPICE_PNP", "PNP", ModelType::Pnp, ModelLevel::Unknown),
        ("RSPICE_NPN_THERMAL", "NPN", ModelType::Npn, ModelLevel::Vbic),
        ("RSPICE_PNP_THERMAL", "PNP", ModelType::Pnp, ModelLevel::Vbic),
        ("RSPICE_NJFET", "NJF", ModelType::Njfet, ModelLevel::Unknown),
        ("RSPICE_PJFET", "PJF", ModelType::Pjfet, ModelLevel::Unknown),
        (
            "RSPICE_NMESFET",
            "NMF",
            ModelType::Nmesfet,
            ModelLevel::Unknown,
        ),
        (
            "RSPICE_PMESFET",
            "PMF",
            ModelType::Pmesfet,
            ModelLevel::Unknown,
        ),
        ("RSPICE_NMOS", "NMOS", ModelType::Nmos, ModelLevel::SpiceLevel1),
        ("RSPICE_PMOS", "PMOS", ModelType::Pmos, ModelLevel::SpiceLevel1),
        (
            "RSPICE_NVDMOS",
            "NVDMOS",
            ModelType::NVdmos,
            ModelLevel::Vdmos,
        ),
        (
            "RSPICE_PVDMOS",
            "PVDMOS",
            ModelType::PVdmos,
            ModelLevel::Vdmos,
        ),
        (
            "RSPICE_NMOS_SOI",
            "NMOS",
            ModelType::Nmos,
            ModelLevel::BsimSoi,
        ),
        (
            "RSPICE_PMOS_SOI",
            "PMOS",
            ModelType::Pmos,
            ModelLevel::BsimSoi,
        ),
    ] {
        let model = foundation
            .models
            .get(name)
            .unwrap_or_else(|| panic!("{name} is missing from the catalog"));
        assert_eq!(model.spice_type.as_deref(), Some(spice_type), "{name}");
        assert_eq!(model.model_type, model_type, "{name}");
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
