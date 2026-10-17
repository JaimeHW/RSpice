//! Every card the deck writes because a component is drawn.
//!
//! Most components write one card, their own element, headed with
//! [`Component::written_element_name`]. Some make the deck write others beside
//! it or in its place: a current-controlled source or switch with no
//! controlling element of its own senses its control current through a 0 V
//! source the deck writes for it, a transformer is written as two winding
//! inductors and their coupling, with no card under its own name, and an
//! independent source's series and parallel parasitics are written as a
//! resistor or capacitor of their own beside it. A coupling
//! is written once per winding set, whichever drawn component asks for it, so
//! which couplings the deck writes, and under what names, is a question about
//! the whole cellview.
//!
//! The netlister writes each of these cards under the name given here, and the
//! design check judges every one of them, so a name the deck would write twice
//! is refused before the engine sees it.

use super::{Component, ComponentType};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

/// What the deck writes before a component's name to name the 0 V source it
/// senses the component's control current through.
const SENSE_SOURCE_PREFIX: &str = "VSENSE_";

/// What the deck writes between an element letter and an independent source's
/// id to name a parasitic it writes beside that source.
const SOURCE_PARASITIC_STEM: &str = "__RSPICE_SOURCE_";

/// What a written card is to the component that makes the deck write it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardRole {
    /// The component's own element.
    Element,
    /// The 0 V source a current-controlled source or switch senses its
    /// control current through.
    SenseSource,
    /// A transformer's primary winding.
    PrimaryWinding,
    /// A transformer's secondary winding.
    SecondaryWinding,
    /// A mutual-inductance card a component asks for without being one: a
    /// transformer's own, or the one an inductor's `coupled_to` names.
    Coupling,
    /// The resistor a voltage source's `rs` writes in series with its `+`
    /// terminal.
    SeriesResistance,
    /// The resistor an independent source's `rp` writes across its terminals.
    ParallelResistance,
    /// The capacitor an independent source's `cpar` writes across its
    /// terminals.
    ParallelCapacitance,
}

/// One card the deck writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrittenCard {
    /// The name the deck heads the card with.
    pub name: String,
    /// What the card is to the component that makes the deck write it.
    pub role: CardRole,
}

/// The cards the deck writes for a transformer in place of one of its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformerCards {
    /// The primary winding's inductor.
    pub primary: String,
    /// The secondary winding's inductor.
    pub secondary: String,
    /// The coupling between the two.
    pub coupling: String,
}

/// One parasitic element the deck writes beside an independent source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceParasitic {
    /// The name the deck heads the element's card with.
    pub name: String,
    /// The element's value as authored, without surrounding blanks.
    pub value: String,
}

/// The parasitic elements the deck writes beside an independent source.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceParasitics {
    /// A voltage source's `rs`, in series with its `+` terminal: the source
    /// then drives a node of its own behind the resistor.
    pub series_resistance: Option<SourceParasitic>,
    /// `rp`, across the source's terminals.
    pub parallel_resistance: Option<SourceParasitic>,
    /// `cpar`, across the source's terminals.
    pub parallel_capacitance: Option<SourceParasitic>,
}

impl Component {
    /// The name of the 0 V source the deck writes beside this component to
    /// sense its control current, or `None` when it writes none.
    ///
    /// A CCVS or CCCS that names no controlling element in `vref`, and every
    /// current-controlled switch, has its control terminals wired through
    /// such a source. Its name is built on the emitted name rather than the
    /// written one, so a limited `H1`, written `B1`, senses through
    /// `VSENSE_H1`, which is how every deck already written spells it.
    pub fn sense_source_name(&self) -> Option<String> {
        let senses = match self.kind {
            ComponentType::Ccvs | ComponentType::Cccs => {
                crate::parameters::parse_params_string(&self.params)
                    .get("vref")
                    .is_none_or(|reference| reference.trim().is_empty())
            }
            ComponentType::ISwitch => true,
            _ => false,
        };
        senses.then(|| format!("{SENSE_SOURCE_PREFIX}{}", self.emitted_instance_name()))
    }

    /// The cards the deck writes for a transformer, or `None` for any other
    /// component.
    ///
    /// They are named from the drawn name with every character other than an
    /// ASCII letter or digit made `_` and the underscores at either end
    /// dropped — the component id when nothing is left — so `T1` writes
    /// `LT1_PRI`, `LT1_SEC` and `KT1`.
    pub fn transformer_cards(&self) -> Option<TransformerCards> {
        if self.kind != ComponentType::Transformer {
            return None;
        }
        let base = identifier(&self.spice_instance_name());
        let base = base.trim_matches('_');
        let base = if base.is_empty() {
            self.id.to_string()
        } else {
            base.to_owned()
        };
        let winding = ComponentType::Inductor.spice_prefix();
        Some(TransformerCards {
            primary: format!("{winding}{base}_PRI"),
            secondary: format!("{winding}{base}_SEC"),
            coupling: format!("{}{base}", ComponentType::CoupledInductor.spice_prefix()),
        })
    }

    /// The parasitic elements the deck writes beside this independent
    /// source, or `None` for any other component.
    ///
    /// A voltage source's `rs` is written in series with its `+` terminal, and
    /// any independent source's `rp` and `cpar` across its terminals. Each is
    /// named from the component id rather than its drawn name, so a source
    /// with id 7 and `rs=50` writes `R__RSPICE_SOURCE_7_RS`. A blank or zero
    /// value writes nothing — except that a zero `rp` is a short across the
    /// source, which the netlister refuses rather than leaves out.
    pub fn source_parasitics(&self) -> Option<SourceParasitics> {
        if !self.kind.is_source() {
            return None;
        }
        let drives_voltage = match self.kind.spice_prefix() {
            "V" => true,
            "I" => false,
            _ => return None,
        };
        let params = crate::parameters::parse_params_string(&self.params);
        let written = |key: &str, element: ComponentType, suffix: &str| {
            let value = params.get(key)?.trim();
            (!crate::parameters::source_value_is_zero_or_blank(value)).then(|| SourceParasitic {
                name: format!(
                    "{}{SOURCE_PARASITIC_STEM}{}_{suffix}",
                    element.spice_prefix(),
                    self.id
                ),
                value: value.to_owned(),
            })
        };
        Some(SourceParasitics {
            series_resistance: drives_voltage
                .then(|| written("rs", ComponentType::Resistor, "RS"))
                .flatten(),
            parallel_resistance: written("rp", ComponentType::Resistor, "RP"),
            parallel_capacitance: written("cpar", ComponentType::Capacitor, "CPAR"),
        })
    }

    /// The cards this component makes the deck write on its own account: its
    /// own element where it has one, then the cards written beside or in
    /// place of it. A coupling is a question about the cellview, answered by
    /// [`coupling_cards`].
    fn own_cards(&self) -> Vec<WrittenCard> {
        let card = |name: String, role| WrittenCard { name, role };
        if let Some(transformer) = self.transformer_cards() {
            return vec![
                card(transformer.primary, CardRole::PrimaryWinding),
                card(transformer.secondary, CardRole::SecondaryWinding),
                card(transformer.coupling, CardRole::Coupling),
            ];
        }
        if self.kind.spice_prefix().is_empty() || self.kind == ComponentType::CoupledInductor {
            return Vec::new();
        }
        let mut cards = vec![card(self.written_element_name(), CardRole::Element)];
        cards.extend(
            self.sense_source_name()
                .map(|name| card(name, CardRole::SenseSource)),
        );
        if let Some(parasitics) = self.source_parasitics() {
            cards.extend(
                [
                    (parasitics.series_resistance, CardRole::SeriesResistance),
                    (parasitics.parallel_resistance, CardRole::ParallelResistance),
                    (
                        parasitics.parallel_capacitance,
                        CardRole::ParallelCapacitance,
                    ),
                ]
                .into_iter()
                .filter_map(|(parasitic, role)| Some(card(parasitic?.name, role))),
            );
        }
        cards
    }
}

/// Every card the deck writes for the components of one cellview, by the id
/// of the component that makes the deck write it.
///
/// A coupling is listed once, under the component the deck writes it for: the
/// drawn coupling, or the first inductor whose `coupled_to` asks for it. A
/// drawn coupling the deck does not write — a second one of a winding set, or
/// one it refuses — is listed under nothing.
pub fn written_cards(components: &[Component]) -> HashMap<u64, Vec<WrittenCard>> {
    let mut written = HashMap::<u64, Vec<WrittenCard>>::new();
    for component in components {
        let cards = component.own_cards();
        if !cards.is_empty() {
            written.entry(component.id).or_default().extend(cards);
        }
    }
    for coupling in coupling_cards(components).cards {
        written
            .entry(coupling.component)
            .or_default()
            .push(WrittenCard {
                name: coupling.name,
                role: coupling.role,
            });
    }
    written
}

/// The mutual-inductance cards a cellview's couplings make the deck write.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CouplingCards {
    /// The cards, in the order the deck writes them.
    pub cards: Vec<CouplingCard>,
    /// Why a coupling the drawing asks for is not written, in the order the
    /// cellview's components ask.
    pub refusals: Vec<CouplingRefusal>,
}

/// One mutual-inductance card the deck writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouplingCard {
    /// The component the card is written for.
    pub component: u64,
    /// [`CardRole::Element`] for a drawn coupling, which is the card;
    /// [`CardRole::Coupling`] for one an inductor's `coupled_to` asks for.
    pub role: CardRole,
    /// The name the card is headed with: a drawn coupling's written name, and
    /// for one an inductor asks for, `K` before the windings' written names,
    /// each with every character other than an ASCII letter or digit made
    /// `_`, sorted and joined with `_`.
    pub name: String,
    /// The coupled inductors' written names, in the order the card lists them.
    pub windings: Vec<String>,
    /// The coupling coefficient as authored.
    pub coefficient: String,
}

/// Where a coupling of a winding set is asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CouplingSource {
    /// A drawn coupling, by its drawn name.
    Drawn(String),
    /// An inductor's `coupled_to`, by the inductor's drawn name.
    Requested(String),
}

impl fmt::Display for CouplingSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Drawn(coupling) => write!(formatter, "coupling '{coupling}'"),
            Self::Requested(inductor) => write!(formatter, "inductor '{inductor}'"),
        }
    }
}

/// Why the deck writes no card for a coupling the drawing asks for. Each one
/// refuses the netlist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CouplingRefusal {
    /// One name a coupling could name a winding by answers to two inductors.
    AmbiguousWinding { reference: String },
    /// A drawn coupling names fewer than two inductors.
    TooFewWindings { coupling: String },
    /// A drawn coupling has no coefficient.
    MissingCoefficient { coupling: String },
    /// A drawn coupling's coefficient is a number outside -1..=1.
    InvalidCoefficient {
        coupling: String,
        coefficient: String,
    },
    /// An inductor has a coupling factor and no `coupled_to`.
    FactorWithoutTarget { inductor: String },
    /// An inductor names a `coupled_to` and no coupling factor.
    TargetWithoutFactor { inductor: String, target: String },
    /// An inductor's coupling factor is not a number.
    NonNumericFactor { inductor: String, factor: String },
    /// An inductor's coupling factor is outside -1..=1.
    InvalidFactor { inductor: String, factor: String },
    /// A coupling names an inductor the cellview does not have.
    UnknownWinding { coupling: String, reference: String },
    /// A coupling names one inductor twice.
    RepeatedWinding { coupling: String, reference: String },
    /// Two couplings of one winding set disagree on the coefficient.
    Conflicting {
        windings: String,
        written: String,
        written_by: CouplingSource,
        coefficient: String,
        source: CouplingSource,
    },
}

impl fmt::Display for CouplingRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AmbiguousWinding { reference } => write!(
                formatter,
                "Coupled inductor reference '{reference}' is ambiguous; rename the inductors so each winding has a unique instance name"
            ),
            Self::TooFewWindings { coupling } => write!(
                formatter,
                "Coupled inductor '{coupling}' must reference at least two inductor instances"
            ),
            Self::MissingCoefficient { coupling } => write!(
                formatter,
                "Coupled inductor '{coupling}' is missing a coupling coefficient"
            ),
            Self::InvalidCoefficient {
                coupling,
                coefficient,
            } => write!(
                formatter,
                "Coupled inductor '{coupling}' has invalid coupling coefficient {coefficient} (expected -1 <= k <= 1)"
            ),
            Self::FactorWithoutTarget { inductor } => write!(
                formatter,
                "Inductor '{inductor}' defines a coupling factor but no target winding"
            ),
            Self::TargetWithoutFactor { inductor, target } => write!(
                formatter,
                "Inductor '{inductor}' references '{target}' but is missing a coupling factor"
            ),
            Self::NonNumericFactor { inductor, factor } => write!(
                formatter,
                "Inductor '{inductor}' has non-numeric coupling factor '{factor}'"
            ),
            Self::InvalidFactor { inductor, factor } => write!(
                formatter,
                "Inductor '{inductor}' has invalid coupling factor {factor} (expected -1 <= k <= 1)"
            ),
            Self::UnknownWinding {
                coupling,
                reference,
            } => write!(
                formatter,
                "Coupling '{coupling}' references unknown inductor '{reference}'"
            ),
            Self::RepeatedWinding {
                coupling,
                reference,
            } => write!(
                formatter,
                "Coupling '{coupling}' references '{reference}' more than once"
            ),
            Self::Conflicting {
                windings,
                written,
                written_by,
                coefficient,
                source,
            } => write!(
                formatter,
                "Conflicting coupling definitions for [{windings}]: {written} from {written_by} vs {coefficient} from {source}"
            ),
        }
    }
}

/// The couplings one cellview's components ask for, as the deck writes them.
///
/// Drawn couplings are taken first and inductors' `coupled_to` after them,
/// each in drawing order. A winding set is written once, by the first coupling
/// of it; a later one with the same coefficient writes nothing more, and one
/// with another coefficient is refused. The cards are written in the order of
/// their winding sets, compared without case.
pub fn coupling_cards(components: &[Component]) -> CouplingCards {
    let mut planner = CouplingPlanner::default();
    let windings = planner.windings(components);
    for component in components {
        if component.kind == ComponentType::CoupledInductor
            && let Some((card, source)) = planner.drawn(component, &windings)
        {
            planner.write(card, source);
        }
    }
    for component in components {
        if is_winding(component.kind)
            && let Some((card, source)) = planner.requested(component, &windings)
        {
            planner.write(card, source);
        }
    }
    CouplingCards {
        cards: planner
            .written
            .into_values()
            .map(|(card, _)| card)
            .collect(),
        refusals: planner.refusals,
    }
}

#[derive(Default)]
struct CouplingPlanner {
    /// The card written for each winding set, by its key.
    written: BTreeMap<String, (CouplingCard, CouplingSource)>,
    refusals: Vec<CouplingRefusal>,
}

impl CouplingPlanner {
    /// Every name a coupling may name a winding by — its written name and its
    /// drawn one, compared without case — with the written name it means.
    fn windings(&mut self, components: &[Component]) -> HashMap<String, String> {
        let mut windings = HashMap::new();
        for component in components
            .iter()
            .filter(|component| is_winding(component.kind))
        {
            let written = component.written_element_name();
            self.alias(&mut windings, &written, &written);
            if !component.name.trim().is_empty() {
                self.alias(&mut windings, component.name.trim(), &written);
            }
        }
        windings
    }

    fn alias(&mut self, windings: &mut HashMap<String, String>, alias: &str, written: &str) {
        let key = alias.trim().to_ascii_uppercase();
        if key.is_empty() {
            return;
        }
        if let Some(existing) = windings.get(&key) {
            if existing != written {
                self.refusals.push(CouplingRefusal::AmbiguousWinding {
                    reference: alias.to_owned(),
                });
            }
            return;
        }
        windings.insert(key, written.to_owned());
    }

    /// The card a drawn coupling is written as, which is the coupling itself.
    fn drawn(
        &mut self,
        component: &Component,
        windings: &HashMap<String, String>,
    ) -> Option<(CouplingCard, CouplingSource)> {
        let params = crate::parameters::parse_params_string(&component.params);
        let listed = params.get("inductors").cloned().unwrap_or_else(|| {
            ["l1", "l2", "l3", "l4"]
                .iter()
                .filter_map(|key| params.get(*key))
                .cloned()
                .collect::<Vec<_>>()
                .join(" ")
        });
        let references = listed
            .split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let coupling = component.spice_instance_name();
        if references.len() < 2 {
            self.refusals
                .push(CouplingRefusal::TooFewWindings { coupling });
            return None;
        }

        let coefficient = component.value.trim();
        if coefficient.is_empty() {
            self.refusals
                .push(CouplingRefusal::MissingCoefficient { coupling });
            return None;
        }
        if let Ok(value) = coefficient.parse::<f64>()
            && (!value.is_finite() || !(-1.0..=1.0).contains(&value))
        {
            self.refusals.push(CouplingRefusal::InvalidCoefficient {
                coupling,
                coefficient: coefficient.to_owned(),
            });
            return None;
        }

        let resolved = self.resolve(&references, windings, &coupling)?;
        Some((
            CouplingCard {
                component: component.id,
                role: CardRole::Element,
                name: component.written_element_name(),
                windings: resolved,
                coefficient: coefficient.to_owned(),
            },
            CouplingSource::Drawn(coupling),
        ))
    }

    /// The card an inductor's `coupled_to` asks for, or `None` when it asks
    /// for none. A coupling factor of zero with no target is no request.
    fn requested(
        &mut self,
        component: &Component,
        windings: &HashMap<String, String>,
    ) -> Option<(CouplingCard, CouplingSource)> {
        let params = crate::parameters::parse_params_string(&component.params);
        let target = params
            .get("coupled_to")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());
        let factor = params
            .get("coupling_factor")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());
        let factor_is_default_zero = factor
            .and_then(|value| value.parse::<f64>().ok())
            .is_some_and(|value| value == 0.0);
        if target.is_none() && (factor.is_none() || factor_is_default_zero) {
            return None;
        }

        let inductor = component.spice_instance_name();
        let Some(target) = target else {
            self.refusals
                .push(CouplingRefusal::FactorWithoutTarget { inductor });
            return None;
        };
        let Some(factor) = factor else {
            self.refusals.push(CouplingRefusal::TargetWithoutFactor {
                inductor,
                target: target.to_owned(),
            });
            return None;
        };
        let Ok(value) = factor.parse::<f64>() else {
            self.refusals.push(CouplingRefusal::NonNumericFactor {
                inductor,
                factor: factor.to_owned(),
            });
            return None;
        };
        if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
            self.refusals.push(CouplingRefusal::InvalidFactor {
                inductor,
                factor: factor.to_owned(),
            });
            return None;
        }

        let resolved = self.resolve(
            &[component.written_element_name(), target.to_owned()],
            windings,
            &inductor,
        )?;
        let mut names = resolved
            .iter()
            .map(String::as_str)
            .map(identifier)
            .collect::<Vec<_>>();
        names.sort();
        Some((
            CouplingCard {
                component: component.id,
                role: CardRole::Coupling,
                name: format!(
                    "{}{}",
                    ComponentType::CoupledInductor.spice_prefix(),
                    names.join("_")
                ),
                windings: resolved,
                coefficient: factor.to_owned(),
            },
            CouplingSource::Requested(inductor),
        ))
    }

    /// The written names of the windings a coupling names, or `None` when one
    /// is unknown or named twice.
    fn resolve(
        &mut self,
        references: &[String],
        windings: &HashMap<String, String>,
        coupling: &str,
    ) -> Option<Vec<String>> {
        let mut resolved = Vec::with_capacity(references.len());
        let mut seen = HashSet::new();
        for reference in references {
            let Some(written) = windings.get(&reference.trim().to_ascii_uppercase()) else {
                self.refusals.push(CouplingRefusal::UnknownWinding {
                    coupling: coupling.to_owned(),
                    reference: reference.clone(),
                });
                return None;
            };
            if !seen.insert(written.clone()) {
                self.refusals.push(CouplingRefusal::RepeatedWinding {
                    coupling: coupling.to_owned(),
                    reference: reference.clone(),
                });
                return None;
            }
            resolved.push(written.clone());
        }
        Some(resolved)
    }

    /// Write the card unless its winding set is written already.
    fn write(&mut self, card: CouplingCard, source: CouplingSource) {
        let mut key = card
            .windings
            .iter()
            .map(|name| name.to_ascii_uppercase())
            .collect::<Vec<_>>();
        key.sort();
        let key = key.join("|");
        if let Some((written, written_by)) = self.written.get(&key) {
            if written.coefficient != card.coefficient {
                self.refusals.push(CouplingRefusal::Conflicting {
                    windings: key,
                    written: written.coefficient.clone(),
                    written_by: written_by.clone(),
                    coefficient: card.coefficient,
                    source,
                });
            }
            return;
        }
        self.written.insert(key, (card, source));
    }
}

/// A component a coupling can name as a winding.
fn is_winding(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::Inductor | ComponentType::SaturableInductor
    )
}

/// `name` with every character other than an ASCII letter or digit made `_`.
fn identifier(name: &str) -> String {
    name.chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design_model::Point;

    fn drawn(id: u64, kind: ComponentType, name: &str, params: &str) -> Component {
        let mut component = Component::new(id, kind, Point::new(0, 0)).with_name_value(name, "1");
        component.params = params.to_owned();
        component
    }

    #[test]
    fn a_sense_source_is_written_for_a_source_that_names_no_controlling_element() {
        for (kind, name, params, sense) in [
            (ComponentType::Ccvs, "H1", "", Some("VSENSE_H1")),
            (ComponentType::Ccvs, "H1", "vref= ", Some("VSENSE_H1")),
            (ComponentType::Ccvs, "H1", "vmax=1", Some("VSENSE_H1")),
            (ComponentType::Ccvs, "H1", "vref=Vin", None),
            (ComponentType::Cccs, "out", "", Some("VSENSE_Fout")),
            (ComponentType::Cccs, "", "", Some("VSENSE_F4")),
            (ComponentType::ISwitch, "W1", "", Some("VSENSE_W1")),
            (ComponentType::ISwitch, "W1", "vref=Vin", Some("VSENSE_W1")),
            (ComponentType::Vcvs, "E1", "", None),
            (ComponentType::VoltageSource, "VSENSE_H1", "", None),
        ] {
            assert_eq!(
                drawn(4, kind, name, params).sense_source_name().as_deref(),
                sense,
                "{kind:?} drawn {name:?} with {params:?}"
            );
        }
    }

    #[test]
    fn a_transformer_is_written_as_its_windings_and_their_coupling() {
        for (name, base) in [
            ("T1", "T1"),
            ("X1", "X1"),
            ("T.out-1", "T_out_1"),
            ("_T1_", "T1"),
            ("TΩ", "T"),
            ("", "T9"),
            ("__", "9"),
        ] {
            assert_eq!(
                drawn(9, ComponentType::Transformer, name, "").transformer_cards(),
                Some(TransformerCards {
                    primary: format!("L{base}_PRI"),
                    secondary: format!("L{base}_SEC"),
                    coupling: format!("K{base}"),
                }),
                "{name:?}"
            );
        }
        assert_eq!(
            drawn(9, ComponentType::TransmissionLine, "T1", "").transformer_cards(),
            None
        );
    }

    #[test]
    fn a_sources_parasitics_are_written_as_cards_named_from_its_id() {
        let parasitics = |kind: ComponentType, params: &str| {
            drawn(7, kind, "V1", params)
                .source_parasitics()
                .map(|parasitics| {
                    [
                        parasitics.series_resistance,
                        parasitics.parallel_resistance,
                        parasitics.parallel_capacitance,
                    ]
                    .map(|parasitic| parasitic.map(|parasitic| (parasitic.name, parasitic.value)))
                })
        };
        let written = |name: &str, value: &str| Some((name.to_owned(), value.to_owned()));
        assert_eq!(
            parasitics(ComponentType::VoltageSourcePulse, "rs=' 5 ' rp=1k cpar=1p"),
            Some([
                written("R__RSPICE_SOURCE_7_RS", "5"),
                written("R__RSPICE_SOURCE_7_RP", "1k"),
                written("C__RSPICE_SOURCE_7_CPAR", "1p"),
            ])
        );
        // Blank and every spelling of zero write nothing, and a current source
        // has no series resistance to write.
        assert_eq!(
            parasitics(ComponentType::VoltageSource, "rs=0ohm rp= cpar=0f"),
            Some([None, None, None])
        );
        assert_eq!(
            parasitics(ComponentType::CurrentSourceSin, "rs=5 rp=10k"),
            Some([None, written("R__RSPICE_SOURCE_7_RP", "10k"), None])
        );
        // Only an independent source writes them.
        assert_eq!(parasitics(ComponentType::Vcvs, "rs=5"), None);
        assert_eq!(parasitics(ComponentType::LoopProbe, "rs=5"), None);

        let cards = written_cards(&[drawn(7, ComponentType::VoltageSource, "V1", "rs=5 cpar=1p")]);
        assert_eq!(
            cards[&7],
            [
                WrittenCard {
                    name: "V1".to_owned(),
                    role: CardRole::Element,
                },
                WrittenCard {
                    name: "R__RSPICE_SOURCE_7_RS".to_owned(),
                    role: CardRole::SeriesResistance,
                },
                WrittenCard {
                    name: "C__RSPICE_SOURCE_7_CPAR".to_owned(),
                    role: CardRole::ParallelCapacitance,
                },
            ]
        );
    }

    #[test]
    fn every_card_a_cellview_writes_is_listed_under_the_component_that_asks_for_it() {
        let components = [
            drawn(1, ComponentType::Transformer, "T1", ""),
            drawn(2, ComponentType::Ccvs, "H1", "vmax=1"),
            drawn(
                3,
                ComponentType::Inductor,
                "L1",
                "coupled_to=L2 coupling_factor=0.5",
            ),
            drawn(
                4,
                ComponentType::Inductor,
                "L2",
                "coupled_to=L1 coupling_factor=0.5",
            ),
            drawn(5, ComponentType::Inductor, "A", ""),
            drawn(6, ComponentType::Inductor, "B", ""),
            drawn(
                7,
                ComponentType::CoupledInductor,
                "K1",
                "inductors=\"LA LB\"",
            ),
            drawn(8, ComponentType::CoupledInductor, "K2", "inductors=\"B,A\""),
            drawn(9, ComponentType::Ground, "", ""),
        ];
        let cards = written_cards(&components);
        let named = |id: u64| {
            cards
                .get(&id)
                .map(|cards| {
                    cards
                        .iter()
                        .map(|card| (card.name.as_str(), card.role))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        assert_eq!(
            named(1),
            [
                ("LT1_PRI", CardRole::PrimaryWinding),
                ("LT1_SEC", CardRole::SecondaryWinding),
                ("KT1", CardRole::Coupling),
            ]
        );
        assert_eq!(
            named(2),
            [
                ("B1", CardRole::Element),
                ("VSENSE_H1", CardRole::SenseSource)
            ]
        );
        assert_eq!(
            named(3),
            [("L1", CardRole::Element), ("KL1_L2", CardRole::Coupling)]
        );
        assert_eq!(named(4), [("L2", CardRole::Element)]);
        assert_eq!(named(7), [("K1", CardRole::Element)]);
        assert!(
            named(8).is_empty(),
            "the deck writes one card per winding set"
        );
        assert!(named(9).is_empty());
    }

    #[test]
    fn a_drawn_coupling_is_written_before_one_an_inductor_asks_for() {
        let components = [
            drawn(
                1,
                ComponentType::Inductor,
                "L1",
                "coupled_to=L2 coupling_factor=1",
            ),
            drawn(2, ComponentType::Inductor, "L2", ""),
            drawn(3, ComponentType::CoupledInductor, "K9", "l1=L2 l2=l1"),
        ];
        let plan = coupling_cards(&components);
        assert!(plan.refusals.is_empty(), "{:?}", plan.refusals);
        assert_eq!(
            plan.cards,
            [CouplingCard {
                component: 3,
                role: CardRole::Element,
                name: "K9".to_owned(),
                windings: vec!["L2".to_owned(), "L1".to_owned()],
                coefficient: "1".to_owned(),
            }]
        );
    }

    #[test]
    fn a_coupling_the_deck_cannot_write_is_refused_in_the_netlists_words() {
        let refusals = |components: &[Component]| {
            coupling_cards(components)
                .refusals
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            refusals(&[
                drawn(
                    1,
                    ComponentType::Inductor,
                    "L1",
                    "coupled_to=L2 coupling_factor=0.5"
                ),
                drawn(2, ComponentType::Inductor, "L2", ""),
                drawn(
                    3,
                    ComponentType::CoupledInductor,
                    "K1",
                    "inductors=\"L1 L2\""
                ),
            ]),
            [
                "Conflicting coupling definitions for [L1|L2]: 1 from coupling 'K1' vs 0.5 from inductor 'L1'"
            ]
        );
        assert_eq!(
            refusals(&[
                drawn(1, ComponentType::Inductor, "L1", ""),
                drawn(2, ComponentType::Inductor, "l1", ""),
                drawn(
                    3,
                    ComponentType::CoupledInductor,
                    "K1",
                    "inductors=\"L1 L3\""
                ),
                drawn(4, ComponentType::CoupledInductor, "K2", "inductors=L1"),
            ]),
            [
                "Coupled inductor reference 'l1' is ambiguous; rename the inductors so each winding has a unique instance name",
                "Coupled inductor reference 'l1' is ambiguous; rename the inductors so each winding has a unique instance name",
                "Coupling 'K1' references unknown inductor 'L3'",
                "Coupled inductor 'K2' must reference at least two inductor instances",
            ]
        );
    }
}
