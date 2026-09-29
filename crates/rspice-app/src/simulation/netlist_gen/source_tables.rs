//! Source-file routing through the application host and extracted generator.

use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::StimulusLibrary;
    use crate::state::stimulus_library::definition::{RetainedPwlFile, StimulusDefinition};

    /// A library holding one `PWL FILE` definition that retains `contents`, and
    /// the placed source that has adopted it.
    fn adopted_pwl_file(file: &std::path::Path, contents: &str) -> (StimulusLibrary, Component) {
        let mut definition = StimulusDefinition::new(
            "bridge_step",
            ComponentType::VoltageSourcePwlFile,
            crate::state::stimulus_library::now_unix_ms,
        )
        .expect("a definition");
        definition.params = format!("file={}", file.display());
        definition.pwl_file = Some(RetainedPwlFile::new("step.csv", contents, 0));
        let mut source = Component::new(1, ComponentType::VoltageSourcePwlFile, Point::origin())
            .with_name_value("V1", "");
        definition.adopt_onto(&mut source).expect("adopt");
        let mut library = StimulusLibrary::default();
        library.insert(definition).expect("insert");
        (library, source)
    }

    fn netlist_with_library(library: &StimulusLibrary, source: Component) -> NetlistResult {
        let mut state = SchematicState::default();
        state.document_mut_for_test().components = vec![source];
        let buffers: HashMap<String, SchematicDocument> = HashMap::new();
        let hierarchy = HierarchySource::from_buffers(&buffers);
        let source_data = NetlistSourceData {
            files: &crate::simulation::table_route::SourceFiles,
            data_root: None,
            stimulus_library: Some(library),
        };
        generate_netlist_hierarchical(&state, &[], &hierarchy, &source_data)
    }

    fn scratch_folder(purpose: &str) -> std::path::PathBuf {
        let folder = crate::fixture_root::canonical_temp_dir()
            .join(format!("rspice-netlist-{purpose}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&folder).expect("a scratch folder");
        folder
    }

    /// A missing file on a source whose definition retains the table: the run
    /// reads the retained copy, the deck names a file that exists and holds
    /// exactly those bytes, and the log says whose bytes they are.
    #[test]
    fn a_missing_pwl_data_file_runs_from_the_copy_its_definition_retains() {
        let absent = std::env::temp_dir().join("rspice-no-such-waveform-51d0.csv");
        let contents = "0 0\n1e-9 1.5\n3e-9 0.25\n";
        let (library, source) = adopted_pwl_file(&absent, contents);
        let result = netlist_with_library(&library, source);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        let card = result
            .netlist
            .lines()
            .find(|line| line.starts_with("V1 "))
            .unwrap_or_else(|| panic!("{}", result.netlist));
        let path = card
            .split("FILE=\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .unwrap_or_else(|| panic!("{card}"));
        assert_eq!(
            std::fs::read_to_string(path).expect("the deck names a readable file"),
            contents
        );
        assert!(
            result.warnings.iter().any(|warning| {
                warning.contains("V1")
                    && warning.contains("bridge_step")
                    && warning.contains("retains")
            }),
            "{:?}",
            result.warnings
        );
        rspice_core::netlist::parse_netlist(&result.netlist).expect("engine must accept the card");
    }

    /// The named file wins while it is there. When it no longer holds what the
    /// definition retains, the run still reads it — the card says so — and the
    /// log says the two have parted.
    #[test]
    fn a_named_pwl_file_that_parted_from_its_retained_copy_is_reported() {
        let folder = scratch_folder("parted");
        let named = folder.join("step.csv");
        std::fs::write(&named, "0 0\n1e-9 2\n").expect("a named table");

        let (library, source) = adopted_pwl_file(&named, "0 0\n1e-9 1\n");
        let result = netlist_with_library(&library, source);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(
            result
                .netlist
                .contains(&format!("FILE=\"{}\"", named.display())),
            "{}",
            result.netlist
        );
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning.contains("V1") && warning.contains("no longer holds")),
            "{:?}",
            result.warnings
        );

        // The same bytes in both places is the ordinary case and says nothing.
        let (library, source) = adopted_pwl_file(&named, "0 0\n1e-9 2\n");
        let result = netlist_with_library(&library, source);
        assert!(
            !result
                .warnings
                .iter()
                .any(|warning| warning.contains("retains")),
            "{:?}",
            result.warnings
        );
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// A file that is there and is not a table stops the run here, in the
    /// loader's own words, rather than after dispatch.
    #[test]
    fn a_pwl_data_file_the_engine_cannot_load_blocks_the_run() {
        let folder = scratch_folder("unloadable");
        let named = folder.join("step.csv");
        std::fs::write(&named, "0 0\n200u 1\n").expect("a table with a SPICE suffix");
        let mut state = SchematicState::default();
        let mut source = Component::new(1, ComponentType::VoltageSourcePwlFile, Point::origin())
            .with_name_value("V1", "");
        source.params = format!("file={}", named.display());
        state.document_mut_for_test().components = vec![source];
        let result = generate_netlist(&state);
        assert!(
            result.errors.iter().any(|error| {
                error.contains("V1") && error.contains("not a table the engine reads")
            }),
            "{:?}",
            result.errors
        );
        let _ = std::fs::remove_dir_all(&folder);
    }

    fn netlist_for(components: Vec<Component>) -> String {
        let mut schematic = SchematicState::default();
        schematic.document_mut_for_test().components = components;
        generate_netlist(&schematic).netlist
    }

    fn pwl_file_source(params: &str) -> Component {
        let mut source = Component::new(1, ComponentType::VoltageSourcePwlFile, Point::origin())
            .with_name_value("V1", "");
        source.params = params.to_owned();
        source
    }

    /// The card must be spelled the way the netlist reader accepts it, quotes
    /// and all, and every modifier the sheet exposes must survive the trip.
    #[test]
    fn pwl_file_source_emits_the_readers_spelling() {
        let netlist = netlist_for(vec![pwl_file_source(
            "file=wave.csv td=1u r=0 tscale=2 vscale=3 toffset=1n voffset=0.5",
        )]);
        let card = netlist
            .lines()
            .find(|line| line.starts_with("V1 "))
            .unwrap_or_else(|| panic!("{netlist}"));
        assert!(
            card.ends_with(
                "PWL FILE=\"wave.csv\" TD=1u R=0 TSCALE=2 VSCALE=3 TOFFSET=1n VOFFSET=0.5"
            ),
            "{card}"
        );
        rspice_core::netlist::parse_netlist(&netlist).expect("engine must accept the card");
    }

    /// An untouched modifier has no business on the card: TSCALE and VSCALE are
    /// unset at one, the offsets and delay at zero, and R when it is blank.
    #[test]
    fn unset_pwl_file_modifiers_stay_off_the_card() {
        let netlist = netlist_for(vec![pwl_file_source(
            "file=wave.csv td=0 r= tscale=1 vscale=1 toffset=0 voffset=0",
        )]);
        let card = netlist
            .lines()
            .find(|line| line.starts_with("V1 "))
            .unwrap_or_else(|| panic!("{netlist}"));
        assert!(card.ends_with("PWL FILE=\"wave.csv\""), "{card}");
        rspice_core::netlist::parse_netlist(&netlist).expect("engine must accept the card");
    }

    /// A source with no file selected cannot run, and saying so beats emitting
    /// a card the engine will reject with a path the user never typed.
    #[test]
    fn a_pwl_file_source_without_a_file_blocks_the_run() {
        let mut state = SchematicState::default();
        state.document_mut_for_test().components = vec![pwl_file_source("td=1u")];
        let result = generate_netlist(&state);
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.contains("V1") && error.contains("no data file")),
            "{:?}",
            result.errors
        );
    }

    /// An absolute reference is checkable, so a missing file stops the run
    /// here rather than deep inside the engine's circuit build.
    #[test]
    fn a_missing_pwl_data_file_blocks_the_run() {
        let absent = std::env::temp_dir().join("rspice-no-such-waveform-9c1f.csv");
        let mut state = SchematicState::default();
        state.document_mut_for_test().components =
            vec![pwl_file_source(&format!("file={}", absent.display()))];
        let result = generate_netlist(&state);
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.contains("V1") && error.contains("cannot read data file")),
            "{:?}",
            result.errors
        );
    }
}
