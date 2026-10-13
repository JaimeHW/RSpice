//! `tests/verilog/ivtest-manifest.tsv` and the ratchet that holds it.
//!
//! One row per candidate test: its name, the list it came from, its class,
//! its lane bucket, its outcome and an informational detail. Names and
//! RSpice's outcomes only: no source, gold text or comment of the GPL suite
//! is copied into it.
//!
//! The ratchet compares outcomes and nothing else. The detail column carries
//! RSpice's own diagnostics, which change wording without changing meaning,
//! so it is rewritten by `--record` and never compared. Any outcome that
//! moved fails the run: `regressed` when it got worse in the order
//! pass > racy > refused > fail, and `changed` when it improved, moved
//! between refusals, or moved in or out of the selection, until it is
//! re-recorded.

use std::collections::BTreeMap;
use std::fmt;

use super::judge::Outcome;
use super::list::SourceList;
use super::select::{Class, Lane};

pub const FILE_NAME: &str = "ivtest-manifest.tsv";

pub const HEADER: &str = "# test\tlist\tclass\tlane\toutcome\tdetail";

const PREAMBLE: &str = "# RSpice outcomes on the Icarus Verilog ivtest suite (IEEE 1364-2005 selection).\n\
# Test names and RSpice results only; the suite itself is read from RSPICE_IVTEST_DIR at run time.\n\
# Regenerate with `rspice-ivtest run --record`; see tools/rspice-conformance/src/suites/verilog/ivtest.rs.\n";

/// One test's recorded row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub name: String,
    pub list: SourceList,
    /// `None` for a test whose type is outside the four classes.
    pub class: Option<Class>,
    /// `None` for an excluded test.
    pub lane: Option<Lane>,
    pub outcome: Outcome,
    pub detail: String,
}

pub fn parse(text: &str) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let at = |what: &str| format!("{FILE_NAME}:{}: {what}", index + 1);
        let fields: Vec<&str> = line.splitn(6, '\t').collect();
        let [name, list, class, lane, outcome, detail] = fields.as_slice() else {
            return Err(at("expected 6 tab-separated fields"));
        };
        fn optional(token: &str) -> Option<&str> {
            (token != "-").then_some(token)
        }
        rows.push(Row {
            name: (*name).to_owned(),
            list: SourceList::from_token(list)
                .ok_or_else(|| at(&format!("unknown list {list}")))?,
            class: optional(class)
                .map(|token| {
                    Class::from_token(token).ok_or_else(|| at(&format!("unknown class {token}")))
                })
                .transpose()?,
            lane: optional(lane)
                .map(|token| {
                    Lane::from_token(token).ok_or_else(|| at(&format!("unknown lane {token}")))
                })
                .transpose()?,
            outcome: Outcome::from_token(outcome)
                .ok_or_else(|| at(&format!("unknown outcome {outcome}")))?,
            detail: (*detail).to_owned(),
        });
    }
    Ok(rows)
}

/// The table, sorted by list and then by name.
pub fn render(rows: &[Row]) -> String {
    let mut sorted: Vec<&Row> = rows.iter().collect();
    sorted.sort_by(|a, b| (a.list, &a.name).cmp(&(b.list, &b.name)));
    let mut out = String::from(PREAMBLE);
    out.push_str(HEADER);
    out.push('\n');
    for row in sorted {
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            row.name,
            row.list.token(),
            row.class.map_or("-", Class::token),
            row.lane.map_or("-", Lane::token),
            row.outcome,
            row.detail.replace(['\t', '\n', '\r'], " ")
        ));
    }
    out
}

/// One test whose outcome moved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Drift {
    Regressed {
        name: String,
        recorded: Outcome,
        observed: Outcome,
        detail: String,
    },
    Changed {
        name: String,
        recorded: Outcome,
        observed: Outcome,
        detail: String,
    },
    /// Observed, with no row.
    Unrecorded { name: String, observed: Outcome },
    /// A row whose test the checkout no longer lists.
    Vanished { name: String, recorded: Outcome },
}

impl fmt::Display for Drift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regressed {
                name,
                recorded,
                observed,
                detail,
            } => write!(f, "regressed {name}: {recorded} -> {observed} ({detail})"),
            Self::Changed {
                name,
                recorded,
                observed,
                detail,
            } => write!(
                f,
                "changed {name}: {recorded} -> {observed} ({detail}); re-record"
            ),
            Self::Unrecorded { name, observed } => {
                write!(f, "unrecorded {name}: {observed}; re-record")
            }
            Self::Vanished { name, recorded } => {
                write!(f, "vanished {name}: recorded {recorded}, no longer listed")
            }
        }
    }
}

/// Compare observed rows with the recorded ones. `complete` says the run
/// covered the whole candidate pool, so a recorded test it did not observe
/// has vanished from the checkout.
pub fn ratchet(recorded: &[Row], observed: &[Row], complete: bool) -> Vec<Drift> {
    let table: BTreeMap<&str, &Row> = recorded
        .iter()
        .map(|row| (row.name.as_str(), row))
        .collect();
    let mut drift = Vec::new();
    for row in observed {
        match table.get(row.name.as_str()) {
            None => drift.push(Drift::Unrecorded {
                name: row.name.clone(),
                observed: row.outcome.clone(),
            }),
            Some(old) if old.outcome == row.outcome => {}
            Some(old) if row.outcome.rank() < old.outcome.rank() => drift.push(Drift::Regressed {
                name: row.name.clone(),
                recorded: old.outcome.clone(),
                observed: row.outcome.clone(),
                detail: row.detail.clone(),
            }),
            Some(old) => drift.push(Drift::Changed {
                name: row.name.clone(),
                recorded: old.outcome.clone(),
                observed: row.outcome.clone(),
                detail: row.detail.clone(),
            }),
        }
    }
    if complete {
        let seen: std::collections::BTreeSet<&str> =
            observed.iter().map(|row| row.name.as_str()).collect();
        for row in recorded {
            if !seen.contains(row.name.as_str()) {
                drift.push(Drift::Vanished {
                    name: row.name.clone(),
                    recorded: row.outcome.clone(),
                });
            }
        }
    }
    drift
}

/// The recorded rows with the observed ones written over them. A complete
/// run replaces the table.
pub fn record(recorded: &[Row], observed: &[Row], complete: bool) -> Vec<Row> {
    let mut merged: BTreeMap<String, Row> = if complete {
        BTreeMap::new()
    } else {
        recorded
            .iter()
            .map(|row| (row.name.clone(), row.clone()))
            .collect()
    };
    for row in observed {
        merged.insert(row.name.clone(), row.clone());
    }
    merged.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(name: &str, outcome: Outcome) -> Row {
        Row {
            name: name.to_owned(),
            list: SourceList::Vlg,
            class: Some(Class::Passed),
            lane: Some(Lane::L1),
            outcome,
            detail: "d".to_owned(),
        }
    }

    #[test]
    fn rows_round_trip_through_the_table() {
        let rows = vec![
            row("b", Outcome::Pass),
            Row {
                name: "a".to_owned(),
                list: SourceList::Vvp,
                class: None,
                lane: None,
                outcome: Outcome::Excluded("type-NI".to_owned()),
                detail: String::new(),
            },
            row("c", Outcome::Refused("hdl.option.specify".to_owned())),
        ];
        let text = render(&rows);
        assert!(text.contains(HEADER), "{text}");
        let parsed = parse(&text).expect("the rendered table parses");
        // Sorted by list, then name.
        let names: Vec<&str> = parsed.iter().map(|row| row.name.as_str()).collect();
        assert_eq!(names, ["b", "c", "a"]);
        for original in &rows {
            assert!(parsed.contains(original), "{original:?}");
        }
        assert!(parse("x\tvlg\tpassed\tL1\tpass\n").is_err());
        assert!(parse("x\tvlg\tpassed\tL1\tpassing\td\n").is_err());
    }

    #[test]
    fn every_moved_outcome_is_drift_and_only_worse_ones_regress() {
        let recorded = vec![
            row("same", Outcome::Pass),
            row("worse", Outcome::Pass),
            row("better", Outcome::Fail),
            row("other-refusal", Outcome::Refused("task".to_owned())),
            row("gone", Outcome::Fail),
        ];
        let mut observed = vec![
            row("same", Outcome::Pass),
            row("worse", Outcome::Racy),
            row("better", Outcome::Pass),
            row("other-refusal", Outcome::Refused("fork".to_owned())),
            row("new", Outcome::Fail),
        ];
        // The detail column is never compared.
        observed[0].detail = "reworded".to_owned();
        let drift = ratchet(&recorded, &observed, true);
        assert_eq!(drift.len(), 5, "{drift:#?}");
        assert!(matches!(&drift[0], Drift::Regressed { name, .. } if name == "worse"));
        assert!(matches!(&drift[1], Drift::Changed { name, .. } if name == "better"));
        assert!(matches!(&drift[2], Drift::Changed { name, .. } if name == "other-refusal"));
        assert!(matches!(&drift[3], Drift::Unrecorded { name, .. } if name == "new"));
        assert!(matches!(&drift[4], Drift::Vanished { name, .. } if name == "gone"));
        // A partial run says nothing about the tests it skipped.
        assert_eq!(ratchet(&recorded, &observed, false).len(), 4);
    }

    #[test]
    fn a_partial_record_keeps_the_other_rows() {
        let recorded = vec![row("a", Outcome::Fail), row("b", Outcome::Fail)];
        let observed = vec![row("a", Outcome::Pass)];
        let partial = record(&recorded, &observed, false);
        assert_eq!(partial.len(), 2);
        assert_eq!(partial[0].outcome, Outcome::Pass);
        assert_eq!(partial[1].outcome, Outcome::Fail);
        assert_eq!(record(&recorded, &observed, true).len(), 1);
    }
}
