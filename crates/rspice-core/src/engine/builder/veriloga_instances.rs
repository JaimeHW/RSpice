//! Resolve an external Verilog-A/AMS instance before choosing its execution path.
//!
//! A parameter can change generated structure, packed constants, or whether a
//! discrete process exists. Both device routes must bind the resulting artifact,
//! including its solver unknowns. Ordinary analog numeric parameters retain the
//! compiled-device path without recompiling source.

use std::collections::HashMap;
use std::sync::Arc;

use rspice_veriloga::CompiledModel;
use rspice_veriloga::canonical_ir::CanonicalIrArtifact;

use super::refuse_veriloga_instance as refuse;
use super::veriloga_cache::{CachedVerilogAModel, VerilogACompileControl};
use crate::{ElaborationErrorKind, SimulationError};

/// Base artifacts stay alive in the build's model table, so their addresses
/// identify immutable source artifacts for this cache's lifetime.
pub(super) type InstanceSpecializations = std::collections::HashMap<
    (usize, usize, Vec<(String, SpecializedValue)>),
    (Arc<CompiledModel>, Arc<CanonicalIrArtifact>),
>;

/// One override in a specialization key: a number by its bits, a string by
/// its text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum SpecializedValue {
    Number(u64),
    String(String),
    Array(Vec<u64>),
}

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
    /// A `string` parameter: its value is closed by source specialization.
    is_string: bool,
    /// A parameter array: it takes every element at once, by specialization.
    is_array: bool,
}

/// Public numeric and exact declarations share the external namespace. The
/// compiler rejects case-insensitive collisions before partitioning it into
/// runtime slots and retained elaboration constants. Hidden hierarchy/local
/// declarations never claim an instance-facing name, including engine controls.
fn public_parameter<'a>(entry: &'a CachedVerilogAModel, name: &str) -> Option<PublicParameter<'a>> {
    if let Some(index) = entry.model.parameter_index(name) {
        let parameter = &entry.model.parameters[index];
        let is_string = entry.canonical_ir.as_deref().is_some_and(|artifact| {
            artifact.hir.parameters.iter().any(|declared| {
                declared.name == parameter.name
                    && declared.value_type
                        == rspice_veriloga::canonical_ir::CanonicalValueType::String
            })
        });
        return Some(PublicParameter {
            name: parameter.name.as_str(),
            needs_source: is_string
                || parameter.is_array
                || parameter.elaboration_value.is_some()
                || parameter.elaboration_given.is_some(),
            is_string,
            is_array: parameter.is_array,
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
            is_string: false,
            is_array: false,
        })
}

/// The name a card already set `parameter` by, when `written` is another of
/// its names (the declared one or an alias). Records the first name used.
fn set_by_another_name<'n>(
    set_as: &mut Vec<(&'n str, &'n str)>,
    parameter: &'n str,
    written: &'n str,
) -> Option<&'n str> {
    match set_as.iter().find(|(declared, _)| *declared == parameter) {
        Some(&(_, previous)) => (!previous.eq_ignore_ascii_case(written)).then_some(previous),
        None => {
            set_as.push((parameter, written));
            None
        }
    }
}

/// Whether `name` is an engine control on the instance rather than one of the
/// module's parameters. A public parameter or alias owns its name ahead of
/// multiplicity and temperature.
fn is_engine_control(name: &str, declares: &dyn Fn(&str) -> bool) -> bool {
    super::veriloga_instance_temperature_key(name, declares).is_some()
        || (name.eq_ignore_ascii_case("m") && !declares(name))
}

/// Every value a `.model` card sets, as an instance would write it.
fn card_parameters(
    card: &crate::netlist::ModelDef,
) -> Vec<(String, crate::netlist::ParametricValue)> {
    use crate::netlist::ParametricValue as Value;
    let named = |name: &String, value| (name.clone(), value);
    card.params
        .iter()
        .map(|(name, value)| named(name, Value::Resolved(*value)))
        .chain(
            card.expr_params
                .iter()
                .map(|(name, text)| named(name, Value::Expression(text.clone()))),
        )
        .chain(
            card.string_params
                .iter()
                .map(|(name, text)| named(name, Value::String(text.clone()))),
        )
        .chain(card.real_vector_params.iter().map(|(name, values)| {
            named(
                name,
                Value::Array(values.iter().copied().map(Value::Resolved).collect()),
            )
        }))
        .chain(card.real_vector_expr_params.iter().map(|(name, texts)| {
            named(
                name,
                Value::Array(texts.iter().cloned().map(Value::Expression).collect()),
            )
        }))
        .chain(card.integer_vector_params.iter().map(|(name, values)| {
            named(
                name,
                Value::Array(
                    values
                        .iter()
                        .map(|value| Value::Resolved(*value as f64))
                        .collect(),
                ),
            )
        }))
        .collect()
}

/// A parameter a `.model` card must set: one its declaration does not mark
/// `(* type="instance" *)`. An artifact without canonical IR states no scopes.
fn is_model_only(entry: &CachedVerilogAModel, parameter: &str) -> bool {
    entry.canonical_ir.as_deref().is_some_and(|artifact| {
        artifact.hir.parameters.iter().any(|declared| {
            declared.name == parameter
                && declared.scope == rspice_veriloga::semantic::ParameterScope::Model
        })
    })
}

/// A value the declared bounds and exclusions of `parameter` admit. Bounds
/// that depend on other parameters are the instance setup's to check.
fn in_declared_range(parameter: &rspice_veriloga::codegen::CompiledParameter, value: f64) -> bool {
    let above = parameter.min.is_none_or(|min| {
        if parameter.min_exclusive {
            value > min
        } else {
            value >= min
        }
    });
    let below = parameter.max.is_none_or(|max| {
        if parameter.max_exclusive {
            value < max
        } else {
            value <= max
        }
    });
    above && below && !parameter.exclude.contains(&value)
}

/// One paramset of a same-named group that fits an instance, and what ranks
/// it among the others.
struct SuitableOverload<'g> {
    key: &'g str,
    unset_parameters: usize,
    ranged_local_parameters: usize,
    unconnected_ports: usize,
}

/// The paramset of a same-named group an instance selects (Verilog-AMS LRM
/// 2.4 section 6.4.2): it declares every parameter the instance sets, the
/// values lie in its declared ranges and the instance connects no port it
/// lacks. Among those, the fewest parameters left unset, then the most local
/// parameters with ranges, then the fewest unconnected ports; a tie after
/// that is an error.
fn select_paramset<'g>(
    element: &crate::netlist::Element,
    master: &str,
    params: &[(String, crate::netlist::ParametricValue)],
    group: &'g [String],
    bindings: &HashMap<String, super::VerilogAModelBinding>,
    context: &mut super::InstanceParameterContext<'_>,
) -> Result<&'g str, SimulationError> {
    let mut suitable: Vec<SuitableOverload<'g>> = Vec::new();
    'candidates: for key in group {
        let Some(entry) = bindings.get(key).and_then(|binding| binding.model.as_ref()) else {
            continue;
        };
        if element.nodes.len() > entry.model.num_terminals {
            continue;
        }
        let declares = |name: &str| public_parameter(entry, name).is_some();
        let mut set = Vec::new();
        for (name, value) in params {
            if is_engine_control(name, &declares) {
                continue;
            }
            let Some(parameter) = public_parameter(entry, name) else {
                continue 'candidates;
            };
            set.push(parameter.name);
            if parameter.is_string
                || parameter.is_array
                || matches!(value, crate::netlist::ParametricValue::Array(_))
            {
                continue;
            }
            let number = super::veriloga_instance_numeric_value(
                &element.name,
                master,
                name,
                value,
                context,
            )?;
            if let Some(index) = entry.model.parameter_index(name)
                && !in_declared_range(&entry.model.parameters[index], number)
            {
                continue 'candidates;
            }
        }
        let public = || entry.model.parameters.iter().filter(|p| p.is_public);
        let unset = || public().filter(|p| !set.contains(&p.name.as_str()));
        if unset().any(|p| p.default_program.is_none() && !in_declared_range(p, p.default)) {
            continue;
        }
        suitable.push(SuitableOverload {
            key,
            unset_parameters: unset().count(),
            ranged_local_parameters: entry
                .model
                .parameters
                .iter()
                .filter(|p| {
                    !p.is_public
                        && !p.name.contains('.')
                        && (p.min.is_some() || p.max.is_some() || !p.exclude.is_empty())
                })
                .count(),
            unconnected_ports: entry.model.num_terminals - element.nodes.len(),
        });
    }
    let refuse_selection = |detail: String| {
        refuse(
            &element.name,
            master,
            ElaborationErrorKind::ModuleNotSelected,
            detail,
        )
    };
    let count = group.len();
    if suitable.is_empty() {
        return Err(refuse_selection(format!(
            "none of the {count} paramsets named '{master}' accepts this instance's parameters and connections"
        )));
    }
    let keep_best = |suitable: &mut Vec<SuitableOverload<'g>>,
                     rank: &dyn Fn(&SuitableOverload<'g>) -> usize| {
        let best = suitable.iter().map(rank).min().unwrap_or_default();
        suitable.retain(|overload| rank(overload) == best);
    };
    keep_best(&mut suitable, &|overload| overload.unset_parameters);
    keep_best(&mut suitable, &|overload| {
        usize::MAX - overload.ranged_local_parameters
    });
    keep_best(&mut suitable, &|overload| overload.unconnected_ports);
    match suitable.as_slice() {
        [selected] => Ok(selected.key),
        tied => Err(refuse_selection(format!(
            "{} of the {count} paramsets named '{master}' fit this instance equally; set a parameter only one of them declares",
            tied.len()
        ))),
    }
}

/// The same-named paramsets among the bound modules, by normalized name, each
/// group in declaration order.
fn paramset_groups(
    bindings: &HashMap<String, super::VerilogAModelBinding>,
) -> HashMap<String, Vec<String>> {
    let mut groups: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for key in bindings.keys() {
        if let Some((name, ordinal)) = rspice_veriloga::paramset_overload(key) {
            groups
                .entry(name.to_string())
                .or_default()
                .push((ordinal, key.clone()));
        }
    }
    groups
        .into_iter()
        .map(|(name, mut group)| {
            group.sort();
            (name, group.into_iter().map(|(_, key)| key).collect())
        })
        .collect()
}

/// Bind what each instance card names to the module that builds it: a
/// `.model` card of a Verilog-A module gives the card's values, and the
/// instance may then set only instance parameters; a name several paramsets
/// share selects one per instance. Direct module instances keep their cards.
pub(super) fn bind_instance_masters(
    elements: &mut [crate::netlist::Element],
    netlist: &crate::Netlist,
    bindings: &HashMap<String, super::VerilogAModelBinding>,
    temperature: f64,
) -> Result<(), SimulationError> {
    use super::veriloga_cache::normalize_model_key;
    let groups = paramset_groups(bindings);
    let answers = |name: &str| {
        let key = normalize_model_key(name);
        bindings.contains_key(&key) || groups.contains_key(&key)
    };
    let mut context = super::InstanceParameterContext::new(netlist, temperature);
    for element in elements.iter_mut() {
        let crate::netlist::ElementKind::Subcircuit {
            subckt_name,
            params,
        } = &element.kind
        else {
            continue;
        };
        let card =
            super::find_model_def(netlist, subckt_name).filter(|card| answers(&card.model_type));
        let master = card.map_or(subckt_name.as_str(), |card| card.model_type.as_str());
        let key = normalize_model_key(master);
        let group = groups.get(&key);
        if card.is_none() && group.is_none() {
            continue;
        }
        let card_values = card.map(card_parameters).unwrap_or_default();
        let selected = match group {
            Some(group) => {
                // Selection reads the values the instance will have.
                let written: Vec<_> = card_values
                    .iter()
                    .filter(|(name, _)| {
                        !params.iter().any(|(set, _)| set.eq_ignore_ascii_case(name))
                    })
                    .chain(params.iter())
                    .cloned()
                    .collect();
                select_paramset(element, master, &written, group, bindings, &mut context)?
                    .to_string()
            }
            None => key,
        };
        let entry = bindings
            .get(&selected)
            .and_then(|binding| binding.model.as_ref());
        let mut bound = Vec::with_capacity(card_values.len() + params.len());
        if let Some(entry) = entry.filter(|_| card.is_some()) {
            let declares = |name: &str| public_parameter(entry, name).is_some();
            let declared = |name: &str| public_parameter(entry, name).map(|found| found.name);
            for (name, _) in params {
                if !is_engine_control(name, &declares)
                    && declared(name).is_some_and(|parameter| is_model_only(entry, parameter))
                {
                    return Err(refuse(
                        &element.name,
                        entry.model.name.as_str(),
                        ElaborationErrorKind::ParameterValue,
                        format!(
                            "parameter '{name}' is a model parameter; set it on the model card '{}'",
                            card.map_or("", |card| card.name.as_str())
                        ),
                    ));
                }
            }
            bound.extend(card_values.into_iter().filter(|(name, _)| {
                !params.iter().any(|(set, _)| {
                    set.eq_ignore_ascii_case(name)
                        || declared(set).is_some_and(|parameter| declared(name) == Some(parameter))
                })
            }));
        } else {
            bound.extend(card_values);
        }
        bound.extend(params.iter().cloned());
        if let crate::netlist::ElementKind::Subcircuit {
            subckt_name,
            params,
        } = &mut element.kind
        {
            *subckt_name = selected;
            *params = bound;
        }
    }
    Ok(())
}

pub(super) fn prepare<'a>(
    netlist: &crate::Netlist,
    element: &crate::netlist::Element,
    entry: &'a CachedVerilogAModel,
    specializations: &mut InstanceSpecializations,
    temperature: f64,
    delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
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
    let mut strings = Vec::new();
    let mut arrays = Vec::new();
    let mut multiplicity = None;
    // Verilog-AMS 2.4 section 3.4.7: overriding a parameter by its name and
    // an alias, or by two aliases, is an error even when the values agree.
    let mut set_as = Vec::new();
    let set_twice = |parameter: &str, previous: &str, written: &str| {
        refuse(
            &element.name,
            subckt_name,
            ElaborationErrorKind::ParameterValue,
            format!(
                "parameter '{parameter}' is set twice, as '{previous}' and as '{written}': \
                 an instance sets a parameter by its name or by one alias, not both"
            ),
        )
    };
    let mut specializes = entry
        .canonical_ir
        .as_deref()
        .is_some_and(|artifact| artifact.digital.has_executable_content());
    for (name, value) in params {
        if super::veriloga_instance_temperature_key(name, &declares).is_some() {
            continue;
        }
        if let crate::netlist::ParametricValue::Array(elements) = value {
            let parameter = public_parameter(entry, name).ok_or_else(|| {
                refuse(
                    &element.name,
                    subckt_name,
                    ElaborationErrorKind::ParameterUnknown,
                    format!("unknown parameter '{name}'"),
                )
            })?;
            if !parameter.is_array {
                return Err(refuse(
                    &element.name,
                    subckt_name,
                    ElaborationErrorKind::ParameterValue,
                    format!("parameter '{name}' is not an array and takes one value"),
                ));
            }
            if let Some(previous) = set_by_another_name(&mut set_as, parameter.name, name) {
                return Err(set_twice(parameter.name, previous, name));
            }
            let values = elements
                .iter()
                .map(|item| {
                    super::veriloga_instance_numeric_value(
                        &element.name,
                        subckt_name,
                        name,
                        item,
                        &mut context,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            specializes = true;
            arrays.push((parameter.name, values));
            continue;
        }
        if let Some(parameter) = public_parameter(entry, name).filter(|found| found.is_string) {
            let crate::netlist::ParametricValue::String(text) = value else {
                return Err(refuse(
                    &element.name,
                    subckt_name,
                    ElaborationErrorKind::ParameterValue,
                    format!("parameter '{name}' is a string parameter and requires a string value"),
                ));
            };
            if let Some(previous) = set_by_another_name(&mut set_as, parameter.name, name) {
                return Err(set_twice(parameter.name, previous, name));
            }
            specializes = true;
            strings.push((parameter.name, text.as_str()));
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
        if parameter.is_array {
            return Err(refuse(
                &element.name,
                subckt_name,
                ElaborationErrorKind::ParameterValue,
                format!("parameter '{name}' is an array and takes a value such as [1 2 3]"),
            ));
        }
        if let Some(previous) = set_by_another_name(&mut set_as, parameter.name, name) {
            return Err(set_twice(parameter.name, previous, name));
        }
        specializes |= parameter.needs_source;
        overrides.push((parameter.name, value));
    }

    let (model, canonical_ir) = if specializes
        && !(overrides.is_empty() && strings.is_empty() && arrays.is_empty())
    {
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
        strings.sort_by(|left, right| left.0.cmp(right.0));
        arrays.sort_by(|left, right| left.0.cmp(right.0));
        let key = (
            Arc::as_ptr(&entry.model) as usize,
            artifact as *const _ as usize,
            overrides
                .iter()
                .map(|(name, value)| (name.to_string(), SpecializedValue::Number(value.to_bits())))
                .chain(strings.iter().map(|(name, text)| {
                    (name.to_string(), SpecializedValue::String(text.to_string()))
                }))
                .chain(arrays.iter().map(|(name, values)| {
                    (
                        name.to_string(),
                        SpecializedValue::Array(values.iter().map(|v| v.to_bits()).collect()),
                    )
                }))
                .collect(),
        );
        let specialized = if let Some(specialized) = specializations.get(&key) {
            specialized.clone()
        } else {
            // The artifact was compiled under the deck's `wreal` resolution,
            // with its cells and under its min:typ:max selection, and its
            // specialization reads its declarations and leaves the same way.
            let compiler = rspice_veriloga::VerilogACompiler::new(
                super::veriloga_cache::deck_include_compiler_options(
                    &super::veriloga_cache::DeckHdl {
                        wreal_resolution: super::veriloga_sources::wreal_resolution(
                            netlist.options.wreal_resolution,
                        ),
                        external_cells: Some(Arc::new(
                            super::hdl_spice_leaves::hdl_external_cells(netlist),
                        )),
                        delay_select,
                        defines: Vec::new(),
                    },
                ),
            );
            let typed: Vec<_> = overrides
                .iter()
                .map(|&(name, value)| (name, rspice_veriloga::ScalarParameterValue::Real(value)))
                .chain(strings.iter().map(|&(name, text)| {
                    (
                        name,
                        rspice_veriloga::ScalarParameterValue::String(text.into()),
                    )
                }))
                .chain(arrays.iter().map(|(name, values)| {
                    (
                        *name,
                        rspice_veriloga::ScalarParameterValue::RealArray(values.clone()),
                    )
                }))
                .collect();
            let mut runtime = compiler
                .specialize_mixed_runtime_typed(artifact, &typed, &VerilogACompileControl { abort })
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
            // The specialization compiled the artifact's expanded text, so its
            // analog tasks take the places the model it specializes gave them.
            runtime.model.locate_analog_tasks_as(&entry.model);
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
