//! The Verilog-AMS real-net corpus: case discovery, its stimulus grammar, and
//! manifest cross-checking.
//!
//! A sibling of [`corpus`](super::corpus) rather than a widening of it, and the
//! reason is the oracle column. That corpus's whole discipline is "which
//! foreign simulators may this case be compared against", and the answer here
//! is none: Verilog-AMS LRM 2.4 section 3.7's `wreal` is a different standard
//! from the IEEE 1364-2005 that Icarus Verilog and Verilator implement, and a
//! manifest row saying "no oracles" in a format that requires at least one
//! would either need the requirement relaxed — weakening it for the sixteen
//! cases that do have oracles — or a sentinel value meaning "do not ask", which
//! is the same thing spelled worse.
//!
//! What replaces the oracle is a reference model beside each case, in
//! `tests/verilog_ams.rs`. That is a weaker guarantee and is stated as one.
//!
//! # The stimulus grammar, and how it differs
//!
//! `RSPICE-VERILOG-AMS-STIMULUS 1`, which is the sibling grammar with one
//! change: a port's shape is a *width* or the word `real`, where the sibling
//! admits only a width and refuses zero by name. A real net has no bit width at
//! all (section 3.7), and writing `0` for it would make the two files disagree
//! about what zero means.
//!
//! ```text
//! # RSPICE-VERILOG-AMS-STIMULUS 1
//! module wreal_forms
//! input  vin  real
//! input  code 4
//! output vout real
//! step   10
//! settle 5
//! vector 1.5 0101
//! ```
//!
//! There is no `clock` line. A free-running clock is a four-state stimulus
//! convenience, and none of these cases has one; leaving it out means the
//! parser cannot be wrong about a construct no case uses.
//!
//! # Timing
//!
//! The sibling's rule unchanged, because the harness's timing has nothing to do
//! with the value domain: inputs for vector `k` are applied at `k * step`, and
//! outputs are sampled at `k * step + settle`, with `0 < settle < step` so a
//! sample never lands on a vector boundary.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// The stimulus format version this parser accepts.
pub const STIMULUS_HEADER: &str = "# RSPICE-VERILOG-AMS-STIMULUS 1";

/// File name of the corpus manifest, relative to the corpus root.
pub const MANIFEST_FILE_NAME: &str = "ams-manifest.tsv";

/// Whether a port is driven by the harness or observed by it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AmsDirection {
    Input,
    Output,
}

/// What one port carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmsPortValue {
    /// IEEE 1364-2005 four-state bits, at this width.
    FourState { width: u32 },
    /// Verilog-AMS LRM 2.4 section 3.7's real net, which has no bit width.
    Real,
}

impl AmsPortValue {
    /// The width the engine's stimulus type wants, where zero is a real port.
    ///
    /// The engine spells "carries no bits" as width zero, and so does the front
    /// end for a `wreal` and for a process-local `real`. Converting here rather
    /// than storing a zero means this corpus never has to answer what a
    /// zero-width four-state port would be.
    pub const fn engine_width(self) -> u32 {
        match self {
            Self::FourState { width } => width,
            Self::Real => 0,
        }
    }

    pub const fn is_real(self) -> bool {
        matches!(self, Self::Real)
    }
}

/// One port of the design under test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmsPort {
    pub name: String,
    pub value: AmsPortValue,
    pub direction: AmsDirection,
}

/// A parsed `.stim` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmsStimulus {
    pub module: String,
    pub ports: Vec<AmsPort>,
    pub step: u64,
    pub settle: u64,
    /// One entry per vector; one column per input port, in declaration order.
    pub vectors: Vec<Vec<String>>,
}

impl AmsStimulus {
    pub fn inputs(&self) -> Vec<&AmsPort> {
        self.ports
            .iter()
            .filter(|port| port.direction == AmsDirection::Input)
            .collect()
    }

    pub fn outputs(&self) -> Vec<&AmsPort> {
        self.ports
            .iter()
            .filter(|port| port.direction == AmsDirection::Output)
            .collect()
    }
}

/// One corpus case: a design, its stimulus, and what it is for.
#[derive(Debug, Clone)]
pub struct AmsCase {
    pub name: String,
    pub source: PathBuf,
    pub stimulus_path: PathBuf,
    pub stimulus: AmsStimulus,
    pub note: String,
}

/// Every case in the real-net corpus.
#[derive(Debug, Clone)]
pub struct AmsCorpus {
    pub root: PathBuf,
    pub cases: Vec<AmsCase>,
}

impl AmsCorpus {
    /// Load and fully validate the corpus directory.
    ///
    /// The manifest is checked against the directory in *both* directions, for
    /// the reason the sibling corpus gives: one-directional checking is how a
    /// corpus quietly shrinks, because a renamed file drops out of the run and
    /// the manifest keeps listing a name nothing looks for.
    pub fn load(root: &Path) -> Result<Self, AmsCorpusError> {
        let manifest = parse_manifest(&root.join(MANIFEST_FILE_NAME))?;
        let discovered = discover_designs(root)?;

        let listed: BTreeSet<String> = manifest.iter().map(|(name, _)| name.clone()).collect();
        let present: BTreeSet<String> = discovered.iter().cloned().collect();
        let unlisted: Vec<String> = present.difference(&listed).cloned().collect();
        if !unlisted.is_empty() {
            return Err(AmsCorpusError::UnlistedDesigns(unlisted));
        }
        let orphaned: Vec<String> = listed.difference(&present).cloned().collect();
        if !orphaned.is_empty() {
            return Err(AmsCorpusError::OrphanedManifestEntries(orphaned));
        }

        let mut cases = Vec::with_capacity(discovered.len());
        for file_name in discovered {
            let name = file_name
                .strip_suffix(".v")
                .unwrap_or(&file_name)
                .to_string();
            let source = root.join(&file_name);
            let stimulus_path = root.join(format!("{name}.stim"));
            let text = fs::read_to_string(&stimulus_path).map_err(|err| {
                AmsCorpusError::MissingStimulus {
                    case: name.clone(),
                    path: stimulus_path.clone(),
                    detail: err.to_string(),
                }
            })?;
            let stimulus =
                parse_stimulus(&text).map_err(|detail| AmsCorpusError::MalformedStimulus {
                    case: name.clone(),
                    detail,
                })?;
            if stimulus.module != name {
                return Err(AmsCorpusError::MalformedStimulus {
                    case: name.clone(),
                    detail: format!(
                        "declares module '{}', which is not the case's own name",
                        stimulus.module
                    ),
                });
            }
            let note = manifest
                .iter()
                .find(|(listed, _)| *listed == file_name)
                .map(|(_, note)| note.clone())
                .unwrap_or_default();
            cases.push(AmsCase {
                name,
                source,
                stimulus_path,
                stimulus,
                note,
            });
        }

        if cases.is_empty() {
            return Err(AmsCorpusError::Empty(root.to_path_buf()));
        }
        Ok(Self {
            root: root.to_path_buf(),
            cases,
        })
    }

    pub fn case(&self, name: &str) -> Option<&AmsCase> {
        self.cases.iter().find(|case| case.name == name)
    }
}

/// Why the corpus could not be loaded.
#[derive(Debug, Clone)]
pub enum AmsCorpusError {
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
    UnlistedDesigns(Vec<String>),
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
    Empty(PathBuf),
}

impl fmt::Display for AmsCorpusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnreadableRoot { path, detail } => {
                write!(f, "cannot read corpus root '{}': {detail}", path.display())
            }
            Self::UnreadableManifest { path, detail } => {
                write!(f, "cannot read manifest '{}': {detail}", path.display())
            }
            Self::MalformedManifest { line, detail } => {
                write!(f, "manifest line {line}: {detail}")
            }
            Self::UnlistedDesigns(names) => write!(
                f,
                "these designs are present but not listed in the manifest: {}",
                names.join(", ")
            ),
            Self::OrphanedManifestEntries(names) => write!(
                f,
                "the manifest lists these designs, which are not present: {}",
                names.join(", ")
            ),
            Self::MissingStimulus { case, path, detail } => write!(
                f,
                "case '{case}' has no readable stimulus at '{}': {detail}",
                path.display()
            ),
            Self::MalformedStimulus { case, detail } => {
                write!(f, "case '{case}': {detail}")
            }
            Self::Empty(path) => write!(f, "'{}' contains no cases", path.display()),
        }
    }
}

impl std::error::Error for AmsCorpusError {}

fn discover_designs(root: &Path) -> Result<Vec<String>, AmsCorpusError> {
    let entries = fs::read_dir(root).map_err(|err| AmsCorpusError::UnreadableRoot {
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

/// `<design>\t<note>`, in file order.
fn parse_manifest(path: &Path) -> Result<Vec<(String, String)>, AmsCorpusError> {
    let text = fs::read_to_string(path).map_err(|err| AmsCorpusError::UnreadableManifest {
        path: path.to_path_buf(),
        detail: err.to_string(),
    })?;
    let mut entries: Vec<(String, String)> = Vec::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim_end();
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let Some(design) = fields.next().map(str::trim) else {
            return Err(AmsCorpusError::MalformedManifest {
                line: index + 1,
                detail: "expected <design>\\t<note>".to_string(),
            });
        };
        let note = fields.next().unwrap_or_default().trim().to_string();
        if note.is_empty() {
            return Err(AmsCorpusError::MalformedManifest {
                line: index + 1,
                detail: format!(
                    "'{design}' carries no note; say what the case covers or remove the row"
                ),
            });
        }
        if entries.iter().any(|(listed, _)| listed == design) {
            return Err(AmsCorpusError::MalformedManifest {
                line: index + 1,
                detail: format!("'{design}' is listed twice"),
            });
        }
        entries.push((design.to_string(), note));
    }
    Ok(entries)
}

/// Parse a `.stim` file.
///
/// Returns a human-readable reason rather than a typed error: every failure
/// here is a corpus authoring mistake read by a person fixing the file, and the
/// line-and-reason string is the whole of what they need.
pub fn parse_stimulus(text: &str) -> Result<AmsStimulus, String> {
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

    let mut module: Option<String> = None;
    let mut ports: Vec<AmsPort> = Vec::new();
    let mut step: Option<u64> = None;
    let mut settle: Option<u64> = None;
    let mut vectors: Vec<Vec<String>> = Vec::new();

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
                let name = one(&mut fields, number, "module")?;
                if module.replace(name).is_some() {
                    return Err(format!("line {number}: 'module' declared twice"));
                }
            }
            "input" | "output" => {
                let name = field(&mut fields, number, keyword, "a port name")?;
                let shape = field(&mut fields, number, keyword, "a width or 'real'")?;
                end(&mut fields, number, keyword)?;
                let value = if shape == "real" {
                    AmsPortValue::Real
                } else {
                    let width: u32 = shape.parse().map_err(|err| {
                        format!("line {number}: '{shape}' is neither a width nor 'real': {err}")
                    })?;
                    if width == 0 {
                        return Err(format!(
                            "line {number}: port '{name}' has zero width; a port with no bits \
                             is a real net, and says so by being declared 'real'"
                        ));
                    }
                    AmsPortValue::FourState { width }
                };
                if ports.iter().any(|port| port.name == name) {
                    return Err(format!("line {number}: port '{name}' declared twice"));
                }
                ports.push(AmsPort {
                    name,
                    value,
                    direction: if keyword == "input" {
                        AmsDirection::Input
                    } else {
                        AmsDirection::Output
                    },
                });
            }
            "step" => {
                let value = one(&mut fields, number, "step")?
                    .parse::<u64>()
                    .map_err(|err| format!("line {number}: step is not a number: {err}"))?;
                if step.replace(value).is_some() {
                    return Err(format!("line {number}: 'step' declared twice"));
                }
            }
            "settle" => {
                let value = one(&mut fields, number, "settle")?
                    .parse::<u64>()
                    .map_err(|err| format!("line {number}: settle is not a number: {err}"))?;
                if settle.replace(value).is_some() {
                    return Err(format!("line {number}: 'settle' declared twice"));
                }
            }
            "vector" => vectors.push(fields.map(str::to_string).collect()),
            other => return Err(format!("line {number}: unknown keyword '{other}'")),
        }
    }

    let stimulus = AmsStimulus {
        module: module.ok_or_else(|| "no 'module' line".to_string())?,
        ports,
        step: step.ok_or_else(|| "no 'step' line".to_string())?,
        settle: settle.ok_or_else(|| "no 'settle' line".to_string())?,
        vectors,
    };
    validate(&stimulus)?;
    Ok(stimulus)
}

fn validate(stimulus: &AmsStimulus) -> Result<(), String> {
    if stimulus.step == 0 {
        return Err("step must be positive".to_string());
    }
    if stimulus.settle == 0 || stimulus.settle >= stimulus.step {
        return Err(format!(
            "settle ({}) must satisfy 0 < settle < step ({}); otherwise a sample lands on a \
             vector boundary and which vector it observes is a race",
            stimulus.settle, stimulus.step
        ));
    }
    let inputs = stimulus.inputs();
    if inputs.is_empty() {
        return Err("no input ports; a case with nothing to drive proves nothing".to_string());
    }
    if stimulus.outputs().is_empty() {
        return Err("no output ports; a case with nothing to observe proves nothing".to_string());
    }
    if stimulus.vectors.is_empty() {
        return Err("no vectors".to_string());
    }
    for (index, vector) in stimulus.vectors.iter().enumerate() {
        if vector.len() != inputs.len() {
            return Err(format!(
                "vector {} has {} column(s) for {} input port(s)",
                index + 1,
                vector.len(),
                inputs.len()
            ));
        }
        // Every column is checked against its port's domain here rather than at
        // the engine boundary, so a corpus mistake reports as one instead of as
        // a simulation refusal several layers down.
        for (port, column) in inputs.iter().zip(vector) {
            match port.value {
                AmsPortValue::Real => {
                    column.parse::<f64>().map_err(|err| {
                        format!(
                            "vector {}: column for real port '{}' reads '{column}': {err}",
                            index + 1,
                            port.name
                        )
                    })?;
                }
                AmsPortValue::FourState { width } => {
                    if column.len() as u32 != width {
                        return Err(format!(
                            "vector {}: column for '{}' spells {} bit(s) for a {width}-bit port",
                            index + 1,
                            port.name,
                            column.len()
                        ));
                    }
                    if let Some(bad) = column.chars().find(|c| !matches!(c, '0' | '1' | 'x' | 'z'))
                    {
                        return Err(format!(
                            "vector {}: column for '{}' contains '{bad}', which is not a \
                             four-state digit",
                            index + 1,
                            port.name
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn field<'a>(
    fields: &mut impl Iterator<Item = &'a str>,
    number: usize,
    keyword: &str,
    what: &str,
) -> Result<String, String> {
    fields
        .next()
        .map(str::to_string)
        .ok_or_else(|| format!("line {number}: '{keyword}' needs {what}"))
}

fn one<'a>(
    fields: &mut impl Iterator<Item = &'a str>,
    number: usize,
    keyword: &str,
) -> Result<String, String> {
    let value = field(fields, number, keyword, "one value")?;
    end(fields, number, keyword)?;
    Ok(value)
}

fn end<'a>(
    fields: &mut impl Iterator<Item = &'a str>,
    number: usize,
    keyword: &str,
) -> Result<(), String> {
    match fields.next() {
        None => Ok(()),
        Some(extra) => Err(format!(
            "line {number}: '{keyword}' takes no further field, found '{extra}'"
        )),
    }
}
