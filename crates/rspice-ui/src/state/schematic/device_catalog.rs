//! Built-in device catalog entries that are not represented by legacy
//! `ComponentType` variants.
//!
//! The simulation registry remains authoritative for executable XSPICE port
//! and parameter schemas.  This table owns only product identity, canonical
//! aliasing, display text, and schematic artwork.  Placement consumes both at
//! once and refuses an entry if the executable schema cannot be resolved.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use rspice_core::xspice::{
    CodeModelRegistry, ParamSpec as CoreParamSpec, PortDirection as CorePortDirection,
    PortSpec as CorePortSpec, PortType as CorePortType,
};

use super::{
    BuiltinXspiceInstance, BuiltinXspicePortBinding, BuiltinXspicePortDirection,
    BuiltinXspicePortType, LibraryCellInstance, PortDirection, PortSpec,
};

#[cfg(test)]
use super::ComponentType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogXspiceDeviceDescriptor {
    pub stable_id: &'static str,
    pub model_type: &'static str,
    pub aliases: &'static [&'static str],
    pub display_name: &'static str,
    pub symbol_asset: &'static str,
}

/// User-configurable width contract for one vector port on a catalog device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogXspiceVectorPort {
    pub name: String,
    pub minimum: usize,
    pub maximum: Option<usize>,
    pub default_width: usize,
    pub null_allowed: bool,
}

/// Product safety ceiling for unbounded registry vector ports. This prevents
/// malformed project data from requesting unbounded terminal and symbol
/// allocation while remaining far above practical schematic fan-out.
pub const MAX_BUILTIN_XSPICE_VECTOR_WIDTH: usize = 4096;

macro_rules! xspice_device {
    ($id:literal, $model:literal, $name:literal, $symbol:literal) => {
        CatalogXspiceDeviceDescriptor {
            stable_id: $id,
            model_type: $model,
            aliases: &[],
            display_name: $name,
            symbol_asset: $symbol,
        }
    };
    ($id:literal, $model:literal, [$($alias:literal),+ $(,)?], $name:literal, $symbol:literal) => {
        CatalogXspiceDeviceDescriptor {
            stable_id: $id,
            model_type: $model,
            aliases: &[$($alias),+],
            display_name: $name,
            symbol_asset: $symbol,
        }
    };
}

/// Canonical placeable XSPICE devices that do not have a legacy enum variant.
/// Aliases intentionally share the canonical symbol and placement contract.
pub const ENGINE_ONLY_XSPICE_DEVICES: &[CatalogXspiceDeviceDescriptor] = &[
    xspice_device!(
        "rspice.xspice.astate",
        "astate",
        "Analog State",
        "xspice_analog_state.svg"
    ),
    xspice_device!(
        "rspice.xspice.aswitch",
        "aswitch",
        "Analog Switch",
        "switch_voltage.svg"
    ),
    xspice_device!(
        "rspice.xspice.bidi_bridge",
        "bidi_bridge",
        "Bidirectional Bridge",
        "xspice_bidi_bridge.svg"
    ),
    xspice_device!(
        "rspice.xspice.capacitor",
        "capacitor",
        "XSPICE Capacitor",
        "cap_unpolarized.svg"
    ),
    xspice_device!(
        "rspice.xspice.capacitoric",
        "capacitoric",
        "XSPICE Capacitor with IC",
        "cap_unpolarized.svg"
    ),
    xspice_device!(
        "rspice.xspice.climit",
        "climit",
        "Controlled Limiter",
        "xspice_controlled_limiter.svg"
    ),
    xspice_device!(
        "rspice.xspice.cmeter",
        "cmeter",
        "Capacitance Meter",
        "xspice_capacitance_meter.svg"
    ),
    xspice_device!(
        "rspice.xspice.core",
        "core",
        "Magnetic Core",
        "xspice_magnetic_core.svg"
    ),
    xspice_device!(
        "rspice.xspice.cpline",
        "cpline",
        "Coupled Transmission Line",
        "tline_coupled.svg"
    ),
    xspice_device!(
        "rspice.xspice.cpmlin",
        "cpmlin",
        "Coupled Microstrip Line",
        "xspice_coupled_microstrip_line.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_cosim",
        "d_cosim",
        "Digital Co-Simulation",
        "xspice_digital_cosim.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_dlatch",
        "d_dlatch",
        ["xyce_d_dlatch"],
        "D Latch",
        "xspice_d_latch.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_fdiv",
        "d_fdiv",
        "Frequency Divider",
        "xspice_frequency_divider.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_lut",
        "d_lut",
        ["d_genlut"],
        "Digital Lookup Table",
        "xspice_lut.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_open_c",
        "d_open_c",
        "Open-Collector Buffer",
        "xspice_open_collector.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_open_e",
        "d_open_e",
        "Open-Emitter Buffer",
        "xspice_open_emitter.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_osc",
        "d_osc",
        "Digital Oscillator",
        "xspice_digital_oscillator.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_process",
        "d_process",
        "External Digital Process",
        "xspice_digital_process.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_pulldown",
        "d_pulldown",
        "Digital Pulldown",
        "xspice_pulldown.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_pullup",
        "d_pullup",
        "Digital Pullup",
        "xspice_pullup.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_pwm",
        "d_pwm",
        "PWM Oscillator",
        "xspice_pwm.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_ram",
        "d_ram",
        "Digital RAM",
        "xspice_ram.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_source",
        "d_source",
        "Digital File Source",
        "xspice_digital_source.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_srff",
        "d_srff",
        "SR Flip-Flop",
        "xspice_sr_flip_flop.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_state",
        "d_state",
        "Digital State Machine",
        "xspice_state_machine.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_tff",
        "d_tff",
        ["xyce_d_tff"],
        "T Flip-Flop",
        "xspice_tff.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_to_real",
        "d_to_real",
        "Digital-to-Real Bridge",
        "xspice_digital_to_real.svg"
    ),
    xspice_device!(
        "rspice.xspice.d_xnor",
        "d_xnor",
        ["xyce_d_xnor", "xyce_legacy_d_xnor"],
        "XNOR Gate",
        "xspice_xnor.svg"
    ),
    xspice_device!(
        "rspice.xspice.delay",
        "delay",
        "Analog Delay",
        "xspice_delay.svg"
    ),
    xspice_device!(
        "rspice.xspice.file_source",
        "file_source",
        ["filesource"],
        "Analog File Source",
        "xspice_file_source.svg"
    ),
    xspice_device!(
        "rspice.xspice.hyst",
        "hyst",
        "Hysteresis",
        "xspice_hysteresis.svg"
    ),
    xspice_device!(
        "rspice.xspice.ilimit",
        "ilimit",
        "Current-Limited Driver",
        "xspice_current_limiter.svg"
    ),
    xspice_device!(
        "rspice.xspice.inductor",
        "inductor",
        "XSPICE Inductor",
        "inductor.svg"
    ),
    xspice_device!(
        "rspice.xspice.inductoric",
        "inductoric",
        "XSPICE Inductor with IC",
        "inductor.svg"
    ),
    xspice_device!(
        "rspice.xspice.lcouple",
        "lcouple",
        "Inductor/Core Coupling",
        "xspice_lc_coupling.svg"
    ),
    xspice_device!(
        "rspice.xspice.lmeter",
        "lmeter",
        "Inductance Meter",
        "xspice_inductance_meter.svg"
    ),
    xspice_device!(
        "rspice.xspice.memristor",
        "memristor",
        "XSPICE Memristor",
        "memristor.svg"
    ),
    xspice_device!(
        "rspice.xspice.mlin",
        "mlin",
        "Microstrip Line",
        "xspice_microstrip_line.svg"
    ),
    xspice_device!(
        "rspice.xspice.msopen",
        "msopen",
        "Microstrip Open",
        "xspice_microstrip_open.svg"
    ),
    xspice_device!(
        "rspice.xspice.multi_input_pwl",
        "multi_input_pwl",
        "Multi-Input PWL",
        "xspice_multi_input_pwl.svg"
    ),
    xspice_device!(
        "rspice.xspice.nco",
        "nco",
        "Numerically Controlled Oscillator",
        "xspice_nco.svg"
    ),
    xspice_device!(
        "rspice.xspice.oneshot",
        "oneshot",
        "One Shot",
        "xspice_one_shot.svg"
    ),
    xspice_device!(
        "rspice.xspice.potentiometer",
        "potentiometer",
        "Potentiometer",
        "xspice_potentiometer.svg"
    ),
    xspice_device!(
        "rspice.xspice.pspice_d_stim",
        "pspice_d_stim",
        "PSpice Digital Stimulus",
        "xspice_digital_source.svg"
    ),
    xspice_device!(
        "rspice.xspice.pswitch",
        "pswitch",
        "P-Switch",
        "switch_voltage.svg"
    ),
    xspice_device!("rspice.xspice.pwl", "pwl", "PWL Transfer", "xspice_pwl.svg"),
    xspice_device!(
        "rspice.xspice.pwlts",
        "pwlts",
        "PWL Time-Series Source",
        "xspice_pwlts.svg"
    ),
    xspice_device!(
        "rspice.xspice.real_delay",
        "real_delay",
        "Real-Event Delay",
        "xspice_real_delay.svg"
    ),
    xspice_device!(
        "rspice.xspice.real_gain",
        "real_gain",
        "Real-Event Gain",
        "xspice_real_gain.svg"
    ),
    xspice_device!(
        "rspice.xspice.real_to_v",
        "real_to_v",
        ["r_to_v"],
        "Real-to-Voltage Bridge",
        "xspice_real_to_voltage.svg"
    ),
    xspice_device!(
        "rspice.xspice.s_h",
        "s_h",
        "Sample and Hold",
        "xspice_sample_hold.svg"
    ),
    xspice_device!(
        "rspice.xspice.s_xfer",
        "s_xfer",
        "S-Domain Transfer Function",
        "xspice_s_xfer.svg"
    ),
    xspice_device!(
        "rspice.xspice.seegenerator",
        "seegenerator",
        ["seegen"],
        "Single-Event Effect Generator",
        "xspice_see_generator.svg"
    ),
    xspice_device!("rspice.xspice.sidiode", "sidiode", "S-I Diode", "diode.svg"),
    xspice_device!(
        "rspice.xspice.sine",
        "sine",
        "Controlled Sine Oscillator",
        "xspice_sine.svg"
    ),
    xspice_device!(
        "rspice.xspice.slew",
        "slew",
        "Slew-Rate Limiter",
        "xspice_slew.svg"
    ),
    xspice_device!(
        "rspice.xspice.spice2poly",
        "spice2poly",
        ["icm_spice2poly"],
        "SPICE2 Polynomial Source",
        "xspice_spice2poly.svg"
    ),
    xspice_device!(
        "rspice.xspice.square",
        "square",
        "Controlled Square Oscillator",
        "xspice_square.svg"
    ),
    xspice_device!(
        "rspice.xspice.table2d",
        "table2d",
        "2-D Lookup Table",
        "xspice_table2d.svg"
    ),
    xspice_device!(
        "rspice.xspice.table3d",
        "table3d",
        "3-D Lookup Table",
        "xspice_table3d.svg"
    ),
    xspice_device!(
        "rspice.xspice.tline",
        "tline",
        "XSPICE Transmission Line",
        "transmission_line.svg"
    ),
    xspice_device!(
        "rspice.xspice.triangle",
        "triangle",
        "Controlled Triangle Oscillator",
        "xspice_triangle.svg"
    ),
    xspice_device!(
        "rspice.xspice.xfer",
        "xfer",
        "AC Transfer Table",
        "xspice_xfer.svg"
    ),
    xspice_device!(
        "rspice.xspice.xyce_d_add",
        "xyce_d_add",
        "Full Adder",
        "xspice_full_adder.svg"
    ),
    xspice_device!(
        "rspice.xspice.zener",
        "zener",
        "XSPICE Zener",
        "diode_zener.svg"
    ),
];

pub const fn engine_only_xspice_devices() -> &'static [CatalogXspiceDeviceDescriptor] {
    ENGINE_ONLY_XSPICE_DEVICES
}

fn engine_only_xspice_device_by_id(
    stable_id: &str,
) -> Option<&'static CatalogXspiceDeviceDescriptor> {
    ENGINE_ONLY_XSPICE_DEVICES
        .iter()
        .find(|descriptor| descriptor.stable_id == stable_id)
}

/// Resolve one catalog descriptor against the executable code-model registry
/// and freeze that exact interface into a placement binding.
pub fn builtin_xspice_library_binding(
    descriptor: &CatalogXspiceDeviceDescriptor,
) -> Result<LibraryCellInstance, String> {
    static DEFAULT_BINDINGS: OnceLock<Vec<Result<LibraryCellInstance, String>>> = OnceLock::new();

    let index = ENGINE_ONLY_XSPICE_DEVICES
        .iter()
        .position(|candidate| candidate == descriptor)
        .ok_or_else(|| {
            format!(
                "XSPICE descriptor '{}' is not an authoritative catalog entry",
                descriptor.stable_id
            )
        })?;
    DEFAULT_BINDINGS
        .get_or_init(|| {
            let registry = CodeModelRegistry::with_builtins();
            ENGINE_ONLY_XSPICE_DEVICES
                .iter()
                .map(|descriptor| {
                    materialize_builtin_xspice_binding(descriptor, &registry, &BTreeMap::new())
                })
                .collect()
        })
        .get(index)
        .expect("catalog index and cached binding table stay parallel")
        .clone()
}

/// Return the vector ports whose widths may be selected before placement.
/// Fixed-width vectors remain present so the placement dialog can explain
/// their immutable width instead of hiding executable interface facts.
pub fn builtin_xspice_vector_ports(
    descriptor: &CatalogXspiceDeviceDescriptor,
) -> Result<Vec<CatalogXspiceVectorPort>, String> {
    authoritative_catalog_descriptor(descriptor)?;
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get(descriptor.model_type).ok_or_else(|| {
        format!(
            "XSPICE catalog device '{}' references unavailable model '{}'",
            descriptor.stable_id, descriptor.model_type
        )
    })?;

    Ok(model
        .ports()
        .iter()
        .filter(|port| port.is_vector)
        .map(|port| {
            let minimum = effective_vector_minimum(port);
            CatalogXspiceVectorPort {
                name: port.name.clone(),
                minimum,
                maximum: Some(
                    port.vector_max_len
                        .unwrap_or(MAX_BUILTIN_XSPICE_VECTOR_WIDTH)
                        .min(MAX_BUILTIN_XSPICE_VECTOR_WIDTH),
                ),
                default_width: minimum.max(1),
                null_allowed: port.null_allowed,
            }
        })
        .collect())
}

/// Materialize a catalog placement with explicitly selected logical vector
/// widths. Missing entries use the reviewed default. Unknown/scalar port
/// names and widths outside the executable registry bounds fail closed.
pub fn builtin_xspice_library_binding_with_vector_widths(
    descriptor: &CatalogXspiceDeviceDescriptor,
    vector_widths: &BTreeMap<String, usize>,
) -> Result<LibraryCellInstance, String> {
    authoritative_catalog_descriptor(descriptor)?;
    let registry = CodeModelRegistry::with_builtins();
    materialize_builtin_xspice_binding(descriptor, &registry, vector_widths)
}

fn authoritative_catalog_descriptor(
    descriptor: &CatalogXspiceDeviceDescriptor,
) -> Result<&'static CatalogXspiceDeviceDescriptor, String> {
    ENGINE_ONLY_XSPICE_DEVICES
        .iter()
        .find(|candidate| *candidate == descriptor)
        .ok_or_else(|| {
            format!(
                "XSPICE descriptor '{}' is not an authoritative catalog entry",
                descriptor.stable_id
            )
        })
}

fn materialize_builtin_xspice_binding(
    descriptor: &CatalogXspiceDeviceDescriptor,
    registry: &CodeModelRegistry,
    vector_widths: &BTreeMap<String, usize>,
) -> Result<LibraryCellInstance, String> {
    let model = registry.get(descriptor.model_type).ok_or_else(|| {
        format!(
            "XSPICE catalog device '{}' references unavailable model '{}'",
            descriptor.stable_id, descriptor.model_type
        )
    })?;

    for requested in vector_widths.keys() {
        let Some(port) = model
            .ports()
            .iter()
            .find(|port| port.name.eq_ignore_ascii_case(requested))
        else {
            return Err(format!(
                "XSPICE model '{}' has no port named '{}'",
                descriptor.model_type, requested
            ));
        };
        if !port.is_vector {
            return Err(format!(
                "XSPICE model '{}' port '{}' is scalar and has no configurable width",
                descriptor.model_type, port.name
            ));
        }
    }

    let mut terminal_specs = Vec::new();
    let mut port_bindings = Vec::with_capacity(model.ports().len());
    for port in model.ports() {
        let width = if port.is_vector {
            vector_widths
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(&port.name))
                .map(|(_, width)| *width)
                .unwrap_or_else(|| effective_vector_minimum(port).max(1))
        } else {
            1
        };
        let minimum = if port.is_vector {
            effective_vector_minimum(port)
        } else {
            1
        };
        if width < minimum {
            return Err(format!(
                "XSPICE model '{}' port '{}' width {} is below its minimum {}",
                descriptor.model_type, port.name, width, minimum
            ));
        }
        if let Some(maximum) = port.vector_max_len
            && width > maximum
        {
            return Err(format!(
                "XSPICE model '{}' port '{}' has impossible vector bounds {}..{}",
                descriptor.model_type, port.name, width, maximum
            ));
        }
        if port.is_vector && width > MAX_BUILTIN_XSPICE_VECTOR_WIDTH {
            return Err(format!(
                "XSPICE model '{}' port '{}' width {} exceeds the product safety limit {}",
                descriptor.model_type, port.name, width, MAX_BUILTIN_XSPICE_VECTOR_WIDTH
            ));
        }

        let differential = core_port_type_is_differential(port.default_type);
        let direction = state_port_direction(port.direction);
        let mut terminals = Vec::with_capacity(width * if differential { 2 } else { 1 });
        for element in 0..width {
            let base = if port.is_vector {
                format!("{}[{element}]", port.name)
            } else {
                port.name.clone()
            };
            if differential {
                for suffix in ['+', '-'] {
                    terminals.push(terminal_specs.len());
                    terminal_specs.push(PortSpec {
                        name: format!("{base}{suffix}"),
                        direction,
                    });
                }
            } else {
                terminals.push(terminal_specs.len());
                terminal_specs.push(PortSpec {
                    name: base,
                    direction,
                });
            }
        }

        port_bindings.push(BuiltinXspicePortBinding {
            name: port.name.clone(),
            direction: persisted_port_direction(port.direction),
            port_type: persisted_port_type(port.default_type),
            is_vector: port.is_vector,
            vector_width: width,
            null_allowed: port.null_allowed,
            terminals,
        });
    }

    let mut binding = LibraryCellInstance::new("rspice", descriptor.model_type, "xspice");
    binding.module_name = Some(descriptor.model_type.to_owned());
    binding.reference_prefix = Some("A".to_owned());
    binding.parameter_order = model
        .parameters()
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect();
    binding.bind_interface(&terminal_specs);
    binding.builtin_xspice = Some(BuiltinXspiceInstance {
        schema_revision: 2,
        stable_id: descriptor.stable_id.to_owned(),
        model_type: descriptor.model_type.to_owned(),
        symbol_asset: descriptor.symbol_asset.to_owned(),
        schema_signature: xspice_schema_signature(model.ports(), model.parameters()),
        ports: port_bindings,
    });
    Ok(binding)
}

fn effective_vector_minimum(port: &CorePortSpec) -> usize {
    let declared = port.vector_min_len.unwrap_or(0);
    if port.null_allowed {
        declared
    } else {
        declared.max(1)
    }
}

/// Validate that a persisted built-in XSPICE placement still exactly matches
/// the compiled catalog and executable registry contract.
///
/// Built-ins are not ordinary project L/C/V cells, so hierarchy resolution
/// and netlisting both use this check as their authority boundary.  Comparing
/// the complete materialized binding prevents edited or stale persisted data
/// from changing port shaping, parameter allowlists, or implementation
/// identity without an explicit migration.
pub fn validate_builtin_xspice_binding(
    binding: &LibraryCellInstance,
) -> Result<&'static CatalogXspiceDeviceDescriptor, String> {
    let contract = binding
        .builtin_xspice
        .as_ref()
        .ok_or_else(|| "binding is not a built-in XSPICE device".to_owned())?;
    let descriptor = engine_only_xspice_device_by_id(&contract.stable_id).ok_or_else(|| {
        format!(
            "built-in XSPICE device has unknown stable catalog identity '{}'",
            contract.stable_id
        )
    })?;
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get(descriptor.model_type).ok_or_else(|| {
        format!(
            "built-in XSPICE model '{}' is unavailable in this build",
            descriptor.model_type
        )
    })?;
    if contract.ports.len() != model.ports().len() {
        return Err(format!(
            "built-in XSPICE device '{}' has a stale executable port count",
            contract.stable_id
        ));
    }
    let mut widths = BTreeMap::new();
    for (persisted, executable) in contract.ports.iter().zip(model.ports()) {
        if persisted.name != executable.name || persisted.is_vector != executable.is_vector {
            return Err(format!(
                "built-in XSPICE device '{}' has stale port identity metadata",
                contract.stable_id
            ));
        }
        if persisted.is_vector {
            widths.insert(persisted.name.clone(), persisted.vector_width);
        } else if persisted.vector_width != 1 {
            return Err(format!(
                "built-in XSPICE device '{}' scalar port '{}' has an invalid width",
                contract.stable_id, persisted.name
            ));
        }
    }
    let expected = materialize_builtin_xspice_binding(descriptor, &registry, &widths)?;
    let expected_contract = expected
        .builtin_xspice
        .as_ref()
        .expect("catalog builder always creates a built-in XSPICE contract");

    if contract != expected_contract {
        return Err(format!(
            "built-in XSPICE device '{}' has a stale or modified executable schema",
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
    {
        return Err(format!(
            "built-in XSPICE device '{}' has modified binding metadata",
            contract.stable_id
        ));
    }

    Ok(descriptor)
}

fn state_port_direction(direction: CorePortDirection) -> PortDirection {
    match direction {
        CorePortDirection::In => PortDirection::In,
        CorePortDirection::Out => PortDirection::Out,
        CorePortDirection::InOut => PortDirection::InOut,
    }
}

fn persisted_port_direction(direction: CorePortDirection) -> BuiltinXspicePortDirection {
    match direction {
        CorePortDirection::In => BuiltinXspicePortDirection::In,
        CorePortDirection::Out => BuiltinXspicePortDirection::Out,
        CorePortDirection::InOut => BuiltinXspicePortDirection::InOut,
    }
}

fn persisted_port_type(port_type: CorePortType) -> BuiltinXspicePortType {
    match port_type {
        CorePortType::Voltage => BuiltinXspicePortType::Voltage,
        CorePortType::DifferentialVoltage => BuiltinXspicePortType::DifferentialVoltage,
        CorePortType::Conductance => BuiltinXspicePortType::Conductance,
        CorePortType::DifferentialConductance => BuiltinXspicePortType::DifferentialConductance,
        CorePortType::Hybrid => BuiltinXspicePortType::Hybrid,
        CorePortType::DifferentialHybrid => BuiltinXspicePortType::DifferentialHybrid,
        CorePortType::Current => BuiltinXspicePortType::Current,
        CorePortType::DifferentialCurrent => BuiltinXspicePortType::DifferentialCurrent,
        CorePortType::VoltageName => BuiltinXspicePortType::VoltageName,
        CorePortType::Digital => BuiltinXspicePortType::Digital,
        CorePortType::Real => BuiltinXspicePortType::Real,
        CorePortType::Integer => BuiltinXspicePortType::Integer,
        CorePortType::UserDefined => BuiltinXspicePortType::UserDefined,
    }
}

const fn core_port_type_is_differential(port_type: CorePortType) -> bool {
    matches!(
        port_type,
        CorePortType::DifferentialVoltage
            | CorePortType::DifferentialConductance
            | CorePortType::DifferentialHybrid
            | CorePortType::DifferentialCurrent
    )
}

fn xspice_schema_signature(ports: &[CorePortSpec], parameters: &[CoreParamSpec]) -> String {
    let mut signature = String::from("xspice-v1|");
    for port in ports {
        signature.push_str(&format!(
            "p:{}:{:?}:{:?}:{:?}:{}:{}:{:?}:{:?};",
            port.name,
            port.direction,
            port.default_type,
            port.allowed_types,
            port.is_vector,
            port.null_allowed,
            port.vector_min_len,
            port.vector_max_len
        ));
    }
    for parameter in parameters {
        signature.push_str(&format!(
            "a:{}:{:?}:{}:{:016x}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{}:{:?}:{}:{:?}:{:?};",
            parameter.name,
            parameter.param_type,
            parameter.required,
            parameter.default.to_bits(),
            parameter.string_default,
            parameter.complex_default,
            parameter.string_vector_default,
            parameter.complex_vector_default,
            parameter.real_vector_default,
            parameter.integer_vector_default,
            parameter.min.map(f64::to_bits),
            parameter.min_is_soft,
            parameter.max.map(f64::to_bits),
            parameter.max_is_soft,
            parameter.vector_min_len,
            parameter.vector_max_len
        ));
    }
    signature
}

/// Compatibility names that share an existing enum-backed GUI device.
#[cfg(test)]
const fn legacy_xspice_aliases(kind: ComponentType) -> &'static [&'static str] {
    match kind {
        ComponentType::XspiceDivider => &["divider"],
        ComponentType::XspiceIntegrator => &["integrator"],
        ComponentType::XspiceDifferentiator => &["differentiator"],
        ComponentType::XspiceBuffer => &["xyce_d_buffer"],
        ComponentType::XspiceAndGate => &["xyce_d_and", "xyce_legacy_d_and"],
        ComponentType::XspiceInverter => &["xyce_d_inverter", "xyce_legacy_d_inverter"],
        ComponentType::XspiceNandGate => &["xyce_d_nand", "xyce_legacy_d_nand"],
        ComponentType::XspiceNorGate => &["xyce_d_nor", "xyce_legacy_d_nor"],
        ComponentType::XspiceOrGate => &["xyce_d_or", "xyce_legacy_d_or"],
        ComponentType::XspiceXorGate => &["xyce_d_xor", "xyce_legacy_d_xor"],
        ComponentType::XspiceDFlipFlop => &["xyce_d_dff", "xyce_legacy_d_dff"],
        ComponentType::XspiceJkFlipFlop => &["xyce_d_jkff"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use rspice_core::xspice::CodeModelRegistry;

    use super::*;
    use crate::state::DeviceImplementation;

    #[test]
    fn every_registered_xspice_name_has_one_reviewed_gui_disposition() {
        let registry = CodeModelRegistry::with_builtins();
        let mut dispositions = HashMap::<&str, &str>::new();

        for kind in ComponentType::ALL {
            if let DeviceImplementation::Xspice { model_type } = kind.descriptor().implementation {
                assert!(
                    registry.contains(model_type),
                    "missing registry model {model_type}"
                );
                assert!(
                    dispositions
                        .insert(model_type, kind.descriptor().stable_id)
                        .is_none(),
                    "duplicate disposition for {model_type}"
                );
                for alias in legacy_xspice_aliases(kind) {
                    assert!(registry.contains(alias), "missing registry alias {alias}");
                    assert!(
                        dispositions
                            .insert(alias, kind.descriptor().stable_id)
                            .is_none(),
                        "duplicate disposition for {alias}"
                    );
                }
            }
        }

        let mut stable_ids = ComponentType::ALL
            .into_iter()
            .map(|kind| kind.descriptor().stable_id)
            .collect::<HashSet<_>>();
        for descriptor in engine_only_xspice_devices() {
            assert!(
                stable_ids.insert(descriptor.stable_id),
                "duplicate catalog id {}",
                descriptor.stable_id
            );
            for name in
                std::iter::once(descriptor.model_type).chain(descriptor.aliases.iter().copied())
            {
                assert!(
                    registry.contains(name),
                    "catalog model/alias {name} is not registered"
                );
                assert!(
                    dispositions.insert(name, descriptor.stable_id).is_none(),
                    "duplicate disposition for {name}"
                );
            }
        }

        // Diagnostic conformance model; useful in decks/tests, not a circuit
        // device and therefore deliberately absent from the placement UI.
        assert!(
            dispositions
                .insert(
                    "print_param_types",
                    "non-device.diagnostic.print_param_types"
                )
                .is_none()
        );

        let mut missing = registry
            .model_names()
            .into_iter()
            .filter(|name| !dispositions.contains_key(name))
            .collect::<Vec<_>>();
        missing.sort_unstable();
        assert!(
            missing.is_empty(),
            "unclassified XSPICE registry names: {missing:?}"
        );
        assert_eq!(dispositions.len(), registry.len());
    }

    #[test]
    fn every_catalog_device_materializes_and_validates_an_exact_frozen_binding() {
        for descriptor in engine_only_xspice_devices() {
            let binding = builtin_xspice_library_binding(descriptor)
                .unwrap_or_else(|error| panic!("{}: {error}", descriptor.stable_id));
            let validated = validate_builtin_xspice_binding(&binding)
                .unwrap_or_else(|error| panic!("{}: {error}", descriptor.stable_id));
            assert_eq!(validated.stable_id, descriptor.stable_id);
            assert!(binding.interface_bound);
            assert_eq!(binding.terminal_order.len(), binding.terminal_dirs.len());
            let contract = binding.builtin_xspice.as_ref().expect("built-in contract");
            assert_eq!(contract.schema_revision, 2);
            assert!(!contract.schema_signature.is_empty());
            assert!(contract.ports.iter().all(|port| {
                !port.terminals.is_empty()
                    && port
                        .terminals
                        .iter()
                        .all(|terminal| *terminal < binding.terminal_order.len())
            }));

            let encoded = serde_json::to_vec(&binding).expect("binding serializes");
            let decoded: LibraryCellInstance =
                serde_json::from_slice(&encoded).expect("binding deserializes");
            assert_eq!(decoded, binding);
            validate_builtin_xspice_binding(&decoded).expect("round-trip remains executable");
        }
    }

    #[test]
    fn pspice_digital_stimulus_has_a_placeable_exact_catalog_contract() {
        let descriptor = engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.model_type == "pspice_d_stim")
            .expect("PSpice digital stimulus catalog disposition");
        assert_eq!(descriptor.stable_id, "rspice.xspice.pspice_d_stim");
        assert_eq!(descriptor.display_name, "PSpice Digital Stimulus");
        assert_eq!(descriptor.symbol_asset, "xspice_digital_source.svg");

        let binding = builtin_xspice_library_binding(descriptor)
            .expect("PSpice digital stimulus placement binding");
        let validated = validate_builtin_xspice_binding(&binding)
            .expect("PSpice digital stimulus exact executable binding");
        assert_eq!(validated.model_type, "pspice_d_stim");
        assert_eq!(binding.terminal_order, ["out[0]"]);
        let contract = binding.builtin_xspice.as_ref().expect("built-in contract");
        assert_eq!(contract.ports.len(), 1);
        assert_eq!(contract.ports[0].name, "out");
        assert_eq!(contract.ports[0].vector_width, 1);
        assert_eq!(binding.parameter_order, ["stim_program"]);
    }

    #[test]
    fn modified_catalog_binding_fails_closed() {
        let mut binding = builtin_xspice_library_binding(&ENGINE_ONLY_XSPICE_DEVICES[0])
            .expect("catalog binding");
        binding.terminal_order[0].push_str("_tampered");

        assert!(validate_builtin_xspice_binding(&binding).is_err());
    }

    #[test]
    fn configurable_vector_widths_are_frozen_and_revalidated() {
        let descriptor = engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.model_type == "d_lut")
            .expect("lookup-table descriptor");
        let ports = builtin_xspice_vector_ports(descriptor).expect("vector contract");
        let input = ports
            .iter()
            .find(|port| port.name == "in")
            .expect("vector input");
        assert_eq!(input.minimum, 1);
        assert_eq!(input.default_width, 1);
        assert_eq!(input.maximum, Some(MAX_BUILTIN_XSPICE_VECTOR_WIDTH));

        let widths = BTreeMap::from([("in".to_owned(), 5)]);
        let binding = builtin_xspice_library_binding_with_vector_widths(descriptor, &widths)
            .expect("five-input lookup-table binding");
        let contract = binding.builtin_xspice.as_ref().expect("built-in contract");
        let input = contract
            .ports
            .iter()
            .find(|port| port.name == "in")
            .expect("persisted input");
        assert_eq!(input.vector_width, 5);
        assert_eq!(input.terminals.len(), 5);
        assert_eq!(binding.terminal_order.len(), 6);
        validate_builtin_xspice_binding(&binding).expect("custom width remains executable");

        let too_narrow = BTreeMap::from([("in".to_owned(), 0)]);
        assert!(
            builtin_xspice_library_binding_with_vector_widths(descriptor, &too_narrow).is_err()
        );

        let mut tampered = binding;
        tampered
            .builtin_xspice
            .as_mut()
            .expect("contract")
            .ports
            .iter_mut()
            .find(|port| port.name == "in")
            .expect("input")
            .vector_width = 4;
        assert!(validate_builtin_xspice_binding(&tampered).is_err());
    }

    #[test]
    fn nullable_vector_ports_may_be_explicitly_omitted() {
        let descriptor = engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.model_type == "d_cosim")
            .expect("co-simulation descriptor");
        let widths = BTreeMap::from([
            ("d_in".to_owned(), 0),
            ("d_out".to_owned(), 0),
            ("d_inout".to_owned(), 0),
        ]);
        let binding = builtin_xspice_library_binding_with_vector_widths(descriptor, &widths)
            .expect("all-null co-simulation binding");
        assert!(binding.terminal_order.is_empty());
        assert!(
            binding
                .builtin_xspice
                .as_ref()
                .expect("contract")
                .ports
                .iter()
                .all(|port| port.vector_width == 0 && port.terminals.is_empty())
        );
        validate_builtin_xspice_binding(&binding).expect("zero-width nullable ports validate");
    }
}
