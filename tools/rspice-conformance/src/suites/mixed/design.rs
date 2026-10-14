//! `design.toml` and `tolerance.toml`, and the integrity checks that bind a
//! design's files to each other without running a simulation.
//!
//! The checks are what keeps the corpus honest between nightly runs: every
//! declared analysis has its deck with exactly one card of its kind, every
//! quantity names a declared analysis, variant and oracle, every checked-in
//! reference is bound to the deck it was captured from, and `outcomes.tsv`
//! covers exactly the cells the design declares.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

pub const DESIGN_SCHEMA: &str = "rspice-mixed-design/1";
pub const TOLERANCE_SCHEMA: &str = "rspice-mixed-tolerance/1";

/// The analysis kinds a corpus deck may carry.
pub const ANALYSIS_KINDS: [&str; 15] = [
    "op", "dc", "ac", "noise", "tf", "stb", "pz", "sens", "tran", "trnoise", "mc-tran", "ckpt",
    "pss", "pac", "pnoise",
];

/// Plan §7's analysis list per design. A design may split one kind across
/// several decks (`dc-line`, `dc-load`), so the check is that every listed
/// kind is declared and nothing outside the kind set is.
pub fn required_analyses(id: &str) -> Option<&'static [&'static str]> {
    Some(match id {
        "C1" => &["tran", "op", "ac", "trnoise"],
        "C2" => &["tran", "op", "ac", "mc-tran", "pss", "pnoise"],
        "C3" => &["tran", "op"],
        "C4" => &["tran", "ac", "noise", "mc-tran"],
        "C5" => &["op", "dc", "ac", "stb", "noise", "tf", "sens", "pz", "tran"],
        "C6" => &["tran", "ckpt", "mc-tran"],
        "C7" => &["tran", "op"],
        "C8" => &["tran", "ac"],
        "C9" => &["tran", "dc"],
        "C10" => &["tran"],
        "C11" => &["tran", "pss", "pac", "pnoise"],
        "C12" => &["tran", "op"],
        // P0.6 probes carry their own analysis lists.
        "P06a" => &["trnoise"],
        "P06b" => &["mc-tran", "step-tran"],
        "P06c" => &["tran"],
        _ => return None,
    })
}

/// Plan §7's oracle list per design, which each analysis must carry in full
/// (extras allowed, never fewer). Probes compare against analytic assertions.
pub fn required_oracles(id: &str) -> Option<&'static [&'static str]> {
    Some(match id {
        "C1" => &["O1", "O2", "O6"],
        "C2" => &["O1", "O2", "O6"],
        "C3" => &["O2"],
        "C4" => &["O6", "O2", "O5"],
        "C5" => &["O5", "O1", "O4"],
        "C6" => &["O6", "O2"],
        "C7" => &["O6", "O3"],
        "C8" => &["O6", "O7"],
        "C9" => &["O6"],
        "C10" => &["O6", "O3"],
        "C11" => &["O6"],
        "C12" => &["O5", "O6"],
        "P06a" | "P06b" | "P06c" => &["O6"],
        _ => return None,
    })
}

/// The kind an analysis id names: the id itself, or the known kind it
/// prefixes with a `-tag` (`dc-line` is a `dc`; `mc-tran` is its own kind)
/// or an `@state` (`ac@trim` is an `ac` around a transient state).
/// `step-tran` is the probe-only `.step` of a transient.
pub fn analysis_kind(id: &str) -> Option<&'static str> {
    if id == "step-tran" {
        return Some("step-tran");
    }
    ANALYSIS_KINDS
        .iter()
        .filter(|kind| {
            id == **kind
                || id
                    .strip_prefix(**kind)
                    .is_some_and(|rest| rest.starts_with(['-', '@']))
        })
        .max_by_key(|kind| kind.len())
        .copied()
}

/// Whether an analysis id names a small-signal card around a transient
/// state (`ac@trim`): its deck carries the `.TRAN` card whose state the
/// card's `ACTIMES=` names, and it compares with its held twin only.
pub fn at_transient_state(id: &str) -> bool {
    id.contains('@')
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignFile {
    pub schema: String,
    pub id: String,
    pub title: String,
    pub exit_phases: Vec<String>,
    pub dialect: String,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(rename = "variant")]
    pub variants: Vec<Variant>,
    #[serde(rename = "analysis")]
    pub analyses: Vec<AnalysisEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,
    pub families: Vec<String>,
    pub today: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisEntry {
    pub id: String,
    pub variants: Vec<String>,
    pub oracles: Vec<String>,
    pub timeout_s: u64,
    #[serde(default)]
    pub extended: bool,
    #[serde(default)]
    pub checkpoint_at: Vec<f64>,
    /// The held-digital circuit this analysis's O5 twin runs when it is not
    /// the variant's own: `o5-c15` is `oracle/<variant>.o5-c15.cir`.
    #[serde(default)]
    pub o5: Option<String>,
    pub phase: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToleranceFile {
    pub schema: String,
    pub time_precision: f64,
    #[serde(rename = "q", default)]
    pub quantities: Vec<Quantity>,
}

/// A per-variant spelling of one value: the same for every variant, or a map.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PerVariant<T> {
    All(T),
    Map(BTreeMap<String, T>),
}

impl<T: Clone> PerVariant<T> {
    pub fn for_variant(&self, variant: &str) -> Option<T> {
        match self {
            Self::All(value) => Some(value.clone()),
            Self::Map(map) => map.get(variant).cloned(),
        }
    }
}

/// A regular time grid `start + k·step`, `k = 0..count`.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeGrid {
    pub start: f64,
    pub step: f64,
    pub count: usize,
}

impl TimeGrid {
    pub fn times(&self) -> Vec<f64> {
        (0..self.count)
            .map(|k| self.start + k as f64 * self.step)
            .collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lattice {
    pub t0: f64,
    pub period: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exclusion {
    pub window: [f64; 2],
    pub why: String,
}

/// How one quantity is measured on any producer's observation. Shared by a
/// top-level `[[q]]` and the inner measure of `stat`/`per_trial`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasureSpec {
    pub kind: String,
    #[serde(default)]
    pub signal: Option<PerVariant<String>>,
    #[serde(default)]
    pub signals: Option<Vec<String>>,
    #[serde(default)]
    pub times: Option<TimeGrid>,
    #[serde(default)]
    pub at: Option<Vec<f64>>,
    #[serde(default)]
    pub window: Option<[f64; 2]>,
    #[serde(default)]
    pub level: Option<f64>,
    #[serde(default)]
    pub edge: Option<String>,
    #[serde(default)]
    pub n: Option<usize>,
    #[serde(default)]
    pub target: Option<f64>,
    #[serde(default)]
    pub band: Option<f64>,
    #[serde(default)]
    pub points: Option<Vec<f64>>,
    #[serde(default)]
    pub nets: Option<PerVariant<Vec<String>>>,
    #[serde(default)]
    pub strobe: Option<String>,
    #[serde(default)]
    pub lattice: Option<Lattice>,
    #[serde(default)]
    pub smax: Option<f64>,
    #[serde(default)]
    pub params: Option<Vec<String>>,
    #[serde(default)]
    pub scale: Option<f64>,
    /// `fft`: `hann` (default) or `rect`.
    #[serde(default)]
    pub taper: Option<String>,
    /// `fft`: uniform samples of the window (default
    /// [`super::measure::FFT_SAMPLES`]).
    #[serde(default)]
    pub samples: Option<usize>,
    #[serde(default)]
    pub exclude: Vec<Exclusion>,
}

/// One `[[q]]`. Unknown keys are rejected by the flattened [`MeasureSpec`],
/// which receives every key this struct does not consume (serde cannot deny
/// unknown fields on a struct that flattens another).
#[derive(Debug, Clone, Deserialize)]
pub struct Quantity {
    pub id: String,
    pub analysis: String,
    pub variants: Vec<String>,
    pub oracle: String,
    #[serde(flatten)]
    pub measure: MeasureSpec,
    /// `stat` and `per_trial`: the per-sample or per-trial measure.
    #[serde(default)]
    pub of: Option<MeasureSpec>,
    /// `stat`: `mean | std | var`.
    #[serde(default)]
    pub stat: Option<String>,
    /// `stat`: what the statistic runs over: `samples` of `of`, or `trials`.
    #[serde(default)]
    pub over: Option<String>,
    #[serde(default)]
    pub interval: Option<[f64; 2]>,
    #[serde(default)]
    pub ratio: Option<[f64; 2]>,
    /// `per_trial`/`stat over trials`: the asserted trial count.
    #[serde(default)]
    pub trials: Option<usize>,
    /// O6: `cNN::name` in [`super::analytic`].
    #[serde(default)]
    pub expect: Option<String>,
    /// Captured oracles: the reference table, relative to the design.
    #[serde(rename = "ref", default)]
    pub reference: Option<PerVariant<String>>,
    #[serde(default)]
    pub abs: Option<f64>,
    #[serde(default)]
    pub rel: Option<f64>,
    #[serde(default)]
    pub abs_db: Option<f64>,
    #[serde(default)]
    pub abs_deg: Option<f64>,
    #[serde(default)]
    pub edge_abs: Option<f64>,
    pub why: String,
}

/// Keys a measure (a `[[q]]` or its `of`) may carry.
const MEASURE_KEYS: [&str; 21] = [
    "kind", "signal", "signals", "times", "at", "window", "level", "edge", "n", "target", "band",
    "points", "nets", "strobe", "lattice", "smax", "params", "scale", "taper", "samples",
    "exclude",
];

/// Keys only a `[[q]]` carries.
const QUANTITY_KEYS: [&str; 18] = [
    "id", "analysis", "variants", "oracle", "of", "stat", "over", "interval", "ratio", "trials",
    "expect", "ref", "abs", "rel", "abs_db", "abs_deg", "edge_abs", "why",
];

/// Reject unknown keys. serde cannot deny unknown fields across the flattened
/// measure, and a misspelled tolerance key would silently mean "zero".
pub fn check_quantity_keys(text: &str) -> Result<(), String> {
    let table: toml::Table = toml::from_str(text).map_err(|error| error.to_string())?;
    let Some(quantities) = table.get("q") else {
        return Ok(());
    };
    let quantities = quantities
        .as_array()
        .ok_or("q must be an array of tables")?;
    for (index, quantity) in quantities.iter().enumerate() {
        let quantity = quantity.as_table().ok_or("q must be an array of tables")?;
        for key in quantity.keys() {
            if !QUANTITY_KEYS.contains(&key.as_str()) && !MEASURE_KEYS.contains(&key.as_str()) {
                return Err(format!("q[{index}]: unknown key {key}"));
            }
        }
        if let Some(inner) = quantity.get("of").and_then(toml::Value::as_table) {
            for key in inner.keys() {
                if !MEASURE_KEYS.contains(&key.as_str()) {
                    return Err(format!("q[{index}].of: unknown key {key}"));
                }
            }
        }
    }
    Ok(())
}

/// The measure kinds `tolerance.toml` may name.
pub const MEASURE_KINDS: [&str; 22] = [
    "op",
    "held_at",
    "interp_at",
    "mean",
    "min",
    "max",
    "pp",
    "fft",
    "noise_int",
    "cross",
    "period",
    "settle",
    "dc_curve",
    "ac",
    "noise",
    "tf",
    "stb",
    "pz",
    "sens",
    "events",
    "codes",
    "identical",
];

/// The oracle identities a quantity may compare against.
pub const ORACLES: [&str; 9] = ["O1", "O2", "O3", "O4", "O5", "O5-O1", "O5-O4", "O6", "O7"];

/// One design directory, loaded.
#[derive(Debug, Clone)]
pub struct Design {
    pub dir: PathBuf,
    pub slug: String,
    pub file: DesignFile,
    pub tolerance: ToleranceFile,
}

impl Design {
    pub fn load(dir: &Path) -> Result<Self, String> {
        let slug = dir
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("{}: unnamed design directory", dir.display()))?
            .to_owned();
        let read = |name: &str| {
            fs::read_to_string(dir.join(name)).map_err(|error| format!("{slug}/{name}: {error}"))
        };
        let file: DesignFile = toml::from_str(&read("design.toml")?)
            .map_err(|error| format!("{slug}/design.toml: {error}"))?;
        let tolerance_text = read("tolerance.toml")?;
        check_quantity_keys(&tolerance_text)
            .map_err(|error| format!("{slug}/tolerance.toml: {error}"))?;
        let tolerance: ToleranceFile = toml::from_str(&tolerance_text)
            .map_err(|error| format!("{slug}/tolerance.toml: {error}"))?;
        Ok(Self {
            dir: dir.to_path_buf(),
            slug,
            file,
            tolerance,
        })
    }

    pub fn id(&self) -> &str {
        &self.file.id
    }

    pub fn deck_path(&self, variant: &str, analysis: &str) -> PathBuf {
        self.dir
            .join("deck")
            .join(format!("{variant}.{analysis}.cir"))
    }

    pub fn o5_path(&self, variant: &str) -> PathBuf {
        self.dir.join("oracle").join(format!("{variant}.o5.cir"))
    }

    /// The held-digital circuit `analysis`'s O5 twin runs: the variant's
    /// own, or the one the analysis names.
    pub fn o5_path_for(&self, variant: &str, analysis: &str) -> PathBuf {
        match self
            .analysis(analysis)
            .and_then(|entry| entry.o5.as_deref())
        {
            Some(circuit) => self
                .dir
                .join("oracle")
                .join(format!("{variant}.{circuit}.cir")),
            None => self.o5_path(variant),
        }
    }

    /// The directory of a variant's O3 testbench template, `tb.v`.
    pub fn o3_dir(&self, variant: &str) -> PathBuf {
        self.dir.join("oracle").join(format!("{variant}.o3"))
    }

    pub fn analysis(&self, id: &str) -> Option<&AnalysisEntry> {
        self.file.analyses.iter().find(|analysis| analysis.id == id)
    }

    /// Every declared (variant, analysis) cell, in declaration order.
    pub fn cells(&self) -> Vec<(String, String)> {
        let mut cells = Vec::new();
        for analysis in &self.file.analyses {
            for variant in &analysis.variants {
                cells.push((variant.clone(), analysis.id.clone()));
            }
        }
        cells
    }

    pub fn quantities_for<'a>(
        &'a self,
        variant: &'a str,
        analysis: &'a str,
    ) -> impl Iterator<Item = &'a Quantity> + 'a {
        self.tolerance.quantities.iter().filter(move |quantity| {
            quantity.analysis == analysis && quantity.variants.iter().any(|name| name == variant)
        })
    }
}

/// Every design directory under the corpus root, probes included, sorted.
pub fn discover(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dirs = Vec::new();
    if !root.is_dir() {
        return Ok(dirs);
    }
    let mut push = |dir: &Path| -> Result<(), String> {
        for entry in fs::read_dir(dir).map_err(|error| format!("{}: {error}", dir.display()))? {
            let path = entry.map_err(|error| error.to_string())?.path();
            if path.join("design.toml").is_file() {
                dirs.push(path);
            }
        }
        Ok(())
    };
    push(root)?;
    let probes = root.join("probes");
    if probes.is_dir() {
        push(&probes)?;
    }
    dirs.sort();
    Ok(dirs)
}

/// Card keywords that make a deck line an analysis request.
pub const ANALYSIS_CARDS: [&str; 18] = [
    ".op", ".dc", ".ac", ".noise", ".tf", ".stb", ".pz", ".sens", ".tran", ".mc", ".step", ".pss",
    ".pac", ".pnoise", ".pxf", ".four", ".fft", ".disto",
];

/// The analysis cards of a deck (first token lower-cased), with their logical
/// line text. Continuation lines are folded; comments and `.control` blocks
/// are skipped.
pub fn deck_cards(source: &str) -> Vec<(String, String)> {
    let mut cards: Vec<(String, String)> = Vec::new();
    let mut in_control = false;
    let mut last_was_card = false;
    for (index, line) in source.lines().enumerate() {
        if index == 0 {
            continue; // title
        }
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();
        if lower.starts_with(".control") {
            in_control = true;
            continue;
        }
        if lower.starts_with(".endc") {
            in_control = false;
            continue;
        }
        if in_control || trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix('+') {
            if last_was_card && let Some(card) = cards.last_mut() {
                card.1.push(' ');
                card.1.push_str(rest.trim());
            }
            continue;
        }
        let word = lower
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_owned();
        last_was_card = ANALYSIS_CARDS.contains(&word.as_str());
        if last_was_card {
            cards.push((word, trimmed.to_owned()));
        }
    }
    cards
}

/// The deck cards an analysis kind requires, in order: `mc-tran` is `.mc` +
/// `.tran`, `trnoise` and `ckpt` are a `.tran`, `step-tran` is `.step` +
/// `.tran`.
pub fn expected_cards(kind: &str) -> Vec<&'static str> {
    match kind {
        "op" => vec![".op"],
        "dc" => vec![".dc"],
        "ac" => vec![".ac"],
        "noise" => vec![".noise"],
        "tf" => vec![".tf"],
        "stb" => vec![".stb"],
        "pz" => vec![".pz"],
        "sens" => vec![".sens"],
        "tran" | "trnoise" | "ckpt" => vec![".tran"],
        "mc-tran" => vec![".mc", ".tran"],
        "step-tran" => vec![".step", ".tran"],
        "pss" => vec![".pss"],
        "pac" => vec![".pss", ".pac"],
        "pnoise" => vec![".pss", ".pnoise"],
        _ => Vec::new(),
    }
}

/// One integrity finding: which file and what is wrong with it.
pub type Finding = String;

/// Check one design. Returns every finding, not just the first.
pub fn integrity(design: &Design) -> Vec<Finding> {
    let mut findings = Vec::new();
    let slug = &design.slug;
    let file = &design.file;
    if file.schema != DESIGN_SCHEMA {
        findings.push(format!("{slug}/design.toml: schema {:?}", file.schema));
    }
    if design.tolerance.schema != TOLERANCE_SCHEMA {
        findings.push(format!(
            "{slug}/tolerance.toml: schema {:?}",
            design.tolerance.schema
        ));
    }
    if !(design.tolerance.time_precision.is_finite() && design.tolerance.time_precision > 0.0) {
        findings.push(format!(
            "{slug}/tolerance.toml: time_precision must be positive"
        ));
    }
    let variants: BTreeSet<&str> = file.variants.iter().map(|v| v.name.as_str()).collect();
    if variants.len() != file.variants.len() {
        findings.push(format!("{slug}/design.toml: duplicate variant names"));
    }
    let mut analysis_ids = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for analysis in &file.analyses {
        if !analysis_ids.insert(analysis.id.as_str()) {
            findings.push(format!("{slug}: analysis {} declared twice", analysis.id));
        }
        let Some(kind) = analysis_kind(&analysis.id) else {
            findings.push(format!(
                "{slug}: analysis id {} names no known kind",
                analysis.id
            ));
            continue;
        };
        kinds.insert(kind);
        if analysis.timeout_s == 0 {
            findings.push(format!("{slug}: analysis {} has no budget", analysis.id));
        }
        if (kind == "ckpt") == analysis.checkpoint_at.is_empty() {
            findings.push(format!(
                "{slug}: analysis {}: checkpoint_at is for ckpt analyses only and required there",
                analysis.id
            ));
        }
        // A card around a transient state compares with its held twin: no
        // captured oracle linearizes around a transient's state.
        if let Some(required) = required_oracles(&file.id)
            && !at_transient_state(&analysis.id)
        {
            for oracle in required {
                if !analysis.oracles.iter().any(|listed| listed == oracle) {
                    findings.push(format!(
                        "{slug}: analysis {} lists fewer oracles than §7 ({oracle} missing)",
                        analysis.id
                    ));
                }
            }
        }
        for oracle in &analysis.oracles {
            if !ORACLES.contains(&oracle.as_str()) {
                findings.push(format!(
                    "{slug}: analysis {}: unknown oracle {oracle}",
                    analysis.id
                ));
            }
        }
        for variant in &analysis.variants {
            if !variants.contains(variant.as_str()) {
                findings.push(format!(
                    "{slug}: analysis {} names undeclared variant {variant}",
                    analysis.id
                ));
                continue;
            }
            check_deck(design, variant, analysis, kind, &mut findings);
        }
    }
    match required_analyses(&file.id) {
        Some(required) => {
            for kind in required {
                if !kinds.contains(kind) {
                    findings.push(format!("{slug}: §7 analysis {kind} is not declared"));
                }
            }
        }
        None => findings.push(format!("{slug}: design id {} is not in plan §7", file.id)),
    }
    for quantity in &design.tolerance.quantities {
        check_quantity(design, quantity, &mut findings);
    }
    findings
}

fn check_deck(
    design: &Design,
    variant: &str,
    analysis: &AnalysisEntry,
    kind: &str,
    findings: &mut Vec<Finding>,
) {
    let slug = &design.slug;
    let path = design.deck_path(variant, &analysis.id);
    let Ok(source) = fs::read_to_string(&path) else {
        findings.push(format!(
            "{slug}: missing deck deck/{variant}.{}.cir",
            analysis.id
        ));
        return;
    };
    let cards: Vec<String> = deck_cards(&source)
        .into_iter()
        .map(|(word, _)| word)
        .collect();
    let mut expected = expected_cards(kind);
    if at_transient_state(&analysis.id) {
        expected.insert(0, ".tran");
    }
    let mut sorted_cards = cards.clone();
    sorted_cards.sort();
    let mut sorted_expected: Vec<String> = expected.iter().map(|card| (*card).to_owned()).collect();
    sorted_expected.sort();
    if sorted_cards != sorted_expected {
        findings.push(format!(
            "{slug}: deck/{variant}.{}.cir carries cards {cards:?}; a {kind} deck carries exactly {expected:?}",
            analysis.id
        ));
    }
}

fn check_quantity(design: &Design, quantity: &Quantity, findings: &mut Vec<Finding>) {
    let slug = &design.slug;
    let id = &quantity.id;
    let Some(analysis) = design.analysis(&quantity.analysis) else {
        findings.push(format!(
            "{slug}: quantity {id} names undeclared analysis {}",
            quantity.analysis
        ));
        return;
    };
    for variant in &quantity.variants {
        if !analysis.variants.contains(variant) {
            findings.push(format!(
                "{slug}: quantity {id} names variant {variant}, which analysis {} does not run",
                analysis.id
            ));
        }
    }
    if !analysis.oracles.contains(&quantity.oracle) {
        findings.push(format!(
            "{slug}: quantity {id} compares against {}, which analysis {} does not list",
            quantity.oracle, analysis.id
        ));
    }
    let kind = quantity.measure.kind.as_str();
    let known = MEASURE_KINDS.contains(&kind) || matches!(kind, "stat" | "per_trial" | "assert");
    if !known {
        findings.push(format!("{slug}: quantity {id}: unknown kind {kind}"));
    }
    if matches!(kind, "stat" | "per_trial") && quantity.of.is_none() {
        findings.push(format!(
            "{slug}: quantity {id}: {kind} needs an `of` measure"
        ));
    }
    if kind == "stat" && quantity.interval.is_none() && quantity.ratio.is_none() {
        findings.push(format!(
            "{slug}: quantity {id}: stat needs an interval or a ratio"
        ));
    }
    if quantity.why.trim().is_empty() {
        findings.push(format!("{slug}: quantity {id} has no why"));
    }
    match quantity.oracle.as_str() {
        "O6" => match quantity.expect.as_deref() {
            Some(expect) if kind == "assert" && super::probes::ASSERTIONS.contains(&expect) => {}
            Some(expect) if kind != "assert" && super::analytic::lookup(expect).is_some() => {}
            Some(expect) => findings.push(format!(
                "{slug}: quantity {id}: analytic expectation {expect} is not registered"
            )),
            None if kind == "stat" && quantity.interval.is_some() => {}
            None => findings.push(format!("{slug}: quantity {id}: O6 needs `expect`")),
        },
        "O5" | "O7" => {}
        "O3" => check_o3_templates(design, quantity, findings),
        _ => {
            if quantity.reference.is_none() {
                findings.push(format!(
                    "{slug}: quantity {id}: captured oracle {} needs `ref`",
                    quantity.oracle
                ));
            }
        }
    }
    if quantity.oracle == "O5" {
        for variant in &quantity.variants {
            let path = design.o5_path_for(variant, &analysis.id);
            if !path.is_file() {
                findings.push(format!(
                    "{slug}: quantity {id}: missing oracle/{}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
        }
    }
    // An O3 `ref` was reported above; there is no deck to bind it to.
    if let Some(reference) = &quantity.reference
        && quantity.oracle != "O3"
    {
        for variant in &quantity.variants {
            let Some(path) = reference.for_variant(variant) else {
                findings.push(format!(
                    "{slug}: quantity {id}: no ref for variant {variant}"
                ));
                continue;
            };
            check_reference_binding(design, &path, findings);
        }
    }
}

/// O3 runs Icarus live on every run (design of record §2.9), replaying the
/// boundary events RSpice produced on the inputs the template names (C7's
/// testbenches name none: their modules drive the whole protocol), so like O5
/// it has nothing captured to bind: it takes no `ref`. Each variant instead
/// needs its template `oracle/<v>.o3/tb.v`, with the stimulus marker, a
/// timescale the runner reads, and sources that exist.
fn check_o3_templates(design: &Design, quantity: &Quantity, findings: &mut Vec<Finding>) {
    use super::oracle::icarus::{STIMULUS_MARKER, template_directive};
    let slug = &design.slug;
    let id = &quantity.id;
    if quantity.reference.is_some() {
        findings.push(format!(
            "{slug}: quantity {id}: O3 runs live on every run and takes no `ref`"
        ));
    }
    for variant in &quantity.variants {
        let dir = design.o3_dir(variant);
        let Ok(template) = fs::read_to_string(dir.join("tb.v")) else {
            findings.push(format!(
                "{slug}: quantity {id}: missing oracle/{variant}.o3/tb.v"
            ));
            continue;
        };
        let at = format!("{slug}: oracle/{variant}.o3/tb.v");
        if !template.contains(STIMULUS_MARKER) {
            findings.push(format!("{at} lacks {STIMULUS_MARKER}"));
        }
        if let Some(scale) = template_directive(&template, "rspice-o3-timescale")
            && !matches!(scale.as_str(), "1ps" | "1ns" | "1fs")
        {
            findings.push(format!("{at}: timescale {scale} is not 1ps, 1ns or 1fs"));
        }
        for source in template_directive(&template, "rspice-o3-sources")
            .unwrap_or_default()
            .split_whitespace()
        {
            if !dir.join(source).is_file() {
                findings.push(format!("{at} names missing source {source}"));
            }
        }
    }
}

/// A checked-in reference must be bound to the oracle deck it came from; an
/// absent one is legal (O2 before P0.3) and makes the quantity unverified.
fn check_reference_binding(design: &Design, reference: &str, findings: &mut Vec<Finding>) {
    let slug = &design.slug;
    let path = design.dir.join(reference);
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        findings.push(format!("{slug}: malformed ref path {reference}"));
        return;
    };
    let deck = match super::oracle::deck_for_reference(&design.dir, name) {
        Ok(deck) => deck,
        Err(error) => {
            findings.push(format!("{slug}: {reference}: {error}"));
            return;
        }
    };
    if !deck.is_file() {
        findings.push(format!(
            "{slug}: {reference}: its oracle deck {} is missing",
            deck.strip_prefix(&design.dir).unwrap_or(&deck).display()
        ));
        return;
    }
    if !path.is_file() {
        return;
    }
    match super::oracle::verify_binding(&path, &deck) {
        Ok(()) => {}
        Err(error) => findings.push(format!("{slug}: {reference}: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analysis_ids_resolve_to_their_kind() {
        assert_eq!(analysis_kind("tran"), Some("tran"));
        assert_eq!(analysis_kind("mc-tran"), Some("mc-tran"));
        assert_eq!(analysis_kind("dc-line"), Some("dc"));
        assert_eq!(analysis_kind("dc-load"), Some("dc"));
        assert_eq!(analysis_kind("trnoise"), Some("trnoise"));
        assert_eq!(analysis_kind("step-tran"), Some("step-tran"));
        assert_eq!(analysis_kind("ac@trim"), Some("ac"));
        assert!(at_transient_state("ac@lock") && !at_transient_state("dc-line"));
        assert_eq!(analysis_kind("tranx"), None);
        assert_eq!(analysis_kind("bogus"), None);
    }

    #[test]
    fn deck_cards_fold_continuations_and_skip_control_blocks() {
        let deck = "title .tran is not a card\nR1 a 0 1k\n.tran 1n\n+ 10n\n* .ac dec 1 1 2\n.control\nrun\n.endc\n.MC 4 SEED 9\n.end\n";
        let cards = deck_cards(deck);
        assert_eq!(
            cards,
            vec![
                (".tran".to_owned(), ".tran 1n 10n".to_owned()),
                (".mc".to_owned(), ".MC 4 SEED 9".to_owned())
            ]
        );
        assert_eq!(expected_cards("mc-tran"), vec![".mc", ".tran"]);
    }

    #[test]
    fn per_variant_values_resolve_by_name() {
        let all: PerVariant<String> = PerVariant::All("v(a)".into());
        assert_eq!(all.for_variant("xs").as_deref(), Some("v(a)"));
        let map: PerVariant<String> = toml::from_str::<BTreeMap<String, PerVariant<String>>>(
            "s = { xs = \"v(x.a)\", vams = \"v(a)\" }",
        )
        .unwrap()
        .remove("s")
        .unwrap();
        assert_eq!(map.for_variant("vams").as_deref(), Some("v(a)"));
        assert_eq!(map.for_variant("other"), None);
    }

    #[test]
    fn o3_takes_a_template_not_a_ref() {
        let dir = tempfile::tempdir().unwrap();
        let file: DesignFile = toml::from_str(
            "schema = \"rspice-mixed-design/1\"\nid = \"C10\"\ntitle = \"t\"\nexit_phases = [\"P2\"]\ndialect = \"d\"\n\
             [[variant]]\nname = \"a\"\nfamilies = [\"spice\"]\ntoday = \"runs\"\n\
             [[variant]]\nname = \"b\"\nfamilies = [\"spice\"]\ntoday = \"runs\"\n\
             [[analysis]]\nid = \"tran\"\nvariants = [\"a\", \"b\"]\noracles = [\"O3\"]\ntimeout_s = 10\nphase = \"P2\"\n",
        )
        .unwrap();
        let quantity = "[[q]]\nid = \"vec\"\nanalysis = \"tran\"\nvariants = [\"a\", \"b\"]\nkind = \"codes\"\nnets = [\"o\"]\nstrobe = \"s\"\noracle = \"O3\"\nwhy = \"w\"\n";
        let tolerance = |extra: &str| -> ToleranceFile {
            toml::from_str(&format!(
                "schema = \"rspice-mixed-tolerance/1\"\ntime_precision = 1e-12\n{quantity}{extra}"
            ))
            .unwrap()
        };
        let template = |dir: &Path, text: &str| {
            fs::create_dir_all(dir).unwrap();
            fs::write(dir.join("tb.v"), text).unwrap();
        };
        let design = Design {
            dir: dir.path().to_path_buf(),
            slug: "c10".into(),
            file,
            tolerance: tolerance(""),
        };
        fs::create_dir_all(dir.path().join("net")).unwrap();
        fs::write(dir.path().join("net/x.v"), "module x; endmodule\n").unwrap();
        template(
            &design.o3_dir("a"),
            "// rspice-o3-sources: ../../net/x.v\n// rspice-o3-inputs: i s\n// rspice-o3-timescale: 1ps\nmodule tb;\n// RSPICE-O3-STIMULUS\nendmodule\n",
        );
        let findings = |design: &Design| {
            let mut findings = Vec::new();
            check_quantity(design, &design.tolerance.quantities[0], &mut findings);
            findings
        };
        assert_eq!(
            findings(&design),
            ["c10: quantity vec: missing oracle/b.o3/tb.v"]
        );
        template(
            &design.o3_dir("b"),
            "// rspice-o3-sources: ../../net/y.v\n// rspice-o3-timescale: 1us\nmodule tb;\nendmodule\n",
        );
        let with_ref = Design {
            tolerance: tolerance("ref = \"ref/a.tran.O3.out\"\n"),
            ..design.clone()
        };
        assert_eq!(
            findings(&with_ref),
            [
                "c10: quantity vec: O3 runs live on every run and takes no `ref`",
                "c10: oracle/b.o3/tb.v lacks // RSPICE-O3-STIMULUS",
                "c10: oracle/b.o3/tb.v: timescale 1us is not 1ps, 1ns or 1fs",
                "c10: oracle/b.o3/tb.v names missing source ../../net/y.v",
            ]
        );
    }

    #[test]
    fn quantity_schema_rejects_unknown_fields() {
        let good = "schema = \"rspice-mixed-tolerance/1\"\ntime_precision = 1e-9\n[[q]]\nid = \"a\"\nanalysis = \"op\"\nvariants = [\"x\"]\nkind = \"op\"\nsignal = \"v(a)\"\noracle = \"O6\"\nexpect = \"c01::opdiff\"\nrel = 1e-4\nwhy = \"w\"\n";
        let parsed: ToleranceFile = toml::from_str(good).unwrap();
        assert_eq!(parsed.quantities[0].measure.kind, "op");
        check_quantity_keys(good).unwrap();
        let bad = good.replace("rel = 1e-4", "rel = 1e-4\ntypo = 1");
        assert!(check_quantity_keys(&bad).is_err());
        let bad_inner = good.replace(
            "rel = 1e-4",
            "rel = 1e-4\nof = { kind = \"op\", sigal = \"v(a)\" }",
        );
        assert!(check_quantity_keys(&bad_inner).is_err());
    }
}
