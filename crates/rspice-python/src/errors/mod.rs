//! Python exception types for RSpice errors
//!
//! Provides properly typed Python exceptions for different error conditions:
//! - `RSpiceError` - Base exception for all RSpice errors
//! - `ParseError` - Netlist parsing failures
//! - `SimulationError` - General simulation failures
//! - `ConvergenceError` - Newton-Raphson convergence failures
//! - `CancelledError` - Programmatic simulation cancellation
//! - `MeasurementError` - Failed .MEAS verification (raised by
//!   `RunReport.assert_passed`)
//!
//! Lookup and argument failures additionally satisfy two contracts at once.
//! `RSpiceKeyError`, `RSpiceIndexError`, `RSpiceValueError`, and
//! `RSpiceTypeError` derive from *both* `RSpiceError` and the builtin Python
//! exception a caller already expects, so `except rspice.RSpiceError` catches
//! everything this library raises while `except KeyError` keeps working
//! unchanged. PyO3's `create_exception!` accepts a single base, so these are
//! built by calling Python's `type()` with a two-base tuple.

use pyo3::create_exception;
use pyo3::exceptions::{
    PyException, PyIndexError, PyKeyError, PyNotImplementedError, PyTypeError, PyValueError,
};
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyDict, PyTuple, PyType};

pub(crate) fn public_path_string(path: &std::path::Path) -> String {
    let path = path.to_string_lossy();
    #[cfg(windows)]
    {
        if let Some(rest) = path.strip_prefix(r"\\?\UNC\") {
            return format!(r"\\{rest}");
        }
        if let Some(rest) = path.strip_prefix(r"\\?\") {
            return rest.to_owned();
        }
    }
    path.into_owned()
}

#[cfg(all(test, windows))]
mod path_tests {
    use super::public_path_string;
    use std::path::Path;

    #[test]
    fn public_paths_hide_windows_verbatim_drive_prefixes() {
        assert_eq!(
            public_path_string(Path::new(r"\\?\C:\circuits\deck.cir")),
            r"C:\circuits\deck.cir"
        );
    }

    #[test]
    fn public_paths_convert_windows_verbatim_unc_prefixes() {
        assert_eq!(
            public_path_string(Path::new(r"\\?\UNC\server\share\deck.cir")),
            r"\\server\share\deck.cir"
        );
    }
}

mod parse;
mod simulation;

pub(crate) use parse::PyUnresolvedOutputSymbol;
pub use parse::parse_error_to_pyerr;
pub use simulation::simulation_error_to_pyerr;

// Base exception for all RSpice errors
create_exception!(
    rspice,
    RSpiceError,
    PyException,
    "Base exception for all RSpice errors."
);

// Netlist parsing errors
create_exception!(
    rspice,
    ParseError,
    RSpiceError,
    "Raised when netlist parsing fails due to syntax or semantic errors."
);

// General simulation errors
create_exception!(
    rspice,
    SimulationError,
    RSpiceError,
    "Raised when simulation fails due to circuit or solver errors."
);

// Convergence failures
create_exception!(
    rspice,
    ConvergenceError,
    SimulationError,
    "Raised when Newton-Raphson iteration fails to converge."
);

// Programmatic cancellation (Ctrl-C remains KeyboardInterrupt).
create_exception!(
    rspice,
    CancelledError,
    SimulationError,
    "Raised in a simulation's calling thread after Engine.cancel()."
);

// Measurement verification failures
create_exception!(
    rspice,
    MeasurementError,
    RSpiceError,
    "Raised when .MEAS verification fails (see RunReport.assert_passed)."
);

//=============================================================================
// Hybrid RSpiceError / builtin exception types
//=============================================================================

/// One of the four hybrid exception classes.
///
/// Each is an `RSpiceError` *and* the builtin exception Python callers
/// already catch for that failure mode, so neither contract has to be
/// traded away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HybridError {
    /// Unknown node, branch, device, or vector name.
    Key,
    /// Out-of-range node, time, sweep, or frequency index.
    Index,
    /// An argument that is the right type but an unusable value.
    Value,
    /// An argument of the wrong Python type.
    Type,
    /// An authored analysis this surface cannot execute yet.
    NotImplemented,
}

impl HybridError {
    fn class_name(self) -> &'static str {
        match self {
            HybridError::Key => "RSpiceKeyError",
            HybridError::Index => "RSpiceIndexError",
            HybridError::Value => "RSpiceValueError",
            HybridError::Type => "RSpiceTypeError",
            HybridError::NotImplemented => "RSpiceNotImplementedError",
        }
    }

    fn doc(self) -> &'static str {
        match self {
            HybridError::Key => {
                "Unknown node, branch, device, or vector name. Both an RSpiceError and a KeyError."
            }
            HybridError::Index => {
                "Out-of-range result index. Both an RSpiceError and an IndexError."
            }
            HybridError::Value => "Invalid argument value. Both an RSpiceError and a ValueError.",
            HybridError::Type => "Invalid argument type. Both an RSpiceError and a TypeError.",
            HybridError::NotImplemented => {
                "An authored analysis this surface cannot execute yet. \
                 Both an RSpiceError and a NotImplementedError."
            }
        }
    }

    fn cell(self) -> &'static PyOnceLock<Py<PyType>> {
        static KEY: PyOnceLock<Py<PyType>> = PyOnceLock::new();
        static INDEX: PyOnceLock<Py<PyType>> = PyOnceLock::new();
        static VALUE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
        static TYPE: PyOnceLock<Py<PyType>> = PyOnceLock::new();
        static NOT_IMPLEMENTED: PyOnceLock<Py<PyType>> = PyOnceLock::new();
        match self {
            HybridError::Key => &KEY,
            HybridError::Index => &INDEX,
            HybridError::Value => &VALUE,
            HybridError::Type => &TYPE,
            HybridError::NotImplemented => &NOT_IMPLEMENTED,
        }
    }

    fn builtin_base(self, py: Python<'_>) -> Bound<'_, PyType> {
        match self {
            HybridError::Key => py.get_type::<PyKeyError>(),
            HybridError::Index => py.get_type::<PyIndexError>(),
            HybridError::Value => py.get_type::<PyValueError>(),
            HybridError::Type => py.get_type::<PyTypeError>(),
            HybridError::NotImplemented => py.get_type::<PyNotImplementedError>(),
        }
    }

    /// Fall back to the plain builtin when class construction is unavailable,
    /// so a failure to build the hybrid type can never swallow the real error.
    fn fallback(self, message: String) -> PyErr {
        match self {
            HybridError::Key => PyKeyError::new_err(message),
            HybridError::Index => PyIndexError::new_err(message),
            HybridError::Value => PyValueError::new_err(message),
            HybridError::Type => PyTypeError::new_err(message),
            HybridError::NotImplemented => PyNotImplementedError::new_err(message),
        }
    }

    /// The class object, created on first use and cached for the process.
    pub(crate) fn class<'py>(self, py: Python<'py>) -> PyResult<Bound<'py, PyType>> {
        self.cell()
            .get_or_try_init(py, || {
                // `KeyError` and `IndexError` share `LookupError` as a base,
                // so a hybrid built from both would have an inconsistent MRO.
                // Each hybrid therefore takes exactly one builtin base.
                let bases =
                    PyTuple::new(py, [py.get_type::<RSpiceError>(), self.builtin_base(py)])?;
                let namespace = PyDict::new(py);
                namespace.set_item("__doc__", self.doc())?;
                namespace.set_item("__module__", "rspice")?;
                py.get_type::<PyType>()
                    .call1((self.class_name(), bases, namespace))?
                    .cast_into::<PyType>()
                    .map(Bound::unbind)
                    .map_err(PyErr::from)
            })
            .map(|class| class.bind(py).clone())
    }

    fn raise(self, message: String) -> PyErr {
        Python::attach(|py| match self.class(py) {
            Ok(class) => PyErr::from_type(class, (message,)),
            Err(_) => self.fallback(message),
        })
    }
}

/// Unknown node, branch, device, or vector name.
pub(crate) fn key_error(message: impl Into<String>) -> PyErr {
    HybridError::Key.raise(message.into())
}

/// Out-of-range result index.
pub(crate) fn index_error(message: impl Into<String>) -> PyErr {
    HybridError::Index.raise(message.into())
}

/// Invalid argument value.
pub(crate) fn value_error(message: impl Into<String>) -> PyErr {
    HybridError::Value.raise(message.into())
}

/// Invalid argument type.
pub(crate) fn type_error(message: impl Into<String>) -> PyErr {
    HybridError::Type.raise(message.into())
}

/// An authored analysis this surface has no execution route for.
pub(crate) fn not_implemented_error(message: impl Into<String>) -> PyErr {
    HybridError::NotImplemented.raise(message.into())
}

/// Register the hybrid classes on the module.
///
/// They are created eagerly so `rspice.RSpiceKeyError` is importable and
/// `__all__` stays truthful even before any error has been raised.
pub(crate) fn register_hybrid_errors(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = module.py();
    for kind in [
        HybridError::Key,
        HybridError::Index,
        HybridError::Value,
        HybridError::Type,
        HybridError::NotImplemented,
    ] {
        module.add(kind.class_name(), kind.class(py)?)?;
    }
    Ok(())
}

/// Render an exception as `Type: message` for embedding in a report record.
pub(crate) fn describe_pyerr(py: Python<'_>, error: &PyErr) -> String {
    let name = error
        .get_type(py)
        .name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "Error".to_string());
    let value = error.value(py).str().map_or_else(
        |_| error.to_string(),
        |text| text.to_string_lossy().into_owned(),
    );
    if value.is_empty() {
        name
    } else {
        format!("{name}: {value}")
    }
}
