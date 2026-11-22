//! Calculator Expression Parser
//!
//! Parses strings like `V(out) * 2 + avg(V(in))` into a `CalculatorExpr` AST.
//! Uses a recursive descent parser with precedence climbing.
//!
//! `/` is both the division operator and the first character of a hierarchical
//! signal name, and position is what tells them apart: a `/` that follows a
//! value — a number, an identifier, or a closing parenthesis — divides, and a
//! `/` anywhere else opens a name. So `V(a)/V(b)` and `a/b` still divide while
//! `V(/X1/out)` reads one signal.

use super::ast::{BinaryOp, CalculatorConstant, CalculatorExpr, Cursor, UnaryOp};
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    position: usize,
    message: String,
}

impl ParseError {
    fn new(position: usize, message: impl Into<String>) -> Self {
        Self {
            position,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at character {}", self.message, self.position + 1)
    }
}

impl std::error::Error for ParseError {}

/// The quoted words a measurement function reads as an option rather than a
/// signal: edge types, pulse levels and bandwidth shapes, spelled as ViVA
/// spells them. The set is closed so that every other quoted text keeps
/// naming a signal.
pub(super) const OPTION_WORDS: &[&str] = &[
    "rising",
    "falling",
    "either",
    "high",
    "low",
    "band",
    "population",
    "sample",
    "plot",
    "sd",
];

// =============================================================================
// Lexer
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    /// A quoted option word ([`OPTION_WORDS`]), already lower-cased.
    Keyword(String),
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Caret, // ^ or **
    /// A comparison: `<`, `>`, `<=`, `>=`, `==` or `!=`.
    Compare(BinaryOp),
    // Delimiters
    LParen,
    RParen,
    Comma,
    /// `@`, which names the run a signal is read from.
    At,
    // End of input
    Eof,
}

struct Lexer<'a> {
    chars: Peekable<CharIndices<'a>>,
    len: usize,
    parse_number: fn(&str) -> Result<f64, String>,
    /// Whether the token just produced ends a value, which is the whole of the
    /// rule that separates division from a hierarchical name.
    after_value: bool,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str, parse_number: fn(&str) -> Result<f64, String>) -> Self {
        Self {
            chars: input.char_indices().peekable(),
            len: input.len(),
            parse_number,
            after_value: false,
        }
    }

    fn position(&mut self) -> usize {
        self.chars.peek().map(|(idx, _)| *idx).unwrap_or(self.len)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<f64, ParseError> {
        let start = self.position();
        let mut s = String::new();
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_ascii_digit()
                || c == '.'
                || c == 'e'
                || c == 'E'
                || (c == '+' || c == '-') && (s.ends_with('e') || s.ends_with('E'))
                || c.is_ascii_alphabetic()
                || c == '\u{00b5}'
                || c == '\u{03bc}'
            {
                if let Some((_, consumed)) = self.chars.next() {
                    s.push(consumed);
                }
            } else {
                break;
            }
        }
        let normalized = s.replace(['\u{00b5}', '\u{03bc}'], "u");
        (self.parse_number)(&normalized).map_err(|err| ParseError::new(start, err))
    }

    /// Reads one identifier. A signal name carries the engine's hierarchy
    /// qualifiers `.` and `:`; `hierarchical` additionally takes `/`, and is
    /// true only in prefix position, where no division can begin.
    fn read_ident(&mut self, hierarchical: bool) -> String {
        let mut s = String::new();
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_alphanumeric() || matches!(c, '_' | '.' | ':') || (hierarchical && c == '/') {
                if let Some((_, consumed)) = self.chars.next() {
                    s.push(consumed);
                }
            } else {
                break;
            }
        }
        s
    }

    fn read_string_literal(&mut self) -> Result<String, ParseError> {
        let start = self.position();
        let mut s = String::new();
        if self.chars.peek().is_some_and(|(_, c)| *c == '"') {
            self.chars.next(); // consume opening "
            while let Some((_, c)) = self.chars.next() {
                if c == '"' {
                    return Ok(s);
                }
                if c == '\\' {
                    let Some((_, escaped)) = self.chars.next() else {
                        return Err(ParseError::new(start, "unterminated string literal"));
                    };
                    s.push(escaped);
                } else {
                    s.push(c);
                }
            }
        }
        Err(ParseError::new(start, "unterminated string literal"))
    }

    /// A comparison operator. A lone `=` or `!` is refused with the spelling
    /// meant, since neither assigns or negates here.
    fn read_comparison(&mut self, position: usize, first: char) -> Result<Token, ParseError> {
        self.chars.next();
        let equals = self.chars.peek().is_some_and(|(_, c)| *c == '=');
        if equals {
            self.chars.next();
        }
        Ok(Token::Compare(match (first, equals) {
            ('<', false) => BinaryOp::Lt,
            ('<', true) => BinaryOp::Le,
            ('>', false) => BinaryOp::Gt,
            ('>', true) => BinaryOp::Ge,
            ('=', true) => BinaryOp::Eq,
            ('!', true) => BinaryOp::Ne,
            _ => {
                return Err(ParseError::new(
                    position,
                    format!("unexpected '{first}'; compare with == or !="),
                ));
            }
        }))
    }

    /// V/I arguments name signals. Read their spelling before the arithmetic
    /// lexer can interpret `-`, `+`, a SPICE suffix, or an overflowing exponent.
    /// Returns where the argument's spelling starts, quotes included.
    fn read_probe_argument(&mut self) -> Result<(usize, String), ParseError> {
        self.skip_whitespace();
        let start = self.position();
        if self.chars.peek().is_some_and(|(_, c)| *c == '"') {
            return Ok((start, self.read_string_literal()?));
        }
        let mut name = String::new();
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_whitespace() || matches!(c, '(' | ')' | ',' | '"') {
                break;
            }
            self.chars.next();
            name.push(c);
        }
        Ok((start, name))
    }

    fn next_token(&mut self) -> Result<SpannedToken, ParseError> {
        self.skip_whitespace();

        let position = self.position();
        let token = match self.chars.peek() {
            None => Token::Eof,
            Some(&(_, c)) => match c {
                '0'..='9' | '.' => Token::Number(self.read_number()?),
                'a'..='z' | 'A'..='Z' | '_' => Token::Ident(self.read_ident(false)),
                // A leading separator opens a canonical hierarchical name.
                '/' if !self.after_value => Token::Ident(self.read_ident(true)),
                '"' => {
                    let text = self.read_string_literal()?;
                    let word = text.to_ascii_lowercase();
                    if OPTION_WORDS.contains(&word.as_str()) {
                        Token::Keyword(word)
                    } else {
                        Token::Ident(text)
                    }
                }
                '+' => {
                    self.chars.next();
                    Token::Plus
                }
                '-' => {
                    self.chars.next();
                    Token::Minus
                }
                '*' => {
                    self.chars.next();
                    if self.chars.peek().is_some_and(|(_, c)| *c == '*') {
                        self.chars.next();
                        Token::Caret // Treat ** as ^
                    } else {
                        Token::Star
                    }
                }
                '/' => {
                    self.chars.next();
                    Token::Slash
                }
                '^' => {
                    self.chars.next();
                    Token::Caret
                }
                '<' | '>' | '=' | '!' => self.read_comparison(position, c)?,
                '(' => {
                    self.chars.next();
                    Token::LParen
                }
                ')' => {
                    self.chars.next();
                    Token::RParen
                }
                ',' => {
                    self.chars.next();
                    Token::Comma
                }
                '@' => {
                    self.chars.next();
                    Token::At
                }
                _ => {
                    return Err(ParseError::new(
                        position,
                        format!("unexpected character '{}'", c),
                    ));
                }
            },
        };

        self.after_value = matches!(
            token,
            Token::Number(_) | Token::Ident(_) | Token::Keyword(_) | Token::RParen
        );
        Ok(SpannedToken { token, position })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SpannedToken {
    token: Token,
    position: usize,
}

// =============================================================================
// Parser
// =============================================================================

/// One `V(...)` or `I(...)` argument as the parser read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeArgument {
    /// A voltage names a node; a current names a device or branch.
    pub voltage: bool,
    /// The name the argument reads, quotes removed.
    pub name: String,
    /// The bytes of the input that spell it, quotes included.
    pub span: std::ops::Range<usize>,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: SpannedToken,
    initial_error: Option<ParseError>,
    probe_arguments: Vec<ProbeArgument>,
}

impl<'a> Parser<'a> {
    /// Parse expression syntax using the caller's canonical numeric literal policy.
    pub fn new(input: &'a str, parse_number: fn(&str) -> Result<f64, String>) -> Self {
        let mut lexer = Lexer::new(input, parse_number);
        let (current, initial_error) = match lexer.next_token() {
            Ok(token) => (token, None),
            Err(error) => (
                SpannedToken {
                    position: error.position,
                    token: Token::Eof,
                },
                Some(error),
            ),
        };
        Self {
            lexer,
            current,
            initial_error,
            probe_arguments: Vec::new(),
        }
    }

    /// Parse the input, and say where each probe argument it names is spelled,
    /// in source order.
    ///
    /// The argument a probe reads is a name, and only the parser knows where
    /// one starts and ends: `V(out-)` names `out-`, `V(a)-V(b)` subtracts, and
    /// `/` divides or opens a hierarchical name by position. A caller that has
    /// to spell a name differently replaces exactly these bytes.
    pub fn try_parse_probe_arguments(&mut self) -> Result<Vec<ProbeArgument>, ParseError> {
        self.try_parse()?;
        Ok(std::mem::take(&mut self.probe_arguments))
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.current = self.lexer.next_token()?;
        Ok(())
    }

    /// Parse the input, or say why it cannot be parsed.
    ///
    /// There is deliberately no infallible sibling. One used to exist and
    /// recovered a syntax error to the literal `0`, so `V(out) +` evaluated,
    /// printed `= 0.0000` in the success colour, and was filed in the recall
    /// history as a working expression.
    pub fn try_parse(&mut self) -> Result<CalculatorExpr, ParseError> {
        if let Some(error) = self.initial_error.take() {
            return Err(error);
        }
        if matches!(self.current.token, Token::Eof) {
            return Err(ParseError::new(0, "empty expression"));
        }
        let expr = self.parse_comparison()?;
        if !matches!(self.current.token, Token::Eof) {
            return Err(ParseError::new(
                self.current.position,
                format!(
                    "unexpected {} after expression",
                    describe_token(&self.current.token)
                ),
            ));
        }
        Ok(expr)
    }

    // Precedence: (low to high)
    // 1. <, >, <=, >=, ==, !=
    // 2. +, -
    // 3. *, /
    // 4. ^
    // 5. unary -, unary +
    // 6. primary (parens, numbers, idents, calls)

    fn parse_comparison(&mut self) -> Result<CalculatorExpr, ParseError> {
        let mut left = self.parse_additive()?;
        while let Token::Compare(op) = self.current.token {
            self.advance()?;
            let right = self.parse_additive()?;
            left = CalculatorExpr::binary(op, left, right);
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<CalculatorExpr, ParseError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.current.token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_multiplicative()?;
            left = CalculatorExpr::binary(op, left, right);
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<CalculatorExpr, ParseError> {
        let mut left = self.parse_power()?;

        loop {
            let op = match self.current.token {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_power()?;
            left = CalculatorExpr::binary(op, left, right);
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<CalculatorExpr, ParseError> {
        let left = self.parse_unary()?;

        if matches!(self.current.token, Token::Caret) {
            self.advance()?;
            let right = self.parse_power()?; // Right associative
            Ok(CalculatorExpr::binary(BinaryOp::Pow, left, right))
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<CalculatorExpr, ParseError> {
        match self.current.token {
            Token::Minus => {
                self.advance()?;
                Ok(CalculatorExpr::unary(UnaryOp::Neg, self.parse_unary()?))
            }
            Token::Plus => {
                self.advance()?; // Unary plus is no-op
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<CalculatorExpr, ParseError> {
        match self.current.token.clone() {
            Token::Number(val) => {
                self.advance()?;
                Ok(CalculatorExpr::Number(val))
            }
            Token::Ident(name) => {
                self.advance()?;
                self.parse_ident_or_call(&name)
            }
            Token::Keyword(word) => {
                self.advance()?;
                Ok(CalculatorExpr::Keyword(word))
            }
            Token::LParen => {
                let start = self.current.position;
                self.advance()?;
                let expr = self.parse_comparison()?; // Reset to lowest precedence
                self.expect_rparen(start)?;
                Ok(expr)
            }
            _ => Err(ParseError::new(
                self.current.position,
                format!(
                    "expected expression, found {}",
                    describe_token(&self.current.token)
                ),
            )),
        }
    }

    fn expect_rparen(&mut self, start_position: usize) -> Result<(), ParseError> {
        if matches!(self.current.token, Token::RParen) {
            self.advance()?;
            Ok(())
        } else {
            Err(ParseError::new(
                start_position,
                format!("missing ')' before {}", describe_token(&self.current.token)),
            ))
        }
    }

    fn parse_ident_or_call(&mut self, name: &str) -> Result<CalculatorExpr, ParseError> {
        let upper_name = name.to_uppercase();

        // 1. Handle V(node) and I(branch) special forms
        if (upper_name == "V" || upper_name == "I") && matches!(self.current.token, Token::LParen) {
            let start = self.current.position;
            let argument_start = self.lexer.position();
            let (spelling_start, arg) = self.lexer.read_probe_argument()?;
            if arg.trim().is_empty() {
                return Err(ParseError::new(
                    argument_start,
                    format!("expected signal name inside {}(...)", upper_name),
                ));
            }
            self.probe_arguments.push(ProbeArgument {
                voltage: upper_name == "V",
                name: arg.clone(),
                span: spelling_start..self.lexer.position(),
            });
            self.advance()?; // consume arg
            self.expect_rparen(start)?;

            // Construct signal name: e.g. "V(out)" or "I(R1)"
            let signal_name = format!("{}({})", upper_name, arg);
            return self.signal_of_run(signal_name);
        }

        // A digital net or bus, `D(q[3:0])`: its name is read literally, as
        // a probe argument is, so a bus range is part of it. It is no node
        // voltage or branch current, so it is not listed with those.
        if upper_name == "D" && matches!(self.current.token, Token::LParen) {
            let start = self.current.position;
            let argument_start = self.lexer.position();
            let (_, arg) = self.lexer.read_probe_argument()?;
            if arg.trim().is_empty() {
                return Err(ParseError::new(
                    argument_start,
                    "expected a net or bus name inside D(...)".to_owned(),
                ));
            }
            self.advance()?;
            self.expect_rparen(start)?;
            return self.signal_of_run(format!("D({arg})"));
        }

        // 2. Constants — but only where the name stands alone. `freq` is the
        //    AC sweep's own axis; `freq(V(out))` is the measurement function
        //    that reports a signal's repetition rate, and the parenthesis
        //    that follows is what tells the two apart. Claiming the constant
        //    first made every `freq(...)` call a syntax error.
        if !matches!(self.current.token, Token::LParen) {
            if upper_name == "TIME" {
                return Ok(CalculatorExpr::Constant(CalculatorConstant::Time));
            }
            if upper_name == "FREQ" {
                return Ok(CalculatorExpr::Constant(CalculatorConstant::Frequency));
            }
            // The cursor pair's positions on the x-axis, spelled exactly, so
            // a node named `xa` keeps its bare name.
            if name == "xA" {
                return Ok(CalculatorExpr::Constant(CalculatorConstant::Cursor(
                    Cursor::A,
                )));
            }
            if name == "xB" {
                return Ok(CalculatorExpr::Constant(CalculatorConstant::Cursor(
                    Cursor::B,
                )));
            }
            // A placed marker's coordinate, `m1.x` or `m1.y`, as ADS reads
            // a marker by its name.
            if let Some(reading) = super::ast::MarkerReading::parse(name) {
                return Ok(CalculatorExpr::Constant(CalculatorConstant::Marker(
                    reading,
                )));
            }
        }

        // 3. Function Calls
        if matches!(self.current.token, Token::LParen) {
            let start = self.current.position;
            self.advance()?; // consume (
            let mut args = Vec::new();

            if !matches!(self.current.token, Token::RParen) {
                args.push(self.parse_comparison()?);
                while matches!(self.current.token, Token::Comma) {
                    self.advance()?;
                    args.push(self.parse_comparison()?);
                }
            }

            self.expect_rparen(start)?;
            return Ok(CalculatorExpr::func(name, args));
        }

        // 4. Naked Identifier -> Assume it's a signal reference if nothing else matches?
        // Or maybe a named parameter/variable in future.
        // For now, treat as Waveform Reference (e.g. "out" implies "V(out)"? No, be strict)
        // Actually, maybe it's a variable or alias. Let's treat as WaveformRef for flexibility
        self.signal_of_run(name.to_owned())
    }

    /// A signal, and the run it is read from when `@` names one: the pinned
    /// baseline, `V(out)@baseline`, or a retained run by its name,
    /// `V(out)@"Run 3"`. Without `@` it is read from the result evaluated.
    fn signal_of_run(&mut self, signal: String) -> Result<CalculatorExpr, ParseError> {
        if !matches!(self.current.token, Token::At) {
            return Ok(CalculatorExpr::WaveformRef {
                signal,
                dataset: None,
            });
        }
        let at = self.current.position;
        self.advance()?;
        match self.current.token.clone() {
            Token::Ident(run) | Token::Keyword(run) if !run.trim().is_empty() => {
                self.advance()?;
                Ok(CalculatorExpr::WaveformRef {
                    signal,
                    dataset: Some(run),
                })
            }
            token => Err(ParseError::new(
                at,
                format!(
                    "expected a run after '@', such as baseline or \"Run 3\", found {}",
                    describe_token(&token)
                ),
            )),
        }
    }
}

fn describe_token(token: &Token) -> String {
    match token {
        Token::Number(_) => "number".to_string(),
        Token::Ident(name) => format!("identifier '{name}'"),
        Token::Keyword(word) => format!("option \"{word}\""),
        Token::Plus => "'+'".to_string(),
        Token::Minus => "'-'".to_string(),
        Token::Star => "'*'".to_string(),
        Token::Slash => "'/'".to_string(),
        Token::Caret => "'^'".to_string(),
        Token::Compare(op) => format!(
            "'{}'",
            match op {
                BinaryOp::Lt => "<",
                BinaryOp::Gt => ">",
                BinaryOp::Le => "<=",
                BinaryOp::Ge => ">=",
                BinaryOp::Eq => "==",
                _ => "!=",
            }
        ),
        Token::LParen => "'('".to_string(),
        Token::RParen => "')'".to_string(),
        Token::Comma => "','".to_string(),
        Token::At => "'@'".to_string(),
        Token::Eof => "end of expression".to_string(),
    }
}

/// Helper to parse a string and report syntax errors.
/// Parse an expression with the caller's numeric literal policy.
pub fn try_parse(
    input: &str,
    parse_number: fn(&str) -> Result<f64, String>,
) -> Result<CalculatorExpr, ParseError> {
    Parser::new(input, parse_number).try_parse()
}

// =============================================================================
// Tests
// =============================================================================
