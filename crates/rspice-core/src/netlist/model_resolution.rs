//! Builder-accurate existence checking for device model references.
//!
//! The circuit builder rejects a device whose model name resolves to nothing
//! at bind time — the "references unknown model" family of diagnostics. An
//! editor wants that answer before a run, and the raw source-map lint is not
//! it: it reads the deck text, so it reports elements inside subcircuits
//! nobody instantiates, and its allow-list admits names no bind path accepts.
//!
//! This module asks the builder's question against the builder's inputs: the
//! flattened instantiated topology, and the top-level `.MODEL` cards plus the
//! scoped cards flattening mints. Existence is the whole question here —
//! whether a card's type suits the element that names it stays the builder's
//! job, because only the builder knows what it is about to construct.
//!
//! The per-family fallbacks live in [`FamilyRule`]. [`builtin_model_names`]
//! is their union, for the lint that has no element family in hand; deriving
//! both from one table is what keeps the lint from drifting away from the
//! builder a second time.

use std::collections::HashSet;
use std::sync::OnceLock;

use super::{ElementKind, ModelDef, Netlist, flatten_netlist_with_models};
use crate::builtin_lib::{BJT_LIB, DIODE_LIB};

/// One instantiated device whose model name the builder would reject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedDeviceModelReference {
    /// Flattened instance name, e.g. `x1.m2`.
    pub element: String,
    /// Device family, spelled as the builder's own diagnostic spells it.
    pub device_kind: &'static str,
    /// Model name as the card writes it.
    pub model: String,
}

/// Builder-accurate model-existence check over the instantiated topology only.
///
/// Resolution per family, mirroring the bind-time sites one for one:
///
/// | Family | `.MODEL` card | Additional acceptance |
/// | --- | --- | --- |
/// | Resistor, Capacitor, Inductor | required | none |
/// | Jiles-Atherton inductor | required | none |
/// | Diode | required | embedded `D`/`DIODE` library cards |
/// | BJT | required | embedded `NPN`/`PNP` library cards |
/// | MOSFET | required, or a `<name>.<bin>` bin family | bare `NMOS`/`PMOS` |
/// | JFET | required | bare `NJF`/`PJF` |
/// | MESFET | required | bare `NMF`/`PMF`/`NHFET`/`PHFET` |
/// | Xyce memristor | required | none |
/// | Switches (`S`/`W`/generic) | required | none |
/// | Transmission line | required only when the card carries no `Z0` | none |
/// | Coupling | required | none |
/// | XSPICE code model | required | built-in code-model names |
///
/// A Verilog-A module named by a `.VERILOGA` directive resolves for every
/// family, matching the source-map lint.
///
/// The check is deliberately one-sided: it never reports a name the builder
/// accepts, and where the builder rejects a resolvable-looking name for a
/// reason other than existence — an uncovered bin, an incompatible card type —
/// it stays silent and leaves the diagnostic to the builder.
///
/// The one exception is `veriloga-builtins-base`, which lets a resistor,
/// diode, BJT or MOSFET name a generated Verilog-A model type directly and
/// bind with no card at all. That alias table lives in `engine::builder`,
/// twelve architectural layers above this one, so it cannot be consulted from
/// here; under that feature the check can report a card-free generated type
/// name the builder would have accepted.
pub fn unresolved_device_model_references(
    netlist: &Netlist,
) -> Result<Vec<UnresolvedDeviceModelReference>, String> {
    let flattened = flatten_netlist_with_models(netlist)
        .map_err(|error| format!("subcircuit flattening failed: {error}"))?;
    let declared = DeclaredModels::collect(netlist, &flattened.scoped_models);

    let mut unresolved = Vec::new();
    for element in &flattened.elements {
        let Some((rule, model)) = model_reference(&element.kind) else {
            continue;
        };
        if declared.resolves(rule, model) {
            continue;
        }
        unresolved.push(UnresolvedDeviceModelReference {
            element: element.name.clone(),
            device_kind: rule.kind,
            model: model.to_string(),
        });
    }
    Ok(unresolved)
}

/// Every model name the builder resolves without a `.MODEL` card.
///
/// The union of the [`FamilyRule`] fallbacks. The source-map lint works from
/// deck text and never knows which family a reference belongs to, so the union
/// is the strongest allow-list it can apply;
/// [`unresolved_device_model_references`] applies the exact per-family form.
pub(super) fn builtin_model_names() -> &'static HashSet<String> {
    static NAMES: OnceLock<HashSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| {
        let mut names = HashSet::new();
        for bare_type in BARE_MOS_TYPES
            .iter()
            .chain(BARE_JFET_TYPES)
            .chain(BARE_MESFET_TYPES)
        {
            names.insert((*bare_type).to_string());
        }
        names.extend(builtin_diode_model_names().iter().cloned());
        names.extend(builtin_bjt_model_names().iter().cloned());
        for code_model in crate::codemodels::BUILTIN_MODEL_NAMES {
            names.insert(code_model.to_ascii_uppercase());
        }
        names
    })
}

/// Type names a MOSFET accepts in place of a card.
const BARE_MOS_TYPES: &[&str] = &["NMOS", "PMOS"];

/// Type names a JFET accepts in place of a card.
const BARE_JFET_TYPES: &[&str] = &["NJF", "PJF"];

/// Type names a MESFET accepts in place of a card.
const BARE_MESFET_TYPES: &[&str] = &["NMF", "PMF", "NHFET", "PHFET"];

/// What the builder consults when no `.MODEL` card carries the name.
#[derive(Clone, Copy)]
enum ModelFallback {
    /// A card is the only resolution.
    CardOnly,
    /// Embedded diode library, as `builtin_diode_model_map` keys it.
    DiodeLibrary,
    /// Embedded bipolar library, as `builtin_bjt_model_map` keys it.
    BjtLibrary,
    /// Bare type names, uppercase.
    BareTypes(&'static [&'static str]),
    /// XSPICE code models compiled into this build.
    CodeModel,
}

/// How one element family resolves a model name at bind time.
#[derive(Clone, Copy)]
struct FamilyRule {
    kind: &'static str,
    fallback: ModelFallback,
    /// MOSFET names may address a bin family whose cards are `<name>.<bin>`.
    bin_family: bool,
}

const fn card_only(kind: &'static str) -> FamilyRule {
    FamilyRule {
        kind,
        fallback: ModelFallback::CardOnly,
        bin_family: false,
    }
}

const fn with_fallback(kind: &'static str, fallback: ModelFallback) -> FamilyRule {
    FamilyRule {
        kind,
        fallback,
        bin_family: false,
    }
}

/// The model name one element references, with the rule that resolves it.
///
/// A transmission line names a model optionally and only needs it when the
/// card carries no `Z0`, so an inline `T` line with a stray model name is not
/// a bind-time failure and is not reported here.
fn model_reference(kind: &ElementKind) -> Option<(FamilyRule, &str)> {
    match kind {
        ElementKind::Resistor {
            model: Some(model), ..
        } => Some((card_only("Resistor"), model)),
        ElementKind::Capacitor {
            model: Some(model), ..
        } => Some((card_only("Capacitor"), model)),
        ElementKind::Inductor {
            model: Some(model), ..
        } => Some((card_only("Inductor"), model)),
        ElementKind::JilesAthertonInductor { model, .. } => {
            Some((card_only("Jiles-Atherton inductor"), model))
        }
        ElementKind::Diode { model, .. } => Some((
            with_fallback("Diode", ModelFallback::DiodeLibrary),
            model.as_str(),
        )),
        ElementKind::Bjt { model, .. } => Some((
            with_fallback("BJT", ModelFallback::BjtLibrary),
            model.as_str(),
        )),
        ElementKind::Mosfet { model, .. } => Some((
            FamilyRule {
                kind: "MOSFET",
                fallback: ModelFallback::BareTypes(BARE_MOS_TYPES),
                bin_family: true,
            },
            model.as_str(),
        )),
        ElementKind::Jfet { model, .. } => Some((
            with_fallback("JFET", ModelFallback::BareTypes(BARE_JFET_TYPES)),
            model.as_str(),
        )),
        ElementKind::Mesfet { model, .. } => Some((
            with_fallback("MESFET", ModelFallback::BareTypes(BARE_MESFET_TYPES)),
            model.as_str(),
        )),
        ElementKind::XyceMemristor { model, .. } => Some((card_only("Xyce memristor"), model)),
        ElementKind::VSwitch { model, .. } => Some((card_only("Voltage-controlled switch"), model)),
        ElementKind::ISwitch { model, .. } => Some((card_only("Current-controlled switch"), model)),
        ElementKind::GenericSwitch { model, .. } => Some((card_only("Generic switch"), model)),
        ElementKind::TransmissionLine {
            z0: None,
            model: Some(model),
            ..
        } => Some((card_only("Transmission line"), model)),
        ElementKind::Coupling {
            model: Some(model), ..
        } => Some((card_only("Nonlinear magnetic coupling"), model)),
        ElementKind::Xspice { model, .. } => Some((
            with_fallback("XSPICE code model", ModelFallback::CodeModel),
            model.as_str(),
        )),
        _ => None,
    }
}

/// The model names one build sees: top-level cards, the scoped cards
/// flattening minted, and the Verilog-A modules the deck pulled in.
struct DeclaredModels {
    names: HashSet<String>,
    /// Prefixes addressable as a MOSFET bin family, from every `<family>.<bin>`
    /// card name.
    bin_families: HashSet<String>,
}

impl DeclaredModels {
    fn collect(netlist: &Netlist, scoped_models: &[ModelDef]) -> Self {
        let mut names = HashSet::new();
        let mut bin_families = HashSet::new();
        for model in netlist.models.iter().chain(scoped_models) {
            let name = model.name.to_ascii_uppercase();
            collect_bin_families(&name, &mut bin_families);
            names.insert(name);
        }
        for include in &netlist.veriloga_includes {
            if let Some(model_name) = &include.model_name {
                names.insert(model_name.to_ascii_uppercase());
            }
        }
        Self {
            names,
            bin_families,
        }
    }

    fn resolves(&self, rule: FamilyRule, model: &str) -> bool {
        let name = model.to_ascii_uppercase();
        if self.names.contains(&name) {
            return true;
        }
        if rule.bin_family && self.bin_families.contains(&name) {
            return true;
        }
        match rule.fallback {
            ModelFallback::CardOnly => false,
            ModelFallback::DiodeLibrary => builtin_diode_model_names().contains(&name),
            ModelFallback::BjtLibrary => builtin_bjt_model_names().contains(&name),
            ModelFallback::BareTypes(types) => types.contains(&name.as_str()),
            ModelFallback::CodeModel => crate::codemodels::is_builtin_model_name(model),
        }
    }
}

/// Record every prefix of an uppercased card name that a MOSFET may address as
/// a bin family: `nch.1.2` is reachable as `NCH` and as `NCH.1`. A trailing
/// dot names no bin, so it contributes nothing.
fn collect_bin_families(card_name: &str, families: &mut HashSet<String>) {
    let mut cut = 0;
    while let Some(offset) = card_name[cut..].find('.') {
        let end = cut + offset;
        if end > 0 && end + 1 < card_name.len() {
            families.insert(card_name[..end].to_string());
        }
        cut = end + 1;
    }
}

fn builtin_diode_model_names() -> &'static HashSet<String> {
    static NAMES: OnceLock<HashSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| library_model_names(DIODE_LIB, &["D", "DIODE"], "diode"))
}

fn builtin_bjt_model_names() -> &'static HashSet<String> {
    static NAMES: OnceLock<HashSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| library_model_names(BJT_LIB, &["NPN", "PNP"], "transistor"))
}

/// Card names of the given types in an embedded library.
///
/// Parsed and filtered by card type rather than text-scanned, because that is
/// how `builtin_diode_model_map` and `builtin_bjt_model_map` key the parameter
/// maps the builder applies. A card the builder would not key on must not read
/// as resolvable here.
fn library_model_names(library: &str, model_types: &[&str], description: &str) -> HashSet<String> {
    let Ok(netlist) = super::parse_netlist(library) else {
        log::warn!("Failed to parse embedded {description} library for fallback model names");
        return HashSet::new();
    };
    netlist
        .models
        .iter()
        .filter(|model| {
            model_types
                .iter()
                .any(|model_type| model.model_type.eq_ignore_ascii_case(model_type))
        })
        .map(|model| model.name.to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{UnresolvedDeviceModelReference, unresolved_device_model_references};
    use crate::netlist::Netlist;

    fn unresolved(deck: &str) -> Vec<UnresolvedDeviceModelReference> {
        let netlist = Netlist::parse(deck).expect("deck parses");
        unresolved_device_model_references(&netlist).expect("topology flattens")
    }

    fn flagged_models(deck: &str) -> Vec<String> {
        unresolved(deck)
            .into_iter()
            .map(|reference| reference.model)
            .collect()
    }

    #[test]
    fn declared_model_card_resolves() {
        let deck = "model resolution\n\
            M1 d g s b nch W=1u L=1u\n\
            .model nch nmos\n\
            .end\n";

        assert_eq!(unresolved(deck), Vec::new());
    }

    #[test]
    fn unknown_model_on_an_instantiated_device_is_flagged() {
        let deck = "model resolution\n\
            .subckt cell d g s b\n\
            M2 d g s b missing_n W=1u L=1u\n\
            .ends\n\
            X1 d g s b cell\n\
            .end\n";

        let unresolved = unresolved(deck);
        assert_eq!(unresolved.len(), 1, "{unresolved:?}");
        assert!(
            unresolved[0].element.eq_ignore_ascii_case("x1.m2"),
            "{:?}",
            unresolved[0].element
        );
        assert_eq!(unresolved[0].device_kind, "MOSFET");
        assert!(
            unresolved[0].model.eq_ignore_ascii_case("missing_n"),
            "{:?}",
            unresolved[0].model
        );
    }

    #[test]
    fn bare_type_names_follow_the_builder() {
        // A MOSFET, JFET or MESFET resolves its own type name with no card.
        // A diode or BJT has no such fallback: the builder consults only the
        // embedded library, which carries part names.
        let accepted = "model resolution\n\
            M1 d g s b NMOS W=1u L=1u\n\
            J1 jd jg js NJF\n\
            Z1 zd zg zs NMF\n\
            .end\n";
        assert_eq!(unresolved(accepted), Vec::new());

        let rejected = "model resolution\n\
            D1 a 0 d\n\
            Q1 c b e npn\n\
            .end\n";
        let flagged = flagged_models(rejected);
        assert_eq!(flagged.len(), 2, "{flagged:?}");
        assert!(flagged[0].eq_ignore_ascii_case("d"), "{flagged:?}");
        assert!(flagged[1].eq_ignore_ascii_case("npn"), "{flagged:?}");
    }

    #[test]
    fn embedded_library_cards_resolve_for_their_own_family() {
        let deck = "model resolution\n\
            D1 a 0 1N4148\n\
            Q1 c b e 2N3904\n\
            .end\n";
        assert_eq!(unresolved(deck), Vec::new());

        // The embedded libraries are per-family fallbacks, not one pool: the
        // builder's diode path never reads the bipolar map. The MOSFET and
        // JFET libraries back no bind path at all.
        let crossed = "model resolution\n\
            D2 a 0 2N3904\n\
            M1 d g s b 2N7000 W=1u L=1u\n\
            J1 jd jg js 2N3819\n\
            .end\n";
        let flagged = flagged_models(crossed);
        assert_eq!(flagged.len(), 3, "{flagged:?}");
    }

    #[test]
    fn uninstantiated_subcircuit_bodies_are_not_checked() {
        let deck = "model resolution\n\
            .subckt unused a k\n\
            D1 a k missing_d\n\
            .ends\n\
            R1 a 0 1k\n\
            .end\n";

        assert_eq!(unresolved(deck), Vec::new());
    }

    #[test]
    fn scoped_model_minted_by_parameterized_expansion_resolves() {
        let deck = "model resolution\n\
            .subckt cell a k scale=1\n\
            D1 a k dmod\n\
            .model dmod d (is='1e-14*scale')\n\
            .ends\n\
            X1 in 0 cell scale=2\n\
            V1 in 0 1\n\
            .end\n";

        assert_eq!(unresolved(deck), Vec::new());
    }

    #[test]
    fn model_names_match_case_insensitively() {
        let deck = "model resolution\n\
            D1 a 0 DMOD\n\
            Q1 c b e 2n3904\n\
            M1 d g s b nmos W=1u L=1u\n\
            .model dmod d\n\
            .end\n";

        assert_eq!(unresolved(deck), Vec::new());
    }
}
