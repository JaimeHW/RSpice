//! User-defined primitives (IEEE 1364-2005 section 8).
//!
//! A UDP is read into its table, and each instance becomes constructs this
//! compiler already runs, as a gate becomes a continuous assignment:
//!
//! * **Combinational** (section 8.2): a continuous assignment of the output
//!   from a selection over the table's rows, `x` when no row matches.
//! * **Sequential** (sections 8.3-8.5): the state is a variable of the
//!   instantiating module, and a continuous assignment drives the output from
//!   it. Each input has a process of its own that waits on that input alone
//!   and compares it with the value the primitive last took from it, so every
//!   change it acts on is one edge on one input (section 8.4). Two inputs that
//!   change in one step wake their processes in the order the changes were
//!   made, and each reads the other inputs as the primitive last took them,
//!   which are the values they had when its own change arrived: an input
//!   change is an event of its own, in turn (section 8.6). A process tries the
//!   level rows first, which dominate (section 8.8), then its input's edge
//!   rows, and takes `x` when neither matches. It compares before it first
//!   waits, so a change made at time zero before it runs is still seen, as the
//!   `x` to value edge it is.
//!
//! An input reads `z` as `x` (section 8.1.5), and `initial` (section 8.5)
//! gives the state its starting value.

use super::Parser;
use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::lexer::TokenKind;
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::HashMap;

/// A set of input values: bit 0 for `0`, bit 1 for `1`, bit 2 for `x`.
type Levels = u8;
const ZERO: Levels = 1;
const ONE: Levels = 2;
const UNKNOWN: Levels = 4;
const ANY: Levels = 7;
/// The values in [`Levels`] order, as the literal each is compared with.
const VALUES: [(Levels, char); 3] = [(ZERO, '0'), (ONE, '1'), (UNKNOWN, 'x')];

/// One input column of a table row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Level(Levels),
    /// The `(from, to)` value pairs the edge covers, never one of `from ==
    /// to`.
    Edge(&'static [(Levels, Levels)]),
    /// An edge written `(vw)`, as the pairs of `v` and `w`.
    Pair(Levels, Levels),
}

impl Symbol {
    const fn is_edge(self) -> bool {
        !matches!(self, Self::Level(_))
    }

    /// The `(from, to)` pairs of an edge.
    fn pairs(self) -> Vec<(Levels, Levels)> {
        match self {
            Self::Level(_) => Vec::new(),
            Self::Edge(pairs) => pairs.to_vec(),
            Self::Pair(from, to) => {
                let mut pairs = Vec::new();
                for (a, _) in VALUES {
                    for (b, _) in VALUES {
                        if a != b && from & a != 0 && to & b != 0 {
                            pairs.push((a, b));
                        }
                    }
                }
                pairs
            }
        }
    }
}

/// The next output a row gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Output {
    Zero,
    One,
    Unknown,
    /// `-`: the current state.
    Unchanged,
}

#[derive(Debug, Clone)]
struct Row {
    inputs: Vec<Symbol>,
    /// The current-state column; empty (zero) for a combinational row.
    state: Levels,
    output: Output,
}

/// One `primitive ... endprimitive`.
#[derive(Debug, Clone)]
pub(super) struct UdpDefinition {
    pub(super) name: SmolStr,
    /// The output, then the inputs, as the header names them.
    ports: Vec<SmolStr>,
    inputs: usize,
    sequential: bool,
    initial: Option<char>,
    rows: Vec<Row>,
}

fn refusal(context: &str, found: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(
        ParseErrorKind::UnsupportedConstruct {
            context: context.to_string(),
            found: found.into(),
        },
        span,
    )
}

/// A table row the language does not allow: IEEE 1364-2005 section 8.1.4
/// writes a combinational row `inputs : output ;` and a sequential one
/// `inputs : state : next ;`, with one symbol for each input and one each for
/// the state and the output.
fn malformed(context: &str, found: impl Into<String>, span: Span) -> ParseError {
    ParseError::new(
        ParseErrorKind::Expected {
            expected: format!(
                "a table row of {context} in the form IEEE 1364-2005 section 8.1.4 gives one"
            ),
            found: found.into(),
        },
        span,
    )
}

impl Parser<'_> {
    /// Whether the cursor is at a `primitive` declaration.
    pub(super) fn at_primitive(&self) -> bool {
        self.check(TokenKind::AmsDigital) && self.current().text.as_deref() == Some("primitive")
    }

    fn at_word(&self, word: &str) -> bool {
        self.current().text.as_deref() == Some(word)
    }

    /// `primitive name (ports); declarations [initial] table ... endtable
    /// endprimitive`, with the port declarations in the header or the body
    /// (A.5.1).
    pub(super) fn parse_primitive(&mut self) -> Result<UdpDefinition, ParseError> {
        let start = self.current_span();
        self.advance(); // `primitive`
        let name: SmolStr = self.expect_identifier("primitive name")?.into();
        let context = format!("primitive `{name}`");
        let mut ports: Vec<SmolStr> = Vec::new();
        let mut output: Option<SmolStr> = None;
        let mut registered = false;
        let mut initial = None;
        self.expect(TokenKind::LParen)?;
        loop {
            if self.match_token(TokenKind::Output) {
                registered |= self.match_token(TokenKind::Reg);
                let port: SmolStr = self.expect_identifier("primitive output")?.into();
                if self.match_token(TokenKind::Assign_) {
                    initial = Some(self.udp_initial_value(&context)?);
                }
                output = Some(port.clone());
                ports.push(port);
            } else {
                self.match_token(TokenKind::Input);
                ports.push(self.expect_identifier("primitive port")?.into());
            }
            if !self.match_token(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Semicolon)?;

        let mut rows = None;
        loop {
            if self.match_token(TokenKind::Output) {
                registered |= self.match_token(TokenKind::Reg);
                output = Some(self.expect_identifier("primitive output")?.into());
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Reg) {
                registered = true;
                self.expect_identifier("primitive output")?;
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Input) {
                loop {
                    self.expect_identifier("primitive input")?;
                    if !self.match_token(TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(TokenKind::Semicolon)?;
            } else if self.match_token(TokenKind::Initial) {
                self.expect_identifier("primitive output")?;
                self.expect(TokenKind::Assign_)?;
                initial = Some(self.udp_initial_value(&context)?);
                self.expect(TokenKind::Semicolon)?;
            } else if self.at_word("table") {
                self.advance();
                rows = Some(self.parse_udp_table(&context)?);
            } else if self.at_word("endprimitive") {
                self.advance();
                break;
            } else {
                return Err(refusal(
                    &context,
                    format!(
                        "{}; a primitive declares its ports, an `initial` and a table",
                        self.current().describe()
                    ),
                    self.current_span(),
                ));
            }
        }
        let span = start.extend(self.previous_span());
        let rows =
            rows.ok_or_else(|| refusal(&context, "no `table` (IEEE 1364-2005 section 8.1)", span))?;
        if output.as_ref() != ports.first() {
            return Err(refusal(
                &context,
                "an output that is not its first port; IEEE 1364-2005 section 8.1.1 puts the \
                 one output first",
                span,
            ));
        }
        let inputs = ports.len() - 1;
        let limit = if registered { 9 } else { 10 };
        if inputs == 0 || inputs > limit {
            return Err(refusal(
                &context,
                format!(
                    "{inputs} inputs; IEEE 1364-2005 section 8.1.2 allows 1 to {limit} for a \
                     {} primitive",
                    if registered {
                        "sequential"
                    } else {
                        "combinational"
                    }
                ),
                span,
            ));
        }
        if initial.is_some() && !registered {
            return Err(refusal(
                &context,
                "an `initial` on a combinational primitive; IEEE 1364-2005 section 8.5 gives \
                 one only to a sequential primitive",
                span,
            ));
        }
        for row in &rows {
            if row.0.inputs.len() != inputs {
                return Err(malformed(
                    &context,
                    format!(
                        "a row with {} input symbols for {inputs} inputs",
                        row.0.inputs.len()
                    ),
                    row.1,
                ));
            }
            let edges = row
                .0
                .inputs
                .iter()
                .filter(|symbol| symbol.is_edge())
                .count();
            let sequential_row = row.0.state != 0;
            if edges > 1
                || (!registered && (edges > 0 || sequential_row))
                || (registered && !sequential_row)
                || (!registered && row.0.output == Output::Unchanged)
            {
                return Err(malformed(
                    &context,
                    "a table row of the wrong form; IEEE 1364-2005 sections 8.1.6 and 8.2-8.4 \
                     give a combinational row `inputs : output`, and a sequential one `inputs \
                     : state : next` with at most one edge",
                    row.1,
                ));
            }
        }
        Ok(UdpDefinition {
            name,
            ports,
            inputs,
            sequential: registered,
            initial,
            rows: rows.into_iter().map(|(row, _)| row).collect(),
        })
    }

    /// `1'b0`, `1'b1`, `1'bx`, `0` or `1`: the value an `initial` gives.
    fn udp_initial_value(&mut self, context: &str) -> Result<char, ParseError> {
        let span = self.current_span();
        let text = self
            .current()
            .text
            .clone()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let value = text.chars().last().filter(|_| {
            matches!(self.current().kind, TokenKind::IntegerLiteral) || text.starts_with("1'b")
        });
        match value {
            Some(value @ ('0' | '1' | 'x')) => {
                self.advance();
                Ok(value)
            }
            _ => Err(refusal(
                context,
                "an initial value that is not 1'b0, 1'b1, 1'bx, 0 or 1 (IEEE 1364-2005 \
                 section 8.5)",
                span,
            )),
        }
    }

    /// The rows up to `endtable`. Read character by character from the
    /// tokens, because the lexer reads `01` as a number and `x0` as a name,
    /// and a table separates its symbols by nothing at all.
    fn parse_udp_table(&mut self, context: &str) -> Result<Vec<(Row, Span)>, ParseError> {
        let mut characters: Vec<(char, Span)> = Vec::new();
        loop {
            let token = self.current().clone();
            if token.text.as_deref() == Some("endtable") {
                self.advance();
                break;
            }
            let fixed = match token.kind {
                TokenKind::Question => Some('?'),
                TokenKind::Star => Some('*'),
                TokenKind::Minus => Some('-'),
                TokenKind::Colon => Some(':'),
                TokenKind::Semicolon => Some(';'),
                TokenKind::LParen => Some('('),
                TokenKind::RParen => Some(')'),
                TokenKind::Identifier | TokenKind::IntegerLiteral | TokenKind::RealLiteral => None,
                _ => {
                    return Err(refusal(
                        context,
                        format!("{} in its table", token.describe()),
                        token.span,
                    ));
                }
            };
            match fixed {
                Some(character) => characters.push((character, token.span)),
                None => characters.extend(
                    token
                        .text
                        .as_deref()
                        .unwrap_or_default()
                        .chars()
                        .map(|character| (character, token.span)),
                ),
            }
            self.advance();
        }
        let mut rows = Vec::new();
        let mut rest = characters.as_slice();
        while !rest.is_empty() {
            let end = rest
                .iter()
                .position(|(character, _)| *character == ';')
                .ok_or_else(|| refusal(context, "a table row with no `;`", rest[0].1))?;
            let (row, tail) = rest.split_at(end);
            rest = &tail[1..];
            let span = row
                .first()
                .map_or(tail[0].1, |first| first.1.extend(tail[0].1));
            rows.push((Self::udp_row(row, context, span)?, span));
        }
        Ok(rows)
    }

    fn udp_row(row: &[(char, Span)], context: &str, span: Span) -> Result<Row, ParseError> {
        let fields: Vec<&[(char, Span)]> = row.split(|(character, _)| *character == ':').collect();
        let bad = |what: &str| malformed(context, format!("a row with {what}"), span);
        let output = |field: &[(char, Span)]| match field {
            [(character, _)] => match character.to_ascii_lowercase() {
                '0' => Ok(Output::Zero),
                '1' => Ok(Output::One),
                'x' => Ok(Output::Unknown),
                '-' => Ok(Output::Unchanged),
                _ => Err(bad("an output symbol that is not 0, 1, x or -")),
            },
            _ => Err(bad("an output that is not one symbol")),
        };
        let (inputs, state, next) = match fields.as_slice() {
            [inputs, next] => (inputs, None, next),
            [inputs, state, next] => (inputs, Some(state), next),
            _ => return Err(bad("a field count other than two or three")),
        };
        let state = match state {
            None => 0,
            Some([(character, _)]) => {
                level(*character).ok_or_else(|| bad("a state symbol that is not a level"))?
            }
            Some(_) => return Err(bad("a state that is not one symbol")),
        };
        let mut symbols = Vec::new();
        let mut at = 0;
        while at < inputs.len() {
            let character = inputs[at].0;
            if character == '(' {
                let (Some((from, _)), Some((to, _)), Some((')', _))) =
                    (inputs.get(at + 1), inputs.get(at + 2), inputs.get(at + 3))
                else {
                    return Err(bad("an edge that is not `(vw)`"));
                };
                let (Some(from), Some(to)) = (level(*from), level(*to)) else {
                    return Err(bad("an edge of symbols that are not levels"));
                };
                symbols.push(Symbol::Pair(from, to));
                at += 4;
                continue;
            }
            symbols.push(match (level(character), edge(character)) {
                (Some(levels), _) => Symbol::Level(levels),
                (None, Some(pairs)) => Symbol::Edge(pairs),
                (None, None) => return Err(bad(&format!("the symbol `{character}`"))),
            });
            at += 1;
        }
        Ok(Row {
            inputs: symbols,
            state,
            output: output(next)?,
        })
    }
}

/// A level symbol (table 8-1).
fn level(character: char) -> Option<Levels> {
    Some(match character.to_ascii_lowercase() {
        '0' => ZERO,
        '1' => ONE,
        'x' => UNKNOWN,
        '?' => ANY,
        'b' => ZERO | ONE,
        _ => return None,
    })
}

/// An edge abbreviation (table 8-1).
fn edge(character: char) -> Option<&'static [(Levels, Levels)]> {
    const ALL: &[(Levels, Levels)] = &[
        (ZERO, ONE),
        (ZERO, UNKNOWN),
        (ONE, ZERO),
        (ONE, UNKNOWN),
        (UNKNOWN, ZERO),
        (UNKNOWN, ONE),
    ];
    Some(match character.to_ascii_lowercase() {
        'r' => &[(ZERO, ONE)],
        'f' => &[(ONE, ZERO)],
        'p' => &[(ZERO, ONE), (ZERO, UNKNOWN), (UNKNOWN, ONE)],
        'n' => &[(ONE, ZERO), (ONE, UNKNOWN), (UNKNOWN, ZERO)],
        '*' => ALL,
        _ => return None,
    })
}

/// Expression and statement builders, every node at the instance's span.
struct Build {
    span: Span,
}

impl Build {
    fn name(&self, name: &str) -> Expression {
        Expression::Identifier(Identifier {
            name: name.into(),
            span: self.span,
        })
    }

    fn bit(&self, value: char) -> Expression {
        let value =
            crate::four_state::decode(&format!("1'b{value}")).expect("a one-bit literal decodes");
        Expression::Digital(DigitalExpr::FourState(FourStateLit {
            value,
            span: self.span,
        }))
    }

    fn case_equal(&self, left: &Expression, value: char, negate: bool) -> Expression {
        self.identical(left.clone(), self.bit(value), negate)
    }

    /// `left === right`, or `!==` when `negate`.
    fn identical(&self, left: Expression, right: Expression, negate: bool) -> Expression {
        Expression::Digital(DigitalExpr::CaseEquality(CaseEqualityExpr {
            left: Box::new(left),
            right: Box::new(right),
            negate,
            span: self.span,
        }))
    }

    fn binary(&self, op: BinaryOp, left: Expression, right: Expression) -> Expression {
        Expression::Binary(BinaryExpr {
            op,
            left: Box::new(left),
            right: Box::new(right),
            span: self.span,
        })
    }

    fn all(&self, terms: Vec<Expression>) -> Option<Expression> {
        terms
            .into_iter()
            .reduce(|left, right| self.binary(BinaryOp::And, left, right))
    }

    /// Whether `value` lies in `levels`, reading `z` as `x`; `None` for every
    /// value.
    fn within(&self, value: &Expression, levels: Levels) -> Option<Expression> {
        if levels & ANY == ANY {
            return None;
        }
        if levels & UNKNOWN != 0 {
            // Neither of the known values outside the set.
            return self.all(
                VALUES[..2]
                    .iter()
                    .filter(|(member, _)| levels & member == 0)
                    .map(|(_, spelling)| self.case_equal(value, *spelling, true))
                    .collect(),
            );
        }
        VALUES[..2]
            .iter()
            .filter(|(member, _)| levels & member != 0)
            .map(|(_, spelling)| self.case_equal(value, *spelling, false))
            .reduce(|left, right| self.binary(BinaryOp::Or, left, right))
    }

    fn select(&self, rows: Vec<(Option<Expression>, Expression)>) -> Expression {
        rows.into_iter().rev().fold(
            self.bit('x'),
            |otherwise, (condition, value)| match condition {
                Some(condition) => Expression::Conditional(ConditionalExpr {
                    condition: Box::new(condition),
                    then_expr: Box::new(value),
                    else_expr: Box::new(otherwise),
                    span: self.span,
                }),
                None => value,
            },
        )
    }

    fn assign(&self, target: &str, value: Expression) -> DigitalStatement {
        DigitalStatement::BlockingAssign(DigitalAssign {
            target: DigitalLValue::Identifier {
                name: target.into(),
                span: self.span,
            },
            timing: None,
            value,
            span: self.span,
        })
    }

    fn block(&self, statements: Vec<DigitalStatement>) -> DigitalStatement {
        DigitalStatement::Block(DigitalBlock {
            name: None,
            variables: Vec::new(),
            digital_variables: Vec::new(),
            statements,
            span: self.span,
        })
    }

    fn reg(&self, name: &str, init: Option<Expression>) -> DigitalVariableDecl {
        DigitalVariableDecl {
            kind: DigitalVariableKind::Reg,
            signedness: Signedness::Unsigned,
            range: None,
            items: vec![DigitalDeclItem {
                name: name.into(),
                dimensions: Vec::new(),
                init,
                span: self.span,
            }],
            span: self.span,
        }
    }
}

impl UdpDefinition {
    fn output(&self, build: &Build, output: Output, state: &Expression) -> Expression {
        match output {
            Output::Zero => build.bit('0'),
            Output::One => build.bit('1'),
            Output::Unknown => build.bit('x'),
            Output::Unchanged => state.clone(),
        }
    }

    /// The rows with no edge, in the order that settles a conflict as Icarus
    /// Verilog does: those giving 0, then 1, then `x`, then `-`.
    fn ordered(&self, edges: bool) -> Vec<&Row> {
        let mut rows: Vec<&Row> = self
            .rows
            .iter()
            .filter(|row| row.inputs.iter().any(|symbol| symbol.is_edge()) == edges)
            .collect();
        rows.sort_by_key(|row| row.output);
        rows
    }

    /// The combinational output of `inputs`.
    fn combinational(&self, build: &Build, inputs: &[Expression]) -> Expression {
        let x = build.bit('x');
        let rows = self
            .ordered(false)
            .into_iter()
            .map(|row| {
                let terms = row
                    .inputs
                    .iter()
                    .zip(inputs)
                    .filter_map(|(symbol, input)| match symbol {
                        Symbol::Level(levels) => build.within(input, *levels),
                        _ => None,
                    })
                    .collect();
                (build.all(terms), self.output(build, row.output, &x))
            })
            .collect();
        build.select(rows)
    }

    /// The next state after input `changed` moved from `previous` to
    /// `current`, the other inputs reading `seen`: the values the primitive
    /// took from them, every change delivered before this one included.
    fn next_state(
        &self,
        build: &Build,
        changed: usize,
        (previous, current): (&Expression, &Expression),
        seen: &[Expression],
        state: &Expression,
    ) -> Expression {
        let view = |column: usize| {
            if column == changed {
                current
            } else {
                &seen[column]
            }
        };
        let mut rows = Vec::new();
        for row in self.ordered(false) {
            let mut terms: Vec<Expression> = row
                .inputs
                .iter()
                .enumerate()
                .filter_map(|(column, symbol)| match symbol {
                    Symbol::Level(levels) => build.within(view(column), *levels),
                    _ => None,
                })
                .collect();
            terms.extend(build.within(state, row.state));
            rows.push((build.all(terms), self.output(build, row.output, state)));
        }
        for row in self.ordered(true) {
            let Some(edge) = row.inputs.get(changed).filter(|symbol| symbol.is_edge()) else {
                continue;
            };
            let pairs = edge
                .pairs()
                .into_iter()
                .map(|(from, to)| {
                    let spelling = |levels: Levels| {
                        VALUES
                            .iter()
                            .find(|(member, _)| *member == levels)
                            .map_or('x', |(_, spelling)| *spelling)
                    };
                    build.binary(
                        BinaryOp::And,
                        build.case_equal(previous, spelling(from), false),
                        build.case_equal(current, spelling(to), false),
                    )
                })
                .reduce(|left, right| build.binary(BinaryOp::Or, left, right));
            let Some(pairs) = pairs else {
                continue;
            };
            let mut terms = vec![pairs];
            for (column, symbol) in row.inputs.iter().enumerate() {
                if let (true, Symbol::Level(levels)) = (column != changed, symbol) {
                    terms.extend(build.within(&seen[column], *levels));
                }
            }
            terms.extend(build.within(state, row.state));
            rows.push((build.all(terms), self.output(build, row.output, state)));
        }
        build.select(rows)
    }
}

/// Replace every instance of a primitive in `items` with what it runs as.
pub(super) fn expand_instances(
    items: &mut [Item],
    primitives: &[UdpDefinition],
) -> Result<(), ParseError> {
    let primitives: HashMap<&str, &UdpDefinition> = primitives
        .iter()
        .map(|primitive| (primitive.name.as_str(), primitive))
        .collect();
    for item in items {
        let Item::Module(module) = item else {
            continue;
        };
        let mut next_process = module
            .digital_processes
            .iter()
            .map(|process| process.id.0 + 1)
            .max()
            .unwrap_or(0);
        let instances = std::mem::take(&mut module.instances);
        let mut unnamed = 0usize;
        for mut instance in instances {
            let Some(primitive) = primitives.get(instance.module.as_str()) else {
                if instance.name.is_empty() {
                    return Err(refusal(
                        &format!("instance of `{}`", instance.module),
                        "no instance name; IEEE 1364-2005 section 12.1.2 names every module \
                         instance, and only a primitive's may be left out",
                        instance.span,
                    ));
                }
                module.instances.push(instance);
                continue;
            };
            if instance.name.is_empty() {
                unnamed += 1;
                instance.name = format!("{}${unnamed}", primitive.name).into();
            }
            instantiate(module, primitive, &instance, &mut next_process)?;
        }
    }
    Ok(())
}

fn instantiate(
    module: &mut Module,
    primitive: &UdpDefinition,
    instance: &ModuleInstance,
    next_process: &mut u32,
) -> Result<(), ParseError> {
    let context = format!(
        "instance `{}` of primitive `{}`",
        instance.name, primitive.name
    );
    let span = instance.span;
    if instance.array.is_some() {
        return Err(refusal(
            &context,
            "an array of primitive instances (IEEE 1364-2005 section 8.6)",
            span,
        ));
    }
    // Connected in order, or by the port names the primitive declares.
    let named = instance
        .connections
        .iter()
        .any(|connection| matches!(connection, Connection::Named { .. }));
    let mut terminals = if named {
        vec![None; primitive.ports.len()]
    } else {
        Vec::new()
    };
    for connection in &instance.connections {
        match connection {
            Connection::Ordered { signal, .. } if !named => terminals.push(signal.clone()),
            Connection::Named { port, signal, .. } => {
                let position = primitive
                    .ports
                    .iter()
                    .position(|declared| declared == port)
                    .ok_or_else(|| {
                        refusal(&context, format!("a connection to no port `{port}`"), span)
                    })?;
                terminals[position] = signal.clone();
            }
            Connection::Ordered { .. } => {
                return Err(refusal(
                    &context,
                    "ordered and named connections together (IEEE 1364-2005 section 12.3.6)",
                    span,
                ));
            }
        }
    }
    if terminals.len() != primitive.inputs + 1 {
        return Err(refusal(
            &context,
            format!(
                "{} terminals; the primitive has an output and {} inputs",
                terminals.len(),
                primitive.inputs
            ),
            span,
        ));
    }
    // Section 8.6: a rise and a fall delay at most, on the output's driver
    // with a gate's rules (section 7.14).
    let delay = match instance.parameters.as_slice() {
        [] => None,
        delays if delays.len() <= 2 && delays.iter().all(|delay| delay.name.is_none()) => {
            Some(Delay3 {
                values: delays.iter().map(|delay| delay.value.clone()).collect(),
                span,
            })
        }
        _ => {
            return Err(refusal(
                &context,
                "a named delay, or more than a rise and a fall delay, on a primitive \
                 instance (IEEE 1364-2005 section 8.6)",
                span,
            ));
        }
    };
    let build = Build { span };
    let Some(Some(output)) = terminals.first() else {
        return Err(refusal(&context, "an unconnected output", span));
    };
    let target = output_lvalue(output).ok_or_else(|| {
        refusal(
            &context,
            "an output that is not a net, a bit select or a part select of one",
            span,
        )
    })?;
    // An unconnected input reads `z`, and so `x`.
    let inputs: Vec<Expression> = terminals[1..]
        .iter()
        .map(|terminal| terminal.clone().unwrap_or_else(|| build.bit('z')))
        .collect();

    let value = if primitive.sequential {
        let prefix = format!("{}$udp", instance.name);
        let state = format!("{prefix}_q");
        let current = format!("{prefix}_c");
        let seen: Vec<String> = (0..inputs.len())
            .map(|k| format!("{prefix}_i{k}"))
            .collect();
        module.digital_variables.push(build.reg(&state, None));
        module.digital_variables.push(build.reg(&current, None));
        for name in &seen {
            module.digital_variables.push(build.reg(name, None));
        }
        let state_value = build.name(&state);
        let current_value = build.name(&current);
        let seen_values: Vec<Expression> = seen.iter().map(|name| build.name(name)).collect();
        for (k, input) in inputs.iter().enumerate() {
            let next = primitive.next_state(
                &build,
                k,
                (&seen_values[k], &current_value),
                &seen_values,
                &state_value,
            );
            let statements = vec![
                // `z` reads as `x`: `v ^ 1'b0` is `v` for a known value.
                build.assign(
                    &current,
                    build.binary(BinaryOp::BitXor, input.clone(), build.bit('0')),
                ),
                DigitalStatement::Conditional(DigitalConditional {
                    condition: build.identical(current_value.clone(), seen_values[k].clone(), true),
                    then_branch: Box::new(build.block(vec![
                        build.assign(&state, next),
                        build.assign(&seen[k], current_value.clone()),
                    ])),
                    else_branch: None,
                    span,
                }),
            ];
            let mut body = Vec::new();
            // The `initial` value is set as an inactive event at time zero,
            // after every process has started (section 8.5), so one waiting on
            // the output sees it arrive, as Icarus Verilog schedules it. Every
            // input's process waits for it, so the first changes they compare
            // still meet the state it sets, in input order.
            if let Some(value) = primitive.initial {
                body.push(DigitalStatement::Timing(DigitalTiming {
                    control: TimingControl::Delay(DelayControl {
                        value: Expression::Number(NumberLit {
                            value: 0.0,
                            raw: "0".into(),
                            span,
                        }),
                        span,
                    }),
                    statement: (k == 0).then(|| Box::new(build.assign(&state, build.bit(value)))),
                    span,
                }));
            }
            if is_literal(input) {
                body.extend(statements);
            } else {
                let mut statements = statements;
                statements.push(DigitalStatement::Timing(DigitalTiming {
                    control: TimingControl::Event(EventControl {
                        repeat: None,
                        sensitivity: Sensitivity::Explicit(vec![EventTerm {
                            edge: None,
                            signal: input.clone(),
                            span,
                        }]),
                        span,
                    }),
                    statement: None,
                    span,
                }));
                body.push(DigitalStatement::Forever(DigitalForever {
                    body: Box::new(build.block(statements)),
                    span,
                }));
            }
            module.digital_processes.push(DigitalProcess {
                id: DigitalProcessId(*next_process),
                kind: DigitalProcessKind::Initial,
                body: build.block(body),
                generate_scope: None,
                span,
            });
            *next_process += 1;
        }
        state_value
    } else {
        primitive.combinational(&build, &inputs)
    };
    module.continuous_assigns.push(ContinuousAssign {
        target,
        value,
        delay,
        strength: None,
        gate: None,
        span,
    });
    Ok(())
}

fn is_literal(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::Number(_) | Expression::Digital(DigitalExpr::FourState(_))
    )
}

/// A primitive's output terminal as the net it drives.
fn output_lvalue(terminal: &Expression) -> Option<DigitalLValue> {
    Some(match terminal {
        Expression::Identifier(identifier) => DigitalLValue::Identifier {
            name: identifier.name.clone(),
            span: identifier.span,
        },
        Expression::ArrayAccess(access) if access.packed.is_none() => DigitalLValue::BitSelect {
            name: access.array.clone(),
            index: access.index.clone(),
            span: access.span,
        },
        Expression::Digital(DigitalExpr::PartSelect(select)) => DigitalLValue::PartSelect {
            name: select.name.clone(),
            msb: select.msb.clone(),
            lsb: select.lsb.clone(),
            span: select.span,
        },
        _ => return None,
    })
}
