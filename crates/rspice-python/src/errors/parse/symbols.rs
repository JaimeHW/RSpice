//! Structured items carried by aggregate output-validation errors.

use crate::errors::public_path_string;
use pyo3::prelude::*;

/// One unresolved circuit symbol retained by an aggregate output-validation
/// error. The string-valued tags are deliberately stable across core enum
/// evolution so Python automation does not need to parse display messages.
#[pyclass(
    name = "UnresolvedOutputSymbol",
    module = "rspice",
    frozen,
    from_py_object
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyUnresolvedOutputSymbol {
    #[pyo3(get)]
    pub directive: String,
    #[pyo3(get)]
    pub operator: String,
    #[pyo3(get)]
    pub symbol: String,
    #[pyo3(get)]
    pub kind: String,
    #[pyo3(get)]
    pub line: usize,
    #[pyo3(get)]
    pub source: Option<String>,
}

impl From<&rspice_core::netlist::UnresolvedOutputSymbol> for PyUnresolvedOutputSymbol {
    fn from(item: &rspice_core::netlist::UnresolvedOutputSymbol) -> Self {
        use rspice_core::netlist::{OutputDirectiveKind, OutputSymbolKind};

        let directive = match item.directive {
            OutputDirectiveKind::Save => "save",
            OutputDirectiveKind::Probe => "probe",
            OutputDirectiveKind::Print => "print",
            OutputDirectiveKind::Plot => "plot",
            OutputDirectiveKind::Measure => "measure",
            OutputDirectiveKind::Four => "four",
            OutputDirectiveKind::Fft => "fft",
        };
        let kind = match item.kind {
            OutputSymbolKind::Node => "node",
            OutputSymbolKind::Device => "device",
        };
        Self {
            directive: directive.to_string(),
            operator: item.operator.clone(),
            symbol: item.symbol.clone(),
            kind: kind.to_string(),
            line: item.origin.line,
            source: item
                .origin
                .path
                .as_ref()
                .map(|path| public_path_string(path)),
        }
    }
}
