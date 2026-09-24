//! The data files file-backed sources read.
//!
//! A `PWL FILE=` card names a table, and three questions about it are asked
//! while a deck is written: which file the card is pointed at, whether that
//! file will load, and whether the run should be told the table came from
//! somewhere other than where the card says. The rule that answers the first
//! is [`table_route`](crate::simulation::table_route)'s, shared with every
//! preview; this binds it to what the generator was given — the project's data
//! folder and its stimulus library — so a run and a preview cannot read
//! different files.

use super::*;
use crate::simulation::table_route::{self, TableRoute};

impl<'a> NetlistGenerator<'a> {
    /// The data-file reference a file-backed source stores.
    fn stored_data_file<'c>(
        component: &'c Component,
        params: &'c std::collections::HashMap<String, String>,
    ) -> &'c str {
        params
            .get("file")
            .map_or(component.value.as_str(), String::as_str)
            .trim()
    }

    /// The file a source's stored data-file reference is read from.
    ///
    /// Project files record the path relative to the project folder so a design
    /// survives being moved or handed to someone else; the engine opens what it
    /// is given and does not resolve against the deck, so the reference is made
    /// absolute against the bound data root. When the file it names is not
    /// there and the source's stimulus definition retains the table, the route
    /// is that retained copy instead.
    ///
    /// The second half is why a retained copy could not stand in, when it
    /// could not.
    pub(super) fn source_table_route(
        &self,
        component: &Component,
        stored: &str,
    ) -> (TableRoute, Option<String>) {
        table_route::route_retaining(
            stored,
            self.hierarchy.and_then(HierarchySource::data_root),
            self.hierarchy
                .and_then(|hierarchy| hierarchy.retained_table(component)),
        )
    }

    /// Reason a file-backed PWL source cannot run, or `None` when its table is
    /// present — as the file the card names, or as the copy its stimulus
    /// definition retains — and is one the engine's loader reads.
    ///
    /// The engine already refuses to build a circuit whose PWL file will not
    /// load, but that happens after a run has been dispatched and reports a
    /// resolved absolute path the user never typed. Catching it here names the
    /// component and blocks the run before it starts.
    pub(super) fn pwl_data_file_defect(
        &self,
        component: &Component,
        params: &std::collections::HashMap<String, String>,
    ) -> Option<String> {
        let stored = Self::stored_data_file(component, params);
        if stored.is_empty() {
            return Some(format!(
                "{} '{}' has no data file selected",
                component.kind.display_name(),
                component.name
            ));
        }

        let (route, retained_failure) = self.source_table_route(component, stored);
        let unloadable = |path: &str| {
            table_route::engine_refusal(path).map(|refusal| {
                format!(
                    "{} '{}' data file '{}' is not a table the engine reads: {}",
                    component.kind.display_name(),
                    component.name,
                    stored,
                    refusal
                )
            })
        };
        // A definition's retained copy standing in for the file is a run that
        // can happen, and `pwl_table_notice` says that it did.
        let resolved = match route {
            TableRoute::Named(resolved) => resolved,
            TableRoute::Retained(retained) => return unloadable(&retained),
        };
        // Only a bound data root makes the reference checkable: without one a
        // relative path is resolved by the engine against its own working
        // directory, which is not this process's to test.
        if std::path::Path::new(&resolved).is_relative() {
            return None;
        }
        let retained_failure = retained_failure
            .map(|failure| format!("; {failure}"))
            .unwrap_or_default();
        match std::fs::metadata(&resolved) {
            Ok(metadata) if metadata.is_file() => unloadable(&resolved),
            Ok(_) => Some(format!(
                "{} '{}' data file '{}' is a directory{retained_failure}",
                component.kind.display_name(),
                component.name,
                stored
            )),
            Err(error) => Some(format!(
                "{} '{}' cannot read data file '{}': {}{retained_failure}",
                component.kind.display_name(),
                component.name,
                stored,
                error
            )),
        }
    }

    /// What a run should be told about where a file-backed source's table came
    /// from, or `None` when it is simply the file the card names.
    ///
    /// Two things are worth a line in the log. The run read the definition's
    /// retained copy because the named file is not reachable: the deck then
    /// carries a cache path nobody typed, and this is what explains it. Or the
    /// named file is there and no longer holds the bytes the definition
    /// retains: the run reads the file, as the card says, and the definition's
    /// digest describes something else.
    pub(super) fn pwl_table_notice(
        &self,
        component: &Component,
        params: &std::collections::HashMap<String, String>,
    ) -> Option<String> {
        let table = self.hierarchy?.retained_table(component)?;
        let definition = &component.stimulus_provenance.as_ref()?.definition;
        let stored = Self::stored_data_file(component, params);
        match self.source_table_route(component, stored).0 {
            TableRoute::Retained(_) => Some(format!(
                "{} '{}' reads the copy of '{}' that stimulus definition '{definition}' retains: \
                 the file the card names, '{stored}', is not reachable here",
                component.kind.display_name(),
                component.name,
                table.file_name,
            )),
            TableRoute::Named(named) => {
                (table_route::named_file_matches(&named, table) == Some(false)).then(|| {
                    format!(
                        "{} '{}' reads '{stored}', which no longer holds the bytes stimulus \
                             definition '{definition}' retains; the run uses the file",
                        component.kind.display_name(),
                        component.name,
                    )
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::StimulusLibrary;
    use crate::state::stimulus_library::definition::{RetainedPwlFile, StimulusDefinition};

    /// A library holding one `PWL FILE` definition that retains `contents`, and
    /// the placed source that has adopted it.
    fn adopted_pwl_file(file: &std::path::Path, contents: &str) -> (StimulusLibrary, Component) {
        let mut definition =
            StimulusDefinition::new("bridge_step", ComponentType::VoltageSourcePwlFile)
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
        state.components = vec![source];
        let buffers = HashMap::new();
        let hierarchy = HierarchySource::from_buffers(&buffers).with_stimulus_library(library);
        generate_netlist_hierarchical(&state, &[], &hierarchy)
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
        state.components = vec![source];
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
}
