//! The declarations of a Verilog-A file, the scopes they are visible in, and
//! what each name written in the file refers to.
//!
//! Declarations and scopes come from the compiler's parse of the file: every
//! declarator of a declaration, ports, nets, parameters, real, integer and
//! string variables, branches, genvars, analog functions with their
//! arguments and locals, named blocks with theirs, and the natures and
//! disciplines the file declares. Every name the compiler's lexer reads in
//! the file is then resolved as Verilog-A resolves it: in the innermost
//! scope around it that declares it, then outwards. A name after a `.`, a
//! name inside an attribute and a macro's name are not references.
//!
//! The parser reads the file's own text with the preprocessor's syntax made
//! inert byte for byte, so every span it reports is a place in the file.

use std::collections::HashMap;
use std::ops::Range;

use rspice_veriloga::TokenKind;
use rspice_veriloga::ast::{AnalogBlock, AnalogStatement, Item, Module, VarType};

/// What a declaration declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum VerilogASymbolKind {
    Module,
    Port,
    Net,
    Parameter,
    Variable,
    Genvar,
    Branch,
    Function,
    Argument,
    Block,
    Nature,
    Discipline,
}

impl VerilogASymbolKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Port => "port",
            Self::Net => "net",
            Self::Parameter => "parameter",
            Self::Variable => "variable",
            Self::Genvar => "genvar",
            Self::Branch => "branch",
            Self::Function => "function",
            Self::Argument => "argument",
            Self::Block => "block",
            Self::Nature => "nature",
            Self::Discipline => "discipline",
        }
    }
}

/// One declared name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerilogADeclaration {
    pub name: String,
    pub kind: VerilogASymbolKind,
    /// The scope it is declared in; 0 is the file.
    pub scope: usize,
    /// Where the name is first written in a declaration of it.
    pub name_range: Range<usize>,
    /// What its declaration says about it, in the source's words:
    /// `inout electrical`, `parameter real = 1e5 from (0:inf)`.
    pub detail: String,
    /// A parameter's `desc` and `units`: `Open-loop gain (V/V)`.
    pub description: Option<String>,
    /// The arguments of an analog function, in order.
    pub arguments: Vec<String>,
}

/// One name as the file writes it, and the declaration it refers to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerilogAName {
    pub name: String,
    pub range: Range<usize>,
    /// One-based.
    pub line: usize,
    /// One-based, in characters.
    pub column: usize,
    pub declaration: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scope {
    range: Range<usize>,
    /// The file, or a module: what a whole module can see.
    module_level: bool,
}

/// The symbols of one Verilog-A file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct VerilogASymbols {
    scopes: Vec<Scope>,
    declarations: Vec<VerilogADeclaration>,
    names: Vec<VerilogAName>,
    /// The names written after a `.`, as a named connection writes them.
    members: Vec<VerilogAName>,
    /// Why the parser could not read the file, with the line it stopped on.
    unreadable: Option<(usize, String)>,
}

impl VerilogASymbols {
    /// Read `source`. A file the parser cannot read keeps its names, each
    /// referring to nothing, and says why.
    pub(crate) fn read(source: &str) -> Self {
        Self::read_forgiving(source, None)
    }

    /// Read `source` as [`Self::read`] does, and when it does not parse,
    /// read it again with the one-based line `forgiven` blanked: the line a
    /// writer is in the middle of.
    pub(crate) fn read_forgiving(source: &str, forgiven: Option<usize>) -> Self {
        let line_starts = line_starts(source);
        let (names, members) = lexed_names(source, &line_starts);
        let readable = super::source_navigation::without_preprocessor_syntax(source);
        let file = match parse(&readable) {
            Ok(file) => Ok(file),
            Err(failure) => match forgiven.and_then(|line| blank_line(&readable, line)) {
                Some(blanked) => parse(&blanked).map_err(|_| failure),
                None => Err(failure),
            },
        };
        let mut symbols = Self {
            scopes: vec![Scope {
                range: 0..source.len(),
                module_level: true,
            }],
            declarations: Vec::new(),
            names,
            members,
            unreadable: None,
        };
        match file {
            Ok(file) => {
                let mut reader = Reader {
                    source,
                    symbols: &mut symbols,
                };
                for item in &file.items {
                    reader.item(item);
                }
                symbols.resolve_names();
            }
            Err((offset, message)) => {
                let line = line_starts.partition_point(|start| *start <= offset).max(1);
                symbols.unreadable = Some((line, message));
            }
        }
        symbols
    }

    /// The line the parser stopped on, when it could not read the file.
    pub(crate) fn unreadable_line(&self) -> Option<usize> {
        self.unreadable.as_ref().map(|(line, _)| *line)
    }

    /// Why the parser could not read the file: its line and its message.
    pub(crate) fn unreadable(&self) -> Option<(usize, &str)> {
        self.unreadable
            .as_ref()
            .map(|(line, message)| (*line, message.as_str()))
    }

    pub(crate) fn declarations(&self) -> &[VerilogADeclaration] {
        &self.declarations
    }

    pub(crate) fn declaration(&self, index: usize) -> Option<&VerilogADeclaration> {
        self.declarations.get(index)
    }

    pub(crate) fn names(&self) -> &[VerilogAName] {
        &self.names
    }

    /// Whether `scope` is the file or a module.
    pub(crate) fn is_module_level(&self, scope: usize) -> bool {
        self.scopes.get(scope).is_some_and(|scope| scope.module_level)
    }

    /// The first place `name` is written after a `.`.
    pub(crate) fn member_named(&self, name: &str) -> Option<&VerilogAName> {
        self.members.iter().find(|member| member.name == name)
    }

    /// The name written at byte `offset`, or ending right at it.
    pub(crate) fn name_at(&self, offset: usize) -> Option<&VerilogAName> {
        let index = self.names.partition_point(|name| name.range.end < offset);
        self.names
            .get(index)
            .filter(|name| name.range.start <= offset && offset <= name.range.end)
    }

    /// Every place the file refers to declaration `index`, its declarators
    /// included, in order.
    pub(crate) fn references(&self, index: usize) -> impl Iterator<Item = &VerilogAName> {
        self.names
            .iter()
            .filter(move |name| name.declaration == Some(index))
    }

    /// The declaration named `name` in scope `scope` itself, not around it.
    pub(crate) fn declared_in(&self, scope: usize, name: &str) -> Option<usize> {
        self.declarations
            .iter()
            .position(|declaration| declaration.scope == scope && declaration.name == name)
    }

    /// The scopes around byte `offset`, innermost first, ending at the file.
    pub(crate) fn enclosing_scopes(&self, offset: usize) -> impl Iterator<Item = usize> + '_ {
        let mut around = self
            .scopes
            .iter()
            .enumerate()
            .filter(|(_, scope)| scope.range.start <= offset && offset <= scope.range.end)
            .map(|(index, scope)| (scope.range.len(), index))
            .collect::<Vec<_>>();
        around.sort_unstable();
        around.into_iter().map(|(_, index)| index)
    }

    /// The declarations visible at byte `offset`, innermost first, one per
    /// name.
    pub(crate) fn visible_at(&self, offset: usize) -> Vec<&VerilogADeclaration> {
        let mut seen = std::collections::BTreeSet::new();
        let mut visible = Vec::new();
        for scope in self.enclosing_scopes(offset) {
            for declaration in &self.declarations {
                if declaration.scope == scope && seen.insert(declaration.name.as_str()) {
                    visible.push(declaration);
                }
            }
        }
        visible
    }

    /// Every name refers to the declaration around it that declares it.
    fn resolve_names(&mut self) {
        let mut by_scope = HashMap::<(usize, &str), usize>::new();
        for (index, declaration) in self.declarations.iter().enumerate() {
            by_scope
                .entry((declaration.scope, declaration.name.as_str()))
                .or_insert(index);
        }
        let mut resolved = Vec::with_capacity(self.names.len());
        for name in &self.names {
            resolved.push(
                self.enclosing_scopes(name.range.start)
                    .find_map(|scope| by_scope.get(&(scope, name.name.as_str())).copied()),
            );
        }
        for (name, declaration) in self.names.iter_mut().zip(resolved) {
            name.declaration = declaration;
        }
    }
}

/// The byte where the parser stopped, and why, when it could not read
/// `text`.
fn parse(text: &str) -> Result<rspice_veriloga::SourceFile, (usize, String)> {
    let source_id = rspice_veriloga::SourceMap::new().add_source("symbols", String::new());
    let tokens = rspice_veriloga::Lexer::new(text, source_id)
        .collect_tokens()
        .map_err(|error| (error.span.start as usize, error.to_string()))?;
    rspice_veriloga::Parser::new(&tokens)
        .parse()
        .map_err(|error| (error.span.start as usize, error.kind.to_string()))
}

/// `text` with its one-based `line` made blank, byte for byte.
fn blank_line(text: &str, line: usize) -> Option<String> {
    let starts = line_starts(text);
    let start = *starts.get(line.checked_sub(1)?)?;
    let end = text[start..].find('\n').map_or(text.len(), |end| start + end);
    let mut blanked = text.to_owned();
    blanked.replace_range(
        start..end,
        &text[start..end]
            .chars()
            .map(|character| {
                if character == '\r' {
                    "\r".to_owned()
                } else {
                    " ".repeat(character.len_utf8())
                }
            })
            .collect::<String>(),
    );
    Some(blanked)
}

pub(crate) fn line_starts(source: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(source.match_indices('\n').map(|(index, _)| index + 1))
        .collect()
}

/// Every name the compiler's lexer reads in `source`, less those that are
/// not references: an attribute's words and a macro; the names written after
/// a `.` are returned apart. A stretch the lexer refuses is stepped over, so
/// a half-typed file keeps the names around it.
fn lexed_names(source: &str, line_starts: &[usize]) -> (Vec<VerilogAName>, Vec<VerilogAName>) {
    let mut source_map = rspice_veriloga::SourceMap::new();
    let source_id = source_map.add_source_mut("symbols", source);
    let mut names = Vec::new();
    let mut members = Vec::new();
    let mut resume = 0;
    let mut previous = None::<(TokenKind, usize)>;
    let mut attribute = false;
    while resume < source.len() {
        let mut lexer = rspice_veriloga::Lexer::new(&source[resume..], source_id);
        let refused = loop {
            let token = match lexer.next_token() {
                Ok(token) if token.kind == TokenKind::Eof => break None,
                Ok(token) => token,
                Err(error) => break Some(error.span),
            };
            let start = resume + token.span.start as usize;
            let end = resume + token.span.end as usize;
            // `(*` opens an attribute and `*)` closes it.
            match (previous, token.kind) {
                (Some((TokenKind::LParen, at)), TokenKind::Star) if at == start => {
                    attribute = true;
                }
                (Some((TokenKind::Star, at)), TokenKind::RParen) if at == start => {
                    attribute = false;
                }
                _ => {}
            }
            let after_dot = previous.is_some_and(|(kind, _)| kind == TokenKind::Dot);
            previous = Some((token.kind, end));
            if token.kind != TokenKind::Identifier || attribute {
                continue;
            }
            let line = line_starts.partition_point(|line_start| *line_start <= start);
            let name = VerilogAName {
                name: source[start..end].to_owned(),
                range: start..end,
                line,
                column: source[line_starts[line - 1]..start].chars().count() + 1,
                declaration: None,
            };
            if after_dot {
                members.push(name);
            } else {
                names.push(name);
            }
        };
        let Some(span) = refused else {
            break;
        };
        let start = resume + span.start as usize;
        let end = resume + span.end as usize;
        previous = None;
        resume = if end > start {
            end
        } else {
            start + source[start..].chars().next().map_or(1, char::len_utf8)
        };
    }
    (names, members)
}

/// Reads declarations and scopes out of the parse.
struct Reader<'a> {
    source: &'a str,
    symbols: &'a mut VerilogASymbols,
}

impl Reader<'_> {
    fn item(&mut self, item: &Item) {
        match item {
            Item::Module(module) | Item::ConnectModule(module) => self.module(module),
            Item::Nature(nature) => {
                self.declare(
                    0,
                    &nature.name,
                    VerilogASymbolKind::Nature,
                    span(nature.span),
                    "nature".to_owned(),
                );
            }
            Item::Discipline(discipline) => {
                self.declare(
                    0,
                    &discipline.name,
                    VerilogASymbolKind::Discipline,
                    span(discipline.span),
                    "discipline".to_owned(),
                );
            }
            _ => {}
        }
    }

    fn slice(&self, span: rspice_veriloga::Span) -> &str {
        self.source
            .get(span.start as usize..span.end as usize)
            .unwrap_or_default()
            .trim()
    }

    fn scope(&mut self, range: Range<usize>, module_level: bool) -> usize {
        self.symbols.scopes.push(Scope { range, module_level });
        self.symbols.scopes.len() - 1
    }

    /// Declare `name` in `scope` at its first name token inside `within`.
    /// A declaration the parser made up has no such token and is not one.
    fn declare(
        &mut self,
        scope: usize,
        name: &str,
        kind: VerilogASymbolKind,
        within: Range<usize>,
        detail: String,
    ) -> Option<usize> {
        if let Some(existing) = self.symbols.declared_in(scope, name) {
            return Some(existing);
        }
        let token = self.symbols.names.iter().find(|token| {
            token.name == name && within.start <= token.range.start && token.range.end <= within.end
        })?;
        self.symbols.declarations.push(VerilogADeclaration {
            name: name.to_owned(),
            kind,
            scope,
            name_range: token.range.clone(),
            detail,
            description: None,
            arguments: Vec::new(),
        });
        Some(self.symbols.declarations.len() - 1)
    }

    fn module(&mut self, module: &Module) {
        let range = span(module.span);
        self.declare(
            0,
            &module.name,
            VerilogASymbolKind::Module,
            range.clone(),
            "module".to_owned(),
        );
        let scope = self.scope(range, true);
        for port in &module.ports {
            let declaration = module
                .port_declarations
                .iter()
                .find(|declaration| declaration.names.contains(&port.name));
            let discipline = declaration
                .and_then(|declaration| declaration.discipline.clone())
                .or_else(|| {
                    module
                        .nets
                        .iter()
                        .filter(|net| net.names.contains(&port.name))
                        .find_map(|net| net.discipline.clone())
                });
            let detail = [
                declaration.map(|declaration| declaration.direction.label().to_owned()),
                discipline.map(|discipline| discipline.to_string()),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" ");
            let within = span(port.span);
            let within = if within.is_empty() {
                declaration.map_or(within, |declaration| span(declaration.span))
            } else {
                within
            };
            self.declare(
                scope,
                &port.name,
                VerilogASymbolKind::Port,
                within,
                if detail.is_empty() {
                    "port".to_owned()
                } else {
                    detail
                },
            );
        }
        for declaration in &module.port_declarations {
            for name in &declaration.names {
                self.declare(
                    scope,
                    name,
                    VerilogASymbolKind::Port,
                    span(declaration.span),
                    declaration.direction.label().to_owned(),
                );
            }
        }
        for net in &module.nets {
            let detail = net.discipline.as_ref().map_or_else(
                || if net.is_ground { "ground" } else { "net" }.to_owned(),
                ToString::to_string,
            );
            for name in &net.names {
                self.declare(
                    scope,
                    name,
                    VerilogASymbolKind::Net,
                    span(net.span),
                    detail.clone(),
                );
            }
        }
        let parameters = module
            .parameters
            .iter()
            .map(|parameter| (parameter, "parameter"))
            .chain(
                module
                    .localparams
                    .iter()
                    .map(|parameter| (parameter, "localparam")),
            );
        for (parameter, keyword) in parameters {
            let mut words = vec![
                keyword.to_owned(),
                match parameter.param_type {
                    rspice_veriloga::ast::ParamType::Real => "real",
                    rspice_veriloga::ast::ParamType::Integer => "integer",
                    rspice_veriloga::ast::ParamType::String => "string",
                }
                .to_owned(),
            ];
            if let Some(default) = &parameter.default {
                words.push(format!("= {}", self.slice(default.span())));
            }
            if let Some(range) = &parameter.range {
                words.push(self.slice(range.span).to_owned());
            }
            let description = match (&parameter.description, &parameter.units) {
                (Some(description), Some(units)) => Some(format!("{description} ({units})")),
                (Some(description), None) => Some(description.to_string()),
                (None, Some(units)) => Some(format!("In {units}.")),
                (None, None) => None,
            };
            if let Some(index) = self.declare(
                scope,
                &parameter.name,
                VerilogASymbolKind::Parameter,
                span(parameter.span),
                words.join(" "),
            ) {
                self.symbols.declarations[index].description = description;
            }
        }
        for alias in &module.aliasparams {
            self.declare(
                scope,
                &alias.alias,
                VerilogASymbolKind::Parameter,
                span(alias.span),
                format!("aliasparam = {}", alias.target),
            );
        }
        self.variables(scope, &module.variables);
        for declaration in &module.digital_variables {
            let keyword = self
                .slice(declaration.span)
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_owned();
            for item in &declaration.items {
                self.declare(
                    scope,
                    &item.name,
                    VerilogASymbolKind::Variable,
                    span(item.span),
                    keyword.clone(),
                );
            }
        }
        for declaration in &module.digital_nets {
            let keyword = self
                .slice(declaration.span)
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_owned();
            for item in &declaration.items {
                self.declare(
                    scope,
                    &item.name,
                    VerilogASymbolKind::Net,
                    span(declaration.span),
                    keyword.clone(),
                );
            }
        }
        for branch in &module.branches {
            let terminals = if branch.neg.is_empty() {
                format!("branch ({})", branch.pos)
            } else {
                format!("branch ({}, {})", branch.pos, branch.neg)
            };
            self.declare(
                scope,
                &branch.name,
                VerilogASymbolKind::Branch,
                span(branch.span),
                terminals,
            );
        }
        for genvar in &module.genvars {
            for name in &genvar.names {
                self.declare(
                    scope,
                    name,
                    VerilogASymbolKind::Genvar,
                    span(genvar.span),
                    "genvar".to_owned(),
                );
            }
        }
        for function in &module.functions {
            let range = span(function.span);
            let arguments = function
                .params
                .iter()
                .map(|parameter| parameter.name.to_string())
                .collect::<Vec<_>>();
            if let Some(index) = self.declare(
                scope,
                &function.name,
                VerilogASymbolKind::Function,
                range.clone(),
                format!(
                    "analog function {} {}({})",
                    var_type(function.return_type),
                    function.name,
                    arguments.join(", ")
                ),
            ) {
                self.symbols.declarations[index].arguments = arguments;
            }
            let inner = self.scope(range.clone(), false);
            for parameter in &function.params {
                let direction = match parameter.direction {
                    rspice_veriloga::ast::ParamDirection::Input => "input",
                    rspice_veriloga::ast::ParamDirection::Output => "output",
                    rspice_veriloga::ast::ParamDirection::Inout => "inout",
                };
                // The parser places an argument at its direction keyword;
                // its name is written after that, inside the function.
                let within = span(parameter.span).start.max(range.start)..range.end;
                self.declare(
                    inner,
                    &parameter.name,
                    VerilogASymbolKind::Argument,
                    within,
                    format!("{direction} {}", var_type(parameter.param_type)),
                );
            }
            self.variables(inner, &function.locals);
            self.block(inner, &function.body);
        }
        for block in [
            &module.analog_block,
            &module.analog_initial,
            &module.analog_final,
        ]
        .into_iter()
        .flatten()
        {
            self.block(scope, block);
        }
    }

    fn variables(&mut self, scope: usize, declarations: &[rspice_veriloga::ast::VariableDecl]) {
        for declaration in declarations {
            for item in &declaration.items {
                let within = match span(item.span) {
                    within if within.is_empty() => span(declaration.span),
                    within => within,
                };
                self.declare(
                    scope,
                    &item.name,
                    VerilogASymbolKind::Variable,
                    within,
                    var_type(declaration.var_type).to_owned(),
                );
            }
        }
    }

    fn block(&mut self, scope: usize, block: &AnalogBlock) {
        for statement in &block.statements {
            self.statement(scope, statement);
        }
    }

    /// Named blocks, with what they declare, wherever statements nest.
    fn statement(&mut self, scope: usize, statement: &AnalogStatement) {
        match statement {
            AnalogStatement::Block(block) => {
                let range = span(block.span);
                let inner = if block.name.is_some() || !block.variables.is_empty() {
                    if let Some(name) = &block.name {
                        self.declare(
                            scope,
                            name,
                            VerilogASymbolKind::Block,
                            range.clone(),
                            "named block".to_owned(),
                        );
                    }
                    let inner = self.scope(range, false);
                    self.variables(inner, &block.variables);
                    inner
                } else {
                    scope
                };
                for statement in &block.statements {
                    self.statement(inner, statement);
                }
            }
            AnalogStatement::Conditional(conditional) => {
                self.statement(scope, &conditional.then_branch);
                if let Some(otherwise) = &conditional.else_branch {
                    self.statement(scope, otherwise);
                }
            }
            AnalogStatement::Case(case) => {
                for item in &case.items {
                    self.statement(scope, &item.statement);
                }
                if let Some(default) = &case.default {
                    self.statement(scope, default);
                }
            }
            AnalogStatement::For(looped) => self.statement(scope, &looped.body),
            AnalogStatement::While(looped) => self.statement(scope, &looped.body),
            AnalogStatement::Repeat(looped) => self.statement(scope, &looped.body),
            AnalogStatement::EventControl(event) => self.statement(scope, &event.statement),
            AnalogStatement::Contribution(_)
            | AnalogStatement::IndirectContribution(_)
            | AnalogStatement::Assignment(_)
            | AnalogStatement::Call(_)
            | AnalogStatement::Disable(_)
            | AnalogStatement::Null(_) => {}
        }
    }
}

fn span(span: rspice_veriloga::Span) -> Range<usize> {
    span.start as usize..span.end as usize
}

const fn var_type(kind: VarType) -> &'static str {
    match kind {
        VarType::Real => "real",
        VarType::Integer => "integer",
        VarType::String => "string",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_SCOPES: &str = "`include \"disciplines.vams\"\nmodule one(a, b);\n  inout a, b;\n  electrical a, b;\n  real x, y = 2, z;\n  analog function real twice;\n    input b;\n    real b;\n    twice = 2 * b;\n  endfunction\n  analog begin : body\n    integer k;\n    k = 1;\n    x = twice(y) + k;\n    V(a, b) <+ x * z;\n  end\nendmodule\nmodule two(b, c);\n  inout b, c;\n  electrical b, c;\n  analog V(b, c) <+ 0;\nendmodule\n";

    fn declaration<'a>(
        symbols: &'a VerilogASymbols,
        source: &str,
        written: &str,
    ) -> &'a VerilogADeclaration {
        let offset = source.find(written).expect("the text is in the source");
        let name = symbols.name_at(offset).expect("a name is written there");
        symbols
            .declaration(name.declaration.expect("the name is declared"))
            .expect("its declaration")
    }

    fn uses(symbols: &VerilogASymbols, source: &str, written: &str) -> Vec<(usize, usize)> {
        let offset = source.find(written).expect("the text is in the source");
        let index = symbols
            .name_at(offset)
            .and_then(|name| name.declaration)
            .expect("a declared name");
        symbols
            .references(index)
            .map(|name| (name.line, name.column))
            .collect()
    }

    #[test]
    fn every_declarator_is_indexed_in_its_own_scope() {
        let symbols = VerilogASymbols::read(TWO_SCOPES);
        assert_eq!(symbols.unreadable_line(), None);
        let kinds = |name: &str| {
            symbols
                .declarations()
                .iter()
                .filter(|declaration| declaration.name == name)
                .map(|declaration| declaration.kind)
                .collect::<Vec<_>>()
        };
        // Every declarator of `real x, y = 2, z;`, not the first alone.
        for name in ["x", "y", "z", "k"] {
            assert_eq!(kinds(name), [VerilogASymbolKind::Variable], "{name}");
        }
        assert_eq!(
            kinds("b"),
            [
                VerilogASymbolKind::Port,
                VerilogASymbolKind::Argument,
                VerilogASymbolKind::Port
            ]
        );
        assert_eq!(kinds("twice"), [VerilogASymbolKind::Function]);
        assert_eq!(kinds("body"), [VerilogASymbolKind::Block]);
        assert_eq!(kinds("one"), [VerilogASymbolKind::Module]);
        let a = declaration(&symbols, TWO_SCOPES, "a, b);\n  inout");
        assert_eq!(a.detail, "inout electrical");
    }

    #[test]
    fn a_name_refers_to_the_innermost_declaration_around_it() {
        let symbols = VerilogASymbols::read(TWO_SCOPES);
        // Module one's `b`: its header, both declarations and the
        // contribution, but neither the function's `b` nor module two's.
        assert_eq!(
            uses(&symbols, TWO_SCOPES, "b);\n  inout a"),
            [(2, 15), (3, 12), (4, 17), (15, 10)]
        );
        // The function's argument: declared twice inside it, used once.
        assert_eq!(
            uses(&symbols, TWO_SCOPES, "b;\n    real b"),
            [(7, 11), (8, 10), (9, 17)]
        );
        assert_eq!(
            uses(&symbols, TWO_SCOPES, "b, c);\n  inout b"),
            [(18, 12), (19, 9), (20, 14), (21, 12)]
        );
        // The block's local.
        assert_eq!(uses(&symbols, TWO_SCOPES, "k;\n"), [(12, 13), (13, 5), (14, 20)]);
        // `V` is an access function and `twice(y)` calls the function.
        let access = TWO_SCOPES.find("V(a, b)").expect("the contribution");
        assert_eq!(symbols.name_at(access).and_then(|name| name.declaration), None);
        assert_eq!(uses(&symbols, TWO_SCOPES, "twice;").len(), 3);
    }

    #[test]
    fn attributes_members_and_macros_are_not_references() {
        let source = "module m(p);\n  inout p;\n  electrical p;\n  (* desc=\"gain\" *) parameter real desc = 1;\n  analog V(p) <+ desc * `M_PI;\nendmodule\n";
        let symbols = VerilogASymbols::read(source);
        assert_eq!(
            uses(&symbols, source, "desc = 1"),
            [(4, 36), (5, 18)],
            "the attribute's own word is not the parameter"
        );
        assert!(symbols.names().iter().all(|name| name.name != "M_PI"));
    }

    #[test]
    fn a_line_being_written_is_forgiven_and_an_unreadable_file_says_where() {
        let source = "module m(p);\n  inout p;\n  electrical p;\n  real gain;\n  analog begin\n    gai\n    V(p) <+ gain;\n  end\nendmodule\n";
        let strict = VerilogASymbols::read(source);
        assert!(strict.unreadable_line().is_some());
        assert!(strict.declarations().is_empty());
        let forgiving = VerilogASymbols::read_forgiving(source, Some(6));
        assert_eq!(forgiving.unreadable_line(), None);
        let caret = source.find("gai\n").expect("the half-typed name") + 3;
        let visible = forgiving
            .visible_at(caret)
            .into_iter()
            .map(|declaration| declaration.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(visible, ["p", "gain", "m"]);
    }
}
