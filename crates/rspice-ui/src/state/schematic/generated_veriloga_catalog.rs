//! Data-driven schematic bindings for build-time generated Verilog-A models.
//!
//! The compiled engine registry is the only catalog authority. Projects keep
//! a frozen identity/signature so a generator or source update cannot silently
//! reinterpret old terminals or parameters.

use rspice_core::device::veriloga_builtins::{
    GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION, GeneratedVerilogAModelDescriptor,
    GeneratedVerilogATerminalDirection, generated_veriloga_model_descriptor,
    generated_veriloga_model_descriptors,
};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

use super::{GeneratedVerilogAInstance, LibraryCellInstance, PortDirection, PortSpec};

pub const GENERATED_VERILOGA_BINDING_SCHEMA_REVISION: u32 = 1;

pub fn generated_veriloga_devices() -> &'static [GeneratedVerilogAModelDescriptor] {
    generated_veriloga_model_descriptors()
}

pub fn generated_veriloga_library_binding(
    descriptor: &GeneratedVerilogAModelDescriptor,
) -> Result<LibraryCellInstance, String> {
    let authoritative =
        generated_veriloga_model_descriptor(descriptor.model_name).ok_or_else(|| {
            format!(
                "generated Verilog-A model '{}' is not compiled into this build",
                descriptor.model_name
            )
        })?;
    if authoritative != descriptor {
        return Err(format!(
            "generated Verilog-A descriptor '{}' is not the authoritative compiled contract",
            descriptor.model_name
        ));
    }
    validate_generated_veriloga_descriptor(descriptor)?;

    let ports = descriptor
        .terminals
        .iter()
        .map(|terminal| PortSpec {
            name: terminal.name.to_owned(),
            direction: terminal_direction(terminal.direction),
        })
        .collect::<Vec<_>>();
    let mut binding =
        LibraryCellInstance::new("rspice", descriptor.model_name, "veriloga-generated");
    binding.module_name = Some(descriptor.model_name.to_owned());
    binding.reference_prefix = Some("X".to_owned());
    binding.parameter_order = descriptor
        .parameters
        .iter()
        .map(|parameter| parameter.name.to_owned())
        .collect();
    binding.bind_interface(&ports);
    binding.generated_veriloga = Some(GeneratedVerilogAInstance {
        schema_revision: GENERATED_VERILOGA_BINDING_SCHEMA_REVISION,
        stable_id: generated_veriloga_stable_id(descriptor),
        descriptor_abi_version: descriptor.abi_version,
        model_name: descriptor.model_name.to_owned(),
        module_name: descriptor.module_name.to_owned(),
        source_digest: descriptor.source_digest.to_owned(),
        checkpoint_identity: descriptor.checkpoint_identity.to_owned(),
        descriptor_signature: generated_veriloga_descriptor_signature(descriptor),
    });
    Ok(binding)
}

pub fn validate_generated_veriloga_descriptor(
    descriptor: &GeneratedVerilogAModelDescriptor,
) -> Result<(), String> {
    if descriptor.abi_version != GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION {
        return Err(format!(
            "generated Verilog-A model '{}' uses descriptor ABI {}, expected {}",
            descriptor.model_name,
            descriptor.abi_version,
            GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION
        ));
    }
    if descriptor.model_name.trim().is_empty() || descriptor.module_name.trim().is_empty() {
        return Err("generated Verilog-A descriptor has an empty model/module identity".to_owned());
    }
    if !is_hex_digest(descriptor.source_digest, 16) {
        return Err(format!(
            "generated Verilog-A model '{}' has an invalid canonical source fingerprint",
            descriptor.model_name
        ));
    }
    if !is_hex_digest(descriptor.checkpoint_identity, 64) {
        return Err(format!(
            "generated Verilog-A model '{}' has an invalid checkpoint identity",
            descriptor.model_name
        ));
    }
    if descriptor.terminals.is_empty() {
        return Err(format!(
            "generated Verilog-A model '{}' has no external terminals",
            descriptor.model_name
        ));
    }
    if descriptor.total_node_count
        != descriptor.terminals.len() + descriptor.internal_node_names.len()
    {
        return Err(format!(
            "generated Verilog-A model '{}' has inconsistent external/internal node metadata",
            descriptor.model_name
        ));
    }
    let mut node_names = HashSet::new();
    let mut terminal_current_parameters = HashSet::new();
    for terminal in descriptor.terminals {
        if terminal.name.trim().is_empty()
            || terminal.discipline.trim().is_empty()
            || !node_names.insert(terminal.name.to_ascii_lowercase())
            || terminal.current_parameter.trim().is_empty()
            || !terminal_current_parameters.insert(terminal.current_parameter.to_ascii_lowercase())
        {
            return Err(format!(
                "generated Verilog-A model '{}' has invalid or duplicate terminal metadata",
                descriptor.model_name
            ));
        }
    }
    for internal_node in descriptor.internal_node_names {
        if internal_node.trim().is_empty() || !node_names.insert(internal_node.to_ascii_lowercase())
        {
            return Err(format!(
                "generated Verilog-A model '{}' has invalid or duplicate internal-node metadata",
                descriptor.model_name
            ));
        }
    }
    let mut parameter_names = HashSet::new();
    for parameter in descriptor.parameters {
        if parameter.name.trim().is_empty()
            || !parameter_names.insert(parameter.name.to_ascii_lowercase())
        {
            return Err(format!(
                "generated Verilog-A model '{}' has an invalid, duplicate, or reserved parameter name",
                descriptor.model_name
            ));
        }
        for alias in parameter.aliases {
            if alias.trim().is_empty() || !parameter_names.insert(alias.to_ascii_lowercase()) {
                return Err(format!(
                    "generated Verilog-A model '{}' has duplicate parameter alias '{}'",
                    descriptor.model_name, alias
                ));
            }
        }
        if parameter.default.is_some_and(|value| !value.is_finite())
            || parameter
                .minimum
                .is_some_and(|bound| !bound.value.is_finite())
            || parameter
                .maximum
                .is_some_and(|bound| !bound.value.is_finite())
            || parameter
                .excluded_values
                .iter()
                .any(|value| !value.is_finite())
        {
            return Err(format!(
                "generated Verilog-A model '{}' parameter '{}' has non-finite metadata",
                descriptor.model_name, parameter.name
            ));
        }
        if let (Some(minimum), Some(maximum)) = (parameter.minimum, parameter.maximum)
            && (minimum.value > maximum.value
                || (minimum.value == maximum.value && (minimum.exclusive || maximum.exclusive)))
        {
            return Err(format!(
                "generated Verilog-A model '{}' parameter '{}' has an empty static range",
                descriptor.model_name, parameter.name
            ));
        }
    }
    Ok(())
}

pub fn validate_generated_veriloga_binding(
    binding: &LibraryCellInstance,
) -> Result<&'static GeneratedVerilogAModelDescriptor, String> {
    let contract = binding
        .generated_veriloga
        .as_ref()
        .ok_or_else(|| "binding is not a build-time generated Verilog-A device".to_owned())?;
    let descriptor =
        generated_veriloga_model_descriptor(&contract.model_name).ok_or_else(|| {
            format!(
                "generated Verilog-A model '{}' is unavailable in this build",
                contract.model_name
            )
        })?;
    let expected = generated_veriloga_library_binding(descriptor)?;
    let expected_contract = expected
        .generated_veriloga
        .as_ref()
        .expect("generated catalog builder always installs its contract");
    if contract != expected_contract {
        return Err(format!(
            "generated Verilog-A device '{}' has a stale or modified executable contract",
            contract.stable_id
        ));
    }
    if !binding.library.eq_ignore_ascii_case(&expected.library)
        || !binding.cell.eq_ignore_ascii_case(&expected.cell)
        || !binding.view.eq_ignore_ascii_case(&expected.view)
        || binding.source_path.is_some()
        || binding.netlist_template.is_some()
        || binding.model_section.is_some()
        || binding.module_name != expected.module_name
        || binding.reference_prefix != expected.reference_prefix
        || binding.parameter_order != expected.parameter_order
        || binding.terminal_order != expected.terminal_order
        || binding.terminal_dirs != expected.terminal_dirs
        || binding.interface_bound != expected.interface_bound
        || binding.builtin_xspice.is_some()
    {
        return Err(format!(
            "generated Verilog-A device '{}' has modified binding metadata",
            contract.stable_id
        ));
    }
    Ok(descriptor)
}

fn generated_veriloga_stable_id(descriptor: &GeneratedVerilogAModelDescriptor) -> String {
    let model = descriptor
        .model_name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    let digest = descriptor
        .source_digest
        .get(..16)
        .unwrap_or(descriptor.source_digest);
    format!("rspice.veriloga.generated.{model}.{digest}")
}

fn generated_veriloga_descriptor_signature(
    descriptor: &GeneratedVerilogAModelDescriptor,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-generated-veriloga-descriptor-signature-v2\0");
    hash_u64(&mut hasher, u64::from(descriptor.abi_version));
    hash_text(&mut hasher, descriptor.model_name);
    hash_text(&mut hasher, descriptor.module_name);
    hash_text(&mut hasher, descriptor.source_digest);
    hash_text(&mut hasher, descriptor.checkpoint_identity);
    hash_u64(&mut hasher, descriptor.total_node_count as u64);
    hash_u64(&mut hasher, descriptor.branch_count as u64);
    hash_u64(&mut hasher, descriptor.terminals.len() as u64);
    for terminal in descriptor.terminals {
        hash_text(&mut hasher, terminal.name);
        hasher.update([match terminal.direction {
            GeneratedVerilogATerminalDirection::Input => 0,
            GeneratedVerilogATerminalDirection::Output => 1,
            GeneratedVerilogATerminalDirection::InOut => 2,
        }]);
        hash_text(&mut hasher, terminal.discipline);
        hash_text(&mut hasher, terminal.current_parameter);
    }
    hash_u64(&mut hasher, descriptor.internal_node_names.len() as u64);
    for internal_node in descriptor.internal_node_names {
        hash_text(&mut hasher, internal_node);
    }
    hash_u64(&mut hasher, descriptor.parameters.len() as u64);
    for parameter in descriptor.parameters {
        hash_text(&mut hasher, parameter.name);
        hasher.update([match parameter.scope {
            rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterScope::Model => 0,
            rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterScope::Instance => 1,
            rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterScope::Dual => 2,
        }]);
        hasher.update([u8::from(parameter.is_integer)]);
        hash_optional_number(&mut hasher, parameter.default);
        hash_optional_bound(&mut hasher, parameter.minimum);
        hash_optional_bound(&mut hasher, parameter.maximum);
        hash_u64(&mut hasher, parameter.aliases.len() as u64);
        for alias in parameter.aliases {
            hash_text(&mut hasher, alias);
        }
        hash_u64(&mut hasher, parameter.excluded_values.len() as u64);
        for excluded in parameter.excluded_values {
            hasher.update(excluded.to_bits().to_le_bytes());
        }
        hasher.update([u8::from(parameter.has_dynamic_constraints)]);
    }
    format!("{:x}", hasher.finalize())
}

fn is_hex_digest(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn hash_u64(hasher: &mut Sha256, value: u64) {
    hasher.update(value.to_le_bytes());
}

fn hash_text(hasher: &mut Sha256, value: &str) {
    hash_u64(hasher, value.len() as u64);
    hasher.update(value.as_bytes());
}

fn hash_optional_number(hasher: &mut Sha256, value: Option<f64>) {
    match value {
        Some(value) => {
            hasher.update([1]);
            hasher.update(value.to_bits().to_le_bytes());
        }
        None => hasher.update([0]),
    }
}

fn hash_optional_bound(
    hasher: &mut Sha256,
    bound: Option<rspice_core::device::veriloga_builtins::GeneratedVerilogAParameterBound>,
) {
    match bound {
        Some(bound) => {
            hasher.update([1]);
            hasher.update(bound.value.to_bits().to_le_bytes());
            hasher.update([u8::from(bound.exclusive)]);
        }
        None => hasher.update([0]),
    }
}

const fn terminal_direction(direction: GeneratedVerilogATerminalDirection) -> PortDirection {
    match direction {
        GeneratedVerilogATerminalDirection::Input => PortDirection::In,
        GeneratedVerilogATerminalDirection::Output => PortDirection::Out,
        GeneratedVerilogATerminalDirection::InOut => PortDirection::InOut,
    }
}

#[cfg(test)]
mod descriptor_v2_tests {
    use super::*;
    use rspice_core::device::veriloga_builtins::GeneratedVerilogATerminalDescriptor;

    #[test]
    fn descriptor_signature_covers_terminal_current_identity() {
        const TERMINALS_A: [GeneratedVerilogATerminalDescriptor; 1] =
            [GeneratedVerilogATerminalDescriptor {
                name: "p",
                direction: GeneratedVerilogATerminalDirection::InOut,
                discipline: "electrical",
                current_parameter: "ip",
            }];
        const TERMINALS_B: [GeneratedVerilogATerminalDescriptor; 1] =
            [GeneratedVerilogATerminalDescriptor {
                current_parameter: "ilead",
                ..TERMINALS_A[0]
            }];
        const BASE: GeneratedVerilogAModelDescriptor = GeneratedVerilogAModelDescriptor {
            abi_version: GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION,
            model_name: "signature_probe",
            module_name: "signature_probe",
            source_digest: "0123456789abcdef",
            checkpoint_identity: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            terminals: &TERMINALS_A,
            parameters: &[],
            total_node_count: 1,
            internal_node_names: &[],
            branch_count: 0,
        };
        const CHANGED: GeneratedVerilogAModelDescriptor = GeneratedVerilogAModelDescriptor {
            terminals: &TERMINALS_B,
            ..BASE
        };

        assert_ne!(
            generated_veriloga_descriptor_signature(&BASE),
            generated_veriloga_descriptor_signature(&CHANGED)
        );
    }
}

#[cfg(all(test, feature = "generated-veriloga-catalog"))]
mod tests {
    use super::*;

    const EXPECTED_SHIPPED_MODEL_COUNT: usize = 43;

    #[test]
    fn every_compiled_model_materializes_and_round_trips_exactly() {
        let descriptors = generated_veriloga_devices();
        assert_eq!(descriptors.len(), EXPECTED_SHIPPED_MODEL_COUNT);
        let mut stable_ids = HashSet::new();
        let mut signatures = HashSet::new();

        for descriptor in descriptors {
            validate_generated_veriloga_descriptor(descriptor)
                .unwrap_or_else(|error| panic!("{}: {error}", descriptor.model_name));
            let binding = generated_veriloga_library_binding(descriptor)
                .unwrap_or_else(|error| panic!("{}: {error}", descriptor.model_name));
            assert_eq!(binding.terminal_order.len(), descriptor.terminals.len());
            assert_eq!(binding.terminal_dirs.len(), descriptor.terminals.len());
            assert_eq!(binding.parameter_order.len(), descriptor.parameters.len());

            let contract = binding
                .generated_veriloga
                .as_ref()
                .expect("generated binding owns its frozen contract");
            assert!(stable_ids.insert(contract.stable_id.clone()));
            assert!(signatures.insert(contract.descriptor_signature.clone()));
            assert_eq!(contract.descriptor_signature.len(), 64);

            let encoded = serde_json::to_vec(&binding).expect("binding serializes");
            let decoded: LibraryCellInstance =
                serde_json::from_slice(&encoded).expect("binding deserializes");
            assert_eq!(decoded, binding);
            assert!(std::ptr::eq(
                validate_generated_veriloga_binding(&decoded).expect("round trip remains valid"),
                descriptor
            ));
        }
    }

    #[test]
    fn frozen_contract_rejects_identity_interface_and_parameter_tampering() {
        let descriptor = generated_veriloga_devices()
            .first()
            .expect("shipped catalog is not empty");
        let binding = generated_veriloga_library_binding(descriptor).expect("valid binding");

        let mut identity_tamper = binding.clone();
        identity_tamper
            .generated_veriloga
            .as_mut()
            .expect("generated contract")
            .source_digest
            .replace_range(..1, "0");
        assert!(validate_generated_veriloga_binding(&identity_tamper).is_err());

        let mut terminal_tamper = binding.clone();
        terminal_tamper.terminal_order[0].push_str("_modified");
        assert!(validate_generated_veriloga_binding(&terminal_tamper).is_err());

        let mut parameter_tamper = binding;
        parameter_tamper
            .parameter_order
            .push("not_a_parameter".to_owned());
        assert!(validate_generated_veriloga_binding(&parameter_tamper).is_err());
    }
}
