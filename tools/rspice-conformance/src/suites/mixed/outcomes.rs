//! `outcomes.tsv`: the P0.5 baseline of every design × variant × analysis ×
//! route cell, and the ratchet that holds it.
//!
//! `run` compares each observed outcome with its recorded row and fails on any
//! difference — `regressed` when the cell got worse in the order
//! pass > unverified > refused > fail > wrong, `stale` when it got better
//! without being re-recorded. `run --record` rewrites the rows of the routes it
//! ran from what it observed; the landing manager reviews that diff. The detail
//! column is informational (it carries measured numbers that move in the last
//! digit between machines) and is not compared; the paths it names are written
//! relative to the design ([`normalize_paths`]) so a re-record on another
//! machine or worktree does not churn the file.
//!
//! The interpreter and native routes are separate builds, so two runners may
//! record one design at once. Each read-merge-write holds an exclusive lock on
//! the table itself ([`record_locked`]); a ratchet read holds a shared one.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions, TryLockError};
use std::io::{Read as _, Seek as _, SeekFrom, Write as _};
use std::path::Path;
use std::time::{Duration, Instant};

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

/// How long a recorder waits for another runner's read-merge-write, which
/// takes milliseconds; past this the holder is hung, not slow.
const LOCK_WAIT: Duration = Duration::from_secs(120);
const LOCK_POLL: Duration = Duration::from_millis(20);

/// Take the table's lock, retrying until [`LOCK_WAIT`].
///
/// The lock is the operating system's lock on the open file, not a marker
/// file, so there is no stale lock to detect: the kernel releases it when the
/// holding handle closes, and a killed runner's handles close with its
/// process. A lock that is still held therefore has a live holder.
fn acquire(file: &File, path: &Path, exclusive: bool) -> Result<(), String> {
    let started = Instant::now();
    loop {
        let attempt = if exclusive {
            file.try_lock()
        } else {
            file.try_lock_shared()
        };
        match attempt {
            Ok(()) => return Ok(()),
            Err(TryLockError::WouldBlock) if started.elapsed() < LOCK_WAIT => {
                std::thread::sleep(LOCK_POLL);
            }
            Err(TryLockError::WouldBlock) => {
                return Err(format!(
                    "{}: another runner held the lock for {} s",
                    path.display(),
                    LOCK_WAIT.as_secs()
                ));
            }
            Err(TryLockError::Error(error)) => {
                return Err(format!("{}: lock: {error}", path.display()));
            }
        }
    }
}

/// The recorded table, read under a shared lock so a concurrent recorder's
/// rewrite is never seen half done. A missing table has no rows.
pub fn read_locked(path: &Path) -> Result<Vec<Row>, String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(format!("{}: {error}", path.display())),
    };
    acquire(&file, path, false)?;
    let mut text = String::new();
    file.read_to_string(&mut text)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    parse(&text)
}

/// [`record`] `observed` into the table at `path`: read, merge and rewrite
/// under one exclusive lock, so a recorder of another route running at the
/// same time can neither be overwritten nor overwrite this one.
pub fn record_locked(
    path: &Path,
    observed: &[Row],
    order: &[(String, String)],
) -> Result<(), String> {
    let error = |error: std::io::Error| format!("{}: {error}", path.display());
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .map_err(error)?;
    acquire(&file, path, true)?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(error)?;
    let rendered = render(&record(&parse(&text)?, observed, order));
    if rendered != text {
        // Rewritten in place through the locked handle: replacing the file
        // would leave a waiting recorder locking the old one.
        file.seek(SeekFrom::Start(0)).map_err(error)?;
        file.set_len(0).map_err(error)?;
        file.write_all(rendered.as_bytes()).map_err(error)?;
        file.flush().map_err(error)?;
    }
    Ok(())
}

/// Rewrite the absolute paths in a cell's detail so the table reads the same
/// on every machine and worktree. A path inside `design_dir` is written
/// relative to it, any other relative to `corpus_root`; separators become `/`,
/// `.` and `..` are resolved lexically, and a trailing `:line[:column]` is
/// kept. Windows (`C:\`, `C:/`, `\\?\C:\`) and Unix spellings are both read
/// whatever the host, and Windows components compare case-insensitively.
pub fn normalize_paths(detail: &str, corpus_root: &str, design_dir: &str) -> String {
    let (Some(corpus), Some(design)) = (Lexical::parse(corpus_root), Lexical::parse(design_dir))
    else {
        return detail.to_owned();
    };
    let mut out = String::with_capacity(detail.len());
    let mut index = 0;
    let mut previous = None;
    while let Some(tail) = detail.get(index..).filter(|tail| !tail.is_empty()) {
        if let Some(length) = path_token(tail, previous, &corpus) {
            let token = &tail[..length];
            match rewrite(token, &corpus, &design) {
                Some(relative) => out.push_str(&relative),
                None => out.push_str(token),
            }
            previous = token.chars().last();
            index += length;
            continue;
        }
        let character = tail.chars().next().expect("non-empty");
        out.push(character);
        previous = Some(character);
        index += character.len_utf8();
    }
    out
}

/// An absolute path split into its root and lexically resolved components.
#[derive(Debug, PartialEq)]
struct Lexical {
    /// The upper-case drive letter, or `None` for a Unix root.
    drive: Option<char>,
    parts: Vec<String>,
}

impl Lexical {
    fn parse(path: &str) -> Option<Self> {
        let path = path.strip_prefix(r"\\?\").unwrap_or(path);
        let (drive, rest) = match path.as_bytes() {
            [letter, b':', b'\\' | b'/', ..] if letter.is_ascii_alphabetic() => {
                (Some(letter.to_ascii_uppercase() as char), &path[3..])
            }
            [b'/', ..] => (None, &path[1..]),
            _ => return None,
        };
        let mut parts: Vec<String> = Vec::new();
        for part in rest.split(['\\', '/']) {
            match part {
                "" | "." => {}
                ".." => {
                    parts.pop();
                }
                other => parts.push(other.to_owned()),
            }
        }
        Some(Self { drive, parts })
    }

    fn same(&self, a: &str, b: &str) -> bool {
        if self.drive.is_some() {
            a.eq_ignore_ascii_case(b)
        } else {
            a == b
        }
    }

    /// `target`'s components below `self`, if `self` contains it.
    fn below(&self, target: &Self) -> Option<Vec<String>> {
        (self.drive == target.drive
            && target.parts.len() >= self.parts.len()
            && self
                .parts
                .iter()
                .zip(&target.parts)
                .all(|(a, b)| self.same(a, b)))
        .then(|| target.parts[self.parts.len()..].to_vec())
    }

    /// `target` relative to `self`, climbing with `..` where it must.
    fn relative(&self, target: &Self) -> Option<Vec<String>> {
        if self.drive != target.drive {
            return None;
        }
        let common = self
            .parts
            .iter()
            .zip(&target.parts)
            .take_while(|(a, b)| self.same(a, b))
            .count();
        let mut parts = vec!["..".to_owned(); self.parts.len() - common];
        parts.extend(target.parts[common..].iter().cloned());
        Some(parts)
    }
}

/// Characters that end a path inside a detail.
fn ends_path(character: char) -> bool {
    character.is_whitespace()
        || matches!(
            character,
            '"' | '\'' | ';' | ',' | '|' | '<' | '>' | '(' | ')' | '[' | ']' | '{' | '}' | '*'
        )
}

/// The byte length of the absolute path `text` starts with, if it starts one.
/// A path runs to the first [`ends_path`] character, except that the corpus
/// root's own components are matched first, so a checkout under a directory
/// with a space in its name still reads as one path.
fn path_token(text: &str, previous: Option<char>, corpus: &Lexical) -> Option<usize> {
    let verbatim = if text.starts_with(r"\\?\") { 4 } else { 0 };
    let body = &text[verbatim..];
    let (anchor, drive) = match body.as_bytes() {
        [letter, b':', b'\\' | b'/', ..] if letter.is_ascii_alphabetic() => {
            let boundary = previous.is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
            (boundary || verbatim > 0).then_some((3, true))?
        }
        // A Unix path starts a word, so prose with a slash in it is not one.
        [b'/', next, ..] if verbatim == 0 && *next != b'/' && !ends_path(*next as char) => {
            let boundary =
                previous.is_none_or(|c| c.is_whitespace() || matches!(c, '"' | '\'' | '=' | '('));
            boundary.then_some((1, false))?
        }
        _ => return None,
    };
    let mut start = anchor;
    if drive == corpus.drive.is_some() {
        for part in &corpus.parts {
            let rest = &body[start..];
            let Some(head) = rest
                .get(..part.len())
                .filter(|head| corpus.same(head, part))
            else {
                break;
            };
            match rest[head.len()..].chars().next() {
                Some('\\' | '/') => start += head.len() + 1,
                Some(c) if !ends_path(c) => break,
                _ => {
                    start += head.len();
                    break;
                }
            }
        }
    }
    let end = start + body[start..].find(ends_path).unwrap_or(body.len() - start);
    // A lone `/word` is more likely prose than a path.
    if !drive
        && body[..end]
            .split(['\\', '/'])
            .filter(|part| !part.is_empty())
            .count()
            < 2
    {
        return None;
    }
    Some(verbatim + end)
}

/// One path token, relative to the design or the corpus root.
fn rewrite(token: &str, corpus: &Lexical, design: &Lexical) -> Option<String> {
    let mut end = token.len();
    // A trailing `:line[:column]` is kept as written.
    while let Some(colon) = token[..end].rfind(':') {
        let digits = &token[colon + 1..end];
        if colon < 3 || digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            break;
        }
        end = colon;
    }
    let target = Lexical::parse(&token[..end])?;
    let parts = design.below(&target).or_else(|| corpus.relative(&target))?;
    let path = if parts.is_empty() {
        ".".to_owned()
    } else {
        parts.join("/")
    };
    Some(path + &token[end..])
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

    #[test]
    fn windows_paths_normalize_relative_to_the_design() {
        let corpus = r"C:\work\RSpice\tools\rspice-conformance\..\..\tests\mixed-corpus";
        let design = format!(r"{corpus}\c04-sar8");
        let deck = r"C:\work\RSpice\tests\mixed-corpus\c04-sar8\deck\../net/sar8_full.v";
        let detail = format!(
            "compile_refusal; label=\"{deck}\"; instance=sar8_full.v form=structured; site={deck}"
        );
        assert_eq!(
            normalize_paths(&detail, corpus, &design),
            "compile_refusal; label=\"net/sar8_full.v\"; instance=sar8_full.v form=structured; site=net/sar8_full.v"
        );
        let verbatim = r"\\?\C:\work\RSpice\tests\mixed-corpus\c04-sar8\net\sar8_full.v:17";
        let detail = format!(
            r#"compile_refusal; label="{verbatim}"; instance=c:\WORK\RSpice\tests\mixed-corpus\c04-sar8\deck\..\net\sar8_full.v form=structured; site={verbatim}"#
        );
        assert_eq!(
            normalize_paths(&detail, corpus, &design),
            "compile_refusal; label=\"net/sar8_full.v:17\"; instance=net/sar8_full.v form=structured; site=net/sar8_full.v:17"
        );
        // Outside the design: relative to the corpus root, climbing out of it.
        assert_eq!(
            normalize_paths(
                r"see C:/work/RSpice/tests/mixed-corpus/c05-ldo-trim/net/trim.v:3:9 and C:\work\RSpice\tests\verilog\a.v",
                corpus,
                &design
            ),
            "see c05-ldo-trim/net/trim.v:3:9 and ../verilog/a.v"
        );
        // Another drive cannot be made relative and is left alone.
        assert_eq!(
            normalize_paths(r"D:\tools\x.v", corpus, &design),
            r"D:\tools\x.v"
        );
    }

    #[test]
    fn unix_paths_normalize_relative_to_the_design() {
        let corpus = "/home/ci/RSpice/tools/rspice-conformance/../../tests/mixed-corpus";
        let design = format!("{corpus}/probes/p06c-dd-mixed");
        let detail = "compile_refusal; label=\"/home/ci/RSpice/tools/rspice-conformance/../../tests/mixed-corpus/probes/p06c-dd-mixed/deck/../net/dd.v:4\"; site=/home/ci/RSpice/tests/mixed-corpus/probes/p06c-dd-mixed/net/dd.v";
        assert_eq!(
            normalize_paths(detail, corpus, &design),
            "compile_refusal; label=\"net/dd.v:4\"; site=net/dd.v"
        );
        assert_eq!(
            normalize_paths(
                "include /home/ci/RSpice/tests/mixed-corpus/c01-sigma-delta/net/q.v",
                corpus,
                &design
            ),
            "include c01-sigma-delta/net/q.v"
        );
    }

    #[test]
    fn prose_and_relative_paths_are_left_alone() {
        let corpus = "/r/tests/mixed-corpus";
        let design = "/r/tests/mixed-corpus/c05-ldo-trim";
        for detail in [
            "analysis.mixed_signal.discrete_state; label=\"ac analysis\"; instance=xtrim form=prose",
            "ac (O6) |a−e|=1.0e-3 for 100/(1 + jf/31.83 MHz) and a / b",
            "dnl (O6) std over 10 samples: 6.456155e-2 in [3.600000e-2, 1.360000e-1]",
            "label=\"net/sar8_full.v:17\"; instance=net/sar8_full.v",
            "ratio /x only",
        ] {
            assert_eq!(normalize_paths(detail, corpus, design), detail);
        }
    }

    #[test]
    fn a_corpus_root_with_a_space_still_reads_as_one_path() {
        let corpus = r"C:\Users\Jo Smith\RSpice\tests\mixed-corpus";
        let design = format!(r"{corpus}\c09-por");
        assert_eq!(
            normalize_paths(
                r"site=C:\Users\Jo Smith\RSpice\tests\mixed-corpus\c09-por\net\por.vams; x",
                corpus,
                &design
            ),
            "site=net/por.vams; x"
        );
    }

    fn cells() -> Vec<(String, String)> {
        vec![
            ("vams".to_owned(), "ac".to_owned()),
            ("vams".to_owned(), "tran".to_owned()),
        ]
    }

    #[test]
    fn concurrent_recorders_of_disjoint_routes_both_land() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("outcomes.tsv");
        let cells = cells();
        const ROUNDS: usize = 40;
        std::thread::scope(|scope| {
            for route in [Route::Interp, Route::Native] {
                let (path, cells) = (&path, &cells);
                scope.spawn(move || {
                    for round in 0..ROUNDS {
                        let outcome = if round % 2 == 0 {
                            Outcome::Pass
                        } else {
                            Outcome::Refused
                        };
                        let observed = [row("ac", route, outcome), row("tran", route, outcome)];
                        record_locked(path, &observed, cells).unwrap();
                    }
                });
            }
        });
        let rows = read_locked(&path).unwrap();
        assert!(coverage(&rows, &cells).is_empty(), "{rows:?}");
        // Each route's last write survives the other route's later merges.
        assert!(
            rows.iter().all(|row| row.outcome == Outcome::Refused),
            "{rows:?}"
        );
    }

    const HOLD_ENV: &str = "RSPICE_OUTCOMES_LOCK_HOLD";
    const HELD: &str = "rspice-outcomes-lock-held";

    /// The child half of `a_killed_recorder_does_not_wedge_the_next_one`,
    /// which re-runs this test binary on it; on its own it does nothing.
    #[test]
    fn hold_the_table_lock_until_killed() {
        let Some(path) = std::env::var_os(HOLD_ENV) else {
            return;
        };
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        file.lock().unwrap();
        println!("{HELD}");
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_secs(120));
    }

    #[test]
    fn a_killed_recorder_does_not_wedge_the_next_one() {
        use std::io::BufRead as _;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("outcomes.tsv");
        std::fs::write(&path, render(&[row("ac", Route::Native, Outcome::Pass)])).unwrap();
        let mut holder = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "suites::mixed::outcomes::tests::hold_the_table_lock_until_killed",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(HOLD_ENV, &path)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        // libtest prints the test's name on the same line before its output.
        let stdout = std::io::BufReader::new(holder.stdout.take().unwrap());
        let held = stdout
            .lines()
            .any(|line| line.is_ok_and(|line| line.contains(HELD)));
        if !held {
            let _ = holder.kill();
            panic!("the holder process never took the lock");
        }
        // A live holder blocks a reader.
        let probe = File::open(&path).unwrap();
        assert!(matches!(
            probe.try_lock_shared(),
            Err(TryLockError::WouldBlock)
        ));
        drop(probe);
        holder.kill().unwrap();
        holder.wait().unwrap();
        // Its death released the lock: the next recorder proceeds at once.
        let started = Instant::now();
        record_locked(
            &path,
            &[row("ac", Route::Interp, Outcome::Refused)],
            &cells(),
        )
        .unwrap();
        assert!(started.elapsed() < Duration::from_secs(10));
        assert_eq!(read_locked(&path).unwrap().len(), 2);
    }
}
