//! The names the compiler gives a meaning of its own, listed for an editor.
//!
//! An editor that completes, explains and guards names has to know which
//! names belong to the language: the reserved words, the functions and analog
//! operators called by their plain name, the `$` system functions and tasks
//! analog code accepts, the constants read by their bare name, and the
//! standard headers built into the compiler. Each list is the table the
//! compiler itself decides by, or is pinned against it, so an editor can
//! neither offer a name the compiler refuses nor miss one it accepts.

use crate::lexer::TokenKind;

/// What a built-in name is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltinKind {
    /// A function or analog operator called by its plain name: `exp`, `ddt`.
    Function,
    /// A `$` system function analog code reads a value from: `$temperature`.
    SystemFunction,
    /// A `$` system task analog code calls as a statement: `$strobe`.
    SystemTask,
    /// A constant read by its bare name: `M_PI`.
    Constant,
}

/// The system tasks an analog block accepts as a statement. Every other
/// standard one is refused there as unsupported (pinned by
/// `analog_code_accepts_exactly_the_listed_system_tasks`).
const ANALOG_SYSTEM_TASKS: &[&str] = &[
    "$strobe",
    "$display",
    "$write",
    "$monitor",
    "$info",
    "$warning",
    "$error",
    "$finish",
    "$bound_step",
    "$discontinuity",
    "$analog_node_alias",
    "$analog_port_alias",
];

/// The words the lexer reserves, so that none of them is ever a name: every
/// word [`TokenKind::keyword`] reads as a keyword, less the digital ones it
/// reserves only to refuse (pinned by `the_reserved_words_are_the_lexers`).
const RESERVED_WORDS: &[&str] = &[
    "above",
    "abstol",
    "access",
    "aliasparam",
    "always",
    "analog",
    "assign",
    "begin",
    "case",
    "casex",
    "casez",
    "connect",
    "connectmodule",
    "connectrules",
    "continuous",
    "cross",
    "current",
    "ddt_nature",
    "deassign",
    "default",
    "disable",
    "discipline",
    "discrete",
    "domain",
    "electrical",
    "else",
    "end",
    "endcase",
    "endconnectrules",
    "enddiscipline",
    "endfunction",
    "endgenerate",
    "endmodule",
    "endnature",
    "endspecify",
    "endtask",
    "exclude",
    "final",
    "flow",
    "for",
    "force",
    "forever",
    "fork",
    "from",
    "function",
    "generate",
    "genvar",
    "ground",
    "idt_nature",
    "if",
    "inf",
    "initial",
    "inout",
    "input",
    "integer",
    "join",
    "localparam",
    "macromodule",
    "merged",
    "module",
    "nature",
    "negedge",
    "output",
    "parameter",
    "posedge",
    "potential",
    "real",
    "reg",
    "release",
    "repeat",
    "resolveto",
    "signed",
    "specify",
    "split",
    "string",
    "supply0",
    "supply1",
    "task",
    "timer",
    "tri",
    "tri0",
    "tri1",
    "triand",
    "trior",
    "units",
    "unsigned",
    "uwire",
    "voltage",
    "wand",
    "while",
    "wire",
    "wor",
    "wreal",
    "wrealavg",
    "wrealmax",
    "wrealmin",
    "wrealsum",
];

/// The names an include of a standard header may write, one per header.
const BUILTIN_HEADER_NAMES: &[&str] = &["disciplines.vams", "constants.vams"];

/// Every built-in name, with what it is, in the compiler's own order.
///
/// Names the compiler reserves for itself (`__rspice_…`) are not listed: no
/// source writes them.
pub fn builtins() -> impl Iterator<Item = (&'static str, BuiltinKind)> {
    let function = |name: &&'static str| (*name, BuiltinKind::Function);
    crate::expr_converter::LOWERED_CALL_NAMES
        .iter()
        .filter(|name| !name.starts_with("__rspice"))
        .map(function)
        .chain(
            crate::expr_converter::LOWERED_SYSTEM_FUNCTION_NAMES
                .iter()
                .map(|name| (*name, BuiltinKind::SystemFunction)),
        )
        .chain(
            ANALOG_SYSTEM_TASKS
                .iter()
                .map(|name| (*name, BuiltinKind::SystemTask)),
        )
        .chain(
            crate::expr_converter::BUILTIN_CONSTANT_NAMES
                .iter()
                // `inf` is a reserved word as well as a value.
                .filter(|name| TokenKind::keyword(name).is_none())
                .map(|name| (*name, BuiltinKind::Constant)),
        )
}

/// What `name` is, when it is a built-in name.
pub fn builtin_kind(name: &str) -> Option<BuiltinKind> {
    builtins()
        .find(|(builtin, _)| *builtin == name)
        .map(|(_, kind)| kind)
}

/// The reserved words, in alphabetical order.
pub fn reserved_words() -> impl Iterator<Item = &'static str> {
    RESERVED_WORDS.iter().copied()
}

/// Whether the lexer reads `word` as a keyword rather than a name, the
/// digital words it reserves only to refuse included.
pub fn is_reserved_word(word: &str) -> bool {
    TokenKind::keyword(word).is_some()
}

/// The standard headers built into the compiler: the name an include writes
/// for each, and its text.
pub fn builtin_headers() -> impl Iterator<Item = (&'static str, &'static str)> {
    BUILTIN_HEADER_NAMES.iter().filter_map(|name| {
        crate::stdlib::builtin_include_document(name).map(|(_, source)| (*name, source))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// The reserved-word list is the lexer's table: a word added to
    /// [`TokenKind::keyword`] and missing here fails, and so does a word here
    /// the lexer reads as a name.
    #[test]
    fn the_reserved_words_are_the_lexers() {
        let lexer = include_str!("lexer.rs");
        let start = lexer
            .find("pub fn keyword(s: &str)")
            .expect("the lexer's keyword table");
        let end = start
            + lexer[start..]
                .find("_ => return None")
                .expect("the table's end");
        let mut table = BTreeSet::new();
        for line in lexer[start..end].lines() {
            let code = line.split("//").next().unwrap_or_default();
            if code.contains("AmsDigital") {
                continue;
            }
            for word in code.split('"').skip(1).step_by(2) {
                table.insert(word);
            }
        }
        let listed = RESERVED_WORDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(listed, table);
        assert!(RESERVED_WORDS.is_sorted(), "the list stays alphabetical");
        for word in RESERVED_WORDS {
            assert!(
                TokenKind::keyword(word).is_some_and(|kind| kind != TokenKind::AmsDigital),
                "{word} lexes as a keyword"
            );
        }
    }

    /// A statement calling a standard system task is accepted in analog
    /// code exactly when the task is listed; every other is refused there by
    /// name.
    #[test]
    fn analog_code_accepts_exactly_the_listed_system_tasks() {
        let names = crate::analog_tasks::STANDARD_SYSTEM_NAMES
            .iter()
            .chain(ANALOG_SYSTEM_TASKS)
            .copied()
            .collect::<BTreeSet<_>>();
        for name in names {
            let source = format!(
                "module dut(p, n);\ninout p, n;\nelectrical p, n;\nanalog begin\n{name}(1);\nI(p, n) <+ V(p, n);\nend\nendmodule\n"
            );
            let tokens = crate::Lexer::new(&source, crate::SourceId::new(0))
                .collect_tokens()
                .expect("the module lexes");
            let file = crate::Parser::new(&tokens)
                .parse()
                .expect("the module parses");
            let refused = match crate::SemanticAnalyzer::new().analyze(&file) {
                Ok(_) => false,
                Err(error) => {
                    // Unknown, unsupported here, or (`$stop`, `$fatal`)
                    // asking for simulation control the engine lacks.
                    let message = error.to_string();
                    message.contains(name)
                        && ["not supported", "Unknown system task", "not implemented"]
                            .iter()
                            .any(|refusal| message.contains(refusal))
                }
            };
            assert_eq!(
                refused,
                !ANALOG_SYSTEM_TASKS.contains(&name),
                "{name} as an analog statement"
            );
        }
    }

    #[test]
    fn every_builtin_header_is_listed_once() {
        let identities = BUILTIN_HEADER_NAMES
            .iter()
            .map(|name| crate::stdlib::builtin_include_document(name).map(|(identity, _)| identity))
            .collect::<Option<BTreeSet<_>>>()
            .expect("every listed name is a header");
        assert_eq!(identities.len(), BUILTIN_HEADER_NAMES.len());
        assert_eq!(
            identities.len(),
            crate::stdlib::BUILTIN_INCLUDE_DOCUMENT_COUNT
        );
        assert_eq!(builtin_headers().count(), BUILTIN_HEADER_NAMES.len());
    }

    #[test]
    fn builtins_are_named_once() {
        let mut seen = BTreeSet::new();
        for (name, _) in builtins() {
            assert!(seen.insert(name), "{name} is listed once");
        }
        assert_eq!(builtin_kind("ddt"), Some(BuiltinKind::Function));
        assert_eq!(
            builtin_kind("$temperature"),
            Some(BuiltinKind::SystemFunction)
        );
        assert_eq!(builtin_kind("$strobe"), Some(BuiltinKind::SystemTask));
        assert_eq!(builtin_kind("M_PI"), Some(BuiltinKind::Constant));
        assert_eq!(builtin_kind("gain"), None);
    }
}
