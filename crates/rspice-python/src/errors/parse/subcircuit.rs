//! Attribute projections for the subcircuit family: port binding, parameter
//! resolution, and definition lookup.

use super::ParseErrorAttributes;

pub(super) fn duplicate_subcircuit_binding_attributes(
    error: &rspice_core::netlist::DuplicateSubcircuitPortBindingError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("duplicate_subcircuit_port_binding");
    attributes.category = Some("subcircuit_binding");
    attributes.detail = Some(error.formal_port.clone());
    attributes.authored_name = Some(error.instance_name.clone());
    attributes.canonical_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_name = Some(error.qualified_instance_name.clone());
    attributes.subcircuit_name = Some(error.subcircuit_name.clone());
    attributes.canonical_subcircuit_name = Some(error.canonical_subcircuit_name.clone());
    attributes.instance_name = Some(error.instance_name.clone());
    attributes.canonical_instance_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_instance_name = Some(error.qualified_instance_name.clone());
    attributes.formal_port = Some(error.formal_port.clone());
    attributes.first_position = Some(error.first_position);
    attributes.conflicting_position = Some(error.conflicting_position);
    attributes.first_actual_node = Some(error.first_actual_node.clone());
    attributes.conflicting_actual_node = Some(error.conflicting_actual_node.clone());
    attributes
}

pub(super) fn global_subcircuit_binding_attributes(
    error: &rspice_core::netlist::GlobalSubcircuitPortBindingError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("global_subcircuit_port_binding");
    attributes.category = Some("subcircuit_binding");
    attributes.detail = Some(error.formal_port.clone());
    attributes.authored_name = Some(error.instance_name.clone());
    attributes.canonical_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_name = Some(error.qualified_instance_name.clone());
    attributes.subcircuit_name = Some(error.subcircuit_name.clone());
    attributes.canonical_subcircuit_name = Some(error.canonical_subcircuit_name.clone());
    attributes.instance_name = Some(error.instance_name.clone());
    attributes.canonical_instance_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_instance_name = Some(error.qualified_instance_name.clone());
    attributes.formal_port = Some(error.formal_port.clone());
    attributes.position = Some(error.position);
    attributes.actual_node = Some(error.actual_node.clone());
    attributes
}

pub(super) fn unresolved_subcircuit_parameter_attributes(
    error: &rspice_core::netlist::UnresolvedSubcircuitParameterError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("unresolved_subcircuit_parameter");
    attributes.category = Some("subcircuit_parameter_resolution");
    attributes.detail = Some(error.reason.clone());
    attributes.authored_name = Some(error.parameter_name.clone());
    attributes.canonical_name = Some(error.canonical_parameter_name.clone());
    attributes.qualified_name = Some(error.qualified_instance_name.clone());
    attributes.subcircuit_name = Some(error.subcircuit_name.clone());
    attributes.canonical_subcircuit_name = Some(error.canonical_subcircuit_name.clone());
    attributes.instance_name = Some(error.instance_name.clone());
    attributes.canonical_instance_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_instance_name = Some(error.qualified_instance_name.clone());
    attributes.parameter_name = Some(error.parameter_name.clone());
    attributes.canonical_parameter_name = Some(error.canonical_parameter_name.clone());
    attributes.expression = Some(error.expression.clone());
    attributes.missing_dependency = error.missing_dependency.clone();
    attributes.reason = Some(error.reason.clone());
    attributes
}

pub(super) fn undefined_subcircuit_attributes(
    error: &rspice_core::netlist::UndefinedSubcircuitError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("undefined_subcircuit");
    attributes.category = Some("subcircuit_resolution");
    attributes.detail = Some(error.subcircuit_name.clone());
    attributes.authored_name = Some(error.instance_name.clone());
    attributes.canonical_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_name = Some(error.qualified_instance_name.clone());
    attributes.subcircuit_name = Some(error.subcircuit_name.clone());
    attributes.canonical_subcircuit_name = Some(error.canonical_subcircuit_name.clone());
    attributes.instance_name = Some(error.instance_name.clone());
    attributes.canonical_instance_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_instance_name = Some(error.qualified_instance_name.clone());
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::{
        DuplicateSubcircuitPortBindingError, GlobalSubcircuitPortBindingError,
        UndefinedSubcircuitError, UnresolvedSubcircuitParameterError,
    };

    #[test]
    fn duplicate_subcircuit_binding_exposes_structured_python_attributes() {
        let error = DuplicateSubcircuitPortBindingError {
            subcircuit_name: "inv1".into(),
            canonical_subcircuit_name: "INV1".into(),
            instance_name: "Xinv1".into(),
            canonical_instance_name: "XINV1".into(),
            qualified_instance_name: "TOP.Xinv1".into(),
            formal_port: "GND".into(),
            first_position: 4,
            conflicting_position: 8,
            first_actual_node: "0".into(),
            conflicting_actual_node: "VDD".into(),
        };
        let attributes = duplicate_subcircuit_binding_attributes(&error);

        assert_eq!(attributes.kind, "duplicate_subcircuit_port_binding");
        assert_eq!(attributes.category, Some("subcircuit_binding"));
        assert_eq!(attributes.subcircuit_name.as_deref(), Some("inv1"));
        assert_eq!(
            attributes.canonical_subcircuit_name.as_deref(),
            Some("INV1")
        );
        assert_eq!(attributes.canonical_instance_name.as_deref(), Some("XINV1"));
        assert_eq!(attributes.first_position, Some(4));
        assert_eq!(attributes.conflicting_actual_node.as_deref(), Some("VDD"));
    }

    #[test]
    fn global_subcircuit_binding_exposes_structured_python_attributes() {
        let error = GlobalSubcircuitPortBindingError {
            subcircuit_name: "cell".into(),
            canonical_subcircuit_name: "CELL".into(),
            instance_name: "X1".into(),
            canonical_instance_name: "X1".into(),
            qualified_instance_name: "TOP.X1".into(),
            formal_port: "$G_SHARED".into(),
            position: 1,
            actual_node: "LOCAL".into(),
        };
        let attributes = global_subcircuit_binding_attributes(&error);

        assert_eq!(attributes.kind, "global_subcircuit_port_binding");
        assert_eq!(attributes.subcircuit_name.as_deref(), Some("cell"));
        assert_eq!(
            attributes.canonical_subcircuit_name.as_deref(),
            Some("CELL")
        );
        assert_eq!(attributes.formal_port.as_deref(), Some("$G_SHARED"));
        assert_eq!(attributes.actual_node.as_deref(), Some("LOCAL"));
    }

    #[test]
    fn unresolved_subcircuit_parameter_exposes_structured_python_attributes() {
        let attributes =
            unresolved_subcircuit_parameter_attributes(&UnresolvedSubcircuitParameterError {
                subcircuit_name: "cell".into(),
                canonical_subcircuit_name: "CELL".into(),
                instance_name: "x1".into(),
                canonical_instance_name: "X1".into(),
                qualified_instance_name: "TOP.X1".into(),
                parameter_name: "foo".into(),
                canonical_parameter_name: "FOO".into(),
                expression: "TIME + meh".into(),
                missing_dependency: Some("MEH".into()),
                reason: "Undefined parameter: MEH".into(),
            });

        assert_eq!(attributes.kind, "unresolved_subcircuit_parameter");
        assert_eq!(attributes.category, Some("subcircuit_parameter_resolution"));
        assert_eq!(attributes.subcircuit_name.as_deref(), Some("cell"));
        assert_eq!(
            attributes.qualified_instance_name.as_deref(),
            Some("TOP.X1")
        );
        assert_eq!(attributes.parameter_name.as_deref(), Some("foo"));
        assert_eq!(attributes.canonical_parameter_name.as_deref(), Some("FOO"));
        assert_eq!(attributes.expression.as_deref(), Some("TIME + meh"));
        assert_eq!(attributes.missing_dependency.as_deref(), Some("MEH"));
        assert_eq!(
            attributes.reason.as_deref(),
            Some("Undefined parameter: MEH")
        );
    }

    #[test]
    fn unresolved_subcircuit_parameter_is_declared_by_the_public_type_stub() {
        let stub = include_str!("../../../rspice.pyi");
        for declaration in [
            "\"unresolved_subcircuit_parameter\"",
            "\"subcircuit_parameter_resolution\"",
            "parameter_name: str | None",
            "canonical_parameter_name: str | None",
            "expression: str | None",
            "missing_dependency: str | None",
            "reason: str | None",
        ] {
            assert!(
                stub.contains(declaration),
                "public Python type stub is missing {declaration:?}"
            );
        }
    }

    #[test]
    fn undefined_subcircuit_exposes_structured_python_attributes() {
        let attributes = undefined_subcircuit_attributes(&UndefinedSubcircuitError {
            subcircuit_name: "missing".into(),
            canonical_subcircuit_name: "MISSING".into(),
            instance_name: "x1".into(),
            canonical_instance_name: "X1".into(),
            qualified_instance_name: "TOP.X1".into(),
        });

        assert_eq!(attributes.kind, "undefined_subcircuit");
        assert_eq!(attributes.category, Some("subcircuit_resolution"));
        assert_eq!(attributes.subcircuit_name.as_deref(), Some("missing"));
        assert_eq!(
            attributes.canonical_subcircuit_name.as_deref(),
            Some("MISSING")
        );
        assert_eq!(attributes.instance_name.as_deref(), Some("x1"));
        assert_eq!(attributes.canonical_instance_name.as_deref(), Some("X1"));
        assert_eq!(
            attributes.qualified_instance_name.as_deref(),
            Some("TOP.X1")
        );
    }

    #[test]
    fn undefined_subcircuit_is_declared_by_the_public_type_stub() {
        let stub = include_str!("../../../rspice.pyi");
        for declaration in ["\"undefined_subcircuit\"", "\"subcircuit_resolution\""] {
            assert!(
                stub.contains(declaration),
                "public Python type stub is missing {declaration:?}"
            );
        }
    }
}
