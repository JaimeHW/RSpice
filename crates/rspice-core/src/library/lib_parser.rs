//! SPICE Library File Parser
//!
//! Advanced parser for foundry .lib files with support for:
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

use std::collections::{HashMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::manager::{ModelDefinition, ModelType};

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
    /// Exact normalized SPICE model type token from the source card.
    pub spice_type: String,
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

    //=========================================================================
    // Model Binning Parameters (for PDK geometry-based selection)
    //=========================================================================
    /// Minimum channel length for this bin (meters)
    pub lmin: Option<f64>,
    /// Maximum channel length for this bin (meters)
    pub lmax: Option<f64>,
    /// Minimum channel width for this bin (meters)
    pub wmin: Option<f64>,
    /// Maximum channel width for this bin (meters)
    pub wmax: Option<f64>,
    /// Bin name prefix (e.g., "nch" for "nch.1", "nch.2", etc.)
    pub bin_prefix: Option<String>,
}

impl ParsedModel {
    /// Create a new parsed model
    pub fn new(name: impl Into<String>, model_type: ModelType) -> Self {
        Self {
            name: name.into(),
            model_type,
            spice_type: match model_type {
                ModelType::Diode => "D",
                ModelType::NpnBjt => "NPN",
                ModelType::PnpBjt => "PNP",
                ModelType::Nmos => "NMOS",
                ModelType::Pmos => "PMOS",
                ModelType::Njfet => "NJF",
                ModelType::Pjfet => "PJF",
                ModelType::Resistor => "R",
                ModelType::Capacitor => "C",
                ModelType::Other => "OTHER",
            }
            .to_owned(),
            level: None,
            version: None,
            parameters: HashMap::new(),
            string_params: HashMap::new(),
            description: None,
            source_file: None,
            source_line: None,
            lmin: None,
            lmax: None,
            wmin: None,
            wmax: None,
            bin_prefix: None,
        }
    }

    /// Check if this model matches the given geometry (W, L)
    /// Returns true if the geometry falls within this model's bin range
    pub fn matches_geometry(&self, width: f64, length: f64) -> bool {
        // Check length bounds
        if let Some(lmin) = self.lmin
            && length < lmin
        {
            return false;
        }
        if let Some(lmax) = self.lmax
            && length > lmax
        {
            return false;
        }

        // Check width bounds
        if let Some(wmin) = self.wmin
            && width < wmin
        {
            return false;
        }
        if let Some(wmax) = self.wmax
            && width > wmax
        {
            return false;
        }

        true
    }

    /// Check if this model has any binning constraints
    pub fn has_binning(&self) -> bool {
        self.lmin.is_some() || self.lmax.is_some() || self.wmin.is_some() || self.wmax.is_some()
    }

    /// Extract bin prefix from model name (e.g., "nch.1" -> "nch")
    pub fn extract_bin_prefix(&self) -> Option<String> {
        // Common PDK naming: prefix.binnum (e.g., nch.1, nch_hvt.2)
        if let Some(dot_pos) = self.name.rfind('.') {
            let suffix = &self.name[dot_pos + 1..];
            // Check if suffix is numeric (bin number)
            if suffix.chars().all(|c| c.is_ascii_digit()) {
                return Some(self.name[..dot_pos].to_string());
            }
        }
        None
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
    pub fn to_model_definition(&self, library: impl Into<Arc<str>>) -> ModelDefinition {
        let mut def = ModelDefinition::new(self.name.clone(), self.model_type, library);
        if let Some(ref desc) = self.description {
            def = def.with_description(desc.clone());
        }
        // Transfer binning parameters
        def = def.with_binning(self.lmin, self.lmax, self.wmin, self.wmax);
        if let Some(ref prefix) = self.bin_prefix {
            def = def.with_bin_prefix(prefix.clone());
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
    /// Exact source tokens for every parameter default, including expressions
    /// and quoted strings that cannot be reduced to a numeric constant.
    pub parameter_defaults: HashMap<String, String>,
    /// Full subcircuit content (for expansion)
    pub content: String,
    /// Description from comments
    pub description: Option<String>,
    /// Source file path
    pub source_file: Option<PathBuf>,
    /// One-based physical line containing the `.SUBCKT` declaration.
    pub source_line: Option<usize>,
}

impl ParsedSubcircuit {
    /// Create a new parsed subcircuit
    pub fn new(name: impl Into<String>, pins: Vec<String>) -> Self {
        Self {
            name: name.into(),
            pins,
            parameters: HashMap::new(),
            parameter_defaults: HashMap::new(),
            content: String::new(),
            description: None,
            source_file: None,
            source_line: None,
        }
    }
}

//=============================================================================
// Library Parser
//=============================================================================

/// Parser for .lib format files
pub struct LibParser {
    /// Base directory for resolving .include paths
    base_dir: PathBuf,
    /// Top-level library directory used after the including file directory,
    /// matching the executable include processor's fallback order.
    root_dir: PathBuf,
    /// Current file being parsed
    current_file: Option<PathBuf>,
    /// Include depth (to prevent infinite recursion)
    include_depth: usize,
    /// Maximum include depth
    max_include_depth: usize,
    /// Canonical source stack used for deterministic cycle detection.
    include_stack: Vec<PathBuf>,
    /// Exact UTF-8 contents read during this parse, keyed by canonical path.
    resolved_sources: Vec<ResolvedLibSource>,
    resolved_source_paths: HashSet<PathBuf>,
    /// Canonical include-resolution decisions captured while filesystem
    /// semantics (symlinks, case rules, and search precedence) are available.
    resolved_dependencies: Vec<ResolvedLibDependency>,
    resolved_dependency_targets: HashMap<(PathBuf, String), PathBuf>,
    /// Collected sections
    sections: Vec<LibSection>,
    /// Models outside any section (top-level)
    top_level_models: Vec<ParsedModel>,
    /// Subcircuits outside any section
    top_level_subcircuits: Vec<ParsedSubcircuit>,
    /// Parse errors encountered
    errors: Vec<ParseError>,
    /// Optional authenticated source resolver. When populated, include
    /// traversal is satisfied exclusively from these retained bytes and exact
    /// owner/literal edges; the host filesystem is never consulted.
    authenticated_sources: Option<HashMap<PathBuf, Arc<[u8]>>>,
    authenticated_dependencies: Option<HashMap<(PathBuf, String), PathBuf>>,
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
        let base_dir = base_dir.into();
        Self {
            root_dir: base_dir.clone(),
            base_dir,
            current_file: None,
            include_depth: 0,
            max_include_depth: crate::resource::DEFAULT_MAX_INCLUDE_DEPTH,
            include_stack: Vec::new(),
            resolved_sources: Vec::new(),
            resolved_source_paths: HashSet::new(),
            resolved_dependencies: Vec::new(),
            resolved_dependency_targets: HashMap::new(),
            sections: Vec::new(),
            top_level_models: Vec::new(),
            top_level_subcircuits: Vec::new(),
            errors: Vec::new(),
            authenticated_sources: None,
            authenticated_dependencies: None,
        }
    }

    /// Parse a .lib file and return the result
    pub fn parse_file(&mut self, path: impl AsRef<Path>) -> io::Result<LibParseResult> {
        self.reset_parse_state();
        let path = std::fs::canonicalize(path.as_ref())?;
        let bytes = std::fs::read(&path)?;
        let content = crate::netlist::decode_source_bytes(&bytes)?;

        self.current_file = Some(path.clone());
        self.base_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        self.root_dir = self.base_dir.clone();
        // The root library occupies the first include frame when it is
        // materialized from a generated deck. Starting at one keeps closure
        // discovery's exact boundary aligned with IncludeProcessor.
        self.include_depth = 1;
        self.include_stack.push(path.clone());
        self.record_resolved_source(path, &bytes, &content);

        self.parse_content(&content);
        self.include_stack.pop();

        Ok(self.build_result())
    }

    /// Parse library content from a string
    pub fn parse_string(&mut self, content: &str) -> LibParseResult {
        self.reset_parse_state();
        self.parse_content(content);
        self.build_result()
    }

    /// Parse a complete authenticated source closure without consulting the
    /// host filesystem. Every dependency must resolve through the exact
    /// retained `(owner, normalized literal) -> target` edge supplied by the
    /// caller. The result preserves each parsed model's member path and source
    /// line just like [`Self::parse_file`].
    pub fn parse_authenticated_closure(
        &mut self,
        root: impl Into<PathBuf>,
        sources: impl IntoIterator<Item = (PathBuf, Vec<u8>)>,
        dependencies: impl IntoIterator<Item = ResolvedLibDependency>,
    ) -> Result<LibParseResult, String> {
        self.reset_parse_state();
        let root = root.into();
        let mut authenticated_sources = HashMap::new();
        for (path, bytes) in sources {
            if authenticated_sources
                .insert(path.clone(), Arc::<[u8]>::from(bytes))
                .is_some()
            {
                return Err(format!(
                    "authenticated library closure repeats source '{}'",
                    path.display()
                ));
            }
        }
        let root_bytes = authenticated_sources.get(&root).cloned().ok_or_else(|| {
            format!(
                "authenticated library closure does not contain root '{}'",
                root.display()
            )
        })?;
        let mut authenticated_dependencies = HashMap::new();
        for dependency in dependencies {
            if !authenticated_sources.contains_key(&dependency.owner)
                || !authenticated_sources.contains_key(&dependency.target)
            {
                return Err(format!(
                    "authenticated dependency '{}' -> '{}' references a source outside the retained closure",
                    dependency.owner.display(),
                    dependency.target.display()
                ));
            }
            let requested_path =
                crate::netlist::normalize_source_path_literal(&dependency.requested_path)
                    .map_err(|error| error.to_string())?;
            let key = (dependency.owner.clone(), requested_path.clone());
            if let Some(existing) =
                authenticated_dependencies.insert(key, dependency.target.clone())
                && existing != dependency.target
            {
                return Err(format!(
                    "authenticated dependency '{}' in '{}' has conflicting targets '{}' and '{}'",
                    requested_path,
                    dependency.owner.display(),
                    existing.display(),
                    dependency.target.display()
                ));
            }
        }
        let content = crate::netlist::decode_source_bytes(&root_bytes)
            .map_err(|error| format!("authenticated root cannot be decoded: {error}"))?;
        self.authenticated_sources = Some(authenticated_sources);
        self.authenticated_dependencies = Some(authenticated_dependencies);
        self.current_file = Some(root.clone());
        self.base_dir = root.parent().unwrap_or(Path::new(".")).to_path_buf();
        self.root_dir = self.base_dir.clone();
        self.include_depth = 1;
        self.include_stack.push(root.clone());
        self.record_resolved_source(root, &root_bytes, &content);
        self.parse_content(&content);
        self.include_stack.pop();
        Ok(self.build_result())
    }

    fn reset_parse_state(&mut self) {
        self.current_file = None;
        self.include_depth = 0;
        self.include_stack.clear();
        self.resolved_sources.clear();
        self.resolved_source_paths.clear();
        self.resolved_dependencies.clear();
        self.resolved_dependency_targets.clear();
        self.sections.clear();
        self.top_level_models.clear();
        self.top_level_subcircuits.clear();
        self.errors.clear();
        self.authenticated_sources = None;
        self.authenticated_dependencies = None;
    }

    fn record_resolved_source(&mut self, path: PathBuf, bytes: &[u8], content: &str) {
        if self.resolved_source_paths.insert(path.clone()) {
            self.resolved_sources.push(ResolvedLibSource {
                path,
                bytes: Arc::from(bytes),
                content: Arc::from(content),
            });
        }
    }

    fn record_resolved_dependency(
        &mut self,
        owner: PathBuf,
        requested_path: &str,
        target: PathBuf,
    ) -> Result<(), String> {
        let requested_path = crate::netlist::normalize_source_path_literal(requested_path)
            .map_err(|error| error.to_string())?;
        let key = (owner.clone(), requested_path.clone());
        if let Some(existing) = self.resolved_dependency_targets.get(&key) {
            if existing != &target {
                return Err(format!(
                    "Conflicting dependency resolution for normalized path '{}' in '{}': '{}' and '{}'",
                    requested_path,
                    owner.display(),
                    existing.display(),
                    target.display()
                ));
            }
            return Ok(());
        }

        self.resolved_dependency_targets.insert(key, target.clone());
        self.resolved_dependencies.push(ResolvedLibDependency {
            owner,
            requested_path,
            target,
        });
        Ok(())
    }

    /// Internal parsing implementation
    fn parse_content(&mut self, content: &str) {
        let lines = self.preprocess_lines(content);
        let mut current_section: Option<LibSection> = None;
        let mut current_section_start_line: Option<usize> = None;
        let mut subckt_content: Option<(ParsedSubcircuit, Vec<String>, usize)> = None;
        let mut last_comment = String::new();

        for (line_number, line) in &lines {
            let line_number = *line_number;
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

            // A two-argument `.lib <file> <section>` is an external library
            // dependency; a one-argument `.lib <section>` opens an inline
            // section. Use the netlist parser's established tokenization so
            // quoted paths and directive boundaries stay consistent.
            if let Some((name_or_path, external_section)) =
                crate::netlist::parse_lib_directive(line)
            {
                if external_section.is_some() {
                    self.process_include(&name_or_path, line_number);
                    last_comment.clear();
                    continue;
                }
                if let Some(section) = current_section.take() {
                    self.errors.push(ParseError {
                        message: format!(
                            ".lib {} at line {} is not closed by .endl before a new .lib section",
                            section.name,
                            current_section_start_line.unwrap_or(line_number)
                        ),
                        file: self.current_file.clone(),
                        line: current_section_start_line,
                    });
                    self.sections.push(section);
                }
                current_section = Some(LibSection::new(name_or_path));
                current_section_start_line = Some(line_number);
                last_comment.clear();
                continue;
            }

            // Handle .endl section end
            if upper.starts_with(".ENDL") {
                if let Some(section) = current_section.take() {
                    self.sections.push(section);
                    current_section_start_line = None;
                }
                continue;
            }

            // Handle .include
            if let Some(include_path) = crate::netlist::parse_include_directive(line) {
                self.process_include(&include_path, line_number);
                continue;
            }

            // Handle .subckt start
            if Self::directive_rest(line, ".subckt").is_some() {
                match self.parse_subckt_start(line, line_number) {
                    Ok(mut subckt) => {
                        subckt.description = if last_comment.is_empty() {
                            None
                        } else {
                            Some(last_comment.clone())
                        };
                        subckt.source_file = self.current_file.clone();
                        subckt_content = Some((subckt, vec![line.to_string()], line_number));
                    }
                    Err(message) => self.errors.push(ParseError {
                        message,
                        file: self.current_file.clone(),
                        line: Some(line_number),
                    }),
                }
                last_comment.clear();
                continue;
            }

            // Handle .ends
            if upper.starts_with(".ENDS") {
                if let Some((mut subckt, mut content_lines, _)) = subckt_content.take() {
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
            if let Some((_, ref mut content_lines, _)) = subckt_content {
                content_lines.push(line.to_string());
                continue;
            }

            // Handle .model. Match the complete directive token so a vendor
            // extension such as `.modelcheck` is not mistaken for a model.
            if Self::directive_rest(line, ".model").is_some() {
                match self.parse_model_line(line) {
                    Ok(mut model) => {
                        model.description = if last_comment.is_empty() {
                            None
                        } else {
                            Some(last_comment.clone())
                        };
                        model.source_file = self.current_file.clone();
                        model.source_line = Some(line_number);

                        let duplicate_line = if let Some(section) = current_section.as_ref() {
                            section
                                .models
                                .iter()
                                .find(|existing| existing.name.eq_ignore_ascii_case(&model.name))
                                .and_then(|existing| existing.source_line)
                        } else {
                            self.top_level_models
                                .iter()
                                .find(|existing| existing.name.eq_ignore_ascii_case(&model.name))
                                .and_then(|existing| existing.source_line)
                        };
                        if let Some(original_line) = duplicate_line {
                            self.errors.push(ParseError {
                                message: format!(
                                    "Duplicate .model name '{}' in the same scope (first declared at line {original_line})",
                                    model.name
                                ),
                                file: self.current_file.clone(),
                                line: Some(line_number),
                            });
                        } else if let Some(ref mut section) = current_section {
                            section.models.push(model);
                        } else {
                            self.top_level_models.push(model);
                        }
                    }
                    Err(message) => self.errors.push(ParseError {
                        message,
                        file: self.current_file.clone(),
                        line: Some(line_number),
                    }),
                }
                last_comment.clear();
            }
        }

        if let Some((subckt, _, start_line)) = subckt_content {
            self.errors.push(ParseError {
                message: format!(
                    ".subckt {} at line {} is not closed by .ends",
                    subckt.name, start_line
                ),
                file: self.current_file.clone(),
                line: Some(start_line),
            });
        }

        if let Some(section) = current_section {
            let start_line = current_section_start_line;
            self.errors.push(ParseError {
                message: format!(
                    ".lib {} at line {} is not closed by .endl",
                    section.name,
                    start_line.unwrap_or(0)
                ),
                file: self.current_file.clone(),
                line: start_line,
            });
        }
    }

    /// Preprocess content: handle line continuations
    fn preprocess_lines(&self, content: &str) -> Vec<(usize, String)> {
        let mut result = Vec::new();
        let mut current_line = String::new();
        let mut current_line_number = 0;

        for (line_index, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Line continuation with +
            if trimmed.starts_with('+') {
                current_line.push(' ');
                current_line.push_str(trimmed.trim_start_matches('+').trim());
            } else {
                if !current_line.is_empty() {
                    result.push((current_line_number, std::mem::take(&mut current_line)));
                }
                current_line_number = line_index + 1;
                current_line = trimmed.to_string();
            }
        }

        if !current_line.is_empty() {
            result.push((current_line_number, current_line));
        }

        result
    }

    /// Process an include directive
    fn process_include(&mut self, include_literal: &str, directive_line: usize) {
        if self.authenticated_sources.is_some() {
            self.process_authenticated_include(include_literal, directive_line);
            return;
        }
        if self.include_depth >= self.max_include_depth {
            self.errors.push(ParseError {
                message: format!(
                    "Maximum include depth ({}) exceeded",
                    self.max_include_depth
                ),
                file: self.current_file.clone(),
                line: Some(directive_line),
            });
            return;
        }

        let include_path = crate::netlist::source_path_literal_to_host_path(include_literal);
        let candidates = if include_path.is_absolute() {
            vec![include_path.clone()]
        } else {
            let mut candidates = vec![
                self.base_dir.join(&include_path),
                self.root_dir.join(&include_path),
            ];
            for common in ["lib", "models", "../lib", "../models"] {
                candidates.push(self.base_dir.join(common).join(&include_path));
            }
            candidates
        };
        let mut last_error = None;
        let mut attempted_path = None;
        let canonical_path = candidates.into_iter().find_map(|candidate| {
            attempted_path.get_or_insert_with(|| candidate.clone());
            match std::fs::canonicalize(&candidate) {
                Ok(path) => Some(path),
                Err(error) => {
                    last_error = Some(error);
                    None
                }
            }
        });
        let canonical_path = match canonical_path {
            Some(path) => path,
            None => {
                let full_path = attempted_path.unwrap_or_else(|| include_path.clone());
                self.errors.push(ParseError {
                    message: format!(
                        "Include file not found: '{}' ({})",
                        full_path.display(),
                        last_error
                            .map(|error| error.to_string())
                            .unwrap_or_else(|| "no candidate path was produced".to_owned())
                    ),
                    file: self.current_file.clone(),
                    line: Some(directive_line),
                });
                return;
            }
        };
        if let Some(owner) = self.current_file.clone() {
            if let Err(message) =
                self.record_resolved_dependency(owner, include_literal, canonical_path.clone())
            {
                self.errors.push(ParseError {
                    message,
                    file: self.current_file.clone(),
                    line: Some(directive_line),
                });
                return;
            }
        }

        if self.include_stack.contains(&canonical_path) {
            let mut cycle = self
                .include_stack
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>();
            cycle.push(canonical_path.display().to_string());
            self.errors.push(ParseError {
                message: format!("Cyclic include dependency: {}", cycle.join(" -> ")),
                file: self.current_file.clone(),
                line: Some(directive_line),
            });
            return;
        }

        match std::fs::read(&canonical_path) {
            Ok(bytes) => match crate::netlist::decode_source_bytes(&bytes) {
                Ok(content) => {
                    let saved_file = self.current_file.take();
                    let saved_dir = self.base_dir.clone();

                    self.current_file = Some(canonical_path.clone());
                    self.base_dir = canonical_path
                        .parent()
                        .unwrap_or(Path::new("."))
                        .to_path_buf();
                    self.include_depth += 1;
                    self.include_stack.push(canonical_path.clone());
                    self.record_resolved_source(canonical_path.clone(), &bytes, &content);

                    self.parse_content(&content);

                    self.include_stack.pop();
                    self.include_depth -= 1;
                    self.current_file = saved_file;
                    self.base_dir = saved_dir;
                }
                Err(error) => {
                    self.errors.push(ParseError {
                        message: format!(
                            "Failed to decode include file '{}': {error}",
                            canonical_path.display()
                        ),
                        file: self.current_file.clone(),
                        line: Some(directive_line),
                    });
                }
            },
            Err(e) => {
                self.errors.push(ParseError {
                    message: format!(
                        "Failed to read include file '{}': {e}",
                        canonical_path.display()
                    ),
                    file: self.current_file.clone(),
                    line: Some(directive_line),
                });
            }
        }
    }

    fn process_authenticated_include(&mut self, include_literal: &str, directive_line: usize) {
        if self.include_depth >= self.max_include_depth {
            self.errors.push(ParseError {
                message: format!(
                    "Maximum include depth ({}) exceeded",
                    self.max_include_depth
                ),
                file: self.current_file.clone(),
                line: Some(directive_line),
            });
            return;
        }
        let Some(owner) = self.current_file.clone() else {
            self.errors.push(ParseError {
                message: "Authenticated dependency has no owning source identity".to_owned(),
                file: None,
                line: Some(directive_line),
            });
            return;
        };
        let requested_path = match crate::netlist::normalize_source_path_literal(include_literal) {
            Ok(path) => path,
            Err(error) => {
                self.errors.push(ParseError {
                    message: format!("Invalid authenticated dependency path: {error}"),
                    file: Some(owner),
                    line: Some(directive_line),
                });
                return;
            }
        };
        let target = self
            .authenticated_dependencies
            .as_ref()
            .and_then(|dependencies| {
                dependencies
                    .get(&(owner.clone(), requested_path.clone()))
                    .cloned()
            });
        let Some(target) = target else {
            self.errors.push(ParseError {
                message: format!(
                    "Authenticated dependency '{}' has no retained resolution edge",
                    requested_path
                ),
                file: Some(owner),
                line: Some(directive_line),
            });
            return;
        };
        let bytes = self
            .authenticated_sources
            .as_ref()
            .and_then(|sources| sources.get(&target).cloned());
        let Some(bytes) = bytes else {
            self.errors.push(ParseError {
                message: format!(
                    "Authenticated dependency target '{}' has no retained source bytes",
                    target.display()
                ),
                file: Some(owner),
                line: Some(directive_line),
            });
            return;
        };
        if let Err(message) =
            self.record_resolved_dependency(owner.clone(), &requested_path, target.clone())
        {
            self.errors.push(ParseError {
                message,
                file: Some(owner),
                line: Some(directive_line),
            });
            return;
        }
        if self.include_stack.contains(&target) {
            let mut cycle = self
                .include_stack
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>();
            cycle.push(target.display().to_string());
            self.errors.push(ParseError {
                message: format!("Cyclic include dependency: {}", cycle.join(" -> ")),
                file: Some(owner),
                line: Some(directive_line),
            });
            return;
        }
        let content = match crate::netlist::decode_source_bytes(&bytes) {
            Ok(content) => content,
            Err(error) => {
                self.errors.push(ParseError {
                    message: format!(
                        "Failed to decode authenticated dependency '{}': {error}",
                        target.display()
                    ),
                    file: Some(owner),
                    line: Some(directive_line),
                });
                return;
            }
        };
        let saved_file = self.current_file.replace(target.clone());
        let saved_dir = std::mem::replace(
            &mut self.base_dir,
            target.parent().unwrap_or(Path::new(".")).to_path_buf(),
        );
        self.include_depth += 1;
        self.include_stack.push(target.clone());
        self.record_resolved_source(target, &bytes, &content);
        self.parse_content(&content);
        self.include_stack.pop();
        self.include_depth -= 1;
        self.current_file = saved_file;
        self.base_dir = saved_dir;
    }

    /// Parse .subckt start line
    fn parse_subckt_start(
        &self,
        line: &str,
        line_number: usize,
    ) -> Result<ParsedSubcircuit, String> {
        // Format: .SUBCKT name pin1 pin2 ... [param1=val1 ...]
        let rest = Self::directive_rest(line, ".subckt")
            .ok_or_else(|| "Expected a .subckt directive".to_owned())?;
        let header = super::parser::split_library_subcircuit_header(rest)
            .map_err(|message| format!(".subckt {message}"))?;
        let name = header.name;
        if header.parenthesized_ports.is_none() && header.tail.is_empty() {
            return Err(format!(".subckt '{name}' is missing its formal-port list"));
        }

        let mut fields = vec![name.to_owned()];
        let parenthesized_port_count = if let Some(formals) = header.parenthesized_ports {
            let formal_fields = Self::tokenize_subcircuit_header(formals);
            let count = formal_fields.len();
            fields.extend(formal_fields);
            fields.extend(Self::tokenize_subcircuit_header(header.tail));
            Some(count)
        } else {
            fields.extend(Self::tokenize_subcircuit_header(header.tail));
            None
        };
        let name = fields[0].to_string();

        let mut pins = Vec::new();
        let mut params = HashMap::new();
        let mut parameter_defaults = HashMap::new();
        let mut index = 1;
        if let Some(port_count) = parenthesized_port_count {
            for field in fields.iter().skip(1).take(port_count) {
                if super::parser::is_subcircuit_params_marker(field)
                    || super::parser::is_subcircuit_optional_marker(field)
                    || field.contains(['=', '(', ')'])
                {
                    return Err(format!(
                        ".subckt '{name}' parenthesized formal-port list may contain only ports"
                    ));
                }
                pins.push(field.to_string());
            }
            index += port_count;
        } else {
            while index < fields.len() {
                let field = fields[index].as_str();
                if super::parser::is_subcircuit_params_marker(field)
                    || super::parser::is_subcircuit_optional_marker(field)
                    || field.contains('=')
                    || matches!(fields.get(index + 1).map(String::as_str), Some("="))
                {
                    break;
                }
                if field.contains(['(', ')']) {
                    return Err(format!(
                        ".subckt '{name}' parentheses around formal ports must form one balanced outer pair"
                    ));
                }
                pins.push(field.to_string());
                index += 1;
            }
        }

        let mut optional_mode = false;
        while index < fields.len() {
            let field = fields[index].as_str();
            if super::parser::is_subcircuit_params_marker(field) {
                optional_mode = false;
                index += 1;
                continue;
            }
            if super::parser::is_subcircuit_optional_marker(field) {
                optional_mode = true;
                index += 1;
                continue;
            }
            if optional_mode {
                index += 1;
                continue;
            }

            let assignment = if let Some((key, value)) = field.split_once('=') {
                if value.is_empty() && index + 1 < fields.len() {
                    index += 1;
                    Some((key, fields[index].as_str()))
                } else {
                    Some((key, value))
                }
            } else if index + 2 < fields.len() && fields[index + 1] == "=" {
                let value = fields[index + 2].as_str();
                index += 2;
                Some((field, value))
            } else {
                None
            };

            if let Some((key, value)) = assignment {
                let key = key.trim();
                let value = value.trim();
                if !key.is_empty() && !value.is_empty() {
                    parameter_defaults.insert(key.to_string(), value.to_string());
                    if let Ok(value) = Self::parse_spice_number(value) {
                        params.insert(key.to_string(), value);
                    }
                }
            }
            index += 1;
        }

        let mut subckt = ParsedSubcircuit::new(name, pins);
        subckt.parameters = params;
        subckt.parameter_defaults = parameter_defaults;
        subckt.source_line = Some(line_number);
        Ok(subckt)
    }

    /// Split one folded `.SUBCKT` header while keeping quoted, braced, and
    /// parenthesized parameter expressions intact.
    fn tokenize_subcircuit_header(header: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut quote = None;
        let mut escaped = false;
        let mut brace_depth = 0usize;
        let mut paren_depth = 0usize;

        for character in header.chars() {
            if escaped {
                current.push(character);
                escaped = false;
                continue;
            }
            if quote.is_some() && character == '\\' {
                current.push(character);
                escaped = true;
                continue;
            }
            if let Some(delimiter) = quote {
                current.push(character);
                if character == delimiter {
                    quote = None;
                }
                continue;
            }
            match character {
                '\'' | '"' => {
                    quote = Some(character);
                    current.push(character);
                }
                '{' => {
                    brace_depth += 1;
                    current.push(character);
                }
                '}' => {
                    brace_depth = brace_depth.saturating_sub(1);
                    current.push(character);
                }
                '(' => {
                    paren_depth += 1;
                    current.push(character);
                }
                ')' => {
                    paren_depth = paren_depth.saturating_sub(1);
                    current.push(character);
                }
                character
                    if (character.is_whitespace() || character == ',')
                        && brace_depth == 0
                        && paren_depth == 0 =>
                {
                    if !current.is_empty() {
                        fields.push(std::mem::take(&mut current));
                    }
                }
                _ => current.push(character),
            }
        }
        if !current.is_empty() {
            fields.push(current);
        }
        fields
    }

    /// Return the text following one complete, case-insensitive directive
    /// token. Prefix lookalikes are intentionally not accepted.
    fn directive_rest<'a>(line: &'a str, directive: &str) -> Option<&'a str> {
        let line = line.trim();
        let token_end = line.find(char::is_whitespace).unwrap_or(line.len());
        line[..token_end]
            .eq_ignore_ascii_case(directive)
            .then(|| line[token_end..].trim())
    }

    /// Parse .model line with full parameter extraction.
    fn parse_model_line(&self, line: &str) -> Result<ParsedModel, String> {
        // Format: .MODEL name type (param1=val1 param2=val2 ...)
        // or:     .MODEL name type param1=val1 param2=val2 ...
        let rest = Self::directive_rest(line, ".model")
            .ok_or_else(|| "Expected a .model directive".to_owned())?;
        let name_end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let name = rest[..name_end].trim();
        if name.is_empty() {
            return Err(".model declaration is missing its model name".to_owned());
        }

        let after_name = rest[name_end..].trim_start();
        if after_name.is_empty() {
            return Err(format!(".model '{name}' is missing its device model type"));
        }
        let type_end = after_name
            .find(|character: char| character.is_whitespace() || character == '(')
            .unwrap_or(after_name.len());
        let type_str = &after_name[..type_end];
        if type_str.is_empty() {
            return Err(format!(".model '{name}' is missing its device model type"));
        }
        if type_str.chars().any(|character| {
            character.is_control() || matches!(character, '=' | ',' | ')' | '{' | '}')
        }) {
            return Err(format!(
                ".model '{name}' has an invalid device model type '{type_str}'"
            ));
        }
        let parameter_text = after_name[type_end..].trim();

        let model_type = ModelType::from_spice_type(type_str);
        let mut model = ParsedModel::new(name.to_owned(), model_type);
        model.spice_type = type_str.to_ascii_uppercase();

        if !parameter_text.is_empty() {
            let parameter_text = if parameter_text.starts_with('(') {
                Self::strip_model_parameter_parentheses(parameter_text)?
            } else {
                parameter_text
            };
            self.parse_model_parameters(parameter_text, &mut model)?;
        }

        // Extract level and version
        if model.string_params.contains_key("level") {
            return Err(format!(
                ".model '{}' LEVEL must be a non-negative integer",
                model.name
            ));
        }
        model.level = match model.get_param("level") {
            Some(level) if level >= 0.0 && level.fract() == 0.0 && level <= f64::from(u32::MAX) => {
                Some(level as u32)
            }
            Some(_) => {
                return Err(format!(
                    ".model '{}' LEVEL must be a non-negative integer in the u32 range",
                    model.name
                ));
            }
            None => None,
        };
        model.version = model.get_param("version");

        // Extract binning parameters (PDK geometry-based model selection)
        // These are critical for foundry PDKs like TSMC, GF, etc.
        model.lmin = model.get_param("lmin");
        model.lmax = model.get_param("lmax");
        model.wmin = model.get_param("wmin");
        model.wmax = model.get_param("wmax");

        // Extract bin prefix from model name for grouping related bins
        model.bin_prefix = model.extract_bin_prefix();

        Ok(model)
    }

    /// Remove one complete pair of outer parameter-list parentheses.
    fn strip_model_parameter_parentheses(parameters: &str) -> Result<&str, String> {
        debug_assert!(parameters.starts_with('('));
        let mut quote = None;
        let mut escaped = false;
        let mut paren_depth = 0usize;
        let mut brace_depth = 0usize;

        for (index, character) in parameters.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            if quote.is_some() && character == '\\' {
                escaped = true;
                continue;
            }
            if let Some(delimiter) = quote {
                if character == delimiter {
                    quote = None;
                }
                continue;
            }
            match character {
                '\'' | '"' => quote = Some(character),
                '{' => brace_depth += 1,
                '}' if brace_depth == 0 => {
                    return Err("Unexpected '}' in .model parameter list".to_owned());
                }
                '}' => brace_depth -= 1,
                '(' if brace_depth == 0 => paren_depth += 1,
                ')' if brace_depth == 0 => {
                    if paren_depth == 0 {
                        return Err("Unexpected ')' in .model parameter list".to_owned());
                    }
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        if !parameters[index + character.len_utf8()..].trim().is_empty() {
                            return Err(
                                "Unexpected content after the closing .model parameter list"
                                    .to_owned(),
                            );
                        }
                        return Ok(&parameters[1..index]);
                    }
                }
                _ => {}
            }
        }

        if quote.is_some() {
            Err("Unterminated quoted value in .model parameter list".to_owned())
        } else if brace_depth != 0 {
            Err("Unterminated braced value in .model parameter list".to_owned())
        } else {
            Err("Unterminated parenthesized .model parameter list".to_owned())
        }
    }

    /// Parse model assignments without losing quoted strings, braced
    /// expressions, nested function calls, or whitespace around `=`.
    fn parse_model_parameters(
        &self,
        params_str: &str,
        model: &mut ParsedModel,
    ) -> Result<(), String> {
        let bytes = params_str.as_bytes();
        let mut index = 0usize;
        let mut declared = HashSet::new();

        while index < bytes.len() {
            while index < bytes.len()
                && (bytes[index].is_ascii_whitespace() || bytes[index] == b',')
            {
                index += 1;
            }
            if index == bytes.len() {
                break;
            }

            let key_start = index;
            while index < bytes.len()
                && !bytes[index].is_ascii_whitespace()
                && !matches!(bytes[index], b'=' | b',')
            {
                index += 1;
            }
            let key = &params_str[key_start..index];
            if key.is_empty()
                || !key.chars().enumerate().all(|(position, character)| {
                    (position > 0 && character.is_ascii_digit())
                        || character.is_ascii_alphabetic()
                        || matches!(character, '_' | '.' | '$')
                })
            {
                return Err(format!("Invalid .model parameter name '{key}'"));
            }
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            if index == bytes.len() || bytes[index] != b'=' {
                return Err(format!(
                    ".model parameter '{key}' is missing its '=' assignment"
                ));
            }
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            if index == bytes.len() || bytes[index] == b',' {
                return Err(format!(".model parameter '{key}' has an empty value"));
            }

            let value_start = index;
            let mut quote = None;
            let mut escaped = false;
            let mut brace_depth = 0usize;
            let mut paren_depth = 0usize;
            while index < bytes.len() {
                let character = bytes[index];
                if escaped {
                    escaped = false;
                    index += 1;
                    continue;
                }
                if quote.is_some() && character == b'\\' {
                    escaped = true;
                    index += 1;
                    continue;
                }
                if let Some(delimiter) = quote {
                    if character == delimiter {
                        quote = None;
                    }
                    index += 1;
                    continue;
                }
                match character {
                    b'\'' | b'"' => quote = Some(character),
                    b'{' => brace_depth += 1,
                    b'}' if brace_depth == 0 => {
                        return Err(format!(
                            "Unexpected '}}' in value for .model parameter '{key}'"
                        ));
                    }
                    b'}' => brace_depth -= 1,
                    b'(' => paren_depth += 1,
                    b')' if paren_depth == 0 => {
                        return Err(format!(
                            "Unexpected ')' in value for .model parameter '{key}'"
                        ));
                    }
                    b')' => paren_depth -= 1,
                    b',' if brace_depth == 0 && paren_depth == 0 => break,
                    character
                        if character.is_ascii_whitespace()
                            && brace_depth == 0
                            && paren_depth == 0 =>
                    {
                        break;
                    }
                    _ => {}
                }
                index += 1;
            }
            if quote.is_some() {
                return Err(format!(
                    "Unterminated quoted value for .model parameter '{key}'"
                ));
            }
            if brace_depth != 0 {
                return Err(format!(
                    "Unterminated braced value for .model parameter '{key}'"
                ));
            }
            if paren_depth != 0 {
                return Err(format!(
                    "Unterminated parenthesized value for .model parameter '{key}'"
                ));
            }

            let value = params_str[value_start..index].trim();
            if value.is_empty() {
                return Err(format!(".model parameter '{key}' has an empty value"));
            }
            let key = key.to_ascii_lowercase();
            if !declared.insert(key.clone()) {
                return Err(format!(
                    ".model parameter '{key}' is declared more than once (parameter names are case-insensitive)"
                ));
            }
            if let Ok(value) = Self::parse_spice_number(value) {
                model.parameters.insert(key, value);
            } else {
                model
                    .string_params
                    .insert(key, Self::parse_model_string_value(value));
            }
        }
        Ok(())
    }

    /// Decode one quoted string-valued `.model` parameter while preserving
    /// the established unquoted-token behavior used by foundry libraries.
    fn parse_model_string_value(value: &str) -> String {
        let inner = value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .or_else(|| {
                value
                    .strip_prefix('\'')
                    .and_then(|value| value.strip_suffix('\''))
            });
        let Some(inner) = inner else {
            return value.to_owned();
        };
        let mut decoded = String::with_capacity(inner.len());
        let mut characters = inner.chars();
        while let Some(character) = characters.next() {
            if character == '\\' {
                if let Some(escaped) = characters.next() {
                    decoded.push(escaped);
                } else {
                    decoded.push('\\');
                }
            } else {
                decoded.push(character);
            }
        }
        decoded
    }

    /// Parse SPICE number format (handles suffixes like 1e-9, 1n, 1u, etc.)
    fn parse_spice_number(s: &str) -> Result<f64, ()> {
        let s = s.trim();
        if s.is_empty() {
            return Err(());
        }

        // Try direct parse first
        if let Ok(value) = s.parse::<f64>() {
            return value.is_finite().then_some(value).ok_or(());
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

        let value = base * multiplier;
        value.is_finite().then_some(value).ok_or(())
    }

    /// Build the final parse result
    fn build_result(&self) -> LibParseResult {
        LibParseResult {
            sections: self.sections.clone(),
            top_level_models: self.top_level_models.clone(),
            top_level_subcircuits: self.top_level_subcircuits.clone(),
            errors: self.errors.clone(),
            resolved_sources: self.resolved_sources.clone(),
            resolved_dependencies: self.resolved_dependencies.clone(),
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
    /// Canonical paths and exact contents read for the root file and every
    /// transitive include. Entries are unique by canonical path.
    pub resolved_sources: Vec<ResolvedLibSource>,
    /// Authenticated canonical resolution edges for every include/library path
    /// literal encountered while capturing the closure.
    pub resolved_dependencies: Vec<ResolvedLibDependency>,
}

/// One exact source captured while resolving a library dependency closure.
#[derive(Clone)]
pub struct ResolvedLibSource {
    pub path: PathBuf,
    /// Exact bytes read from the source, before BOM/encoding decoding.
    pub bytes: Arc<[u8]>,
    /// Decoded text consumed by the parser.
    pub content: Arc<str>,
}

impl std::fmt::Debug for ResolvedLibSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ResolvedLibSource")
            .field("path", &self.path)
            .field("byte_len", &self.bytes.len())
            .finish()
    }
}

/// One canonical dependency edge captured during library import.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResolvedLibDependency {
    pub owner: PathBuf,
    pub requested_path: String,
    pub target: PathBuf,
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

    fn unique_lib_parser_temp_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rspice-lib-parser-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time after epoch")
                .as_nanos()
        ))
    }

    #[test]
    fn parser_rejects_unclosed_lib_section() {
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(".lib TT\n.model nch NMOS (LEVEL=1)\n");

        assert!(
            !result.is_ok(),
            "missing .endl must be a parser error, not an implicit close"
        );
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.message.contains(".lib TT")
                    && error.message.to_ascii_lowercase().contains(".endl")),
            "errors should name the unterminated section: {:?}",
            result.errors
        );
    }

    #[test]
    fn parser_rejects_unclosed_subcircuit() {
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(".subckt amp in out\nr1 in out 1k\n");

        assert!(
            !result.is_ok(),
            "missing .ends must be a parser error, not dropped subcircuit content"
        );
        assert!(
            result
                .errors
                .iter()
                .any(|error| error.message.contains(".subckt amp")
                    && error.message.to_ascii_lowercase().contains(".ends")),
            "errors should name the unterminated subcircuit: {:?}",
            result.errors
        );
    }

    #[test]
    fn subcircuit_interface_preserves_ports_defaults_and_physical_source_line() {
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(
            "* interface contract\n\
             \n\
             .SuBcKt precision_amp inp inn out vdd vss params: GAIN=100\n\
             + MODE=\"low noise\" SCALE={GAIN * 2} OFFSET = 2m\n\
             e1 out 0 inp inn {GAIN}\n\
             .ends precision_amp\n",
        );

        assert!(result.is_ok(), "{:?}", result.errors);
        let subcircuit = result
            .top_level_subcircuits
            .first()
            .expect("one top-level subcircuit");
        assert_eq!(subcircuit.name, "precision_amp");
        assert_eq!(
            subcircuit.pins,
            ["inp", "inn", "out", "vdd", "vss"],
            "PARAMS: and defaults must never become terminals"
        );
        assert_eq!(
            subcircuit
                .parameter_defaults
                .get("GAIN")
                .map(String::as_str),
            Some("100")
        );
        assert_eq!(
            subcircuit
                .parameter_defaults
                .get("MODE")
                .map(String::as_str),
            Some("\"low noise\"")
        );
        assert_eq!(
            subcircuit
                .parameter_defaults
                .get("SCALE")
                .map(String::as_str),
            Some("{GAIN * 2}")
        );
        assert_eq!(
            subcircuit
                .parameter_defaults
                .get("OFFSET")
                .map(String::as_str),
            Some("2m")
        );
        assert_eq!(subcircuit.parameters.get("GAIN").copied(), Some(100.0));
        assert_eq!(
            subcircuit
                .parameters
                .get("OFFSET")
                .map(|value| value.to_bits()),
            Some((2e-3_f64).to_bits())
        );
        assert_eq!(subcircuit.source_line, Some(3));
    }

    #[test]
    fn parenthesized_subcircuit_interface_preserves_parameter_expressions() {
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(
            ".SuBcKt precision_filter (input, output, reference) OPTIONAL: substrate=0\n\
             + PARAMS: CURVE=lookup(1, 2) SCALE={pow(GAIN, 2)}\n\
             r1 input output 1k\n\
             .ends precision_filter\n",
        );

        assert!(result.is_ok(), "{:?}", result.errors);
        let subcircuit = result
            .top_level_subcircuits
            .first()
            .expect("one top-level subcircuit");
        assert_eq!(subcircuit.name, "precision_filter");
        assert_eq!(subcircuit.pins, ["input", "output", "reference"]);
        assert_eq!(
            subcircuit
                .parameter_defaults
                .get("CURVE")
                .map(String::as_str),
            Some("lookup(1, 2)")
        );
        assert_eq!(
            subcircuit
                .parameter_defaults
                .get("SCALE")
                .map(String::as_str),
            Some("{pow(GAIN, 2)}")
        );
        assert!(
            !subcircuit.parameter_defaults.contains_key("substrate"),
            "OPTIONAL terminal defaults are not ordinary subcircuit parameters"
        );
        assert!(!subcircuit.parameters.contains_key("substrate"));
    }

    #[test]
    fn subcircuit_names_retain_internal_parentheses() {
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(
            ".subckt S861(C1)_5000/SIE input output\n\
             .ends S861(C1)_5000/SIE\n",
        );

        assert!(result.is_ok(), "{:?}", result.errors);
        let subcircuit = result
            .top_level_subcircuits
            .first()
            .expect("one top-level subcircuit");
        assert_eq!(subcircuit.name, "S861(C1)_5000/SIE");
        assert_eq!(subcircuit.pins, ["input", "output"]);
    }

    #[test]
    fn subckt_prefix_lookalike_is_not_a_declaration() {
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(
            ".SUBCKTfoo vendor extension\n\
             .model visible D\n",
        );

        assert!(result.is_ok(), "{:?}", result.errors);
        assert!(result.top_level_subcircuits.is_empty());
        assert!(result.find_model("visible").is_some());
    }

    #[test]
    fn plain_subcircuit_interfaces_stop_at_optional_params_and_bare_assignments() {
        for source in [
            ".subckt optional input output OPTIONAL: reference=0 PARAMS: GAIN=(1+2)\n\
             .ends optional\n",
            ".subckt bare input output GAIN=(1+2)\n.ends bare\n",
            ".subckt param input output PARAM GAIN=(1+2)\n.ends param\n",
            ".subckt param_colon input output PARAM: GAIN=(1+2)\n.ends param_colon\n",
            ".subckt parameters input output PARAMETERS GAIN=(1+2)\n.ends parameters\n",
            ".subckt parameters_colon input output PARAMETERS: GAIN=(1+2)\n\
             .ends parameters_colon\n",
        ] {
            let mut parser = LibParser::new(".");
            let result = parser.parse_string(source);
            assert!(result.is_ok(), "{source:?}: {:?}", result.errors);
            let subcircuit = result
                .top_level_subcircuits
                .first()
                .expect("one top-level subcircuit");
            assert_eq!(subcircuit.pins, ["input", "output"]);
            assert_eq!(
                subcircuit
                    .parameter_defaults
                    .get("GAIN")
                    .map(String::as_str),
                Some("(1+2)")
            );
            assert!(!subcircuit.parameter_defaults.contains_key("reference"));
            assert!(!subcircuit.parameters.contains_key("reference"));
        }
    }

    #[test]
    fn malformed_parenthesized_subcircuit_interfaces_are_diagnostic() {
        for (source, expected) in [
            (
                ".subckt empty ()\n.ends empty\n",
                "formal-port list cannot be empty",
            ),
            (
                ".subckt nested ((input output))\n.ends nested\n",
                "nested parentheses",
            ),
            (
                ".subckt unclosed (input output\n.ends unclosed\n",
                "missing its closing ')'",
            ),
            (
                ".subckt trailing (input output) reference\n.ends trailing\n",
                "only OPTIONAL or parameter declarations may follow",
            ),
            (
                ".subckt assigned (input GAIN=1)\n.ends assigned\n",
                "may contain only ports",
            ),
            (
                ".subckt adjacent(input output)\n.ends adjacent\n",
                "balanced outer pair",
            ),
        ] {
            let mut parser = LibParser::new(".");
            let result = parser.parse_string(source);
            assert!(!result.is_ok(), "{source:?} must fail");
            assert!(
                result
                    .errors
                    .iter()
                    .any(|error| { error.message.contains(expected) && error.line == Some(1) }),
                "{source:?} should report {expected:?}: {:?}",
                result.errors
            );
            assert!(
                result.top_level_subcircuits.is_empty(),
                "{source:?} must not publish a partial interface"
            );
        }
    }

    #[test]
    fn quoted_model_string_parameters_are_decoded_exactly() {
        let mut parser = LibParser::new(".");
        let result = parser
            .parse_string(".model nch NMOS (LEVEL=1 VERSION_TAG=\"release\\\\candidate\\\"1\")\n");

        assert!(result.is_ok(), "quoted model parameter must parse");
        let model = result
            .top_level_models
            .first()
            .expect("one top-level model");
        assert_eq!(
            model.string_params.get("version_tag").map(String::as_str),
            Some("release\\candidate\"1")
        );
    }

    #[test]
    fn model_parser_preserves_grouped_values_spacing_and_physical_source_line() {
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(
            "\n\
             .MoDeL nch NMOS ( LEVEL = 54,\n\
             + VERSION_TAG = \"release candidate\" SCALE={GAIN * 2}\n\
             + TABLE=lookup(1, 2) OFFSET = 2m )\n",
        );

        assert!(result.is_ok(), "{:?}", result.errors);
        let model = result
            .top_level_models
            .first()
            .expect("one top-level model");
        assert_eq!(model.level, Some(54));
        assert_eq!(model.parameters.get("offset").copied(), Some(2e-3));
        assert_eq!(
            model.string_params.get("version_tag").map(String::as_str),
            Some("release candidate")
        );
        assert_eq!(
            model.string_params.get("scale").map(String::as_str),
            Some("{GAIN * 2}")
        );
        assert_eq!(
            model.string_params.get("table").map(String::as_str),
            Some("lookup(1, 2)")
        );
        assert_eq!(model.source_line, Some(2));
    }

    #[test]
    fn malformed_model_cards_are_diagnostic_and_never_partially_published() {
        for (source, expected) in [
            (".model\n", "missing its model name"),
            (".model nch\n", "missing its device model type"),
            (".model nch NMOS (A=)\n", "empty value"),
            (
                ".model nch NMOS (A=\"unterminated)\n",
                "Unterminated quoted",
            ),
            (".model nch NMOS (A={unterminated)\n", "Unterminated braced"),
            (
                ".model nch NMOS (A=lookup(1, 2)\n",
                "Unterminated parenthesized",
            ),
            (
                ".model nch NMOS (A=1) trailing\n",
                "Unexpected content after",
            ),
            (".model nch NMOS (stray)\n", "missing its '='"),
        ] {
            let mut parser = LibParser::new(".");
            let result = parser.parse_string(source);
            assert!(!result.is_ok(), "{source:?} must fail");
            assert!(
                result
                    .errors
                    .iter()
                    .any(|error| error.message.contains(expected) && error.line == Some(1)),
                "{source:?} should report {expected:?}: {:?}",
                result.errors
            );
            assert!(
                result.top_level_models.is_empty(),
                "{source:?} must not publish a partial model"
            );
        }
    }

    #[test]
    fn model_parameter_and_model_names_are_unique_case_insensitively_per_scope() {
        let mut duplicate_parameter_parser = LibParser::new(".");
        let duplicate_parameter =
            duplicate_parameter_parser.parse_string(".model nch NMOS (VTH0=0.4 vth0=0.5)\n");
        assert!(!duplicate_parameter.is_ok());
        assert!(duplicate_parameter.errors.iter().any(|error| {
            error
                .message
                .contains("declared more than once (parameter names are case-insensitive)")
        }));
        assert!(duplicate_parameter.top_level_models.is_empty());

        let mut duplicate_model_parser = LibParser::new(".");
        let duplicate_model = duplicate_model_parser.parse_string(
            ".model nch NMOS (LEVEL=1)\n\
             .model NCH NMOS (LEVEL=2)\n",
        );
        assert!(!duplicate_model.is_ok());
        assert_eq!(duplicate_model.top_level_models.len(), 1);
        assert!(duplicate_model.errors.iter().any(|error| {
            error.message.contains("Duplicate .model name 'NCH'") && error.line == Some(2)
        }));

        let mut separate_scopes_parser = LibParser::new(".");
        let separate_scopes = separate_scopes_parser.parse_string(
            ".lib TT\n\
             .model nch NMOS (LEVEL=1)\n\
             .endl TT\n\
             .lib FF\n\
             .model NCH NMOS (LEVEL=2)\n\
             .endl FF\n",
        );
        assert!(separate_scopes.is_ok(), "{:?}", separate_scopes.errors);
        assert_eq!(separate_scopes.sections.len(), 2);
    }

    #[test]
    fn model_directive_requires_a_complete_token_and_level_is_exact() {
        let mut lookalike_parser = LibParser::new(".");
        let lookalike = lookalike_parser.parse_string(".modelcheck nch NMOS (LEVEL=1)\n");
        assert!(lookalike.is_ok());
        assert!(lookalike.top_level_models.is_empty());

        for level in ["-1", "1.5", "4294967296", "NaN", "inf"] {
            let mut parser = LibParser::new(".");
            let result = parser.parse_string(&format!(".model nch NMOS (LEVEL={level})\n"));
            assert!(!result.is_ok(), "LEVEL={level} must fail");
            assert!(
                result
                    .errors
                    .iter()
                    .any(|error| error.message.contains("LEVEL must be"))
            );
            assert!(result.top_level_models.is_empty());
        }
    }

    #[test]
    fn parser_retains_unknown_spice_model_type_token_exactly() {
        let mut parser = LibParser::new(".");
        let result = parser.parse_string(".model power_fet VDMOS (LEVEL=1 KP=2e-3)\n");

        assert!(result.is_ok(), "custom model type must parse");
        let model = result
            .top_level_models
            .first()
            .expect("one top-level model");
        assert_eq!(model.model_type, ModelType::Other);
        assert_eq!(model.spice_type, "VDMOS");
    }

    fn write_depth_fixture(directory: &Path, source_count: usize) -> PathBuf {
        std::fs::create_dir_all(directory).expect("create depth fixture");
        for index in 0..source_count {
            let content = if index + 1 < source_count {
                format!(".incl \"{}.inc\"\n", index + 1)
            } else {
                ".model depth_n NMOS (LEVEL=1)\n".to_owned()
            };
            std::fs::write(directory.join(format!("{index}.inc")), content)
                .expect("write depth source");
        }
        directory.join("0.inc")
    }

    #[test]
    fn dependency_capture_matches_the_64_frame_execution_boundary() {
        let accepted_dir = unique_lib_parser_temp_dir("depth-accepted");
        let accepted_root =
            write_depth_fixture(&accepted_dir, crate::resource::DEFAULT_MAX_INCLUDE_DEPTH);
        let mut accepted_parser = LibParser::new(&accepted_dir);
        let accepted = accepted_parser
            .parse_file(&accepted_root)
            .expect("accepted depth fixture reads");
        assert!(accepted.is_ok(), "{:?}", accepted.errors);
        assert_eq!(
            accepted.resolved_sources.len(),
            crate::resource::DEFAULT_MAX_INCLUDE_DEPTH
        );
        assert_eq!(
            accepted.resolved_dependencies.len(),
            crate::resource::DEFAULT_MAX_INCLUDE_DEPTH - 1
        );

        let rejected_dir = unique_lib_parser_temp_dir("depth-rejected");
        let rejected_root = write_depth_fixture(
            &rejected_dir,
            crate::resource::DEFAULT_MAX_INCLUDE_DEPTH + 1,
        );
        let mut rejected_parser = LibParser::new(&rejected_dir);
        let rejected = rejected_parser
            .parse_file(&rejected_root)
            .expect("rejected depth fixture still reads its root");
        assert!(!rejected.is_ok());
        assert!(rejected.errors.iter().any(|error| {
            error
                .message
                .contains("Maximum include depth (64) exceeded")
                && error.line == Some(1)
        }));

        let _ = std::fs::remove_dir_all(accepted_dir);
        let _ = std::fs::remove_dir_all(rejected_dir);
    }

    #[test]
    fn discovery_treats_both_spice_separator_styles_consistently() {
        let directory = unique_lib_parser_temp_dir("mixed-separators");
        let subdirectory = directory.join("sub");
        std::fs::create_dir_all(&subdirectory).expect("create nested source directory");
        let dependency = subdirectory.join("device.inc");
        std::fs::write(&dependency, ".model nested_n NMOS (LEVEL=1)\n")
            .expect("write nested dependency");
        let root = directory.join("root.lib");
        std::fs::write(&root, ".include \"sub\\device.inc\"\n").expect("write backslash include");

        let mut parser = LibParser::new(&directory);
        let result = parser.parse_file(&root).expect("root source reads");

        assert!(result.is_ok(), "{:?}", result.errors);
        assert_eq!(result.resolved_dependencies.len(), 1);
        assert_eq!(
            result.resolved_dependencies[0].requested_path,
            "sub/device.inc"
        );
        assert_eq!(
            result.resolved_dependencies[0].target,
            std::fs::canonicalize(&dependency).expect("canonical dependency")
        );
        assert!(result.find_model("nested_n").is_some());

        let _ = std::fs::remove_dir_all(directory);
    }

    #[test]
    fn conflicting_targets_for_one_normalized_edge_are_rejected() {
        let mut parser = LibParser::new(".");
        let owner = PathBuf::from("owner.lib");
        let first = PathBuf::from("first.inc");
        let second = PathBuf::from("second.inc");
        parser
            .record_resolved_dependency(owner.clone(), r"sub\device.inc", first.clone())
            .expect("first resolution is captured");

        let error = parser
            .record_resolved_dependency(owner, "sub/device.inc", second)
            .expect_err("normalized edge identity cannot silently change target");

        assert!(
            error.contains("Conflicting dependency resolution"),
            "{error}"
        );
        assert!(error.contains("sub/device.inc"), "{error}");
        assert_eq!(parser.resolved_dependencies.len(), 1);
        assert_eq!(parser.resolved_dependencies[0].target, first);
    }

    #[test]
    fn authenticated_closure_preserves_included_model_identity_without_filesystem_reads() {
        let root = PathBuf::from(r"C:\retained\root.lib");
        let included = PathBuf::from(r"C:\retained\models\device.inc");
        let mut parser = LibParser::new(r"C:\retained");
        let result = parser
            .parse_authenticated_closure(
                root.clone(),
                [
                    (root.clone(), b".include \"models/device.inc\"\n".to_vec()),
                    (
                        included.clone(),
                        b"* included card\n.model nested_n NMOS (LEVEL=1 KP=1e-3)\n".to_vec(),
                    ),
                ],
                [ResolvedLibDependency {
                    owner: root,
                    requested_path: "models/device.inc".to_owned(),
                    target: included.clone(),
                }],
            )
            .expect("sealed closure is structurally valid");

        assert!(result.is_ok(), "{:?}", result.errors);
        let model = result
            .find_model("nested_n")
            .expect("included model parses");
        assert_eq!(model.source_file.as_ref(), Some(&included));
        assert_eq!(model.source_line, Some(2));
        assert_eq!(result.resolved_sources.len(), 2);
        assert_eq!(result.resolved_dependencies.len(), 1);
    }

    #[test]
    fn authenticated_closure_never_falls_back_to_the_host_filesystem() {
        let root = PathBuf::from(r"C:\retained\root.lib");
        let included = PathBuf::from(r"C:\retained\device.inc");
        let mut parser = LibParser::new(r"C:\retained");
        let result = parser
            .parse_authenticated_closure(
                root.clone(),
                [(root, b".include \"device.inc\"\n".to_vec())],
                std::iter::empty(),
            )
            .expect("sealed closure is structurally valid");

        assert!(!result.is_ok());
        assert!(
            result
                .errors
                .iter()
                .any(|error| { error.message.contains("has no retained resolution edge") })
        );
        assert!(result.find_model("nested_n").is_none());
        assert!(!included.exists(), "test must not depend on a host file");
    }
}
