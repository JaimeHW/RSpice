//! The `Element` projection over a parsed netlist element.
//!
//! Exposes name, kind, nodes, value, value expression, model, and instance
//! parameters, so a caller can walk a deck's devices without string-editing
//! the source text.

use super::*;

/// One device instance from the parsed netlist.
///
/// This is a read-only projection of the parsed element, not a handle into
/// the netlist: mutating it changes nothing. Use `Netlist.with_parameters`
/// to produce a modified netlist.
#[pyclass(name = "Element", module = "rspice", frozen, from_py_object)]
#[derive(Debug, Clone, PartialEq)]
pub struct PyElement {
    /// Instance name as authored, for example `R1` or `Xamp`.
    #[pyo3(get)]
    pub name: String,
    /// Device family, for example `Resistor`, `Mosfet`, or `Subcircuit`.
    #[pyo3(get)]
    pub kind: String,
    /// Connected nodes in the order the instance line declared them.
    #[pyo3(get)]
    pub nodes: Vec<String>,
    /// Resolved primary value for elements that carry one (R, C, L).
    #[pyo3(get)]
    pub value: Option<f64>,
    /// Unevaluated `{...}` expression behind `value`, when the deck used one.
    #[pyo3(get)]
    pub value_expr: Option<String>,
    /// `.MODEL` name this instance references, when it references one.
    #[pyo3(get)]
    pub model: Option<String>,
    instance_params: Vec<(String, f64)>,
}

#[pymethods]
impl PyElement {
    /// Resolved instance parameters (for example `W`, `L`, `TEMP`).
    #[getter]
    fn instance_params<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let result = PyDict::new(py);
        for (name, value) in &self.instance_params {
            result.set_item(name, value)?;
        }
        Ok(result)
    }

    fn __repr__(&self) -> String {
        format!(
            "Element(name='{}', kind='{}', nodes={:?})",
            self.name, self.kind, self.nodes
        )
    }
}

/// Device-family name for an element.
///
/// Taken from the Rust variant name rather than a hand-written table: the
/// variant set is core's public device vocabulary and grows with every new
/// device, and a mapping here would silently fall out of date or force this
/// crate to be edited for every core addition.
pub(super) fn element_kind_name(kind: &rspice_core::netlist::ElementKind) -> String {
    let rendered = format!("{kind:?}");
    rendered
        .split(|character: char| !character.is_ascii_alphanumeric())
        .next()
        .unwrap_or("Unknown")
        .to_string()
}

/// Project a parsed element onto the read-only Python view.
pub(super) fn describe_element(element: &rspice_core::netlist::Element) -> PyElement {
    use rspice_core::netlist::ElementKind;

    let (value, value_expr, model, instance_params) = match &element.kind {
        ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        }
        | ElementKind::Capacitor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        }
        | ElementKind::Inductor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } => (
            Some(*value),
            value_expr.clone(),
            model.clone(),
            instance_params.clone(),
        ),
        ElementKind::JilesAthertonInductor { value, model, .. } => {
            (Some(*value), None, Some(model.clone()), Vec::new())
        }
        ElementKind::Diode {
            model,
            instance_params,
            ..
        }
        | ElementKind::Bjt {
            model,
            instance_params,
            ..
        }
        | ElementKind::Mosfet {
            model,
            instance_params,
            ..
        }
        | ElementKind::Jfet {
            model,
            instance_params,
            ..
        }
        | ElementKind::Mesfet {
            model,
            instance_params,
            ..
        }
        | ElementKind::XyceMemristor {
            model,
            instance_params,
            ..
        } => (None, None, Some(model.clone()), instance_params.clone()),
        _ => (None, None, None, Vec::new()),
    };

    PyElement {
        name: element.name.clone(),
        kind: element_kind_name(&element.kind),
        nodes: element.nodes.clone(),
        value,
        value_expr,
        model,
        instance_params,
    }
}
