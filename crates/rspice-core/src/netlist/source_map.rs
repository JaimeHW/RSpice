use super::lexer::{Token, TokenKind, tokenize};
use super::{Netlist, SubcircuitDef};
use std::collections::HashSet;
use std::sync::OnceLock;

const BUILTIN_TRANSISTOR_LIB: &str = include_str!("../../models/spice/transistor.lib");
const BUILTIN_DIODE_LIB: &str = include_str!("../../models/spice/diode.lib");

/// Byte range on one physical source line.
///
/// Lines are 1-based; `start` and `end` are zero-based byte columns in the
/// original source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedNetlistReferenceKind {
    Model { element: String, name: String },
    Subcircuit { element: String, name: String },
    ControlLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNetlistReference {
    pub kind: ParsedNetlistReferenceKind,
    pub range: SourceRange,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedNetlistSourceMap {
    pub references: Vec<ParsedNetlistReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnknownReferenceKind {
    Model,
    Subcircuit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownReferenceDiagnostic {
    pub kind: UnknownReferenceKind,
    pub name: String,
    pub element: String,
    pub range: SourceRange,
    pub message: String,
}

impl Netlist {
    pub fn source_map(&self) -> ParsedNetlistSourceMap {
        ParsedNetlistSourceMap::from_netlist(self)
    }

    pub fn lint_unknown_references(&self) -> Vec<UnknownReferenceDiagnostic> {
        self.source_map().lint_unknown_references(self)
    }
}

impl ParsedNetlistSourceMap {
    pub fn from_netlist(netlist: &Netlist) -> Self {
        let Some(source) = netlist.source_text.as_deref() else {
            return Self::default();
        };

        let mut builder = SourceMapBuilder::default();
        builder.scan(source);
        builder.into_map()
    }

    pub fn lint_unknown_references(&self, netlist: &Netlist) -> Vec<UnknownReferenceDiagnostic> {
        let known_models = known_model_names(netlist);
        let known_subckts = known_subckt_names(netlist);
        let mut diagnostics = Vec::new();

        for reference in &self.references {
            match &reference.kind {
                ParsedNetlistReferenceKind::Model { element, name } => {
                    if !model_reference_is_known(name, reference.scope.as_deref(), &known_models) {
                        diagnostics.push(UnknownReferenceDiagnostic {
                            kind: UnknownReferenceKind::Model,
                            name: name.clone(),
                            element: element.clone(),
                            range: reference.range,
                            message: format!(
                                "Unknown model `{name}` referenced by element `{element}`"
                            ),
                        });
                    }
                }
                ParsedNetlistReferenceKind::Subcircuit { element, name } => {
                    if !subckt_reference_is_known(name, reference.scope.as_deref(), &known_subckts)
                    {
                        diagnostics.push(UnknownReferenceDiagnostic {
                            kind: UnknownReferenceKind::Subcircuit,
                            name: name.clone(),
                            element: element.clone(),
                            range: reference.range,
                            message: format!(
                                "Unknown subcircuit `{name}` referenced by element `{element}`"
                            ),
                        });
                    }
                }
                ParsedNetlistReferenceKind::ControlLine => {}
            }
        }

        diagnostics
    }
}

#[derive(Default)]
struct SourceMapBuilder {
    references: Vec<ParsedNetlistReference>,
    pending: Option<LogicalLine>,
    scope_stack: Vec<String>,
    in_control: bool,
}

impl SourceMapBuilder {
    fn scan(&mut self, source: &str) {
        for (idx, line) in source.lines().enumerate().skip(1) {
            let line_num = idx + 1;
            let comment_start = inline_comment_start(line);
            let stripped = &line[..comment_start];
            let trimmed = stripped.trim();

            if trimmed.is_empty() || trimmed.starts_with('*') {
                continue;
            }

            let upper = trimmed.to_ascii_uppercase();
            if upper.starts_with(".CONTROL") {
                self.flush_pending();
                self.in_control = true;
                self.references.push(ParsedNetlistReference {
                    kind: ParsedNetlistReferenceKind::ControlLine,
                    range: trimmed_range(line_num, line, stripped, trimmed),
                    scope: self.current_scope(),
                });
                continue;
            }

            if self.in_control {
                self.references.push(ParsedNetlistReference {
                    kind: ParsedNetlistReferenceKind::ControlLine,
                    range: trimmed_range(line_num, line, stripped, trimmed),
                    scope: self.current_scope(),
                });
                if upper.starts_with(".ENDC") {
                    self.in_control = false;
                }
                continue;
            }

            if let Some(rest) = trimmed.strip_prefix('+') {
                if let Some(pending) = &mut self.pending {
                    let trim_start = stripped.find(trimmed).unwrap_or(0);
                    pending.push_inserted_space(line_num, trim_start);
                    pending.push_slice(line_num, trim_start + 1, rest);
                }
                continue;
            }

            self.flush_pending();

            if dot_command_matches(trimmed, ".SUBCKT") {
                if let Some(local_name) = dot_command_argument(trimmed, 1) {
                    let local_name = local_name.to_ascii_uppercase();
                    let qualified = match self.current_scope() {
                        Some(scope) => format!("{scope}.{local_name}"),
                        None => local_name,
                    };
                    self.scope_stack.push(qualified);
                }
                continue;
            }

            if dot_command_matches(trimmed, ".ENDS") {
                self.scope_stack.pop();
                continue;
            }

            if trimmed.eq_ignore_ascii_case(".end") {
                break;
            }

            let trim_start = stripped.find(trimmed).unwrap_or(0);
            let mut logical = LogicalLine {
                scope: self.current_scope(),
                ..LogicalLine::default()
            };
            logical.push_slice(line_num, trim_start, trimmed);
            self.pending = Some(logical);
        }

        self.flush_pending();
    }

    fn flush_pending(&mut self) {
        let Some(logical) = self.pending.take() else {
            return;
        };
        self.references
            .extend(references_for_logical_line(&logical));
    }

    fn into_map(self) -> ParsedNetlistSourceMap {
        ParsedNetlistSourceMap {
            references: self.references,
        }
    }

    fn current_scope(&self) -> Option<String> {
        self.scope_stack.last().cloned()
    }
}

#[derive(Debug, Clone, Copy)]
struct ByteOrigin {
    line: usize,
    col: usize,
}

#[derive(Default)]
struct LogicalLine {
    text: String,
    origins: Vec<ByteOrigin>,
    scope: Option<String>,
}

impl LogicalLine {
    fn push_inserted_space(&mut self, line: usize, col: usize) {
        self.text.push(' ');
        self.origins.push(ByteOrigin { line, col });
    }

    fn push_slice(&mut self, line: usize, start_col: usize, text: &str) {
        self.text.push_str(text);
        for (offset, ch) in text.char_indices() {
            for byte_offset in 0..ch.len_utf8() {
                self.origins.push(ByteOrigin {
                    line,
                    col: start_col + offset + byte_offset,
                });
            }
        }
    }

    fn token_range(&self, token: &Token) -> Option<SourceRange> {
        if token.span.start >= token.span.end || token.span.end > self.origins.len() {
            return None;
        }
        let start = self.origins[token.span.start];
        let end = self.origins[token.span.end - 1];
        if start.line != end.line {
            return None;
        }
        Some(SourceRange {
            line: start.line,
            start: start.col,
            end: end.col + 1,
        })
    }
}

fn references_for_logical_line(logical: &LogicalLine) -> Vec<ParsedNetlistReference> {
    let Ok(tokens) = tokenize(&logical.text) else {
        return Vec::new();
    };
    let tokens = significant_tokens(&tokens);
    if tokens.is_empty() {
        return Vec::new();
    }

    let Some(first) = ident_text(tokens[0]) else {
        return Vec::new();
    };
    let element = first.to_ascii_uppercase();
    let first_char = element.chars().next().unwrap_or(' ');

    let reference_token = match first_char {
        'R' | 'C' | 'L' => passive_model_token(&tokens),
        'D' => nth_token(&tokens, 3),
        'J' | 'Z' => nth_token(&tokens, 4),
        'Q' => bjt_model_token(&tokens),
        'M' => mos_model_token(&tokens),
        'S' => nth_token(&tokens, 5),
        'W' => nth_token(&tokens, 4),
        'T' => named_model_value_token(&tokens),
        'O' | 'Y' => tline_model_token(&tokens),
        'P' => subckt_token(&tokens),
        'X' => subckt_token(&tokens),
        'A' => xspice_model_token(&tokens),
        _ => None,
    };

    let Some(token) = reference_token else {
        return Vec::new();
    };
    let Some(name) = ident_text(token).map(str::to_ascii_uppercase) else {
        return Vec::new();
    };
    let Some(range) = logical.token_range(token) else {
        return Vec::new();
    };

    let kind = if first_char == 'X' {
        ParsedNetlistReferenceKind::Subcircuit { element, name }
    } else {
        ParsedNetlistReferenceKind::Model { element, name }
    };
    vec![ParsedNetlistReference {
        kind,
        range,
        scope: logical.scope.clone(),
    }]
}

fn significant_tokens(tokens: &[Token]) -> Vec<&Token> {
    tokens
        .iter()
        .filter(|token| {
            !matches!(
                token.kind,
                TokenKind::Newline | TokenKind::Eof | TokenKind::Comma
            )
        })
        .collect()
}

fn nth_token<'a>(tokens: &'a [&'a Token], idx: usize) -> Option<&'a Token> {
    tokens.get(idx).copied()
}

fn passive_model_token<'a>(tokens: &'a [&'a Token]) -> Option<&'a Token> {
    named_model_value_token(tokens).or_else(|| {
        let candidate = nth_token(tokens, 3)?;
        ident_text(candidate)?;
        if matches!(
            tokens.get(4).map(|token| &token.kind),
            Some(TokenKind::Equals)
        ) {
            return None;
        }
        Some(candidate)
    })
}

fn bjt_model_token<'a>(tokens: &'a [&'a Token]) -> Option<&'a Token> {
    let first_tail = *tokens.get(4)?;
    if matches!(first_tail.kind, TokenKind::Number(_)) {
        return tokens.get(5).copied();
    }

    if ident_text(first_tail).is_some() {
        let Some(next) = tokens.get(5).copied() else {
            return Some(first_tail);
        };
        if let Some(next_ident) = ident_text(next) {
            if matches!(
                tokens.get(6).map(|token| &token.kind),
                Some(TokenKind::Equals)
            ) || next_ident.eq_ignore_ascii_case("OFF")
            {
                Some(first_tail)
            } else {
                Some(next)
            }
        } else {
            Some(first_tail)
        }
    } else {
        None
    }
}

fn mos_model_token<'a>(tokens: &'a [&'a Token]) -> Option<&'a Token> {
    let mut model = None;
    let mut idx = 5usize;
    while idx < tokens.len() {
        let token = tokens[idx];
        if ident_text(token).is_some() {
            if matches!(
                tokens.get(idx + 1).map(|next| &next.kind),
                Some(TokenKind::Equals)
            ) {
                break;
            }
            model = Some(token);
        }
        idx += 1;
    }
    model
}

fn tline_model_token<'a>(tokens: &'a [&'a Token]) -> Option<&'a Token> {
    named_model_value_token(tokens).or_else(|| {
        let candidate = nth_token(tokens, 5)?;
        ident_text(candidate)?;
        if matches!(
            tokens.get(6).map(|token| &token.kind),
            Some(TokenKind::Equals)
        ) {
            return None;
        }
        Some(candidate)
    })
}

fn xspice_model_token<'a>(tokens: &'a [&'a Token]) -> Option<&'a Token> {
    subckt_token(tokens)
}

fn subckt_token<'a>(tokens: &'a [&'a Token]) -> Option<&'a Token> {
    let mut param_start = tokens.len();
    for idx in 1..tokens.len() {
        if ident_text(tokens[idx]).is_some_and(|name| {
            name.eq_ignore_ascii_case("PARAMS") || name.eq_ignore_ascii_case("PARAMS:")
        }) || matches!(
            tokens.get(idx + 1).map(|token| &token.kind),
            Some(TokenKind::Equals)
        ) {
            param_start = idx;
            break;
        }
    }

    if param_start < 2 {
        None
    } else {
        tokens.get(param_start - 1).copied()
    }
}

fn named_model_value_token<'a>(tokens: &'a [&'a Token]) -> Option<&'a Token> {
    for idx in 1..tokens.len().saturating_sub(2) {
        if ident_text(tokens[idx]).is_some_and(|name| name.eq_ignore_ascii_case("MODEL"))
            && matches!(
                tokens.get(idx + 1).map(|token| &token.kind),
                Some(TokenKind::Equals)
            )
        {
            return tokens.get(idx + 2).copied();
        }
    }
    None
}

fn ident_text(token: &Token) -> Option<&str> {
    match &token.kind {
        TokenKind::Ident(text) => Some(text),
        _ => None,
    }
}

fn known_model_names(netlist: &Netlist) -> HashSet<String> {
    let mut names = builtin_model_names().clone();
    for model in &netlist.models {
        names.insert(model.name.to_ascii_uppercase());
    }
    for include in &netlist.veriloga_includes {
        if let Some(model_name) = &include.model_name {
            names.insert(model_name.to_ascii_uppercase());
        }
    }
    names
}

fn known_subckt_names(netlist: &Netlist) -> HashSet<String> {
    let mut names = HashSet::new();
    for subckt in &netlist.subcircuits {
        collect_subckt_names(subckt, &mut names);
    }
    for include in &netlist.veriloga_includes {
        if let Some(model_name) = &include.model_name {
            names.insert(model_name.to_ascii_uppercase());
        }
    }
    names
}

fn collect_subckt_names(subckt: &SubcircuitDef, names: &mut HashSet<String>) {
    names.insert(subckt.name.to_ascii_uppercase());
    for nested in &subckt.nested_subcircuits {
        collect_subckt_names(nested, names);
    }
}

fn model_reference_is_known(
    name: &str,
    scope: Option<&str>,
    known_models: &HashSet<String>,
) -> bool {
    scoped_reference_is_known(name, scope, "::", known_models)
}

fn subckt_reference_is_known(
    name: &str,
    scope: Option<&str>,
    known_subckts: &HashSet<String>,
) -> bool {
    scoped_reference_is_known(name, scope, ".", known_subckts)
}

fn scoped_reference_is_known(
    name: &str,
    scope: Option<&str>,
    separator: &str,
    known_names: &HashSet<String>,
) -> bool {
    let name = name.to_ascii_uppercase();
    if known_names.contains(&name) {
        return true;
    }

    let mut current = scope.map(str::to_string);
    while let Some(scope_name) = current {
        if known_names.contains(&format!("{scope_name}{separator}{name}")) {
            return true;
        }
        current = scope_name
            .rsplit_once('.')
            .map(|(parent, _)| parent.to_string());
    }

    false
}

fn builtin_model_names() -> &'static HashSet<String> {
    static NAMES: OnceLock<HashSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| {
        let mut names = HashSet::new();
        for model_type in [
            "D", "DIODE", "NPN", "PNP", "NMOS", "PMOS", "NJF", "PJF", "NMF", "PMF", "NHFET",
            "PHFET",
        ] {
            names.insert(model_type.to_string());
        }
        insert_model_names_from_library(BUILTIN_TRANSISTOR_LIB, &mut names);
        insert_model_names_from_library(BUILTIN_DIODE_LIB, &mut names);
        for model_name in crate::xspice::CodeModelRegistry::with_builtins().model_names() {
            names.insert(model_name.to_ascii_uppercase());
        }
        names
    })
}

fn insert_model_names_from_library(library: &str, names: &mut HashSet<String>) {
    for line in library.lines() {
        let trimmed = line.trim_start();
        if !dot_command_matches(trimmed, ".MODEL") {
            continue;
        }
        if let Some(name) = dot_command_argument(trimmed, 1) {
            names.insert(name.to_ascii_uppercase());
        }
    }
}

fn dot_command_matches(line: &str, command: &str) -> bool {
    line.split_whitespace()
        .next()
        .is_some_and(|token| token.eq_ignore_ascii_case(command))
}

fn dot_command_argument(line: &str, index: usize) -> Option<&str> {
    line.split_whitespace().nth(index)
}

fn inline_comment_start(line: &str) -> usize {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;

    for (idx, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' if in_single_quote || in_double_quote => escaped = true,
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            ';' | '$' if !in_single_quote && !in_double_quote => return idx,
            _ => {}
        }
    }
    line.len()
}

fn trimmed_range(line_num: usize, line: &str, stripped: &str, trimmed: &str) -> SourceRange {
    let start = stripped.find(trimmed).unwrap_or(0);
    SourceRange {
        line: line_num,
        start,
        end: start + trimmed.len().min(line.len().saturating_sub(start)),
    }
}

#[cfg(test)]
mod tests {
    use super::{ParsedNetlistReferenceKind, UnknownReferenceKind};
    use crate::netlist::Netlist;

    #[test]
    fn maps_references_and_lints_unknown_models_and_subckts() {
        let deck = "source map\n\
            D1 in 0 missing_d\n\
            M1 d g s b good_n\n\
            Mbad d g s b missing_m\n\
            Q1 c b e missing_q\n\
            J1 d g s missing_j\n\
            Xbad in out missing_cell\n\
            Xgood in out good_cell PARAMS: gain=2\n\
            .model good_n nmos\n\
            .subckt good_cell in out\n\
            D2 in out local_d\n\
            .model local_d d\n\
            .ends\n\
            .control\n\
            run\n\
            plot v(in)\n\
            .endc\n\
            .end\n";

        let netlist = Netlist::parse(deck).expect("deck parses");
        let map = netlist.source_map();

        let d_ref = map
            .references
            .iter()
            .find(|reference| {
                matches!(
                    &reference.kind,
                    ParsedNetlistReferenceKind::Model { element, name }
                        if element == "D1" && name == "MISSING_D"
                )
            })
            .expect("diode model reference is mapped");
        assert_eq!(d_ref.range.line, 2);
        assert_eq!(
            &deck.lines().nth(1).unwrap()[d_ref.range.start..d_ref.range.end],
            "missing_d"
        );

        assert!(map.references.iter().any(|reference| {
            matches!(
                &reference.kind,
                ParsedNetlistReferenceKind::Subcircuit { element, name }
                    if element == "XGOOD" && name == "GOOD_CELL"
            )
        }));
        assert!(
            map.references
                .iter()
                .any(|reference| matches!(reference.kind, ParsedNetlistReferenceKind::ControlLine))
        );

        let diagnostics = map.lint_unknown_references(&netlist);
        assert_eq!(
            diagnostics
                .iter()
                .map(|diagnostic| (
                    &diagnostic.kind,
                    diagnostic.name.as_str(),
                    diagnostic.range.line
                ))
                .collect::<Vec<_>>(),
            vec![
                (&UnknownReferenceKind::Model, "MISSING_D", 2),
                (&UnknownReferenceKind::Model, "MISSING_M", 4),
                (&UnknownReferenceKind::Model, "MISSING_Q", 5),
                (&UnknownReferenceKind::Model, "MISSING_J", 6),
                (&UnknownReferenceKind::Subcircuit, "MISSING_CELL", 7),
            ]
        );
    }

    #[test]
    fn lints_additional_model_bearing_element_forms() {
        let deck = "source map\n\
            R1 a b missing_r L=1u\n\
            C1 b 0 missing_c W=1u\n\
            L1 b c missing_l\n\
            S1 a b ctrl 0 missing_sw\n\
            VSENSE ctrl 0 0\n\
            W1 a b VSENSE missing_csw\n\
            O1 a b c d missing_tline\n\
            P1 p1 p2 p3 p4 missing_multiline\n\
            A1 in out missing_code gain=2\n\
            .end\n";

        let netlist = Netlist::parse(deck).expect("deck parses");
        let diagnostics = netlist.lint_unknown_references();
        let names = diagnostics
            .iter()
            .map(|diagnostic| diagnostic.name.as_str())
            .collect::<Vec<_>>();

        for expected in [
            "MISSING_R",
            "MISSING_C",
            "MISSING_L",
            "MISSING_SW",
            "MISSING_CSW",
            "MISSING_TLINE",
            "MISSING_MULTILINE",
            "MISSING_CODE",
        ] {
            assert!(names.contains(&expected), "{expected} was not linted");
        }
    }

    #[test]
    fn builtin_and_embedded_fallback_models_do_not_lint_unknown() {
        let deck = "source map\n\
            D1 a 0 1N4148\n\
            Q1 c b e 2N2222\n\
            M1 d g s b NMOS\n\
            J1 jd jg js NJF\n\
            Z1 zd zg zs NMF\n\
            .end\n";

        let netlist = Netlist::parse(deck).expect("deck parses");
        let diagnostics = netlist.lint_unknown_references();

        assert_eq!(diagnostics, Vec::new());
    }

    #[test]
    fn local_models_are_visible_only_in_their_subcircuit_scope() {
        let deck = "source map\n\
            .subckt cell in out\n\
            D1 in out local_d\n\
            .model local_d d\n\
            .ends\n\
            DTOP in out local_d\n\
            .end\n";

        let netlist = Netlist::parse(deck).expect("deck parses");
        let diagnostics = netlist.lint_unknown_references();

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].name, "LOCAL_D");
        assert_eq!(diagnostics[0].element, "DTOP");
        assert_eq!(diagnostics[0].range.line, 6);
    }

    #[test]
    fn veriloga_alias_is_known_as_external_subcircuit() {
        let deck = "source map\n\
            .va \"amp.va\" va_amp\n\
            X1 in out va_amp\n\
            .end\n";

        let netlist = Netlist::parse(deck).expect("deck parses");
        let diagnostics = netlist.lint_unknown_references();

        assert_eq!(diagnostics, Vec::new());
    }

    #[test]
    fn path_parse_source_map_stays_on_authored_deck_lines() {
        let root = std::env::temp_dir().join(format!(
            "rspice_source_map_{}_{}",
            std::process::id(),
            "path_parse_source_map"
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("temp dir");
        std::fs::write(
            root.join("models.inc"),
            "* included models\n* line padding\n.model inc_d d\n",
        )
        .expect("include file");

        let deck = "source map\n.include \"models.inc\"\nD1 in 0 inc_d\n.end\n";
        let deck_path = root.join("top.cir");
        let netlist = Netlist::parse_with_path(deck, &deck_path).expect("deck parses");
        let map = netlist.source_map();
        let d_ref = map
            .references
            .iter()
            .find(|reference| {
                matches!(
                    &reference.kind,
                    ParsedNetlistReferenceKind::Model { element, name }
                        if element == "D1" && name == "INC_D"
                )
            })
            .expect("diode reference is mapped");

        assert_eq!(d_ref.range.line, 3);
        assert_eq!(map.lint_unknown_references(&netlist), Vec::new());

        let _ = std::fs::remove_dir_all(root);
    }
}

// Lightweight source mapping for editor diagnostics.

use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceKind {
    Model,
    Subcircuit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistDefinition {
    pub name: String,
    pub span: Range<usize>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistReference {
    pub name: String,
    pub kind: ReferenceKind,
    pub span: Range<usize>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NetlistSourceMap {
    pub model_defs: Vec<NetlistDefinition>,
    pub subckt_defs: Vec<NetlistDefinition>,
    pub references: Vec<NetlistReference>,
}

pub fn source_map_for_editor(buffer: &str) -> NetlistSourceMap {
    let mut map = NetlistSourceMap::default();
    let mut logical = None;
    let mut scopes = Vec::new();
    let mut offset = 0usize;

    for (line_index, physical) in buffer.split_inclusive('\n').enumerate() {
        if line_index == 0 {
            offset += physical.len();
            continue;
        }

        let raw = physical.strip_suffix('\n').unwrap_or(physical);
        let raw = raw.strip_suffix('\r').unwrap_or(raw);
        let code_end = find_inline_comment(raw).unwrap_or(raw.len());
        let code = &raw[..code_end];
        let trimmed = code.trim_start();
        let lead = code.len() - trimmed.len();

        if trimmed.is_empty() || trimmed.starts_with('*') {
            flush_logical_line(&mut logical, &mut scopes, &mut map);
            offset += physical.len();
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix('+') {
            if let Some(line) = logical.as_mut() {
                let continued = rest.trim_start();
                let continued_lead = rest.len() - continued.len();
                line.push_join_space();
                line.push_segment(continued, offset + lead + 1 + continued_lead);
            }
        } else {
            flush_logical_line(&mut logical, &mut scopes, &mut map);
            let mut line = MappedLine::default();
            line.push_segment(trimmed, offset + lead);
            logical = Some(line);
        }

        offset += physical.len();
    }

    if !buffer.ends_with('\n') && !buffer.is_empty() {
        // `split_inclusive` already yielded the final unterminated line.
    }
    flush_logical_line(&mut logical, &mut scopes, &mut map);
    map
}

fn flush_logical_line(
    line: &mut Option<MappedLine>,
    scopes: &mut Vec<String>,
    map: &mut NetlistSourceMap,
) {
    if let Some(line) = line.take() {
        process_logical_line(&line, scopes, map);
    }
}

#[derive(Default)]
struct MappedLine {
    text: String,
    byte_offsets: Vec<usize>,
}

impl MappedLine {
    fn push_join_space(&mut self) {
        if !self.text.is_empty()
            && !self
                .text
                .chars()
                .next_back()
                .is_some_and(char::is_whitespace)
        {
            self.text.push(' ');
            let offset = self.byte_offsets.last().copied().unwrap_or(0);
            self.byte_offsets.push(offset);
        }
    }

    fn push_segment(&mut self, segment: &str, original_start: usize) {
        self.text.push_str(segment);
        self.byte_offsets
            .extend((0..segment.len()).map(|idx| original_start + idx));
    }

    fn original_span(&self, span: Range<usize>) -> Option<Range<usize>> {
        if span.is_empty() || span.end > self.byte_offsets.len() {
            return None;
        }
        let start = self.byte_offsets[span.start];
        let end = self.byte_offsets[span.end - 1] + 1;
        Some(start..end)
    }
}

fn process_logical_line(line: &MappedLine, scopes: &mut Vec<String>, map: &mut NetlistSourceMap) {
    let tokens = token_spans(&line.text);
    let Some(first) = tokens.first() else {
        return;
    };
    let head = &line.text[first.clone()];
    let scope = current_scope(scopes);

    if head.eq_ignore_ascii_case(".model") {
        if let Some(name) = tokens.get(1)
            && let Some(span) = line.original_span(name.clone())
        {
            map.model_defs.push(NetlistDefinition {
                name: line.text[name.clone()].to_string(),
                span,
                scope,
            });
        }
        return;
    }

    if head.eq_ignore_ascii_case(".subckt") {
        if let Some(name) = tokens.get(1)
            && let Some(span) = line.original_span(name.clone())
        {
            let name_text = line.text[name.clone()].to_string();
            map.subckt_defs.push(NetlistDefinition {
                name: name_text.clone(),
                span,
                scope: scope.clone(),
            });
            scopes.push(match scope {
                Some(parent) => format!("{parent}.{name_text}"),
                None => name_text,
            });
        }
        return;
    }

    if head.eq_ignore_ascii_case(".ends") {
        scopes.pop();
        return;
    }

    if let Some(reference) = reference_from_element_line(line, &tokens, scope) {
        map.references.push(reference);
    }
}

fn current_scope(scopes: &[String]) -> Option<String> {
    scopes.last().cloned()
}

fn reference_from_element_line(
    line: &MappedLine,
    tokens: &[Range<usize>],
    scope: Option<String>,
) -> Option<NetlistReference> {
    let first = line.text[tokens.first()?.clone()]
        .chars()
        .next()?
        .to_ascii_uppercase();
    match first {
        'R' | 'C' | 'L' => passive_model_reference(line, tokens, scope),
        'A' => xspice_model_reference(line, tokens, scope),
        'D' => fixed_token_reference(line, tokens, 3, ReferenceKind::Model, scope),
        'J' | 'Z' | 'W' => fixed_token_reference(line, tokens, 4, ReferenceKind::Model, scope),
        'S' => fixed_token_reference(line, tokens, 5, ReferenceKind::Model, scope),
        'O' | 'Y' => tline_model_reference(line, tokens, 5, scope),
        'P' => p_line_model_reference(line, tokens, scope),
        'Q' => tail_reference(line, tokens, 4, ReferenceKind::Model, scope),
        'M' => tail_reference(line, tokens, 5, ReferenceKind::Model, scope),
        'X' => tail_reference(line, tokens, 1, ReferenceKind::Subcircuit, scope),
        _ => None,
    }
}

fn passive_model_reference(
    line: &MappedLine,
    tokens: &[Range<usize>],
    scope: Option<String>,
) -> Option<NetlistReference> {
    model_assignment_span(line, tokens, 3)
        .and_then(|span| reference_from_span(line, span, ReferenceKind::Model, scope))
}

fn tline_model_reference(
    line: &MappedLine,
    tokens: &[Range<usize>],
    start: usize,
    scope: Option<String>,
) -> Option<NetlistReference> {
    if let Some(span) = model_assignment_span(line, tokens, start) {
        return reference_from_span(line, span, ReferenceKind::Model, scope);
    }

    tokens
        .iter()
        .skip(start)
        .find(|span| {
            let token = &line.text[(*span).clone()];
            !is_tline_key_or_value(token) && !looks_numeric(token)
        })
        .cloned()
        .and_then(|span| reference_from_span(line, span, ReferenceKind::Model, scope))
}

fn p_line_model_reference(
    line: &MappedLine,
    tokens: &[Range<usize>],
    scope: Option<String>,
) -> Option<NetlistReference> {
    let span = tokens.last()?.clone();
    let token = &line.text[span.clone()];
    if tokens.len() >= 4 && !looks_numeric(token) && !token.contains('=') {
        reference_from_span(line, span, ReferenceKind::Model, scope)
    } else {
        None
    }
}

fn xspice_model_reference(
    line: &MappedLine,
    tokens: &[Range<usize>],
    scope: Option<String>,
) -> Option<NetlistReference> {
    let mut candidate = None;
    for span in tokens.iter().skip(1) {
        if line.text[span.clone()].contains('=') {
            break;
        }
        let token = &line.text[(*span).clone()];
        if !token.eq_ignore_ascii_case("null") && !token.starts_with('[') && !token.starts_with('%')
        {
            candidate = Some(span.clone());
        }
    }
    candidate.and_then(|span| reference_from_span(line, span, ReferenceKind::Model, scope))
}

fn model_assignment_span(
    line: &MappedLine,
    tokens: &[Range<usize>],
    start: usize,
) -> Option<Range<usize>> {
    for (idx, span) in tokens.iter().enumerate().skip(start) {
        let token = &line.text[span.clone()];
        if let Some((key, value)) = token.split_once('=')
            && key.eq_ignore_ascii_case("model")
            && !value.is_empty()
        {
            let value_start = span.start + key.len() + 1;
            return Some(value_start..span.end);
        }
        if token.eq_ignore_ascii_case("model") {
            if let Some(eq) = tokens.get(idx + 1)
                && &line.text[eq.clone()] == "="
            {
                return tokens.get(idx + 2).cloned();
            }
            return tokens.get(idx + 1).cloned();
        }
    }
    None
}

fn fixed_token_reference(
    line: &MappedLine,
    tokens: &[Range<usize>],
    index: usize,
    kind: ReferenceKind,
    scope: Option<String>,
) -> Option<NetlistReference> {
    let span = tokens.get(index)?.clone();
    reference_from_span(line, span, kind, scope)
}

fn tail_reference(
    line: &MappedLine,
    tokens: &[Range<usize>],
    start: usize,
    kind: ReferenceKind,
    scope: Option<String>,
) -> Option<NetlistReference> {
    let boundary = tokens
        .iter()
        .enumerate()
        .skip(start)
        .find(|(_, span)| is_param_boundary(&line.text[(*span).clone()]))
        .map(|(idx, _)| idx)
        .unwrap_or(tokens.len());
    if boundary <= start {
        return None;
    }
    reference_from_span(line, tokens[boundary - 1].clone(), kind, scope)
}

fn reference_from_span(
    line: &MappedLine,
    span: Range<usize>,
    kind: ReferenceKind,
    scope: Option<String>,
) -> Option<NetlistReference> {
    if line.text[span.clone()].contains('=') {
        return None;
    }
    let original = line.original_span(span.clone())?;
    Some(NetlistReference {
        name: line.text[span].to_string(),
        kind,
        span: original,
        scope,
    })
}

fn token_spans(line: &str) -> Vec<Range<usize>> {
    let mut spans = Vec::new();
    let mut start = None;
    for (byte, ch) in line.char_indices() {
        if ch.is_whitespace() || ch == ',' {
            if let Some(token_start) = start.take() {
                spans.push(token_start..byte);
            }
        } else if start.is_none() {
            start = Some(byte);
        }
    }
    if let Some(token_start) = start {
        spans.push(token_start..line.len());
    }
    spans
}

fn is_param_boundary(token: &str) -> bool {
    token.eq_ignore_ascii_case("params")
        || token.eq_ignore_ascii_case("params:")
        || token.eq_ignore_ascii_case("off")
        || token.contains('=')
}

fn is_tline_key_or_value(token: &str) -> bool {
    token.contains('=')
        || matches!(
            token.to_ascii_uppercase().as_str(),
            "Z0" | "ZO" | "TD" | "F" | "FREQ" | "NL" | "MODEL"
        )
}

fn looks_numeric(token: &str) -> bool {
    token
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_digit() || ch == '+' || ch == '-' || ch == '.')
}

fn find_inline_comment(line: &str) -> Option<usize> {
    let semicolon = line.find(';');
    let dollar = line
        .char_indices()
        .find(|(idx, ch)| {
            *ch == '$'
                && *idx > 0
                && line[..*idx]
                    .chars()
                    .next_back()
                    .is_some_and(char::is_whitespace)
        })
        .map(|(idx, _)| idx);
    match (semicolon, dollar) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    }
}

#[cfg(test)]
mod editor_tests {
    use super::*;

    #[test]
    fn source_map_records_mos_model_reference_span() {
        let src = "deck\nM1 d g s b nch W=1u L=1u\n.model nch nmos\n.end\n";

        let map = source_map_for_editor(src);

        let reference = map
            .references
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case("nch"))
            .unwrap();
        assert_eq!(reference.kind, ReferenceKind::Model);
        assert_eq!(&src[reference.span.clone()], "nch");
        assert_eq!(&src[map.model_defs[0].span.clone()], "nch");
        assert_eq!(map.model_defs[0].scope, None);
    }

    #[test]
    fn source_map_records_subckt_reference_span() {
        let src = "deck\nX1 a b inv\n.subckt inv a b\n.ends\n.end\n";

        let map = source_map_for_editor(src);

        let reference = map
            .references
            .iter()
            .find(|r| r.kind == ReferenceKind::Subcircuit)
            .unwrap();
        assert_eq!(&src[reference.span.clone()], "inv");
        assert_eq!(&src[map.subckt_defs[0].span.clone()], "inv");
        assert_eq!(map.subckt_defs[0].scope, None);
    }

    #[test]
    fn source_map_maps_model_reference_on_continuation_line() {
        let src = "deck\nM1 d g s b\n+ nch W=1u L=1u\n.model nch nmos\n.end\n";

        let map = source_map_for_editor(src);

        let reference = map
            .references
            .iter()
            .find(|r| r.kind == ReferenceKind::Model)
            .unwrap();
        assert_eq!(&src[reference.span.clone()], "nch");
        assert!(reference.span.start > src.find("+ ").unwrap());
    }

    #[test]
    fn source_map_scopes_local_model_definitions_and_references() {
        let src = "deck\n.subckt amp in out\nM1 out in 0 0 nch\n.model nch nmos\n.ends\n.subckt buf in out\nM1 out in 0 0 nch\n.ends\n.end\n";

        let map = source_map_for_editor(src);

        let def = map
            .model_defs
            .iter()
            .find(|definition| definition.name == "nch")
            .unwrap();
        assert_eq!(def.scope.as_deref(), Some("amp"));

        let refs = map
            .references
            .iter()
            .filter(|reference| reference.kind == ReferenceKind::Model)
            .collect::<Vec<_>>();
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].scope.as_deref(), Some("amp"));
        assert_eq!(refs[1].scope.as_deref(), Some("buf"));
    }

    #[test]
    fn source_map_records_passive_model_assignment_span() {
        let src = "deck\nR1 in out MODEL=rmod L=1u W=2u\n.model rmod r\n.end\n";

        let map = source_map_for_editor(src);

        let reference = map.references.iter().find(|r| r.name == "rmod").unwrap();
        assert_eq!(reference.kind, ReferenceKind::Model);
        assert_eq!(&src[reference.span.clone()], "rmod");
    }

    #[test]
    fn source_map_records_switch_and_legacy_tline_model_spans() {
        let src = "deck\nS1 a b ctl 0 swmod\nO1 a b c d omod z0=50\nP1 a b c d pmod\n.model swmod sw\n.model omod txl\n.model pmod txl\n.end\n";

        let map = source_map_for_editor(src);
        let refs = map
            .references
            .iter()
            .filter(|r| r.kind == ReferenceKind::Model)
            .map(|r| (&r.name, &src[r.span.clone()]))
            .collect::<Vec<_>>();

        assert!(refs.contains(&(&"swmod".to_string(), "swmod")));
        assert!(refs.contains(&(&"omod".to_string(), "omod")));
        assert!(refs.contains(&(&"pmod".to_string(), "pmod")));
    }

    #[test]
    fn source_map_records_xspice_model_span() {
        let src = "deck\nA1 in out gain gain=2\n.model gain xspice\n.end\n";

        let map = source_map_for_editor(src);

        let reference = map.references.iter().find(|r| r.name == "gain").unwrap();
        assert_eq!(reference.kind, ReferenceKind::Model);
        assert_eq!(&src[reference.span.clone()], "gain");
    }
}
