use super::lexer::{Token, TokenKind, tokenize};
use super::model_resolution::builtin_model_names;
use super::{Netlist, SubcircuitDef};
use std::collections::HashSet;

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
        builder.scan(
            source,
            netlist.params.expression_dialect() != crate::config::ExpressionDialect::Xyce,
        );
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
    fn scan(&mut self, source: &str, allow_dollar_comments: bool) {
        for (idx, line) in source.lines().enumerate().skip(1) {
            let line_num = idx + 1;
            let comment_start = inline_comment_start(line, allow_dollar_comments);
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
        let name = ident_text(candidate)?;
        if super::lexer::parse_spice_value_complete(name).is_ok() {
            return None;
        }
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

/// Names a model reference in the deck text may carry without being unknown.
///
/// A text reference does not say which device family names it, so the
/// card-free names are taken as one union. `unresolved_device_model_references`
/// applies the per-family form to the instantiated topology, where the family
/// is known.
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
    for subckt in super::foundation_subcircuits() {
        collect_subckt_names(subckt, &mut names);
    }
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

fn dot_command_matches(line: &str, command: &str) -> bool {
    line.split_whitespace()
        .next()
        .is_some_and(|token| token.eq_ignore_ascii_case(command))
}

fn dot_command_argument(line: &str, index: usize) -> Option<&str> {
    line.split_whitespace().nth(index)
}

fn inline_comment_start(line: &str, allow_dollar_comments: bool) -> usize {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;

    let mut previous = None;
    let mut chars = line.char_indices().peekable();
    while let Some((idx, ch)) = chars.next() {
        if escaped {
            escaped = false;
            previous = Some(ch);
            continue;
        }

        match ch {
            '\\' if in_single_quote || in_double_quote => escaped = true,
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            ';' if !in_single_quote && !in_double_quote => return idx,
            '$' if allow_dollar_comments && !in_single_quote && !in_double_quote => {
                if previous.is_none_or(char::is_whitespace)
                    && chars.peek().is_none_or(|(_, next)| next.is_whitespace())
                {
                    return idx;
                }
            }
            _ => {}
        }
        previous = Some(ch);
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
    fn passive_engineering_values_are_not_model_references() {
        let deck = "source map\n\
            R1 a b 10k\n\
            C1 b 0 2u\n\
            L1 b c 3m\n\
            .end\n";

        let netlist = Netlist::parse(deck).expect("deck parses");
        assert_eq!(netlist.lint_unknown_references(), Vec::new());
    }

    #[test]
    fn builtin_and_embedded_fallback_models_do_not_lint_unknown() {
        let deck = "source map\n\
            D1 a 0 RSPICE_DIODE\n\
            Q1 c b e RSPICE_NPN\n\
            M1 d g s b RSPICE_NMOS\n\
            J1 jd jg js RSPICE_NJFET\n\
            Z1 zd zg zs NMF\n\
            .end\n";

        let netlist = Netlist::parse(deck).expect("deck parses");
        let diagnostics = netlist.lint_unknown_references();

        assert_eq!(diagnostics, Vec::new());
    }

    #[test]
    fn bare_diode_and_bjt_type_names_lint_unknown() {
        // A bare `D`/`NPN` is a card's type, never a model the builder can
        // resolve. Foundation JFET and MOSFET card names are real fallbacks.
        let deck = "source map\n\
            D1 a 0 d\n\
            Q1 c b e npn\n\
            J1 jd jg js RSPICE_NJFET\n\
            M1 md mg ms mb RSPICE_NMOS\n\
            .end\n";

        let netlist = Netlist::parse(deck).expect("deck parses");
        let diagnostics = netlist.lint_unknown_references();

        assert_eq!(
            diagnostics
                .iter()
                .map(|diagnostic| diagnostic.name.as_str())
                .collect::<Vec<_>>(),
            vec!["D", "NPN"]
        );
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

/// What an evaluatable span of the editor buffer denotes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetlistValueKind {
    /// The name a `.PARAM`-family card or a `.SUBCKT ... PARAMS:` list binds.
    ParamDefinition,
    /// A parameter name read inside an expression or a value slot.
    ParamReference,
    /// A braced, quoted, or bare expression that is not a single name.
    Expression,
}

/// One span of the editor buffer whose value can be evaluated in place.
///
/// `span` is always contained in a single physical line, so a hover can be
/// resolved from a byte offset. An expression continued across `+` lines
/// contributes one span per physical line, each carrying the whole joined
/// `expression`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistValueSpan {
    /// What the span denotes.
    pub kind: NetlistValueKind,
    /// Byte range of the span in the editor buffer.
    pub span: Range<usize>,
    /// Bound or referenced parameter name; empty for [`NetlistValueKind::Expression`].
    pub name: String,
    /// Expression text to evaluate, joined across continuation lines and with
    /// the outer braces or quotes removed.
    pub expression: String,
    /// One-based physical line `span` starts on.
    pub line: usize,
    /// Enclosing `.SUBCKT` scope, `None` at the top level.
    pub scope: Option<String>,
}

/// A `.FUNC` definition the editor's evaluator needs before it can resolve an
/// expression that calls it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistFunctionDefinition {
    /// Function name as written.
    pub name: String,
    /// Formal argument names, in order.
    pub arguments: Vec<String>,
    /// Body expression, braces or quotes removed.
    pub body: String,
    /// One-based physical line the definition starts on.
    pub line: usize,
    /// Enclosing `.SUBCKT` scope, `None` at the top level.
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NetlistSourceMap {
    pub model_defs: Vec<NetlistDefinition>,
    pub subckt_defs: Vec<NetlistDefinition>,
    pub references: Vec<NetlistReference>,
    /// Parameter definitions, references, and expressions, in source order.
    pub value_spans: Vec<NetlistValueSpan>,
    /// `.FUNC` definitions, in source order.
    pub functions: Vec<NetlistFunctionDefinition>,
}

impl NetlistSourceMap {
    /// The parameter environment an editor reader sees at `line` inside
    /// `scope`.
    ///
    /// Definitions are applied in source order up to and including `line`, so
    /// a later redefinition shadows an earlier one exactly the way the
    /// engine's default [`ParameterRedefinitionPolicy::UseLast`] does, and a
    /// definition further down the deck is not yet visible. Top-level
    /// definitions are visible inside every subcircuit; a subcircuit's own
    /// definitions are visible only in that subcircuit and its descendants.
    ///
    /// Statistical operators evaluate at their nominal value
    /// ([`StatisticalParamMode::Nominal`]): a reader hovering a parameter is
    /// asking what the deck says, not what one seeded draw of a Monte Carlo
    /// family produced.
    ///
    /// [`ParameterRedefinitionPolicy::UseLast`]: super::ParameterRedefinitionPolicy::UseLast
    /// [`StatisticalParamMode::Nominal`]: super::StatisticalParamMode::Nominal
    pub fn param_context_at(&self, line: usize, scope: Option<&str>) -> super::ParamContext {
        let mut context = super::ParamContext::new();
        context.set_statistical_mode(super::StatisticalParamMode::Nominal);
        for function in &self.functions {
            if function.line <= line && scope_encloses(function.scope.as_deref(), scope) {
                context.define_function(&function.name, function.arguments.clone(), &function.body);
            }
        }
        for definition in &self.value_spans {
            if definition.kind != NetlistValueKind::ParamDefinition
                || definition.line > line
                || !scope_encloses(definition.scope.as_deref(), scope)
            {
                continue;
            }
            if let Ok(value) = super::expr::eval_expression(&definition.expression, &context) {
                context.set(&definition.name, value);
            }
        }
        context
    }

    /// The definition a reference resolves to: the last same-named definition
    /// at or above `line` whose scope encloses `scope`.
    pub fn param_definition_for(
        &self,
        name: &str,
        line: usize,
        scope: Option<&str>,
    ) -> Option<&NetlistValueSpan> {
        self.value_spans.iter().rfind(|span| {
            span.kind == NetlistValueKind::ParamDefinition
                && span.line <= line
                && span.name.eq_ignore_ascii_case(name)
                && scope_encloses(span.scope.as_deref(), scope)
        })
    }
}

/// Whether a definition written in `definition_scope` is visible from
/// `reader_scope`. `None` is the top level, which every scope can see.
fn scope_encloses(definition_scope: Option<&str>, reader_scope: Option<&str>) -> bool {
    let Some(definition_scope) = definition_scope else {
        return true;
    };
    let Some(reader_scope) = reader_scope else {
        return false;
    };
    reader_scope.eq_ignore_ascii_case(definition_scope)
        || reader_scope
            .get(..definition_scope.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(definition_scope))
            && reader_scope.as_bytes().get(definition_scope.len()) == Some(&b'.')
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
                line.push_segment(
                    continued,
                    offset + lead + 1 + continued_lead,
                    line_index + 1,
                );
            }
        } else {
            flush_logical_line(&mut logical, &mut scopes, &mut map);
            let mut line = MappedLine::default();
            line.push_segment(trimmed, offset + lead, line_index + 1);
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
    /// One-based physical line each byte of `text` came from.
    physical_lines: Vec<usize>,
    /// Whether each byte of `text` was inserted by the joiner rather than
    /// read from the buffer.
    inserted: Vec<bool>,
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
            self.physical_lines
                .push(self.physical_lines.last().copied().unwrap_or(1));
            self.inserted.push(true);
        }
    }

    fn push_segment(&mut self, segment: &str, original_start: usize, physical_line: usize) {
        self.text.push_str(segment);
        self.byte_offsets
            .extend((0..segment.len()).map(|idx| original_start + idx));
        self.physical_lines
            .extend(std::iter::repeat_n(physical_line, segment.len()));
        self.inserted
            .extend(std::iter::repeat_n(false, segment.len()));
    }

    fn original_span(&self, span: Range<usize>) -> Option<Range<usize>> {
        if span.is_empty() || span.end > self.byte_offsets.len() {
            return None;
        }
        let start = self.byte_offsets[span.start];
        let end = self.byte_offsets[span.end - 1] + 1;
        Some(start..end)
    }

    /// Split a logical span into the buffer ranges it occupies, one per
    /// contiguous run on one physical line. Bytes the joiner inserted own no
    /// buffer position and end the run they follow.
    fn original_runs(&self, span: Range<usize>) -> Vec<(Range<usize>, usize)> {
        let mut runs: Vec<(Range<usize>, usize)> = Vec::new();
        for index in span.start..span.end.min(self.byte_offsets.len()) {
            if self.inserted[index] {
                continue;
            }
            let offset = self.byte_offsets[index];
            let line = self.physical_lines[index];
            match runs.last_mut() {
                Some((range, run_line)) if range.end == offset && *run_line == line => {
                    range.end += 1;
                }
                _ => runs.push((offset..offset + 1, line)),
            }
        }
        runs
    }

    /// The buffer range and physical line of a logical span that lies on one
    /// physical line, or `None` when it crosses a continuation.
    fn original_run(&self, span: Range<usize>) -> Option<(Range<usize>, usize)> {
        match self.original_runs(span).as_slice() {
            [run] => Some(run.clone()),
            _ => None,
        }
    }
}

fn process_logical_line(line: &MappedLine, scopes: &mut Vec<String>, map: &mut NetlistSourceMap) {
    let tokens = token_spans(&line.text);
    let Some(first) = tokens.first() else {
        return;
    };
    let head = &line.text[first.clone()];
    let scope = current_scope(scopes);

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
        // A `PARAMS:` default belongs to the subcircuit it opens, so it is
        // collected against the scope the header just pushed.
        collect_value_spans(line, &tokens, head, current_scope(scopes), map);
        return;
    }

    if head.eq_ignore_ascii_case(".ends") {
        scopes.pop();
        return;
    }

    collect_value_spans(line, &tokens, head, scope.clone(), map);

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

    if let Some(reference) = reference_from_element_line(line, &tokens, scope) {
        map.references.push(reference);
    }
}

/// Whether a directive's assignments bind parameter names the rest of the deck
/// reads, rather than a device, model, or instance property.
fn defines_parameters(head: &str) -> bool {
    head.eq_ignore_ascii_case(".param")
        || head.eq_ignore_ascii_case(".csparam")
        || head.eq_ignore_ascii_case(".global_param")
        || head.eq_ignore_ascii_case(".subckt")
}

/// One value slot found on a logical line.
struct LogicalValue {
    /// Name this value is assigned to, when it is the right side of `name=`.
    name: Option<Range<usize>>,
    /// The value as written, braces or quotes included.
    token: Range<usize>,
    /// The expression body, braces or quotes removed.
    body: Range<usize>,
}

fn collect_value_spans(
    line: &MappedLine,
    tokens: &[Range<usize>],
    head: &str,
    scope: Option<String>,
    map: &mut NetlistSourceMap,
) {
    if head.eq_ignore_ascii_case(".func") {
        collect_function_definition(line, tokens, scope, map);
        return;
    }

    let binds_names = defines_parameters(head);
    for value in logical_values(&line.text) {
        if binds_names
            && let Some(name) = value.name.clone()
            && let Some((span, physical)) = line.original_run(name.clone())
        {
            map.value_spans.push(NetlistValueSpan {
                kind: NetlistValueKind::ParamDefinition,
                span,
                name: line.text[name].to_owned(),
                expression: line.text[value.body.clone()].to_owned(),
                line: physical,
                scope: scope.clone(),
            });
        }
        collect_expression_spans(line, &value, scope.as_deref(), &[], map);
    }

    // A passive's positional value slot is lexically a bare name; the editor
    // offers it as a parameter and stays quiet when it turns out to name a
    // model instead.
    if matches!(
        head.as_bytes().first().map(u8::to_ascii_uppercase),
        Some(b'R' | b'C' | b'L')
    ) && let Some(token) = tokens.get(3)
        && is_bare_identifier(&line.text[token.clone()])
        && let Some((span, physical)) = line.original_run(token.clone())
    {
        map.value_spans.push(NetlistValueSpan {
            kind: NetlistValueKind::ParamReference,
            span,
            name: line.text[token.clone()].to_owned(),
            expression: line.text[token.clone()].to_owned(),
            line: physical,
            scope,
        });
    }
}

fn collect_function_definition(
    line: &MappedLine,
    tokens: &[Range<usize>],
    scope: Option<String>,
    map: &mut NetlistSourceMap,
) {
    let Some(header) = tokens.get(1) else {
        return;
    };
    let text = &line.text;
    let Some(open) = text[header.clone()].find('(').map(|at| header.start + at) else {
        return;
    };
    let Some(close) = text[open..].find(')').map(|at| open + at) else {
        return;
    };
    let Some(value) = logical_values(text)
        .into_iter()
        .find(|value| value.body.start > close)
    else {
        return;
    };
    let Some((_, physical)) = line.original_run(header.start..open) else {
        return;
    };
    let arguments = text[open + 1..close]
        .split(',')
        .map(str::trim)
        .filter(|argument| !argument.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    collect_expression_spans(line, &value, scope.as_deref(), &arguments, map);
    map.functions.push(NetlistFunctionDefinition {
        name: text[header.start..open].to_owned(),
        arguments,
        body: text[value.body.clone()].to_owned(),
        line: physical,
        scope,
    });
}

/// Emit the expression span for one value slot plus a reference span for every
/// parameter name it reads. `formals` names the function arguments a `.FUNC`
/// body binds itself, which are not deck parameters.
fn collect_expression_spans(
    line: &MappedLine,
    value: &LogicalValue,
    scope: Option<&str>,
    formals: &[String],
    map: &mut NetlistSourceMap,
) {
    let body = &line.text[value.body.clone()];
    // A bare name is offered as that reference alone; a complete SPICE literal
    // has no value a reader does not already see.
    let braced = value.token != value.body;
    if (braced || !is_bare_identifier(body))
        && super::lexer::parse_spice_value_complete(body).is_err()
    {
        for (span, physical) in line.original_runs(value.token.clone()) {
            map.value_spans.push(NetlistValueSpan {
                kind: NetlistValueKind::Expression,
                span,
                name: String::new(),
                expression: body.to_owned(),
                line: physical,
                scope: scope.map(str::to_owned),
            });
        }
    }
    for reference in identifier_references(&line.text, value.body.clone()) {
        let name = &line.text[reference.clone()];
        if formals
            .iter()
            .any(|formal| formal.eq_ignore_ascii_case(name))
        {
            continue;
        }
        let Some((span, physical)) = line.original_run(reference.clone()) else {
            continue;
        };
        map.value_spans.push(NetlistValueSpan {
            kind: NetlistValueKind::ParamReference,
            span,
            name: line.text[reference.clone()].to_owned(),
            expression: line.text[reference].to_owned(),
            line: physical,
            scope: scope.map(str::to_owned),
        });
    }
}

/// Every value slot on a logical line: braced, quoted, and bare right sides of
/// `name=`, plus free-standing brace and quote groups.
fn logical_values(text: &str) -> Vec<LogicalValue> {
    let bytes = text.as_bytes();
    let mut values = Vec::new();
    let mut pending_name = None;
    let mut index = 0usize;
    while index < bytes.len() {
        match bytes[index] {
            b'{' | b'\'' | b'"' => {
                let Some((token, body)) = delimited_group(bytes, index) else {
                    break;
                };
                index = token.end;
                values.push(LogicalValue {
                    name: pending_name.take(),
                    token,
                    body,
                });
            }
            b'=' if !is_comparison(bytes, index) => {
                let name = identifier_before(text, index);
                let mut start = index + 1;
                while bytes.get(start).is_some_and(u8::is_ascii_whitespace) {
                    start += 1;
                }
                if matches!(bytes.get(start), Some(b'{' | b'\'' | b'"')) {
                    pending_name = name;
                    index = start;
                    continue;
                }
                let mut end = start;
                while end < bytes.len() && !bytes[end].is_ascii_whitespace() && bytes[end] != b',' {
                    end += 1;
                }
                if start < end {
                    values.push(LogicalValue {
                        name,
                        token: start..end,
                        body: start..end,
                    });
                }
                pending_name = None;
                index = end.max(index + 1);
            }
            _ => index += 1,
        }
    }
    values
}

/// The token and body of a brace or quote group opening at `open`.
fn delimited_group(bytes: &[u8], open: usize) -> Option<(Range<usize>, Range<usize>)> {
    let close = match bytes[open] {
        b'{' => {
            let mut depth = 0usize;
            let mut index = open;
            loop {
                match bytes.get(index)? {
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            break index;
                        }
                    }
                    _ => {}
                }
                index += 1;
            }
        }
        delimiter => open + 1 + bytes[open + 1..].iter().position(|b| *b == delimiter)?,
    };
    Some((open..close + 1, open + 1..close))
}

/// Whether the `=` at `index` is part of a comparison operator.
fn is_comparison(bytes: &[u8], index: usize) -> bool {
    bytes.get(index + 1) == Some(&b'=')
        || index
            .checked_sub(1)
            .and_then(|before| bytes.get(before))
            .is_some_and(|before| matches!(before, b'=' | b'<' | b'>' | b'!'))
}

/// The identifier immediately to the left of `index`, skipping whitespace.
fn identifier_before(text: &str, index: usize) -> Option<Range<usize>> {
    let bytes = text.as_bytes();
    let mut end = index;
    while end > 0 && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    let mut start = end;
    while start > 0 && is_identifier_byte(bytes[start - 1]) {
        start -= 1;
    }
    (start < end && is_identifier_start(bytes[start])).then_some(start..end)
}

const fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$')
}

const fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$')
}

fn is_bare_identifier(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.first().copied().is_some_and(is_identifier_start)
        && bytes.iter().copied().all(is_identifier_byte)
}

/// Parameter names read inside `range`.
///
/// Numeric literals and their engineering suffixes are consumed whole, a name
/// followed by `(` is a function call rather than a parameter, and the
/// arguments of the `V()`/`I()` probes are node and device names.
fn identifier_references(text: &str, range: Range<usize>) -> Vec<Range<usize>> {
    let bytes = text.as_bytes();
    let mut references = Vec::new();
    let mut index = range.start;
    while index < range.end {
        let byte = bytes[index];
        if byte.is_ascii_digit()
            || (byte == b'.' && bytes.get(index + 1).is_some_and(u8::is_ascii_digit))
        {
            index += 1;
            while index < range.end
                && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'.')
            {
                index += 1;
            }
            continue;
        }
        if !is_identifier_start(byte) {
            index += 1;
            continue;
        }
        let start = index;
        while index < range.end && is_identifier_byte(bytes[index]) {
            index += 1;
        }
        let mut after = index;
        while after < range.end && bytes[after].is_ascii_whitespace() {
            after += 1;
        }
        if bytes.get(after) != Some(&b'(') {
            references.push(start..index);
            continue;
        }
        let name = &text[start..index];
        if name.eq_ignore_ascii_case("v") || name.eq_ignore_ascii_case("i") {
            index = delimited_call_end(bytes, after).min(range.end);
        }
    }
    references
}

/// One past the `)` closing the call whose `(` is at `open`.
fn delimited_call_end(bytes: &[u8], open: usize) -> usize {
    let mut depth = 0usize;
    let mut index = open;
    while index < bytes.len() {
        match bytes[index] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return index + 1;
                }
            }
            _ => {}
        }
        index += 1;
    }
    bytes.len()
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

    /// The spans of one kind, as `(name, source text, line, expression)`.
    fn spans_of<'a>(
        map: &'a NetlistSourceMap,
        src: &'a str,
        kind: NetlistValueKind,
    ) -> Vec<(&'a str, &'a str, usize, &'a str)> {
        map.value_spans
            .iter()
            .filter(|span| span.kind == kind)
            .map(|span| {
                (
                    span.name.as_str(),
                    &src[span.span.clone()],
                    span.line,
                    span.expression.as_str(),
                )
            })
            .collect()
    }

    #[test]
    fn value_spans_index_definitions_references_and_expressions() {
        let src = "deck\n\
.param w=2u\n\
.param l={w*3}\n\
M1 d g s b nch W={w} L={l*2}\n\
.model nch nmos\n\
.end\n";

        let map = source_map_for_editor(src);

        assert_eq!(
            spans_of(&map, src, NetlistValueKind::ParamDefinition),
            vec![("w", "w", 2, "2u"), ("l", "l", 3, "w*3")]
        );
        assert_eq!(
            spans_of(&map, src, NetlistValueKind::ParamReference),
            vec![("w", "w", 3, "w"), ("w", "w", 4, "w"), ("l", "l", 4, "l")]
        );
        assert_eq!(
            spans_of(&map, src, NetlistValueKind::Expression),
            vec![
                ("", "{w*3}", 3, "w*3"),
                ("", "{w}", 4, "w"),
                ("", "{l*2}", 4, "l*2"),
            ],
            "a literal value slot such as a node or model name carries no expression span"
        );

        let context = map.param_context_at(4, None);
        assert_eq!(context.get("w"), Some(2e-6));
        assert_eq!(context.get("l"), Some(6e-6));
        assert_eq!(
            map.param_context_at(2, None).get("l"),
            None,
            "a definition below the reader's line is not visible yet"
        );
    }

    #[test]
    fn value_spans_follow_an_expression_across_a_continuation_line() {
        let src = "deck\n.param w=1u\nM1 d g s b nch\n+ W={w\n+ *2}\n.end\n";

        let map = source_map_for_editor(src);

        assert_eq!(
            spans_of(&map, src, NetlistValueKind::Expression),
            vec![("", "{w", 4, "w *2"), ("", "*2}", 5, "w *2")],
            "each physical line of a continued expression carries the joined text"
        );
        assert_eq!(
            spans_of(&map, src, NetlistValueKind::ParamReference),
            vec![("w", "w", 4, "w")]
        );
        assert_eq!(
            rspice_core_eval(&map, 4, "w *2"),
            Some(2e-6),
            "the joined expression evaluates against the line it is hovered on"
        );
    }

    fn rspice_core_eval(map: &NetlistSourceMap, line: usize, expression: &str) -> Option<f64> {
        crate::netlist::expr::eval_expression(expression, &map.param_context_at(line, None)).ok()
    }

    #[test]
    fn param_context_at_line_applies_definitions_in_source_order() {
        let src = "deck\n.param a=1\n.param b={a*2}\n.param a=10\n.param c={a*2}\n.end\n";

        let map = source_map_for_editor(src);

        let early = map.param_context_at(3, None);
        assert_eq!(early.get("a"), Some(1.0));
        assert_eq!(early.get("b"), Some(2.0));

        let late = map.param_context_at(5, None);
        assert_eq!(
            late.get("a"),
            Some(10.0),
            "a redefinition shadows the first"
        );
        assert_eq!(late.get("b"), Some(2.0));
        assert_eq!(late.get("c"), Some(20.0));
    }

    #[test]
    fn param_context_at_line_scopes_subcircuit_parameters() {
        let src = "deck\n\
.param g=1\n\
.subckt cell in out params: g=5\n\
.param k={g*2}\n\
.ends\n\
.param t={g*2}\n\
.end\n";

        let map = source_map_for_editor(src);

        let inside = map.param_context_at(4, Some("cell"));
        assert_eq!(inside.get("g"), Some(5.0), "the subcircuit default shadows");
        assert_eq!(inside.get("k"), Some(10.0));

        let outside = map.param_context_at(6, None);
        assert_eq!(outside.get("g"), Some(1.0));
        assert_eq!(outside.get("t"), Some(2.0));
        assert_eq!(
            outside.get("k"),
            None,
            "a subcircuit-local parameter is invisible at the top level"
        );
        assert_eq!(
            map.param_definition_for("g", 4, Some("cell"))
                .map(|definition| definition.line),
            Some(3)
        );
        assert_eq!(
            map.param_definition_for("g", 6, None)
                .map(|definition| definition.line),
            Some(2)
        );
    }

    #[test]
    fn param_context_at_line_resolves_functions_and_nominal_statistics() {
        let src = "deck\n\
.param w=2u\n\
.func dbl(x)={x*2}\n\
.param d={dbl(w)}\n\
.param s={agauss(1,0.5,3)}\n\
.end\n";

        let map = source_map_for_editor(src);
        let context = map.param_context_at(5, None);

        assert_eq!(context.get("d"), Some(4e-6));
        assert_eq!(
            context.get("s"),
            Some(1.0),
            "a statistical operator reports its nominal, not one seeded draw"
        );
        assert!(
            !map.value_spans.iter().any(|span| span.name == "x"),
            "a function formal is not a deck parameter: {:?}",
            map.value_spans
        );
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
