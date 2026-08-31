//! Case discovery, stimulus parsing, and manifest cross-checking.
//!
//! Discovery sweeps the corpus directory and the manifest is checked against
//! the sweep in *both* directions. A case present on disk but absent from the
//! manifest is an error, and so is the reverse. One-directional checking is how
//! a corpus quietly shrinks: a renamed file drops out of the run and the
//! manifest keeps listing a name nothing looks for.
//!
//! The stimulus grammar is deliberately small — declarations, three scalars,
//! and vectors — because it is read by a hand-written parser and every
//! construct it admits is a construct that parser can be wrong about.

use super::oracle::VerilogEngine;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// The stimulus format version this parser accepts.
pub const STIMULUS_HEADER: &str = "# RSPICE-VERILOG-STIMULUS 1";

/// File name of the corpus manifest, relative to the corpus root.
pub const MANIFEST_FILE_NAME: &str = "verilog-manifest.tsv";

/// Whether a port is driven by the harness or observed by it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CaseDirection {
    /// Driven by the generated testbench from the vector columns.
    Input,
    /// Sampled into the trace.
    Output,
}

/// One port of the design under test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasePort {
    pub name: String,
    pub width: u32,
    pub direction: CaseDirection,
}

/// A free-running clock driven by the testbench rather than by vectors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    pub port: String,
    pub half_period: u64,
}

/// A parsed `.stim` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stimulus {
    pub module: String,
    pub ports: Vec<CasePort>,
    pub clock: Option<Clock>,
    pub step: u64,
    pub settle: u64,
    /// One entry per vector; each is one value per driven input, in
    /// declaration order, excluding the clock port.
    pub vectors: Vec<Vec<String>>,
}

impl Stimulus {
    /// Inputs the vector columns drive, in declaration order.
    ///
    /// The clock is an input port but is not one of these: it is driven by the
    /// testbench's own `always` block, so giving it a vector column would let a
    /// case fight its own clock generator.
    pub fn driven_inputs(&self) -> Vec<&CasePort> {
        self.ports
            .iter()
            .filter(|port| port.direction == CaseDirection::Input)
            .filter(|port| !self.is_clock(&port.name))
            .collect()
    }

    /// Ports sampled into the trace, in declaration order.
    pub fn observed_outputs(&self) -> Vec<&CasePort> {
        self.ports
            .iter()
            .filter(|port| port.direction == CaseDirection::Output)
            .collect()
    }

    pub fn is_clock(&self, name: &str) -> bool {
        self.clock.as_ref().is_some_and(|clock| clock.port == name)
    }

    fn port(&self, name: &str) -> Option<&CasePort> {
        self.ports.iter().find(|port| port.name == name)
    }
}

/// One corpus case: a design, its stimulus, and the oracles it is comparable
/// across.
#[derive(Debug, Clone)]
pub struct Case {
    pub name: String,
    pub source: PathBuf,
    pub stimulus_path: PathBuf,
    pub stimulus: Stimulus,
    pub oracles: Vec<VerilogEngine>,
    pub note: String,
}

impl Case {
    /// Whether this case may be compared against `engine`.
    pub fn admits(&self, engine: VerilogEngine) -> bool {
        self.oracles.contains(&engine)
    }
}

/// Every case in one corpus directory.
#[derive(Debug, Clone)]
pub struct Corpus {
    pub root: PathBuf,
    pub cases: Vec<Case>,
}

impl Corpus {
    /// Load and fully validate a corpus directory.
    ///
    /// Every consistency rule is checked here rather than at use, so a
    /// malformed corpus fails once with a specific message instead of
    /// surfacing later as a confusing oracle disagreement.
    pub fn load(root: &Path) -> Result<Self, CorpusError> {
        let manifest = parse_manifest(&root.join(MANIFEST_FILE_NAME))?;
        let discovered = discover_designs(root)?;

        let listed = manifest.keys().cloned().collect::<BTreeSet<_>>();
        let present = discovered.iter().cloned().collect::<BTreeSet<_>>();
        let unlisted = present.difference(&listed).cloned().collect::<Vec<_>>();
        if !unlisted.is_empty() {
            return Err(CorpusError::UnlistedDesigns(unlisted));
        }
        let orphaned = listed.difference(&present).cloned().collect::<Vec<_>>();
        if !orphaned.is_empty() {
            return Err(CorpusError::OrphanedManifestEntries(orphaned));
        }

        let mut cases = Vec::with_capacity(discovered.len());
        for file_name in discovered {
            let source = root.join(&file_name);
            let name = file_name
                .strip_suffix(".v")
                .unwrap_or(&file_name)
                .to_string();
            let stimulus_path = root.join(format!("{name}.stim"));
            let text =
                fs::read_to_string(&stimulus_path).map_err(|err| CorpusError::MissingStimulus {
                    case: name.clone(),
                    path: stimulus_path.clone(),
                    detail: err.to_string(),
                })?;
            let stimulus =
                parse_stimulus(&text).map_err(|detail| CorpusError::MalformedStimulus {
                    case: name.clone(),
                    detail,
                })?;

            let design =
                fs::read_to_string(&source).map_err(|err| CorpusError::UnreadableDesign {
                    case: name.clone(),
                    path: source.clone(),
                    detail: err.to_string(),
                })?;
            check_ports_against_design(&name, &stimulus, &design)?;

            let entry = &manifest[&file_name];
            cases.push(Case {
                name,
                source,
                stimulus_path,
                stimulus,
                oracles: entry.oracles.clone(),
                note: entry.note.clone(),
            });
        }

        if cases.is_empty() {
            return Err(CorpusError::Empty(root.to_path_buf()));
        }
        Ok(Self {
            root: root.to_path_buf(),
            cases,
        })
    }

    /// Cases comparable against `engine`, in corpus order.
    pub fn cases_for(&self, engine: VerilogEngine) -> Vec<&Case> {
        self.cases
            .iter()
            .filter(|case| case.admits(engine))
            .collect()
    }

    pub fn case(&self, name: &str) -> Option<&Case> {
        self.cases.iter().find(|case| case.name == name)
    }
}

#[derive(Debug, Clone)]
struct ManifestEntry {
    oracles: Vec<VerilogEngine>,
    note: String,
}

fn discover_designs(root: &Path) -> Result<Vec<String>, CorpusError> {
    let entries = fs::read_dir(root).map_err(|err| CorpusError::UnreadableRoot {
        path: root.to_path_buf(),
        detail: err.to_string(),
    })?;
    let mut designs = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if name.ends_with(".v") {
            designs.push(name.to_string());
        }
    }
    designs.sort();
    Ok(designs)
}

fn parse_manifest(path: &Path) -> Result<BTreeMap<String, ManifestEntry>, CorpusError> {
    let text = fs::read_to_string(path).map_err(|err| CorpusError::UnreadableManifest {
        path: path.to_path_buf(),
        detail: err.to_string(),
    })?;
    let mut entries = BTreeMap::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let (Some(design), Some(oracles)) = (fields.next(), fields.next()) else {
            return Err(CorpusError::MalformedManifest {
                line: index + 1,
                detail: "expected at least <design>\\t<oracles>".to_string(),
            });
        };
        let design = design.trim().to_string();
        let mut parsed = Vec::new();
        for token in oracles.split(',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            let engine = VerilogEngine::from_manifest_token(token).ok_or(
                CorpusError::MalformedManifest {
                    line: index + 1,
                    detail: format!("unknown oracle '{token}'"),
                },
            )?;
            parsed.push(engine);
        }
        if parsed.is_empty() {
            return Err(CorpusError::MalformedManifest {
                line: index + 1,
                detail: format!("'{design}' lists no oracles; remove the row or name one"),
            });
        }
        let note = fields.next().unwrap_or_default().trim().to_string();
        if entries
            .insert(
                design.clone(),
                ManifestEntry {
                    oracles: parsed,
                    note,
                },
            )
            .is_some()
        {
            return Err(CorpusError::MalformedManifest {
                line: index + 1,
                detail: format!("'{design}' is listed twice"),
            });
        }
    }
    Ok(entries)
}

/// Parse a `.stim` file.
///
/// Returns a human-readable reason rather than a typed error: every failure
/// here is a corpus authoring mistake read by a person fixing the file, and the
/// line-and-reason string is the whole of what they need.
pub fn parse_stimulus(text: &str) -> Result<Stimulus, String> {
    let mut lines = text.lines().enumerate();
    let header = lines
        .by_ref()
        .map(|(index, line)| (index, line.trim()))
        .find(|(_, line)| !line.is_empty())
        .ok_or_else(|| "file is empty".to_string())?;
    if header.1 != STIMULUS_HEADER {
        return Err(format!(
            "line {}: expected '{STIMULUS_HEADER}', found '{}'",
            header.0 + 1,
            header.1
        ));
    }

    let mut module = None::<String>;
    let mut ports = Vec::<CasePort>::new();
    let mut clock = None::<Clock>;
    let mut step = None::<u64>;
    let mut settle = None::<u64>;
    let mut vectors = Vec::<Vec<String>>::new();

    for (index, raw) in lines {
        let line = raw.trim();
        let number = index + 1;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        let keyword = fields.next().unwrap_or_default();
        match keyword {
            "module" => {
                let name = single(&mut fields, number, "module")?;
                if module.replace(name).is_some() {
                    return Err(format!("line {number}: 'module' declared twice"));
                }
            }
            "input" | "output" => {
                let name = next_field(&mut fields, number, keyword, "a port name")?;
                let width = next_field(&mut fields, number, keyword, "a width")?
                    .parse::<u32>()
                    .map_err(|err| format!("line {number}: width is not a number: {err}"))?;
                expect_end(&mut fields, number, keyword)?;
                if width == 0 {
                    return Err(format!("line {number}: port '{name}' has zero width"));
                }
                if ports.iter().any(|port| port.name == name) {
                    return Err(format!("line {number}: port '{name}' declared twice"));
                }
                ports.push(CasePort {
                    name,
                    width,
                    direction: if keyword == "input" {
                        CaseDirection::Input
                    } else {
                        CaseDirection::Output
                    },
                });
            }
            "clock" => {
                let port = next_field(&mut fields, number, "clock", "a port name")?;
                let half_period = next_field(&mut fields, number, "clock", "a half period")?
                    .parse::<u64>()
                    .map_err(|err| format!("line {number}: half period is not a number: {err}"))?;
                expect_end(&mut fields, number, "clock")?;
                if half_period == 0 {
                    return Err(format!("line {number}: clock half period must be positive"));
                }
                if clock.replace(Clock { port, half_period }).is_some() {
                    return Err(format!(
                        "line {number}: a second 'clock' line; the harness drives at most one"
                    ));
                }
            }
            "step" => {
                let value = single(&mut fields, number, "step")?
                    .parse::<u64>()
                    .map_err(|err| format!("line {number}: step is not a number: {err}"))?;
                if step.replace(value).is_some() {
                    return Err(format!("line {number}: 'step' declared twice"));
                }
            }
            "settle" => {
                let value = single(&mut fields, number, "settle")?
                    .parse::<u64>()
                    .map_err(|err| format!("line {number}: settle is not a number: {err}"))?;
                if settle.replace(value).is_some() {
                    return Err(format!("line {number}: 'settle' declared twice"));
                }
            }
            "vector" => {
                vectors.push(fields.map(str::to_string).collect());
            }
            other => {
                return Err(format!("line {number}: unknown keyword '{other}'"));
            }
        }
    }

    let module = module.ok_or_else(|| "no 'module' line".to_string())?;
    let step = step.ok_or_else(|| "no 'step' line".to_string())?;
    let settle = settle.ok_or_else(|| "no 'settle' line".to_string())?;
    let stimulus = Stimulus {
        module,
        ports,
        clock,
        step,
        settle,
        vectors,
    };
    validate_stimulus(&stimulus)?;
    Ok(stimulus)
}

fn validate_stimulus(stimulus: &Stimulus) -> Result<(), String> {
    if stimulus.step == 0 {
        return Err("step must be positive".to_string());
    }
    if stimulus.settle == 0 || stimulus.settle >= stimulus.step {
        return Err(format!(
            "settle ({}) must satisfy 0 < settle < step ({}); otherwise a sample \
             lands on a vector boundary and which vector it observes is a race",
            stimulus.settle, stimulus.step
        ));
    }
    if let Some(clock) = &stimulus.clock {
        let port = stimulus
            .port(&clock.port)
            .ok_or_else(|| format!("clock port '{}' is not a declared port", clock.port))?;
        if port.direction != CaseDirection::Input {
            return Err(format!("clock port '{}' must be an input", clock.port));
        }
        if port.width != 1 {
            return Err(format!("clock port '{}' must be one bit wide", clock.port));
        }
    }
    let driven = stimulus.driven_inputs();
    if driven.is_empty() {
        return Err("no driven inputs; the vectors would have nothing to apply".to_string());
    }
    if stimulus.observed_outputs().is_empty() {
        return Err("no outputs; the trace would be empty and every run would agree".to_string());
    }
    if stimulus.vectors.is_empty() {
        return Err("no vectors".to_string());
    }
    for (index, vector) in stimulus.vectors.iter().enumerate() {
        if vector.len() != driven.len() {
            return Err(format!(
                "vector {index} has {} value(s) for {} driven input(s)",
                vector.len(),
                driven.len()
            ));
        }
        for (value, port) in vector.iter().zip(&driven) {
            if value.len() != port.width as usize {
                return Err(format!(
                    "vector {index}: '{value}' is {} bit(s) for {}-bit port '{}'",
                    value.len(),
                    port.width,
                    port.name
                ));
            }
            if let Some(bad) = value
                .chars()
                .find(|ch| !matches!(ch, '0' | '1' | 'x' | 'z'))
            {
                return Err(format!(
                    "vector {index}: '{value}' contains '{bad}'; only 0, 1, x, z are values"
                ));
            }
        }
    }
    Ok(())
}

/// Cross-check the stimulus port list against the design's module header.
///
/// Names only, and deliberately so. Widths do not appear in a non-ANSI module
/// header, and chasing them would mean parsing the declaration body — a real
/// Verilog parser, in the harness, whose bugs would be indistinguishable from
/// corpus bugs. A width disagreement is caught downstream anyway and caught
/// hard: the generated testbench declares the port at the stimulus's width, so
/// a mismatch either truncates or pads, and `%b` then prints a different number
/// of characters than the other oracle does.
fn check_ports_against_design(
    case: &str,
    stimulus: &Stimulus,
    design: &str,
) -> Result<(), CorpusError> {
    let Some(header) = module_header_ports(design, &stimulus.module) else {
        return Err(CorpusError::ModuleNotFound {
            case: case.to_string(),
            module: stimulus.module.clone(),
        });
    };
    let declared = stimulus
        .ports
        .iter()
        .map(|port| port.name.clone())
        .collect::<BTreeSet<_>>();
    let missing = header.difference(&declared).cloned().collect::<Vec<_>>();
    let extra = declared.difference(&header).cloned().collect::<Vec<_>>();
    if missing.is_empty() && extra.is_empty() {
        return Ok(());
    }
    Err(CorpusError::PortMismatch {
        case: case.to_string(),
        module: stimulus.module.clone(),
        missing,
        extra,
    })
}

/// Extract the port-name set from `module <name> ( ... );`.
///
/// Comments are stripped first so a port name mentioned in prose cannot enter
/// the set, and the scan stops at the first `)` because everything after it is
/// the module body.
fn module_header_ports(design: &str, module: &str) -> Option<BTreeSet<String>> {
    let source = strip_comments(design);
    let mut rest = source.as_str();
    loop {
        let at = rest.find("module")?;
        let after = &rest[at + "module".len()..];
        // Require a token boundary so `module` inside a longer identifier does
        // not match.
        let boundary_ok = !after
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_');
        let before_ok = rest[..at]
            .chars()
            .next_back()
            .is_none_or(|ch| !(ch.is_ascii_alphanumeric() || ch == '_'));
        if boundary_ok && before_ok {
            let after = after.trim_start();
            let name_len = after
                .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '$'))
                .unwrap_or(after.len());
            let (name, tail) = after.split_at(name_len);
            if name == module {
                let open = tail.find('(')?;
                let close = tail.find(')')?;
                if close < open {
                    return None;
                }
                return Some(
                    tail[open + 1..close]
                        .split(',')
                        .map(str::trim)
                        .filter(|token| !token.is_empty())
                        .map(str::to_string)
                        .collect(),
                );
            }
        }
        rest = &rest[at + "module".len()..];
    }
}

fn strip_comments(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '/' {
            match chars.peek() {
                Some('/') => {
                    for next in chars.by_ref() {
                        if next == '\n' {
                            out.push('\n');
                            break;
                        }
                    }
                    continue;
                }
                Some('*') => {
                    chars.next();
                    let mut previous = '\0';
                    for next in chars.by_ref() {
                        if previous == '*' && next == '/' {
                            break;
                        }
                        previous = next;
                    }
                    out.push(' ');
                    continue;
                }
                _ => {}
            }
        }
        out.push(ch);
    }
    out
}

fn next_field<'a, I>(
    fields: &mut I,
    line: usize,
    keyword: &str,
    what: &str,
) -> Result<String, String>
where
    I: Iterator<Item = &'a str>,
{
    fields
        .next()
        .map(str::to_string)
        .ok_or_else(|| format!("line {line}: '{keyword}' needs {what}"))
}

fn expect_end<'a, I>(fields: &mut I, line: usize, keyword: &str) -> Result<(), String>
where
    I: Iterator<Item = &'a str>,
{
    match fields.next() {
        None => Ok(()),
        Some(extra) => Err(format!(
            "line {line}: '{keyword}' has trailing text '{extra}'"
        )),
    }
}

fn single<'a, I>(fields: &mut I, line: usize, keyword: &str) -> Result<String, String>
where
    I: Iterator<Item = &'a str>,
{
    let value = next_field(fields, line, keyword, "a value")?;
    expect_end(fields, line, keyword)?;
    Ok(value)
}

/// Everything that can be wrong with a corpus directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorpusError {
    UnreadableRoot {
        path: PathBuf,
        detail: String,
    },
    UnreadableManifest {
        path: PathBuf,
        detail: String,
    },
    MalformedManifest {
        line: usize,
        detail: String,
    },
    /// A `.v` on disk that the manifest does not list.
    UnlistedDesigns(Vec<String>),
    /// A manifest row naming a `.v` that is not on disk.
    OrphanedManifestEntries(Vec<String>),
    MissingStimulus {
        case: String,
        path: PathBuf,
        detail: String,
    },
    MalformedStimulus {
        case: String,
        detail: String,
    },
    UnreadableDesign {
        case: String,
        path: PathBuf,
        detail: String,
    },
    ModuleNotFound {
        case: String,
        module: String,
    },
    PortMismatch {
        case: String,
        module: String,
        missing: Vec<String>,
        extra: Vec<String>,
    },
    Empty(PathBuf),
}

impl fmt::Display for CorpusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnreadableRoot { path, detail } => {
                write!(
                    f,
                    "cannot read corpus directory '{}': {detail}",
                    path.display()
                )
            }
            Self::UnreadableManifest { path, detail } => {
                write!(f, "cannot read manifest '{}': {detail}", path.display())
            }
            Self::MalformedManifest { line, detail } => {
                write!(f, "{MANIFEST_FILE_NAME} line {line}: {detail}")
            }
            Self::UnlistedDesigns(designs) => write!(
                f,
                "these designs are on disk but not in {MANIFEST_FILE_NAME}, so nothing would \
                 run them: {}",
                designs.join(", ")
            ),
            Self::OrphanedManifestEntries(designs) => write!(
                f,
                "{MANIFEST_FILE_NAME} lists designs that do not exist: {}",
                designs.join(", ")
            ),
            Self::MissingStimulus { case, path, detail } => write!(
                f,
                "case '{case}' has no readable stimulus at '{}': {detail}",
                path.display()
            ),
            Self::MalformedStimulus { case, detail } => {
                write!(f, "case '{case}' stimulus: {detail}")
            }
            Self::UnreadableDesign { case, path, detail } => write!(
                f,
                "case '{case}' design '{}' is unreadable: {detail}",
                path.display()
            ),
            Self::ModuleNotFound { case, module } => write!(
                f,
                "case '{case}': the stimulus names module '{module}' but the design declares no \
                 such module header"
            ),
            Self::PortMismatch {
                case,
                module,
                missing,
                extra,
            } => write!(
                f,
                "case '{case}': stimulus and module '{module}' disagree on ports; \
                 declared by the design but not the stimulus: [{}]; declared by the stimulus but \
                 not the design: [{}]",
                missing.join(", "),
                extra.join(", ")
            ),
            Self::Empty(path) => write!(
                f,
                "corpus '{}' contains no cases; an empty suite passes vacuously",
                path.display()
            ),
        }
    }
}

impl std::error::Error for CorpusError {}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = "# RSPICE-VERILOG-STIMULUS 1\n\
        module demo\n\
        input a 1\n\
        input b 2\n\
        output y 1\n\
        step 10\n\
        settle 5\n\
        vector 0 01\n\
        vector 1 10\n";

    #[test]
    fn a_minimal_stimulus_round_trips_its_declarations() {
        let stimulus = parse_stimulus(MINIMAL).expect("minimal stimulus parses");

        assert_eq!(stimulus.module, "demo");
        assert_eq!(stimulus.step, 10);
        assert_eq!(stimulus.settle, 5);
        assert_eq!(stimulus.vectors.len(), 2);
        assert_eq!(
            stimulus
                .driven_inputs()
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["a", "b"]
        );
        assert_eq!(
            stimulus
                .observed_outputs()
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["y"]
        );
    }

    #[test]
    fn the_clock_port_is_excluded_from_the_vector_columns() {
        let text = "# RSPICE-VERILOG-STIMULUS 1\n\
            module demo\n\
            input clk 1\n\
            input d 1\n\
            output q 1\n\
            clock clk 5\n\
            step 10\n\
            settle 8\n\
            vector 1\n";

        let stimulus = parse_stimulus(text).expect("clocked stimulus parses");

        assert_eq!(stimulus.driven_inputs().len(), 1);
        assert_eq!(stimulus.driven_inputs()[0].name, "d");
        assert!(stimulus.is_clock("clk"));
    }

    #[test]
    fn a_settle_outside_the_step_is_rejected_as_a_race() {
        for settle in ["0", "10", "11"] {
            let text = MINIMAL.replace("settle 5", &format!("settle {settle}"));
            let error = parse_stimulus(&text).expect_err("settle must lie inside the step");
            assert!(error.contains("0 < settle < step"), "{error}");
        }
    }

    #[test]
    fn a_vector_of_the_wrong_arity_names_both_counts() {
        let text = MINIMAL.replace("vector 0 01", "vector 0");
        let error = parse_stimulus(&text).expect_err("arity is checked");
        assert!(
            error.contains("1 value(s) for 2 driven input(s)"),
            "{error}"
        );
    }

    #[test]
    fn a_vector_of_the_wrong_width_names_the_port() {
        let text = MINIMAL.replace("vector 0 01", "vector 0 011");
        let error = parse_stimulus(&text).expect_err("width is checked");
        assert!(error.contains("3 bit(s) for 2-bit port 'b'"), "{error}");
    }

    #[test]
    fn four_state_values_are_admitted_and_others_are_not() {
        let good = MINIMAL.replace("vector 0 01", "vector x zx");
        assert!(parse_stimulus(&good).is_ok());

        let bad = MINIMAL.replace("vector 0 01", "vector 0 0q");
        let error = parse_stimulus(&bad).expect_err("only 0/1/x/z are values");
        assert!(error.contains("only 0, 1, x, z"), "{error}");
    }

    #[test]
    fn a_stimulus_without_outputs_is_rejected_as_vacuous() {
        let text = "# RSPICE-VERILOG-STIMULUS 1\n\
            module demo\n\
            input a 1\n\
            step 10\n\
            settle 5\n\
            vector 0\n";

        let error = parse_stimulus(text).expect_err("a trace-free case proves nothing");
        assert!(error.contains("every run would agree"), "{error}");
    }

    #[test]
    fn a_clock_that_is_not_a_one_bit_input_is_rejected() {
        let wide = MINIMAL.replace("input b 2", "input b 2\nclock b 5");
        let error = parse_stimulus(&wide).expect_err("a clock must be one bit");
        assert!(error.contains("one bit wide"), "{error}");

        let output = MINIMAL.replace("output y 1", "output y 1\nclock y 5");
        let error = parse_stimulus(&output).expect_err("a clock must be an input");
        assert!(error.contains("must be an input"), "{error}");

        let absent = MINIMAL.replace("step 10", "clock nope 5\nstep 10");
        let error = parse_stimulus(&absent).expect_err("a clock must be declared");
        assert!(error.contains("not a declared port"), "{error}");
    }

    #[test]
    fn a_wrong_header_is_named_rather_than_ignored() {
        let error = parse_stimulus("# RSPICE-VERILOG-STIMULUS 2\nmodule demo\n")
            .expect_err("the version is part of the contract");
        assert!(error.contains(STIMULUS_HEADER), "{error}");
    }

    #[test]
    fn module_header_ports_survive_comments_and_similar_identifiers() {
        let design = "// module fake (not, real);\n\
            /* module also_fake (nope); */\n\
            module submodule (q, r);\n\
            endmodule\n\
            module demo (a, b, y);\n\
              input a, b;\n\
              output y;\n\
            endmodule\n";

        let ports = module_header_ports(design, "demo").expect("demo is declared");
        assert_eq!(
            ports,
            ["a", "b", "y"].into_iter().map(str::to_string).collect()
        );

        let ports = module_header_ports(design, "submodule").expect("submodule is declared");
        assert_eq!(ports, ["q", "r"].into_iter().map(str::to_string).collect());

        assert!(module_header_ports(design, "fake").is_none());
        assert!(module_header_ports(design, "also_fake").is_none());
    }

    #[test]
    fn a_port_the_design_does_not_declare_is_reported_in_the_right_direction() {
        let stimulus = parse_stimulus(MINIMAL).expect("parses");
        let design = "module demo (a, b);\ninput a, b;\nendmodule\n";

        let error = check_ports_against_design("demo", &stimulus, design)
            .expect_err("y is in the stimulus only");
        let CorpusError::PortMismatch { missing, extra, .. } = &error else {
            panic!("expected a port mismatch, got {error:?}");
        };
        assert!(missing.is_empty(), "{missing:?}");
        assert_eq!(extra, &vec!["y".to_string()]);
    }
}
