//! Verilog-A/AMS Lexer
//!
//! Tokenizes Verilog-A source code according to the LRM 2.4 specification.
//! Handles all lexical elements including:
//!
//! - Keywords (module, endmodule, analog, parameter, etc.)
//! - Operators (arithmetic, comparison, contribution `<+`)
//! - Identifiers and system identifiers ($temperature, $vt)
//! - Numeric literals (integer, real, with scale factors)
//! - String literals
//! - Comments (line and block)
//! - Compiler directives (`include, `define, etc.)

use crate::error::{LexerError, LexerErrorKind};
use crate::source::{SourceId, Span};

/// Token produced by the lexer
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    /// The raw text of the token (for identifiers, numbers, strings)
    pub text: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self {
            kind,
            span,
            text: None,
        }
    }

    pub fn with_text(kind: TokenKind, span: Span, text: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            text: Some(text.into()),
        }
    }
}

/// Token kinds for Verilog-A/AMS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // === Literals ===
    /// Integer literal: 123, 0xFF, 0b101
    IntegerLiteral,
    /// Real literal: 1.5, 1e-3, 2.5M
    RealLiteral,
    /// String literal: "hello"
    StringLiteral,

    // === Identifiers ===
    /// Regular identifier: foo, bar123
    Identifier,
    /// System identifier: $temperature, $vt
    SystemIdentifier,
    /// Escaped identifier: \foo+bar
    EscapedIdentifier,

    // === Keywords - Module Structure ===
    Module,
    Macromodule,
    Endmodule,
    Connectmodule,

    // === Keywords - Ports ===
    Input,
    Output,
    Inout,

    // === Keywords - Types ===
    Real,
    Integer,
    String,
    Genvar,

    // === Keywords - Parameter ===
    Parameter,
    Localparam,
    Aliasparam,

    // === Keywords - Disciplines (Verilog-AMS) ===
    Discipline,
    Enddiscipline,
    Nature,
    Endnature,
    Potential,
    Flow,
    Domain,
    Discrete,
    Continuous,

    // === Keywords - Nets ===
    Wire,
    Ground,
    Electrical, // Common discipline
    Voltage,    // Common discipline
    Current,    // Common discipline

    // === Keywords - Analog Block ===
    Analog,
    Initial,
    Begin,
    End,
    Fork,
    Join,

    // === Keywords - Control Flow ===
    If,
    Else,
    Case,
    Casex,
    Casez,
    Endcase,
    Default,
    For,
    While,
    Repeat,
    Forever,
    Disable,

    // === Keywords - Analog Operators ===
    Ddt,        // ddt(x)
    Idt,        // idt(x, ic)
    Idtmod,     // idtmod(x, ic, modulus)
    Ddx,        // ddx(f, x) partial derivative
    Limexp,     // limexp(x)
    Absdelay,   // absdelay(x, delay)
    Transition, // transition(x, td, rise, fall)
    Slew,       // slew(x, max_rise, max_fall)
    Laplace,    // Laplace transform operators
    Zi,         // z-transform operators

    // === Keywords - Noise Functions ===
    WhiteNoise,   // white_noise(pwr)
    FlickerNoise, // flicker_noise(pwr, exp)
    NoiseTable,   // noise_table(table)

    // === Keywords - Events ===
    Posedge,
    Negedge,
    Cross,
    Above,
    Timer,
    Final,

    // === Keywords - Other ===
    Function,
    Endfunction,
    Task,
    Endtask,
    Specify,
    Endspecify,
    Assign,
    Deassign,
    Force,
    Release,
    Generate,
    Endgenerate,
    Exclude,
    From,
    Inf,
    Abstol,
    Access,
    Units,
    #[allow(non_camel_case_types)]
    Idt_Nature,
    #[allow(non_camel_case_types)]
    Ddt_Nature,

    // === Operators - Arithmetic ===
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    Percent,  // %
    StarStar, // ** (power)

    // === Operators - Comparison ===
    Eq, // ==
    Ne, // !=
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=

    // === Operators - Logical ===
    And, // &&
    Or,  // ||
    Not, // !

    // === Operators - Bitwise ===
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    BitNot, // ~
    Shl,    // <<
    Shr,    // >>

    // === Operators - Assignment ===
    Assign_, // =

    // === Operators - Contribution (Verilog-A specific) ===
    Contribute, // <+

    // === Operators - Conditional ===
    Question, // ?
    Colon,    // :
    At,       // @

    // === Delimiters ===
    LParen,     // (
    RParen,     // )
    LBracket,   // [
    RBracket,   // ]
    LBrace,     // {
    RBrace,     // }
    Comma,      // ,
    Semicolon,  // ;
    Dot,        // .
    Hash,       // #
    DoubleHash, // ##

    // === Compiler Directives ===
    Directive, // `include, `define, etc.

    // === Special ===
    Eof,
    Error,
}

impl TokenKind {
    /// Get the keyword for a string, if it matches
    pub fn keyword(s: &str) -> Option<TokenKind> {
        Some(match s {
            // Module structure
            "module" => TokenKind::Module,
            "macromodule" => TokenKind::Macromodule,
            "endmodule" => TokenKind::Endmodule,
            "connectmodule" => TokenKind::Connectmodule,

            // Ports
            "input" => TokenKind::Input,
            "output" => TokenKind::Output,
            "inout" => TokenKind::Inout,

            // Types
            "real" => TokenKind::Real,
            "integer" => TokenKind::Integer,
            "string" => TokenKind::String,
            "genvar" => TokenKind::Genvar,

            // Parameters
            "parameter" => TokenKind::Parameter,
            "localparam" => TokenKind::Localparam,
            "aliasparam" => TokenKind::Aliasparam,

            // Disciplines
            "discipline" => TokenKind::Discipline,
            "enddiscipline" => TokenKind::Enddiscipline,
            "nature" => TokenKind::Nature,
            "endnature" => TokenKind::Endnature,
            "potential" => TokenKind::Potential,
            "flow" => TokenKind::Flow,
            "domain" => TokenKind::Domain,
            "discrete" => TokenKind::Discrete,
            "continuous" => TokenKind::Continuous,

            // Nets
            "wire" => TokenKind::Wire,
            "ground" => TokenKind::Ground,
            "electrical" => TokenKind::Electrical,
            "voltage" => TokenKind::Voltage,
            "current" => TokenKind::Current,

            // Analog block
            "analog" => TokenKind::Analog,
            "initial" => TokenKind::Initial,
            "begin" => TokenKind::Begin,
            "end" => TokenKind::End,
            "fork" => TokenKind::Fork,
            "join" => TokenKind::Join,

            // Control flow
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "case" => TokenKind::Case,
            "casex" => TokenKind::Casex,
            "casez" => TokenKind::Casez,
            "endcase" => TokenKind::Endcase,
            "default" => TokenKind::Default,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "repeat" => TokenKind::Repeat,
            "forever" => TokenKind::Forever,
            "disable" => TokenKind::Disable,

            // Events
            "posedge" => TokenKind::Posedge,
            "negedge" => TokenKind::Negedge,
            "cross" => TokenKind::Cross,
            "above" => TokenKind::Above,
            "timer" => TokenKind::Timer,
            "final" => TokenKind::Final,

            // Other
            "function" => TokenKind::Function,
            "endfunction" => TokenKind::Endfunction,
            "task" => TokenKind::Task,
            "endtask" => TokenKind::Endtask,
            "specify" => TokenKind::Specify,
            "endspecify" => TokenKind::Endspecify,
            "assign" => TokenKind::Assign,
            "deassign" => TokenKind::Deassign,
            "force" => TokenKind::Force,
            "release" => TokenKind::Release,
            "generate" => TokenKind::Generate,
            "endgenerate" => TokenKind::Endgenerate,
            "exclude" => TokenKind::Exclude,
            "from" => TokenKind::From,
            "inf" => TokenKind::Inf,
            "abstol" => TokenKind::Abstol,
            "access" => TokenKind::Access,
            "units" => TokenKind::Units,
            "idt_nature" => TokenKind::Idt_Nature,
            "ddt_nature" => TokenKind::Ddt_Nature,

            _ => return None,
        })
    }

    /// Check if this token kind is a keyword
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            TokenKind::Module
                | TokenKind::Macromodule
                | TokenKind::Endmodule
                | TokenKind::Connectmodule
                | TokenKind::Input
                | TokenKind::Output
                | TokenKind::Inout
                | TokenKind::Real
                | TokenKind::Integer
                | TokenKind::String
                | TokenKind::Genvar
                | TokenKind::Parameter
                | TokenKind::Localparam
                | TokenKind::Aliasparam
                | TokenKind::Discipline
                | TokenKind::Enddiscipline
                | TokenKind::Nature
                | TokenKind::Endnature
                | TokenKind::Potential
                | TokenKind::Flow
                | TokenKind::Domain
                | TokenKind::Discrete
                | TokenKind::Continuous
                | TokenKind::Wire
                | TokenKind::Ground
                | TokenKind::Electrical
                | TokenKind::Voltage
                | TokenKind::Current
                | TokenKind::Analog
                | TokenKind::Initial
                | TokenKind::Begin
                | TokenKind::End
                | TokenKind::Fork
                | TokenKind::Join
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::Case
                | TokenKind::Casex
                | TokenKind::Casez
                | TokenKind::Endcase
                | TokenKind::Default
                | TokenKind::For
                | TokenKind::While
                | TokenKind::Repeat
                | TokenKind::Forever
                | TokenKind::Disable
                | TokenKind::Posedge
                | TokenKind::Negedge
                | TokenKind::Cross
                | TokenKind::Above
                | TokenKind::Timer
                | TokenKind::Final
                | TokenKind::Function
                | TokenKind::Endfunction
                | TokenKind::Task
                | TokenKind::Endtask
                | TokenKind::Specify
                | TokenKind::Endspecify
                | TokenKind::Assign
                | TokenKind::Deassign
                | TokenKind::Force
                | TokenKind::Release
                | TokenKind::Generate
                | TokenKind::Endgenerate
                | TokenKind::Exclude
                | TokenKind::From
                | TokenKind::Inf
                | TokenKind::Abstol
                | TokenKind::Access
                | TokenKind::Units
                | TokenKind::Idt_Nature
                | TokenKind::Ddt_Nature
        )
    }
}

/// Lexer for Verilog-A/AMS source code
pub struct Lexer<'a> {
    source: &'a str,
    source_id: SourceId,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source
    pub fn new(source: &'a str, source_id: SourceId) -> Self {
        Self {
            source,
            source_id,
            chars: source.char_indices().peekable(),
            pos: 0,
        }
    }

    /// Collect all tokens from the source
    pub fn collect_tokens(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace_and_comments()?;

        let start = self.pos;
        let Some((_, ch)) = self.advance() else {
            return Ok(Token::new(
                TokenKind::Eof,
                Span::new(self.source_id, start as u32, self.pos as u32),
            ));
        };

        let kind = match ch {
            // Single character tokens
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '.' if self.peek_char().is_some_and(|next| next.is_ascii_digit()) => {
                return self.scan_number(start, ch);
            }
            '.' => TokenKind::Dot,
            '@' => TokenKind::At,
            '?' => TokenKind::Question,
            ':' => TokenKind::Colon,
            '~' => TokenKind::BitNot,
            '%' => TokenKind::Percent,

            // Multi-character operators
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => {
                if self.peek_char() == Some('*') {
                    self.advance();
                    TokenKind::StarStar
                } else {
                    TokenKind::Star
                }
            }
            '/' => TokenKind::Slash,
            '=' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    TokenKind::Eq
                } else {
                    TokenKind::Assign_
                }
            }
            '!' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    TokenKind::Ne
                } else {
                    TokenKind::Not
                }
            }
            '<' => {
                if self.peek_char() == Some('+') {
                    self.advance();
                    TokenKind::Contribute
                } else if self.peek_char() == Some('=') {
                    self.advance();
                    TokenKind::Le
                } else if self.peek_char() == Some('<') {
                    self.advance();
                    TokenKind::Shl
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                if self.peek_char() == Some('=') {
                    self.advance();
                    TokenKind::Ge
                } else if self.peek_char() == Some('>') {
                    self.advance();
                    TokenKind::Shr
                } else {
                    TokenKind::Gt
                }
            }
            '&' => {
                if self.peek_char() == Some('&') {
                    self.advance();
                    TokenKind::And
                } else {
                    TokenKind::BitAnd
                }
            }
            '|' => {
                if self.peek_char() == Some('|') {
                    self.advance();
                    TokenKind::Or
                } else {
                    TokenKind::BitOr
                }
            }
            '^' => TokenKind::BitXor,
            '#' => {
                if self.peek_char() == Some('#') {
                    self.advance();
                    TokenKind::DoubleHash
                } else {
                    TokenKind::Hash
                }
            }
            // Assignment pattern `'{ ... }` (LRM 2.4 array initializers):
            // lexes as the same brace token as a concatenation literal
            '\'' if self.peek_char() == Some('{') => {
                self.advance();
                TokenKind::LBrace
            }

            // Compiler directive
            '`' => return self.scan_directive(start),

            // String literal
            '"' => return self.scan_string(start),

            // System identifier
            '$' => return self.scan_system_identifier(start),

            // Escaped identifier
            '\\' => return self.scan_escaped_identifier(start),

            // Number or identifier
            _ => {
                if ch.is_ascii_digit() {
                    return self.scan_number(start, ch);
                } else if ch.is_ascii_alphabetic() || ch == '_' {
                    return self.scan_identifier(start);
                } else {
                    return Err(LexerError::new(
                        LexerErrorKind::UnexpectedChar(ch),
                        Span::new(self.source_id, start as u32, self.pos as u32),
                    ));
                }
            }
        };

        Ok(Token::new(
            kind,
            Span::new(self.source_id, start as u32, self.pos as u32),
        ))
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        let result = self.chars.next();
        if let Some((i, ch)) = result {
            self.pos = i + ch.len_utf8();
        }
        result
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, ch)| *ch)
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), LexerError> {
        loop {
            // Skip whitespace
            while let Some(ch) = self.peek_char() {
                if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }

            // Check for comments - peek first to see if it's // or /*
            if self.peek_char() == Some('/') {
                // We need to look ahead another character WITHOUT consuming
                // Clone the iterator to peek ahead
                let mut lookahead = self.chars.clone();
                lookahead.next(); // skip the '/'
                let next_char = lookahead.next().map(|(_, c)| c);

                if next_char == Some('/') {
                    // Line comment
                    let _start = self.pos;
                    self.advance(); // consume '/'
                    self.advance(); // consume second '/'
                    while let Some(ch) = self.peek_char() {
                        if ch == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else if next_char == Some('*') {
                    // Block comment
                    let start = self.pos;
                    self.advance(); // consume '/'
                    self.advance(); // consume '*'
                    loop {
                        match self.advance() {
                            Some((_, '*')) if self.peek_char() == Some('/') => {
                                self.advance();
                                break;
                            }
                            Some(_) => {}
                            None => {
                                return Err(LexerError::new(
                                    LexerErrorKind::UnterminatedComment,
                                    Span::new(self.source_id, start as u32, self.pos as u32),
                                ));
                            }
                        }
                    }
                } else {
                    // Not a comment, just a slash operator - don't consume it
                    break;
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn scan_identifier(&mut self, start: usize) -> Result<Token, LexerError> {
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[start..self.pos];
        let span = Span::new(self.source_id, start as u32, self.pos as u32);

        // Check for keywords - preserve text for parser to use as identifier if needed
        if let Some(keyword) = TokenKind::keyword(text) {
            Ok(Token::with_text(keyword, span, text))
        } else {
            Ok(Token::with_text(TokenKind::Identifier, span, text))
        }
    }

    fn scan_system_identifier(&mut self, start: usize) -> Result<Token, LexerError> {
        // Already consumed '$'
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[start..self.pos];
        let span = Span::new(self.source_id, start as u32, self.pos as u32);
        Ok(Token::with_text(TokenKind::SystemIdentifier, span, text))
    }

    fn scan_escaped_identifier(&mut self, start: usize) -> Result<Token, LexerError> {
        // Already consumed '\'
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                break;
            }
            self.advance();
        }

        let text = &self.source[start..self.pos];
        let span = Span::new(self.source_id, start as u32, self.pos as u32);
        Ok(Token::with_text(TokenKind::EscapedIdentifier, span, text))
    }

    fn scan_number(&mut self, start: usize, first: char) -> Result<Token, LexerError> {
        let mut has_dot = first == '.';
        let mut has_exp = false;

        // Continue scanning digits
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !has_dot && !has_exp {
                // Check if next char is digit (to avoid parsing "1.2.3")
                self.advance();
                if let Some(next) = self.peek_char()
                    && !next.is_ascii_digit()
                {
                    // Backtrack - this is integer followed by '.'
                    // We can't backtrack easily, so mark as error or handle differently
                    // For now, require digit after dot
                }
                has_dot = true;
            } else if (ch == 'e' || ch == 'E') && !has_exp {
                has_exp = true;
                self.advance();
                // Optional sign after exponent
                if let Some(sign) = self.peek_char()
                    && (sign == '+' || sign == '-')
                {
                    self.advance();
                }
                if !self.peek_char().is_some_and(|next| next.is_ascii_digit()) {
                    let text = self.source[start..self.pos].to_string();
                    return Err(LexerError::new(
                        LexerErrorKind::InvalidNumber(text),
                        Span::new(self.source_id, start as u32, self.pos as u32),
                    ));
                }
            } else {
                break;
            }
        }

        // Check for scale factors (T, G, M, k, m, u, n, p, f, a)
        let mut has_scale = false;
        if let Some(ch) = self.peek_char()
            && matches!(
                ch,
                'T' | 'G' | 'M' | 'k' | 'K' | 'm' | 'u' | 'n' | 'p' | 'f' | 'a'
            )
        {
            has_scale = true;
            self.advance();
            // SPICE-compatible 'meg' suffix: only consume "eg" as a unit so
            // that "1me" is never mangled into a phantom scale factor.
            if ch == 'm' || ch == 'M' {
                let mut lookahead = self.chars.clone();
                let next1 = lookahead.next().map(|(_, c)| c);
                let next2 = lookahead.next().map(|(_, c)| c);
                if matches!(next1, Some('e' | 'E')) && matches!(next2, Some('g' | 'G')) {
                    self.advance();
                    self.advance();
                }
            }
        }

        let text = &self.source[start..self.pos];
        let span = Span::new(self.source_id, start as u32, self.pos as u32);
        // Numbers with scale factors, dot, or exponent are real literals
        let kind = if has_dot || has_exp || has_scale {
            TokenKind::RealLiteral
        } else {
            TokenKind::IntegerLiteral
        };

        Ok(Token::with_text(kind, span, text))
    }

    fn scan_string(&mut self, start: usize) -> Result<Token, LexerError> {
        // Already consumed opening '"'
        let mut value = String::new();

        loop {
            match self.advance() {
                Some((_, '"')) => break,
                Some((_, '\\')) => {
                    // Escape sequence or line continuation
                    match self.advance() {
                        Some((_, 'n')) => value.push('\n'),
                        Some((_, 't')) => value.push('\t'),
                        Some((_, 'r')) => value.push('\r'),
                        Some((_, '\\')) => value.push('\\'),
                        Some((_, '"')) => value.push('"'),
                        // Octal escape: \d, \dd, or \ddd (LRM 2.6.3)
                        Some((_, ch @ '0'..='7')) => {
                            let mut code = ch as u32 - '0' as u32;
                            for _ in 0..2 {
                                match self.peek_char() {
                                    Some(next @ '0'..='7') => {
                                        code = code * 8 + (next as u32 - '0' as u32);
                                        self.advance();
                                    }
                                    _ => break,
                                }
                            }
                            value.push(char::from_u32(code).unwrap_or('\0'));
                        }
                        // Line continuation: backslash followed by newline
                        Some((_, '\n')) => {
                            // Skip the newline and continue on next line
                            // This is Verilog-A line continuation syntax
                        }
                        Some((_, '\r')) => {
                            // Handle Windows-style line endings: \r\n
                            if let Some((_, '\n')) = self.chars.clone().next() {
                                self.advance(); // consume the \n
                            }
                            // Skip both and continue on next line
                        }
                        Some((_, ch)) => {
                            value.push(ch);
                        }
                        None => {
                            return Err(LexerError::new(
                                LexerErrorKind::UnterminatedString,
                                Span::new(self.source_id, start as u32, self.pos as u32),
                            ));
                        }
                    }
                }
                Some((_, ch)) => value.push(ch),
                None => {
                    return Err(LexerError::new(
                        LexerErrorKind::UnterminatedString,
                        Span::new(self.source_id, start as u32, self.pos as u32),
                    ));
                }
            }
        }

        let span = Span::new(self.source_id, start as u32, self.pos as u32);
        Ok(Token::with_text(TokenKind::StringLiteral, span, value))
    }

    fn scan_directive(&mut self, start: usize) -> Result<Token, LexerError> {
        // Already consumed '`'
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[start..self.pos];
        let span = Span::new(self.source_id, start as u32, self.pos as u32);
        Ok(Token::with_text(TokenKind::Directive, span, text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> Vec<Token> {
        Lexer::new(source, SourceId::new(0))
            .collect_tokens()
            .expect("lexing failed")
    }

    fn kinds(source: &str) -> Vec<TokenKind> {
        lex(source).iter().map(|t| t.kind).collect()
    }

    #[test]
    fn tokenizes_module_skeleton() {
        let toks = kinds("module r(p, n); endmodule");
        assert_eq!(
            toks,
            vec![
                TokenKind::Module,
                TokenKind::Identifier,
                TokenKind::LParen,
                TokenKind::Identifier,
                TokenKind::Comma,
                TokenKind::Identifier,
                TokenKind::RParen,
                TokenKind::Semicolon,
                TokenKind::Endmodule,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn contribution_operator_lexes_before_le() {
        assert_eq!(
            kinds("<+ <= < <<"),
            vec![
                TokenKind::Contribute,
                TokenKind::Le,
                TokenKind::Lt,
                TokenKind::Shl,
                TokenKind::Eof
            ]
        );
    }

    #[test]
    fn multibyte_comment_does_not_break_positions() {
        // Non-ASCII text in comments (common in vendor model headers) must not
        // desynchronize byte positions for following tokens.
        let toks = lex("/* Modèle © Fabrikant µA */ module");
        assert_eq!(toks[0].kind, TokenKind::Module);
        assert_eq!(toks[0].text.as_deref(), Some("module"));
    }

    #[test]
    fn multibyte_string_contents_round_trip() {
        let toks = lex("\"héllo µ\" x");
        assert_eq!(toks[0].kind, TokenKind::StringLiteral);
        assert_eq!(toks[0].text.as_deref(), Some("héllo µ"));
        assert_eq!(toks[1].text.as_deref(), Some("x"));
    }

    #[test]
    fn scale_factors_lex_as_real_literals() {
        for src in ["1.5k", "2u", "3n", "4p", "5f", "6a", "7T", "8G", "9M"] {
            let toks = lex(src);
            assert_eq!(toks[0].kind, TokenKind::RealLiteral, "for {src}");
            assert_eq!(toks[0].text.as_deref(), Some(src), "for {src}");
        }
    }

    #[test]
    fn meg_suffix_consumed_only_as_unit() {
        let toks = lex("1meg");
        assert_eq!(toks[0].kind, TokenKind::RealLiteral);
        assert_eq!(toks[0].text.as_deref(), Some("1meg"));

        // "1m" followed by identifier must leave the identifier intact.
        let toks = lex("1m eg");
        assert_eq!(toks[0].text.as_deref(), Some("1m"));
        assert_eq!(toks[1].kind, TokenKind::Identifier);
        assert_eq!(toks[1].text.as_deref(), Some("eg"));

        // "2me" is the real "2m" then the identifier "e".
        let toks = lex("2me");
        assert_eq!(toks[0].text.as_deref(), Some("2m"));
        assert_eq!(toks[1].kind, TokenKind::Identifier);
        assert_eq!(toks[1].text.as_deref(), Some("e"));
    }

    #[test]
    fn exponent_notation_is_real() {
        for src in ["1e3", "1.5e-12", "2E+6"] {
            let toks = lex(src);
            assert_eq!(toks[0].kind, TokenKind::RealLiteral, "for {src}");
        }
        assert_eq!(lex("42")[0].kind, TokenKind::IntegerLiteral);
    }

    #[test]
    fn leading_dot_numbers_are_real_literals() {
        for src in [".5", ".5e-3", ".5k"] {
            let toks = lex(src);
            assert_eq!(toks[0].kind, TokenKind::RealLiteral, "for {src}");
            assert_eq!(toks[0].text.as_deref(), Some(src), "for {src}");
        }
    }

    #[test]
    fn exponent_requires_digits() {
        for src in ["1e", "1e+", "1E-"] {
            let err = Lexer::new(src, SourceId::new(0))
                .collect_tokens()
                .expect_err("malformed exponent must be rejected");
            assert!(matches!(err.kind, LexerErrorKind::InvalidNumber(_)));
        }
    }

    #[test]
    fn octal_escapes_in_strings() {
        let toks = lex(r#""a\101b\0c""#);
        assert_eq!(toks[0].text.as_deref(), Some("aAb\0c"));
    }

    #[test]
    fn non_special_string_escapes_preserve_character() {
        let toks = lex(r#""hisimsoi\_fb""#);
        assert_eq!(toks[0].kind, TokenKind::StringLiteral);
        assert_eq!(toks[0].text.as_deref(), Some("hisimsoi_fb"));
    }

    #[test]
    fn line_continuation_in_string() {
        let toks = lex("\"ab\\\ncd\"");
        assert_eq!(toks[0].text.as_deref(), Some("abcd"));
    }

    #[test]
    fn system_identifiers_and_directives() {
        let toks = lex("$temperature `include");
        assert_eq!(toks[0].kind, TokenKind::SystemIdentifier);
        assert_eq!(toks[0].text.as_deref(), Some("$temperature"));
        assert_eq!(toks[1].kind, TokenKind::Directive);
        assert_eq!(toks[1].text.as_deref(), Some("`include"));
    }

    #[test]
    fn unterminated_block_comment_errors() {
        let result = Lexer::new("/* never closed", SourceId::new(0)).collect_tokens();
        assert!(result.is_err());
    }

    #[test]
    fn block_comment_with_stars_terminates() {
        let toks = lex("/* ** stars ** */ x");
        assert_eq!(toks[0].kind, TokenKind::Identifier);
    }
}
