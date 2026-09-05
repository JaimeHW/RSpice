//! The set of definitions one project owns.
//!
//! The library is a field of `ProjectWorkspace`, not a file beside it. A
//! sidecar would spend from the durable-path budget every save already hangs
//! sidecars off, and it would give a project two halves that can be separated;
//! definitions ride the project document exactly as report and visualization
//! documents do, and a project written before the library existed loads with an
//! empty one.

use serde::{Deserialize, Serialize};

use super::definition::{StimulusDefinition, StimulusDefinitionError, StimulusFamily};
use super::draft::DefinitionDraft;
use crate::state::ComponentType;

/// Every stimulus definition this project owns, in the order they are listed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StimulusLibrary {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    definitions: Vec<StimulusDefinition>,
}

impl StimulusLibrary {
    /// Whether this project has authored any stimulus at all.
    ///
    /// Named rather than derived because it is the `skip_serializing_if` the
    /// project field uses: an untouched library must leave no trace in the
    /// document, so a project that never opened the workspace round-trips byte
    /// for byte.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// How many definitions the library holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Every definition, in list order.
    #[must_use]
    pub fn definitions(&self) -> &[StimulusDefinition] {
        &self.definitions
    }

    /// The definition of that name, if the library holds one.
    ///
    /// Names are compared case-insensitively, because SPICE reads instance and
    /// source names that way and a library that let `VDD` and `vdd` coexist
    /// would produce two definitions that place the same card.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&StimulusDefinition> {
        self.definitions
            .iter()
            .find(|definition| definition.name().eq_ignore_ascii_case(name))
    }

    /// Add a definition, refusing a name the library already holds.
    pub fn insert(
        &mut self,
        definition: StimulusDefinition,
    ) -> Result<(), StimulusDefinitionError> {
        if self.get(definition.name()).is_some() {
            return Err(StimulusDefinitionError::DuplicateName(
                definition.name().to_owned(),
            ));
        }
        self.definitions.push(definition);
        Ok(())
    }

    /// Publish the draft's working record as the next revision.
    ///
    /// Returns the revision the library now holds. Adopters are deliberately
    /// untouched: they own their cards, so publishing moves the library and
    /// leaves every instance reading `behind` until someone re-adopts.
    pub fn apply(&mut self, draft: &mut DefinitionDraft) -> u32 {
        let published = draft.apply();
        let revision = published.revision();
        match self
            .definitions
            .iter_mut()
            .find(|definition| definition.name().eq_ignore_ascii_case(published.name()))
        {
            Some(existing) => *existing = published,
            None => self.definitions.push(published),
        }
        revision
    }

    /// Remove a definition. Adopters keep their cards and read
    /// `definition removed`.
    pub fn delete(&mut self, name: &str) -> Option<StimulusDefinition> {
        let index = self
            .definitions
            .iter()
            .position(|definition| definition.name().eq_ignore_ascii_case(name))?;
        Some(self.definitions.remove(index))
    }

    /// Copy a definition under a fresh name, starting its revisions over.
    ///
    /// A duplicate is a new definition rather than a branch of an old one, so
    /// it opens at `r1`: there is no history for it to be behind.
    pub fn duplicate(
        &mut self,
        name: &str,
    ) -> Result<&StimulusDefinition, StimulusDefinitionError> {
        let source = self
            .get(name)
            .ok_or_else(|| StimulusDefinitionError::Unknown(name.to_owned()))?;
        let mut copy = source.clone();
        copy.rename(self.unique_name(&format!("{}_copy", source.name())))?;
        copy.restart_revisions();
        self.definitions.push(copy);
        self.definitions.last().ok_or_else(|| {
            StimulusDefinitionError::Unknown("the definition just pushed".to_owned())
        })
    }

    /// A fresh `r1` definition of one family, under a generated unique name.
    pub fn new_definition(
        &mut self,
        component_type: ComponentType,
    ) -> Result<&StimulusDefinition, StimulusDefinitionError> {
        let stem = StimulusFamily::of(component_type)
            .ok_or(StimulusDefinitionError::NotASource(component_type))?
            .name_stem();
        let definition = StimulusDefinition::new(self.unique_name(stem), component_type)?;
        self.definitions.push(definition);
        self.definitions.last().ok_or_else(|| {
            StimulusDefinitionError::Unknown("the definition just pushed".to_owned())
        })
    }

    /// `stem`, or the first `stem_2`, `stem_3`, … the library does not hold.
    fn unique_name(&self, stem: &str) -> String {
        if self.get(stem).is_none() {
            return stem.to_owned();
        }
        (2_u32..)
            .map(|suffix| format!("{stem}_{suffix}"))
            .find(|candidate| self.get(candidate).is_none())
            .unwrap_or_else(|| stem.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ProjectWorkspace;

    #[test]
    fn a_project_written_before_the_library_existed_loads_with_an_empty_one() {
        // A document with no `stimulus_library` key is exactly what every
        // project saved before this field existed contains, and it is also what
        // an untouched project saves today, so one encoding proves both.
        let encoded = ron::ser::to_string(&ProjectWorkspace::default()).expect("serialize");
        assert!(
            !encoded.contains("stimulus_library"),
            "an untouched library must not be written: {encoded}"
        );

        let decoded: ProjectWorkspace = ron::from_str(&encoded).expect("deserialize");
        assert!(decoded.stimulus_library.is_empty());
    }

    #[test]
    fn a_library_with_a_definition_round_trips_through_the_project_document() {
        let mut workspace = ProjectWorkspace::default();
        let mut definition =
            StimulusDefinition::new("sensor_diff_1k", ComponentType::VoltageSourceSin)
                .expect("definition");
        definition.value = "0".to_owned();
        definition.params = "va=3m freq=1k".to_owned();
        definition.purpose = "differential sensor drive".to_owned();
        workspace
            .stimulus_library
            .insert(definition)
            .expect("insert");

        let encoded = ron::ser::to_string(&workspace).expect("serialize");
        let decoded: ProjectWorkspace = ron::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded.stimulus_library, workspace.stimulus_library);
        assert_eq!(
            decoded
                .stimulus_library
                .get("sensor_diff_1k")
                .map(StimulusDefinition::revision),
            Some(1)
        );
    }

    #[test]
    fn names_are_unique_case_insensitively() {
        let mut library = StimulusLibrary::default();
        library
            .insert(StimulusDefinition::new("vdd", ComponentType::VoltageSource).expect("ok"))
            .expect("insert");
        assert!(
            library
                .insert(StimulusDefinition::new("VDD", ComponentType::VoltageSource).expect("ok"))
                .is_err()
        );
        assert!(library.get("VdD").is_some());
    }

    #[test]
    fn a_new_definition_is_named_after_its_family_and_never_collides() {
        let mut library = StimulusLibrary::default();
        assert_eq!(
            library
                .new_definition(ComponentType::VoltageSourcePulse)
                .expect("new")
                .name(),
            "pulse"
        );
        assert_eq!(
            library
                .new_definition(ComponentType::CurrentSourcePulse)
                .expect("new")
                .name(),
            "pulse_2"
        );
        assert_eq!(
            library
                .new_definition(ComponentType::VoltageSourcePwlFile)
                .expect("new")
                .name(),
            "pwl_file"
        );
        assert!(library.new_definition(ComponentType::Resistor).is_err());
    }

    #[test]
    fn a_duplicate_opens_at_r1_under_a_fresh_name() {
        let mut library = StimulusLibrary::default();
        let mut definition =
            StimulusDefinition::new("vdd_operate", ComponentType::VoltageSource).expect("ok");
        definition.publish_next_revision();
        definition.publish_next_revision();
        library.insert(definition).expect("insert");

        let name = library
            .duplicate("vdd_operate")
            .expect("duplicate")
            .name()
            .to_owned();
        assert_eq!(name, "vdd_operate_copy");
        assert_eq!(
            library.get(&name).map(StimulusDefinition::revision),
            Some(1)
        );
        assert_eq!(
            library.duplicate("vdd_operate").expect("duplicate").name(),
            "vdd_operate_copy_2"
        );
        assert!(library.duplicate("nothing").is_err());
    }

    #[test]
    fn apply_publishes_the_next_revision_in_place() {
        let mut library = StimulusLibrary::default();
        let definition =
            StimulusDefinition::new("vdd_operate", ComponentType::VoltageSource).expect("ok");
        library.insert(definition.clone()).expect("insert");

        let mut draft = DefinitionDraft::new(definition);
        draft.edit(|working| working.value = "1.8".to_owned());
        assert_eq!(library.apply(&mut draft), 2);
        assert_eq!(library.len(), 1);
        assert_eq!(library.get("vdd_operate").expect("held").value, "1.8");
    }

    #[test]
    fn delete_removes_exactly_one_definition() {
        let mut library = StimulusLibrary::default();
        library
            .insert(StimulusDefinition::new("a", ComponentType::VoltageSource).expect("ok"))
            .expect("insert");
        library
            .insert(StimulusDefinition::new("b", ComponentType::VoltageSource).expect("ok"))
            .expect("insert");

        assert!(library.delete("A").is_some());
        assert!(library.delete("A").is_none());
        assert_eq!(library.len(), 1);
    }
}
