//! Case-sensitive Spectre names.
//!
//! Spectre names are case-sensitive: `a` and `A` are two nodes, `r0` and `R0`
//! two instances. Every name the adapter lowers reaches a reader that ignores
//! case, which would merge each such pair without a word. So the adapter
//! records every Spectre-language spelling it reads, per namespace and scope,
//! and refuses any two that differ only in case before a card is lowered. One
//! spelling used consistently is accepted as written; nothing is renamed.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::{SpectreModelAdapterError, error, values::spectre_value_identifiers};

/// A namespace Spectre resolves names in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum SpectreNameKind {
    /// Nets of the top level or of one subckt body.
    Node,
    /// Instance and analysis names, and the `dev=`/`iprobe=` references to
    /// them.
    Instance,
    /// Model and subckt names, and the instance masters that name them. They
    /// share one namespace because an instance's master may be either.
    Master,
    /// Netlist and subckt parameters, and every name a value refers to.
    Parameter,
}

impl SpectreNameKind {
    const fn noun(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Instance => "instance",
            Self::Master => "model or subckt",
            Self::Parameter => "parameter",
        }
    }
}

/// Where a name is resolved: the top level, or inside one subckt body.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SpectreNameScope {
    Top,
    Subcircuit(String),
}

#[derive(Debug, Clone)]
struct SpectreName {
    kind: SpectreNameKind,
    scope: SpectreNameScope,
    spelling: String,
    line: usize,
    /// The library section the name was read in. Sections are alternatives,
    /// so names in two different sections never meet.
    section: Option<String>,
    /// Declared rather than referred to. A subckt body sees the top level's
    /// declared parameters.
    declared: bool,
    /// A `global` node, which every subckt body sees.
    global: bool,
}

/// One instance parameter of an instance whose master may be a subckt. Its
/// name refers to that subckt's parameters, which only the whole file knows.
#[derive(Debug, Clone)]
struct SubcircuitOverride {
    master: String,
    parameter: String,
    line: usize,
    section: Option<String>,
}

/// Every Spectre-language name one source spells, in reading order.
#[derive(Debug, Default)]
pub(super) struct SpectreNames {
    names: Vec<SpectreName>,
    subcircuits: HashSet<String>,
    overrides: Vec<SubcircuitOverride>,
    scopes: Vec<String>,
    section: Option<String>,
}

impl SpectreNames {
    fn scope(&self) -> SpectreNameScope {
        self.scopes.last().map_or(SpectreNameScope::Top, |name| {
            SpectreNameScope::Subcircuit(name.clone())
        })
    }

    fn record(&mut self, kind: SpectreNameKind, spelling: &str, line: usize, declared: bool) {
        self.record_in(kind, self.scope(), spelling, line, declared, false);
    }

    fn record_in(
        &mut self,
        kind: SpectreNameKind,
        scope: SpectreNameScope,
        spelling: &str,
        line: usize,
        declared: bool,
        global: bool,
    ) {
        self.names.push(SpectreName {
            kind,
            scope,
            spelling: spelling.to_owned(),
            line,
            section: self.section.clone(),
            declared,
            global,
        });
    }

    pub(super) fn enter_section(&mut self, name: &str) {
        self.section = Some(name.to_owned());
    }

    pub(super) fn leave_section(&mut self) {
        self.section = None;
    }

    pub(super) fn model(&mut self, name: &str, line: usize) {
        self.record_in(
            SpectreNameKind::Master,
            SpectreNameScope::Top,
            name,
            line,
            true,
            false,
        );
    }

    /// Open a subckt body: its name is a master, its ports are its first
    /// nodes, and its defaults are its first parameters.
    pub(super) fn enter_subcircuit(&mut self, name: &str, ports: &[String], line: usize) {
        self.record_in(
            SpectreNameKind::Master,
            SpectreNameScope::Top,
            name,
            line,
            true,
            false,
        );
        self.subcircuits.insert(name.to_owned());
        self.scopes.push(name.to_owned());
        for port in ports {
            self.node(port, line);
        }
    }

    pub(super) fn leave_subcircuit(&mut self) {
        self.scopes.pop();
    }

    pub(super) fn node(&mut self, spelling: &str, line: usize) {
        self.record(SpectreNameKind::Node, spelling, line, true);
    }

    pub(super) fn global_node(&mut self, spelling: &str, line: usize) {
        self.record_in(
            SpectreNameKind::Node,
            SpectreNameScope::Top,
            spelling,
            line,
            true,
            true,
        );
    }

    pub(super) fn instance(&mut self, spelling: &str, line: usize, declared: bool) {
        self.record(SpectreNameKind::Instance, spelling, line, declared);
    }

    pub(super) fn master_reference(&mut self, spelling: &str, line: usize) {
        self.record_in(
            SpectreNameKind::Master,
            SpectreNameScope::Top,
            spelling,
            line,
            false,
            false,
        );
    }

    pub(super) fn parameter(&mut self, spelling: &str, line: usize, declared: bool) {
        self.record(SpectreNameKind::Parameter, spelling, line, declared);
    }

    /// Every name a value refers to, read as a parameter reference.
    pub(super) fn value(&mut self, value: &str, line: usize) {
        for identifier in spectre_value_identifiers(value) {
            self.parameter(identifier, line, false);
        }
    }

    pub(super) fn subcircuit_override(&mut self, master: &str, parameter: &str, line: usize) {
        self.overrides.push(SubcircuitOverride {
            master: master.to_owned(),
            parameter: parameter.to_owned(),
            line,
            section: self.section.clone(),
        });
    }

    /// Refuse the first two spellings that differ only in case within one
    /// namespace and scope, naming both with their lines. What passes is the
    /// part of the source a deck that includes it shares: its top-level names.
    pub(super) fn check(
        mut self,
        path: &Path,
    ) -> Result<SpectreSourceNames, SpectreModelAdapterError> {
        for entry in std::mem::take(&mut self.overrides) {
            if self.subcircuits.contains(&entry.master) {
                self.names.push(SpectreName {
                    kind: SpectreNameKind::Parameter,
                    scope: SpectreNameScope::Subcircuit(entry.master),
                    spelling: entry.parameter,
                    line: entry.line,
                    section: entry.section,
                    declared: false,
                    global: false,
                });
            }
        }

        let mut groups = HashMap::<_, Vec<&SpectreName>>::new();
        for name in &self.names {
            groups
                .entry((
                    name.kind,
                    name.scope.clone(),
                    name.spelling.to_ascii_lowercase(),
                ))
                .or_default()
                .push(name);
        }
        // A subckt body sees the global nodes and the top level's declared
        // parameters, so a spelling there meets them as well.
        let inherited = self
            .names
            .iter()
            .filter(|name| name.scope == SpectreNameScope::Top)
            .filter(|name| match name.kind {
                SpectreNameKind::Node => name.global,
                SpectreNameKind::Parameter => name.declared,
                SpectreNameKind::Instance | SpectreNameKind::Master => false,
            })
            .collect::<Vec<_>>();
        for ((kind, scope, key), members) in &mut groups {
            if matches!(scope, SpectreNameScope::Subcircuit(_)) {
                members.extend(
                    inherited.iter().copied().filter(|name| {
                        name.kind == *kind && name.spelling.eq_ignore_ascii_case(key)
                    }),
                );
            }
        }

        let mut first: Option<(&SpectreName, &SpectreName, &SpectreNameScope)> = None;
        for ((_, scope, _), members) in &groups {
            // The first use of each spelling stands for every later one, so a
            // name used ten thousand times costs ten thousand steps.
            let mut spellings = Vec::<&SpectreName>::new();
            for member in members {
                for spelling in &spellings {
                    if spelling.spelling == member.spelling || !sections_meet(spelling, member) {
                        continue;
                    }
                    let (earlier, later) = if spelling.line <= member.line {
                        (*spelling, *member)
                    } else {
                        (*member, *spelling)
                    };
                    if first.is_none_or(|(first_earlier, first_later, first_scope)| {
                        clash_order(earlier, later, scope)
                            < clash_order(first_earlier, first_later, first_scope)
                    }) {
                        first = Some((earlier, later, scope));
                    }
                }
                if !spellings.iter().any(|spelling| {
                    spelling.spelling == member.spelling && spelling.section == member.section
                }) {
                    spellings.push(member);
                }
            }
        }
        if let Some((earlier, later, scope)) = first {
            let within = match scope {
                SpectreNameScope::Top => String::new(),
                SpectreNameScope::Subcircuit(name) => format!(" inside subckt '{name}'"),
            };
            return Err(clash(
                earlier.kind,
                &within,
                (&earlier.spelling, path, earlier.line),
                (&later.spelling, path, later.line),
            ));
        }

        // A subckt body's reference to a parameter it does not declare names
        // a top-level one, which may live in another file.
        let declared_locally = self
            .names
            .iter()
            .filter(|name| name.kind == SpectreNameKind::Parameter && name.declared)
            .filter_map(|name| match &name.scope {
                SpectreNameScope::Subcircuit(subcircuit) => {
                    Some((subcircuit.clone(), name.spelling.to_ascii_lowercase()))
                }
                SpectreNameScope::Top => None,
            })
            .collect::<HashSet<_>>();
        let names = self
            .names
            .into_iter()
            .filter(|name| match &name.scope {
                SpectreNameScope::Top => true,
                SpectreNameScope::Subcircuit(subcircuit) => {
                    name.kind == SpectreNameKind::Parameter
                        && !declared_locally
                            .contains(&(subcircuit.clone(), name.spelling.to_ascii_lowercase()))
                }
            })
            .map(|name| SpectreSourceName {
                kind: name.kind,
                spelling: name.spelling,
                line: name.line,
                section: name.section,
            })
            .collect();
        Ok(SpectreSourceNames {
            path: path.to_path_buf(),
            names,
        })
    }
}

/// The refusal for two spellings of one name, each with its file and line.
fn clash(
    kind: SpectreNameKind,
    within: &str,
    (earlier, earlier_path, earlier_line): (&str, &Path, usize),
    (later, later_path, later_line): (&str, &Path, usize),
) -> SpectreModelAdapterError {
    let noun = kind.noun();
    error(
        later_line,
        format!(
            "Spectre {noun} names '{earlier}' ({}:{earlier_line}) and '{later}' ({}:{later_line}) differ only in case{within}; Spectre names are case-sensitive, so they are two different {noun}s, but RSpice reads names without case and would merge them; spell one {noun} one way, and give different {noun}s different names",
            earlier_path.display(),
            later_path.display(),
        ),
    )
}

/// The order clashes are reported in: the one met first in reading order. A
/// clash of two top-level names is reported at the top level rather than in a
/// subckt body that sees them, and the rest only makes the choice stable.
fn clash_order<'a>(
    earlier: &'a SpectreName,
    later: &'a SpectreName,
    scope: &SpectreNameScope,
) -> (usize, usize, bool, SpectreNameKind, &'a str, &'a str) {
    (
        later.line,
        earlier.line,
        matches!(scope, SpectreNameScope::Subcircuit(_)),
        earlier.kind,
        &earlier.spelling,
        &later.spelling,
    )
}

fn sections_meet(first: &SpectreName, second: &SpectreName) -> bool {
    match (&first.section, &second.section) {
        (Some(first), Some(second)) => first == second,
        _ => true,
    }
}

#[derive(Debug, Clone)]
struct SpectreSourceName {
    kind: SpectreNameKind,
    spelling: String,
    line: usize,
    section: Option<String>,
}

/// The top-level names of one Spectre source: what a deck that reads it,
/// directly or through an include, shares with it.
#[derive(Debug)]
pub(crate) struct SpectreSourceNames {
    path: PathBuf,
    names: Vec<SpectreSourceName>,
}

/// Every top-level Spectre name one deck expansion has read so far, across
/// the deck and every source it includes. An included source shares the
/// including deck's top level, so a node `a` in one file and `A` in another
/// would merge exactly as they would within one file.
#[derive(Debug, Default)]
pub(crate) struct SpectreDeckNames {
    spellings: HashMap<(SpectreNameKind, String), (String, PathBuf, usize)>,
}

impl SpectreDeckNames {
    pub(crate) fn clear(&mut self) {
        self.spellings.clear();
    }

    /// Add the names of one source read with `selected_section` (a library
    /// read through `.lib`) or whole, and refuse the first that another file
    /// spells in a different case. A source's own clashes were refused when
    /// it was adapted, so each name has one spelling per source here.
    pub(crate) fn merge(
        &mut self,
        source: SpectreSourceNames,
        selected_section: Option<&str>,
    ) -> Result<(), SpectreModelAdapterError> {
        for name in source.names {
            let read = match (&name.section, selected_section) {
                (None, _) => true,
                (Some(section), Some(selected)) => section.eq_ignore_ascii_case(selected),
                (Some(_), None) => false,
            };
            if !read {
                continue;
            }
            let key = (name.kind, name.spelling.to_ascii_lowercase());
            match self.spellings.get(&key) {
                Some((spelling, path, line)) if *spelling != name.spelling => {
                    return Err(clash(
                        name.kind,
                        " across the deck's files",
                        (spelling, path, *line),
                        (&name.spelling, &source.path, name.line),
                    ));
                }
                Some(_) => {}
                None => {
                    self.spellings
                        .insert(key, (name.spelling, source.path.clone(), name.line));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::super::adapt_spectre_model_library;

    fn refusal(source: &str) -> String {
        adapt_spectre_model_library(Path::new("deck.scs"), source)
            .expect_err("a case clash is refused")
            .to_string()
    }

    fn accepted(source: &str) -> String {
        adapt_spectre_model_library(Path::new("deck.scs"), source)
            .unwrap_or_else(|error| panic!("one spelling per name is accepted: {error}"))
            .into_owned()
    }

    #[test]
    fn nodes_that_differ_only_in_case_are_refused_with_both_lines() {
        let message = refusal(
            "simulator lang=spectre\nv2 (a 0) vsource dc=1\nra (a 0) resistor r=1k\nv3 (A b) vsource dc=2\nrb (b 0) resistor r=1k\nop1 dc\n",
        );
        assert!(
            message.starts_with("line 4: ")
                && message.contains("Spectre node names 'a' (deck.scs:2) and 'A' (deck.scs:4)"),
            "{message}"
        );
    }

    #[test]
    fn instances_that_differ_only_in_case_are_refused_with_both_lines() {
        let message = refusal(
            "simulator lang=spectre\nV0 (net1 0) vsource dc=1\nr0 (net4 0) resistor r=1k\nR0 (net1 net4) resistor r=1k\nN1 (a 0) resistor r=1k\nn1 (A 0) resistor r=3k\nop1 dc\n",
        );
        assert!(
            message.contains("Spectre instance names 'r0' (deck.scs:3) and 'R0' (deck.scs:4)"),
            "{message}"
        );
        // An analysis's dev= names an instance in that same namespace.
        let message = refusal(
            "simulator lang=spectre\nv1 (a 0) vsource dc=1\nr1 (a 0) resistor r=1k\nsweep1 dc dev=V1 start=0 stop=1 step=0.5\n",
        );
        assert!(
            message.contains("Spectre instance names 'v1' (deck.scs:2) and 'V1' (deck.scs:4)"),
            "{message}"
        );
    }

    #[test]
    fn one_spelling_used_consistently_is_accepted_as_written() {
        let adapted = accepted(
            "simulator lang=spectre\nparameters Rload=1k\nVin (In 0) vsource dc=1\nRa (In Out) resistor r=Rload\nRb (Out 0) resistor r=Rload\nop1 dc\n",
        );
        assert!(adapted.contains("RRa In Out Rload"), "{adapted}");
        assert!(adapted.contains(".param Rload=1k"), "{adapted}");
        // A primitive's type= names a keyword, not a parameter.
        accepted(
            "simulator lang=spectre\nparameters Sine=1\nv1 (a 0) vsource type=sine ampl=Sine freq=1k\n",
        );
    }

    #[test]
    fn subckt_bodies_are_their_own_node_and_instance_scopes() {
        accepted(
            "simulator lang=spectre\nsubckt cell (a)\n  r1 (a 0) resistor r=1k\nends cell\nsubckt other (A)\n  R1 (A 0) resistor r=1k\nends other\nx1 (n) cell\nx2 (N2) other\nr1 (n 0) resistor r=1k\n",
        );
        let message = refusal(
            "simulator lang=spectre\nsubckt cell (a)\n  r1 (a b) resistor r=1k\n  r2 (B 0) resistor r=1k\nends cell\n",
        );
        assert!(
            message.contains(
                "Spectre node names 'b' (deck.scs:3) and 'B' (deck.scs:4) differ only in case inside subckt 'cell'"
            ),
            "{message}"
        );
        // A global node is seen inside every subckt body.
        let message = refusal(
            "simulator lang=spectre\nglobal 0 vdd\nsubckt cell (a)\n  r1 (a VDD) resistor r=1k\nends cell\n",
        );
        assert!(
            message.contains("'vdd' (deck.scs:2) and 'VDD' (deck.scs:4)"),
            "{message}"
        );
    }

    #[test]
    fn models_and_subckts_share_one_master_namespace_with_their_references() {
        let message = refusal("simulator lang=spectre\nmodel dm diode is=1e-14\nd1 (a 0) DM\n");
        assert!(
            message
                .contains("Spectre model or subckt names 'dm' (deck.scs:2) and 'DM' (deck.scs:3)"),
            "{message}"
        );
        let message = refusal(
            "simulator lang=spectre\nsubckt load (a)\n  r1 (a 0) resistor r=1k\nends load\nmodel Load diode is=1e-14\n",
        );
        assert!(
            message.contains("'load' (deck.scs:2) and 'Load' (deck.scs:5)"),
            "{message}"
        );
    }

    #[test]
    fn parameters_clash_with_their_references_and_subckt_overrides() {
        let message =
            refusal("simulator lang=spectre\nparameters rr=1k gg=2\nr1 (a 0) resistor r=RR*gg\n");
        assert!(
            message.contains("Spectre parameter names 'rr' (deck.scs:2) and 'RR' (deck.scs:3)"),
            "{message}"
        );
        let message = refusal("simulator lang=spectre\nparameters rr=1k\nparameters RR=2k\n");
        assert!(
            message.contains("'rr' (deck.scs:2) and 'RR' (deck.scs:3)"),
            "{message}"
        );
        // An instance parameter of a subckt instance names that subckt's
        // parameter, even when the subckt is declared after it.
        let message = refusal(
            "simulator lang=spectre\nx1 (n1) load RV=2k\nsubckt load (a)\nparameters rv=1k\n  r1 (a 0) resistor r=rv\nends load\n",
        );
        assert!(
            message.contains(
                "'RV' (deck.scs:2) and 'rv' (deck.scs:4) differ only in case inside subckt 'load'"
            ),
            "{message}"
        );
        // A subckt body sees the top level's parameters.
        let message = refusal(
            "simulator lang=spectre\nparameters gain=2\nsubckt amp (a)\n  r1 (a 0) resistor r=Gain*1k\nends amp\n",
        );
        assert!(
            message.contains("'gain' (deck.scs:2) and 'Gain' (deck.scs:4)"),
            "{message}"
        );
    }

    #[test]
    fn library_sections_are_alternatives_and_never_clash() {
        accepted(
            "simulator lang=spectre\nsection tt\nmodel nch bsim4 type=n\nendsection tt\nsection ff\nmodel NCH bsim4 type=n\nendsection ff\n",
        );
    }

    #[test]
    fn keywords_and_primitives_keep_their_exact_spectre_case() {
        let message = refusal("simulator lang=spectre\nModel dm diode is=1e-14\n");
        assert!(
            message.contains("'Model' is not the Spectre keyword 'model'"),
            "{message}"
        );
        let message = refusal("simulator lang=spectre\nr1 (a 0) Resistor r=1k\n");
        assert!(
            message.contains("master 'Resistor' is not declared")
                && message.contains("so it is not 'resistor'"),
            "{message}"
        );
        // A deck master spelled like a primitive in another case is the
        // deck's own, as it is in Spectre.
        let adapted = accepted(
            "simulator lang=spectre\nsubckt Resistor (a b)\n  r1 (a b) resistor r=2k\nends Resistor\nx1 (n 0) Resistor\n",
        );
        assert!(adapted.contains("Xx1 n 0 Resistor"), "{adapted}");
    }
}
