//! Every definition name an instance could reference, across the loaded
//! model libraries, and which of them more than one library provides.
//!
//! The Models & PDKs workspace's include page and the Simulation Studio's
//! Models page both state it, from this one walk of the corpus.

use std::collections::{BTreeMap, BTreeSet};

use super::Model;

/// One name an instance could reference, and everything that defines it.
///
/// Readable outside the manager so the Simulation Studio's Models page can
/// state contested definitions without computing them a second time. Two
/// answers to "is this name contested" would eventually differ, and the one
/// on the page an operator was looking at would be the one they believed.
pub struct DefinitionRow {
    pub definition: String,
    pub scope: crate::state::model_library::ModelConsumerScope,
    pub providers: Vec<String>,
    pub resolution: String,
    /// The library the manager's resolution record picked, where it picked one.
    ///
    /// Carried rather than parsed back out of [`Self::resolution`], which is
    /// display prose: a sentence that becomes a load-bearing format is a
    /// sentence nobody can rewrite.
    pub resolved_provider: Option<String>,
    /// The `.lib` sections the providers declare this name inside.
    ///
    /// A name declared outside any section has none, which is not the same as
    /// a name whose section is unknown: the parser records the section every
    /// definition came from, so an empty list here is positive evidence that
    /// the definition sits in the file's unsectioned body. Where two providers
    /// declare the same name in different sections both are carried, because
    /// which one executes is a corner decision made on another page.
    pub sections: Vec<String>,
}

impl DefinitionRow {
    /// A contested name has no winner: the duplicate has to be removed or
    /// renamed before an instance can bind at all.
    ///
    /// Several providers is not that on its own. The manager records an
    /// explicit resolution for a duplicate it settled, and a name it settled
    /// binds — so counting every multi-provider name as contested marked rows
    /// the RESOLUTION column beside them was already calling resolved, and
    /// inflated this card's own count of what has to be repaired.
    pub fn contested(&self) -> bool {
        self.providers.len() > 1 && self.resolved_provider.is_none()
    }

    /// The provider the flat executable namespace will use, where one exists.
    ///
    /// A contested name has none — that is what contested means — and the cell
    /// that shows this says so rather than printing the first candidate. The
    /// column that used to head "WINNING PROVIDER" printed exactly that first
    /// candidate for every row, which asserted a resolution policy the product
    /// does not have.
    pub fn effective_provider(&self) -> Option<&str> {
        self.resolved_provider
            .as_deref()
            .or(match self.providers.as_slice() {
                [only] => Some(only.as_str()),
                _ => None,
            })
    }

    /// Every other authenticated provider of the same name.
    ///
    /// For a settled duplicate these are the losers the override record left
    /// standing — recorded, never dropped. For a contested name it is every
    /// provider, since none of them won.
    pub fn other_candidates(&self) -> String {
        let effective = self.effective_provider();
        let others = self
            .providers
            .iter()
            .filter(|provider| Some(provider.as_str()) != effective)
            .cloned()
            .collect::<Vec<_>>();
        if others.is_empty() {
            "—".to_owned()
        } else {
            others.join(", ")
        }
    }

    /// Where the providers declare it.
    pub fn section_label(&self) -> String {
        if self.sections.is_empty() {
            "—".to_owned()
        } else {
            self.sections.join(", ")
        }
    }
}

impl Model {
    /// Every definition name across the loaded libraries, with its providers.
    ///
    /// Model names and subcircuit names share one namespace as far as an instance
    /// reference is concerned, so both are here for "contested" to mean anything.
    pub fn definition_index(&self) -> Vec<DefinitionRow> {
        use crate::state::model_library::ModelConsumerScope;
        let mut providers = BTreeMap::<(ModelConsumerScope, String), BTreeSet<String>>::new();
        // The section a name is declared inside comes out of the same walk rather
        // than a second lookup per row: the parser already recorded it on every
        // model and subcircuit, and asking the catalog again per definition would
        // turn one pass over the corpus into one per name in it.
        let mut sections = BTreeMap::<(ModelConsumerScope, String), BTreeSet<String>>::new();
        for library in self.model_library_manager.libraries_sorted() {
            let active_sections = library.active_section_names();
            for model in library.models.values() {
                let key = (
                    ModelConsumerScope::PrimitiveModel,
                    model.name.to_ascii_lowercase(),
                );
                providers
                    .entry(key.clone())
                    .or_default()
                    .insert(library.name.clone());
                if let Some(section) = model.section.as_deref() {
                    sections.entry(key).or_default().insert(section.to_owned());
                }
            }
            // A binned family is a name an instance binds, provided as the
            // catalog's provider resolution provides it, so a family two
            // libraries share is contested, and resolved, here like a card.
            for family in library.bin_families() {
                let key = (
                    ModelConsumerScope::PrimitiveModel,
                    family.name.to_ascii_lowercase(),
                );
                providers
                    .entry(key.clone())
                    .or_default()
                    .insert(library.name.clone());
                for section in family.bins.iter().filter_map(|bin| bin.section.as_deref()) {
                    sections
                        .entry(key.clone())
                        .or_default()
                        .insert(section.to_owned());
                }
            }
            for subcircuit in library.subcircuits.values() {
                if subcircuit.section.as_deref().is_none_or(|section| {
                    active_sections
                        .iter()
                        .any(|active| active.eq_ignore_ascii_case(section))
                }) {
                    let key = (
                        ModelConsumerScope::Subcircuit,
                        subcircuit.name.to_ascii_lowercase(),
                    );
                    providers
                        .entry(key.clone())
                        .or_default()
                        .insert(library.name.clone());
                    if let Some(section) = subcircuit.section.as_deref() {
                        sections.entry(key).or_default().insert(section.to_owned());
                    }
                }
            }
        }
        providers
            .into_iter()
            .map(|(key, candidates)| {
                let (scope, definition) = key.clone();
                let resolved_provider = self
                    .model_library_manager
                    .model_resolution_record(scope, &definition)
                    .map(|record| record.provider_library.clone());
                let resolution = resolved_provider.as_ref().map_or_else(
                    || {
                        if candidates.len() > 1 {
                            "contested · fails closed".to_owned()
                        } else {
                            "unique".to_owned()
                        }
                    },
                    |library| format!("resolved · {library}"),
                );
                DefinitionRow {
                    definition,
                    scope,
                    // Every provider, in order. The joined display spelling this
                    // used to carry beside it went unread the moment the table
                    // split the effective provider from the candidates that lost.
                    providers: candidates.into_iter().collect(),
                    resolution,
                    resolved_provider,
                    sections: sections
                        .remove(&key)
                        .map(|sections| sections.into_iter().collect())
                        .unwrap_or_default(),
                }
            })
            .collect()
    }
}
