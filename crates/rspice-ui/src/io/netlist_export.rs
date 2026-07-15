//! Netlist Export
//!
//! Generate Spectre-compatible netlists from schematic data.
//! Supports hierarchical design with subcircuit expansion.
//!
//! # Features
//!
//! - Multiple output formats (Spectre, SPICE, HSPICE)
//! - Hierarchical subcircuit handling
//! - Parameter expression preservation
//! - Include directive generation
//! - Comment and documentation export

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as FmtWrite;
use std::path::{Path, PathBuf};

// =============================================================================
// Netlist Format
// =============================================================================

/// Output netlist format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum NetlistFormat {
    /// Cadence Spectre format
    #[default]
    Spectre,
    /// Standard SPICE format
    Spice,
    /// Synopsys HSPICE format
    Hspice,
    /// Xyce format
    Xyce,
}

impl NetlistFormat {
    /// Comment prefix for this format
    pub fn comment_prefix(&self) -> &'static str {
        match self {
            NetlistFormat::Spectre => "// ",
            NetlistFormat::Spice | NetlistFormat::Hspice | NetlistFormat::Xyce => "* ",
        }
    }

    /// Option directive prefix
    pub fn option_prefix(&self) -> &'static str {
        match self {
            NetlistFormat::Spectre => "simulator lang=spectre\n",
            _ => "",
        }
    }

    /// Subcircuit keyword
    pub fn subckt_keyword(&self) -> &'static str {
        match self {
            NetlistFormat::Spectre => "subckt",
            _ => ".SUBCKT",
        }
    }

    /// Subcircuit end keyword
    pub fn ends_keyword(&self) -> &'static str {
        match self {
            NetlistFormat::Spectre => "ends",
            _ => ".ENDS",
        }
    }

    /// Parameter assignment operator
    pub fn param_op(&self) -> &'static str {
        match self {
            NetlistFormat::Spectre => "=",
            _ => "=",
        }
    }

    /// File extension
    pub fn extension(&self) -> &'static str {
        match self {
            NetlistFormat::Spectre => "scs",
            NetlistFormat::Spice => "spice",
            NetlistFormat::Hspice => "sp",
            NetlistFormat::Xyce => "cir",
        }
    }
}

// =============================================================================
// Export Options
// =============================================================================

/// Netlist export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    /// Output format
    pub format: NetlistFormat,
    /// Include header comment
    pub include_header: bool,
    /// Include timestamp
    pub include_timestamp: bool,
    /// Flatten hierarchy
    pub flatten: bool,
    /// Maximum line length (0 = unlimited)
    pub max_line_length: usize,
    /// Continuation character
    pub continuation: String,
    /// Model library includes to add
    pub model_includes: Vec<PathBuf>,
    /// Corner to use
    pub corner: Option<String>,
    /// Global parameters
    pub global_params: HashMap<String, String>,
    /// Preserve comments from schematic
    pub preserve_comments: bool,
    /// Sort components by type
    pub sort_by_type: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: NetlistFormat::Spectre,
            include_header: true,
            include_timestamp: true,
            flatten: false,
            max_line_length: 80,
            continuation: "+ ".to_string(),
            model_includes: Vec::new(),
            corner: None,
            global_params: HashMap::new(),
            preserve_comments: true,
            sort_by_type: true,
        }
    }
}

// =============================================================================
// Component Instance
// =============================================================================

/// A component instance to export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInstance {
    /// Instance name
    pub name: String,
    /// Component type (R, C, M, X, etc.)
    pub comp_type: char,
    /// Connected nodes
    pub nodes: Vec<String>,
    /// Model/subcircuit name
    pub model: Option<String>,
    /// Parameters
    pub params: HashMap<String, String>,
    /// Comment
    pub comment: Option<String>,
}

impl ComponentInstance {
    /// Create a new instance
    pub fn new(name: impl Into<String>, comp_type: char, nodes: Vec<String>) -> Self {
        Self {
            name: name.into(),
            comp_type,
            nodes,
            model: None,
            params: HashMap::new(),
            comment: None,
        }
    }

    /// Set model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Add parameter
    pub fn with_param(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(name.into(), value.into());
        self
    }

    /// Format for Spectre
    fn format_spectre(&self) -> String {
        let mut s = self.name.clone();
        s.push(' ');
        s.push('(');
        s.push_str(&self.nodes.join(" "));
        s.push(')');

        if let Some(ref model) = self.model {
            s.push(' ');
            s.push_str(model);
        }

        for (k, v) in &self.params {
            s.push(' ');
            s.push_str(k);
            s.push('=');
            s.push_str(v);
        }

        s
    }

    /// Format for SPICE
    fn format_spice(&self) -> String {
        let mut s = self.name.clone();

        for node in &self.nodes {
            s.push(' ');
            s.push_str(node);
        }

        if let Some(ref model) = self.model {
            s.push(' ');
            s.push_str(model);
        }

        for (k, v) in &self.params {
            s.push(' ');
            s.push_str(k);
            s.push('=');
            s.push_str(v);
        }

        s
    }

    /// Format for given format
    pub fn format(&self, fmt: NetlistFormat) -> String {
        match fmt {
            NetlistFormat::Spectre => self.format_spectre(),
            _ => self.format_spice(),
        }
    }
}

// =============================================================================
// Subcircuit Definition
// =============================================================================

/// Subcircuit for export
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubcircuitExport {
    /// Subcircuit name
    pub name: String,
    /// Port names
    pub ports: Vec<String>,
    /// Parameters with defaults
    pub params: HashMap<String, String>,
    /// Internal instances
    pub instances: Vec<ComponentInstance>,
    /// Comment
    pub comment: Option<String>,
}

impl SubcircuitExport {
    /// Create new subcircuit
    pub fn new(name: impl Into<String>, ports: Vec<String>) -> Self {
        Self {
            name: name.into(),
            ports,
            ..Default::default()
        }
    }

    /// Add instance
    pub fn add_instance(&mut self, inst: ComponentInstance) {
        self.instances.push(inst);
    }

    /// Format for given format
    pub fn format(&self, fmt: NetlistFormat, options: &ExportOptions) -> String {
        let mut s = String::new();

        // Comment
        if let Some(ref comment) = self.comment {
            writeln!(s, "{}{}", fmt.comment_prefix(), comment).ok();
        }

        // Header
        match fmt {
            NetlistFormat::Spectre => {
                write!(s, "{} {} (", fmt.subckt_keyword(), self.name).ok();
                s.push_str(&self.ports.join(" "));
                s.push(')');

                if !self.params.is_empty() {
                    for (k, v) in &self.params {
                        write!(s, " {}={}", k, v).ok();
                    }
                }
                s.push('\n');
            }
            _ => {
                write!(s, "{} {}", fmt.subckt_keyword(), self.name).ok();
                for port in &self.ports {
                    write!(s, " {}", port).ok();
                }
                s.push('\n');

                if !self.params.is_empty() {
                    s.push_str(".PARAM");
                    for (k, v) in &self.params {
                        write!(s, " {}={}", k, v).ok();
                    }
                    s.push('\n');
                }
            }
        }

        // Instances
        for inst in &self.instances {
            let line = inst.format(fmt);
            s.push_str(&wrap_line(
                &line,
                options.max_line_length,
                &options.continuation,
            ));
            s.push('\n');
        }

        // End
        writeln!(s, "{} {}", fmt.ends_keyword(), self.name).ok();

        s
    }
}

// =============================================================================
// Netlist Exporter
// =============================================================================

/// Main netlist exporter
#[derive(Debug, Clone, Default)]
pub struct NetlistExporter {
    /// Export options
    pub options: ExportOptions,
    /// Title
    pub title: String,
    /// Subcircuits
    pub subcircuits: Vec<SubcircuitExport>,
    /// Top-level instances
    pub instances: Vec<ComponentInstance>,
    /// Analysis commands
    pub analyses: Vec<String>,
    /// Global nodes
    pub global_nodes: HashSet<String>,
}

impl NetlistExporter {
    /// Create new exporter
    pub fn new(options: ExportOptions) -> Self {
        let mut global_nodes = HashSet::new();
        global_nodes.insert("0".to_string());
        global_nodes.insert("gnd".to_string());

        Self {
            options,
            global_nodes,
            ..Default::default()
        }
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Add subcircuit
    pub fn add_subcircuit(&mut self, subckt: SubcircuitExport) {
        self.subcircuits.push(subckt);
    }

    /// Add top-level instance
    pub fn add_instance(&mut self, inst: ComponentInstance) {
        self.instances.push(inst);
    }

    /// Add analysis command
    pub fn add_analysis(&mut self, cmd: impl Into<String>) {
        self.analyses.push(cmd.into());
    }

    /// Add global node
    pub fn add_global(&mut self, node: impl Into<String>) {
        self.global_nodes.insert(node.into());
    }

    /// Generate netlist string
    pub fn generate(&self) -> String {
        let mut output = String::new();
        let fmt = self.options.format;

        // Header
        if self.options.include_header {
            self.write_header(&mut output);
        }

        // Format prefix
        output.push_str(fmt.option_prefix());

        // Includes
        self.write_includes(&mut output);

        // Global params
        self.write_global_params(&mut output);

        // Global nodes
        if !self.global_nodes.is_empty() && fmt != NetlistFormat::Spectre {
            output.push_str(".GLOBAL");
            for node in &self.global_nodes {
                write!(output, " {}", node).ok();
            }
            output.push('\n');
        }

        output.push('\n');

        // Subcircuits
        for subckt in &self.subcircuits {
            output.push_str(&subckt.format(fmt, &self.options));
            output.push('\n');
        }

        // Top-level instances
        for inst in &self.instances {
            let line = inst.format(fmt);
            output.push_str(&wrap_line(
                &line,
                self.options.max_line_length,
                &self.options.continuation,
            ));
            output.push('\n');
        }

        // Analyses
        if !self.analyses.is_empty() {
            output.push('\n');
            for analysis in &self.analyses {
                output.push_str(analysis);
                output.push('\n');
            }
        }

        // End
        match fmt {
            NetlistFormat::Spectre => {}
            _ => output.push_str(".END\n"),
        }

        output
    }

    /// Write header comment
    fn write_header(&self, output: &mut String) {
        let prefix = self.options.format.comment_prefix();

        writeln!(output, "{}{}", prefix, "=".repeat(70)).ok();

        if !self.title.is_empty() {
            writeln!(output, "{}{}", prefix, self.title).ok();
        }

        if self.options.include_timestamp {
            let now = chrono_lite_now();
            writeln!(output, "{}Generated: {}", prefix, now).ok();
        }

        write!(output, "{}{}\n\n", prefix, "=".repeat(70)).ok();
    }

    /// Write include directives
    fn write_includes(&self, output: &mut String) {
        for path in &self.options.model_includes {
            match self.options.format {
                NetlistFormat::Spectre => {
                    if let Some(corner) = &self.options.corner {
                        writeln!(output, "include \"{}\" section={}", path.display(), corner).ok();
                    } else {
                        writeln!(output, "include \"{}\"", path.display()).ok();
                    }
                }
                _ => {
                    if let Some(corner) = &self.options.corner {
                        writeln!(output, ".LIB \"{}\" {}", path.display(), corner).ok();
                    } else {
                        writeln!(output, ".INCLUDE \"{}\"", path.display()).ok();
                    }
                }
            }
        }

        if !self.options.model_includes.is_empty() {
            output.push('\n');
        }
    }

    /// Write global parameters
    fn write_global_params(&self, output: &mut String) {
        if self.options.global_params.is_empty() {
            return;
        }

        match self.options.format {
            NetlistFormat::Spectre => {
                output.push_str("parameters ");
                for (i, (k, v)) in self.options.global_params.iter().enumerate() {
                    if i > 0 {
                        output.push(' ');
                    }
                    write!(output, "{}={}", k, v).ok();
                }
                output.push('\n');
            }
            _ => {
                output.push_str(".PARAM");
                for (k, v) in &self.options.global_params {
                    write!(output, " {}={}", k, v).ok();
                }
                output.push('\n');
            }
        }
    }

    /// Write to file
    pub fn write_to_file(&self, path: &Path) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = path;
            Err(
                "Path-based netlist export is unavailable in the browser; use the generated netlist with the browser download workflow"
                    .to_string(),
            )
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let expected = crate::io::durable_file::observe_expected_content(path)
                .map_err(|error| format!("Failed to authorize netlist destination: {error}"))?;
            let content = self.generate();
            publish_netlist(path, expected, content.as_bytes())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_netlist(
    path: &Path,
    expected: crate::io::durable_file::ExpectedContent,
    content: &[u8],
) -> Result<(), String> {
    crate::io::durable_file::compare_exchange_bytes(path, expected, content)
        .map_err(|error| format!("Failed to publish netlist: {error}"))
}

/// Line wrapping helper
fn wrap_line(line: &str, max_len: usize, continuation: &str) -> String {
    if max_len == 0 || line.len() <= max_len {
        return line.to_string();
    }

    let mut result = String::new();
    let mut current_line = String::new();

    for word in line.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= max_len {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&current_line);
            current_line = format!("{}{}", continuation, word);
        }
    }

    if !current_line.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&current_line);
    }

    result
}

/// Simple timestamp without chrono dependency
fn chrono_lite_now() -> String {
    let secs = crate::common::time_compat::unix_epoch().as_secs();
    format!("{}", secs)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn write_to_file_publishes_complete_netlist() {
        let root = unique_temp_dir("complete");
        let path = root.join("design.scs");
        let mut exporter = NetlistExporter::new(ExportOptions {
            include_timestamp: false,
            ..ExportOptions::default()
        });
        exporter.title = "Durable netlist".to_string();
        exporter.add_instance(ComponentInstance::new(
            "R1",
            'R',
            vec!["in".to_string(), "0".to_string()],
        ));

        exporter.write_to_file(&path).expect("write netlist");

        let saved = std::fs::read_to_string(&path).expect("read netlist");
        assert!(saved.contains("Durable netlist"));
        assert!(saved.contains("R1"));
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn publication_rejects_late_external_change() {
        let root = unique_temp_dir("late-change");
        let path = root.join("design.cir");
        std::fs::write(&path, b"authorized predecessor").expect("write predecessor");
        let expected =
            crate::io::durable_file::observe_expected_content(&path).expect("observe destination");
        let content = NetlistExporter::default().generate();
        std::fs::write(&path, b"late external edit").expect("race destination");

        let result = publish_netlist(&path, expected, content.as_bytes());

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"late external edit");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-netlist-export-{label}-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }
}
