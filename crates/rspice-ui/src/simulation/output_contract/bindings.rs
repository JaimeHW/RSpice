//! Bind authored references once against the engine basis of their owning task.
//! Deferred evaluation uses exact retained columns, never authored output labels
//! or whichever deck happens to be open later.

use std::collections::BTreeMap;

use super::*;
use crate::state::{
    SavedOutputAxis, SavedOutputBoundSource, SavedOutputSourceBindings, saved_output_references,
};
use rspice_core::netlist::GroundPolicy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Candidate {
    Probe(String),
    Ground,
}

pub(super) type Candidates = BTreeMap<String, Vec<Candidate>>;

impl PreparedSavedOutput {
    pub(in crate::simulation) fn bind_deck(
        contracts: &mut [Self],
        netlist: &rspice_core::Netlist,
    ) -> Result<(), String> {
        let mut references = std::collections::BTreeSet::new();
        for contract in contracts.iter() {
            references.extend(
                saved_output_references(contract.kind, &contract.source_expression)?
                    .into_iter()
                    .flatten(),
            );
        }
        let requested = references
            .iter()
            .filter_map(|signal| {
                let (current, node) = probe_identity(signal);
                (!current).then(|| {
                    crate::state::ProbeTarget::engine_alias(node).unwrap_or_else(|| node.to_owned())
                })
            })
            .collect();
        let aliases = rspice_core::netlist::collect_requested_interface_node_aliases_with_abort(
            netlist,
            &requested,
            &rspice_core::abort_signal::NoAbort,
        )
        .map_err(|error| format!("saved-output interface binding failed: {error}"))?;
        let candidates = Arc::new(
            references
                .into_iter()
                .map(|signal| {
                    let list = candidates(&signal, netlist.ground_policy(), Some(&aliases));
                    (signal, list)
                })
                .collect(),
        );
        for contract in contracts {
            contract.candidates = Some(Arc::clone(&candidates));
        }
        Ok(())
    }
}

fn candidates(
    signal: &str,
    ground: GroundPolicy,
    aliases: Option<&rspice_core::netlist::InterfaceNodeAliases>,
) -> Vec<Candidate> {
    let (current, node) = probe_identity(signal);
    let mut result = Vec::new();
    let mut add = |node: &str| {
        let candidate = if !current && ground.is_ground(node) {
            Candidate::Ground
        } else {
            Candidate::Probe(format!("{}({node})", if current { "I" } else { "V" }))
        };
        if !result.contains(&candidate) {
            result.push(candidate);
        }
    };
    // Literal slash/dotted nodes win before hierarchy and formal-port aliases.
    add(node);
    let engine = crate::state::ProbeTarget::engine_alias(node);
    if let Some(engine) = &engine {
        add(engine);
    }
    if !current
        && let Some(target) =
            aliases.and_then(|aliases| aliases.resolve(engine.as_deref().unwrap_or(node)))
    {
        add(target);
    }
    result
}

pub(super) fn capture(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult,
    source: &[WaveformData],
    family: Option<&Result<dc_family::Sources<'_>, String>>,
) -> Result<Option<SavedOutputSourceBindings>, String> {
    let Some(references) = saved_output_references(contract.kind, &contract.source_expression)?
    else {
        return Ok(None);
    };
    let family = family
        .map(|family| family.as_ref().map_err(Clone::clone))
        .transpose()?;
    let axis = if family.is_some() {
        SavedOutputAxis::DcFamily
    } else if analysis.dc_op.is_some() {
        SavedOutputAxis::OperatingPoint
    } else if let Some(waveform) = source.first() {
        SavedOutputAxis::Waveform {
            name: waveform.name.clone(),
        }
    } else {
        SavedOutputAxis::Missing
    };
    let references = references
        .into_iter()
        .map(|signal| {
            let fallback;
            let candidates = if let Some(prepared) = contract
                .candidates
                .as_ref()
                .and_then(|map| map.get(&signal))
            {
                prepared
            } else {
                // Old OP/DC evidence and low-level callers know only canonical zero.
                fallback = candidates(&signal, GroundPolicy::OnlyZero, None);
                &fallback
            };
            let bound = candidates
                .iter()
                .find_map(|candidate| match candidate {
                    Candidate::Ground => Some(SavedOutputBoundSource::Ground),
                    Candidate::Probe(probe) => {
                        if let Some(family) = family {
                            family
                                .quantity(probe)
                                .map(|quantity| SavedOutputBoundSource::DcQuantity { quantity })
                        } else {
                            find_literal_waveform(source, probe).map(|waveform| {
                                SavedOutputBoundSource::Waveform {
                                    name: waveform.name.clone(),
                                }
                            })
                        }
                    }
                })
                .unwrap_or(SavedOutputBoundSource::Missing);
            (signal, bound)
        })
        .collect();
    Ok(Some(SavedOutputSourceBindings { axis, references }))
}

pub(super) struct Context<'a> {
    pub bindings: &'a SavedOutputSourceBindings,
    pub waveforms: &'a [WaveformData],
    pub family: Option<(&'a dc_family::Sources<'a>, usize)>,
    pub axis: Option<&'a WaveformData>,
}

impl<'a> Context<'a> {
    pub fn resolve(&self, signal: &str) -> Result<probe::Source<'a>, String> {
        match self.bindings.references.get(&signal.to_ascii_lowercase()) {
            Some(SavedOutputBoundSource::Ground) => Ok(probe::Source::Ground),
            Some(SavedOutputBoundSource::Waveform { name }) => self
                .waveforms
                .iter()
                .find(|waveform| waveform.name == *name)
                .map(probe::Source::Waveform)
                .ok_or_else(|| format!("bound source '{name}' is no longer retained")),
            Some(SavedOutputBoundSource::DcQuantity { quantity }) => self
                .family
                .and_then(|(family, member)| family.curve(*quantity, member))
                .map(probe::Source::Waveform)
                .ok_or_else(|| format!("bound DC quantity for '{signal}' is no longer retained")),
            Some(SavedOutputBoundSource::Missing) | None => Err(format!(
                "source probe '{signal}' was absent from the solved basis"
            )),
        }
    }
}

impl calculator::EvaluationContext for Context<'_> {
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, calculator::EvaluationError> {
        let absent = || calculator::EvaluationError::IdentifierNotFound(signal.to_owned());
        if dataset.is_some() {
            return Err(absent());
        }
        if matches!(
            signal.to_ascii_uppercase().as_str(),
            "TIME" | "T" | "FREQ" | "FREQUENCY"
        ) {
            return self
                .axis
                .map(|wave| CalcValue::create_waveform(wave.x.to_vec(), wave.x.to_vec()))
                .ok_or_else(absent);
        }
        match self.resolve(signal).map_err(|_| absent())? {
            probe::Source::Ground => self
                .axis
                .map(|wave| CalcValue::create_waveform(wave.x.to_vec(), vec![0.0; wave.x.len()]))
                .ok_or_else(absent),
            probe::Source::Waveform(wave) => {
                Ok(CalcValue::create_waveform(wave.x.to_vec(), wave.y.to_vec()))
            }
        }
    }
}

pub(super) fn resolve(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult,
    source: &[WaveformData],
    bindings: &SavedOutputSourceBindings,
) -> Result<WaveformData, String> {
    let op_axis = WaveformData::new("OP axis", vec![0.0], vec![0.0], "#f5b700");
    let axis = match &bindings.axis {
        SavedOutputAxis::OperatingPoint if analysis.dc_op.is_some() => Some(&op_axis),
        SavedOutputAxis::Waveform { name } => source.iter().find(|wave| wave.name == *name),
        _ => None,
    };
    let context = Context {
        bindings,
        waveforms: source,
        family: None,
        axis,
    };
    match contract.kind {
        SavedOutputKind::RawVoltageOrCurrent => probe::resolve_bound_raw_probe(
            &contract.source_expression,
            &contract.name,
            axis,
            analysis.analysis_type.uses_complex_bode_projection(),
            |signal| context.resolve(signal),
        ),
        SavedOutputKind::DerivedExpression => {
            resolve_derived_with(&contract.source_expression, &contract.name, &context, axis)
        }
        _ => Err("source bindings require a voltage/current or derived output".to_owned()),
    }
}
