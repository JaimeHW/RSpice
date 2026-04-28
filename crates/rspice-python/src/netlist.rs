//! Netlist Python bindings
//!
//! Provides Python access to netlist parsing and manipulation:
//! - Parse netlists from strings or files
//! - Handle include directives with path resolution
//! - Access parsed netlist information

use pyo3::prelude::*;
use rspice_core::Netlist;
use std::borrow::Cow;
use std::path::Path;

/// A parsed SPICE netlist ready for simulation
///
/// The Netlist class represents a parsed circuit description that can be
/// passed to an Engine for simulation.
///
/// Example:
///     >>> netlist = Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")
///     >>> netlist = Netlist.parse_file("circuit.sp")
#[pyclass(name = "Netlist")]
pub struct PyNetlist {
    pub(crate) inner: Netlist,
}

impl PyNetlist {
    fn parse_content(content: &str) -> Result<Netlist, rspice_core::error::ParseError> {
        let normalized = normalize_titleless_netlist(content);
        Netlist::parse(&normalized)
    }

    fn parse_content_with_includes(
        content: &str,
        base_path: &Path,
    ) -> Result<Netlist, rspice_core::error::ParseError> {
        let normalized = normalize_titleless_netlist(content);
        Netlist::parse_with_path(&normalized, base_path)
    }
}

fn normalize_titleless_netlist(content: &str) -> Cow<'_, str> {
    if content
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line.starts_with('*'))
    {
        return Cow::Borrowed(content);
    }

    let meaningful_lines: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('*'))
        .take(2)
        .collect();

    if meaningful_lines.len() == 2
        && meaningful_lines
            .iter()
            .copied()
            .all(parses_as_spice_statement)
    {
        Cow::Owned(format!("* Untitled circuit\n{content}"))
    } else {
        Cow::Borrowed(content)
    }
}

fn parses_as_spice_statement(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('*') || trimmed.starts_with('+') {
        return false;
    }

    let candidate = format!("* Statement probe\n{trimmed}\n.end");
    match Netlist::parse(&candidate) {
        Ok(parsed) => {
            if trimmed.starts_with('.') {
                return true;
            }

            !parsed.elements.is_empty()
                || !parsed.analyses.is_empty()
                || !parsed.models.is_empty()
                || !parsed.subcircuits.is_empty()
                || !parsed.global_nodes.is_empty()
                || !parsed.measurements.is_empty()
                || !parsed.veriloga_includes.is_empty()
                || !parsed.initial_conditions.is_empty()
                || !parsed.node_sets.is_empty()
        }
        Err(_) => false,
    }
}

#[pymethods]
impl PyNetlist {
    /// Parse a netlist from a string
    ///
    /// Args:
    ///     content: SPICE netlist content as a string
    ///
    /// Returns:
    ///     Netlist: Parsed netlist object
    ///
    /// Raises:
    ///     ParseError: If the netlist contains syntax errors
    ///
    /// Example:
    ///     >>> netlist = Netlist.parse('''
    ///     ... * Voltage divider
    ///     ... V1 1 0 10
    ///     ... R1 1 2 1k
    ///     ... R2 2 0 1k
    ///     ... .end
    ///     ... ''')
    #[staticmethod]
    pub fn parse(content: &str) -> PyResult<Self> {
        let inner = Self::parse_content(content).map_err(crate::errors::parse_error_to_pyerr)?;
        Ok(Self { inner })
    }

    /// Parse a netlist from a file with include resolution
    ///
    /// This method reads a netlist file and automatically expands any
    /// .include and .lib directives relative to the file's location.
    ///
    /// Args:
    ///     path: Path to the netlist file
    ///
    /// Returns:
    ///     Netlist: Parsed netlist object
    ///
    /// Raises:
    ///     ParseError: If the file cannot be read or contains syntax errors
    ///
    /// Example:
    ///     >>> netlist = Netlist.parse_file("circuits/amplifier.sp")
    #[staticmethod]
    pub fn parse_file(path: &str) -> PyResult<Self> {
        let path = Path::new(path);
        let inner = Netlist::parse_file(path).map_err(crate::errors::parse_error_to_pyerr)?;
        Ok(Self { inner })
    }

    /// Parse a netlist from a string with include resolution
    ///
    /// Like parse(), but resolves .include and .lib directives relative
    /// to the specified base path.
    ///
    /// Args:
    ///     content: SPICE netlist content as a string
    ///     base_path: Base path for resolving include directives
    ///
    /// Returns:
    ///     Netlist: Parsed netlist object
    ///
    /// Raises:
    ///     ParseError: If the netlist contains syntax errors
    ///
    /// Example:
    ///     >>> netlist = Netlist.parse_with_includes(content, "circuits/")
    #[staticmethod]
    pub fn parse_with_includes(content: &str, base_path: &str) -> PyResult<Self> {
        let path = Path::new(base_path);
        let inner = Self::parse_content_with_includes(content, path)
            .map_err(crate::errors::parse_error_to_pyerr)?;
        Ok(Self { inner })
    }

    /// Get the number of device elements in the netlist
    #[getter]
    pub fn num_elements(&self) -> usize {
        self.inner.elements.len()
    }

    /// Get the number of model definitions in the netlist
    #[getter]
    fn num_models(&self) -> usize {
        self.inner.models.len()
    }

    /// Get the number of subcircuit definitions in the netlist
    #[getter]
    fn num_subcircuits(&self) -> usize {
        self.inner.subcircuits.len()
    }

    /// Get the number of analysis commands in the netlist
    #[getter]
    fn num_analyses(&self) -> usize {
        self.inner.analyses.len()
    }

    /// Get the netlist title (first line comment)
    #[getter]
    fn title(&self) -> String {
        self.inner.title.clone()
    }

    /// Check if a node is marked as global
    fn is_global(&self, node: &str) -> bool {
        self.inner.is_global(node)
    }

    fn __repr__(&self) -> String {
        format!(
            "Netlist(elements={}, models={}, subcircuits={}, analyses={})",
            self.inner.elements.len(),
            self.inner.models.len(),
            self.inner.subcircuits.len(),
            self.inner.analyses.len()
        )
    }

    fn __str__(&self) -> String {
        if !self.inner.title.is_empty() {
            format!(
                "Netlist '{}' with {} elements",
                self.inner.title.trim(),
                self.inner.elements.len()
            )
        } else {
            format!("Netlist with {} elements", self.inner.elements.len())
        }
    }
}
