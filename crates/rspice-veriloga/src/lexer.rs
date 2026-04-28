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
        if let Some((i, _)) = result {
            self.pos = i + 1;
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

    fn scan_number(&mut self, start: usize, _first: char) -> Result<Token, LexerError> {
        let mut has_dot = false;
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
            // Handle 'meg' specifically
            if (ch == 'm' || ch == 'M') && self.peek_char() == Some('e') {
                self.advance();
                if self.peek_char() == Some('g') {
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
                        Some((_, '0')) => value.push('\0'),
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
                            return Err(LexerError::new(
                                LexerErrorKind::InvalidEscape(ch),
                                Span::new(self.source_id, start as u32, self.pos as u32),
                            ));
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

