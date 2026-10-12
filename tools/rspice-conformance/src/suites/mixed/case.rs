//! `case-result.json` (schema `rspice-mixed-case/1`): what one case child, or an
//! external producer standing in for one, hands the comparator.
//!
//! The file carries what the run delivered and nothing the comparator decides:
//! the typed failure if there was one, and otherwise one [`Observation`] per
//! task in a producer-neutral shape. Signal names are canonical (lower case,
//! `v(node)` / `i(source)`); a non-finite series is never written as a number
//! but listed in [`Observation::nonfinite`], so a producer cannot hide one
//! behind JSON's lack of NaN.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub const CASE_SCHEMA: &str = "rspice-mixed-case/1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaseResult {
    pub schema: String,
    pub route: String,
    pub rspice_rev: String,
    /// `simulation` (headless preparation + runner), `engine` (the A1
    /// fallback), or `none` when the case could not be dispatched at all.
    pub dispatch: String,
    #[serde(default)]
    pub dispatch_reason: Option<String>,
    /// `completed` or `error`. A crash leaves no file.
    pub outcome: String,
    #[serde(default)]
    pub error: Option<ErrorRecord>,
    #[serde(default)]
    pub results: Vec<Observation>,
    #[serde(default)]
    pub warnings: Vec<String>,
    pub elapsed_s: f64,
}

impl CaseResult {
    pub fn new(route: &str, dispatch: &str) -> Self {
        Self {
            schema: CASE_SCHEMA.to_owned(),
            route: route.to_owned(),
            rspice_rev: rspice_rev(),
            dispatch: dispatch.to_owned(),
            dispatch_reason: None,
            outcome: "completed".to_owned(),
            error: None,
            results: Vec::new(),
            warnings: Vec::new(),
            elapsed_s: 0.0,
        }
    }

    pub fn failed(mut self, error: ErrorRecord) -> Self {
        self.outcome = "error".to_owned();
        self.error = Some(error);
        self
    }

    pub fn write(&self, path: &Path) -> Result<(), String> {
        let text = serde_json::to_string(self).map_err(|error| error.to_string())?;
        fs::write(path, text).map_err(|error| format!("{}: {error}", path.display()))
    }

    /// Read a result file. `Err` means the producer crashed or wrote garbage.
    pub fn read(path: &Path) -> Result<Self, String> {
        let text =
            fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
        let result: Self =
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
        if result.schema != CASE_SCHEMA {
            return Err(format!("{}: schema {:?}", path.display(), result.schema));
        }
        Ok(result)
    }
}

/// Every field of the typed failure the producer saw, plus its `Display`.
///
/// `form` records where each identity came from: `structured` when the
/// producer's error type carried it, `prose` when the classifier can only find
/// it in the message. Both runners carry a capability refusal's token,
/// analysis kind, instance and module as typed fields.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorRecord {
    /// `capability`, `elaboration`, `parse`, `compile`, `convergence`,
    /// `solver`, `simulation`, `internal`, `resource`, `timeout`,
    /// `preparation`, `harness`.
    pub category: String,
    #[serde(default)]
    pub code: Option<String>,
    /// Capability token or elaboration/parse/compile kind.
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    pub token_form: Option<String>,
    #[serde(default)]
    pub instance: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    /// Structured analysis instance identity, when the error carried one.
    #[serde(default)]
    pub analysis: Option<String>,
    /// Tag of the analysis kind that refused (`ac`, `pss`), when the error
    /// carried one but no authored card identity.
    #[serde(default)]
    pub analysis_kind: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    /// `preparation`, `compile` or `execution`.
    #[serde(default)]
    pub stage: Option<String>,
    /// The reason, without the token.
    pub detail: String,
    /// The complete message a person reads.
    pub display: String,
}

/// One producer-neutral analysis result.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    /// `op`, `dc`, `ac`, `noise`, `tf`, `stb`, `pz`, `sens`, `tran`, `mc`,
    /// `step`, `pss`, `pnoise`, `pac` or `other`.
    pub kind: String,
    pub label: String,
    /// Time, frequency or sweep values; empty for an operating point.
    #[serde(default)]
    pub axis: Vec<f64>,
    #[serde(default)]
    pub real: BTreeMap<String, Vec<f64>>,
    /// `[re, im]` per axis point.
    #[serde(default)]
    pub complex: BTreeMap<String, Vec<[f64; 2]>>,
    /// Named scalars: transfer-function `gain|zin|zout`, stability
    /// `pm|gm|ugf`, sensitivities by element name.
    #[serde(default)]
    pub scalars: BTreeMap<String, f64>,
    #[serde(default)]
    pub poles: Vec<[f64; 2]>,
    #[serde(default)]
    pub zeros: Vec<[f64; 2]>,
    /// Digital traces as `(time, 0|1|x|z)` and real traces as `(time, value)`.
    #[serde(default)]
    pub events: BTreeMap<String, Vec<(f64, String)>>,
    /// MC trials or `.step` points, each a complete observation.
    #[serde(default)]
    pub trials: Vec<Observation>,
    /// Signals the producer delivered with non-finite values (withheld).
    #[serde(default)]
    pub nonfinite: Vec<String>,
    /// Every signal name the producer delivered before retention filtering.
    #[serde(default)]
    pub available: Vec<String>,
}

impl Observation {
    /// Keep only the named signals and nets; record what was available.
    pub fn retain(&mut self, signals: &[String], nets: &[String]) {
        let mut available: Vec<String> = self
            .real
            .keys()
            .chain(self.complex.keys())
            .chain(self.events.keys())
            .cloned()
            .collect();
        available.sort();
        available.dedup();
        self.available = available;
        if !signals.iter().any(|name| name == "*") {
            self.real.retain(|name, _| signals.contains(name));
            self.complex.retain(|name, _| signals.contains(name));
        }
        if !nets.iter().any(|name| name == "*") {
            self.events.retain(|name, _| nets.contains(name));
        }
        for trial in &mut self.trials {
            trial.retain(signals, nets);
        }
    }

    /// Move non-finite series out of the numeric maps into `nonfinite`.
    pub fn quarantine_nonfinite(&mut self) {
        let mut bad = Vec::new();
        self.real.retain(|name, values| {
            let finite = values.iter().all(|value| value.is_finite());
            if !finite {
                bad.push(name.clone());
            }
            finite
        });
        self.complex.retain(|name, values| {
            let finite = values
                .iter()
                .all(|[re, im]| re.is_finite() && im.is_finite());
            if !finite {
                bad.push(name.clone());
            }
            finite
        });
        self.scalars.retain(|name, value| {
            if !value.is_finite() {
                bad.push(name.clone());
            }
            value.is_finite()
        });
        for (name, points) in &mut self.events {
            if points.iter().any(|(time, _)| !time.is_finite()) {
                bad.push(name.clone());
            }
        }
        if self.axis.iter().any(|value| !value.is_finite()) {
            bad.push("<axis>".to_owned());
            self.axis.clear();
        }
        for trial in &mut self.trials {
            trial.quarantine_nonfinite();
        }
        self.nonfinite.extend(bad);
    }
}

/// Canonical spelling of an output name: lower case; a bare node is `v(node)`;
/// a SPICE-style branch (`vdd#branch`, ngspice's `v.xlf.vdd#branch`) is
/// `i(name)` with the device-letter prefix ngspice adds inside subcircuits
/// removed.
pub fn canonical_signal(name: &str) -> String {
    let lower = name.trim().to_ascii_lowercase();
    if let Some(branch) = lower.strip_suffix("#branch") {
        let branch = branch
            .strip_prefix("v.")
            .or_else(|| branch.strip_prefix("l."))
            .or_else(|| branch.strip_prefix("e."))
            .or_else(|| branch.strip_prefix("h."))
            .unwrap_or(branch);
        return format!("i({branch})");
    }
    if (lower.starts_with("v(") || lower.starts_with("i(")) && lower.ends_with(')') {
        let inner = lower[2..lower.len() - 1].trim();
        // ngspice spells a subcircuit source's current `i(v.xlf.vdd)`.
        if lower.starts_with("i(") {
            let inner = inner.strip_prefix("v.").unwrap_or(inner);
            return format!("i({inner})");
        }
        return format!("v({inner})");
    }
    format!("v({lower})")
}

/// The repository revision the producer was built from, best effort.
pub fn rspice_rev() -> String {
    if let Some(rev) = option_env!("RSPICE_GIT_REV") {
        return rev.to_owned();
    }
    let root = super::workspace_root();
    std::process::Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .current_dir(&root)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|rev| !rev.is_empty())
        .unwrap_or_else(|| "unknown".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_names_canonicalize_across_producers() {
        assert_eq!(canonical_signal("V(OUT)"), "v(out)");
        assert_eq!(canonical_signal("xmod.outintp"), "v(xmod.outintp)");
        assert_eq!(canonical_signal("v(xmod.outintp)"), "v(xmod.outintp)");
        assert_eq!(canonical_signal("vdd#branch"), "i(vdd)");
        assert_eq!(canonical_signal("v.xlf.vdd#branch"), "i(xlf.vdd)");
        assert_eq!(canonical_signal("I(XLF.VDD)"), "i(xlf.vdd)");
        assert_eq!(canonical_signal("i(v.xlf.vdd)"), "i(xlf.vdd)");
    }

    #[test]
    fn nonfinite_series_are_withheld_and_named() {
        let mut observation = Observation {
            kind: "tran".into(),
            axis: vec![0.0, 1.0],
            ..Default::default()
        };
        observation.real.insert("v(a)".into(), vec![1.0, f64::NAN]);
        observation.real.insert("v(b)".into(), vec![1.0, 2.0]);
        observation.quarantine_nonfinite();
        assert_eq!(observation.nonfinite, vec!["v(a)".to_owned()]);
        assert!(observation.real.contains_key("v(b)"));
        assert!(serde_json::to_string(&observation).is_ok());
    }

    #[test]
    fn result_file_round_trips_and_rejects_other_schemas() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("case-result.json");
        let mut result = CaseResult::new("interp", "simulation");
        result.results.push(Observation {
            kind: "op".into(),
            ..Default::default()
        });
        result.write(&path).unwrap();
        let read = CaseResult::read(&path).unwrap();
        assert_eq!(read.results.len(), 1);
        fs::write(&path, "{\"schema\":\"other\"}").unwrap();
        assert!(CaseResult::read(&path).is_err());
        assert!(CaseResult::read(&dir.path().join("missing.json")).is_err());
    }
}
