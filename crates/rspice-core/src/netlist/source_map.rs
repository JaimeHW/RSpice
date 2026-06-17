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
pub enum NetlistReferenceKind {
    Model { element: String, name: String },
    Subcircuit { element: String, name: String },
    ControlLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistReference {
    pub kind: NetlistReferenceKind,
    pub range: SourceRange,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NetlistSourceMap {
    pub references: Vec<NetlistReference>,
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
    pub fn source_map(&self) -> NetlistSourceMap {
        NetlistSourceMap::from_netlist(self)
    }

    pub fn lint_unknown_references(&self) -> Vec<UnknownReferenceDiagnostic> {
        self.source_map().lint_unknown_references(self)
    }
}

impl NetlistSourceMap {
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
                NetlistReferenceKind::Model { element, name } => {
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
                NetlistReferenceKind::Subcircuit { element, name } => {
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
                NetlistReferenceKind::ControlLine => {}
            }
        }

        diagnostics
    }
}

#[derive(Default)]
struct SourceMapBuilder {
    references: Vec<NetlistReference>,
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
                self.references.push(NetlistReference {
                    kind: NetlistReferenceKind::ControlLine,
                    range: trimmed_range(line_num, line, stripped, trimmed),
                    scope: self.current_scope(),
                });
                continue;
            }

            if self.in_control {
                self.references.push(NetlistReference {
                    kind: NetlistReferenceKind::ControlLine,
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

    fn into_map(self) -> NetlistSourceMap {
        NetlistSourceMap {
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

fn references_for_logical_line(logical: &LogicalLine) -> Vec<NetlistReference> {
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
        NetlistReferenceKind::Subcircuit { element, name }
    } else {
        NetlistReferenceKind::Model { element, name }
    };
    vec![NetlistReference {
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
        if ident_text(candidate).is_none() {
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
        if ident_text(candidate).is_none() {
            return None;
        }
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
    use super::{NetlistReferenceKind, UnknownReferenceKind};
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
                    NetlistReferenceKind::Model { element, name }
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
                NetlistReferenceKind::Subcircuit { element, name }
                    if element == "XGOOD" && name == "GOOD_CELL"
            )
        }));
        assert!(
            map.references
                .iter()
                .any(|reference| matches!(reference.kind, NetlistReferenceKind::ControlLine))
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
                    NetlistReferenceKind::Model { element, name }
                        if element == "D1" && name == "INC_D"
                )
            })
            .expect("diode reference is mapped");

        assert_eq!(d_ref.range.line, 3);
        assert_eq!(map.lint_unknown_references(&netlist), Vec::new());

        let _ = std::fs::remove_dir_all(root);
    }
}
