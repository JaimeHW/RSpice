//! Verilog-A Preprocessor
//!
//! Implements a full C-like preprocessor for Verilog-A source files.
//! Handles:
//! - `include "file"` / `include <file>` - File inclusion
//! - `define NAME value` - Macro definitions
//! - `define NAME(args) value` - Parameterized macros
//! - `ifdef NAME` / `ifndef NAME` / `else` / `elsif NAME` / `endif` - Conditionals
//! - `undef NAME` - Undefine macros
//!
//! This preprocessor runs before lexing, producing a single expanded source string.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Error type for preprocessor errors
#[derive(Debug, Clone)]
pub struct PreprocessorError {
    pub message: String,
    pub file: Option<PathBuf>,
    pub line: usize,
}

impl std::fmt::Display for PreprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file {
            Some(path) => write!(f, "{}:{}: {}", path.display(), self.line, self.message),
            None => write!(f, "line {}: {}", self.line, self.message),
        }
    }
}

impl std::error::Error for PreprocessorError {}

impl PreprocessorError {
    pub fn new(message: impl Into<String>, file: Option<PathBuf>, line: usize) -> Self {
        Self {
            message: message.into(),
            file,
            line,
        }
    }
}

/// Macro definition with optional parameters
#[derive(Debug, Clone)]
pub struct MacroDef {
    /// Parameter names (empty for simple macros)
    pub params: Vec<String>,
    /// Replacement text
    pub body: String,
}

impl MacroDef {
    pub fn simple(body: impl Into<String>) -> Self {
        Self {
            params: Vec::new(),
            body: body.into(),
        }
    }

    pub fn with_params(params: Vec<String>, body: impl Into<String>) -> Self {
        Self {
            params,
            body: body.into(),
        }
    }

    /// Expand macro with given arguments
    /// Uses word-boundary matching to prevent replacing substrings within identifiers
    pub fn expand(&self, args: &[String]) -> String {
        if self.params.is_empty() {
            return self.body.clone();
        }

        let mut result = self.body.clone();
        for (i, param) in self.params.iter().enumerate() {
            if let Some(arg) = args.get(i) {
                // Replace parameter with argument using word-boundary matching
                // Only replace when param is a complete identifier, not part of another word
                result = Self::replace_identifier(&result, param, arg);
            }
        }
        result
    }

    /// Replace identifier occurrences respecting word boundaries
    fn replace_identifier(text: &str, pattern: &str, replacement: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        while let Some(ch) = chars.next() {
            // Check if this could be the start of an identifier match
            if ch == pattern_chars[0] {
                // Check if we're not in the middle of an identifier
                let prev_was_alnum = result
                    .chars()
                    .last()
                    .map(|c| c.is_ascii_alphanumeric() || c == '_')
                    .unwrap_or(false);

                if prev_was_alnum {
                    // We're in the middle of an identifier, can't match
                    result.push(ch);
                    continue;
                }

                // Try to match the pattern
                let mut matched = true;
                let mut consumed: Vec<char> = vec![ch];

                for &pat_ch in &pattern_chars[1..] {
                    if let Some(&next) = chars.peek() {
                        if next == pat_ch {
                            consumed.push(chars.next().unwrap());
                        } else {
                            matched = false;
                            break;
                        }
                    } else {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    // Check if the next character is NOT an identifier char (word boundary)
                    let next_is_alnum = chars
                        .peek()
                        .map(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .unwrap_or(false);

                    if !next_is_alnum {
                        // Full match with word boundary - replace!
                        result.push_str(replacement);
                    } else {
                        // Next char is part of identifier, not a match
                        for c in consumed {
                            result.push(c);
                        }
                    }
                } else {
                    // Pattern didn't match completely
                    for c in consumed {
                        result.push(c);
                    }
                }
            } else {
                result.push(ch);
            }
        }

        result
    }
}

/// Verilog-A Preprocessor
pub struct Preprocessor {
    /// Defined macros
    macros: HashMap<String, MacroDef>,
    /// Include search paths
    include_paths: Vec<PathBuf>,
    /// Files currently being included (for circular detection)
    include_stack: HashSet<PathBuf>,
    /// Current file being processed
    current_file: Option<PathBuf>,
    /// Canonical dependency set encountered during preprocessing.
    dependencies: BTreeSet<PathBuf>,
}

impl Default for Preprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor {
    /// Create a new preprocessor with standard definitions
    pub fn new() -> Self {
        let mut pp = Self {
            macros: HashMap::new(),
            include_paths: Vec::new(),
            include_stack: HashSet::new(),
            current_file: None,
            dependencies: BTreeSet::new(),
        };

        // Add standard Verilog-AMS predefined macros
        pp.define("__VAMS_ENABLE__", MacroDef::simple("1"));
        pp.define("__VAMS_COMPACT_MODELING__", MacroDef::simple("1"));

        pp
    }

    /// Add an include search path
    pub fn add_include_path(&mut self, path: impl Into<PathBuf>) {
        self.include_paths.push(path.into());
    }

    /// Define a macro
    pub fn define(&mut self, name: impl Into<String>, def: MacroDef) {
        self.macros.insert(name.into(), def);
    }

    /// Check if a macro is defined
    pub fn is_defined(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    /// Undefine a macro
    pub fn undefine(&mut self, name: &str) {
        self.macros.remove(name);
    }

    /// Preprocess a file, returning the expanded source
    pub fn preprocess_file(&mut self, path: &Path) -> Result<String, PreprocessorError> {
        let is_root_call = self.include_stack.is_empty();
        if is_root_call {
            self.dependencies.clear();
        }

        let canonical = path.canonicalize().map_err(|e| {
            PreprocessorError::new(
                format!("Cannot open file: {}", e),
                Some(path.to_path_buf()),
                0,
            )
        })?;

        self.dependencies.insert(canonical.clone());

        // Add the file's directory to include paths
        if let Some(parent) = canonical.parent()
            && !self.include_paths.contains(&parent.to_path_buf())
        {
            self.include_paths.insert(0, parent.to_path_buf());
        }

        // Check for circular includes
        if self.include_stack.contains(&canonical) {
            return Err(PreprocessorError::new(
                format!("Circular include detected: {}", canonical.display()),
                Some(path.to_path_buf()),
                0,
            ));
        }

        self.include_stack.insert(canonical.clone());
        let prev_file = self.current_file.replace(canonical.clone());

        let content = std::fs::read_to_string(&canonical).map_err(|e| {
            PreprocessorError::new(
                format!("Cannot read file: {}", e),
                Some(path.to_path_buf()),
                0,
            )
        })?;

        let result = self.preprocess_source(&content);

        self.include_stack.remove(&canonical);
        self.current_file = prev_file;

        result
    }

    /// Return canonical source/include dependencies captured during the most
    /// recent top-level `preprocess_file` call.
    pub fn dependencies(&self) -> Vec<PathBuf> {
        self.dependencies.iter().cloned().collect()
    }

    /// Consume and return the captured dependency list.
    pub fn take_dependencies(&mut self) -> Vec<PathBuf> {
        std::mem::take(&mut self.dependencies).into_iter().collect()
    }

    /// Preprocess source string
    pub fn preprocess_source(&mut self, source: &str) -> Result<String, PreprocessorError> {
        let mut output = String::with_capacity(source.len());
        let mut lines = source.lines().enumerate().peekable();

        // Conditional stack: true = include this section, false = skip
        let mut cond_stack: Vec<bool> = Vec::new();
        // Track if we've seen a true branch at each level (for else handling)
        let mut seen_true: Vec<bool> = Vec::new();

        while let Some((line_num, line)) = lines.next() {
            let trimmed = line.trim();

            // Check if we're in a skipped conditional block
            let include_line = cond_stack.iter().all(|&b| b);

            // Handle directives
            if trimmed.starts_with('`') {
                let (directive, rest) = Self::split_directive(trimmed);

                match directive {
                    "include" => {
                        if include_line {
                            let included = self.handle_include(rest, line_num + 1)?;
                            output.push_str(&included);
                            output.push('\n');
                        }
                    }
                    "define" => {
                        if include_line {
                            // Handle multi-line defines with backslash continuation
                            let mut full_define = rest.to_string();
                            output.push('\n'); // Line for the `define directive

                            // Check if this define continues on next lines (ends with backslash)
                            while full_define.trim_end().ends_with('\\') {
                                // Remove trailing backslash
                                let trimmed = full_define.trim_end();
                                full_define = trimmed[..trimmed.len() - 1].to_string();
                                full_define.push(' ');

                                // Consume next line
                                if let Some((_, next_line)) = lines.next() {
                                    full_define.push_str(next_line);
                                    output.push('\n'); // Keep line count consistent
                                } else {
                                    break;
                                }
                            }

                            self.handle_define(&full_define, line_num + 1)?;
                        } else {
                            output.push('\n');
                            // Still need to skip continuation lines even when not including
                            let mut current_line = rest;
                            while current_line.trim_end().ends_with('\\') {
                                if let Some((_, next_line)) = lines.next() {
                                    current_line = next_line;
                                    output.push('\n');
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    "undef" => {
                        if include_line {
                            let name = rest.trim();
                            self.undefine(name);
                        }
                        output.push('\n');
                    }
                    "ifdef" => {
                        let name = rest.trim();
                        let is_defined = self.is_defined(name);
                        let include = include_line && is_defined;
                        cond_stack.push(include);
                        seen_true.push(is_defined);
                        output.push('\n');
                    }
                    "ifndef" => {
                        let name = rest.trim();
                        let is_defined = self.is_defined(name);
                        let include = include_line && !is_defined;
                        cond_stack.push(include);
                        seen_true.push(!is_defined);
                        output.push('\n');
                    }
                    "else" => {
                        if cond_stack.is_empty() {
                            return Err(PreprocessorError::new(
                                "`else without matching `ifdef/`ifndef",
                                self.current_file.clone(),
                                line_num + 1,
                            ));
                        }
                        let parent_active = cond_stack.len() <= 1
                            || cond_stack[..cond_stack.len() - 1].iter().all(|&b| b);
                        let already_true = seen_true.last().copied().unwrap_or(false);
                        let new_val = parent_active && !already_true;
                        *cond_stack.last_mut().unwrap() = new_val;
                        if new_val {
                            *seen_true.last_mut().unwrap() = true;
                        }
                        output.push('\n');
                    }
                    "elsif" => {
                        if cond_stack.is_empty() {
                            return Err(PreprocessorError::new(
                                "`elsif without matching `ifdef/`ifndef",
                                self.current_file.clone(),
                                line_num + 1,
                            ));
                        }
                        let name = rest.trim();
                        let parent_active = cond_stack.len() <= 1
                            || cond_stack[..cond_stack.len() - 1].iter().all(|&b| b);
                        let already_true = seen_true.last().copied().unwrap_or(false);
                        let is_defined = self.is_defined(name);
                        let new_val = parent_active && !already_true && is_defined;
                        *cond_stack.last_mut().unwrap() = new_val;
                        if new_val {
                            *seen_true.last_mut().unwrap() = true;
                        }
                        output.push('\n');
                    }
                    "endif" => {
                        if cond_stack.is_empty() {
                            return Err(PreprocessorError::new(
                                "`endif without matching `ifdef/`ifndef",
                                self.current_file.clone(),
                                line_num + 1,
                            ));
                        }
                        cond_stack.pop();
                        seen_true.pop();
                        output.push('\n');
                    }
                    _ => {
                        // Unknown directive or macro invocation - output as-is for later expansion
                        if include_line {
                            output.push_str(line);
                            output.push('\n');
                        } else {
                            output.push('\n');
                        }
                    }
                }
            } else if include_line {
                // Regular line - output as-is for later expansion
                output.push_str(line);
                output.push('\n');
            } else {
                // Skipped line - preserve line count
                output.push('\n');
            }
        }

        // Check for unclosed conditionals
        if !cond_stack.is_empty() {
            return Err(PreprocessorError::new(
                format!("Unclosed conditional ({} `endif missing)", cond_stack.len()),
                self.current_file.clone(),
                0,
            ));
        }

        // Final pass: expand all macros on the complete output
        // This handles multi-line macro invocations correctly
        self.expand_macros(&output)
    }

    /// Split a directive line into directive name and rest
    fn split_directive(line: &str) -> (&str, &str) {
        let line = &line[1..]; // Skip the backtick
        if let Some(space_pos) = line.find(|c: char| c.is_whitespace()) {
            (&line[..space_pos], line[space_pos..].trim())
        } else {
            (line, "")
        }
    }

    /// Handle `include directive
    fn handle_include(&mut self, rest: &str, line_num: usize) -> Result<String, PreprocessorError> {
        let rest = rest.trim();

        // Parse filename from "file" or <file>
        let filename = if (rest.starts_with('"') && rest.ends_with('"'))
            || (rest.starts_with('<') && rest.ends_with('>'))
        {
            &rest[1..rest.len() - 1]
        } else {
            return Err(PreprocessorError::new(
                format!("Invalid include syntax: {}", rest),
                self.current_file.clone(),
                line_num,
            ));
        };

        // Search for the file
        let path = self.find_include(filename).ok_or_else(|| {
            PreprocessorError::new(
                format!("Include file not found: {}", filename),
                self.current_file.clone(),
                line_num,
            )
        })?;

        // Recursively preprocess
        self.preprocess_file(&path)
    }

    /// Find an include file in search paths
    fn find_include(&self, filename: &str) -> Option<PathBuf> {
        // First try relative to current file
        if let Some(ref current) = self.current_file
            && let Some(parent) = current.parent()
        {
            let path = parent.join(filename);
            if path.exists() {
                return Some(path);
            }
        }

        // Search include paths
        for inc_path in &self.include_paths {
            let path = inc_path.join(filename);
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    /// Handle `define directive
    fn handle_define(&mut self, rest: &str, _line_num: usize) -> Result<(), PreprocessorError> {
        let rest = rest.trim();

        // Check for parameterized macro: NAME(a, b, c) body
        if let Some(paren_pos) = rest.find('(') {
            let name = &rest[..paren_pos];
            let after_name = &rest[paren_pos..];

            if let Some(close_paren) = after_name.find(')') {
                let params_str = &after_name[1..close_paren];
                let params: Vec<String> = params_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                let body = after_name[close_paren + 1..].trim().to_string();
                self.define(name.trim(), MacroDef::with_params(params, body));
            }
        } else {
            // Simple macro: NAME body
            let mut parts = rest.splitn(2, char::is_whitespace);
            let name = parts.next().unwrap_or("");
            let body = parts.next().unwrap_or("").trim();

            if !name.is_empty() {
                self.define(name, MacroDef::simple(body));
            }
        }

        Ok(())
    }

    /// Expand macros in a line (with recursive expansion for nested macros)
    fn expand_macros(&self, line: &str) -> Result<String, PreprocessorError> {
        let mut result = line.to_string();

        // Recursively expand until no more macros are found (max 100 iterations for safety)
        for _ in 0..100 {
            let expanded = self.expand_macros_single_pass(&result)?;
            if expanded == result {
                break; // No more expansions
            }
            result = expanded;
        }

        Ok(result)
    }

    /// Single pass of macro expansion
    fn expand_macros_single_pass(&self, line: &str) -> Result<String, PreprocessorError> {
        let mut result = String::with_capacity(line.len());
        let mut chars = line.chars().peekable();
        let mut in_string = false;

        while let Some(ch) = chars.next() {
            if ch == '"' && !in_string {
                in_string = true;
                result.push(ch);
            } else if ch == '"' && in_string {
                in_string = false;
                result.push(ch);
            } else if ch == '\\' && in_string {
                // Escape in string
                result.push(ch);
                if let Some(next) = chars.next() {
                    result.push(next);
                }
            } else if ch == '`' && !in_string {
                // Potential macro invocation
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Some(macro_def) = self.macros.get(&name) {
                    // Check for arguments
                    if !macro_def.params.is_empty() && chars.peek() == Some(&'(') {
                        chars.next(); // consume '('
                        let args = Self::parse_macro_args(&mut chars);
                        result.push_str(&macro_def.expand(&args));
                    } else {
                        result.push_str(&macro_def.expand(&[]));
                    }
                } else {
                    // Not a defined macro, keep as-is (might be a directive)
                    result.push('`');
                    result.push_str(&name);
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    /// Parse macro arguments from function-like invocation
    fn parse_macro_args(chars: &mut std::iter::Peekable<std::str::Chars>) -> Vec<String> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut paren_depth = 1;

        for ch in chars.by_ref() {
            match ch {
                '(' => {
                    paren_depth += 1;
                    current_arg.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        args.push(current_arg.trim().to_string());
                        break;
                    }
                    current_arg.push(ch);
                }
                ',' if paren_depth == 1 => {
                    args.push(current_arg.trim().to_string());
                    current_arg.clear();
                }
                _ => current_arg.push(ch),
            }
        }

        args
    }
}

//=============================================================================
// Tests
//=============================================================================
