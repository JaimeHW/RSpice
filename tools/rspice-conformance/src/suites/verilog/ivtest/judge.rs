//! Judging one run of one test, and combining the two schedules' verdicts.
//!
//! # Pass rules (design P2 section 2.1)
//!
//! - A `passed` test passes on a line reading `passed` on its own, in any
//!   case and with any surrounding white space. A test from the JSON list is
//!   held to its own runner's stricter rule: a line that is exactly `PASSED`.
//! - A `gold` test passes when its normalized output equals its normalized
//!   gold file, line for line (or as sorted lines for an `unordered` gold).
//! - A `ce` test passes only on a **language** error. A construct RSpice
//!   refuses by name is recorded as `refused`, never as a pass, so refusing a
//!   construct cannot pass a compile-error test.
//! - A `co` test passes when it compiles.
//!
//! An internal error, a diagnostic RSpice marks as its own defect rather
//! than a verdict on the source, fails the test whatever its class, on
//! either schedule: it is never the language error a `ce` test expects, and
//! never a refusal or a race.
//!
//! # Normalization (section 2.3)
//!
//! A gold file is an Icarus log: the compiler's own messages, the run's
//! output, and the runtime's own lines. One fixed rule set drops the Icarus
//! text from both sides before comparing, and nothing else:
//!
//! - `^VCD info:` and `^VCD warning:`;
//! - `: $finish called at ` and `: $stop called at ` (with an optional level);
//! - `^<path>:<line>: warning:` and `^<path>:<line>: sorry:`;
//! - `^WARNING: `;
//! - `^ERROR: <path>:<line>: `, a system task's located run-time error, which
//!   RSpice reports as a diagnostic of its own rather than as output;
//! - the continuation lines of a compiler warning or sorry message:
//!   `^<path>:<line>:<white space>: `, each directly after the message or
//!   after another of its continuation lines;
//! - the continuation lines of a located run-time `WARNING:` or `ERROR:`
//!   message: indented with spaces to exactly the width of the message's
//!   `<kind>: <path>:<line>: ` prefix, each directly after the message or
//!   after another of its continuation lines.
//!
//! A file the design wrote (a `diff` test) and its gold first lose their VCD
//! tool-identity blocks, `$date ... $end` and `$version ... $end`, and
//! nothing else.
//!
//! Line ends are compared after `\r\n` becomes `\n`. A gold file that still
//! differs is a failure, never a tolerance.
//!
//! # Racy tests (section 2.4)
//!
//! A test whose pass/fail verdict under the reverse schedule differs from its
//! verdict under source order is `racy`: its result depends on an order IEEE
//! 1364-2005 leaves open, and it is excluded from the exact gate.

use serde::{Deserialize, Serialize};

use super::select::Class;

/// A file the design wrote, as its `$fdisplay`-family records spell it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducedFile {
    pub path: String,
    pub text: String,
}

/// What one schedule's run of a test produced.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RunEnd {
    /// The run ended (`$finish`, `$stop`, no event left, or the tick
    /// ceiling), with its standard output and the files it wrote.
    Completed {
        end: String,
        stdout: String,
        files: Vec<ProducedFile>,
    },
    /// The front end rejected the source with a language error.
    CompileError { message: String },
    /// RSpice refused a construct or option by name.
    Refused { token: String, message: String },
    /// The design compiled, and has nothing to run.
    NoContent { message: String },
    /// The design compiled and then failed while running.
    RuntimeError { message: String },
    /// RSpice reported an internal error, compiling or running: a defect of
    /// RSpice's, with the marker taken out of the diagnostic.
    InternalError { message: String },
    /// The run outlived its budget and was killed.
    Timeout { seconds: f64 },
    /// The run died without reporting.
    Crash { message: String },
}

impl RunEnd {
    /// Whether the source got past the front end.
    pub fn compiled(&self) -> bool {
        matches!(
            self,
            Self::Completed { .. } | Self::NoContent { .. } | Self::RuntimeError { .. }
        )
    }
}

/// What a run is judged against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expectation {
    /// No gold file: a `passed` line, case-insensitive unless `strict`.
    PassedLine { strict: bool },
    /// The run's standard output against a gold file's text.
    Gold { text: String, unordered: bool },
    /// A file the run wrote against a gold file's text, both read past their
    /// first `skip` lines.
    Diff {
        produced: String,
        gold: String,
        skip: usize,
    },
    /// Nothing to compare: compile-only and compile-error tests.
    None,
}

/// One run's verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Pass(String),
    Fail(String),
    Refused {
        token: String,
        detail: String,
    },
    /// A failure that is RSpice's internal error: it fails the test whatever
    /// the other schedule's verdict.
    Internal(String),
}

impl Verdict {
    fn passed(&self) -> bool {
        matches!(self, Self::Pass(_))
    }
}

/// A test's recorded outcome (the manifest's outcome column).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Outcome {
    Pass,
    Fail,
    Refused(String),
    Racy,
    Excluded(String),
    /// A failing run the manifest accepts: the test expects what the cited
    /// clause does not give, and RSpice follows the clause (design §2.5). The
    /// clause is the row's detail; only the manifest marks a row so, and a
    /// run that fails such a row keeps the mark.
    Deviation,
}

impl Outcome {
    pub fn token(&self) -> String {
        match self {
            Self::Pass => "pass".to_owned(),
            Self::Fail => "fail".to_owned(),
            Self::Refused(token) => format!("refused:{token}"),
            Self::Racy => "racy".to_owned(),
            Self::Excluded(reason) => format!("excluded:{reason}"),
            Self::Deviation => "deviation".to_owned(),
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "pass" => Some(Self::Pass),
            "fail" => Some(Self::Fail),
            "racy" => Some(Self::Racy),
            "deviation" => Some(Self::Deviation),
            _ => {
                if let Some(refused) = token.strip_prefix("refused:") {
                    Some(Self::Refused(refused.to_owned()))
                } else {
                    token
                        .strip_prefix("excluded:")
                        .map(|reason| Self::Excluded(reason.to_owned()))
                }
            }
        }
    }

    /// The ratchet order pass > deviation > racy > refused > fail; an
    /// exclusion is the selection's, not the run's, and ranks below
    /// everything.
    pub const fn rank(&self) -> u8 {
        match self {
            Self::Pass => 5,
            Self::Deviation => 4,
            Self::Racy => 3,
            Self::Refused(_) => 2,
            Self::Fail => 1,
            Self::Excluded(_) => 0,
        }
    }

    /// The outcome's class for counting: `pass`, `fail`, `refused`, `racy`,
    /// `excluded` or `deviation`.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Refused(_) => "refused",
            Self::Racy => "racy",
            Self::Excluded(_) => "excluded",
            Self::Deviation => "deviation",
        }
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.token())
    }
}

/// Whether a log line is Icarus's own text rather than the design's output,
/// by the rules that read one line alone. A continuation line is Icarus's
/// only after a compiler message, which [`normalize`] tracks.
pub fn is_icarus_text(line: &str) -> bool {
    if line.starts_with("VCD info:") || line.starts_with("VCD warning:") {
        return true;
    }
    if line.starts_with("WARNING: ") || runtime_message_indent(line).is_some() {
        return true;
    }
    if finish_report(line) {
        return true;
    }
    compiler_message(line)
}

/// For a located run-time message, `^WARNING: <path>:<line>: ` or
/// `^ERROR: <path>:<line>: `, the width of that prefix: Icarus indents the
/// message's further lines (`"%*s "` of the prefix) to it. An `ERROR:` line
/// is dropped only in this located form, the form of a system task's own
/// run-time error, which RSpice reports as a diagnostic of its own.
fn runtime_message_indent(line: &str) -> Option<usize> {
    let rest = line
        .strip_prefix("WARNING: ")
        .or_else(|| line.strip_prefix("ERROR: "))?;
    let message = after_location(rest)?;
    message
        .strip_prefix(' ')
        .map(|text| line.len() - text.len())
}

/// A further line of a run-time message: exactly `indent` spaces, then text.
fn runtime_continuation(line: &str, indent: usize) -> bool {
    line.len() > indent
        && line.as_bytes()[..indent].iter().all(|byte| *byte == b' ')
        && !line.as_bytes()[indent].is_ascii_whitespace()
}

/// `<anything>: $finish called at ` or `: $stop(<digit>) called at `.
fn finish_report(line: &str) -> bool {
    ["$finish", "$stop"].iter().any(|task| {
        line.match_indices(&format!(": {task}"))
            .any(|(at, matched)| {
                let rest = &line[at + matched.len()..];
                let rest = match rest.as_bytes() {
                    [b'(', digit, b')', ..] if digit.is_ascii_digit() => &rest[3..],
                    _ => rest,
                };
                rest.starts_with(" called at ")
            })
    })
}

/// The text after a line's `<path>:<line>:` location, or `None` when the line
/// does not start with one. The location is the first white-space-free run,
/// and must end in `:<digits>:`.
fn after_location(line: &str) -> Option<&str> {
    let space = line.find(char::is_whitespace).unwrap_or(line.len());
    let (path, digits) = line[..space].strip_suffix(':')?.rsplit_once(':')?;
    (!path.is_empty() && !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| &line[space..])
}

/// `^\S+:\d+: (warning|sorry):`.
fn compiler_message(line: &str) -> bool {
    after_location(line)
        .is_some_and(|rest| rest.starts_with(" warning:") || rest.starts_with(" sorry:"))
}

/// `^\S+:\d+:\s+:`: a further line of a compiler message, white space where
/// the message kind was.
fn continuation(line: &str) -> bool {
    after_location(line).is_some_and(|rest| {
        let message = rest.trim_start();
        message.len() < rest.len() && message.starts_with(':')
    })
}

/// The lines of a log with Icarus's own text dropped, each with its line end
/// (`\n`, or none for an unterminated last line), `\r\n` read as `\n`.
pub fn normalize(text: &str) -> Vec<String> {
    // Whether the previous line was a dropped compiler message or one of its
    // continuation lines.
    let mut in_message = false;
    // The indent of a dropped run-time message's further lines, while the
    // previous line was that message or one of them.
    let mut runtime_indent: Option<usize> = None;
    text.split_inclusive('\n')
        .map(|line| match line.strip_suffix("\r\n") {
            Some(body) => format!("{body}\n"),
            None => line.to_owned(),
        })
        .filter(|line| {
            let line = line.trim_end_matches('\n');
            let compiler_continued = in_message && continuation(line);
            let runtime_continued = !compiler_continued
                && runtime_indent.is_some_and(|indent| runtime_continuation(line, indent));
            in_message = compiler_continued || compiler_message(line);
            runtime_indent = match runtime_message_indent(line) {
                Some(indent) => Some(indent),
                None if runtime_continued => runtime_indent,
                None => None,
            };
            !(compiler_continued || runtime_continued || is_icarus_text(line))
        })
        .collect()
}

/// A written file with its VCD tool-identity blocks dropped: `$date ... $end`
/// and `$version ... $end` (IEEE 1364-2005 section 18.2.3.1), which name the
/// day the dump was made and the tool that made it, never the design. A
/// written file and its gold both lose them before they are compared;
/// nothing else in either is touched.
pub fn drop_vcd_identity(text: &str) -> String {
    let opens = |line: &str| {
        let line = line.trim_start();
        ["$date", "$version"].iter().any(|keyword| {
            line.strip_prefix(keyword)
                .is_some_and(|rest| rest.is_empty() || rest.starts_with(char::is_whitespace))
        })
    };
    let mut in_block = false;
    text.split_inclusive('\n')
        .filter(|line| {
            if !in_block && !opens(line) {
                return true;
            }
            // The block's last line is the one that holds its `$end`, which
            // may be the line that opened it.
            let body = if in_block {
                line.trim_start()
            } else {
                line.trim_start()
                    .trim_start_matches("$version")
                    .trim_start_matches("$date")
            };
            in_block = !body.contains("$end");
            false
        })
        .collect()
}

/// Whether the output has a line that passes the self-checking rule.
pub fn has_passed_line(stdout: &str, strict: bool) -> bool {
    stdout
        .lines()
        .map(|line| line.trim_end_matches('\r'))
        .any(|line| {
            if strict {
                line == "PASSED"
            } else {
                line.trim().eq_ignore_ascii_case("passed")
            }
        })
}

/// Compare a run's output with a gold text after normalization; the first
/// differing line (one-based, of the normalized gold) on a mismatch.
pub fn compare_gold(output: &str, gold: &str, unordered: bool, skip: usize) -> Result<(), String> {
    let mut ours = normalize(output);
    let mut theirs = normalize(gold);
    ours.drain(..skip.min(ours.len()));
    theirs.drain(..skip.min(theirs.len()));
    if unordered {
        ours.sort();
        theirs.sort();
    }
    if ours == theirs {
        return Ok(());
    }
    let first = ours
        .iter()
        .zip(&theirs)
        .position(|(a, b)| a != b)
        .unwrap_or(ours.len().min(theirs.len()));
    Err(format!(
        "differs from the gold file at line {} ({} lines against {})",
        first + 1 + skip,
        ours.len() + skip,
        theirs.len() + skip
    ))
}

/// Judge one run.
pub fn judge(class: Class, expectation: &Expectation, end: &RunEnd) -> Verdict {
    if let RunEnd::InternalError { message } = end {
        return Verdict::Internal(format!("internal error: {}", clip(message)));
    }
    if let RunEnd::Refused { token, message } = end {
        return Verdict::Refused {
            token: token.clone(),
            detail: message.clone(),
        };
    }
    match class {
        Class::CompileError => match end {
            RunEnd::CompileError { message } => {
                Verdict::Pass(format!("language error: {}", clip(message)))
            }
            other => Verdict::Fail(format!("compiled without error; {}", describe(other))),
        },
        Class::CompileOnly => match end {
            RunEnd::CompileError { message } => {
                Verdict::Fail(format!("compile error: {}", clip(message)))
            }
            RunEnd::Timeout { .. } | RunEnd::Crash { .. } => Verdict::Fail(describe(end)),
            _ => Verdict::Pass("compiled".to_owned()),
        },
        Class::RuntimeError => match end {
            RunEnd::RuntimeError { message } => {
                Verdict::Pass(format!("run-time error: {}", clip(message)))
            }
            other => Verdict::Fail(format!("no run-time error; {}", describe(other))),
        },
        Class::Passed | Class::Gold => {
            let RunEnd::Completed { stdout, files, .. } = end else {
                return Verdict::Fail(describe(end));
            };
            match expectation {
                Expectation::PassedLine { strict } => {
                    if has_passed_line(stdout, *strict) {
                        Verdict::Pass("PASSED".to_owned())
                    } else {
                        Verdict::Fail(format!(
                            "no PASSED line ({} output lines)",
                            stdout.lines().count()
                        ))
                    }
                }
                Expectation::Gold { text, unordered } => {
                    match compare_gold(stdout, text, *unordered, 0) {
                        Ok(()) => Verdict::Pass("matches the gold file".to_owned()),
                        Err(detail) => Verdict::Fail(detail),
                    }
                }
                Expectation::Diff {
                    produced,
                    gold,
                    skip,
                } => {
                    let written: String = files
                        .iter()
                        .filter(|file| same_path(&file.path, produced))
                        .map(|file| file.text.as_str())
                        .collect();
                    if !files.iter().any(|file| same_path(&file.path, produced)) {
                        return Verdict::Fail(format!("did not write {produced}"));
                    }
                    let (written, gold) = (drop_vcd_identity(&written), drop_vcd_identity(gold));
                    match compare_gold(&written, &gold, false, *skip) {
                        Ok(()) => Verdict::Pass(format!("{produced} matches the gold file")),
                        Err(detail) => Verdict::Fail(format!("{produced} {detail}")),
                    }
                }
                Expectation::None => Verdict::Pass("ran".to_owned()),
            }
        }
    }
}

fn same_path(written: &str, expected: &str) -> bool {
    let canonical = |path: &str| path.replace('\\', "/").trim_start_matches("./").to_owned();
    canonical(written) == canonical(expected)
}

/// The outcome of a test from its source-order verdict and, for a test that
/// ran, its reverse-schedule verdict.
pub fn combine(source: Verdict, reverse: Option<Verdict>) -> (Outcome, String) {
    match (source, reverse) {
        (Verdict::Internal(detail), _) | (_, Some(Verdict::Internal(detail))) => {
            (Outcome::Fail, detail)
        }
        (Verdict::Refused { token, detail }, _) => (Outcome::Refused(token), detail),
        (source, Some(reverse)) if source.passed() != reverse.passed() => {
            let (passed, failed) = if source.passed() {
                ("source order", reverse)
            } else {
                ("reverse order", source)
            };
            let why = match failed {
                Verdict::Pass(detail) | Verdict::Fail(detail) | Verdict::Internal(detail) => detail,
                Verdict::Refused { detail, .. } => detail,
            };
            (Outcome::Racy, format!("passes in {passed} only; {why}"))
        }
        (Verdict::Pass(detail), _) => (Outcome::Pass, detail),
        (Verdict::Fail(detail), _) => (Outcome::Fail, detail),
    }
}

/// The marker RSpice's compiler and runtime put on an error that is their own
/// defect rather than a verdict on the source (the `Internal` kinds of the
/// code generator's and the XSPICE layer's errors).
const INTERNAL_ERROR: &str = "Internal error: ";

/// A diagnostic that reports an internal error, with the marker taken out.
fn internal_error(diagnostic: &str) -> Option<String> {
    let (before, after) = diagnostic.split_once(INTERNAL_ERROR)?;
    Some(format!("{before}{after}"))
}

/// Classify a run-time failure: RSpice's internal error, or the run's own.
pub fn run_failure(message: String) -> RunEnd {
    match message.lines().find_map(internal_error) {
        Some(internal) => RunEnd::InternalError { message: internal },
        None => RunEnd::RuntimeError { message },
    }
}

/// Classify a front-end failure: RSpice's internal error, a construct the
/// compiler refuses by name, or a language error.
///
/// The testbench entry point reports a compile failure as the compiler's
/// rendered diagnostics, one per line when there are several. An internal
/// error among them makes the whole failure one: it is RSpice's defect, and
/// says nothing about whether the source is legal. Otherwise a diagnostic is
/// a refusal when its message is one of the compiler's refusal forms:
/// `Unsupported <context>: ...` (the `VA-*-UNSUPPORTED-*` codes), `... not yet
/// supported: ...`, or `... has no executable form ...`. One refusal among
/// several diagnostics makes the whole failure a refusal: a compile-error
/// test must not pass on a source RSpice could not read in full.
pub fn compile_failure(message: &str) -> RunEnd {
    let diagnostics = message
        .lines()
        .map(|line| line.trim().trim_start_matches("- "));
    if let Some(internal) = diagnostics.clone().find_map(internal_error) {
        return RunEnd::InternalError { message: internal };
    }
    for line in diagnostics.clone() {
        if let Some(token) = refusal_token(line) {
            return RunEnd::Refused {
                token,
                message: clip(line),
            };
        }
    }
    // A collection's first line only counts its errors; the first error is
    // the useful summary.
    let first = diagnostics
        .clone()
        .find(|line| !line.is_empty() && !line.starts_with("Compilation failed with"))
        .unwrap_or(message);
    RunEnd::CompileError {
        message: first.to_owned(),
    }
}

/// The construct a refusal diagnostic names, as a manifest token.
fn refusal_token(line: &str) -> Option<String> {
    // The phase prefix: `Parse error at offset N: `, `Semantic error at
    // offset N: `, `Code generation error: `.
    let message = [
        "Parse error",
        "Semantic error",
        "Lexer error",
        "Code generation error",
    ]
    .iter()
    .find_map(|phase| line.strip_prefix(phase))
    .and_then(|rest| rest.split_once(": ").map(|(_, message)| message))
    .unwrap_or(line);
    // A statement or item the parser stops on names its first token: a
    // keyword is the construct, anything else is the user's own name.
    if let Some((_, named)) = message.split_once("construct not yet supported: ") {
        return Some(construct(named).unwrap_or_else(|| "statement".to_owned()));
    }
    if let Some(rest) = message.strip_prefix("Verilog-AMS digital construct `")
        && message.contains("has no executable form")
    {
        return Some(construct(&format!("`{rest}")).unwrap_or_else(|| "statement".to_owned()));
    }
    if let Some(rest) = message.strip_prefix("Unsupported construct '") {
        return Some(token(rest.split('\'').next().unwrap_or("construct")));
    }
    let rest = message.strip_prefix("Unsupported ")?;
    let (context, found) = rest.split_once(": ").unwrap_or((rest, ""));
    if context == "feature" {
        // `Unsupported feature: <phrase>: <detail>`, or a sentence.
        return Some(construct(found).unwrap_or_else(|| {
            found
                .split_once(':')
                .map(|(phrase, _)| phrase)
                .filter(|phrase| !phrase.contains(['`', '\'']) && words(phrase) <= 4)
                .map_or_else(|| "feature".to_owned(), token)
        }));
    }
    // `Unsupported <context>: <found>`: a construct quoted in either, then a
    // keyword leading `found`, then the context itself.
    if let Some(named) = construct(context).or_else(|| construct(found)) {
        return Some(named);
    }
    if let Some(word) = found
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .next()
        .filter(|word| VERILOG_KEYWORDS.contains(word))
    {
        return Some(token(word));
    }
    let context: String = context.split('`').step_by(2).collect();
    let context: Vec<&str> = context.split_whitespace().take(4).collect();
    Some(token(&context.join("-")))
}

/// The first quoted construct name of a message (`` `fork` ``, `` `$dumpvars` ``).
fn construct(text: &str) -> Option<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .find(|quoted| is_construct_name(quoted))
        .map(token)
}

fn words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Whether a quoted name is a construct (a keyword, a system task, or a
/// keyword phrase) rather than the user's own identifier.
fn is_construct_name(name: &str) -> bool {
    let mut words = name
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .filter(|word| !word.is_empty())
        .peekable();
    if name.starts_with('$') {
        return words.peek().is_some();
    }
    words.peek().is_some() && words.all(|word| VERILOG_KEYWORDS.contains(&word))
}

/// A manifest-safe token: no white space, tabs or separators.
fn token(text: &str) -> String {
    let cleaned: String = text
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || "_$.-+:/=<>".contains(c) {
                c
            } else {
                '-'
            }
        })
        .collect();
    let cleaned = cleaned.trim_matches('-');
    if cleaned.is_empty() {
        "construct".to_owned()
    } else {
        cleaned.to_ascii_lowercase()
    }
}

/// IEEE 1364-2005 Annex B.
const VERILOG_KEYWORDS: &[&str] = &[
    "always",
    "and",
    "assign",
    "automatic",
    "begin",
    "buf",
    "bufif0",
    "bufif1",
    "case",
    "casex",
    "casez",
    "cell",
    "cmos",
    "config",
    "deassign",
    "default",
    "defparam",
    "design",
    "disable",
    "edge",
    "else",
    "end",
    "endcase",
    "endconfig",
    "endfunction",
    "endgenerate",
    "endmodule",
    "endprimitive",
    "endspecify",
    "endtable",
    "endtask",
    "event",
    "for",
    "force",
    "forever",
    "fork",
    "function",
    "generate",
    "genvar",
    "highz0",
    "highz1",
    "if",
    "ifnone",
    "incdir",
    "include",
    "initial",
    "inout",
    "input",
    "instance",
    "integer",
    "join",
    "large",
    "liblist",
    "library",
    "localparam",
    "macromodule",
    "medium",
    "module",
    "nand",
    "negedge",
    "nmos",
    "nor",
    "noshowcancelled",
    "not",
    "notif0",
    "notif1",
    "or",
    "output",
    "parameter",
    "pmos",
    "posedge",
    "primitive",
    "pull0",
    "pull1",
    "pulldown",
    "pullup",
    "pulsestyle_onevent",
    "pulsestyle_ondetect",
    "rcmos",
    "real",
    "realtime",
    "reg",
    "release",
    "repeat",
    "rnmos",
    "rpmos",
    "rtran",
    "rtranif0",
    "rtranif1",
    "scalared",
    "showcancelled",
    "signed",
    "small",
    "specify",
    "specparam",
    "strong0",
    "strong1",
    "supply0",
    "supply1",
    "table",
    "task",
    "time",
    "tran",
    "tranif0",
    "tranif1",
    "tri",
    "tri0",
    "tri1",
    "triand",
    "trior",
    "trireg",
    "unsigned",
    "use",
    "uwire",
    "vectored",
    "wait",
    "wand",
    "weak0",
    "weak1",
    "while",
    "wire",
    "wor",
    "xnor",
    "xor",
];

/// A one-line account of a run that did not complete.
fn describe(end: &RunEnd) -> String {
    match end {
        RunEnd::Completed { end, .. } => format!("ran to {end}"),
        RunEnd::CompileError { message } => format!("compile error: {}", clip(message)),
        RunEnd::Refused { token, .. } => format!("refused {token}"),
        RunEnd::NoContent { message } => format!("nothing to run: {}", clip(message)),
        RunEnd::RuntimeError { message } => format!("run-time error: {}", clip(message)),
        RunEnd::InternalError { message } => format!("internal error: {}", clip(message)),
        RunEnd::Timeout { seconds } => format!("timeout after {seconds:.0} s"),
        RunEnd::Crash { message } => format!("crash: {}", clip(message)),
    }
}

/// The first line of a diagnostic, without its source offset, bounded, and
/// free of tabs so it fits a manifest column.
pub fn clip(message: &str) -> String {
    const LIMIT: usize = 160;
    let line = message
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("");
    let mut line = strip_offsets(line.trim()).replace('\t', " ");
    if line.chars().count() > LIMIT {
        line = line.chars().take(LIMIT).collect::<String>() + "...";
    }
    line
}

/// `... at offset 1234: ...` loses its number, which moves with every edit to
/// an unrelated part of the source.
fn strip_offsets(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(at) = rest.find(" at offset ") {
        out.push_str(&rest[..at]);
        let tail = &rest[at + " at offset ".len()..];
        let digits = tail.bytes().take_while(u8::is_ascii_digit).count();
        rest = &tail[digits..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn completed(stdout: &str) -> RunEnd {
        RunEnd::Completed {
            end: "finished".to_owned(),
            stdout: stdout.to_owned(),
            files: Vec::new(),
        }
    }

    fn passed_line(strict: bool) -> Expectation {
        Expectation::PassedLine { strict }
    }

    #[test]
    fn passed_is_a_line_of_its_own_in_any_case() {
        for stdout in ["PASSED\n", "  passed  \n", "a\nPassed\nb\n", "PASSED"] {
            assert!(has_passed_line(stdout, false), "{stdout:?}");
        }
        for stdout in ["", "PASSED!\n", "test PASSED\n", "FAILED\n", "not passed\n"] {
            assert!(!has_passed_line(stdout, false), "{stdout:?}");
        }
        // The JSON tests' runner takes only the exact spelling.
        assert!(has_passed_line("PASSED\r\n", true));
        assert!(!has_passed_line("passed\n", true));
        assert!(!has_passed_line(" PASSED\n", true));
    }

    #[test]
    fn a_passed_test_needs_its_line_and_a_completed_run() {
        assert_eq!(
            judge(
                Class::Passed,
                &passed_line(false),
                &completed("x = 1\nPASSED\n")
            ),
            Verdict::Pass("PASSED".to_owned())
        );
        assert!(matches!(
            judge(Class::Passed, &passed_line(false), &completed("FAILED\n")),
            Verdict::Fail(_)
        ));
        assert!(matches!(
            judge(
                Class::Passed,
                &passed_line(false),
                &RunEnd::RuntimeError {
                    message: "process top.p could not run".to_owned()
                }
            ),
            Verdict::Fail(detail) if detail.starts_with("run-time error")
        ));
        assert!(matches!(
            judge(
                Class::Passed,
                &passed_line(false),
                &RunEnd::Timeout { seconds: 20.0 }
            ),
            Verdict::Fail(detail) if detail.starts_with("timeout")
        ));
    }

    #[test]
    fn normalization_drops_exactly_the_icarus_lines() {
        for line in [
            "VCD info: dumpfile dump.vcd opened for output.",
            "VCD warning: $dumpvars: Unsupported argument type (vpiPackage)",
            "./ivltests/t.v:12: $finish called at 100 (1s)",
            "ivltests/t.v:3: $stop(1) called at 5 (1ns)",
            "./ivltests/t.v:7: warning: Port 1 (a) of m expects 2 bits, got 1.",
            "ivltests/t.v:9: sorry: constant selects are not supported.",
            "WARNING: ivltests/t.v:4: $readmemh: Standard inconsistency",
            "ERROR: ./ivltests/t.v:59: $readmemb: Start address -1 is out of bounds",
        ] {
            assert!(is_icarus_text(line), "{line}");
        }
        for line in [
            "PASSED",
            "ERROR: not located",
            "ERROR: t.v:x: not a line number",
            "VCD infos",
            "$finish called at 100",
            "t.v:12: error: something",
            "warning: not located",
            "a:b: warning: no line number",
            "WARNING:no space",
            "time 10: a = 1",
        ] {
            assert!(!is_icarus_text(line), "{line}");
        }
    }

    #[test]
    fn a_located_run_time_error_and_its_indented_lines_are_icarus_text() {
        let kept = |text: &str| normalize(text).concat();
        // The lines of ivtest's readmem-error gold: a located `ERROR:`, and
        // a `WARNING:` whose second line is indented to its prefix.
        let indent = |prefix: &str| " ".repeat(prefix.len());
        let (two_digit, three_digit) = (
            "WARNING: ./ivltests/r.v:33: ",
            "WARNING: ./ivltests/r.v:138: ",
        );
        let gold = format!(
            "ERROR: ./ivltests/r.v:59: $readmemb: Start address -1 is out of bounds!\n\
             {two_digit}$readmemb's file name argument contains non-printable characters.\n\
             {}\"r.tx\\002\"\n\
             {three_digit}name.\n{}\"two\"\n{}\"and a third\"\n\
             out 1\n",
            indent(two_digit),
            indent(three_digit),
            indent(three_digit),
        );
        assert_eq!(kept(&gold), "out 1\n");
        // An unlocated `ERROR:` is the design's own text, as is an indented
        // line whose indent is not the prefix's, or that follows no message.
        let located = "WARNING: ./a.v:3: ";
        for text in [
            "ERROR: not located\n".to_owned(),
            "ERROR:./a.v:3: no space\n".to_owned(),
            format!("{located}w.\n{}short\n", " ".repeat(located.len() - 1)),
            format!("{located}w.\n{}deep\n", " ".repeat(located.len() + 1)),
            format!("out\n{}\"x\"\n", indent(located)),
        ] {
            let expected: String = text
                .split_inclusive('\n')
                .filter(|line| !line.starts_with("WARNING: "))
                .collect();
            assert_eq!(kept(&text), expected, "{text:?}");
        }
        // A located error takes its own indented lines, and only those.
        let error = "ERROR: a.v:1: ";
        assert_eq!(kept(&format!("{error}e.\n{}\"gone\"\n", indent(error))), "");
        let shallow = format!("{}kept\n", " ".repeat(error.len() - 2));
        assert_eq!(kept(&format!("{error}e.\n{shallow}")), shallow);
    }

    #[test]
    fn a_written_file_loses_exactly_its_vcd_identity_blocks() {
        let icarus = "$date\n\tSun May 15 18:09:25 2022\n$end\n$version\n\tIcarus Verilog\n$end\n\
                      $timescale\n\t1s\n$end\n$comment Show the parameter values. $end\n";
        assert_eq!(
            drop_vcd_identity(icarus),
            "$timescale\n\t1s\n$end\n$comment Show the parameter values. $end\n"
        );
        // One-line blocks, and keywords that only begin a longer word.
        assert_eq!(
            drop_vcd_identity("$date today $end\n$version RSpice $end\n$dates\n$versions $end\n"),
            "$dates\n$versions $end\n"
        );
        let rspice = "$version\n  RSpice\n$end\n$timescale 1s $end\n";
        assert_eq!(drop_vcd_identity(rspice), "$timescale 1s $end\n");
        // A diff test compares what is left.
        let diff = Expectation::Diff {
            produced: "work/d.vcd".to_owned(),
            gold: icarus.to_owned(),
            skip: 0,
        };
        let wrote = RunEnd::Completed {
            end: "finish".to_owned(),
            stdout: String::new(),
            files: vec![ProducedFile {
                path: "work/d.vcd".to_owned(),
                text: "$date\n  Oct 13\n$end\n$version\n  RSpice\n$end\n$timescale\n\t1s\n$end\n\
                       $comment Show the parameter values. $end\n"
                    .to_owned(),
            }],
        };
        assert!(matches!(
            judge(Class::Gold, &diff, &wrote),
            Verdict::Pass(_)
        ));
    }

    #[test]
    fn a_compiler_message_takes_its_continuation_lines_with_it() {
        let kept = |text: &str| normalize(text).concat();
        // After a warning or a sorry message, each continuation line goes,
        // however many there are and whatever location they name.
        assert_eq!(
            kept(
                "src/a.v:4: warning: first line of a note.\n\
                 src/a.v:4:      : its second line.\n\
                 src/a.v:9:   : a third, pointing elsewhere.\n\
                 out 1\n"
            ),
            "out 1\n"
        );
        assert_eq!(
            kept("a.v:2: sorry: not handled.\r\na.v:2:  : why.\r\nout 2\r\n"),
            "out 2\n"
        );
        // Two messages in a row: the second ends the first's continuation and
        // starts its own.
        assert_eq!(
            kept("a.v:1: warning: one.\na.v:1:  : more.\na.v:3: warning: two.\na.v:3:  : more.\n"),
            ""
        );
        // A continuation-shaped line is the design's own output unless a
        // dropped message, or another continuation of one, comes directly
        // before it.
        let design = "a.v:7:   : printed by the design\n";
        for (text, expected) in [
            (design.to_owned(), design.to_owned()),
            (format!("out 3\n{design}"), format!("out 3\n{design}")),
            (
                format!("a.v:4: warning: w.\nout 4\n{design}"),
                format!("out 4\n{design}"),
            ),
            // An error is not one of the dropped messages, so neither is
            // what follows it.
            (
                format!("a.v:4: error: e.\n{design}"),
                format!("a.v:4: error: e.\n{design}"),
            ),
        ] {
            assert_eq!(kept(&text), expected, "{text:?}");
        }
        // The shape itself: location, white space, then a colon.
        for line in ["a.v:4:  : x", "./d/a.v:12:\t: x", "a.v:4: :"] {
            assert!(continuation(line), "{line}");
        }
        for line in ["a.v:4:: x", "a.v:4: x", "a.v:x:  : x", ":4:  : x", "  : x"] {
            assert!(!continuation(line), "{line}");
        }
        // A gold file with a message and its continuation equals an output
        // without them.
        assert_eq!(
            compare_gold(
                "x = 1\n",
                "./t.v:3: warning: w.\n./t.v:3:        : more.\nx = 1\n",
                false,
                0
            ),
            Ok(())
        );
    }

    #[test]
    fn gold_comparison_is_exact_after_normalization() {
        let gold = "./ivltests/t.v:3: warning: implicit wire\r\nx = 1\r\ny = 2\r\n\
                    ./ivltests/t.v:9: $finish called at 20 (1s)\r\n";
        assert_eq!(compare_gold("x = 1\ny = 2\n", gold, false, 0), Ok(()));
        // A different value, a missing line, an extra line, a missing final
        // newline: each is a failure.
        for output in [
            "x = 1\ny = 3\n",
            "x = 1\n",
            "x = 1\ny = 2\nz\n",
            "x = 1\ny = 2",
        ] {
            assert!(compare_gold(output, gold, false, 0).is_err(), "{output:?}");
        }
        let error = compare_gold("x = 1\ny = 3\n", gold, false, 0).unwrap_err();
        assert!(error.contains("line 2"), "{error}");
        // Unordered golds compare sorted lines.
        assert_eq!(compare_gold("b\na\n", "a\nb\n", true, 0), Ok(()));
        assert!(compare_gold("b\na\n", "a\nb\n", false, 0).is_err());
        // A skip drops the leading lines of both.
        assert_eq!(compare_gold("date 1\nv\n", "date 2\nv\n", false, 1), Ok(()));
    }

    #[test]
    fn a_gold_test_compares_its_output_and_a_diff_test_its_file() {
        let gold = Expectation::Gold {
            text: "a\n".to_owned(),
            unordered: false,
        };
        assert!(judge(Class::Gold, &gold, &completed("a\n")).passed());
        assert!(!judge(Class::Gold, &gold, &completed("b\n")).passed());

        let diff = Expectation::Diff {
            produced: "work/out.txt".to_owned(),
            gold: "header\nrow\n".to_owned(),
            skip: 1,
        };
        let wrote = |path: &str, text: &str| RunEnd::Completed {
            end: "finished".to_owned(),
            stdout: String::new(),
            files: vec![ProducedFile {
                path: path.to_owned(),
                text: text.to_owned(),
            }],
        };
        assert!(judge(Class::Gold, &diff, &wrote("./work/out.txt", "other\nrow\n")).passed());
        assert!(!judge(Class::Gold, &diff, &wrote("work/out.txt", "other\nrow2\n")).passed());
        assert!(matches!(
            judge(Class::Gold, &diff, &wrote("work/else.txt", "row\n")),
            Verdict::Fail(detail) if detail.contains("did not write")
        ));
    }

    #[test]
    fn a_compile_error_test_passes_on_a_language_error_and_never_on_a_refusal() {
        let none = Expectation::None;
        assert!(matches!(
            judge(
                Class::CompileError,
                &none,
                &RunEnd::CompileError {
                    message: "Parse error at offset 12: Expected ';', found 'endmodule'".to_owned()
                }
            ),
            Verdict::Pass(detail) if detail == "language error: Parse error: Expected ';', found 'endmodule'"
        ));
        assert_eq!(
            judge(
                Class::CompileError,
                &none,
                &RunEnd::Refused {
                    token: "fork".to_owned(),
                    message: "not yet supported".to_owned()
                }
            ),
            Verdict::Refused {
                token: "fork".to_owned(),
                detail: "not yet supported".to_owned()
            }
        );
        for end in [
            completed("PASSED\n"),
            RunEnd::RuntimeError {
                message: "boom".to_owned(),
            },
            RunEnd::NoContent {
                message: "empty".to_owned(),
            },
        ] {
            assert!(
                matches!(judge(Class::CompileError, &none, &end), Verdict::Fail(_)),
                "{end:?}"
            );
        }
    }

    #[test]
    fn refusal_forms_are_told_from_language_errors() {
        let refused = |message: &str| match compile_failure(message) {
            RunEnd::Refused { token, .. } => Some(token),
            RunEnd::CompileError { .. } => None,
            other => panic!("{other:?}"),
        };
        assert_eq!(
            refused(
                "Parse error at offset 40: Verilog-AMS digital construct not yet supported: `fork`"
            ),
            Some("fork".to_owned())
        );
        assert_eq!(
            refused(
                "Parse error at offset 9: Unsupported module item: `defparam`; IEEE 1364-2005 \
                 section 12.2.1 gives it the same effect"
            ),
            Some("defparam".to_owned())
        );
        assert_eq!(
            refused(
                "Semantic error at offset 3: Unsupported feature: unpacked net/port array `mem` requires array connection elaboration"
            ),
            Some("feature".to_owned())
        );
        assert_eq!(
            refused(
                "Code generation error: Verilog-AMS digital construct `wait` has no executable \
                 form in this compiler yet: x"
            ),
            Some("wait".to_owned())
        );
        // The token names the construct, never the user's own identifier.
        for (message, expected) in [
            (
                "Parse error at offset 1: Unsupported module item: function",
                "function",
            ),
            (
                "Parse error at offset 1: Unsupported module item: m",
                "module-item",
            ),
            (
                "Parse error at offset 1: Unsupported module instance `buffer`: an instance \
                 array range",
                "module-instance",
            ),
            (
                "Parse error at offset 1: Verilog-AMS digital construct not yet supported: `minus`",
                "statement",
            ),
            (
                "Semantic error at offset 1: Unsupported feature: hierarchical reference: `u1.q` \
                 is not a continuous net",
                "hierarchical-reference",
            ),
            (
                "Semantic error at offset 1: Unsupported feature: `$signed` takes one integral \
                 argument",
                "$signed",
            ),
            (
                "Semantic error at offset 1: Unsupported feature: `@*` names no signal",
                "feature",
            ),
        ] {
            assert_eq!(refused(message).as_deref(), Some(expected), "{message}");
        }
        // One refusal among several diagnostics refuses the whole source.
        assert_eq!(
            refused(
                "Compilation failed with 2 error(s):\n  - Semantic error at offset 1: Undeclared symbol: 'a'\n  \
                 - Parse error at offset 5: Verilog-AMS digital construct not yet supported: `task`"
            ),
            Some("task".to_owned())
        );
        for language_error in [
            "Parse error at offset 12: Expected ';', found 'endmodule'",
            "Semantic error at offset 3: Undeclared symbol: 'b'",
            "Semantic error at offset 3: Undefined module: 'sub'",
            "Lexer error at offset 0: Unterminated string literal",
        ] {
            assert_eq!(refused(language_error), None, "{language_error}");
        }
    }

    #[test]
    fn an_internal_error_fails_every_class_and_is_never_a_match_or_a_race() {
        let internal = compile_failure(
            "Code generation error: Internal error: the lowering lost symbol 'q' at 0:5-6",
        );
        assert_eq!(
            internal,
            RunEnd::InternalError {
                message: "Code generation error: the lowering lost symbol 'q' at 0:5-6".to_owned()
            }
        );
        // Among several diagnostics it wins over a refusal and a language
        // error alike.
        assert!(matches!(
            compile_failure(
                "Compilation failed with 3 error(s):\n  - Parse error at offset 5: Verilog-AMS \
                 digital construct not yet supported: `task`\n  - Semantic error at offset 1: \
                 Undeclared symbol: 'a'\n  - Code generation error: Internal error: broken"
            ),
            RunEnd::InternalError { message } if message == "Code generation error: broken"
        ));
        // At run time too, and only with the marker.
        assert!(matches!(
            run_failure("process top.p could not run: Internal error: no plan".to_owned()),
            RunEnd::InternalError { message } if message == "process top.p could not run: no plan"
        ));
        assert!(matches!(
            run_failure("tick 4 did not settle".to_owned()),
            RunEnd::RuntimeError { .. }
        ));
        // Every class fails on it, with the detail saying what it was.
        for class in Class::ALL {
            let verdict = judge(class, &Expectation::None, &internal);
            assert!(
                matches!(&verdict, Verdict::Internal(detail)
                    if detail == "internal error: Code generation error: the lowering lost symbol 'q' at 0:5-6"),
                "{class}: {verdict:?}"
            );
            assert_eq!(combine(verdict, None).0, Outcome::Fail, "{class}");
        }
        // On either schedule, whatever the other did: not a race.
        let pass = || Verdict::Pass("PASSED".to_owned());
        let broken = || Verdict::Internal("internal error: broken".to_owned());
        assert_eq!(
            combine(pass(), Some(broken())),
            (Outcome::Fail, "internal error: broken".to_owned())
        );
        assert_eq!(combine(broken(), Some(pass())).0, Outcome::Fail);
    }

    #[test]
    fn a_compile_only_test_passes_once_compiled() {
        let none = Expectation::None;
        for end in [
            completed(""),
            RunEnd::NoContent {
                message: "no processes".to_owned(),
            },
            RunEnd::RuntimeError {
                message: "tick did not settle".to_owned(),
            },
        ] {
            assert!(judge(Class::CompileOnly, &none, &end).passed(), "{end:?}");
        }
        assert!(
            !judge(
                Class::CompileOnly,
                &none,
                &RunEnd::CompileError {
                    message: "Parse error".to_owned()
                }
            )
            .passed()
        );
    }

    #[test]
    fn the_racy_rule_compares_verdicts_across_schedules() {
        let pass = || Verdict::Pass("PASSED".to_owned());
        let fail = || Verdict::Fail("no PASSED line".to_owned());
        assert_eq!(combine(pass(), Some(pass())).0, Outcome::Pass);
        assert_eq!(combine(fail(), Some(fail())).0, Outcome::Fail);
        let (outcome, detail) = combine(pass(), Some(fail()));
        assert_eq!(outcome, Outcome::Racy);
        assert!(detail.contains("source order only"), "{detail}");
        let (outcome, detail) = combine(fail(), Some(pass()));
        assert_eq!(outcome, Outcome::Racy);
        assert!(detail.contains("reverse order only"), "{detail}");
        // A test that did not run has no second schedule.
        assert_eq!(combine(fail(), None).0, Outcome::Fail);
        assert_eq!(
            combine(
                Verdict::Refused {
                    token: "task".to_owned(),
                    detail: String::new()
                },
                None
            )
            .0,
            Outcome::Refused("task".to_owned())
        );
    }

    #[test]
    fn outcome_tokens_round_trip_and_rank() {
        for outcome in [
            Outcome::Pass,
            Outcome::Fail,
            Outcome::Racy,
            Outcome::Refused("hdl.option.specify".to_owned()),
            Outcome::Excluded("flag-g2009".to_owned()),
            Outcome::Deviation,
        ] {
            assert_eq!(Outcome::from_token(&outcome.token()), Some(outcome));
        }
        assert_eq!(Outcome::from_token("passed"), None);
        assert!(Outcome::Pass.rank() > Outcome::Deviation.rank());
        assert!(Outcome::Deviation.rank() > Outcome::Racy.rank());
        assert!(Outcome::Pass.rank() > Outcome::Racy.rank());
        assert!(Outcome::Racy.rank() > Outcome::Refused(String::new()).rank());
        assert!(Outcome::Refused(String::new()).rank() > Outcome::Fail.rank());
    }

    #[test]
    fn details_lose_offsets_and_tabs() {
        assert_eq!(
            clip("Parse error at offset 1234: Expected\t';'\nsecond line"),
            "Parse error: Expected ';'"
        );
    }
}
