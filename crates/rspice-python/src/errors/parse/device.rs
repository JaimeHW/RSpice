//! Attribute projections for the device family: model resolution, mutual
//! inductor references, and device initial conditions.

use super::ParseErrorAttributes;

pub(super) fn missing_device_model_attributes(
    error: &rspice_core::netlist::MissingDeviceModelError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("missing_device_model");
    attributes.category = Some("device_model_resolution");
    attributes.line = Some(error.line);
    attributes.primary_line = Some(error.line);
    attributes.authored_name = Some(error.device_name.clone());
    attributes.canonical_name = Some(error.canonical_device_name.clone());
    attributes.device = Some(error.device_name.clone());
    attributes.device_type = Some(error.device_type.clone());
    attributes
}

pub(super) fn undefined_mutual_inductor_reference_attributes(
    error: &rspice_core::netlist::UndefinedMutualInductorReferenceError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("undefined_mutual_inductor_reference");
    attributes.category = Some("mutual_inductor_reference");
    attributes.set_primary(&error.origin);
    attributes.detail = Some(error.authored_inductor_name.clone());
    attributes.authored_name = Some(error.authored_coupling_name.clone());
    attributes.canonical_name = Some(error.canonical_coupling_name.clone());
    attributes.qualified_name = Some(error.qualified_coupling_name.clone());
    attributes.authored_coupling_name = Some(error.authored_coupling_name.clone());
    attributes.canonical_coupling_name = Some(error.canonical_coupling_name.clone());
    attributes.qualified_coupling_name = Some(error.qualified_coupling_name.clone());
    attributes.authored_inductor_name = Some(error.authored_inductor_name.clone());
    attributes.canonical_inductor_name = Some(error.canonical_inductor_name.clone());
    attributes.qualified_inductor_name = Some(error.qualified_inductor_name.clone());
    attributes.scope_name = error.scope_name.clone();
    attributes.reference_position = Some(error.reference_position);
    attributes
}

pub(super) fn device_initial_condition_attributes(
    error: &rspice_core::netlist::DeviceInitialConditionError,
) -> ParseErrorAttributes {
    use rspice_core::netlist::DeviceInitialConditionError;

    let mut attributes = ParseErrorAttributes::new("device_initial_condition");
    attributes.category = Some("device_initial_condition");
    match error {
        DeviceInitialConditionError::DuplicateDirective { first, duplicate } => {
            attributes.kind = "device_initial_condition_duplicate_directive";
            attributes.set_primary(duplicate);
            attributes.set_related(first);
        }
        DeviceInitialConditionError::MissingInformation { origin } => {
            attributes.kind = "device_initial_condition_missing_information";
            attributes.set_primary(origin);
        }
        DeviceInitialConditionError::MalformedDirective { origin, detail } => {
            attributes.kind = "device_initial_condition_malformed_directive";
            attributes.set_primary(origin);
            attributes.detail = Some(detail.clone());
        }
        DeviceInitialConditionError::SourceUnavailable {
            origin,
            requested_path,
        } => {
            attributes.kind = "device_initial_condition_source_unavailable";
            attributes.set_primary(origin);
            attributes.requested_path = Some(requested_path.clone());
        }
        DeviceInitialConditionError::MalformedSource {
            origin,
            requested_path,
            record_origin,
            detail,
        } => {
            attributes.kind = "device_initial_condition_malformed_source";
            attributes.set_primary(record_origin);
            attributes.set_related(origin);
            attributes.requested_path = Some(requested_path.clone());
            attributes.detail = Some(detail.clone());
        }
        DeviceInitialConditionError::NonFiniteValue {
            origin,
            device,
            value_index,
            value,
        } => {
            attributes.kind = "device_initial_condition_nonfinite_value";
            attributes.set_primary(origin);
            attributes.device = Some(device.clone());
            attributes.value_index = Some(*value_index);
            attributes.value = Some(*value);
        }
        DeviceInitialConditionError::UnresolvedSource {
            origin,
            requested_path,
        } => {
            attributes.kind = "device_initial_condition_unresolved_source";
            attributes.set_primary(origin);
            attributes.requested_path = Some(requested_path.clone());
        }
        DeviceInitialConditionError::InvalidArity {
            origin,
            device,
            expected,
            actual,
        } => {
            attributes.kind = "device_initial_condition_invalid_arity";
            attributes.set_primary(origin);
            attributes.device = Some(device.clone());
            attributes.expected = Some(expected.clone());
            attributes.actual = Some(*actual);
        }
        DeviceInitialConditionError::UnsupportedTarget {
            origin,
            device,
            device_type,
        } => {
            attributes.kind = "device_initial_condition_unsupported_target";
            attributes.set_primary(origin);
            attributes.device = Some(device.clone());
            attributes.device_type = Some(device_type.clone());
        }
    }
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::{
        MissingDeviceModelError, NetlistSourceLocation, UndefinedMutualInductorReferenceError,
    };

    #[test]
    fn missing_device_model_exposes_structured_python_attributes() {
        let attributes = missing_device_model_attributes(&MissingDeviceModelError {
            line: 4,
            device_name: "d1".into(),
            canonical_device_name: "D1".into(),
            device_type: "DIODE".into(),
        });

        assert_eq!(attributes.kind, "missing_device_model");
        assert_eq!(attributes.category, Some("device_model_resolution"));
        assert_eq!(attributes.line, Some(4));
        assert_eq!(attributes.device.as_deref(), Some("d1"));
        assert_eq!(attributes.canonical_name.as_deref(), Some("D1"));
        assert_eq!(attributes.device_type.as_deref(), Some("DIODE"));
    }

    #[test]
    fn missing_device_model_is_declared_by_the_public_type_stub() {
        let stub = include_str!("../../../rspice.pyi");
        for declaration in ["\"missing_device_model\"", "\"device_model_resolution\""] {
            assert!(
                stub.contains(declaration),
                "public Python type stub is missing {declaration:?}"
            );
        }
    }

    #[test]
    fn undefined_mutual_inductor_exposes_structured_python_attributes() {
        let error = UndefinedMutualInductorReferenceError {
            origin: NetlistSourceLocation::in_file("bug75.cir", 12),
            authored_coupling_name: "K3".into(),
            canonical_coupling_name: "K3".into(),
            qualified_coupling_name: "K3".into(),
            authored_inductor_name: "L2".into(),
            canonical_inductor_name: "L2".into(),
            qualified_inductor_name: "L2".into(),
            scope_name: None,
            reference_position: 2,
        };
        let attributes = undefined_mutual_inductor_reference_attributes(&error);

        assert_eq!(attributes.kind, "undefined_mutual_inductor_reference");
        assert_eq!(attributes.category, Some("mutual_inductor_reference"));
        assert_eq!(attributes.line, Some(12));
        assert_eq!(attributes.source.as_deref(), Some("bug75.cir"));
        assert_eq!(attributes.authored_coupling_name.as_deref(), Some("K3"));
        assert_eq!(attributes.authored_inductor_name.as_deref(), Some("L2"));
        assert_eq!(attributes.reference_position, Some(2));
    }
}
