//! Resolve an external Verilog-A/AMS instance before choosing its execution path.
//!
//! A parameter can change generated structure, packed constants, or whether a
//! discrete process exists. Both device routes must bind the resulting artifact,
//! including its solver unknowns. Ordinary analog numeric parameters retain the
//! compiled-device path without recompiling source.

use std::sync::Arc;

use rspice_veriloga::CompiledModel;
use rspice_veriloga::canonical_ir::CanonicalIrArtifact;

use super::refuse_veriloga_instance as refuse;
use super::veriloga_cache::{CachedVerilogAModel, VerilogACompileControl};
use crate::{ElaborationErrorKind, SimulationError};

/// Base artifacts stay alive in the build's model table, so their addresses
/// identify immutable source artifacts for this cache's lifetime.
pub(super) type InstanceSpecializations = std::collections::HashMap<
    (usize, usize, Vec<(String, u64)>),
    (Arc<CompiledModel>, Arc<CanonicalIrArtifact>),
>;

pub(super) struct PreparedInstance<'a> {
    pub model: Arc<CompiledModel>,
    pub canonical_ir: Option<Arc<CanonicalIrArtifact>>,
    pub overrides: Vec<(&'a str, f64)>,
    pub temperature: f64,
    pub multiplicity: Option<f64>,
}

struct PublicParameter<'a> {
    name: &'a str,
    needs_source: bool,
}

/// Public numeric and exact declarations share the external namespace. The
/// compiler rejects case-insensitive collisions before partitioning it into
/// runtime slots and retained elaboration constants. Hidden hierarchy/local
/// declarations never claim an instance-facing name, including engine controls.
fn public_parameter<'a>(entry: &'a CachedVerilogAModel, name: &str) -> Option<PublicParameter<'a>> {
    if let Some(index) = entry.model.parameter_index(name) {
        let parameter = &entry.model.parameters[index];
        return Some(PublicParameter {
            name: parameter.name.as_str(),
            needs_source: parameter.elaboration_value.is_some()
                || parameter.elaboration_given.is_some(),
        });
    }
    let parameters = &entry
        .canonical_ir
        .as_deref()?
        .digital
        .elaboration_parameters;
    parameters
        .iter()
        .find(|parameter| {
            parameter.is_public
                && (parameter.name.eq_ignore_ascii_case(name)
                    || parameter
                        .aliases
                        .iter()
                        .any(|alias| alias.eq_ignore_ascii_case(name)))
        })
        .map(|parameter| PublicParameter {
            name: parameter.name.as_str(),
            needs_source: true,
        })
}

pub(super) fn prepare<'a>(
    netlist: &crate::Netlist,
    element: &crate::netlist::Element,
    entry: &'a CachedVerilogAModel,
    specializations: &mut InstanceSpecializations,
    temperature: f64,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<PreparedInstance<'a>, SimulationError> {
    let crate::netlist::ElementKind::Subcircuit {
        subckt_name,
        params,
    } = &element.kind
    else {
        unreachable!("Verilog-A instance preparation requires an X-card");
    };
    let declares = |name: &str| public_parameter(entry, name).is_some();
    let mut context = super::InstanceParameterContext::new(netlist, temperature);
    let temperature = super::veriloga_instance_temperature(
        &element.name,
        subckt_name,
        params,
        &declares,
        &mut context,
        temperature,
    )?;
    context.retarget(temperature);
    let mut overrides = Vec::with_capacity(params.len());
    let mut multiplicity = None;
    let mut specializes = entry
        .canonical_ir
        .as_deref()
        .is_some_and(|artifact| artifact.digital.has_executable_content());
    for (name, value) in params {
        if super::veriloga_instance_temperature_key(name, &declares).is_some() {
            continue;
        }
        let value = super::veriloga_instance_numeric_value(
            &element.name,
            subckt_name,
            name,
            value,
            &mut context,
        )?;
        // A public model parameter or alias owns its name ahead of engine
        // multiplicity and temperature controls on either execution path.
        if name.eq_ignore_ascii_case("m") && !declares(name) {
            if !value.is_finite() || value <= 0.0 {
                return Err(refuse(
                    &element.name,
                    subckt_name,
                    ElaborationErrorKind::ParameterValue,
                    format!("multiplicity must be a positive finite value, got {value}"),
                ));
            }
            multiplicity = Some(value);
            continue;
        }
        let parameter = public_parameter(entry, name).ok_or_else(|| {
            refuse(
                &element.name,
                subckt_name,
                ElaborationErrorKind::ParameterUnknown,
                format!("unknown parameter '{name}'"),
            )
        })?;
        specializes |= parameter.needs_source;
        overrides.push((parameter.name, value));
    }

    let (model, canonical_ir) = if specializes && !overrides.is_empty() {
        let artifact = entry.canonical_ir.as_deref().ok_or_else(|| {
            refuse(
                &element.name,
                subckt_name,
                ElaborationErrorKind::CacheCorrupt,
                "structural parameter overrides require canonical IR; recompile the model",
            )
        })?;
        // Canonical names and order let equivalent cards share immutable
        // compilation results. Device state remains private to each instance.
        overrides.sort_by(|left, right| left.0.cmp(right.0));
        let key = (
            Arc::as_ptr(&entry.model) as usize,
            artifact as *const _ as usize,
            overrides
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_bits()))
                .collect(),
        );
        let specialized = if let Some(specialized) = specializations.get(&key) {
            specialized.clone()
        } else {
            let compiler =
                rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
                    enable_ams: true,
                    ..Default::default()
                });
            let runtime = compiler
                .specialize_mixed_runtime(artifact, &overrides, &VerilogACompileControl { abort })
                .map_err(|error| {
                    if abort.is_aborted() {
                        SimulationError::Aborted
                    } else {
                        refuse(
                            &element.name,
                            subckt_name,
                            ElaborationErrorKind::CompileRefusal,
                            format!("parameter elaboration failed: {error}"),
                        )
                    }
                })?;
            let specialized = (Arc::new(runtime.model), Arc::new(runtime.canonical_ir));
            specializations.insert(key, specialized.clone());
            specialized
        };
        (specialized.0, Some(specialized.1))
    } else {
        (Arc::clone(&entry.model), entry.canonical_ir.clone())
    };
    // Exact values have already undergone source assignment conversion. They
    // have no numeric runtime slot. Inspect the effective model: an implicit
    // parameter can change from an exact integral value to a real override.
    if specializes {
        overrides.retain(|(name, _)| {
            model
                .parameters
                .iter()
                .any(|parameter| parameter.is_public && parameter.name == *name)
        });
    }
    Ok(PreparedInstance {
        model,
        canonical_ir,
        overrides,
        temperature,
        multiplicity,
    })
}
