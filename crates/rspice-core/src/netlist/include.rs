//! .INCLUDE and .LIB directive processing
//!
//! Handles file inclusion for SPICE netlists, supporting:
//! - `.INCLUDE "filename"` - Include entire file contents
//! - `.LIB "filename" [section]` - Include library section
//!
//! Features:
//! - Relative path resolution from parent file
//! - Circular inclusion detection
//! - Library section extraction
//! - Case-insensitive matching for Windows compatibility

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::ParseError;

//=============================================================================
// Include Processor
//=============================================================================

/// Processes .INCLUDE and .LIB directives
///
/// Maintains state to prevent infinite recursion from circular includes
/// and resolves relative paths based on the including file's location.
#[derive(Debug)]
pub struct IncludeProcessor {
    /// Base directory for resolving relative paths
    base_dir: PathBuf,
    /// Currently active include/lib stack entries used for recursion detection
    active_includes: HashSet<IncludeKey>,
    /// Additional library search paths
    lib_paths: Vec<PathBuf>,
    /// Maximum include depth to prevent stack overflow
    max_depth: usize,
    /// Current include depth
    current_depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IncludeKey {
    path: PathBuf,
    section: Option<String>,
}

impl IncludeKey {
    fn new(path: PathBuf, section: Option<&str>) -> Self {
        Self {
            path,
            section: section.map(|name| name.to_ascii_uppercase()),
        }
    }

    fn describe(&self) -> String {
        match &self.section {
            Some(section) => format!("{} [{}]", self.path.display(), section),
            None => self.path.display().to_string(),
        }
    }
}

impl IncludeProcessor {
    /// Create a new include processor
    ///
    /// # Arguments
    /// * `base_path` - Path to the main netlist file (or its directory)
    pub fn new(base_path: &Path) -> Self {
        let base_dir = if base_path.is_file() {
            base_path.parent().unwrap_or(Path::new(".")).to_path_buf()
        } else {
            base_path.to_path_buf()
        };

        Self {
            base_dir,
            active_includes: HashSet::new(),
            lib_paths: Vec::new(),
            max_depth: 10, // Reasonable limit for include nesting
            current_depth: 0,
        }
    }

    /// Add a library search path
    pub fn add_lib_path(&mut self, path: PathBuf) {
        if !self.lib_paths.contains(&path) {
            self.lib_paths.push(path);
        }
    }

    /// Process a .INCLUDE directive
    ///
    /// Reads and returns the entire contents of the specified file.
    ///
    /// # Arguments
    /// * `filename` - Path to include (relative to base_dir or absolute)
    ///
    /// # Returns
    /// The file contents, or an error if the file cannot be read
    pub fn process_include(&mut self, filename: &str) -> Result<String, ParseError> {
        let base_dir = self.base_dir.clone();
        self.process_include_from(&base_dir, filename)
    }

    /// Process a .LIB directive
    ///
    /// Reads a library file and extracts the specified section.
    /// If no section is specified, returns the entire file.
    ///
    /// # Arguments
    /// * `filename` - Path to library file
    /// * `section` - Optional section name to extract
    ///
    /// # Returns
    /// The section contents, or an error if not found
    pub fn process_lib(
        &mut self,
        filename: &str,
        section: Option<&str>,
    ) -> Result<String, ParseError> {
        let base_dir = self.base_dir.clone();
        self.process_lib_from(&base_dir, filename, section)
    }

    /// Recursively expand `.INCLUDE` and `.LIB` directives in raw content.
    pub fn expand_content(
        &mut self,
        content: &str,
        current_path: &Path,
    ) -> Result<String, ParseError> {
        let current_dir = current_path.parent().unwrap_or(Path::new("."));
        self.expand_content_from(content, current_dir)
    }

    /// Resolve a filename to an absolute path
    fn resolve_path_from(&self, base_dir: &Path, filename: &str) -> Result<PathBuf, ParseError> {
        // Remove quotes if present
        let clean_name = filename.trim_matches('"').trim_matches('\'');

        let path = Path::new(clean_name);

        // If absolute, use as-is
        if path.is_absolute() {
            if path.exists() {
                return Ok(path.to_path_buf());
            }
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("File not found: {}", clean_name),
            });
        }

        // Try relative to base directory first
        let relative = base_dir.join(path);
        if relative.exists() {
            return Ok(relative);
        }

        // Try library search paths
        for lib_path in &self.lib_paths {
            let candidate = lib_path.join(path);
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        // Try common library locations
        let common_paths = ["lib", "models", "../lib", "../models"];

        for common in common_paths {
            let candidate = base_dir.join(common).join(path);
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(ParseError::Syntax {
            line: 0,
            message: format!(
                "Include file not found: {} (searched {})",
                clean_name,
                base_dir.display()
            ),
        })
    }

    fn process_include_from(
        &mut self,
        base_dir: &Path,
        filename: &str,
    ) -> Result<String, ParseError> {
        let path = self.resolve_path_from(base_dir, filename)?;
        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        let key = IncludeKey::new(canonical, None);
        self.enter_include(&key)?;

        let result = (|| {
            let content = fs::read_to_string(&path).map_err(|e| ParseError::Syntax {
                line: 0,
                message: format!("Failed to include '{}': {}", filename, e),
            })?;

            let content = strip_terminal_end_cards(&content);
            self.expand_content(&content, &path)
        })();

        self.leave_include(&key);
        result
    }

    fn process_lib_from(
        &mut self,
        base_dir: &Path,
        filename: &str,
        section: Option<&str>,
    ) -> Result<String, ParseError> {
        let path = self.resolve_path_from(base_dir, filename)?;
        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        let key = IncludeKey::new(canonical, section);
        self.enter_include(&key)?;

        let result = (|| {
            let content = fs::read_to_string(&path).map_err(|e| ParseError::Syntax {
                line: 0,
                message: format!("Failed to include '{}': {}", filename, e),
            })?;
            let content = strip_terminal_end_cards(&content);
            let selected = match section {
                Some(sect) => self.extract_section(&content, sect)?,
                None => content,
            };
            self.expand_content(&selected, &path)
        })();

        self.leave_include(&key);
        result
    }

    fn expand_content_from(
        &mut self,
        content: &str,
        base_dir: &Path,
    ) -> Result<String, ParseError> {
        let mut result = String::new();

        for line in content.lines() {
            let trimmed = line.trim();
            let upper = trimmed.to_ascii_uppercase();

            if (upper.starts_with(".INCLUDE") || upper.starts_with(".INC "))
                && let Some(filename) = parse_include_directive(trimmed)
            {
                let included = self.process_include_from(base_dir, &filename)?;
                result.push_str(&included);
                if !included.ends_with('\n') {
                    result.push('\n');
                }
                continue;
            }

            if upper.starts_with(".LIB")
                && !upper.starts_with(".LIBS")
                && let Some((filename, section)) = parse_lib_directive(trimmed)
            {
                let included = self.process_lib_from(base_dir, &filename, section.as_deref())?;
                result.push_str(&included);
                if !included.ends_with('\n') {
                    result.push('\n');
                }
                continue;
            }

            result.push_str(line);
            result.push('\n');
        }

        Ok(result)
    }

    fn enter_include(&mut self, key: &IncludeKey) -> Result<(), ParseError> {
        if self.current_depth >= self.max_depth {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Include depth exceeded maximum of {}", self.max_depth),
            });
        }

        if !self.active_includes.insert(key.clone()) {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Circular include/lib detected: {}", key.describe()),
            });
        }

        self.current_depth += 1;
        Ok(())
    }

    fn leave_include(&mut self, key: &IncludeKey) {
        self.current_depth = self.current_depth.saturating_sub(1);
        self.active_includes.remove(key);
    }

    /// Extract a named section from library content
    ///
    /// Library sections are delimited by:
    /// ```text
    /// .LIB section_name
    /// ... content ...
    /// .ENDL [section_name]
    /// ```
    fn extract_section(&self, content: &str, section: &str) -> Result<String, ParseError> {
        let _section_upper = section.to_uppercase();
        let mut in_section = false;
        let mut section_content = Vec::new();
        let mut found = false;

        for line in content.lines() {
            let trimmed = line.trim();
            let upper = trimmed.to_uppercase();

            if upper.starts_with(".LIB") && !upper.starts_with(".LIBS") {
                // Check if this is our section start
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 && parts[1].eq_ignore_ascii_case(section) {
                    in_section = true;
                    found = true;
                    continue;
                }
            }

            if in_section {
                if upper.starts_with(".ENDL") {
                    // Check if this ends our section
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() == 1
                        || (parts.len() >= 2 && parts[1].eq_ignore_ascii_case(section))
                    {
                        in_section = false;
                        break;
                    }
                }
                section_content.push(line);
            }
        }

        if !found {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Library section '{}' not found", section),
            });
        }

        if in_section {
            log::warn!("Library section '{}' missing .ENDL", section);
        }

        Ok(section_content.join("\n"))
    }

    /// Reset the processor for a new netlist
    pub fn reset(&mut self) {
        self.active_includes.clear();
        self.current_depth = 0;
    }

    /// Set base directory (useful when changing context)
    pub fn set_base_dir(&mut self, path: &Path) {
        self.base_dir = if path.is_file() {
            path.parent().unwrap_or(Path::new(".")).to_path_buf()
        } else {
            path.to_path_buf()
        };
    }
}

impl Default for IncludeProcessor {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}

fn strip_terminal_end_cards(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    for line in content.lines() {
        if line.trim().eq_ignore_ascii_case(".END") {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

//=============================================================================
// Helper Functions
//=============================================================================

/// Parse an include or lib directive line
///
/// Extracts the filename and optional section from the line.
/// Handles various quote styles and whitespace.
pub fn parse_include_directive(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let upper = trimmed.to_uppercase();

    if !upper.starts_with(".INCLUDE") && !upper.starts_with(".INC") {
        return None;
    }

    let rest = upper
        .strip_prefix(".INCLUDE")
        .map(|_| trimmed[8..].trim())
        .or_else(|| upper.strip_prefix(".INC").map(|_| trimmed[4..].trim()))?;

    // Handle quoted paths
    if let Some(quoted) = rest.strip_prefix('"') {
        if let Some(end) = quoted.find('"') {
            return Some(quoted[..end].to_string());
        }
    } else if let Some(quoted) = rest.strip_prefix('\'')
        && let Some(end) = quoted.find('\'')
    {
        return Some(quoted[..end].to_string());
    }

    // Unquoted - take first word
    Some(rest.split_whitespace().next()?.to_string())
}

/// Parse a lib directive line
///
/// Returns (filename, optional_section)
pub fn parse_lib_directive(line: &str) -> Option<(String, Option<String>)> {
    let trimmed = line.trim();
    let upper = trimmed.to_uppercase();

    if !upper.starts_with(".LIB") || upper.starts_with(".LIBS") {
        return None;
    }

    let rest = &trimmed[4..].trim();
    let parts: Vec<&str> = rest.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    // Handle quoted filename
    let filename;
    let section_idx;

    if parts[0].starts_with('"') {
        // Find closing quote
        let combined = rest.to_string();
        if let Some(end) = combined[1..].find('"') {
            filename = combined[1..end + 1].to_string();
            // Section is after the quoted filename
            let after_quote = &combined[end + 2..].trim();
            if after_quote.is_empty() {
                return Some((filename, None));
            }
            return Some((
                filename,
                Some(after_quote.split_whitespace().next()?.to_string()),
            ));
        }
        return None;
    } else {
        filename = parts[0].to_string();
        section_idx = 1;
    }

    let section = if parts.len() > section_idx {
        Some(parts[section_idx].to_string())
    } else {
        None
    };

    Some((filename, section))
}

//=============================================================================
// Tests
//=============================================================================

