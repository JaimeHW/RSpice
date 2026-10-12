//! Oracles: tool discovery, checked-in reference tables, and capture.
//!
//! The normal run never executes another simulator. References are captured
//! once (`rspice-mixed-corpus capture`) and checked in under `ref/`, each
//! bound to the BLAKE3 of its oracle deck's expanded source; a reference whose
//! deck changed fails the integrity check rather than comparing against the
//! wrong circuit. `capture` without `--update` re-runs the oracle and requires
//! a byte-identical table.
//!
//! | Reference | Oracle deck |
//! |---|---|
//! | `ref/<v>.<a>.O1.out` | `oracle/<v>.<a>.ngspice.cir` |
//! | `ref/<v>.<a>.O4.out` | `oracle/<v>.<a>.xyce.cir` |
//! | `ref/<v>.<a>.O5-O1.out` | `oracle/<v>.<a>.o5.ngspice.cir` |
//! | `ref/<v>.<a>.O5-O4.out` | `oracle/<v>.<a>.o5.xyce.cir` |
//! | `ref/<v>.<a>.O2.out` | `oracle/<v>.o2/deck.cir` (+ `build.txt`) |
//!
//! Event traces sit beside the table as `ref/<v>.<a>.<oracle>.events.tsv`.
//!
//! A table's transient grid is the oracle's own and is decimated to at most
//! [`MAX_TRANSIENT_ROWS`] rows by uniform index (first and last kept): only
//! measures are compared, never samples, and a measure that needs finer
//! resolution than that states it in its tolerance.

pub mod cosim;
pub mod icarus;
pub mod ngspice;
pub mod xyce;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::Digest as _;

use super::case::{Observation, canonical_signal};

pub const MAX_TRANSIENT_ROWS: usize = 20_001;

/// Which program produced a reference table, by header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Producer {
    Ngspice,
    Xyce,
    Cosim,
}

impl Producer {
    pub fn header(self) -> &'static str {
        match self {
            Self::Ngspice => "# RSPICE-NGSPICE-ORACLE 2",
            Self::Xyce => "# RSPICE-XYCE-ORACLE 2",
            Self::Cosim => "# RSPICE-COSIM-ORACLE 2",
        }
    }

    fn from_header(line: &str) -> Option<Self> {
        [Self::Ngspice, Self::Xyce, Self::Cosim]
            .into_iter()
            .find(|producer| producer.header() == line.trim())
    }
}

/// The oracle deck a reference file name was captured from.
pub fn deck_for_reference(design_dir: &Path, reference_name: &str) -> Result<PathBuf, String> {
    let stem = reference_name
        .strip_suffix(".events.tsv")
        .or_else(|| reference_name.strip_suffix(".out"))
        .ok_or_else(|| format!("reference {reference_name} is neither .out nor .events.tsv"))?;
    let (base, oracle) = stem
        .rsplit_once('.')
        .ok_or_else(|| format!("reference {reference_name} names no oracle"))?;
    let oracle_dir = design_dir.join("oracle");
    Ok(match oracle {
        "O1" => oracle_dir.join(format!("{base}.ngspice.cir")),
        "O4" => oracle_dir.join(format!("{base}.xyce.cir")),
        "O5-O1" => oracle_dir.join(format!("{base}.o5.ngspice.cir")),
        "O5-O4" => oracle_dir.join(format!("{base}.o5.xyce.cir")),
        "O2" => {
            let variant = base.split('.').next().unwrap_or(base);
            oracle_dir.join(format!("{variant}.o2")).join("deck.cir")
        }
        "O3" => {
            return Err(format!(
                "reference {reference_name}: O3 runs live on every run and has no captured reference"
            ));
        }
        other => {
            return Err(format!(
                "reference {reference_name}: oracle {other} is not captured"
            ));
        }
    })
}

/// Expand an oracle deck's includes and hash it (CRLF-normalized).
pub fn deck_fingerprint(deck: &Path) -> Result<String, String> {
    let source =
        fs::read_to_string(deck).map_err(|error| format!("{}: {error}", deck.display()))?;
    let directory = deck.parent().unwrap_or(Path::new("."));
    let mut processor = rspice_core::netlist::IncludeProcessor::new(directory);
    let expanded = processor
        .expand_content(&source, deck)
        .map_err(|error| format!("{}: {error}", deck.display()))?;
    // Files a deck names outside .include (HDL sources for O2) are bound by
    // their own build description, which is part of the expanded text.
    Ok(blake3::hash(expanded.replace("\r\n", "\n").as_bytes())
        .to_hex()
        .to_string())
}

/// The header line binding a reference to its deck.
pub fn binding_line(fingerprint: &str) -> String {
    format!("# input-blake3: {fingerprint}")
}

/// The reference must carry the current fingerprint of `deck`.
pub fn verify_binding(reference: &Path, deck: &Path) -> Result<(), String> {
    let text = fs::read_to_string(reference).map_err(|error| error.to_string())?;
    let expected = binding_line(&deck_fingerprint(deck)?);
    if text.lines().any(|line| line.trim() == expected) {
        Ok(())
    } else {
        Err("reference is not bound to its oracle deck's current source; recapture".into())
    }
}

/// SHA-256 of an executable, for the reference header.
pub fn executable_sha256(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(format!("{:x}", sha2::Sha256::digest(&bytes)))
}

/// One analysis section of a reference.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Table {
    /// `time`, `frequency`, `sweep`, `op`, `scalar` or `root`.
    pub axis_name: String,
    pub axis: Vec<f64>,
    pub real: BTreeMap<String, Vec<f64>>,
    pub complex: BTreeMap<String, Vec<[f64; 2]>>,
}

/// A parsed reference file.
#[derive(Debug, Clone, PartialEq)]
pub struct Reference {
    pub producer: Producer,
    pub header: Vec<String>,
    pub sections: BTreeMap<String, Table>,
}

/// Decimate a transient table to at most `max` rows by uniform index.
pub fn decimate(table: &mut Table, max: usize) {
    let length = table.axis.len();
    if length <= max || max < 2 {
        return;
    }
    let indices: Vec<usize> = (0..max)
        .map(|k| ((k as f64) * ((length - 1) as f64) / ((max - 1) as f64)).round() as usize)
        .collect();
    let pick = |series: &Vec<f64>| {
        indices
            .iter()
            .map(|index| series[*index])
            .collect::<Vec<_>>()
    };
    table.axis = pick(&table.axis);
    for series in table.real.values_mut() {
        *series = pick(series);
    }
    for series in table.complex.values_mut() {
        *series = indices.iter().map(|index| series[*index]).collect();
    }
}

/// Every series has one value per axis point (an operating point or scalar
/// table has one row), and the table holds at least one series.
pub fn validate(sections: &BTreeMap<String, Table>) -> Result<(), String> {
    if sections.is_empty() {
        return Err("the oracle produced no analysis section".into());
    }
    for (label, table) in sections {
        let rows = table_rows(table);
        if table.real.is_empty() && table.complex.is_empty() {
            return Err(format!("{label}: no oracle vectors"));
        }
        for (name, length) in table
            .real
            .iter()
            .map(|(name, series)| (name, series.len()))
            .chain(
                table
                    .complex
                    .iter()
                    .map(|(name, series)| (name, series.len())),
            )
        {
            if length != rows {
                return Err(format!(
                    "{label}: {name} has {length} values for {rows} rows"
                ));
            }
        }
    }
    Ok(())
}

/// Rows a table renders: one per axis point; an operating-point or scalar
/// table has one; a root table may have none.
fn table_rows(table: &Table) -> usize {
    if table.axis_name == "root" {
        table.axis.len()
    } else {
        table.axis.len().max(1)
    }
}

/// Render a reference file. Numbers use Rust's shortest round-trip form, so a
/// recapture of identical oracle output is byte-identical.
pub fn render(producer: Producer, header: &[String], sections: &BTreeMap<String, Table>) -> String {
    let mut out = String::from(producer.header());
    out.push('\n');
    for line in header {
        out.push_str("# ");
        out.push_str(line);
        out.push('\n');
    }
    for (label, table) in sections {
        let _ = writeln!(out, "# analysis: {label}");
        let mut columns = vec!["Index".to_owned(), table.axis_name.clone()];
        columns.extend(table.real.keys().cloned());
        for name in table.complex.keys() {
            columns.push(format!("{name}.re"));
            columns.push(format!("{name}.im"));
        }
        out.push_str(&columns.join(" "));
        out.push('\n');
        for row in 0..table_rows(table) {
            let mut fields = vec![
                row.to_string(),
                table.axis.get(row).copied().unwrap_or(0.0).to_string(),
            ];
            for series in table.real.values() {
                fields.push(series[row].to_string());
            }
            for series in table.complex.values() {
                fields.push(series[row][0].to_string());
                fields.push(series[row][1].to_string());
            }
            out.push_str(&fields.join(" "));
            out.push('\n');
        }
        out.push_str("# end-analysis\n");
    }
    out
}

/// Parse a reference file.
pub fn parse(text: &str) -> Result<Reference, String> {
    let mut lines = text.lines();
    let first = lines.next().unwrap_or_default();
    let producer = Producer::from_header(first)
        .ok_or_else(|| format!("unknown reference header {first:?}"))?;
    let mut header = Vec::new();
    let mut sections = BTreeMap::new();
    let mut current: Option<(String, Vec<String>, Table)> = None;
    for line in lines {
        if let Some(label) = line.strip_prefix("# analysis: ") {
            if current.is_some() {
                return Err("nested analysis section".into());
            }
            current = Some((label.to_owned(), Vec::new(), Table::default()));
            continue;
        }
        if line == "# end-analysis" {
            let (label, _, table) = current.take().ok_or("terminator without a section")?;
            if sections.insert(label.clone(), table).is_some() {
                return Err(format!("duplicate section {label}"));
            }
            continue;
        }
        match &mut current {
            None => {
                if let Some(comment) = line.strip_prefix("# ") {
                    header.push(comment.to_owned());
                } else if !line.trim().is_empty() {
                    return Err(format!("unexpected reference line {line:?}"));
                }
            }
            Some((_, columns, table)) if columns.is_empty() => {
                *columns = line.split_whitespace().map(str::to_owned).collect();
                if columns.len() < 2 || columns[0] != "Index" {
                    return Err(format!("bad table header {line:?}"));
                }
                table.axis_name = columns[1].clone();
            }
            Some((_, columns, table)) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() != columns.len() {
                    return Err(format!(
                        "row has {} fields, header {}",
                        fields.len(),
                        columns.len()
                    ));
                }
                let values = fields[1..]
                    .iter()
                    .map(|field| {
                        field
                            .parse::<f64>()
                            .ok()
                            .filter(|value| value.is_finite())
                            .ok_or_else(|| format!("bad number {field}"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                table.axis.push(values[0]);
                let mut index = 2;
                while index < columns.len() {
                    let name = &columns[index];
                    if let Some(base) = name.strip_suffix(".re") {
                        let im = values[index];
                        table
                            .complex
                            .entry(base.to_owned())
                            .or_default()
                            .push([values[index - 1], im]);
                        index += 2;
                    } else {
                        table
                            .real
                            .entry(name.clone())
                            .or_default()
                            .push(values[index - 1]);
                        index += 1;
                    }
                }
            }
        }
    }
    if current.is_some() {
        return Err("unterminated analysis section".into());
    }
    let refused = header.iter().any(|line| line.starts_with(REFUSED_PREFIX));
    if sections.is_empty() != refused {
        return Err(if refused {
            "an oracle-refused reference carries no analysis".into()
        } else {
            "reference has no analysis".into()
        });
    }
    Ok(Reference {
        producer,
        header,
        sections,
    })
}

/// Header line of a reference the oracle could not produce (design §3 C5:
/// "if ngspice pz fails, the ref records oracle-refused and only O5 applies").
pub const REFUSED_PREFIX: &str = "oracle-refused: ";

/// A loaded reference: the oracle's observation, or its recorded refusal.
#[derive(Debug, Clone)]
pub enum LoadedReference {
    Observed(Box<Observation>),
    Refused(String),
}

impl Reference {
    pub fn refusal(&self) -> Option<&str> {
        self.header
            .iter()
            .find_map(|line| line.strip_prefix(REFUSED_PREFIX))
    }

    /// The reference as an observation of its (single) analysis.
    pub fn observation(&self) -> Result<Observation, String> {
        let mut observation = Observation::default();
        for (label, table) in &self.sections {
            match table.axis_name.as_str() {
                "root" => {
                    let roots: Vec<[f64; 2]> = table
                        .real
                        .get("re")
                        .zip(table.real.get("im"))
                        .map(|(re, im)| re.iter().zip(im).map(|(re, im)| [*re, *im]).collect())
                        .unwrap_or_default();
                    if label.ends_with("zeros") {
                        observation.zeros = roots;
                    } else {
                        observation.poles = roots;
                    }
                    observation.kind = "pz".into();
                }
                "scalar" => {
                    for (name, series) in &table.real {
                        if let Some(value) = series.first() {
                            observation.scalars.insert(name.clone(), *value);
                        }
                    }
                    observation.kind = label.clone();
                }
                _ => {
                    observation.kind = label.clone();
                    if table.axis_name != "op" {
                        observation.axis = table.axis.clone();
                    }
                    for (name, series) in &table.real {
                        observation.real.insert(name.clone(), series.clone());
                    }
                    for (name, series) in &table.complex {
                        observation.complex.insert(name.clone(), series.clone());
                    }
                }
            }
        }
        Ok(observation)
    }
}

/// Load a reference (and its events, if captured) as an observation.
/// `Ok(None)` when the file does not exist: the oracle has not run yet.
pub fn load_reference(
    design_dir: &Path,
    reference: &str,
) -> Result<Option<LoadedReference>, String> {
    let path = design_dir.join(reference);
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(|error| format!("{reference}: {error}"))?;
    let parsed = parse(&text).map_err(|error| format!("{reference}: {error}"))?;
    if let Some(reason) = parsed.refusal() {
        return Ok(Some(LoadedReference::Refused(reason.to_owned())));
    }
    let mut observation = parsed.observation()?;
    let events = path.with_file_name(
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .replace(".out", ".events.tsv"),
    );
    if events.is_file() {
        let text = fs::read_to_string(&events).map_err(|error| error.to_string())?;
        let (_, traces) = super::events::read_events_tsv(&text)?;
        observation.events = traces;
    }
    Ok(Some(LoadedReference::Observed(Box::new(observation))))
}

/// Canonical column name for a raw vector from any oracle.
pub fn column_name(raw: &str) -> String {
    canonical_signal(raw)
}

/// An oracle program that could not be found, with what to install.
#[derive(Debug, Clone)]
pub struct OracleAbsent {
    pub oracle: &'static str,
    pub probed: Vec<String>,
    pub hint: &'static str,
}

impl std::fmt::Display for OracleAbsent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} oracle absent (probed {}); {}",
            self.oracle,
            self.probed.join(", "),
            self.hint
        )
    }
}

/// `RSPICE_MIXED_ORACLES_REQUIRED=1` turns an absent live oracle into a
/// failure instead of an unverified quantity.
pub fn oracles_required() -> bool {
    std::env::var("RSPICE_MIXED_ORACLES_REQUIRED").is_ok_and(|value| value == "1")
}

/// Resolve an executable from an override variable or a default path.
pub fn discover(
    oracle: &'static str,
    override_env: &str,
    default: &str,
    hint: &'static str,
) -> Result<PathBuf, OracleAbsent> {
    let mut probed = Vec::new();
    if let Some(path) = std::env::var_os(override_env) {
        let path = PathBuf::from(path);
        probed.push(format!("{override_env}={}", path.display()));
        return if path.is_file() {
            Ok(path)
        } else {
            Err(OracleAbsent {
                oracle,
                probed,
                hint,
            })
        };
    }
    let path = PathBuf::from(default);
    probed.push(path.display().to_string());
    if path.is_file() {
        Ok(path)
    } else {
        Err(OracleAbsent {
            oracle,
            probed,
            hint,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_round_trips_real_complex_and_scalar_sections() {
        let mut sections = BTreeMap::new();
        let mut tran = Table {
            axis_name: "time".into(),
            axis: vec![0.0, 1e-9, 2e-9],
            ..Default::default()
        };
        tran.real.insert("v(a)".into(), vec![0.0, 0.5, 1.0]);
        sections.insert("tran".to_owned(), tran);
        let text = render(
            Producer::Ngspice,
            &["ngspice: test".into(), binding_line("abc")],
            &sections,
        );
        let parsed = parse(&text).unwrap();
        assert_eq!(parsed.sections, sections);
        assert_eq!(
            render(Producer::Ngspice, &parsed.header, &parsed.sections),
            text
        );
        let observation = parsed.observation().unwrap();
        assert_eq!(observation.real["v(a)"], vec![0.0, 0.5, 1.0]);

        let mut ac = Table {
            axis_name: "frequency".into(),
            axis: vec![1.0, 10.0],
            ..Default::default()
        };
        ac.complex
            .insert("v(o)".into(), vec![[1.0, -0.5], [0.1, -0.9]]);
        let mut sections = BTreeMap::new();
        sections.insert("ac".to_owned(), ac);
        let text = render(Producer::Xyce, &[], &sections);
        assert_eq!(
            parse(&text).unwrap().observation().unwrap().complex["v(o)"][1],
            [0.1, -0.9]
        );
        assert!(parse(&text.replace("0.1 -0.9", "0.1 NaN")).is_err());
        assert!(parse("# RSPICE-OTHER 2\n").is_err());
    }

    #[test]
    fn decimation_keeps_the_ends() {
        let mut table = Table {
            axis_name: "time".into(),
            axis: (0..101).map(f64::from).collect(),
            ..Default::default()
        };
        table
            .real
            .insert("v(a)".into(), (0..101).map(f64::from).collect());
        decimate(&mut table, 11);
        assert_eq!(table.axis.len(), 11);
        assert_eq!(table.axis[0], 0.0);
        assert_eq!(table.axis[10], 100.0);
        assert_eq!(table.real["v(a)"][5], 50.0);
    }

    #[test]
    fn references_name_their_oracle_decks() {
        let dir = Path::new("c05");
        assert_eq!(
            deck_for_reference(dir, "mx.op.O5-O1.out").unwrap(),
            dir.join("oracle/mx.op.o5.ngspice.cir")
        );
        assert_eq!(
            deck_for_reference(dir, "xs.tran.O1.out").unwrap(),
            dir.join("oracle/xs.tran.ngspice.cir")
        );
        assert_eq!(
            deck_for_reference(dir, "vams.tran.O2.events.tsv").unwrap(),
            dir.join("oracle/vams.o2/deck.cir")
        );
        assert!(deck_for_reference(dir, "xs.tran.O6.out").is_err());
    }
}
