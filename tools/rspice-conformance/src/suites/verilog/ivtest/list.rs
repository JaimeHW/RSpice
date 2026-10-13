//! Reading the ivtest regression lists.
//!
//! Three lists feed the selection: `regress-vlg.list` (1364-2005 tests),
//! `regress-synth.list` (synthesis designs, run here as ordinary simulation)
//! and `regress-vvp.list` (one JSON description per test). `regress-ivl1.list`
//! is read only for the names it overrides. The text format is the one the
//! suite's own `vvp_reg.pl` reads: `#` starts a comment, a trailing `\`
//! continues a line, and the fields are
//!
//! ```text
//! name  type[,arg,...]  directory  [module | gold=f | unordered=f | diff=out:gold[:skip]]
//! ```
//!
//! An argument starting with `+` is a plusarg; the rest are compiler flags. A
//! name is taken from the first list that defines it, and within a list from
//! its first definition.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// The list a test came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceList {
    Vlg,
    Synth,
    Vvp,
}

impl SourceList {
    pub const ALL: [Self; 3] = [Self::Vlg, Self::Synth, Self::Vvp];

    pub const fn token(self) -> &'static str {
        match self {
            Self::Vlg => "vlg",
            Self::Synth => "synth",
            Self::Vvp => "vvp",
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|list| list.token() == token)
    }

    pub const fn file_name(self) -> &'static str {
        match self {
            Self::Vlg => "regress-vlg.list",
            Self::Synth => "regress-synth.list",
            Self::Vvp => "regress-vvp.list",
        }
    }
}

/// How a test's output is judged once it has run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Check {
    /// A line reading `passed` on its own. The JSON-described tests use the
    /// stricter rule of their runner: a line that is exactly `PASSED`.
    PassedLine { strict: bool },
    /// The output against a gold file (relative to the checkout), line by
    /// line, or as sorted lines when `unordered`.
    Gold { path: PathBuf, unordered: bool },
    /// The JSON form: `gold/<stem>-vvp-stdout.gold`, absent meaning empty.
    VvpGold { stem: String },
    /// A file the test writes against a gold file, both read past their
    /// first `skip` lines.
    Diff {
        produced: String,
        gold: PathBuf,
        skip: usize,
    },
}

/// One test as its list describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub list: SourceList,
    /// The list's type token: `normal`, `CE`, `CO`, `RE`, `NI`, `EF`, ...
    pub kind: String,
    /// Compiler flags, in list order.
    pub args: Vec<String>,
    /// `+name[=value]` runtime arguments.
    pub plusargs: Vec<String>,
    /// Icarus runtime (`vvp`) flags before and after the compiled design,
    /// from the JSON list's `vvp-args` and `vvp-args-extended`. RSpice has no
    /// equivalent; only the Icarus self-check passes them.
    pub runtime_flags: (Vec<String>, Vec<String>),
    /// The source file, relative to the checkout.
    pub source: PathBuf,
    /// The module the list names as the root (`-s`).
    pub module: Option<String>,
    pub check: Check,
}

/// The logical lines of a list file: comments and blank lines dropped,
/// continuations joined, inline comments stripped, split on white space.
pub fn list_lines(text: &str) -> Vec<Vec<String>> {
    let mut lines = Vec::new();
    let mut raw = text.lines().map(|line| line.trim_end_matches('\r'));
    while let Some(first) = raw.next() {
        let mut line = first.to_owned();
        while line.ends_with('\\') {
            line.pop();
            match raw.next() {
                Some(next) => line.push_str(next.trim_start()),
                None => break,
            }
        }
        if line.trim_start().starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if let Some(hash) = line.find('#') {
            line.truncate(hash);
        }
        let fields: Vec<String> = line.split_whitespace().map(str::to_owned).collect();
        if fields.len() >= 2 {
            lines.push(fields);
        }
    }
    lines
}

/// The type token and the flag and plusarg lists of a `type[,arg,...]` field.
fn split_type(field: &str) -> (String, Vec<String>, Vec<String>) {
    let mut parts = field.split(',');
    let kind = parts.next().unwrap_or_default().to_owned();
    let (mut args, mut plusargs) = (Vec::new(), Vec::new());
    for part in parts.filter(|part| !part.is_empty()) {
        if part.starts_with('+') {
            plusargs.push(part.to_owned());
        } else {
            args.push(part.to_owned());
        }
    }
    (kind, args, plusargs)
}

/// One `.list` line as a test, or `None` for a version-qualified name
/// (`v13:name`), which no list of the selected generation uses.
fn text_entry(fields: &[String], list: SourceList) -> Option<Entry> {
    let name = fields[0].clone();
    if name.contains(':') {
        return None;
    }
    let (kind, args, plusargs) = split_type(&fields[1]);
    let dir = fields.get(2).cloned().unwrap_or_default();
    let mut module = None;
    let mut check = Check::PassedLine { strict: false };
    // `vvp_reg.pl` reads the fourth field when there are four, and only the
    // fifth when there are five; a module name in a five-field line is
    // ignored there, so it is here.
    let extra = match fields.len() {
        4 => Some(&fields[3]),
        5 => Some(&fields[4]),
        _ => None,
    };
    if let Some(extra) = extra {
        if let Some(gold) = extra.strip_prefix("gold=") {
            check = Check::Gold {
                path: Path::new("gold").join(gold),
                unordered: false,
            };
        } else if let Some(gold) = extra.strip_prefix("unordered=") {
            check = Check::Gold {
                path: Path::new("gold").join(gold),
                unordered: true,
            };
        } else if let Some(diff) = extra.strip_prefix("diff=") {
            let mut parts = diff.split(':');
            let produced = parts.next().unwrap_or_default().to_owned();
            let gold = PathBuf::from(parts.next().unwrap_or_default());
            let skip = parts.next().and_then(|skip| skip.parse().ok()).unwrap_or(0);
            check = Check::Diff {
                produced,
                gold,
                skip,
            };
        } else if fields.len() == 4 {
            module = Some(extra.clone());
        }
    }
    let source = Path::new(&dir).join(format!("{name}.v"));
    Some(Entry {
        name,
        list,
        kind,
        args,
        plusargs,
        runtime_flags: (Vec::new(), Vec::new()),
        source,
        module,
        check,
    })
}

/// The names `regress-ivl1.list` defines: Icarus-specific variants that take
/// precedence over the standard list's definition of the same test.
pub fn ivl1_names(root: &Path) -> Result<BTreeSet<String>, String> {
    let text = read(root, "regress-ivl1.list")?;
    Ok(list_lines(&text)
        .into_iter()
        .map(|fields| fields[0].rsplit(':').next().unwrap_or_default().to_owned())
        .collect())
}

/// Every test of `regress-vlg.list`, then the names `regress-synth.list` adds,
/// then `regress-vvp.list`, each name once.
pub fn read_entries(root: &Path) -> Result<Vec<Entry>, String> {
    let mut seen = BTreeSet::new();
    let mut entries = Vec::new();
    for list in [SourceList::Vlg, SourceList::Synth] {
        for fields in list_lines(&read(root, list.file_name())?) {
            if let Some(entry) = text_entry(&fields, list)
                && seen.insert(entry.name.clone())
            {
                entries.push(entry);
            }
        }
    }
    let vvp = read(root, SourceList::Vvp.file_name())?;
    for line in vvp.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        let (Some(name), Some(json)) = (fields.next(), fields.next()) else {
            continue;
        };
        // The vvp list may name a test twice; the first definition wins, as
        // it does in the text lists.
        if !seen.insert(name.to_owned()) {
            continue;
        }
        entries.push(vvp_entry(root, name, json)?);
    }
    Ok(entries)
}

/// One `vvp_tests/<name>.json` description.
fn vvp_entry(root: &Path, name: &str, json: &str) -> Result<Entry, String> {
    let path = root.join(json);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| format!("{} is not JSON: {error}", path.display()))?;
    let field = |key: &str| value.get(key).and_then(serde_json::Value::as_str);
    let strings = |key: &str| -> Vec<String> {
        value
            .get(key)
            .and_then(serde_json::Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    };
    let (plusargs, args): (Vec<String>, Vec<String>) = strings("iverilog-args")
        .into_iter()
        .partition(|arg| arg.starts_with('+'));
    let (extended_plusargs, after): (Vec<String>, Vec<String>) = strings("vvp-args-extended")
        .into_iter()
        .partition(|arg| arg.starts_with('+'));
    let mut plusargs = plusargs;
    plusargs.extend(extended_plusargs);
    let source = field("source").map_or_else(|| format!("{name}.v"), str::to_owned);
    let check = match field("gold") {
        Some(stem) if !stem.is_empty() => Check::VvpGold {
            stem: stem.to_owned(),
        },
        _ => Check::PassedLine { strict: true },
    };
    Ok(Entry {
        name: name.to_owned(),
        list: SourceList::Vvp,
        kind: field("type").unwrap_or("normal").to_owned(),
        args,
        plusargs,
        runtime_flags: (strings("vvp-args"), after),
        source: Path::new("ivltests").join(source),
        module: None,
        check,
    })
}

fn read(root: &Path, file: &str) -> Result<String, String> {
    let path = root.join(file);
    fs::read(&path)
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .map_err(|error| format!("cannot read {}: {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_continuations_and_short_lines_are_handled_as_the_perl_runner_does() {
        let text = "# header\n\
                    \n\
                    alpha normal ivltests # trailing comment\n\
                    beta CE,-gspecify,+arg=1 \\\n    ivltests gold=beta.gold\n\
                    lonely\n";
        let lines = list_lines(text);
        assert_eq!(
            lines,
            vec![
                vec!["alpha", "normal", "ivltests"],
                vec!["beta", "CE,-gspecify,+arg=1", "ivltests", "gold=beta.gold"],
            ]
        );
    }

    #[test]
    fn the_fourth_field_is_a_check_or_a_module() {
        let entry = |line: &str| {
            text_entry(
                &line
                    .split_whitespace()
                    .map(str::to_owned)
                    .collect::<Vec<_>>(),
                SourceList::Vlg,
            )
            .expect("an unqualified name")
        };
        let gold = entry("t1 normal ivltests gold=t1.gold");
        assert_eq!(
            gold.check,
            Check::Gold {
                path: Path::new("gold").join("t1.gold"),
                unordered: false
            }
        );
        assert_eq!(gold.source, Path::new("ivltests").join("t1.v"));

        let unordered = entry("t2 normal ivltests unordered=u.gold");
        assert!(matches!(
            unordered.check,
            Check::Gold {
                unordered: true,
                ..
            }
        ));

        let diff = entry("t3 normal ivltests diff=work/t3.out:gold/t3.gold:2");
        assert_eq!(
            diff.check,
            Check::Diff {
                produced: "work/t3.out".into(),
                gold: PathBuf::from("gold/t3.gold"),
                skip: 2
            }
        );

        let module = entry("t4 normal ivltests main");
        assert_eq!(module.module.as_deref(), Some("main"));
        assert_eq!(module.check, Check::PassedLine { strict: false });

        let args = entry("t5 normal,-gspecify,+foo,-Tmax ivltests");
        assert_eq!(args.kind, "normal");
        assert_eq!(args.args, vec!["-gspecify", "-Tmax"]);
        assert_eq!(args.plusargs, vec!["+foo"]);

        assert!(text_entry(&["v13:t6".to_owned(), "normal".to_owned()], SourceList::Vlg).is_none());
    }
}
