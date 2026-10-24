//! The reserved-word set each region of source text is lexed under (IEEE
//! 1800-2017 section 22.14).
//!
//! One compiler reads both languages, so the difference between a Verilog-AMS
//! file and a SystemVerilog file is which words are keywords. A file named
//! `*.sv` or `*.svh` is SystemVerilog 2017 and every other file is
//! Verilog-AMS; `` `begin_keywords `` and `` `end_keywords `` change the set
//! for the text between them.
//!
//! The preprocessor owns the bookkeeping: it knows where each file starts and
//! ends and which directives are live, so it keeps the stack and writes one
//! [`KEYWORD_MARKER`] into the expanded text wherever the set in force
//! changes. The marker is absolute (it names the set, not a push or a pop),
//! so the lexer needs no stack of its own and the expanded text says
//! everything about its own reading, which a replay of retained text needs.

use crate::lexer::TokenKind;

/// The marker the preprocessor writes where the keyword set changes:
/// `` `__rspice_keywords "<specifier>" "<cause>" ``. The cause names what
/// changed it (a file's start or end, or a directive), for the refusal of a
/// change inside a design element.
pub(crate) const KEYWORD_MARKER: &str = "__rspice_keywords";

/// A reserved-word set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum KeywordSet {
    /// Verilog-AMS LRM 2.4 over IEEE 1364-2005: today's table.
    #[default]
    VerilogAms,
    /// IEEE 1800-2017 Annex B.
    SystemVerilog2017,
}

impl KeywordSet {
    /// The specifier a marker writes for this set.
    pub(crate) const fn specifier(self) -> &'static str {
        match self {
            Self::VerilogAms => "1364-2005",
            Self::SystemVerilog2017 => "1800-2017",
        }
    }

    /// The language a design element begun under this set is written in.
    pub(crate) const fn language(self) -> crate::ast::SourceLanguage {
        match self {
            Self::VerilogAms => crate::ast::SourceLanguage::VerilogAms,
            Self::SystemVerilog2017 => crate::ast::SourceLanguage::SystemVerilog,
        }
    }

    /// The set a `` `begin_keywords `` version specifier selects.
    ///
    /// Every IEEE 1364 specifier keeps the Verilog-AMS table: Verilog-AMS is a
    /// superset of 1364, and reading `"1364-2005"` literally would drop the
    /// analog words from the files that write it. So does a Verilog-AMS
    /// specifier (Icarus Verilog reads `"VAMS-2.3"`). The other 1800 versions
    /// have tables of their own, which this compiler does not carry yet, and
    /// anything else names no set at all.
    pub(crate) fn from_specifier(specifier: &str) -> Result<Self, String> {
        match specifier {
            "1364-1995" | "1364-2001" | "1364-2001-noconfig" | "1364-2005" | "VAMS-2.3"
            | "VAMS-2.4" => Ok(Self::VerilogAms),
            "1800-2017" => Ok(Self::SystemVerilog2017),
            "1800-2005" | "1800-2009" | "1800-2012" => Err(format!(
                "`begin_keywords \"{specifier}\" selects the IEEE {specifier} reserved words, \
                 which this compiler does not carry; it reads SystemVerilog under \
                 \"1800-2017\" only"
            )),
            _ => Err(format!(
                "`begin_keywords \"{specifier}\" names no keyword set; IEEE 1800-2017 section \
                 22.14 defines \"1364-1995\", \"1364-2001\", \"1364-2001-noconfig\", \
                 \"1364-2005\", \"1800-2005\", \"1800-2009\", \"1800-2012\" and \"1800-2017\""
            )),
        }
    }

    /// The set a marker's specifier names.
    pub(crate) fn from_marker(specifier: &str) -> Option<Self> {
        match specifier {
            "1364-2005" => Some(Self::VerilogAms),
            "1800-2017" => Some(Self::SystemVerilog2017),
            _ => None,
        }
    }

    /// The set a file of this name is read under before any directive.
    pub(crate) fn for_path(path: &std::path::Path) -> Self {
        match path.extension().and_then(|extension| extension.to_str()) {
            Some(extension)
                if extension.eq_ignore_ascii_case("sv")
                    || extension.eq_ignore_ascii_case("svh") =>
            {
                Self::SystemVerilog2017
            }
            _ => Self::VerilogAms,
        }
    }

    /// The token kind `word` lexes as under this set, if it is reserved.
    pub(crate) fn keyword(self, word: &str) -> Option<TokenKind> {
        match self {
            Self::VerilogAms => TokenKind::keyword(word),
            Self::SystemVerilog2017 => systemverilog_keyword(word),
        }
    }
}

/// IEEE 1800-2017 Annex B, read through this compiler's token kinds.
///
/// A word 1364-2005 already reserves keeps the kind the Verilog-AMS table
/// gives it (or stays an identifier the parser reads by its text, as the gate
/// names do). The SystemVerilog words of the supported subset have kinds of
/// their own; every other SystemVerilog word is [`TokenKind::SvReserved`], so
/// it can never be taken for a name and the parser refuses it by name. The
/// Verilog-AMS words that neither standard reserves (`analog`, `electrical`,
/// `wreal`, ...) are ordinary identifiers here.
fn systemverilog_keyword(word: &str) -> Option<TokenKind> {
    match word {
        "package" => return Some(TokenKind::Package),
        "endpackage" => return Some(TokenKind::Endpackage),
        "import" => return Some(TokenKind::Import),
        "nettype" => return Some(TokenKind::Nettype),
        _ => {}
    }
    if SYSTEMVERILOG_ONLY.binary_search(&word).is_ok() {
        return Some(TokenKind::SvReserved);
    }
    if IEEE_1364_2005.binary_search(&word).is_ok() {
        return TokenKind::keyword(word);
    }
    None
}

/// The IEEE 1364-2005 reserved words (its Annex B), sorted.
const IEEE_1364_2005: [&str; 124] = [
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
    "pulsestyle_ondetect",
    "pulsestyle_onevent",
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

/// The IEEE 1800-2017 reserved words that IEEE 1364-2005 does not reserve,
/// sorted.
const SYSTEMVERILOG_ONLY: [&str; 124] = [
    "accept_on",
    "alias",
    "always_comb",
    "always_ff",
    "always_latch",
    "assert",
    "assume",
    "before",
    "bind",
    "bins",
    "binsof",
    "bit",
    "break",
    "byte",
    "chandle",
    "checker",
    "class",
    "clocking",
    "const",
    "constraint",
    "context",
    "continue",
    "cover",
    "covergroup",
    "coverpoint",
    "cross",
    "dist",
    "do",
    "endchecker",
    "endclass",
    "endclocking",
    "endgroup",
    "endinterface",
    "endpackage",
    "endprogram",
    "endproperty",
    "endsequence",
    "enum",
    "eventually",
    "expect",
    "export",
    "extends",
    "extern",
    "final",
    "first_match",
    "foreach",
    "forkjoin",
    "global",
    "iff",
    "ignore_bins",
    "illegal_bins",
    "implements",
    "implies",
    "import",
    "inside",
    "int",
    "interconnect",
    "interface",
    "intersect",
    "join_any",
    "join_none",
    "let",
    "local",
    "logic",
    "longint",
    "matches",
    "modport",
    "nettype",
    "new",
    "nexttime",
    "null",
    "package",
    "packed",
    "priority",
    "program",
    "property",
    "protected",
    "pure",
    "rand",
    "randc",
    "randcase",
    "randsequence",
    "ref",
    "reject_on",
    "restrict",
    "return",
    "s_always",
    "s_eventually",
    "s_nexttime",
    "s_until",
    "s_until_with",
    "sequence",
    "shortint",
    "shortreal",
    "soft",
    "solve",
    "static",
    "string",
    "strong",
    "struct",
    "super",
    "sync_accept_on",
    "sync_reject_on",
    "tagged",
    "this",
    "throughout",
    "timeprecision",
    "timeunit",
    "type",
    "typedef",
    "union",
    "unique",
    "unique0",
    "until",
    "until_with",
    "untyped",
    "var",
    "virtual",
    "void",
    "wait_order",
    "weak",
    "wildcard",
    "with",
    "within",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tables_are_sorted_and_disjoint() {
        assert!(IEEE_1364_2005.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(SYSTEMVERILOG_ONLY.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(
            SYSTEMVERILOG_ONLY
                .iter()
                .all(|word| IEEE_1364_2005.binary_search(word).is_err())
        );
    }

    #[test]
    fn verilog_ams_words_are_names_in_systemverilog() {
        let sv = KeywordSet::SystemVerilog2017;
        for word in [
            "analog",
            "electrical",
            "wreal",
            "discipline",
            "ground",
            "branch",
        ] {
            assert_eq!(sv.keyword(word), None, "{word}");
        }
        assert_eq!(sv.keyword("module"), Some(TokenKind::Module));
        assert_eq!(sv.keyword("primitive"), Some(TokenKind::AmsDigital));
        assert_eq!(sv.keyword("and"), None);
        assert_eq!(sv.keyword("cross"), Some(TokenKind::SvReserved));
        assert_eq!(sv.keyword("logic"), Some(TokenKind::SvReserved));
        assert_eq!(sv.keyword("nettype"), Some(TokenKind::Nettype));
        assert_eq!(
            KeywordSet::VerilogAms.keyword("analog"),
            Some(TokenKind::Analog)
        );
        assert_eq!(KeywordSet::VerilogAms.keyword("package"), None);
    }

    #[test]
    fn specifiers_select_a_set_or_are_refused() {
        for specifier in ["1364-1995", "1364-2001", "1364-2001-noconfig", "1364-2005"] {
            assert_eq!(
                KeywordSet::from_specifier(specifier),
                Ok(KeywordSet::VerilogAms)
            );
        }
        assert_eq!(
            KeywordSet::from_specifier("1800-2017"),
            Ok(KeywordSet::SystemVerilog2017)
        );
        for specifier in ["1800-2005", "1800-2009", "1800-2012"] {
            let refusal = KeywordSet::from_specifier(specifier).unwrap_err();
            assert!(refusal.contains("does not carry"), "{refusal}");
        }
        assert!(
            KeywordSet::from_specifier("1364-2008")
                .unwrap_err()
                .contains("names no keyword set")
        );
    }
}
