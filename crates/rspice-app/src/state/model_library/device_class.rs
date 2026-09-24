//! The catalog's device class for one model card.
//!
//! The shipped corpus index classifies a `.model` card by its type token, and
//! `tools/models/build_manifest.py` owns that table. A card this project
//! parsed for itself never went through the generator, so the same token has
//! to be classified again — and every surface that does so has to reach the
//! same answer, or the class a part is filed under depends on which list it
//! appears in.
//!
//! That is what had happened. The Models shelf classified a parsed card by
//! its token and got `mosfet-n`; the Model Hub's part index classified the
//! same card by rendering its model type's *display name* and got `NMOS`. One
//! search box over both, one set of class chips over both, and two
//! vocabularies underneath: searching `mosfet-n` found the pack's parts and
//! not the project's, and searching `NMOS` found the project's and not the
//! pack's.
//!
//! [`DEVICE_CLASS`] is the mirror of the generator's table, and it lives here
//! rather than beside either reader so there is one of it. A test below reads
//! the generator's own source and refuses to let the two drift.

use super::{DeviceModel, ModelType};

/// The `.MODEL` type token to canonical device class map, mirrored from
/// `tools/models/build_manifest.py`.
///
/// Kept in the generator's own order and spelling so the drift test can
/// compare them literally.
///
/// One class per family rather than per spelling: every VDMOS token answers
/// `mosfet-vdmos`, because a class the client offers no chip for is a class
/// that hides its parts. `NVDMOS` and `PVDMOS` used to stand for themselves,
/// which is exactly how the two foundation power MOSFETs ended up under
/// classes no facet filtered on.
pub(crate) const DEVICE_CLASS: &[(&str, &str)] = &[
    ("d", "diode"),
    ("npn", "bjt-npn"),
    ("pnp", "bjt-pnp"),
    ("lpnp", "bjt-pnp"),
    ("njf", "jfet-n"),
    ("pjf", "jfet-p"),
    ("nmos", "mosfet-n"),
    ("pmos", "mosfet-p"),
    ("nmf", "mesfet-n"),
    ("pmf", "mesfet-p"),
    ("nsw", "switch"),
    ("sw", "switch"),
    ("vswitch", "switch"),
    ("csw", "switch-current"),
    ("iswitch", "switch-current"),
    ("r", "resistor"),
    ("res", "resistor"),
    ("c", "capacitor"),
    ("cap", "capacitor"),
    ("l", "inductor"),
    ("ind", "inductor"),
    ("k", "coupling"),
    ("core", "magnetic-core"),
    ("vdmos", "mosfet-vdmos"),
    ("nvdmos", "mosfet-vdmos"),
    ("pvdmos", "mosfet-vdmos"),
    ("vdmosn", "mosfet-vdmos"),
    ("vdmosp", "mosfet-vdmos"),
    ("txl", "transmission-line"),
    ("cpl", "transmission-line"),
    ("ltra", "transmission-line"),
    ("urc", "distributed-rc"),
    ("pzt", "piezo"),
];

/// The class the catalog files a subcircuit under.
///
/// The generator's own word, so a project's subcircuits and a pack's land on
/// the same chip.
pub(crate) const SUBCIRCUIT_CLASS: &str = "subckt";

/// The catalog's device class for one parsed card.
///
/// The declared type token is the generator's input, so it is this function's
/// input too. A card that carries no token at all — a synthesized or partially
/// migrated one — is classified from the type parsing settled on instead,
/// which names the same families in a different spelling.
pub(crate) fn card_device(card: &DeviceModel) -> String {
    let token = card
        .spice_type
        .as_deref()
        .map(str::trim)
        .unwrap_or_default()
        .to_ascii_lowercase();
    if token.is_empty() {
        return parsed_type_device(card.model_type).to_owned();
    }
    device_class(&token)
}

/// The generator's classification of one `.MODEL` type token.
pub(crate) fn device_class(token: &str) -> String {
    if let Some((_, device)) = DEVICE_CLASS
        .iter()
        .find(|(candidate, _)| *candidate == token)
    {
        return (*device).to_owned();
    }
    // PSpice digital primitives and the XSPICE behavioural family are folded
    // into two classes rather than enumerated, exactly as the generator does.
    if token.starts_with("d_") {
        return "digital".to_owned();
    }
    if token.starts_with('u') {
        return "digital-behavioral".to_owned();
    }
    if token.is_empty() {
        return "unknown".to_owned();
    }
    token.to_owned()
}

/// The class a card whose token was lost still belongs to.
///
/// [`ModelType`] is what parsing concluded from the same token, so this names
/// the same families the generator's table does. The variants that carry no
/// device family — `Rf`, `Other` — answer `unknown` rather than inventing one,
/// which keeps them off every class chip but on the shelf.
pub(crate) const fn parsed_type_device(model_type: ModelType) -> &'static str {
    match model_type {
        ModelType::Nmos => "mosfet-n",
        ModelType::Pmos => "mosfet-p",
        ModelType::Npn => "bjt-npn",
        ModelType::Pnp => "bjt-pnp",
        ModelType::Njfet => "jfet-n",
        ModelType::Pjfet => "jfet-p",
        ModelType::Nmesfet => "mesfet-n",
        ModelType::Pmesfet => "mesfet-p",
        // One class for both polarities, because the generator's table has
        // one. Splitting it here would invent a class no shipped index row
        // carries, and the chip built from it would find nothing.
        ModelType::NVdmos | ModelType::PVdmos => "mosfet-vdmos",
        ModelType::Resistor => "resistor",
        ModelType::Capacitor => "capacitor",
        ModelType::Inductor => "inductor",
        ModelType::Diode | ModelType::Varactor | ModelType::Esd => "diode",
        ModelType::Rf | ModelType::Other => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The mirror and the generator classify the same tokens the same way.
    ///
    /// A client whose class chips disagree with the shipped index would put a
    /// project's diodes and the library's diodes under different chips, which
    /// is worse than having no chips at all. The generator is Python and
    /// cannot be called from here, so its table is read instead — a mirror
    /// nobody checks is a mirror that has already drifted.
    #[test]
    fn the_device_class_mirror_matches_the_generator() {
        const GENERATOR: &str = include_str!("../../../../../tools/models/build_manifest.py");

        let table = GENERATOR
            .split_once("DEVICE_CLASS = {")
            .expect("the generator declares DEVICE_CLASS")
            .1
            .split_once('}')
            .expect("the DEVICE_CLASS literal is closed")
            .0;
        let declared = table
            .lines()
            .filter_map(|line| line.trim().strip_suffix(','))
            .filter_map(|entry| entry.split_once(':'))
            .map(|(token, device)| (unquote(token), unquote(device)))
            .collect::<Vec<_>>();

        assert!(
            declared.len() >= 25,
            "only {} entries were read out of the generator; the literal's shape has changed",
            declared.len()
        );
        assert_eq!(
            declared,
            DEVICE_CLASS
                .iter()
                .map(|(token, device)| ((*token).to_owned(), (*device).to_owned()))
                .collect::<Vec<_>>(),
            "DEVICE_CLASS here no longer matches tools/models/build_manifest.py"
        );
    }

    fn unquote(value: &str) -> String {
        value.trim().trim_matches('"').trim_matches('\'').to_owned()
    }

    #[test]
    fn a_parsed_card_lands_on_the_class_its_token_names() {
        let card = |spice_type: Option<&str>, model_type| {
            let mut card = DeviceModel::new("X", model_type);
            card.spice_type = spice_type.map(str::to_owned);
            card
        };
        assert_eq!(card_device(&card(Some("NJF"), ModelType::Njfet)), "jfet-n");
        assert_eq!(card_device(&card(Some("D"), ModelType::Diode)), "diode");
        assert_eq!(
            card_device(&card(Some("d_and"), ModelType::Other)),
            "digital"
        );
        // A token nothing classifies stands for itself rather than becoming a
        // class the card never claimed.
        assert_eq!(
            card_device(&card(Some("psp103"), ModelType::Other)),
            "psp103"
        );
        // Every VDMOS spelling is one class, so a foundation power MOSFET is
        // found under the same chip an ngspice one is.
        for token in ["VDMOS", "NVDMOS", "PVDMOS", "VDMOSN", "VDMOSP"] {
            assert_eq!(
                card_device(&card(Some(token), ModelType::Other)),
                "mosfet-vdmos",
                "token '{token}'"
            );
        }
        // And a card that lost its token is still filed under its family.
        assert_eq!(card_device(&card(None, ModelType::Nmos)), "mosfet-n");
        assert_eq!(card_device(&card(None, ModelType::PVdmos)), "mosfet-vdmos");
        assert_eq!(card_device(&card(None, ModelType::Other)), "unknown");
    }
}
