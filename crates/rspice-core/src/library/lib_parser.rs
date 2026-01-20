//! SPICE Library File Parser
//!
//! Professional-grade parser for foundry .lib files with support for:
//! - `.lib/.endl` section syntax (corners, process variants)
//! - `.include` directive handling
//! - Model parameter extraction and validation
//! - Corner selection (TT, FF, SS, SF, FS)
//!
//! # Format Overview
//! ```text
//! .lib TT
//! .model nmos nmos level=54 ...
//! .model pmos pmos level=54 ...
//! .endl TT
//!
//! .lib FF
//! .model nmos nmos level=54 ...
//! .endl FF
//! ```

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use super::manager::{ModelDefinition, ModelType, SubcircuitDefinition};

//=============================================================================
// Library Section (Corner)
//=============================================================================

/// A named section within a .lib file (typically a process corner)
#[derive(Debug, Clone)]
pub struct LibSection {
    /// Section name (e.g., "TT", "FF", "SS", "SF", "FS")
    pub name: String,
    /// Models defined in this section
    pub models: Vec<ParsedModel>,
    /// Subcircuits defined in this section
    pub subcircuits: Vec<ParsedSubcircuit>,
}

impl LibSection {
    /// Create a new empty section
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            models: Vec::new(),
            subcircuits: Vec::new(),
        }
    }
}

//=============================================================================
// Parsed Model with Parameters
//=============================================================================

/// A fully parsed model with all parameters extracted
#[derive(Debug, Clone)]
pub struct ParsedModel {
    /// Model name
    pub name: String,
    /// Model type (NMOS, PMOS, D, NPN, etc.)
    pub model_type: ModelType,
    /// SPICE level parameter (e.g., 54 for BSIM4)
    pub level: Option<u32>,
    /// Version parameter
    pub version: Option<f64>,
    /// All model parameters as key-value pairs
    pub parameters: HashMap<String, f64>,
    /// Original parameter strings (for non-numeric values)
    pub string_params: HashMap<String, String>,
    /// Description from comments
    pub description: Option<String>,
    /// Source file path
    pub source_file: Option<PathBuf>,
    /// Line number in source file
    pub source_line: Option<usize>,
}

impl ParsedModel {
    /// Create a new parsed model
    pub fn new(name: impl Into<String>, model_type: ModelType) -> Self {
        Self {
            name: name.into(),
            model_type,
            level: None,
            version: None,
            parameters: HashMap::new(),
            string_params: HashMap::new(),
            description: None,
            source_file: None,
            source_line: None,
        }
    }

    /// Get a parameter value, returning default if not found
    pub fn get_param(&self, name: &str) -> Option<f64> {
        // Case-insensitive lookup
        let name_lower = name.to_lowercase();
        self.parameters
            .iter()
            .find(|(k, _)| k.to_lowercase() == name_lower)
            .map(|(_, v)| *v)
    }

    /// Convert to ModelDefinition for library manager
    pub fn to_model_definition(&self, library: &'static str) -> ModelDefinition {
        let mut def = ModelDefinition::new(self.name.clone(), self.model_type, library);
        if let Some(ref desc) = self.description {
            def = def.with_description(desc.clone());
        }
        def
    }
}

//=============================================================================
// Parsed Subcircuit
//=============================================================================

/// A fully parsed subcircuit with content
#[derive(Debug, Clone)]
pub struct ParsedSubcircuit {
    /// Subcircuit name
    pub name: String,
    /// Pin names in order
    pub pins: Vec<String>,
    /// Parameters with default values
    pub parameters: HashMap<String, f64>,
    /// Full subcircuit content (for expansion)
    pub content: String,
    /// Description from comments
    pub description: Option<String>,
    /// Source file path
    pub source_file: Option<PathBuf>,
}

impl ParsedSubcircuit {
    /// Create a new parsed subcircuit
    pub fn new(name: impl Into<String>, pins: Vec<String>) -> Self {
        Self {
            name: name.into(),
            pins,
            parameters: HashMap::new(),
            content: String::new(),
            description: None,
            source_file: None,
        }
    }

    /// Convert to SubcircuitDefinition for library manager
    pub fn to_subcircuit_definition(&self, library: &'static str) -> SubcircuitDefinition {
        let mut def = SubcircuitDefinition::new(self.name.clone(), self.pins.clone(), library);
        if let Some(ref desc) = self.description {
            def = def.with_description(desc.clone());
        }
        def
    }
}

//=============================================================================
// Library Parser
//=============================================================================

/// Parser for .lib format files
pub struct LibParser {
    /// Base directory for resolving .include paths
    base_dir: PathBuf,
    /// Current file being parsed
    current_file: Option<PathBuf>,
    /// Include depth (to prevent infinite recursion)
    include_depth: usize,
    /// Maximum include depth
    max_include_depth: usize,
    /// Collected sections
    sections: Vec<LibSection>,
    /// Models outside any section (top-level)
    top_level_models: Vec<ParsedModel>,
    /// Subcircuits outside any section
    top_level_subcircuits: Vec<ParsedSubcircuit>,
    /// Parse errors encountered
    errors: Vec<ParseError>,
}

/// Parse error information
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Error message
    pub message: String,
    /// File where error occurred
    pub file: Option<PathBuf>,
    /// Line number (1-indexed)
    pub line: Option<usize>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.file, self.line) {
            (Some(file), Some(line)) => {
                write!(f, "{}:{}: {}", file.display(), line, self.message)
            }
            (Some(file), None) => write!(f, "{}: {}", file.display(), self.message),
            _ => write!(f, "{}", self.message),
        }
    }
}

impl LibParser {
    /// Create a new parser with the given base directory
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
            current_file: None,
            include_depth: 0,
            max_include_depth: 10,
            sections: Vec::new(),
            top_level_models: Vec::new(),
            top_level_subcircuits: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Parse a .lib file and return the result
    pub fn parse_file(&mut self, path: impl AsRef<Path>) -> io::Result<LibParseResult> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;

        self.current_file = Some(path.to_path_buf());
        self.base_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();

        self.parse_content(&content);

        Ok(self.build_result())
    }

    /// Parse library content from a string
    pub fn parse_string(&mut self, content: &str) -> LibParseResult {
        self.parse_content(content);
        self.build_result()
    }

    /// Internal parsing implementation
    fn parse_content(&mut self, content: &str) {
        let lines = self.preprocess_lines(content);
        let mut current_section: Option<LibSection> = None;
        let mut subckt_content: Option<(ParsedSubcircuit, Vec<String>)> = None;
        let mut last_comment = String::new();

        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Track comments for descriptions
            if line.starts_with('*') {
                let comment = line.trim_start_matches('*').trim();
                if !comment.is_empty() && !comment.starts_with('=') {
                    last_comment = comment.to_string();
                }
                continue;
            }

            let upper = line.to_uppercase();

            // Handle .lib section start
            if upper.starts_with(".LIB") && !upper.contains("INCLUDE") {
                if let Some(section) = current_section.take() {
                    self.sections.push(section);
                }
                if let Some(name) = self.parse_lib_directive(line) {
                    current_section = Some(LibSection::new(name));
                }
                last_comment.clear();
                continue;
            }

            // Handle .endl section end
            if upper.starts_with(".ENDL") {
                if let Some(section) = current_section.take() {
                    self.sections.push(section);
                }
                continue;
            }

            // Handle .include
            if upper.starts_with(".INCLUDE") || upper.starts_with(".INC") {
                if let Some(include_path) = self.parse_include_directive(line) {
                    self.process_include(&include_path);
                }
                continue;
            }

            // Handle .subckt start
            if upper.starts_with(".SUBCKT") {
                if let Some(mut subckt) = self.parse_subckt_start(line) {
                    subckt.description = if last_comment.is_empty() {
                        None
                    } else {
                        Some(last_comment.clone())
                    };
                    subckt.source_file = self.current_file.clone();
                    subckt_content = Some((subckt, vec![line.to_string()]));
                }
                last_comment.clear();
                continue;
            }

            // Handle .ends
            if upper.starts_with(".ENDS") {
                if let Some((mut subckt, mut content_lines)) = subckt_content.take() {
                    content_lines.push(line.to_string());
                    subckt.content = content_lines.join("\n");

                    if let Some(ref mut section) = current_section {
                        section.subcircuits.push(subckt);
                    } else {
                        self.top_level_subcircuits.push(subckt);
                    }
                }
                continue;
            }

            // Inside subcircuit - collect content
            if let Some((_, ref mut content_lines)) = subckt_content {
                content_lines.push(line.to_string());
                continue;
            }

            // Handle .model
            if upper.starts_with(".MODEL") {
                if let Some(mut model) = self.parse_model_line(line, line_num + 1) {
                    model.description = if last_comment.is_empty() {
                        None
                    } else {
                        Some(last_comment.clone())
                    };
                    model.source_file = self.current_file.clone();
                    model.source_line = Some(line_num + 1);

                    if let Some(ref mut section) = current_section {
                        section.models.push(model);
                    } else {
                        self.top_level_models.push(model);
                    }
                }
                last_comment.clear();
            }
        }

        // Close any unclosed section
        if let Some(section) = current_section {
            self.sections.push(section);
        }
    }

    /// Preprocess content: handle line continuations
    fn preprocess_lines(&self, content: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_line = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Line continuation with +
            if trimmed.starts_with('+') {
                current_line.push(' ');
                current_line.push_str(trimmed.trim_start_matches('+').trim());
            } else {
                if !current_line.is_empty() {
                    result.push(std::mem::take(&mut current_line));
                }
                current_line = trimmed.to_string();
            }
        }

        if !current_line.is_empty() {
            result.push(current_line);
        }

        result
    }

    /// Parse .lib directive to extract section name
    fn parse_lib_directive(&self, line: &str) -> Option<String> {
        // Format: .lib section_name or .lib "section_name"
        let rest = line
            .trim()
            .strip_prefix(".lib")
            .or_else(|| line.trim().strip_prefix(".LIB"))
            .or_else(|| line.trim().strip_prefix(".Lib"))?
            .trim();

        if rest.is_empty() {
            return None;
        }

        // Handle quoted section names
        if rest.starts_with('"') {
            rest.strip_prefix('"')?
                .split('"')
                .next()
                .map(|s| s.to_string())
        } else {
            rest.split_whitespace().next().map(|s| s.to_string())
        }
    }

    /// Parse .include directive
    fn parse_include_directive(&self, line: &str) -> Option<PathBuf> {
        let rest = line.trim().to_lowercase();

        let rest = if rest.starts_with(".include") {
            &line.trim()[8..]
        } else if rest.starts_with(".inc") {
            &line.trim()[4..]
        } else {
            return None;
        };

        let rest = rest.trim();

        // Handle quoted paths
        let path_str = if rest.starts_with('"') {
            rest.strip_prefix('"')?.split('"').next()?
        } else if rest.starts_with('\'') {
            rest.strip_prefix('\'')?.split('\'').next()?
        } else {
            rest.split_whitespace().next()?
        };

        Some(PathBuf::from(path_str))
    }

    /// Process an include directive
    fn process_include(&mut self, include_path: &Path) {
        if self.include_depth >= self.max_include_depth {
            self.errors.push(ParseError {
                message: format!(
                    "Maximum include depth ({}) exceeded",
                    self.max_include_depth
                ),
                file: self.current_file.clone(),
                line: None,
            });
            return;
        }

        let full_path = if include_path.is_absolute() {
            include_path.to_path_buf()
        } else {
            self.base_dir.join(include_path)
        };

        if !full_path.exists() {
            self.errors.push(ParseError {
                message: format!("Include file not found: {}", full_path.display()),
                file: self.current_file.clone(),
                line: None,
            });
            return;
        }

        match std::fs::read_to_string(&full_path) {
            Ok(content) => {
                let saved_file = self.current_file.take();
                let saved_dir = self.base_dir.clone();

                self.current_file = Some(full_path.clone());
                self.base_dir = full_path.parent().unwrap_or(Path::new(".")).to_path_buf();
                self.include_depth += 1;

                self.parse_content(&content);

                self.include_depth -= 1;
                self.current_file = saved_file;
                self.base_dir = saved_dir;
            }
            Err(e) => {
                self.errors.push(ParseError {
                    message: format!("Failed to read include file: {}", e),
                    file: Some(full_path),
                    line: None,
                });
            }
        }
    }

    /// Parse .subckt start line
    fn parse_subckt_start(&self, line: &str) -> Option<ParsedSubcircuit> {
        // Format: .SUBCKT name pin1 pin2 ... [param1=val1 ...]
        let rest = line
            .trim()
            .strip_prefix(".SUBCKT")
            .or_else(|| line.trim().strip_prefix(".subckt"))
            .or_else(|| line.trim().strip_prefix(".Subckt"))?
            .trim();

        let mut parts = rest.split_whitespace();
        let name = parts.next()?.to_string();

        let mut pins = Vec::new();
        let mut params = HashMap::new();

        for part in parts {
            if part.contains('=') {
                // Parameter with default value
                let mut kv = part.splitn(2, '=');
                if let (Some(key), Some(val)) = (kv.next(), kv.next()) {
                    if let Ok(v) = Self::parse_spice_number(val) {
                        params.insert(key.to_string(), v);
                    }
                }
            } else {
                pins.push(part.to_string());
            }
        }

        let mut subckt = ParsedSubcircuit::new(name, pins);
        subckt.parameters = params;
        Some(subckt)
    }

    /// Parse .model line with full parameter extraction
    fn parse_model_line(&self, line: &str, _line_num: usize) -> Option<ParsedModel> {
        // Format: .MODEL name type (param1=val1 param2=val2 ...)
        // or:     .MODEL name type param1=val1 param2=val2 ...
        let rest = line
            .trim()
            .strip_prefix(".MODEL")
            .or_else(|| line.trim().strip_prefix(".model"))
            .or_else(|| line.trim().strip_prefix(".Model"))?
            .trim();

        // Split on '(' to separate name+type from parameters
        let (name_type_part, params_part) = if let Some(paren_pos) = rest.find('(') {
            (&rest[..paren_pos], Some(&rest[paren_pos..]))
        } else {
            (rest, None)
        };

        // Split name and type by whitespace
        let mut name_type_parts = name_type_part.split_whitespace();
        let name = name_type_parts.next()?.to_string();
        let type_str = name_type_parts.next()?;

        // Remaining parts before '(' could be parameters
        let extra_params: String = name_type_parts.collect::<Vec<_>>().join(" ");

        let model_type = ModelType::from_spice_type(type_str);
        let mut model = ParsedModel::new(name, model_type);

        // Parse parameters from parentheses
        if let Some(params) = params_part {
            let params_str = params.trim_start_matches('(').trim_end_matches(')');
            self.parse_model_parameters(params_str, &mut model);
        }

        // Also parse any extra params between type and '('
        if !extra_params.is_empty() {
            self.parse_model_parameters(&extra_params, &mut model);
        }

        // Extract level and version
        model.level = model.get_param("level").map(|v| v as u32);
        model.version = model.get_param("version");

        Some(model)
    }

    /// Parse model parameters string
    fn parse_model_parameters(&self, params_str: &str, model: &mut ParsedModel) {
        // Handle space or newline separated key=value pairs
        let mut current_key = String::new();
        let mut in_value = false;

        for part in params_str.split(|c: char| c.is_whitespace() || c == '(' || c == ')') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if part.contains('=') {
                let mut kv = part.splitn(2, '=');
                if let (Some(key), Some(val)) = (kv.next(), kv.next()) {
                    let key = key.trim().to_lowercase();
                    let val = val.trim();

                    if val.is_empty() {
                        current_key = key;
                        in_value = true;
                    } else if let Ok(v) = Self::parse_spice_number(val) {
                        model.parameters.insert(key, v);
                    } else {
                        model.string_params.insert(key, val.to_string());
                    }
                }
            } else if in_value && !current_key.is_empty() {
                if let Ok(v) = Self::parse_spice_number(part) {
                    model.parameters.insert(std::mem::take(&mut current_key), v);
                }
                in_value = false;
            }
        }
    }

    /// Parse SPICE number format (handles suffixes like 1e-9, 1n, 1u, etc.)
    fn parse_spice_number(s: &str) -> Result<f64, ()> {
        let s = s.trim();
        if s.is_empty() {
            return Err(());
        }

        // Try direct parse first
        if let Ok(v) = s.parse::<f64>() {
            return Ok(v);
        }

        // Handle SPICE suffixes
        let (num_part, suffix) = split_number_suffix(s);
        let base: f64 = num_part.parse().map_err(|_| ())?;

        let multiplier = match suffix.to_lowercase().as_str() {
            "t" | "tera" => 1e12,
            "g" | "giga" => 1e9,
            "meg" | "mega" | "x" => 1e6,
            "k" | "kilo" => 1e3,
            "m" | "milli" => 1e-3,
            "u" | "micro" => 1e-6,
            "n" | "nano" => 1e-9,
            "p" | "pico" => 1e-12,
            "f" | "femto" => 1e-15,
            "a" | "atto" => 1e-18,
            "" => 1.0,
            _ => return Err(()),
        };

        Ok(base * multiplier)
    }

    /// Build the final parse result
    fn build_result(&self) -> LibParseResult {
        LibParseResult {
            sections: self.sections.clone(),
            top_level_models: self.top_level_models.clone(),
            top_level_subcircuits: self.top_level_subcircuits.clone(),
            errors: self.errors.clone(),
        }
    }
}

/// Split a SPICE number into numeric part and suffix
fn split_number_suffix(s: &str) -> (&str, &str) {
    let s = s.trim();

    // Find where suffix starts (first non-numeric, non-sign, non-exponent char)
    let mut suffix_start = s.len();
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if !c.is_ascii_digit() && *c != '.' && *c != '-' && *c != '+' && *c != 'e' && *c != 'E' {
            // Check if this is part of scientific notation
            if i > 0 && (chars[i - 1] == 'e' || chars[i - 1] == 'E') {
                continue;
            }
            suffix_start = i;
            break;
        }
    }

    (&s[..suffix_start], &s[suffix_start..])
}

//=============================================================================
// Parse Result
//=============================================================================

/// Result of parsing a .lib file
#[derive(Debug, Clone)]
pub struct LibParseResult {
    /// Named sections (corners)
    pub sections: Vec<LibSection>,
    /// Models defined at top level (outside any section)
    pub top_level_models: Vec<ParsedModel>,
    /// Subcircuits defined at top level
    pub top_level_subcircuits: Vec<ParsedSubcircuit>,
    /// Errors encountered during parsing
    pub errors: Vec<ParseError>,
}

impl LibParseResult {
    /// Check if parsing was successful (no errors)
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get all section names
    pub fn section_names(&self) -> Vec<&str> {
        self.sections.iter().map(|s| s.name.as_str()).collect()
    }

    /// Get a section by name
    pub fn get_section(&self, name: &str) -> Option<&LibSection> {
        self.sections
            .iter()
            .find(|s| s.name.eq_ignore_ascii_case(name))
    }

    /// Get all models from a specific section
    pub fn models_in_section(&self, section_name: &str) -> Vec<&ParsedModel> {
        self.get_section(section_name)
            .map(|s| s.models.iter().collect())
            .unwrap_or_default()
    }

    /// Get all models (top-level + all sections)
    pub fn all_models(&self) -> Vec<&ParsedModel> {
        let mut models: Vec<_> = self.top_level_models.iter().collect();
        for section in &self.sections {
            models.extend(section.models.iter());
        }
        models
    }

    /// Find a model by name (searches all sections)
    pub fn find_model(&self, name: &str) -> Option<&ParsedModel> {
        // First check top-level
        if let Some(m) = self
            .top_level_models
            .iter()
            .find(|m| m.name.eq_ignore_ascii_case(name))
        {
            return Some(m);
        }
        // Then check all sections
        for section in &self.sections {
            if let Some(m) = section
                .models
                .iter()
                .find(|m| m.name.eq_ignore_ascii_case(name))
            {
                return Some(m);
            }
        }
        None
    }

    /// Get total model count
    pub fn model_count(&self) -> usize {
        self.top_level_models.len() + self.sections.iter().map(|s| s.models.len()).sum::<usize>()
    }

    /// Get total subcircuit count
    pub fn subcircuit_count(&self) -> usize {
        self.top_level_subcircuits.len()
            + self
                .sections
                .iter()
                .map(|s| s.subcircuits.len())
                .sum::<usize>()
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_spice_number() {
        assert_eq!(LibParser::parse_spice_number("1.5"), Ok(1.5));
        assert_eq!(LibParser::parse_spice_number("1e-9"), Ok(1e-9));
        assert_eq!(LibParser::parse_spice_number("1n"), Ok(1e-9));
        assert_eq!(LibParser::parse_spice_number("1.5u"), Ok(1.5e-6));
        assert_eq!(LibParser::parse_spice_number("10k"), Ok(10e3));
        assert_eq!(LibParser::parse_spice_number("1meg"), Ok(1e6));
        assert_eq!(LibParser::parse_spice_number("2.5p"), Ok(2.5e-12));
    }

    #[test]
    fn test_parse_model_line() {
        let content = ".MODEL NMOS1 NMOS (LEVEL=54 VTH0=0.3 K1=0.5)";

        let mut test_parser = LibParser::new(".");
        test_parser.parse_content(content);

        assert_eq!(test_parser.top_level_models.len(), 1);
        let model = &test_parser.top_level_models[0];
        assert_eq!(model.name, "NMOS1");
        assert_eq!(model.model_type, ModelType::Nmos);
        assert_eq!(model.level, Some(54));
    }

    #[test]
    fn test_parse_lib_sections() {
        let content = r#"
* Test PDK Library
.lib TT
.MODEL nch NMOS LEVEL=54 VTH0=0.4
.MODEL pch PMOS LEVEL=54 VTH0=-0.4
.endl TT

.lib FF
.MODEL nch NMOS LEVEL=54 VTH0=0.35
.MODEL pch PMOS LEVEL=54 VTH0=-0.35
.endl FF
"#;
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(content);

        assert_eq!(result.sections.len(), 2);
        assert_eq!(result.section_names(), vec!["TT", "FF"]);

        let tt = result.get_section("TT").unwrap();
        assert_eq!(tt.models.len(), 2);

        let nch = result.find_model("nch").unwrap();
        assert_eq!(nch.model_type, ModelType::Nmos);
    }

    #[test]
    fn test_parse_subcircuit() {
        let content = r#"
.SUBCKT inverter IN OUT VDD VSS
M1 OUT IN VDD VDD pch W=1u L=100n
M2 OUT IN VSS VSS nch W=500n L=100n
.ENDS inverter
"#;
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(content);

        assert_eq!(result.top_level_subcircuits.len(), 1);
        let inv = &result.top_level_subcircuits[0];
        assert_eq!(inv.name, "inverter");
        assert_eq!(inv.pins, vec!["IN", "OUT", "VDD", "VSS"]);
    }
}
