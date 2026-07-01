//! SPICE netlist lexer/tokenizer using nom
//!
//! Provides robust tokenization for SPICE netlists, handling:
//! - Engineering notation (1k, 1MEG, 1u, 1n, 1p)
//! - Sloppy syntax (commas, missing spaces around `=`)
//! - Parenthesized expressions (PULSE(...), SIN(...))
//! - Parameter expressions ({value+1k})

use crate::Value;

//=============================================================================
// Token Types
//=============================================================================

/// A token with source location information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

/// Source location span
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

/// Token variants
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Identifier (R1, VCC, node_name)
    Ident(String),
    /// Numeric value with optional suffix (1k = 1000, 1u = 1e-6)
    Number(Value),
    /// String literal "..."
    StringLit(String),
    /// Expression in braces {expr}
    Expression(String),
    /// Equals sign
    Equals,
    /// Comma (treated as whitespace in SPICE)
    Comma,
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
    /// Plus sign (for line continuation or expressions)
    Plus,
    /// Minus sign
    Minus,
    /// Asterisk (multiplication or comment start)
    Star,
    /// Forward slash (division)
    Slash,
    /// At sign (for device parameter references @device[param])
    AtSign,
    /// Tilde (for inverted XSPICE digital event ports)
    Tilde,
    /// Left bracket (for parameter indexing)
    LBracket,
    /// Right bracket
    RBracket,
    /// Non-delimiter punctuation, kept for parser contexts such as liberal
    /// ngspice-style XSPICE net names.
    Other(char),
    /// Newline (significant for line structure)
    Newline,
    /// End of input
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::Number(v) => write!(f, "{}", v),
            TokenKind::StringLit(s) => write!(f, "\"{}\"", s),
            TokenKind::Expression(s) => write!(f, "{{{}}}", s),
            TokenKind::Equals => write!(f, "="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::AtSign => write!(f, "@"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::Other(c) => write!(f, "{}", c),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

//=============================================================================
// Lexer State
//=============================================================================

/// Lexer for SPICE netlists
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while self.pos < self.input.len() {
            // Skip horizontal whitespace (not newlines)
            self.skip_whitespace();

            if self.pos >= self.input.len() {
                break;
            }

            let remaining = &self.input[self.pos..];
            let start_pos = self.pos;
            let start_line = self.line;

            // Try to match a token
            let (kind, consumed) = self.next_token(remaining)?;

            let span = Span {
                start: start_pos,
                end: start_pos + consumed,
                line: start_line,
            };
            let lexeme = self.input[start_pos..span.end].to_string();

            // Track newlines for line counting
            if matches!(kind, TokenKind::Newline) {
                self.line += 1;
            }

            tokens.push(Token { kind, span, lexeme });
            self.pos += consumed;
        }

        // Add EOF token
        tokens.push(Token {
            kind: TokenKind::Eof,
            span: Span {
                start: self.pos,
                end: self.pos,
                line: self.line,
            },
            lexeme: String::new(),
        });

        Ok(tokens)
    }

    /// Skip horizontal whitespace (spaces and tabs, not newlines)
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c == ' ' || c == '\t' || c == '\r' {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }

    /// Parse the next token, returning (TokenKind, bytes_consumed)
    fn next_token(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let c = input.chars().next().unwrap();

        match c {
            '\n' => Ok((TokenKind::Newline, 1)),
            '=' => Ok((TokenKind::Equals, 1)),
            ',' => Ok((TokenKind::Comma, 1)),
            '(' => Ok((TokenKind::LParen, 1)),
            ')' => Ok((TokenKind::RParen, 1)),
            '+' => Ok((TokenKind::Plus, 1)),
            '-' => {
                // Could be minus or start of negative number
                if input.len() > 1 {
                    let next = input[1..].chars().next();
                    if matches!(next, Some('0'..='9')) {
                        return self.parse_number_or_ident(input);
                    }
                }
                Ok((TokenKind::Minus, 1))
            }
            '*' => Ok((TokenKind::Star, 1)),
            '/' => Ok((TokenKind::Slash, 1)),
            '@' => Ok((TokenKind::AtSign, 1)),
            '~' => Ok((TokenKind::Tilde, 1)),
            '[' => Ok((TokenKind::LBracket, 1)),
            ']' => Ok((TokenKind::RBracket, 1)),
            '{' => self.parse_expression(input),
            '\'' => self.parse_quoted_expression(input), // ngspice-style '1+2'
            '"' => self.parse_string(input),
            '.' => {
                // Could be a dot-command (.PARAM, .OP) or a decimal number
                if input.len() > 1 {
                    let next = input[1..].chars().next();
                    if matches!(next, Some('0'..='9')) {
                        // It's a decimal number like .5
                        self.parse_number(input)
                    } else {
                        // It's a dot-command like .PARAM
                        self.parse_ident(input)
                    }
                } else {
                    Err(LexError::InvalidNumber(".".to_string(), self.line))
                }
            }
            '0'..='9' => self.parse_number_or_ident(input),
            _ if is_ident_start(c) => self.parse_ident(input),
            _ if is_other_token_char(c) => Ok((TokenKind::Other(c), c.len_utf8())),
            _ => Err(LexError::UnexpectedChar(c, self.line)),
        }
    }

    /// Parse an identifier
    fn parse_ident(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut end = 0;
        for c in input.chars() {
            if is_ident_char(c) {
                end += c.len_utf8();
            } else {
                break;
            }
        }

        let ident = &input[..end];
        Ok((TokenKind::Ident(ident.to_uppercase()), end))
    }

    /// Try to parse a number, but keep digit-leading alphanumeric tokens as
    /// identifiers in ambiguous cases. Parser value contexts call
    /// `parse_spice_value` for numeric-looking identifiers, while node/model
    /// contexts need to preserve names like `1A` and `1N4148`.
    fn parse_number_or_ident(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        // Skip optional leading sign
        let mut had_sign = false;
        if i < chars.len() && (chars[i] == '-' || chars[i] == '+') {
            had_sign = true;
            i += 1;
        }

        // Skip digits
        let digit_start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }

        // Unsigned digit-leading names are legal SPICE node/model names
        // (`1A`, `2N2222`). Preserve them as identifiers; value parsers still
        // resolve `1A`, `4U`, `10V`, etc. through parse_spice_value. Keep
        // scientific notation on the numeric path.
        if !had_sign && i < chars.len() && i > digit_start {
            let next_char = chars[i].to_ascii_uppercase();
            let sci_exp = next_char == 'E'
                && i + 1 < chars.len()
                && (chars[i + 1].is_ascii_digit() || chars[i + 1] == '+' || chars[i + 1] == '-');
            if next_char.is_ascii_alphabetic() && !sci_exp {
                return self.parse_ident(input);
            }
        }

        // If followed by a letter that's NOT a valid SPICE suffix start or E (exponent),
        // it's likely a model name like 1N4148
        if i < chars.len() && i > digit_start {
            let next_char = chars[i].to_ascii_uppercase();
            // Check if it could be a model name pattern (digit + letter + more alphanumeric)
            if next_char.is_ascii_alphabetic() && i + 1 < chars.len() {
                let after = chars[i + 1].to_ascii_uppercase();
                // If the letter is followed by more alphanumeric, it's likely a model name
                // E.g., "1N4148" - after 'N' we have '4' which makes it a model name
                // vs "1n" which is just 1 * nano
                // But "1ns" (nanosecond) should be parsed as number - 's' is for 'seconds' unit
                if after.is_ascii_alphanumeric() {
                    // Check if it's a valid suffix pattern
                    let could_be_suffix = match next_char {
                        'E' => after == '+' || after == '-' || after.is_ascii_digit(), // Scientific notation
                        'M' => {
                            // MEG suffix, MS (milliseconds), MHZ (megahertz)
                            if chars.len() > i + 2 {
                                let m2 = chars[i + 1].to_ascii_uppercase();
                                let m3 = chars[i + 2].to_ascii_uppercase();
                                (m2 == 'E' && m3 == 'G')
                                    || m2 == 'S'
                                    || m2 == 'A'
                                    || m2 == 'V'
                                    || (m2 == 'H' && m3 == 'Z')
                            } else {
                                matches!(after, 'S' | 'H' | 'A' | 'V') // MS, MH, mA, mV
                            }
                        }
                        // Unit suffixes followed by seconds/farad/henry and common
                        // source units (A/V).
                        'N' => {
                            // ns, nF, nH, nA, nV - but NOT when followed by digit (like 1N4148)
                            matches!(after, 'S' | 'F' | 'H' | 'A' | 'V')
                        }
                        'P' => {
                            // ps, pF, pH, pA, pV - but NOT when followed by digit
                            matches!(after, 'S' | 'F' | 'H' | 'A' | 'V')
                        }
                        'U' => {
                            // us, uF, uH, uA, uV - but NOT when followed by digit
                            matches!(after, 'S' | 'F' | 'H' | 'A' | 'V')
                        }
                        'F' => {
                            // fs, fA, fV - but NOT when followed by digit
                            matches!(after, 'S' | 'A' | 'V')
                        }
                        'K' => {
                            // kHz
                            if chars.len() > i + 2 {
                                let k2 = chars[i + 1].to_ascii_uppercase();
                                let k3 = chars[i + 2].to_ascii_uppercase();
                                k2 == 'H' && k3 == 'Z'
                            } else {
                                !after.is_ascii_alphabetic() || matches!(after, 'A' | 'V')
                            }
                        }
                        'G' => {
                            // GHz
                            if chars.len() > i + 2 {
                                let g2 = chars[i + 1].to_ascii_uppercase();
                                let g3 = chars[i + 2].to_ascii_uppercase();
                                g2 == 'H' && g3 == 'Z'
                            } else {
                                !after.is_ascii_alphabetic() || matches!(after, 'A' | 'V')
                            }
                        }
                        'T' => {
                            // THz (terahertz)
                            if chars.len() > i + 2 {
                                let t2 = chars[i + 1].to_ascii_uppercase();
                                let t3 = chars[i + 2].to_ascii_uppercase();
                                t2 == 'H' && t3 == 'Z'
                            } else {
                                !after.is_ascii_alphabetic() || matches!(after, 'A' | 'V')
                            }
                        }
                        _ => false,
                    };

                    if !could_be_suffix {
                        // It's a model name - parse as identifier
                        return self.parse_ident(input);
                    }
                }
            }
        }

        // Not a model name, parse as number
        self.parse_number(input)
    }

    /// Parse a numeric value with optional SPICE suffix
    fn parse_number(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut end = 0;
        let mut _has_dot = false;
        let chars: Vec<char> = input.chars().collect();

        // Optional leading sign
        if end < chars.len() && (chars[end] == '-' || chars[end] == '+') {
            end += 1;
        }

        // Integer part
        while end < chars.len() && chars[end].is_ascii_digit() {
            end += 1;
        }

        // Decimal part
        if end < chars.len() && chars[end] == '.' {
            _has_dot = true;
            end += 1;
            while end < chars.len() && chars[end].is_ascii_digit() {
                end += 1;
            }
        }

        // Exponent part (e or E)
        if end < chars.len() && (chars[end] == 'e' || chars[end] == 'E') {
            end += 1;
            // Optional exponent sign
            if end < chars.len() && (chars[end] == '-' || chars[end] == '+') {
                end += 1;
            }
            // Exponent digits
            let exp_start = end;
            while end < chars.len() && chars[end].is_ascii_digit() {
                end += 1;
            }
            if end == exp_start {
                // No exponent digits after E
                return Err(LexError::InvalidNumber(input[..end].to_string(), self.line));
            }
        }

        // Calculate byte position
        let num_end_bytes: usize = chars[..end].iter().map(|c| c.len_utf8()).sum();
        let num_str = &input[..num_end_bytes];

        // Parse the numeric part
        let base: Value =
            if num_str.is_empty() || num_str == "." || num_str == "-" || num_str == "+" {
                return Err(LexError::InvalidNumber(num_str.to_string(), self.line));
            } else {
                num_str
                    .parse()
                    .map_err(|_| LexError::InvalidNumber(num_str.to_string(), self.line))?
            };

        // Check for SPICE suffix. Xyce/ngspice-style decks can append a scale
        // suffix after a valid exponent (`1e3k`, `1e6um`).
        let (multiplier, suffix_len) = if end < chars.len() {
            parse_spice_suffix(&chars[end..])
        } else {
            (1.0, 0)
        };

        let suffix_bytes: usize = chars[end..end + suffix_len]
            .iter()
            .map(|c| c.len_utf8())
            .sum();
        let total_bytes = num_end_bytes + suffix_bytes;

        Ok((TokenKind::Number(base * multiplier), total_bytes))
    }

    /// Parse a braced expression {expr}
    fn parse_expression(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut depth = 0;
        let mut end = 0;

        for c in input.chars() {
            end += c.len_utf8();
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
        }

        if depth != 0 {
            return Err(LexError::UnterminatedExpression(self.line));
        }

        // Extract content without braces
        let content = &input[1..end - 1];
        Ok((TokenKind::Expression(content.to_string()), end))
    }

    /// Parse a single-quoted expression 'expr' (ngspice-style)
    fn parse_quoted_expression(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut end = 1; // Skip opening quote
        let mut terminated = false;

        for c in input[1..].chars() {
            end += c.len_utf8();
            if c == '\'' {
                terminated = true;
                break;
            }
        }

        if !terminated {
            return Err(LexError::UnterminatedExpression(self.line));
        }

        // Extract content without quotes
        let content = &input[1..end - 1];
        Ok((TokenKind::Expression(content.to_string()), end))
    }

    /// Parse a quoted string "..."
    fn parse_string(&self, input: &str) -> Result<(TokenKind, usize), LexError> {
        let mut end = 1; // Skip opening quote
        let mut content = String::new();
        let chars: Vec<char> = input.chars().collect();

        while end < chars.len() {
            let c = chars[end];
            if c == '"' {
                end += 1;
                break;
            } else if c == '\\' && end + 1 < chars.len() {
                // Escape sequence
                end += 1;
                content.push(chars[end]);
                end += 1;
            } else {
                content.push(c);
                end += 1;
            }
        }

        let byte_len: usize = chars[..end].iter().map(|c| c.len_utf8()).sum();
        Ok((TokenKind::StringLit(content), byte_len))
    }
}

//=============================================================================
// Helper Functions
//=============================================================================

/// Check if character can start an identifier
fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c == '%'
}

/// Check if character can be part of an identifier
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '#' || c == ':' || c == '%'
}

/// Check if an otherwise-special character should remain available to parsers.
fn is_other_token_char(c: char) -> bool {
    !c.is_whitespace() && !c.is_control()
}

/// Parse SPICE engineering suffix and return (multiplier, chars_consumed)
fn parse_spice_suffix(chars: &[char]) -> (Value, usize) {
    if chars.is_empty() {
        return (1.0, 0);
    }

    // Try three-char suffixes first
    if chars.len() >= 3 {
        let three: String = chars[..3].iter().collect();
        let three_upper = three.to_uppercase();
        match three_upper.as_str() {
            "MEG" => return (1e6, 3),
            "MIL" => return (25.4e-6, 3), // mil = 1/1000 inch
            "GHZ" => return (1e9, 3),     // gigahertz
            "MHZ" => return (1e6, 3),     // megahertz
            "KHZ" => return (1e3, 3),     // kilohertz
            "THZ" => return (1e12, 3),    // terahertz
            _ => {}
        }
    }

    // Two-char unit suffixes
    if chars.len() >= 2 {
        let c1 = chars[0].to_ascii_uppercase();
        let c2 = chars[1].to_ascii_uppercase();

        // Time units (seconds)
        if c2 == 'S' {
            match c1 {
                'N' => return (1e-9, 2),  // nanoseconds
                'P' => return (1e-12, 2), // picoseconds
                'U' => return (1e-6, 2),  // microseconds
                'M' => return (1e-3, 2),  // milliseconds
                'F' => return (1e-15, 2), // femtoseconds
                _ => {}
            }
        }

        // Capacitance units (farads) - just consume the F, value already scaled
        if c2 == 'F' {
            match c1 {
                'N' => return (1e-9, 2),  // nanofarads
                'P' => return (1e-12, 2), // picofarads
                'U' => return (1e-6, 2),  // microfarads
                'M' => return (1e-3, 2),  // millifarads (rare but valid)
                _ => {}
            }
        }

        // Inductance units (henrys)
        if c2 == 'H' {
            match c1 {
                'N' => return (1e-9, 2),  // nanohenrys
                'P' => return (1e-12, 2), // picohenrys
                'U' => return (1e-6, 2),  // microhenrys
                'M' => return (1e-3, 2),  // millihenrys
                _ => {}
            }
        }

        // Voltage/current units with engineering prefix (e.g., mV, uA, kV).
        if c2 == 'V' || c2 == 'A' {
            match c1 {
                'T' => return (1e12, 2),
                'G' => return (1e9, 2),
                'K' => return (1e3, 2),
                'M' => return (1e-3, 2),
                'U' => return (1e-6, 2),
                'N' => return (1e-9, 2),
                'P' => return (1e-12, 2),
                'F' => return (1e-15, 2),
                _ => {}
            }
        }

        // Length units (meters) with engineering prefix. Bare `m` remains the
        // SPICE milli scale; only a second `m` unit designator is neutral.
        if c2 == 'M' {
            match c1 {
                'T' => return (1e12, 2),
                'G' => return (1e9, 2),
                'K' => return (1e3, 2),
                'M' => return (1e-3, 2),
                'U' => return (1e-6, 2),
                'N' => return (1e-9, 2),
                'P' => return (1e-12, 2),
                'F' => return (1e-15, 2),
                _ => {}
            }
        }
    }

    // Single char suffixes
    let c = chars[0].to_ascii_uppercase();
    match c {
        'T' => (1e12, 1),
        'G' => (1e9, 1),
        'K' => (1e3, 1),
        'M' => (1e-3, 1), // milli (MEG already handled above)
        'U' => (1e-6, 1),
        'N' => (1e-9, 1),
        'P' => (1e-12, 1),
        'F' => (1e-15, 1),
        'X' if chars.len() == 1 || !chars[1].is_ascii_alphabetic() => (1e6, 1),
        // Unit designators (e.g., "1V", "1A", ".1s") are treated as neutral scale.
        'V' | 'A' | 'S' => (1.0, 1),
        _ => (1.0, 0),
    }
}

//=============================================================================
// Lexer Errors
//=============================================================================

/// Errors that can occur during lexing
#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnexpectedChar(char, usize),
    InvalidNumber(String, usize),
    UnterminatedExpression(usize),
    UnterminatedString(usize),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnexpectedChar(c, line) => {
                write!(f, "Unexpected character '{}' at line {}", c, line)
            }
            LexError::InvalidNumber(s, line) => {
                write!(f, "Invalid number '{}' at line {}", s, line)
            }
            LexError::UnterminatedExpression(line) => {
                write!(f, "Unterminated expression at line {}", line)
            }
            LexError::UnterminatedString(line) => {
                write!(f, "Unterminated string at line {}", line)
            }
        }
    }
}

impl std::error::Error for LexError {}

//=============================================================================
// Convenience Functions
//=============================================================================

/// Tokenize a SPICE netlist string
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}

/// Parse a single SPICE value with engineering suffix
pub fn parse_spice_value(s: &str) -> Result<Value, LexError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(LexError::InvalidNumber("empty".to_string(), 0));
    }

    let lexer = Lexer::new(s);
    match lexer.parse_number(s) {
        Ok((TokenKind::Number(v), _)) => Ok(v),
        Ok(_) => Err(LexError::InvalidNumber(s.to_string(), 0)),
        Err(e) => Err(e),
    }
}

//=============================================================================
// Token Stream for Parser
//=============================================================================

/// Token stream for parsing - provides lookahead and consumption
#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    /// Create a new token stream from tokens
    pub fn new(mut tokens: Vec<Token>) -> Self {
        if !matches!(tokens.last().map(|token| &token.kind), Some(TokenKind::Eof)) {
            let span = tokens.last().map_or(
                Span {
                    start: 0,
                    end: 0,
                    line: 1,
                },
                |token| Span {
                    start: token.span.end,
                    end: token.span.end,
                    line: token.span.line,
                },
            );
            tokens.push(Token {
                kind: TokenKind::Eof,
                span,
                lexeme: String::new(),
            });
        }
        Self { tokens, pos: 0 }
    }

    /// Peek at the current token without consuming
    #[inline]
    pub fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    /// Peek at token N positions ahead
    #[inline]
    pub fn peek_n(&self, n: usize) -> &Token {
        let idx = (self.pos + n).min(self.tokens.len() - 1);
        &self.tokens[idx]
    }

    /// Check if current token matches a kind
    #[inline]
    pub fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    /// Consume and return the current token
    pub fn advance(&mut self) -> &Token {
        let current = self.pos.min(self.tokens.len() - 1);
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        &self.tokens[current]
    }

    /// Consume if current token matches, return true if consumed
    pub fn consume(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Expect a specific token kind, error if not found
    pub fn expect(&mut self, kind: &TokenKind) -> Result<&Token, String> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(format!(
                "Expected {:?}, found {:?} at line {}",
                kind,
                self.peek().kind,
                self.peek().span.line
            ))
        }
    }

    /// Get current line number
    #[inline]
    pub fn line(&self) -> usize {
        self.peek().span.line
    }

    /// Check if at end of input
    #[inline]
    pub fn is_eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    /// Skip newlines, return count skipped
    pub fn skip_newlines(&mut self) -> usize {
        let mut count = 0;
        while matches!(self.peek().kind, TokenKind::Newline) {
            self.advance();
            count += 1;
        }
        count
    }

    /// Skip to end of line (for error recovery)
    pub fn skip_to_eol(&mut self) {
        while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            self.advance();
        }
    }

    /// Collect tokens until end of line as a vec
    pub fn collect_line(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            tokens.push(self.peek().clone());
            self.advance();
        }
        tokens
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn expression_tokens(input: &str) -> Vec<String> {
        tokenize(input)
            .expect("tokenize failed")
            .into_iter()
            .filter_map(|t| match t.kind {
                TokenKind::Expression(e) => Some(e),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn quoted_expressions_keep_multibyte_content() {
        // The byte/char index mix-up used to reject any quoted expression
        // containing a multibyte character as unterminated.
        assert_eq!(expression_tokens("R1 a b 'µ0*2'"), ["µ0*2"]);
        assert_eq!(expression_tokens("R1 a b '2*3'"), ["2*3"]);
    }

    #[test]
    fn braced_expressions_keep_multibyte_content() {
        assert_eq!(expression_tokens("R1 a b {µ0*2}"), ["µ0*2"]);
    }

    #[test]
    fn digit_leading_node_names_remain_identifiers() {
        let tokens = tokenize("VMON 1 1a 0v\n").expect("tokenize");

        assert_eq!(tokens[1].kind, TokenKind::Number(1.0));
        assert_eq!(tokens[2].kind, TokenKind::Ident("1A".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Ident("0V".to_string()));
        assert_eq!(parse_spice_value("0V").expect("0V parses"), 0.0);
    }

    #[test]
    fn bare_seconds_suffix_is_consumed_after_decimal_values() {
        let tokens = tokenize(".tran .1s 10s\n").expect("tokenize");

        assert!(
            tokens
                .iter()
                .any(|token| token.kind == TokenKind::Number(0.1) && token.lexeme == ".1s")
        );
        assert!(
            !tokens
                .iter()
                .any(|token| token.kind == TokenKind::Ident("S".to_string())),
            "bare seconds unit must not remain as a stray identifier: {tokens:?}"
        );
        assert_eq!(parse_spice_value(".1s").expect(".1s parses"), 0.1);
        assert_eq!(parse_spice_value("10s").expect("10s parses"), 10.0);
    }

    #[test]
    fn prefixed_meter_suffixes_are_consumed_after_decimal_and_exponent_values() {
        let tokens = tokenize("M1 d g s b nmos L=0.35um W=.6uM TOX=50.0nm\n").expect("tokenize");
        let has_number = |lexeme: &str, expected: Value| {
            tokens.iter().any(|token| match token.kind {
                TokenKind::Number(value) if token.lexeme == lexeme => {
                    (value - expected).abs() <= expected.abs().max(1.0) * 1.0e-15
                }
                _ => false,
            })
        };

        assert!(has_number("0.35um", 0.35e-6));
        assert!(has_number(".6uM", 0.6e-6));
        assert!(has_number("50.0nm", 50.0e-9));
        assert!(
            !tokens
                .iter()
                .any(|token| token.kind == TokenKind::Ident("M".to_string())),
            "prefixed meter unit must not leave a stray M token: {tokens:?}"
        );
        assert!((parse_spice_value("1.0e6um").expect("1e6um parses") - 1.0).abs() <= 1.0e-15);
        assert!((parse_spice_value("1.0e3mm").expect("1e3mm parses") - 1.0).abs() <= 1.0e-15);
        assert!((parse_spice_value("50nm").expect("50nm parses") - 50.0e-9).abs() <= 1.0e-21);
        assert!((parse_spice_value("1.0e3m").expect("bare m is milli") - 1.0).abs() <= 1.0e-15);
        assert!((parse_spice_value("1.0e3M").expect("bare M is milli") - 1.0).abs() <= 1.0e-15);
    }

    #[test]
    fn xyce_x_scale_suffix_matches_meg() {
        assert_eq!(parse_spice_value("1X").expect("1X parses"), 1.0e6);
        assert_eq!(parse_spice_value("1x").expect("1x parses"), 1.0e6);
        assert_eq!(parse_spice_value("2.5X").expect("2.5X parses"), 2.5e6);
    }

    #[test]
    fn tokens_keep_original_lexeme_text() {
        let tokens = tokenize(".model co d_cosim (sim_args=[1e3, deck])\n").expect("tokenize");

        assert!(
            tokens
                .iter()
                .any(|token| token.kind == TokenKind::Number(1.0e3) && token.lexeme == "1e3")
        );
        assert!(
            tokens
                .iter()
                .any(|token| token.kind == TokenKind::Ident("DECK".to_string())
                    && token.lexeme == "deck")
        );
    }

    #[test]
    fn unterminated_quoted_expression_is_rejected() {
        assert!(tokenize("R1 a b '2*3").is_err());
        assert!(tokenize("R1 a b 'µ0*2").is_err());
    }
    #[test]
    fn non_delimiter_punctuation_tokens_keep_original_lexeme_text() {
        let tokens = tokenize("A1 !bias^1 bus|2 ctrl?0 model\n").expect("tokenize");

        for ch in ['!', '^', '|', '?'] {
            assert!(
                tokens
                    .iter()
                    .any(|token| token.kind == TokenKind::Other(ch)
                        && token.lexeme == ch.to_string()),
                "missing Other({ch:?}) token in {tokens:?}"
            );
        }
    }

    #[test]
    fn token_stream_empty_input_behaves_as_eof() {
        let mut stream = TokenStream::new(Vec::new());

        assert!(stream.is_eof());
        assert_eq!(stream.line(), 1);
        assert!(matches!(stream.peek().kind, TokenKind::Eof));
        assert!(matches!(stream.peek_n(8).kind, TokenKind::Eof));
        assert!(matches!(stream.advance().kind, TokenKind::Eof));
        assert!(matches!(stream.advance().kind, TokenKind::Eof));
    }
}
