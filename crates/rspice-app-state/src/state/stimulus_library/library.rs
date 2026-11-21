//! Project integration for the portable stimulus library.
pub use rspice_design::stimulus_library::library::StimulusLibrary;

#[cfg(test)]
mod tests {
    use super::super::definition::StimulusDefinition;
    use crate::state::{ComponentType, ProjectWorkspace};

    #[test]
    fn a_library_with_a_definition_round_trips_through_the_project_document() {
        let mut workspace = ProjectWorkspace::default();
        let mut definition = StimulusDefinition::new(
            "sensor_diff_1k",
            ComponentType::VoltageSourceSin,
            crate::state::stimulus_library::now_unix_ms,
        )
        .expect("definition");
        definition.value = "0".to_owned();
        definition.params = "va=3m freq=1k".to_owned();
        definition.purpose = "differential sensor drive".to_owned();
        workspace
            .content
            .stimulus_library
            .insert(definition)
            .expect("insert");

        let encoded = ron::ser::to_string(&workspace).expect("serialize");
        let decoded: ProjectWorkspace = ron::from_str(&encoded).expect("deserialize");
        assert_eq!(
            decoded.content.stimulus_library,
            workspace.content.stimulus_library
        );
        assert_eq!(
            decoded
                .content
                .stimulus_library
                .get("sensor_diff_1k")
                .map(StimulusDefinition::revision),
            Some(1)
        );
    }
}
