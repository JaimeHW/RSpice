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
use std::path::{Path, PathBuf};
use std::fs;

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
    /// Set of already-included files (canonical paths) to detect cycles
    included_files: HashSet<PathBuf>,
    /// Additional library search paths
    lib_paths: Vec<PathBuf>,
    /// Maximum include depth to prevent stack overflow
    max_depth: usize,
    /// Current include depth
    current_depth: usize,
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
            included_files: HashSet::new(),
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
        self.current_depth += 1;
        
        if self.current_depth > self.max_depth {
            self.current_depth -= 1;
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Include depth exceeded maximum of {}", self.max_depth),
            });
        }
        
        let path = self.resolve_path(filename)?;
        
        // Check for circular inclusion
        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        if self.included_files.contains(&canonical) {
            self.current_depth -= 1;
            // Already included - return empty string (not an error, just skip)
            log::debug!("Skipping already-included file: {:?}", path);
            return Ok(String::new());
        }
        
        self.included_files.insert(canonical);
        
        let content = fs::read_to_string(&path).map_err(|e| ParseError::Syntax {
            line: 0,
            message: format!("Failed to include '{}': {}", filename, e),
        })?;
        
        self.current_depth -= 1;
        Ok(content)
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
        section: Option<&str>
    ) -> Result<String, ParseError> {
        let content = self.process_include(filename)?;
        
        if content.is_empty() {
            return Ok(content);
        }
        
        match section {
            Some(sect) => self.extract_section(&content, sect),
            None => Ok(content),
        }
    }

    /// Resolve a filename to an absolute path
    fn resolve_path(&self, filename: &str) -> Result<PathBuf, ParseError> {
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
        let relative = self.base_dir.join(path);
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
        
        // Try common library locations (LTspice compatibility)
        let common_paths = [
            "lib",
            "models",
            "../lib",
            "../models",
        ];
        
        for common in common_paths {
            let candidate = self.base_dir.join(common).join(path);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
        
        Err(ParseError::Syntax {
            line: 0,
            message: format!("Include file not found: {} (searched {})", 
                clean_name, self.base_dir.display()),
        })
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
                    if parts.len() == 1 || 
                       (parts.len() >= 2 && parts[1].eq_ignore_ascii_case(section)) {
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
        self.included_files.clear();
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
    
    if !upper.starts_with(".INCLUDE") {
        return None;
    }
    
    // Skip the .INCLUDE keyword
    let rest = &trimmed[8..].trim();
    
    // Handle quoted paths
    if rest.starts_with('"') {
        if let Some(end) = rest[1..].find('"') {
            return Some(rest[1..end+1].to_string());
        }
    } else if rest.starts_with('\'') {
        if let Some(end) = rest[1..].find('\'') {
            return Some(rest[1..end+1].to_string());
        }
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
            filename = combined[1..end+1].to_string();
            // Section is after the quoted filename
            let after_quote = &combined[end+2..].trim();
            if after_quote.is_empty() {
                return Some((filename, None));
            }
            return Some((filename, Some(after_quote.split_whitespace().next()?.to_string())));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_processor_creation() {
        let proc = IncludeProcessor::new(Path::new("."));
        assert_eq!(proc.current_depth, 0);
        assert!(proc.included_files.is_empty());
    }

    #[test]
    fn test_parse_include_directive() {
        assert_eq!(
            parse_include_directive(".INCLUDE \"models.lib\""),
            Some("models.lib".to_string())
        );
        assert_eq!(
            parse_include_directive(".include 'test.mod'"),
            Some("test.mod".to_string())
        );
        assert_eq!(
            parse_include_directive(".INCLUDE standard.lib"),
            Some("standard.lib".to_string())
        );
        assert_eq!(
            parse_include_directive(".MODEL NPN NPN"),
            None
        );
    }

    #[test]
    fn test_parse_lib_directive() {
        assert_eq!(
            parse_lib_directive(".LIB standard.lib"),
            Some(("standard.lib".to_string(), None))
        );
        assert_eq!(
            parse_lib_directive(".LIB standard.lib POWER"),
            Some(("standard.lib".to_string(), Some("POWER".to_string())))
        );
        assert_eq!(
            parse_lib_directive(".LIB \"path with spaces.lib\" section"),
            Some(("path with spaces.lib".to_string(), Some("section".to_string())))
        );
    }

    #[test]
    fn test_missing_file_error() {
        let mut proc = IncludeProcessor::new(Path::new("."));
        let result = proc.process_include("nonexistent_file_12345.lib");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_section() {
        let proc = IncludeProcessor::new(Path::new("."));
        let content = r#"
* Library file
.LIB POWER
.MODEL PWR_NMOS NMOS(VTO=1.5)
.ENDL POWER

.LIB LOGIC
.MODEL LOG_NMOS NMOS(VTO=0.7)
.ENDL LOGIC
"#;
        
        let power = proc.extract_section(content, "POWER").unwrap();
        assert!(power.contains("PWR_NMOS"));
        assert!(!power.contains("LOG_NMOS"));
        
        let logic = proc.extract_section(content, "LOGIC").unwrap();
        assert!(logic.contains("LOG_NMOS"));
        assert!(!logic.contains("PWR_NMOS"));
    }

    #[test]
    fn test_extract_section_not_found() {
        let proc = IncludeProcessor::new(Path::new("."));
        let content = ".LIB POWER\n.MODEL M1 NMOS\n.ENDL POWER";
        
        let result = proc.extract_section(content, "NONEXISTENT");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_lib_path() {
        let mut proc = IncludeProcessor::new(Path::new("."));
        proc.add_lib_path(PathBuf::from("/path/to/lib"));
        proc.add_lib_path(PathBuf::from("/path/to/models"));
        
        assert_eq!(proc.lib_paths.len(), 2);
        
        // Adding duplicate should not increase count
        proc.add_lib_path(PathBuf::from("/path/to/lib"));
        assert_eq!(proc.lib_paths.len(), 2);
    }

    #[test]
    fn test_reset() {
        let mut proc = IncludeProcessor::new(Path::new("."));
        proc.current_depth = 5;
        proc.included_files.insert(PathBuf::from("/test/file"));
        
        proc.reset();
        
        assert_eq!(proc.current_depth, 0);
        assert!(proc.included_files.is_empty());
    }
}

