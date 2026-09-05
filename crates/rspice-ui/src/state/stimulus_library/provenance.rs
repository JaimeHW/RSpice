//! What a placed source remembers about the definition it was copied from.
//!
//! Adoption is a copy. The instance keeps owning its card exactly as it did
//! before there was a library — the netlister, the deck, the Excitations page
//! and every frozen manifest all still read `Component::{kind, value, params}`
//! — and the definition leaves behind a receipt: which definition, which
//! revision, and the two fields as they were copied.
//!
//! That receipt is what makes every lifecycle word a *comparison* rather than a
//! flag someone has to remember to set. "Modified" is the instance's card
//! differing from its copy; "behind" is the library's revision being past the
//! adopted one. Nothing can drift, because nothing is recorded twice.
//!
//! The operations hang off the two nouns they act on — a definition is adopted
//! onto an instance, a library is asked where an instance stands — rather than
//! standing as free verbs named `adopt` and `adopters`, which say nothing about
//! their subject at a call site.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::definition::{
    StimulusDefinition, StimulusDefinitionError, StimulusFamily, StimulusKind, normalize_params,
};
use super::library::StimulusLibrary;
use crate::state::{Component, ComponentType};

/// The copy a placed source took when it adopted a definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StimulusProvenance {
    /// The definition's name, as the library spells it.
    pub definition: String,
    /// The revision that was copied.
    pub revision: u32,
    /// `Component::value` as it was copied.
    pub value: String,
    /// `Component::params` as it was copied.
    pub params: String,
}

impl StimulusProvenance {
    /// The receipt for copying this definition onto an instance.
    #[must_use]
    pub fn of(definition: &StimulusDefinition) -> Self {
        Self {
            definition: definition.name().to_owned(),
            revision: definition.revision(),
            value: definition.value.clone(),
            params: definition.params.clone(),
        }
    }

    /// Whether the component still carries exactly the card it copied.
    ///
    /// Both sides are normalized, so a parameter string someone re-ordered by
    /// editing an unrelated field does not read as an edit to the waveform.
    #[must_use]
    pub fn matches_card(&self, component: &Component) -> bool {
        component.value.trim() == self.value.trim()
            && normalize_params(&component.params) == normalize_params(&self.params)
    }
}

/// Where a placed source stands with respect to the library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceState {
    /// The instance was drawn on the sheet and adopted nothing.
    FromSchematic,
    /// The instance carries the copy it took, and the library has not moved.
    Adopted {
        /// The adopted revision.
        revision: u32,
    },
    /// The instance carries its copy, and the library has published past it.
    Behind {
        /// The revision the instance copied.
        adopted: u32,
        /// The revision the library holds now.
        library: u32,
    },
    /// The instance's card has been edited away from the copy it took.
    Modified {
        /// The revision the edit departs from.
        from: u32,
    },
    /// The card was edited *and* the library has published past the copy.
    ModifiedBehind {
        /// The revision the edit departs from.
        from: u32,
        /// The revision the library holds now.
        library: u32,
    },
    /// The definition the instance names is no longer in the library.
    Removed {
        /// The revision the instance copied before the definition went away.
        revision: u32,
    },
}

impl ProvenanceState {
    /// The chip Component Properties, the realization band, the inspector and
    /// the schematic hero all show for this state.
    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::FromSchematic => "from schematic".to_owned(),
            Self::Adopted { revision } => format!("adopted · r{revision}"),
            Self::Behind { library, .. } => format!("behind · library r{library}"),
            Self::Modified { from } => format!("modified from r{from}"),
            Self::ModifiedBehind { from, library } => {
                format!("modified from r{from} · library r{library}")
            }
            Self::Removed { .. } => "definition removed".to_owned(),
        }
    }

    /// The Studio Excitations "Definition" cell, which names the definition as
    /// well as the state because the column has no other place to put it.
    ///
    /// An instance that adopted nothing has no name to show, so the cell opens
    /// with an em dash rather than leaving the column ragged.
    #[must_use]
    pub fn studio_cell(&self, definition: Option<&str>) -> String {
        let definition = definition.unwrap_or("—");
        match self {
            Self::FromSchematic => "— · from schematic".to_owned(),
            Self::Adopted { revision } => format!("{definition} · r{revision}"),
            Self::Behind { adopted, library } => {
                format!("{definition} · r{adopted} · library r{library}")
            }
            Self::Modified { from } => format!("{definition} · modified from r{from}"),
            Self::ModifiedBehind { from, library } => {
                format!("{definition} · modified from r{from} · library r{library}")
            }
            Self::Removed { revision } => {
                format!("{definition} · r{revision} · definition removed")
            }
        }
    }

    /// Whether re-adopting would change anything.
    ///
    /// A removed definition deliberately does not offer it: there is no
    /// revision left to copy, and the instance keeps the card it has.
    #[must_use]
    pub const fn offers_readoption(&self) -> bool {
        matches!(
            self,
            Self::Behind { .. } | Self::Modified { .. } | Self::ModifiedBehind { .. }
        )
    }
}

/// How far a definition is from an instance it is about to be adopted onto.
///
/// Family is shape and kind is topology. Changing the shape of a placed source
/// is what an engineer means by "make this a pulse instead", and it is a typed
/// re-place: a new component type, a new symbol and a new property sheet, with
/// both terminals keeping their nets. Changing the quantity is not a shape
/// change at all, and nobody means it by "adopt".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdoptionFit {
    /// The instance is already of the definition's type: adoption is a copy.
    Same,
    /// Same quantity, different shape: the caller re-places, then adopts.
    Replace {
        /// The type the instance has now.
        from: ComponentType,
        /// The type it must be re-placed as.
        to: ComponentType,
    },
    /// Different quantity, or not an independent source at all: refused.
    Kind,
}

impl AdoptionFit {
    /// The second-level warning a re-place shows before it happens.
    #[must_use]
    pub fn replace_warning(&self) -> Option<String> {
        let Self::Replace { from, to } = self else {
            return None;
        };
        let from = StimulusFamily::of(*from)?.label();
        let to = StimulusFamily::of(*to)?.label();
        Some(format!(
            "Adopting a {to} definition onto a {from} source re-places the instance as {to} (new \
             component type, symbol and property sheet); both terminals keep their nets. Undo \
             restores the {from} instance."
        ))
    }
}

impl StimulusDefinition {
    /// How far this definition is from this instance.
    #[must_use]
    pub fn adoption_fit(&self, component: &Component) -> AdoptionFit {
        match StimulusKind::of(component.kind) {
            Some(kind) if kind == self.kind() => {
                if component.kind == self.component_type() {
                    AdoptionFit::Same
                } else {
                    AdoptionFit::Replace {
                        from: component.kind,
                        to: self.component_type(),
                    }
                }
            }
            _ => AdoptionFit::Kind,
        }
    }

    /// Why this definition cannot be adopted onto this instance at all.
    #[must_use]
    pub fn kind_refusal(&self, component: &Component) -> String {
        let Some(kind) = StimulusKind::of(component.kind) else {
            return format!(
                "{} is not an independent source, so there is no waveform on it for '{}' to \
                 replace.",
                component.name,
                self.name()
            );
        };
        format!(
            "{} is a {} definition and this instance is a {} source — kind is a circuit topology, \
             not a shape. Duplicate the definition as {} in the library first.",
            self.name(),
            self.kind().word(),
            kind.word(),
            kind.letter()
        )
    }

    /// Copy this definition onto a placed source and record the copy.
    ///
    /// Refuses anything but [`AdoptionFit::Same`]: a shape change is a re-place
    /// the caller performs first — the instance's terminals belong to the
    /// schematic, not to this function — and a quantity change is refused
    /// outright.
    pub fn adopt_onto(&self, component: &mut Component) -> Result<(), String> {
        match self.adoption_fit(component) {
            AdoptionFit::Same => {
                component.value = self.value.clone();
                component.params = self.params.clone();
                component.stimulus_provenance = Some(StimulusProvenance::of(self));
                Ok(())
            }
            fit @ AdoptionFit::Replace { .. } => Err(fit
                .replace_warning()
                .unwrap_or_else(|| self.kind_refusal(component))),
            AdoptionFit::Kind => Err(self.kind_refusal(component)),
        }
    }

    /// Copy this definition's current revision onto an instance that already
    /// adopted it.
    ///
    /// Deliberately narrower than adoption: an instance that names some other
    /// definition is not re-adopting, it is adopting, and the caller should say
    /// so rather than letting one verb quietly mean both.
    pub fn readopt_onto(&self, component: &mut Component) -> Result<(), String> {
        let adopted = component
            .stimulus_provenance
            .as_ref()
            .map(|provenance| provenance.definition.as_str());
        if adopted != Some(self.name()) {
            return Err(format!(
                "{} has not adopted '{}', so there is no revision of it to re-adopt.",
                component.name,
                self.name()
            ));
        }
        self.adopt_onto(component)
    }

    /// Save a placed source's card as a new `r1` definition, and point the
    /// instance's provenance at it.
    ///
    /// The card is untouched: extraction publishes what the instance already
    /// says, which is why the instance reads `adopted · r1` immediately
    /// afterwards rather than `modified`.
    pub fn extract_from(
        component: &mut Component,
        name: impl Into<String>,
        purpose: impl Into<String>,
    ) -> Result<Self, StimulusDefinitionError> {
        let mut definition = Self::new(name, component.kind)?;
        definition.value = component.value.clone();
        definition.params = component.params.clone();
        definition.purpose = purpose.into();
        component.stimulus_provenance = Some(StimulusProvenance::of(&definition));
        Ok(definition)
    }
}

impl StimulusLibrary {
    /// Where this placed source stands with respect to this library.
    ///
    /// The component type takes part in the comparison, but only while the
    /// adopted revision is the one the library holds: the receipt does not
    /// retain the type the definition had at revision N, so once the library
    /// has published past N a type difference is the library's move to explain
    /// and not the instance's.
    #[must_use]
    pub fn provenance_state(&self, component: &Component) -> ProvenanceState {
        let Some(provenance) = component.stimulus_provenance.as_ref() else {
            return ProvenanceState::FromSchematic;
        };
        let Some(definition) = self.get(&provenance.definition) else {
            return ProvenanceState::Removed {
                revision: provenance.revision,
            };
        };
        let behind = definition.revision() > provenance.revision;
        let retyped = !behind && component.kind != definition.component_type();
        let modified = retyped || !provenance.matches_card(component);
        match (modified, behind) {
            (false, false) => ProvenanceState::Adopted {
                revision: provenance.revision,
            },
            (false, true) => ProvenanceState::Behind {
                adopted: provenance.revision,
                library: definition.revision(),
            },
            (true, false) => ProvenanceState::Modified {
                from: provenance.revision,
            },
            (true, true) => ProvenanceState::ModifiedBehind {
                from: provenance.revision,
                library: definition.revision(),
            },
        }
    }

    /// The Studio Excitations "Definition" cell for one placed source.
    #[must_use]
    pub fn studio_definition_cell(&self, component: &Component) -> String {
        self.provenance_state(component).studio_cell(
            component
                .stimulus_provenance
                .as_ref()
                .map(|provenance| provenance.definition.as_str()),
        )
    }

    /// Which placed sources adopted each definition this library holds.
    ///
    /// Keyed by definition name and ordered by it, so two surfaces listing the
    /// same library list it the same way. An instance naming a definition the
    /// library no longer holds is deliberately absent: it has no definition to
    /// be an adopter of, and [`Self::provenance_state`] is where it says so.
    #[must_use]
    pub fn adopters<'a>(
        &self,
        components: &'a [Component],
    ) -> BTreeMap<String, Vec<&'a Component>> {
        let mut adopters: BTreeMap<String, Vec<&'a Component>> = self
            .definitions()
            .iter()
            .map(|definition| (definition.name().to_owned(), Vec::new()))
            .collect();
        for component in components {
            let Some(provenance) = component.stimulus_provenance.as_ref() else {
                continue;
            };
            if let Some(entry) = adopters.get_mut(&provenance.definition) {
                entry.push(component);
            }
        }
        adopters
    }

    /// The table a `PWL FILE=` instance's definition retains, if it retains one.
    ///
    /// Only an adopted instance has one, and only while its card still names
    /// the file the copy did: retained bytes belong to a definition, and
    /// offering them for a card someone has since pointed elsewhere would
    /// describe a run that will not happen.
    ///
    /// This is what makes a project self-contained — it is written into the
    /// document and travels with it — and it is deliberately *not* what a
    /// preview evaluates: the engine's PWL loader takes a path, so bytes the
    /// app is holding are not something it can be asked to step through.
    #[must_use]
    pub fn retained_pwl_contents(&self, component: &Component) -> Option<&str> {
        if !matches!(
            self.provenance_state(component),
            ProvenanceState::Adopted { .. } | ProvenanceState::Behind { .. }
        ) {
            return None;
        }
        let provenance = component.stimulus_provenance.as_ref()?;
        let retained = self.get(&provenance.definition)?.pwl_file.as_ref()?;
        Some(retained.contents.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;
    use crate::state::stimulus_library::definition::RetainedPwlFile;

    fn source(kind: ComponentType, value: &str, params: &str) -> Component {
        let mut component = Component::new(7, kind, Point::new(0, 0));
        component.name = "V1".to_owned();
        component.value = value.to_owned();
        component.params = params.to_owned();
        component
    }

    fn definition(
        name: &str,
        kind: ComponentType,
        value: &str,
        params: &str,
    ) -> StimulusDefinition {
        let mut definition = StimulusDefinition::new(name, kind).expect("definition");
        definition.value = value.to_owned();
        definition.params = params.to_owned();
        definition
    }

    fn library(definitions: Vec<StimulusDefinition>) -> StimulusLibrary {
        let mut library = StimulusLibrary::default();
        for definition in definitions {
            library.insert(definition).expect("insert");
        }
        library
    }

    #[test]
    fn an_unadopted_instance_reads_from_schematic_in_both_forms() {
        let component = source(ComponentType::VoltageSourceSin, "0", "va=1 freq=1k");
        let library = library(Vec::new());
        let state = library.provenance_state(&component);

        assert_eq!(state, ProvenanceState::FromSchematic);
        assert_eq!(state.label(), "from schematic");
        assert_eq!(
            library.studio_definition_cell(&component),
            "— · from schematic"
        );
        assert!(!state.offers_readoption());
    }

    #[test]
    fn the_six_states_produce_the_vocabulary_verbatim() {
        for (state, label, cell) in [
            (
                ProvenanceState::Adopted { revision: 4 },
                "adopted · r4",
                "sensor_diff_1k · r4",
            ),
            (
                ProvenanceState::Behind {
                    adopted: 4,
                    library: 5,
                },
                "behind · library r5",
                "sensor_diff_1k · r4 · library r5",
            ),
            (
                ProvenanceState::Modified { from: 2 },
                "modified from r2",
                "sensor_diff_1k · modified from r2",
            ),
            (
                ProvenanceState::ModifiedBehind {
                    from: 4,
                    library: 5,
                },
                "modified from r4 · library r5",
                "sensor_diff_1k · modified from r4 · library r5",
            ),
            (
                ProvenanceState::Removed { revision: 2 },
                "definition removed",
                "sensor_diff_1k · r2 · definition removed",
            ),
        ] {
            assert_eq!(state.label(), label);
            assert_eq!(state.studio_cell(Some("sensor_diff_1k")), cell);
        }
        assert_eq!(
            ProvenanceState::FromSchematic.studio_cell(None),
            "— · from schematic"
        );
    }

    #[test]
    fn only_a_state_with_a_revision_left_to_copy_offers_re_adoption() {
        assert!(!ProvenanceState::Adopted { revision: 1 }.offers_readoption());
        assert!(
            ProvenanceState::Behind {
                adopted: 1,
                library: 2
            }
            .offers_readoption()
        );
        assert!(ProvenanceState::Modified { from: 1 }.offers_readoption());
        assert!(
            ProvenanceState::ModifiedBehind {
                from: 1,
                library: 2
            }
            .offers_readoption()
        );
        assert!(!ProvenanceState::Removed { revision: 1 }.offers_readoption());
    }

    #[test]
    fn adoption_copies_the_card_and_the_instance_then_reads_adopted() {
        let mut component = source(ComponentType::VoltageSourceSin, "1", "va=9");
        let definition = definition(
            "sensor_diff_1k",
            ComponentType::VoltageSourceSin,
            "0",
            "va=3m freq=1k",
        );
        definition.adopt_onto(&mut component).expect("adopt");

        assert_eq!(component.value, "0");
        assert_eq!(component.params, "va=3m freq=1k");
        assert_eq!(
            library(vec![definition]).provenance_state(&component),
            ProvenanceState::Adopted { revision: 1 }
        );
    }

    #[test]
    fn a_reordered_parameter_string_is_not_an_edit() {
        let mut component = source(ComponentType::VoltageSourceSin, "0", "");
        let definition = definition(
            "sensor_diff_1k",
            ComponentType::VoltageSourceSin,
            "0",
            "va=3m freq=1k",
        );
        definition.adopt_onto(&mut component).expect("adopt");
        component.params = "freq=1k   va=3m".to_owned();

        assert_eq!(
            library(vec![definition]).provenance_state(&component),
            ProvenanceState::Adopted { revision: 1 }
        );
    }

    #[test]
    fn editing_the_card_reads_modified_and_publishing_reads_behind() {
        let mut component = source(ComponentType::VoltageSourceSin, "0", "");
        let mut definition = definition(
            "sensor_diff_1k",
            ComponentType::VoltageSourceSin,
            "0",
            "va=3m freq=1k",
        );
        definition.adopt_onto(&mut component).expect("adopt");

        component.params = "va=6m freq=1k".to_owned();
        assert_eq!(
            library(vec![definition.clone()]).provenance_state(&component),
            ProvenanceState::Modified { from: 1 }
        );

        definition.publish_next_revision();
        assert_eq!(
            library(vec![definition.clone()]).provenance_state(&component),
            ProvenanceState::ModifiedBehind {
                from: 1,
                library: 2
            }
        );

        component.params = "va=3m freq=1k".to_owned();
        assert_eq!(
            library(vec![definition]).provenance_state(&component),
            ProvenanceState::Behind {
                adopted: 1,
                library: 2
            }
        );
    }

    #[test]
    fn deleting_the_definition_leaves_the_card_and_reads_removed() {
        let mut component = source(ComponentType::VoltageSourceSin, "0", "");
        let definition = definition("gone", ComponentType::VoltageSourceSin, "0", "va=1");
        definition.adopt_onto(&mut component).expect("adopt");

        assert_eq!(
            library(Vec::new()).provenance_state(&component),
            ProvenanceState::Removed { revision: 1 }
        );
        assert_eq!(component.params, "va=1");
    }

    #[test]
    fn a_retyped_adopter_reads_modified_while_the_library_has_not_moved() {
        let mut component = source(ComponentType::VoltageSourceSin, "0", "va=1");
        let definition = definition("shape", ComponentType::VoltageSourceSin, "0", "va=1");
        definition.adopt_onto(&mut component).expect("adopt");
        component.kind = ComponentType::VoltageSourcePulse;

        assert_eq!(
            library(vec![definition]).provenance_state(&component),
            ProvenanceState::Modified { from: 1 }
        );
    }

    #[test]
    fn a_shape_change_is_a_replace_and_a_quantity_change_is_refused() {
        let component = source(ComponentType::VoltageSource, "5", "");
        let pulse = definition("edge", ComponentType::VoltageSourcePulse, "0", "");
        assert_eq!(
            pulse.adoption_fit(&component),
            AdoptionFit::Replace {
                from: ComponentType::VoltageSource,
                to: ComponentType::VoltageSourcePulse,
            }
        );
        assert_eq!(
            pulse.adoption_fit(&component).replace_warning().as_deref(),
            Some(
                "Adopting a PULSE definition onto a DC source re-places the instance as PULSE \
                 (new component type, symbol and property sheet); both terminals keep their nets. \
                 Undo restores the DC instance."
            )
        );

        let current = definition("bias", ComponentType::CurrentSource, "1m", "");
        assert_eq!(current.adoption_fit(&component), AdoptionFit::Kind);
        assert_eq!(
            current.kind_refusal(&component),
            "bias is a current definition and this instance is a voltage source — kind is a \
             circuit topology, not a shape. Duplicate the definition as V in the library first."
        );
    }

    #[test]
    fn adopt_refuses_a_replace_and_leaves_the_instance_alone() {
        let mut component = source(ComponentType::VoltageSource, "5", "");
        let pulse = definition("edge", ComponentType::VoltageSourcePulse, "0", "v2=5");
        assert!(pulse.adopt_onto(&mut component).is_err());
        assert_eq!(component.value, "5");
        assert!(component.stimulus_provenance.is_none());
    }

    #[test]
    fn adopt_refuses_a_component_that_is_not_an_independent_source() {
        let mut resistor = source(ComponentType::Resistor, "1k", "");
        let definition = definition("edge", ComponentType::VoltageSourcePulse, "0", "");
        assert_eq!(definition.adoption_fit(&resistor), AdoptionFit::Kind);
        assert!(
            definition
                .kind_refusal(&resistor)
                .contains("is not an independent source")
        );
        assert!(definition.adopt_onto(&mut resistor).is_err());
    }

    #[test]
    fn readopt_refuses_an_instance_that_never_adopted_this_definition() {
        let mut component = source(ComponentType::VoltageSourceSin, "0", "");
        let one = definition("one", ComponentType::VoltageSourceSin, "0", "va=1");
        let two = definition("two", ComponentType::VoltageSourceSin, "0", "va=2");
        assert!(one.readopt_onto(&mut component).is_err());

        one.adopt_onto(&mut component).expect("adopt");
        assert!(two.readopt_onto(&mut component).is_err());
        assert!(one.readopt_onto(&mut component).is_ok());
    }

    #[test]
    fn extraction_publishes_the_card_and_points_the_instance_at_it() {
        let mut component = source(ComponentType::CurrentSourceExp, "0", "i2=1m tau1=1u");
        let definition =
            StimulusDefinition::extract_from(&mut component, "ramp_1u", "startup ramp")
                .expect("extract");

        assert_eq!(definition.revision(), 1);
        assert_eq!(definition.component_type(), ComponentType::CurrentSourceExp);
        assert_eq!(definition.params, "i2=1m tau1=1u");
        assert_eq!(definition.purpose, "startup ramp");
        assert_eq!(component.params, "i2=1m tau1=1u");
        assert_eq!(
            library(vec![definition]).provenance_state(&component),
            ProvenanceState::Adopted { revision: 1 }
        );
    }

    #[test]
    fn extraction_refuses_a_component_that_is_not_an_independent_source() {
        let mut resistor = source(ComponentType::Resistor, "1k", "");
        assert_eq!(
            StimulusDefinition::extract_from(&mut resistor, "r", ""),
            Err(StimulusDefinitionError::NotASource(ComponentType::Resistor))
        );
        assert!(resistor.stimulus_provenance.is_none());
    }

    #[test]
    fn adopters_are_computed_from_the_instances_that_name_each_definition() {
        let one = definition("one", ComponentType::VoltageSourceSin, "0", "va=1");
        let two = definition("two", ComponentType::VoltageSourceSin, "0", "va=2");
        let mut first = source(ComponentType::VoltageSourceSin, "0", "");
        let mut second = source(ComponentType::VoltageSourceSin, "0", "");
        let unadopted = source(ComponentType::VoltageSourceSin, "0", "");
        one.adopt_onto(&mut first).expect("adopt");
        one.adopt_onto(&mut second).expect("adopt");

        let components = vec![first, second, unadopted];
        let adopters = library(vec![one, two]).adopters(&components);
        assert_eq!(adopters["one"].len(), 2);
        assert!(adopters["two"].is_empty());
    }

    #[test]
    fn retained_pwl_bytes_are_offered_only_while_the_card_is_still_the_copy() {
        let mut definition = definition(
            "bridge_cal_step",
            ComponentType::VoltageSourcePwlFile,
            "step.csv",
            "file=step.csv",
        );
        definition.pwl_file = Some(RetainedPwlFile::new("step.csv", "0 0\n1e-9 1\n", 5));
        let mut component = source(ComponentType::VoltageSourcePwlFile, "", "");
        definition.adopt_onto(&mut component).expect("adopt");
        let library = library(vec![definition]);

        assert_eq!(
            library.retained_pwl_contents(&component),
            Some("0 0\n1e-9 1\n")
        );

        component.params = "file=other.csv".to_owned();
        assert_eq!(library.retained_pwl_contents(&component), None);
    }
}
