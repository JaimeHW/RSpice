//! `outcomes.tsv`: the P0.5 baseline of every design × variant × analysis ×
//! route cell, and the ratchet that holds it.
//!
//! `run` compares each observed outcome with its recorded row and fails on any
//! difference — `regressed` when the cell got worse in the order
//! pass > unverified > refused > fail > wrong, `stale` when it got better
//! without being re-recorded. `run --record` rewrites the rows of the routes it
//! ran from what it observed; the landing manager reviews that diff. The detail
//! column is informational (it carries measured numbers that move in the last
//! digit between machines) and is not compared.

use std::collections::BTreeMap;

use super::{Outcome, Route};

pub const HEADER: &str = "# variant\tanalysis\troute\toutcome\tdetail";

/// One recorded cell.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub variant: String,
    pub analysis: String,
    pub route: Route,
    pub outcome: Outcome,
    pub detail: String,
}

pub type Key = (String, String, Route);

impl Row {
    pub fn key(&self) -> Key {
        (self.variant.clone(), self.analysis.clone(), self.route)
    }
}

pub fn parse(text: &str) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.splitn(5, '\t').collect();
        let [variant, analysis, route, outcome, detail] = fields.as_slice() else {
            return Err(format!(
                "outcomes.tsv:{}: expected 5 tab-separated fields",
                index + 1
            ));
        };
        rows.push(Row {
            variant: (*variant).to_owned(),
            analysis: (*analysis).to_owned(),
            route: Route::from_token(route)
                .ok_or_else(|| format!("outcomes.tsv:{}: unknown route {route}", index + 1))?,
            outcome: Outcome::from_token(outcome)
                .ok_or_else(|| format!("outcomes.tsv:{}: unknown outcome {outcome}", index + 1))?,
            detail: (*detail).to_owned(),
        });
    }
    Ok(rows)
}

pub fn render(rows: &[Row]) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');
    for row in rows {
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            row.variant,
            row.analysis,
            row.route,
            row.outcome,
            row.detail.replace(['\t', '\n', '\r'], " ")
        ));
    }
    out
}

/// One difference between the recorded table and a run.
#[derive(Debug, Clone, PartialEq)]
pub enum Drift {
    Regressed {
        key: Key,
        recorded: Outcome,
        observed: Outcome,
        detail: String,
    },
    Stale {
        key: Key,
        recorded: Outcome,
        observed: Outcome,
        detail: String,
    },
    Unrecorded {
        key: Key,
        observed: Outcome,
    },
}

impl std::fmt::Display for Drift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regressed {
                key,
                recorded,
                observed,
                detail,
            } => write!(
                f,
                "regressed {} {} {}: {recorded} -> {observed} ({detail})",
                key.0, key.1, key.2
            ),
            Self::Stale {
                key,
                recorded,
                observed,
                detail,
            } => write!(
                f,
                "stale {} {} {}: recorded {recorded}, now {observed} ({detail}); re-record",
                key.0, key.1, key.2
            ),
            Self::Unrecorded { key, observed } => {
                write!(f, "unrecorded {} {} {}: {observed}", key.0, key.1, key.2)
            }
        }
    }
}

/// Compare observed rows against the recorded table.
pub fn ratchet(recorded: &[Row], observed: &[Row]) -> Vec<Drift> {
    let table: BTreeMap<Key, &Row> = recorded.iter().map(|row| (row.key(), row)).collect();
    let mut drift = Vec::new();
    for row in observed {
        match table.get(&row.key()) {
            None => drift.push(Drift::Unrecorded {
                key: row.key(),
                observed: row.outcome,
            }),
            Some(recorded) if recorded.outcome == row.outcome => {}
            Some(recorded) if row.outcome.rank() < recorded.outcome.rank() => {
                drift.push(Drift::Regressed {
                    key: row.key(),
                    recorded: recorded.outcome,
                    observed: row.outcome,
                    detail: row.detail.clone(),
                })
            }
            Some(recorded) => drift.push(Drift::Stale {
                key: row.key(),
                recorded: recorded.outcome,
                observed: row.outcome,
                detail: row.detail.clone(),
            }),
        }
    }
    drift
}

/// Replace the rows of the observed routes, keep the other routes' rows, and
/// order the table as the design declares its cells.
pub fn record(recorded: &[Row], observed: &[Row], order: &[(String, String)]) -> Vec<Row> {
    let mut merged: BTreeMap<Key, Row> = recorded
        .iter()
        .map(|row| (row.key(), row.clone()))
        .collect();
    for row in observed {
        merged.insert(row.key(), row.clone());
    }
    let position = |row: &Row| {
        order
            .iter()
            .position(|(variant, analysis)| *variant == row.variant && *analysis == row.analysis)
            .unwrap_or(usize::MAX)
    };
    let mut rows: Vec<Row> = merged.into_values().collect();
    rows.sort_by_key(|row| (position(row), row.route));
    rows
}

/// The table must hold exactly the declared cells for every route it names.
pub fn coverage(rows: &[Row], cells: &[(String, String)]) -> Vec<String> {
    let mut findings = Vec::new();
    let mut routes: Vec<Route> = rows.iter().map(|row| row.route).collect();
    routes.sort();
    routes.dedup();
    for route in [Route::Interp, Route::Native] {
        if !routes.contains(&route) {
            findings.push(format!("outcomes.tsv has no {route} rows"));
        }
    }
    let mut seen = BTreeMap::new();
    for row in rows {
        if seen.insert(row.key(), ()).is_some() {
            findings.push(format!(
                "outcomes.tsv repeats {} {} {}",
                row.variant, row.analysis, row.route
            ));
        }
        if !cells
            .iter()
            .any(|(variant, analysis)| *variant == row.variant && *analysis == row.analysis)
        {
            findings.push(format!(
                "outcomes.tsv row {} {} is not a declared cell",
                row.variant, row.analysis
            ));
        }
    }
    for route in routes {
        for (variant, analysis) in cells {
            if !seen.contains_key(&(variant.clone(), analysis.clone(), route)) {
                findings.push(format!("outcomes.tsv lacks {variant} {analysis} {route}"));
            }
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(analysis: &str, route: Route, outcome: Outcome) -> Row {
        Row {
            variant: "vams".into(),
            analysis: analysis.into(),
            route,
            outcome,
            detail: "d".into(),
        }
    }

    #[test]
    fn table_round_trips() {
        let rows = vec![
            row("ac", Route::Interp, Outcome::Refused),
            row("tran", Route::Native, Outcome::Pass),
        ];
        assert_eq!(parse(&render(&rows)).unwrap(), rows);
        assert!(parse("vams\tac\tinterp\tpass").is_err());
        assert!(parse("vams\tac\tmoon\tpass\td").is_err());
    }

    #[test]
    fn ratchet_reports_regressions_and_stale_improvements() {
        let recorded = vec![
            row("ac", Route::Interp, Outcome::Refused),
            row("tran", Route::Interp, Outcome::Unverified),
        ];
        let observed = vec![
            row("ac", Route::Interp, Outcome::Wrong),
            row("tran", Route::Interp, Outcome::Pass),
        ];
        let drift = ratchet(&recorded, &observed);
        assert!(matches!(drift[0], Drift::Regressed { .. }));
        assert!(matches!(drift[1], Drift::Stale { .. }));
        assert!(ratchet(&recorded, &recorded).is_empty());
        assert!(matches!(
            ratchet(&[], &recorded)[0],
            Drift::Unrecorded { .. }
        ));
    }

    #[test]
    fn record_keeps_other_routes_and_coverage_is_two_way() {
        let cells = vec![
            ("vams".to_owned(), "ac".to_owned()),
            ("vams".to_owned(), "tran".to_owned()),
        ];
        let native = vec![
            row("ac", Route::Native, Outcome::Refused),
            row("tran", Route::Native, Outcome::Pass),
        ];
        let interp = vec![
            row("tran", Route::Interp, Outcome::Pass),
            row("ac", Route::Interp, Outcome::Refused),
        ];
        let merged = record(&native, &interp, &cells);
        assert_eq!(merged.len(), 4);
        assert_eq!(merged[0].analysis, "ac");
        assert!(coverage(&merged, &cells).is_empty());
        assert!(!coverage(&merged[..3], &cells).is_empty());
        let extra = [
            merged.clone(),
            vec![row("op", Route::Interp, Outcome::Pass)],
        ]
        .concat();
        assert!(!coverage(&extra, &cells).is_empty());
    }
}
