//! Bind retained source columns and resolve deferred output expressions.

use super::*;
use crate::output_contract::SourceCandidate as Candidate;
use rspice_results::saved_output::{
    SavedOutputAxis, SavedOutputBoundSource, SavedOutputSourceBindings, saved_output_references,
};

pub(super) fn capture<W: OutputWaveform>(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult<W>,
    source: &[W],
    family: Option<&Result<dc_family::Sources<'_, W>, String>>,
) -> Result<Option<SavedOutputSourceBindings>, String> {
    let Some(references) = saved_output_references(contract.kind(), contract.source_expression())?
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
            name: waveform.as_ref().name.clone(),
        }
    } else {
        SavedOutputAxis::Missing
    };
    let references = references
        .into_iter()
        .map(|signal| {
            let candidates = contract.source_candidates(&signal);
            let bound = candidates
                .iter()
                .find_map(|candidate| match candidate {
                    Candidate::Trace(name) if family.is_none() => source
                        .iter()
                        .find(|w| {
                            w.as_ref().name.eq_ignore_ascii_case(name)
                                || w.as_ref()
                                    .complex
                                    .as_ref()
                                    .is_some_and(|c| c.source_name.eq_ignore_ascii_case(name))
                        })
                        .map(|w| SavedOutputBoundSource::Waveform {
                            name: w.as_ref().name.clone(),
                        }),
                    Candidate::Trace(_) => None,
                    Candidate::Ground => Some(SavedOutputBoundSource::Ground),
                    Candidate::Probe(probe) => {
                        if let Some(family) = family {
                            family
                                .quantity(probe)
                                .map(|quantity| SavedOutputBoundSource::DcQuantity { quantity })
                        } else {
                            let requested = if let Some(
                                rspice_results::analysis_payload::AnalysisResultPayload::Qpac {
                                    response,
                                },
                            ) = &analysis.result_payload
                            {
                                format!(
                                    "{probe} [k={:?}]",
                                    response.metadata.request.output_lattice
                                )
                            } else {
                                probe.clone()
                            };
                            find_literal_waveform(source, &requested).map(|waveform| {
                                SavedOutputBoundSource::Waveform {
                                    name: waveform.as_ref().name.clone(),
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

pub(super) struct Context<'a, W> {
    pub bindings: &'a SavedOutputSourceBindings,
    pub waveforms: &'a [W],
    pub family: Option<(&'a dc_family::Sources<'a, W>, usize)>,
    pub axis: Option<&'a W>,
    pub complex_policy: rspice_results::saved_output::ComplexExpressionPolicy,
}

impl<'a, W: OutputWaveform> Context<'a, W> {
    pub fn resolve(&self, signal: &str) -> Result<probe::Source<'a, W>, String> {
        match self.bindings.references.get(&signal.to_ascii_lowercase()) {
            Some(SavedOutputBoundSource::Ground) => Ok(probe::Source::Ground),
            Some(SavedOutputBoundSource::Waveform { name }) => self
                .waveforms
                .iter()
                .find(|waveform| waveform.as_ref().name == *name)
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

impl<W: OutputWaveform> calculator::evaluator::EvaluationContext for Context<'_, W> {
    fn get_magnitude(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, calculator::evaluator::EvaluationError> {
        let source = match self.resolve(signal) {
            Ok(probe::Source::Waveform(waveform)) => Some(waveform),
            _ => None,
        };
        calculator::retained::magnitude_value(
            self.get_waveform(signal, dataset),
            source.map(AsRef::as_ref),
        )
    }

    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, calculator::evaluator::EvaluationError> {
        let absent =
            || calculator::evaluator::EvaluationError::IdentifierNotFound(signal.to_owned());
        if dataset.is_some() {
            return Err(absent());
        }
        if matches!(
            signal.to_ascii_uppercase().as_str(),
            "TIME" | "T" | "FREQ" | "FREQUENCY"
        ) {
            return self
                .axis
                .map(|wave| {
                    CalcValue::create_waveform(wave.as_ref().x.to_vec(), wave.as_ref().x.to_vec())
                })
                .ok_or_else(absent);
        }
        match self.resolve(signal).map_err(|_| absent())? {
            probe::Source::Ground => self
                .axis
                .map(|wave| {
                    CalcValue::create_waveform(
                        wave.as_ref().x.to_vec(),
                        vec![0.0; wave.as_ref().x.len()],
                    )
                })
                .ok_or_else(absent),
            probe::Source::Waveform(wave) => {
                calculator::retained::waveform_value(wave.as_ref(), self.complex_policy)
            }
        }
    }
}

pub(super) fn resolve<W: OutputWaveform>(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult<W>,
    source: &[W],
    bindings: &SavedOutputSourceBindings,
) -> Result<W, String> {
    let op_axis = W::from_retained(RetainedWaveform::new("OP axis", vec![0.0], vec![0.0]));
    let axis = match &bindings.axis {
        SavedOutputAxis::OperatingPoint if analysis.dc_op.is_some() => Some(&op_axis),
        SavedOutputAxis::Waveform { name } => {
            source.iter().find(|wave| wave.as_ref().name == *name)
        }
        _ => None,
    };
    let context = Context {
        bindings,
        waveforms: source,
        family: None,
        axis,
        complex_policy: contract.complex_policy(),
    };
    match contract.kind() {
        SavedOutputKind::RawVoltageOrCurrent => probe::resolve_bound_raw_probe(
            contract.source_expression(),
            contract.name(),
            axis,
            analysis.analysis_type.uses_complex_bode_projection(),
            |signal| context.resolve(signal),
        ),
        SavedOutputKind::DerivedExpression => {
            let mut waveform = resolve_derived_with(
                contract.source_expression(),
                contract.name(),
                &context,
                axis,
            )?;
            // A direct quoted trace keeps the producer's units. Composite
            // calculator expressions retain their existing unstated-unit contract.
            if let Ok(calculator::ast::CalculatorExpr::WaveformRef { signal, .. }) =
                calculator::spice_parser::Parser::new(contract.source_expression()).try_parse()
                && let Ok(probe::Source::Waveform(source)) = context.resolve(&signal)
            {
                waveform.as_mut().unit = source.as_ref().unit.clone();
            }
            Ok(waveform)
        }
        _ => Err("source bindings require a voltage/current or derived output".to_owned()),
    }
}
