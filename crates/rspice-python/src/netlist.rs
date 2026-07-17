//! Netlist Python bindings
//!
//! Provides Python access to netlist parsing and introspection.
//!
//! Title handling: classic SPICE treats the first line of a deck as a title,
//! which silently swallows the first element of inline strings (or turns a
//! statement-looking title into a device). The Python API avoids both traps
//! with an explicit rule:
//!
//! - [`PyNetlist::parse`] treats the content as *statements only*. Unless the
//!   first non-blank line is a `*` comment (which becomes the title), a
//!   synthetic title is prepended — a typo'd first element raises ParseError
//!   instead of becoming the title.
//! - [`PyNetlist::parse_spice`] applies raw SPICE deck semantics: the first
//!   line is always the title.
//! - [`PyNetlist::parse_file`] keeps raw SPICE semantics, matching every
//!   other SPICE tool's treatment of `.sp`/`.cir` files.
//! - All four public entry points perform strict output-symbol validation;
//!   permissive AST parsing remains an explicit core-only tooling facility.

use pyo3::prelude::*;
use rspice_core::netlist::{AnalysisCommand, DcSecondSweep, DcSweepMode};
use rspice_core::{Netlist, Value};
use std::borrow::Cow;

/// A parsed SPICE netlist ready for simulation
///
/// The Netlist class represents a parsed circuit description that can be
/// passed to an Engine for simulation.
///
/// Example:
///     >>> netlist = Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")
///     >>> netlist = Netlist.parse_file("circuit.sp")
#[pyclass(name = "Netlist", module = "rspice")]
pub struct PyNetlist {
    pub(crate) inner: Netlist,
}

/// Non-fatal parser diagnostic attached to a parsed netlist.
#[pyclass(name = "ParseDiagnostic", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyParseDiagnostic {
    #[pyo3(get)]
    pub line: usize,
    #[pyo3(get)]
    pub severity: String,
    #[pyo3(get)]
    pub code: String,
    #[pyo3(get)]
    pub message: String,
}

impl From<&rspice_core::netlist::ParseDiagnostic> for PyParseDiagnostic {
    fn from(diagnostic: &rspice_core::netlist::ParseDiagnostic) -> Self {
        Self {
            line: diagnostic.line,
            severity: match diagnostic.severity {
                rspice_core::netlist::DiagnosticSeverity::Warning => "warning".to_string(),
            },
            code: diagnostic.code.clone(),
            message: diagnostic.message.clone(),
        }
    }
}

/// Prepend a synthetic title unless the content already starts with a
/// `*` comment line (which SPICE treats as the title here).
fn ensure_statement_content(content: &str) -> Cow<'_, str> {
    let first_meaningful = content.lines().map(str::trim).find(|line| !line.is_empty());
    match first_meaningful {
        Some(line) if line.starts_with('*') => Cow::Borrowed(content),
        Some(_) => Cow::Owned(format!("* Untitled circuit\n{content}")),
        None => Cow::Borrowed(content),
    }
}

fn describe_dc_sweep_spec(
    source: &str,
    start: Value,
    stop: Value,
    step: Value,
    mode: &DcSweepMode,
) -> String {
    match mode {
        DcSweepMode::Linear => format!("{source} {start} {stop} {step}"),
        DcSweepMode::List(values) => {
            let values = values
                .iter()
                .map(Value::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            format!("{source} list {values}")
        }
        DcSweepMode::Decade { points_per_decade } => {
            format!("{source} dec {start} {stop} {points_per_decade}")
        }
        DcSweepMode::Octave { points_per_octave } => {
            format!("{source} oct {start} {stop} {points_per_octave}")
        }
    }
}

fn describe_dc_analysis(
    source: &str,
    start: Value,
    stop: Value,
    step: Value,
    mode: &DcSweepMode,
    sweep2: Option<&DcSecondSweep>,
) -> String {
    let mut out = format!(
        ".dc {}",
        describe_dc_sweep_spec(source, start, stop, step, mode)
    );
    if let Some(outer) = sweep2 {
        out.push(' ');
        out.push_str(&describe_dc_sweep_spec(
            &outer.source,
            outer.start,
            outer.stop,
            outer.step,
            &outer.mode,
        ));
    }
    out
}

/// Render an analysis command as a short human-readable summary.
pub(crate) fn describe_analysis(analysis: &AnalysisCommand) -> String {
    match analysis {
        AnalysisCommand::Op => ".op".to_string(),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2,
        } => describe_dc_analysis(source, *start, *stop, *step, mode, sweep2.as_ref()),
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => format!(
            ".ac {} {points} {start_freq} {stop_freq}",
            format!("{variation:?}").to_lowercase()
        ),
        AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            probe,
        } => format!(
            ".stb {} {points} {start_freq} {stop_freq} probe={probe}",
            format!("{variation:?}").to_lowercase()
        ),
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic: _,
        } => {
            let mut out = format!(".tran {step} {stop}");
            if let Some(start) = start {
                out.push_str(&format!(" {start}"));
            }
            if let Some(max_step) = max_step {
                out.push_str(&format!(" {max_step}"));
            }
            out
        }
        AnalysisCommand::Noise {
            output_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        } => format!(
            ".noise v({output_node}) {input_source} {} {points} {start_freq} {stop_freq}",
            format!("{variation:?}").to_lowercase()
        ),
        AnalysisCommand::Tf {
            output_node,
            input_source,
            output_is_current,
            ..
        } => {
            let probe = if *output_is_current { "i" } else { "v" };
            format!(".tf {probe}({output_node}) {input_source}")
        }
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            filters,
            ac_sweep,
        } => {
            let probe = if *output_is_current {
                format!("i({output_node})")
            } else if let Some(reference) = reference_node {
                format!("v({output_node},{reference})")
            } else {
                format!("v({output_node})")
            };
            let mut description = format!(".sens {probe}");
            if !filters.is_empty() {
                description.push(' ');
                description.push_str(&filters.join(" "));
            }
            if let Some(sweep) = ac_sweep {
                description.push_str(&format!(
                    " ac {} {} {} {}",
                    format!("{:?}", sweep.variation).to_lowercase(),
                    sweep.points,
                    sweep.start_freq,
                    sweep.stop_freq
                ));
            }
            description
        }
        AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } => format!(
            ".four {fundamental} {} ({num_harmonics} harmonics)",
            outputs.join(" ")
        ),
        other => format!("{other:?}"),
    }
}

#[pymethods]
impl PyNetlist {
    /// Parse a netlist from a string of circuit statements
    ///
    /// Every line is treated as a statement. If the first non-blank line is
    /// a `*` comment it becomes the title; otherwise a synthetic title is
    /// added. A malformed first element therefore raises ParseError instead
    /// of being silently consumed as a title (use `parse_spice` for raw
    /// SPICE deck semantics).
    ///
    /// Args:
    ///     content: SPICE circuit statements
    ///
    /// Returns:
    ///     Netlist: Parsed netlist object
    ///
    /// Raises:
    ///     ParseError: If the netlist contains syntax or semantic errors
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
        let normalized = ensure_statement_content(content);
        let inner =
            Netlist::parse_validated(&normalized).map_err(crate::errors::parse_error_to_pyerr)?;
        Ok(Self { inner })
    }

    /// Parse a raw SPICE deck where the first line is always the title
    ///
    /// Mirrors classic SPICE semantics exactly: line one is the title even
    /// if it looks like an element statement.
    ///
    /// Args:
    ///     content: Raw SPICE deck text
    ///
    /// Returns:
    ///     Netlist: Parsed netlist object
    ///
    /// Raises:
    ///     ParseError: If the netlist contains syntax or semantic errors
    ///
    /// Example:
    ///     >>> netlist = Netlist.parse_spice("My Amplifier\nV1 1 0 10\n.end")
    #[staticmethod]
    pub fn parse_spice(content: &str) -> PyResult<Self> {
        let inner =
            Netlist::parse_validated(content).map_err(crate::errors::parse_error_to_pyerr)?;
        Ok(Self { inner })
    }

    /// Parse a netlist from a file with include resolution
    ///
    /// Reads a SPICE deck (first line is the title, per universal `.sp`
    /// convention) and expands `.include`/`.lib` directives relative to the
    /// file's location.
    ///
    /// Args:
    ///     path: Path to the netlist file (str or os.PathLike)
    ///
    /// Returns:
    ///     Netlist: Parsed netlist object
    ///
    /// Raises:
    ///     ParseError: If the file cannot be read or contains syntax or semantic errors
    ///
    /// Example:
    ///     >>> netlist = Netlist.parse_file("circuits/amplifier.sp")
    ///     >>> netlist = Netlist.parse_file(pathlib.Path("circuits") / "amplifier.sp")
    #[staticmethod]
    pub fn parse_file(path: std::path::PathBuf) -> PyResult<Self> {
        let content = Netlist::read_source(&path).map_err(crate::errors::parse_error_to_pyerr)?;
        let inner = Netlist::parse_validated_with_path(&content, &path)
            .map_err(crate::errors::parse_error_to_pyerr)?;
        Ok(Self { inner })
    }

    /// Parse statements from a string, resolving includes against a base path
    ///
    /// Same statement semantics as `parse`, but `.include`/`.lib` directives
    /// resolve relative to `base_path`.
    ///
    /// Args:
    ///     content: SPICE circuit statements
    ///     base_path: Base path for resolving include directives
    ///
    /// Returns:
    ///     Netlist: Parsed netlist object
    ///
    /// Raises:
    ///     ParseError: If the netlist contains syntax or semantic errors
    #[staticmethod]
    pub fn parse_with_includes(content: &str, base_path: std::path::PathBuf) -> PyResult<Self> {
        let normalized = ensure_statement_content(content);
        // Core resolves includes relative to the *parent* of the given path
        // (it expects a file path). Accept a directory by anchoring a
        // synthetic file inside it.
        let anchor = if base_path.is_dir() {
            base_path.join("__inline_netlist__.sp")
        } else {
            base_path
        };
        let inner = Netlist::parse_validated_with_path(&normalized, &anchor)
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

    /// Get the number of .MEAS statements in the netlist
    #[getter]
    fn num_measurements(&self) -> usize {
        self.inner.measurements.len()
    }

    /// Get the netlist title (first line comment)
    #[getter]
    fn title(&self) -> String {
        self.inner.title.clone()
    }

    /// Non-fatal parser diagnostics for accepted but ignored/downgraded syntax
    #[getter]
    fn diagnostics(&self) -> Vec<PyParseDiagnostic> {
        self.inner.diagnostics.iter().map(Into::into).collect()
    }

    /// Names of all device elements, in netlist order
    #[getter]
    fn element_names(&self) -> Vec<String> {
        self.inner.elements.iter().map(|e| e.name.clone()).collect()
    }

    /// Names of all .MODEL definitions
    #[getter]
    fn model_names(&self) -> Vec<String> {
        self.inner.models.iter().map(|m| m.name.clone()).collect()
    }

    /// Names of all .SUBCKT definitions
    #[getter]
    fn subcircuit_names(&self) -> Vec<String> {
        self.inner
            .subcircuits
            .iter()
            .map(|s| s.name.clone())
            .collect()
    }

    /// Names of all .MEAS statements
    #[getter]
    fn measurement_names(&self) -> Vec<String> {
        self.inner
            .measurements
            .iter()
            .map(|m| m.name.clone())
            .collect()
    }

    /// Parsed .MEAS verification contracts as
    /// `(name, analysis, goal, tolerance)` tuples.
    #[getter]
    fn measurement_specs(&self) -> Vec<(String, String, Option<f64>, Option<f64>)> {
        self.inner
            .measurements
            .iter()
            .map(|m| (m.name.clone(), m.analysis.clone(), m.goal, m.tolerance))
            .collect()
    }

    /// Human-readable summaries of the netlist's analysis directives
    ///
    /// Example:
    ///     >>> netlist.analyses
    ///     ['.tran 1e-06 0.001', '.ac dec 10 1 1000000']
    #[getter]
    fn analyses(&self) -> Vec<String> {
        self.inner.analyses.iter().map(describe_analysis).collect()
    }

    /// Check if a node is marked as global
    fn is_global(&self, node: &str) -> bool {
        self.inner.is_global(node)
    }

    fn __repr__(&self) -> String {
        format!(
            "Netlist(elements={}, models={}, subcircuits={}, analyses={}, measurements={})",
            self.inner.elements.len(),
            self.inner.models.len(),
            self.inner.subcircuits.len(),
            self.inner.analyses.len(),
            self.inner.measurements.len()
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
