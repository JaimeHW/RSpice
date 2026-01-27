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
use std::io::Write;
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
            write!(s, "{}{}\n", fmt.comment_prefix(), comment).ok();
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
        write!(s, "{} {}\n", fmt.ends_keyword(), self.name).ok();

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

        write!(output, "{}{}\n", prefix, "=".repeat(70)).ok();

        if !self.title.is_empty() {
            write!(output, "{}{}\n", prefix, self.title).ok();
        }

        if self.options.include_timestamp {
            let now = chrono_lite_now();
            write!(output, "{}Generated: {}\n", prefix, now).ok();
        }

        write!(output, "{}{}\n\n", prefix, "=".repeat(70)).ok();
    }

    /// Write include directives
    fn write_includes(&self, output: &mut String) {
        for path in &self.options.model_includes {
            match self.options.format {
                NetlistFormat::Spectre => {
                    if let Some(corner) = &self.options.corner {
                        write!(
                            output,
                            "include \"{}\" section={}\n",
                            path.display(),
                            corner
                        )
                        .ok();
                    } else {
                        write!(output, "include \"{}\"\n", path.display()).ok();
                    }
                }
                _ => {
                    if let Some(corner) = &self.options.corner {
                        write!(output, ".LIB \"{}\" {}\n", path.display(), corner).ok();
                    } else {
                        write!(output, ".INCLUDE \"{}\"\n", path.display()).ok();
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
        let content = self.generate();
        std::fs::write(path, content).map_err(|e| format!("Failed to write: {}", e))
    }
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
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{}", secs)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // NetlistFormat Tests
    // =========================================================================

    #[test]
    fn test_format_comment_prefix() {
        assert_eq!(NetlistFormat::Spectre.comment_prefix(), "// ");
        assert_eq!(NetlistFormat::Spice.comment_prefix(), "* ");
    }

    #[test]
    fn test_format_extension() {
        assert_eq!(NetlistFormat::Spectre.extension(), "scs");
        assert_eq!(NetlistFormat::Spice.extension(), "spice");
    }

    // =========================================================================
    // ComponentInstance Tests
    // =========================================================================

    #[test]
    fn test_instance_creation() {
        let inst = ComponentInstance::new("R1", 'R', vec!["a".into(), "b".into()]);
        assert_eq!(inst.name, "R1");
        assert_eq!(inst.comp_type, 'R');
    }

    #[test]
    fn test_instance_with_params() {
        let inst = ComponentInstance::new(
            "M1",
            'M',
            vec!["d".into(), "g".into(), "s".into(), "b".into()],
        )
        .with_model("nmos")
        .with_param("w", "1u")
        .with_param("l", "180n");

        assert_eq!(inst.model, Some("nmos".to_string()));
        assert!(inst.params.contains_key("w"));
    }

    #[test]
    fn test_instance_format_spectre() {
        let inst = ComponentInstance::new("R1", 'R', vec!["a".into(), "b".into()])
            .with_model("resistor")
            .with_param("r", "1k");

        let s = inst.format(NetlistFormat::Spectre);
        assert!(s.contains("R1"));
        assert!(s.contains("(a b)"));
        assert!(s.contains("r=1k"));
    }

    #[test]
    fn test_instance_format_spice() {
        let inst =
            ComponentInstance::new("R1", 'R', vec!["a".into(), "b".into()]).with_param("r", "1k");

        let s = inst.format(NetlistFormat::Spice);
        assert!(s.contains("R1 a b"));
    }

    // =========================================================================
    // SubcircuitExport Tests
    // =========================================================================

    #[test]
    fn test_subcircuit_creation() {
        let subckt = SubcircuitExport::new("inv", vec!["in".into(), "out".into()]);
        assert_eq!(subckt.name, "inv");
        assert_eq!(subckt.ports.len(), 2);
    }

    #[test]
    fn test_subcircuit_format_spectre() {
        let mut subckt = SubcircuitExport::new(
            "inv",
            vec!["in".into(), "out".into(), "vdd".into(), "vss".into()],
        );
        subckt.add_instance(ComponentInstance::new(
            "M1",
            'M',
            vec!["out".into(), "in".into(), "vdd".into(), "vdd".into()],
        ));

        let options = ExportOptions::default();
        let s = subckt.format(NetlistFormat::Spectre, &options);

        assert!(s.contains("subckt inv"));
        assert!(s.contains("(in out vdd vss)"));
        assert!(s.contains("ends inv"));
    }

    // =========================================================================
    // NetlistExporter Tests
    // =========================================================================

    #[test]
    fn test_exporter_creation() {
        let exporter = NetlistExporter::new(ExportOptions::default());
        assert!(exporter.global_nodes.contains("0"));
    }

    #[test]
    fn test_exporter_with_title() {
        let exporter = NetlistExporter::new(ExportOptions::default()).with_title("Test Circuit");
        assert_eq!(exporter.title, "Test Circuit");
    }

    #[test]
    fn test_exporter_generate_simple() {
        let mut exporter = NetlistExporter::new(ExportOptions {
            include_header: false,
            ..Default::default()
        });

        exporter.add_instance(
            ComponentInstance::new("R1", 'R', vec!["a".into(), "b".into()])
                .with_model("resistor")
                .with_param("r", "1k"),
        );

        let netlist = exporter.generate();
        assert!(netlist.contains("R1"));
    }

    #[test]
    fn test_exporter_generate_with_subcircuit() {
        let mut exporter = NetlistExporter::new(ExportOptions {
            include_header: false,
            ..Default::default()
        });

        let mut subckt = SubcircuitExport::new("inv", vec!["in".into(), "out".into()]);
        subckt.add_instance(ComponentInstance::new(
            "M1",
            'M',
            vec!["out".into(), "in".into(), "vdd".into(), "vdd".into()],
        ));
        exporter.add_subcircuit(subckt);

        let netlist = exporter.generate();
        assert!(netlist.contains("subckt inv"));
        assert!(netlist.contains("ends inv"));
    }

    #[test]
    fn test_exporter_with_includes() {
        let mut options = ExportOptions::default();
        options.include_header = false;
        options.model_includes.push(PathBuf::from("models.lib"));
        options.corner = Some("tt".to_string());

        let exporter = NetlistExporter::new(options);
        let netlist = exporter.generate();

        assert!(netlist.contains("include"));
        assert!(netlist.contains("section=tt"));
    }

    // =========================================================================
    // Line Wrap Tests
    // =========================================================================

    #[test]
    fn test_wrap_line_short() {
        let line = "R1 a b 1k";
        let wrapped = wrap_line(line, 80, "+ ");
        assert_eq!(wrapped, line);
    }

    #[test]
    fn test_wrap_line_long() {
        let line = "M1 drain gate source bulk nmos w=1u l=180n nf=10 mult=1";
        let wrapped = wrap_line(line, 40, "+ ");
        assert!(wrapped.contains("\n+ "));
    }

    #[test]
    fn test_wrap_line_unlimited() {
        let line = "very long line";
        let wrapped = wrap_line(line, 0, "+ ");
        assert_eq!(wrapped, line);
    }
}
