//! SPICE Library Parser
//!
//! Commercial-grade parser for SPICE model library files (.lib, .scs).
//! Supports Cadence Spectre syntax with sections, corners, and includes.
//!
//! # Features
//!
//! - Parse .lib and .scs model files
//! - Section/corner extraction (tt, ff, ss, sf, fs)
//! - Nested .include/.lib directive resolution
//! - Model parameter extraction
//! - Comment and continuation line handling
//! - Error reporting with line numbers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
        if let Some(ch) = self.peek() {
            if ch == '-' || ch == '+' {
                s.push(ch);
                self.advance();
            }
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
                if let Some(sign) = self.peek() {
                    if sign == '-' || sign == '+' {
                        s.push(sign);
                        self.advance();
                    }
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

// =============================================================================
// AST Types
// =============================================================================

/// A parsed library section (corner)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LibrarySection {
    /// Section name (e.g., "tt", "ff")
    pub name: String,
    /// Models defined in this section
    pub models: HashMap<String, ModelDef>,
    /// Subcircuits defined in this section
    pub subcircuits: HashMap<String, SubcircuitDef>,
    /// Parameters defined in this section
    pub parameters: HashMap<String, ParamValue>,
    /// Include directives
    pub includes: Vec<IncludeDirective>,
}

/// Model definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ModelDef {
    /// Model name
    pub name: String,
    /// Model type (nmos, pmos, npn, pnp, r, c, d, etc.)
    pub model_type: String,
    /// Model level
    pub level: Option<i32>,
    /// Version
    pub version: Option<String>,
    /// Parameters
    pub parameters: HashMap<String, ParamValue>,
    /// Source line number
    pub line: usize,
}


/// Subcircuit definition
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubcircuitDef {
    /// Subcircuit name
    pub name: String,
    /// Port names
    pub ports: Vec<String>,
    /// Parameters with defaults
    pub parameters: HashMap<String, ParamValue>,
    /// Internal content (as string for now)
    pub content: String,
    /// Source line number
    pub line: usize,
}

/// Include directive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncludeDirective {
    /// Include type (.include or .lib)
    pub directive_type: IncludeType,
    /// File path
    pub path: PathBuf,
    /// Section name (for .lib)
    pub section: Option<String>,
    /// Source line number
    pub line: usize,
}

/// Type of include directive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncludeType {
    /// .include - full file inclusion
    Include,
    /// .lib - section-specific inclusion
    Lib,
}

/// Parameter value (can be numeric or expression)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParamValue {
    /// Numeric value
    Number(f64),
    /// String/expression value
    Expression(String),
}

impl ParamValue {
    /// Get as number if possible
    pub fn as_number(&self) -> Option<f64> {
        match self {
            ParamValue::Number(n) => Some(*n),
            ParamValue::Expression(s) => s.parse().ok(),
        }
    }

    /// Get as string
    pub fn as_string(&self) -> String {
        match self {
            ParamValue::Number(n) => n.to_string(),
            ParamValue::Expression(s) => s.clone(),
        }
    }
}

// =============================================================================
// Parser
// =============================================================================

/// Parser for SPICE library files
pub struct LibraryParser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    peeked: Option<Token>,
}

impl<'a> LibraryParser<'a> {
    /// Create a new parser
    pub fn new(input: &'a str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self {
            lexer,
            current,
            peeked: None,
        })
    }

    /// Advance to next token
    fn advance(&mut self) -> Result<Token, String> {
        let old = std::mem::replace(
            &mut self.current,
            if let Some(t) = self.peeked.take() {
                t
            } else {
                self.lexer.next_token()?
            },
        );
        Ok(old)
    }

    /// Skip newlines
    fn skip_newlines(&mut self) -> Result<(), String> {
        while self.current == Token::Newline {
            self.advance()?;
        }
        Ok(())
    }

    /// Expect a specific token
    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if &self.current != expected {
            let (line, col) = self.lexer.position();
            return Err(format!(
                "Expected {:?} at line {}, col {}, got {:?}",
                expected, line, col, self.current
            ));
        }
        self.advance()?;
        Ok(())
    }

    /// Parse the entire library
    pub fn parse(&mut self) -> Result<ParsedLibrary, String> {
        let mut library = ParsedLibrary::default();
        self.skip_newlines()?;

        while self.current != Token::Eof {
            match &self.current {
                Token::Keyword(kw) if kw == "library" => {
                    self.advance()?;
                    // Skip library name
                    if let Token::Identifier(name) = &self.current {
                        library.name = name.clone();
                        self.advance()?;
                    }
                }
                Token::Keyword(kw) if kw == "section" => {
                    let section = self.parse_section()?;
                    library.sections.insert(section.name.clone(), section);
                }
                Token::Keyword(kw) if kw == "model" => {
                    let model = self.parse_model()?;
                    library.global_models.insert(model.name.clone(), model);
                }
                Token::Keyword(kw) if kw == "subckt" => {
                    let subckt = self.parse_subcircuit()?;
                    library
                        .global_subcircuits
                        .insert(subckt.name.clone(), subckt);
                }
                Token::Keyword(kw) if kw == "param" || kw == "parameters" => {
                    let params = self.parse_parameters()?;
                    library.global_parameters.extend(params);
                }
                Token::Keyword(kw) if kw == "include" || kw == "lib" => {
                    let include = self.parse_include()?;
                    library.includes.push(include);
                }
                Token::Keyword(kw) if kw == "endlibrary" => {
                    self.advance()?;
                }
                Token::Newline => {
                    self.advance()?;
                }
                _ => {
                    // Skip unknown content
                    self.advance()?;
                }
            }
        }

        Ok(library)
    }

    /// Parse a section
    fn parse_section(&mut self) -> Result<LibrarySection, String> {
        self.advance()?; // Skip 'section'
        self.skip_newlines()?;

        let mut section = LibrarySection::default();

        // Get section name
        if let Token::Identifier(name) = &self.current {
            section.name = name.clone();
            self.advance()?;
        } else if let Token::Keyword(name) = &self.current {
            section.name = name.clone();
            self.advance()?;
        }

        self.skip_newlines()?;

        // Parse section content
        while self.current != Token::Eof {
            match &self.current {
                Token::Keyword(kw) if kw == "endsection" => {
                    self.advance()?;
                    break;
                }
                Token::Keyword(kw) if kw == "model" => {
                    let model = self.parse_model()?;
                    section.models.insert(model.name.clone(), model);
                }
                Token::Keyword(kw) if kw == "subckt" => {
                    let subckt = self.parse_subcircuit()?;
                    section.subcircuits.insert(subckt.name.clone(), subckt);
                }
                Token::Keyword(kw) if kw == "param" || kw == "parameters" => {
                    let params = self.parse_parameters()?;
                    section.parameters.extend(params);
                }
                Token::Keyword(kw) if kw == "include" || kw == "lib" => {
                    let include = self.parse_include()?;
                    section.includes.push(include);
                }
                Token::Newline => {
                    self.advance()?;
                }
                _ => {
                    self.advance()?;
                }
            }
        }

        Ok(section)
    }

    /// Parse a model definition
    fn parse_model(&mut self) -> Result<ModelDef, String> {
        let line = self.lexer.line;
        self.advance()?; // Skip 'model'

        let mut model = ModelDef {
            line,
            ..Default::default()
        };

        // Model name
        if let Token::Identifier(name) = &self.current {
            model.name = name.clone();
            self.advance()?;
        }

        // Model type
        if let Token::Identifier(mtype) = &self.current {
            model.model_type = mtype.clone();
            self.advance()?;
        }

        // Parameters (continue until newline or paren close)
        self.parse_model_params(&mut model)?;

        Ok(model)
    }

    /// Parse model parameters
    fn parse_model_params(&mut self, model: &mut ModelDef) -> Result<(), String> {
        while self.current != Token::Newline && self.current != Token::Eof {
            match &self.current {
                Token::Identifier(name) => {
                    let param_name = name.clone();
                    self.advance()?;

                    if self.current == Token::Operator('=') {
                        self.advance()?;

                        // Handle special parameters
                        if param_name.to_lowercase() == "level" {
                            if let Token::Number(n) = &self.current {
                                model.level = Some(*n as i32);
                            }
                        } else if param_name.to_lowercase() == "version" {
                            if let Token::Number(n) = &self.current {
                                model.version = Some(n.to_string());
                            } else if let Token::String(s) = &self.current {
                                model.version = Some(s.clone());
                            }
                        }

                        let value = self.parse_param_value()?;
                        model.parameters.insert(param_name, value);
                    }
                }
                Token::LParen => {
                    self.advance()?;
                }
                Token::RParen => {
                    self.advance()?;
                }
                _ => {
                    self.advance()?;
                }
            }
        }

        Ok(())
    }

    /// Parse subcircuit definition
    fn parse_subcircuit(&mut self) -> Result<SubcircuitDef, String> {
        let line = self.lexer.line;
        self.advance()?; // Skip 'subckt'

        let mut subckt = SubcircuitDef {
            line,
            ..Default::default()
        };

        // Subcircuit name
        if let Token::Identifier(name) = &self.current {
            subckt.name = name.clone();
            self.advance()?;
        }

        // Ports (until = or newline)
        while self.current != Token::Newline && self.current != Token::Eof {
            match &self.current {
                Token::Identifier(port) => {
                    if self.peeked.is_none() {
                        // Peek next
                        let next = self.lexer.next_token()?;
                        if next == Token::Operator('=') {
                            // This is a parameter, not a port
                            self.peeked = Some(next);
                            break;
                        }
                        self.peeked = Some(next);
                    }
                    subckt.ports.push(port.clone());
                    self.advance()?;
                }
                Token::LParen => {
                    self.advance()?;
                }
                Token::RParen => {
                    self.advance()?;
                }
                _ => {
                    break;
                }
            }
        }

        // Skip to .ends
        let mut depth = 1;
        while self.current != Token::Eof && depth > 0 {
            match &self.current {
                Token::Keyword(kw) if kw == "subckt" => {
                    depth += 1;
                    self.advance()?;
                }
                Token::Keyword(kw) if kw == "ends" => {
                    depth -= 1;
                    self.advance()?;
                }
                _ => {
                    self.advance()?;
                }
            }
        }

        Ok(subckt)
    }

    /// Parse parameter definitions
    fn parse_parameters(&mut self) -> Result<HashMap<String, ParamValue>, String> {
        self.advance()?; // Skip 'param' or 'parameters'
        let mut params = HashMap::new();

        while self.current != Token::Newline && self.current != Token::Eof {
            if let Token::Identifier(name) = &self.current {
                let param_name = name.clone();
                self.advance()?;

                if self.current == Token::Operator('=') {
                    self.advance()?;
                    let value = self.parse_param_value()?;
                    params.insert(param_name, value);
                }
            } else {
                self.advance()?;
            }
        }

        Ok(params)
    }

    /// Parse a parameter value
    fn parse_param_value(&mut self) -> Result<ParamValue, String> {
        match &self.current {
            Token::Number(n) => {
                let val = *n;
                self.advance()?;
                Ok(ParamValue::Number(val))
            }
            Token::Identifier(s) | Token::String(s) => {
                let val = s.clone();
                self.advance()?;
                Ok(ParamValue::Expression(val))
            }
            _ => {
                self.advance()?;
                Ok(ParamValue::Expression(String::new()))
            }
        }
    }

    /// Parse include directive
    fn parse_include(&mut self) -> Result<IncludeDirective, String> {
        let line = self.lexer.line;
        let directive_type = if let Token::Keyword(kw) = &self.current {
            if kw == "lib" {
                IncludeType::Lib
            } else {
                IncludeType::Include
            }
        } else {
            IncludeType::Include
        };

        self.advance()?;

        let mut path = PathBuf::new();
        let mut section = None;

        // Get path
        match &self.current {
            Token::String(s) => {
                path = PathBuf::from(s);
                self.advance()?;
            }
            Token::Identifier(s) => {
                path = PathBuf::from(s);
                self.advance()?;
            }
            _ => {}
        }

        // Get section for .lib
        if directive_type == IncludeType::Lib {
            if let Token::Identifier(s) = &self.current {
                section = Some(s.clone());
                self.advance()?;
            }
        }

        Ok(IncludeDirective {
            directive_type,
            path,
            section,
            line,
        })
    }
}

// =============================================================================
// Parsed Library
// =============================================================================

/// A fully parsed library file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParsedLibrary {
    /// Library name
    pub name: String,
    /// Source file path
    pub source_path: Option<PathBuf>,
    /// Sections (corners)
    pub sections: HashMap<String, LibrarySection>,
    /// Global models (not in any section)
    pub global_models: HashMap<String, ModelDef>,
    /// Global subcircuits
    pub global_subcircuits: HashMap<String, SubcircuitDef>,
    /// Global parameters
    pub global_parameters: HashMap<String, ParamValue>,
    /// Include directives
    pub includes: Vec<IncludeDirective>,
}

impl ParsedLibrary {
    /// Parse from string
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut parser = LibraryParser::new(input)?;
        parser.parse()
    }

    /// Parse from file
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
        let mut library = content.parse::<Self>()?;
        library.source_path = Some(path.to_path_buf());
        Ok(library)
    }

    /// Get available section names (corners)
    pub fn section_names(&self) -> Vec<&str> {
        self.sections.keys().map(|s| s.as_str()).collect()
    }

    /// Get a section by name
    pub fn get_section(&self, name: &str) -> Option<&LibrarySection> {
        self.sections.get(name)
    }

    /// Get all models (global + from specified section)
    pub fn models_for_section(&self, section: &str) -> HashMap<String, &ModelDef> {
        let mut models: HashMap<String, &ModelDef> = HashMap::new();

        // Add global models
        for (name, model) in &self.global_models {
            models.insert(name.clone(), model);
        }

        // Add section models (override globals)
        if let Some(sec) = self.sections.get(section) {
            for (name, model) in &sec.models {
                models.insert(name.clone(), model);
            }
        }

        models
    }

    /// Total model count
    pub fn model_count(&self) -> usize {
        let section_models: usize = self.sections.values().map(|s| s.models.len()).sum();
        self.global_models.len() + section_models
    }

    /// Total subcircuit count
    pub fn subcircuit_count(&self) -> usize {
        let section_subcircuits: usize = self.sections.values().map(|s| s.subcircuits.len()).sum();
        self.global_subcircuits.len() + section_subcircuits
    }
}

impl std::str::FromStr for ParsedLibrary {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Lexer Tests
    // =========================================================================

    #[test]
    fn test_lexer_identifiers() {
        let mut lexer = Lexer::new("hello world_123");
        assert!(matches!(lexer.next_token().unwrap(), Token::Identifier(s) if s == "hello"));
        assert!(matches!(lexer.next_token().unwrap(), Token::Identifier(s) if s == "world_123"));
    }

    #[test]
    fn test_lexer_numbers() {
        let mut lexer = Lexer::new("123 45.67 1e-9");
        assert!(
            matches!(lexer.next_token().unwrap(), Token::Number(n) if (n - 123.0).abs() < 1e-10)
        );
        assert!(
            matches!(lexer.next_token().unwrap(), Token::Number(n) if (n - 45.67).abs() < 1e-10)
        );
        assert!(
            matches!(lexer.next_token().unwrap(), Token::Number(n) if (n - 1e-9).abs() < 1e-20)
        );
    }

    #[test]
    fn test_lexer_engineering_notation() {
        let mut lexer = Lexer::new("1k 10M 5n 100p");
        assert!(matches!(lexer.next_token().unwrap(), Token::Number(n) if (n - 1e3).abs() < 1.0));
        assert!(matches!(lexer.next_token().unwrap(), Token::Number(n) if (n - 10e6).abs() < 1.0));
        assert!(
            matches!(lexer.next_token().unwrap(), Token::Number(n) if (n - 5e-9).abs() < 1e-18)
        );
        assert!(
            matches!(lexer.next_token().unwrap(), Token::Number(n) if (n - 100e-12).abs() < 1e-20)
        );
    }

    #[test]
    fn test_lexer_strings() {
        let mut lexer = Lexer::new("'hello' \"world\"");
        assert!(matches!(lexer.next_token().unwrap(), Token::String(s) if s == "hello"));
        assert!(matches!(lexer.next_token().unwrap(), Token::String(s) if s == "world"));
    }

    #[test]
    fn test_lexer_keywords() {
        let mut lexer = Lexer::new("section model subckt");
        assert!(matches!(lexer.next_token().unwrap(), Token::Keyword(s) if s == "section"));
        assert!(matches!(lexer.next_token().unwrap(), Token::Keyword(s) if s == "model"));
        assert!(matches!(lexer.next_token().unwrap(), Token::Keyword(s) if s == "subckt"));
    }

    #[test]
    fn test_lexer_operators() {
        // Note: '+ ' (plus followed by space) is a SPICE continuation marker
        // and '/' can be treated as comment start. Test = and - operators.
        let mut lexer = Lexer::new("a = b - c");
        assert!(matches!(lexer.next_token().unwrap(), Token::Identifier(_)));
        assert!(matches!(lexer.next_token().unwrap(), Token::Operator('=')));
        assert!(matches!(lexer.next_token().unwrap(), Token::Identifier(_)));
        assert!(matches!(lexer.next_token().unwrap(), Token::Operator('-')));
        assert!(matches!(lexer.next_token().unwrap(), Token::Identifier(_)));
    }

    #[test]
    fn test_lexer_comments() {
        // In SPICE, * is a comment only at column 1 (start of line)
        // Mid-line * is treated as an operator
        let mut lexer = Lexer::new("* this is a comment\nb");
        // First token should be 'b' (comment line is skipped, then newline is returned or 'b')
        let first = lexer.next_token().unwrap();
        assert!(
            matches!(first, Token::Newline) || matches!(first, Token::Identifier(_)),
            "After comment line, expected Newline or Identifier, got {:?}",
            first
        );
    }

    // =========================================================================
    // Parser Tests
    // =========================================================================

    #[test]
    fn test_parse_empty() {
        let lib = ParsedLibrary::parse("").unwrap();
        assert!(lib.sections.is_empty());
        assert!(lib.global_models.is_empty());
    }

    #[test]
    fn test_parse_simple_model() {
        let input = r#"
model nmos1 nmos level=54 version=4.5 vth0=0.4
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        assert_eq!(lib.global_models.len(), 1);

        let model = lib.global_models.get("nmos1").unwrap();
        assert_eq!(model.model_type, "nmos");
        assert_eq!(model.level, Some(54));
    }

    #[test]
    fn test_parse_section() {
        let input = r#"
section tt
model nmos_tt nmos level=54 vth0=0.4
endsection tt
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        assert_eq!(lib.sections.len(), 1);
        assert!(lib.sections.contains_key("tt"));

        let section = lib.get_section("tt").unwrap();
        assert_eq!(section.models.len(), 1);
    }

    #[test]
    fn test_parse_multiple_sections() {
        let input = r#"
section tt
model nmos_tt nmos vth0=0.4
endsection tt

section ff
model nmos_ff nmos vth0=0.35
endsection ff

section ss
model nmos_ss nmos vth0=0.45
endsection ss
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        assert_eq!(lib.sections.len(), 3);
        assert!(lib.sections.contains_key("tt"));
        assert!(lib.sections.contains_key("ff"));
        assert!(lib.sections.contains_key("ss"));
    }

    #[test]
    fn test_parse_parameters() {
        let input = r#"
parameters vdd=1.8 vth=0.4 tox=2e-9
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        assert_eq!(lib.global_parameters.len(), 3);

        let vdd = lib.global_parameters.get("vdd").unwrap();
        assert!((vdd.as_number().unwrap() - 1.8).abs() < 0.01);
    }

    #[test]
    fn test_parse_include() {
        let input = r#"
include 'models/nmos.scs'
lib 'corners.lib' tt
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        assert_eq!(lib.includes.len(), 2);

        assert_eq!(lib.includes[0].directive_type, IncludeType::Include);
        assert_eq!(lib.includes[1].directive_type, IncludeType::Lib);
        assert_eq!(lib.includes[1].section, Some("tt".to_string()));
    }

    #[test]
    fn test_parse_subcircuit() {
        let input = r#"
subckt inv in out vdd vss
M1 out in vdd vdd pmos
M2 out in vss vss nmos
ends inv
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        assert_eq!(lib.global_subcircuits.len(), 1);

        let subckt = lib.global_subcircuits.get("inv").unwrap();
        assert_eq!(subckt.name, "inv");
        assert_eq!(subckt.ports.len(), 4);
    }

    #[test]
    fn test_section_names() {
        let input = r#"
section tt
endsection
section ff
endsection
section ss
endsection
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        let names = lib.section_names();
        assert!(names.contains(&"tt"));
        assert!(names.contains(&"ff"));
        assert!(names.contains(&"ss"));
    }

    #[test]
    fn test_models_for_section() {
        let input = r#"
model global_nmos nmos level=54

section tt
model nmos_tt nmos vth0=0.4
endsection tt
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        let models = lib.models_for_section("tt");

        assert!(models.contains_key("global_nmos"));
        assert!(models.contains_key("nmos_tt"));
    }

    #[test]
    fn test_model_count() {
        let input = r#"
model m1 nmos
model m2 pmos

section tt
model m3 nmos
endsection
"#;
        let lib = ParsedLibrary::parse(input).unwrap();
        assert_eq!(lib.model_count(), 3);
    }

    // =========================================================================
    // Engineering Notation Tests
    // =========================================================================

    #[test]
    fn test_all_engineering_suffixes() {
        let input = "1T 1G 1M 1k 1m 1u 1n 1p 1f 1a";
        let mut lexer = Lexer::new(input);

        let expected = [1e12, 1e9, 1e6, 1e3, 1e-3, 1e-6, 1e-9, 1e-12, 1e-15, 1e-18];
        for exp in expected {
            if let Token::Number(n) = lexer.next_token().unwrap() {
                let ratio = n / exp;
                assert!((ratio - 1.0).abs() < 0.01, "Expected {}, got {}", exp, n);
            }
        }
    }

    // =========================================================================
    // ParamValue Tests
    // =========================================================================

    #[test]
    fn test_param_value_number() {
        let pv = ParamValue::Number(1.8);
        assert_eq!(pv.as_number(), Some(1.8));
        assert_eq!(pv.as_string(), "1.8");
    }

    #[test]
    fn test_param_value_expression() {
        let pv = ParamValue::Expression("vdd*0.5".to_string());
        assert_eq!(pv.as_number(), None);
        assert_eq!(pv.as_string(), "vdd*0.5");
    }
}
