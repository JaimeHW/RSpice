// =============================================================================
// Token Types
// =============================================================================

/// Token type for lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Keyword (section, endsection, subckt, ends, model, etc.)
    Keyword(String),
    /// Identifier (model name, node name, parameter name)
    Identifier(String),
    /// Numeric literal
    Number(f64),
    /// String literal (quoted)
    String(String),
    /// Operator (=, +, -, *, /, etc.)
    Operator(char),
    /// Opening paren
    LParen,
    /// Closing paren
    RParen,
    /// Opening bracket
    LBracket,
    /// Closing bracket
    RBracket,
    /// Newline
    Newline,
    /// End of file
    Eof,
}

// =============================================================================
// Lexer
// =============================================================================

/// Lexer for SPICE library files
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// Current position info
    pub fn position(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    /// Peek at current character
    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Advance to next character
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    /// Skip whitespace (except newlines)
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip line comment
    fn skip_comment(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    /// Read identifier
    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    /// Read number (including engineering notation)
    fn read_number(&mut self) -> Result<f64, String> {
        let mut s = String::new();

        // Sign
        if let Some(ch) = self.peek()
            && (ch == '-' || ch == '+')
        {
            s.push(ch);
            self.advance();
        }

        // Integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Exponent or engineering suffix
        if let Some(ch) = self.peek() {
            if ch == 'e' || ch == 'E' {
                s.push(ch);
                self.advance();
                if let Some(sign) = self.peek()
                    && (sign == '-' || sign == '+')
                {
                    s.push(sign);
                    self.advance();
                }
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_digit() {
                        s.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
            } else {
                // Engineering suffix (T, G, M, k, m, u, n, p, f, a)
                let multiplier = match ch {
                    'T' => Some(1e12),
                    'G' => Some(1e9),
                    'M' => Some(1e6),
                    'k' | 'K' => Some(1e3),
                    'm' => Some(1e-3),
                    'u' | 'U' => Some(1e-6),
                    'n' => Some(1e-9),
                    'p' => Some(1e-12),
                    'f' => Some(1e-15),
                    'a' => Some(1e-18),
                    _ => None,
                };

                if let Some(mult) = multiplier {
                    self.advance();
                    // Skip optional unit suffix (e.g., "k" in "1kOhm")
                    while let Some(ch) = self.peek() {
                        if ch.is_alphabetic() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let base: f64 = s.parse().map_err(|e| format!("Invalid number: {}", e))?;
                    return Ok(base * mult);
                }
            }
        }

        s.parse()
            .map_err(|e| format!("Invalid number '{}': {}", s, e))
    }

    /// Read quoted string
    fn read_string(&mut self) -> Result<String, String> {
        let quote = self
            .advance()
            .ok_or_else(|| "Expected opening quote".to_string())?; // ' or "
        let mut result = String::new();

        while let Some(ch) = self.peek() {
            if ch == quote {
                self.advance();
                return Ok(result);
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        '\'' => result.push('\''),
                        '"' => result.push('"'),
                        _ => result.push(escaped),
                    }
                }
            } else if ch == '\n' {
                return Err("Unterminated string".to_string());
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Err("Unterminated string at end of file".to_string())
    }

    /// Get next token
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        // Handle continuation lines
        if self.peek() == Some('+') {
            let next_pos = self.pos + 1;
            if next_pos < self.input.len() {
                let next_ch = self.input[next_pos..].chars().next();
                if next_ch == Some(' ') || next_ch == Some('\t') {
                    self.advance(); // Skip +
                    self.skip_whitespace();
                    return self.next_token();
                }
            }
        }

        match self.peek() {
            None => Ok(Token::Eof),
            Some('\n') => {
                self.advance();
                Ok(Token::Newline)
            }
            Some('*') | Some('/') if self.peek() == Some('/') => {
                self.skip_comment();
                self.next_token()
            }
            Some('*') => {
                // Could be comment or multiplication
                if self.col == 1 {
                    self.skip_comment();
                    self.next_token()
                } else {
                    self.advance();
                    Ok(Token::Operator('*'))
                }
            }
            Some(';') => {
                self.skip_comment();
                self.next_token()
            }
            Some('\'') | Some('"') => {
                let s = self.read_string()?;
                Ok(Token::String(s))
            }
            Some('(') => {
                self.advance();
                Ok(Token::LParen)
            }
            Some(')') => {
                self.advance();
                Ok(Token::RParen)
            }
            Some('[') => {
                self.advance();
                Ok(Token::LBracket)
            }
            Some(']') => {
                self.advance();
                Ok(Token::RBracket)
            }
            Some(ch) if ch == '=' || ch == '+' || ch == '-' || ch == '/' => {
                self.advance();
                Ok(Token::Operator(ch))
            }
            Some(ch) if ch.is_ascii_digit() || (ch == '-' && self.is_number_start()) => {
                let num = self.read_number()?;
                Ok(Token::Number(num))
            }
            Some(ch) if ch.is_alphabetic() || ch == '_' || ch == '.' => {
                let ident = self.read_identifier();
                let lower = ident.to_lowercase();

                // Check for keywords
                match lower.as_str() {
                    "section" | "endsection" | "subckt" | "ends" | "model" | "include" | "lib"
                    | "library" | "endlibrary" | "parameters" | "param" | "simulator"
                    | "inline" | "ahdl_include" | "if" | "else" | "endif" => {
                        Ok(Token::Keyword(lower))
                    }
                    _ => Ok(Token::Identifier(ident)),
                }
            }
            Some(ch) => {
                self.advance();
                Ok(Token::Operator(ch))
            }
        }
    }

    /// Check if next char starts a number
    fn is_number_start(&self) -> bool {
        let next = &self.input[self.pos + 1..];
        next.chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
    }
}
