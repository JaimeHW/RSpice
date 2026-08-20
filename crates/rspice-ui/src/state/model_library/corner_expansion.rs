//! One rule for whether a run can expand a corner into executable sections.
//!
//! The rule has two halves: which sections a corner demands, and whether the
//! retained closure carries them. Both used to exist twice — once in
//! `io::project_execution`, which is what a run actually obeys, and once in
//! the Corners page, which restated it. Restatements drift, and these two did:
//! the page had no counterpart for the run's project-owned escape, so it
//! painted "run expansion blocked" over runs that proceeded, and it asked
//! whether a section *defined* anything while the run asks only whether the
//! authenticated source *parsed* one, so it blocked on sections the run
//! accepts.
//!
//! The run is the semantic authority. This module is the run's rule, and the
//! page conforms by calling it rather than by agreeing with it.

use std::collections::{BTreeMap, HashMap};

use super::{
    DeviceModel, ModelDefinitionMetadata, ModelLibrary, ModelSourceAuthority, ModelSourcePin,
    ProcessCorner,
};

/// The complete section catalog of a retained closure.
///
/// One entry per section the authenticated source parses to, whether or not it
/// defines anything — which is exactly the set a run matches a requested
/// section name against. `io::project_execution` rebuilds this projection from
/// the closure at every save and validates it against the closure at every
/// load, so its keys are the parsed section names by construction.
pub type SectionCatalog = HashMap<String, HashMap<String, DeviceModel>>;

/// The library facts corner expansion depends on, borrowed from whichever
/// shape happens to hold them: the live catalog, or the persisted execution
/// context that a run reads.
#[derive(Clone, Copy)]
pub struct RetainedClosure<'a> {
    pub source_authority: &'a ModelSourceAuthority,
    pub model_definition_metadata: &'a HashMap<String, ModelDefinitionMetadata>,
    pub source_closure: &'a [ModelSourcePin],
    pub section_models: &'a SectionCatalog,
}

impl<'a> From<&'a ModelLibrary> for RetainedClosure<'a> {
    fn from(library: &'a ModelLibrary) -> Self {
        Self {
            source_authority: &library.source_authority,
            model_definition_metadata: &library.model_definition_metadata,
            source_closure: &library.source_closure,
            section_models: &library.section_models,
        }
    }
}

impl RetainedClosure<'_> {
    /// Whether project-owned authoring metadata supersedes section expansion.
    ///
    /// A project that authored its own definitions carries them directly; the
    /// run resolves no sections at all for such a library, so nothing about a
    /// corner's section bindings can block it.
    #[must_use]
    pub fn definitions_supersede_sections(&self) -> bool {
        matches!(
            self.source_authority,
            ModelSourceAuthority::ProjectOwned { .. }
        ) && !self.model_definition_metadata.is_empty()
    }

    /// Whether the authenticated closure carries a section by this name.
    ///
    /// Parsed, not populated: a section that declares nothing is still a
    /// section a corner may bind to, and the run accepts it.
    #[must_use]
    pub fn carries_section(&self, section: &str) -> bool {
        self.section_models
            .keys()
            .any(|candidate| candidate.eq_ignore_ascii_case(section))
    }

    /// The sections a run must resolve before it can expand `corner`, or the
    /// reason it cannot expand it at all.
    ///
    /// An empty list means the run checks no section bindings for this
    /// library — either because authored definitions supersede them, or
    /// because the corner is bound to no source at all.
    pub fn required_sections(&self, corner: &ProcessCorner) -> Result<Vec<String>, String> {
        if self.definitions_supersede_sections() {
            return Ok(Vec::new());
        }
        let mut sections = BTreeMap::<String, String>::new();
        for binding in corner.effective_section_bindings() {
            sections
                .entry(binding.section.to_ascii_lowercase())
                .or_insert(binding.section);
        }
        if sections.is_empty() {
            // A corner that names no section and has no retained source is not
            // bound to a file at all; there is nothing for a run to resolve
            // and nothing to report.
            if corner.file_path.is_none() && self.source_closure.is_empty() {
                return Ok(Vec::new());
            }
            return Err(format!(
                "corner '{}' has no executable section bindings",
                corner.name
            ));
        }
        Ok(sections.into_values().collect())
    }

    /// Why a run cannot expand `corner`, or `None` when it can.
    pub fn expansion_blocker(&self, corner: &ProcessCorner) -> Option<String> {
        let sections = match self.required_sections(corner) {
            Ok(sections) => sections,
            Err(reason) => return Some(reason),
        };
        let mut missing = sections
            .into_iter()
            .filter(|section| !self.carries_section(section))
            .collect::<Vec<_>>();
        if missing.is_empty() {
            return None;
        }
        missing.sort();
        missing.dedup();
        Some(format!(
            "the retained closure carries no section named {}",
            missing
                .iter()
                .map(|section| format!("'{section}'"))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}
