//! A path-backed root source as a parse read it, kept for replays of the deck.
//!
//! Every coordinate of a study (`.STEP`, `.TEMP`, `.DATA`, Monte Carlo trials,
//! sensitivity perturbations) replays the deck's root source with its own
//! parameter values. Reading the deck up to the parser does not depend on
//! those values: the includes and library sections it expands, the control
//! script it extracts and the control regions it sanitizes are the same for
//! every coordinate. A replay therefore reuses the first parse's reading
//! instead of reading and expanding the deck's files again, and every
//! coordinate reads them as the first parse did.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::control_source::{self, ControlScriptSource};
use super::include::{ExpandedSource, IncludeProcessor, ResolvedIncludeDependency};
use super::{
    ControlCommandRecord, Netlist, NetlistParseOptions, ParseDiagnostic, ParseWithAbortError,
};
use crate::abort_signal::AbortSignal;
use crate::config::ExpressionDialect;
use crate::resource::ResourceLimits;

/// A root source read up to the parser, with what decided how it was read.
#[derive(Debug)]
pub(crate) struct ReplaySource {
    input: Box<str>,
    file_path: PathBuf,
    expression_dialect: ExpressionDialect,
    retain_control_script: bool,
    resource_limits: ResourceLimits,
    /// What the include processor recorded reading.
    dependencies: Vec<ResolvedIncludeDependency>,
    /// The expanded, control-extracted and sanitized source the parser reads.
    pub(super) expanded: ExpandedSource,
    pub(super) diagnostics: Vec<ParseDiagnostic>,
    pub(super) dispositions: Vec<ControlCommandRecord>,
    pub(super) control_script: Option<Arc<ControlScriptSource>>,
    /// What every parse of `expanded` scans ahead of its first card, kept
    /// by the first parse for the replays after it.
    pub(super) scans: super::parser::RootScansCell,
}

impl ReplaySource {
    /// Read `input` from `file_path` through `processor`, or take `earlier`
    /// when it read the same root text from the same path the same way. The
    /// processor then holds the dependencies that reading recorded, as it
    /// would had it expanded them itself.
    pub(super) fn read_with_abort(
        processor: &mut IncludeProcessor,
        earlier: Option<Arc<Self>>,
        input: &str,
        file_path: &Path,
        options: &NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<Self>, ParseWithAbortError> {
        if let Some(earlier) = earlier.filter(|earlier| earlier.reads(input, file_path, options)) {
            processor.adopt_resolved_dependencies(earlier.dependencies.clone());
            return Ok(earlier);
        }
        let expanded = processor.expand_content_mapped_with_abort(input, file_path, abort)?;
        let (expanded, control_script) =
            control_source::prepare_expanded(expanded, options.retain_control_script, abort)?;
        let (expanded, diagnostics, dispositions) =
            Netlist::sanitize_expanded_source_with_abort(expanded, abort)?;
        Ok(Arc::new(Self {
            input: input.into(),
            file_path: file_path.to_path_buf(),
            expression_dialect: options.expression_dialect,
            retain_control_script: options.retain_control_script,
            resource_limits: options.resource_limits,
            dependencies: processor.resolved_dependencies().to_vec(),
            expanded,
            diagnostics,
            dispositions,
            control_script,
            scans: Default::default(),
        }))
    }

    /// Whether reading `input` from `file_path` under `options` reads what
    /// this reading read.
    fn reads(&self, input: &str, file_path: &Path, options: &NetlistParseOptions) -> bool {
        // A reading that kept the control script and found none met no
        // control region, so keeping or dropping one changes nothing.
        let control = self.retain_control_script == options.retain_control_script
            || self.retain_control_script && self.control_script.is_none();
        control
            && self.expression_dialect == options.expression_dialect
            && self.resource_limits == options.resource_limits
            && self.file_path == file_path
            && *self.input == *input
    }
}

#[cfg(test)]
mod tests {
    use crate::abort_signal::NoAbort;
    use crate::netlist::parser::ParameterOverride;
    use crate::netlist::{ElementKind, Netlist, NetlistParseOptions};

    fn resistances(netlist: &Netlist) -> Vec<(String, f64)> {
        netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. } => Some((element.name.clone(), *value)),
                _ => None,
            })
            .collect()
    }

    /// Every coordinate of a study may replay the deck from its one reading,
    /// and from the netlist that keeps it, each on a thread of its own.
    #[test]
    fn a_reading_and_its_netlist_are_shared_across_threads() {
        fn shared<T: Send + Sync>() {}
        shared::<super::ReplaySource>();
        shared::<Netlist>();
    }

    /// A replay reads what its first parse scanned the root's lines for (a
    /// card naming a parameter declared further down, the `.OPTIONS`
    /// temperature) with its own parameter values.
    #[test]
    fn a_replay_reads_the_scans_of_its_first_parse() {
        let path = std::env::temp_dir()
            .join(format!("rspice-replay-scans-{}", std::process::id()))
            .join("deck.cir");
        let deck =
            "scans\nR1 a 0 {rload}\nV1 a 0 1\n.param rload=1k\n.options temp=50\n.op\n.end\n";
        let netlist = Netlist::parse_with_path(deck, &path).expect("deck parses");
        assert_eq!(resistances(&netlist), [("R1".to_owned(), 1e3)]);
        assert_eq!(netlist.options.temp, Some(50.0));
        let overrides = [ParameterOverride {
            name: "RLOAD".to_owned(),
            value: 2e3,
            global: false,
            direction: false,
        }];
        for _ in 0..2 {
            let replayed = netlist
                .replay_root_source_with_parameter_overrides_and_abort(
                    deck,
                    NetlistParseOptions::default(),
                    &overrides,
                    &NoAbort,
                )
                .expect("replay parses");
            assert_eq!(resistances(&replayed), [("R1".to_owned(), 2e3)]);
            assert_eq!(replayed.options.temp, Some(50.0));
        }
    }

    #[test]
    fn a_replay_reads_the_deck_as_its_first_parse_read_it() {
        let dir = std::env::temp_dir().join(format!("rspice-replay-source-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let include = dir.join("parts.inc");
        std::fs::write(&include, "R2 a 0 {rload}\n").expect("include");
        let path = dir.join("deck.cir");
        let deck = "replay\n.param rload=1k\nV1 a 0 1\n.include parts.inc\n.op\n.end\n";
        let netlist = Netlist::parse_with_path(deck, &path).expect("deck parses");
        assert_eq!(resistances(&netlist), [("R2".to_owned(), 1e3)]);

        // The include changes after the first parse: a replay of the same
        // source keeps the first reading, with the new parameter value.
        std::fs::write(&include, "R3 a 0 {rload}\n").expect("include");
        let overrides = [ParameterOverride {
            name: "RLOAD".to_owned(),
            value: 2e3,
            global: false,
            direction: false,
        }];
        let replay = |input: &str| {
            netlist
                .replay_root_source_with_parameter_overrides_and_abort(
                    input,
                    NetlistParseOptions::default(),
                    &overrides,
                    &NoAbort,
                )
                .expect("replay parses")
        };
        let replayed = replay(deck);
        assert_eq!(resistances(&replayed), [("R2".to_owned(), 2e3)]);
        assert_eq!(
            replayed.included_source_paths(),
            netlist.included_source_paths()
        );

        // Another root text is read afresh.
        let edited = deck.replace(".op", "* edited\n.op");
        assert_eq!(resistances(&replay(&edited)), [("R3".to_owned(), 2e3)]);
        std::fs::remove_dir_all(&dir).ok();
    }
}
