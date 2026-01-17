//! SPICE Syntax Highlighting
//!
//! Tokenizes SPICE netlist text for syntax highlighting.

/// Token types for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    /// Comment (lines starting with * or $)
    Comment,
    /// SPICE command (.TRAN, .AC, .OP, etc.)
    Command,
    /// Numeric value (1k, 1.5e-3, etc.)
    Number,
    /// Component name (R1, C1, V1, etc.)
    Component,
    /// Node name
    Node,
    /// Model/subcircuit name
    Model,
    /// Keyword (DC, AC, SIN, PULSE, etc.)
    Keyword,
    /// Plain text
    Text,
}

impl TokenType {
    /// Get the color for this token type
    pub fn color(&self) -> &'static str {
        match self {
            TokenType::Comment => "#6a9955",   // Green
            TokenType::Command => "#569cd6",   // Blue
            TokenType::Number => "#ce9178",    // Orange
            TokenType::Component => "#dcdcaa", // Yellow
            TokenType::Node => "#9cdcfe",      // Light blue
            TokenType::Model => "#c586c0",     // Purple
            TokenType::Keyword => "#569cd6",   // Blue
            TokenType::Text => "#d4d4d4",      // Gray
        }
    }
}

/// A highlighted token
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
}

/// Tokenize a line of SPICE netlist for syntax highlighting
pub fn tokenize_line(line: &str) -> Vec<Token> {
    let trimmed = line.trim_start();

    // Empty line
    if trimmed.is_empty() {
        return vec![Token {
            text: line.to_string(),
            token_type: TokenType::Text,
        }];
    }

    // Comment line
    if trimmed.starts_with('*') || trimmed.starts_with('$') || trimmed.starts_with(';') {
        return vec![Token {
            text: line.to_string(),
            token_type: TokenType::Comment,
        }];
    }

    // Command line
    if trimmed.starts_with('.') {
        return tokenize_command_line(line);
    }

    // Element line (component)
    tokenize_element_line(line)
}

/// Tokenize a command line (.TRAN, .AC, etc.)
fn tokenize_command_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();
    let mut current = String::new();

    // Preserve leading whitespace
    while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
        current.push(chars.next().unwrap());
    }
    if !current.is_empty() {
        tokens.push(Token {
            text: current.clone(),
            token_type: TokenType::Text,
        });
        current.clear();
    }

    // Read command word
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '.' || c == '_' {
            current.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    if !current.is_empty() {
        tokens.push(Token {
            text: current.clone(),
            token_type: TokenType::Command,
        });
        current.clear();
    }

    // Rest of the line - tokenize as values
    tokenize_values(&mut chars, &mut tokens);

    tokens
}

/// Tokenize an element line (R1, C1, V1, etc.)
fn tokenize_element_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();
    let mut current = String::new();
    let mut first_word = true;

    while chars.peek().is_some() {
        let c = *chars.peek().unwrap();

        // Whitespace
        if c == ' ' || c == '\t' {
            if !current.is_empty() {
                let token_type = classify_word(&current, first_word);
                tokens.push(Token {
                    text: current.clone(),
                    token_type,
                });
                current.clear();
                first_word = false;
            }
            current.push(chars.next().unwrap());
            while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                current.push(chars.next().unwrap());
            }
            tokens.push(Token {
                text: current.clone(),
                token_type: TokenType::Text,
            });
            current.clear();
            continue;
        }

        // Continue building word
        current.push(chars.next().unwrap());
    }

    // Final word
    if !current.is_empty() {
        let token_type = classify_word(&current, first_word);
        tokens.push(Token {
            text: current,
            token_type,
        });
    }

    tokens
}

/// Tokenize value arguments
fn tokenize_values(chars: &mut std::iter::Peekable<std::str::Chars>, tokens: &mut Vec<Token>) {
    let mut current = String::new();

    while chars.peek().is_some() {
        let c = *chars.peek().unwrap();

        // Whitespace
        if c == ' ' || c == '\t' {
            if !current.is_empty() {
                let token_type = classify_word(&current, false);
                tokens.push(Token {
                    text: current.clone(),
                    token_type,
                });
                current.clear();
            }
            current.push(chars.next().unwrap());
            while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                current.push(chars.next().unwrap());
            }
            tokens.push(Token {
                text: current.clone(),
                token_type: TokenType::Text,
            });
            current.clear();
            continue;
        }

        // Continue building word
        current.push(chars.next().unwrap());
    }

    // Final word
    if !current.is_empty() {
        let token_type = classify_word(&current, false);
        tokens.push(Token {
            text: current,
            token_type,
        });
    }
}

/// Classify a word as a token type
fn classify_word(word: &str, is_first: bool) -> TokenType {
    let upper = word.to_uppercase();

    // Commands
    if word.starts_with('.') {
        return TokenType::Command;
    }

    // First word is component name
    if is_first && word.len() >= 1 {
        let first_char = word.chars().next().unwrap().to_ascii_uppercase();
        if "RLCVIBQMDJKXWTSEFGH".contains(first_char) {
            return TokenType::Component;
        }
    }

    // Keywords
    let keywords = [
        "DC", "AC", "TRAN", "SIN", "PULSE", "PWL", "EXP", "SFFM", "DEC", "OCT", "LIN", "OP", "IC",
        "UIC", "OFF", "ON",
    ];
    if keywords.contains(&upper.as_str()) {
        return TokenType::Keyword;
    }

    // Numbers (may contain SI prefixes)
    if is_number(word) {
        return TokenType::Number;
    }

    // Node names (typically numbers or short identifiers)
    if word.chars().all(|c| c.is_alphanumeric() || c == '_') {
        if word
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
        {
            return TokenType::Node;
        }
        // Short names are likely nodes
        if word.len() <= 4 {
            return TokenType::Node;
        }
    }

    TokenType::Text
}

/// Check if a string is a number (with optional SI prefix)
fn is_number(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }

    // Remove optional sign
    let s = s
        .strip_prefix('-')
        .or_else(|| s.strip_prefix('+'))
        .unwrap_or(s);
    if s.is_empty() {
        return false;
    }

    // Check for digits, decimal, exponent, and SI prefixes
    let mut has_digit = false;
    let mut in_exponent = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if c == '.' && !in_exponent {
            // Decimal point
        } else if (c == 'e' || c == 'E') && has_digit && !in_exponent {
            in_exponent = true;
        } else if (c == '+' || c == '-') && in_exponent && i > 0 {
            // Exponent sign
        } else if i == s.len() - 1 || (i == s.len() - 2 && s.ends_with("eg")) {
            // SI prefix at end: f, p, n, u, m, k, K, M, G, T, meg
            let suffixes = ['f', 'p', 'n', 'u', 'µ', 'm', 'k', 'K', 'M', 'G', 'T'];
            if suffixes.contains(&c) || s.to_lowercase().ends_with("meg") {
                return has_digit;
            }
            return false;
        } else {
            return false;
        }
    }

    has_digit
}
