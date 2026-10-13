//! Which ivtest tests are in the selection, and which lane's bucket each is in.
//!
//! The rules are the P2 design of record's (section 2.2). A test is taken from
//! the standard and synthesis lists, or from the JSON list when it asks for no
//! language generation beyond 1364-2005. It is excluded, with the reason
//! recorded, when:
//!
//! - an Icarus-specific list overrides it;
//! - its type is not `normal`, `CE`, `RE` or `CO` (`NI` and `EF` are upstream's
//!   own not-implemented and expected-fail markers);
//! - it needs an Icarus generation or extension flag (`-g2005-sv`, `-g2009`,
//!   `-S`, `-W...`, `-I`, ...). `-gspecify`, `-T{min,typ,max}` and plusargs
//!   stay in: they map onto run options;
//! - its source uses SystemVerilog syntax or a SystemVerilog system function,
//!   a system task outside IEEE 1364-2005 clauses 15, 17 and 18, or
//!   `$sdf_annotate`.
//!
//! One rule goes past the design's, and applies only to a test those rules
//! keep: a synthesis-list `CE` is excluded as `synthesis-error`, because its
//! error is Icarus's synthesizer's and a synthesis test runs here as
//! simulation, where the design is legal.
//!
//! The source scan reads code only: comments and string literals are removed
//! first, in one pass, so a `//` inside a string does not hide the rest of a
//! line. The scan is lexical, a classifier rather than a parser: it can
//! mistake a construct, and the run records what RSpice actually did.
//!
//! Each selected test's bucket is the last lane, in landing order, whose
//! constructs it uses (section 2.2's unlock table, with `config` in L8).

use std::fmt;
use std::fs;
use std::path::Path;

use super::list::{Check, Entry, SourceList, ivl1_names, read_entries};

/// How a test is judged (the manifest's class column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    /// Runs; passes on a line reading `passed`.
    Passed,
    /// Runs; passes when its output matches a gold file.
    Gold,
    /// Must fail to compile with a language error.
    CompileError,
    /// Must compile.
    CompileOnly,
    /// Must stop with a run-time error.
    RuntimeError,
}

impl Class {
    pub const ALL: [Self; 5] = [
        Self::Passed,
        Self::Gold,
        Self::CompileError,
        Self::CompileOnly,
        Self::RuntimeError,
    ];

    pub const fn token(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Gold => "gold",
            Self::CompileError => "ce",
            Self::CompileOnly => "co",
            Self::RuntimeError => "re",
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|class| class.token() == token)
    }

    /// Whether the test is executed rather than only compiled.
    pub const fn runs(self) -> bool {
        matches!(self, Self::Passed | Self::Gold | Self::RuntimeError)
    }

    fn of(entry: &Entry) -> Option<Self> {
        Some(match entry.kind.as_str() {
            "CE" => Self::CompileError,
            "CO" => Self::CompileOnly,
            "RE" => Self::RuntimeError,
            "normal" => match entry.check {
                Check::PassedLine { .. } => Self::Passed,
                _ => Self::Gold,
            },
            _ => return None,
        })
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}

/// The P2 lanes, in the order their buckets unlock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lane {
    /// System tasks and functions; also every test that needs no lane.
    L1,
    /// Tasks, functions, fork/join, events, `wait`, `disable`, time variables.
    L3,
    /// `force`/`release`, `defparam`, hierarchical names, port expressions,
    /// instance arrays, indexed part-selects, the `celldefine` directives.
    L4,
    /// File I/O, `$readmem*`, `$dump*`.
    L5,
    /// Strengths, switch and pull primitives, UDPs, wired net types.
    L7,
    /// Delays on gates, nets and `assign`; min:typ:max; `specify`.
    L6,
    /// `config` and library mapping.
    L8,
}

impl Lane {
    pub const ALL: [Self; 7] = [
        Self::L1,
        Self::L3,
        Self::L4,
        Self::L5,
        Self::L7,
        Self::L6,
        Self::L8,
    ];

    pub const fn token(self) -> &'static str {
        match self {
            Self::L1 => "L1",
            Self::L3 => "L3",
            Self::L4 => "L4",
            Self::L5 => "L5",
            Self::L6 => "L6",
            Self::L7 => "L7",
            Self::L8 => "L8",
        }
    }

    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.token() == token)
    }
}

impl fmt::Display for Lane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.token())
    }
}

/// One test of the candidate pool, selected or excluded.
#[derive(Debug, Clone)]
pub struct Candidate {
    pub entry: Entry,
    /// `None` for a type outside the classes (always excluded).
    pub class: Option<Class>,
    pub lane: Lane,
    /// Why it is not in the selection; `None` when it is.
    pub exclusion: Option<String>,
    /// What of Icarus's run of a selected test the testbench entry point has
    /// no option for, and so a run here cannot reproduce: see
    /// [`testbench_gap`]. The run goes ahead as the entry point allows, and
    /// its row's detail names the gap.
    pub gap: Option<String>,
}

impl Candidate {
    pub fn selected(&self) -> bool {
        self.exclusion.is_none()
    }
}

/// Every candidate test of the checkout at `root`, in list order.
pub fn candidates(root: &Path) -> Result<Vec<Candidate>, String> {
    let overridden = ivl1_names(root)?;
    let mut out = Vec::new();
    for entry in read_entries(root)? {
        let class = Class::of(&entry);
        let mut exclusion = if entry.list == SourceList::Vlg && overridden.contains(&entry.name) {
            Some("icarus-override".to_owned())
        } else if class.is_none() {
            Some(format!("type-{}", entry.kind))
        } else {
            excluded_flag(&entry.args).map(|flag| format!("flag{flag}"))
        };
        let mut lane = Lane::L1;
        let mut gap = None;
        match fs::read(root.join(&entry.source)) {
            Err(_) => {
                exclusion.get_or_insert_with(|| "missing-source".to_owned());
            }
            Ok(bytes) if exclusion.is_none() => {
                let code = code_only(&String::from_utf8_lossy(&bytes));
                exclusion = source_exclusion(&code);
                lane = lane_of(&code, &entry.args);
                gap = testbench_gap(&entry, &code).filter(|_| exclusion.is_none());
            }
            Ok(_) => {}
        }
        // Past the design's own rules: a synthesis list's compile error is
        // Icarus's synthesizer (`-S`) rejecting the design; run as
        // simulation, the design is legal.
        if exclusion.is_none()
            && entry.list == SourceList::Synth
            && class == Some(Class::CompileError)
        {
            exclusion = Some("synthesis-error".to_owned());
            gap = None;
        }
        out.push(Candidate {
            entry,
            class,
            lane,
            exclusion,
            gap,
        });
    }
    Ok(out)
}

/// What Icarus's run of a test sets up that `run_digital_testbench` has no
/// option for:
///
/// - a root module the list names (`-s`); the entry point elaborates every
///   module nothing instantiates;
/// - a macro the source tests with `` `ifdef ``, `` `ifndef `` or
///   `` `elsif `` that Icarus's run defines: `__ICARUS__`, which the compiler
///   predefines, and `__ICARUS_UNSIZED__`, which the text lists' runner
///   passes (`-D`) to every test it compiles.
pub fn testbench_gap(entry: &Entry, code: &str) -> Option<String> {
    let mut gaps = Vec::new();
    if let Some(module) = &entry.module {
        gaps.push(format!("no root selection (-s {module})"));
    }
    let defined: &[&str] = match entry.list {
        SourceList::Vvp => &["__ICARUS__"],
        SourceList::Vlg | SourceList::Synth => &["__ICARUS__", "__ICARUS_UNSIZED__"],
    };
    let tested: Vec<&str> = defined
        .iter()
        .copied()
        .filter(|name| tests_macro(code, name))
        .collect();
    if !tested.is_empty() {
        gaps.push(format!("no defines ({})", tested.join(", ")));
    }
    (!gaps.is_empty()).then(|| format!("testbench gap: {}", gaps.join(", ")))
}

/// Whether a conditional-compilation directive tests the macro `name`.
fn tests_macro(code: &str, name: &str) -> bool {
    let bytes = code.as_bytes();
    words(code).any(|(start, word)| {
        start > 0 && bytes[start - 1] == b'`' && matches!(word, "ifdef" | "ifndef" | "elsif") && {
            let at = skip_space(bytes, start + word.len());
            identifier_end(bytes, at).is_some_and(|end| &code[at..end] == name)
        }
    })
}

/// The first compiler flag that takes a test out of the selection: an
/// Icarus generation, extension, warning, include, define or library flag.
fn excluded_flag(args: &[String]) -> Option<&str> {
    const EXCLUDED: [&str; 31] = [
        "g2005-sv",
        "g2009",
        "g2012",
        "g2017",
        "gverilog-ams",
        "pallowsigned",
        "gxtypes",
        "gno-xtypes",
        "m",
        "S",
        "g2",
        "g1995",
        "gstrict",
        "gno-io-range-error",
        "grelative-include",
        "f",
        "W",
        "P",
        "ginterconnect",
        "gno-specify",
        "g2001-noconfig",
        "g2001",
        "gstrict-ca-eval",
        "gstrict-expr-width",
        "L",
        "l",
        "y",
        "I",
        "D",
        "N",
        "E",
    ];
    args.iter()
        .map(String::as_str)
        .filter(|arg| !arg.starts_with('+') && !arg.starts_with("-T") && *arg != "-gspecify")
        .find(|arg| {
            arg.strip_prefix('-')
                .is_some_and(|flag| EXCLUDED.iter().any(|prefix| flag.starts_with(prefix)))
        })
}

/// The source with comments removed and every string literal emptied.
pub fn code_only(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut out = String::with_capacity(source.len());
    let mut index = 0;
    let mut start = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'/' if bytes.get(index + 1) == Some(&b'/') => {
                out.push_str(&source[start..index]);
                out.push(' ');
                index = position_from(bytes, index, |byte| byte == b'\n').unwrap_or(bytes.len());
                start = index;
            }
            b'/' if bytes.get(index + 1) == Some(&b'*') => {
                out.push_str(&source[start..index]);
                out.push(' ');
                index = bytes[index + 2..]
                    .windows(2)
                    .position(|window| window == b"*/")
                    .map_or(bytes.len(), |offset| index + 2 + offset + 2);
                start = index;
            }
            b'"' => {
                out.push_str(&source[start..index]);
                out.push_str("\"\"");
                index += 1;
                while index < bytes.len() && bytes[index] != b'"' && bytes[index] != b'\n' {
                    index += if bytes[index] == b'\\' { 2 } else { 1 };
                }
                index = (index + 1).min(bytes.len());
                start = index;
            }
            _ => index += 1,
        }
    }
    out.push_str(&source[start.min(source.len())..]);
    out
}

fn position_from(bytes: &[u8], from: usize, found: impl Fn(u8) -> bool) -> Option<usize> {
    bytes
        .get(from..)?
        .iter()
        .position(|&byte| found(byte))
        .map(|offset| from + offset)
}

fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn is_word_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

/// Maximal runs of word characters (`[A-Za-z0-9_]`), with their offsets.
fn words(code: &str) -> impl Iterator<Item = (usize, &str)> + '_ {
    let bytes = code.as_bytes();
    let mut index = 0;
    std::iter::from_fn(move || {
        while index < bytes.len() && !is_word(bytes[index]) {
            index += 1;
        }
        if index >= bytes.len() {
            return None;
        }
        let start = index;
        while index < bytes.len() && is_word(bytes[index]) {
            index += 1;
        }
        Some((start, &code[start..index]))
    })
}

/// The offset just past any white space from `index`.
fn skip_space(bytes: &[u8], mut index: usize) -> usize {
    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
        index += 1;
    }
    index
}

/// The end of the identifier (`[A-Za-z_]\w*`) at `index`, if one starts there.
fn identifier_end(bytes: &[u8], index: usize) -> Option<usize> {
    if !bytes.get(index).copied().is_some_and(is_word_start) {
        return None;
    }
    let mut end = index + 1;
    while end < bytes.len() && is_word(bytes[end]) {
        end += 1;
    }
    Some(end)
}

const SYSTEMVERILOG_KEYWORDS: &[&str] = &[
    "logic",
    "bit",
    "byte",
    "shortint",
    "int",
    "longint",
    "shortreal",
    "typedef",
    "struct",
    "union",
    "enum",
    "class",
    "interface",
    "modport",
    "package",
    "import",
    "program",
    "always_comb",
    "always_ff",
    "always_latch",
    "unique",
    "unique0",
    "priority",
    "final",
    "assert",
    "assume",
    "cover",
    "property",
    "sequence",
    "clocking",
    "virtual",
    "extends",
    "rand",
    "randc",
    "constraint",
    "foreach",
    "return",
    "break",
    "continue",
    "do",
    "void",
    "string",
    "var",
    "const",
    "static",
    "ref",
    "timeunit",
    "timeprecision",
    "join_any",
    "join_none",
    "inside",
    "type",
    "nettype",
    "interconnect",
    "let",
    "chandle",
    "alias",
    "bind",
    "checker",
];

/// System tasks and functions of IEEE 1364-2005 (clauses 17 and 18, and the
/// clause 15 timing checks).
const STANDARD_SYSTEM_NAMES: &[&str] = &[
    "display",
    "displayb",
    "displayo",
    "displayh",
    "write",
    "writeb",
    "writeo",
    "writeh",
    "strobe",
    "strobeb",
    "strobeo",
    "strobeh",
    "monitor",
    "monitorb",
    "monitoro",
    "monitorh",
    "monitoron",
    "monitoroff",
    "sdf_annotate",
    "printtimescale",
    "timeformat",
    "finish",
    "stop",
    "time",
    "stime",
    "realtime",
    "bitstoreal",
    "realtobits",
    "itor",
    "rtoi",
    "signed",
    "unsigned",
    "random",
    "dist_chi_square",
    "dist_erlang",
    "dist_exponential",
    "dist_normal",
    "dist_poisson",
    "dist_t",
    "dist_uniform",
    "test$plusargs",
    "value$plusargs",
    "clog2",
    "ln",
    "log10",
    "exp",
    "sqrt",
    "pow",
    "floor",
    "ceil",
    "sin",
    "cos",
    "tan",
    "asin",
    "acos",
    "atan",
    "atan2",
    "hypot",
    "sinh",
    "cosh",
    "tanh",
    "asinh",
    "acosh",
    "atanh",
];

/// The clause 17.2 file tasks and the clause 18 VCD tasks (lane L5).
const FILE_SYSTEM_NAMES: &[&str] = &[
    "fopen",
    "fclose",
    "fdisplay",
    "fdisplayb",
    "fdisplayo",
    "fdisplayh",
    "fwrite",
    "fwriteb",
    "fwriteo",
    "fwriteh",
    "fstrobe",
    "fstrobeb",
    "fstrobeo",
    "fstrobeh",
    "fmonitor",
    "fmonitorb",
    "fmonitoro",
    "fmonitorh",
    "swrite",
    "swriteb",
    "swriteo",
    "swriteh",
    "sformat",
    "fgetc",
    "ungetc",
    "fgets",
    "fscanf",
    "sscanf",
    "fread",
    "ftell",
    "fseek",
    "rewind",
    "fflush",
    "ferror",
    "feof",
    "readmemb",
    "readmemh",
    "dumpfile",
    "dumpvars",
    "dumpoff",
    "dumpon",
    "dumpall",
    "dumplimit",
    "dumpflush",
];

/// The clause 15 timing checks (lane L6, with `specify`).
const TIMING_CHECK_NAMES: &[&str] = &[
    "setup",
    "hold",
    "setuphold",
    "recovery",
    "removal",
    "recrem",
    "skew",
    "timeskew",
    "fullskew",
    "period",
    "width",
    "nochange",
];

/// IEEE 1800 system functions and severity tasks.
const SYSTEMVERILOG_SYSTEM_NAMES: &[&str] = &[
    "urandom",
    "urandom_range",
    "bits",
    "size",
    "left",
    "right",
    "low",
    "high",
    "increment",
    "dimensions",
    "unpacked_dimensions",
    "countones",
    "onehot",
    "onehot0",
    "isunknown",
    "error",
    "fatal",
    "warning",
    "info",
    "cast",
    "typename",
    "past",
    "rose",
    "fell",
    "stable",
    "sampled",
];

/// Why the code puts its test out of the selection, if it does.
fn source_exclusion(code: &str) -> Option<String> {
    if let Some((_, keyword)) = words(code).find(|(_, word)| SYSTEMVERILOG_KEYWORDS.contains(word))
    {
        return Some(format!("systemverilog:{keyword}"));
    }
    if let Some(found) = systemverilog_operator(code) {
        return Some(format!("systemverilog:{found}"));
    }
    let names = system_names(code);
    let standard = |name: &&str| {
        STANDARD_SYSTEM_NAMES.contains(name)
            || FILE_SYSTEM_NAMES.contains(name)
            || TIMING_CHECK_NAMES.contains(name)
    };
    let nonstandard: Vec<&str> = names
        .iter()
        .copied()
        .filter(|name| !standard(name))
        .collect();
    if let Some(name) = nonstandard
        .iter()
        .find(|name| SYSTEMVERILOG_SYSTEM_NAMES.contains(name))
    {
        return Some(format!("systemverilog-systf:${name}"));
    }
    if let Some(name) = nonstandard.first() {
        return Some(format!("nonstandard-systf:${name}"));
    }
    names
        .contains(&"sdf_annotate")
        .then(|| "sdf-annotate".to_owned())
}

/// Every `$name` the code calls (`\$[A-Za-z_][\w$]*`), sorted, once each.
fn system_names(code: &str) -> Vec<&str> {
    let bytes = code.as_bytes();
    let mut names = Vec::new();
    let mut index = 0;
    while let Some(dollar) = position_from(bytes, index, |byte| byte == b'$') {
        let start = dollar + 1;
        index = start;
        if !bytes.get(start).copied().is_some_and(is_word_start) {
            continue;
        }
        while index < bytes.len() && (is_word(bytes[index]) || bytes[index] == b'$') {
            index += 1;
        }
        names.push(&code[start..index]);
    }
    names.sort_unstable();
    names.dedup();
    names
}

/// The leftmost SystemVerilog operator form: `++`, `--`, an `op=` compound
/// assignment, `'{`, `::`, `.*`, `##` or a `'(` cast.
fn systemverilog_operator(code: &str) -> Option<&str> {
    const COMPOUND: [&str; 12] = [
        "+", "-", "*", "/", "%", "&", "|", "^", "<<<", ">>>", "<<", ">>",
    ];
    const BEFORE: &[u8] = b"<>=!+-*/%&|^~";
    const OTHERS: [&str; 5] = ["'{", "::", ".*", "##", "'("];
    let bytes = code.as_bytes();
    for start in 0..bytes.len() {
        if !code.is_char_boundary(start) {
            continue;
        }
        let rest = &code[start..];
        if rest.starts_with("++") || rest.starts_with("--") {
            return Some(&rest[..2]);
        }
        let preceded = start > 0 && BEFORE.contains(&bytes[start - 1]);
        if !preceded {
            for op in COMPOUND {
                let length = op.len();
                if rest.starts_with(op)
                    && rest.as_bytes().get(length) == Some(&b'=')
                    && rest.as_bytes().get(length + 1) != Some(&b'=')
                {
                    return Some(&rest[..=length]);
                }
            }
        }
        if let Some(other) = OTHERS.iter().find(|other| rest.starts_with(**other)) {
            return Some(&rest[..other.len()]);
        }
    }
    None
}

/// The bucket: the last lane in landing order that the code or its run
/// options need.
fn lane_of(code: &str, args: &[String]) -> Lane {
    let mut lane = Lane::L1;
    let mut need = |needed: Lane| lane = lane.max(needed);
    for name in system_names(code) {
        if FILE_SYSTEM_NAMES.contains(&name) {
            need(Lane::L5);
        } else if TIMING_CHECK_NAMES.contains(&name) {
            need(Lane::L6);
        }
    }
    let bytes = code.as_bytes();
    for (start, word) in words(code) {
        let end = start + word.len();
        match word {
            "task" | "function" | "automatic" | "fork" | "disable" | "wait" | "event" => {
                need(Lane::L3);
            }
            // A `time` or `realtime` variable declaration.
            "time" | "realtime" => {
                let next = skip_space(bytes, end);
                if next > end
                    && bytes
                        .get(next)
                        .is_some_and(|&byte| is_word_start(byte) || byte == b'[')
                {
                    need(Lane::L3);
                }
            }
            "force" | "release" | "deassign" | "defparam" | "scalared" | "vectored" => {
                need(Lane::L4);
            }
            "specify" => need(Lane::L6),
            "assign" if bytes.get(skip_space(bytes, end)) == Some(&b'#') => need(Lane::L6),
            "and" | "nand" | "or" | "nor" | "xor" | "xnor" | "buf" | "not" | "bufif0"
            | "bufif1" | "notif0" | "notif1" | "nmos" | "pmos" | "cmos" | "rnmos" | "rpmos"
            | "rcmos" | "pullup" | "pulldown" | "tranif0" | "tranif1" | "rtranif0" | "rtranif1"
                if gate_delay(bytes, end) =>
            {
                need(Lane::L6);
            }
            "wire" | "tri" | "wand" | "wor" | "tri0" | "tri1" | "trireg" | "supply0"
            | "supply1" | "triand" | "trior"
                if net_delay(bytes, end) =>
            {
                need(Lane::L6);
            }
            _ => {}
        }
        if matches!(
            word,
            "bufif0"
                | "bufif1"
                | "notif0"
                | "notif1"
                | "pullup"
                | "pulldown"
                | "nmos"
                | "pmos"
                | "rnmos"
                | "rpmos"
                | "cmos"
                | "rcmos"
                | "tran"
                | "rtran"
                | "tranif0"
                | "tranif1"
                | "rtranif0"
                | "rtranif1"
                | "primitive"
                | "wand"
                | "wor"
                | "triand"
                | "trior"
                | "tri0"
                | "tri1"
                | "supply0"
                | "supply1"
                | "trireg"
        ) {
            need(Lane::L7);
        }
        if matches!(word, "config" | "library") && first_on_line(bytes, start) {
            need(Lane::L8);
        }
        if start > 0
            && bytes[start - 1] == b'`'
            && matches!(
                word,
                "celldefine"
                    | "endcelldefine"
                    | "unconnected_drive"
                    | "nounconnected_drive"
                    | "line"
                    | "pragma"
                    | "begin_keywords"
                    | "end_keywords"
                    | "__FILE__"
                    | "__LINE__"
            )
        {
            need(Lane::L4);
        }
    }
    if code.contains("->") {
        need(Lane::L3);
    }
    if indexed_part_select(bytes)
        || instance_array(bytes)
        || port_expression(bytes)
        || hierarchical_reference(code)
    {
        need(Lane::L4);
    }
    if min_typ_max(bytes) {
        need(Lane::L6);
    }
    if strength(bytes) {
        need(Lane::L7);
    }
    // `specify` timing and min/max delay selection are L6 run options.
    if args
        .iter()
        .any(|arg| arg == "-gspecify" || arg == "-Tmin" || arg == "-Tmax")
    {
        need(Lane::L6);
    }
    lane
}

/// After a gate keyword: an optional `( ... )` strength, then `#`.
fn gate_delay(bytes: &[u8], end: usize) -> bool {
    let mut index = skip_space(bytes, end);
    if bytes.get(index) == Some(&b'(') {
        let Some(close) = position_from(bytes, index, |byte| byte == b')') else {
            return false;
        };
        let after = skip_space(bytes, close + 1);
        if bytes.get(after) == Some(&b'#') {
            return true;
        }
        index = skip_space(bytes, end);
    }
    bytes.get(index) == Some(&b'#')
}

/// After a net type keyword: a `#` before the declaration's `;`.
fn net_delay(bytes: &[u8], end: usize) -> bool {
    bytes[end..]
        .iter()
        .take_while(|&&byte| byte != b';')
        .any(|&byte| byte == b'#')
}

/// Whether the word at `start` is the first thing on its line.
fn first_on_line(bytes: &[u8], start: usize) -> bool {
    bytes[..start]
        .iter()
        .rev()
        .take_while(|&&byte| byte != b'\n')
        .all(u8::is_ascii_whitespace)
}

/// `[ ... +: ... ]` or `[ ... -: ... ]` with no bracket inside.
fn indexed_part_select(bytes: &[u8]) -> bool {
    let mut index = 0;
    while let Some(open) = position_from(bytes, index, |byte| byte == b'[') {
        index = open + 1;
        let Some(close) = position_from(bytes, index, |byte| byte == b'[' || byte == b']') else {
            break;
        };
        if bytes[close] == b']'
            && bytes[open + 1..close]
                .windows(2)
                .any(|pair| pair == b"+:" || pair == b"-:")
        {
            return true;
        }
    }
    false
}

/// An instance array: `<module> [#( ... )] <name> [<a>:<b>] (` starting a
/// line.
fn instance_array(bytes: &[u8]) -> bool {
    (0..bytes.len())
        .filter(|&start| start == 0 || bytes[start - 1] == b'\n')
        .any(|line| {
            let start = skip_space(bytes, line);
            let Some(module_end) = identifier_end(bytes, start) else {
                return false;
            };
            let after_module = skip_space(bytes, module_end);
            if after_module == module_end {
                return false;
            }
            // Without a parameter list, or with one: `#`, `(`, then each `)`
            // before the next `;` as its possible end.
            let mut name_starts = vec![after_module];
            if bytes.get(after_module) == Some(&b'#') {
                let open = skip_space(bytes, after_module + 1);
                if bytes.get(open) == Some(&b'(') {
                    let limit =
                        position_from(bytes, open, |byte| byte == b';').unwrap_or(bytes.len());
                    name_starts.extend(
                        (open + 1..limit)
                            .filter(|&index| bytes[index] == b')')
                            .map(|close| skip_space(bytes, close + 1)),
                    );
                }
            }
            name_starts.into_iter().any(|name| {
                let Some(name_end) = identifier_end(bytes, name) else {
                    return false;
                };
                let open = skip_space(bytes, name_end);
                if bytes.get(open) != Some(&b'[') {
                    return false;
                }
                let Some(close) = position_from(bytes, open + 1, |byte| byte == b']') else {
                    return false;
                };
                let range = &bytes[open + 1..close];
                let colon = range
                    .iter()
                    .enumerate()
                    .any(|(at, &byte)| byte == b':' && at > 0 && at + 1 < range.len());
                colon && bytes.get(skip_space(bytes, close + 1)) == Some(&b'(')
            })
        })
}

/// A named port connection to an expression: `.port( <expr> )` where the
/// expression starts with a digit or holds an operator, concatenation or
/// based literal.
fn port_expression(bytes: &[u8]) -> bool {
    const OPERATORS: &[u8] = b"{}~&|^+*/<>!?'";
    let mut index = 0;
    while let Some(dot) = position_from(bytes, index, |byte| byte == b'.') {
        index = dot + 1;
        let mut end = index;
        while end < bytes.len() && is_word(bytes[end]) {
            end += 1;
        }
        if end == index {
            continue;
        }
        let open = skip_space(bytes, end);
        if bytes.get(open) != Some(&b'(') {
            continue;
        }
        let Some(close) = position_from(bytes, open + 1, |byte| byte == b'(' || byte == b')')
        else {
            continue;
        };
        if bytes[close] != b')' {
            continue;
        }
        let inside = &bytes[skip_space(bytes, open + 1).min(close)..close];
        if inside.first().is_some_and(u8::is_ascii_digit)
            || inside.iter().any(|byte| OPERATORS.contains(byte))
        {
            return true;
        }
    }
    false
}

/// `#( a : b : c )` delays, or `= ( a : b : c )` values.
fn min_typ_max(bytes: &[u8]) -> bool {
    let mut index = 0;
    while let Some(at) = position_from(bytes, index, |byte| byte == b'#' || byte == b'=') {
        index = at + 1;
        let open = skip_space(bytes, at + 1);
        if bytes.get(open) != Some(&b'(') {
            continue;
        }
        let Some(close) = position_from(bytes, open + 1, |byte| b"();".contains(&byte)) else {
            continue;
        };
        if bytes[close] != b')' {
            continue;
        }
        let inside = &bytes[open + 1..close];
        let fields: Vec<&[u8]> = inside.split(|&byte| byte == b':').collect();
        let delay = bytes[at] == b'#' && fields.len() >= 3;
        let value = bytes[at] == b'='
            && fields.len() == 3
            && fields.iter().all(|field| {
                let field = field.trim_ascii();
                !field.is_empty() && field.iter().all(|&byte| is_word(byte) || byte == b'.')
            });
        if delay || value {
            return true;
        }
    }
    false
}

/// A drive or charge strength: `( strong0, ...`, `( pull1 )`, `(weak0 ...`.
fn strength(bytes: &[u8]) -> bool {
    const STRENGTHS: [&[u8]; 8] = [
        b"strong", b"weak", b"pull", b"supply", b"highz", b"small", b"medium", b"large",
    ];
    let mut index = 0;
    while let Some(open) = position_from(bytes, index, |byte| byte == b'(') {
        index = open + 1;
        let start = skip_space(bytes, index);
        let Some(end) = identifier_end(bytes, start) else {
            continue;
        };
        let word = &bytes[start..end];
        let base = match word.last() {
            Some(b'0' | b'1') => &word[..word.len() - 1],
            _ => word,
        };
        let known = STRENGTHS.contains(&base) || STRENGTHS.contains(&word);
        if known && matches!(bytes.get(skip_space(bytes, end)), Some(b',' | b')')) {
            return true;
        }
    }
    false
}

/// Whether the code names something through a dotted path (`top.u1.q`): a
/// whole identifier, not preceded by `.`, `$`, `'` or a backtick, followed
/// directly by `.` and another identifier.
fn hierarchical_reference(code: &str) -> bool {
    let bytes = code.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if !is_word_start(bytes[index])
            || (index > 0 && (is_word(bytes[index - 1]) || b".$'`".contains(&bytes[index - 1])))
        {
            index += 1;
            continue;
        }
        let end = identifier_end(bytes, index).unwrap_or(index + 1);
        if bytes.get(end) == Some(&b'.') && bytes.get(end + 1).copied().is_some_and(is_word_start) {
            return true;
        }
        index = end;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_and_strings_are_removed_in_one_pass() {
        let code = code_only(
            "a = 1; // comment with task\n$display(\"http://x fork\"); /* fork\n */ b = 2;\n",
        );
        assert!(!code.contains("comment"), "{code}");
        assert!(!code.contains("fork"), "{code}");
        assert!(code.contains("$display(\"\")"), "{code}");
        assert!(code.contains("b = 2;"), "{code}");
        // An escaped quote does not end the literal.
        assert_eq!(code_only(r#"x("a\"b") y"#), r#"x("") y"#);
    }

    #[test]
    fn systemverilog_operators_are_found_and_verilog_ones_are_not() {
        for (code, expected) in [
            ("i++;", Some("++")),
            ("x <<= 1;", Some("<<=")),
            ("x += y;", Some("+=")),
            ("a = '{1, 2};", Some("'{")),
            ("p::q", Some("::")),
            ("m u(.*);", Some(".*")),
            ("##1", Some("##")),
            ("int'(x)", Some("'(")),
            ("a <= b; c >= d; e == f; g != h; i === j;", None),
            ("x = -1; y = a - b;", None),
            ("x[i+:4] = y[j-:2];", None),
            ("if (a >= b) c = d;", None),
        ] {
            assert_eq!(systemverilog_operator(code), expected, "{code}");
        }
    }

    #[test]
    fn exclusions_name_their_reason() {
        assert_eq!(
            source_exclusion("logic a;").as_deref(),
            Some("systemverilog:logic")
        );
        assert_eq!(source_exclusion("reg logical;"), None);
        assert_eq!(
            source_exclusion("initial $display($bits(a));").as_deref(),
            Some("systemverilog-systf:$bits")
        );
        assert_eq!(
            source_exclusion("initial $countdrivers(a);").as_deref(),
            Some("nonstandard-systf:$countdrivers")
        );
        assert_eq!(
            source_exclusion("initial $sdf_annotate(\"\", top);").as_deref(),
            Some("sdf-annotate")
        );
        assert_eq!(
            source_exclusion("initial begin $display(\"\"); $fopen(\"\"); end"),
            None
        );
        assert_eq!(
            source_exclusion("initial if ($test$plusargs(\"\")) $finish;"),
            None
        );
    }

    #[test]
    fn options_the_testbench_lacks_are_named_as_gaps() {
        let entry = |list: SourceList, module: Option<&str>| Entry {
            name: "t".to_owned(),
            list,
            kind: "normal".to_owned(),
            args: Vec::new(),
            plusargs: Vec::new(),
            runtime_flags: (Vec::new(), Vec::new()),
            source: std::path::PathBuf::from("ivltests/t.v"),
            module: module.map(str::to_owned),
            check: Check::PassedLine { strict: false },
        };
        let vlg = entry(SourceList::Vlg, None);
        assert_eq!(testbench_gap(&vlg, "module m; endmodule"), None);
        assert_eq!(
            testbench_gap(&entry(SourceList::Vlg, Some("main")), "").as_deref(),
            Some("testbench gap: no root selection (-s main)")
        );
        assert_eq!(
            testbench_gap(
                &vlg,
                "`ifdef __ICARUS__\n`elsif  __ICARUS_UNSIZED__\n`endif"
            )
            .as_deref(),
            Some("testbench gap: no defines (__ICARUS__, __ICARUS_UNSIZED__)")
        );
        // The JSON list's runner defines nothing of its own.
        assert_eq!(
            testbench_gap(
                &entry(SourceList::Vvp, None),
                "`ifndef __ICARUS_UNSIZED__ `endif"
            ),
            None
        );
        // Only a directive that tests the macro counts, and only that name.
        for code in [
            "`define __ICARUS__",
            "`ifdef __ICARUS__X",
            "`ifdef OTHER __ICARUS__",
            "ifdef __ICARUS__",
        ] {
            assert_eq!(testbench_gap(&vlg, code), None, "{code}");
        }
        assert_eq!(
            testbench_gap(
                &entry(SourceList::Synth, Some("top")),
                "`ifndef __ICARUS__ `endif"
            )
            .as_deref(),
            Some("testbench gap: no root selection (-s top), no defines (__ICARUS__)")
        );
    }

    #[test]
    fn flags_outside_the_selection_are_named_and_mapped_flags_stay_in() {
        let args = |list: &[&str]| list.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
        assert_eq!(excluded_flag(&args(&["-g2009"])), Some("-g2009"));
        assert_eq!(excluded_flag(&args(&["-Wall"])), Some("-Wall"));
        assert_eq!(excluded_flag(&args(&["-Iivltests"])), Some("-Iivltests"));
        assert_eq!(excluded_flag(&args(&["-gspecify", "-Tmax", "+plus"])), None);
    }

    #[test]
    fn the_bucket_is_the_last_lane_in_landing_order() {
        let none: [String; 0] = [];
        let lane = |code: &str| lane_of(code, &none);
        assert_eq!(lane("initial $display(\"\");"), Lane::L1);
        assert_eq!(lane("initial $display($time, a);"), Lane::L1);
        assert_eq!(lane("task t; endtask"), Lane::L3);
        assert_eq!(lane("initial -> e;"), Lane::L3);
        assert_eq!(lane("time t;"), Lane::L3);
        assert_eq!(lane("initial force a = 1;"), Lane::L4);
        assert_eq!(lane("initial top.u1.q = 1;"), Lane::L4);
        assert_eq!(lane("initial r = 1.5e3;"), Lane::L1);
        assert_eq!(lane("assign y = a[i +: 4];"), Lane::L4);
        assert_eq!(lane("sub #(.W(4)) u [3:0] (.a(a));"), Lane::L4);
        assert_eq!(lane("sub u (.a(b & c));"), Lane::L4);
        assert_eq!(lane("sub u (.a(b), .c(1'b0));"), Lane::L4);
        assert_eq!(lane("sub u (.a(b), .c(d));"), Lane::L1);
        assert_eq!(lane("`celldefine\nmodule m; endmodule"), Lane::L4);
        assert_eq!(
            lane("initial begin $fopen(\"\"); force a = 1; end"),
            Lane::L5
        );
        assert_eq!(lane("bufif1 b(o, i, e); task t; endtask"), Lane::L7);
        assert_eq!(lane("assign (strong0, weak1) y = a;"), Lane::L7);
        assert_eq!(lane("assign #1 a = b; tri1 t;"), Lane::L6);
        assert_eq!(lane("and #(1:2:3) g(o, a, b);"), Lane::L6);
        assert_eq!(
            lane("and (strong0, strong1) #2 g(o, a, b);"),
            Lane::L7.max(Lane::L6)
        );
        assert_eq!(lane("wire #3 w = a;"), Lane::L6);
        assert_eq!(lane("parameter p = (1:2:3);"), Lane::L6);
        assert_eq!(lane("specify endspecify"), Lane::L6);
        assert_eq!(lane("config cfg;\nendconfig"), Lane::L8);
        assert_eq!(lane_of("", &["-gspecify".to_owned()]), Lane::L6);
    }
}
