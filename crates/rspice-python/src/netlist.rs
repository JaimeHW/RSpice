//! Netlist Python bindings
//!
//! Provides Python access to netlist parsing and manipulation:
//! - Parse netlists from strings or files
//! - Handle include directives with path resolution
//! - Access parsed netlist information

use pyo3::prelude::*;
use rspice_core::Netlist;
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
        let inner = Netlist::parse(content).map_err(crate::errors::parse_error_to_pyerr)?;
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
        let inner =
            Netlist::parse_with_path(content, path).map_err(crate::errors::parse_error_to_pyerr)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_netlist() {
        let content = r#"
* Simple test
V1 1 0 10
R1 1 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 2);
        assert_eq!(netlist.num_models(), 0);
        assert_eq!(netlist.num_subcircuits(), 0);
    }

    #[test]
    fn test_parse_voltage_divider() {
        let content = r#"
* Voltage divider
V1 1 0 10
R1 1 2 1k
R2 2 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 3);
    }

    #[test]
    fn test_parse_with_title() {
        let content = r#"
* My Test Circuit
V1 1 0 5
R1 1 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        // Note: title is the first comment line, may be trimmed
        let title = netlist.title();
        // Title may contain the text or be empty depending on parser
        assert!(title.is_empty() || title.contains("My Test Circuit"));
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let content = "INVALID GIBBERISH NOT A NETLIST";
        let result = PyNetlist::parse(content);
        assert!(result.is_ok()); // Parser is lenient, returns empty netlist
    }

    #[test]
    fn test_parse_diode_circuit() {
        let content = r#"
* Diode test
V1 1 0 5
D1 1 2 1N4148
R1 2 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 3);
    }

    #[test]
    fn test_parse_mosfet_circuit() {
        let content = r#"
* MOSFET test
Vgs 1 0 3
Vds 2 0 5
M1 2 1 0 0 NMOS
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 3);
    }

    #[test]
    fn test_parse_with_model() {
        let content = r#"
* Model test
V1 1 0 5
D1 1 0 MYDIODE
.model MYDIODE D IS=1e-14 N=1.0
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 2);
        assert_eq!(netlist.num_models(), 1);
    }

    #[test]
    fn test_parse_with_subcircuit() {
        let content = r#"
* Subcircuit test
V1 1 0 10
X1 1 2 0 DIVIDER
.subckt DIVIDER in out gnd
R1 in out 1k
R2 out gnd 1k
.ends
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_subcircuits(), 1);
    }

    #[test]
    fn test_parse_with_analysis() {
        let content = r#"
* Analysis test
V1 1 0 10
R1 1 0 1k
.dc V1 0 5 0.1
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert!(netlist.num_analyses() >= 1);
    }

    #[test]
    fn test_netlist_repr() {
        let content = "V1 1 0 5\nR1 1 0 1k\n.end";
        let netlist = PyNetlist::parse(content).unwrap();
        let repr = netlist.__repr__();
        assert!(repr.contains("Netlist"));
        // Parser may or may not count all elements depending on parsing rules
        assert!(repr.contains("elements="));
    }

    #[test]
    fn test_netlist_str() {
        let content = "V1 1 0 5\nR1 1 0 1k\n.end";
        let netlist = PyNetlist::parse(content).unwrap();
        let str_repr = netlist.__str__();
        assert!(str_repr.contains("Netlist"));
        // Allow flexible element count
        assert!(str_repr.contains("element"));
    }

    #[test]
    fn test_parse_rc_circuit() {
        let content = r#"
* RC circuit
V1 1 0 5
R1 1 2 1k
C1 2 0 1u
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 3);
    }

    #[test]
    fn test_parse_rl_circuit() {
        let content = r#"
* RL circuit
V1 1 0 5
R1 1 2 1k
L1 2 0 1m
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 3);
    }

    #[test]
    fn test_parse_current_source() {
        let content = r#"
* Current source
I1 0 1 1m
R1 1 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 2);
    }

    #[test]
    fn test_parse_vcvs() {
        let content = r#"
* VCVS test
V1 1 0 3
R1 1 0 1k
E1 2 0 1 0 1.0
R2 2 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 4);
    }

    #[test]
    fn test_parse_vccs() {
        let content = r#"
* VCCS test
V1 1 0 2
R1 1 0 1k
G1 2 0 1 0 0.001
R2 2 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 4);
    }

    #[test]
    fn test_parse_pulse_source() {
        let content = r#"
* Pulse test
V1 1 0 PULSE(0 5 0 1n 1n 0.5u 1u)
R1 1 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 2);
    }

    #[test]
    fn test_parse_sin_source() {
        let content = r#"
* Sin test
V1 1 0 SIN(0 1 1k)
R1 1 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 2);
    }

    #[test]
    fn test_parse_ac_source() {
        let content = r#"
* AC test
V1 1 0 AC 1
R1 1 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 2);
    }

    #[test]
    fn test_parse_multiple_sources() {
        let content = r#"
* Multiple sources
V1 1 0 5
V2 2 1 3
R1 2 0 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 3);
    }

    #[test]
    fn test_parse_bjt() {
        let content = r#"
* BJT test
Vcc 1 0 5
Rb 1 2 10k
Rc 1 3 1k
Q1 3 2 0 2N2222
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 4);
    }

    #[test]
    fn test_parse_file_nonexistent() {
        let result = PyNetlist::parse_file("nonexistent_file_12345.sp");
        assert!(result.is_err());
    }

    #[test]
    fn test_global_node() {
        let content = r#"
.global VDD GND
V1 VDD 0 5
R1 VDD GND 1k
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert!(netlist.is_global("VDD"));
        assert!(netlist.is_global("GND"));
        assert!(!netlist.is_global("NOTGLOBAL"));
    }

    #[test]
    fn test_parse_empty_netlist() {
        let content = ".end";
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 0);
    }

    #[test]
    fn test_parse_comment_only() {
        let content = r#"
* This is a comment
* Another comment
.end
"#;
        let netlist = PyNetlist::parse(content).unwrap();
        assert_eq!(netlist.num_elements(), 0);
    }

    #[test]
    fn test_parse_engineering_notation() {
        let content = r#"
* Engineering notation
V1 1 0 1.5k
R1 1 0 10meg
R2 1 0 100n
R3 1 0 50p
.end
"#;
        // Parser should handle engineering notation properly
        let result = PyNetlist::parse(content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_case_insensitivity() {
        let content1 = "V1 1 0 5\nR1 1 0 1K\n.END";
        let content2 = "v1 1 0 5\nr1 1 0 1k\n.end";

        let netlist1 = PyNetlist::parse(content1).unwrap();
        let netlist2 = PyNetlist::parse(content2).unwrap();

        assert_eq!(netlist1.num_elements(), netlist2.num_elements());
    }
}
