//! .INCLUDE and .LIB directive processing
//!
//! Handles file inclusion for SPICE netlists, supporting:
//! - `.INCLUDE "filename"` - Include entire file contents
//! - `.INC "filename"` / `.INCL "filename"` - HSPICE/Xyce include aliases
//! - `.LIB "filename" [section]` - Include library section
//!
//! Features:
//! - Relative path resolution from parent file
//! - Circular inclusion detection
//! - Library section extraction
//! - Case-insensitive matching for Windows compatibility

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::{ParseError, read_file_with_encoding};

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
    /// Execution directory used as Xyce's final relative include fallback
    execution_dir: PathBuf,
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

#[derive(Debug)]
struct InlineLibFrame {
    name: String,
    opened_at_line: usize,
    selected: bool,
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
        Self::new_with_execution_dir(base_path, None)
    }

    /// Create a new include processor with an explicit execution directory.
    ///
    /// Xyce resolves nested includes relative to the including file first, then
    /// the top-level netlist directory, then the process execution directory.
    /// Most callers use the top-level directory as the execution directory, but
    /// upstream wrapper tests can intentionally run a deck from another
    /// directory.
    pub fn new_with_execution_dir(base_path: &Path, execution_dir: Option<&Path>) -> Self {
        let base_dir = if base_path.is_file() {
            base_path.parent().unwrap_or(Path::new(".")).to_path_buf()
        } else {
            base_path.to_path_buf()
        };
        let execution_dir = execution_dir
            .map(Path::to_path_buf)
            .unwrap_or_else(|| base_dir.clone());

        Self {
            base_dir,
            execution_dir,
            active_includes: HashSet::new(),
            lib_paths: Vec::new(),
            max_depth: 64, // Foundry PDK trees nest .include/.lib deeply
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
        self.expand_content_from(content, current_dir, None)
    }

    /// Resolve a filename to an absolute path
    fn resolve_path_from(&self, base_dir: &Path, filename: &str) -> Result<PathBuf, ParseError> {
        // Remove quotes if present
        let clean_name = filename.trim_matches('"').trim_matches('\'');
        let path = Path::new(clean_name);

        if let Some(relative_to_execution_dir) = windows_drive_relative_suffix(clean_name) {
            let relative = spice_relative_path(relative_to_execution_dir);
            let candidate = self.base_dir.join(relative);
            if candidate.exists() {
                return Ok(candidate);
            }
            return Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "Include file not found: {} (searched {})",
                    clean_name,
                    self.base_dir.display()
                ),
            });
        }

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
        let relative_path = spice_relative_path(clean_name);
        let relative = base_dir.join(&relative_path);
        if relative.exists() {
            return Ok(relative);
        }

        // Xyce resolves nested include/lib paths relative to the including file
        // first, then falls back to the top-level netlist directory.
        let top_level_relative = self.base_dir.join(&relative_path);
        if top_level_relative.exists() {
            return Ok(top_level_relative);
        }

        let execution_relative = self.execution_dir.join(&relative_path);
        if execution_relative.exists() {
            return Ok(execution_relative);
        }

        // Try library search paths
        for lib_path in &self.lib_paths {
            let candidate = lib_path.join(&relative_path);
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        // Try common library locations
        let common_paths = ["lib", "models", "../lib", "../models"];

        for common in common_paths {
            let candidate = base_dir.join(common).join(&relative_path);
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
        self.process_include_from_with_selection(base_dir, filename, None)
    }

    fn process_include_from_with_selection(
        &mut self,
        base_dir: &Path,
        filename: &str,
        selected_section: Option<&str>,
    ) -> Result<String, ParseError> {
        let path = self.resolve_path_from(base_dir, filename)?;
        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        let key = IncludeKey::new(canonical, None);
        self.enter_include(&key)?;

        let result = (|| {
            let content = read_file_with_encoding(&path).map_err(|e| ParseError::Syntax {
                line: 0,
                message: format!("Failed to include '{}': {}", filename, e),
            })?;

            let content = strip_terminal_end_cards(&content);
            let include_dir = path.parent().unwrap_or(Path::new("."));
            self.expand_content_from(&content, include_dir, selected_section)
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
            let content = read_file_with_encoding(&path).map_err(|e| ParseError::Syntax {
                line: 0,
                message: format!("Failed to include '{}': {}", filename, e),
            })?;
            let content = strip_terminal_end_cards(&content);
            let library_dir = path.parent().unwrap_or(Path::new("."));
            self.expand_content_from(&content, library_dir, section)
        })();

        self.leave_include(&key);
        result
    }

    fn expand_content_from(
        &mut self,
        content: &str,
        base_dir: &Path,
        selected_section: Option<&str>,
    ) -> Result<String, ParseError> {
        let mut result = String::new();
        let mut inline_sections: Vec<InlineLibFrame> = Vec::new();

        for (line_index, line) in content.lines().enumerate() {
            let line_number = line_index + 1;
            let trimmed = line.trim();
            let upper = trimmed.to_ascii_uppercase();

            if upper.starts_with(".LIB")
                && !upper.starts_with(".LIBS")
                && let Some((filename, section)) = parse_lib_directive(trimmed)
            {
                if section.is_none() {
                    let parent_selected = inline_sections
                        .last()
                        .map(|frame| frame.selected)
                        .unwrap_or(true);
                    let selected = parent_selected
                        && selected_section
                            .is_some_and(|wanted| filename.eq_ignore_ascii_case(wanted));
                    inline_sections.push(InlineLibFrame {
                        name: filename,
                        opened_at_line: line_number,
                        selected,
                    });
                    continue;
                }
                if inline_sections.last().is_some_and(|frame| !frame.selected) {
                    continue;
                }

                let included = self.process_lib_from(base_dir, &filename, section.as_deref())?;
                result.push_str(&included);
                if !included.ends_with('\n') {
                    result.push('\n');
                }
                continue;
            }

            if let Some(end_name) = parse_endl_directive(trimmed, line_number)? {
                let Some(open_frame) = inline_sections.last() else {
                    return Err(ParseError::Syntax {
                        line: line_number,
                        message: ".ENDL encountered without an open .LIB section".to_string(),
                    });
                };
                if let Some(end_name) = end_name
                    && !end_name.eq_ignore_ascii_case(&open_frame.name)
                {
                    return Err(ParseError::Syntax {
                        line: line_number,
                        message: format!(
                            ".ENDL section '{end_name}' does not match open .LIB section '{}'",
                            open_frame.name
                        ),
                    });
                }
                inline_sections.pop();
                continue;
            }

            if inline_sections.last().is_some_and(|frame| !frame.selected) {
                continue;
            }

            if let Some(filename) = parse_include_directive(trimmed) {
                // SPEF files are parasitic data, not SPICE text: route to
                // the back-annotation pass (`.spef_include`) with the path
                // resolved here, where include search rules apply.
                if filename.to_ascii_lowercase().ends_with(".spef") {
                    let path = self.resolve_path_from(base_dir, &filename)?;
                    let normalized = path.display().to_string().replace('\\', "/");
                    result.push_str(&format!(".spef_include \"{normalized}\"\n"));
                    continue;
                }
                let included = self.process_include_from_with_selection(
                    base_dir,
                    &filename,
                    selected_section,
                )?;
                result.push_str(&included);
                if !included.ends_with('\n') {
                    result.push('\n');
                }
                continue;
            }

            result.push_str(line);
            result.push('\n');
        }

        if let Some(frame) = inline_sections.last() {
            return Err(ParseError::Syntax {
                line: frame.opened_at_line,
                message: format!("Library section '{}' missing .ENDL", frame.name),
            });
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
    #[cfg(test)]
    fn extract_section(&self, content: &str, section: &str) -> Result<String, ParseError> {
        let mut in_section = false;
        let mut section_content = Vec::new();
        let mut found = false;
        // Section *definitions* nested inside the requested one (a 2-token
        // `.LIB name` line, as opposed to the 3-token `.LIB file section`
        // call form) open their own `.ENDL` scope; counting them keeps an
        // inner `.ENDL` from terminating the outer section early.
        let mut nested_definitions = 0usize;

        for line in content.lines() {
            let trimmed = line.trim();
            let upper = trimmed.to_uppercase();

            if upper.starts_with(".LIB") && !upper.starts_with(".LIBS") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if !in_section {
                    // Check if this is our section start
                    if parts.len() >= 2 && parts[1].eq_ignore_ascii_case(section) {
                        in_section = true;
                        found = true;
                        continue;
                    }
                } else if parts.len() == 2 {
                    nested_definitions += 1;
                }
            }

            if in_section {
                if upper.starts_with(".ENDL") {
                    if nested_definitions > 0 {
                        nested_definitions -= 1;
                        section_content.push(line);
                        continue;
                    }
                    // Check if this ends our section
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() == 1
                        || (parts.len() >= 2 && parts[1].eq_ignore_ascii_case(section))
                    {
                        in_section = false;
                        break;
                    }
                    log::warn!(
                        ".ENDL '{}' does not match the open library section '{}'",
                        parts.get(1).copied().unwrap_or(""),
                        section
                    );
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
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Library section '{}' missing .ENDL", section),
            });
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

/// Parse an include directive line
///
/// Extracts the filename from `.include`, `.inc`, and `.incl` directives.
/// Handles quote styles and whitespace without accepting longer lookalike
/// directive names.
pub fn parse_include_directive(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let (directive, rest) = split_directive(trimmed)?;
    if !matches_ignore_ascii_case(directive, &[".include", ".inc", ".incl"]) {
        return None;
    }
    let rest = rest.trim();

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

fn split_directive(line: &str) -> Option<(&str, &str)> {
    let mut end = line.len();
    for (index, ch) in line.char_indices() {
        if ch.is_whitespace() {
            end = index;
            break;
        }
    }
    let directive = &line[..end];
    if directive.is_empty() {
        return None;
    }
    Some((directive, &line[end..]))
}

fn matches_ignore_ascii_case(value: &str, accepted: &[&str]) -> bool {
    accepted
        .iter()
        .any(|accepted| value.eq_ignore_ascii_case(accepted))
}

fn windows_drive_relative_suffix(path: &str) -> Option<&str> {
    let bytes = path.as_bytes();
    if bytes.len() < 3 || bytes[1] != b':' || !bytes[0].is_ascii_alphabetic() {
        return None;
    }
    if matches!(bytes[2], b'/' | b'\\') {
        return None;
    }
    Some(&path[2..])
}

fn spice_relative_path(path: &str) -> PathBuf {
    path.split(['/', '\\'])
        .filter(|component| !component.is_empty() && *component != ".")
        .fold(PathBuf::new(), |mut out, component| {
            out.push(component);
            out
        })
}

/// Parse a lib directive line
///
/// Returns (filename, optional_section)
pub fn parse_lib_directive(line: &str) -> Option<(String, Option<String>)> {
    let trimmed = line.trim();
    let (directive, rest) = split_directive(trimmed)?;
    if !directive.eq_ignore_ascii_case(".lib") {
        return None;
    }
    let rest = rest.trim();
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

fn parse_endl_directive(
    line: &str,
    line_number: usize,
) -> Result<Option<Option<String>>, ParseError> {
    let Some((directive, rest)) = split_directive(line.trim()) else {
        return Ok(None);
    };
    if !directive.eq_ignore_ascii_case(".endl") {
        return Ok(None);
    }

    let fields = rest.split_whitespace().collect::<Vec<_>>();
    if fields.len() > 1 {
        log::warn!(
            ".ENDL at line {line_number} has extraneous fields after the section name; ignoring them"
        );
    }
    Ok(Some(fields.first().map(|name| (*name).to_string())))
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include_directive_parser_accepts_xyce_aliases_exactly() {
        assert_eq!(
            parse_include_directive(".include \"model cards/mod.inc\"").as_deref(),
            Some("model cards/mod.inc")
        );
        assert_eq!(
            parse_include_directive(".INC incFile1").as_deref(),
            Some("incFile1")
        );
        assert_eq!(
            parse_include_directive(".incl 'sub1/include1'").as_deref(),
            Some("sub1/include1")
        );
        assert_eq!(parse_include_directive(".includex bad"), None);
        assert_eq!(parse_include_directive(".incbin bad"), None);
    }

    #[test]
    fn include_processor_expands_xyce_include_aliases() {
        let dir = unique_include_temp_dir("aliases");
        std::fs::create_dir_all(&dir).expect("create include alias fixture");
        let deck_path = dir.join("deck.cir");
        std::fs::write(dir.join("incFile1"), "R1 1 2 1\n").expect("write incFile1");
        std::fs::write(dir.join("incFile2"), "R2 2 3 1\n").expect("write incFile2");
        std::fs::write(dir.join("incFile3"), "R3 3 0 1\n").expect("write incFile3");

        let deck = ".INC incFile1\n.INCL incFile2\n.INCLUDE incFile3\n";
        std::fs::write(&deck_path, deck).expect("write deck");
        let mut processor = IncludeProcessor::new(&deck_path);
        let expanded = processor
            .expand_content(deck, &deck_path)
            .expect("include aliases expand");

        assert!(expanded.contains("R1 1 2 1"), "{expanded}");
        assert!(expanded.contains("R2 2 3 1"), "{expanded}");
        assert!(expanded.contains("R3 3 0 1"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn include_processor_uses_xyce_top_level_fallback_after_local_path() {
        let dir = unique_include_temp_dir("fallback");
        let sub1 = dir.join("sub1");
        let sub2 = sub1.join("sub2");
        std::fs::create_dir_all(&sub2).expect("create nested include fixture");
        let deck_path = dir.join("deck.cir");
        std::fs::write(
            sub1.join("include1"),
            ".INC sub2/local\n.INC sub1/sub2/top\n.INC precedence/wins_local\n",
        )
        .expect("write include1");
        std::fs::write(sub2.join("local"), "RLOCAL 1 0 3\n").expect("write local include");
        std::fs::write(sub2.join("top"), "RTOP 1 0 4\n").expect("write fallback include");
        std::fs::create_dir_all(sub1.join("precedence")).expect("create local precedence dir");
        std::fs::create_dir_all(dir.join("precedence")).expect("create top precedence dir");
        std::fs::write(sub1.join("precedence").join("wins_local"), "RWIN 1 0 5\n")
            .expect("write local precedence include");
        std::fs::write(dir.join("precedence").join("wins_local"), "RLOSE 1 0 6\n")
            .expect("write top precedence include");

        let deck = ".INC sub1/include1\n";
        std::fs::write(&deck_path, deck).expect("write deck");
        let mut processor = IncludeProcessor::new(&deck_path);
        let expanded = processor
            .expand_content(deck, &deck_path)
            .expect("nested include fallback expands");

        assert!(expanded.contains("RLOCAL 1 0 3"), "{expanded}");
        assert!(expanded.contains("RTOP 1 0 4"), "{expanded}");
        assert!(expanded.contains("RWIN 1 0 5"), "{expanded}");
        assert!(!expanded.contains("RLOSE"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn include_processor_uses_xyce_execution_dir_fallback_after_top_level_path() {
        let exec_dir = unique_include_temp_dir("execution-fallback");
        let top_dir = exec_dir.join("top");
        let include_dir = top_dir.join("sub1");
        std::fs::create_dir_all(&include_dir).expect("create nested include fixture");
        let deck_path = top_dir.join("deck.cir");
        std::fs::write(
            include_dir.join("include1"),
            ".INC local\n.INC top\n.INC execution\n.INC precedence/wins_top\n",
        )
        .expect("write include1");
        std::fs::write(include_dir.join("local"), "RLOCAL 1 0 3\n").expect("write local include");
        std::fs::write(top_dir.join("top"), "RTOP 1 0 4\n").expect("write top include");
        std::fs::write(exec_dir.join("execution"), "REXEC 1 0 5\n")
            .expect("write execution include");
        std::fs::create_dir_all(top_dir.join("precedence")).expect("create top precedence dir");
        std::fs::create_dir_all(exec_dir.join("precedence"))
            .expect("create execution precedence dir");
        std::fs::write(top_dir.join("precedence").join("wins_top"), "RWIN 1 0 6\n")
            .expect("write top precedence include");
        std::fs::write(
            exec_dir.join("precedence").join("wins_top"),
            "RLOSE 1 0 7\n",
        )
        .expect("write execution precedence include");

        let deck = ".INC sub1/include1\n";
        std::fs::write(&deck_path, deck).expect("write deck");
        let mut processor = IncludeProcessor::new_with_execution_dir(&deck_path, Some(&exec_dir));
        let expanded = processor
            .expand_content(deck, &deck_path)
            .expect("execution fallback expands");

        assert!(expanded.contains("RLOCAL 1 0 3"), "{expanded}");
        assert!(expanded.contains("RTOP 1 0 4"), "{expanded}");
        assert!(expanded.contains("REXEC 1 0 5"), "{expanded}");
        assert!(expanded.contains("RWIN 1 0 6"), "{expanded}");
        assert!(!expanded.contains("RLOSE"), "{expanded}");
        let _ = std::fs::remove_dir_all(exec_dir);
    }

    #[test]
    fn include_processor_resolves_xyce_drive_relative_paths_from_top_level() {
        let dir = unique_include_temp_dir("drive-relative");
        let sub1 = dir.join("sub1");
        std::fs::create_dir_all(&sub1).expect("create drive-relative fixture");
        let deck_path = dir.join("deck.cir");
        std::fs::write(sub1.join("include1"), ".INC C:drive_file\n")
            .expect("write drive-relative include");
        std::fs::write(dir.join("drive_file"), "RDRIVE 1 0 7\n").expect("write drive file");

        let deck = ".INC sub1\\include1\n";
        std::fs::write(&deck_path, deck).expect("write deck");
        let mut processor = IncludeProcessor::new(&deck_path);
        let expanded = processor
            .expand_content(deck, &deck_path)
            .expect("drive-relative include expands");

        assert!(expanded.contains("RDRIVE 1 0 7"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn inline_library_definitions_are_omitted_with_nested_scope_tracking() {
        let deck = "\
.lib Unused
.invalid line hidden from the parser
.include missing-file.inc
.lib missing-library.lib nominal
.lib nested
Rhidden 1 0 1
.endl NESTED
.endl unused
Rkept 1 0 2
";
        let deck_path = Path::new("deck.cir");
        let expanded = IncludeProcessor::new(deck_path)
            .expand_content(deck, deck_path)
            .expect("inline library definitions preprocess");

        assert_eq!(expanded, "Rkept 1 0 2\n");
    }

    #[test]
    fn external_library_section_still_expands_after_inline_definition() {
        let dir = unique_include_temp_dir("inline-and-external-lib");
        std::fs::create_dir_all(&dir).expect("create library fixture");
        let deck_path = dir.join("deck.cir");
        std::fs::write(
            dir.join("models.lib"),
            ".lib nominal\n.param selected=7\n.include child.lib\n.endl NOMINAL\n",
        )
        .expect("write library fixture");
        std::fs::write(
            dir.join("child.lib"),
            ".lib low\n.param inherited=1\n.endl low\n.lib nominal\n.param inherited=9\n.endl nominal\n",
        )
        .expect("write nested library fixture");
        let deck = "\
.lib ignored
.invalid hidden
.endl ignored
.lib models.lib nominal
R1 1 0 {selected}
";
        let expanded = IncludeProcessor::new(&deck_path)
            .expand_content(deck, &deck_path)
            .expect("external library section expands");

        assert!(expanded.contains(".param selected=7"), "{expanded}");
        assert!(expanded.contains(".param inherited=9"), "{expanded}");
        assert!(!expanded.contains(".param inherited=1"), "{expanded}");
        assert!(expanded.contains("R1 1 0 {selected}"), "{expanded}");
        assert!(!expanded.contains("hidden"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn inline_library_scope_errors_are_line_aware() {
        for (deck, expected_line, expected_message) in [
            (
                ".endl orphan\n",
                1,
                ".ENDL encountered without an open .LIB section",
            ),
            (
                ".lib first\n.endl second\n",
                2,
                ".ENDL section 'second' does not match open .LIB section 'first'",
            ),
            (
                ".lib unfinished\nR1 1 0 1\n",
                1,
                "Library section 'unfinished' missing .ENDL",
            ),
        ] {
            let err = IncludeProcessor::new(Path::new("deck.cir"))
                .expand_content(deck, Path::new("deck.cir"))
                .expect_err("malformed inline library scope must reject");
            match err {
                ParseError::Syntax { line, message } => {
                    assert_eq!(line, expected_line, "{message}");
                    assert!(message.contains(expected_message), "{message}");
                }
                other => panic!("expected syntax error, got {other:?}"),
            }
        }
    }

    #[test]
    fn lib_parser_rejects_longer_directive_lookalikes() {
        assert_eq!(parse_lib_directive(".libs foo"), None);
        assert_eq!(parse_lib_directive(".library foo"), None);
        assert_eq!(
            parse_lib_directive(".LIB \"model cards.lib\" nominal"),
            Some(("model cards.lib".to_string(), Some("nominal".to_string())))
        );
    }

    fn unique_include_temp_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rspice-include-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time after epoch")
                .as_nanos()
        ))
    }

    fn extract(content: &str, section: &str) -> String {
        IncludeProcessor::new(std::path::Path::new("."))
            .extract_section(content, section)
            .expect("section extracts")
    }

    #[test]
    fn nested_section_definition_does_not_terminate_outer() {
        let lib = "\
.lib outer
r1 a b 1k
.lib inner
r2 b c 2k
.endl
r3 c d 3k
.endl outer
";
        let body = extract(lib, "outer");
        assert!(body.contains("r1"), "outer head kept: {body}");
        assert!(
            body.contains(".lib inner") && body.contains("r2"),
            "nested definition preserved intact: {body}"
        );
        assert!(
            body.contains("r3"),
            "content after the nested definition still belongs to outer: {body}"
        );
    }

    #[test]
    fn bare_endl_still_terminates_unnested_section() {
        let lib = "\
.lib tt
r1 a b 1k
.endl
.lib ss
r2 a b 9k
.endl
";
        let tt = extract(lib, "tt");
        assert!(tt.contains("r1") && !tt.contains("r2"), "{tt}");
        let ss = extract(lib, "ss");
        assert!(ss.contains("r2") && !ss.contains("r1"), "{ss}");
    }

    #[test]
    fn mismatched_endl_name_does_not_end_the_section() {
        let lib = "\
.lib tt
r1 a b 1k
.endl ff
r2 a b 2k
.endl tt
";
        let tt = extract(lib, "tt");
        assert!(
            tt.contains("r1") && tt.contains("r2"),
            "mismatched .endl is content, not a terminator: {tt}"
        );
    }

    #[test]
    fn unterminated_selected_library_section_is_rejected() {
        let lib = "\
.lib tt
r1 a b 1k
.lib ss
r2 a b 2k
.endl ss
";
        let err = IncludeProcessor::default()
            .extract_section(lib, "tt")
            .expect_err("unterminated selected library section must reject");

        match err {
            ParseError::Syntax { message, .. } => {
                assert!(
                    message.contains("Library section 'tt' missing .ENDL"),
                    "unexpected error: {message}"
                );
            }
            other => panic!("expected syntax error, got {other:?}"),
        }
    }
}
